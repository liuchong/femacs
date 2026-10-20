#!/bin/bash
# Android GUI input probe - one script, six stages, no rebuild.
#
#   ./tools/android-input-probe.sh <stage> [...]
#
# Stages (each boots the emulator, installs the APK already in /tmp/femacs-apk,
# and writes its readings under docs/baseline/android-focus/):
#
#   focus    who holds window/input/IME focus, and the view tree
#   key      one numeric KEYCODE_A plus one `input text` commit, screenshot md5s
#   back     three BACK presses; EmacsWindow.onKeyDown swallows BACK, so the
#            activity surviving three presses is evidence the key reached the view
#   rotate   rotate to landscape and back; a re-laid-out landscape frame proves
#            redisplay + flip still work, which isolates input from rendering
#   tap      tap, then a key, then F10, comparing content-area screenshots
#   applog   every logcat line from the app process around two key presses
#   probes   with the temporary PROBE4953 build: did the key reach activity/view/neither
#   signal   deliver SIGIO by hand after a key, to separate 'never woken' from 'never queued'
#   insert-rotate  type into the buffer, then force a full redraw by rotating
#
# Why this exists: four separate readings disagreed (white screen, no frames,
# no key dispatch, no redisplay). Each stage answers one question and writes a
# PNG or a log, so a later run can re-check rather than re-argue.
#
# Disk: the emulator refuses to start below roughly 3 GiB free on the data
# volume, so check `df -h /System/Volumes/Data` before running a stage.
set -u

SDK=${ANDROID_SDK_ROOT:-$HOME/Library/Android/sdk}
ADB=$SDK/platform-tools/adb
EMU=$SDK/emulator/emulator
AVD=${FEMACS_AVD:-femacs-arm64-api36}
APK=${FEMACS_APK:-/tmp/femacs-apk/emacs.apk}
HERE=$(cd "$(dirname "$0")/.." && pwd)
OUT=$HERE/docs/baseline/android-focus
mkdir -p "$OUT"

stage=${1:-focus}

boot ()
{
  $ADB emu kill >/dev/null 2>&1; sleep 2
  $EMU -avd "$AVD" -no-window -no-audio -no-boot-anim -no-snapshot-save \
       -memory 2048 -gpu guest >"$OUT/emulator-$stage.log" 2>&1 &
  for _ in $(seq 1 300); do
    [ "$($ADB shell getprop sys.boot_completed 2>/dev/null | tr -d '\r')" = "1" ] && break
    sleep 1
  done
  $ADB root >/dev/null 2>&1; sleep 3
  $ADB install -r "$APK" >/dev/null 2>&1
}

launch ()
{
  $ADB shell am start -n org.femacs/.EmacsActivity >/dev/null 2>&1
  # Wait for the process to exist before trusting any later reading: a screenshot or a
  # logcat grep taken before the activity is up reads as "nothing happened".
  for _ in $(seq 1 30); do
    [ -n "$($ADB shell pidof org.femacs 2>/dev/null | tr -d '\r')" ] && break
    sleep 1
  done
  sleep 9
}

shot () { $ADB exec-out screencap -p > "$OUT/$1.png" 2>/dev/null; }

