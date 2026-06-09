//! HTTP API 服务器
//! 为浏览器插件提供REST API接口

use crate::extension::ExtensionRegistry;
use crate::sync::Task;
use anyhow::Result;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

// ============ 请求/响应类型 ============

#[derive(Debug, Deserialize)]
struct DownloadRequest {
    url: String,
    platform: Option<String>,
    title: Option<String>,
    quality: Option<String>,
}

#[derive(Debug, Serialize)]
struct DownloadResponse {
    task_id: String,
    status: String,
    message: String,
}

#[derive(Debug, Deserialize)]
struct SyncRequest {
    queue: Vec<Task>,
}

#[derive(Debug, Serialize)]
struct SyncResponse {
    processed: usize,
    queue: Vec<Task>,
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: String,
    version: String,
    extensions: Vec<String>,
}

#[derive(Debug, Serialize)]
struct QueueResponse {
    queue: Vec<Task>,
    total: usize,
}

// ============ 应用状态 ============

pub struct AppState {
    pub registry: Arc<RwLock<ExtensionRegistry>>,
    pub download_queue: Arc<RwLock<Vec<Task>>>,
}

impl AppState {
    pub fn new(registry: Arc<RwLock<ExtensionRegistry>>) -> Self {
        Self {
            registry,
            download_queue: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

// ============ 路由处理器 ============

async fn health_handler(state: State<Arc<AppState>>) -> impl IntoResponse {
    let registry = state.registry.read().await;
    let extensions: Vec<String> = registry.get_all_manifests()
        .iter()
        .map(|m| m.name.clone())
        .collect();
    
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        extensions,
    })
}

async fn download_handler(
    state: State<Arc<AppState>>,
    Json(req): Json<DownloadRequest>,
) -> impl IntoResponse {
    let task_id = uuid::Uuid::new_v4().to_string();
    
    // 创建任务
    let ext_name = req.platform.or_else(|| {
        state.registry.blocking_read().match_intent(&req.url)
    });
    
    let task = match Task::new(&req.url, ext_name) {
        Ok(t) => t,
        Err(_) => {
            return Json(DownloadResponse {
                task_id: task_id.clone(),
                status: "error".to_string(),
                message: "创建任务失败".to_string(),
            }).into_response();
        }
    };
    
    // 添加到队列
    state.download_queue.write().await.push(task.clone());
    
    Json(DownloadResponse {
        task_id,
        status: "created".to_string(),
        message: "下载任务已创建".to_string(),
    }).into_response()
}

async fn queue_handler(state: State<Arc<AppState>>) -> impl IntoResponse {
    let queue = state.download_queue.read().await.clone();
    let total = queue.len();
    
    Json(QueueResponse { queue, total }).into_response()
}

async fn sync_handler(
    state: State<Arc<AppState>>,
    Json(req): Json<SyncRequest>,
) -> impl IntoResponse {
    let mut queue = state.download_queue.write().await;
    
    for task in req.queue {
        if !queue.iter().any(|t| t.id == task.id) {
            queue.push(task);
        }
    }
    
    let processed = queue.len();
    let result = queue.clone();
    
    Json(SyncResponse {
        processed,
        queue: result,
    }).into_response()
}

async fn extensions_handler(state: State<Arc<AppState>>) -> impl IntoResponse {
    let registry = state.registry.read().await;
    let extensions: Vec<String> = registry.get_all_manifests()
        .iter()
        .map(|m| m.name.clone())
        .collect();
    
    Json(extensions).into_response()
}

async fn match_handler(
    state: State<Arc<AppState>>,
    axum::extract::Query(params): axum::extract::Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let query = params.get("q").cloned().unwrap_or_default();
    
    let registry = state.registry.read().await;
    let best_match = registry.match_intent(&query);
    let manifests = registry.get_all_manifests();
    
    let matches: Vec<_> = manifests.iter()
        .map(|m| {
            serde_json::json!({
                "name": m.name,
                "score": m.match_score(&query)
            })
        })
        .collect();
    
    Json(serde_json::json!({
        "query": query,
        "match": best_match,
        "extensions": matches
    })).into_response()
}

// ============ 错误处理 ============

struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": self.0.to_string()
            })),
        )
            .into_response()
    }
}

impl<T: Into<anyhow::Error>> From<T> for AppError {
    fn from(err: T) -> Self {
        AppError(err.into())
    }
}

// ============ 服务器启动 ============

pub async fn start_server(state: Arc<AppState>, port: u16) -> Result<()> {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    
    let app = Router::new()
        .route("/api/health", get(health_handler))
        .route("/api/download", post(download_handler))
        .route("/api/queue", get(queue_handler))
        .route("/api/sync", post(sync_handler))
        .route("/api/extensions", get(extensions_handler))
        .route("/api/match", get(match_handler))
        .layer(cors)
        .with_state(state);
    
    let addr = format!("0.0.0.0:{}", port);
    info!("🚀 HTTP API 服务器启动: http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
