# OpenFetch

> 全能下载工具 - 开源 · 跨平台 · AI Native · 无限扩展

## 🎯 功能特性

| 类别 | 平台/能力 |
|------|----------|
| 🎬 **视频下载** | YouTube/B站(bilibili)/抖音/微博/TikTok/Reddit 等50+平台 |
| 🎵 **音频下载** | 网易云/QQ音乐/喜马拉雅 |
| 📺 **直播录制** | B站/抖音/虎牙/斗鱼/快手 |
| 🖼️ **图片下载** | 微博/小红书/Unsplash |
| 🗜️ **文件压缩** | 图片/音频/视频压缩 |
| 🌐 **浏览器插件** | Firefox + Safari(iOS/Mac) + Chrome/Edge |

## 🚀 快速开始

### 1. 安装依赖

```bash
# Python 依赖（yt-dlp通用下载器）
./scripts/install-deps.sh

# 或者手动安装
pip3 install yt-dlp requests aiohttp
```

### 2. 编译运行

```bash
git clone https://github.com/youbanzhishi/open-fetch.git
cd open-fetch
cargo build --release
./target/release/open-fetch --help
```

### 3. 启动服务

```bash
# 启动HTTP API服务器（供浏览器插件调用）
./target/release/open-fetch server --port 8080

# 或者直接下载
./target/release/open-fetch download "https://www.bilibili.com/video/BVxxx"
```

### 4. 安装浏览器插件

```bash
# Firefox
# about:debugging → 此 Firefox → 临时加载附加组件 → 选择 browser-ext/firefox/manifest.json

# Safari (iOS/Mac)
# Xcode 打开 browser-ext/safari → 签名 → 运行

# Chrome/Edge
# chrome://extensions → 开发者模式 → 加载已解压扩展程序 → browser-ext/firefox
```

## 💡 使用示例

```bash
# AI语义下载 - 说人话就行
open-fetch ai "下载这个B站视频 https://bilibili.com/video/BVxxx"

# 直接下载
open-fetch download "https://youtube.com/watch?v=xxx"

# 列出扩展
open-fetch ext list

# 查看帮助
open-fetch --help
```

## 🏗️ 架构设计

### Extension First

```
┌──────────────────────────────────────────────────────────┐
│                     OpenFetch Core                         │
│  ┌──────────────────────────────────────────────────┐   │
│  │ Plugin Registry | Runtime | Hooks | AI Manifest    │   │
│  └──────────────────────────────────────────────────┘   │
│                         ↓                                 │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐        │
│  │universal│ │bilibili │ │ douyin  │ │ youtube │  ...   │
│  │(yt-dlp) │ │         │ │         │ │         │        │
│  └─────────┘ └─────────┘ └─────────┘ └─────────┘        │
└──────────────────────────────────────────────────────────┘
```

### 核心创新

1. **AI Manifest** - 每个扩展自描述能力关键词，AI无需预设规则
2. **DAW四柱架构** - Plugin API + Registry + Runtime + Hooks
3. **Extension First** - 核心永远不改，新功能=新扩展
4. **多端同步** - CLI/浏览器/Safari/Web/AI Agent 无缝切换

### 技术栈

- **核心引擎**: Rust (async/await + tokio)
- **下载器**: Python (yt-dlp)
- **浏览器插件**: JavaScript (Manifest V3)
- **HTTP API**: axum

## 📦 扩展开发

### 创建新扩展

1. 创建扩展目录
```bash
mkdir src/extensions/myplatform
```

2. 编写 extension.yaml
```yaml
name: myplatform
version: 1.0.0
description: "我的平台下载"

ai_manifest:
  keywords: ["我的平台", "myplatform"]
  examples:
    - "下载这个myplatform视频 https://myplatform.com/video/xxx"

platforms:
  - "myplatform.com"

capabilities:
  - video_download
  - audio_download
```

3. 编写下载脚本 (myplatform.py)
```python
import json

def resolve(params):
    url = params["url"]
    # 解析URL，获取视频信息
    return {"title": "...", "files": [...]}
```

4. 放入 `src/extensions/myplatform/`

## 🔧 API接口

HTTP API 服务器提供以下接口：

| 接口 | 方法 | 描述 |
|------|------|------|
| `/api/health` | GET | 健康检查 |
| `/api/download` | POST | 创建下载任务 |
| `/api/queue` | GET | 获取下载队列 |
| `/api/extensions` | GET | 列出扩展 |
| `/api/match` | GET | AI意图匹配 |
| `/api/sync` | POST | 多端同步 |

## 📁 项目结构

```
open-fetch/
├── src/
│   ├── main.rs              # 入口
│   ├── lib.rs               # 库导出
│   ├── core/                # 核心引擎
│   ├── extension/           # 扩展系统
│   ├── plugin/              # DAW四柱架构
│   ├── runtime/             # Python运行时
│   ├── server/              # HTTP API服务器
│   ├── sync/                # 多端同步
│   ├── cli/                 # 命令行
│   └── extensions/          # 内置扩展
│       ├── universal/       # yt-dlp通用下载
│       └── bilibili/        # B站专用
├── browser-ext/
│   ├── firefox/             # Firefox插件
│   └── safari/              # Safari插件
└── scripts/
    └── install-deps.sh      # 依赖安装
```

## 🧪 测试

```bash
cargo test          # 运行所有测试
cargo clippy        # 代码检查
```

## 📄 许可证

MIT License

## 🙏 致谢

- [yt-dlp](https://github.com/yt-dlp/yt-dlp) - 通用下载器核心
- [Reaper](https://www.reaper.fm) - DAW架构灵感
