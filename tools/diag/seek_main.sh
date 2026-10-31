#!/system/bin/sh
# 用「抓帧 → probe → 点一下」的闭环，把游戏从任意页面带回主界面。
#
#   ./tools/diag/seek_main.sh [最多迭代次数]
#
# 判据：白色主题主界面按钮在 probe 里同时命中 >= 4 个（>0.9）即为主界面。
# 这是**开发期探索脚本**，不参与运行时。
set -u
# ⚠️ 安全闸门：会真实注入点击；没有显式 opt-in 拒绝执行
if [ "${MAAGUI_ALLOW_INPUT:-}" != "1" ]; then
    echo "拒绝执行：该脚本会注入真实点击。确认目标无误后加 MAAGUI_ALLOW_INPUT=1 再跑。"
    exit 2
fi
MAX="${1:-18}"
SHOT=/data/local/tmp/seek.png
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../.." && pwd)"
PY=python3

sf_id() { su -c "dumpsys SurfaceFlinger --display-id" 2>/dev/null | grep "Virtual display" | sed 's/Display \([0-9]*\).*/\1/' | head -1; }
logical_id() { su -c "dumpsys window displays" 2>/dev/null | grep -o "mDisplayId=[0-9]* (organized)" | sed 's/[^0-9]//g' | tail -1; }

grab() {
    sf=$(sf_id)
    [ -z "$sf" ] && return 1
    su -c "screencap -d $sf -p $SHOT" >/dev/null 2>&1 || return 1
    su -c "cp $SHOT /sdcard/seek.png" >/dev/null 2>&1
}

hits() {
    $PY "$ROOT/tools/diag/probe_frame.py" /sdcard/seek.png --top 0 --filter ui_white_main_goto 2>/dev/null \
        | sed -n 's/.*>0.9 命中 \([0-9]*\) 个.*/\1/p'
}

i=0
while [ "$i" -lt "$MAX" ]; do
    i=$((i + 1))
    if ! grab; then echo "第 $i 轮：VD 不存在"; sleep 3; continue; fi
    h=$(hits); h=${h:-0}
    echo "第 $i 轮：主界面按钮命中 $h 个"
    [ "$h" -ge 4 ] && { echo "✓ 已在主界面"; exit 0; }

    lid=$(logical_id)
    case $((i % 4)) in
        1) pos="60 40" ;;      # 左上返回
        2) pos="640 660" ;;    # 底部中间
        3) pos="1180 60" ;;    # 右上关闭
        0) pos="640 360" ;;    # 屏幕中心
    esac
    echo "   点 ($pos)"
    su -c "input -d $lid tap $pos" >/dev/null 2>&1
    sleep 3
done
echo "✗ $MAX 轮内没回到主界面"
exit 1
