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

say "阶段 3/4：写入版本号并 gradle 出包"
# 版本号唯一真源是 Android/version.txt（未发布 0.x.y，正式版 1.x.y 起）。
# MaaFwApp 默认用 git describe 当 versionName（会显示成 7e2dc7d 这种哈希），
# 这里覆盖成我们自己的版本号，versionCode 用 major*10000+minor*100+patch。
VER=$(cat "$HERE/version.txt" 2>/dev/null || echo "0.1.0")
MAJ=$(echo "$VER" | cut -d. -f1); MIN=$(echo "$VER" | cut -d. -f2); PAT=$(echo "$VER" | cut -d. -f3)
CODE=$((MAJ * 10000 + MIN * 100 + PAT))
echo "版本 $VER（versionCode $CODE）"
python3 - "$MAAFW/app/build.gradle.kts" "$VER" "$CODE" <<'PYEOF'
import re, sys
path, ver, code = sys.argv[1], sys.argv[2], sys.argv[3]
text = open(path, encoding='utf-8').read()
block = f'        versionCode = {code}\n        versionName = "{ver}"\n'
if 'versionName = "' in text and 'MAAGUI_VERSION_BLOCK' not in text:
    text = re.sub(r'\n?\s*versionCode = \d+\n\s*versionName = "[^"]*"\n', '\n', text)
if 'MAAGUI_VERSION_BLOCK' not in text:
    text = text.replace('    defaultConfig {\n', '    defaultConfig {\n        // MAAGUI_VERSION_BLOCK（脚本写入，勿手改）\n' + block, 1)
open(path, 'w', encoding='utf-8').write(text)
print('已写入 versionName/versionCode')
PYEOF


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
