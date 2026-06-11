#!/usr/bin/env python3
"""
YouTube下载器 - 支持4K/8K/直播/音乐
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

def download_video(url, quality="best", format="mp4", audio_only=False, 
                   output_dir="./downloads"):
    """下载视频"""
    yt_dlp = ensure_yt_dlp()
    
    if audio_only:
        # 仅音频模式
        ydl_opts = {
            'format': 'bestaudio/best',
            'outtmpl': f'{output_dir}/%(title)s_[%(id)s].%(ext)s',
            'postprocessors': [{
                'key': 'FFmpegExtractAudio',
                'preferredcodec': format if format != 'best' else 'mp3',
                'preferredquality': '192',
            }],
        }
    else:
        # 视频模式 - 质量映射
        quality_map = {
            "4320p": "4320",
            "2160p": "2160", 
            "1440p": "1440",
            "1080p": "1080",
            "720p": "720",
            "480p": "480",
            "best": "best"
        }
        height = quality_map.get(quality, "best")
        
        if height == "best":
            format_spec = f'bestvideo[ext={format}]+bestaudio[ext=m4a]/best[ext={format}]/best'
        else:
            format_spec = f'bestvideo[height<={height}][ext={format}]+bestaudio[ext=m4a]/best[height<={height}][ext={format}]/best'
        
        ydl_opts = {
            'format': format_spec,
            'outtmpl': f'{output_dir}/%(title)s_[%(id)s].%(ext)s',
            'merge_output_format': format,
        }
    
    # YouTube特定
    ydl_opts.update({
        'writesubtitles': True,
        'writeautomaticsub': True,
        'subtitleslangs': ['zh-Hans', 'zh-Hant', 'en', 'ja', 'ko'],
        'writeinfojson': True,
        'writethumbnail': True,
    })
    
    try:
        with yt_dlp.YoutubeDL(ydl_opts) as ydl:
            info = ydl.extract_info(url, download=True)
            return {
                "success": True,
                "title": info.get('title'),
                "id": info.get('id'),
                "duration": info.get('duration'),
                "view_count": info.get('view_count'),
                "like_count": info.get('like_count'),
                "uploader": info.get('uploader'),
                "resolution": info.get('resolution'),
                "filesize": info.get('filesize') or info.get('original_url')
            }
    except Exception as e:
        return {"success": False, "error": str(e)}

def download_playlist(url, start=1, end=None, output_dir="./downloads"):
    """下载播放列表"""
    yt_dlp = ensure_yt_dlp()
    
    ydl_opts = {
        'format': 'bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]/best',
        'outtmpl': f'{output_dir}/%(playlist_title)s/%(title)s_[%(id)s].%(ext)s',
        'playliststart': start,
        'playlistend': end,
        'merge_output_format': 'mp4',
    }
    
    try:
        with yt_dlp.YoutubeDL(ydl_opts) as ydl:
            info = ydl.extract_info(url, download=True)
            videos = []
            if 'entries' in info:
                for entry in info['entries']:
                    if entry:
                        videos.append({
                            "title": entry.get('title'),
                            "id": entry.get('id'),
                            "duration": entry.get('duration')
                        })
            return {
                "success": True,
                "playlist": info.get('title'),
                "count": len(videos),
                "videos": videos
            }
    except Exception as e:
        return {"success": False, "error": str(e)}

def download_music(url, format="mp3", output_dir="./downloads"):
    """下载音乐"""
    return download_video(url, audio_only=True, format=format, output_dir=output_dir)

def main():
    parser = argparse.ArgumentParser(description='YouTube下载器')
    parser.add_argument('action', choices=['video', 'playlist', 'music'],
                        help='操作类型')
    parser.add_argument('--url', help='视频/播放列表URL')
    parser.add_argument('--quality', default='best', help='画质')
    parser.add_argument('--format', default='mp4', help='格式')
    parser.add_argument('--start', type=int, default=1, help='起始序号(playlist)')
    parser.add_argument('--end', type=int, help='结束序号(playlist)')
    parser.add_argument('--output', default='./downloads', help='输出目录')
    
    args = parser.parse_args()
    Path(args.output).mkdir(parents=True, exist_ok=True)
    
    if args.action == 'video':
        result = download_video(args.url, args.quality, args.format, False, args.output)
    elif args.action == 'playlist':
        result = download_playlist(args.url, args.start, args.end, args.output)
    elif args.action == 'music':
        result = download_music(args.url, args.format, args.output)
    
    print(json.dumps(result, ensure_ascii=False, indent=2))

if __name__ == '__main__':
    main()
