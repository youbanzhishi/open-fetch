//! OpenFetch Extensions Registry
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
}

/// 扩展能力
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionCapability {
    pub method: String,
    pub description: String,
    pub params: Vec<ParamInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamInfo {
    pub name: String,
    pub param_type: String,
    pub required: bool,
    pub description: String,
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
        },
        ExtensionInfo {
            id: "bilibili".to_string(),
            name: "B站下载器".to_string(),
            version: "1.0.0".to_string(),
            description: "B站视频/番剧/直播/漫画下载，支持弹幕字幕封面".to_string(),
            platforms: vec!["video".to_string(), "live".to_string(), "comic".to_string()],
            enabled: true,
        },
        ExtensionInfo {
            id: "youtube".to_string(),
            name: "YouTube下载器".to_string(),
            version: "1.0.0".to_string(),
            description: "YouTube视频/Shorts/音乐下载，支持4K/8K".to_string(),
            platforms: vec!["video".to_string(), "music".to_string(), "live".to_string()],
            enabled: true,
        },
        ExtensionInfo {
            id: "douyin".to_string(),
            name: "抖音/TikTok下载器".to_string(),
            version: "1.0.0".to_string(),
            description: "抖音/TikTok无水印下载，支持作者批量".to_string(),
            platforms: vec!["video".to_string(), "short_video".to_string()],
            enabled: true,
        },
        ExtensionInfo {
            id: "weibo".to_string(),
            name: "微博下载器".to_string(),
            version: "1.0.0".to_string(),
            description: "微博/秒拍视频下载".to_string(),
            platforms: vec!["video".to_string()],
            enabled: true,
        },
        ExtensionInfo {
            id: "live".to_string(),
            name: "直播录制器".to_string(),
            version: "1.0.0".to_string(),
            description: "多平台直播录制，B站/抖音/斗鱼/虎牙/Twitch".to_string(),
            platforms: vec!["live".to_string()],
            enabled: true,
        },
        ExtensionInfo {
            id: "compress".to_string(),
            name: "音视频压缩器".to_string(),
            version: "1.0.0".to_string(),
            description: "FFmpeg驱动，高质量压缩，支持批量".to_string(),
            platforms: vec!["tool".to_string()],
            enabled: true,
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
        "live" => Some("src/extensions/live/downloader.py"),
        "compress" => Some("src/extensions/compress/compressor.py"),
        _ => None,
    }
}

pub use bilibili_extension::*;
pub use youtube_extension::*;
pub use douyin_extension::*;
pub use weibo_extension::*;
pub use live_extension::*;
pub use compress_extension::*;
