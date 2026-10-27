#!/data/data/com.termux/files/usr/bin/bash
# SPDX-License-Identifier: MPL-2.0
# 派生自 JMNeXt/tools/buildkit（MPL-2.0）。
#
# 组装发布目录并打 zip（布局见 docs/5-engineering/5.3-release.md）。
#
# 分工：只有「从 chroot 的 /root 里取二进制」这一步需要 root，
# 其余装配与压缩都在宿主机用户身份下做——否则整个发布目录会变成 root 所有，
# 用户连删都删不掉。
#
# 用法：./50-package.sh <version>
set -eu
HERE="$(cd "$(dirname "$0")" && pwd)"
[ -f "$HERE/build.env" ] && . "$HERE/build.env" || { echo "缺 build.env"; exit 1; }
: "${PROJECT_DIR:?}" "${CHROOT_ROOT:?}" "${RUST_TARGET:?}" "${MAA_SDK_DIR:?}"

VERSION="${1:?用法: $0 <version>}"
NAME="MaaAzure-v${VERSION}-$(echo "$RUST_TARGET" | sed 's/-unknown//; s/-pc//')"
OUT="$PROJECT_DIR/dist/$NAME"
STAGE="$PROJECT_DIR/dist/.stage"
TARGET_DIR="/root/target-$(echo "$RUST_TARGET" | tr '/' '-')"
UID_N="$(id -u)"
GID_N="$(id -g)"

rm -rf "$OUT" "$STAGE"
mkdir -p "$OUT" "$STAGE"

# 1) 从 chroot 的 /root 取二进制（需要 root），取完把属主改回宿主用户
su -c "chroot '$CHROOT_ROOT' /usr/bin/env -i PATH=/usr/bin:/bin HOME=/root /bin/bash -c '
  set -e
  src=\"$TARGET_DIR/$RUST_TARGET/release/${BIN_NAME}.exe\"
  [ -f \"\$src\" ] || src=\"$TARGET_DIR/$RUST_TARGET/release/${BIN_NAME}\"
  cp \"\$src\" \"$STAGE/\"
  chown -R $UID_N:$GID_N \"$STAGE\"
'"
cp "$STAGE"/* "$OUT/"
echo "二进制: $(ls "$OUT" | grep -E '\.exe$|^maazure-daemon$')"

# 2) MaaFramework 动态库：独立文件，允许用户自行替换（LGPL 合规）
case "$RUST_TARGET" in
  *windows*) cp "$MAA_SDK_DIR"/bin/*.dll "$OUT/" ;;
  *)         cp "$MAA_SDK_DIR"/bin/*.so "$OUT/" ;;
esac
echo "动态库: $(ls "$OUT" | grep -cE '\.dll$|\.so$') 个"

# 3) 资源与数据（宿主机身份，权限干净）
cp -r "$PROJECT_DIR/resource" "$OUT/resource"
cp -r "$PROJECT_DIR/data" "$OUT/data"
cp "$PROJECT_DIR/interface.json" "$PROJECT_DIR/LICENSE" "$PROJECT_DIR/NOTICE" "$OUT/"

# 4) zip（宿主没有 zip，借容器里的）
su -c "chroot '$CHROOT_ROOT' /usr/bin/env -i PATH=/usr/bin:/bin HOME=/root /bin/bash -c '
  cd \"$PROJECT_DIR/dist\" && zip -qr \"$NAME.zip\" \"$NAME\" && chown $UID_N:$GID_N \"$NAME.zip\"
'" || echo "提示：zip 失败不影响发布目录本身"

rm -rf "$STAGE"
echo "== 发布目录 =="
du -sh "$OUT"
du -sh "$OUT/resource" "$OUT/data" 2>/dev/null
ls -lh "$PROJECT_DIR/dist"/*.zip 2>/dev/null || true
