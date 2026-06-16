//! OpenFetch - 开源全能下载器
//! 入口文件

mod cli;
#[cfg(feature = "cloud")]
mod cloud;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(clap::Parser, Debug)]
#[command(name = "open-fetch")]
#[command(version = "0.9.2")]
#[command(about = "OpenFetch - 开源全能下载器", long_about = None)]
enum Command {
    /// 启动云端服务（Web UI + API + WebSocket）
    #[cfg(feature = "cloud")]
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
    
    /// 列出支持的平台
    List,
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
        #[cfg(feature = "cloud")]
        Command::Cloud { port, download_dir, host } => {
            println!("☁️ 启动 OpenFetch Cloud v0.9.2");
            let addr = format!("{}:{}", host, port).parse()?;
            let dir = download_dir.unwrap_or_else(|| "./downloads".to_string());
            tokio::runtime::Runtime::new()?.block_on(async {
                cloud::start_cloud_server(addr, dir, port).await;
            });
        }
        
        Command::Download { url, extractor, quality, format, output } => {
            cli::download::run_download(&url, extractor.as_deref(), &quality, &format, output.as_deref())?;
        }
        
        Command::List => {
            cli::list::show_platforms();
        }
    }
    
    Ok(())
}
