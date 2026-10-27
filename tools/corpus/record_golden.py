#!/usr/bin/env python3
"""golden corpus 录制器（一次性工具，不属于 MaaAzure 运行时）。

它挂在**冻结的 AzurPilot** 上，把一次真实运行录成「逐帧截图 + 每帧决策」，
供新线的 `ReplayEngine` 在无模拟器、无 MaaFramework 的环境里回放断言。

用法（在 AzurPilot 目录里跑，保证能 import 它的 module）：

    cd ../AzurPilot
    uv run python ../MaaAzure/tools/corpus/record_golden.py \
        --task commission --config alas --out ../MaaAzure/corpus

自检（不需要模拟器，用合成帧验证录制逻辑与产物格式）：

    uv run python ../MaaAzure/tools/corpus/record_golden.py --selftest

产物布局：

    corpus/<task>/
    ├── case.json              用例元信息
    ├── 0001.png               PNG（禁止 JPEG：压缩伪影会让模板匹配漂移）
    └── 0001.decision.json     该帧的决策
"""

from __future__ import annotations

import argparse
import json
import os
import sys
import time
import traceback

# 只保留最近 N 帧的 PNG 也可能很大；默认全留，靠调用方裁剪
DEFAULT_MAX_FRAMES = 4000


def _ensure_repo_on_path() -> None:
    """把当前目录（应为 AzurPilot）加进 sys.path，并校验关键模块存在。"""
    cwd = os.getcwd()
    if cwd not in sys.path:
        sys.path.insert(0, cwd)
    if not os.path.isdir(os.path.join(cwd, "module")):
        sys.exit(
            "请在 AzurPilot 仓库根目录运行（需要能 import module.*）。\n"
            f"当前目录：{cwd}"
        )


class Recorder:
    """采集帧与决策，按用例落盘。"""

    def __init__(self, out_dir: str, task: str, server: str = "cn",
                 resolution=(1280, 720), max_frames: int = DEFAULT_MAX_FRAMES):
        self.case_dir = os.path.join(out_dir, task)
        self.task = task
        self.server = server
        self.resolution = list(resolution)
        self.max_frames = max_frames

        self.frames = 0
        self.started_at = time.time()
        self.last_decision: dict | None = None
        self.decisions: list[dict] = []
        self.skipped_same = 0

        os.makedirs(self.case_dir, exist_ok=True)

    # ── 帧 ───────────────────────────────────────────────
    def on_screenshot(self, image) -> None:
        """每张截图存一张 PNG，并结算上一帧的决策。"""
        if self.frames >= self.max_frames:
            return
        self.frames += 1
        seq = self.frames
        png = os.path.join(self.case_dir, f"{seq:04d}.png")
        image.save(png)

        # 上一帧若记录了决策，就绑定到这张截图上（决策发生在看图之后）
        if self.last_decision is not None:
            self._write_decision(seq - 1, self.last_decision)
            self.last_decision = None

    def _write_decision(self, seq: int, decision: dict) -> None:
        record = {"frame": seq, **decision}
        path = os.path.join(self.case_dir, f"{seq:04d}.decision.json")
        with open(path, "w", encoding="utf-8") as f:
            json.dump(record, f, ensure_ascii=False, indent=2)
            f.write("\n")
        self.decisions.append(record)

    # ── 决策 ─────────────────────────────────────────────
    def on_appear(self, name: str, hit: bool) -> None:
        d = self.last_decision or {}
        d.setdefault("hits", {})[name] = bool(hit)
        self.last_decision = d

    def on_click(self, area, point) -> None:
        d = self.last_decision or {}
        d.setdefault("action", {"kind": "click", "area": list(area or ()),
                                "point": list(point or ())})
        self.last_decision = d

    def on_ocr(self, kind: str, area, text) -> None:
        d = self.last_decision or {}
        d.setdefault("ocr", []).append(
            {"kind": kind, "area": list(area or ()), "text": text}
        )
        self.last_decision = d

    # ── 收尾 ─────────────────────────────────────────────
    def finish(self, outcome: str, error: str | None = None) -> str:
        if self.last_decision is not None:
            self._write_decision(self.frames, self.last_decision)
            self.last_decision = None

        case = {
            "name": self.task,
            "source_rev": _git_rev(),
            "task": self.task,
            "server": self.server,
            "resolution": self.resolution,
            "frames": self.frames,
            "decisions": len(self.decisions),
            "duration_secs": round(time.time() - self.started_at, 1),
            "outcome": outcome,
            "error": error,
        }
        path = os.path.join(self.case_dir, "case.json")
        with open(path, "w", encoding="utf-8") as f:
            json.dump(case, f, ensure_ascii=False, indent=2)
            f.write("\n")
        return path


def _git_rev() -> str:
    try:
        import subprocess
        out = subprocess.run(["git", "rev-parse", "--short", "HEAD"],
                             capture_output=True, text=True, check=False)
        return out.stdout.strip() or "unknown"
    except Exception:
        return "unknown"


