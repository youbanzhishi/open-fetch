/**
 * OpenFetch Content Script - 平台检测与下载触发
 * 自动检测页面上的媒体资源并提供下载选项
 */

(function() {
    'use strict';
    
    const PLATFORMS = {
        bilibili: {
            hosts: ['bilibili.com', 'bilibili.co'],
            selectors: {
                video: '.video-page-player video, .bp-video-player video',
                title: '.video-title h1, .video-info-container h1',
                cover: '.video-cover img, .cover-container img'
            },
            extractor: 'bilibili'
        },
        youtube: {
            hosts: ['youtube.com', 'youtu.be'],
            selectors: {
                video: '#movie_player video, ytd-player video',
                title: '#title h1 yt-formatted-string, .ytd-video-primary-info-renderer h1'
            },
            extractor: 'youtube'
        },
        douyin: {
            hosts: ['douyin.com', 'iesdouyin.com', 'v.douyin.com'],
            selectors: {
                video: 'video[src*="douyin"]',
                title: '.video-title'
            },
            extractor: 'douyin'
        },
        weibo: {
            hosts: ['weibo.com', 'weibo.cn'],
            selectors: {
                video: '.player_video video, video[node-type="video"]'
            },
            extractor: 'weibo'
        },
        twitch: {
            hosts: ['twitch.tv'],
            selectors: {
                video: '.video-player video, .player video'
            },
            extractor: 'twitch'
        }
    };
    
    class MediaDetector {
        constructor() {
            this.currentPlatform = null;
            this.mediaInfo = null;
            this.init();
        }
        
        init() {
            this.detectPlatform();
            this.attachListeners();
            
            // 检测页面变化（SPA支持）
            const observer = new MutationObserver(() => {
                this.detectMedia();
            });
            observer.observe(document.body, { childList: true, subtree: true });
        }
        
        detectPlatform() {
            const hostname = window.location.hostname;
            
            for (const [name, platform] of Object.entries(PLATFORMS)) {
                if (platform.hosts.some(host => hostname.includes(host))) {
                    this.currentPlatform = { name, ...platform };
                    break;
                }
            }
        }
        
        detectMedia() {
            if (!this.currentPlatform) return;
            
            const platform = this.currentPlatform;
            let video = null;
            let title = null;
            
            // 尝试多个选择器
            for (const selector of Object.values(platform.selectors || {})) {
                video = document.querySelector(selector);
                if (video && video.src) break;
            }
            
            // 获取标题
            const titleEl = document.querySelector(platform.selectors?.title);
            title = titleEl?.textContent?.trim() || document.title;
            
            if (video && video.src) {
                this.mediaInfo = {
                    url: video.src || video.currentSrc,
                    title: title,
                    platform: platform.name,
                    extractor: platform.extractor,
                    timestamp: Date.now()
                };
                
                // 通知background
                chrome.runtime.sendMessage({
                    type: 'MEDIA_DETECTED',
                    data: this.mediaInfo
                });
            }
        }
        
        attachListeners() {
            // 监听视频加载
            document.addEventListener('loadedmetadata', () => {
                this.detectMedia();
            });
            
            // 监听URL变化
            let lastUrl = location.href;
            new MutationObserver(() => {
                if (location.href !== lastUrl) {
                    lastUrl = location.href;
                    setTimeout(() => this.detectMedia(), 1000);
                }
            }).observe(document, { subtree: true, childList: true });
        }
    }
    
    // 启动检测器
    if (document.readyState === 'complete') {
        new MediaDetector();
    } else {
        window.addEventListener('load', () => new MediaDetector());
    }
    
    // 暴露全局接口供popup调用
    window.OpenFetch = {
        getCurrentMedia: () => mediaInfo,
        getPlatform: () => currentPlatform?.name,
        download: (options) => {
            chrome.runtime.sendMessage({
                type: 'DOWNLOAD_REQUEST',
                data: { ...options, url: window.location.href }
            });
        }
    };
})();
