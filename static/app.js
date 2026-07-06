const translations = {
    en: {
        pageTitle: "nodemates-scanner - Advanced TLS Node Probing",
        navDocs: "Documentation",
        title: "nodemates-scanner",
        subtitle: "High-performance asynchronous TLS probing, Reality node verification, and GeoIP discovery platform.",
        feat1: "⚡ Asynchronous TLS Probing",
        feat2: "🛡️ Anti-abuse Protection",
        feat3: "🌍 Real-time GeoIP",
        tabScanner: "Scanner",
        tabHistory: "Scan History",
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
        thIP: "IP",
        thPort: "Port",
        thEndpoint: "Endpoint",
        thDomain: "Domain",
        thALPN: "ALPN",
        thIssuer: "Issuer",
        thCertType: "Cert Type",
        thScannedAt: "Scanned At",
        thAction: "Action",
        historyEyebrow: "Scan History",
        historyTitle: "Past Scans",
        historyEmptyTitle: "No scan history found",
        historyEmptyText: "Run a new scan to start building your history.",
        btnRefresh: "Refresh",
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
        badgeFailed: "Invalid",
        btnBack: "Back",
        settingsTitle: "System Settings"
    },
    zh: {
        pageTitle: "nodemates-scanner - 高级 TLS 节点嗅探",
        navDocs: "文档",
        title: "nodemates-scanner",
        subtitle: "高性能异步 TLS 嗅探、Reality 节点验证与自动化 GeoIP 探测平台。",
        feat1: "⚡ 纯异步 TLS 嗅探",
        feat2: "🛡️ 防滥用与拦截机制",
        feat3: "🌍 实时 GeoIP 映射",
        tabScanner: "雷达扫描",
        tabHistory: "扫描历史",
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
        thIP: "IP",
        thPort: "端口",
        thEndpoint: "端点",
        thDomain: "域名",
        thALPN: "ALPN",
        thIssuer: "颁发者",
        thCertType: "证书类型",
        thScannedAt: "扫描时间",
        thAction: "操作",
        historyEyebrow: "任务列表",
        historyTitle: "历史扫描记录",
        historyEmptyTitle: "暂无扫描历史",
        historyEmptyText: "去雷达扫描页运行一次扫描即可生成历史记录。",
        btnRefresh: "刷新",
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
        badgeFailed: "无效",
        btnBack: "返回",
        settingsTitle: "系统设置"
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
const navSettings = document.getElementById('nav-settings');
const scanSection = document.getElementById('scan-section');
const historySection = document.getElementById('history-section');
const settingsSection = document.getElementById('settings-section');
const historyAccordionContainer = document.getElementById('history-accordion-container');
const refreshHistoryBtn = document.getElementById('refresh-history-btn');
const historyEmpty = document.getElementById('history-empty');
const langToggle = document.getElementById('lang-toggle');
const tabsContainer = document.querySelector('.glass-container > header > .tabs');
const settingsBackBtn = document.getElementById('settings-back-btn');

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

// Custom Toast and Confirmation Dialog functions
function showToast(message, type = 'info') {
    const container = document.getElementById('toast-container');
    if (!container) return;

    const toast = document.createElement('div');
    toast.className = `toast toast-${type}`;
    
    let icon = 'ℹ️';
    if (type === 'success') icon = '✅';
    if (type === 'error') icon = '❌';

    toast.innerHTML = `
        <span class="toast-icon">${icon}</span>
        <span class="toast-message">${message}</span>
    `;

    container.appendChild(toast);

    // Fade in
    requestAnimationFrame(() => {
        toast.classList.add('show');
    });

    // Fade out and remove
    setTimeout(() => {
        toast.classList.remove('show');
        const handleTransitionEnd = () => {
            toast.remove();
            toast.removeEventListener('transitionend', handleTransitionEnd);
        };
        toast.addEventListener('transitionend', handleTransitionEnd);
    }, 4000);
}

function showConfirm(title, message) {
    return new Promise((resolve) => {
        const modal = document.getElementById('modal-container');
        const modalTitle = document.getElementById('modal-title');
        const modalMessage = document.getElementById('modal-message');
        const confirmBtn = document.getElementById('modal-confirm-btn');
        const cancelBtn = document.getElementById('modal-cancel-btn');
        const backdrop = document.getElementById('modal-backdrop');

        if (!modal || !modalTitle || !modalMessage || !confirmBtn || !cancelBtn) {
            resolve(false);
            return;
        }

        modalTitle.textContent = title;
        modalMessage.textContent = message;

        // Localized button text
        confirmBtn.textContent = currentLang === 'zh' ? '确定' : 'Confirm';
        cancelBtn.textContent = currentLang === 'zh' ? '取消' : 'Cancel';

        modal.style.display = 'flex';
        // Trigger transition
        requestAnimationFrame(() => {
            modal.classList.add('show');
        });

        const cleanup = (value) => {
            modal.classList.remove('show');
            
            const handleTransitionEnd = () => {
                modal.style.display = 'none';
                modal.removeEventListener('transitionend', handleTransitionEnd);
            };
            modal.addEventListener('transitionend', handleTransitionEnd);

            // Remove event listeners
            confirmBtn.removeEventListener('click', onConfirm);
            cancelBtn.removeEventListener('click', onCancel);
            backdrop.removeEventListener('click', onCancel);

            resolve(value);
        };

        const onConfirm = () => cleanup(true);
        const onCancel = () => cleanup(false);

        confirmBtn.addEventListener('click', onConfirm);
        cancelBtn.addEventListener('click', onCancel);
        backdrop.addEventListener('click', onCancel);
    });
}

// Tabs logic
tabScan.addEventListener('click', () => {
    tabScan.classList.add('active');
    tabHistory.classList.remove('active');
    scanSection.style.display = 'block';
    historySection.style.display = 'none';
    settingsSection.style.display = 'none';
    if (tabsContainer) tabsContainer.style.display = 'inline-flex';
});

tabHistory.addEventListener('click', () => {
    tabHistory.classList.add('active');
    tabScan.classList.remove('active');
    scanSection.style.display = 'none';
    historySection.style.display = 'block';
    settingsSection.style.display = 'none';
    if (tabsContainer) tabsContainer.style.display = 'inline-flex';
    fetchHistoryTasks();
});

navSettings.addEventListener('click', (e) => {
    e.preventDefault();
    tabScan.classList.remove('active');
    tabHistory.classList.remove('active');
    scanSection.style.display = 'none';
    historySection.style.display = 'none';
    settingsSection.style.display = 'block';
    if (tabsContainer) tabsContainer.style.display = 'none';
    fetchSettings();
});

if (settingsBackBtn) {
    settingsBackBtn.addEventListener('click', () => {
        tabScan.click();
    });
}

function clearChildren(element) {
    while (element.firstChild) {
        element.removeChild(element.firstChild);
    }
}

function appendTextCell(tr, text, tooltipText = null) {
    const td = document.createElement('td');
    td.textContent = text;
    if (tooltipText) {
        td.title = tooltipText;
    }
    tr.appendChild(td);
    return td;
}

function appendLatencyCell(tr, latencyMs) {
    const td = document.createElement('td');
    td.style.fontWeight = '600';
    
    if (latencyMs === 0 || !latencyMs) {
        td.textContent = '-';
        td.style.color = 'var(--text-secondary)';
    } else {
        td.textContent = `${latencyMs}ms`;
        if (latencyMs < 100) {
            td.style.color = '#34C759'; // Green
        } else if (latencyMs < 250) {
            td.style.color = '#FF9500'; // Orange
        } else {
            td.style.color = '#FF3B30'; // Red
        }
    }
    
    tr.appendChild(td);
    return td;
}

function extractIssuerName(dn) {
    if (!dn) return '-';
    const oMatch = dn.match(/O=([^,]+)/i);
    if (oMatch) return oMatch[1].trim();
    const cnMatch = dn.match(/CN=([^,]+)/i);
    return cnMatch ? cnMatch[1].trim() : dn;
}

function appendIssuerCell(row, value) {
    const cleanIssuer = extractIssuerName(value);
    const td = appendTextCell(row, cleanIssuer);
    td.style.maxWidth = '200px';
    td.style.whiteSpace = 'nowrap';
    td.style.overflow = 'hidden';
    td.style.textOverflow = 'ellipsis';
    td.title = value || '';
    return td;
}

function extractCN(dn) {
    if (!dn) return '-';
    const match = dn.match(/CN=([^,]+)/i);
    return match ? match[1].trim() : dn;
}

function appendDomainCell(tr, fullDomain, fullIssuer, asnOrg) {
    const td = document.createElement('td');
    td.style.maxWidth = '200px';
    td.style.whiteSpace = 'nowrap';
    td.style.overflow = 'hidden';
    td.style.textOverflow = 'ellipsis';
    td.title = fullDomain || '';
    
    const cleanDomain = extractCN(fullDomain);
    const span = document.createElement('span');
    span.textContent = cleanDomain;
    td.appendChild(span);
    
    if (fullDomain) {
        const dUpper = fullDomain.toUpperCase();
        const fakeKeywords = ['TRAEFIK DEFAULT CERT', 'KUBERNETES INGRESS', 'LOCALHOST', 'FAKE'];
        const popularDest = ['CLOUDFLARE', 'GOOGLE', 'APPLE', 'MICROSOFT', 'BING', 'ITUNES', 'AKAMAI', 'FASTLY', 'AMAZON', 'AWS'];
        
        let badgeText = null;
        let badgeType = null; // 'warning' or 'info' or 'danger'
        
        if (fakeKeywords.some(k => dUpper.includes(k))) {
            badgeText = currentLang === 'zh' ? '伪造' : 'Fake';
            badgeType = 'warning';
        } else if (fullDomain === fullIssuer) {
            badgeText = currentLang === 'zh' ? '自签' : 'Self-Signed';
            badgeType = 'warning';
        } else if (popularDest.some(k => dUpper.includes(k))) {
            const orgUpper = (asnOrg || '').toUpperCase();
            const domainKeyword = popularDest.find(k => dUpper.includes(k));
            
            if (orgUpper && !orgUpper.includes(domainKeyword) && !orgUpper.includes('CLOUDFLARE') && !orgUpper.includes('GOOGLE') && !orgUpper.includes('AKAMAI')) {
                badgeText = currentLang === 'zh' ? '高可疑/伪造' : 'High Suspicion/Fake';
                badgeType = 'danger';
            } else {
                badgeText = currentLang === 'zh' ? '大厂/疑似转发' : 'CDN/Proxy';
                badgeType = 'info';
            }
        }
        
        if (badgeText) {
            const badge = document.createElement('span');
            badge.className = `badge badge-${badgeType}`;
            badge.style.marginLeft = '0.4rem';
            badge.style.fontSize = '0.65rem';
            badge.style.padding = '0.15rem 0.35rem';
            badge.style.borderRadius = '4px';
            badge.style.verticalAlign = 'middle';
            
            if (badgeType === 'warning') {
                badge.style.background = 'rgba(255, 149, 0, 0.15)';
                badge.style.color = '#FF9500';
                badge.style.border = '1px solid rgba(255, 149, 0, 0.3)';
            } else if (badgeType === 'info') {
                badge.style.background = 'rgba(41, 151, 255, 0.15)';
                badge.style.color = 'var(--accent-primary)';
                badge.style.border = '1px solid rgba(41, 151, 255, 0.3)';
            } else if (badgeType === 'danger') {
                badge.style.background = 'rgba(255, 59, 48, 0.15)';
                badge.style.color = '#FF3B30';
                badge.style.border = '1px solid rgba(255, 59, 48, 0.3)';
            }
            
            badge.textContent = badgeText;
            td.appendChild(badge);
        }
    }
    
    tr.appendChild(td);
    return td;
}

function formatElapsed(ms) {
    const totalSeconds = Math.max(0, Math.floor(ms / 1000));
    const minutes = Math.floor(totalSeconds / 60);
    const seconds = totalSeconds % 60;
    return `${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}`;
}

function formatLocalTime(utcString) {
    if (!utcString) return '-';
    let isoString = utcString;
    if (utcString.includes(' ') && !utcString.includes('T')) {
        isoString = utcString.replace(' ', 'T') + 'Z';
    } else if (!utcString.endsWith('Z') && !utcString.includes('+') && !utcString.includes('-')) {
        isoString = utcString + 'Z';
    }
    const date = new Date(isoString);
    if (isNaN(date.getTime())) {
        return utcString;
    }
    const pad = (num) => String(num).padStart(2, '0');
    const year = date.getFullYear();
    const month = pad(date.getMonth() + 1);
    const day = pad(date.getDate());
    const hours = pad(date.getHours());
    const minutes = pad(date.getMinutes());
    const seconds = pad(date.getSeconds());
    return `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`;
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

async function fetchHistoryTasks() {
    refreshHistoryBtn.classList.add('loading');
    try {
        const res = await fetch('/history/tasks');
        if (!res.ok) throw new Error('Failed to fetch history tasks');
        const data = await res.json();

        clearChildren(historyAccordionContainer);
        historyEmpty.style.display = data.length === 0 ? 'block' : 'none';

        for (const task of data) {
            const accordionItem = document.createElement('div');
            accordionItem.className = 'accordion-item';

            const header = document.createElement('div');
            header.className = 'accordion-header';
            
            const targetInfo = document.createElement('div');
            targetInfo.className = 'accordion-target';
            const localScannedAt = formatLocalTime(task.scanned_at);
            const scannedAtText = currentLang === 'zh' ? '扫描时间' : 'Scanned At';
            const statusText = currentLang === 'zh' ? '状态' : 'Status';
            const completedText = currentLang === 'zh' ? '完成进度' : 'Completed';
            targetInfo.innerHTML = `<strong>${task.target}</strong><span class="accordion-meta">${completedText}: ${task.completed_tasks} / ${task.total_tasks} | ${statusText}: ${task.status} | ${scannedAtText}: ${localScannedAt}</span>`;
            
            const taskActionGroup = document.createElement('div');
            const taskDeleteBtn = document.createElement('button');
            taskDeleteBtn.className = 'secondary-btn';
            taskDeleteBtn.style.padding = '0.25rem 0.5rem';
            taskDeleteBtn.style.fontSize = '0.8rem';
            taskDeleteBtn.textContent = t.btnDelete || 'Delete';
            
            taskDeleteBtn.addEventListener('click', async (e) => {
                e.stopPropagation(); // prevent expanding accordion
                const confirmTitle = currentLang === 'zh' ? '删除确认' : 'Delete Confirmation';
                const confirmMessage = currentLang === 'zh' ? `确定要删除 ${task.target} 的扫描历史吗？` : `Delete scan history for ${task.target}?`;
                const confirmed = await showConfirm(confirmTitle, confirmMessage);
                if (confirmed) {
                    try {
                        const delRes = await fetch(`/history/tasks/${task.id}`, { method: 'DELETE' });
                        if (!delRes.ok) throw new Error('Failed to delete history task');
                        accordionItem.remove();
                        showToast(currentLang === 'zh' ? '删除成功' : 'Deleted successfully', 'success');
                    } catch (err) {
                        showToast(err.message, 'error');
                    }
                }
            });
            taskActionGroup.appendChild(taskDeleteBtn);

            header.appendChild(targetInfo);
            header.appendChild(taskActionGroup);

            const content = document.createElement('div');
            content.className = 'accordion-content';
            content.style.display = 'none';

            header.addEventListener('click', async () => {
                const isActive = accordionItem.classList.contains('active');

                document.querySelectorAll('.accordion-item').forEach(item => {
                    item.classList.remove('active');
                    item.querySelector('.accordion-content').style.display = 'none';
                });

                if (!isActive) {
                    accordionItem.classList.add('active');
                    content.style.display = 'block';

                    if (content.innerHTML === '') {
                        content.innerHTML = '<div class="spinner"></div>';
                        try {
                            const res = await fetch(`/results?history_id=${task.id}`);
                            if (!res.ok) throw new Error('Failed to fetch results');
                            const resultsData = await res.json();
                            renderTaskResults(content, resultsData);
                        } catch (err) {
                            content.innerHTML = `<span style="color:var(--danger)">${err.message}</span>`;
                        }
                    }
                }
            });

            accordionItem.appendChild(header);
            accordionItem.appendChild(content);
            historyAccordionContainer.appendChild(accordionItem);
        }
    } catch (e) {
        showToast(e.message, 'error');
    } finally {
        refreshHistoryBtn.classList.remove('loading');
    }
}

function renderTaskResults(container, data) {
    clearChildren(container);
    if (data.length === 0) {
        container.innerHTML = '<div style="padding: 1rem; color: var(--text-secondary);">No feasible nodes found in this scan.</div>';
        return;
    }

    const wrapper = document.createElement('div');
    wrapper.className = 'table-wrapper';
    
    const table = document.createElement('table');
    table.innerHTML = `
        <thead>
            <tr>
                <th data-i18n="thIP">${t.thIP}</th>
                <th data-i18n="thPort">${t.thPort}</th>
                <th data-i18n="thLatency">${t.thLatency}</th>
                <th data-i18n="thTLS">${t.thTLS}</th>
                <th data-i18n="thDomain">${t.thDomain}</th>
                <th data-i18n="thValidity">${t.thValidity}</th>
                <th data-i18n="thALPN">${t.thALPN}</th>
                <th data-i18n="thIssuer">${t.thIssuer}</th>
                <th data-i18n="thScannedAt">${t.thScannedAt}</th>
                <th data-i18n="thAction">${t.thAction}</th>
            </tr>
        </thead>
    `;
    const tbody = document.createElement('tbody');

    for (const row of data) {
        const tr = document.createElement('tr');
        appendTextCell(tr, row.ip, row.asn_org ? `ASN: ${row.asn_org}` : null);
        appendTextCell(tr, String(row.port));
        appendLatencyCell(tr, row.latency);
        appendTextCell(tr, row.tls_version);
        appendDomainCell(tr, row.cert_domain, row.cert_issuer, row.asn_org);
        appendTextCell(tr, row.cert_validity);
        appendTextCell(tr, row.alpn);
        appendIssuerCell(tr, row.cert_issuer);
        const localScannedAt = formatLocalTime(row.scanned_at);
        const scannedAtCell = appendTextCell(tr, localScannedAt);
        scannedAtCell.style.fontSize = '0.85em';
        scannedAtCell.style.color = 'var(--text-secondary)';

        const actionCell = document.createElement('td');
        const actionGroup = document.createElement('div');
        actionGroup.className = 'action-group';

        const endpoint = `${row.ip}:${row.port}`;
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
        tbody.appendChild(tr);
    }
    
    table.appendChild(tbody);
    wrapper.appendChild(table);
    container.appendChild(wrapper);
}

refreshHistoryBtn.addEventListener('click', fetchHistoryTasks);
historyAccordionContainer.addEventListener('click', handleHistoryResultAction);

async function handleHistoryResultAction(e) {
    const copyBtn = e.target.closest('.copy-btn');
    if (copyBtn) {
        await copyText(copyBtn.dataset.endpoint);
        copyBtn.textContent = t.btnCopied;
        setTimeout(() => {
            copyBtn.textContent = t.btnCopy;
        }, 1200);
        return;
    }

    const deleteBtn = e.target.closest('.delete-btn');
    if (!deleteBtn) return;

    const { ip, port } = deleteBtn.dataset;
    const confirmTitle = currentLang === 'zh' ? '删除确认' : 'Delete Confirmation';
    const confirmed = await showConfirm(confirmTitle, t.alertDeleteConfirm.replace('{ip}', ip));
    if (!confirmed) return;

    try {
        const delRes = await fetch(`/results/${encodeURIComponent(ip)}?port=${encodeURIComponent(port)}`, { method: 'DELETE' });
        if (!delRes.ok) throw new Error('Failed to delete');
        deleteBtn.closest('tr').remove();
        showToast(currentLang === 'zh' ? '删除成功' : 'Deleted successfully', 'success');
    } catch (err) {
        showToast(err.message, 'error');
    }
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
        showToast(e.message, 'error');
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
        showToast(currentLang === 'zh' ? '设置已保存' : 'Settings saved successfully', 'success');
    } catch (err) {
        showToast(err.message, 'error');
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
        showToast(t.alertNoPort, 'error');
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
                showToast(event.data, 'error');
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
    tr.className = result.feasible ? 'row-feasible' : 'row-invalid';
    tr.style.opacity = '0';
    tr.style.transform = 'translateY(10px)';
    tr.style.transition = 'all 0.3s ease';

    appendTextCell(tr, result.ip, result.asn_org ? `ASN: ${result.asn_org}` : null);
    appendTextCell(tr, String(result.port));
    appendLatencyCell(tr, result.latency);
    appendTextCell(tr, result.tls_version);
    appendDomainCell(tr, result.cert_domain, result.cert_issuer, result.asn_org);
    appendTextCell(tr, result.cert_validity);
    appendTextCell(tr, result.alpn);
    appendIssuerCell(tr, result.cert_issuer);

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

// Apple Style Select Logic
const portSelectBtn = document.getElementById('port-select-btn');
const portSelectDropdown = document.getElementById('port-select-dropdown');
const portSelectText = document.getElementById('port-select-text');
const portCheckboxesApple = document.querySelectorAll('#port-select-dropdown input[name="ports"]');

if (portSelectBtn && portSelectDropdown && portSelectText) {
    portSelectBtn.addEventListener('click', (e) => {
        e.stopPropagation();
        portSelectBtn.classList.toggle('open');
    });

    document.addEventListener('click', (e) => {
        if (!portSelectDropdown.contains(e.target) && !portSelectBtn.contains(e.target)) {
            portSelectBtn.classList.remove('open');
        }
    });

    const updatePortText = () => {
        const selected = Array.from(portCheckboxesApple)
            .filter(cb => cb.checked)
            .map(cb => cb.value);
        if (selected.length === 0) {
            portSelectText.textContent = currentLang === 'zh' ? '未选择' : 'None';
        } else if (selected.length <= 3) {
            portSelectText.textContent = selected.join(', ');
        } else {
            portSelectText.textContent = currentLang === 'zh' ? `${selected.length} 个端口` : `${selected.length} selected`;
        }
    };

    portCheckboxesApple.forEach(cb => {
        cb.addEventListener('change', updatePortText);
    });

    updatePortText();
}
