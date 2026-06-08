//! 扩展系统 - Extension First
//! 
//! 所有功能都是扩展，核心永远不改

use anyhow::{Context, Result};
use async_trait::async_trait;
use glob::glob;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::fs;

/// 扩展元信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    
    #[serde(default)]
    pub ai_manifest: Option<AIManifest>,
    
    #[serde(default)]
    pub capabilities: Vec<String>,
    
    #[serde(default)]
    pub platforms: Vec<String>,
}

/// AI Manifest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIManifest {
    pub description: String,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub examples: Vec<String>,
    #[serde(default)]
    pub intent_patterns: Vec<String>,
    #[serde(default)]
    pub confidence_keywords: HashMap<String, Vec<String>>,
}

impl ExtensionManifest {
    pub async fn from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path).await
            .context(format!("读取扩展文件失败: {:?}", path))?;
        let manifest: ExtensionManifest = serde_yaml::from_str(&content)
            .context("解析扩展YAML失败")?;
        Ok(manifest)
    }
    
    pub fn match_score(&self, query: &str) -> f32 {
        let query_lower = query.to_lowercase();
        let mut score = 0.0;
        
        if let Some(ref ai) = self.ai_manifest {
            for keyword in &ai.keywords {
                if query_lower.contains(&keyword.to_lowercase()) { score += 1.0; }
            }
            for pattern in &ai.intent_patterns {
                if query_lower.contains(&pattern.replace('*', "").to_lowercase()) { score += 0.5; }
            }
            for platform in &self.platforms {
                if query_lower.contains(&platform.to_lowercase()) { score += 1.5; }
            }
        }
        if self.description.to_lowercase().contains(&query_lower) { score += 0.5; }
        
        score
    }
}

/// 扩展实例
pub struct ExtInstance {
    pub manifest: ExtensionManifest,
    pub path: std::path::PathBuf,
}

impl ExtInstance {
    pub fn new(manifest: ExtensionManifest, path: std::path::PathBuf) -> Self {
        Self { manifest, path }
    }
}

#[async_trait]
impl DownloadExtension for ExtInstance {
    async fn download(&self, url: &str, dest: &Path) -> Result<std::path::PathBuf> {
        anyhow::bail!("扩展下载未实现: {}", self.manifest.name)
    }
    
    async fn get_metadata(&self, _url: &str) -> Result<crate::sync::TaskMetadata> {
        anyhow::bail!("扩展元信息获取未实现: {}", self.manifest.name)
    }
    
    fn manifest(&self) -> &ExtensionManifest {
        &self.manifest
    }
}

/// 扩展能力特征
#[async_trait]
pub trait DownloadExtension: Send + Sync {
    async fn download(&self, url: &str, dest: &Path) -> Result<std::path::PathBuf>;
    async fn get_metadata(&self, url: &str) -> Result<crate::sync::TaskMetadata>;
    
    fn supports(&self, url: &str) -> bool {
        for platform in &self.manifest().platforms {
            if url.contains(platform) { return true; }
        }
        false
    }
    
    fn manifest(&self) -> &ExtensionManifest;
}

/// 扩展注册表
pub struct ExtensionRegistry {
    extensions: HashMap<String, Arc<dyn DownloadExtension>>,
    manifests: Vec<ExtensionManifest>,
}

impl ExtensionRegistry {
    pub fn new() -> Self {
        Self { extensions: HashMap::new(), manifests: Vec::new() }
    }
    
    pub async fn load_from_dir(&mut self, dir: &Path) -> Result<Vec<ExtensionManifest>> {
        let pattern = dir.join("**/extension.yaml");
        
        for entry in glob(pattern.to_str().unwrap())? {
            if let Ok(path) = entry {
                match ExtensionManifest::from_file(&path).await {
                    Ok(manifest) => {
                        let ext: Arc<dyn DownloadExtension> = Arc::new(ExtInstance::new(
                            manifest.clone(),
                            path.parent().unwrap().to_path_buf()
                        ));
                        self.extensions.insert(manifest.name.clone(), ext);
                        self.manifests.push(manifest);
                    }
                    Err(e) => tracing::warn!("加载扩展失败 {:?}: {}", path, e),
                }
            }
        }
        Ok(self.manifests.clone())
    }
    
    pub fn get_all_manifests(&self) -> Vec<ExtensionManifest> { self.manifests.clone() }
    pub fn get(&self, name: &str) -> Option<Arc<dyn DownloadExtension>> { self.extensions.get(name).cloned() }
    
    pub fn match_intent(&self, query: &str) -> Option<String> {
        let mut best_score = 0.0f32;
        let mut best_name = None;
        for manifest in &self.manifests {
            let score = manifest.match_score(query);
            if score > best_score { best_score = score; best_name = Some(manifest.name.clone()); }
        }
        if best_score > 0.0 { best_name } else { None }
    }
    
    pub fn len(&self) -> usize { self.extensions.len() }
    pub fn is_empty(&self) -> bool { self.extensions.is_empty() }
}

impl Default for ExtensionRegistry {
    fn default() -> Self { Self::new() }
}
