#!/usr/bin/env python3
"""在 VD 上扫点：找出这块屏上「点了有反应」的位置。

原理：对网格里的每个坐标点一下，比较点击前后的帧差；差大说明该处可交互。
页面只有一个按钮时，这个方法能在无人看屏的情况下把它找出来。

    python3 tools/diag/scan_click.py [--cols 6] [--rows 4] [--thresh 8]
"""
from __future__ import annotations

import argparse
import os
import subprocess
import time

from PIL import Image
import numpy as np

SHOT = "/data/local/tmp/scan.png"


def sh(cmd: str) -> str:
    return subprocess.run(["su", "-c", cmd], capture_output=True, text=True).stdout


def sf_id() -> str:
    out = sh("dumpsys SurfaceFlinger --display-id")
    for line in out.splitlines():
        if "Virtual display" in line:
            return line.split()[1]
    return ""


def logical_id(sf: str) -> str:
    """把 SurfaceFlinger 的 display id 映射到逻辑 display id。

    注意：不能简单取「最后一个 mDisplayId」，那可能是物理屏（0）。
    之前就是这样把点击打到用户主屏幕上的。
    """
    out = sh("dumpsys display")
    cur = None
    for line in out.splitlines():
        for tok in line.split():
            if tok.startswith("mDisplayId="):
                cur = tok.split("=")[1].strip("(,")
            if tok.startswith("uniqueId=") and "virtual" in line.lower() and cur:
                return cur
    # 兜底：从 SurfaceFlinger 的 display id 反查 dumpsys display 里的同名块
    for line in out.splitlines():
        if sf in line:
            for tok in line.split():
                if tok.startswith("mDisplayId="):
                    return tok.split("=")[1].strip("(,")
    return ""


def grab(sf: str) -> np.ndarray | None:
    if not sf:
        return None
    subprocess.run(["su", "-c", f"screencap -d {sf} -p {SHOT}"], capture_output=True)
    subprocess.run(["su", "-c", f"cp {SHOT} /sdcard/scan.png"], capture_output=True)
    try:
        return np.asarray(Image.open("/sdcard/scan.png").convert("RGB"), dtype=np.float32)
    except Exception:
        return None


def main() -> int:
    # ⚠️ 安全闸门：这个脚本会**真实注入点击**。没有显式 opt-in 一律拒绝执行。
    # 之前它把点击打到了物理屏（逻辑 display 取错成 0），踩过坑。
    if os.environ.get("MAAGUI_ALLOW_INPUT") != "1":
        print("拒绝执行：该脚本会注入真实点击。确认目标无误后加 MAAGUI_ALLOW_INPUT=1 再跑。")
        return 2

    ap = argparse.ArgumentParser()
    ap.add_argument("--cols", type=int, default=6)
    ap.add_argument("--rows", type=int, default=4)
    ap.add_argument("--thresh", type=float, default=8.0)
    ap.add_argument("--wait", type=float, default=2.5)
    args = ap.parse_args()

    sf = sf_id()
    if not sf:
        print("没有虚拟屏（先在 App 里启动任务）")
        return 1
    lid = logical_id(sf)
    if not lid:
        print(f"拿不到 {sf} 对应的逻辑 display id，拒绝执行（防止点到物理屏）")
        return 1
    print(f"虚拟屏 SF={sf} 逻辑 display={lid}")

    base = grab(sf)
    if base is None:
        print("抓帧失败")
        return 1
    h, w = base.shape[:2]
    print(f"帧 {w}x{h}，扫 {args.cols}x{args.rows} = {args.cols * args.rows} 个点")

    hits = []
    for r in range(args.rows):
        for c in range(args.cols):
            x = int((c + 0.5) * w / args.cols)
            y = int((r + 0.5) * h / args.rows)
            sh(f"input -d {lid} tap {x} {y}")
            time.sleep(args.wait)
            after = grab(sf)
            if after is None:
                continue
            d = float(np.abs(base - after).mean())
            flag = "★" if d >= args.thresh else " "
            print(f"  {flag} ({x:4d},{y:4d}) 帧差 {d:5.2f}")
            if d >= args.thresh:
                hits.append((d, x, y))
            base = after

    print()
    if hits:
        hits.sort(reverse=True)
        print("可交互位置（帧差从大到小）：")
        for d, x, y in hits:
            print(f"  ({x},{y})  {d:.2f}")
    else:
        print("没有找到可交互位置（整屏点击都无反应）")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
