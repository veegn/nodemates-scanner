use axum::{
    extract::{
        Json, Path, Query, State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    http::StatusCode,
    response::IntoResponse,
};
use futures::stream::StreamExt;
use ipnet::IpNet;
use maxminddb::Reader;
use rustls::ClientConfig;
use serde_json::json;
use std::{net::IpAddr, str::FromStr, sync::Arc};
use tokio_rustls::TlsConnector;

use crate::models::{
    AppState, DbScanResult, DeleteResultQuery, ResultsQuery, ScanRequest, SystemSettings,
};
use crate::scanner::{DummyVerifier, is_internal_ip, scan_tls};

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

pub async fn load_settings(db: &sqlx::SqlitePool) -> SystemSettings {
    let rows: Vec<(String, String)> = sqlx::query_as("SELECT key, value FROM system_settings")
        .fetch_all(db)
        .await
        .unwrap_or_default();

    let mut s = default_settings();

    for (k, v) in rows {
        match k.as_str() {
            "concurrency_limit" => s.concurrency_limit = v.parse().unwrap_or(s.concurrency_limit),
            "max_cidr_ipv4" => s.max_cidr_ipv4 = v.parse().unwrap_or(s.max_cidr_ipv4),
            "max_cidr_ipv6" => s.max_cidr_ipv6 = v.parse().unwrap_or(s.max_cidr_ipv6),
            "cooldown_days" => s.cooldown_days = v.parse().unwrap_or(s.cooldown_days),
            "allowed_ports" => s.allowed_ports = v,
            _ => {}
        }
    }
    validate_settings(s).unwrap_or_else(|_| default_settings())
}

fn default_settings() -> SystemSettings {
    SystemSettings {
        concurrency_limit: 50,
        max_cidr_ipv4: 24,
        max_cidr_ipv6: 120,
        cooldown_days: 30,
        allowed_ports: "443,8443,2053,2083,2087,2096".into(),
    }
}

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

fn validate_settings(mut settings: SystemSettings) -> Result<SystemSettings, String> {
    if !(1..=1000).contains(&settings.concurrency_limit) {
        return Err("concurrency_limit must be between 1 and 1000".into());
    }
    if !(8..=32).contains(&settings.max_cidr_ipv4) {
        return Err("max_cidr_ipv4 must be between 8 and 32".into());
    }
    if !(64..=128).contains(&settings.max_cidr_ipv6) {
        return Err("max_cidr_ipv6 must be between 64 and 128".into());
    }
    if settings.cooldown_days > 365 {
        return Err("cooldown_days must be between 0 and 365".into());
    }

    let mut ports = parse_ports(&settings.allowed_ports)?;
    ports.sort_unstable();
    ports.dedup();
    settings.allowed_ports = ports
        .into_iter()
        .map(|p| p.to_string())
        .collect::<Vec<_>>()
        .join(",");

    Ok(settings)
}

fn parse_ports(raw: &str) -> Result<Vec<u16>, String> {
    let mut ports = Vec::new();
    for part in raw.split(',') {
        let trimmed = part.trim();
        if trimmed.is_empty() {
            continue;
        }
        let port = trimmed
            .parse::<u16>()
            .map_err(|_| format!("invalid port: {}", trimmed))?;
        if port == 0 {
            return Err("port 0 is not allowed".into());
        }
        ports.push(port);
    }
    if ports.is_empty() {
        return Err("at least one allowed port is required".into());
    }
    Ok(ports)
}

fn normalize_requested_ports(
    mut ports: Vec<u16>,
    settings: &SystemSettings,
) -> Result<Vec<u16>, String> {
    if ports.is_empty() {
        return Err("Error: At least one port is required".into());
    }
    ports.sort_unstable();
    ports.dedup();

    let allowed_ports = parse_ports(&settings.allowed_ports)?;
    for port in &ports {
        if *port == 0 {
            return Err("Error: Port 0 is restricted".into());
        }
        if !allowed_ports.contains(port) {
            return Err(format!("Error: Port {} restricted", port));
        }
    }

    Ok(ports)
}

fn scan_history_key(target: &str, ports: &[u16], timeout_secs: u64) -> String {
    let ports = ports
        .iter()
        .map(|p| p.to_string())
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "target={};ports={};timeout={}",
        target.trim(),
        ports,
        timeout_secs
    )
}

