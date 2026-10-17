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
# configure compiles a Java program to prove the toolchain works, and /usr/bin/javac on a Mac
# without a JDK is a stub that fails there ("the Java compiler does not work" - measured).  The
# JDK Android Studio ships is the one the rest of the Android leg already uses.
STUDIO=${STUDIO:-/Applications/Android Studio.app/Contents}
# The rule for this project is "the latest LTS JDK only, never an old JDK to work around a
# check".  A Homebrew `openjdk@25` is the newest LTS when it is installed; Android Studio's
# bundled JBR is the fallback (an LTS too, just older).  Both were measured to build the
# identical signed APK, so this only picks the default - `JAVA_HOME` still overrides it.
for cand in /opt/homebrew/opt/openjdk@25 /usr/local/opt/openjdk@25; do
  if [ -x "$cand/bin/javac" ]; then NEWEST_LTS_JDK=$cand; break; fi
done
# Resolution order: an explicit `FEMACS_JAVA_HOME`, then the newest LTS JDK, then an ambient
# `JAVA_HOME`, then Studio's JBR.  The project rule is "the latest LTS JDK only", so an ambient
# older `JAVA_HOME` must not silently override it - it is a fallback, not a preference.
if [ -n "${FEMACS_JAVA_HOME:-}" ]; then JDK_HOME=$FEMACS_JAVA_HOME
elif [ -n "${NEWEST_LTS_JDK:-}" ]; then JDK_HOME=$NEWEST_LTS_JDK
elif [ -n "${JAVA_HOME:-}" ]; then JDK_HOME=$JAVA_HOME
else JDK_HOME=$STUDIO/jbr/Contents/Home; fi
JDK_BIN="$JDK_HOME/bin"
JAVAC=${JAVAC:-$JDK_BIN/javac}
JARSIGNER=${JARSIGNER:-$JDK_BIN/jarsigner}

missing=0
report() { printf '%-28s %s\n' "$1" "${2:-missing}"; [ -n "${2:-}" ] || missing=1; }
report android.jar "$([ -f "$ANDROID_JAR" ] && echo "$ANDROID_JAR")"
report build-tools "$BUILD_TOOLS"
for tool in aapt apksigner d8; do
  report "$tool" "$([ -x "$BUILD_TOOLS/$tool" ] && echo "$BUILD_TOOLS/$tool")"
done
report ndk-clang "$CC"
report javac "$([ -x "$JAVAC" ] && echo "$JAVAC")"
# configure also requires makeinfo (or a pre-built info/emacs); the reference build hit the same
# wall (build/emacs-ref/config.log) and the acceptance record works around it with an empty
# $srcdir/info/emacs.  Neither is this script's call to make: it reports what is missing.
MAKEINFO_BIN=$(command -v makeinfo 2>/dev/null)
report makeinfo "${MAKEINFO_BIN:-}"
report jarsigner "$([ -x "$JARSIGNER" ] && echo "$JARSIGNER")"
report configure "$([ -x "$REPO/emacs/configure" ] && echo "$REPO/emacs/configure")"

[ "$CHECK" = 1 ] && { [ "$missing" = 0 ] && echo "prerequisites: complete" || echo "prerequisites: incomplete"; exit "$missing"; }
[ "$missing" = 0 ] || { echo "prerequisites are incomplete; nothing was run" >&2; exit 2; }

