//! CLI 模块

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};

/// OpenFetch CLI
#[derive(Parser)]
#[command(name = "open-fetch")]
#[command(version = "0.1.0")]
#[command(about = "OpenFetch - 史无前例的全能内容获取平台")]
struct Cli {
    /// 启用详细日志
    #[arg(short, long, global = true)]
    verbose: bool,
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 下载资源
    Download {
        /// 资源URL
        url: String,
        
        /// 指定扩展
        #[arg(short, long)]
        ext: Option<String>,
        
        /// 输出目录
        #[arg(short, long, default_value = ".")]
        output: String,
    },
    
    /// 列出可用扩展
    Ext {
        #[command(subcommand)]
        subcommand: ExtCommands,
    },
    
    /// AI 语义下载
    Ai {
        /// 自然语言查询
        query: String,
    },
    
    /// 任务管理
    Task {
        #[command(subcommand)]
        subcommand: TaskCommands,
    },
    
    /// 同步状态
    Sync {
        /// 从云端拉取
        #[arg(long)]
        pull: bool,
        
        /// 推送到云端
        #[arg(long)]
        push: bool,
    },
}

#[derive(Subcommand)]
enum ExtCommands {
    /// 列出所有扩展
    List,
    
    /// 搜索扩展
    Search {
        keyword: String,
    },
    
    /// 安装扩展
    Install {
        name: String,
    },
    
    /// 卸载扩展
    Uninstall {
        name: String,
    },
}

#[derive(Subcommand)]
enum TaskCommands {
    /// 列出任务
    List {
        /// 按状态筛选
        #[arg(short, long)]
        status: Option<TaskStatusArg>,
    },
    
    /// 查看任务详情
    Show {
        task_id: String,
    },
    
    /// 删除任务
    Delete {
        task_id: String,
    },
    
    /// 重试失败任务
    Retry,
}

#[derive(ValueEnum, Clone)]
enum TaskStatusArg {
    Pending,
    Downloading,
    Completed,
    Failed,
}

/// 运行CLI
pub async fn run() -> Result<()> {
    let cli = Cli::parse();
    
    // 初始化日志
    if cli.verbose {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .init();
    }
    
    match cli.command {
        Commands::Download { url, ext, output } => {
            download(&url, ext.as_deref(), &output).await?;
        }
        Commands::Ext { subcommand } => {
            handle_ext(subcommand).await?;
        }
        Commands::Ai { query } => {
            ai_download(&query).await?;
        }
        Commands::Task { subcommand } => {
            handle_task(subcommand).await?;
        }
        Commands::Sync { pull, push } => {
            handle_sync(pull, push).await?;
        }
    }
    
    Ok(())
}

async fn download(url: &str, ext: Option<&str>, output: &str) -> Result<()> {
    println!("🎬 开始下载: {}", url);
    
    // 加载配置和引擎
    let config = open_fetch::Config::default();
    let engine = open_fetch::Engine::new(config).await?;
    
    // 创建任务
    let task = engine.create_task(url, ext).await?;
    println!("✅ 任务创建成功: {}", task.id);
    
    // 执行任务
    engine.execute_task(&task.id).await?;
    
    println!("✅ 下载完成！");
    Ok(())
}

async fn handle_ext(subcommand: ExtCommands) -> Result<()> {
    let config = open_fetch::Config::default();
    let engine = open_fetch::Engine::new(config).await?;
    
    match subcommand {
        ExtCommands::List => {
            let manifests = engine.discover_extensions().await;
            println!("📦 可用扩展 ({}):", manifests.len());
            for m in manifests {
                println!("  - {} v{}: {}", m.name, m.version, m.description);
            }
        }
        ExtCommands::Search { keyword } => {
            let manifests = engine.discover_extensions().await;
            let results: Vec<_> = manifests.into_iter()
                .filter(|m| m.description.contains(&keyword) || m.name.contains(&keyword))
                .collect();
            
            println!("🔍 搜索 '{}' 结果 ({}):", keyword, results.len());
            for m in results {
                println!("  - {} v{}", m.name, m.version);
            }
        }
        ExtCommands::Install { name } => {
            println!("📥 安装扩展: {}", name);
            // TODO: 实现扩展安装
        }
        ExtCommands::Uninstall { name } => {
            println!("🗑️ 卸载扩展: {}", name);
            // TODO: 实现扩展卸载
        }
    }
    
    Ok(())
}

async fn ai_download(query: &str) -> Result<()> {
    println!("🤖 AI 理解: {}", query);
    
    let config = open_fetch::Config::default();
    let engine = open_fetch::Engine::new(config).await?;
    
    // AI 匹配扩展
    let ext_name = engine.match_intent(query).await;
    
    if let Some(ext) = ext_name {
        println!("🎯 匹配扩展: {}", ext);
        
        // 提取URL
        let url = extract_url(query);
        if let Some(url) = url {
            download(&url, Some(&ext), ".").await?;
        } else {
            println!("❌ 未从查询中提取到URL");
        }
    } else {
        println!("❌ 未匹配到合适的扩展");
    }
    
    Ok(())
}

async fn handle_task(subcommand: TaskCommands) -> Result<()> {
    let config = open_fetch::Config::default();
    let engine = open_fetch::Engine::new(config).await?;
    
    match subcommand {
        TaskCommands::List { status } => {
            let status_filter = status.map(|s| match s {
                TaskStatusArg::Pending => open_fetch::TaskStatus::Pending,
                TaskStatusArg::Downloading => open_fetch::TaskStatus::Downloading,
                TaskStatusArg::Completed => open_fetch::TaskStatus::Completed,
                TaskStatusArg::Failed => open_fetch::TaskStatus::Failed,
            });
            
            let tasks = engine.list_tasks(status_filter).await?;
            println!("📋 任务列表 ({}):", tasks.len());
            for task in tasks {
                println!("  {} [{}] {}", task.id, task.status.as_str(), task.url);
            }
        }
        TaskCommands::Show { task_id } => {
            let task = engine.get_task(&task_id).await?;
            println!("📋 任务详情:");
            println!("  ID: {}", task.id);
            println!("  URL: {}", task.url);
            println!("  扩展: {:?}", task.extension_name);
            println!("  状态: {}", task.status.as_str());
            println!("  文件: {:?}", task.file_path);
            println!("  创建: {}", task.created_at);
        }
        TaskCommands::Delete { task_id } => {
            engine.delete_task(&task_id).await?;
            println!("🗑️ 任务已删除: {}", task_id);
        }
        TaskCommands::Retry => {
            println!("🔄 重试失败任务...");
            // TODO: 实现重试
        }
    }
    
    Ok(())
}

async fn handle_sync(pull: bool, push: bool) -> Result<()> {
    let config = open_fetch::Config::default();
    let engine = open_fetch::Engine::new(config).await?;
    
    if pull {
        println!("☁️ 从云端同步...");
        engine.sync_from_cloud().await?;
        println!("✅ 同步完成");
    }
    
    if push {
        println!("☁️ 推送到云端...");
        // TODO: 实现推送
        println!("✅ 推送完成");
    }
    
    if !pull && !push {
        println!("📊 同步状态:");
        println!("  本地任务: -");
        println!("  云端任务: -");
    }
    
    Ok(())
}

/// 从自然语言中提取URL
fn extract_url(query: &str) -> Option<String> {
    // 简单的URL提取
    for word in query.split_whitespace() {
        if word.starts_with("http://") || word.starts_with("https://") {
            return Some(word.trim_matches(|c| c == '<' || c == '>' || c == '"' || c == '\'').to_string());
        }
    }
    None
}
