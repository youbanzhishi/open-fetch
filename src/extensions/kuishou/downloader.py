#!/usr/bin/env python3
"""快手下载器"""
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

def download_kuishou(url, output_dir="./downloads"):
    """下载快手内容"""
    yt_dlp_lib = ensure_yt_dlp()
    
    ydl_opts = {
        'outtmpl': f'{output_dir}/%(uploader)s_%(id)s.%(ext)s',
        'http_headers': {
            'User-Agent': 'Mozilla/5.0',
            'Referer': 'https://www.kuaishou.com/'
        }
    }
    
    try:
        with yt_dlp_lib.YoutubeDL(ydl_opts) as ydl:
            info = ydl.extract_info(url, download=True)
            return {
                "success": True,
                "uploader": info.get('uploader'),
                "title": info.get('title'),
                "id": info.get('id')
            }
    except Exception as e:
        return {"success": False, "error": str(e)}

def main():
    parser = argparse.ArgumentParser(description='快手下载器')
    parser.add_argument('--url', required=True)
    parser.add_argument('--output', default='./downloads')
    
    args = parser.parse_args()
    Path(args.output).mkdir(parents=True, exist_ok=True)
    
    result = download_kuishou(args.url, args.output)
    print(json.dumps(result, ensure_ascii=False, indent=2))

if __name__ == '__main__':
    main()
