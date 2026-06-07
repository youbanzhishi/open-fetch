# OpenFetch - 全能下载工具

> 开源 · 跨平台 · 无限扩展

## 使命
成为开发者首选的「下载基础设施」，像 curl 一样无处不在，像 yt-dlp 一样专业。

## 核心定位
- **图片**：微博/小红书/Unsplash/微博长图
- **视频**：抖音/抖音直播/哔哩/b站/YouTube/TikTok
- **音频**：网易云/QQ音乐/QT咪咕
- **直播**：B站直播/抖音直播/虎牙/斗鱼/快手直播录制
- **压缩**：图片压缩/音频压缩/视频压缩（本地文件压缩处理）
- **浏览器插件**：Chrome/Firefox/Edge桌面端 + Safari（iOS/iPadOS苹果手机/平板）

## 四大设计原则
1. **Extension First** - 所有功能都是扩展，核心永远不改
2. **AI Native** - 每个扩展自描述能力，AI Agent 自主发现和调用
3. **无限扩展** - 新平台/新格式/新AI能力 = 新扩展，架构零修改
4. **跨端一致** - CLI/浏览器插件/手机App 共用同一扩展协议

## 技术栈
- 语言：Rust（核心）+ Python（扩展开发）
- 浏览器插件：JavaScript/TypeScript（WebExtensions API）+ Safari Web Extension
- 构建：cargo build --release
- 扩展协议：YAML 描述 + JSON Schema 输入输出

## 项目结构
```
open-fetch/
├── src/
│   ├── core/           # 核心引擎（下载/调度/扩展加载）
│   ├── extensions/     # 内置扩展（内置必装）
│   │   ├── http/       # HTTP基础下载
│   │   ├── bilibili/   # B站视频+直播
│   │   ├── douyin/     # 抖音视频+直播
│   │   ├── compress/   # 音视频图片压缩
│   │   └── ...
│   └── utils/          # 工具函数
├── browser-ext/        # 浏览器插件
│   ├── firefox/       # Firefox/Chrome/Edge（WebExtensions）
│   └── safari/        # Safari扩展（iOS/iPadOS/Mac）
├── scripts/            # 工具脚本
│   ├── new-ext.sh     # 创建新扩展脚手架
│   └── quick-start.sh # 快速启动
└── docs/              # 文档

## 发布计划
- **v0.1.0**：命令行工具 + 基础HTTP下载
- **v0.2.0**：内置扩展10个（视频/音频/图片）
- **v0.3.0**：直播录制扩展
- **v0.4.0**：压缩功能（图片/音视频）
- **v0.5.0**：浏览器插件 v1（Chrome/Firefox/Edge）
- **v0.6.0**：Safari 插件（iOS/iPadOS/Mac）
- **v0.7.0**：Web在线版（PWA，无需安装）
- **v1.0.0**：全平台覆盖 + Agent SDK + 扩展市场

## 当前状态
- [ ] 项目初始化
- [ ] 核心引擎开发
- [ ] 内置扩展开发
- [ ] CI/CD配置

## 负责人
- 主人：@小龙
- Agent：小龙（秘书管家）

---
*最后更新：2026-06-06*
