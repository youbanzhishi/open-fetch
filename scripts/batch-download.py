#!/usr/bin/env python3
"""
OpenFetch 批量下载工具
支持从文件读取URL列表、剪贴板导入、订阅源批量下载
"""

import sys
import json
import argparse
import subprocess
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor, as_completed
from dataclasses import dataclass
from typing import List, Optional
import re

@dataclass
class DownloadTask:
    url: str
    extractor: str
    quality: str = "best"
    output: str = "./downloads"
    title: str = ""

class BatchDownloader:
    def __init__(self, max_workers: int = 3):
        self.max_workers = max_workers
        self.results = []
    
    def detect_extractor(self, url: str) -> str:
        """自动检测下载器"""
        url_lower = url.lower()
        
        patterns = [
            (["bilibili.com", "b23.tv"], "bilibili"),
            (["youtube.com", "youtu.be"], "youtube"),
            (["douyin.com", "iesdouyin.com", "tiktok.com", "v.douyin.com"], "douyin"),
            (["weibo.com", "weibo.cn"], "weibo"),
            (["twitter.com", "x.com"], "twitter"),
            (["instagram.com"], "instagram"),
            (["xiaohongshu.com", "xhs.co"], "xiaohongshu"),
            (["zhihu.com"], "zhihu"),
            (["kuaishou.com", "ksyun.com"], "kuishou"),
            (["twitch.tv"], "live"),
            (["douyu.com"], "live"),
            (["huya.com"], "live"),
        ]
        
        for keywords, extractor in patterns:
            if any(k in url_lower for k in keywords):
                return extractor
        return "universal"
    
    def parse_file(self, file_path: str) -> List[DownloadTask]:
        """从文件解析下载任务"""
        tasks = []
        with open(file_path, 'r', encoding='utf-8') as f:
            for line in f:
                line = line.strip()
                if not line or line.startswith('#'):
                    continue
                
                # 支持格式: url 或 url|extractor|quality
                parts = line.split('|')
                url = parts[0].strip()
                
                if url.startswith(('http://', 'https://')):
                    extractor = parts[1].strip() if len(parts) > 1 else self.detect_extractor(url)
                    quality = parts[2].strip() if len(parts) > 2 else "best"
                    
                    tasks.append(DownloadTask(
                        url=url,
                        extractor=extractor,
                        quality=quality
                    ))
        
        return tasks
    
    def download_single(self, task: DownloadTask) -> dict:
        """下载单个任务"""
        try:
            # 构造命令
            cmd = [
                sys.executable,
                f"src/extensions/{task.extractor}/downloader.py",
                "--url", task.url,
                "--output", task.output
            ]
            
            if task.extractor in ["youtube", "bilibili"]:
                cmd.extend(["--quality", task.quality])
            
            result = subprocess.run(
                cmd,
                capture_output=True,
                text=True,
                timeout=600  # 10分钟超时
            )
            
            if result.returncode == 0:
                try:
                    data = json.loads(result.stdout)
                    return {"success": True, "url": task.url, "data": data}
                except:
                    return {"success": True, "url": task.url, "data": result.stdout}
            else:
                return {"success": False, "url": task.url, "error": result.stderr[-500:]}
                
        except subprocess.TimeoutExpired:
            return {"success": False, "url": task.url, "error": "下载超时(10分钟)"}
        except Exception as e:
            return {"success": False, "url": task.url, "error": str(e)}
    
    def download_batch(self, tasks: List[DownloadTask], progress_callback=None) -> List[dict]:
        """批量下载"""
        results = []
        total = len(tasks)
        
        with ThreadPoolExecutor(max_workers=self.max_workers) as executor:
            futures = {executor.submit(self.download_single, task): task for task in tasks}
            
            for i, future in enumerate(as_completed(futures), 1):
                result = future.result()
                results.append(result)
                
                if progress_callback:
                    progress_callback(i, total, result)
                else:
                    status = "✓" if result["success"] else "✗"
                    print(f"[{i}/{total}] {status} {result['url'][:60]}")
        
        return results
    
    def print_summary(self, results: List[dict]):
        """打印统计摘要"""
        total = len(results)
        success = sum(1 for r in results if r["success"])
        failed = total - success
        
        print("\n" + "="*50)
        print(f"📊 下载统计")
        print(f"   总计: {total}")
        print(f"   成功: {success} ✓")
        print(f"   失败: {failed} ✗")
        print("="*50)
        
        if failed > 0:
            print("\n失败任务:")
            for r in results:
                if not r["success"]:
                    print(f"  - {r['url']}")
                    if "error" in r:
                        print(f"    错误: {r['error'][:100]}")

def main():
    parser = argparse.ArgumentParser(description='OpenFetch 批量下载')
    parser.add_argument('--file', '-f', help='URL列表文件(每行一个URL)')
    parser.add_argument('--urls', '-u', nargs='+', help='URL列表(命令行参数)')
    parser.add_argument('--output', '-o', default='./downloads', help='输出目录')
    parser.add_argument('--quality', '-q', default='best', help='画质')
    parser.add_argument('--workers', '-w', type=int, default=3, help='并发数')
    parser.add_argument('--list', '-l', action='store_true', help='列出所有扩展')
    
    args = parser.parse_args()
    
    batch = BatchDownloader(max_workers=args.workers)
    
    if args.list:
        print("支持的平台扩展:")
        extensions = [
            ("universal", "通用下载器(50+平台)"),
            ("bilibili", "B站"),
            ("youtube", "YouTube"),
            ("douyin", "抖音/TikTok"),
            ("weibo", "微博"),
            ("twitter", "Twitter/X"),
            ("instagram", "Instagram"),
            ("xiaohongshu", "小红书"),
            ("zhihu", "知乎"),
            ("kuishou", "快手"),
            ("live", "直播录制"),
        ]
        for ext, name in extensions:
            print(f"  {ext:15} - {name}")
        return
    
    tasks = []
    
    # 从文件读取
    if args.file:
        tasks.extend(batch.parse_file(args.file))
    
    # 从命令行参数读取
    if args.urls:
        for url in args.urls:
            tasks.append(DownloadTask(
                url=url,
                extractor=batch.detect_extractor(url),
                quality=args.quality,
                output=args.output
            ))
    
    if not tasks:
        parser.print_help()
        print("\n示例:")
        print("  openfetch-batch -u 'https://bilibili.com/video/BVxxx' 'https://youtube.com/watch?v=xxx'")
        print("  openfetch-batch -f urls.txt -o /tmp/downloads")
        return
    
    # 确保输出目录存在
    Path(args.output).mkdir(parents=True, exist_ok=True)
    
    print(f"🚀 开始批量下载 {len(tasks)} 个任务 (并发: {args.workers})")
    
    results = batch.download_batch(tasks)
    batch.print_summary(results)
    
    # 保存结果到JSON
    result_file = Path(args.output) / "batch_results.json"
    with open(result_file, 'w', encoding='utf-8') as f:
        json.dump(results, f, ensure_ascii=False, indent=2)
    print(f"\n📄 结果已保存到: {result_file}")

if __name__ == '__main__':
    main()
