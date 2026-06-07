# OpenFetch

> 全能下载工具 - 开源 · 跨平台 · 无限扩展

## 功能特性

- 🖼️ **图片下载**：微博/小红书/Unsplash/微博长图
- 🎬 **视频下载**：抖音/b站/YouTube/TikTok
- 🎵 **音频下载**：网易云/QQ音乐/QT咪咕
- 📺 **直播录制**：B站/抖音/虎牙/斗鱼/快手
- 🗜️ **文件压缩**：图片压缩/音频压缩/视频压缩
- 📦 **通用下载**：直链/分片/M3U8/BCD
- 🌐 **浏览器插件**：Chrome/Firefox/Edge + **Safari（iOS/iPadOS）**

## 设计理念

**Extension First** - 所有功能都是扩展，核心永远不改

**AI Native** - 每个扩展自描述能力，AI Agent 自主发现和调用

**无限扩展** - 新平台/新格式/新AI能力 = 新扩展，架构零修改

```
┌─────────────────────────────────────────┐
│              OpenFetch Core              │
│   ┌─────────┐ ┌─────────┐ ┌─────────┐  │
│   │ bilibili│ │douyin   │ │ http    │  │
│   └─────────┘ └─────────┘ └─────────┘  │
│   ┌─────────┐ ┌─────────┐ ┌─────────┐  │
│   │ netease │ │ weibo   │ │ AI-EXT  │  │
│   └─────────┘ └─────────┘ └─────────┘  │
│   ┌─────────┐ ┌─────────┐ ┌─────────┐  │
│   │ compress│ │ live    │ │ future  │  │
│   └─────────┘ └─────────┘ └─────────┘  │
└─────────────────────────────────────────┘
```

## 快速开始

### 安装
```bash
# 从源码编译
git clone https://github.com/youbanzhishi/open-fetch.git
cd open-fetch
cargo build --release

# 运行
./target/release/open-fetch --help

# 安装浏览器插件
# Chrome/Edge：打开 chrome://extensions/ → 开发者模式 → 加载已解压的扩展程序 → 选择 browser-ext/firefox
# Firefox：打开 about:debugging → 此 Firefox → 临时加载附加组件 → 选择 browser-ext/firefox/manifest.json
# Safari（iOS/iPadOS/Mac）：Xcode 打开 browser-ext/safari → 运行到设备
```

### 使用
```bash
# 基础下载
open-fetch https://example.com/file.mp4

# 指定平台
open-fetch bilibili --url "https://www.bilibili.com/video/BVxxx"

# 列出可用扩展
open-fetch ext list

# 安装扩展
open-fetch ext install weibo
```

## 扩展开发

扩展采用 YAML + Python 协议：

```yaml
name: weibo
version: 1.0.0
description: "微博图片/视频下载"
platform: weibo.com
entry: weibo.py

input_schema:
  url:
    type: string
    required: true
    description: "微博链接"

output_schema:
  files:
    type: array
    items:
      type: object
      properties:
        url: { type: string }
        filename: { type: string }
```

详见 [扩展开发指南](docs/extension-dev-guide.md)

## 架构设计

### 核心组件
- `core/downloader.rs` - 下载引擎
- `core/extension.rs` - 扩展加载器
- `core/scheduler.rs` - 任务调度
- `core/resolver.rs` - 平台解析

### 扩展协议
- YAML 描述扩展元信息
- JSON Schema 定义输入输出
- Python/Rust 实现业务逻辑

## 贡献

欢迎提交扩展！请参考 `docs/extension-dev-guide.md`

## 许可证

MIT License

## 赞助

如果这个项目对你有帮助，欢迎赞助 ☕
