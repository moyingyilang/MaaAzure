#!/system/bin/sh
# 一条命令出 MaaAzure 的 APK，全部原始输出流式写到一个日志里。
#
#   ./build-apk.sh [日志路径]          默认 /sdcard/build_maaazure/stream.log
#
# 做四件事：
#   1) 用 Termux 的 clang 预编译 MaaFwApp 的 native 模块（没有 arm64 宿主 NDK/CMake）
#   2) 把 .so 放进 jniLibs
#   3) chroot 里跑 gradle :app:assembleRelease（verbose）
#   4) 把产物拷到 /sdcard/build_maaazure/
set -u
# 退出码要一路传到最后，但中间某步失败也要先把日志收完
trap 'true' EXIT

LOG="${1:-/sdcard/build_maaazure/stream.log}"
OUTDIR="$(dirname "$LOG")"
MAAFW="${MAAFW_APP:-/data/data/com.termux/files/home/build/MaaFwApp}"
HERE="$(cd "$(dirname "$0")" && pwd)"
CHROOT="${CHROOT:-/data/data/com.termux/files/home/.local/share/tmoe-linux/containers/chroot/debian-bookworm_arm64}"
JDK="${JDK:-/opt/jdk21-linux}"
GRADLE="${GRADLE:-/opt/gradle-9.8.0/bin/gradle}"

mkdir -p "$OUTDIR"
# 追加而不是覆盖：一次完整构建的每个阶段都留在同一份日志里
exec >>"$LOG" 2>&1

say() { printf '\n===== %s | %s =====\n' "$(date '+%F %T')" "$*"; }

say "开始构建 MaaAzure APK"
echo "MaaFwApp : $MAAFW"
echo "日志     : $LOG"
echo "chroot   : $CHROOT"

say "阶段 1/4：预编译 native 模块"
"$HERE/build-native.sh" "$MAAFW" "$MAAFW/app/src/main/jniLibs/arm64-v8a" || {
    echo "!! native 预编译失败"
    exit 1
}

say "阶段 2/4：同步资源与 agent 到 jniLibs"
ls -l "$MAAFW/app/src/main/jniLibs/arm64-v8a"

say "阶段 3/4：gradle :app:assembleRelease（verbose）"
# 先修属主：chroot 里以 root 跑 gradle，产物会变成 root 所有，
# 不修下一次宿主侧读不了（build dir 里已出现过 Permission denied）
chown -R 10446:10446 "$MAAFW" 2>/dev/null
su -c "chroot '$CHROOT' /usr/bin/env -i \
    PATH=$JDK/bin:/usr/bin:/bin \
    HOME=/root \
    JAVA_HOME=$JDK \
    ANDROID_HOME=/data/data/com.termux/files/home/android-sdk \
    ANDROID_SDK_ROOT=/data/data/com.termux/files/home/android-sdk \
    /bin/bash -c 'cd $MAAFW && $GRADLE :app:assembleRelease --no-daemon --console=plain --info'"

RC=$?
say "gradle 退出码 $RC"

say "阶段 4/4：收取产物"
APK=$(find "$MAAFW/app/build/outputs/apk" -name '*.apk' 2>/dev/null | head -1)
if [ -n "$APK" ]; then
    cp "$APK" "$OUTDIR/" && echo "APK → $OUTDIR/$(basename "$APK")"
    ls -lh "$OUTDIR"/*.apk
else
    echo "!! 没找到 APK"
fi
chown -R 10446:10446 "$MAAFW" 2>/dev/null

say "结束，退出码 $RC"
exit $RC
