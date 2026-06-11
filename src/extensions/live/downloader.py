#!/usr/bin/env python3
"""
直播下载器 - 多平台直播录制
"""

import sys
import json
import argparse
from pathlib import Path
import time

def ensure_yt_dlp():
    try:
        import yt_dlp
        return yt_dlp
    except ImportError:
        print("ERROR: 需要yt-dlp", file=sys.stderr)
        sys.exit(1)

def detect_platform(url):
    """检测直播平台"""
    platforms = {
        'bilibili.com/live': 'bilibili',
        'live.bilibili.com': 'bilibili',
        'douyin.com/live': 'douyin',
        'v.douyin.com': 'douyin',
        'live.douyin.com': 'douyin',
        'douyu.com': 'douyu',
        'huya.com': 'huya',
        'twitch.tv': 'twitch',
        'youtube.com/live': 'youtube',
        'chaturbate.com': 'chaturbate',
    }
    for key, platform in platforms.items():
        if key in url:
            return platform
    return 'unknown'

def record_live(url, output_dir="./downloads/live", quality="best"):
    """录制直播"""
    yt_dlp = ensure_yt_dlp()
    platform = detect_platform(url)
    
    import datetime
    timestamp = datetime.datetime.now().strftime("%Y%m%d_%H%M%S")
    
    ydl_opts = {
        'format': 'bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]/best',
        'outtmpl': f'{output_dir}/{platform}_%(title)s_{timestamp}.%(ext)s',
        'merge_output_format': 'mp4',
        'live_start': True,
        'wait_for_video': True,  # 等待开播
        'max_sleep_interval': 60,  # 最大等待间隔
        '几次': 5,  # 最大重试次数
    }
    
    # 平台特定Headers
    headers = {'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36'}
    if platform == 'bilibili':
        headers['Referer'] = 'https://live.bilibili.com/'
    elif platform == 'douyin':
        headers['Referer'] = 'https://live.douyin.com/'
    elif platform == 'twitch':
        headers['Client-ID'] = 'jzkbprff40iqj646a697cyrvl0zt2y6'
    
    ydl_opts['http_headers'] = headers
    
    print(f"正在录制 {platform} 直播: {url}")
    print("按 Ctrl+C 停止录制...")
    
    try:
        with yt_dlp.YoutubeDL(ydl_opts) as ydl:
            info = ydl.extract_info(url, download=True)
            return {
                "success": True,
                "platform": platform,
                "title": info.get('title'),
                "streamer": info.get('uploader'),
                "started_at": timestamp
            }
    except KeyboardInterrupt:
        return {"success": True, "message": "录制已手动停止"}
    except Exception as e:
        return {"success": False, "error": str(e)}

def check_live_status(url):
    """检查开播状态"""
    yt_dlp = ensure_yt_dlp()
    platform = detect_platform(url)
    
    ydl_opts = {
        'skip_download': True,
        'listsubtitles': False,
        'http_headers': {
            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36'
        }
    }
    
    try:
        with yt_dlp.YoutubeDL(ydl_opts) as ydl:
            info = ydl.extract_info(url, download=False)
            is_live = info.get('is_live', False)
            return {
                "success": True,
                "platform": platform,
                "is_live": is_live,
                "title": info.get('title'),
                "viewer_count": info.get('viewer_count', 0) if is_live else 0
            }
    except Exception as e:
        return {"success": False, "error": str(e)}

def main():
    parser = argparse.ArgumentParser(description='直播下载器')
    parser.add_argument('action', choices=['record', 'status'])
    parser.add_argument('--url', required=True, help='直播间URL')
    parser.add_argument('--output', default='./downloads/live', help='输出目录')
    parser.add_argument('--quality', default='best', help='画质')
    
    args = parser.parse_args()
    Path(args.output).mkdir(parents=True, exist_ok=True)
    
    if args.action == 'record':
        result = record_live(args.url, args.output, args.quality)
    elif args.action == 'status':
        result = check_live_status(args.url)
    
    print(json.dumps(result, ensure_ascii=False, indent=2))

if __name__ == '__main__':
    main()
