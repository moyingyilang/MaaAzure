#!/usr/bin/env bash
# 零 Python 门禁：运行时与资源目录里不允许出现 .py。
#
# 唯一例外是 tools/corpus/record_golden.py —— 它是一次性录制工具，
# 跑在**冻结的 AzurPilot** 上，不属于新线运行时（docs/5-engineering/5.2-testing.md）。
set -euo pipefail

bad=$(find crates resource data -name '*.py' -not -path '*/target/*' 2>/dev/null || true)

if [ -n "$bad" ]; then
    echo "✗ 发现 Python 文件（零 Python 约束）："
    echo "$bad"
    exit 1
fi

echo "✓ 零 Python：crates/ resource/ data/ 下没有 .py"
