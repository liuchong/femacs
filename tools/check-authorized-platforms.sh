#!/bin/bash
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
#       -- bash tools/check-authorized-platforms.sh
#
# The emulator smoke is not here: it boots a device and takes minutes, so it stays a separate
# step (`tools/try-android-emulator.sh`, printed at the end with a copy-pasteable command).
# The runtime profile holds QEMU's own footprint by itself (R38: 4 GiB start, 16 GiB hard cap),
# so the old E1/E2/E3 budget question is closed (docs/decisions-needed.md).
set -uo pipefail
REPO=$(cd "$(dirname "$0")/.." && pwd)
APK_DIR=${FEMACS_APK_DIR:-$REPO/build/android/apk}
mkdir -p "$REPO/build/acceptance/platform-audit"
status=0

echo "== macOS: smoke =="
if bash "$REPO/tools/smoke.sh" 2>&1 | tail -1; then :; else status=1; fi

# The differential corpus (DoD-3's last line): every form runs in the port and in the reference
# build and the bytes must match.  `tools/smoke.sh` is the hand-picked 11; this is the growing
# corpus, so it runs here rather than only by hand (status 4.893).
echo "== macOS: differential =="
DIFF_LOG=$(mktemp "$REPO/build/acceptance/platform-audit/differential.XXXXXX")
if bash "$REPO/tools/differential.sh" > "$DIFF_LOG" 2>&1; then
  tail -1 "$DIFF_LOG"
else
  echo "   differential mismatches:"
  grep -a '^MISMATCH' "$DIFF_LOG" | head -5 | sed 's/^/   /'
  tail -1 "$DIFF_LOG"
  status=1
fi
rm -f "$DIFF_LOG"

echo "== Android: gate-android =="
if (cd "$REPO" && CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-$REPO/build/cargo} \
      cargo run -q -p xtask -- gate-android 2>&1 | tail -1); then :; else status=1; fi

echo "== Android: native library =="
(
  cd "$REPO" || exit 1
  FEMACS_CONFIG=android-arm64 CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-$REPO/build/cargo} \
    cargo run -q -p xtask -- gen-config >/dev/null || exit 1
  # The objects are a build product and are cleaned between rounds, so build them here rather
  # than depending on a sweep that ran earlier in the session.
  FEMACS_CONFIG=android-arm64 FEMACS_TARGET=aarch64-linux-android \
    CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-$REPO/build/cargo} \
    bash "$REPO/tools/check-all-units.sh" \
    "$REPO/docs/baseline/frozen-build-sources-android.txt" >/dev/null || exit 1
  # The gnulib `lib/` layer ships as `libgnu.a` in C, so it is in neither source list; the
  # Android configuration needs it for the same reason the frozen one does, and its `lib/` set is
  # not the darwin set (the Android headers do not declare `strmode`, the system has no
  # `memset_s`, and the bundled mini-gmp is what answers `mpz_*` there).  Without this sweep the
  # link leaves 59 mini-gmp imports unresolved on its own (status 4.708).
  for list in "$REPO/docs/baseline/lib-link-sources.txt" "$REPO/docs/baseline/lib-link-sources-android.txt"; do
    [ -f "$list" ] || continue
    FEMACS_CONFIG=android-arm64 FEMACS_TARGET=aarch64-linux-android \
      CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-$REPO/build/cargo} \
      bash "$REPO/tools/check-all-units.sh" "$list" >/dev/null || exit 1
  done
  FEMACS_CONFIG=android-arm64 FEMACS_TARGET=aarch64-linux-android \
    CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-$REPO/build/cargo} \
    cargo run -q -p xtask -- link-shared aarch64-linux-android 2>&1 | tail -1 || exit 1
  # `gen-config` above rewrote the tree's mirror (`src/include/config.rs`) with the Android
  # constants.  The frozen configuration is the one that belongs in git, so put it back here and
  # fail loudly if the tree does not return to its committed state: the first scripted run of this
  # file left the Android values behind, dirty, for the next session to find (measured 4.705).
  FEMACS_CONFIG=darwin-arm64 CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-$REPO/build/cargo} \
    cargo run -q -p xtask -- gen-config >/dev/null || exit 1
  git -C "$REPO" diff --quiet -- src/include/config.rs || {
    echo "   config.rs did not come back to the frozen configuration" >&2; exit 1; }
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

