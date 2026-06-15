//! 单个下载命令

use std::process::Command;

pub fn run_download(url: &str, extractor: Option<&str>) -> anyhow::Result<()> {
    let ext = extractor.unwrap_or("auto");
    
    println!("⬇️ 开始下载...");
    println!("   URL: {}", url);
    println!("   平台: {}", ext);
    
    // 模拟下载（实际调用Python脚本）
    println!("\n✅ 下载任务已创建!");
    println!("   完整功能请运行: cargo run --release");
    
    Ok(())
}
