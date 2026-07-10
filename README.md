# nodemates-scanner

[English Documentation](README.en.md)

一个先进的、高性能的 Web 端异步 TLS 探测与 Reality 节点验证工具。完全基于 Rust (Axum + Tokio) 重写，提供无与伦比的并发性能、实时的 WebSocket 反馈以及强大的防滥用机制。

## 主要特性

- ⚡ **纯异步 Rust**: 基于 Tokio, Axum, 和 tokio-rustls 构建，实现极速并发扫描。
- 🎨 **现代化 Web UI**: 采用玻璃拟物化 (Glassmorphism) 主题，支持暗黑模式和微动画。
- 🌐 **双语支持**: 纯前端 i18n 支持，自动检测用户语言 (中/英)，并支持手动切换。
- 📡 **多端口扫描**: 可同时探测多个常见的 SSL 端口 (如 443, 8443, 2053, 2083, 2087, 2096)。
- 🌍 **实时 GeoIP**: 集成 MaxMind GeoLite2，瞬间识别服务器地理位置。
- ☁️ **Cloudflare Radar ASN 画像**: 鼠标悬浮在 IP 上时展示 ASN、Human/Bot、HTTP 协议、TLS/IP/设备分布、BGP 异常与攻击画像。
- 📤 **过滤与导出**: 历史结果支持按域名、地区、ASN、端口和延迟过滤，并可导出 CSV。
- 🛡️ **防滥用与限速**: 内置 SQLite 数据库，对已完成的 CIDR/IP 扫描结果实施 30 天的锁定缓存，防止扫描滥用。
- ⏯️ **断点续扫**: 自动保存大段 CIDR 扫描进度。如果扫描被中止或断开，可精准从中断处恢复。

## 构建与运行

### 环境要求

- Rust (Cargo) 1.85+
- SQLite (已通过 sqlx 随标准库包含)

### 本地运行

1. **克隆仓库**:

   ```bash
   git clone https://github.com/veegn/nodemates-scanner.git
   cd nodemates-scanner
   ```

2. **下载 GeoIP 数据库** (可选，但建议下载以支持地理位置查询):

   将 MaxMind GeoLite2/GeoIP2 Country 数据库放置在项目根目录下，命名为 `Country.mmdb`。项目会在首次运行时尝试自动下载，但您也可以 [在此手动下载](https://github.com/Loyalsoldier/geoip/releases/latest/download/Country.mmdb)。

3. **配置 Cloudflare Radar API Token** (可选，用于悬浮窗 Human/Bot 数据):

   ```bash
   export CLOUDFLARE_API_TOKEN=your_cloudflare_api_token
   ```

   未配置时扫描和 ASN 展示仍可使用，但悬浮窗中的 Radar 画像会显示为未配置。

4. **运行服务端**:

   ```bash
   cargo run --release
   ```

5. **访问 Web UI**:

   在浏览器中打开 `http://localhost:3000`。

### 可选环境变量

| 变量 | 默认值 | 说明 |
| --- | --- | --- |
| `BIND_ADDR` | `0.0.0.0:3000` | 服务监听地址 |
| `DATA_DIR` | `.` | 数据库与 GeoIP 文件目录 |
| `DATABASE_URL` | `sqlite:${DATA_DIR}/results.db?mode=rwc` | SQLite 连接地址 |
| `COUNTRY_DB_PATH` | `${DATA_DIR}/Country.mmdb` | Country GeoIP 数据库路径 |
| `ASN_DB_PATH` | `${DATA_DIR}/GeoLite2-ASN.mmdb` | ASN GeoIP 数据库路径 |
| `CLOUDFLARE_API_TOKEN` | 空 | Cloudflare Radar API Token |
| `RADAR_DATE_RANGE` | `7d` | Radar ASN 画像查询窗口 |
| `RADAR_CACHE_TTL_SECS` | `21600` | Radar ASN 缓存时间 |
| `SCAN_CHECKPOINT_INTERVAL` | `100` | 扫描进度写库间隔 |

健康检查端点：`/healthz` 与 `/readyz`。

### Docker 支持

您可以使用 Docker 轻松部署 nodemates-scanner：

```bash
# 构建镜像
docker build -t nodemates-scanner .

# 运行容器，持久化 SQLite 与 GeoIP 数据
docker run -d -p 3000:3000 \
  -v nodemates-data:/app/data \
  -e CLOUDFLARE_API_TOKEN=your_cloudflare_api_token \
  --name nodemates nodemates-scanner
```

## 使用方法与界面

- **目标输入**: 支持单个 IP、域名 (如 `example.com`)，或整个 CIDR IP 段 (如 `107.172.103.0/24`)。私有/内网 IP 会被自动跳过。
- **端口选择**: 点击药丸形状的复选框即可同时扫描多个端口。
- **实时数据流**: 探测节点时，结果会通过 WebSocket 立即流式传输到 UI 上。“可行 (Feasible)” 节点 (支持 TLS 1.3 + ALPN h2 且域名有效) 将被高亮并置顶显示。
- **数据库历史**: 切换到 “Database (节点图库)” 标签页，可以查看、筛选 (按地区/域名) 或删除 SQLite 数据库中之前发现的可用节点。
- **CSV 导出**: 历史页顶部的导出按钮会按当前筛选条件导出结果。
- **ASN 悬浮画像**: IP 悬浮窗会读取 Cloudflare Radar，展示 ASN 的 HTTP 画像、BGP hijack/route leak/RPKI 摘要，以及 L7/L3 攻击分布。

## 架构

- **后端**: Rust, Axum, Tokio, tokio-rustls, sqlx (SQLite)。
- **前端**: 原生 JavaScript (ES6+), CSS3 (Flexbox/Grid, Backdrop-filter), HTML5, WebSocket API。
- **数据库**:
  - `scan_results`: 存储健康且可行的节点，及其 ALPN、证书颁发机构和地理位置数据。
  - `scan_history`: 跟踪扫描进度、总任务数和完成时间，以支持精确的断点续扫和 30 天防滥用缓存。

## 免责声明

本工具仅用于教育目的和内部网络审计。请勿扫描您不拥有或未获得明确测试授权的网络。
