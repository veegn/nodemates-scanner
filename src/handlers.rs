use axum::{
    extract::{State, Json, Query, Path, ws::{WebSocket, WebSocketUpgrade, Message}},
    response::IntoResponse,
    http::StatusCode,
};
use futures::stream::StreamExt;
use ipnet::IpNet;
use maxminddb::Reader;
use rustls::ClientConfig;
use std::{net::IpAddr, str::FromStr, sync::Arc};
use tokio_rustls::TlsConnector;

use crate::models::{AppState, ScanRequest, DbScanResult, ResultsQuery, SystemSettings};
use crate::scanner::{scan_tls, DummyVerifier, is_internal_ip};

pub async fn load_settings(db: &sqlx::SqlitePool) -> SystemSettings {
    let rows: Vec<(String, String)> = sqlx::query_as("SELECT key, value FROM system_settings").fetch_all(db).await.unwrap_or_default();
    
    let mut s = SystemSettings {
        concurrency_limit: 50,
        max_cidr_ipv4: 24,
        max_cidr_ipv6: 120,
        cooldown_days: 30,
        allowed_ports: "443,8443,2053,2083,2087,2096".into(),
    };
    
    for (k, v) in rows {
        match k.as_str() {
            "concurrency_limit" => s.concurrency_limit = v.parse().unwrap_or(50),
            "max_cidr_ipv4" => s.max_cidr_ipv4 = v.parse().unwrap_or(24),
            "max_cidr_ipv6" => s.max_cidr_ipv6 = v.parse().unwrap_or(120),
            "cooldown_days" => s.cooldown_days = v.parse().unwrap_or(30),
            "allowed_ports" => s.allowed_ports = v,
            _ => {}
        }
    }
    s
}

pub async fn get_settings_handler(State(state): State<AppState>) -> Json<SystemSettings> {
    Json(load_settings(&state.db).await)
}

pub async fn put_settings_handler(State(state): State<AppState>, Json(settings): Json<SystemSettings>) -> Result<StatusCode, (StatusCode, String)> {
    let queries = vec![
        ("concurrency_limit", settings.concurrency_limit.to_string()),
        ("max_cidr_ipv4", settings.max_cidr_ipv4.to_string()),
        ("max_cidr_ipv6", settings.max_cidr_ipv6.to_string()),
        ("cooldown_days", settings.cooldown_days.to_string()),
        ("allowed_ports", settings.allowed_ports.clone()),
    ];
    
    for (k, v) in queries {
        sqlx::query("UPDATE system_settings SET value = ? WHERE key = ?")
            .bind(v).bind(k).execute(&state.db).await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }
    
    Ok(StatusCode::OK)
}

