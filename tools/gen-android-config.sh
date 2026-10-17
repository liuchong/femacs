#!/bin/sh
# Produce the Android target's frozen configuration from the reference's own configure.
#
# The platform rows need a second `docs/baseline/config/*.json` (P-203/P-204), and that file is
# what the reference's `configure` decides on an Android host.  This script runs exactly the
# command surface PORTING.md P-205 records and then turns the resulting `config.h` into the JSON
# with `tools/gen-config-json.py`:
#
#   tools/gen-android-config.sh --check-prereqs     # only report what is present
#   tools/gen-android-config.sh [--api 21] [--build-dir DIR]
#
# Nothing is downloaded: the SDK's build-tools must already provide aapt/apksigner/d8 (configure
# looks for them through SDK_BUILD_TOOLS) and an NDK must provide aarch64-linux-androidNN-clang.
set -u
REPO=$(cd "$(dirname "$0")/.." && pwd)
SDK=${ANDROID_HOME:-$HOME/Library/Android/sdk}
API=${ANDROID_API:-36}
NDK_API=${ANDROID_NDK_API:-21}
BUILD=${BUILD_DIR:-/tmp/femacs-android-config}
CHECK=0
while [ $# -gt 0 ]; do
  case "$1" in
    --check-prereqs) CHECK=1; shift ;;
    --api) API=$2; shift 2 ;;
    --build-dir) BUILD=$2; shift 2 ;;
    *) echo "unknown argument: $1" >&2; exit 2 ;;
  esac
done

ANDROID_JAR="$SDK/platforms/android-$API/android.jar"
BUILD_TOOLS=$(ls -d "$SDK"/build-tools/* 2>/dev/null | tail -1)
CC=$(ls "$SDK"/ndk/*/toolchains/llvm/prebuilt/*/bin/aarch64-linux-android"$NDK_API"-clang 2>/dev/null | tail -1)

missing=0
report() { printf '%-28s %s\n' "$1" "${2:-missing}"; [ -n "${2:-}" ] || missing=1; }
report android.jar "$([ -f "$ANDROID_JAR" ] && echo "$ANDROID_JAR")"
report build-tools "$BUILD_TOOLS"
for tool in aapt apksigner d8; do
  report "$tool" "$([ -x "$BUILD_TOOLS/$tool" ] && echo "$BUILD_TOOLS/$tool")"
done
report ndk-clang "$CC"
report configure "$([ -x "$REPO/emacs/configure" ] && echo "$REPO/emacs/configure")"

[ "$CHECK" = 1 ] && { [ "$missing" = 0 ] && echo "prerequisites: complete" || echo "prerequisites: incomplete"; exit "$missing"; }
[ "$missing" = 0 ] || { echo "prerequisites are incomplete; nothing was run" >&2; exit 2; }

rm -rf "$BUILD" && mkdir -p "$BUILD" && cd "$BUILD" || exit 1
SDK_BUILD_TOOLS="$BUILD_TOOLS" "$REPO/emacs/configure" \
  --host=aarch64-linux-android --with-android="$ANDROID_JAR" CC="$CC" \
  --without-all --with-x=no --with-ns=no > configure.log 2>&1
status=$?
echo "configure: exit $status (log: $BUILD/configure.log)"
[ "$status" = 0 ] || { tail -5 configure.log; exit "$status"; }
[ -f src/config.h ] || { echo "configure succeeded but src/config.h is missing" >&2; exit 3; }
python3 "$REPO/tools/gen-config-json.py" src/config.h \
  --target "android-arm64 (NDK ${CC##*/})" \
  --out "$REPO/docs/baseline/config/android-arm64.json"
