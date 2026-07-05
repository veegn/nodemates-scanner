# nodemates-scanner

An advanced, high-performance web-based utility for asynchronous TLS probing and Reality node verification. Fully rewritten in Rust (Axum + Tokio) to provide unparalleled concurrency, real-time WebSocket feedback, and robust abuse-prevention.

## Key Features

- ⚡ **Pure Asynchronous Rust**: Built on `tokio`, `axum`, and `tokio-rustls` for blazingly fast concurrent scanning.
- 🎨 **Apple-inspired Web UI**: Modern, glassmorphism-themed interface with dark mode and micro-animations.
- 🌐 **Bilingual Support**: Pure frontend i18n supporting both English and Chinese (zh/en), automatically detecting user language with a manual toggle.
- 📡 **Multi-Port Scanning**: Concurrently sniff across multiple common SSL ports (443, 8443, 2053, 2083, 2087, 2096).
- 🌍 **Real-time GeoIP**: MaxMind GeoLite2 integration to identify server locations instantly.
- 🛡️ **Anti-abuse & Rate Limiting**: 30-day target locking on completed CIDRs/IPs to prevent scanning abuse, powered by an internal SQLite database.
- ⏯️ **Breakpoint Resumption**: Automatically saves progress on large CIDR scans. If a scan is stopped or interrupted, it can resume precisely where it left off.
- ⏱️ **Hourly Health Checks**: A built-in cron job runs hourly to re-verify stored feasible nodes, automatically updating dead endpoints.

## Building & Running

### Requirements
- Rust (Cargo) 1.70+
- SQLite (included with standard libraries via `sqlx`)

### Run Locally (Recommended)

1. **Clone the repository**:
   ```bash
   git clone https://github.com/veegn/nodemates-scanner.git
   cd nodemates-scanner
   ```

2. **Download GeoIP Database** (Optional but recommended for Geo lookup):
   Place a MaxMind GeoLite2/GeoIP2 Country Database in the project root named `Country.mmdb`.
   [Download here](https://github.com/Loyalsoldier/geoip/releases/latest/download/Country.mmdb)

3. **Run the Server**:
   ```bash
   cargo run --release
   ```
   
4. **Access the UI**:
   Open `http://localhost:3000` in your web browser.

### Docker Support

You can easily deploy `nodemates-scanner` using Docker:

```bash
# Build the image
docker build -t nodemates-scanner .

# Run the container
docker run -d -p 3000:3000 --name nodemates nodemates-scanner
```

## Usage & Interface

- **Target Input**: Accepts single IPs, Domains (e.g. `example.com`), or entire CIDR blocks (e.g. `107.172.103.0/24`). Private/Internal IPs are automatically skipped.
- **Port Selection**: Simply toggle the pill-shaped checkboxes to scan multiple ports concurrently.
- **Real-Time Pipeline**: As nodes are probed, results stream immediately into the UI via WebSockets. "Feasible" nodes (TLS 1.3 + ALPN h2 with valid domains) are highlighted and pushed to the top.
- **Database History**: Switch to the **Database (节点图库)** tab to view, filter (by Geo/Domain), or delete previously discovered feasible nodes from the SQLite database.

## Architecture

- **Backend**: Rust, Axum, Tokio, tokio-rustls, sqlx (SQLite).
- **Frontend**: Vanilla JavaScript (ES6+), CSS3 (Flexbox/Grid, Backdrop-filter), HTML5, WebSocket API.
- **Database**: 
  - `scan_results`: Stores healthy, feasible nodes along with their ALPN, Issuer, and Geo data.
  - `scan_history`: Tracks scan progress, total tasks, and completion timestamps to enable exact breakpoint resumption and 30-day anti-abuse caching.

## Disclaimer

This tool is designed for educational purposes and internal network auditing. Please do not scan networks you do not own or have explicit permission to test.
