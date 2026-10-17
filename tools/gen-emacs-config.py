#!/usr/bin/env python3
"""Generate the Android shell's `EmacsConfig` (the reference's build-time class).

`emacs/java/Makefile.in` 411-424 generates `EmacsConfig.java` after the native libraries are
built: it is a bare class holding `EMACS_SHARED_LIBRARIES`, the names the Java (here Kotlin)
side passes to `System.loadLibrary`'s list.  Nothing else in it is configuration, which is why
the port can generate it before its Android native build exists:

    python3 tools/gen-emacs-config.py                 # print the class
    python3 tools/gen-emacs-config.py --out DIR       # write DIR/EmacsConfig.kt

The port's list is the two libraries `emacs/java/Makefile.in` 313-314 always copies into the
APK (`libemacs.so`, `libandroid-emacs.so`); the reference's list also names whatever the NDK
build produced, which the port's Android native build will add when it exists (P-200).  The
package is `org.femacs`, the package the ported Kotlin sources declare.
"""

from __future__ import annotations

import argparse
import pathlib
import sys

HEADER = """// femacs: generated from emacs/java/Makefile.in 411-424 (fd7afced403).
// This is a generated file.  Do not edit!

package org.femacs

object EmacsConfig {
  /* The reference's `public static final String[] EMACS_SHARED_LIBRARIES`.  `@JvmField`
     keeps it a plain field, which is what the Java side hands to the runtime.  */
  @JvmField
  val EMACS_SHARED_LIBRARIES: Array<String> = arrayOf({libraries})
}
"""

LIBRARIES = ("libemacs.so", "libandroid-emacs.so")


def render() -> str:
    joined = ", ".join(f'"{name}"' for name in LIBRARIES)
    return HEADER.replace("{libraries}", joined)


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", type=pathlib.Path, default=None,
                    help="directory to write EmacsConfig.kt into (default: stdout)")
    a = ap.parse_args()
    text = render()
    if a.out is None:
        sys.stdout.write(text)
        return 0
    a.out.mkdir(parents=True, exist_ok=True)
    path = a.out / "EmacsConfig.kt"
    path.write_text(text, encoding="utf-8")
    print(f"wrote {path} ({len(LIBRARIES)} libraries)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
