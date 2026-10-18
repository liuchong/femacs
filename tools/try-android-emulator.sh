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
#   tools/try-android-emulator.sh [--avd NAME] [--apk FILE] [--memory MIB] [--keep]
#   tools/try-android-emulator.sh --plan      # print every step and exit (nothing is started)
#
# This script starts the emulator in the background (`&`) because `adb` has to talk to it while
# it runs.  That is a *human-facing* tool: an AI-started run must go through the controlled
# executor, and the executor's runtime ceiling cannot hold QEMU's own footprint (docs/
# decisions-needed.md E1/E2/E3).  The guard below therefore refuses to start anything unless
# `FEMACS_EMULATOR_BUDGET_OK=1` is set - which is the user's decision, not the agent's.
#
# `--memory` is the *guest's* RAM.  It exists because the controlled runner's runtime ceiling is
# 2 GiB: a 1 GiB guest plus QEMU's own overhead can cross that, and the rule is to lower the work
# rather than to raise the ceiling (status 4.610).
set -u
REPO=$(cd "$(dirname "$0")/.." && pwd)
SDK=${ANDROID_HOME:-$HOME/Library/Android/sdk}
AVD=${AVD_NAME:-Pixel_4_API_30}
APK=${APK_PATH:-}
MEM=${EMULATOR_MEMORY_MIB:-1024}
# The runner samples the *sum* of the owned tree's RSS (run.py 251-256), so the emulator's own
# helper processes (a software renderer, crashpad) count towards the budget as well.  `off` keeps
# the headless run to the guest's own footprint, which is what an install-and-launch check needs;
# rendering is a separate question and gets its own reading when the app is actually drawn.
GPU=${EMULATOR_GPU:-swiftshader_indirect}
KEEP=0
PLAN=0
while [ $# -gt 0 ]; do
  case "$1" in
    --avd) AVD=$2; shift 2 ;;
    --apk) APK=$2; shift 2 ;;
    --memory) MEM=$2; shift 2 ;;
    --keep) KEEP=1; shift ;;
    --plan) PLAN=1; shift ;;
    *) echo "unknown argument: $1" >&2; exit 2 ;;
  esac
done
[ -n "$APK" ] || APK=/tmp/femacs-emulator/emacs.apk
if [ ! -f "$APK" ]; then
  mkdir -p "$(dirname "$APK")"
  python3 "$REPO/tools/check-android-apk.py" --keep "$(dirname "$APK")" >/dev/null || exit $?
fi
if [ "$PLAN" = 1 ]; then
  echo "plan (nothing is started):"
  echo "  1. emulator -avd $AVD -no-window -no-audio -no-boot-anim -no-snapshot -no-metrics -memory $MEM -gpu $GPU"
  echo "  2. adb wait-for-device; poll sys.boot_completed (60 x 2 s)"
  echo "  3. adb install -r $APK"
  echo "  4. adb logcat -c; adb shell am start -n org.femacs/.EmacsActivity; sleep 4"
  echo "  5. adb logcat -d | grep -iE 'org\\.femacs|UnsatisfiedLink|dlopen|FATAL' | head -6"
  echo "  blocked on: the runtime ceiling (docs/decisions-needed.md E1/E2/E3); the guest's own"
  echo "  memory is $MEM MiB and QEMU's fixed overhead is what does not fit."
  exit 0
fi
if [ "${FEMACS_EMULATOR_BUDGET_OK:-0}" != 1 ]; then
  echo "refusing to start the emulator: the runtime ceiling cannot hold it (E1/E2/E3 in" >&2
  echo "docs/decisions-needed.md).  Re-run with FEMACS_EMULATOR_BUDGET_OK=1 once the budget" >&2
  echo "decision exists, or use --plan to see the steps without starting anything." >&2
  exit 2
fi
ADB="$SDK/platform-tools/adb"
EMU="$SDK/emulator/emulator"
[ -x "$ADB" ] && [ -x "$EMU" ] || { echo "missing adb or emulator under $SDK" >&2; exit 2; }

"$EMU" -avd "$AVD" -no-window -no-audio -no-boot-anim -no-snapshot -no-metrics -memory "$MEM" \
       -gpu "$GPU" > /tmp/femacs-emulator.log 2>&1 &
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
