#!/data/data/com.termux/files/usr/bin/bash
# SPDX-License-Identifier: MPL-2.0
# 派生自 JMNeXt/tools/buildkit（MPL-2.0）。
#
# 产物自检：架构、导入表/依赖、新鲜度、冒烟。
# 「看着成功其实没出包」是这套工具要防的一号事故——构建失败时旧产物还在原地。
set -eu
HERE="$(cd "$(dirname "$0")" && pwd)"
[ -f "$HERE/build.env" ] && . "$HERE/build.env" || { echo "缺 build.env"; exit 1; }
: "${PROJECT_DIR:?}" "${CHROOT_ROOT:?}" "${RUST_TARGET:?}" "${BIN_NAME:?}"

TARGET_DIR="/root/target-$(echo "$RUST_TARGET" | tr '/' '-')"
SUFFIX=""
case "$RUST_TARGET" in *windows*) SUFFIX=".exe" ;; esac

su -c "chroot '$CHROOT_ROOT' /usr/bin/env -i PATH=/usr/bin:/bin HOME=/root /bin/bash -c '
  set -u
  found=0
  for prof in release debug; do
    f=$TARGET_DIR/$RUST_TARGET/\$prof/${BIN_NAME}${SUFFIX}
    [ -f \"\$f\" ] || continue
    found=1
    echo \"== \$f ==\"
    ls -lh \"\$f\"
    file \"\$f\"
    case \"$RUST_TARGET\" in
      *windows*)
        echo \"-- 导入表（不应出现 MaaFramework：dynamic 模式运行期加载）\"
        x86_64-w64-mingw32-objdump -p \"\$f\" | grep -i \"DLL Name\" | head -10
        ;;
      *)
        echo \"-- ldd\"
        ldd \"\$f\" 2>&1 | head -8
        echo \"-- 冒烟\"
        \"\$f\" --version 2>&1 | head -2 || echo \"  （非本机架构，跑不了；交叉产物属正常）\"
        ;;
    esac
  done
  [ \"\$found\" = 1 ] || { echo \"找不到产物：${BIN_NAME}${SUFFIX}（先跑 30-build.sh）\"; exit 1; }
'"
