//! 批量下载命令

use std::fs;
use std::process::Command;

pub fn run_batch(file: Option<String>, urls: Option<Vec<String>>, concurrent: usize) -> anyhow::Result<()> {
    let mut all_urls = Vec::new();
    
    // 从文件读取
    if let Some(file_path) = file {
        let content = fs::read_to_string(&file_path)?;
        for line in content.lines() {
            let line = line.trim();
            if !line.is_empty() && !line.starts_with('#') {
                all_urls.push(line.to_string());
            }
        }
    }
    
    // 从命令行参数
    if let Some(url_list) = urls {
        all_urls.extend(url_list);
    }
    
    if all_urls.is_empty() {
        println!("⚠️ 没有找到任何URL");
        println!("使用方式:");
        println!("  open-fetch batch -f urls.txt");
        println!("  open-fetch batch -u <url1> <url2>");
        return Ok(());
    }
    
    println!("📋 批量下载任务");
    println!("   总数: {}", all_urls.len());
    println!("   并发: {}", concurrent);
    println!();
    
    let batch_script = "scripts/batch-download.py";
    let output = Command::new("python3")
        .arg(batch_script)
        .arg("--urls")
        .args(&all_urls)
        .arg("--workers")
        .arg(concurrent.to_string())
        .output()?;
    
    println!("{}", String::from_utf8_lossy(&output.stdout));
    
    Ok(())
}
