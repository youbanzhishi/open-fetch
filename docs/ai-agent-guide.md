# OpenFetch AI Agent 集成

> 让 AI Agent 能自主发现和使用 OpenFetch 的下载能力

## 核心理念

每个扩展自带 **AI-Readable Manifest**，AI Agent 无需预设规则，通过扩展自描述能力自动理解如何使用。

## AI 调用协议

### 1. 扩展发现
```bash
# AI Agent 查询可用扩展
open-fetch ext discover --ai-readable

# 返回每个扩展的 AI Manifest
{
  "name": "bilibili",
  "ai_description": "下载B站视频和直播，支持批量、弹幕、封面",
  "intent_keywords": ["b站", "bilibili", "B站视频", "B站直播"],
  "input_schema": {...},
  "ai_examples": [
    "下载这个BV号的视频",
    "批量下载UP主的所有视频",
    "下载弹幕和封面"
  ]
}
```

### 2. 语义调用
```bash
# AI 自然语言调用
open-fetch ai "下载这个B站视频 https://bilibili.com/video/BVxxx"

# 自动解析意图 → 选择 bilibili 扩展 → 执行下载
```

### 3. Agent API
```rust
// Rust: Agent 调用接口
pub trait DownloadAgent {
    fn discover_extensions(&self) -> Vec<ExtensionManifest>;
    fn match_intent(&self, query: &str) -> Option<ExtensionRef>;
    fn execute(&self, ext: &ExtensionRef, params: Value) -> Result<DownloadResult>;
}
```

### 4. 扩展 AI Manifest 示例
```yaml
name: bilibili
version: 1.0.0

# AI 可读描述
ai_manifest:
  description: "B站视频/直播下载，支持批量、弹幕、封面提取"
  
  # 意图关键词 - AI通过这些匹配用户需求
  intent_keywords:
    - "b站"
    - "bilibili"
    - "B站视频"
    - "B站直播"
    - "BV号"
    - "AV号"
    - "弹幕"
    - "UP主"
  
  # 用自然语言描述输入输出
  input_narrative: |
    输入：B站视频链接（BV号或完整URL）
    可选：画质选择、是否下载弹幕/封面、是否批量
    
  output_narrative: |
    输出：视频文件 + 可选弹幕(ass/xml)、封面(jpg)
  
  # AI 使用示例
  examples:
    - "下载 https://bilibili.com/video/BV1xx 的高清版本"
    - "批量下载 UP主 http://space.bilibili.com/xxx 的视频"
    - "下载这个视频的弹幕和封面"
    - "录制这个B站直播间"
  
  # 错误处理
  error_handling:
    "视频不存在": "检查BV号是否正确"
    "需要登录": "提示用户登录或使用cookie"
    "直播未开始": "提示预约录制"
```

## AI Agent 集成方式

### 方式一：CLI 调用
```python
# Python AI Agent
result = subprocess.run(
    ["open-fetch", "ai", user_query],
    capture_output=True, text=True
)
```

### 方式二：API 调用
```rust
// Rust Agent 集成
use open_fetch::core::Agent;

let agent = Agent::new();
let extensions = agent.discover();
let matched = agent.intent_match("下载这个b站视频", &extensions);
let result = agent.execute(&matched, params).await;
```

### 方式三：HTTP API
```bash
# 启动 HTTP 服务
open-fetch serve --port 8080

# REST API 调用
curl -X POST http://localhost:8080/api/download \
  -H "Content-Type: application/json" \
  -d '{"query": "下载这个B站视频 https://bilibili.com/video/BVxxx"}'
```

## 扩展 AI 能力分级

| 等级 | 能力 | 说明 |
|------|------|------|
| L1 | 自描述 | 有 AI Manifest，能被 AI 发现 |
| L2 | 语义匹配 | 支持意图关键词，自动选择扩展 |
| L3 | 上下文理解 | 理解对话上下文，支持代词指代 |
| L4 | 自主决策 | AI 自动决定最优下载策略 |
| L5 | 主动推荐 | 根据用户习惯主动推荐下载方案 |

## 未来 AI 能力扩展

### v0.5.0 - L2 语义匹配
- 意图关键词匹配
- 扩展自动选择

### v0.6.0 - L3 上下文理解
- 对话上下文缓存
- 代词指代解析

### v1.0.0 - L4 自主决策
- 画质/格式自动选择
- 多源并行下载
- 断点续传

### v2.0.0 - L5 主动推荐
- 学习用户习惯
- 智能去重
- 订阅自动下载

---

*AI 让下载更简单*
