#!/bin/sh
# Install the port's APK into an emulator and record what Android itself says.
#
# The port's Kotlin shell builds and signs (tools/check-android-apk.py), but it carries no native
# libraries yet (P-207: the reference's configure needs a JDK <= 19).  Installing and launching
# it on an emulator therefore separates two claims that are easy to conflate:
#
#   * the package, manifest and dex are acceptable to Android  -> `adb install` succeeds;
#   * the program cannot run yet                               -> the launch fails on the missing
#     library, and the log names it.
#
#   tools/try-android-emulator.sh [--avd NAME] [--apk FILE] [--keep]
set -u
REPO=$(cd "$(dirname "$0")/.." && pwd)
SDK=${ANDROID_HOME:-$HOME/Library/Android/sdk}
AVD=${AVD_NAME:-Pixel_4_API_30}
APK=${APK_PATH:-}
KEEP=0
while [ $# -gt 0 ]; do
  case "$1" in
    --avd) AVD=$2; shift 2 ;;
    --apk) APK=$2; shift 2 ;;
    --keep) KEEP=1; shift ;;
    *) echo "unknown argument: $1" >&2; exit 2 ;;
  esac
done
[ -n "$APK" ] || APK=/tmp/femacs-emulator/emacs.apk
if [ ! -f "$APK" ]; then
  mkdir -p "$(dirname "$APK")"
  python3 "$REPO/tools/check-android-apk.py" --keep "$(dirname "$APK")" >/dev/null || exit $?
fi
ADB="$SDK/platform-tools/adb"
EMU="$SDK/emulator/emulator"
[ -x "$ADB" ] && [ -x "$EMU" ] || { echo "missing adb or emulator under $SDK" >&2; exit 2; }

"$EMU" -avd "$AVD" -no-window -no-audio -no-boot-anim -no-snapshot -memory 1024 \
       -gpu swiftshader_indirect > /tmp/femacs-emulator.log 2>&1 &
EMU_PID=$!
trap 'kill $EMU_PID 2>/dev/null; "$ADB" emu kill >/dev/null 2>&1' EXIT

"$ADB" wait-for-device || exit 3
i=0
while [ "$("$ADB" shell getprop sys.boot_completed 2>/dev/null | tr -d '\r')" != "1" ]; do
  i=$((i+1)); [ $i -gt 60 ] && { echo "emulator did not finish booting" >&2; exit 3; }
  sleep 2
done
echo "boot: completed (avd $AVD)"

"$ADB" install -r "$APK" 2>&1 | tail -2
echo "install: exit $?"

"$ADB" logcat -c
"$ADB" shell am start -n org.femacs/.EmacsActivity >/dev/null 2>&1
sleep 4
echo "== launch log (first matching lines) =="
"$ADB" logcat -d 2>/dev/null | grep -iE "org\.femacs|UnsatisfiedLink|dlopen|FATAL" | head -6
[ "$KEEP" = 1 ] || true
