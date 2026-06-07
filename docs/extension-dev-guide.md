# OpenFetch 扩展开发指南

> Extension First - 所有功能都是扩展，核心永远不改

## 扩展协议

每个扩展是一个目录，包含：

```
extension-name/
├── __init__.py          # 可选，模块初始化
├── extension.yaml       # 扩展描述（必须）
├── resolver.py          # 链接解析（必须）
├── downloader.py        # 下载逻辑（可选）
└── requirements.txt      # Python依赖（可选）
```

## extension.yaml 协议

```yaml
# ========== 必需字段 ==========
name: extension-name        # 扩展唯一标识（小写+短横线）
version: 1.0.0              # 语义版本
description: "扩展描述"    # 供AI理解的功能描述

# ========== 平台匹配 ==========
platform:
  - domain: "example.com"   # 支持的域名
  - pattern: "/video/.*"    # URL路径正则（可选）

# ========== 输入输出 ==========
input_schema:
  type: object
  properties:
    url:
      type: string
      required: true
      description: "资源链接"
    quality:
      type: string
      enum: ["low", "medium", "high"]
      default: "high"
  required: ["url"]

output_schema:
  type: object
  properties:
    title:
      type: string
    files:
      type: array
      items:
        type: object
        properties:
          url: { type: string }
          filename: { type: string }
          size: { type: integer }
          format: { type: string }

# ========== 能力声明 ==========
capabilities:
  - download          # 支持下载
  - batch_download     # 支持批量
  - live_record       # 支持直播录制
  - playlist          # 支持播放列表
  - metadata          # 支持获取元信息
  - compress          # 支持压缩

# ========== AI 能力声明 ==========
ai_capable: true
ai_description: "用自然语言描述扩展能力，供AI Agent理解和使用"
ai_keywords: ["下载", "视频", "图片", "bilibili", "b站"]
ai_examples:
  - "帮我下载这个B站视频"
  - "批量下载这个用户的全部视频"
  - "下载高清版本"
ai_intent_patterns:
  - "下载*视频"
  - "获取*图片"
  - "*素材"

# ========== 扩展发现协议 ==========
discovery:
  auto_register: true      # 自动注册到扩展市场
  ai_indexable: true      # 可被AI索引
  ranking_score: 100       # AI推荐权重

```

## resolver.py 协议

```python
"""
OpenFetch 扩展解析器
入参：URL 或参数对象
出参：元信息 + 下载链接列表
"""

def resolve(params: dict) -> dict:
    """
    解析资源URL，返回元信息和下载链接
    
    Args:
        params: 包含 url, quality 等参数的字典
        
    Returns:
        {
            "title": "资源标题",
            "files": [{
                "url": "https://cdn.example.com/video.mp4",
                "filename": "video_1080p.mp4",
                "size": 1024000,
                "format": "mp4"
            }],
            "metadata": {
                "duration": 300,
                "width": 1920,
                "height": 1080
            }
        }
    """
    url = params.get("url")
    quality = params.get("quality", "high")
    
    # 1. 获取页面信息
    html = fetch_page(url)
    
    # 2. 解析真实URL
    real_url = parse_real_url(html, quality)
    
    # 3. 提取元信息
    title = extract_title(html)
    
    return {
        "title": title,
        "files": [{
            "url": real_url,
            "filename": f"{title}.mp4",
            "size": get_file_size(real_url),
            "format": "mp4"
        }]
    }


def fetch_page(url: str) -> str:
    """获取页面内容"""
    import requests
    headers = {
        "User-Agent": "Mozilla/5.0 (compatible; OpenFetch/1.0)"
    }
    return requests.get(url, headers=headers, timeout=30).text


def parse_real_url(html: str, quality: str) -> str:
    """从页面解析真实下载链接"""
    raise NotImplementedError()


def extract_title(html: str) -> str:
    """提取资源标题"""
    import re
    match = re.search(r'<title>(.*?)</title>', html)
    return match.group(1) if match else "unknown"
```

## 内置扩展 vs 第三方扩展

| 类型 | 位置 | 安装方式 | 更新方式 |
|------|------|----------|----------|
| 内置 | `src/extensions/` | 随主程序 | 随主程序 |
| 第三方 | `~/.open-fetch/extensions/` | `open-fetch ext install` | `open-fetch ext update` |

## 扩展市场

扩展发布到 GitHub Release 或指定市场：

```bash
# 搜索扩展
open-fetch ext search "b站"

# 安装扩展
open-fetch ext install bilibili

# 发布扩展
open-fetch ext publish --path ./my-extension
```

## 测试

```bash
# 本地测试扩展
open-fetch ext test --path ./my-extension --url "https://example.com/video"

# 运行测试套件
pytest tests/
```

## 发布流程

1. 创建 `extension.yaml` 和实现代码
2. 测试通过后提交 PR
3. 审核通过后进入扩展市场
4. 用户通过 `open-fetch ext install` 安装

---

*有问题？提交 Issue 或 PR*
