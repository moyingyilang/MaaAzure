#!/system/bin/sh
# 把游戏拉起来、等到真正出画面、并把窗口固定成 1280×720（与资源同分辨率）。
#
#   ./scripts/game-up.sh [包名] [宽] [高] [最长等待秒]
#
# 「启动成功」的判定标准是**画面非黑**，不是进程存在：
# 冷启动会先黑屏几秒到几十秒，进程在 ≠ 画面在。
set -u
PKG="${1:-com.bilibili.azurlane}"
W="${2:-1280}"
H="${3:-720}"
WAIT="${4:-120}"
HERE="$(cd "$(dirname "$0")" && pwd)"
DAEMON="${DAEMON:-$HERE/../target/debug/maazure-daemon}"
ACTIVITY="${ACTIVITY:-com.manjuu.azurlane.MainActivity}"

say() { printf '%s\n' "$*"; }

say "→ 启动 $PKG"
su -c "am force-stop $PKG" >/dev/null 2>&1
su -c "am start -n $PKG/$ACTIVITY" >/dev/null 2>&1

# 桌面模式下窗口尺寸是**逻辑坐标**，物理 = 逻辑 × (物理宽 / 逻辑宽)
logical_width() {
    su -c "dumpsys display" 2>/dev/null | grep -o 'largest app [0-9]* x [0-9]*' | tail -1 | awk '{print $3}'
}
physical_width() {
    su -c "wm size" 2>/dev/null | sed -n 's/Physical size: \([0-9]*\)x.*/\1/p'
}

i=0
while [ "$i" -lt "$WAIT" ]; do
    sleep 3
    i=$((i + 3))
    out=$("$DAEMON" --capture-window "$PKG" --out /data/local/tmp/game-up-crop.png 2>/dev/null) || continue
    b=$(printf '%s' "$out" | sed -n 's/.*平均亮度 *\([0-9.]*\).*/\1/p')
    rect=$(printf '%s' "$out" | sed -n 's/.*窗口物理矩形 *left=\([0-9]*\) top=\([0-9]*\) \([0-9]*\)x\([0-9]*\).*/\1,\2 \3x\4/p')
    say "  t=${i}s 亮度=${b:-无} 窗口=${rect:-?}"
    # 亮度 < 10 视为还在黑屏/过渡（调试用的是实测阈值，不是拍脑袋）
    if [ -n "$b" ]; then
        case "$b" in
            0.*|1.*|2.*|3.*|4.*|5.*|6.*|7.*|8.*|9.*) continue ;;
        esac
    else
        continue
    fi

    say "✓ 游戏已渲染（亮度 $b，窗口 $rect）"
    case "$rect" in
        *"$W"x"$H")
            say "  窗口已是 ${W}×${H}，识别可以 1:1 走"
            exit 0
            ;;
    esac

    tid=$(su -c "am stack list" 2>/dev/null | sed -n "s/.*taskId=\([0-9]*\): $PKG.*/\1/p" | head -1)
    lw=$(logical_width)
    pw=$(physical_width)
    if [ -n "$tid" ] && [ -n "$lw" ] && [ -n "$pw" ]; then
        tw=$(( W * lw / pw ))
        th=$(( H * lw / pw ))
        su -c "am task resize $tid 100 200 $((100 + tw)) $((200 + th))" >/dev/null 2>&1
        say "  已把窗口调成逻辑 ${tw}x${th}（物理 ${W}×${H}）"
        exit 0
    fi
    say "  ! 拿不到 taskId / 桌面尺寸，窗口保持原样"
    exit 0
done

say "✗ ${WAIT}s 内没等到非黑画面"
exit 1
