//! OpenFetch - 开源全能下载器
//! 入口文件

mod cli;
mod cloud;
mod core;
mod extension;
mod extensions;
mod gui;
mod plugin;
mod runtime;
mod server;
mod sync;
mod utils;

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(clap::Parser, Debug)]
#[command(name = "open-fetch")]
#[command(version = "0.9.0")]
#[command(about = "OpenFetch - 开源全能下载器", long_about = None)]
enum Command {
    /// 启动GUI桌面应用
    #[cfg(feature = "gui")]
    Gui,
    
    /// 启动HTTP API服务器
    #[cfg(feature = "server")]
    Server {
        /// 服务器端口
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
    },
    
    /// 启动云端服务（Web UI + API + WebSocket）
    #[cfg(feature = "server")]
    Cloud {
        /// 服务端口
        #[arg(short, long, default_value_t = 3000)]
        port: u16,
        
        /// 下载目录
        #[arg(short, long, default_value = "./downloads")]
        download_dir: Option<String>,
        
        /// 绑定地址
        #[arg(short, long, default_value = "0.0.0.0")]
        host: String,
    },
    
    /// 下载视频
    Download {
        /// 视频URL
        url: String,
        
        /// 下载器类型
        #[arg(short, long)]
        extractor: Option<String>,
        
        /// 画质
        #[arg(short, long, default_value = "best")]
        quality: String,
        
        /// 输出格式
        #[arg(short, long, default_value = "mp4")]
        format: String,
        
        /// 输出目录
        #[arg(short, long)]
        output: Option<String>,
    },
    
    /// 批量下载
    Batch {
        /// URL列表文件
        #[arg(short, long)]
        file: Option<String>,
        
        /// 直接指定URL
        #[arg(short, long, num_args = 1..)]
        urls: Option<Vec<String>>,
        
        /// 并发数
        #[arg(short, long, default_value_t = 3)]
        concurrent: usize,
    },
    
    /// 压缩音视频
    Compress {
        /// 输入文件
        input: String,
        
        /// 输出文件
        #[arg(short, long)]
        output: Option<String>,
        
        /// CRF质量值
        #[arg(short, long, default_value_t = 23)]
        crf: u32,
        
        /// 编码速度
        #[arg(short, long, default_value = "medium")]
        preset: String,
    },
    
    /// 列出支持的平台
    List,
    
    /// 录制直播
    Live {
        /// 直播间URL
        url: String,
        
        /// 输出目录
        #[arg(short, long, default_value = "./downloads/live")]
        output: String,
    },
}

fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env()
            .add_directive("open_fetch=info".parse()?))
        .init();
    
    // 解析命令
    let cmd: Command = clap::Parser::parse();
    
    match cmd {
        #[cfg(feature = "gui")]
        Command::Gui => {
            println!("🖥️ 启动 OpenFetch GUI v0.9.0");
            let state = Arc::new(Mutex::new(gui::app::AppState::default()));
            gui::run_gui(state);
        }
        
        #[cfg(feature = "server")]
        Command::Server { port } => {
            println!("🚀 启动 HTTP API 服务器 v0.9.0");
            tokio::runtime::Runtime::new()?.block_on(async {
                server::start_server(port).await;
            });
        }
        
        #[cfg(feature = "server")]
        Command::Cloud { port, download_dir, host } => {
            println!("☁️ 启动 OpenFetch Cloud v0.9.0");
            let addr = format!("{}:{}", host, port).parse()?;
            let dir = download_dir.unwrap_or_else(|| "./downloads".to_string());
            tokio::runtime::Runtime::new()?.block_on(async {
                cloud::start_cloud_server(addr, dir, port).await;
            });
        }
        
        Command::Download { url, extractor, quality, format, output } => {
            cli::download::run_download(&url, extractor.as_deref(), &quality, &format, output.as_deref())?;
        }
        
        Command::Batch { file, urls, concurrent } => {
            cli::batch::run_batch(file, urls, concurrent)?;
        }
        
        Command::Compress { input, output, crf, preset } => {
            cli::compress::run_compress(&input, output.as_deref(), crf, &preset)?;
        }
        
        Command::List => {
            cli::list::show_platforms();
        }
        
        Command::Live { url, output } => {
            cli::live::run_live(&url, &output)?;
        }
    }
    
    Ok(())
}
