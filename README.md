# nodemates-scanner

一个先进的、高性能的 Web 端异步 TLS 探测与 Reality 节点验证工具。完全基于 Rust (Axum + Tokio) 重写，提供无与伦比的并发性能、实时的 WebSocket 反馈以及强大的防滥用机制。

An advanced, high-performance web-based utility for asynchronous TLS probing and Reality node verification. Fully rewritten in Rust (Axum + Tokio) to provide unparalleled concurrency, real-time WebSocket feedback, and robust abuse-prevention.

[English Documentation Below](#english-documentation)

## 主要特性 (Key Features)

- ⚡ **纯异步 Rust (Pure Asynchronous Rust)**: 基于 	okio, xum, 和 	okio-rustls 构建，实现极速并发扫描。
- 🎨 **现代化 Web UI (Apple-inspired Web UI)**: 采用玻璃拟物化 (Glassmorphism) 主题，支持暗黑模式和微动画。
- 🌐 **双语支持 (Bilingual Support)**: 纯前端 i18n 支持，自动检测用户语言 (中/英)，并支持手动切换。
- 📡 **多端口扫描 (Multi-Port Scanning)**: 可同时探测多个常见的 SSL 端口 (如 443, 8443, 2053, 2083, 2087, 2096)。
- 🌍 **实时 GeoIP (Real-time GeoIP)**: 集成 MaxMind GeoLite2，瞬间识别服务器地理位置。
- 🛡️ **防滥用与限速 (Anti-abuse & Rate Limiting)**: 内置 SQLite 数据库，对已完成的 CIDR/IP 扫描结果实施 30 天的锁定缓存，防止扫描滥用。
- ⏯️ **断点续扫 (Breakpoint Resumption)**: 自动保存大段 CIDR 扫描进度。如果扫描被中止或断开，可精准从中断处恢复。
- ⏱️ **每小时健康检查 (Hourly Health Checks)**: 内置 Cron 定时任务，每小时自动重新验证已入库的可用节点，及时淘汰失效端点。

## 构建与运行 (Building & Running)

### 环境要求 (Requirements)
- Rust (Cargo) 1.85+
- SQLite (已通过 sqlx 随标准库包含)

### 本地运行 (Run Locally) (推荐)

1. **克隆仓库**:
   `ash
   git clone https://github.com/veegn/nodemates-scanner.git
   cd nodemates-scanner
   `

2. **下载 GeoIP 数据库** (可选，但建议下载以支持地理位置查询):
   将 MaxMind GeoLite2/GeoIP2 Country 数据库放置在项目根目录下，命名为 Country.mmdb。
   项目会在首次运行时尝试自动下载，但您也可以 [在此手动下载](https://github.com/Loyalsoldier/geoip/releases/latest/download/Country.mmdb)。

3. **运行服务端**:
   `ash
   cargo run --release
   `
   
4. **访问 Web UI**:
   在浏览器中打开 http://localhost:3000。

### Docker 支持 (Docker Support)

您可以使用 Docker 轻松部署 
odemates-scanner：

`ash
# 构建镜像
docker build -t nodemates-scanner .

# 运行容器
docker run -d -p 3000:3000 --name nodemates nodemates-scanner
`

## 使用方法与界面 (Usage & Interface)

- **目标输入 (Target Input)**: 支持单个 IP、域名 (如 example.com)，或整个 CIDR IP 段 (如 107.172.103.0/24)。私有/内网 IP 会被自动跳过。
- **端口选择 (Port Selection)**: 点击药丸形状的复选框即可同时扫描多个端口。
- **实时数据流 (Real-Time Pipeline)**: 探测节点时，结果会通过 WebSockets 立即流式传输到 UI 上。“可行 (Feasible)” 节点 (支持 TLS 1.3 + ALPN h2 且域名有效) 将被高亮并置顶显示。
- **数据库历史 (Database History)**: 切换到 **“Database (节点图库)”** 标签页，可以查看、筛选 (按地区/域名) 或删除 SQLite 数据库中之前发现的可用节点。

## 架构 (Architecture)

- **后端 (Backend)**: Rust, Axum, Tokio, tokio-rustls, sqlx (SQLite)。
- **前端 (Frontend)**: 原生 JavaScript (ES6+), CSS3 (Flexbox/Grid, Backdrop-filter), HTML5, WebSocket API。
- **数据库 (Database)**: 
  - scan_results: 存储健康且可行的节点，及其 ALPN、证书颁发机构和地理位置数据。
  - scan_history: 跟踪扫描进度、总任务数和完成时间，以支持精确的断点续扫和 30 天防滥用缓存。

## 免责声明 (Disclaimer)

本工具仅用于教育目的和内部网络审计。请勿扫描您不拥有或未获得明确测试授权的网络。

---

# English Documentation

## Building & Running

### Requirements
- Rust (Cargo) 1.85+
- SQLite (included with standard libraries via sqlx)

### Run Locally (Recommended)

1. **Clone the repository**:
   `ash
   git clone https://github.com/veegn/nodemates-scanner.git
   cd nodemates-scanner
   `

2. **Download GeoIP Database** (Optional but recommended for Geo lookup):
   Place a MaxMind GeoLite2/GeoIP2 Country Database in the project root named Country.mmdb. The project will try to download it automatically on the first run, but you can also [download it manually here](https://github.com/Loyalsoldier/geoip/releases/latest/download/Country.mmdb).

3. **Run the Server**:
   `ash
   cargo run --release
   `
   
4. **Access the UI**:
   Open http://localhost:3000 in your web browser.

### Docker Support

You can easily deploy 
odemates-scanner using Docker:

`ash
# Build the image
docker build -t nodemates-scanner .

# 运行容器
docker run -d -p 3000:3000 --name nodemates nodemates-scanner
`

## Usage & Interface

- **Target Input**: Accepts single IPs, Domains (e.g. example.com), or entire CIDR blocks (e.g. 107.172.103.0/24). Private/Internal IPs are automatically skipped.
- **Port Selection**: Simply toggle the pill-shaped checkboxes to scan multiple ports concurrently.
- **Real-Time Pipeline**: As nodes are probed, results stream immediately into the UI via WebSockets. "Feasible" nodes (TLS 1.3 + ALPN h2 with valid domains) are highlighted and pushed to the top.
- **Database History**: Switch to the **Database** tab to view, filter (by Geo/Domain), or delete previously discovered feasible nodes from the SQLite database.

## Architecture

- **Backend**: Rust, Axum, Tokio, tokio-rustls, sqlx (SQLite).
- **Frontend**: Vanilla JavaScript (ES6+), CSS3 (Flexbox/Grid, Backdrop-filter), HTML5, WebSocket API.
- **Database**: 
  - scan_results: Stores healthy, feasible nodes along with their ALPN, Issuer, and Geo data.
  - scan_history: Tracks scan progress, total tasks, and completion timestamps to enable exact breakpoint resumption and 30-day anti-abuse caching.

## Disclaimer

This tool is designed for educational purposes and internal network auditing. Please do not scan networks you do not own or have explicit permission to test.
