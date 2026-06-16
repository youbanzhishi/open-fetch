# OpenFetch v0.9.2 - 云端服务 + 跨平台 CI

## 🎉 里程碑
首个包含完整 Cloud 模块的跨平台 Release！

## 📦 产物
| 平台 | 大小 |
|------|------|
| Ubuntu | 656 KB |
| macOS | 602 KB |
| Windows | 601 KB |

## ✨ 新增功能
- ☁️ **Cloud 模块**：RESTful API + WebSocket 实时推送
- 🌐 **Web UI**：主界面、任务列表、服务设置
- 📡 **API 端点**：/api/tasks, /api/batch, /api/stats, /api/platforms

## 🔧 CI 修复
- 修复可选依赖配置（axum/tokio/tower 等）
- 修复函数参数不匹配问题
- 三平台编译全部通过

## 🚀 使用
```bash
# CLI 下载
./open-fetch download <URL>

# 启动云端服务
./open-fetch cloud --port 3000

# 查看支持的平台
./open-fetch list
```

---
Build #45 | Commit a9edc6f | 2026-06-16
