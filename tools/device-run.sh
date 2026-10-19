#!/bin/sh
# Start the emulator, install the APK, run the app, and print every reading the run produced.
#
# This is the durable form of the ad-hoc scripts the 4.8xx rounds used.  It differs from
# tools/try-android-emulator.sh in the three things those rounds needed on every iteration:
#
#   * `adb uninstall` before `adb install`: with a different signature `install -r` is silently
#     ignored, and the run then measures the *previous* library (status 4.823);
#   * the durable `probe.log` (status 4.826): written with `O_CREAT|O_APPEND` and closed on every
#     call, because logcat loses the tail when the process dies;
#   * the crash record: the `crash` log buffer, `/data/tombstones`, and `dumpsys activity
#     exit-info`, which is what names the signal when the app leaves no message of its own.
#
# `-no-snapshot-save` is deliberate: the AVD had grown a fresh ~700 MB snapshot per run, and the
# emulator refuses to start when the host volume has less room than the AVD needs (status 4.830).
#
# An AI-started run goes through the controlled executor (R38); this script starts QEMU in the
# background and needs that budget:
#
#   FEMACS_EMULATOR_BUDGET_OK=1 python3 ~/.assistant/skills/controlled-run/scripts/run.py \
#     --profile runtime --timeout 1800 -- bash tools/device-run.sh --apk /tmp/femacs-apk/emacs.apk
#
#   tools/device-run.sh [--avd NAME] [--apk FILE] [--memory MIB] [--tail N] [--plan] [--dump-only]
#
# `--dump-only` boots the emulator and reads what is already on the device (the probe.log of the
# previous run survives there) without installing or starting anything.  That is how a finished run
# can be analysed at length - e.g. the stack addresses `FA e0` prints, which move if a path leaks
# stack.
set -u
REPO=$(cd "$(dirname "$0")/.." && pwd)
SDK=${ANDROID_SDK_ROOT:-${ANDROID_HOME:-$HOME/Library/Android/sdk}}
ADB=$SDK/platform-tools/adb
EMU=$SDK/emulator/emulator
AVD=${AVD_NAME:-femacs-arm64-api36}
APK=${APK_PATH:-/tmp/femacs-apk/emacs.apk}
SO=${NATIVE_SO:-$REPO/build/port/aarch64-linux-android/libemacs.so}
MEM=${EMULATOR_MEMORY_MIB:-2048}
TAIL=${PROBE_TAIL:-120}
GPU=${EMULATOR_GPU:-swiftshader_indirect}
BOOT_WAIT=${BOOT_WAIT:-240}
SETTLE=${SETTLE:-25}
PLAN=0
DUMP_ONLY=0
while [ $# -gt 0 ]; do
  case "$1" in
    --avd) AVD=$2; shift 2 ;;
    --apk) APK=$2; shift 2 ;;
    --memory) MEM=$2; shift 2 ;;
    --tail) TAIL=$2; shift 2 ;;
    --plan) PLAN=1; shift ;;
    --dump-only) DUMP_ONLY=1; shift ;;
    *) echo "unknown argument: $1" >&2; exit 2 ;;
  esac
done

if [ "$PLAN" = 1 ]; then
  echo "plan (nothing is started):"
  echo "  1. $EMU -avd $AVD -no-window -no-audio -no-boot-anim -no-snapshot-save -memory $MEM -gpu $GPU"
  echo "  2. adb wait-for-device; poll sys.boot_completed (bounded at ${BOOT_WAIT}s)"
  echo "  3. adb uninstall org.femacs; adb install -r $APK"
  echo "  4. adb shell am start -W -n org.femacs/.EmacsActivity; wait ${SETTLE}s"
  echo "  5. adb shell tail -$TAIL /data/data/org.femacs/files/probe.log"
  echo "  6. adb logcat -b crash -d; adb shell ls -lt /data/tombstones; dumpsys activity exit-info"
  echo "  7. adb logcat -d | grep android_run_debug_thread | tail -20"
  echo "  8. adb logcat -d | grep 'femacs-bt|Fatal signal|debuggerd|Abort message' | tail -25"
  exit 0
