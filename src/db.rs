use sqlx::{Row, SqlitePool};

pub async fn init_db(db: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query("PRAGMA journal_mode=WAL;").execute(db).await?;
    sqlx::query("PRAGMA foreign_keys=ON;").execute(db).await?;

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
            asn_number INTEGER DEFAULT 0,
            asn_org TEXT DEFAULT '',
            latency INTEGER DEFAULT 0,
            cert_validity TEXT DEFAULT '',
            failure_reason TEXT DEFAULT '',
            scanned_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
    )
    .execute(db)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS scan_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            target TEXT NOT NULL,
            total_tasks INTEGER DEFAULT 0,
            completed_tasks INTEGER DEFAULT 0,
            status TEXT DEFAULT 'COMPLETED',
            scanned_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
    )
    .execute(db)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS scan_task_results (
            history_id INTEGER NOT NULL,
            result_id INTEGER NOT NULL,
            PRIMARY KEY (history_id, result_id)
        )",
    )
    .execute(db)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS system_settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        )",
    )
    .execute(db)
    .await?;

    add_column_if_missing(db, "scan_results", "port", "INTEGER DEFAULT 443").await?;
    add_column_if_missing(db, "scan_results", "asn_org", "TEXT DEFAULT ''").await?;
    add_column_if_missing(db, "scan_results", "asn_number", "INTEGER DEFAULT 0").await?;
    add_column_if_missing(db, "scan_results", "latency", "INTEGER DEFAULT 0").await?;
    add_column_if_missing(db, "scan_results", "cert_validity", "TEXT DEFAULT ''").await?;
    add_column_if_missing(db, "scan_results", "failure_reason", "TEXT DEFAULT ''").await?;
    add_column_if_missing(db, "scan_history", "total_tasks", "INTEGER DEFAULT 0").await?;
    add_column_if_missing(db, "scan_history", "completed_tasks", "INTEGER DEFAULT 0").await?;
    add_column_if_missing(db, "scan_history", "status", "TEXT DEFAULT 'COMPLETED'").await?;

    let default_settings = [
        ("concurrency_limit", "50"),
        ("max_cidr_ipv4", "24"),
        ("max_cidr_ipv6", "120"),
        ("cooldown_days", "30"),
        ("allowed_ports", "443,8443,2053,2083,2087,2096"),
    ];

    for (key, value) in default_settings {
        sqlx::query("INSERT OR IGNORE INTO system_settings (key, value) VALUES (?, ?)")
            .bind(key)
            .bind(value)
            .execute(db)
            .await?;
    }

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_ip ON scan_results(ip)")
        .execute(db)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_domain ON scan_results(cert_domain)")
        .execute(db)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_geo ON scan_results(geo_code)")
        .execute(db)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_asn ON scan_results(asn_number)")
        .execute(db)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_latency ON scan_results(latency)")
        .execute(db)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_feasible ON scan_results(feasible)")
        .execute(db)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_scanned_at ON scan_results(scanned_at)")
        .execute(db)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_target ON scan_history(target)")
        .execute(db)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_history_status ON scan_history(status)")
        .execute(db)
        .await?;

    sqlx::query(
        "DELETE FROM scan_results
         WHERE id NOT IN (SELECT MAX(id) FROM scan_results GROUP BY ip, port)",
    )
    .execute(db)
    .await?;
    sqlx::query("CREATE UNIQUE INDEX IF NOT EXISTS idx_ip_port ON scan_results(ip, port)")
        .execute(db)
        .await?;

    Ok(())
}

async fn add_column_if_missing(
    db: &SqlitePool,
    table: &str,
    column: &str,
    definition: &str,
) -> Result<(), sqlx::Error> {
    if column_exists(db, table, column).await? {
        return Ok(());
    }

    let sql = format!("ALTER TABLE {table} ADD COLUMN {column} {definition}");
    sqlx::query(&sql).execute(db).await?;
    Ok(())
}

async fn column_exists(db: &SqlitePool, table: &str, column: &str) -> Result<bool, sqlx::Error> {
    let sql = format!("PRAGMA table_info({table})");
    let rows = sqlx::query(&sql).fetch_all(db).await?;

    Ok(rows.iter().any(|row| {
        let name: String = row.get("name");
        name == column
    }))
}
