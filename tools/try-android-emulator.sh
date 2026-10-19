#!/bin/sh
# Install the port's APK into an emulator and record what Android itself says.
#
# Two claims, deliberately kept apart:
#
#   * the package, manifest and dex are acceptable to Android   -> `adb install` succeeds,
#     `am start -W` reports Status: ok, and the process is started;
#   * the native side really runs                               -> the app's *own* log carries the
#     port's `setEmacsParams` line ("Initializing Emacs, where filesDir = ..."), the `EmacsThread` line that calls
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
DEVICE_LIBS=${DEVICE_LIBS:-/tmp/femacs-device-libs}
SO="$REPO/build/port/aarch64-linux-android/libemacs.so"
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
  echo "  5. app assertions: the pid over a 20 s window (alive at the end), the app's own log's"
  echo "     setEmacsParams line, the EmacsThread line that calls initEmacs, native stderr lines,"
  echo "     no linker/tombstone failure, and the device still booted afterwards"
  echo "  6. pull the device's own libraries (DT_NEEDED) into $DEVICE_LIBS and read the import"
  echo "     table against them: what the delivery device provides, not what API 21's stubs do"
  echo "  7. crash evidence: the crash buffer, `dumpsys activity exit-info`, and the newest tombstone"
  echo "     under /data/tombstones (read after `adb root`), so a native abort arrives with its reason"
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

# A tombstone is only evidence if the guest manages to write one: every tombstone this AVD left
# behind from an earlier run is 0 bytes (debuggerd could not finish the write), and an empty file
# then reads as "nothing crashed" while the crash buffer says otherwise.  Root the device, clear
# the directory so the newest file belongs to this run, and print the guest's free space, so a
# full /data is a reading instead of an inference.
"$ADB" root >/dev/null 2>&1
"$ADB" wait-for-device >/dev/null 2>&1
"$ADB" shell rm -f /data/tombstones/tombstone_* >/dev/null 2>&1
echo "guest /data: $("$ADB" shell df -h /data 2>/dev/null | tail -1 | tr -d '\r')"

# `install -r` is not enough: a *different* signing key leaves the old package in place, adb
# prints the failure and exits non-zero, and the launch below then runs the package that was
# already installed - a reading about a binary nobody built this session (measured 4.749: the run
# reported `install: exit 0` because the status came from `tail`, and the logcat it analysed was
# the previous session's library).  Uninstall first, check the exit status itself, and then assert
# that the library on the device is the one this tree built.
"$ADB" uninstall org.femacs >/dev/null 2>&1
# The status has to come from `adb install` itself: piping it into `tail` made `$?` the pipe's,
# so a failed install still read as success (status 4.778).
INSTALL_OUT=$("$ADB" install "$APK" 2>&1)
printf '%s\n' "$INSTALL_OUT" | tail -2
case "$INSTALL_OUT" in
  *Success*) : ;;
  *) echo "install: failed; nothing was launched" >&2; exit 3 ;;
esac
if [ -f "$SO" ]; then
  LOCAL_SO_MD5=$(md5 -q "$SO" 2>/dev/null || md5sum "$SO" 2>/dev/null | cut -d' ' -f1)
  # Ask the package manager where it put the package instead of guessing the `/data/app` layout:
  # the `~~hash==` form made the old glob read an empty md5 (status 4.778).
  DEVICE_APK=$("$ADB" shell pm path org.femacs | tr -d '\r' | sed -n 's/^package://p' | head -1)
  DEVICE_SO=$(printf '%s' "$DEVICE_APK" | sed 's#/base.apk$#/lib/arm64/libemacs.so#')
  DEVICE_SO_MD5=$("$ADB" shell "md5sum '$DEVICE_SO' 2>/dev/null" \
                    | tr -d '\r' | awk '{print $1}' | head -1)
  if [ -n "$LOCAL_SO_MD5" ] && [ "$LOCAL_SO_MD5" != "$DEVICE_SO_MD5" ]; then
    echo "install: the device's libemacs.so is not this build's ($DEVICE_SO_MD5 vs $LOCAL_SO_MD5)" >&2
    exit 3
  fi
  echo "install: ok, libemacs.so on the device matches this build ($LOCAL_SO_MD5)"
fi

"$ADB" logcat -c 2>/dev/null
echo "== launch =="
"$ADB" shell am start -W -n org.femacs/.EmacsActivity 2>&1 | sed 's/^/  /' | head -6

echo "== app =="
# One `pidof` a few seconds after `am start` is not an assertion about the native side: the
# activity starts `EmacsThread`, which calls `initEmacs` **in that thread**, so the process can be
# alive at 6 s and gone at 20 s, with the single snapshot deciding the verdict.  Sample it over a
# 20 s window and require it to be alive at the end, then read the app's *own* log instead of the
# whole buffer.
PID=""
DIED_AFTER=""
i=0
while [ $i -lt 10 ]; do
  P=$("$ADB" shell pidof org.femacs 2>/dev/null | tr -d '\r')
  if [ -n "$P" ]; then
    PID=$P
  elif [ -n "$PID" ]; then
    DIED_AFTER=$((i * 2))
    break
  fi
  i=$((i + 1))
  sleep 2
done
if [ -n "$DIED_AFTER" ]; then
  echo "   process:  org.femacs (pid $PID) died after ~${DIED_AFTER}s of the launch" >&2
  status=1
  PID=""
elif [ -n "$PID" ]; then
  echo "   process:  org.femacs pid $PID, alive for the whole 20 s window"
else
  echo "   process:  org.femacs is NOT running" >&2
  status=1
