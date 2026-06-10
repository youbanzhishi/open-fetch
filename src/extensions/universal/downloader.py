#!/usr/bin/env python3
"""
OpenFetch - 通用下载器扩展
基于 yt-dlp，支持多平台下载

入口函数：resolve(params: dict) -> dict
"""

import json
import os
import sys
import tempfile
from typing import Dict, Any, Optional, List

# 尝试导入 yt-dlp
try:
    import yt_dlp
except ImportError:
    print("需要安装 yt-dlp: pip install yt-dlp")
    sys.exit(1)


class UniversalDownloader:
    """通用下载器 - 基于yt-dlp"""
    
    def __init__(self):
        self.supported_platforms = [
            'youtube.com', 'youtu.be',
            'bilibili.com', 'b23.tv',
            'douyin.com', 'v.douyin.com',
            'weibo.com',
            'twitter.com', 'x.com',
            'instagram.com',
            'reddit.com',
            'tiktok.com',
        ]
    
    def resolve(self, params: Dict[str, Any]) -> Dict[str, Any]:
        """
        解析URL并获取下载信息
        
        Args:
            params: 包含 url, quality, output 等参数的字典
            
        Returns:
            包含 title, files, metadata 等的字典
        """
        url = params.get("url", "")
        
        # 如果是B站视频，走B站专用逻辑
        if 'bilibili.com' in url:
            return self.resolve_bilibili(url, params)
        
        # 通用yt-dlp解析
        return self.resolve_generic(url, params)
    
    def resolve_bilibili(self, url: str, params: Dict[str, Any]) -> Dict[str, Any]:
        """解析B站视频"""
        ydl_opts = {
            'quiet': True,
            'no_warnings': True,
            'extract_flat': False,
        }
        
        with yt_dlp.YoutubeDL(ydl_opts) as ydl:
            info = ydl.extract_info(url, download=False)
            
            result = {
                "title": info.get('title', 'unknown'),
                "platform": "bilibili",
                "duration": info.get('duration'),
                "thumbnail": info.get('thumbnail'),
                "uploader": info.get('uploader'),
                "view_count": info.get('view_count'),
                "like_count": info.get('like_count'),
                "files": [],
                "metadata": info
            }
            
            # 提取下载链接
            quality = params.get("quality", "best")
            
            # 处理视频格式
            if 'formats' in info:
                for fmt in info['formats']:
                    if fmt.get('ext') == 'mp4' or fmt.get('vcodec') != 'none':
                        tbr = fmt.get('tbr', 0)
                        height = fmt.get('height')
                        
                        if quality == "best" or quality == "1080p":
                            if height and height <= 1080:
                                result["files"].append({
                                    "url": fmt.get('url', ''),
                                    "format_id": fmt.get('format_id', ''),
                                    "quality": f"{height}p" if height else f"{int(tbr)}k",
                                    "ext": fmt.get('ext', 'mp4'),
                                    "filesize": fmt.get('filesize'),
                                })
            
            return result
    
    def resolve_generic(self, url: str, params: Dict[str, Any]) -> Dict[str, Any]:
        """通用解析"""
        ydl_opts = {
            'quiet': True,
            'no_warnings': True,
            'extract_flat': False,
        }
        
        with yt_dlp.YoutubeDL(ydl_opts) as ydl:
            info = ydl.extract_info(url, download=False)
            
            return {
                "title": info.get('title', 'unknown'),
                "platform": info.get('extractor', 'unknown'),
                "duration": info.get('duration'),
                "thumbnail": info.get('thumbnail'),
                "uploader": info.get('uploader'),
                "view_count": info.get('view_count'),
                "files": [{
                    "url": fmt.get('url', ''),
                    "quality": f"{fmt.get('height', 0)}p" if fmt.get('height') else fmt.get('format_id', ''),
                    "ext": fmt.get('ext', 'mp4'),
                } for fmt in info.get('formats', []) if fmt.get('url')],
                "metadata": info
            }
    
    def download(self, params: Dict[str, Any]) -> Dict[str, Any]:
        """
        下载视频/音频
        
        Args:
            params: 包含 url, quality, format, output_dir 等参数
            
        Returns:
            包含 path, size 等的字典
        """
        url = params.get("url", "")
        output_dir = params.get("output_dir", tempfile.gettempdir())
        quality = params.get("quality", "best")
        format_spec = params.get("format", "bestvideo+bestaudio/best")
        
        # 创建输出文件名模板
        output_template = os.path.join(output_dir, '%(title)s-%(id)s.%(ext)s')
        
        ydl_opts = {
            'format': format_spec,
            'outtmpl': output_template,
            'quiet': False,
            'no_warnings': True,
            'progress_hooks': [self._progress_hook],
        }
        
        result = {"success": False, "path": None, "error": None}
        
        def progress_hook(d):
            if d['status'] == 'finished':
                result['path'] = d['filename']
            elif d['status'] == 'error':
                result['error'] = d.get('error', 'Unknown error')
        
        ydl_opts['progress_hooks'] = [progress_hook]
        
        try:
            with yt_dlp.YoutubeDL(ydl_opts) as ydl:
                ydl.download([url])
            
            if result['path'] and os.path.exists(result['path']):
                result['success'] = True
                result['size'] = os.path.getsize(result['path'])
            
        except Exception as e:
            result['error'] = str(e)
        
        return result
    
    def _progress_hook(self, d):
        """进度回调"""
        if d['status'] == 'progress':
            print(f"进度: {d.get('_percent_str', 'N/A')}")
        elif d['status'] == 'finished':
            print(f"完成: {d['filename']}")


def resolve(params: Dict[str, Any]) -> Dict[str, Any]:
    """
    OpenFetch 扩展入口函数
    """
    action = params.get("action", "resolve")
    
    downloader = UniversalDownloader()
    
    if action == "download":
        return downloader.download(params)
    else:
        return downloader.resolve(params)


if __name__ == "__main__":
    # 测试
    test_params = {
        "url": "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
        "action": "resolve"
    }
    
    try:
        result = resolve(test_params)
        print(json.dumps(result, ensure_ascii=False, indent=2))
    except Exception as e:
        print(f"错误: {e}", file=sys.stderr)
        sys.exit(1)
