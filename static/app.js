const translations = {
    en: {
        pageTitle: "nodemates-scanner - Advanced TLS Node Probing",
        navDocs: "Documentation",
        title: "nodemates-scanner",
        subtitle: "High-performance asynchronous TLS probing, Reality node verification, and GeoIP discovery platform.",
        feat1: "⚡ Asynchronous TLS Probing",
        feat2: "🛡️ Anti-abuse Protection",
        feat3: "🌍 Real-time GeoIP",
        feat4: "⏱️ Hourly Health Checks",
        tabScanner: "Scanner",
        tabHistory: "Database",
        tabSettings: "Settings",
        targetLabel: "Target (IP, CIDR, or Domain)",
        targetPlaceholder: "e.g. 1.1.1.1, 107.172.103.0/24, or example.com",
        portsLabel: "Common SSL Ports",
        timeoutLabel: "Timeout (s)",
        btnStart: "Initiate Scan",
        btnStop: "Stop Scanning",
        btnSaveSettings: "Save Settings",
        statScanned: "Scanned",
        statFeasible: "Feasible",
        thStatus: "Status",
        thIP: "IP",
        thPort: "Port",
        thDomain: "Domain",
        thALPN: "ALPN",
        thIssuer: "Issuer",
        thGeo: "Geo",
        thScannedAt: "Scanned At",
        thAction: "Action",
        histGeoLabel: "Filter by Geo (e.g. US)",
        histGeoPlaceholder: "US",
        histDomainLabel: "Filter by Domain",
        histDomainPlaceholder: "example.com",
        btnFetch: "Fetch",
        btnDelete: "Delete",
        setConcurrency: "Concurrency Limit",
        setCooldown: "Cooldown (Days)",
        setIpv4: "IPv4 Max CIDR",
        setIpv6: "IPv6 Max CIDR",
        setPorts: "Allowed Ports",
        alertNoPort: "Please select at least one port.",
        alertDeleteConfirm: "Delete result for {ip}?",
        badgeFeasible: "Feasible",
        badgeFailed: "Failed"
    },
    zh: {
        pageTitle: "nodemates-scanner - 高级 TLS 节点嗅探",
        navDocs: "文档",
        title: "nodemates-scanner",
        subtitle: "高性能异步 TLS 嗅探、Reality 节点验证与自动化 GeoIP 探测平台。",
        feat1: "⚡ 纯异步 TLS 嗅探",
        feat2: "🛡️ 防滥用与拦截机制",
        feat3: "🌍 实时 GeoIP 映射",
        feat4: "⏱️ 小时级存活检查",
        tabScanner: "雷达扫描",
        tabHistory: "节点图库",
        tabSettings: "系统设置",
        targetLabel: "目标 (IP, CIDR 或域名)",
        targetPlaceholder: "例如 1.1.1.1, 107.172.103.0/24, 或 example.com",
        portsLabel: "常见 SSL 端口",
        timeoutLabel: "超时时间 (秒)",
        btnStart: "开始扫描",
        btnStop: "停止扫描",
        btnSaveSettings: "保存设置",
        statScanned: "已扫描数",
        statFeasible: "健康节点数",
        thStatus: "状态",
        thIP: "IP",
        thPort: "端口",
        thDomain: "域名",
        thALPN: "ALPN",
        thIssuer: "颁发者",
        thGeo: "地理位置",
        thScannedAt: "扫描时间",
        thAction: "操作",
        histGeoLabel: "按地理代码过滤 (如 US)",
        histGeoPlaceholder: "例: US",
        histDomainLabel: "按域名筛选",
        histDomainPlaceholder: "例: example.com",
        btnFetch: "拉取记录",
        btnDelete: "删除",
        setConcurrency: "并发线程数限制",
        setCooldown: "扫描记录缓存期 (天)",
        setIpv4: "IPv4 最大 CIDR (如 24 代表 /24)",
        setIpv6: "IPv6 最大 CIDR (如 120 代表 /120)",
        setPorts: "允许扫描的端口 (逗号分隔)",
        alertNoPort: "请至少选择一个扫描端口。",
        alertDeleteConfirm: "确定要删除 {ip} 的记录吗？",
        badgeFeasible: "存活",
        badgeFailed: "离线"
    }
};

let currentLang = localStorage.getItem('lang') || (navigator.language.startsWith('zh') ? 'zh' : 'en');
let t = translations[currentLang];

