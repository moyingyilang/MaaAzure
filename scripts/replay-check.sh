#!/bin/bash
# 快速回放校验：用**不带 maa feature** 的 daemon 跑所有已录制的 fixture 语料。
#
#   ./scripts/replay-check.sh
#
# 为什么单独一个脚本：
#   --synth 要经 MaaFramework 引擎，每个没命中的 next 候选吃 20 秒默认超时，
#   而且等待期间是阻塞 sleep（CPU 空转），15 个用例要几分钟。
#   --replay 是纯 Rust 的逐帧比较，不需要 libMaaFramework，实测 **0.076 秒**。
#
# 分工：
#   改动期 → replay-check.sh（毫秒级，验证决策与路径没退化）
#   提交前 → synth-check.sh（分钟级，验证真实 Pipeline 语义）
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/.." && pwd)"
CHROOT="${CHROOT:-$HOME/.local/share/tmoe-linux/containers/chroot/debian-bookworm_arm64}"
BIN="${BIN:-/root/target-noMaa/debug/maazure-daemon}"

pass=0; fail=0
for dir in "$ROOT"/corpus/fixture-*; do
    [ -d "$dir" ] || continue
    name=$(basename "$dir")
    out=$(su -c "chroot '$CHROOT' /usr/bin/env -i PATH=/usr/bin:/bin HOME=/root /bin/bash -c 'cd $ROOT && timeout 20 $BIN --replay $dir 2>&1 | tail -6'" 2>&1)
    if echo "$out" | grep -q "结束状态  ok"; then
        echo "✓ $name"; pass=$((pass+1))
    else
        echo "✗ $name"; echo "$out" | sed 's/^/    /'; fail=$((fail+1))
    fi
done
echo "--- $pass 通过 / $fail 失败（毫秒级）---"
[ "$fail" -eq 0 ]
