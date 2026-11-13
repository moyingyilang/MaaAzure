#!/system/bin/sh
# vd.sh —— 对指定虚拟屏抓帧 / 注入输入（通用工具，与具体项目无关）
#
#   vd.sh info                      打印 SF id 与逻辑 id
#   vd.sh capture [png]             抓帧（默认 /sdcard/vd.png）
#   vd.sh tap X Y                   点击
#   vd.sh swipe X1 Y1 X2 Y2 [ms]    滑动（默认 300ms）
#   vd.sh check                     抓帧并断言「有画面」（亮度>50 且颜色数>100）
#
# 环境变量：
#   VD_NAME   虚拟屏名字（displayName）；留空则取第一个虚拟屏，默认 TestVD
#   VD_OUT    抓帧输出路径，默认 /sdcard/vd.png
#
# 三条铁律：
#   1) screencap 认 SurfaceFlinger id，input 认逻辑 id，不能互换
#   2) 逻辑 id 必须由 SF id 反查，查不到就失败，绝不回落 0（否则会点到物理屏）
#   3) 不改 wm size / wm density 等全局显示设置
set -u
NAME="${VD_NAME:-TestVD}"
OUT="${VD_OUT:-/sdcard/vd.png}"

sf_id() {
    # VD_NAME 为空时取第一个虚拟屏；否则按 displayName 精确匹配
    if [ -z "$NAME" ]; then
        dumpsys SurfaceFlinger --display-id 2>/dev/null \
          | sed -n "s/.*Display \([0-9]*\) (Virtual display).*/\1/p" | head -1
    else
        dumpsys SurfaceFlinger --display-id 2>/dev/null \
          | sed -n "s/.*Display \([0-9]*\) (Virtual display).*displayName=\"$NAME\".*/\1/p" | head -1
    fi
}

logical_id() {
    sf=$(sf_id)
    [ -n "$sf" ] || { echo "找不到虚拟屏 displayName=\"$NAME\"" >&2; return 1; }
    lid=$(dumpsys display 2>/dev/null | awk -v sf="$sf" '
        index($0, sf) { f=1 }
        f && match($0, /displayId=[0-9]+/) { print substr($0, RSTART+10, RLENGTH-10); exit }')
    [ -n "$lid" ] || { echo "SF id $sf 反查不到逻辑 display id" >&2; return 1; }
    printf '%s' "$lid"
}

check_frame() {
    python3 - "$OUT" <<'PY'
import sys
from PIL import Image
import numpy as np
a = np.asarray(Image.open(sys.argv[1]).convert('RGB'))
bright = float(a.mean())
non_black = float((a.sum(axis=2) >= 30).mean() * 100)
colors = len(np.unique(a.reshape(-1, 3), axis=0))
ok = bright > 50 and colors > 100
print(f"尺寸 {a.shape[1]}x{a.shape[0]} 亮度 {bright:.1f} 非黑 {non_black:.1f}% 颜色 {colors} → {'有画面' if ok else '黑屏/异常'}")
sys.exit(0 if ok else 1)
PY
}

case "${1:-}" in
  info)
      sf=$(sf_id) || exit 1
      lid=$(logical_id) || exit 1
      printf 'sf=%s logical=%s\n' "$sf" "$lid" ;;
  capture)
      sf=$(sf_id) || exit 1
      screencap -d "$sf" -p "${2:-$OUT}" || exit 1
      echo "已抓帧 → ${2:-$OUT}" ;;
  check)
      sf=$(sf_id) || exit 1
      screencap -d "$sf" -p "$OUT" || exit 1
      check_frame ;;
  tap)
      [ $# -ge 3 ] || { echo "用法: vd.sh tap X Y" >&2; exit 2; }
      lid=$(logical_id) || exit 1
      input -d "$lid" tap "$2" "$3" ;;
  swipe)
      [ $# -ge 5 ] || { echo "用法: vd.sh swipe X1 Y1 X2 Y2 [ms]" >&2; exit 2; }
      lid=$(logical_id) || exit 1
      input -d "$lid" swipe "$2" "$3" "$4" "$5" "${6:-300}" ;;
  *)
      echo "用法: vd.sh {info|capture [png]|check|tap X Y|swipe X1 Y1 X2 Y2 [ms]}" >&2
      exit 2 ;;
esac
