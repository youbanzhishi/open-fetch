//! 云端认证系统
//! 支持API Key认证和简单Token

use axum::{
    extract::Request,
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
};
use serde::{Deserialize, Serialize};

/// API Key配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyConfig {
    pub key: String,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// 从请求中提取API Key
pub fn extract_api_key(request: &Request) -> Option<String> {
    request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.trim_start_matches("Bearer ").to_string())
}

/// 认证中间件
pub async fn auth_middleware(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // 检查Authorization头
    if let Some(key) = extract_api_key(&request) {
        // TODO: 验证API Key
        if validate_api_key(&key) {
            return Ok(next.run(request).await);
        }
    }
    
    // TODO: 支持无认证模式（开发环境）
    // return Ok(next.run(request).await);
    
    Err(StatusCode::UNAUTHORIZED)
}

/// 验证API Key
fn validate_api_key(key: &str) -> bool {
    // 简化验证：开发环境接受任何非空key
    !key.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extract_api_key() {
        let request = Request::builder()
            .header(AUTHORIZATION, "Bearer test-key-123")
            .body(())
            .unwrap();
        
        let key = extract_api_key(&request);
        assert_eq!(key, Some("test-key-123".to_string()));
    }
}
