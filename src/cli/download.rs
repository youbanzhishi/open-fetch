//! 单个下载命令

use std::process::Command;

pub fn run_download(url: &str, extractor: Option<&str>, quality: &str, format: &str, output: Option<&str>) -> anyhow::Result<()> {
    let ext = extractor.unwrap_or("auto");
    let output_dir = output.unwrap_or("./downloads");
    
    println!("⬇️ 开始下载...");
    println!("   URL: {}", url);
    println!("   平台: {}", ext);
    println!("   画质: {}", quality);
    println!("   格式: {}", format);
    
    // 根据平台选择下载器
    let script = match ext {
        "bilibili" => "src/extensions/bilibili/downloader.py",
        "youtube" => "src/extensions/youtube/downloader.py",
        "douyin" => "src/extensions/douyin/downloader.py",
        "twitter" => "src/extensions/twitter/downloader.py",
        "instagram" => "src/extensions/instagram/downloader.py",
        "xiaohongshu" => "src/extensions/xiaohongshu/downloader.py",
        "universal" | "auto" | _ => "src/extensions/universal/downloader.py",
    };
    
    // 执行Python下载器
    let output = Command::new("python3")
        .arg(script)
        .arg("--url")
        .arg(url)
        .arg("--quality")
        .arg(quality)
        .arg("--format")
        .arg(format)
        .arg("--output")
        .arg(output_dir)
        .output()?;
    
    if output.status.success() {
        println!("\n✅ 下载完成!");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("\n❌ 下载失败!");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }
    
    Ok(())
}
