#!/bin/bash
#
# OpenFetch 一键安装脚本
#

set -e

echo "🚀 OpenFetch 安装脚本"
echo "======================"

# 检测操作系统
OS="$(uname -s)"
case "$OS" in
    Linux*)     DISTRO=linux;;
    Darwin*)    DISTRO=macos;;
    CYGWIN*)    DISTRO=windows;;
    MINGW*)     DISTRO=windows;;
    *)          DISTRO=unknown;;
esac

echo "检测到系统: $DISTRO"

# 安装Python依赖
echo ""
echo "📦 安装Python依赖..."
if command -v pip3 &> /dev/null; then
    pip3 install yt-dlp requests aiohttp --upgrade
elif command -v pip &> /dev/null; then
    pip install yt-dlp requests aiohttp --upgrade
else
    echo "❌ 未找到pip，请先安装Python"
    exit 1
fi

# 安装FFmpeg
echo ""
echo "📦 安装FFmpeg..."
if command -v ffmpeg &> /dev/null; then
    echo "✓ FFmpeg已安装: $(ffmpeg -version | head -1)"
else
    case "$DISTRO" in
        linux)
            if command -v apt-get &> /dev/null; then
                sudo apt-get update && sudo apt-get install -y ffmpeg
            elif command -v yum &> /dev/null; then
                sudo yum install -y ffmpeg
            elif command -v dnf &> /dev/null; then
                sudo dnf install -y ffmpeg
            fi
            ;;
        macos)
            if command -v brew &> /dev/null; then
                brew install ffmpeg
            else
                echo "❌ 请安装Homebrew: https://brew.sh"
            fi
            ;;
        windows)
            echo "⚠️ Windows用户请手动下载: https://ffmpeg.org/download.html"
            ;;
    esac
fi

# 创建下载目录
echo ""
echo "📁 创建下载目录..."
DOWNLOAD_DIR="${HOME}/OpenFetchDownloads"
mkdir -p "$DOWNLOAD_DIR"
echo "✓ 下载目录: $DOWNLOAD_DIR"

# 添加PATH（可选）
echo ""
echo "💡 提示：可执行以下命令添加到PATH:"
echo "   echo 'export PATH=\"\$PATH:${HOME}/open-fetch\"' >> ~/.bashrc"
echo "   source ~/.bashrc"

echo ""
echo "======================"
echo "✅ 安装完成！"
echo ""
echo "使用方式:"
echo "  1. CLI:   ./open-fetch bilibili <URL>"
echo "  2. 服务:  ./open-fetch server --port 8080"
echo "  3. 批量:  python3 scripts/batch-download.py -u <URL1> <URL2>"
echo ""
echo "浏览器插件:"
echo "  Chrome:  加载 browser-ext/chrome/"
echo "  Firefox: 加载 browser-ext/firefox/"
