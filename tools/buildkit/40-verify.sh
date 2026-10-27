#!/data/data/com.termux/files/usr/bin/bash
# SPDX-License-Identifier: MPL-2.0
# 派生自 JMNeXt/tools/buildkit（MPL-2.0）。
#
# 产物自检：架构、动态依赖、能否起来。
# 「看着成功其实没出包」是这套工具要防的第一号事故：构建失败时旧产物还在原地，
# ls 一看有文件就以为成功。所以这里必须校验**新鲜度**（今天生成）。
set -eu
HERE="$(cd "$(dirname "$0")" && pwd)"
[ -f "$HERE/build.env" ] && . "$HERE/build.env" || { echo "缺 build.env"; exit 1; }
: "${PROJECT_DIR:?}" "${CHROOT_ROOT:?}" "${RUST_TARGET:?}" "${BIN_NAME:?}"

OUT="$PROJECT_DIR/target/$RUST_TARGET/release/$BIN_NAME"
[ -f "$OUT" ] || OUT="$PROJECT_DIR/target/$RUST_TARGET/debug/$BIN_NAME"
[ -f "$OUT" ] || { echo "找不到产物：$BIN_NAME（先跑 30-build.sh）"; exit 1; }

echo "== 产物 =="
ls -lh "$OUT"
echo "== 架构 =="
file "$OUT" 2>/dev/null || true
echo "== 新鲜度 =="
mtime=$(date -r "$OUT" +%Y-%m-%d)
today=$(date +%Y-%m-%d)
if [ "$mtime" = "$today" ]; then
  echo "  今天生成：$mtime ✓"
else
  echo "  ⚠ 产物不是今天生成的（$mtime），可能构建失败但旧文件还在"
fi

echo "== 动态依赖（容器内 ldd）=="
su -c "chroot '$CHROOT_ROOT' /usr/bin/env -i PATH=/usr/bin:/bin HOME=/root \
  /bin/bash -c 'ldd \"$OUT\" 2>/dev/null | head -20 || echo \"  （静态链接或非本机架构）\"'"

echo "== 冒烟 =="
su -c "chroot '$CHROOT_ROOT' /usr/bin/env -i PATH=/usr/bin:/bin HOME=/root \
  /bin/bash -c '\"$OUT\" --version 2>&1 | head -3 || echo \"  --version 未实现（P0 前正常）\"'"
