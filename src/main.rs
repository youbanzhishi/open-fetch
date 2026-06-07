//! OpenFetch - 史无前例的全能内容获取平台
//! 
//! 运行命令：cargo run -- --help

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    open_fetch::cli::run().await
}
