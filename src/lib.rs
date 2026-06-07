//! OpenFetch - 史无前例的全能内容获取平台
//! 
//! 核心特性：
//! - Extension First: 所有功能都是扩展，核心永远不改
//! - AI Native: 每个扩展自描述能力，AI自主发现
//! - 多端同步: CLI/浏览器/Safari/Web/AI Agent 无缝切换

pub mod core;
pub mod extension;
pub mod sync;
pub mod cli;
pub mod utils;

// 导出公共接口
pub use core::{Engine, Config};
pub use extension::{Extension, ExtensionRegistry, ExtensionManifest};
pub use sync::{SyncManager, Task, TaskStatus};
pub use utils::error::OpenFetchError;

/// OpenFetch 库版本
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
