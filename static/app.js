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
        targetLabel: "Target (IP expands to subnet, CIDR, or Domain)",
        targetPlaceholder: "e.g. 1.1.1.1 scans its /24, or use 107.172.103.0/24, example.com",
        portsLabel: "Common SSL Ports",
        timeoutLabel: "Timeout (s)",
        btnStart: "Initiate Scan",
        btnStop: "Stop Scanning",
        btnSaveSettings: "Save Settings",
        statScanned: "Scanned",
        statFeasible: "Feasible",
        taskEyebrow: "Current Task",
        taskIdle: "Ready to scan",
        taskPreparing: "Preparing scan",
        taskScanning: "Scanning",
        taskReplaying: "Replaying cached results",
        taskCompleted: "Scan completed",
        taskStopped: "Scan paused",
        taskFailed: "Scan failed",
        taskTarget: "Target",
        taskPorts: "Ports",
        taskElapsed: "Elapsed",
        taskRate: "Rate",
        taskWaiting: "Waiting for a scan request.",
        taskResolving: "Resolving target and preparing tasks...",
        taskCurrent: "Now checking {ip}:{port}",
        taskFinishedSummary: "Finished {completed} of {total} tasks.",
        taskStoppedSummary: "Stopped at {completed} of {total}. Start again to resume.",
        taskPerMinute: "{count}/min",
        thStatus: "Status",
        thIP: "IP",
        thPort: "Port",
        thEndpoint: "Endpoint",
        thDomain: "Domain",
        thALPN: "ALPN",
        thIssuer: "Issuer",
        thGeo: "Geo",
        thScannedAt: "Scanned At",
        thAction: "Action",
        libraryEyebrow: "Node Library",
        libraryTitle: "Discovered Nodes",
        libraryReturned: "Returned",
        libraryGeos: "Geos",
        libraryDomains: "Domains",
        libraryEmptyTitle: "No nodes found",
        libraryEmptyText: "Adjust filters or run a new scan to populate the library.",
        histGeoLabel: "Geo",
        histGeoPlaceholder: "US",
        histDomainLabel: "Domain",
        histDomainPlaceholder: "example.com",
        histPortLabel: "Port",
        histPortPlaceholder: "443",
        histLimitLabel: "Limit",
        btnFetch: "Fetch",
        btnClearFilters: "Clear filters",
        btnCopy: "Copy",
        btnCopied: "Copied",
        btnDelete: "Delete",
        setConcurrency: "Concurrency Limit",
        setCooldown: "Cooldown (Days)",
        setIpv4: "IPv4 Max CIDR",
        setIpv6: "IPv6 Max CIDR",
        setPorts: "Allowed Ports",
        alertNoPort: "Please select at least one port.",
        alertDeleteConfirm: "Delete result for {ip}?",
        badgeFeasible: "Feasible",
        badgeFailed: "Invalid"
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
        targetLabel: "目标 (IP 自动扩展为所属网段、CIDR 或域名)",
        targetPlaceholder: "例如 1.1.1.1 会扫描其 /24，也可输入 107.172.103.0/24 或 example.com",
        portsLabel: "常见 SSL 端口",
        timeoutLabel: "超时时间 (秒)",
        btnStart: "开始扫描",
        btnStop: "停止扫描",
        btnSaveSettings: "保存设置",
        statScanned: "已扫描数",
        statFeasible: "健康节点数",
        taskEyebrow: "当前任务",
        taskIdle: "等待扫描",
        taskPreparing: "正在准备扫描",
        taskScanning: "扫描中",
        taskReplaying: "正在回放缓存结果",
        taskCompleted: "扫描完成",
        taskStopped: "扫描已暂停",
        taskFailed: "扫描失败",
        taskTarget: "目标",
        taskPorts: "端口",
        taskElapsed: "已用时",
        taskRate: "速率",
        taskWaiting: "等待发起扫描任务。",
        taskResolving: "正在解析目标并准备任务...",
        taskCurrent: "正在检测 {ip}:{port}",
        taskFinishedSummary: "已完成 {completed} / {total} 个任务。",
        taskStoppedSummary: "已停在 {completed} / {total}。再次开始可继续。",
        taskPerMinute: "{count}/分钟",
        thStatus: "状态",
        thIP: "IP",
        thPort: "端口",
        thEndpoint: "节点",
        thDomain: "域名",
        thALPN: "ALPN",
        thIssuer: "颁发者",
        thGeo: "地理位置",
        thScannedAt: "扫描时间",
        thAction: "操作",
        libraryEyebrow: "节点图库",
        libraryTitle: "已发现节点",
        libraryReturned: "当前结果",
        libraryGeos: "地区数",
        libraryDomains: "域名数",
        libraryEmptyTitle: "暂无节点",
        libraryEmptyText: "调整筛选条件，或先运行一次扫描写入节点。",
        histGeoLabel: "地区",
        histGeoPlaceholder: "例: US",
        histDomainLabel: "域名",
        histDomainPlaceholder: "例: example.com",
        histPortLabel: "端口",
        histPortPlaceholder: "443",
        histLimitLabel: "数量",
        btnFetch: "拉取记录",
        btnClearFilters: "清空筛选",
        btnCopy: "复制",
        btnCopied: "已复制",
        btnDelete: "删除",
        setConcurrency: "并发线程数限制",
        setCooldown: "扫描记录缓存期 (天)",
        setIpv4: "IPv4 最大 CIDR (如 24 代表 /24)",
        setIpv6: "IPv6 最大 CIDR (如 120 代表 /120)",
        setPorts: "允许扫描的端口 (逗号分隔)",
        alertNoPort: "请至少选择一个扫描端口。",
        alertDeleteConfirm: "确定要删除 {ip} 的记录吗？",
        badgeFeasible: "可用",
        badgeFailed: "无效"
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

    renderTaskState();
}

