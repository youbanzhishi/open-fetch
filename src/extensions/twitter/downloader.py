#!/usr/bin/env python3
"""Twitter/X下载器"""
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

def download_twitter(url, quality="best", output_dir="./downloads"):
    """下载Twitter/X媒体"""
    yt_dlp_lib = ensure_yt_dlp()
    
    ydl_opts = {
        'format': 'bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]/best',
        'outtmpl': f'{output_dir}/%(uploader)s_%(id)s.%(ext)s',
        'merge_output_format': 'mp4',
        'http_headers': {
            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36',
        }
    }
    
    try:
        with yt_dlp_lib.YoutubeDL(ydl_opts) as ydl:
            info = ydl.extract_info(url, download=True)
            return {
                "success": True,
                "title": info.get('title'),
                "id": info.get('id'),
                "uploader": info.get('uploader'),
                "like_count": info.get('like_count', 0),
                "retweet_count": info.get('repost_count', 0)
            }
    except Exception as e:
        return {"success": False, "error": str(e)}

def main():
    parser = argparse.ArgumentParser(description='Twitter/X下载器')
    parser.add_argument('--url', required=True)
    parser.add_argument('--quality', default='best')
    parser.add_argument('--output', default='./downloads')
    
    args = parser.parse_args()
    Path(args.output).mkdir(parents=True, exist_ok=True)
    
    result = download_twitter(args.url, args.quality, args.output)
    print(json.dumps(result, ensure_ascii=False, indent=2))

if __name__ == '__main__':
    main()
