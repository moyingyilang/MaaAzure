#!/bin/bash
# 把命令的**完整原始输出**（stdout+stderr，不 grep 不 tail）追加到
# /sdcard/build_maaazure/stream.log，同时在终端显示一份。
#
#   ./scripts/logrun.sh <命令...>
#   LOG=/sdcard/xxx.log ./scripts/logrun.sh ...
#
# 规则：日志里必须是原始全量输出，方便 tail -f 和对齐时间线；
# 终端里可以只看摘要，但日志不许截断。
set -uo pipefail
LOG="${LOG:-/sdcard/build_maaazure/stream.log}"
mkdir -p "$(dirname "$LOG")" 2>/dev/null || true

{
    echo
    echo "===== $(date '+%F %T') $* ====="
    # -i 保留交互式输出顺序；2>&1 合并 stderr
    "$@" 2>&1
    echo "===== $(date '+%F %T') 退出码 $? ====="
} 2>&1 | tee -a "$LOG"
