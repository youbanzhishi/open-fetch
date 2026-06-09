// OpenFetch - Popup 脚本

document.addEventListener('DOMContentLoaded', async () => {
    // 获取当前标签页
    const [tab] = await browser.tabs.query({ active: true, currentWindow: true });
    
    // 元素引用
    const pageInfoEl = document.getElementById('pageInfo');
    const downloadBtn = document.getElementById('downloadBtn');
    const cliStatusEl = document.getElementById('cliStatus');
    const cliStatusTextEl = document.getElementById('cliStatusText');
    const queueCountEl = document.getElementById('queueCount');
    const queueListEl = document.getElementById('queueList');
    const settingsBtn = document.getElementById('settingsBtn');
    const settingsPanel = document.getElementById('settingsPanel');
    const settingsLink = document.getElementById('settingsLink');
    const openCliBtn = document.getElementById('openCliBtn');
    
    // 状态
    let currentInfo = null;
    let settings = {};
    
    // 初始化
    async function init() {
        await loadSettings();
        await checkCLI();
        await loadQueue();
        await detectCurrentPage();
        setupEventListeners();
    }
    
    // 加载设置
    async function loadSettings() {
        try {
            const result = await browser.runtime.sendMessage({ action: 'getSettings' });
            settings = result;
            updateSettingsUI();
        } catch (e) {
            console.error('加载设置失败', e);
        }
    }
    
    // 更新设置UI
    function updateSettingsUI() {
        document.getElementById('autoDetectToggle').classList.toggle('active', settings.autoDetect);
        document.getElementById('notifyToggle').classList.toggle('active', settings.showNotification);
        document.getElementById('maxConcurrent').value = settings.maxConcurrent || 3;
    }
    
    // 检查CLI状态
    async function checkCLI() {
        try {
            const result = await browser.runtime.sendMessage({ action: 'checkCLI' });
            cliStatusEl.classList.toggle('online', result.online);
            cliStatusTextEl.textContent = result.online ? 'CLI在线' : 'CLI离线';
        } catch (e) {
            cliStatusEl.classList.remove('online');
            cliStatusTextEl.textContent = 'CLI离线';
        }
    }
    
    // 加载下载队列
    async function loadQueue() {
        try {
            const result = await browser.runtime.sendMessage({ action: 'getQueue' });
            const queue = result.queue || [];
            queueCountEl.textContent = `队列: ${queue.length}`;
            renderQueue(queue);
        } catch (e) {
            console.error('加载队列失败', e);
        }
    }
    
    // 渲染队列
    function renderQueue(queue) {
        if (queue.length === 0) {
            queueListEl.innerHTML = '<div class="empty-state"><div>暂无下载任务</div></div>';
            return;
        }
        
        queueListEl.innerHTML = queue.slice(0, 10).map(item => `
            <div class="queue-item">
                <div class="queue-icon">${getPlatformIcon(item.platform)}</div>
                <div class="queue-info">
                    <div class="queue-title">${escapeHtml(item.title || item.url)}</div>
                    <div class="queue-status">${item.status} · ${formatTime(item.timestamp)}</div>
                </div>
            </div>
        `).join('');
    }
    
    // 检测当前页面
    async function detectCurrentPage() {
        try {
            const result = await browser.tabs.sendMessage(tab.id, { action: 'getCurrentInfo' });
            if (result) {
                currentInfo = result;
                renderPageInfo(result);
                downloadBtn.disabled = false;
            }
        } catch (e) {
            // 无法获取页面信息
            pageInfoEl.innerHTML = '<div class="empty-state"><div class="empty-icon">🚫</div><div>无法检测此页面</div></div>';
        }
    }
    
    // 渲染页面信息
    function renderPageInfo(info) {
        pageInfoEl.innerHTML = `
            <div class="info-row">
                <span class="info-label">平台</span>
                <span class="platform-badge">${getPlatformName(info.platform)}</span>
            </div>
            <div class="info-row">
                <span class="info-label">类型</span>
                <span class="info-value">${getTypeName(info.type)}</span>
            </div>
            <div class="info-row">
                <span class="info-label">标题</span>
                <span class="info-value">${escapeHtml(info.title || '未知')}</span>
            </div>
            ${info.bvid ? `<div class="info-row"><span class="info-label">BV号</span><span class="info-value">${info.bvid}</span></div>` : ''}
            ${info.roomId ? `<div class="info-row"><span class="info-label">房间号</span><span class="info-value">${info.roomId}</span></div>` : ''}
        `;
    }
    
    // 事件监听
    function setupEventListeners() {
        // 下载按钮
        downloadBtn.addEventListener('click', async () => {
            if (!currentInfo) return;
            
            downloadBtn.disabled = true;
            downloadBtn.innerHTML = '<span class="spinner"></span> 添加中...';
            
            try {
                const result = await browser.runtime.sendMessage({
                    action: 'manualDownload',
                    data: currentInfo
                });
                
                if (result.id) {
                    downloadBtn.innerHTML = '✓ 已添加';
                    await loadQueue();
                }
            } catch (e) {
                downloadBtn.innerHTML = '✗ 添加失败';
            }
            
            setTimeout(() => {
                downloadBtn.disabled = false;
                downloadBtn.innerHTML = '⬇ 添加到下载队列';
            }, 2000);
        });
        
        // 设置按钮
        settingsBtn.addEventListener('click', () => {
            settingsPanel.classList.toggle('show');
        });
        
        settingsLink.addEventListener('click', () => {
            settingsPanel.classList.toggle('show');
        });
        
        // 自动检测开关
        document.getElementById('autoDetectToggle').addEventListener('click', async () => {
            settings.autoDetect = !settings.autoDetect;
            await saveSettings();
            updateSettingsUI();
        });
        
        // 通知开关
        document.getElementById('notifyToggle').addEventListener('click', async () => {
            settings.showNotification = !settings.showNotification;
            await saveSettings();
            updateSettingsUI();
        });
        
        // 最大并发
        document.getElementById('maxConcurrent').addEventListener('change', async (e) => {
            settings.maxConcurrent = parseInt(e.target.value);
            await saveSettings();
        });
        
        // 打开CLI
        openCliBtn.addEventListener('click', () => {
            // 尝试打开本地应用
            browser.tabs.create({ url: 'http://localhost:8080' });
        });
    }
    
    // 保存设置
    async function saveSettings() {
        try {
            await browser.runtime.sendMessage({
                action: 'updateSettings',
                settings: settings
            });
        } catch (e) {
            console.error('保存设置失败', e);
        }
    }
    
    // 工具函数
    function getPlatformIcon(platform) {
        const icons = {
            bilibili: '📺',
            youtube: '▶️',
            douyin: '🎵',
            weibo: '📰',
            generic: '🖼️'
        };
        return icons[platform] || '⬇️';
    }
    
    function getPlatformName(platform) {
        const names = {
            bilibili: '哔哩哔哩',
            bilibiliLive: 'B站直播',
            youtube: 'YouTube',
            douyin: '抖音',
            weibo: '微博',
            generic: '通用'
        };
        return names[platform] || platform;
    }
    
    function getTypeName(type) {
        const types = {
            video: '视频',
            live: '直播',
            images: '图片',
            audio: '音频'
        };
        return types[type] || type;
    }
    
    function formatTime(timestamp) {
        const date = new Date(timestamp);
        return `${date.getHours()}:${String(date.getMinutes()).padStart(2, '0')}`;
    }
    
    function escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }
    
    // 启动
    init();
});
