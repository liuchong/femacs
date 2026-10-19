#!/usr/bin/env bash
# Start the Android GUI activity with a Lisp expression, and report how it ended.
#
# Why (status 4.881): batch mode on Android now completes `loadup.el` and evaluates Lisp
# (`BATCH-OK`, docs/baseline/android-batch-1.txt), so every remaining failure is in the GUI
# start-up: `init_display` -> `android_term_init` -> first frame -> first redisplay.  This script
# turns that into a bisect tool: the activity accepts `--eval EXPR` through
# `EmacsActivity.EXTRA_STARTUP_ARGUMENTS`, so each run can stop at a chosen point and the exit
# reason says whether that point was reached.
#
#   tools/android-gui-eval.sh --eval '(progn (redisplay) (kill-emacs 0))' [--wait 20]
#   tools/android-gui-eval.sh --eval '(kill-emacs 0)'
#
# It prints: the process's exit reason, the last of our own log lines, and the crash buffer head.
# Nothing is stored outside the caller's capture (docs/baseline/ is where a run worth keeping goes).
set -u
REPO=$(cd "$(dirname "$0")/.." && pwd)
SDK=${ANDROID_SDK_ROOT:-${ANDROID_HOME:-$HOME/Library/Android/sdk}}
ADB=$SDK/platform-tools/adb
EMU=$SDK/emulator/emulator
AVD=${AVD_NAME:-femacs-arm64-api36}
APK=${APK_PATH:-/tmp/femacs-apk/emacs.apk}
WAIT=${WAIT:-20}
EXPR=""
while [ $# -gt 0 ]; do
  case "$1" in
    --eval) EXPR=$2; shift 2 ;;
    --wait) WAIT=$2; shift 2 ;;
    --apk) APK=$2; shift 2 ;;
    *) echo "unknown argument: $1" >&2; exit 2 ;;
  esac
done
[ -n "$EXPR" ] || { echo "usage: $0 --eval EXPR [--wait N]" >&2; exit 2; }

"$ADB" emu kill >/dev/null 2>&1; sleep 2
"$EMU" -avd "$AVD" -no-window -no-audio -no-boot-anim -no-snapshot-save \
       -memory 2048 -gpu swiftshader_indirect >/tmp/android-gui-eval-emu.log 2>&1 &
booted=0
for _ in $(seq 1 240); do
  if [ "$("$ADB" shell getprop sys.boot_completed 2>/dev/null | tr -d '\r')" = "1" ]; then booted=1; break; fi
  sleep 1
done
[ "$booted" = 1 ] || { echo "no boot"; tail -3 /tmp/android-gui-eval-emu.log; exit 1; }
"$ADB" root >/dev/null 2>&1; sleep 3
"$ADB" install -r "$APK" >/dev/null 2>&1
"$ADB" logcat -c
# `--esa` splits its value on commas, so the expression must not contain one; the whole
# `--eval,EXPR` pair is quoted for the *remote* shell so `am` receives it as one word and the
# space inside EXPR survives.
"$ADB" shell "am start -n org.femacs/.EmacsActivity --esa org.femacs.STARTUP_ARGUMENTS '--eval,$EXPR'" 2>&1 | tail -2
sleep "$WAIT"
echo "== exit info =="
"$ADB" shell dumpsys activity exit-info org.femacs 2>/dev/null | grep -a -m3 "reason=\|timestamp="
echo "== our last log lines =="
"$ADB" logcat -d 2>/dev/null | grep -a 'android_run_debug_thread' | tail -6 | cut -c1-160
echo "== crash buffer: the first frames =="
"$ADB" logcat -b crash -d 2>/dev/null | grep -a -m14 '#0[0-9] pc\|Abort message\|Fatal signal' | cut -c1-190
"$ADB" emu kill >/dev/null 2>&1
