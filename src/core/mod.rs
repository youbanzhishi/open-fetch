//! OpenFetch 核心引擎
//! 
//! 负责任务调度、扩展加载、下载协调

use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::extension::{ExtensionRegistry, ExtensionManifest};
use crate::sync::{SyncManager, Task, TaskStatus};
use crate::utils::error::OpenFetchError;

/// 核心引擎
pub struct Engine {
    /// 扩展注册表
    extensions: Arc<RwLock<ExtensionRegistry>>,
    /// 同步管理器
    sync: Arc<SyncManager>,
    /// 配置
    config: Config,
}

/// 全局配置
#[derive(Debug, Clone)]
pub struct Config {
    /// 数据目录
    pub data_dir: std::path::PathBuf,
    /// 并发下载数
    pub concurrent_downloads: usize,
    /// 下载目录
    pub download_dir: std::path::PathBuf,
    /// 是否启用云端同步
    pub sync_enabled: bool,
}

impl Default for Config {
    fn default() -> Self {
        let data_dir = dirs::data_local_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("open-fetch");
        
        let download_dir = dirs::download_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("open-fetch");
        
        Self {
            data_dir,
            concurrent_downloads: 3,
            download_dir,
            sync_enabled: true,
        }
    }
}

impl Engine {
    /// 创建新引擎
    pub async fn new(config: Config) -> Result<Self> {
        // 确保目录存在
        std::fs::create_dir_all(&config.data_dir)?;
        std::fs::create_dir_all(&config.download_dir)?;
        
        let extensions = Arc::new(RwLock::new(ExtensionRegistry::new()));
        let sync = Arc::new(SyncManager::new(&config.data_dir).await?);
        
        Ok(Self {
            extensions,
            sync,
            config,
        })
    }
    
    /// 加载扩展
    pub async fn load_extensions(&self, ext_dir: &std::path::Path) -> Result<Vec<ExtensionManifest>> {
        let mut registry = self.extensions.write().await;
        registry.load_from_dir(ext_dir).await
    }
    
    /// 发现扩展（AI可读）
    pub async fn discover_extensions(&self) -> Vec<ExtensionManifest> {
        let registry = self.extensions.read().await;
        registry.get_all_manifests()
    }
    
    /// AI意图匹配
    pub async fn match_intent(&self, query: &str) -> Option<String> {
        let registry = self.extensions.read().await;
        registry.match_intent(query)
    }
    
    /// 创建下载任务
    pub async fn create_task(&self, url: &str, ext_name: Option<&str>) -> Result<Task> {
        let ext_name = if let Some(name) = ext_name {
            Some(name.to_string())
        } else {
            self.match_intent(url).await
        };
        
        let task = Task::new(url, ext_name)?;
        
        // 同步到本地数据库
        self.sync.save_task(&task).await?;
        
        // 如果启用了云同步，同步到云端
        if self.config.sync_enabled {
            self.sync.sync_to_cloud(&task).await?;
        }
        
        Ok(task)
    }
    
    /// 执行任务
    pub async fn execute_task(&self, task_id: &str) -> Result<()> {
        let task = self.sync.get_task(task_id).await?;
        
        let ext_name = task.extension_name
            .as_ref()
            .ok_or_else(|| OpenFetchError::ExtensionNotFound("no extension specified".into()))?;
        
        let registry = self.extensions.read().await;
        let extension = registry.get(ext_name)
            .ok_or_else(|| OpenFetchError::ExtensionNotFound(ext_name.clone()))?;
        
        // 执行下载
        self.sync.update_task_status(task_id, TaskStatus::Downloading).await?;
        extension.as_ref().download(&task.url, &self.config.download_dir).await?;
        self.sync.update_task_status(task_id, TaskStatus::Completed).await?;
        
        // 同步状态到云端
        if self.config.sync_enabled {
            self.sync.sync_status_to_cloud(task_id).await?;
        }
        
        Ok(())
    }
    
    /// 获取任务列表
    pub async fn list_tasks(&self, status: Option<TaskStatus>) -> Result<Vec<Task>> {
        self.sync.list_tasks(status).await
    }
    
    /// 获取任务
    pub async fn get_task(&self, task_id: &str) -> Result<Task> {
        self.sync.get_task(task_id).await
    }
    
    /// 删除任务
    pub async fn delete_task(&self, task_id: &str) -> Result<()> {
        self.sync.delete_task(task_id).await
    }
    
    /// 同步状态（从云端拉取）
    pub async fn sync_from_cloud(&self) -> Result<Vec<Task>> {
        self.sync.sync_from_cloud().await
    }
    
    /// 获取扩展注册表
    pub fn registry(&self) -> Arc<RwLock<ExtensionRegistry>> {
        self.extensions.clone()
    }
    
    /// 获取同步管理器
    pub fn sync_manager(&self) -> Arc<SyncManager> {
        self.sync.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_engine_create() {
        let config = Config::default();
        let engine = Engine::new(config).await;
        assert!(engine.is_ok());
    }
}
