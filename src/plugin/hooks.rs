//! Hook System - 钩子系统
//! 
//! 核心层零业务逻辑：所有业务通过钩子注入

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// 钩子事件类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HookEvent {
    /// 下载前
    PreDownload,
    /// 下载后
    PostDownload,
    /// 下载失败
    DownloadFailed,
    /// 任务创建
    TaskCreated,
    /// 任务完成
    TaskCompleted,
    /// 任务取消
    TaskCancelled,
    /// 插件加载
    PluginLoaded,
    /// 插件卸载
    PluginUnloaded,
    /// AI意图解析
    AIIntentParsed,
    /// 同步前
    PreSync,
    /// 同步后
    PostSync,
    /// 压缩前
    PreCompress,
    /// 压缩后
    PostCompress,
}

/// 钩子优先级
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HookPriority {
    /// 最低优先级
    Lowest = -100,
    Low = -50,
    Normal = 0,
    High = 50,
    Highest = 100,
}

/// 钩子处理器
pub trait HookHandler: Send + Sync {
    /// 处理钩子事件
    fn handle(&self, event: HookEvent, ctx: &HookContext) -> Result<HookResult>;
    
    /// 获取钩子名称
    fn name(&self) -> &'static str;
    
    /// 获取优先级
    fn priority(&self) -> HookPriority { HookPriority::Normal }
}

/// 钩子上下文
#[derive(Debug, Clone)]
pub struct HookContext {
    /// 事件类型
    pub event: HookEvent,
    /// 关联的任务ID
    pub task_id: Option<String>,
    /// 关联的URL
    pub url: Option<String>,
    /// 关联的插件ID
    pub plugin_id: Option<String>,
    /// 扩展数据
    pub data: HashMap<String, serde_json::Value>,
}

/// 钩子结果
#[derive(Debug, Clone, Default)]
pub struct HookResult {
    /// 是否继续执行
    pub proceed: bool,
    /// 阻止原因
    pub reason: Option<String>,
    /// 修改后的数据
    pub modified_data: Option<HashMap<String, serde_json::Value>>,
    /// 错误信息
    pub error: Option<String>,
}

impl HookResult {
    pub fn continue_() -> Self {
        Self { proceed: true, reason: None, modified_data: None, error: None }
    }
    
    pub fn stop(reason: impl Into<String>) -> Self {
        Self { proceed: false, reason: Some(reason.into()), modified_data: None, error: None }
    }
    
    pub fn modify(data: HashMap<String, serde_json::Value>) -> Self {
        Self { proceed: true, reason: None, modified_data: Some(data), error: None }
    }
}

/// 钩子系统
pub struct HookSystem {
    /// 钩子处理器映射
    handlers: HashMap<HookEvent, Vec<Arc<dyn HookHandler>>>,
    /// 钩子调用统计
    stats: HashMap<HookEvent, HookStats>,
}

#[derive(Debug, Clone, Default)]
pub struct HookStats {
    pub call_count: u64,
    pub error_count: u64,
    pub total_duration_ms: u64,
}