pub async fn ws_scan_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn check_scan_history(db: &sqlx::SqlitePool, target: &str, settings: &SystemSettings) -> Result<usize, String> {
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

async fn resolve_target(target: &str, settings: &SystemSettings) -> Result<(Vec<IpAddr>, Vec<String>), String> {
    let mut ips_to_scan = vec![];
    let mut origins = vec![];

    if let Ok(net) = IpNet::from_str(target) {
        match net {
            IpNet::V4(net_v4) if net_v4.prefix_len() < settings.max_cidr_ipv4 => return Err(format!("Error: CIDR too large (max /{})", settings.max_cidr_ipv4)),
            IpNet::V6(net_v6) if net_v6.prefix_len() < settings.max_cidr_ipv6 => return Err(format!("Error: CIDR too large (max /{})", settings.max_cidr_ipv6)),
            _ => {}
        }
        for ip in net.hosts() {
            if !is_internal_ip(&ip) { ips_to_scan.push(ip); origins.push(ip.to_string()); }
        }
    } else if let Ok(ip) = IpAddr::from_str(target) {
        if !is_internal_ip(&ip) { ips_to_scan.push(ip); origins.push(target.to_string()); }
    } else {
        if let Ok(resolved) = tokio::net::lookup_host((target, 443)).await {
            for addr in resolved {
                let ip = addr.ip();
                if !is_internal_ip(&ip) { ips_to_scan.push(ip); origins.push(target.to_string()); }
            }
        }
    }

    if ips_to_scan.is_empty() {
        return Err("Error: No public IPs found".into());
    }
    Ok((ips_to_scan, origins))
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

    let ports = payload.ports.unwrap_or_else(|| vec![443]);
    let timeout_secs = payload.timeout.unwrap_or(10);
    
    let allowed_ports: Vec<u16> = settings.allowed_ports.split(',').filter_map(|s| s.trim().parse().ok()).collect();
    for port in &ports {
        if !allowed_ports.contains(port) {
            let _ = socket.send(Message::Text(format!("Error: Port {} restricted", port))).await;
            return;
        }
    }

    let mut use_cache = false;
    let skip_tasks = match check_scan_history(&state.db, &payload.target, &settings).await {
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
        let total: i64 = sqlx::query_scalar("SELECT total_tasks FROM scan_history WHERE target = ? ORDER BY id DESC LIMIT 1")
            .bind(&payload.target).fetch_one(&state.db).await.unwrap_or(0);
        let _ = socket.send(Message::Text(format!(r#"{{"type":"info","message":"Resuming scan from task {}/{}..."}}"#, skip_tasks, total))).await;
    }

    let (ips_to_scan, origins) = match resolve_target(&payload.target, &settings).await {
        Ok(res) => res,
        Err(e) => {
            let _ = socket.send(Message::Text(e)).await;
            return;
        }
    };

    let total_tasks_count = ips_to_scan.len() * ports.len();
    let _ = socket.send(Message::Text(format!(r#"{{"type":"start","total":{}}}"#, total_tasks_count))).await;

    if use_cache {
        let _ = socket.send(Message::Text(r#"{"type":"info","message":"Target scanned recently. Fetching results from cache..."}"#.to_string())).await;
        
        for (i, ip) in ips_to_scan.iter().enumerate() {
            let origin = &origins[i];
            for port in &ports {
                let row: Option<(String, i64, String, String, String, String, String, String, bool)> = sqlx::query_as(
                    "SELECT ip, port, origin, tls_version, alpn, cert_domain, cert_issuer, geo_code, feasible FROM scan_results WHERE ip = ? AND port = ? ORDER BY scanned_at DESC LIMIT 1"
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
                        feasible: r.8,
                    }
                } else {
                    crate::scanner::default_fail_result(*ip, *port, origin.clone(), "N/A".into())
                };

                let json = serde_json::to_string(&res).unwrap();
                if socket.send(Message::Text(json)).await.is_err() {
                    return;
                }
            }
        }
        return;
    }

    let mut config = ClientConfig::builder()
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(DummyVerifier))
        .with_no_client_auth();
    config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];
    let tls_connector = TlsConnector::from(Arc::new(config));
    
    let geo_reader = Reader::open_readfile("Country.mmdb").ok().map(Arc::new);

    let concurrency_limit = settings.concurrency_limit as usize;
    let total_tasks_count = ips_to_scan.len() * ports.len();

    let iter = ips_to_scan.into_iter().zip(origins).flat_map(|(ip, origin)| {
        let ports = ports.clone();
        ports.into_iter().map(move |p| (ip, origin.clone(), p))
    }).map(|(ip, origin, port)| {
        let tls = tls_connector.clone();
        let geo = geo_reader.clone();
        let db = state.db.clone();
        async move {
            let res = scan_tls(ip, origin, port, timeout_secs, tls, geo).await;
            if res.feasible {
                let _ = sqlx::query(
                    "INSERT INTO scan_results (ip, port, origin, tls_version, alpn, cert_domain, cert_issuer, geo_code, feasible)
                     VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
                )
                .bind(&res.ip).bind(&res.port).bind(&res.origin).bind(&res.tls_version).bind(&res.alpn).bind(&res.cert_domain).bind(&res.cert_issuer).bind(&res.geo_code).bind(res.feasible)
                .execute(&db).await;
            }
            res
        }
    });

    let rows_affected = sqlx::query("UPDATE scan_history SET total_tasks = ?, completed_tasks = ?, status = 'IN_PROGRESS', scanned_at = CURRENT_TIMESTAMP WHERE target = ?")
        .bind(total_tasks_count as i64).bind(skip_tasks as i64).bind(&payload.target).execute(&state.db).await.unwrap_or_default().rows_affected();
    if rows_affected == 0 {
        let _ = sqlx::query("INSERT INTO scan_history (target, total_tasks, completed_tasks, status) VALUES (?, ?, ?, 'IN_PROGRESS')")
            .bind(&payload.target).bind(total_tasks_count as i64).bind(skip_tasks as i64).execute(&state.db).await;
    }

    let _ = socket.send(Message::Text(format!(r#"{{"type":"start","total":{}}}"#, total_tasks_count))).await;

    let mut stream = futures::stream::iter(iter.skip(skip_tasks)).buffer_unordered(concurrency_limit);
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
                } else {
                    break;
                }
            },
            msg_opt = socket.recv() => {
                if let Some(Ok(Message::Text(text))) = msg_opt {
                    if text.contains("\"stop\"") {
                        broken = true;
                        break;
                    }
                } else if msg_opt.is_none() {
                    broken = true;
                    break;
                }
            }
        }
    }

    if broken {
        let _ = sqlx::query("UPDATE scan_history SET completed_tasks = ?, status = 'IN_PROGRESS' WHERE target = ?")
            .bind(completed_tasks as i64).bind(&payload.target).execute(&state.db).await;
    } else {
        let _ = sqlx::query("UPDATE scan_history SET completed_tasks = ?, status = 'COMPLETED', scanned_at = CURRENT_TIMESTAMP WHERE target = ?")
            .bind(completed_tasks as i64).bind(&payload.target).execute(&state.db).await;
    }
}

pub async fn get_results_handler(
    State(state): State<AppState>,
    Query(query): Query<ResultsQuery>,
) -> Result<Json<Vec<DbScanResult>>, (StatusCode, String)> {
    let mut query_builder = sqlx::QueryBuilder::new(
        "SELECT ip, port, origin, tls_version, alpn, cert_domain, cert_issuer, geo_code, scanned_at FROM scan_results WHERE feasible = true"
    );

    if let Some(geo) = query.geo_code {
        query_builder.push(" AND geo_code = ");
        query_builder.push_bind(geo);
    }
    
    if let Some(domain) = query.domain {
        query_builder.push(" AND cert_domain LIKE ");
        query_builder.push_bind(format!("%{}%", domain));
    }

    query_builder.push(" ORDER BY scanned_at DESC");
    
    let limit = query.limit.unwrap_or(100).clamp(1, 1000);
    query_builder.push(" LIMIT ");
    query_builder.push_bind(limit);

    let rows: Vec<(String, i64, String, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>)> = query_builder
        .build_query_as()
        .fetch_all(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let results = rows.into_iter().map(|row| DbScanResult {
        ip: row.0,
        port: row.1 as u16,
        origin: row.2,
        tls_version: row.3.unwrap_or_default(),
        alpn: row.4.unwrap_or_default(),
        cert_domain: row.5.unwrap_or_default(),
        cert_issuer: row.6.unwrap_or_default(),
        geo_code: row.7.unwrap_or_default(),
        scanned_at: row.8.unwrap_or_default(),
    }).collect();

    Ok(Json(results))
}

pub async fn delete_result_handler(
    State(state): State<AppState>,
    Path(ip): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    sqlx::query("DELETE FROM scan_results WHERE ip = ?")
        .bind(ip)
        .execute(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        
    Ok(StatusCode::OK)
}
