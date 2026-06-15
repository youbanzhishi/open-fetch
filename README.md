# ☁️ OpenFetch - 开源全能下载器

[![Version](https://img.shields.io/badge/version-v0.9.0-blue.svg)](https://github.com/youbanzhishi/open-fetch)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)

> 一款开源、跨平台、高性能的全能下载工具，支持B站、油管、抖音、微博等全平台视频下载。

## ✨ 特性

- 🚀 **全平台支持**: B站/YouTube/抖音/微博/西瓜视频/通用链接
- 🎯 **多种模式**: CLI命令行 / GUI桌面应用 / 云端Web服务
- ⚡ **高性能**: Rust编写，纯异步IO，高并发下载
- 🔌 **插件扩展**: 支持自定义扩展，按需加载
- ☁️ **云端服务**: Web UI + RESTful API + WebSocket实时推送
- 📦 **批量下载**: 支持文件列表批量处理
- 🛠️ **灵活配置**: 画质/格式/并发数可调

## 📦 安装

### 从源码编译

```bash
git clone https://github.com/youbanzhishi/open-fetch.git
cd open-fetch

# 仅CLI
cargo build --release

# CLI + GUI
cargo build --release --features gui

# 完整功能(含云服务)
cargo build --release --features all
```

### 下载预编译二进制

前往 [Releases](https://github.com/youbanzhishi/open-fetch/releases) 页面下载对应平台的二进制文件。

## 🚀 快速开始

### 1. 命令行模式 (CLI)

```bash
# 单个视频下载
open-fetch download "https://www.bilibili.com/video/BV1xx..."

# 指定画质和格式
open-fetch download "https://www.youtube.com/watch?v=xxx" -q 1080p -f mp4

# 批量下载
open-fetch batch -f urls.txt -c 5

# 压缩视频
open-fetch compress input.mp4 -o output.mp4 --crf 23

# 查看支持的平台
open-fetch list
```

### 2. GUI桌面应用

```bash
open-fetch gui
```

- 📺 图形化下载界面
- ⏳ 实时进度显示
- ⚙️ 可视化设置面板
- 🖥️ 跨平台支持 (Windows/macOS/Linux)

### 3. ☁️ 云端服务 (v0.9.0新增)

```bash
# 启动云端服务
open-fetch cloud --port 3000 --download-dir ./downloads

# 或指定绑定地址
open-fetch cloud -p 8080 -h 0.0.0.0
```

启动后访问:
- **Web UI**: http://localhost:3000/
- **API文档**: http://localhost:3000/api
- **WebSocket**: ws://localhost:3000/ws

## ☁️ 云端服务 API

### 创建下载任务

```bash
curl -X POST http://localhost:3000/api/tasks \
  -H "Content-Type: application/json" \
  -d '{"url": "https://www.bilibili.com/video/BV1xx...", "platform": "bilibili"}'
```

### 获取任务列表

```bash
curl http://localhost:3000/api/tasks
```

### 批量创建任务

```bash
curl -X POST http://localhost:3000/api/batch \
  -H "Content-Type: application/json" \
  -d '{"urls": ["url1", "url2", "url3"]}'
```

### 获取统计信息

```bash
curl http://localhost:3000/api/stats
```

### WebSocket实时推送

```javascript
const ws = new WebSocket('ws://localhost:3000/ws');

ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log('任务更新:', data);
  
  // 类型: task_created, task_updated, task_completed, task_failed
};
```

## 📁 项目结构

```
open-fetch/
├── src/
│   ├── cli/           # 命令行模块
│   ├── cloud/         # 云端服务模块 (v0.9.0)
│   │   ├── api.rs     # RESTful API
│   │   ├── auth.rs    # 认证系统
│   │   ├── state.rs   # 状态管理
│   │   ├── websocket.rs  # WebSocket
│   │   └── web/       # Web UI静态文件
│   ├── core/          # 核心下载逻辑
│   ├── extensions/   # 平台扩展
│   │   └── bilibili/  # B站扩展示例
│   ├── gui/           # GUI桌面应用
│   └── server/        # HTTP API服务
├── browser-ext/       # 浏览器插件
├── scripts/           # 工具脚本
├── docs/              # 文档
└── Cargo.toml
```

## 🛠️ 开发指南

### 添加新平台扩展

1. 在 `src/extensions/` 创建新目录
2. 实现 `Extension` trait
3. 在 `extensions/mod.rs` 注册

```rust
use crate::extension::{Extension, DownloadResult};

pub struct MyPlatform;

impl Extension for MyPlatform {
    fn name(&self) -> &str { "my-platform" }
    
    fn supports(&self, url: &str) -> bool {
        url.contains("myplatform.com")
    }
    
    async fn fetch(&self, url: &str) -> anyhow::Result<DownloadResult> {
        // 实现获取逻辑
        Ok(DownloadResult { ... })
    }
}
```

### Feature flags

| Feature | 描述 |
|---------|------|
| `cli` | 命令行界面 (默认) |
| `server` | HTTP API服务 |
| `gui` | GUI桌面应用 |
| `cloud` | 云端Web服务 |
| `all` | 启用所有功能 |

## 📊 支持的平台

| 平台 | 状态 | 说明 |
|------|------|------|
| 📺 哔哩哔哩 | ✅ 完善 | 支持番剧/直播/弹幕 |
| ▶️ YouTube | ✅ 完善 | 支持4K/字幕 |
| 🎵 抖音 | ✅ 完善 | 支持合集 |
| 🌐 微博视频 | ✅ 完善 | - |
| 🍉 西瓜视频 | ✅ 完善 | - |
| 🌾 通用链接 | ✅ 完善 | 支持任意直链 |

## 📜 License

MIT License - 详见 [LICENSE](LICENSE) 文件

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📚 相关文档

- [架构设计文档](./docs/architecture.md)
- [插件开发指南](./docs/plugin-guide.md)
- [API参考文档](./docs/api.md)

---

<div align="center">
  <p><strong>Made with ❤️ by Rust</strong></p>
  <p>如果对你有帮助，欢迎 ⭐ Star！</p>
</div>
