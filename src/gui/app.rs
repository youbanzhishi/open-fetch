//! GUI主应用
//! 跨平台桌面下载管理器

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};

/// 下载任务状态
#[derive(Debug, Clone)]
pub enum DownloadStatus {
    Pending,
    Downloading { progress: f32, speed: String },
    Completed { path: String },
    Failed { error: String },
    Cancelled,
}

/// 下载任务
#[derive(Debug, Clone)]
pub struct DownloadTask {
    pub id: String,
    pub url: String,
    pub title: String,
    pub platform: String,
    pub status: DownloadStatus,
    pub created_at: DateTime<Utc>,
    pub file_size: Option<u64>,
    pub downloaded: u64,
}

impl DownloadTask {
    pub fn new(url: String, platform: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            url,
            title: String::new(),
            platform,
            status: DownloadStatus::Pending,
            created_at: Utc::now(),
            file_size: None,
            downloaded: 0,
        }
    }
}

/// 全局应用状态
pub struct AppState {
    pub tasks: HashMap<String, DownloadTask>,
    pub settings: AppSettings,
    pub is_server_running: bool,
    pub server_port: u16,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            tasks: HashMap::new(),
            settings: AppSettings::default(),
            is_server_running: false,
            server_port: 8080,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppSettings {
    pub download_path: String,
    pub max_concurrent: usize,
    pub auto_server: bool,
    pub notifications: bool,
    pub theme: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            download_path: directories::UserDirs::new()
                .and_then(|d| d.download_dir().map(|p| p.to_path_buf()))
                .unwrap_or_else(|| std::path::PathBuf::from("./downloads"))
                .to_string_lossy().to_string(),
            max_concurrent: 3,
            auto_server: true,
            notifications: true,
            theme: "dark".to_string(),
        }
    }
}

/// 平台信息
pub struct Platform {
    pub id: &'static str,
    pub name: &'static str,
    pub icon: &'static str,
    pub color: &'static str,
}

pub fn get_all_platforms() -> Vec<Platform> {
    vec![
        Platform { id: "auto", name: "自动检测", icon: "🔍", color: "#888" },
        Platform { id: "bilibili", name: "哔哩哔哩", icon: "🟢", color: "#00A1D6" },
        Platform { id: "youtube", name: "YouTube", icon: "🔴", color: "#FF0000" },
        Platform { id: "douyin", name: "抖音/TikTok", icon: "🎵", color: "#000" },
        Platform { id: "twitter", name: "Twitter/X", icon: "🐦", color: "#1DA1F2" },
        Platform { id: "instagram", name: "Instagram", icon: "📸", color: "#E4405F" },
        Platform { id: "xiaohongshu", name: "小红书", icon: "📖", color: "#FF2442" },
        Platform { id: "weibo", name: "微博", icon: "📱", color: "#E6162D" },
        Platform { id: "zhihu", name: "知乎", icon: "💬", color: "#0066FF" },
        Platform { id: "kuishou", name: "快手", icon: "📺", color: "#FF4906" },
    ]
}
