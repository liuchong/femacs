#!/bin/sh
# Install the port's APK into an emulator and record what Android itself says.
#
# Two claims, deliberately kept apart:
#
#   * the package, manifest and dex are acceptable to Android   -> `adb install` succeeds,
#     `am start -W` reports Status: ok, and the process is started;
#   * the native side really runs                               -> the app's *own* log carries the
#     port's `setEmacsParams` line ("Initializing femacs ..."), the `EmacsThread` line that calls
#     `initEmacs`, and none of the ways that load can fail (a linker error, an
#     `UnsatisfiedLinkError`, a tombstone).
#
#   tools/try-android-emulator.sh [--avd NAME] [--apk FILE] [--memory MIB]
#   tools/try-android-emulator.sh --plan      # print every step and exit (nothing is started)
#
# This script starts the emulator in the background (`&`) because `adb` has to talk to it while it
# runs, so an AI-started run goes through the controlled executor (R38: the runtime profile starts
# at 4 GiB and grows toward a 16 GiB hard cap on its own, which holds QEMU's own footprint):
#
#   FEMACS_EMULATOR_BUDGET_OK=1 ANDROID_SDK_ROOT=$HOME/Library/Android/sdk \
#     python3 ~/.assistant/skills/controlled-run/scripts/run.py --profile runtime -- \
#     bash tools/try-android-emulator.sh --avd femacs-arm64-api36 --apk /tmp/femacs-apk/emacs.apk
#
# `FEMACS_EMULATOR_BUDGET_OK=1` still guards a bare shell: starting QEMU from an interactive
# command has to be a deliberate act, not a side effect of running a script.
#
# `--memory` is the *guest's* RAM, not the executor's allowance.
set -u
REPO=$(cd "$(dirname "$0")/.." && pwd)
SDK=${ANDROID_SDK_ROOT:-${ANDROID_HOME:-$HOME/Library/Android/sdk}}
# `femacs-arm64-api36` is the AVD the Android leg is measured on
# (`system-images;android-36;default;arm64-v8a`).  The old default, `Pixel_4_API_30`, points at a
# system image this machine no longer has; starting it fails inside the emulator and the script
# then waited for a device that was never going to appear (measured 4.704).
AVD=${AVD_NAME:-femacs-arm64-api36}
APK=${APK_PATH:-}
MEM=${EMULATOR_MEMORY_MIB:-2048}
# The runner samples the *sum* of the owned tree's RSS (run.py 251-256), so the emulator's own
# helper processes (a software renderer, crashpad) count towards the budget as well.  `swiftshader_
# indirect` keeps the run headless; rendering quality is a separate question.
GPU=${EMULATOR_GPU:-swiftshader_indirect}
PLAN=0
while [ $# -gt 0 ]; do
  case "$1" in
    --avd) AVD=$2; shift 2 ;;
    --apk) APK=$2; shift 2 ;;
    --memory) MEM=$2; shift 2 ;;
    --plan) PLAN=1; shift ;;
    *) echo "unknown argument: $1" >&2; exit 2 ;;
  esac
done
[ -n "$APK" ] || APK=/tmp/femacs-emulator/emacs.apk

if [ "$PLAN" = 1 ]; then
  echo "plan (nothing is started):"
  echo "  1. $SDK/emulator/emulator -avd $AVD -no-window -no-audio -no-boot-anim -no-snapshot -no-metrics -memory $MEM -gpu $GPU"
  echo "  2. adb wait-for-device (bounded: the emulator's own process must stay alive); poll sys.boot_completed"
  echo "  3. adb install -r $APK   (must carry lib/arm64-v8a/libemacs.so)"
  echo "  4. adb logcat -c; adb shell am start -W -n org.femacs/.EmacsActivity"
  echo "  5. app assertions: pid of org.femacs, the port's setEmacsParams line, the EmacsThread"
  echo "     line that calls initEmacs, native stderr lines, and no linker/tombstone failure"
  exit 0
fi
if [ "${FEMACS_EMULATOR_BUDGET_OK:-0}" != 1 ]; then
  echo "refusing to start the emulator from a bare shell: run it through the controlled executor" >&2
  echo "(R38 - the runtime profile holds QEMU's footprint) and set FEMACS_EMULATOR_BUDGET_OK=1." >&2
  echo "Use --plan to see the steps without starting anything." >&2
  exit 2
fi
if [ ! -f "$APK" ]; then
  mkdir -p "$(dirname "$APK")"
  python3 "$REPO/tools/check-android-apk.py" --keep "$(dirname "$APK")" >/dev/null || exit $?
fi
ADB="$SDK/platform-tools/adb"
EMU="$SDK/emulator/emulator"
[ -x "$ADB" ] && [ -x "$EMU" ] || { echo "missing adb or emulator under $SDK" >&2; exit 2; }