pub async fn ws_scan_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
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

async fn resolve_target(
    target: &str,
    settings: &SystemSettings,
) -> Result<(Vec<IpAddr>, Vec<String>, String), String> {
    let mut ips_to_scan = vec![];
    let mut origins = vec![];
    let mut resolved_target = target.to_string();

    if let Ok(net) = IpNet::from_str(target) {
        match net {
            IpNet::V4(net_v4) if net_v4.prefix_len() < settings.max_cidr_ipv4 => {
                return Err(format!(
                    "Error: CIDR too large (max /{})",
                    settings.max_cidr_ipv4
                ));
            }
            IpNet::V6(net_v6) if net_v6.prefix_len() < settings.max_cidr_ipv6 => {
                return Err(format!(
                    "Error: CIDR too large (max /{})",
                    settings.max_cidr_ipv6
                ));
            }
            _ => {}
        }
        for ip in net.hosts() {
            if !is_internal_ip(&ip) {
                ips_to_scan.push(ip);
                origins.push(ip.to_string());
            }
        }
    } else if let Ok(ip) = IpAddr::from_str(target) {
        let prefix_len = match ip {
            IpAddr::V4(_) => settings.max_cidr_ipv4,
            IpAddr::V6(_) => settings.max_cidr_ipv6,
        };
        let net = IpNet::new(ip, prefix_len)
            .map_err(|_| "Error: Failed to expand IP into subnet".to_string())?
            .trunc();
        resolved_target = net.to_string();
        for ip in net.hosts() {
            if !is_internal_ip(&ip) {
                ips_to_scan.push(ip);
                origins.push(ip.to_string());
            }
        }
    } else {
        if let Ok(resolved) = tokio::net::lookup_host((target, 443)).await {
            let mut addrs: Vec<_> = resolved.map(|a| a.ip()).collect();
            addrs.sort();
            for ip in addrs {
                if !is_internal_ip(&ip) {
                    ips_to_scan.push(ip);
                    origins.push(target.to_string());
                }
            }
        }
    }

    if ips_to_scan.is_empty() {
        return Err("Error: No public IPs found".into());
    }
    Ok((ips_to_scan, origins, resolved_target))
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

    let (ips_to_scan, origins, resolved_target) = match resolve_target(&target, &settings).await {
        Ok(res) => res,
        Err(e) => {
            let _ = socket.send(Message::Text(e)).await;
            return;
        }
    };

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
        let _ = socket
            .send(Message::Text(format!(
                r#"{{"type":"info","message":"Resuming scan from task {}/{}..."}}"#,
                skip_tasks, total
            )))
            .await;
    }

    let total_tasks_count = ips_to_scan.len() * ports.len();
    let _ = socket
        .send(Message::Text(
            json!({
                "type": "start",
                "mode": if use_cache { "cache" } else { "scan" },
                "target": resolved_target,
                "ports": ports,
                "total": total_tasks_count,
                "completed": skip_tasks,
                "resumed": skip_tasks > 0,
            })
            .to_string(),
        ))
        .await;

    if use_cache {
        let _ = socket.send(Message::Text(r#"{"type":"info","message":"Target scanned recently. Fetching results from cache..."}"#.to_string())).await;

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
                let _ = socket
                    .send(Message::Text(
                        json!({
                            "type": "progress",
                            "mode": "cache",
                            "completed": completed_tasks,
                            "total": total_tasks_count,
                            "ip": ip.to_string(),
                            "port": port,
                        })
                        .to_string(),
                    ))
                    .await;
            }
        }
        let _ = socket
            .send(Message::Text(
                json!({
                    "type": "done",
                    "status": "completed",
                    "completed": completed_tasks,
                    "total": total_tasks_count,
                })
                .to_string(),
            ))
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
    let asn_reader = Reader::open_readfile("GeoLite2-ASN.mmdb").ok().map(Arc::new);

    let concurrency_limit = settings.concurrency_limit as usize;
    let total_tasks_count = ips_to_scan.len() * ports.len();

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
            if res.feasible
                && let Err(e) = sqlx::query(
                    "INSERT INTO scan_results (ip, port, origin, tls_version, alpn, cert_domain, cert_issuer, geo_code, asn_org, latency, cert_validity, feasible, cert_type, scanned_at)
                     VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP)
                     ON CONFLICT(ip, port) DO UPDATE SET
                     origin=excluded.origin, tls_version=excluded.tls_version, alpn=excluded.alpn, 
                     cert_domain=excluded.cert_domain, cert_issuer=excluded.cert_issuer, 
                     geo_code=excluded.geo_code, asn_org=excluded.asn_org, latency=excluded.latency, cert_validity=excluded.cert_validity, feasible=excluded.feasible, cert_type=excluded.cert_type, scanned_at=CURRENT_TIMESTAMP"
                )
                .bind(&res.ip).bind(res.port).bind(&res.origin).bind(&res.tls_version).bind(&res.alpn).bind(&res.cert_domain).bind(&res.cert_issuer).bind(&res.geo_code).bind(&res.asn_org).bind(res.latency).bind(&res.cert_validity).bind(res.feasible).bind(&res.cert_publickey)
                .execute(&db).await
            {
                eprintln!("DB Insert Error: {}", e);
            }
            res
        }
    });

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
                    let _ = socket
                        .send(Message::Text(
                            json!({
                                "type": "progress",
                                "mode": "scan",
                                "completed": completed_tasks,
                                "total": total_tasks_count,
                                "ip": res.ip,
                                "port": res.port,
                            })
                            .to_string(),
                        ))
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
        let _ = socket
            .send(Message::Text(
                json!({
                    "type": "done",
                    "status": "stopped",
                    "completed": completed_tasks,
                    "total": total_tasks_count,
                })
                .to_string(),
            ))
            .await;
    } else {
        let _ = sqlx::query("UPDATE scan_history SET completed_tasks = ?, status = 'COMPLETED', scanned_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(completed_tasks as i64).bind(history_id).execute(&state.db).await;
        let _ = socket
            .send(Message::Text(
                json!({
                    "type": "done",
                    "status": "completed",
                    "completed": completed_tasks,
                    "total": total_tasks_count,
                })
                .to_string(),
            ))
            .await;
    }
}