const form = document.getElementById('scan-form');
const submitBtn = document.getElementById('submit-btn');
const resultsBody = document.getElementById('results-body');
const statScanned = document.getElementById('stat-scanned');
const statFeasible = document.getElementById('stat-feasible');
const taskTitle = document.getElementById('task-title');
const taskPercent = document.getElementById('task-percent');
const progressFill = document.getElementById('progress-fill');
const taskTarget = document.getElementById('task-target');
const taskPorts = document.getElementById('task-ports');
const taskElapsed = document.getElementById('task-elapsed');
const taskRate = document.getElementById('task-rate');
const taskCurrent = document.getElementById('task-current');

const tabScan = document.getElementById('tab-scan');
const tabHistory = document.getElementById('tab-history');
const tabSettings = document.getElementById('tab-settings');
const scanSection = document.getElementById('scan-section');
const historySection = document.getElementById('history-section');
const settingsSection = document.getElementById('settings-section');
const fetchHistoryBtn = document.getElementById('fetch-history-btn');
const clearHistoryFiltersBtn = document.getElementById('clear-history-filters-btn');
const historyBody = document.getElementById('history-body');
const historyTotal = document.getElementById('history-total');
const historyGeoCount = document.getElementById('history-geo-count');
const historyDomainCount = document.getElementById('history-domain-count');
const historyEmpty = document.getElementById('history-empty');
const langToggle = document.getElementById('lang-toggle');

const taskState = {
    status: 'idle',
    target: '-',
    ports: '-',
    completed: 0,
    total: 0,
    startedAt: null,
    elapsedTimer: null,
    current: '',
};

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

function clearChildren(element) {
    while (element.firstChild) {
        element.removeChild(element.firstChild);
    }
}

function appendTextCell(row, value) {
    const td = document.createElement('td');
    td.textContent = value || '-';
    row.appendChild(td);
    return td;
}

function appendIssuerCell(row, value) {
    const td = appendTextCell(row, value);
    td.style.maxWidth = '200px';
    td.style.whiteSpace = 'nowrap';
    td.style.overflow = 'hidden';
    td.style.textOverflow = 'ellipsis';
    td.title = value || '';
    return td;
}

