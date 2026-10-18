#!/usr/bin/env python3
"""Generate the Android shell's `EmacsConfig` (the reference's build-time class).

`emacs/java/Makefile.in` 409-427 generates `EmacsConfig.java` in a second `make` instance, once
the shared libraries exist: it is a bare class holding `EMACS_SHARED_LIBRARIES`, the names the
Java (here Kotlin) side hands to `System.loadLibrary` in its static initialiser.  Nothing else in
it is configuration, which is why the port can generate it before its own Android native build
exists:

    python3 tools/gen-emacs-config.py --out DIR --library build/port/aarch64-linux-android/libemacs.so
    python3 tools/gen-emacs-config.py --out DIR          # no library named: an empty list

**What the list is.**  `Makefile.in` 375-404 computes `ALL_DEPENDENCIES` from the *linked*
libraries: for each NDK-built shared library it runs `llvm-readelf -d` (make's
`NDK_BUILD_READELF`), takes every `(NEEDED)` entry, recurses into those that are themselves in
the ndk-build directory, and de-duplicates.  Every entry is written with
`$(basename $(notdir ...))`, so the names carry **no** `.so` suffix (`"libgnutls"`, `"libc"`).
That is the shape `EmacsNative`'s loader expects: it strips a leading `lib` and passes the rest
to `System.loadLibrary` (`"libc"` -> `"c"`, which its system-library test then skips).

**What the list is not.**  It is not the set of files the packaging step copies into the APK:
those are `CROSS_LIBS` (the linked `libemacs.so`) and `CROSS_BINS` (the cross-built executables,
renamed `lib<name>.so` - among them `libandroid-emacs.so`, which is the `android-emacs`
*launcher program* of `src/android-emacs.c`, not a library anything loads).  The port's first
version of this generator used that copy list - `("libemacs.so", "libandroid-emacs.so")` - and
the app then died on the device with `dlopen failed: library "libemacs.so.so" not found` (the
`lib` strip left the `.so` in place; measured on the emulator, status 4.706).  A name with a
`.so` suffix, or a name whose file the APK does not carry, breaks the static initialiser
however plausible it looks next to the reference's own copy list.
"""

from __future__ import annotations

import argparse
import os
import pathlib
import re
import subprocess
import sys

HEADER = """// femacs: generated from emacs/java/Makefile.in 409-427 (fd7afced403).
// This is a generated file.  Do not edit!

package org.femacs

object EmacsConfig {
  /* The reference's `public static final String[] EMACS_SHARED_LIBRARIES`, the list
     `EmacsNative`'s static initialiser loads through `System.loadLibrary`.  Its recipe is
     `java/Makefile.in` 375-404: the `(NEEDED)` closure of the linked library, each entry
     `$(basename $(notdir ...))` - a library *name* with no `.so` suffix.  */
  @JvmField
  val EMACS_SHARED_LIBRARIES: Array<String> = arrayOf({libraries})
}
"""

# The libraries the loader skips: `EmacsNative`'s own test, on the name with `lib` stripped.
SYSTEM_LIBRARIES = ("z", "c", "m", "dl", "log", "android")

NEEDED = re.compile(r"\(NEEDED\)\s+Shared library: \[(.+?)\]")


def find_readelf(explicit: str | None) -> str | None:
    """The NDK's `llvm-readelf`, the tool the reference's make step uses."""
    if explicit:
        return explicit
    for env in ("ANDROID_NDK_HOME", "ANDROID_NDK_ROOT"):
        root = os.environ.get(env)
        if root:
            found = sorted(pathlib.Path(root).glob("toolchains/llvm/prebuilt/*/bin/llvm-readelf"))
            if found:
                return str(found[-1])
    home = os.environ.get("ANDROID_SDK_ROOT") or os.environ.get("ANDROID_HOME")
    sdk = pathlib.Path(home) if home else pathlib.Path.home() / "Library/Android/sdk"
    found = sorted(sdk.glob("ndk/*/toolchains/llvm/prebuilt/*/bin/llvm-readelf"))
    return str(found[-1]) if found else None


def needed(readelf: str, library: pathlib.Path) -> list[str]:
    run = subprocess.run([readelf, "-d", str(library)], capture_output=True, text=True)
    if run.returncode != 0:
        sys.exit(f"{readelf} could not read {library}: {(run.stderr or run.stdout).strip()[:200]}")
    return NEEDED.findall(run.stdout)


def dependency_closure(readelf: str, library: pathlib.Path) -> list[str]:
    """`Makefile.in` 375-404: each dependency's own dependencies first, each name once."""
    order: list[str] = []
    seen: set[pathlib.Path] = set()

    def visit(path: pathlib.Path) -> None:
        if path in seen:
            return
        seen.add(path)
        for entry in needed(readelf, path):
            # The reference recurses only into a dependency that lives in the ndk-build output
            # directory (`$(wildcard $(top_builddir)/cross/ndk-build/$1.so)`); a system library
            # is not there, so it is recorded and not descended into.
            sibling = path.parent / entry
            if sibling.is_file():
                visit(sibling)
            name = entry[:-3] if entry.endswith(".so") else entry
            if name not in order:
                order.append(name)

    visit(library)
    return order


def render(libraries: tuple[str, ...]) -> str:
    joined = ", ".join(f'"{name}"' for name in libraries)
    return HEADER.replace("{libraries}", joined)


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", type=pathlib.Path, default=None,
                    help="directory to write EmacsConfig.kt into (default: stdout)")
    ap.add_argument("--library", type=pathlib.Path, default=None,
                    help="the linked Android library whose (NEEDED) closure the class names, "
                         "e.g. build/port/aarch64-linux-android/libemacs.so")
    ap.add_argument("--readelf", default=None, help="llvm-readelf to use (default: the NDK's)")
    a = ap.parse_args()

    libraries: tuple[str, ...] = ()
    if a.library is not None:
        if not a.library.is_file():
            sys.exit(f"--library {a.library} does not exist (build it, or leave the flag off)")
        readelf = find_readelf(a.readelf)
        if readelf is None:
            sys.exit("no llvm-readelf found: pass --readelf, or set ANDROID_NDK_HOME / "
                     "ANDROID_SDK_ROOT")
        libraries = tuple(dependency_closure(readelf, a.library))

    text = render(libraries)
    if a.out is None:
        sys.stdout.write(text)
        return 0
    a.out.mkdir(parents=True, exist_ok=True)
    path = a.out / "EmacsConfig.kt"
    path.write_text(text, encoding="utf-8")
    loaded = [n for n in libraries if n.removeprefix("lib") not in SYSTEM_LIBRARIES]
    detail = (f"{len(libraries)} (NEEDED) entr{'y' if len(libraries) == 1 else 'ies'} of "
              f"{a.library}") if a.library is not None else "no library named"
    print(f"wrote {path} ({detail})")
    if a.library is not None:
        print(f"EmacsConfig: loads at startup: {' '.join(loaded) if loaded else 'none'} "
              f"(the APK must carry every one of those)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
