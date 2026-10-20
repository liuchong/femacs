#!/bin/bash
# Android GUI acceptance, one run, three readings:
#   1. rendering   - the frame shows the menu bar and buffer text
#   2. input       - typing inserts characters, and the mode line turns modified
#   3. liveness    - the process survives typing, a tap and a rotation
# Everything runs inside this script because a backgrounded emulator dies with the calling
# shell (measured).  Readings land in docs/baseline/android-accept/.
set -u
SDK=${ANDROID_SDK_ROOT:-$HOME/Library/Android/sdk}
PORT=${FEMACS_EMU_PORT:-5560}
SERIAL=emulator-$PORT
ADB="$SDK/platform-tools/adb -s $SERIAL"
EMU=$SDK/emulator/emulator
AVD=${FEMACS_AVD:-femacs-arm64-api36}
APK=${FEMACS_APK:-/tmp/femacs-apk/emacs.apk}
HERE=$(cd "$(dirname "$0")/.." && pwd)
OUT=$HERE/docs/baseline/android-accept
mkdir -p "$OUT"
shot () { $ADB exec-out screencap -p > "$OUT/$1.png" 2>/dev/null; }
alive () { [ -n "$($ADB shell pidof org.femacs 2>/dev/null | tr -d '\r')" ] && echo "alive" || echo "gone"; }

$ADB emu kill >/dev/null 2>&1; sleep 2
$EMU -avd "$AVD" -port "$PORT" -no-window -no-audio -no-boot-anim -no-snapshot-save \
     -memory 1536 -gpu guest >"$OUT/emulator.log" 2>&1 &
EMU_PID=$!
on_exit () { $ADB logcat -d -b crash -b main > "$OUT/logcat.txt" 2>/dev/null; $ADB emu kill >/dev/null 2>&1; }
trap on_exit EXIT   # keep the log for the next reader, and never leave the emulator running
for _ in $(seq 1 300); do
  [ "$($ADB shell getprop sys.boot_completed 2>/dev/null | tr -d '\r')" = "1" ] && break
  sleep 1
done
[ -f "$APK" ] || { echo "android-accept: missing APK: $APK" >&2; exit 2; }
$ADB uninstall org.femacs >/dev/null 2>&1   # the previous run may be signed with another key
$ADB root >/dev/null 2>&1; sleep 3
if ! $ADB install -r "$APK" >"$OUT/install.log" 2>&1; then
  echo "android-accept: install failed (see $OUT/install.log)" >&2; tail -3 "$OUT/install.log" >&2; exit 2
fi
$ADB logcat -c
$ADB shell am start -n org.femacs/.EmacsActivity >/dev/null 2>&1
for _ in $(seq 1 40); do
  [ -n "$($ADB shell pidof org.femacs 2>/dev/null | tr -d '\r')" ] && break
  sleep 1
done
sleep 14

echo "== 1. rendering =="
shot 1-baseline
echo "pid: $(alive)"

echo "== 2. input =="
for k in H E L L O; do
  $ADB shell input keyevent KEYCODE_$k >/dev/null 2>&1
  sleep 0.4
done
sleep 2
shot 2-typed
echo "pid: $(alive)"

echo "== 3. tap and rotate =="
$ADB shell input tap 540 900 >/dev/null 2>&1; sleep 2
shot 3-tapped
$ADB shell settings put system accelerometer_rotation 0 >/dev/null 2>&1
$ADB shell settings put system user_rotation 1 >/dev/null 2>&1; sleep 6
shot 4-landscape
$ADB shell settings put system user_rotation 0 >/dev/null 2>&1; sleep 5
shot 5-back-to-portrait
echo "pid: $(alive)"
$ADB logcat -d 2>/dev/null > "$OUT/logcat.txt"
echo "== md5: baseline / typed / tapped / landscape / portrait =="
md5 -q "$OUT/1-baseline.png" "$OUT/2-typed.png" "$OUT/3-tapped.png" \
       "$OUT/4-landscape.png" "$OUT/5-back-to-portrait.png"
echo "== fatal signals in the log =="
grep -c "Fatal signal" "$OUT/logcat.txt" || true
$ADB emu kill >/dev/null 2>&1; wait $EMU_PID 2>/dev/null
echo done
