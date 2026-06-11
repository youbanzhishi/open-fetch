#!/usr/bin/env python3
"""
Bilibili下载器 - 支持视频/番剧/直播/漫画
使用yt-dlp作为核心引擎
"""

import sys
import json
import argparse
from pathlib import Path

def ensure_yt_dlp():
    """确保yt-dlp可用"""
    try:
        import yt_dlp
        return yt_dlp
    except ImportError:
        print("ERROR: 需要yt-dlp库，运行: pip install yt-dlp", file=sys.stderr)
        sys.exit(1)

def download_video(url, quality="最高画质", output_dir="./downloads", format="mp4"):
    """下载B站视频"""
    yt_dlp = ensure_yt_dlp()
    
    # 质量映射
    quality_map = {
        "1080P": "1920x1080",
        "720P": "1280x720", 
        "480P": "854x480",
        "最高画质": "best"
    }
    
    ydl_opts = {
        'format': f'bestvideo[height<={quality_map.get(quality, "best")}][ext={format}]+bestaudio[ext=m4a]/best[ext={format}]/best',
        'outtmpl': f'{output_dir}/%(title)s_[%(id)s].%(ext)s',
        'merge_output_format': format,
        'writeinfojson': True,
        'writethumbnail': True,
        'postprocessors': [{
            'key': 'EmbedThumbnail',
        }],
        # B站特定参数
        'extractor_args': {
            'bilibili': {
                'embed_watermark': False,  # 去除水印
                'dual_dash': True,
                'first_webpage': True
            }
        },
        'http_headers': {
            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36',
            'Referer': 'https://www.bilibili.com'
        }
    }
    
    try:
        with yt_dlp.YoutubeDL(ydl_opts) as ydl:
            info = ydl.extract_info(url, download=True)
            return {
                "success": True,
                "title": info.get('title', '未知'),
                "id": info.get('id', ''),
                "duration": info.get('duration', 0),
                "uploader": info.get('uploader', '未知'),
                "path": f"{output_dir}/{info.get('title', '')}_{info.get('id', '')}.{format}"
            }
    except Exception as e:
        return {"success": False, "error": str(e)}

def download_subtitles(url, languages=None, output_dir="./downloads"):
    """下载字幕"""
    yt_dlp = ensure_yt_dlp()
    
    if languages is None:
        languages = ["zh-CN", "zh-TW", "en"]
    
    ydl_opts = {
        'writesubtitles': True,
        'writeautomaticsub': True,
        'subtitleslangs': languages,
        'outtmpl': f'{output_dir}/%(title)s_[%(id)s]_%(lang)s.%(ext)s',
        'skip_download': True  # 只下载字幕
    }
    
    try:
        with yt_dlp.YoutubeDL(ydl_opts) as ydl:
            info = ydl.extract_info(url, download=False)
            return {
                "success": True,
                "subtitles": info.get('subtitles', {}),
                "automatic_captions": info.get('automatic_captions', {})
            }
    except Exception as e:
        return {"success": False, "error": str(e)}

def batch_download_uid(uid, limit=10, output_dir="./downloads"):
    """批量下载UP主视频"""
    yt_dlp = ensure_yt_dlp()
    
    # 构建用户空间URL
    user_url = f"https://space.bilibili.com/{uid}/video"
    
    ydl_opts = {
        'outtmpl': f'{output_dir}/%(uploader)s/%(title)s_[%(id)s].%(ext)s',
        'playlistend': limit,
        'merge_output_format': 'mp4',
        'extractor_args': {
            'bilibili': {'embed_watermark': False}
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
                            "id": entry.get('id'),
                            "url": entry.get('webpage_url')
                        })
            return {"success": True, "count": len(videos), "videos": videos}
    except Exception as e:
        return {"success": False, "error": str(e)}

def download_live(room_id, output_dir="./downloads"):
    """下载直播间"""
    yt_dlp = ensure_yt_dlp()
    
    room_url = f"https://live.bilibili.com/{room_id}"
    
    ydl_opts = {
        'outtmpl': f'{output_dir}/live_{room_id}_%(timestamp)s.%(ext)s',
        'live_start': True,
        'extractor_args': {
            'bilibili': {'live': True}
        }
    }
    
    try:
        with yt_dlp.YoutubeDL(ydl_opts) as ydl:
            info = ydl.extract_info(room_url, download=True)
            return {
                "success": True,
                "room_id": room_id,
                "title": info.get('title', ''),
                "streamer": info.get('uploader', '')
            }
    except Exception as e:
        return {"success": False, "error": str(e)}

def main():
    parser = argparse.ArgumentParser(description='Bilibili下载器')
    parser.add_argument('action', choices=['video', 'subtitle', 'batch', 'live'], 
                        help='操作类型')
    parser.add_argument('--url', help='视频URL')
    parser.add_argument('--uid', help='用户UID (batch模式)')
    parser.add_argument('--room', help='直播间ID (live模式)')
    parser.add_argument('--quality', default='最高画质', help='画质')
    parser.add_argument('--format', default='mp4', help='格式')
    parser.add_argument('--limit', type=int, default=10, help='批量数量')
    parser.add_argument('--output', default='./downloads', help='输出目录')
    parser.add_argument('--languages', nargs='+', default=['zh-CN', 'en'], help='字幕语言')
    
    args = parser.parse_args()
    
    # 确保输出目录存在
    Path(args.output).mkdir(parents=True, exist_ok=True)
    
    if args.action == 'video':
        if not args.url:
            print("ERROR: video模式需要--url参数")
            sys.exit(1)
        result = download_video(args.url, args.quality, args.output, args.format)
    elif args.action == 'subtitle':
        if not args.url:
            print("ERROR: subtitle模式需要--url参数")
            sys.exit(1)
        result = download_subtitles(args.url, args.languages, args.output)
    elif args.action == 'batch':
        if not args.uid:
            print("ERROR: batch模式需要--uid参数")
            sys.exit(1)
        result = batch_download_uid(args.uid, args.limit, args.output)
    elif args.action == 'live':
        if not args.room:
            print("ERROR: live模式需要--room参数")
            sys.exit(1)
        result = download_live(args.room, args.output)
    
    print(json.dumps(result, ensure_ascii=False, indent=2))

if __name__ == '__main__':
    main()
