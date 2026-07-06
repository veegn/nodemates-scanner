use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}

#[derive(Deserialize)]
pub struct ScanRequest {
    pub target: String,
    pub ports: Option<Vec<u16>>,
    pub timeout: Option<u64>,
}

#[derive(Serialize)]
pub struct ScanResult {
    pub ip: String,
    pub port: u16,
    pub origin: String,
    pub tls_version: String,
    pub alpn: String,
    pub cert_length: String,
    pub cert_signature: String,
    pub cert_publickey: String,
    pub cert_domain: String,
    pub cert_issuer: String,
    pub geo_code: String,
    pub feasible: bool,
}

#[derive(Deserialize)]
pub struct ResultsQuery {
    pub geo_code: Option<String>,
    pub domain: Option<String>,
    pub limit: Option<i64>,
}

#[derive(Deserialize)]
pub struct DeleteResultQuery {
    pub port: Option<u16>,
}

#[derive(Serialize)]
pub struct DbScanResult {
    pub ip: String,
    pub port: u16,
    pub origin: String,
    pub tls_version: String,
    pub alpn: String,
    pub cert_domain: String,
    pub cert_issuer: String,
    pub geo_code: String,
    pub scanned_at: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SystemSettings {
    pub concurrency_limit: u32,
    pub max_cidr_ipv4: u8,
    pub max_cidr_ipv6: u8,
    pub cooldown_days: u32,
    pub allowed_ports: String,
}
