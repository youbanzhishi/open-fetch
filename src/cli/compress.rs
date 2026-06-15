//! 音视频压缩命令

use std::process::Command;

pub fn run_compress(input: &str, output: Option<&str>, crf: u32, preset: &str) -> anyhow::Result<()> {
    println!("📦 压缩文件...");
    println!("   输入: {}", input);
    if let Some(out) = output {
        println!("   输出: {}", out);
    }
    println!("   质量: CRF {}", crf);
    println!("   速度: {}", preset);
    
    let script = "src/extensions/compress/compressor.py";
    let mut args = vec!["video", "--input", input, "--crf", &crf.to_string(), "--preset", preset];
    
    if let Some(out) = output {
        args.extend(["--output", out]);
    }
    
    let result = Command::new("python3").arg(script).args(&args).output()?;
    
    if result.status.success() {
        println!("\n✅ 压缩完成!");
    } else {
        eprintln!("\n❌ 压缩失败!");
        eprintln!("{}", String::from_utf8_lossy(&result.stderr));
    }
    
    Ok(())
}
