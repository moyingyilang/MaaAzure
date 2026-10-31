#!/system/bin/sh
# 把构建需要的目录重新挂进 chroot（这些 bind mount 会莫名掉，比如 Termux 进程重启后）。
#
#   ./scripts/mount-chroot.sh [chroot 路径]
set -u
R="${1:-$HOME/.local/share/tmoe-linux/containers/chroot/debian-bookworm_arm64}"
[ -d "$R" ] || { echo "找不到 chroot: $R"; exit 1; }

mounted=0
for d in MaaAzure build android-sdk; do
    src="$HOME/$d"
    [ -d "$src" ] || continue
    dst="$R$src"
    su -c "mkdir -p '$dst'" 2>/dev/null
    if su -c "mountpoint -q '$dst'" 2>/dev/null; then
        :
    else
        su -c "mount -o bind '$src' '$dst'" 2>/dev/null && mounted=$((mounted+1))
    fi
done
# /proc /dev /sys 也是构建/跑测试需要的
for m in proc dev sys; do
    su -c "mountpoint -q '$R/$m' || mount -t ${m}fs $m '$R/$m'" 2>/dev/null || \
    su -c "mountpoint -q '$R/$m' || mount --bind /$m '$R/$m'" 2>/dev/null
done

echo "新挂载 $mounted 个目录"
su -c "chroot '$R' /usr/bin/env -i PATH=/usr/bin:/bin HOME=/root /bin/bash -c 'ls -d /data/data/com.termux/files/home/build/MaaFwApp /data/data/com.termux/files/home/android-sdk/platforms /data/data/com.termux/files/home/MaaAzure/Cargo.toml 2>&1'" 2>&1 | tail -4
