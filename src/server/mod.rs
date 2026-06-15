//! OpenFetch HTTP API服务器
//! 基于axum的RESTful API

use axum::{
    Router,
    routing::{get, post, delete},
    extract::{Path, Query, State},
    response::Json,
    middleware::from_fn_with_state,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};
use uuid::Uuid;

// ============= Types =============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadRequest {
    pub url: String,
    pub extractor: Option<String>,
    pub quality: Option<String>,
    pub format: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadResponse {
    pub success: bool,
    pub task_id: Option<String>,
    pub message: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskInfo {
    pub id: String,
    pub url: String,
    pub title: String,
    pub platform: String,
    pub status: String,
    pub progress: f32,
    pub speed: Option<String>,
    pub error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub output_path: Option<String>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressRequest {
    pub input: String,
    pub output: Option<String>,
    pub crf: Option<u32>,
    pub preset: Option<String>,
    pub format: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchRequest {
    pub urls: Vec<DownloadRequest>,
    pub concurrent: Option<usize>,
}

// ============= App State =============

#[derive(Clone)]
pub struct ServerState {
    pub tasks: Arc<Mutex<HashMap<String, TaskInfo>>>,
    pub extensions: Vec<ExtensionInfo>,
}

impl ServerState {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(HashMap::new())),
            extensions: get_default_extensions(),
        }
    }
    
    pub fn add_task(&self, url: String, extractor: String) -> String {
        let task_id = Uuid::new_v4().to_string();
        let mut task = TaskInfo {
            id: task_id.clone(),
            url: url.clone(),
            title: String::new(),
            platform: extractor.clone(),
            status: "pending".to_string(),
            progress: 0.0,
            speed: None,
            error: None,
            created_at: Utc::now(),
            output_path: None,
        };
        
        // 检测平台
        let url_lower = url.to_lowercase();
        if url_lower.contains("bilibili") {
            task.platform = "bilibili".to_string();
        } else if url_lower.contains("youtube") {
            task.platform = "youtube".to_string();
        } else if url_lower.contains("douyin") || url_lower.contains("tiktok") {
            task.platform = "douyin".to_string();
        }
        
        let mut tasks = self.tasks.lock().unwrap();
        tasks.insert(task_id.clone(), task);
        
        task_id
    }
    
    pub fn get_task(&self, task_id: &str) -> Option<TaskInfo> {
        let tasks = self.tasks.lock().unwrap();
        tasks.get(task_id).cloned()
    }
    
    pub fn list_tasks(&self) -> Vec<TaskInfo> {
        let tasks = self.tasks.lock().unwrap();
        tasks.values().cloned().collect()
    }
    
    pub fn remove_task(&self, task_id: &str) -> bool {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.remove(task_id).is_some()
    }
}

fn get_default_extensions() -> Vec<ExtensionInfo> {
    vec![
        ExtensionInfo { id: "universal".into(), name: "通用下载器".into(), version: "1.0.0".into(), description: "支持50+平台".into(), platforms: vec!["video".into()], enabled: true, ai_capable: true },
        ExtensionInfo { id: "bilibili".into(), name: "B站下载器".into(), version: "1.0.0".into(), description: "视频/番剧/直播".into(), platforms: vec!["video".into(), "live".into()], enabled: true, ai_capable: true },
        ExtensionInfo { id: "youtube".into(), name: "YouTube下载器".into(), version: "1.0.0".into(), description: "支持4K/8K".into(), platforms: vec!["video".into()], enabled: true, ai_capable: true },
        ExtensionInfo { id: "douyin".into(), name: "抖音/TikTok".into(), version: "1.0.0".into(), description: "无水印下载".into(), platforms: vec!["video".into()], enabled: true, ai_capable: true },
        ExtensionInfo { id: "twitter".into(), name: "Twitter/X".into(), version: "1.0.0".into(), description: "视频/图片".into(), platforms: vec!["video".into()], enabled: true, ai_capable: true },
        ExtensionInfo { id: "instagram".into(), name: "Instagram".into(), version: "1.0.0".into(), description: "图片/视频/Reels".into(), platforms: vec!["image".into(), "video".into()], enabled: true, ai_capable: true },
        ExtensionInfo { id: "xiaohongshu".into(), name: "小红书".into(), version: "1.0.0".into(), description: "笔记/视频".into(), platforms: vec!["video".into(), "image".into()], enabled: true, ai_capable: true },
        ExtensionInfo { id: "live".into(), name: "直播录制".into(), version: "1.0.0".into(), description: "多平台直播".into(), platforms: vec!["live".into()], enabled: true, ai_capable: true },
        ExtensionInfo { id: "compress".into(), name: "音视频压缩".into(), version: "1.0.0".into(), description: "FFmpeg压缩".into(), platforms: vec!["tool".into()], enabled: true, ai_capable: true },
    ]
}

