# OpenFetch - 开源全能下载器

![Version](https://img.shields.io/badge/version-0.7.0-blue)
![Rust](https://img.shields.io/badge/Rust-1.70+-orange)
![License](https://img.shields.io/badge/license-MIT-green)

## 🚀 一句话介绍

史无前例的全能下载工具——**12+平台覆盖**、**无限扩展**、**AI Native**，让下载变得简单。

## ✨ 核心特性

| 特性 | 说明 |
|------|------|
| **12+平台** | B站、抖音、YouTube、微博、小红书、知乎、Twitter、Instagram、快手... |
| **多端支持** | CLI命令行 / 桌面端 / 浏览器插件(Chrome/Firefox/Safari) |
| **无限扩展** | 插件系统支持任何人贡献新平台 |
| **AI Native** | 插件自描述，AI自主发现和调用 |
| **直播录制** | 多平台直播实时录制 |
| **音视频压缩** | FFmpeg驱动，高质量压缩 |
| **批量下载** | 支持文件导入、并发下载 |

## 📦 支持的平台

### 视频平台
- 🟢 **Bilibili** - 视频/番剧/直播/漫画，支持4K/弹幕/字幕/封面
- 🔴 **YouTube** - 视频/Shorts/音乐，支持4K/8K/HDR
- 🎵 **抖音/TikTok** - 无水印下载，支持作者批量
- 📱 **微博** - 视频下载
- 🎬 **西瓜视频/今日头条**
- 📺 **斗鱼/虎牙/快手**
- 🐦 **Twitter/X** - 视频/图片下载
- 📸 **Instagram** - 图片/视频/Reels/Stories
- 📖 **小红书** - 笔记/视频/图文
- 💬 **知乎** - 文章/视频/问答

### 工具扩展
- 🔇 **直播录制** - 全平台直播录制
- 📦 **音视频压缩** - FFmpeg批量压缩
- 📋 **批量下载** - 多任务并发

## 🏗 架构设计

```
┌─────────────────────────────────────────────────────┐
│                    OpenFetch v0.7.0                 │
├─────────────────────────────────────────────────────┤
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐ │
│  │   CLI   │  │  HTTP   │  │Browser  │  │ Batch   │ │
│  │         │  │ Server  │  │ Plugin  │  │Download │ │
│  └────┬────┘  └────┬────┘  └────┬────┘  └────┬────┘ │
│       └───────────┴───────────┴───────────┘       │
│                       │                             │
│              ┌────────▼────────┐                   │
│              │  Extension       │                   │
│              │  Registry (12+)   │                   │
│              └────────┬────────┘                   │
│       ┌──────────────┼──────────────┐              │
│  ┌────▼────┐   ┌────▼────┐   ┌────▼────┐         │
│  │ bilibili│   │youtube  │   │douyin   │  ...    │
│  └─────────┘   └─────────┘   └─────────┘         │
└─────────────────────────────────────────────────────┘
```

## 🔧 安装

### 一键安装
```bash
git clone https://github.com/youbanzhishi/open-fetch.git
cd open-fetch
bash scripts/install.sh
```

### 手动安装

**依赖:**
```bash
# Python (核心下载器)
pip install yt-dlp requests aiohttp

# FFmpeg (音视频压缩)
# Linux:   sudo apt install ffmpeg
# macOS:   brew install ffmpeg
# Windows:  https://ffmpeg.org/download.html
```

**编译:**
```bash
cargo build --release
```

## 📖 使用方式

### 1. CLI命令行
```bash
# B站视频
./target/release/open-fetch bilibili "https://www.bilibili.com/video/BVxxx"

# YouTube 4K
./target/release/open-fetch youtube "https://youtube.com/watch?v=xxx" --quality 2160p

# 抖音无水印
./target/release/open-fetch douyin "https://v.douyin.com/xxx"

# Twitter/X
./target/release/open-fetch twitter "https://twitter.com/xxx"

# 小红书
./target/release/open-fetch xiaohongshu "https://xiaohongshu.com/xxx"

# 直播录制
./target/release/open-fetch live "https://live.bilibili.com/xxx"

# 视频压缩
./target/release/open-fetch compress input.mp4 --crf 23
```

### 2. 批量下载
```bash
# 从URL列表文件
python3 scripts/batch-download.py -f urls.txt

# 直接指定URL
python3 scripts/batch-download.py -u "https://bilibili.com/video/BV1" "https://youtube.com/watch?v=xxx"

# 并发数控制
python3 scripts/batch-download.py -f urls.txt -w 5
```

**urls.txt格式:**
```
# 每行一个URL，可选指定extractor和质量
https://www.bilibili.com/video/BVxxx|bilibili|1080p
https://youtube.com/watch?v=xxx|youtube|4k
https://v.douyin.com/xxx|douyin|best
```

### 3. 桌面端 + 浏览器插件

```bash
# 启动桌面端服务
./target/release/open-fetch server --port 8080

# 浏览器插件 (Chrome/Firefox/Safari)
# 打开扩展管理页面，加载对应目录
```

## 🌐 浏览器插件

### Chrome
1. 打开 `chrome://extensions/`
2. 开启「开发者模式」
3. 点击「加载已解压的扩展程序」
4. 选择 `browser-ext/chrome/`

### Firefox
1. 打开 `about:addons`
2. 点击齿轮 → 「安装附加组件」
3. 选中 `browser-ext/firefox/manifest.json`

### Safari
1. 打开Safari偏好设置 → 启用「开发菜单」
2. 选择「加载扩展」
3. 加载 `browser-ext/safari/`

## 🛠 扩展开发

### 创建新扩展

1. 创建目录 `src/extensions/{your-platform}/`
2. 编写 `extension.yaml`:
```yaml
id: my-platform
name: 我的平台
version: 1.0.0
description: 描述
platforms: [video, audio]
ai_capable: true
```

3. 实现下载器 `downloader.py`
4. 注册到 `src/extensions/mod.rs`
5. 提交PR

## 📝 API 文档

### POST /api/download
```bash
curl -X POST http://localhost:8080/api/download \
  -H "Content-Type: application/json" \
  -d '{"url":"https://...","extractor":"bilibili"}'
```

### GET /api/extensions
获取已安装的扩展列表

### GET /api/downloads
获取下载历史

## 🎯 Roadmap

- [x] v0.5.0 - 核心引擎 + 浏览器插件
- [x] v0.6.0 - 全平台扩展
- [x] v0.7.0 - 批量下载 + 12平台覆盖
- [ ] v0.8.0 - GUI桌面应用
- [ ] v1.0.0 - 全功能Release

## 📄 License

MIT License

## 🔗 Links

- [GitHub](https://github.com/youbanzhishi/open-fetch)
- [问题反馈](https://github.com/youbanzhishi/open-fetch/issues)
