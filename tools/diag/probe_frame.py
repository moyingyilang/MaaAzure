#!/usr/bin/env python3
"""对一张真实帧做「模板 × 帧」匹配排行，用来判断画面停在哪一屏。

这是**开发期诊断脚本**，不参与运行时（MaaAzure 运行时零 Python）。

用法：
    python3 tools/diag/probe_frame.py <frame.png> [--top 12] [--filter ui_white]

原理：遍历 resource/cn/pipeline/gen 里所有带 roi 的节点，用 NCC 比较
「模板」与「帧在 roi 处的裁切」，输出分数排行。
> 0.9 基本可认为是同一图案；一排同类按钮同时高说明确实在该页，
只有一个高、其余都很低，说明画面不在那一页。

注意：roi 是 MaaFramework 的 [x, y, w, h]，不是 [x1,y1,x2,y2]。
"""
from __future__ import annotations

import argparse
import glob
import json
import os

from PIL import Image
import numpy as np

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))


def load_nodes():
    nodes = {}
    for path in glob.glob(os.path.join(ROOT, "resource/cn/pipeline/gen/*.json")):
        with open(path, encoding="utf-8") as f:
            for name, node in json.load(f).items():
                if name.startswith("_"):
                    continue
                nodes[name] = node
    return nodes


def score(frame, tpl_path, roi):
    if not os.path.isfile(tpl_path):
        return None
    im = Image.open(tpl_path).convert("RGB")
    x, y, w, h = roi
    if im.size != (w, h):
        return None
    if x + w > frame.shape[1] or y + h > frame.shape[0]:
        return None
    a = frame[y:y + h, x:x + w]
    b = np.asarray(im, dtype=np.float32)
    a = a - a.mean()
    b = b - b.mean()
    denom = np.linalg.norm(a) * np.linalg.norm(b)
    return float((a * b).sum() / denom) if denom else None


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("frame")
    ap.add_argument("--top", type=int, default=12)
    ap.add_argument("--filter", default="", help="只看名字包含该子串的节点")
    args = ap.parse_args()

    frame = np.asarray(Image.open(args.frame).convert("RGB"), dtype=np.float32)
    nodes = load_nodes()
    rows = []
    for name, node in nodes.items():
        tpl, roi = node.get("template"), node.get("roi")
        if not isinstance(tpl, str) or not roi or tpl.endswith("/"):
            continue
        if args.filter and args.filter not in name:
            continue
        s = score(frame, os.path.join(ROOT, "resource/cn/image", tpl), roi)
        if s is not None:
            rows.append((s, name))

    rows.sort(reverse=True)
    print(f"帧 {frame.shape[1]}x{frame.shape[0]}，参与比对 {len(rows)} 个节点")
    print(f"{'分数':>8}  节点")
    for s, name in rows[: args.top]:
        print(f"{s:+8.3f}  {name}")
    hits = [n for s, n in rows if s > 0.9]
    print(f"\n>0.9 命中 {len(hits)} 个：{', '.join(hits[:10])}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