fi

[ "$DUMP_ONLY" = 1 ] || [ -f "$APK" ] || { echo "no APK at $APK" >&2; exit 2; }

# Guard before spending a boot: an APK built from objects that were not compiled for this target
# loads nothing and only says so inside the app's first line ("dlopen failed: cannot locate symbol
# ..." - measured after `gate-fast` rebuilt the host objects, status 4.836).  The library inside the
# APK must carry the JNI entry points and the Android globals the shell touches.
if [ "$DUMP_ONLY" != 1 ] && [ -n "${CHECK_NATIVE:-1}" ]; then
  NDK_BIN=$(ls -d "$SDK"/ndk/*/toolchains/llvm/prebuilt/*/bin 2>/dev/null | tail -1)
  if [ -n "$NDK_BIN" ] && [ -f "$SO" ]; then
    missing=""
    for sym in android_init_gui Java_org_femacs_EmacsNative_initEmacs Fandroid_enumerate_fonts emacs_pipe; do
      "$NDK_BIN/llvm-nm" -D --defined-only "$SO" 2>/dev/null | grep -q " $sym\$" || missing="$missing $sym"
    done
    if [ -n "$missing" ]; then
      echo "libemacs.so lacks$missing - rebuild the Android units (tools/check-all-units.sh) and" >&2
      echo "re-run link-shared before packaging; this library would fail its first dlopen." >&2
      exit 2
    fi
  fi
fi
"$ADB" emu kill >/dev/null 2>&1
sleep 2
"$EMU" -avd "$AVD" -no-window -no-audio -no-boot-anim -no-snapshot-save \
       -memory "$MEM" -gpu "$GPU" > /tmp/device-run-emulator.log 2>&1 &
EMUPID=$!
trap '"$ADB" emu kill >/dev/null 2>&1; sleep 2; kill $EMUPID 2>/dev/null' EXIT

booted=0
i=0
while [ "$i" -lt "$BOOT_WAIT" ]; do
  if ! kill -0 $EMUPID 2>/dev/null; then
    echo "emulator exited during boot:"; tail -5 /tmp/device-run-emulator.log; exit 1
  fi
  if [ "$("$ADB" shell getprop sys.boot_completed 2>/dev/null | tr -d '\r')" = "1" ]; then
    booted=1; break
  fi
  i=$((i + 1)); sleep 1
done
[ "$booted" = 1 ] || { echo "no boot after ${BOOT_WAIT}s"; tail -5 /tmp/device-run-emulator.log; exit 1; }

"$ADB" root >/dev/null 2>&1
sleep 2
if [ "$DUMP_ONLY" = 1 ]; then
  echo "== probe.log size =="
  "$ADB" shell "wc -l /data/data/org.femacs/files/probe.log 2>/dev/null"
  echo "== stack addresses the FA e0 tag printed: first, last, minimum =="
  "$ADB" shell "awk '/^FA .* e0 /{print \$4}' /data/data/org.femacs/files/probe.log | head -1"
  "$ADB" shell "awk '/^FA .* e0 /{print \$4}' /data/data/org.femacs/files/probe.log | tail -1"
  "$ADB" shell "awk '/^FA .* e0 /{print \$4}' /data/data/org.femacs/files/probe.log | sort -n | head -1"
  "$ADB" shell "awk '/^FA .* e0 /{print \$4, \$5}' /data/data/org.femacs/files/probe.log | awk 'NR%500==1'"
  echo "== tag/thread summary =="
  "$ADB" shell "awk '{print \$1, \$2}' /data/data/org.femacs/files/probe.log | sort | uniq -c | sort -rn | head -8"
  exit 0