function formatElapsed(ms) {
    const totalSeconds = Math.max(0, Math.floor(ms / 1000));
    const minutes = Math.floor(totalSeconds / 60);
    const seconds = totalSeconds % 60;
    return `${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}`;
}

function formatTemplate(template, values) {
    return template.replace(/\{(\w+)\}/g, (_, key) => values[key] ?? '');
}

function setTaskTimer(active) {
    if (taskState.elapsedTimer) {
        clearInterval(taskState.elapsedTimer);
        taskState.elapsedTimer = null;
    }
    if (active) {
        taskState.elapsedTimer = setInterval(renderTaskState, 1000);
    }
}

function resetTaskState(status = 'idle') {
    setTaskTimer(false);
    taskState.status = status;
    taskState.target = '-';
    taskState.ports = '-';
    taskState.completed = 0;
    taskState.total = 0;
    taskState.startedAt = null;
    taskState.current = '';
    renderTaskState();
}

function startTaskState(target, ports) {
    taskState.status = 'preparing';
    taskState.target = target || '-';
    taskState.ports = ports.length ? ports.join(', ') : '-';
    taskState.completed = 0;
    taskState.total = 0;
    taskState.startedAt = Date.now();
    taskState.current = t.taskResolving;
    setTaskTimer(true);
    renderTaskState();
}

function updateTaskProgress({ status, completed, total, current }) {
    if (status) taskState.status = status;
    if (Number.isFinite(completed)) taskState.completed = completed;
    if (Number.isFinite(total)) taskState.total = total;
    if (current !== undefined) taskState.current = current;
    renderTaskState();
}

function finishTaskState(status, completed, total) {
    taskState.status = status;
    if (Number.isFinite(completed)) taskState.completed = completed;
    if (Number.isFinite(total)) taskState.total = total;
    setTaskTimer(false);
    if (status === 'completed') {
        taskState.current = formatTemplate(t.taskFinishedSummary, {
            completed: taskState.completed,
            total: taskState.total,
        });
    } else if (status === 'stopped') {
        taskState.current = formatTemplate(t.taskStoppedSummary, {
            completed: taskState.completed,
            total: taskState.total,
        });
    }
    renderTaskState();
}

function renderTaskState() {
    if (!taskTitle) return;

    const percent = taskState.total > 0
        ? Math.min(100, Math.round((taskState.completed / taskState.total) * 100))
        : 0;
    const elapsedMs = taskState.startedAt ? Date.now() - taskState.startedAt : 0;
    const elapsedMinutes = elapsedMs > 0 ? elapsedMs / 60000 : 0;
    const rate = elapsedMinutes > 0 ? Math.round(taskState.completed / elapsedMinutes) : 0;

    const titleByStatus = {
        idle: t.taskIdle,
        preparing: t.taskPreparing,
        scanning: t.taskScanning,
        replaying: t.taskReplaying,
        completed: t.taskCompleted,
        stopped: t.taskStopped,
        failed: t.taskFailed,
    };

    taskTitle.textContent = titleByStatus[taskState.status] || t.taskIdle;
    taskPercent.textContent = `${percent}%`;
    progressFill.style.width = `${percent}%`;
    taskTarget.textContent = taskState.target;
    taskPorts.textContent = taskState.ports;
    taskElapsed.textContent = formatElapsed(elapsedMs);
    taskRate.textContent = formatTemplate(t.taskPerMinute, { count: rate });
    taskCurrent.textContent = taskState.current || t.taskWaiting;
}