function applyLanguage(lang) {
    currentLang = lang;
    localStorage.setItem('lang', lang);
    t = translations[lang];

    document.querySelectorAll('[data-i18n]').forEach(el => {
        const key = el.getAttribute('data-i18n');
        if (t[key]) {
            if (el.tagName === 'INPUT') {
                el.placeholder = t[key];
            } else if (el.tagName === 'TITLE') {
                document.title = t[key];
            } else {
                el.textContent = t[key];
            }
        }
    });

    const startBtnText = document.querySelector('#submit-btn .btn-text');
    if (submitBtn && startBtnText) {
        if (submitBtn.classList.contains('loading')) {
            startBtnText.textContent = t.btnStop;
        } else {
            startBtnText.textContent = t.btnStart;
        }
    }

    if (historySection && historySection.style.display === 'block') {
        fetchHistory();
    }
}

const form = document.getElementById('scan-form');
const submitBtn = document.getElementById('submit-btn');
const resultsBody = document.getElementById('results-body');
const statScanned = document.getElementById('stat-scanned');
const statFeasible = document.getElementById('stat-feasible');

const tabScan = document.getElementById('tab-scan');
const tabHistory = document.getElementById('tab-history');
const tabSettings = document.getElementById('tab-settings');
const scanSection = document.getElementById('scan-section');
const historySection = document.getElementById('history-section');
const settingsSection = document.getElementById('settings-section');
const fetchHistoryBtn = document.getElementById('fetch-history-btn');
const historyBody = document.getElementById('history-body');
const langToggle = document.getElementById('lang-toggle');

langToggle.addEventListener('click', (e) => {
    e.preventDefault();
    applyLanguage(currentLang === 'en' ? 'zh' : 'en');
});

// Init lang
applyLanguage(currentLang);

// Tabs logic
tabScan.addEventListener('click', () => {
    tabScan.classList.add('active');
    tabHistory.classList.remove('active');
    tabSettings.classList.remove('active');
    scanSection.style.display = 'block';
    historySection.style.display = 'none';
    settingsSection.style.display = 'none';
});

tabHistory.addEventListener('click', () => {
    tabHistory.classList.add('active');
    tabScan.classList.remove('active');
    tabSettings.classList.remove('active');
    scanSection.style.display = 'none';
    historySection.style.display = 'block';
    settingsSection.style.display = 'none';
    fetchHistory();
});

tabSettings.addEventListener('click', () => {
    tabSettings.classList.add('active');
    tabScan.classList.remove('active');
    tabHistory.classList.remove('active');
    scanSection.style.display = 'none';
    historySection.style.display = 'none';
    settingsSection.style.display = 'block';
    fetchSettings();
});

async function fetchHistory() {
    const geo = document.getElementById('hist-geo').value.trim();
    const domain = document.getElementById('hist-domain').value.trim();
    
    let url = '/results?';
    if (geo) url += `geo_code=${encodeURIComponent(geo)}&`;
    if (domain) url += `domain=${encodeURIComponent(domain)}&`;

    try {
        const res = await fetch(url);
        if (!res.ok) throw new Error('Failed to fetch history');
        const data = await res.json();
        
        historyBody.innerHTML = '';
        for (const row of data) {
            const tr = document.createElement('tr');
            tr.innerHTML = `
                <td>${row.ip}</td>
                <td>${row.port}</td>
                <td>${row.cert_domain || '-'}</td>
                <td style="max-width: 200px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;" title="${row.cert_issuer}">${row.cert_issuer || '-'}</td>
                <td>${row.geo_code}</td>
                <td style="font-size: 0.85em; color: var(--text-secondary);">${row.scanned_at}</td>
                <td>
                    <button class="delete-btn" data-ip="${row.ip}">${t.btnDelete}</button>
                </td>
            `;
            historyBody.appendChild(tr);
        }

        document.querySelectorAll('.delete-btn').forEach(btn => {
            btn.addEventListener('click', async (e) => {
                const ip = e.target.getAttribute('data-ip');
                if (confirm(t.alertDeleteConfirm.replace('{ip}', ip))) {
                    try {
                        const delRes = await fetch(`/results/${encodeURIComponent(ip)}`, { method: 'DELETE' });
                        if (!delRes.ok) throw new Error('Failed to delete');
                        e.target.closest('tr').remove();
                    } catch (err) {
                        alert(err.message);
                    }
                }
            });
        });

    } catch (e) {
        alert(e.message);
    }
}

fetchHistoryBtn.addEventListener('click', fetchHistory);

async function fetchSettings() {
    try {
        const res = await fetch('/settings');
        if (!res.ok) throw new Error('Failed to fetch settings');
        const data = await res.json();
        
        document.getElementById('set-concurrency').value = data.concurrency_limit;
        document.getElementById('set-ipv4').value = data.max_cidr_ipv4;
        document.getElementById('set-ipv6').value = data.max_cidr_ipv6;
        document.getElementById('set-cooldown').value = data.cooldown_days;
        document.getElementById('set-ports').value = data.allowed_ports;
    } catch (e) {
        alert(e.message);
    }
}

