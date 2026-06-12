#!/usr/bin/env python3
"""小红书下载器"""
import sys
import json
import argparse
import re
from pathlib import Path

def ensure_yt_dlp():
    try:
        import yt_dlp
        return yt_dlp
    except ImportError:
        print("ERROR: 需要yt-dlp", file=sys.stderr)
        sys.exit(1)

def download_xhs(url, download_images=True, download_video=True, output_dir="./downloads"):
    """下载小红书笔记"""
    yt_dlp_lib = ensure_yt_dlp()
    
    ydl_opts = {
        'outtmpl': f'{output_dir}/%(uploader)s_%(title)s_[%(id)s].%(ext)s',
        'http_headers': {
            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36',
            'Referer': 'https://www.xiaohongshu.com/'
        }
    }
    
    if download_video:
        ydl_opts['format'] = 'best[ext=mp4]/best'
    
    try:
        with yt_dlp_lib.YoutubeDL(ydl_opts) as ydl:
            info = ydl.extract_info(url, download=True)
            return {
                "success": True,
                "title": info.get('title'),
                "id": info.get('id'),
                "uploader": info.get('uploader'),
                "like_count": info.get('like_count', 0),
                "type": info.get('extractor', 'xiaohongshu')
            }
    except Exception as e:
        return {"success": False, "error": str(e)}

def main():
    parser = argparse.ArgumentParser(description='小红书下载器')
    parser.add_argument('--url', required=True)
    parser.add_argument('--output', default='./downloads')
    parser.add_argument('--no-images', action='store_true')
    parser.add_argument('--no-video', action='store_true')
    
    args = parser.parse_args()
    Path(args.output).mkdir(parents=True, exist_ok=True)
    
    result = download_xhs(args.url, not args.no_images, not args.no_video, args.output)
    print(json.dumps(result, ensure_ascii=False, indent=2))

if __name__ == '__main__':
    main()