// ============= Routes =============

// 健康检查
async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "success": true,
        "service": "OpenFetch",
        "version": "0.8.0"
    }))
}

// 状态检查
async fn status_check(State(state): State<ServerState>) -> Json<serde_json::Value> {
    let tasks = state.list_tasks();
    let active = tasks.iter().filter(|t| t.status == "downloading").count();
    
    Json(serde_json::json!({
        "success": true,
        "status": "running",
        "tasks_total": tasks.len(),
        "tasks_active": active,
        "server": "OpenFetch HTTP API v0.8.0"
    }))
}

// 获取扩展列表
async fn get_extensions(State(state): State<ServerState>) -> Json<Vec<ExtensionInfo>> {
    Json(state.extensions.clone())
}

// 创建下载任务
async fn create_download(
    State(state): State<ServerState>,
    Json(req): Json<DownloadRequest>,
) -> Json<DownloadResponse> {
    if req.url.is_empty() {
        return Json(DownloadResponse {
            success: false,
            task_id: None,
            message: None,
            error: Some("URL不能为空".to_string()),
        });
    }
    
    let extractor = req.extractor.unwrap_or_else(|| "auto".to_string());
    let task_id = state.add_task(req.url.clone(), extractor.clone());
    
    // TODO: 异步启动下载
    
    Json(DownloadResponse {
        success: true,
        task_id: Some(task_id),
        message: Some("任务已创建".to_string()),
        error: None,
    })
}

// 获取任务列表
async fn list_tasks(State(state): State<ServerState>) -> Json<Vec<TaskInfo>> {
    Json(state.list_tasks())
}

// 获取单个任务
async fn get_task(
    State(state): State<ServerState>,
    Path(task_id): Path<String>,
) -> Json<TaskInfo> {
    match state.get_task(&task_id) {
        Some(task) => Json(task),
        None => Json(TaskInfo {
            id: task_id,
            url: String::new(),
            title: String::new(),
            platform: String::new(),
            status: "not_found".to_string(),
            progress: 0.0,
            speed: None,
            error: Some("任务不存在".to_string()),
            created_at: Utc::now(),
            output_path: None,
        }),
    }
}

// 删除任务
async fn delete_task(
    State(state): State<ServerState>,
    Path(task_id): Path<String>,
) -> Json<serde_json::Value> {
    let removed = state.remove_task(&task_id);
    Json(serde_json::json!({
        "success": removed,
        "message": if removed { "任务已删除" } else { "任务不存在" }
    }))
}

// 批量下载
async fn batch_download(
    State(state): State<ServerState>,
    Json(req): Json<BatchRequest>,
) -> Json<Vec<DownloadResponse>> {
    let concurrent = req.concurrent.unwrap_or(3);
    let mut responses = Vec::new();
    
    for (i, download_req) in req.urls.iter().enumerate() {
        if i >= concurrent {
            // 达到并发限制，等待
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        
        let extractor = download_req.extractor.clone().unwrap_or_else(|| "auto".to_string());
        let task_id = state.add_task(download_req.url.clone(), extractor);
        
        responses.push(DownloadResponse {
            success: true,
            task_id: Some(task_id),
            message: Some("任务已创建".to_string()),
            error: None,
        });
    }
    
    Json(responses)
}

// 压缩请求
async fn compress(
    Json(req): Json<CompressRequest>,
) -> Json<serde_json::Value> {
    if req.input.is_empty() {
        return Json(serde_json::json!({
            "success": false,
            "error": "输入文件不能为空"
        }));
    }
    
    // TODO: 调用FFmpeg压缩
    
    Json(serde_json::json!({
        "success": true,
        "message": "压缩任务已创建"
    }))
}

// ============= Server Builder =============

pub fn create_router() -> Router {
    let state = ServerState::new();
    
    Router::new()
        .route("/", get(health_check))
        .route("/api/status", get(status_check))
        .route("/api/extensions", get(get_extensions))
        .route("/api/download", post(create_download))
        .route("/api/tasks", get(list_tasks))
        .route("/api/tasks/:id", get(get_task))
        .route("/api/tasks/:id", delete(delete_task))
        .route("/api/batch", post(batch_download))
        .route("/api/compress", post(compress))
        .with_state(state)
}

pub async fn start_server(port: u16) {
    let app = create_router();
    let addr = format!("0.0.0.0:{}", port);
    
    println!("🚀 OpenFetch HTTP API 服务器启动中...");
    println!("📍 地址: http://{}", addr);
    println!("📖 文档: http://{}/api/docs", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    
    axum::serve(listener, app).await.ok();
}
