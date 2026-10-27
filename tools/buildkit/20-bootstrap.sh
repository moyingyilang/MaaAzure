#!/data/data/com.termux/files/usr/bin/bash
# SPDX-License-Identifier: MPL-2.0
# 派生自 JMNeXt/tools/buildkit（MPL-2.0）。
#
# 容器内一次性依赖安装：rustup + libclang（bindgen）+ 交叉编译工具链。
# 只需跑一次；重复跑是幂等的（apt 会说 already installed，rustup 会说 unchanged）。
set -eu
HERE="$(cd "$(dirname "$0")" && pwd)"
[ -f "$HERE/build.env" ] && . "$HERE/build.env" || { echo "缺 build.env"; exit 1; }
: "${PROJECT_DIR:?}" "${CHROOT_ROOT:?}"

su -c "chroot '$CHROOT_ROOT' /usr/bin/env -i \
  PATH=/root/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin \
  HOME=/root DEBIAN_FRONTEND=noninteractive /bin/bash -c '
  set -e
  echo \"== apt 依赖 ==\"
  apt-get update -qq
  apt-get install -y -qq build-essential pkg-config libssl-dev \
      clang libclang-dev llvm unzip curl git ca-certificates \
      mingw-w64 gcc-x86-64-linux-gnu

  echo \"== rustup ==\"
  if ! command -v cargo >/dev/null 2>&1; then
      curl --proto \"=https\" -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
  fi

  echo \"== rust targets ==\"
  rustup target add x86_64-unknown-linux-gnu x86_64-pc-windows-gnu 2>/dev/null || true

  echo \"== 结果 ==\"
  cargo --version
  rustc --version
  clang --version | head -1
  echo \"CI=true 时跳过交互\"
'"
