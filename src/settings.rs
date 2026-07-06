use crate::models::SystemSettings;

pub async fn load_settings(db: &sqlx::SqlitePool) -> SystemSettings {
    let rows: Vec<(String, String)> = sqlx::query_as("SELECT key, value FROM system_settings")
        .fetch_all(db)
        .await
        .unwrap_or_default();

    let mut settings = default_settings();

    for (key, value) in rows {
        match key.as_str() {
            "concurrency_limit" => {
                settings.concurrency_limit = value.parse().unwrap_or(settings.concurrency_limit)
            }
            "max_cidr_ipv4" => {
                settings.max_cidr_ipv4 = value.parse().unwrap_or(settings.max_cidr_ipv4)
            }
            "max_cidr_ipv6" => {
                settings.max_cidr_ipv6 = value.parse().unwrap_or(settings.max_cidr_ipv6)
            }
            "cooldown_days" => {
                settings.cooldown_days = value.parse().unwrap_or(settings.cooldown_days)
            }
            "allowed_ports" => settings.allowed_ports = value,
            _ => {}
        }
    }

    validate_settings(settings).unwrap_or_else(|_| default_settings())
}

pub fn default_settings() -> SystemSettings {
    SystemSettings {
        concurrency_limit: 50,
        max_cidr_ipv4: 24,
        max_cidr_ipv6: 120,
        cooldown_days: 30,
        allowed_ports: "443,8443,2053,2083,2087,2096".into(),
    }
}

pub fn validate_settings(mut settings: SystemSettings) -> Result<SystemSettings, String> {
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
        .map(|port| port.to_string())
        .collect::<Vec<_>>()
        .join(",");

    Ok(settings)
}

pub fn parse_ports(raw: &str) -> Result<Vec<u16>, String> {
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

pub fn normalize_requested_ports(
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

pub fn scan_history_key(target: &str, ports: &[u16], timeout_secs: u64) -> String {
    let ports = ports
        .iter()
        .map(|port| port.to_string())
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "target={};ports={};timeout={}",
        target.trim(),
        ports,
        timeout_secs
    )
}
