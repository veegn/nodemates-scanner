use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use std::{collections::HashMap, sync::Arc, time::Instant};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub config: AppConfig,
    pub http_client: reqwest::Client,
    pub radar_cache: RadarCache,
}

pub type RadarCache = Arc<RwLock<HashMap<u32, RadarCacheEntry>>>;

#[derive(Clone)]
pub struct RadarCacheEntry {
    pub summary: RadarAsnSummary,
    pub expires_at: Instant,
}

#[derive(Clone)]
pub struct AppConfig {
    pub bind_addr: String,
    pub database_url: String,
    pub data_dir: String,
    pub country_db_path: String,
    pub asn_db_path: String,
    pub radar_date_range: String,
    pub radar_cache_ttl_secs: u64,
    pub scan_checkpoint_interval: usize,
}

#[derive(Deserialize)]
pub struct ScanRequest {
    pub target: String,
    pub ports: Option<Vec<u16>>,
    pub timeout: Option<u64>,
}

#[derive(Serialize, Clone)]
pub struct ScanResult {
    pub ip: String,
    pub port: u16,
    pub origin: String,
    pub tls_version: String,
    pub alpn: String,
    pub cert_length: String,
    pub cert_signature: String,
    pub cert_publickey: String,
    pub cert_validity: String,
    pub cert_domain: String,
    pub cert_issuer: String,
    pub geo_code: String,
    pub asn_number: u32,
    pub asn_org: String,
    pub latency: u32,
    pub failure_reason: String,
    pub feasible: bool,
}

#[derive(Deserialize, Clone)]
pub struct ResultsQuery {
    pub history_id: Option<i64>,
    pub geo_code: Option<String>,
    pub domain: Option<String>,
    pub port: Option<u16>,
    pub asn: Option<u32>,
    pub tls_version: Option<String>,
    pub alpn: Option<String>,
    pub latency_min: Option<u32>,
    pub latency_max: Option<u32>,
    pub since: Option<String>,
    pub limit: Option<i64>,
}

#[derive(Serialize)]
pub struct DbScanHistory {
    pub id: i64,
    pub target: String,
    pub total_tasks: i32,
    pub completed_tasks: i32,
    pub status: String,
    pub scanned_at: String,
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
    pub cert_validity: String,
    pub geo_code: String,
    pub cert_type: String,
    pub asn_number: u32,
    pub asn_org: String,
    pub latency: u32,
    pub scanned_at: String,
}

#[derive(Serialize, Clone)]
pub struct RadarDimensionShare {
    pub label: String,
    pub value: f64,
}

#[derive(Serialize, Clone, Default)]
pub struct RadarBgpRiskSummary {
    pub hijack_events: usize,
    pub high_confidence_hijacks: usize,
    pub route_leak_events: usize,
    pub ongoing_route_leaks: usize,
    pub rpki_roa_coverage: Option<f64>,
    pub recent_event: String,
}

#[derive(Serialize, Clone, Default)]
pub struct RadarAttackSummary {
    pub layer7_mitigation: Vec<RadarDimensionShare>,
    pub layer3_protocol: Vec<RadarDimensionShare>,
    pub layer3_vector: Vec<RadarDimensionShare>,
}

#[derive(Serialize, Clone)]
pub struct RadarAsnSummary {
    pub asn: u32,
    pub human: f64,
    pub bot: f64,
    pub date_range: String,
    pub last_updated: String,
    pub radar_url: String,
    pub device_type: Vec<RadarDimensionShare>,
    pub http_protocol: Vec<RadarDimensionShare>,
    pub ip_version: Vec<RadarDimensionShare>,
    pub tls_version: Vec<RadarDimensionShare>,
    pub bgp: RadarBgpRiskSummary,
    pub attacks: RadarAttackSummary,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SystemSettings {
    pub concurrency_limit: u32,
    pub max_cidr_ipv4: u8,
    pub max_cidr_ipv6: u8,
    pub cooldown_days: u32,
    pub allowed_ports: String,
}

#[derive(Serialize)]
pub struct HealthStatus {
    pub status: String,
    pub database: bool,
    pub country_db: bool,
    pub asn_db: bool,
    pub radar_token_configured: bool,
    pub radar_date_range: String,
}
