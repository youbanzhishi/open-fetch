#!/usr/bin/env python3
"""
抖音/TikTok下载器 - 无水印下载
支持抖音/TikTok视频、作者主页批量、评论采集
"""

import sys
import json
import re
import argparse
from pathlib import Path

def ensure_yt_dlp():
    try:
        import yt_dlp
        return yt_dlp
    except ImportError:
        print("ERROR: 需要yt-dlp库，运行: pip install yt-dlp", file=sys.stderr)
        sys.exit(1)

def get_platform(url):
    """识别平台"""
    if 'tiktok.com' in url:
        return 'tiktok'
    elif 'douyin.com' in url or 'iesdouyin.com' in url:
        return 'douyin'
    elif 'v.douyin.com' in url:
        return 'douyin_share'
    else:
        return 'unknown'

def download_video(url, no_watermark=True, output_dir="./downloads"):
    """下载单个视频"""
    yt_dlp = ensure_yt_dlp()
    platform = get_platform(url)
    
    # 针对无水印的format选择
    if no_watermark:
        # 优先选择不带水印的格式
        format_spec = 'bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]/best'
    else:
        format_spec = 'best'
    
    ydl_opts = {
        'format': format_spec,
        'outtmpl': f'{output_dir}/%(uploader)s_%(title)s_[%(id)s].%(ext)s',
        'merge_output_format': 'mp4',
        'writeinfojson': True,
        'http_headers': {
            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36',
        }
    }
    
    # 平台特定参数
    if platform == 'douyin' or platform == 'douyin_share':
        ydl_opts['extractor_args'] = {
            'tiktok': {
                'watermark': False,
                'forbidden_ext': ['public_intl']  # 国际版不要
            }
        }
        ydl_opts['http_headers']['Referer'] = 'https://www.douyin.com/'
    elif platform == 'tiktok':
        ydl_opts['extractor_args'] = {
            'tiktok': {'watermark': False}
        }
        ydl_opts['http_headers']['Referer'] = 'https://www.tiktok.com/'
    
    try:
        with yt_dlp.YoutubeDL(ydl_opts) as ydl:
            info = ydl.extract_info(url, download=True)
            return {
                "success": True,
                "platform": platform,
                "title": info.get('title', '未知'),
                "id": info.get('id', ''),
                "uploader": info.get('uploader', '未知'),
                "like_count": info.get('like_count', 0),
                "play_count": info.get('view_count', 0),
                "duration": info.get('duration', 0)
            }
    except Exception as e:
        return {"success": False, "error": str(e)}

def download_author(sec_uid, limit=20, output_dir="./downloads"):
    """下载作者全部视频"""
    yt_dlp = ensure_yt_dlp()
    
    # 构建作者主页URL
    if 'douyin.com' in sec_uid or sec_uid.startswith('MS4'):
        author_url = f"https://www.douyin.com/user/{sec_uid}"
    else:
        author_url = f"https://www.douyin.com/user/{sec_uid}"
    
    ydl_opts = {
        'outtmpl': f'{output_dir}/%(uploader)s/%(title)s_[%(id)s].%(ext)s',
        'playlistend': limit,
        'merge_output_format': 'mp4',
        'extractor_args': {
            'tiktok': {'watermark': False}
        }
    }
    
    try:
        with yt_dlp.YoutubeDL(ydl_opts) as ydl:
            info = ydl.extract_info(author_url, download=True)
            videos = []
            if 'entries' in info:
                for entry in info['entries']:
                    if entry:
                        videos.append({
                            "title": entry.get('title'),
                            "id": entry.get('id'),
                            "url": entry.get('webpage_url'),
                            "like_count": entry.get('like_count', 0)
                        })
            return {
                "success": True,
                "author": info.get('uploader', '未知'),
                "count": len(videos),
                "videos": videos
            }
    except Exception as e:
        return {"success": False, "error": str(e)}

def fetch_comments(url, limit=100):
    """采集评论"""
    yt_dlp = ensure_yt_dlp()
    
    ydl_opts = {
        'skip_download': True,
        'getcomments': True,
        'extractor_args': {
            'tiktok': {'forbidden_ext': ['public_intl']}
        }
    }
    
    try:
        with yt_dlp.YoutubeDL(ydl_opts) as ydl:
            info = ydl.extract_info(url, download=False)
            comments = []
            if 'comments' in info:
                for c in info['comments'][:limit]:
                    comments.append({
                        "text": c.get('text', ''),
                        "author": c.get('author', ''),
                        "like_count": c.get('like_count', 0),
                        "timestamp": c.get('timestamp', 0)
                    })
            return {
                "success": True,
                "comment_count": len(comments),
                "comments": comments
            }
    except Exception as e:
        return {"success": False, "error": str(e)}

def main():
    parser = argparse.ArgumentParser(description='抖音/TikTok下载器')
    parser.add_argument('action', choices=['video', 'author', 'comments'],
                        help='操作类型')
    parser.add_argument('--url', help='视频URL')
    parser.add_argument('--uid', dest='sec_uid', help='作者SEC_UID (author模式)')
    parser.add_argument('--limit', type=int, default=20, help='下载/评论数量')
    parser.add_argument('--output', default='./downloads', help='输出目录')
    parser.add_argument('--no-watermark', action='store_true', default=True,
                        help='去除水印')
    
    args = parser.parse_args()
    Path(args.output).mkdir(parents=True, exist_ok=True)
    
    if args.action == 'video':
        if not args.url:
            print("ERROR: video模式需要--url参数")
            sys.exit(1)
        result = download_video(args.url, args.no_watermark, args.output)
    elif args.action == 'author':
        if not args.sec_uid:
            print("ERROR: author模式需要--uid参数")
            sys.exit(1)
        result = download_author(args.sec_uid, args.limit, args.output)
    elif args.action == 'comments':
        if not args.url:
            print("ERROR: comments模式需要--url参数")
            sys.exit(1)
        result = fetch_comments(args.url, args.limit)
    
    print(json.dumps(result, ensure_ascii=False, indent=2))

if __name__ == '__main__':
    main()
