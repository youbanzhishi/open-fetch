#!/usr/bin/env python3
"""
微博下载器
"""

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

def download_video(url, output_dir="./downloads"):
    """下载视频"""
    yt_dlp = ensure_yt_dlp()
    
    ydl_opts = {
        'format': 'bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]/best',
        'outtmpl': f'{output_dir}/%(uploader)s_%(title)s_[%(id)s].%(ext)s',
        'merge_output_format': 'mp4',
        'http_headers': {
            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36',
            'Referer': 'https://weibo.com/'
        }
    }
    
    try:
        with yt_dlp.YoutubeDL(ydl_opts) as ydl:
            info = ydl.extract_info(url, download=True)
            return {
                "success": True,
                "title": info.get('title'),
                "id": info.get('id'),
                "uploader": info.get('uploader'),
                "timestamp": info.get('timestamp')
            }
    except Exception as e:
        return {"success": False, "error": str(e)}

def download_user(uid, limit=20, output_dir="./downloads"):
    """下载用户视频"""
    yt_dlp = ensure_yt_dlp()
    
    user_url = f"https://weibo.com/u/{uid}"
    
    ydl_opts = {
        'outtmpl': f'{output_dir}/%(uploader)s/%(title)s_[%(id)s].%(ext)s',
        'playlistend': limit,
        'http_headers': {
            'Referer': 'https://weibo.com/'
        }
    }
    
    try:
        with yt_dlp.YoutubeDL(ydl_opts) as ydl:
            info = ydl.extract_info(user_url, download=True)
            videos = []
            if 'entries' in info:
                for entry in info['entries']:
                    if entry:
                        videos.append({
                            "title": entry.get('title'),
                            "id": entry.get('id')
                        })
            return {"success": True, "count": len(videos), "videos": videos}
    except Exception as e:
        return {"success": False, "error": str(e)}

def main():
    parser = argparse.ArgumentParser(description='微博下载器')
    parser.add_argument('action', choices=['video', 'user'])
    parser.add_argument('--url', help='视频URL')
    parser.add_argument('--uid', help='用户UID')
    parser.add_argument('--limit', type=int, default=20)
    parser.add_argument('--output', default='./downloads')
    
    args = parser.parse_args()
    Path(args.output).mkdir(parents=True, exist_ok=True)
    
    if args.action == 'video':
        result = download_video(args.url, args.output)
    elif args.action == 'user':
        result = download_user(args.uid, args.limit, args.output)
    
    print(json.dumps(result, ensure_ascii=False, indent=2))

if __name__ == '__main__':
    main()
