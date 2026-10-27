#!/data/data/com.termux/files/usr/bin/bash
# SPDX-License-Identifier: MPL-2.0
# 派生自 JMNeXt/tools/buildkit（MPL-2.0）。
#
# 容器内构建。用法：./30-build.sh [debug|release]
set -eu
HERE="$(cd "$(dirname "$0")" && pwd)"
[ -f "$HERE/build.env" ] && . "$HERE/build.env" || { echo "缺 build.env"; exit 1; }
: "${PROJECT_DIR:?}" "${CHROOT_ROOT:?}" "${RUST_TARGET:?}"

PROFILE="${1:-debug}"
case "$PROFILE" in
  debug)   FLAGS="" ;;
  release) FLAGS="--release" ;;
  *) echo "用法: $0 [debug|release]"; exit 1 ;;
esac

su -c "chroot '$CHROOT_ROOT' /usr/bin/env -i \
  PATH=/root/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin \
  HOME=/root CARGO_TARGET_DIR=/root/target-$RUST_TARGET \
  /bin/bash -c '
  set -e
  cd \"$PROJECT_DIR\"
  echo \"== cargo build ($PROFILE, $RUST_TARGET) ==\"
  cargo build $FLAGS --target $RUST_TARGET -p ${BIN_NAME:-maazure-daemon}
  echo \"== 完成 ==\"
'"
