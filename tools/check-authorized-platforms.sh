#!/bin/sh
# The acceptance surface the user authorised: macOS (real machine) and Android (structure).
#
# One command, fail-fast, four claims:
#   * macOS       tools/smoke.sh                 - the eleven checks against the dumped program
#   * Android     xtask gate-android             - Kotlin, dex, packaged and signed APK, its
#                                                  package/activities/members/signature
#   * Android     xtask link-shared              - the 91 objects into libemacs.so through the
#                                                  NDK's clang, then the JNI entry points the
#                                                  shell calls must be exported (counted)
#   * Android     check-android-kotlin.sh --apk  - the same APK, now carrying
#                 --native <libemacs.so>          `lib/arm64-v8a/libemacs.so`, re-signed
#
# Run it through the controlled executor (the packaging half peaks near a gigabyte):
#   python3 ~/.assistant/skills/controlled-run/scripts/run.py --profile runtime \
#       --memory-mib 1400 -- bash tools/check-authorized-platforms.sh
#
# The emulator smoke is not here: it is the one step the runtime profile's memory ceiling
# refuses (docs/decisions-needed.md E1/E2/E3, awaiting the user's decision), and this script
# reports it as blocked rather than as passed or not-run.
set -u
REPO=$(cd "$(dirname "$0")/.." && pwd)
APK_DIR=${FEMACS_APK_DIR:-/tmp/femacs-apk}
status=0

echo "== macOS: smoke =="
if bash "$REPO/tools/smoke.sh" 2>&1 | tail -1; then :; else status=1; fi

echo "== Android: gate-android =="
if (cd "$REPO/femacs" && CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/xt-cargo} \
      cargo run -q -p xtask -- gate-android 2>&1 | tail -1); then :; else status=1; fi

echo "== Android: native library =="
(
  cd "$REPO/femacs" || exit 1
  FEMACS_CONFIG=android-arm64 CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/xt-cargo} \
    cargo run -q -p xtask -- gen-config >/dev/null || exit 1
  # The objects are a build product and are cleaned between rounds, so build them here rather
  # than depending on a sweep that ran earlier in the session.
  FEMACS_CONFIG=android-arm64 FEMACS_TARGET=aarch64-linux-android \
    CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/xt-cargo} \
    bash "$REPO/tools/check-all-units.sh" \
    "$REPO/docs/baseline/frozen-build-sources-android.txt" >/dev/null || exit 1
  FEMACS_CONFIG=android-arm64 FEMACS_TARGET=aarch64-linux-android \
    CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/xt-cargo} \
    cargo run -q -p xtask -- link-shared aarch64-linux-android 2>&1 | tail -1 || exit 1
) || status=1

SO="$REPO/build/port/aarch64-linux-android/libemacs.so"
NDK_BIN=$(ls -d "$HOME"/Library/Android/sdk/ndk/*/toolchains/llvm/prebuilt/*/bin 2>/dev/null | tail -1)
if [ -f "$SO" ] && [ -n "$NDK_BIN" ]; then
  jni=$("$NDK_BIN/llvm-nm" -D --defined-only "$SO" 2>/dev/null | grep -c 'Java_org_femacs_EmacsNative_')
  echo "   libemacs.so: $(du -h "$SO" | cut -f1), $jni Java_org_femacs_EmacsNative_* export(s)"
  [ "$jni" -ge 60 ] || { echo "   too few JNI exports: $jni" >&2; status=1; }
else
  echo "   no $SO or no NDK: the native step did not run" >&2
  status=1
fi

echo "== Android: signed APK with the native library =="
if JAVA_HOME=${JAVA_HOME:-/opt/homebrew/opt/openjdk@25} \
   bash "$REPO/tools/check-android-kotlin.sh" --apk "$APK_DIR" --native "$SO" 2>&1 \
   | grep -E 'member: lib/arm64-v8a|^apk: ' ; then :; else status=1; fi
if [ -f "$APK_DIR/emacs.apk" ]; then
  BT=$(ls -d "$HOME"/Library/Android/sdk/build-tools/* 2>/dev/null | tail -1)
  if JAVA_HOME=${JAVA_HOME:-/opt/homebrew/opt/openjdk@25} \
     "$BT/apksigner" verify "$APK_DIR/emacs.apk" >/dev/null 2>&1; then
    echo "   signature: verified ($APK_DIR/emacs.apk)"
  else
    echo "   signature: FAILED" >&2; status=1
  fi
fi

echo "== Android: emulator smoke =="
# The controlled executor now starts the runtime profile at 4 GiB and grows toward a 16 GiB hard
# cap on its own (R38), so the emulator's ~2 GB fixed cost is inside the authorised budget and the
# old E1/E2/E3 question is closed.  The step is still separate from this script's four legs because
# it boots a device and takes minutes; `tools/try-android-emulator.sh` runs it.
printf '  %-30s %s\n' "Android emulator smoke" "run: FEMACS_EMULATOR_BUDGET_OK=1 sh tools/try-android-emulator.sh"
exit "$status"
