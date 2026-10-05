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
# `EmacsConfig` is the second generated class the app layer needs and the reference tree has no
# `.java` for it: `emacs/java/Makefile.in` 409-427 emits it (`cf-stamp-1`) with exactly one
# member, `public static final String[] EMACS_SHARED_LIBRARIES`, whose value is the Android
# build's dependency list.  The probe writes the same shape with an empty list - the *name* is
# what the ported source type-checks against, and the value is the build's business (the `R.kt`
# convention: real names, probe-owned values).  Measured 3.321: without it,
# `EmacsNative.kt:292` is `unresolved reference 'EmacsConfig'` and the whole-directory probe fails.
cat > "$out/EmacsConfig.java" <<'JAVA'
package org.femacs;
/* femacs probe stand-in: emacs/java/Makefile.in 409-427 generates this class in the build tree. */
public class EmacsConfig
{
/* This is a generated file.  Do not edit!  */
public static final String[] EMACS_SHARED_LIBRARIES= {};
}
JAVA
# Targets whose dependencies are not ported yet still get a real compile: the reference's own
# `java/**` files stand in for them (the same API the port will later reproduce), with the ported
# `.kt` names removed so a class is never defined twice.  What this proves is the *ported* file's
# syntax and types against the reference's interfaces; the interface agreement with the port's own
# later files is what the cluster closures in docs/baseline/kotlin-order.tsv check.
ported=""
java_deps=""
for kt in "$@"; do ported="$ported ${kt##*/}"; done
mkdir -p "$out/stubs"
for java in "$root"/emacs/java/org/gnu/emacs/*.java; do
  stem="$(basename "$java" .java)"
  case " $ported " in
    *" ${stem}.kt "*) continue ;;
  esac
  # A stand-in has to move into **our** package with the port (status 3.503): the ported Kotlin
  # lives in `org.femacs` now, and a class in another package is not visible without an import,
  # so the reference file's own `package` line is rewritten on the way in.  The body - every
  # name the port has to agree with - is the reference's, unchanged.
  sed 's/^package org\.gnu\.emacs;/package org.femacs;/' "$java" > "$out/stubs/$stem.java"
  java_deps="$java_deps $out/stubs/$stem.java"
done
"java" -cp "$gradle_lib/*" org.jetbrains.kotlin.cli.jvm.K2JVMCompiler \
  -no-stdlib -classpath "$kotlin_stdlib:$android_jar" -d "$out/classes" "$out/R.kt" \
  "$out/EmacsConfig.java" $java_deps "$@"
