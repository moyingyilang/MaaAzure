#!/data/data/com.termux/files/usr/bin/bash
# SPDX-License-Identifier: MPL-2.0
# 派生自 JMNeXt/tools/buildkit（MPL-2.0）。
#
# 容器内交叉编译。用法：./30-build.sh [debug|release]
#
# 目标与 feature 的对应关系（实测）：
#   aarch64-unknown-linux-gnu  → --features maa        （MAA-linux-aarch64，静态）
#   x86_64-unknown-linux-gnu   → --features maa        （MAA-linux-x86_64，静态）
#   x86_64-pc-windows-gnu      → --features maa-dynamic（MAA-win-x86_64，运行期加载）
#
# Windows 必须 dynamic：MaaFramework 只提供 MSVC 的导入库，mingw 的 ld 读不了；
# dynamic 模式下 maa-framework-sys 不产生任何 rustc-link-lib，于是构建期不需要导入库。
set -eu
HERE="$(cd "$(dirname "$0")" && pwd)"
[ -f "$HERE/build.env" ] && . "$HERE/build.env" || { echo "缺 build.env"; exit 1; }
: "${PROJECT_DIR:?}" "${CHROOT_ROOT:?}" "${RUST_TARGET:?}" "${MAA_SDK_DIR:?}"

PROFILE="${1:-debug}"
case "$PROFILE" in
  debug)   FLAGS="" ;;
  release) FLAGS="--release" ;;
  *) echo "用法: $0 [debug|release]"; exit 1 ;;
esac

TARGET_DIR="/root/target-$(echo "$RUST_TARGET" | tr '/' '-')"

su -c "chroot '$CHROOT_ROOT' /usr/bin/env -i \
  PATH=/root/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin \
  HOME=/root CARGO_TARGET_DIR='$TARGET_DIR' MAA_SDK_PATH='$MAA_SDK_DIR' \
  /bin/bash -c '
  set -e
  cd \"$PROJECT_DIR\"
  echo \"== cargo build ($PROFILE, $RUST_TARGET, features=$CARGO_FEATURES) ==\"
  cargo build $FLAGS --target $RUST_TARGET -p ${BIN_NAME} --features ${CARGO_FEATURES:-}
  echo \"== 产物 ==\"
  ls -lh $TARGET_DIR/$RUST_TARGET/$PROFILE/${BIN_NAME}* 2>/dev/null || true
'"
