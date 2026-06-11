/**
 * OpenFetch Popup Script
 */

const API_BASE = 'http://localhost:8080';
let currentMedia = null;
let selectedQuality = 'best';

const platforms = {
    bilibili: { name: '哔哩哔哩', icon: '🟢' },
    youtube: { name: 'YouTube', icon: '🔴' },
    douyin: { name: '抖音', icon: '🎵' },
    tiktok: { name: 'TikTok', icon: '🎵' },
    weibo: { name: '微博', icon: '📱' },
    twitch: { name: 'Twitch', icon: '💜' },
    douyu: { name: '斗鱼', icon: '🐟' },
    huya: { name: '虎牙', icon: '🐯' },
    universal: { name: '通用', icon: '⬇️' }
};

const qualities = [
    { value: 'best', label: '最高' },
    { value: '1080p', label: '1080P' },
    { value: '720p', label: '720P' },
    { value: '480p', label: '480P' }
];

// 初始化
document.addEventListener('DOMContentLoaded', async () => {
    await checkConnection();
    await detectMedia();
    setupTabs();
});

async function checkConnection() {
    const statusEl = document.getElementById('status');
    try {
        const res = await fetch(`${API_BASE}/api/status`);
        if (res.ok) {
            statusEl.textContent = '✓ 已连接桌面端';
            statusEl.className = 'status connected';
        } else {
            throw new Error();
        }
    } catch {
        statusEl.textContent = '⚠ 未连接桌面端（启动 open-fetch server）';
        statusEl.className = 'status disconnected';
    }
}

async function detectMedia() {
    const mediaInfoEl = document.getElementById('media-info');
    
    try {
        // 获取当前tab
        const [tab] = await chrome.tabs.query({ active: true, currentWindow: true });
        
        // 从content script获取媒体信息
        const response = await chrome.tabs.sendMessage(tab.id, { type: 'GET_MEDIA_INFO' });
        
        if (response && response.media) {
            currentMedia = response.media;
            renderMediaCard(currentMedia);
        } else {
            // 尝试从background获取缓存
            const cached = await new Promise(resolve => {
                chrome.runtime.sendMessage({ type: 'GET_CURRENT_MEDIA' }, resolve);
            });
            
            if (cached) {
                currentMedia = cached;
                renderMediaCard(cached);
            } else {
                mediaInfoEl.innerHTML = `
                    <div class="no-media">
                        <div class="no-media-icon">🔍</div>
                        <p>未检测到可下载媒体</p>
                        <p style="font-size:12px;margin-top:8px;">访问视频页面后自动检测</p>
                    </div>
                `;
            }
        }
    } catch (error) {
        mediaInfoEl.innerHTML = `
            <div class="no-media">
                <div class="no-media-icon">🔍</div>
                <p>未检测到可下载媒体</p>
            </div>
        `;
    }
}

function renderMediaCard(media) {
    const platform = platforms[media.extractor] || platforms.universal;
    const mediaInfoEl = document.getElementById('media-info');
    
    const qualityButtons = qualities.map(q => `
        <button class="quality-btn ${q.value === selectedQuality ? 'selected' : ''}" 
                data-quality="${q.value}">
            ${q.label}
        </button>
    `).join('');
    
    mediaInfoEl.innerHTML = `
        <div class="platform-card detected">
            <div class="platform-name">
                <span>${platform.icon}</span>
                <span>${platform.name}</span>
            </div>
            <div class="video-title" title="${media.title || ''}">
                ${media.title || '未知标题'}
            </div>
            <div class="quality-options">
                ${qualityButtons}
            </div>
            <button class="download-btn" onclick="startDownload()">
                ⬇️ 开始下载
            </button>
        </div>
    `;
    
    // 绑定质量按钮事件
    document.querySelectorAll('.quality-btn').forEach(btn => {
        btn.addEventListener('click', () => {
            document.querySelectorAll('.quality-btn').forEach(b => b.classList.remove('selected'));
            btn.classList.add('selected');
            selectedQuality = btn.dataset.quality;
        });
    });
}

async function startDownload() {
    const btn = document.querySelector('.download-btn');
    btn.disabled = true;
    btn.textContent = '⏳ 下载中...';
    
    try {
        const response = await chrome.runtime.sendMessage({
            type: 'DOWNLOAD_REQUEST',
            data: {
                url: currentMedia.url || currentMedia.pageUrl,
                title: currentMedia.title,
                extractor: currentMedia.extractor,
                quality: selectedQuality
            }
        });
        
        if (response.success) {
            btn.textContent = '✓ 已开始下载';
            setTimeout(() => {
                btn.textContent = '⬇️ 开始下载';
                btn.disabled = false;
            }, 2000);
        } else {
            throw new Error(response.error);
        }
    } catch (error) {
        btn.textContent = '✗ 下载失败';
        btn.disabled = false;
        setTimeout(() => {
            btn.textContent = '⬇️ 开始下载';
        }, 2000);
    }
}

async function loadExtensions() {
    const listEl = document.getElementById('extensions-list');
    
    try {
        const res = await fetch(`${API_BASE}/api/extensions`);
        const data = await res.json();
        
        if (data.success && data.extensions) {
            listEl.innerHTML = data.extensions.map(ext => `
                <div class="extension-item">
                    <span class="extension-name">${ext.name}</span>
                    <span class="extension-status">v${ext.version}</span>
                </div>
            `).join('');
        } else {
            // 显示默认列表
            listEl.innerHTML = Object.entries(platforms).map(([key, p]) => `
                <div class="extension-item">
                    <span class="extension-name">${p.icon} ${p.name}</span>
                    <span class="extension-status">已支持</span>
                </div>
            `).join('');
        }
    } catch {
        listEl.innerHTML = '<div style="color:#888;text-align:center;padding:20px;">加载失败</div>';
    }
}

function setupTabs() {
    const tabs = document.querySelectorAll('.tab');
    const panels = {
        download: document.getElementById('download-panel'),
        extensions: document.getElementById('extensions-panel'),
        history: document.getElementById('history-panel')
    };
    
    tabs.forEach(tab => {
        tab.addEventListener('click', () => {
            tabs.forEach(t => t.classList.remove('active'));
            tab.classList.add('active');
            
            Object.values(panels).forEach(p => p.style.display = 'none');
            panels[tab.dataset.tab].style.display = 'block';
            
            if (tab.dataset.tab === 'extensions') {
                loadExtensions();
            }
        });
    });
}
