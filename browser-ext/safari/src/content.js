// OpenFetch - Safari iOS 内容脚本
// 针对iOS Safari优化，简化版

(function() {
    'use strict';

    // 平台检测
    const PLATFORMS = {
        bilibili: {
            match: () => /bilibili\.com\/video/.test(location.href),
            extract: () => {
                const bv = location.href.match(/BV([a-zA-Z0-9]+)/);
                return {
                    type: 'video',
                    platform: 'bilibili',
                    url: location.href,
                    bvid: bv ? 'BV' + bv[1] : null,
                    title: document.title.replace('_哔哩哔哩 (゜-゜)つロ 干杯~-bilibili', '').trim()
                };
            }
        },
        bilibiliLive: {
            match: () => /live\.bilibili\.com/.test(location.href),
            extract: () => ({
                type: 'live',
                platform: 'bilibili',
                url: location.href,
                title: document.title.replace('_哔哩哔哩直播', '').trim()
            })
        },
        douyin: {
            match: () => /douyin\.com\/video/.test(location.href) || /v\.douyin\.com/.test(location.href),
            extract: () => ({
                type: 'video',
                platform: 'douyin',
                url: location.href,
                title: document.title.replace('_抖音', '').trim()
            })
        },
        youtube: {
            match: () => /youtube\.com\/watch/.test(location.href),
            extract: () => ({
                type: 'video',
                platform: 'youtube',
                url: location.href,
                videoId: new URLSearchParams(location.search).get('v'),
                title: document.title.replace(' - YouTube', '').trim()
            })
        }
    };

    let currentInfo = null;

    // 检测内容
    function detect() {
        for (const [, detector] of Object.entries(PLATFORMS)) {
            if (detector.match()) {
                try {
                    currentInfo = detector.extract();
                    notifyBackground(currentInfo);
                    return true;
                } catch (e) {
                    console.error('OpenFetch: 检测失败', e);
                }
            }
        }
        return false;
    }

    // 通知后台
    function notifyBackground(info) {
        browser.runtime.sendMessage({
            action: 'contentDetected',
            data: info
        }).catch(() => {});
    }

    // 监听消息
    browser.runtime.onMessage.addListener((message, sender, sendResponse) => {
        if (message.action === 'getCurrentInfo') {
            sendResponse(currentInfo);
        }
        return true;
    });

    // 页面加载后检测
    if (document.readyState === 'complete') {
        setTimeout(detect, 1000);
    } else {
        window.addEventListener('load', () => setTimeout(detect, 1000));
    }

    // URL变化时重新检测
    let lastUrl = location.href;
    setInterval(() => {
        if (location.href !== lastUrl) {
            lastUrl = location.href;
            currentInfo = null;
            setTimeout(detect, 1000);
        }
    }, 2000);

    console.log('OpenFetch Safari: 内容脚本已加载');
})();
