/**
 * OpenFetch Background Service Worker
 * 管理下载任务、状态、同步
 */

// 连接本地服务
const API_BASE = 'http://localhost:8080';
const EXTENSIONS = ['bilibili', 'youtube', 'douyin', 'weibo', 'twitch'];

// 状态管理
const state = {
    mediaCache: new Map(),
    downloadQueue: [],
    isConnected: false
};

// API调用封装
async function apiCall(endpoint, method = 'GET', data = null) {
    try {
        const options = {
            method,
            headers: { 'Content-Type': 'application/json' }
        };
        if (data) options.body = JSON.stringify(data);
        
        const response = await fetch(`${API_BASE}${endpoint}`, options);
        if (!response.ok) throw new Error(`API error: ${response.status}`);
        return await response.json();
    } catch (error) {
        console.error('API call failed:', error);
        return { success: false, error: error.message };
    }
}

// 扩展检测器
function detectExtractor(url) {
    const urlLower = url.toLowerCase();
    
    const patterns = {
        bilibili: /bilibili\.com|bilibili\.co/i,
        youtube: /youtube\.com|youtu\.be/i,
        douyin: /douyin\.com|iesdouyin\.com|v\.douyin\.com/i,
        tiktok: /tiktok\.com/i,
        weibo: /weibo\.com|weibo\.cn/i,
        ixigua: /ixigua\.com/i,
        kuaishou: /kuaishou\.com|ksyun\.com/i,
        twitch: /twitch\.tv/i,
        douyu: /douyu\.com/i,
        huya: /huya\.com/i
    };
    
    for (const [name, pattern] of Object.entries(patterns)) {
        if (pattern.test(urlLower)) {
            return name;
        }
    }
    return 'universal';
}

// 消息处理
chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
    const handlers = {
        // 媒体检测
        MEDIA_DETECTED: async (data) => {
            const { url, title, platform } = data;
            const extractor = detectExtractor(url);
            
            state.mediaCache.set(url, {
                ...data,
                extractor,
                detectedAt: Date.now()
            });
            
            // 更新badge
            chrome.action.setBadgeText({ text: '1' });
            chrome.action.setBadgeBackgroundColor({ color: '#4CAF50' });
            
            return { success: true, extractor };
        },
        
        // 下载请求
        DOWNLOAD_REQUEST: async (data) => {
            const { url, quality, format, extractor } = data;
            const finalExtractor = extractor || detectExtractor(url);
            
            // 调用API
            const result = await apiCall('/api/download', 'POST', {
                url,
                extractor: finalExtractor,
                quality: quality || 'best',
                format: format || 'mp4'
            });
            
            if (result.success) {
                showNotification('下载已开始', `${data.title || '视频'}正在下载...`);
            }
            
            return result;
        },
        
        // 获取下载列表
        GET_DOWNLOADS: async () => {
            return await apiCall('/api/downloads');
        },
        
        // 扩展列表
        GET_EXTENSIONS: async () => {
            return await apiCall('/api/extensions');
        },
        
        // 连接检查
        CHECK_CONNECTION: async () => {
            const result = await apiCall('/api/status');
            state.isConnected = result.success;
            return { connected: result.success };
        }
    };
    
    const handler = handlers[message.type];
    if (handler) {
        handler(message.data).then(sendResponse);
        return true; // 异步响应
    }
    return false;
});

// 右键菜单
chrome.runtime.onInstalled.addListener(() => {
    chrome.contextMenus.create({
        id: 'openfetch-download',
        title: '使用OpenFetch下载',
        contexts: ['video', 'audio', 'image']
    });
    
    chrome.contextMenus.create({
        id: 'openfetch-download-page',
        title: '下载页面所有媒体',
        contexts: ['page']
    });
});

chrome.contextMenus.onClicked.addListener(async (info, tab) => {
    if (info.menuItemId === 'openfetch-download') {
        const url = info.srcUrl || info.linkUrl || info.pageUrl;
        const extractor = detectExtractor(url);
        
        await apiCall('/api/download', 'POST', { url, extractor });
        showNotification('下载已开始', url);
    } else if (info.menuItemId === 'openfetch-download-page') {
        // 获取页面所有媒体
        chrome.tabs.sendMessage(tab.id, { type: 'GET_ALL_MEDIA' });
    }
});

// 通知
function showNotification(title, message) {
    chrome.notifications.create({
        type: 'basic',
        iconUrl: 'icons/icon128.png',
        title,
        message
    });
}

// 定时清理缓存
setInterval(() => {
    const now = Date.now();
    for (const [url, data] of state.mediaCache.entries()) {
        if (now - data.detectedAt > 3600000) { // 1小时
            state.mediaCache.delete(url);
        }
    }
}, 300000); // 每5分钟检查
