//! OpenFetch - 开源全能下载器
//! 入口文件

mod cli;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "open-fetch")]
#[command(version = "0.9.0")]
#[command(about = "OpenFetch - 开源全能下载器", long_about = None)]
enum Command {
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
    },
    
    /// 列出支持的平台
    List,
}

fn main() -> anyhow::Result<()> {
    let cmd: Command = Command::parse();
    
    match cmd {
        Command::Download { url, extractor, quality } => {
            cli::download::run_download(&url, extractor.as_deref(), &quality)?;
        }
        
        Command::List => {
            cli::list::show_platforms();
        }
    }
    
    Ok(())
}
