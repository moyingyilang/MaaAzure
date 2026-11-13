#!/system/bin/sh
# chroot-env.sh —— 统一解析「工具链宿主 chroot」路径（**被 source，不单独执行**）
#
# 背景：以前 4 个脚本各自写死一个路径，容器一改名就全线指向空目录，
# 而且失败是静默的（后续命令报别的错）。这里集中解析 + 存在性检查。
#
# 解析顺序：
#   1) 环境变量 $CHROOT（显式指定，最高优先级；指错立即报错退出）
#   2) 默认候选列表里第一个存在的目录
#   3) 都不存在 → 报错退出，并列出试过的路径
#
# 用法（脚本里）：
#   . "$HERE/chroot-env.sh"        # 或者用绝对路径 source
#   # 之后 $CHROOT 可用
#
# 可选：调用方先设 `CHROOT_OVERRIDE=<path>` 再 source，用于「位置参数覆盖」的场景。

CHROOT_DEFAULT_CANDIDATES="
$HOME/.local/share/tmoe-linux/containers/chroot/debian-sid_arm64
$HOME/.local/share/tmoe-linux/containers/chroot/debian-bookworm_arm64
$HOME/.local/share/tmoe-linux/containers/chroot/debian_arm64
"

_maazure_resolve_chroot() {
    # 1) 显式指定（环境变量或调用方的 override）
    _want="${CHROOT_OVERRIDE:-${CHROOT:-}}"
    if [ -n "$_want" ]; then
        if [ -d "$_want" ]; then
            printf '%s' "$_want"
            return 0
        fi
        echo "chroot 不存在：$_want（来自 CHROOT/CHROOT_OVERRIDE）" >&2
        return 1
    fi

    # 2) 默认候选
    _tried=""
    for _c in $CHROOT_DEFAULT_CANDIDATES; do
        [ -n "$_c" ] || continue
        if [ -d "$_c" ]; then
            printf '%s' "$_c"
            return 0
        fi
        _tried="$_tried
  $_c"
    done

    # 3) 全都不在
    echo "找不到可用的 chroot。试过：$_tried" >&2
    echo "请设置 CHROOT=<路径>，或用 ./scripts/mount-chroot.sh 先准备容器。" >&2
    return 1
}

if ! CHROOT="$(_maazure_resolve_chroot)"; then
    exit 1
fi
export CHROOT