# The link's exit status is not evidence that the library can be *loaded*: `-shared` does not
# imply `-z defs`, so lld emits a library whose imports nothing provides, and every structural
# gate above (APK member, JNI exports, signature) passes - the device then refuses the first
# `dlopen` (P-300, status 4.707).  This gate reads the library's imports against the NDK's stub
# libraries and ratchets what is still missing.
if [ -f "$SO" ]; then
  if python3 "$REPO/tools/check-android-imports.py" --so "$SO" 2>&1 | sed 's/^/   /'; then :; else status=1; fi
fi

# Every record the Android build touches must have a layout assertion that *this* configuration
# compiles: the four device-only bugs of 4.788-4.831 were all a record mirrored from the host, and
# `sfnt_font_info`/`MonitorInfo` (4.832/4.833) showed the same class in records that had assertions
# - just not for the arm Android builds.  The baseline holds the records that are safe to leave
# unasserted (X11-only, other-platform, or C-defined), so a *new* one fails here.
if python3 "$REPO/tools/check-android-assertions.py" --check \
     --units frozen-build-sources-android.txt 2>&1 | sed 's/^/   /'; then :; else status=1; fi

# The assets tree is not optional any more: the Android port reads its Lisp from the APK (there is no
# dump), and `tools/gen-android-assets.py` reproduces C's `install_temp` recipe, including
# `assets/directory-tree` from the port's own `asset-directory-tool` (docs/android-app-readiness.md).
# The asset tree needs the port's own `asset-directory-tool`, a *host* program.  Like the object
# files, it is a build product the environment wipes between rounds, and without it the packaging
# step silently produces no directory-tree and this gate fails on a missing line rather than on a
# real defect (measured twice: status 4.963 and 4.980).  Build it here when it is absent.
if [ ! -x "$REPO/build/port/asset-directory-tool" ] && [ ! -x "$REPO/build/port/lib-src/asset-directory-tool" ]; then
  echo "   asset-directory-tool is missing: building it"
  (cd "$REPO" && FEMACS_CONFIG=darwin-arm64 FEMACS_TARGET=aarch64-apple-darwin \
     CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-$REPO/build/cargo} \
     cargo run -q -p xtask -- link-program asset-directory-tool >/dev/null 2>&1) || {
     echo "   could not build asset-directory-tool" >&2; status=1; }
fi

echo "== Android: signed APK with the native library and the assets tree =="
if bash "$REPO/tools/check-android-kotlin.sh" --apk "$APK_DIR" --native "$SO" --assets 2>&1 \
   | grep -E 'member: lib/arm64-v8a|assets: |^apk: ' ; then :; else status=1; fi
if [ -f "$APK_DIR/emacs.apk" ]; then
  BT=$(ls -d "$HOME"/Library/Android/sdk/build-tools/* 2>/dev/null | tail -1)
  if "$BT/apksigner" verify "$APK_DIR/emacs.apk" >/dev/null 2>&1; then
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
echo "== Android: what the application layer still needs =="
python3 "$REPO/tools/android-readiness.py" | sed 's/^/   /'

printf '  %-30s %s\n' "Android emulator smoke" \
  "FEMACS_EMULATOR_BUDGET_OK=1 sh tools/try-android-emulator.sh --avd femacs-arm64-api36 --apk $APK_DIR/emacs.apk"
printf '  %-30s %s\n' "  (through the executor)" \
  "python3 ~/.assistant/skills/controlled-run/scripts/run.py --profile runtime -- sh tools/try-android-emulator.sh --avd femacs-arm64-api36 --apk $APK_DIR/emacs.apk"
exit "$status"
