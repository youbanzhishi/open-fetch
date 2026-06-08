//! Plugin API - 插件接口定义
//! 
//! 系统开发者铁律：核心层零业务逻辑，业务逻辑通过扩展注册

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// 插件能力枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PluginCapability {
    // 下载能力
    Download,
    LiveRecord,
    BatchDownload,
    
    // 处理能力
    Compress,
    Convert,
    Merge,
    
    // 解析能力
    Parse,
    Resolve,
    Extract,
    
    // 特殊能力
    RequiresAuth,
    RequiresProxy,
    SupportsCookies,
    
    // AI 能力
    AIMatchable,
}

/// 插件元信息（核心数据）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    /// 插件ID（唯一标识，小写+短横线）
    pub id: String,
    /// 插件名称
    pub name: String,
    /// 语义版本
    pub version: String,
    /// 描述
    pub description: String,
    /// 作者
    pub author: Option<String>,
    /// 许可证
    pub license: Option<String>,
    
    /// 运行时类型
    pub runtime: RuntimeType,
    /// 入口文件
    pub entry: String,
    
    /// 支持的平台（域名列表）
    #[serde(default)]
    pub platforms: Vec<String>,
    
    /// 能力列表
    #[serde(default)]
    pub capabilities: Vec<PluginCapability>,
    
    /// 优先级（数字越大优先级越高）
    #[serde(default = "default_priority")]
    pub priority: i32,
    
    /// AI Manifest
    #[serde(default)]
    pub ai: Option<AIManifest>,
    
    /// 配置项
    #[serde(default)]
    pub config: Vec<ConfigItem>,
}

fn default_priority() -> i32 { 50 }

/// AI Manifest（核心创新：每个扩展自描述，AI无需预设规则）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIManifest {
    /// AI可读描述
    pub description: String,
    /// 意图关键词
    #[serde(default)]
    pub keywords: Vec<String>,
    /// 使用示例
    #[serde(default)]
    pub examples: Vec<String>,
    /// 意图模式（支持通配符*）
    #[serde(default)]
    pub intent_patterns: Vec<String>,
    /// 置信度配置
    #[serde(default)]
    pub confidence: ConfidenceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceConfig {
    #[serde(default)]
    pub high: Vec<String>,
    #[serde(default)]
    pub medium: Vec<String>,
    #[serde(default)]
    pub low: Vec<String>,
}

impl Default for ConfidenceConfig {
    fn default() -> Self {
        Self { high: Vec::new(), medium: Vec::new(), low: Vec::new() }
    }
}

/// 配置项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigItem {
    pub key: String,
    pub value_type: ConfigType,
    pub default: serde_json::Value,
    pub required: bool,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfigType {
    String,
    Number,
    Boolean,
    Enum(Vec<String>),
}

/// 运行时类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuntimeType {
    Python,
    JavaScript,
    Native,
    Wasm,
}

/// 插件状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PluginStatus {
    Unloaded,
    Loading,
    Loaded,
    Error,
    Disabled,
}

// ============================================================================
// 核心：Plugin Trait（原生插件实现接口）
// 系统开发者铁律：错误处理不偷懒，用Result不用Option
// ============================================================================

#[async_trait]
pub trait Plugin: Send + Sync {
    /// 获取插件元信息
    fn manifest(&self) -> &PluginManifest;
    
    /// 初始化插件
    async fn on_load(&mut self) -> Result<()> { Ok(()) }
    
    /// 卸载插件
    async fn on_unload(&mut self) -> Result<()> { Ok(()) }
    
    /// 处理请求
    async fn handle(&self, ctx: &PluginContext) -> Result<PluginResult>;
    
    /// 健康检查
    async fn health_check(&self) -> Result<bool> { Ok(true) }
}

/// 插件上下文
#[derive(Debug, Clone)]
pub struct PluginContext {
    pub params: serde_json::Value,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub metadata: std::collections::HashMap<String, String>,
}

/// 插件结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginResult {
    pub success: bool,
    #[serde(default)]
    pub data: Option<serde_json::Value>,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub side_effects: Vec<SideEffect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SideEffect {
    pub effect_type: String,
    pub data: serde_json::Value,
}

// ============================================================================
// PluginManifest 方法
// ============================================================================

impl PluginManifest {
    /// 从YAML加载（系统开发者铁律：配置用serde反序列化，不手写parser）
    pub async fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = tokio::fs::read_to_string(path).await?;
        let manifest: PluginManifest = serde_yaml::from_str(&content)?;
        Ok(manifest)
    }
    
    /// 检查是否支持URL
    pub fn supports_url(&self, url: &str) -> bool {
        if self.platforms.is_empty() { return true; }
        self.platforms.iter().any(|p| url.contains(p))
    }
    
    /// 计算AI匹配分数
    pub fn match_score(&self, query: &str) -> f32 {
        let q = query.to_lowercase();
        let mut score = 0.0;
        
        if let Some(ref ai) = self.ai {
            for kw in &ai.keywords {
                if q.contains(&kw.to_lowercase()) { score += 2.0; }
            }
            for pat in &ai.intent_patterns {
                let p = pat.replace(['*', '?'], "").to_lowercase();
                if !p.is_empty() && q.contains(&p) { score += 1.5; }
            }
            for kw in &ai.confidence.high {
                if q.contains(&kw.to_lowercase()) { score += 3.0; }
            }
            for kw in &ai.confidence.medium {
                if q.contains(&kw.to_lowercase()) { score += 1.5; }
            }
        }
        
        for p in &self.platforms {
            if q.contains(&p.to_lowercase()) { score += 5.0; }
        }
        if self.description.to_lowercase().contains(&q) { score += 1.0; }
        if self.name.to_lowercase().contains(&q) { score += 2.0; }
        
        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_manifest_yaml_parse() {
        let yaml = r#"
id: test-plugin
name: Test Plugin
version: 1.0.0
description: "A test plugin"
runtime: python
entry: test.py
platforms:
  - example.com
capabilities:
  - download
"#;
        let manifest: PluginManifest = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(manifest.id, "test-plugin");
        assert!(manifest.supports_url("https://example.com/video"));
        assert!(!manifest.supports_url("https://other.com/video"));
    }
    
    #[test]
    fn test_ai_match_score() {
        let manifest = PluginManifest {
            id: "bilibili".into(),
            name: "Bilibili".into(),
            version: "1.0.0".into(),
            description: "B站下载".into(),
            author: None,
            license: None,
            runtime: RuntimeType::Python,
            entry: "bilibili.py".into(),
            platforms: vec!["bilibili.com".into()],
            capabilities: vec![PluginCapability::Download],
            priority: 50,
            ai: Some(AIManifest {
                description: "B站视频下载".into(),
                keywords: vec!["b站".into(), "bilibili".into(), "BV号".into()],
                examples: vec![],
                intent_patterns: vec!["下载*b站视频".into()],
                confidence: ConfidenceConfig { high: vec!["b站".into()], medium: vec![], low: vec![] },
            }),
            config: vec![],
        };
        
        assert!(manifest.match_score("下载这个b站视频") > 0.0);
        assert!(manifest.match_score("下载youtube视频") < manifest.match_score("下载b站视频"));
    }
}
