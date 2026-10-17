#!/bin/sh
# Type-check the Android Kotlin shell against the platform's android.jar - offline.
#
# `docs/port-plan.md`'s platform row and DoD-6 need the Android leg to build.  The Kotlin
# sources (`femacs/platform/android/**/*.kt`, the port of `java/**/*.java`) do not need a full
# Android build to be *checked*: Android Studio ships a Kotlin compiler and a JDK, and the SDK
# ships `android.jar`, so the compile can run with nothing downloaded and nothing installed.
#
#   tools/check-android-kotlin.sh              # compile to a temp dir, print the class count
#   tools/check-android-kotlin.sh --keep DIR   # keep the output classes in DIR
#
# Environment overrides (all optional): STUDIO (the Android Studio bundle), ANDROID_HOME,
# ANDROID_API (default android-36).  Exit status is the compiler's.
set -u
REPO=$(cd "$(dirname "$0")/.." && pwd)
STUDIO=${STUDIO:-/Applications/Android Studio.app/Contents}
ANDROID_HOME=${ANDROID_HOME:-$HOME/Library/Android/sdk}
ANDROID_API=${ANDROID_API:-android-36}
OUT=""
[ "${1:-}" = "--keep" ] && OUT=${2:-}

# Android Studio ships the complete CLI compiler (not just the IDE plugin): `kotlinc` next to
# its own JDK, which is all this check needs.
JAVA_HOME=${JAVA_HOME:-$STUDIO/jbr/Contents/Home}
KOTLINC="$STUDIO/plugins/Kotlin/kotlinc/bin/kotlinc"
STDLIB="$STUDIO/plugins/Kotlin/kotlinc/lib/kotlin-stdlib.jar"
ANDROID_JAR="$ANDROID_HOME/platforms/$ANDROID_API/android.jar"

for f in "$KOTLINC" "$STDLIB" "$ANDROID_JAR"; do
  if [ ! -f "$f" ]; then
    echo "missing: $f" >&2
    echo "the Android leg needs Android Studio (for kotlinc and a JDK) and the SDK's $ANDROID_API" >&2
    exit 2
  fi
done

[ -n "$OUT" ] || OUT=$(mktemp -d /tmp/femacs-kt.XXXXXX)
mkdir -p "$OUT"

FILES=$(find "$REPO/femacs/platform/android" -name '*.kt' | sort)
COUNT=$(printf '%s\n' "$FILES" | grep -c .)
echo "compiling $COUNT Kotlin file(s) against $ANDROID_JAR"
# shellcheck disable=SC2086
JAVA_HOME="$JAVA_HOME" "$KOTLINC" -no-stdlib -no-reflect \
  -classpath "$ANDROID_JAR" -d "$OUT" $FILES
status=$?
classes=$(find "$OUT" -name '*.class' | wc -l | tr -d ' ')
echo "kotlin: exit $status, $classes class file(s) in $OUT"
exit "$status"
