#!/system/bin/sh
# 只更新设备上的 PI 资源（不重出 APK、不重装），把改动生效时间从 ~2 分钟压到 ~10 秒。
#
#   ./scripts/deploy-pi.sh [--restart]
#
# 原理：MaaFwApp 运行期从
#   /sdcard/Android/data/<包名>/files/pi/
# 读取 interface.json / interface.d / resource，这是它首次启动时从 APK 解包的副本。
# 我们不换 APK 时可以直接覆盖这份副本，然后重启 App 让它重新加载。
#
# ⚠️ 前提：APK 里已经包含过一次完整资源（否则 pi/ 目录不存在）。
set -u
PKG="${PKG:-com.aliothmoon.maafw.maazure}"
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/.." && pwd)"
PI="/sdcard/Android/data/$PKG/files/pi"

if ! su -c "test -d $PI" 2>/dev/null; then
    echo "设备上没有 $PI —— 先装一次 APK（脚本 build-apk.sh）再来。"
    exit 1
fi

echo "→ 同步资源到 $PI"
if [ "${FAST:-0}" = "1" ]; then
    # 快速模式：只同步管线（改 wiring/生成节点时用这个，几 MB，秒级）
    TARGETS="resource/common/pipeline resource/cn/pipeline resource/en/pipeline resource/jp/pipeline resource/tw/pipeline interface.json"
else
    TARGETS="interface.json interface.d resource data"
fi
# su 解包出来的文件属主是 root，App（u0_aXXX）读不了 → EACCES。
# 先记下 pi 目录原本的属主，解包后原样还原。
OWNER=$(su -c "stat -c '%u:%g' $PI" 2>/dev/null)
( cd "$ROOT" && tar cf - $TARGETS 2>/dev/null ) \
    | su -c "cd $PI && tar xf -" || { echo "同步失败"; exit 1; }
if [ -n "${OWNER:-}" ]; then
    su -c "chown -R $OWNER $PI" 2>/dev/null
    su -c "find $PI -type d -exec chmod 771 {} +" 2>/dev/null
    su -c "find $PI -type f -exec chmod 660 {} +" 2>/dev/null
    echo "  属主已还原为 $OWNER"
fi
echo "  $TARGETS"

if [ "${1:-}" = "--restart" ] || [ "${RESTART:-1}" = "1" ]; then
    echo "→ 重启 App 让它重新加载资源"
    su -c "am force-stop $PKG" 2>/dev/null
    sleep 2
    su -c "am start -n $PKG/com.aliothmoon.maafw.MainActivity" >/dev/null 2>&1
    sleep 6
fi
echo "✓ 完成（无需重出 APK）"
