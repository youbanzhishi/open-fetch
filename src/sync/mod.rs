//! 多端同步系统
//! 
//! 实现：CLI/浏览器/Safari/Web/AI Agent 无缝切换

use anyhow::Result;
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::path::Path;
use uuid::Uuid;

/// 任务
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// 任务ID
    pub id: String,
    /// 源URL
    pub url: String,
    /// 扩展名称
    pub extension_name: Option<String>,
    /// 状态
    pub status: TaskStatus,
    /// 文件路径
    pub file_path: Option<String>,
    /// 元信息
    pub metadata: Option<TaskMetadata>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
    /// 设备ID（用于多端识别）
    pub device_id: String,
    /// 同步版本号（乐观锁）
    pub version: i64,
}

impl Task {
    pub fn new(url: &str, extension_name: Option<String>) -> Result<Self> {
        let now = Utc::now();
        let device_id = get_device_id()?;
        
        Ok(Self {
            id: Uuid::new_v4().to_string(),
            url: url.to_string(),
            extension_name,
            status: TaskStatus::Pending,
            file_path: None,
            metadata: None,
            created_at: now,
            updated_at: now,
            device_id,
            version: 1,
        })
    }
}

/// 任务状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Pending,
    Downloading,
    Completed,
    Failed,
    Cancelled,
}

impl TaskStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskStatus::Pending => "pending",
            TaskStatus::Downloading => "downloading",
            TaskStatus::Completed => "completed",
            TaskStatus::Failed => "failed",
            TaskStatus::Cancelled => "cancelled",
        }
    }
    
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "pending" => Some(TaskStatus::Pending),
            "downloading" => Some(TaskStatus::Downloading),
            "completed" => Some(TaskStatus::Completed),
            "failed" => Some(TaskStatus::Failed),
            "cancelled" => Some(TaskStatus::Cancelled),
            _ => None,
        }
    }
}

/// 任务元信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskMetadata {
    /// 文件名
    pub filename: Option<String>,
    /// 文件大小
    pub size: Option<u64>,
    /// 格式
    pub format: Option<String>,
    /// 标题
    pub title: Option<String>,
    /// 时长（秒）
    pub duration: Option<f64>,
    /// 缩略图
    pub thumbnail: Option<String>,
}

/// 获取设备ID
fn get_device_id() -> Result<String> {
    // 尝试从文件读取
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("open-fetch");
    
    let device_file = config_dir.join("device_id");
    
    if let Ok(id) = std::fs::read_to_string(&device_file) {
        return Ok(id.trim().to_string());
    }
    
    // 生成新ID
    let id = Uuid::new_v4().to_string();
    
    // 保存
    std::fs::create_dir_all(&config_dir)?;
    std::fs::write(&device_file, &id)?;
    
    Ok(id)
}

/// 同步管理器
pub struct SyncManager {
    conn: Connection,
}

impl SyncManager {
    /// 创建同步管理器
    pub async fn new(data_dir: &Path) -> Result<Self> {
        let db_path = data_dir.join("tasks.db");
        let conn = Connection::open(&db_path)?;
        
        // 创建表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id TEXT PRIMARY KEY,
                url TEXT NOT NULL,
                extension_name TEXT,
                status TEXT NOT NULL,
                file_path TEXT,
                metadata TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                device_id TEXT NOT NULL,
                version INTEGER NOT NULL DEFAULT 1,
                synced INTEGER NOT NULL DEFAULT 0
            )",
            [],
        )?;
        
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status)",
            [],
        )?;
        
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_tasks_device ON tasks(device_id)",
            [],
        )?;
        
        Ok(Self { conn })
    }
    
    /// 保存任务
    pub async fn save_task(&self, task: &Task) -> Result<()> {
        let metadata_json = task.metadata
            .as_ref()
            .map(|m| serde_json::to_string(m).ok())
            .flatten();
        
        self.conn.execute(
            "INSERT OR REPLACE INTO tasks 
             (id, url, extension_name, status, file_path, metadata, 
              created_at, updated_at, device_id, version, synced)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, 0)",
            params![
                task.id,
                task.url,
                task.extension_name,
                task.status.as_str(),
                task.file_path,
                metadata_json,
                task.created_at.to_rfc3339(),
                task.updated_at.to_rfc3339(),
                task.device_id,
                task.version,
            ],
        )?;
        
        Ok(())
    }
    
    /// 获取任务
    pub async fn get_task(&self, id: &str) -> Result<Task> {
        let task = self.conn.query_row(
            "SELECT * FROM tasks WHERE id = ?1",
            params![id],
            |row| row_to_task(row),
        )?;
        
        Ok(task)
    }
    
    /// 更新任务状态
    pub async fn update_task_status(&self, id: &str, status: TaskStatus) -> Result<()> {
        self.conn.execute(
            "UPDATE tasks SET status = ?1, updated_at = ?2, version = version + 1, synced = 0 
             WHERE id = ?3",
            params![
                status.as_str(),
                Utc::now().to_rfc3339(),
                id,
            ],
        )?;
        
        Ok(())
    }
    
    /// 删除任务
    pub async fn delete_task(&self, id: &str) -> Result<()> {
        self.conn.execute("DELETE FROM tasks WHERE id = ?1", params![id])?;
        Ok(())
    }
    
    /// 列出任务
    pub async fn list_tasks(&self, status: Option<TaskStatus>) -> Result<Vec<Task>> {
        let mut tasks = Vec::new();
        
        let query = match status {
            Some(s) => format!("SELECT * FROM tasks WHERE status = '{}' ORDER BY created_at DESC", s.as_str()),
            None => "SELECT * FROM tasks ORDER BY created_at DESC".to_string(),
        };
        
        let mut stmt = self.conn.prepare(&query)?;
        let rows = stmt.query_map([], |row| row_to_task(row))?;
        
        for row in rows {
            tasks.push(row?);
        }
        
        Ok(tasks)
    }
    
    /// 同步到云端（占位）
    pub async fn sync_to_cloud(&self, _task: &Task) -> Result<()> {
        // TODO: 实现云端同步
        // 可以使用GitHub Gist/私有仓库/自建服务
        Ok(())
    }
    
    /// 从云端同步
    pub async fn sync_from_cloud(&self) -> Result<Vec<Task>> {
        // TODO: 实现云端拉取
        Ok(Vec::new())
    }
    
    /// 同步状态到云端
    pub async fn sync_status_to_cloud(&self, _task_id: &str) -> Result<()> {
        // TODO: 实现状态同步
        Ok(())
    }
}

fn row_to_task(row: &rusqlite::Row) -> rusqlite::Result<Task> {
    let status_str: String = row.get(3)?;
    let status = TaskStatus::from_str(&status_str)
        .unwrap_or(TaskStatus::Pending);
    
    let metadata_str: Option<String> = row.get(5)?;
    let metadata = metadata_str
        .and_then(|s| serde_json::from_str(&s).ok());
    
    let created_at: String = row.get(6)?;
    let updated_at: String = row.get(7)?;
    
    Ok(Task {
        id: row.get(0)?,
        url: row.get(1)?,
        extension_name: row.get(2)?,
        status,
        file_path: row.get(4)?,
        metadata,
        created_at: DateTime::parse_from_rfc3339(&created_at)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now()),
        updated_at: DateTime::parse_from_rfc3339(&updated_at)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now()),
        device_id: row.get(8)?,
        version: row.get(9)?,
    })
}