case $stage in
  focus)
    boot; launch
    echo "== pid =="; $ADB shell pidof org.femacs | tr -d '\r'
    echo "== window manager focus =="
    $ADB shell dumpsys window 2>/dev/null | grep -E "mCurrentFocus|mFocusedApp" | head -4
    echo "== input dispatcher focus and app window config =="
    $ADB shell dumpsys input 2>/dev/null | grep -E "FocusedWindows" -A2 | head -4
    $ADB shell dumpsys input 2>/dev/null | grep -E "org.femacs" | grep inputConfig | head -2
    echo "== view tree =="
    $ADB shell dumpsys activity top 2>/dev/null | grep -E "EmacsView|EmacsSurfacePresenter" | head -4
    echo "== IME served view =="
    $ADB shell dumpsys input_method 2>/dev/null | grep -E "mServedView|mNextServedView" | head -4
    ;;

  key)
    boot; launch
    echo "== harness check: HOME must change the screen =="
    shot key-home-before
    $ADB shell input keyevent KEYCODE_HOME >/dev/null 2>&1; sleep 3
    shot key-home-after
    md5 -q "$OUT/key-home-before.png" "$OUT/key-home-after.png"
    launch
    shot key-before
    $ADB shell input keyevent 29 >/dev/null 2>&1; sleep 2
    shot key-after-numeric
    $ADB shell input text "x" >/dev/null 2>&1; sleep 2
    shot key-after-text
    echo "== md5 before / numeric / text =="
    md5 -q "$OUT/key-before.png" "$OUT/key-after-numeric.png" "$OUT/key-after-text.png"
    ;;

  back)
    boot; launch
    echo "== pid before =="; $ADB shell pidof org.femacs | tr -d '\r'
    for _ in 1 2 3; do
      $ADB shell input keyevent KEYCODE_BACK >/dev/null 2>&1; sleep 2
    done
    echo "== pid after 3x BACK =="; $ADB shell pidof org.femacs | tr -d '\r'
    $ADB shell dumpsys window 2>/dev/null | grep -E "mCurrentFocus" | head -2
    ;;

  rotate)
    boot; launch
    $ADB shell settings put system accelerometer_rotation 0 >/dev/null 2>&1
    $ADB shell settings put system user_rotation 0 >/dev/null 2>&1; sleep 3
    shot rot-portrait
    sips -g pixelWidth -g pixelHeight "$OUT/rot-portrait.png" 2>/dev/null | tail -2
    $ADB shell settings put system user_rotation 1 >/dev/null 2>&1; sleep 6
    shot rot-landscape
    sips -g pixelWidth -g pixelHeight "$OUT/rot-landscape.png" 2>/dev/null | tail -2
    $ADB shell settings put system user_rotation 0 >/dev/null 2>&1; sleep 4
    shot rot-back
    echo "== md5 portrait / landscape / back =="
    md5 -q "$OUT/rot-portrait.png" "$OUT/rot-landscape.png" "$OUT/rot-back.png"
    ;;

  tap)
    boot; launch
    shot t0
    $ADB shell input tap 540 700 >/dev/null 2>&1; sleep 3
    shot t1-tap
    $ADB shell input keyevent KEYCODE_A >/dev/null 2>&1; sleep 3
    shot t2-key
    $ADB shell input keyevent KEYCODE_F10 >/dev/null 2>&1; sleep 4
    shot t3-f10
    echo "== md5 t0 / tap / key / f10 =="
    md5 -q "$OUT/t0.png" "$OUT/t1-tap.png" "$OUT/t2-key.png" "$OUT/t3-f10.png"
    ;;

  signal)
    # If a key is queued but the Lisp thread is never woken, delivering SIGIO by hand
    # should make it drain the queue and draw the character.  SigCgt first says whether
    # the process catches SIGIO at all (SIGIO's default action is to terminate).
    boot; launch
    PID=$($ADB shell pidof org.femacs | tr -d '\r')
    echo "== caught/blocked signal masks (bit 29 = SIGIO) =="
    $ADB shell cat /proc/$PID/status 2>/dev/null | grep -E "SigCgt|SigBlk|SigIgn" | head -4
    shot sig-before
    $ADB shell input keyevent KEYCODE_A >/dev/null 2>&1; sleep 2
    shot sig-after-key
    $ADB shell kill -29 $PID >/dev/null 2>&1; sleep 3
    shot sig-after-sigio
    echo "== md5 before / after key / after SIGIO =="
    md5 -q "$OUT/sig-before.png" "$OUT/sig-after-key.png" "$OUT/sig-after-sigio.png"
    echo "== pid after =="; $ADB shell pidof org.femacs | tr -d '\r'
    ;;

  insert-rotate)
    # Does a key actually reach the buffer?  Type into the buffer, then force a full
    # redraw by rotating.  Baseline and test both end in landscape at the same size, so
    # two byte-identical landscapes mean the text never changed, and any difference means
    # the key was inserted and only incremental presentation is at fault.
    boot; launch
    $ADB shell settings put system accelerometer_rotation 0 >/dev/null 2>&1
    $ADB shell settings put system user_rotation 1 >/dev/null 2>&1; sleep 6
    shot ir-landscape-baseline
    $ADB shell settings put system user_rotation 0 >/dev/null 2>&1; sleep 5
    for _ in 1 2 3 4 5 6 7 8; do
      $ADB shell input keyevent KEYCODE_A >/dev/null 2>&1
    done
    sleep 2
    $ADB shell settings put system user_rotation 1 >/dev/null 2>&1; sleep 6
    shot ir-landscape-after-typing
    echo "== md5 landscape baseline / after typing =="
    md5 -q "$OUT/ir-landscape-baseline.png" "$OUT/ir-landscape-after-typing.png"
    $ADB shell settings put system user_rotation 0 >/dev/null 2>&1
    ;;

  probes)
    # Requires an APK built with the temporary PROBE4953 lines in EmacsActivity and
    # EmacsView.  It answers the one question the screenshots cannot: did the key reach
    # the view, the activity, or neither?
    boot; launch
    $ADB logcat -c
    $ADB shell input keyevent KEYCODE_A >/dev/null 2>&1; sleep 2
    $ADB shell input tap 540 700 >/dev/null 2>&1; sleep 2
    echo "== probe lines =="
    $ADB logcat -d 2>/dev/null | grep -E "PROBE49" | head -20
    echo "== probe count =="
    $ADB logcat -d 2>/dev/null | grep -c "PROBE49"
    ;;

  applog)
    boot; launch
    PID=$($ADB shell pidof org.femacs | tr -d '\r')
    echo "== app pid: $PID =="
    $ADB logcat -c
    $ADB shell input keyevent KEYCODE_A >/dev/null 2>&1; sleep 2
    $ADB shell input keyevent KEYCODE_A >/dev/null 2>&1; sleep 2
    echo "== app log lines during/after the keys =="
    $ADB logcat -d --pid=$PID 2>/dev/null | tail -40
    echo "== exception lines =="
    $ADB logcat -d 2>/dev/null | grep -iE "UnsatisfiedLink|AndroidRuntime|Exception|FATAL" | head -20
    ;;

  *)
    echo "unknown stage: $stage" >&2; exit 2 ;;
esac

$ADB emu kill >/dev/null 2>&1
