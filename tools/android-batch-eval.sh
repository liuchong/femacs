#!/usr/bin/env bash
# Run the port on the Android emulator in *batch* mode and print what it printed.
#
# Why (status 4.881): the GUI activity dies during start-up, but the very same library, the very
# same Lisp tree and the very same loadup complete when the port runs as `EmacsNoninteractive` - the
# first Android reading that reaches the end of `loadup.el` and evaluates Lisp (`BATCH-OK`,
# docs/baseline/android-batch-1.txt).  That makes this the deterministic channel for everything
# the GUI does not own: the Lisp startup, the regex engine, the file/asset layer.
#
#   tools/android-batch-eval.sh --elisp-file FILE   # a .el file pushed to the device and loaded
#   tools/android-batch-eval.sh --eval '(+ 1 2)'     # one expression, printed with %S
#
# `--batch -l FILE` is passed after a dummy program name, because `android_emacs_init` takes the
# first argument as `argv[0]` (measured: without it, `--batch` is consumed as the program name and
# the port goes interactive and exits with "standard input is not a tty").
set -u
SDK=${ANDROID_SDK_ROOT:-${ANDROID_HOME:-$HOME/Library/Android/sdk}}
ADB=$SDK/platform-tools/adb
EMU=$SDK/emulator/emulator
AVD=${AVD_NAME:-femacs-arm64-api36}
APK=${APK_PATH:-/tmp/femacs-apk/emacs.apk}
EL=""
EXPR=""
while [ $# -gt 0 ]; do
  case "$1" in
    --elisp-file) EL=$2; shift 2 ;;
    --eval) EXPR=$2; shift 2 ;;
    --apk) APK=$2; shift 2 ;;
    *) echo "unknown argument: $1" >&2; exit 2 ;;
  esac
done
[ -n "$EL" ] || [ -n "$EXPR" ] || { echo "usage: $0 (--elisp-file FILE | --eval EXPR)" >&2; exit 2; }

"$ADB" emu kill >/dev/null 2>&1; sleep 2
"$EMU" -avd "$AVD" -no-window -no-audio -no-boot-anim -no-snapshot-save \
       -memory 2048 -gpu swiftshader_indirect >/tmp/android-batch-eval-emu.log 2>&1 &
booted=0
for _ in $(seq 1 240); do
  if [ "$("$ADB" shell getprop sys.boot_completed 2>/dev/null | tr -d '\r')" = "1" ]; then booted=1; break; fi
  sleep 1
done
[ "$booted" = 1 ] || { echo "no boot"; tail -3 /tmp/android-batch-eval-emu.log; exit 1; }
"$ADB" root >/dev/null 2>&1; sleep 3
"$ADB" install -r "$APK" >/dev/null 2>&1
APK_PATH_ON_DEVICE=$("$ADB" shell "pm path org.femacs" | tr -d '\r' | sed 's#package:##')
LIBDIR=${APK_PATH_ON_DEVICE%/base.apk}/lib/arm64

if [ -n "$EXPR" ]; then
  printf '(%s (princ (format "%%S\\n" %s)) (kill-emacs 0))\n' 'progn' "$EXPR" > /tmp/android-batch-eval.el
else
  cp "$EL" /tmp/android-batch-eval.el
fi
"$ADB" push /tmp/android-batch-eval.el /data/local/tmp/eval.el >/dev/null 2>&1
"$ADB" shell "LD_LIBRARY_PATH=$LIBDIR CLASSPATH=$APK_PATH_ON_DEVICE app_process /system/bin \
  org.femacs.EmacsNoninteractive libandroid-emacs.so --batch -l /data/local/tmp/eval.el" 2>&1
"$ADB" emu kill >/dev/null 2>&1
