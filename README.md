# OpenFetch - 开源全能下载器

![Version](https://img.shields.io/badge/version-0.6.0-blue)
![Rust](https://img.shields.io/badge/Rust-1.70+-orange)
![License](https://img.shields.io/badge/license-MIT-green)

## 🚀 一句话介绍

史无前例的全能下载工具——多平台覆盖、无限扩展、AI Native，让下载变得简单。

## ✨ 核心特性

| 特性 | 说明 |
|------|------|
| **50+平台** | B站、抖音、YouTube、微博、斗鱼、虎牙、TikTok... |
| **多端支持** | CLI命令行 / 桌面端 / 浏览器插件 / AI Agent |
| **无限扩展** | 插件系统支持任何人贡献新平台 |
| **AI Native** | 插件自描述，AI自主发现和调用 |
| **直播录制** | 多平台直播实时录制 |
| **音视频压缩** | FFmpeg驱动，高质量压缩 |

## 📦 支持的平台

### 视频平台
- 🟢 **Bilibili** - 视频/番剧/直播/漫画
- 🔴 **YouTube** - 视频/Shorts/音乐
- 🎵 **抖音/TikTok** - 无水印下载
- 📱 **微博** - 视频下载
- 🎬 **西瓜视频/今日头条**
- 📺 **斗鱼/虎牙/快手**

### 工具扩展
- 🔇 **直播录制** - 全平台直播录制
- 📦 **音视频压缩** - FFmpeg批量压缩

## 🏗 架构设计

```
┌─────────────────────────────────────────────────────┐
│                    OpenFetch                         │
├─────────────────────────────────────────────────────┤
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐ │
│  │   CLI   │  │  HTTP   │  │Browser  │  │ Agent   │ │
│  │         │  │ Server  │  │ Plugin  │  │ Client  │ │
│  └────┬────┘  └────┬────┘  └────┬────┘  └────┬────┘ │
│       └───────────┴───────────┴───────────┘       │
│                       │                             │
│              ┌────────▼────────┐                   │
│              │  Extension       │                   │
│              │  Registry        │                   │
│              │  (四柱架构)       │                   │
│              └────────┬────────┘                   │
│       ┌──────────────┼──────────────┐              │
│  ┌────▼────┐   ┌────▼────┐   ┌────▼────┐         │
│  │ bilibili│   │youtube  │   │douyin   │  ...    │
│  └─────────┘   └─────────┘   └─────────┘         │
└─────────────────────────────────────────────────────┘
```

## 🔧 安装

### 依赖
```bash
# Python (核心下载器)
pip install yt-dlp requests aiohttp

# FFmpeg (音视频压缩)
# Linux
sudo apt install ffmpeg
# macOS
brew install ffmpeg
# Windows
# 下载 https://ffmpeg.org/download.html
```

### 编译
```bash
git clone https://github.com/youbanzhishi/open-fetch.git
cd open-fetch
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

# 直播录制
./target/release/open-fetch live "https://live.bilibili.com/xxx"

# 视频压缩
./target/release/open-fetch compress input.mp4 --crf 23
```

### 2. 桌面端 + 浏览器插件

```bash
# 启动桌面端服务
./target/release/open-fetch server --port 8080

# 浏览器插件 (Chrome/Firefox/Safari)
# 打开 chrome://extensions 或 about:addons
# 加载 browser-ext/chrome/ 或 browser-ext/firefox/
```

### 3. HTTP API

```bash
# 下载视频
curl -X POST http://localhost:8080/api/download \
  -H "Content-Type: application/json" \
  -d '{"url":"https://bilibili.com/xxx","extractor":"bilibili"}'

# 查看状态
curl http://localhost:8080/api/status

# 扩展列表
curl http://localhost:8080/api/extensions
```

## 🌐 浏览器插件

### Chrome
1. 打开 `chrome://extensions/`
2. 开启「开发者模式」
3. 点击「加载已解压的扩展程序」
4. 选择 `browser-ext/chrome/`

### Firefox
1. 打开 `about:addons`
2. 点击齿轮图标 → 「安装附加组件」
3. 选择「从文件安装」
4. 选中 `browser-ext/firefox/` 目录下的 `manifest.json`

### Safari
1. 打开Safari偏好设置
2. 启用「开发菜单」
3. 选择「加载扩展」
4. 加载 `browser-ext/safari/`

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
apis:
  - method: download
    params:
      - name: url
        type: string
        required: true
ai_capable: true
```

3. 实现下载器 `downloader.py`
4. 注册到 `src/extensions/mod.rs`
5. 提交PR

## 📁 项目结构

```
open-fetch/
├── src/
│   ├── cli/           # 命令行界面
│   ├── core/          # 核心引擎
│   ├── extension/     # 扩展系统
│   ├── runtime/       # 运行时 (Python/JS)
│   ├── server/        # HTTP API服务
│   └── sync/          # 同步管理
├── browser-ext/
│   ├── chrome/        # Chrome插件
│   ├── firefox/       # Firefox插件
│   └── safari/        # Safari插件
├── src/extensions/    # 下载扩展
│   ├── bilibili/
│   ├── youtube/
│   ├── douyin/
│   └── ...
├── scripts/           # 辅助脚本
├── docs/              # 文档
└── README.md
```

## 📝 API 文档

### POST /api/download
下载视频
```json
{
  "url": "https://...",
  "extractor": "bilibili",
  "quality": "1080p",
  "format": "mp4"
}
```

### GET /api/extensions
获取已安装的扩展列表

### GET /api/downloads
获取下载历史

### POST /api/compress
压缩音视频
```json
{
  "input": "/path/to/video.mp4",
  "crf": 23,
  "preset": "medium"
}
```

## 🎯 Roadmap

- [x] v0.5.0 - 核心引擎 + 浏览器插件
- [x] v0.6.0 - 全平台扩展
- [ ] v0.7.0 - GUI桌面应用
- [ ] v0.8.0 - 云端下载服务
- [ ] v1.0.0 - 全功能Release

## 🤝 贡献

欢迎提交Issue和PR！

## 📄 License

MIT License
