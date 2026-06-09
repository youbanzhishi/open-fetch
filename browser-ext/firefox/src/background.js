// OpenFetch - 后台服务脚本
// 负责管理状态、调用CLI、同步数据

const CONFIG = {
    apiEndpoint: 'http://localhost:8080',
    syncInterval: 30000,
    maxQueueSize: 100
};

// 状态管理
let downloadQueue = [];
let currentDownloads = new Map();
let settings = {
    autoDetect: true,
    showNotification: true,
    downloadPath: '',
    maxConcurrent: 3
};

// 初始化
async function init() {
    // 加载设置
    await loadSettings();
    
    // 启动同步
    startSync();
    
    console.log('OpenFetch: 后台服务已启动');
}

// 加载设置
async function loadSettings() {
    try {
        const result = await browser.storage.local.get('settings');
        if (result.settings) {
            settings = { ...settings, ...result.settings };
        }
    } catch (e) {
        console.error('OpenFetch: 加载设置失败', e);
    }
}

// 保存设置
async function saveSettings() {
    try {
        await browser.storage.local.set({ settings });
    } catch (e) {
        console.error('OpenFetch: 保存设置失败', e);
    }
}

// 启动数据同步
function startSync() {
    setInterval(syncData, CONFIG.syncInterval);
    syncData();
}

// 同步数据到CLI
async function syncData() {
    if (downloadQueue.length === 0) return;
    
    try {
        const response = await fetch(`${CONFIG.apiEndpoint}/api/sync`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                action: 'push',
                queue: downloadQueue.slice(0, 10)
            })
        });
        
        if (response.ok) {
            const result = await response.json();
            if (result.processed) {
                downloadQueue = downloadQueue.slice(result.processed);
            }
        }
    } catch (e) {
        // CLI未运行，忽略
    }
}

// 从CLI拉取数据
async function pullData() {
    try {
        const response = await fetch(`${CONFIG.apiEndpoint}/api/sync`, {
            method: 'GET'
        });
        
        if (response.ok) {
            const result = await response.json();
            return result.queue || [];
        }
    } catch (e) {
        return [];
    }
    return [];
}

// 添加到下载队列
async function addToQueue(info) {
    const item = {
        id: generateId(),
        url: info.url,
        platform: info.platform,
        type: info.type,
        title: info.title,
        timestamp: Date.now(),
        status: 'pending'
    };
    
    downloadQueue.push(item);
    
    // 限制队列大小
    if (downloadQueue.length > CONFIG.maxQueueSize) {
        downloadQueue = downloadQueue.slice(-CONFIG.maxQueueSize);
    }
    
    // 保存到存储
    await browser.storage.local.set({ queue: downloadQueue });
    
    // 尝试调用CLI
    await callCLI(item);
    
    // 显示通知
    if (settings.showNotification) {
        showNotification('已添加到下载队列', item.title);
    }
    
    return item;
}

// 调用CLI
async function callCLI(item) {
    try {
        const response = await fetch(`${CONFIG.apiEndpoint}/api/download`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(item)
        });
        
        if (response.ok) {
            const result = await response.json();
            item.status = 'downloading';
            item.taskId = result.taskId;
            currentDownloads.set(item.id, item);
        }
    } catch (e) {
        // CLI未运行，保留在队列中
        console.log('OpenFetch: CLI未运行，下载将在下次同步时处理');
    }
}

// 生成ID
function generateId() {
    return `of_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
}

// 显示通知
function showNotification(title, body) {
    browser.notifications.create({
        type: 'basic',
        iconUrl: browser.runtime.getURL('assets/icon-48.png'),
        title: title,
        message: body
    });
}

// 消息监听
browser.runtime.onMessage.addListener((message, sender, sendResponse) => {
    handleMessage(message).then(sendResponse);
    return true;
});

// 处理消息
async function handleMessage(message) {
    switch (message.action) {
        case 'contentDetected':
            // 收到内容检测结果
            if (settings.autoDetect) {
                await addToQueue(message.data);
            }
            return { success: true, autoAdded: settings.autoDetect };
            
        case 'manualDownload':
            // 手动下载
            return await addToQueue(message.data);
            
        case 'getQueue':
            // 获取下载队列
            return { queue: downloadQueue };
            
        case 'getSettings':
            // 获取设置
            return settings;
            
        case 'updateSettings':
            // 更新设置
            settings = { ...settings, ...message.settings };
            await saveSettings();
            return { success: true };
            
        case 'clearQueue':
            // 清空队列
            downloadQueue = [];
            await browser.storage.local.set({ queue: [] });
            return { success: true };
            
        case 'checkCLI':
            // 检查CLI状态
            try {
                const response = await fetch(`${CONFIG.apiEndpoint}/api/health`);
                return { online: response.ok };
            } catch (e) {
                return { online: false };
            }
            
        default:
            return { error: 'Unknown action' };
    }
}

// 启动初始化
init();