EMU_LOG=/tmp/femacs-emulator.log
LAUNCH_LOG=/tmp/femacs-launch.log
status=0
"$EMU" -avd "$AVD" -no-window -no-audio -no-boot-anim -no-snapshot -no-metrics -memory "$MEM" \
       -gpu "$GPU" > "$EMU_LOG" 2>&1 &
EMU_PID=$!
# The emulator spawns its own qemu child; killing only `$EMU_PID` leaves that one behind and the
# controlled executor reports `descendants_left`.  Shut it down through adb, then wait for the
# process to actually go away (measured: ~2 s), so the run ends with nothing of its own left.
trap '"$ADB" emu kill >/dev/null 2>&1; i=0; while kill -0 $EMU_PID 2>/dev/null && [ $i -lt 20 ]; do i=$((i+1)); sleep 1; done; kill $EMU_PID 2>/dev/null' EXIT

# A failing emulator (a missing system image, a bad AVD) exits within a second or two, and
# `adb wait-for-device` then blocks forever - there is no device to wait for.  Watch the child:
# wait for a device while the emulator is alive, and report its own log if it dies.
i=0
while ! "$ADB" get-state >/dev/null 2>&1; do
  if ! kill -0 $EMU_PID 2>/dev/null; then
    echo "emulator exited before a device appeared; $EMU_LOG tail:" >&2
    tail -5 "$EMU_LOG" >&2
    exit 3
  fi
  i=$((i+1))
  if [ $i -gt 60 ]; then
    echo "no device after 120 s; $EMU_LOG tail:" >&2
    tail -5 "$EMU_LOG" >&2
    exit 3
  fi
  sleep 2
done
i=0
while [ "$("$ADB" shell getprop sys.boot_completed 2>/dev/null | tr -d '\r')" != "1" ]; do
  i=$((i+1)); [ $i -gt 90 ] && { echo "emulator did not finish booting" >&2; exit 3; }
  sleep 2
done
echo "boot: completed (avd $AVD, guest ${MEM} MiB)"

"$ADB" install -r "$APK" 2>&1 | tail -2
echo "install: exit $?"

"$ADB" logcat -c 2>/dev/null
echo "== launch =="
"$ADB" shell am start -W -n org.femacs/.EmacsActivity 2>&1 | sed 's/^/  /' | head -6
sleep 6

echo "== app =="
PID=$("$ADB" shell pidof org.femacs 2>/dev/null | tr -d '\r')
if [ -n "$PID" ]; then
  echo "   process:  org.femacs pid $PID (still alive after the launch)"
else
  echo "   process:  org.femacs is NOT running" >&2
  status=1
fi
"$ADB" logcat -d 2>/dev/null > "$LAUNCH_LOG"

# The port's own native init.  `setEmacsParams` is the JNI entry point the shell calls first, so
# this line exists only if `libemacs.so` was loaded *and* the Rust entry point ran.
if grep -q "Initializing femacs" "$LAUNCH_LOG"; then
  echo "   native:   setEmacsParams line present (libemacs.so loaded and the Rust entry point ran)"
else
  echo "   native:   no setEmacsParams line - the native side did not run" >&2
  status=1
fi
if grep -q "EmacsThread.*libandroid-emacs.so" "$LAUNCH_LOG"; then
  echo "   native:   EmacsThread called initEmacs"
else
  echo "   native:   EmacsThread never reached initEmacs" >&2
  status=1
fi
# `android_run_debug_thread` is the port's own bridge of the native stdout/stderr to the system
# log, so a line under that tag is output produced by the Rust program itself.
NATIVE_OUT=$(grep -c "android_run_debug_thread" "$LAUNCH_LOG" 2>/dev/null)
echo "   native:   $NATIVE_OUT line(s) of native stdout/stderr through the debug thread"
if [ "$NATIVE_OUT" -gt 0 ]; then
  grep "android_run_debug_thread" "$LAUNCH_LOG" | sed 's/^/     | /' | head -5
fi

echo "== what Android says about the launch =="
grep -iE "org\.femacs|EmacsService|EmacsThread|EmacsActivity" "$LAUNCH_LOG" | head -8 | sed 's/^/  /'
echo "== failure modes (linker, JNI, tombstone) =="
if grep -iE "UnsatisfiedLink|dlopen failed|CANNOT LINK|Fatal signal|FATAL EXCEPTION|Unable to load native" "$LAUNCH_LOG" | head -5 | sed 's/^/  /' | grep .; then
  echo "   one of the above is a hard failure" >&2
  status=1
else
  echo "   none"
fi
echo "   full log: $LAUNCH_LOG"
exit "$status"
