# OpenFetch - 开源全能下载器

![Version](https://img.shields.io/badge/version-0.8.0-blue)
![Rust](https://img.shields.io/badge/Rust-1.70+-orange)
![License](https://img.shields.io/badge/license-MIT-green)

## 🚀 一句话介绍

史无前例的全能下载工具——**12+平台覆盖**、**桌面GUI**、**HTTP API**、**无限扩展**，让下载变得简单。

## ✨ 核心特性

| 特性 | 说明 |
|------|------|
| **🖥️ 桌面GUI** | egui跨平台桌面应用，拖拽操作 |
| **🌐 HTTP API** | RESTful接口，供浏览器插件和外部调用 |
| **12+平台** | B站、抖音、YouTube、微博、小红书、知乎、Twitter、Instagram、快手 |
| **无限扩展** | 插件系统支持任何人贡献新平台 |
| **AI Native** | 插件自描述，AI自主发现和调用 |
| **直播录制** | 多平台直播实时录制 |
| **批量下载** | 并发下载，文件导入 |

## 📦 支持的平台

### 视频平台
| 平台 | 图标 | 说明 |
|------|------|------|
| Bilibili | 🟢 | 视频/番剧/直播/漫画，支持4K/弹幕/字幕 |
| YouTube | 🔴 | 视频/Shorts/音乐，支持4K/8K/HDR |
| 抖音/TikTok | 🎵 | 无水印下载，支持作者批量 |
| 微博 | 📱 | 视频下载 |
| Twitter/X | 🐦 | 视频/图片下载 |
| Instagram | 📸 | 图片/视频/Reels/Stories |
| 小红书 | 📖 | 笔记/视频/图文 |
| 知乎 | 💬 | 文章/视频/问答 |
| 快手 | 📺 | 视频/直播下载 |

### 工具扩展
- 🔇 **直播录制** - 全平台直播录制
- 📦 **音视频压缩** - FFmpeg批量压缩
- 🌐 **通用下载** - 基于yt-dlp，支持50+平台

## 🏗 架构设计

```
┌─────────────────────────────────────────────────────┐
│               OpenFetch v0.8.0                      │
├─────────────────────────────────────────────────────┤
│  ┌─────────┐  ┌─────────┐  ┌─────────┐             │
│  │   CLI   │  │  GUI    │  │ Server  │             │
│  │ 命令行   │  │ 桌面应用  │  │ HTTP API │             │
│  └────┬────┘  └────┬────┘  └────┬────┘             │
│       └───────────┴───────────┘                    │
│                       │                             │
│              ┌────────▼────────┐                    │
│              │  Extension       │                   │
│              │  Registry (12+) │                   │
│              └────────┬────────┘                    │
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

### 编译
```bash
# CLI模式
cargo build --release --features cli

# HTTP服务器模式
cargo build --release --features server

# GUI桌面模式
cargo build --release --features gui

# 全功能模式
cargo build --release --features all
```

## 📖 使用方式

### 1. 🖥️ GUI桌面应用
```bash
cargo run --release --features gui -- gui
```
- 拖拽URL自动识别平台
- 实时下载进度
- 任务管理

### 2. 🌐 HTTP API服务
```bash
./target/release/open-fetch server --port 8080
```

**API接口:**
```bash
# 下载视频
curl -X POST http://localhost:8080/api/download \
  -H "Content-Type: application/json" \
  -d '{"url":"https://bilibili.com/video/BVxxx","quality":"1080p"}'

# 获取任务列表
curl http://localhost:8080/api/tasks

# 获取扩展列表
curl http://localhost:8080/api/extensions
```

### 3. 💻 CLI命令行
```bash
# 单个下载
open-fetch download "https://bilibili.com/video/BVxxx" --quality 1080p

# 批量下载
open-fetch batch -u "url1" "url2" "url3" --concurrent 3

# 批量导入
open-fetch batch -f urls.txt

# 音视频压缩
open-fetch compress input.mp4 --crf 23

# 直播录制
open-fetch live "https://live.bilibili.com/xxx"

# 列出支持的平台
open-fetch list
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
1. Safari偏好设置 → 启用「开发菜单」
2. 选择「加载扩展」
3. 加载 `browser-ext/safari/`

## 📁 项目结构

```
open-fetch/
├── src/
│   ├── cli/           # 命令行界面
│   ├── core/          # 核心引擎
│   ├── extension/     # 扩展系统
│   ├── extensions/    # 下载扩展 (12+)
│   │   ├── bilibili/
│   │   ├── youtube/
│   │   ├── douyin/
│   │   └── ...
│   ├── gui/           # 桌面GUI (egui)
│   ├── server/        # HTTP API服务器
│   └── main.rs        # 入口
├── browser-ext/       # 浏览器插件
│   ├── chrome/
│   ├── firefox/
│   └── safari/
└── scripts/           # 辅助脚本
```

## 🎯 Roadmap

- [x] v0.5.0 - 核心引擎 + 浏览器插件
- [x] v0.6.0 - 全平台扩展
- [x] v0.7.0 - 批量下载 + 一键安装
- [x] **v0.8.0** - **GUI桌面应用 + HTTP API增强**
- [ ] v0.9.0 - 云端下载服务
- [ ] v1.0.0 - 全功能Release

## 🤝 贡献

欢迎提交Issue和PR！

## 📄 License

MIT License

## 🔗 Links

- [GitHub](https://github.com/youbanzhishi/open-fetch)
- [问题反馈](https://github.com/youbanzhishi/open-fetch/issues)