pub async fn get_results_handler(
    State(state): State<AppState>,
    Query(query): Query<ResultsQuery>,
) -> Result<Json<Vec<DbScanResult>>, (StatusCode, String)> {
    let mut query_builder = sqlx::QueryBuilder::new(
        "SELECT ip, port, origin, tls_version, alpn, cert_domain, cert_issuer, geo_code, cert_type, scanned_at, asn_org, latency, cert_validity FROM scan_results WHERE feasible = true",
    );

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

    query_builder.push(" ORDER BY scanned_at DESC");

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

#[cfg(test)]
mod tests {
    use super::*;

    fn test_settings() -> SystemSettings {
        SystemSettings {
            concurrency_limit: 50,
            max_cidr_ipv4: 24,
            max_cidr_ipv6: 120,
            cooldown_days: 30,
            allowed_ports: "443".into(),
        }
    }

    #[tokio::test]
    async fn single_ipv4_target_expands_to_configured_subnet() {
        let (ips, origins, resolved_target) = resolve_target("8.8.8.8", &test_settings())
            .await
            .expect("public IPv4 should resolve to its subnet");

        assert_eq!(resolved_target, "8.8.8.0/24");
        assert_eq!(ips.len(), 254);
        assert_eq!(origins.len(), ips.len());
        assert!(ips.iter().any(|ip| ip.to_string() == "8.8.8.8"));
        assert!(origins.iter().any(|origin| origin == "8.8.8.8"));
    }
}
