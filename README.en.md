# nodemates-scanner

[中文文档](README.md)

An advanced, high-performance web-based utility for asynchronous TLS probing and Reality node verification. Fully rewritten in Rust (Axum + Tokio) to provide high concurrency, real-time WebSocket feedback, and abuse-prevention controls.

## Key Features

- ⚡ **Pure asynchronous Rust**: Built with Tokio, Axum, and tokio-rustls for high-concurrency scanning.
- 🎨 **Modern Web UI**: Glassmorphism-inspired interface with dark mode and subtle animations.
- 🌐 **Bilingual UI**: Frontend i18n support with automatic Chinese/English detection and manual switching.
- 📡 **Multi-port scanning**: Probe multiple common SSL ports at the same time, including 443, 8443, 2053, 2083, 2087, and 2096.
- 🌍 **Real-time GeoIP**: Integrates MaxMind GeoLite2 for fast geographic lookup.
- ☁️ **Cloudflare Radar ASN insights**: Hover an IP to view ASN, Human/Bot share, HTTP protocol, TLS/IP/device distribution, BGP anomalies, and attack profile.
- 📤 **Filtering and export**: Filter historical results by domain, region, ASN, port, and latency, then export CSV.
- 🛡️ **Abuse prevention and rate limiting**: Uses SQLite-backed scan history with a 30-day completed-scan cooldown for CIDR/IP targets.
- ⏯️ **Breakpoint resumption**: Saves progress for larger CIDR scans so interrupted scans can resume from the last completed task.

## Building And Running

### Requirements

- Rust (Cargo) 1.85+
- SQLite (via sqlx)

### Run Locally

1. **Clone the repository**:

   ```bash
   git clone https://github.com/veegn/nodemates-scanner.git
   cd nodemates-scanner
   ```

2. **Download the GeoIP database** (optional, but recommended):

   Place a MaxMind GeoLite2/GeoIP2 Country database in the project root named `Country.mmdb`. The project will try to download it automatically on first run, but you can also [download it manually](https://github.com/Loyalsoldier/geoip/releases/latest/download/Country.mmdb).

3. **Configure a Cloudflare Radar API token** (optional, required for Human/Bot hover data):

   ```bash
   export CLOUDFLARE_API_TOKEN=your_cloudflare_api_token
   ```

   Scanning and ASN display still work without the token, but the Radar insights panel will show that the token is not configured.

4. **Run the server**:

   ```bash
   cargo run --release
   ```

5. **Open the Web UI**:

   Visit `http://localhost:3000` in your browser.

### Optional Environment Variables

| Variable | Default | Description |
| --- | --- | --- |
| `BIND_ADDR` | `0.0.0.0:3000` | Server listen address |
| `DATA_DIR` | `.` | Database and GeoIP data directory |
| `DATABASE_URL` | `sqlite:${DATA_DIR}/results.db?mode=rwc` | SQLite connection URL |
| `COUNTRY_DB_PATH` | `${DATA_DIR}/Country.mmdb` | Country GeoIP database path |
| `ASN_DB_PATH` | `${DATA_DIR}/GeoLite2-ASN.mmdb` | ASN GeoIP database path |
| `CLOUDFLARE_API_TOKEN` | empty | Cloudflare Radar API token |
| `RADAR_DATE_RANGE` | `7d` | Radar ASN insights query window |
| `RADAR_CACHE_TTL_SECS` | `21600` | Radar ASN cache TTL |
| `SCAN_CHECKPOINT_INTERVAL` | `100` | Scan progress checkpoint interval |

Health check endpoints: `/healthz` and `/readyz`.

### Docker Support

You can deploy nodemates-scanner with Docker:

```bash
# Build the image
docker build -t nodemates-scanner .

# Run the container with persistent SQLite and GeoIP data
docker run -d -p 3000:3000 \
  -v nodemates-data:/app/data \
  -e CLOUDFLARE_API_TOKEN=your_cloudflare_api_token \
  --name nodemates nodemates-scanner
```

## Usage And Interface

- **Target input**: Accepts single IPs, domains such as `example.com`, or CIDR blocks such as `107.172.103.0/24`. Private and internal IPs are skipped automatically.
- **Port selection**: Toggle the pill-style checkboxes to scan multiple ports concurrently.
- **Real-time pipeline**: Probe results stream to the UI via WebSocket. Feasible nodes, defined as TLS 1.3 + ALPN h2 with certificate metadata, are highlighted and placed at the top.
- **Database history**: Use the Database tab to view, filter by region/domain, or delete previously discovered feasible nodes from SQLite.
- **CSV export**: The export button on the history page downloads results using the current filters.
- **ASN hover insights**: The IP hover panel queries Cloudflare Radar for HTTP posture, BGP hijack/route leak/RPKI summaries, and L7/L3 attack distribution.

## Architecture

- **Backend**: Rust, Axum, Tokio, tokio-rustls, sqlx (SQLite).
- **Frontend**: Vanilla JavaScript (ES6+), CSS3 (Flexbox/Grid, Backdrop-filter), HTML5, WebSocket API.
- **Database**:
  - `scan_results`: Stores healthy, feasible nodes along with ALPN, issuer, and GeoIP data.
  - `scan_history`: Tracks scan progress, total tasks, and completion timestamps for exact resumption and 30-day cooldown caching.

## Disclaimer

This tool is designed for educational use and internal network auditing. Do not scan networks that you do not own or do not have explicit permission to test.