impl HookSystem {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
            stats: HashMap::new(),
        }
    }
    
    /// 注册钩子处理器
    pub fn register<H: HookHandler + 'static>(&mut self, event: HookEvent, handler: H) {
        let handlers = self.handlers.entry(event).or_insert_with(Vec::new);
        
        // 按优先级排序插入
        let handler = Arc::new(handler);
        let pos = handlers.iter()
            .position(|h| h.priority() > handler.priority())
            .unwrap_or(handlers.len());
        handlers.insert(pos, handler);
    }
    
    /// 触发钩子（同步版本）
    pub fn trigger(&mut self, event: HookEvent, ctx: &HookContext) -> HookResult {
        let handlers = match self.handlers.get(&event) {
            Some(h) => h.clone(),
            None => return HookResult::continue_(),
        };
        
        let mut final_proceed = true;
        let mut final_reason: Option<String> = None;
        let mut merged_data: Option<HashMap<String, serde_json::Value>> = None;
        
        for handler in handlers {
            let result = match handler.handle(event, ctx) {
                Ok(r) => r,
                Err(e) => {
                    tracing::warn!("钩子 {} 执行失败: {}", handler.name(), e);
                    HookResult { proceed: true, error: Some(e.to_string()), ..Default::default() }
                }
            };
            
            // 聚合修改的数据
            if let Some(modified) = result.modified_data.clone() {
                merged_data = Some(match merged_data.take() {
                    Some(mut existing) => { existing.extend(modified); existing }
                    None => modified,
                });
            }
            
            // 任何一个钩子阻止就停止
            if !result.proceed {
                final_proceed = false;
                final_reason = result.reason.or(Some(format!("被 {} 阻止", handler.name())));
                break;
            }
            
            // 更新统计
            self.update_stats(event, &result);
        }
        
        HookResult {
            proceed: final_proceed,
            reason: final_reason,
            modified_data: merged_data,
            error: None,
        }
    }
    
    /// 触发钩子（异步版本）
    pub async fn trigger_async(&self, event: HookEvent, ctx: HookContext) -> HookResult {
        // 对于异步钩子，先用同步版本占位
        // 后续可扩展为真正的异步钩子
        let mut sys = HookSystem::new();
        let mut me = self.clone();
        sys.handlers = me.handlers.clone();
        sys.trigger(event, &ctx)
    }
    
    fn update_stats(&mut self, event: HookEvent, result: &HookResult) {
        let stats = self.stats.entry(event).or_default();
        stats.call_count += 1;
        if result.error.is_some() {
            stats.error_count += 1;
        }
    }
    
    /// 获取已注册的事件列表
    pub fn registered_events(&self) -> Vec<HookEvent> {
        self.handlers.keys().cloned().collect()
    }
    
    /// 获取统计信息
    pub fn get_stats(&self, event: HookEvent) -> Option<&HookStats> {
        self.stats.get(&event)
    }
}

impl Default for HookSystem {
    fn default() -> Self { Self::new() }
}

impl Clone for HookSystem {
    fn clone(&self) -> Self {
        Self {
            handlers: self.handlers.clone(),
            stats: self.stats.clone(),
        }
    }
}

// ============================================================================
// 内置钩子示例
// ============================================================================

/// 日志钩子
pub struct LogHook {
    prefix: String,
}

impl LogHook {
    pub fn new(prefix: &str) -> Self {
        Self { prefix: prefix.into() }
    }
}

impl HookHandler for LogHook {
    fn handle(&self, event: HookEvent, ctx: &HookContext) -> Result<HookResult> {
        tracing::info!(
            "{} 钩子触发: {:?}, task_id={:?}, url={:?}",
            self.prefix, event, ctx.task_id, ctx.url
        );
        Ok(HookResult::continue_())
    }
    
    fn name(&self) -> &'static str { "log-hook" }
}

/// 统计钩子
pub struct StatsHook {
    counter: std::sync::atomic::AtomicU64,
}

impl StatsHook {
    pub fn new() -> Self {
        Self { counter: std::sync::atomic::AtomicU64::new(0) }
    }
    
    pub fn count(&self) -> u64 {
        self.counter.load(std::sync::atomic::Ordering::SeqCst)
    }
}

impl HookHandler for StatsHook {
    fn handle(&self, event: HookEvent, _ctx: &HookContext) -> Result<HookResult> {
        self.counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        tracing::debug!("统计钩子: {:?}, 总计 {}", event, self.count());
        Ok(HookResult::continue_())
    }
    
    fn name(&self) -> &'static str { "stats-hook" }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    struct TestHook(u32);
    
    impl HookHandler for TestHook {
        fn handle(&self, _event: HookEvent, _ctx: &HookContext) -> Result<HookResult> {
            tracing::info!("TestHook {} 执行", self.0);
            Ok(HookResult::continue_())
        }
        fn name(&self) -> &'static str { "test-hook" }
    }
    
    #[tokio::test]
    async fn test_hook_system() {
        let mut system = HookSystem::new();
        system.register(HookEvent::PreDownload, TestHook(1));
        system.register(HookEvent::PostDownload, TestHook(2));
        
        let ctx = HookContext {
            event: HookEvent::PreDownload,
            task_id: Some("test-123".into()),
            url: Some("https://example.com".into()),
            plugin_id: None,
            data: HashMap::new(),
        };
        
        let result = system.trigger(HookEvent::PreDownload, &ctx);
        assert!(result.proceed);
        
        let result = system.trigger(HookEvent::PostDownload, &ctx);
        assert!(result.proceed);
    }
}