fi
# The app's own log, not every process's: the previous form read the whole buffer, so a line from
# another app could satisfy the greps below.
if [ -n "$PID" ]; then
  "$ADB" logcat -d --pid="$PID" 2>/dev/null > "$LAUNCH_LOG"
else
  "$ADB" logcat -d 2>/dev/null > "$LAUNCH_LOG"
fi
# A `FATAL EXCEPTION` in the activity takes the *device* back to a fresh boot in some images, and
# then every reading below is about a different run than the one that was launched.
if [ "$("$ADB" shell getprop sys.boot_completed 2>/dev/null | tr -d '\r')" != "1" ]; then
  echo "   device:   sys.boot_completed is no longer 1 - the device restarted during the launch" >&2
  status=1
else
  echo "   device:   still booted (sys.boot_completed=1)"
fi

# The port's own native init.  `setEmacsParams` is the JNI entry point the shell calls first, so
# this line exists only if `libemacs.so` was loaded *and* the Rust entry point ran.  The text read
# is the service's own line, quoted as data: "Initializing " + the package string + ", where
# filesDir = ..." (Kotlin's EmacsService.onCreate).
if grep -q "Initializing " "$LAUNCH_LOG"; then
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

echo "== what the device itself provides =="
# The import reading this project has been quoting is against the NDK's *stub* libraries for API
# 21, which is a proxy: bionic added `posix_spawn` at 28 and `backtrace` at 33, so a name can read
# as missing while the device that runs the APK exports it.  Pull the device's own libraries and
# read the same import table against them - that is the reading the emulator judgement needs.
rm -rf "$DEVICE_LIBS"; mkdir -p "$DEVICE_LIBS"
BIN_DIR=$(ls -d "$SDK"/ndk/*/toolchains/llvm/prebuilt/*/bin 2>/dev/null | tail -1)
if [ -n "$BIN_DIR" ] && [ -f "$SO" ]; then
  needed=$("$BIN_DIR/llvm-readelf" -d "$SO" 2>/dev/null | sed -n 's/.*Shared library: \[\(.*\)\]/\1/p' | sort -u)
  echo "  DT_NEEDED: $(echo $needed | tr '\n' ' ')"
  for name in $needed; do
    for dir in /apex/com.android.runtime/lib64/bionic /system/lib64 /system/lib64/hw; do
      if "$ADB" shell "[ -f $dir/$name ]" 2>/dev/null; then
        if "$ADB" pull "$dir/$name" "$DEVICE_LIBS/$name" >/dev/null 2>&1; then
          echo "  pulled $dir/$name"
          break
        fi
      fi
    done
  done
  if [ -n "$(ls -A "$DEVICE_LIBS" 2>/dev/null)" ]; then
    echo "  reading against the device's own libraries:"
    python3 "$REPO/tools/check-android-imports.py" --so "$SO" --providers-dir "$DEVICE_LIBS" 2>&1 \
      | sed 's/^/   /' | tail -25
  else
    echo "  nothing could be pulled from the device; no device reading was taken" >&2
  fi
else
  echo "  no NDK llvm-readelf or no $SO: no device reading was taken" >&2
fi
echo "== crash evidence =="
# The app's own log is a filtered view (`--pid` above), and a native abort is reported by a thread
# whose lines that filter drops: the whole buffer is saved as well, so the launch reads back as one
# timeline instead of two fragments.  A tombstone can only be read from a rooted device.
CRASH_LOG=/tmp/femacs-crash.log
FULL_LOG=/tmp/femacs-logcat-full.log
"$ADB" logcat -d -b crash 2>/dev/null > "$CRASH_LOG"
"$ADB" logcat -d 2>/dev/null > "$FULL_LOG"
echo "  full buffer: $FULL_LOG ($(wc -l < "$FULL_LOG" | tr -d ' ') lines); crash buffer: $CRASH_LOG"
if [ -s "$CRASH_LOG" ]; then
  echo "  crash buffer, last 20 lines:"
  tail -20 "$CRASH_LOG" | sed 's/^/     | /'
else
  echo "  crash buffer: empty"
fi
echo "  buffer lines naming the package (last 20):"
grep -a "org\.femacs" "$FULL_LOG" | tail -20 | sed 's/^/     | /'
echo "  every pid the package ran under in this buffer:"
grep -aoE "org\.femacs[^:]*: [0-9]+|[0-9]+ +[0-9]+ [A-Z] org\.femacs" "$FULL_LOG" | head -5 | sed 's/^/     | /'
echo "  exit-info:"
"$ADB" shell dumpsys activity exit-info org.femacs 2>&1 | head -12 | sed 's/^/     | /'
"$ADB" root >/dev/null 2>&1
"$ADB" wait-for-device >/dev/null 2>&1
# `ls -t` puts the newest tombstone first, and more than one is printed because a device that
# restarted the app mid-launch leaves the earlier one in place.  A tombstone is reported with its
# size: one written while the guest's disk was full is 0 bytes, and an empty read must not be
# mistaken for "nothing crashed".
echo "  tombstones:"
"$ADB" shell ls -l /data/tombstones 2>&1 | sed 's/^/     | /'
TS=$("$ADB" shell ls -t /data/tombstones 2>/dev/null | tr -d '\r' | grep -v '\.pb$' | head -2)
for t in $TS; do
  echo "  tombstone /data/tombstones/$t:"
  "$ADB" shell cat "/data/tombstones/$t" 2>/dev/null | head -70 | sed 's/^/     | /'
done
exit "$status"
