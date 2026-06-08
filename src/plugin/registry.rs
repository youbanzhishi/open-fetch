//! Plugin Registry - 扩展注册表
//! 
//! 核心层零业务逻辑：所有功能通过注册表动态加载

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use glob::glob;

/// 插件引用
#[derive(Debug, Clone)]
pub struct PluginRef {
    /// 插件ID
    pub id: String,
    /// 插件路径
    pub path: std::path::PathBuf,
    /// 运行时
    pub runtime: super::api::RuntimeType,
}

/// 插件注册表（核心数据结构）
pub struct PluginRegistry {
    /// 已加载插件
    plugins: HashMap<String, Arc<dyn super::api::Plugin>>,
    /// 插件元信息
    manifests: HashMap<String, super::api::PluginManifest>,
    /// 插件路径
    paths: HashMap<String, std::path::PathBuf>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            manifests: HashMap::new(),
            paths: HashMap::new(),
        }
    }
    
    /// 从目录批量加载插件
    pub async fn load_from_dir(&mut self, dir: &std::path::Path) -> Result<Vec<super::api::PluginManifest>> {
        let pattern = dir.join("**/extension.yaml");
        let pattern_str = pattern.to_str().context("路径无效")?;
        
        let mut loaded = Vec::new();
        
        for entry in glob(pattern_str).context("glob模式匹配失败")? {
            match entry {
                Ok(path) => {
                    match self.load_plugin(&path).await {
                        Ok(manifest) => {
                            tracing::info!("加载插件: {} v{}", manifest.id, manifest.version);
                            loaded.push(manifest);
                        }
                        Err(e) => {
                            tracing::warn!("插件加载失败 {:?}: {}", path, e);
                        }
                    }
                }
                Err(e) => tracing::warn!("glob entry错误: {}", e),
            }
        }
        
        Ok(loaded)
    }
    
    /// 加载单个插件
    async fn load_plugin(&mut self, manifest_path: &std::path::Path) -> Result<super::api::PluginManifest> {
        let manifest = super::api::PluginManifest::from_file(manifest_path).await?;
        let plugin_dir = manifest_path.parent().unwrap();
        
        // 放入注册表
        self.manifests.insert(manifest.id.clone(), manifest.clone());
        self.paths.insert(manifest.id.clone(), plugin_dir.to_path_buf());
        
        Ok(manifest)
    }
    
    /// 获取所有插件元信息
    pub fn get_all_manifests(&self) -> Vec<super::api::PluginManifest> {
        self.manifests.values().cloned().collect()
    }
    
    /// 获取指定插件元信息
    pub fn get_manifest(&self, id: &str) -> Option<&super::api::PluginManifest> {
        self.manifests.get(id)
    }
    
    /// 获取插件路径
    pub fn get_path(&self, id: &str) -> Option<&std::path::Path> {
        self.paths.get(id).map(|p| p.as_path())
    }
    
    /// AI意图匹配 - 返回最佳插件ID
    /// 系统开发者铁律：核心层零业务逻辑
    pub fn match_intent(&self, query: &str) -> Option<String> {
        let mut best_score = 0.0f32;
        let mut best_id = None;
        
        for (id, manifest) in &self.manifests {
            let score = manifest.match_score(query);
            if score > best_score {
                best_score = score;
                best_id = Some(id.clone());
            }
        }
        
        // 阈值过滤
        if best_score > 0.0 { best_id } else { None }
    }
    
    /// 按平台查找插件
    pub fn find_by_platform(&self, url: &str) -> Vec<&super::api::PluginManifest> {
        self.manifests.values()
            .filter(|m| m.supports_url(url))
            .collect()
    }
    
    /// 按能力查找插件
    pub fn find_by_capability(&self, cap: &super::api::PluginCapability) -> Vec<&super::api::PluginManifest> {
        self.manifests.values()
            .filter(|m| m.capabilities.contains(cap))
            .collect()
    }
    
    /// 插件数量
    pub fn len(&self) -> usize { self.manifests.len() }
    
    /// 是否为空
    pub fn is_empty(&self) -> bool { self.manifests.is_empty() }
}

impl Default for PluginRegistry {
    fn default() -> Self { Self::new() }
}

// ============================================================================
// 线程安全的注册表包装器
// ============================================================================

/// 线程安全的注册表
pub struct SharedRegistry {
    inner: Arc<RwLock<PluginRegistry>>,
}

impl SharedRegistry {
    pub fn new() -> Self {
        Self { inner: Arc::new(RwLock::new(PluginRegistry::new())) }
    }
    
    pub async fn load_dir(&self, dir: &std::path::Path) -> Result<Vec<super::api::PluginManifest>> {
        let mut registry = self.inner.write().await;
        registry.load_from_dir(dir).await
    }
    
    pub async fn get_all(&self) -> Vec<super::api::PluginManifest> {
        let registry = self.inner.read().await;
        registry.get_all_manifests()
    }
    
    pub async fn match_intent(&self, query: &str) -> Option<String> {
        let registry = self.inner.read().await;
        registry.match_intent(query)
    }
    
    pub async fn find_by_platform(&self, url: &str) -> Vec<super::api::PluginManifest> {
        let registry = self.inner.read().await;
        registry.find_by_platform(url).into_iter().cloned().collect()
    }
}

impl Default for SharedRegistry {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_registry_empty() {
        let registry = PluginRegistry::new();
        assert!(registry.is_empty());
        assert!(registry.match_intent("test").is_none());
    }
}
