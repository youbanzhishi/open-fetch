//! OpenFetch Cloud - 云端下载服务
//! 提供Web UI和远程API，支持多用户和实时状态推送

pub mod api;
pub mod state;
pub mod websocket;
pub mod auth;

use anyhow::Result;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, error};

use crate::cloud::state::CloudState;

/// 启动云端服务
pub async fn start_cloud_server(
    addr: SocketAddr,
    download_dir: String,
    port: u16,
) -> Result<()> {
    info!("☁️ 启动 OpenFetch Cloud 服务 v0.9.0");
    info!("📡 Web UI: http://{}:{}/", addr.ip(), port);
    info!("📡 API: http://{}:{}/api", addr.ip(), port);
    info!("📡 WebSocket: ws://{}:{}/ws", addr.ip(), port);
    
    let cloud_state = CloudState::new(download_dir, port);
    
    // CORS配置
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    
    // 构建路由
    let app = api::router(cloud_state)
        .layer(cors);
    
    let listener = TcpListener::bind(addr).await?;
    info!("✅ 云端服务已启动");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}