def install_hooks(rec: Recorder) -> None:
    """把录制器挂到旧仓的截图 / 识别 / 点击 / OCR 上。

    只做包装，不改行为——旧版逻辑必须原样运行，否则录下来的决策不是「真实决策」。
    """
    import module.base.base as base
    from module.ocr.ocr import Ocr

    # 1) 截图
    original_screenshot = base.ModuleBase.screenshot

    def screenshot(self, *args, **kwargs):
        image = original_screenshot(self, *args, **kwargs)
        try:
            rec.on_screenshot(image)
        except Exception:
            traceback.print_exc()
        return image

    base.ModuleBase.screenshot = screenshot

    # 2) 识别（appear / appear_then_click）
    original_appear = base.ModuleBase.appear

    def appear(self, button, *args, **kwargs):
        result = original_appear(self, button, *args, **kwargs)
        try:
            rec.on_appear(getattr(button, "name", str(button)), result)
        except Exception:
            traceback.print_exc()
        return result

    base.ModuleBase.appear = appear

    # 3) 点击
    original_click = base.ModuleBase.device.fget if isinstance(
        getattr(base.ModuleBase, "device", None), property) else None
    if original_click is None:
        # device 不是 property：包 ModuleBase.click
        original_click = getattr(base.ModuleBase, "click", None)
        if original_click is not None:
            def click(self, button, *args, **kwargs):
                try:
                    rec.on_click(getattr(button, "area", None),
                                 getattr(button, "button", None))
                except Exception:
                    traceback.print_exc()
                return original_click(self, button, *args, **kwargs)

            base.ModuleBase.click = click

    # 4) OCR 文本
    original_ocr = Ocr.ocr

    def ocr(self, image, *args, **kwargs):
        text = original_ocr(self, image, *args, **kwargs)
        try:
            rec.on_ocr(type(self).__name__, getattr(self, "area", None), text)
        except Exception:
            traceback.print_exc()
        return text

    Ocr.ocr = ocr


def run_task(task: str, config: str, out_dir: str, max_frames: int) -> int:
    """真跑：加载 AzurPilot，跑一个任务，全程录制。"""
    _ensure_repo_on_path()

    rec = Recorder(out_dir, task, max_frames=max_frames)
    install_hooks(rec)

    from alas import AzurLaneAutoScript

    outcome, error = "unknown", None
    try:
        az = AzurLaneAutoScript(config_name=config)
        method = getattr(az, task)
        result = method()
        outcome = "ok" if result is True or result is None else str(result)
    except SystemExit as e:  # 旧版在不可恢复错误时 exit(1)
        outcome, error = "exit", f"SystemExit({e.code})"
    except BaseException as e:  # noqa: BLE001 —— 录制的价值就在于连异常一起录
        outcome, error = "exception", f"{type(e).__name__}: {e}"
        traceback.print_exc()
    finally:
        path = rec.finish(outcome, error)
        print(f"[corpus] {rec.frames} 帧 / {len(rec.decisions)} 条决策 → {path}")

    return 0 if outcome in ("ok", "exit") else 1


def selftest(out_dir: str) -> int:
    """不需要模拟器：合成 3 帧 + 假决策，验证产物格式。"""
    from PIL import Image

    rec = Recorder(out_dir, "selftest")
    area = (100, 200, 300, 260)

    for i in range(3):
        img = Image.new("RGB", (1280, 720), (10 + i, 20, 30))
        rec.on_screenshot(img)
        rec.on_appear("TEST_BUTTON", hit=(i != 1))
        if i != 1:
            rec.on_click(area, (200, 230))
        rec.on_ocr("DigitCounter", (120, 210, 280, 250), "14/15")

    path = rec.finish("ok")

    # 校验产物
    with open(path, encoding="utf-8") as f:
        case = json.load(f)
    assert case["frames"] == 3, case
    assert case["decisions"] == 3, case
    assert case["outcome"] == "ok", case

    for seq in (1, 2, 3):
        png = os.path.join(rec.case_dir, f"{seq:04d}.png")
        dec = os.path.join(rec.case_dir, f"{seq:04d}.decision.json")
        assert os.path.isfile(png), png
        assert os.path.isfile(dec), dec
        with open(dec, encoding="utf-8") as f:
            d = json.load(f)
        assert d["frame"] == seq, d
        assert "hits" in d, d

    # 第 2 帧没点击：断言「未命中 + 无 action」也被如实记录
    with open(os.path.join(rec.case_dir, "0002.decision.json"), encoding="utf-8") as f:
        d2 = json.load(f)
    assert d2["hits"]["TEST_BUTTON"] is False, d2
    assert "action" not in d2, d2

    print(f"[selftest] OK：3 帧 / 3 条决策 / case.json 校验通过 → {rec.case_dir}")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description="golden corpus 录制器")
    ap.add_argument("--task", help="AzurPilot 的任务方法名，如 commission")
    ap.add_argument("--config", default="alas", help="配置名（config/<name>.json）")
    ap.add_argument("--out", default="corpus", help="输出目录")
    ap.add_argument("--max-frames", type=int, default=DEFAULT_MAX_FRAMES)
    ap.add_argument("--selftest", action="store_true", help="用合成帧自检，不需要模拟器")
    args = ap.parse_args()

    if args.selftest:
        return selftest(args.out)
    if not args.task:
        ap.error("需要 --task（或用 --selftest）")
    return run_task(args.task, args.config, args.out, args.max_frames)


if __name__ == "__main__":
    sys.exit(main())
