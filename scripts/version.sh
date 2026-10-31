#!/system/bin/sh
# 统一版本号：未发布一律 0.x.y，正式版从 1.x.y 起。
#
#   ./scripts/version.sh           打印当前版本
#   ./scripts/version.sh bump      递增 patch（0.1.0 → 0.1.1）
#   ./scripts/version.sh minor     递增 minor 并把 patch 归零（0.1.3 → 0.2.0）
#   ./scripts/version.sh set 0.2.1 直接设定
#
# 三处一起改：Android/version.txt（唯一真源）、interface.json、MaaFwApp 的 APK 版本。
set -u
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/.." && pwd)"
FILE="$ROOT/Android/version.txt"
PY=python3

cur=$(cat "$FILE" 2>/dev/null || echo "0.1.0")

case "${1:-show}" in
    show) echo "$cur" ;;
    bump)
        new=$($PY - "$cur" <<'EOF'
import sys
a,b,c = sys.argv[1].split('.')
print(f'{a}.{b}.{int(c)+1}')
EOF
)
        echo "$new" > "$FILE"; echo "$cur → $new" ;;
    minor)
        new=$($PY - "$cur" <<'EOF'
import sys
a,b,c = sys.argv[1].split('.')
print(f'{a}.{int(b)+1}.0')
EOF
)
        echo "$new" > "$FILE"; echo "$cur → $new" ;;
    set)
        [ $# -ge 2 ] || { echo "用法: version.sh set 0.2.1"; exit 2; }
        echo "$2" > "$FILE"; echo "$cur → $2"; new="$2" ;;
    *) echo "用法: version.sh [show|bump|minor|set X.Y.Z]"; exit 2 ;;
esac

new=$(cat "$FILE")
# interface.json（资源的版本，用于设备判断是否重新解包）
$PY - "$ROOT/interface.json" "$new" <<'EOF'
import json, sys
p, v = sys.argv[1], sys.argv[2]
d = json.load(open(p, encoding='utf-8'))
d['version'] = v
json.dump(d, open(p, 'w', encoding='utf-8'), ensure_ascii=False, indent=2)
open(p, 'a', encoding='utf-8').write('\n')
EOF
echo "interface.json → $new"
