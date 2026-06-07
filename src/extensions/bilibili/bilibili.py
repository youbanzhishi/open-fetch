#!/usr/bin/env python3
"""
Bilibili 扩展 - B站视频/直播/弹幕/封面下载

入口函数：resolve(params: dict) -> dict
"""

import json
import re
import sys
from typing import Dict, Any, Optional, List

try:
    import requests
except ImportError:
    print("需要安装 requests: pip install requests")
    sys.exit(1)


class BilibiliDownloader:
    """B站下载器"""
    
    USER_AGENT = "Mozilla/5.0 (compatible; OpenFetch/1.0)"
    APP_KEY = "iVGUTjsxvpLeuMfC"
    BASE_URL = "https://api.bilibili.com"
    
    def __init__(self):
        self.session = requests.Session()
        self.session.headers.update({"User-Agent": self.USER_AGENT})
    
    def resolve(self, params: Dict[str, Any]) -> Dict[str, Any]:
        """
        解析B站URL，返回元信息和下载链接
        
        Args:
            params: 包含 url, quality, download_danmu 等参数的字典
            
        Returns:
            包含 title, files, danmu, cover 等的字典
        """
        url = params.get("url", "")
        
        # 解析URL类型
        if "/video/BV" in url or url.startswith("BV"):
            return self.resolve_video(url, params)
        elif "/live.bilibili.com" in url:
            return self.resolve_live(url, params)
        elif "/audio/au" in url:
            return self.resolve_audio(url, params)
        else:
            raise ValueError(f"不支持的URL类型: {url}")
    
    def resolve_video(self, url: str, params: Dict[str, Any]) -> Dict[str, Any]:
        """解析视频"""
        # 提取BV号
        bv_id = self.extract_bv_id(url)
        if not bv_id:
            raise ValueError(f"无法解析BV号: {url}")
        
        # 获取视频信息
        video_info = self.get_video_info(bv_id)
        
        # 提取下载链接
        quality = params.get("quality", "1080p")
        play_url = self.get_play_url(bv_id, quality)
        
        # 构建结果
        result = {
            "title": video_info.get("title", "unknown"),
            "files": [{
                "url": play_url,
                "filename": f"{video_info.get('title', 'video')}.mp4",
                "format": "mp4",
                "quality": quality
            }],
            "metadata": {
                "aid": video_info.get("aid"),
                "bvid": bv_id,
                "cid": video_info.get("cid"),
                "duration": video_info.get("duration"),
                "description": video_info.get("description", ""),
                "owner": video_info.get("owner", {}).get("name", ""),
                "pubdate": video_info.get("pubdate")
            }
        }
        
        # 下载封面
        if params.get("download_cover", False):
            result["cover"] = video_info.get("pic")
        
        # 下载弹幕
        if params.get("download_danmu", False):
            cid = video_info.get("cid")
            if cid:
                result["danmu"] = self.get_danmu(cid)
        
        return result
    
    def resolve_live(self, url: str, params: Dict[str, Any]) -> Dict[str, Any]:
        """解析直播间"""
        room_id = self.extract_room_id(url)
        
        # 获取直播信息
        live_info = self.get_live_info(room_id)
        
        return {
            "title": f"直播: {live_info.get('uname', 'unknown')}",
            "status": live_info.get("live_status"),
            "room_id": room_id,
            "stream_url": live_info.get("live_url"),
            "files": [{
                "url": live_info.get("live_url", ""),
                "filename": f"live_{room_id}.flv",
                "format": "flv"
            }]
        }
    
    def extract_bv_id(self, url: str) -> Optional[str]:
        """提取BV号"""
        # 匹配 BVxxx 或完整URL
        match = re.search(r'BV([a-zA-Z0-9]+)', url)
        if match:
            return "BV" + match.group(1)
        
        # 直接是BV号
        match = re.match(r'^BV[a-zA-Z0-9]+$', url.strip())
        if match:
            return match.group(0)
        
        return None
    
    def extract_room_id(self, url: str) -> Optional[str]:
        """提取房间号"""
        match = re.search(r'live\.bilibili\.com/(\d+)', url)
        if match:
            return match.group(1)
        return None
    
    def get_video_info(self, bv_id: str) -> Dict[str, Any]:
        """获取视频信息"""
        api = f"{self.BASE_URL}/x/web-interface/view"
        params = {"bvid": bv_id}
        
        resp = self.session.get(api, params=params, timeout=30)
        data = resp.json()
        
        if data.get("code") != 0:
            raise ValueError(f"API错误: {data.get('message')}")
        
        return data.get("data", {})
    
    def get_play_url(self, bv_id: str, quality: str) -> str:
        """获取播放链接"""
        # 质量映射
        quality_map = {
            "360p": 16,
            "480p": 32,
            "720p": 64,
            "1080p": 80,
            "1080p+": 112,
            "4k": 116
        }
        qn = quality_map.get(quality, 80)
        
        api = f"{self.BASE_URL}/x/player/playurl"
        params = {
            "bvid": bv_id,
            "qn": qn,
            "fnval": 1,
            "fnver": 0
        }
        
        resp = self.session.get(api, params=params, timeout=30)
        data = resp.json()
        
        if data.get("code") != 0:
            raise ValueError(f"播放链接获取失败: {data.get('message')}")
        
        # 返回最优质量的链接
        durl = data.get("data", {}).get("durl", [])
        if durl:
            return durl[0].get("url", "")
        
        return ""
    
    def get_danmu(self, cid: int) -> List[Dict[str, Any]]:
        """获取弹幕"""
        api = f"{self.BASE_URL}/x/v1/dm/list.so"
        params = {"oid": cid}
        
        resp = self.session.get(api, params=params, timeout=30)
        # 弹幕是XML格式，需要解析
        # 这里返回占位，实际实现需要XML解析
        return []
    
    def get_live_info(self, room_id: str) -> Dict[str, Any]:
        """获取直播信息"""
        api = f"https://api.live.bilibili.com/room/v1/Room/get_info"
        params = {"room_id": room_id}
        
        resp = self.session.get(api, params=params, timeout=30)
        data = resp.json()
        
        if data.get("code") != 0:
            raise ValueError(f"直播间信息获取失败: {data.get('message')}")
        
        info = data.get("data", {})
        return {
            "uname": info.get("uname", ""),
            "live_status": info.get("live_status"),
            "live_url": f"https://live.bilibili.com/{room_id}"
        }


def resolve(params: Dict[str, Any]) -> Dict[str, Any]:
    """
    OpenFetch 扩展入口函数
    """
    downloader = BilibiliDownloader()
    return downloader.resolve(params)


if __name__ == "__main__":
    # 测试
    test_params = {
        "url": "https://www.bilibili.com/video/BV1xx411c7mD",
        "quality": "1080p",
        "download_cover": True
    }
    
    try:
        result = resolve(test_params)
        print(json.dumps(result, ensure_ascii=False, indent=2))
    except Exception as e:
        print(f"错误: {e}", file=sys.stderr)
        sys.exit(1)
