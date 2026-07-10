mod db;
mod handlers;
mod models;
mod scan_events;
mod scanner;
mod settings;
mod targets;

use axum::{Router, routing::get};
use models::{AppConfig, AppState};
use sqlx::sqlite::SqlitePoolOptions;
use std::{collections::HashMap, net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let config = AppConfig::from_env();
    ensure_geo_db(&config).await;
    println!("Initializing node scanner...");

    let db = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("failed to connect SQLite database");

    db::init_db(&db)
        .await
        .expect("failed to initialize database");

    let state = AppState {
        db: db.clone(),
        config: config.clone(),
        http_client: reqwest::Client::new(),
        radar_cache: Arc::new(RwLock::new(HashMap::new())),
    };

    let app = Router::new()
        .nest_service("/", ServeDir::new("static"))
        .route("/healthz", get(handlers::health_handler))
        .route("/readyz", get(handlers::health_handler))
        .route("/scan", get(handlers::ws_scan_handler))
        .route("/results", get(handlers::get_results_handler))
        .route(
            "/results/export.csv",
            get(handlers::export_results_csv_handler),
        )
        .route(
            "/radar/asn/:asn/bot-class",
            get(handlers::get_asn_bot_summary_handler),
        )
        .route(
            "/results/:ip",
            axum::routing::delete(handlers::delete_result_handler),
        )
        .route(
            "/settings",
            get(handlers::get_settings_handler).put(handlers::put_settings_handler),
        )
        .route("/history/tasks", get(handlers::get_history_tasks_handler))
        .route(
            "/history/tasks/:id",
            axum::routing::delete(handlers::delete_history_task_handler),
        )
        .with_state(state);

    let addr: SocketAddr = config
        .bind_addr
        .parse()
        .expect("BIND_ADDR must be a valid socket address");
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind listener");
    axum::serve(listener, app)
        .await
        .expect("server exited unexpectedly");
}

impl AppConfig {
    fn from_env() -> Self {
        let data_dir = std::env::var("DATA_DIR").unwrap_or_else(|_| ".".to_string());
        let country_db_path = std::env::var("COUNTRY_DB_PATH")
            .unwrap_or_else(|_| data_path(&data_dir, "Country.mmdb"));
        let asn_db_path = std::env::var("ASN_DB_PATH")
            .unwrap_or_else(|_| data_path(&data_dir, "GeoLite2-ASN.mmdb"));
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| format!("sqlite:{}?mode=rwc", data_path(&data_dir, "results.db")));

        Self {
            bind_addr: std::env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".to_string()),
            database_url,
            data_dir,
            country_db_path,
            asn_db_path,
            radar_date_range: std::env::var("RADAR_DATE_RANGE")
                .unwrap_or_else(|_| "7d".to_string()),
            radar_cache_ttl_secs: env_u64("RADAR_CACHE_TTL_SECS", 21_600),
            scan_checkpoint_interval: env_usize("SCAN_CHECKPOINT_INTERVAL", 100).max(1),
        }
    }
}

fn data_path(data_dir: &str, filename: &str) -> String {
    let path = PathBuf::from(data_dir).join(filename);
    path.to_string_lossy().replace('\\', "/")
}

fn env_u64(key: &str, default: u64) -> u64 {
    std::env::var(key)
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(default)
}

fn env_usize(key: &str, default: usize) -> usize {
    std::env::var(key)
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(default)
}

async fn ensure_geo_db(config: &AppConfig) {
    if let Err(e) = std::fs::create_dir_all(&config.data_dir) {
        println!("Failed to create data directory {}: {}", config.data_dir, e);
    }

    let dbs = vec![
        (
            &config.country_db_path,
            "https://github.com/Loyalsoldier/geoip/releases/latest/download/Country.mmdb",
        ),
        (
            &config.asn_db_path,
            "https://github.com/P3TERX/GeoLite.mmdb/releases/latest/download/GeoLite2-ASN.mmdb",
        ),
    ];

    for (db_path, url) in dbs {
        if !std::path::Path::new(&db_path).exists() {
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
                        println!(
                            "Failed to download Database {}, status code: {}",
                            db_path,
                            response.status()
                        );
                    }
                }
                Err(e) => {
                    println!("Failed to download Database {}: {}", db_path, e);
                }
            }
        }
    }
}
