#!/usr/bin/env python3
"""
音视频压缩器 - FFmpeg驱动
"""

import sys
import json
import argparse
import subprocess
import os
from pathlib import Path

def check_ffmpeg():
    """检查ffmpeg是否可用"""
    try:
        subprocess.run(['ffmpeg', '-version'], capture_output=True, check=True)
        return True
    except:
        return False

def get_video_info(file_path):
    """获取视频信息"""
    cmd = [
        'ffprobe', '-v', 'quiet',
        '-print_format', 'json',
        '-show_format', '-show_streams',
        file_path
    ]
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, check=True)
        return json.loads(result.stdout)
    except:
        return None

def compress_video(input_file, output_file=None, crf=23, preset='medium', 
                   format=None, video_codec='libx264', audio_codec='aac'):
    """压缩视频"""
    if not check_ffmpeg():
        return {"success": False, "error": "需要安装FFmpeg"}
    
    input_path = Path(input_file)
    if not input_path.exists():
        return {"success": False, "error": f"文件不存在: {input_file}"}
    
    # 自动生成输出文件名
    if output_file is None:
        suffix = f".crf{crf}.{format}" if format else ".compressed.mp4"
        output_file = str(input_path.parent / f"{input_path.stem}{suffix}")
    
    # 格式映射
    if format == 'webm':
        video_codec = 'libvpx-vp9'
        audio_codec = 'libopus'
    elif format == 'mkv':
        format = 'matroska'
    
    cmd = [
        'ffmpeg', '-y', '-i', input_file,
        '-c:v', video_codec,
        '-crf', str(crf),
        '-preset', preset,
        '-c:a', audio_codec,
        '-b:a', '192k',
    ]
    
    if format:
        cmd.extend(['-f', format])
    
    cmd.append(output_file)
    
    try:
        # 获取原始大小
        original_size = input_path.stat().st_size
        
        # 执行压缩
        result = subprocess.run(
            cmd, capture_output=True, text=True,
            stderr=subprocess.STDOUT
        )
        
        if result.returncode != 0:
            return {"success": False, "error": result.stdout[-500:]}
        
        # 获取压缩后大小
        output_path = Path(output_file)
        compressed_size = output_path.stat().st_size if output_path.exists() else 0
        
        ratio = (1 - compressed_size/original_size) * 100 if original_size > 0 else 0
        
        return {
            "success": True,
            "input": input_file,
            "output": output_file,
            "original_size": original_size,
            "compressed_size": compressed_size,
            "compression_ratio": f"{ratio:.1f}%"
        }
    except Exception as e:
        return {"success": False, "error": str(e)}

def compress_audio(input_file, output_file=None, bitrate='192k', format='mp3'):
    """压缩音频"""
    if not check_ffmpeg():
        return {"success": False, "error": "需要安装FFmpeg"}
    
    input_path = Path(input_file)
    if not input_path.exists():
        return {"success": False, "error": f"文件不存在: {input_file}"}
    
    if output_file is None:
        suffix = f".{format}"
        output_file = str(input_path.parent / f"{input_path.stem}_compressed{suffix}")
    
    # 音频编码器映射
    codec_map = {'mp3': 'libmp3lame', 'aac': 'aac', 'ogg': 'libvorbis', 'flac': 'flac'}
    codec = codec_map.get(format, 'libmp3lame')
    
    cmd = ['ffmpeg', '-y', '-i', input_file, '-c:a', codec, '-b:a', bitrate, output_file]
    
    try:
        original_size = input_path.stat().st_size
        result = subprocess.run(cmd, capture_output=True, text=True)
        
        if result.returncode != 0:
            return {"success": False, "error": result.stderr[-500:]}
        
        output_path = Path(output_file)
        compressed_size = output_path.stat().st_size if output_path.exists() else 0
        
        return {
            "success": True,
            "input": input_file,
            "output": output_file,
            "original_size": original_size,
            "compressed_size": compressed_size,
            "compression_ratio": f"{(1-compressed_size/original_size)*100:.1f}%" if original_size > 0 else "0%"
        }
    except Exception as e:
        return {"success": False, "error": str(e)}

def batch_compress(input_files, output_dir="./compressed", crf=23, preset='medium'):
    """批量压缩"""
    Path(output_dir).mkdir(parents=True, exist_ok=True)
    
    results = []
    for input_file in input_files:
        input_path = Path(input_file)
        output_file = str(Path(output_dir) / f"{input_path.stem}_compressed.mp4")
        result = compress_video(input_file, output_file, crf, preset)
        results.append(result)
    
    return {
        "success": True,
        "total": len(results),
        "results": results
    }

def main():
    if not check_ffmpeg():
        print(json.dumps({"success": False, "error": "需要安装FFmpeg: https://ffmpeg.org/download.html"}))
        sys.exit(1)
    
    parser = argparse.ArgumentParser(description='音视频压缩器')
    parser.add_argument('action', choices=['video', 'audio', 'batch', 'info'])
    parser.add_argument('--input', help='输入文件')
    parser.add_argument('--output', help='输出文件')
    parser.add_argument('--crf', type=int, default=23, help='质量(0-51)')
    parser.add_argument('--preset', default='medium', help='编码速度')
    parser.add_argument('--format', help='输出格式')
    parser.add_argument('--bitrate', default='192k', help='音频比特率')
    parser.add_argument('--output-dir', default='./compressed', help='输出目录')
    parser.add_argument('files', nargs='*', help='文件列表')
    
    args = parser.parse_args()
    
    if args.action == 'info':
        info = get_video_info(args.input)
        print(json.dumps(info, indent=2))
    elif args.action == 'video':
        result = compress_video(args.input, args.output, args.crf, args.preset, args.format)
        print(json.dumps(result, indent=2))
    elif args.action == 'audio':
        result = compress_audio(args.input, args.output, args.bitrate, args.format or 'mp3')
        print(json.dumps(result, indent=2))
    elif args.action == 'batch':
        files = args.files if args.files else []
        if not files:
            print("ERROR: batch模式需要指定文件")
            sys.exit(1)
        result = batch_compress(files, args.output_dir, args.crf, args.preset)
        print(json.dumps(result, indent=2))

if __name__ == '__main__':
    main()
