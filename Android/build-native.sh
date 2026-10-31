#!/system/bin/sh
# 用 Termux 的 clang 预编译 MaaFwApp 的 native 模块。
#
# 为什么不用 NDK/CMake：官方 NDK 只发 linux-x86_64 宿主，SDK 自带的
# cmake 3.22.1 也是 x86_64，在这台 arm64 设备上根本跑不起来。
# Termux 的 clang 原生 target 就是 aarch64-linux-android，直接编即可。
#
# 用法：./build-native.sh <MaaFwApp 路径> <输出 jniLibs 目录>

set -e
SRC="${1:?用法: build-native.sh <MaaFwApp> <jniLibs 目录>}"
OUT="${2:?用法: build-native.sh <MaaFwApp> <jniLibs 目录>}"
NATIVE="$SRC/app/src/main/native"
NDK_INC="${ANDROID_NDK_SYSROOT:-/data/data/com.termux/files/home/android-sdk/ndk/29.0.14206865/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/include}"
API="${APP_API:-34}"

mkdir -p "$OUT"

# 主桥接库：帧缓冲、截图、输入注入
clang++ --target=aarch64-linux-android$API -shared -fPIC -O2 -std=c++17 -DANDROID \
    -idirafter "$NDK_INC" -I"$NATIVE" \
    "$NATIVE/bridge.cpp" "$NATIVE/bridge_frame_buffer.cpp" "$NATIVE/bridge_preview.cpp" \
    "$NATIVE/bridge_capture.cpp" "$NATIVE/bridge_input.cpp" "$NATIVE/misc.cpp" \
    -o "$OUT/libbridge.so" -llog -landroid -ljnigraphics -lmediandk -lEGL -lGLESv2

# 两个「可执行但叫 .so」的辅助程序（AGP 要求 jniLibs 里只能是 lib*.so）
clang --target=aarch64-linux-android$API -O2 "$NATIVE/launcher.c" -o "$OUT/liblauncher.so" -llog
clang --target=aarch64-linux-android$API -O2 "$NATIVE/agent_exec.c" -o "$OUT/libagentexec.so"

ls -lh "$OUT"