document.getElementById('settings-form').addEventListener('submit', async (e) => {
    e.preventDefault();
    const btn = document.getElementById('save-settings-btn');
    btn.classList.add('loading');
    
    const settings = {
        concurrency_limit: parseInt(document.getElementById('set-concurrency').value, 10),
        max_cidr_ipv4: parseInt(document.getElementById('set-ipv4').value, 10),
        max_cidr_ipv6: parseInt(document.getElementById('set-ipv6').value, 10),
        cooldown_days: parseInt(document.getElementById('set-cooldown').value, 10),
        allowed_ports: document.getElementById('set-ports').value.trim()
    };
    
    try {
        const res = await fetch('/settings', {
            method: 'PUT',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(settings)
        });
        if (!res.ok) throw new Error('Failed to save settings');
        alert(lang === 'zh' ? '设置已保存' : 'Settings saved successfully');
    } catch (err) {
        alert(err.message);
    } finally {
        btn.classList.remove('loading');
    }
});

let ws = null;
let scannedCount = 0;
let feasibleCount = 0;
let totalTasks = 0;

form.addEventListener('submit', async (e) => {
    e.preventDefault();
    
    if (ws && ws.readyState === WebSocket.OPEN) {
        ws.send(JSON.stringify({ action: 'stop' }));
        ws.close();
        ws = null;
        resetButton();
        return;
    }

    resultsBody.innerHTML = '';
    scannedCount = 0;
    feasibleCount = 0;
    totalTasks = 0;
    updateStats();
    
    const target = document.getElementById('target').value;
    const portCheckboxes = document.querySelectorAll('input[name="ports"]:checked');
    const ports = Array.from(portCheckboxes).map(cb => parseInt(cb.value, 10));

    if (ports.length === 0) {
        alert(t.alertNoPort);
        resetButton();
        return;
    }

    const timeout = parseInt(document.getElementById('timeout').value, 10);

    submitBtn.classList.add('loading');
    document.querySelector('#submit-btn .btn-text').textContent = t.btnStop;
    
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    ws = new WebSocket(`${protocol}//${window.location.host}/scan`);

    ws.onopen = () => {
        ws.send(JSON.stringify({ target, ports, timeout }));
    };

    ws.onmessage = (event) => {
        let result;
        try {
            result = JSON.parse(event.data);
        } catch (e) {
            if (event.data.startsWith('Error:')) {
                alert(event.data);
                resetButton();
            }
            return;
        }

        if (result.type === 'start') {
            totalTasks = result.total;
            updateStats();
            return;
        }

        if (result.type === 'info') {
            const tr = document.createElement('tr');
            tr.innerHTML = `<td colspan="7" style="text-align:center; color:var(--accent-primary); font-weight:500;">${result.message}</td>`;
            resultsBody.insertBefore(tr, resultsBody.firstChild);
            return;
        }

        addResultRow(result);
    };

    ws.onclose = () => {
        resetButton();
        ws = null;
    };

    ws.onerror = (err) => {
        console.error("WS error:", err);
        resetButton();
        ws = null;
    };
});

function resetButton() {
    submitBtn.classList.remove('loading');
    document.querySelector('#submit-btn .btn-text').textContent = t.btnStart;
}

function updateStats() {
    if (totalTasks > 0) {
        statScanned.textContent = `${scannedCount} / ${totalTasks}`;
    } else {
        statScanned.textContent = scannedCount;
    }
    statFeasible.textContent = feasibleCount;
}

function addResultRow(result) {
    scannedCount++;
    if (result.feasible) {
        feasibleCount++;
    }
    updateStats();

    const tr = document.createElement('tr');
    tr.style.opacity = '0';
    tr.style.transform = 'translateY(10px)';
    tr.style.transition = 'all 0.3s ease';

    const statusBadge = result.feasible 
        ? `<span class="badge badge-success">${t.badgeFeasible}</span>`
        : `<span class="badge badge-fail">${t.badgeFailed}</span>`;

    tr.innerHTML = `
        <td>${statusBadge}</td>
        <td>${result.ip}</td>
        <td>${result.port}</td>
        <td>${result.cert_domain || '-'}</td>
        <td>${result.alpn || '-'}</td>
        <td style="max-width: 200px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;" title="${result.cert_issuer}">${result.cert_issuer || '-'}</td>
        <td>${result.geo_code}</td>
    `;

    if (result.feasible) {
        resultsBody.insertBefore(tr, resultsBody.firstChild);
    } else {
        resultsBody.appendChild(tr);
    }

    requestAnimationFrame(() => {
        tr.style.opacity = '1';
        tr.style.transform = 'translateY(0)';
    });
}
