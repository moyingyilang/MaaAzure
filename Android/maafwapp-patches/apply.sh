#!/system/bin/sh
# 给 MaaFwApp 打「本机构建必需」的补丁，并把 MaaFramework 的 .so 铺进 jniLibs。
#
#   ./Android/maafwapp-patches/apply.sh [MaaFwApp 路径]
#
# 为什么需要：这台设备是 **arm64 上的 Termux**，没有 x86_64 宿主的 NDK/CMake，
# AGP 默认还会从 Maven 下 x86_64 的 aapt2。上游仓库原样是构建不了的。
#
# 幂等：已应用的补丁会跳过（用 git apply --check --reverse 判断），
# 所以可以在每次出包前无脑调用。
#
# 环境变量：
#   MAAFW_APP             MaaFwApp 仓库路径（默认 ~/build/MaaFwApp）
#   MAA_ANDROID_SDK_DIR   MAA-android-aarch64 所在目录；默认依次找
#                         <仓库>/MAA-android-aarch64、~/sdk/MAA-android-aarch64
set -u

HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../.." && pwd)"
TARGET="${1:-${MAAFW_APP:-$HOME/build/MaaFwApp}}"

[ -d "$TARGET/.git" ] || { echo "不是 git 仓库：$TARGET" >&2; exit 1; }
echo "MaaFwApp : $TARGET"

# ── 1) 打补丁 ────────────────────────────────────────────────────────────
for p in "$HERE"/*.patch; do
    [ -f "$p" ] || continue
    name="$(basename "$p")"
    if (cd "$TARGET" && git apply --check --reverse "$p" 2>/dev/null); then
        echo "  已应用，跳过  $name"
        continue
    fi
    if (cd "$TARGET" && git apply --check "$p" 2>/dev/null); then
        (cd "$TARGET" && git apply "$p") && echo "  已应用        $name"
    else
        echo "  打不上（上下文已变）$name" >&2
        echo "  → 手工看一眼：cd $TARGET && git apply --check $p" >&2
        exit 1
    fi
done

# ── 2) gradle.properties.local（Gradle 不读它，仅作本机记录）───────────────
LOCAL_P="$TARGET/gradle.properties.local"
if [ ! -f "$LOCAL_P" ]; then
    cat > "$LOCAL_P" <<EOF
# 本机构建专用（不进 git）：AGP 默认从 Maven 下载 aapt2 二进制，
# 那份是 x86_64，这台 arm64 设备跑不了 → 指到 SDK 里的 ARM 版 aapt2。
android.aapt2FromMavenOverride=$HOME/android-sdk/build-tools/35.0.1/aapt2
EOF
    echo "  已写入        gradle.properties.local（不作数，真正生效的是 0002 补丁）"
fi

# ── 3) 把 MaaFramework 的 .so 铺进 jniLibs ────────────────────────────────
SDK_DIR="${MAA_ANDROID_SDK_DIR:-}"
if [ -z "$SDK_DIR" ]; then
    for c in "$ROOT/MAA-android-aarch64" "$HOME/sdk/MAA-android-aarch64"; do
        [ -d "$c" ] && { SDK_DIR="$c"; break; }
    done
fi
[ -n "$SDK_DIR" ] && [ -d "$SDK_DIR" ] || {
    echo "找不到 MAA-android-aarch64（设 MAA_ANDROID_SDK_DIR= 指定）" >&2
    exit 1
}
echo "MaaFW SDK : $SDK_DIR"

DST="$TARGET/app/src/main/jniLibs/arm64-v8a"
mkdir -p "$DST"
copied=0
for so in "$SDK_DIR"/bin/libMaa*.so; do
    [ -f "$so" ] || continue
    base="$(basename "$so")"
    if [ ! -f "$DST/$base" ] || [ "$so" -nt "$DST/$base" ]; then
        cp -f "$so" "$DST/$base"
        copied=$((copied+1))
    fi
done
echo "  jniLibs 同步 $copied 个 .so（共 $(ls "$DST"/libMaa*.so 2>/dev/null | wc -l) 个）"
