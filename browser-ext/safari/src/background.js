// OpenFetch - Safari iOS 后台脚本
// 针对iOS Safari优化

const CONFIG = {
    apiEndpoint: 'http://localhost:8080',
    storageKey: 'openfetch_data'
};

// 状态
let downloadQueue = [];
let settings = {
    autoDetect: true,
    showNotification: true
};

// 初始化
async function init() {
    await loadData();
    setupMessageHandler();
    console.log('OpenFetch Safari: 已初始化');
}

// 加载数据
async function loadData() {
    try {
        const result = await browser.storage.local.get(STORAGE_KEY);
        if (result[STORAGE_KEY]) {
            const data = JSON.parse(result[STORAGE_KEY]);
            downloadQueue = data.queue || [];
            settings = { ...settings, ...data.settings };
        }
    } catch (e) {
        console.error('加载数据失败', e);
    }
}

// 保存数据
async function saveData() {
    try {
        await browser.storage.local.set({
            [STORAGE_KEY]: JSON.stringify({
                queue: downloadQueue,
                settings: settings
            })
        });
    } catch (e) {
        console.error('保存数据失败', e);
    }
}

// 消息处理
function setupMessageHandler() {
    browser.runtime.onMessage.addListener((message, sender, sendResponse) => {
        handleMessage(message).then(sendResponse);
        return true;
    });
}

// 处理消息
async function handleMessage(message) {
    switch (message.action) {
        case 'contentDetected':
            if (settings.autoDetect) {
                await addToQueue(message.data);
            }
            return { success: true, autoAdded: settings.autoDetect };
            
        case 'manualDownload':
            return await addToQueue(message.data);
            
        case 'getQueue':
            return { queue: downloadQueue };
            
        case 'getSettings':
            return settings;
            
        case 'updateSettings':
            settings = { ...settings, ...message.settings };
            await saveData();
            return { success: true };
            
        case 'syncWithCLI':
            return await syncWithCLI();
            
        default:
            return { error: 'Unknown action' };
    }
}

// 添加到队列
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
    await saveData();
    
    // 尝试同步到CLI
    await syncWithCLI();
    
    return item;
}

// 与CLI同步
async function syncWithCLI() {
    if (downloadQueue.length === 0) return { synced: 0 };
    
    try {
        const response = await fetch(`${CONFIG.apiEndpoint}/api/sync`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ queue: downloadQueue })
        });
        
        if (response.ok) {
            const result = await response.json();
            if (result.processed) {
                downloadQueue = downloadQueue.slice(result.processed);
                await saveData();
            }
            return { synced: result.processed || 0, online: true };
        }
    } catch (e) {
        return { synced: 0, online: false };
    }
    
    return { synced: 0, online: false };
}

// 生成ID
function generateId() {
    return `of_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
}

// 启动
init();
