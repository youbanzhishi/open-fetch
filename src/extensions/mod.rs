//! OpenFetch Extensions Registry v1.0
//! 所有扩展统一注册入口

use serde::{Deserialize, Serialize};

/// 扩展元信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub platforms: Vec<String>,
    pub enabled: bool,
    pub ai_capable: bool,
}

/// 所有内置扩展
pub fn get_all_extensions() -> Vec<ExtensionInfo> {
    vec![
        ExtensionInfo {
            id: "universal".to_string(),
            name: "通用下载器".to_string(),
            version: "1.0.0".to_string(),
            description: "基于yt-dlp，支持50+平台通用下载".to_string(),
            platforms: vec!["video".to_string(), "audio".to_string(), "image".to_string()],
            enabled: true,
            ai_capable: true,
        },
        ExtensionInfo {
            id: "bilibili".to_string(),
            name: "B站下载器".to_string(),
            version: "1.0.0".to_string(),
            description: "B站视频/番剧/直播/漫画下载，支持弹幕字幕封面".to_string(),
            platforms: vec!["video".to_string(), "live".to_string(), "comic".to_string()],
            enabled: true,
            ai_capable: true,
        },
        ExtensionInfo {
            id: "youtube".to_string(),
            name: "YouTube下载器".to_string(),
            version: "1.0.0".to_string(),
            description: "YouTube视频/Shorts/音乐下载，支持4K/8K".to_string(),
            platforms: vec!["video".to_string(), "music".to_string(), "live".to_string()],
            enabled: true,
            ai_capable: true,
        },
        ExtensionInfo {
            id: "douyin".to_string(),
            name: "抖音/TikTok下载器".to_string(),
            version: "1.0.0".to_string(),
            description: "抖音/TikTok无水印下载，支持作者批量".to_string(),
            platforms: vec!["video".to_string(), "short_video".to_string()],
            enabled: true,
            ai_capable: true,
        },
        ExtensionInfo {
            id: "weibo".to_string(),
            name: "微博下载器".to_string(),
            version: "1.0.0".to_string(),
            description: "微博/秒拍视频下载".to_string(),
            platforms: vec!["video".to_string()],
            enabled: true,
            ai_capable: true,
        },
        ExtensionInfo {
            id: "twitter".to_string(),
            name: "Twitter/X下载器".to_string(),
            version: "1.0.0".to_string(),
            description: "Twitter/X视频/图片下载".to_string(),
            platforms: vec!["video".to_string(), "image".to_string()],
            enabled: true,
            ai_capable: true,
        },
        ExtensionInfo {
            id: "instagram".to_string(),
            name: "Instagram下载器".to_string(),
            version: "1.0.0".to_string(),
            description: "Instagram图片/视频/Reels/Stories下载".to_string(),
            platforms: vec!["image".to_string(), "video".to_string()],
            enabled: true,
            ai_capable: true,
        },
        ExtensionInfo {
            id: "xiaohongshu".to_string(),
            name: "小红书下载器".to_string(),
            version: "1.0.0".to_string(),
            description: "小红书笔记/视频/图文下载".to_string(),
            platforms: vec!["video".to_string(), "image".to_string(), "article".to_string()],
            enabled: true,
            ai_capable: true,
        },
        ExtensionInfo {
            id: "zhihu".to_string(),
            name: "知乎下载器".to_string(),
            version: "1.0.0".to_string(),
            description: "知乎文章/视频/问答下载".to_string(),
            platforms: vec!["article".to_string(), "video".to_string()],
            enabled: true,
            ai_capable: true,
        },
        ExtensionInfo {
            id: "kuishou".to_string(),
            name: "快手下载器".to_string(),
            version: "1.0.0".to_string(),
            description: "快手视频/直播下载".to_string(),
            platforms: vec!["video".to_string(), "live".to_string()],
            enabled: true,
            ai_capable: true,
        },
        ExtensionInfo {
            id: "live".to_string(),
            name: "直播录制器".to_string(),
            version: "1.0.0".to_string(),
            description: "多平台直播录制，B站/抖音/斗鱼/虎牙/Twitch".to_string(),
            platforms: vec!["live".to_string()],
            enabled: true,
            ai_capable: true,
        },
        ExtensionInfo {
            id: "compress".to_string(),
            name: "音视频压缩器".to_string(),
            version: "1.0.0".to_string(),
            description: "FFmpeg驱动，高质量压缩，支持批量".to_string(),
            platforms: vec!["tool".to_string()],
            enabled: true,
            ai_capable: true,
        },
    ]
}

/// 扩展ID到文件路径的映射
pub fn get_extension_script(id: &str) -> Option<&'static str> {
    match id {
        "universal" => Some("src/extensions/universal/downloader.py"),
        "bilibili" => Some("src/extensions/bilibili/downloader.py"),
        "youtube" => Some("src/extensions/youtube/downloader.py"),
        "douyin" => Some("src/extensions/douyin/downloader.py"),
        "weibo" => Some("src/extensions/weibo/downloader.py"),
        "twitter" => Some("src/extensions/twitter/downloader.py"),
        "instagram" => Some("src/extensions/instagram/downloader.py"),
        "xiaohongshu" => Some("src/extensions/xiaohongshu/downloader.py"),
        "zhihu" => Some("src/extensions/zhihu/downloader.py"),
        "kuishou" => Some("src/extensions/kuishou/downloader.py"),
        "live" => Some("src/extensions/live/downloader.py"),
        "compress" => Some("src/extensions/compress/compressor.py"),
        _ => None,
    }
}

/// 自动检测URL对应的扩展
pub fn detect_extractor(url: &str) -> &'static str {
    let url_lower = url.to_lowercase();
    
    if url_lower.contains("bilibili.com") || url_lower.contains("b23.tv") {
        "bilibili"
    } else if url_lower.contains("youtube.com") || url_lower.contains("youtu.be") {
        "youtube"
    } else if url_lower.contains("douyin.com") || url_lower.contains("iesdouyin.com") || url_lower.contains("tiktok.com") {
        "douyin"
    } else if url_lower.contains("weibo.com") || url_lower.contains("weibo.cn") {
        "weibo"
    } else if url_lower.contains("twitter.com") || url_lower.contains("x.com") {
        "twitter"
    } else if url_lower.contains("instagram.com") {
        "instagram"
    } else if url_lower.contains("xiaohongshu.com") || url_lower.contains("xhs.co") {
        "xiaohongshu"
    } else if url_lower.contains("zhihu.com") {
        "zhihu"
    } else if url_lower.contains("kuaishou.com") || url_lower.contains("ksyun.com") {
        "kuishou"
    } else if url_lower.contains("twitch.tv") {
        "live"
    } else if url_lower.contains("douyu.com") {
        "live"
    } else if url_lower.contains("huya.com") {
        "live"
    } else {
        "universal"
    }
}
