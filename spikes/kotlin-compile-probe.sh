#!/usr/bin/env bash
# Compile the port's Kotlin targets, offline, with the compiler Gradle already ships.
#
# The Kotlin targets (`femacs/platform/android/kotlin/*.kt`, the batch-F rewrite of `java/**`) need
# a compiler this machine did not expose on PATH: `kotlinc` is absent, but Gradle 9.7.1 ships
# `kotlin-compiler-embeddable-2.4.0.jar` in its lib directory, and the Android SDK ships
# `platforms/*/android.jar` - so the CLI runs as a plain `java -cp "<gradle lib>/*"`, with no
# plugin download and no project model (measured 3.286; putting only the compiler's own three jars
# on the classpath fails with `NoClassDefFoundError: kotlin/reflect/...`).
#
#     spikes/kotlin-compile-probe.sh femacs/platform/android/kotlin/EmacsHolder.kt [more.kt ...]
#
# Exit status is the compiler's: a target that does not compile fails here, which is what makes a
# Kotlin file's "ported" claim checkable before any Gradle project exists.  Sources are compiled
# together, so a file that depends on another target has to name it on the same command line (the
# batch's leaves first - the same ordering rule the C side's mirrors have).
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
gradle_lib=""
if [ -n "${GRADLE_HOME:-}" ]; then gradle_lib="$GRADLE_HOME/lib"; else
  gradle_lib=$(ls -d /opt/homebrew/Cellar/gradle/*/libexec/lib 2>/dev/null | tail -1)
fi
android_jar=$(ls "${ANDROID_HOME:-$HOME/Library/Android/sdk}"/platforms/*/android.jar 2>/dev/null | tail -1)
if [ -z "$gradle_lib" ] || [ ! -f "$gradle_lib/kotlin-compiler-embeddable-2.4.0.jar" ]; then
  echo "no Kotlin compiler: set GRADLE_HOME to a Gradle distribution's libexec directory" >&2
  exit 2
fi
if [ -z "$android_jar" ]; then
  echo "no android.jar: set ANDROID_HOME to an Android SDK with a platforms/ directory" >&2
  exit 2
fi
if [ "$#" -eq 0 ]; then
  echo "usage: $0 FILE.kt [FILE.kt ...]" >&2
  exit 2
fi

out=$(mktemp -d)
trap 'rm -rf "$out"' EXIT
kotlin_stdlib="$gradle_lib/kotlin-stdlib-2.4.0.jar"
# `R` is aapt's product and every target that names a resource needs it; it is generated from the
# reference's own `res/` tree (see spikes/android-r-from-res.py) so the names come from the
# reference rather than from a hand-written stub.
"$root/spikes/android-r-from-res.py" > "$out/R.kt"
"java" -cp "$gradle_lib/*" org.jetbrains.kotlin.cli.jvm.K2JVMCompiler \
  -no-stdlib -classpath "$kotlin_stdlib:$android_jar" -d "$out/classes" "$out/R.kt" "$@"