fi
"$ADB" uninstall org.femacs >/dev/null 2>&1
sleep 2
echo "== install =="
"$ADB" install -r "$APK" 2>&1 | tail -2
"$ADB" shell rm -f /data/data/org.femacs/files/probe.log
"$ADB" logcat -c 2>/dev/null
echo "== start =="
"$ADB" shell am start -W -n org.femacs/.EmacsActivity 2>&1 | tail -3
sleep "$SETTLE"
echo "== probe.log size =="
"$ADB" shell "wc -l /data/data/org.femacs/files/probe.log 2>/dev/null"
echo "== probe.log last $TAIL lines =="
"$ADB" shell "tail -$TAIL /data/data/org.femacs/files/probe.log"
# The device keeps the log only until the AVD is torn down, so keep a durable copy in the tree
# (R40: run evidence belongs to the project, not to a temporary directory) for the offline
# analysers (`tools/analyse-free-log.py`).  adb is root by now, which is what makes the app's
# private directory readable.
if "$ADB" pull /data/data/org.femacs/files/probe.log "$REPO/docs/baseline/probe-run.log" >/dev/null 2>&1; then
  echo "== probe.log pulled to docs/baseline/probe-run.log ($(wc -l < "$REPO/docs/baseline/probe-run.log") lines) =="
else
  echo "== probe.log not pulled =="
fi
echo "== writers of the last 4000 lines (thread id is the number after the tag) =="
"$ADB" shell "tail -4000 /data/data/org.femacs/files/probe.log | awk '{print \$2}' | sort | uniq -c | sort -rn | head -6"
echo "== tags and threads over the whole log =="
"$ADB" shell "awk '{print \$1, \$2}' /data/data/org.femacs/files/probe.log | sort | uniq -c | sort -rn | head -12"
echo "== where each thread stopped (last line number per tag) =="
"$ADB" shell "grep -n '^AF ' /data/data/org.femacs/files/probe.log | tail -2; grep -c '^AF ' /data/data/org.femacs/files/probe.log; wc -l < /data/data/org.femacs/files/probe.log"
echo "== crash buffer (head: the signal, the fault address and frame #00) =="
"$ADB" logcat -b crash -d 2>/dev/null | head -14
echo "== crash buffer (tail: the frames near the entry point) =="
"$ADB" logcat -b crash -d 2>/dev/null | tail -8
echo "== tombstones =="
"$ADB" shell ls -lt /data/tombstones 2>/dev/null | head -3
for t in $("$ADB" shell ls /data/tombstones 2>/dev/null | tr -d '\r' | grep -v '\.pb$' | tail -1); do
  echo "--- $t"
  "$ADB" shell "head -40 /data/tombstones/$t" 2>/dev/null
done
echo "== app log tail (the process's own lines) =="
# The handler prints the innermost frame first, so the useful end of its backtrace is the *head*
# of the matching lines; `tail -20` kept only the outermost five frames (status 4.845).
"$ADB" logcat -d 2>/dev/null | grep -a 'android_run_debug_thread' | head -40
# The app's own lines: the fatal handler's message, and anything a probe writes to fd 2.
"$ADB" logcat -d 2>/dev/null | grep -a 'org\.femacs\|^PIPE \|^CL \|assertion failed' | tail -25
echo "== the fatal-signal path (our own handler, debuggerd, malloc debug) =="
"$ADB" logcat -d 2>/dev/null | grep -a 'femacs-bt\|Fatal signal\|debuggerd\|libc  :\|Abort message\|malloc_debug' | tail -25
echo "== exit info =="
"$ADB" shell dumpsys activity exit-info 2>/dev/null | grep -A3 "org.femacs" | head -8
echo "== note =="
echo "This script prints everything it reads and keeps nothing itself: capture its stdout when the"
echo "run matters (docs/baseline/device-run-*.txt is that capture).  Only rebuildable intermediates"
echo "live in /tmp: the APK under /tmp/femacs-apk, the cargo target dir, and the emulator's own log."
