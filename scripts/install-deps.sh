#!/bin/bash
# OpenFetch 依赖安装脚本

set -e

echo "📦 安装 OpenFetch 依赖..."

# 检查 Python
if ! command -v python3 &> /dev/null; then
    echo "❌ Python3 未安装"
    exit 1
fi

echo "✅ Python3 已安装: $(python3 --version)"

# 安装 yt-dlp
echo "📥 安装 yt-dlp..."
pip3 install yt-dlp --quiet

# 检查 requests
pip3 show requests &> /dev/null || pip3 install requests --quiet

# 检查 aiohttp (异步下载)
pip3 show aiohttp &> /dev/null || pip3 install aiohttp --quiet

echo ""
echo "✅ 依赖安装完成！"
echo ""
echo "已安装的包:"
pip3 list | grep -E "yt-dlp|requests|aiohttp"

echo ""
echo "使用示例:"
echo "  open-fetch download https://www.youtube.com/watch?v=xxx"
echo "  open-fetch ai '下载这个B站视频 https://bilibili.com/video/BVxxx'"
