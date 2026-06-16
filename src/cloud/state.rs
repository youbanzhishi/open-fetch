//! 云端服务状态管理
//! 支持多用户、多任务、实时推送

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// 下载任务状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudTask {
    pub id: String,
    pub url: String,
    pub platform: String,
    pub status: TaskStatus,
    pub progress: f32,
    pub speed: String,
    pub size: String,
    pub output_path: Option<String>,
    pub error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Pending,
    Downloading,
    Completed,
    Failed,
    Cancelled,
}

/// 云端服务状态
#[derive(Debug, Clone)]
pub struct CloudState {
    pub tasks: Arc<RwLock<HashMap<String, CloudTask>>>,
    pub download_dir: PathBuf,
    pub port: u16,
    pub tx: broadcast::Sender<TaskEvent>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TaskEvent {
    TaskCreated { task: CloudTask },
    TaskUpdated { task: CloudTask },
    TaskCompleted { task: CloudTask },
    TaskFailed { task_id: String, error: String },
    TaskRemoved { task_id: String },
}

impl CloudState {
    pub fn new(download_dir: String, port: u16) -> Self {
        let (tx, _) = broadcast::channel(1000);
        Self {
            tasks: Arc::new(RwLock::new(HashMap::new())),
            download_dir: PathBuf::from(download_dir),
            port,
            tx,
        }
    }
    
    pub async fn create_task(&self, url: String, platform: String, user_id: String) -> CloudTask {
        let task = CloudTask {
            id: Uuid::new_v4().to_string(),
            url,
            platform,
            status: TaskStatus::Pending,
            progress: 0.0,
            speed: "0 MB/s".to_string(),
            size: "0 MB".to_string(),
            output_path: None,
            error: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            user_id,
        };
        
        let mut tasks = self.tasks.write().await;
        tasks.insert(task.id.clone(), task.clone());
        
        let _ = self.tx.send(TaskEvent::TaskCreated { task: task.clone() });
        task
    }
    
    pub async fn update_task(&self, task_id: &str, progress: f32, speed: String, size: String) -> Option<CloudTask> {
        let mut tasks = self.tasks.write().await;
        if let Some(task) = tasks.get_mut(task_id) {
            task.progress = progress;
            task.speed = speed;
            task.size = size;
            task.updated_at = Utc::now();
            if progress >= 100.0 {
                task.status = TaskStatus::Completed;
            } else {
                task.status = TaskStatus::Downloading;
            }
            let updated = task.clone();
            let _ = self.tx.send(TaskEvent::TaskUpdated { task: updated.clone() });
            return Some(updated);
        }
        None
    }
    
    pub async fn complete_task(&self, task_id: &str, output_path: String) -> Option<CloudTask> {
        let mut tasks = self.tasks.write().await;
        if let Some(task) = tasks.get_mut(task_id) {
            task.status = TaskStatus::Completed;
            task.progress = 100.0;
            task.output_path = Some(output_path);
            task.updated_at = Utc::now();
            let updated = task.clone();
            let _ = self.tx.send(TaskEvent::TaskCompleted { task: updated.clone() });
            return Some(updated);
        }
        None
    }
    
    pub async fn fail_task(&self, task_id: &str, error: String) -> Option<CloudTask> {
        let mut tasks = self.tasks.write().await;
        if let Some(task) = tasks.get_mut(task_id) {
            task.status = TaskStatus::Failed;
            task.error = Some(error.clone());
            task.updated_at = Utc::now();
            let updated = task.clone();
            let _ = self.tx.send(TaskEvent::TaskFailed { task_id: task_id.to_string(), error });
            return Some(updated);
        }
        None
    }
    
    pub async fn remove_task(&self, task_id: &str) -> bool {
        let mut tasks = self.tasks.write().await;
        if tasks.remove(task_id).is_some() {
            let _ = self.tx.send(TaskEvent::TaskRemoved { task_id: task_id.to_string() });
            return true;
        }
        false
    }
    
    pub async fn get_task(&self, task_id: &str) -> Option<CloudTask> {
        let tasks = self.tasks.read().await;
        tasks.get(task_id).cloned()
    }
    
    pub async fn list_tasks(&self) -> Vec<CloudTask> {
        let tasks = self.tasks.read().await;
        let mut list: Vec<CloudTask> = tasks.values().cloned().collect();
        list.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        list
    }
    
    pub async fn list_tasks_by_user(&self, user_id: &str) -> Vec<CloudTask> {
        let tasks = self.tasks.read().await;
        let mut list: Vec<CloudTask> = tasks
            .values()
            .filter(|t| t.user_id == user_id)
            .cloned()
            .collect();
        list.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        list
    }
    
    pub fn subscribe(&self) -> broadcast::Receiver<TaskEvent> {
        self.tx.subscribe()
    }
}
