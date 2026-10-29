#!/usr/bin/env bash
# 跑 data/fixtures/*.json 里的合成帧用例，并顺带验证语料闭环。
#
# 需要带 maa feature 的二进制 + MaaFramework SDK，所以默认在 chroot 里跑：
#   ./scripts/synth-check.sh [chroot_root] [sdk_dir]
#
# 每个用例会：
#   1) --synth --script  跑合成帧并断言（点击次数 / 执行路径 / 结束状态）
#   2) --synth --script --dump  导出成 golden corpus 布局
#   3) --corpus-run      用导出的语料再跑一遍真实 Pipeline，断言与录制一致
# 第 2、3 步验证的是「语料格式 → 回放 → 断言」这条链路，真机语料到位后走同一条路。
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
    corpus="$ROOT/corpus/fixture-${name%.json}"
    out=$(su -c "chroot '$CHROOT' /usr/bin/env -i PATH=/usr/bin:/bin HOME=/root \
        LD_LIBRARY_PATH='$SDK/bin' /bin/bash -c 'cd $ROOT && rm -rf $corpus && timeout 180 $BIN --synth --script $f --dump $corpus 2>&1 | grep -vE \"XDG_RUNTIME|Vulkan|WebGPU|dawn|onnx|request_uuid|^ .\\[0m\"'" 2>&1)
    echo "$out" | grep -E "用例|点击|状态|断言|已导出" | sed 's/^/  /'
    if ! echo "$out" | grep -q "断言      通过"; then
        echo "  ✗ 合成帧断言失败"
        fail=$((fail + 1))
        continue
    fi

    # 语料闭环：用刚导出的语料再跑一遍真实 Pipeline
    entry=$(python3 -c "import json,sys;print(json.load(open('$f'))['entry'])" 2>/dev/null)
    out2=$(su -c "chroot '$CHROOT' /usr/bin/env -i PATH=/usr/bin:/bin HOME=/root \
        LD_LIBRARY_PATH='$SDK/bin' /bin/bash -c 'cd $ROOT && timeout 180 $BIN --corpus-run $corpus --entry $entry 2>&1 | grep -vE \"XDG_RUNTIME|Vulkan|WebGPU|dawn|onnx|request_uuid|^ .\\[0m\"'" 2>&1)
    echo "$out2" | grep -E "执行节点|断言" | sed 's/^/  /'
    if ! echo "$out2" | grep -q "断言      通过"; then
        echo "  ✗ 语料回放断言失败"
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
