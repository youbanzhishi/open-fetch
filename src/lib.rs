//! OpenFetch - 史无前例的全能内容获取平台
//! 
//! 核心特性：
//! - Extension First: 所有功能都是扩展，核心永远不改
//! - AI Native: 每个扩展自描述能力，AI自主发现
//! - 多端同步: CLI/浏览器/Safari/Web/AI Agent 无缝切换
//! - 插件系统: 参考DAW四柱架构（API/Registry/Runtime/Hooks）

pub mod core;
pub mod extension;
pub mod plugin;   // DAW风格四柱：API + Registry + Runtime + Hooks
pub mod sync;
pub mod cli;
pub mod server;    // HTTP API 服务器（浏览器插件通信）
pub mod utils;

// 导出公共接口
pub use core::{Engine, Config};
pub use extension::{ExtInstance, ExtensionRegistry, ExtensionManifest, DownloadExtension};
pub use plugin::{Plugin, PluginManifest, PluginCapability, PluginRegistry, HookSystem, HookEvent};
pub use sync::{SyncManager, Task, TaskStatus};

pub use utils::error::OpenFetchError;

/// OpenFetch 库版本
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