async function fetchHistory() {
    const geo = document.getElementById('hist-geo').value.trim();
    const domain = document.getElementById('hist-domain').value.trim();
    const port = document.getElementById('hist-port').value.trim();
    const limit = document.getElementById('hist-limit').value.trim();

    const params = new URLSearchParams();
    if (geo) params.set('geo_code', geo.toUpperCase());
    if (domain) params.set('domain', domain);
    if (port) params.set('port', port);
    if (limit) params.set('limit', limit);

    fetchHistoryBtn.classList.add('loading');

    try {
        const res = await fetch(`/results?${params.toString()}`);
        if (!res.ok) throw new Error('Failed to fetch history');
        const data = await res.json();

        renderHistorySummary(data);
        clearChildren(historyBody);
        historyEmpty.style.display = data.length === 0 ? 'block' : 'none';

        for (const row of data) {
            const tr = document.createElement('tr');
            appendTextCell(tr, row.ip);
            appendTextCell(tr, String(row.port));
            const endpoint = `${row.ip}:${row.port}`;
            const endpointCell = appendTextCell(tr, endpoint);
            endpointCell.className = 'endpoint-cell';
            appendIssuerCell(tr, row.cert_domain);
            appendIssuerCell(tr, row.cert_issuer);
            appendTextCell(tr, row.geo_code);

            const scannedAtCell = appendTextCell(tr, row.scanned_at);
            scannedAtCell.style.fontSize = '0.85em';
            scannedAtCell.style.color = 'var(--text-secondary)';

            const actionCell = document.createElement('td');
            const actionGroup = document.createElement('div');
            actionGroup.className = 'action-group';

            const copyBtn = document.createElement('button');
            copyBtn.className = 'copy-btn';
            copyBtn.dataset.endpoint = endpoint;
            copyBtn.textContent = t.btnCopy;

            const deleteBtn = document.createElement('button');
            deleteBtn.className = 'delete-btn';
            deleteBtn.dataset.ip = row.ip;
            deleteBtn.dataset.port = row.port;
            deleteBtn.textContent = t.btnDelete;

            actionGroup.appendChild(copyBtn);
            actionGroup.appendChild(deleteBtn);
            actionCell.appendChild(actionGroup);
            tr.appendChild(actionCell);
            historyBody.appendChild(tr);
        }

        document.querySelectorAll('.copy-btn').forEach(btn => {
            btn.addEventListener('click', async (e) => {
                const endpoint = e.target.getAttribute('data-endpoint');
                await copyText(endpoint);
                e.target.textContent = t.btnCopied;
                setTimeout(() => {
                    e.target.textContent = t.btnCopy;
                }, 1200);
            });
        });

        document.querySelectorAll('.delete-btn').forEach(btn => {
            btn.addEventListener('click', async (e) => {
                const ip = e.target.getAttribute('data-ip');
                const port = e.target.getAttribute('data-port');
                if (confirm(t.alertDeleteConfirm.replace('{ip}', ip))) {
                    try {
                        const delRes = await fetch(`/results/${encodeURIComponent(ip)}?port=${encodeURIComponent(port)}`, { method: 'DELETE' });
                        if (!delRes.ok) throw new Error('Failed to delete');
                        e.target.closest('tr').remove();
                        fetchHistory();
                    } catch (err) {
                        alert(err.message);
                    }
                }
            });
        });

    } catch (e) {
        alert(e.message);
    } finally {
        fetchHistoryBtn.classList.remove('loading');
    }
}

fetchHistoryBtn.addEventListener('click', fetchHistory);

clearHistoryFiltersBtn.addEventListener('click', () => {
    document.getElementById('hist-geo').value = '';
    document.getElementById('hist-domain').value = '';
    document.getElementById('hist-port').value = '';
    document.getElementById('hist-limit').value = '100';
    fetchHistory();
});

function renderHistorySummary(rows) {
    const geos = new Set(rows.map(row => row.geo_code).filter(Boolean));
    const domains = new Set(rows.map(row => row.cert_domain).filter(Boolean));
    historyTotal.textContent = rows.length;
    historyGeoCount.textContent = geos.size;
    historyDomainCount.textContent = domains.size;
}

