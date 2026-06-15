//! 云端API路由
//! RESTful API + Web UI

use axum::{
    extract::{Path, Query, State, WebSocketUpgrade},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{delete, get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{error, info};
use uuid::Uuid;

use crate::cloud::state::{CloudState, CloudTask, TaskStatus};
use crate::cloud::websocket::ws_handler;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self { success: true, data: Some(data), error: None }
    }
    
    pub fn error(msg: &str) -> Self {
        Self { success: false, data: None, error: Some(msg.to_string()) }
    }
}

/// API路由构建
pub fn router(cloud_state: Arc<CloudState>) -> Router {
    Router::new()
        // Web UI
        .route("/", get(ui_index))
        .route("/ui", get(ui_index))
        .route("/ui/tasks", get(ui_tasks_page))
        .route("/ui/settings", get(ui_settings_page))
        // WebSocket
        .route("/ws", get(ws_connect))
        // API - 任务管理
        .route("/api/tasks", get(api_list_tasks))
        .route("/api/tasks", post(api_create_task))
        .route("/api/tasks/:id", get(api_get_task))
        .route("/api/tasks/:id", delete(api_delete_task))
        .route("/api/tasks/:id/cancel", post(api_cancel_task))
        // API - 批量操作
        .route("/api/batch", post(api_batch_create))
        // API - 统计
        .route("/api/stats", get(api_stats))
        // API - 平台信息
        .route("/api/platforms", get(api_platforms))
        // 状态检查
        .route("/api/health", get(api_health))
        .with_state(cloud_state)
}

// ============ Web UI ============

pub async fn ui_index() -> Html<String> {
    Html(include_str!("../web/index.html").to_string())
}

pub async fn ui_tasks_page() -> Html<String> {
    Html(include_str!("../web/tasks.html").to_string())
}

pub async fn ui_settings_page() -> Html<String> {
    Html(include_str!("../web/settings.html").to_string())
}

// ============ WebSocket ============

pub async fn ws_connect(
    ws: WebSocketUpgrade,
    State(state): State<Arc<CloudState>>,
) -> Response {
    ws.on_upgrade(move |socket| ws_handler(socket, state))
}

// ============ API Handlers ============

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub url: String,
    pub platform: Option<String>,
    pub user_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BatchCreateRequest {
    pub urls: Vec<String>,
    pub platform: Option<String>,
    pub user_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Stats {
    pub total: usize,
    pub pending: usize,
    pub downloading: usize,
    pub completed: usize,
    pub failed: usize,
}

pub async fn api_health(State(state): State<Arc<CloudState>>) -> Json<ApiResponse<()>> {
    let tasks = state.list_tasks().await;
    Json(ApiResponse::success(()))
}

pub async fn api_list_tasks(
    State(state): State<Arc<CloudState>>,
    Query(params): Query<ListParams>,
) -> Json<ApiResponse<Vec<CloudTask>>> {
    let tasks = if let Some(user_id) = params.user_id {
        state.list_tasks_by_user(&user_id).await
    } else {
        state.list_tasks().await
    };
    Json(ApiResponse::success(tasks))
}

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub user_id: Option<String>,
}

pub async fn api_create_task(
    State(state): State<Arc<CloudState>>,
    Json(req): Json<CreateTaskRequest>,
) -> Json<ApiResponse<CloudTask>> {
    let user_id = req.user_id.unwrap_or_else(|| Uuid::new_v4().to_string());
    let platform = req.platform.unwrap_or_else(|| detect_platform(&req.url));
    
    info!("📥 创建下载任务: {} @ {}", req.url, platform);
    
    let task = state.create_task(req.url, platform, user_id).await;
    Json(ApiResponse::success(task))
}

pub async fn api_get_task(
    State(state): State<Arc<CloudState>>,
    Path(task_id): Path<String>,
) -> Json<ApiResponse<CloudTask>> {
    match state.get_task(&task_id).await {
        Some(task) => Json(ApiResponse::success(task)),
        None => Json(ApiResponse::error("任务不存在")),
    }
}

pub async fn api_delete_task(
    State(state): State<Arc<CloudState>>,
    Path(task_id): Path<String>,
) -> Json<ApiResponse<()>> {
    if state.remove_task(&task_id).await {
        info!("🗑️ 删除任务: {}", task_id);
        Json(ApiResponse::success(()))
    } else {
        Json(ApiResponse::error("任务不存在"))
    }
}

pub async fn api_cancel_task(
    State(state): State<Arc<CloudState>>,
    Path(task_id): Path<String>,
) -> Json<ApiResponse<CloudTask>> {
    match state.fail_task(&task_id, "用户取消".to_string()).await {
        Some(task) => {
            info!("❌ 取消任务: {}", task_id);
            Json(ApiResponse::success(task))
        }
        None => Json(ApiResponse::error("任务不存在")),
    }
}

pub async fn api_batch_create(
    State(state): State<Arc<CloudState>>,
    Json(req): Json<BatchCreateRequest>,
) -> Json<ApiResponse<Vec<CloudTask>>> {
    let user_id = req.user_id.unwrap_or_else(|| Uuid::new_v4().to_string());
    let platform = req.platform.unwrap_or_else(|| "auto".to_string());
    
    info!("📦 批量创建 {} 个任务", req.urls.len());
    
    let mut tasks = Vec::new();
    for url in req.urls {
        let task = state.create_task(url.clone(), platform.clone(), user_id.clone()).await;
        tasks.push(task);
    }
    
    Json(ApiResponse::success(tasks))
}

pub async fn api_stats(State(state): State<Arc<CloudState>>) -> Json<ApiResponse<Stats>> {
    let tasks = state.list_tasks().await;
    let stats = Stats {
        total: tasks.len(),
        pending: tasks.iter().filter(|t| t.status == TaskStatus::Pending).count(),
        downloading: tasks.iter().filter(|t| t.status == TaskStatus::Downloading).count(),
        completed: tasks.iter().filter(|t| t.status == TaskStatus::Completed).count(),
        failed: tasks.iter().filter(|t| t.status == TaskStatus::Failed).count(),
    };
    Json(ApiResponse::success(stats))
}

pub async fn api_platforms() -> Json<ApiResponse<Vec<PlatformInfo>>> {
    Json(ApiResponse::success(vec![
        PlatformInfo { id: "bilibili", name: "哔哩哔哩", icon: "📺" },
        PlatformInfo { id: "youtube", name: "YouTube", icon: "▶️" },
        PlatformInfo { id: "douyin", name: "抖音", icon: "🎵" },
        PlatformInfo { id: "weibo", name: "微博视频", icon: "🌐" },
        PlatformInfo { id: "ixigua", name: "西瓜视频", icon: "🍉" },
        PlatformInfo { id: "general", name: "通用下载", icon: "⬇️" },
    ]))
}

#[derive(Debug, Serialize)]
pub struct PlatformInfo {
    pub id: String,
    pub name: String,
    pub icon: String,
}

/// 检测视频平台
fn detect_platform(url: &str) -> String {
    if url.contains("bilibili.com") || url.contains("b23.tv") {
        "bilibili".to_string()
    } else if url.contains("youtube.com") || url.contains("youtu.be") {
        "youtube".to_string()
    } else if url.contains("douyin.com") {
        "douyin".to_string()
    } else if url.contains("weibo.com") {
        "weibo".to_string()
    } else if url.contains("ixigua.com") {
        "ixigua".to_string()
    } else {
        "general".to_string()
    }
}
