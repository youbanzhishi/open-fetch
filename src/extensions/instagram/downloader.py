#!/usr/bin/env python3
"""Instagram下载器"""
import sys
import json
import argparse
from pathlib import Path

def ensure_yt_dlp():
    try:
        import yt_dlp
        return yt_dlp
    except ImportError:
        print("ERROR: 需要yt-dlp", file=sys.stderr)
        sys.exit(1)

def download_instagram(url, download_stories=False, output_dir="./downloads"):
    """下载Instagram内容"""
    yt_dlp_lib = ensure_yt_dlp()
    
    ydl_opts = {
        'outtmpl': f'{output_dir}/%(username)s_%(id)s.%(ext)s',
        'http_headers': {
            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36',
            'Referer': 'https://www.instagram.com/'
        }
    }
    
    try:
        with yt_dlp_lib.YoutubeDL(ydl_opts) as ydl:
            info = ydl.extract_info(url, download=True)
            return {
                "success": True,
                "username": info.get('username'),
                "title": info.get('title'),
                "id": info.get('id'),
                "like_count": info.get('like_count', 0)
            }
    except Exception as e:
        return {"success": False, "error": str(e)}

def main():
    parser = argparse.ArgumentParser(description='Instagram下载器')
    parser.add_argument('--url', required=True)
    parser.add_argument('--output', default='./downloads')
    parser.add_argument('--stories', action='store_true')
    
    args = parser.parse_args()
    Path(args.output).mkdir(parents=True, exist_ok=True)
    
    result = download_instagram(args.url, args.stories, args.output)
    print(json.dumps(result, ensure_ascii=False, indent=2))

if __name__ == '__main__':
    main()