async function copyText(text) {
    if (navigator.clipboard && window.isSecureContext) {
        await navigator.clipboard.writeText(text);
        return;
    }

    const textarea = document.createElement('textarea');
    textarea.value = text;
    textarea.style.position = 'fixed';
    textarea.style.opacity = '0';
    document.body.appendChild(textarea);
    textarea.focus();
    textarea.select();
    document.execCommand('copy');
    textarea.remove();
}

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
        alert(currentLang === 'zh' ? '设置已保存' : 'Settings saved successfully');
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
        finishTaskState('stopped', scannedCount, totalTasks);
        ws.close();
        ws = null;
        resetButton();
        return;
    }

    clearChildren(resultsBody);
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
    startTaskState(target, ports);

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
                taskState.status = 'failed';
                taskState.current = event.data;
                setTaskTimer(false);
                renderTaskState();
                resetButton();
            }
            return;
        }

        if (result.type === 'start') {
            totalTasks = result.total;
            if (result.target) taskState.target = result.target;
            if (Array.isArray(result.ports)) taskState.ports = result.ports.join(', ');
            const completed = Number.isFinite(result.completed) ? result.completed : scannedCount;
            updateTaskProgress({
                status: result.mode === 'cache' ? 'replaying' : 'scanning',
                completed,
                total: totalTasks,
                current: result.mode === 'cache' ? t.taskReplaying : t.taskResolving,
            });
            updateStats();
            return;
        }

        if (result.type === 'progress') {
            updateTaskProgress({
                status: result.mode === 'cache' ? 'replaying' : 'scanning',
                completed: result.completed,
                total: result.total || totalTasks,
                current: result.ip && result.port
                    ? formatTemplate(t.taskCurrent, { ip: result.ip, port: result.port })
                    : undefined,
            });
            return;
        }

        if (result.type === 'done') {
            finishTaskState(result.status === 'stopped' ? 'stopped' : 'completed', result.completed, result.total || totalTasks);
            return;
        }

        if (result.type === 'info') {
            taskState.current = result.message;
            renderTaskState();
            const tr = document.createElement('tr');
            const td = document.createElement('td');
            td.colSpan = 7;
            td.style.textAlign = 'center';
            td.style.color = 'var(--accent-primary)';
            td.style.fontWeight = '500';
            td.textContent = result.message;
            tr.appendChild(td);
            resultsBody.insertBefore(tr, resultsBody.firstChild);
            return;
        }

        addResultRow(result);
    };

    ws.onclose = () => {
        if (taskState.status === 'preparing' || taskState.status === 'scanning' || taskState.status === 'replaying') {
            finishTaskState('stopped', scannedCount, totalTasks);
        }
        resetButton();
        ws = null;
    };

    ws.onerror = (err) => {
        console.error("WS error:", err);
        taskState.status = 'failed';
        taskState.current = err?.message || t.taskFailed;
        setTaskTimer(false);
        renderTaskState();
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
    updateTaskProgress({
        status: taskState.status === 'replaying' ? 'replaying' : 'scanning',
        completed: scannedCount,
        total: totalTasks,
        current: formatTemplate(t.taskCurrent, { ip: result.ip, port: result.port }),
    });

    if (!result.feasible && !result.cert_domain) {
        return;
    }

    const tr = document.createElement('tr');
    tr.style.opacity = '0';
    tr.style.transform = 'translateY(10px)';
    tr.style.transition = 'all 0.3s ease';

    const statusCell = document.createElement('td');
    const statusBadge = document.createElement('span');
    statusBadge.className = result.feasible ? 'badge badge-success' : 'badge badge-fail';
    statusBadge.textContent = result.feasible ? t.badgeFeasible : t.badgeFailed;
    statusCell.appendChild(statusBadge);
    tr.appendChild(statusCell);

    appendTextCell(tr, result.ip);
    appendTextCell(tr, String(result.port));
    appendIssuerCell(tr, result.cert_domain);
    appendTextCell(tr, result.alpn);
    appendIssuerCell(tr, result.cert_issuer);
    appendTextCell(tr, result.geo_code);

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
