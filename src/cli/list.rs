//! 列出支持的平台

pub fn show_platforms() {
    println!("📦 OpenFetch 支持的平台扩展 (v0.8.0)\n");
    
    println!("=== 视频平台 ===");
    println!("  🟢 bilibili    - B站 (视频/番剧/直播/漫画)");
    println!("  🔴 youtube     - YouTube (支持4K/8K)");
    println!("  🎵 douyin      - 抖音/TikTok (无水印)");
    println!("  📱 weibo       - 微博视频");
    println!("  🐦 twitter     - Twitter/X");
    println!("  📸 instagram   - Instagram");
    println!("  📖 xiaohongshu - 小红书");
    println!("  💬 zhihu       - 知乎");
    println!("  📺 kuishou     - 快手");
    
    println!("\n=== 工具扩展 ===");
    println!("  🔇 live       - 直播录制 (B站/抖音/斗鱼/虎牙/Twitch)");
    println!("  📦 compress   - 音视频压缩 (FFmpeg)");
    
    println!("\n=== 通用 ===");
    println!("  🌐 universal  - 自动检测 (基于yt-dlp，支持50+平台)");
    
    println!("\n总计: 12个专用扩展 + 1个通用扩展");
}
