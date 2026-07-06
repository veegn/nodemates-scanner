mod handlers;
mod models;
mod scanner;

use axum::{Router, routing::get};
use futures::stream::StreamExt;
use rustls::ClientConfig;
use sqlx::sqlite::SqlitePoolOptions;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use tokio_rustls::TlsConnector;
use tower_http::services::ServeDir;

use models::AppState;
use scanner::{DummyVerifier, scan_tls};

#[tokio::main]
async fn main() {
    ensure_geo_db().await;
    println!("Initializing node scanner...");

    let db = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:results.db?mode=rwc")
        .await
        .unwrap();

    let _ = sqlx::query("PRAGMA journal_mode=WAL;").execute(&db).await;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS scan_results (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            ip TEXT NOT NULL,
            port INTEGER DEFAULT 443,
            origin TEXT NOT NULL,
            tls_version TEXT,
            alpn TEXT,
            cert_domain TEXT,
            cert_issuer TEXT,
            geo_code TEXT,
            feasible BOOLEAN,
            cert_type TEXT DEFAULT '-',
            scanned_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
        CREATE INDEX IF NOT EXISTS idx_ip ON scan_results(ip);
        CREATE INDEX IF NOT EXISTS idx_domain ON scan_results(cert_domain);
        CREATE INDEX IF NOT EXISTS idx_geo ON scan_results(geo_code);
        CREATE INDEX IF NOT EXISTS idx_feasible ON scan_results(feasible);

        CREATE TABLE IF NOT EXISTS scan_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            target TEXT NOT NULL,
            total_tasks INTEGER DEFAULT 0,
            completed_tasks INTEGER DEFAULT 0,
            status TEXT DEFAULT 'COMPLETED',
            scanned_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
        CREATE INDEX IF NOT EXISTS idx_target ON scan_history(target);
        
        CREATE TABLE IF NOT EXISTS system_settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );",
    )
    .execute(&db)
    .await
    .unwrap();

    // Clean up duplicates before creating the unique index
    let _ = sqlx::query("DELETE FROM scan_results WHERE id NOT IN (SELECT MAX(id) FROM scan_results GROUP BY ip, port)")
        .execute(&db).await;
    let _ = sqlx::query("CREATE UNIQUE INDEX IF NOT EXISTS idx_ip_port ON scan_results(ip, port)")
        .execute(&db)
        .await;

    let default_settings = vec![
        ("concurrency_limit", "50"),
        ("max_cidr_ipv4", "24"),
        ("max_cidr_ipv6", "120"),
        ("cooldown_days", "30"),
        ("allowed_ports", "443,8443,2053,2083,2087,2096"),
    ];

    for (k, v) in default_settings {
        let _ = sqlx::query("INSERT OR IGNORE INTO system_settings (key, value) VALUES (?, ?)")
            .bind(k)
            .bind(v)
            .execute(&db)
            .await;
    }

    // Ignore error if columns already exist
    let _ = sqlx::query("ALTER TABLE scan_results ADD COLUMN port INTEGER DEFAULT 443")
        .execute(&db)
        .await;
    let _ = sqlx::query("ALTER TABLE scan_history ADD COLUMN total_tasks INTEGER DEFAULT 0")
        .execute(&db)
        .await;
    let _ = sqlx::query("ALTER TABLE scan_results ADD COLUMN asn_org TEXT DEFAULT ''")
        .execute(&db)
        .await;
    let _ = sqlx::query("ALTER TABLE scan_history ADD COLUMN completed_tasks INTEGER DEFAULT 0")
        .execute(&db)
        .await;
    let _ = sqlx::query("ALTER TABLE scan_history ADD COLUMN status TEXT DEFAULT 'COMPLETED'")
        .execute(&db)
        .await;

    let state = AppState { db: db.clone() };

    let sched = tokio_cron_scheduler::JobScheduler::new().await.unwrap();
    let db_clone = db.clone();
    sched
        .add(
            tokio_cron_scheduler::Job::new_async("0 0 * * * *", move |_uuid, _l| {
                let db = db_clone.clone();
                Box::pin(async move {
                    println!("Running health check on feasible nodes...");
                    run_health_check(db).await;
                })
            })
            .unwrap(),
        )
        .await
        .unwrap();
    sched.start().await.unwrap();

    let app = Router::new()
        .nest_service("/", ServeDir::new("static"))
        .route("/scan", get(handlers::ws_scan_handler))
        .route("/results", get(handlers::get_results_handler))
        .route(
            "/results/:ip",
            axum::routing::delete(handlers::delete_result_handler),
        )
        .route(
            "/settings",
            get(handlers::get_settings_handler).put(handlers::put_settings_handler),
        )
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn run_health_check(db: sqlx::sqlite::SqlitePool) {
    let rows: Vec<(String, i64, String)> =
        sqlx::query_as("SELECT DISTINCT ip, port, origin FROM scan_results WHERE feasible = true")
            .fetch_all(&db)
            .await
            .unwrap_or_default();

    let mut config = ClientConfig::builder()
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(DummyVerifier))
        .with_no_client_auth();
    config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];
    let tls_connector = TlsConnector::from(Arc::new(config));

    let iter = rows.into_iter().map(|(ip_str, port, origin)| {
        let db = db.clone();
        let tls = tls_connector.clone();
        async move {
            if let Ok(ip) = IpAddr::from_str(&ip_str) {
                let res = scan_tls(ip, origin, port as u16, 5, tls, None, None).await;
                if !res.feasible {
                    if let Err(e) = sqlx::query("UPDATE scan_results SET feasible = false WHERE ip = ? AND port = ?")
                        .bind(&ip_str).bind(port).execute(&db).await {
                        eprintln!("Health check DB error (feasible=false): {}", e);
                    }
                } else {
                    if let Err(e) = sqlx::query("UPDATE scan_results SET scanned_at = CURRENT_TIMESTAMP WHERE ip = ? AND port = ?")
                        .bind(&ip_str).bind(port).execute(&db).await {
                        eprintln!("Health check DB error (scanned_at update): {}", e);
                    }
                }
            }
        }
    });

    let mut stream = futures::stream::iter(iter).buffer_unordered(20);
    while stream.next().await.is_some() {}
}

async fn ensure_geo_db() {
    let dbs = vec![
        ("Country.mmdb", "https://github.com/Loyalsoldier/geoip/releases/latest/download/Country.mmdb"),
        ("GeoLite2-ASN.mmdb", "https://github.com/P3TERX/GeoLite.mmdb/releases/latest/download/GeoLite2-ASN.mmdb"),
    ];

    for (db_path, url) in dbs {
        if !std::path::Path::new(db_path).exists() {
            println!("Database ({}) not found. Downloading...", db_path);
            match reqwest::get(url).await {
                Ok(response) => {
                    if response.status().is_success() {
                        if let Ok(bytes) = response.bytes().await {
                            if std::fs::write(db_path, bytes).is_ok() {
                                println!("Database {} downloaded successfully.", db_path);
                            } else {
                                println!("Failed to write Database {} to disk.", db_path);
                            }
                        } else {
                            println!("Failed to read Database {} response body.", db_path);
                        }
                    } else {
                        println!("Failed to download Database {}, status code: {}", db_path, response.status());
                    }
                }
                Err(e) => {
                    println!("Failed to download Database {}: {}", db_path, e);
                }
            }
        }
    }
}
