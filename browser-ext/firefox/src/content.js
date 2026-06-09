// OpenFetch - 内容脚本
// 负责检测页面内容并提取下载资源

(function() {
    'use strict';

    // 配置
    const CONFIG = {
        detectInterval: 2000,
        maxRetries: 5
    };

    // 平台检测器
    const PLATFORM_DETECTORS = {
        // B站视频检测
        bilibili: {
            match: () => /bilibili\.com\/video/.test(window.location.href),
            extract: () => {
                const bvMatch = window.location.href.match(/BV([a-zA-Z0-9]+)/);
                return {
                    type: 'video',
                    platform: 'bilibili',
                    url: window.location.href,
                    bvid: bvMatch ? 'BV' + bvMatch[1] : null,
                    title: document.title.replace('_哔哩哔哩 (゜-゜)つロ 干杯~-bilibili', '').trim()
                };
            }
        },

        // B站直播检测
        bilibiliLive: {
            match: () => /live\.bilibili\.com/.test(window.location.href),
            extract: () => {
                const roomMatch = window.location.href.match(/live\.bilibili\.com\/(\d+)/);
                return {
                    type: 'live',
                    platform: 'bilibili',
                    url: window.location.href,
                    roomId: roomMatch ? roomMatch[1] : null,
                    title: document.title.replace('_哔哩哔哩直播', '').trim()
                };
            }
        },

        // 抖音视频检测
        douyin: {
            match: () => /douyin\.com\/video/.test(window.location.href) || /v\.douyin\.com/.test(window.location.href),
            extract: () => ({
                type: 'video',
                platform: 'douyin',
                url: window.location.href,
                title: document.title.replace('_抖音', '').trim()
            })
        },

        // YouTube视频检测
        youtube: {
            match: () => /youtube\.com\/watch/.test(window.location.href),
            extract: () => {
                const videoId = new URLSearchParams(window.location.search).get('v');
                return {
                    type: 'video',
                    platform: 'youtube',
                    url: window.location.href,
                    videoId: videoId,
                    title: document.title.replace(' - YouTube', '').trim()
                };
            }
        },

        // 微博视频检测
        weibo: {
            match: () => /weibo\.com\/tv/.test(window.location.href),
            extract: () => ({
                type: 'video',
                platform: 'weibo',
                url: window.location.href,
                title: document.title.replace('_新浪微博', '').trim()
            })
        },

        // 图片检测
        image: {
            match: () => {
                const images = document.querySelectorAll('img[src]');
                return images.length >= 3;
            },
            extract: () => {
                const images = Array.from(document.querySelectorAll('img[src]'))
                    .filter(img => img.naturalWidth > 200 && img.naturalHeight > 200)
                    .map(img => ({
                        url: img.src,
                        width: img.naturalWidth,
                        height: img.naturalHeight
                    }));
                return {
                    type: 'images',
                    platform: 'generic',
                    count: images.length,
                    images: images.slice(0, 20) // 最多20张
                };
            }
        }
    };

    // 当前检测状态
    let currentInfo = null;
    let retryCount = 0;

    // 检测页面内容
    function detectContent() {
        for (const [name, detector] of Object.entries(PLATFORM_DETECTORS)) {
            if (detector.match()) {
                try {
                    const info = detector.extract();
                    if (info) {
                        currentInfo = info;
                        notifyBackground(info);
                        return true;
                    }
                } catch (e) {
                    console.error('OpenFetch: 检测失败', e);
                }
            }
        }
        
        // 如果没检测到特定平台，尝试通用图片检测
        if (!currentInfo && retryCount < CONFIG.maxRetries) {
            retryCount++;
            setTimeout(detectContent, CONFIG.detectInterval);
        }
        return false;
    }

    // 通知后台
    function notifyBackground(info) {
        browser.runtime.sendMessage({
            action: 'contentDetected',
            data: info
        }).catch(() => {
            // 忽略错误
        });
    }

    // 监听来自popup的请求
    browser.runtime.onMessage.addListener((message, sender, sendResponse) => {
        if (message.action === 'getCurrentInfo') {
            sendResponse(currentInfo);
        }
        return true;
    });

    // 页面加载后开始检测
    if (document.readyState === 'complete') {
        detectContent();
    } else {
        window.addEventListener('load', detectContent);
    }

    // URL变化时重新检测
    let lastUrl = window.location.href;
    new MutationObserver(() => {
        if (window.location.href !== lastUrl) {
            lastUrl = window.location.href;
            currentInfo = null;
            retryCount = 0;
            detectContent();
        }
    }).observe(document.body, { childList: true, subtree: true });

    console.log('OpenFetch: 内容脚本已加载');
})();
