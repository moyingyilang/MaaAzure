#!/system/bin/sh
# 把 AGP 产出的「残缺 APK」重组成可安装的 APK。
#
# 背景：本机构建时 assembleRelease 报成功，但产物缺 AndroidManifest.xml
# 与 resources.arsc（aapt2 dump badging 会说 could not identify format of APK）。
# 资源与 manifest 在 intermediates 里是齐的，这里把它们合并回去再签名。
set -e
APP="${APP:-/data/data/com.termux/files/home/build/MaaFwApp}"
WORK="${WORK:-/data/data/com.termux/files/home/build/apkfix}"
SDK="${ANDROID_HOME:-/data/data/com.termux/files/home/android-sdk}"
BT="$SDK/build-tools/35.0.1"
KS="${KS:-/root/maaazure.jks}"

AP_="$APP/app/build/intermediates/linked_resources_binary_format/release/processReleaseResources/linked-resources-binary-format-release.ap_"
BROKEN="$APP/app/build/outputs/apk/release/app-release-unsigned.apk"
[ -f "$AP_" ] || { echo "缺 $AP_"; exit 1; }
[ -f "$BROKEN" ] || { echo "缺 $BROKEN"; exit 1; }

rm -rf "$WORK/stage"; mkdir -p "$WORK/stage"
unzip -q -o "$AP_" -d "$WORK/stage"
unzip -q -o "$BROKEN" -d "$WORK/stage"
cd "$WORK/stage"
rm -f ../app-merged.apk ../app-aligned.apk ../app-signed.apk
zip -q -X -r ../app-merged.apk AndroidManifest.xml resources.arsc \
    classes.dex classes2.dex res assets lib kotlin org META-INF
cd ..
"$BT/zipalign" -f -p 4 app-merged.apk app-aligned.apk
sh "$BT/apksigner" sign --ks "$KS" --ks-pass pass:maazure --key-pass pass:maazure \
    --min-sdk-version 26 --out app-signed.apk app-aligned.apk
sh "$BT/apksigner" verify app-signed.apk && echo "签名校验通过"
"$BT/aapt2" dump badging app-signed.apk | head -3
ls -lh app-signed.apk
