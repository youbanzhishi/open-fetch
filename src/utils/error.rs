//! 错误类型

use thiserror::Error;

/// OpenFetch 错误
#[derive(Error, Debug)]
pub enum OpenFetchError {
    #[error("扩展未找到: {0}")]
    ExtensionNotFound(String),
    
    #[error("下载失败: {0}")]
    DownloadFailed(String),
    
    #[error("解析失败: {0}")]
    ParseError(String),
    
    #[error("同步失败: {0}")]
    SyncFailed(String),
    
    #[error("存储失败: {0}")]
    StorageError(String),
    
    #[error("配置错误: {0}")]
    ConfigError(String),
    
    #[error("权限错误: {0}")]
    PermissionError(String),
}
