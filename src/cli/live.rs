//! 直播录制命令

use std::process::Command;

pub fn run_live(url: &str, output: &str) -> anyhow::Result<()> {
    println!("🔴 开始录制直播...");
    println!("   URL: {}", url);
    println!("   输出: {}", output);
    println!();
    println!("按 Ctrl+C 停止录制...\n");
    
    let script = "src/extensions/live/downloader.py";
    let output = Command::new("python3")
        .arg(script)
        .arg("record")
        .arg("--url")
        .arg(url)
        .arg("--output")
        .arg(output)
        .output()?;
    
    if output.status.success() {
        println!("\n✅ 录制完成!");
    } else {
        println!("\n❌ 录制失败!");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }
    
    Ok(())
}
