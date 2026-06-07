//! 扩展系统 - Extension First
//! 
//! 所有功能都是扩展，核心永远不改

use anyhow::{Context, Result};
use async_trait::async_trait;
use glob::glob;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tokio::fs;

/// 扩展元信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionManifest {
    /// 扩展名称
    pub name: String,
    /// 版本
    pub version: String,
    /// 描述
    pub description: String,
    
    /// AI可读描述
    #[serde(default)]
    pub ai_manifest: Option<AIManifest>,
    
    /// 能力列表
    #[serde(default)]
    pub capabilities: Vec<String>,
    
    /// 平台匹配
    #[serde(default)]
    pub platforms: Vec<String>,
}

/// AI Manifest - 核心创新
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
    /// 意图模式
    #[serde(default)]
    pub intent_patterns: Vec<String>,
    /// 置信度关键词
    #[serde(default)]
    pub confidence_keywords: HashMap<String, Vec<String>>,
}

impl ExtensionManifest {
    /// 从YAML文件加载
    pub async fn from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path).await
            .context(format!("读取扩展文件失败: {:?}", path))?;
        
        let manifest: ExtensionManifest = serde_yaml::from_str(&content)
            .context("解析扩展YAML失败")?;
        
        Ok(manifest)
    }
    
    /// AI匹配分数
    pub fn match_score(&self, query: &str) -> f32 {
        let query_lower = query.to_lowercase();
        let mut score = 0.0;
        
        if let Some(ref ai) = self.ai_manifest {
            for keyword in &ai.keywords {
                if query_lower.contains(&keyword.to_lowercase()) {
                    score += 1.0;
                }
            }
            
            for pattern in &ai.intent_patterns {
                if query_lower.contains(&pattern.replace('*', "").to_lowercase()) {
                    score += 0.5;
                }
            }
            
            for platform in &self.platforms {
                if query_lower.contains(&platform.to_lowercase()) {
                    score += 1.5;
                }
            }
        }
        
        // 描述匹配
        if self.description.to_lowercase().contains(&query_lower) {
            score += 0.5;
        }
        
        score
    }
}

/// 扩展实例
pub struct Extension {
    /// 元信息
    pub manifest: ExtensionManifest,
    /// 实现路径
    pub path: std::path::PathBuf,
}

impl Extension {
    pub fn new(manifest: ExtensionManifest, path: std::path::PathBuf) -> Self {
        Self { manifest, path }
    }
}

#[async_trait]
impl Extension for Extension {
    /// 下载资源
    async fn download(&self, url: &str, dest: &Path) -> Result<std::path::PathBuf> {
        // 调用扩展的Python脚本或原生实现
        // 这里先做占位，后续实现
        anyhow::bail!("扩展下载未实现: {}", self.manifest.name)
    }
    
    /// 获取元信息
    async fn get_metadata(&self, url: &str) -> Result<crate::sync::TaskMetadata> {
        anyhow::bail!("扩展元信息获取未实现: {}", self.manifest.name)
    }
}

/// 扩展能力特征
#[async_trait]
pub trait DownloadExtension {
    /// 下载资源
    async fn download(&self, url: &str, dest: &Path) -> Result<std::path::PathBuf>;
    
    /// 获取元信息
    async fn get_metadata(&self, url: &str) -> Result<crate::sync::TaskMetadata>;
    
    /// 是否支持该URL
    fn supports(&self, url: &str) -> bool {
        for platform in &self.manifest().platforms {
            if url.contains(platform) {
                return true;
            }
        }
        false
    }
    
    fn manifest(&self) -> &ExtensionManifest;
}

/// 扩展注册表
pub struct ExtensionRegistry {
    /// 已加载的扩展
    extensions: HashMap<String, Extension>,
    /// 扩展列表
    manifests: Vec<ExtensionManifest>,
}

impl ExtensionRegistry {
    pub fn new() -> Self {
        Self {
            extensions: HashMap::new(),
            manifests: Vec::new(),
        }
    }
    
    /// 从目录加载扩展
    pub async fn load_from_dir(&mut self, dir: &Path) -> Result<Vec<ExtensionManifest>> {
        let pattern = dir.join("**/extension.yaml");
        
        for entry in glob(pattern.to_str().unwrap())? {
            if let Ok(path) = entry {
                match ExtensionManifest::from_file(&path).await {
                    Ok(manifest) => {
                        let ext = Extension::new(
                            manifest.clone(),
                            path.parent().unwrap().to_path_buf()
                        );
                        self.extensions.insert(ext.manifest.name.clone(), ext);
                        self.manifests.push(manifest);
                    }
                    Err(e) => {
                        tracing::warn!("加载扩展失败 {:?}: {}", path, e);
                    }
                }
            }
        }
        
        Ok(self.manifests.clone())
    }
    
    /// 获取所有扩展的AI Manifest
    pub fn get_all_manifests(&self) -> Vec<ExtensionManifest> {
        self.manifests.clone()
    }
    
    /// 获取指定扩展
    pub fn get(&self, name: &str) -> Option<&Extension> {
        self.extensions.get(name)
    }
    
    /// AI意图匹配
    pub fn match_intent(&self, query: &str) -> Option<String> {
        let mut best_score = 0.0;
        let mut best_name = None;
        
        for manifest in &self.manifests {
            let score = manifest.match_score(query);
            if score > best_score {
                best_score = score;
                best_name = Some(manifest.name.clone());
            }
        }
        
        if best_score > 0.0 {
            best_name
        } else {
            None
        }
    }
}

impl Default for ExtensionRegistry {
    fn default() -> Self {
        Self::new()
    }
}
