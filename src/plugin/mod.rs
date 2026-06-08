//! OpenFetch 插件系统
//! 
//! 参考 DAW Extension Registry 四柱架构：
//! - Plugin API: 插件接口定义
//! - Registry: 扩展注册表
//! - Runtime: 脚本运行时
//! - Hooks: 钩子系统

pub mod api;
pub mod registry;
pub mod runtime;
pub mod hooks;

pub use api::{Plugin, PluginManifest, PluginCapability};
pub use registry::{PluginRegistry, PluginRef};
pub use runtime::{PluginRuntime, RuntimeType};
pub use hooks::{HookSystem, HookEvent, HookPriority};
