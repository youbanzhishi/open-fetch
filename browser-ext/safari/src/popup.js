// OpenFetch Safari - Popup

document.addEventListener('DOMContentLoaded', async () => {
    const [tab] = await browser.tabs.query({ active: true, currentWindow: true });
    
    let currentInfo = null;
    
    // 初始化
    async function init() {
        await checkCLI();
        await loadQueue();
        await detectPage();
    }
    
    // 检查CLI
    async function checkCLI() {
        try {
            const result = await browser.runtime.sendMessage({ action: 'checkCLI' });
            document.getElementById('statusDot').classList.toggle('online', result.online);
            document.getElementById('statusText').textContent = result.online ? 'CLI在线' : 'CLI离线';
        } catch {
            document.getElementById('statusText').textContent = 'CLI离线';
        }
    }
    
    // 加载队列
    async function loadQueue() {
        try {
            const result = await browser.runtime.sendMessage({ action: 'getQueue' });
            const queue = result.queue || [];
            document.getElementById('queueCount').textContent = `队列: ${queue.length}`;
            renderQueue(queue);
        } catch (e) {
            console.error('加载队列失败', e);
        }
    }
    
    // 渲染队列
    function renderQueue(queue) {
        const el = document.getElementById('queueList');
        if (queue.length === 0) {
            el.innerHTML = '<div class="empty">暂无下载任务</div>';
            return;
        }
        el.innerHTML = queue.slice(0, 5).map(item => `
            <div class="info-row">
                <span class="info-value">${escape(item.title || item.platform)}</span>
                <span style="color:var(--text-muted)">${item.status}</span>
            </div>
        `).join('');
    }
    
    // 检测页面
    async function detectPage() {
        try {
            const result = await browser.tabs.sendMessage(tab.id, { action: 'getCurrentInfo' });
            if (result) {
                currentInfo = result;
                renderPageInfo(result);
                document.getElementById('downloadBtn').disabled = false;
            }
        } catch {
            document.getElementById('pageInfo').innerHTML = '<div class="empty"><div class="empty-icon">🚫</div><div>无法检测此页面</div></div>';
        }
    }
    
    // 渲染页面信息
    function renderPageInfo(info) {
        document.getElementById('pageInfo').innerHTML = `
            <div class="info-row">
                <span class="info-label">平台</span>
                <span class="badge">${getPlatformName(info.platform)}</span>
            </div>
            <div class="info-row">
                <span class="info-label">类型</span>
                <span class="info-value">${getTypeName(info.type)}</span>
            </div>
            <div class="info-row">
                <span class="info-label">标题</span>
                <span class="info-value" style="max-width:180px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap">${escape(info.title || '未知')}</span>
            </div>
        `;
    }
    
    // 下载按钮
    document.getElementById('downloadBtn').addEventListener('click', async () => {
        if (!currentInfo) return;
        const btn = document.getElementById('downloadBtn');
        btn.disabled = true;
        btn.innerHTML = '添加中...';
        
        try {
            await browser.runtime.sendMessage({ action: 'manualDownload', data: currentInfo });
            btn.innerHTML = '✓ 已添加';
            await loadQueue();
        } catch {
            btn.innerHTML = '✗ 失败';
        }
        
        setTimeout(() => {
            btn.disabled = false;
            btn.innerHTML = '⬇ 添加到下载';
        }, 1500);
    });
    
    // 设置按钮
    document.getElementById('settingsBtn').addEventListener('click', () => {
        browser.runtime.openOptionsPage();
    });
    
    // 工具函数
    function getPlatformName(p) {
        const names = { bilibili: '哔哩哔哩', youtube: 'YouTube', douyin: '抖音', weibo: '微博' };
        return names[p] || p;
    }
    
    function getTypeName(t) {
        const types = { video: '视频', live: '直播', images: '图片' };
        return types[t] || t;
    }
    
    function escape(text) {
        const d = document.createElement('div');
        d.textContent = text;
        return d.innerHTML;
    }
    
    init();
});
