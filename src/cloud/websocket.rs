//! WebSocket实时推送
//! 支持任务状态实时更新

use axum::ws::{Message, WebSocket};
use futures_util::{SinkExt, StreamExt};
use tracing::{error, info, warn};

use crate::cloud::state::{CloudState, TaskEvent};

/// WebSocket事件处理器
pub async fn ws_handler(socket: WebSocket, state: Arc<CloudState>) {
    let user_id = Uuid::new_v4().to_string();
    info!("🔌 WebSocket连接: {}", user_id);
    
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.subscribe();
    
    // 发送欢迎消息
    let welcome = serde_json::json!({
        "type": "connected",
        "user_id": user_id,
        "message": "已连接到OpenFetch Cloud"
    });
    if sender.send(Message::Text(welcome.to_string().into())).await.is_err() {
        return;
    }
    
    // 发送当前任务列表
    let tasks = state.list_tasks().await;
    let init_msg = serde_json::json!({
        "type": "init",
        "tasks": tasks
    });
    if sender.send(Message::Text(init_msg.to_string().into())).await.is_err() {
        return;
    }
    
    loop {
        tokio::select! {
            // 接收客户端消息
            msg = receiver.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        if let Err(e) = handle_client_message(&mut sender, &text).await {
                            error!("处理WebSocket消息失败: {}", e);
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => {
                        info!("🔌 WebSocket断开: {}", user_id);
                        break;
                    }
                    _ => {}
                }
            }
            // 接收服务端事件
            event = rx.recv() => {
                match event {
                    Ok(evt) => {
                        let json = serde_json::to_string(&evt).unwrap_or_default();
                        if sender.send(Message::Text(json.into())).await.is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        }
    }
    
    info!("🔌 WebSocket会话结束: {}", user_id);
}

/// 处理客户端消息
async fn handle_client_message(
    sender: &mut futures_util::stream::SplitSink<WebSocket, Message>,
    text: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if let Ok(msg) = serde_json::from_str::<ClientMessage>(text) {
        match msg.action.as_str() {
            "ping" => {
                let resp = serde_json::json!({"type": "pong"});
                sender.send(Message::Text(resp.to_string().into())).await?;
            }
            "subscribe" => {
                let resp = serde_json::json!({
                    "type": "subscribed",
                    "channels": msg.channels
                });
                sender.send(Message::Text(resp.to_string().into())).await?;
            }
            _ => {
                warn!("未知WebSocket消息类型: {}", msg.action);
            }
        }
    }
    Ok(())
}

use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct ClientMessage {
    action: String,
    channels: Option<Vec<String>>,
}