rm -rf "$BUILD" && mkdir -p "$BUILD" && cd "$BUILD" || exit 1
# `AC_PATH_PROGS` looks `javac` up on PATH and prefers that over an env override (measured:
# /usr/bin/javac kept winning), so the JDK's bin goes first.
#
# `MAKEINFO` is passed as a plain assignment: an assignment that comes out of an *expansion*
# (`${X:+MAKEINFO="$Y"}`) is not an assignment to the shell, it is the command word - which is why
# this only broke the day `makeinfo` was installed, with
# `MAKEINFO=/opt/homebrew/bin/makeinfo: No such file or directory` (status 4.610).  The
# prerequisite check above already refuses to run without `makeinfo`, so the conditional was never
# needed.
# The command surface below is what P-207/P-213 settled by running configure:
#   * ANDROID_SDK is the *numeric* API level (configure.ac 48 compares it with 21);
#   * ANDROID_CC is the NDK's cross compiler (configure.ac 893/1077);
#   * two checks compile Java with `-source 1.7`, which no JDK >= 20 accepts, so their cache
#     variables are preset instead of installing an old JDK (the user's rule: LTS only);
#   * the bundled JDK goes first on PATH because AC_PATH_PROGS prefers it.
#
# The flags are exactly `java/INSTALL` 35-37's documented command plus `--host` and the two cache
# variables.  `--without-all --with-x=no --with-ns=no` used to be here and are gone: they make
# `with_toolkit_scroll_bars=no`, and configure.ac 4332-4357 then errors with "Non-toolkit scroll
# bars are not implemented for your system" because `--with-android` sets `window_system=android`
# (configure.ac 2595).  Those flags were never exercised past the `makeinfo` wall (status 4.610).
# `emacs_cv_tputs_lib='none required'` is the third preset cache variable, and it is the same
# *kind* of decision as the two Java ones: the Android pass itself answers "tputs: not required"
# (configure.ac 6082, because the NDK's libc has no terminfo), but the `exec/` sub-configure is a
# fresh process that asks again (configure.ac 215-218 runs `exec/configure` with the cross
# compiler) and cannot know that answer.  Without this the run dies in `exec/configure` with
# "The required function 'tputs' was not found in any library" *after* the real Android pass has
# already written `src/config.h` (status 4.610).
#
# `--with-gnutls=ifavailable` is the flag configure itself names when it finds no Android gnutls
# ("To build anyway, give: --with-gnutls=ifavailable"): the result records gnutls as off, which is
# what the frozen macOS configuration already records - it has no gnutls key at all.
PATH="$JDK_BIN:$PATH" SDK_BUILD_TOOLS="$BUILD_TOOLS" ANDROID_SDK="$API" ANDROID_CC="$CC" \
  JAVAC="$JAVAC" JARSIGNER="$JARSIGNER" JAVA_HOME="$JDK_HOME" MAKEINFO="$MAKEINFO_BIN" \
  "$REPO/emacs/configure" \
  --host=aarch64-linux-android --with-android="$ANDROID_JAR" CC="$CC" \
  --with-gnutls=ifavailable \
  emacs_cv_working_javac=yes emacs_cv_android_x_or_later=yes \
  emacs_cv_tputs_lib='none required' > configure.log 2>&1
status=$?
echo "configure: exit $status (log: $BUILD/configure.log)"
[ "$status" = 0 ] || { tail -5 configure.log; exit "$status"; }
[ -f src/config.h ] || { echo "configure succeeded but src/config.h is missing" >&2; exit 3; }
# `HAVE_SIGNBIT` is not a config.h macro (the darwin JSON's note records the same measurement):
# it is whether the compiler's <math.h> defines `signbit`, which floatfns.c tests with `#ifndef`.
# Measured with *this* target's compiler, or the constant is missing where the darwin one has it.
printf '#include <math.h>\n#ifdef signbit\nSIGNBIT_PRESENT\n#endif\n' > config-signbit.c
if "$CC" -std=gnu23 -E config-signbit.c 2>/dev/null | grep -q SIGNBIT_PRESENT; then
  HAVE_SIGNBIT=1
else
  HAVE_SIGNBIT=0
fi
echo "HAVE_SIGNBIT: $HAVE_SIGNBIT (measured with ${CC##*/})"
# The *inner* pass's header, not the outer one: `configure` runs a second configure with
# `XCONFIGURE=android` (configure.ac 1296-1300) and renames its product to `src/config.h.android`
# (1315-1316), which is what the Android build compiles against.  The outer pass's `src/config.h`
# defines `ANDROID_STUBIFY` (2604-2606, host-only) - generating from it recorded a stubify build,
# i.e. exactly the configuration in which the JNI entry points are compiled out (status 4.615).
python3 "$REPO/tools/gen-config-json.py" --keys-from-header \
  --macro "HAVE_SIGNBIT=$HAVE_SIGNBIT" src/config.h.android \
  --target "android-arm64 (NDK ${CC##*/})" \
  --out "$REPO/docs/baseline/config/android-arm64.json"
