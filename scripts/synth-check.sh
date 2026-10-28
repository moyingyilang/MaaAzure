#!/usr/bin/env bash
# 跑 data/fixtures/*.json 里的合成帧用例。
#
# 需要带 maa feature 的二进制 + MaaFramework SDK，所以默认在 chroot 里跑：
#   ./scripts/synth-check.sh [chroot_root] [sdk_dir]
#
# 本机（Termux 本体）跑不了——没有 glibc 的 MaaFramework。
set -uo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
CHROOT="${1:-$HOME/.local/share/tmoe-linux/containers/chroot/debian-bookworm_arm64}"
SDK="${2:-$ROOT/MAA-linux-aarch64}"
BIN="${SYNTH_BIN:-/root/target/debug/maazure-daemon}"

# 从宿主看不到 chroot 的 /root（权限），所以让容器自己判断
if ! su -c "chroot '$CHROOT' /usr/bin/env -i PATH=/usr/bin:/bin HOME=/root /bin/bash -c 'test -x $BIN'" 2>/dev/null; then
    echo "✗ 找不到二进制 $BIN（先 cargo build -p maazure-daemon --features maa）"
    exit 2
fi

fail=0
total=0
for f in "$ROOT"/data/fixtures/*.json; do
    [ -f "$f" ] || continue
    name="$(basename "$f")"
    total=$((total + 1))
    echo "── $name ──"
    out=$(su -c "chroot '$CHROOT' /usr/bin/env -i PATH=/usr/bin:/bin HOME=/root \
        LD_LIBRARY_PATH='$SDK/bin' /bin/bash -c 'cd $ROOT && timeout 180 $BIN --synth --script $f 2>&1 | grep -vE \"XDG_RUNTIME|Vulkan|WebGPU|dawn|onnx|request_uuid|^ .\\[0m\"'" 2>&1)
    echo "$out" | grep -E "用例|点击|状态|断言" | sed 's/^/  /'
    if ! echo "$out" | grep -q "断言      通过"; then
        echo "  ✗ 失败"
        fail=$((fail + 1))
    fi
done

echo
if [ "$fail" = 0 ]; then
    echo "✓ 全部通过（$total 个用例）"
else
    echo "✗ $fail/$total 失败"
fi
[ "$fail" = 0 ]
