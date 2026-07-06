use axum::{
    extract::{
        Json, Path, Query, State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    http::StatusCode,
    response::IntoResponse,
};
use futures::stream::StreamExt;
use maxminddb::Reader;
use rustls::ClientConfig;
use std::sync::Arc;
use tokio_rustls::TlsConnector;

use crate::models::{
    AppState, DbScanResult, DeleteResultQuery, ResultsQuery, ScanRequest, SystemSettings,
};
use crate::scan_events::{ScanEvent, ScanMode, ScanStatus};
use crate::scanner::{DummyVerifier, scan_tls};
use crate::settings::{
    load_settings, normalize_requested_ports, scan_history_key, validate_settings,
};
use crate::targets::resolve_target;

type CachedResultRow = (
    String,
    i64,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    i64,
    String,
    bool,
);
type DbResultRow = (
    String,
    i64,
    String,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<i64>,
    Option<String>,
);

pub async fn get_settings_handler(State(state): State<AppState>) -> Json<SystemSettings> {
    Json(load_settings(&state.db).await)
}

pub async fn put_settings_handler(
    State(state): State<AppState>,
    Json(settings): Json<SystemSettings>,
) -> Result<StatusCode, (StatusCode, String)> {
    let settings = validate_settings(settings).map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    let queries = vec![
        ("concurrency_limit", settings.concurrency_limit.to_string()),
        ("max_cidr_ipv4", settings.max_cidr_ipv4.to_string()),
        ("max_cidr_ipv6", settings.max_cidr_ipv6.to_string()),
        ("cooldown_days", settings.cooldown_days.to_string()),
        ("allowed_ports", settings.allowed_ports.clone()),
    ];

    for (k, v) in queries {
        sqlx::query(
            "INSERT INTO system_settings (key, value) VALUES (?, ?)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        )
        .bind(k)
        .bind(v)
        .execute(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }

    Ok(StatusCode::OK)
}

pub async fn ws_scan_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn send_event(socket: &mut WebSocket, event: ScanEvent) -> bool {
    socket.send(Message::Text(event.into_text())).await.is_ok()
}

async fn check_scan_history(
    db: &sqlx::SqlitePool,
    target: &str,
    settings: &SystemSettings,
) -> Result<usize, String> {
    let history_row: Option<(i64, i64, String, String)> = sqlx::query_as(
        "SELECT total_tasks, completed_tasks, status, scanned_at FROM scan_history WHERE target = ? ORDER BY id DESC LIMIT 1"
    )
    .bind(target)
    .fetch_optional(db)
    .await
    .unwrap_or(None);

    if let Some((_total, completed, status, _scanned_at)) = history_row {
        if status == "COMPLETED" {
            if settings.cooldown_days > 0 {
                let recent: i64 = sqlx::query_scalar(
                    "SELECT COUNT(*) FROM scan_history WHERE target = ? AND status = 'COMPLETED' AND scanned_at > datetime('now', ?)"
                )
                .bind(target)
                .bind(format!("-{} days", settings.cooldown_days))
                .fetch_one(db)
                .await
                .unwrap_or(0);

                if recent > 0 {
                    return Err("CACHED".into());
                }
            }
        } else if status == "IN_PROGRESS" {
            return Ok(completed as usize);
        }
    }
    Ok(0)
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {
    let settings = load_settings(&state.db).await;

    let msg = match socket.recv().await {
        Some(Ok(Message::Text(text))) => text,
        _ => return,
    };

    let payload: ScanRequest = match serde_json::from_str(&msg) {
        Ok(p) => p,
        Err(_) => return,
    };

    let target = payload.target.trim().to_string();
    if target.is_empty() {
        let _ = socket
            .send(Message::Text("Error: Target is required".into()))
            .await;
        return;
    }

    let ports =
        match normalize_requested_ports(payload.ports.unwrap_or_else(|| vec![443]), &settings) {
            Ok(ports) => ports,
            Err(e) => {
                let _ = socket.send(Message::Text(e)).await;
                return;
            }
        };

    let timeout_secs = payload.timeout.unwrap_or(10);
    if !(1..=60).contains(&timeout_secs) {
        let _ = socket
            .send(Message::Text(
                "Error: Timeout must be between 1 and 60 seconds".into(),
            ))
            .await;
        return;
    }

    let resolved = match resolve_target(&target, &settings).await {
        Ok(res) => res,
        Err(e) => {
            let _ = socket.send(Message::Text(e)).await;
            return;
        }
    };
    let ips_to_scan = resolved.ips;
    let origins = resolved.origins;
    let resolved_target = resolved.display_target;

    let history_key = scan_history_key(&resolved_target, &ports, timeout_secs);

    let mut use_cache = false;
    let skip_tasks = match check_scan_history(&state.db, &history_key, &settings).await {
        Ok(skip) => skip,
        Err(e) => {
            if e == "CACHED" {
                use_cache = true;
                0
            } else {
                let _ = socket.send(Message::Text(e)).await;
                return;
            }
        }
    };

    if skip_tasks > 0 && !use_cache {
        let total: i64 = sqlx::query_scalar(
            "SELECT total_tasks FROM scan_history WHERE target = ? ORDER BY id DESC LIMIT 1",
        )
        .bind(&history_key)
        .fetch_one(&state.db)
        .await
        .unwrap_or(0);
        let _ = send_event(
            &mut socket,
            ScanEvent::Info {
                message: format!("Resuming scan from task {}/{}...", skip_tasks, total),
            },
        )
        .await;
    }

    let total_tasks_count = ips_to_scan.len() * ports.len();
    let start_mode = if use_cache {
        ScanMode::Cache
    } else {
        ScanMode::Scan
    };
    let _ = send_event(
        &mut socket,
        ScanEvent::Start {
            mode: start_mode,
            target: resolved_target.clone(),
            ports: ports.clone(),
            total: total_tasks_count,
            completed: skip_tasks,
            resumed: skip_tasks > 0,
        },
    )
    .await;

    if use_cache {
        let _ = send_event(
            &mut socket,
            ScanEvent::Info {
                message: "Target scanned recently. Fetching results from cache...".into(),
            },
        )
        .await;

        let mut completed_tasks = 0usize;
        for (i, ip) in ips_to_scan.iter().enumerate() {
            let origin = &origins[i];
            for port in &ports {
                let row: Option<CachedResultRow> = sqlx::query_as(
                    "SELECT ip, port, origin, tls_version, alpn, cert_domain, cert_issuer, geo_code, asn_org, latency, cert_validity, feasible FROM scan_results WHERE ip = ? AND port = ? ORDER BY scanned_at DESC LIMIT 1"
                )
                .bind(ip.to_string())
                .bind(*port as i64)
                .fetch_optional(&state.db)
                .await
                .unwrap_or(None);

                let res = if let Some(r) = row {
                    crate::models::ScanResult {
                        ip: r.0,
                        port: r.1 as u16,
                        origin: r.2,
                        tls_version: r.3,
                        alpn: r.4,
                        cert_length: "".into(),
                        cert_signature: "".into(),
                        cert_publickey: "".into(),
                        cert_domain: r.5,
                        cert_issuer: r.6,
                        geo_code: r.7,
                        asn_org: r.8,
                        latency: r.9 as u32,
                        cert_validity: r.10,
                        feasible: r.11,
                    }
                } else {
                    crate::scanner::default_fail_result(*ip, *port, origin.clone(), "N/A".into())
                };

                let json = serde_json::to_string(&res).unwrap();
                if socket.send(Message::Text(json)).await.is_err() {
                    return;
                }
                completed_tasks += 1;
                let _ = send_event(
                    &mut socket,
                    ScanEvent::Progress {
                        mode: ScanMode::Cache,
                        completed: completed_tasks,
                        total: total_tasks_count,
                        ip: ip.to_string(),
                        port: *port,
                    },
                )
                .await;
            }
        }
        let _ = send_event(
            &mut socket,
            ScanEvent::Done {
                status: ScanStatus::Completed,
                completed: completed_tasks,
                total: total_tasks_count,
            },
        )
        .await;
        return;
    }

    let mut config = ClientConfig::builder()
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(DummyVerifier))
        .with_no_client_auth();
    config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];
    let tls_connector = TlsConnector::from(Arc::new(config));

    let geo_reader = Reader::open_readfile("Country.mmdb").ok().map(Arc::new);
    let asn_reader = Reader::open_readfile("GeoLite2-ASN.mmdb")
        .ok()
        .map(Arc::new);

    let concurrency_limit = settings.concurrency_limit as usize;
    let total_tasks_count = ips_to_scan.len() * ports.len();

    let mut history_id: i64 = 0;
    if skip_tasks > 0 {
        history_id = sqlx::query_scalar(
            "SELECT id FROM scan_history WHERE target = ? ORDER BY id DESC LIMIT 1",
        )
        .bind(&history_key)
        .fetch_one(&state.db)
        .await
        .unwrap_or(0);
        let _ = sqlx::query("UPDATE scan_history SET status = 'IN_PROGRESS' WHERE id = ?")
            .bind(history_id)
            .execute(&state.db)
            .await;
    } else {
        if let Ok(res) = sqlx::query("INSERT INTO scan_history (target, total_tasks, completed_tasks, status) VALUES (?, ?, ?, 'IN_PROGRESS')")
            .bind(&history_key).bind(total_tasks_count as i64).bind(skip_tasks as i64).execute(&state.db).await {
            history_id = res.last_insert_rowid();
        }
    }

    let iter = ips_to_scan.into_iter().zip(origins).flat_map(|(ip, origin)| {
        let ports = ports.clone();
        ports.into_iter().map(move |p| (ip, origin.clone(), p))
    }).map(|(ip, origin, port)| {
        let tls = tls_connector.clone();
        let geo = geo_reader.clone();
        let asn = asn_reader.clone();
        let db = state.db.clone();
        async move {
            let res = scan_tls(ip, origin, port, timeout_secs, tls, geo, asn).await;
            if res.feasible {
                let db_res = sqlx::query(
                    "INSERT INTO scan_results (ip, port, origin, tls_version, alpn, cert_domain, cert_issuer, geo_code, asn_org, latency, cert_validity, feasible, cert_type, scanned_at)
                     VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP)
                     ON CONFLICT(ip, port) DO UPDATE SET
                     origin=excluded.origin, tls_version=excluded.tls_version, alpn=excluded.alpn, 
                     cert_domain=excluded.cert_domain, cert_issuer=excluded.cert_issuer, 
                     geo_code=excluded.geo_code, asn_org=excluded.asn_org, latency=excluded.latency, cert_validity=excluded.cert_validity, feasible=excluded.feasible, cert_type=excluded.cert_type, scanned_at=CURRENT_TIMESTAMP
                     RETURNING id"
                )
                .bind(&res.ip).bind(res.port).bind(&res.origin).bind(&res.tls_version).bind(&res.alpn).bind(&res.cert_domain).bind(&res.cert_issuer).bind(&res.geo_code).bind(&res.asn_org).bind(res.latency).bind(&res.cert_validity).bind(res.feasible).bind(&res.cert_publickey)
                .fetch_one(&db).await;

                match db_res {
                    Ok(row) => {
                        let result_id: i64 = sqlx::Row::get(&row, 0);
                        if history_id > 0 {
                            let _ = sqlx::query("INSERT OR IGNORE INTO scan_task_results (history_id, result_id) VALUES (?, ?)")
                                .bind(history_id)
                                .bind(result_id)
                                .execute(&db).await;
                        }
                    }
                    Err(e) => eprintln!("DB Insert Error: {}", e),
                }
            }
            res
        }
    });

    let mut stream = futures::stream::iter(iter.skip(skip_tasks)).buffered(concurrency_limit);
    let mut completed_tasks = skip_tasks;
    let mut broken = false;

    loop {
        tokio::select! {
            res_opt = stream.next() => {
                if let Some(res) = res_opt {
                    completed_tasks += 1;
                    let json = serde_json::to_string(&res).unwrap();
                    if socket.send(Message::Text(json)).await.is_err() {
                        broken = true;
                        break;
                    }
                    let _ = send_event(
                        &mut socket,
                        ScanEvent::Progress {
                            mode: ScanMode::Scan,
                            completed: completed_tasks,
                            total: total_tasks_count,
                            ip: res.ip,
                            port: res.port,
                        },
                    )
                    .await;
                } else {
                    break;
                }
            },
            msg_opt = socket.recv() => {
                match msg_opt {
                    Some(Ok(Message::Text(text))) => {
                        if text.contains("\"stop\"") {
                            broken = true;
                            break;
                        }
                    },
                    Some(Ok(Message::Close(_))) | None | Some(Err(_)) => {
                        broken = true;
                        break;
                    },
                    _ => {}
                }
            }
        }
    }

    if broken {
        let _ = sqlx::query(
            "UPDATE scan_history SET completed_tasks = ?, status = 'IN_PROGRESS' WHERE id = ?",
        )
        .bind(completed_tasks as i64)
        .bind(history_id)
        .execute(&state.db)
        .await;
        let _ = send_event(
            &mut socket,
            ScanEvent::Done {
                status: ScanStatus::Stopped,
                completed: completed_tasks,
                total: total_tasks_count,
            },
        )
        .await;
    } else {
        let _ = sqlx::query("UPDATE scan_history SET completed_tasks = ?, status = 'COMPLETED', scanned_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(completed_tasks as i64).bind(history_id).execute(&state.db).await;
        let _ = send_event(
            &mut socket,
            ScanEvent::Done {
                status: ScanStatus::Completed,
                completed: completed_tasks,
                total: total_tasks_count,
            },
        )
        .await;
    }
}

pub async fn delete_history_task_handler(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<i64>,
) -> Result<StatusCode, (StatusCode, String)> {
    let mut tx = state
        .db
        .begin()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    sqlx::query("DELETE FROM scan_task_results WHERE history_id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let res = sqlx::query("DELETE FROM scan_history WHERE id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tx.commit()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if res.rows_affected() > 0 {
        Ok(StatusCode::OK)
    } else {
        Err((StatusCode::NOT_FOUND, "History task not found".to_string()))
    }
}

pub async fn get_history_tasks_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<crate::models::DbScanHistory>>, (StatusCode, String)> {
    let rows: Vec<(i64, String, i64, i64, String, String)> = sqlx::query_as(
        "SELECT id, target, total_tasks, completed_tasks, status, scanned_at FROM scan_history ORDER BY scanned_at DESC LIMIT 100",
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let results = rows
        .into_iter()
        .map(|row| crate::models::DbScanHistory {
            id: row.0,
            target: row.1,
            total_tasks: row.2 as i32,
            completed_tasks: row.3 as i32,
            status: row.4,
            scanned_at: row.5,
        })
        .collect();

    Ok(Json(results))
}

pub async fn get_results_handler(
    State(state): State<AppState>,
    Query(query): Query<ResultsQuery>,
) -> Result<Json<Vec<DbScanResult>>, (StatusCode, String)> {
    let mut query_builder = sqlx::QueryBuilder::new(
        "SELECT r.ip, r.port, r.origin, r.tls_version, r.alpn, r.cert_domain, r.cert_issuer, r.geo_code, r.cert_type, r.scanned_at, r.asn_org, r.latency, r.cert_validity FROM scan_results r",
    );

    if let Some(history_id) = query.history_id {
        query_builder
            .push(" JOIN scan_task_results str ON r.id = str.result_id WHERE str.history_id = ");
        query_builder.push_bind(history_id);
        query_builder.push(" AND r.feasible = true");
    } else {
        query_builder.push(" WHERE r.feasible = true");
    }

    if let Some(geo) = query.geo_code {
        query_builder.push(" AND geo_code = ");
        query_builder.push_bind(geo);
    }

    if let Some(domain) = query.domain {
        query_builder.push(" AND cert_domain LIKE ");
        query_builder.push_bind(format!("%{}%", domain));
    }

    if let Some(port) = query.port {
        query_builder.push(" AND port = ");
        query_builder.push_bind(port);
    }

    query_builder.push(" ORDER BY r.scanned_at DESC");

    let limit = query.limit.unwrap_or(100).clamp(1, 1000);
    query_builder.push(" LIMIT ");
    query_builder.push_bind(limit);

    let rows: Vec<DbResultRow> = query_builder
        .build_query_as()
        .fetch_all(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let results = rows
        .into_iter()
        .map(|row| DbScanResult {
            ip: row.0,
            port: row.1 as u16,
            origin: row.2,
            tls_version: row.3.unwrap_or_default(),
            alpn: row.4.unwrap_or_default(),
            cert_domain: row.5.unwrap_or_default(),
            cert_issuer: row.6.unwrap_or_default(),
            geo_code: row.7.unwrap_or_default(),
            cert_type: row.8.unwrap_or_default(),
            scanned_at: row.9.unwrap_or_default(),
            asn_org: row.10.unwrap_or_default(),
            latency: row.11.unwrap_or(0) as u32,
            cert_validity: row.12.unwrap_or_default(),
        })
        .collect();

    Ok(Json(results))
}

pub async fn delete_result_handler(
    State(state): State<AppState>,
    Path(ip): Path<String>,
    Query(query): Query<DeleteResultQuery>,
) -> Result<StatusCode, (StatusCode, String)> {
    if let Some(port) = query.port {
        sqlx::query("DELETE FROM scan_results WHERE ip = ? AND port = ?")
            .bind(ip)
            .bind(port)
            .execute(&state.db)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    } else {
        sqlx::query("DELETE FROM scan_results WHERE ip = ?")
            .bind(ip)
            .execute(&state.db)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }

    Ok(StatusCode::OK)
}
