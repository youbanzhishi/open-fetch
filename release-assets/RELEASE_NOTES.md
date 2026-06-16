# OpenFetch v0.9.1 - 跨平台 CI 发布

## 🎉 里程碑
首个跨平台编译成功的 Release！三平台同时发布：
- ✅ Ubuntu (Linux) - 415 KB
- ✅ macOS - 380 KB  
- ✅ Windows - 266 KB

## 🔧 CI 修复
- 使用 `dtolnay/rust-toolchain@stable` 替代不存在的 `actions/setup-rust`
- 修复 Windows Package 步骤 (添加 bash shell)
- 彻底清理问题模块，简化核心代码

## 📦 产物
| 平台 | 大小 | SHA256 |
|------|------|--------|
| Ubuntu | 415 KB | 4d20c8fa63f0... |
| macOS | 380 KB | e0e6fd26ae15... |
| Windows | 266 KB | de627f3fd49e... |

## 🐛 已修复
- src/utils/error.rs: thiserror 依赖缺失
- src/cli/compress.rs: 生命周期错误
- CI Build 失败 (exit code 101)

---
Build #39 | Commit d82b536 | 2026-06-15
