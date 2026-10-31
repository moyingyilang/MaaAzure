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
# 必须用 NDK 的 sysroot，不能用 Termux 的：
# Termux 的库叫 libEGL.so.1 / libGLESv2.so.2，app 进程里只有 libEGL.so / libGLESv2.so，
# 用 Termux sysroot 编出来的 .so 在设备上 dlopen 会报
#   library "libEGL.so.1" not found
NDK_ROOT="${ANDROID_NDK_ROOT:-/data/data/com.termux/files/home/android-sdk/ndk/29.0.14206865}"
NDK_SYSROOT="$NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64/sysroot"
API="${APP_API:-34}"

mkdir -p "$OUT"

# 主桥接库：帧缓冲、截图、输入注入
# 头文件与 libc++ 用 Termux 自带的（--sysroot 会让 libunwind 找不到），
# 但 -lEGL/-lGLESv2 这些**平台库**必须解析到 NDK 的 stub（SONAME 是 libEGL.so），
# 所以把 NDK 的库目录放在 -L 的最前面。
NDK_LIBS="$NDK_SYSROOT/usr/lib/aarch64-linux-android/$API"
clang++ --target=aarch64-linux-android$API -L"$NDK_LIBS" -idirafter "$NDK_SYSROOT/usr/include" \
    -shared -fPIC -O2 -std=c++17 -DANDROID -I"$NATIVE" \
    "$NATIVE/bridge.cpp" "$NATIVE/bridge_frame_buffer.cpp" "$NATIVE/bridge_preview.cpp" \
    "$NATIVE/bridge_capture.cpp" "$NATIVE/bridge_input.cpp" "$NATIVE/misc.cpp" \
    -o "$OUT/libbridge.so" -llog -landroid -ljnigraphics -lmediandk -lEGL -lGLESv2

# 两个「可执行但叫 .so」的辅助程序（AGP 要求 jniLibs 里只能是 lib*.so）
clang --target=aarch64-linux-android$API -L"$NDK_LIBS" -O2 "$NATIVE/launcher.c" -o "$OUT/liblauncher.so" -llog
clang --target=aarch64-linux-android$API -L"$NDK_LIBS" -O2 "$NATIVE/agent_exec.c" -o "$OUT/libagentexec.so"

ls -lh "$OUT"
