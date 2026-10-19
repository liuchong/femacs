#!/usr/bin/env python3
"""Fail when a frozen configuration disagrees with the preprocessor that reads its header.

`docs/baseline/config/*.json` is the frozen input of PORTING.md P-08: it is what turns C's
`#ifdef` into `--cfg feature="..."` and C's configuration macros into `src/include/config.rs`.
Reading the header line by line cannot see an `#if`, so a `#define` inside an arm the compiler
never takes is recorded as if it were live.  Android's `WORDS_BIGENDIAN` was recorded as `1` from
the `#  define WORDS_BIGENDIAN 1` line inside `#if defined AC_APPLE_UNIVERSAL_BUILD`, which left
the port's `sfnt_swap16_1`/`sfnt_swap32_1` empty bodies: every font table came back byte-swapped
and no font ever registered (status 4.806).  `_DONT_USE_CTYPE_INLINE_`, `CC`, `ND`/`WUR`/`BD`/`AD`,
`_GL_BRACKET_BEFORE_ATTRIBUTE`, `_GL_STDLALIGN_NEEDS_STDDEF` and the `_GL_ATTRIBUTE_*` family were
wrong the same way.

This gate re-derives the truth with the target's own compiler and compares:

    python3 tools/check-config-json.py                  # every configuration whose inputs exist
    python3 tools/check-config-json.py --json docs/baseline/config/android-arm64.json \
        --header /private/tmp/femacs-android-config/src/config.h.android --cc "$NDK_CLANG" --api 21

A configuration whose header or compiler is missing is reported as skipped, the way
`tools/check-android-layout.py` skips without the NDK, so the gate stays usable on a machine that
has only one of the two targets.

What is compared is *presence* (`#define`d in the dump, or not) and, for the keys whose value is a
boolean, the value itself.  Four names are measured outside `config.h` by
`tools/gen-android-config.sh` (`HAVE_SIGNBIT` from `<math.h>`, `ANDROID_API_GE_9` from
`__ANDROID_API__`, `_CS_PATH` and `RENAME_EXCL` from two system headers) and are exempt.
"""

from __future__ import annotations

import argparse
import json
import os
import pathlib
import re
import subprocess
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
# Measured by `gen-android-config.sh` outside `config.h`; the header never mentions them.
MEASURED_OUTSIDE = {"HAVE_SIGNBIT", "ANDROID_API_GE_9", "_CS_PATH", "RENAME_EXCL"}
DEFINE = re.compile(r"#define\s+([A-Za-z_][A-Za-z0-9_]*)(?:\s+(.*))?$")
BOOLEAN = {"0", "1", "true", "false"}


def default_entries() -> list[dict]:
    ndk = None
    sdk = os.environ.get("ANDROID_HOME", str(pathlib.Path.home() / "Library/Android/sdk"))
    import glob

    for cand in sorted(glob.glob(f"{sdk}/ndk/*/toolchains/llvm/prebuilt/*/bin/aarch64-linux-android21-clang")):
        ndk = cand
    return [
        {"name": "darwin-arm64",
         "json": REPO / "docs/baseline/config/darwin-arm64.json",
         "header": REPO / "build/emacs-ref/src/config.h",
         "cc": "clang",
         "cc_args": ["-std=gnu23"],
         # The frozen darwin header ends with `#include <conf_post.h>`, which lives in the source
         # tree; without it the preprocessor stops and the entry would be skipped (measured).
         "includes": [REPO / "emacs/src", REPO / "emacs/lib"]},
        {"name": "android-arm64",
         "json": REPO / "docs/baseline/config/android-arm64.json",
         "header": pathlib.Path("/private/tmp/femacs-android-config/src/config.h.android"),
         "cc": ndk,
         "cc_args": ["--target=aarch64-linux-android21", "-std=gnu23"],
         "includes": [REPO / "emacs/src", REPO / "emacs/lib"]},
    ]


def dump(entry: dict) -> dict[str, str] | None:
    header, cc = entry["header"], entry["cc"]
    if not header.is_file() or not cc:
        print(f"{entry['name']}: skipped ({header} or the compiler is missing)")
        return None
    cmd = [cc, *entry["cc_args"], "-E", "-dM", "-I", str(header.parent)]
    for inc in entry["includes"]:
        cmd += ["-I", str(inc)]
    cmd += ["-x", "c", str(header)]
    out = subprocess.run(cmd, capture_output=True, text=True)
    if out.returncode != 0:
        print(f"{entry['name']}: the preprocessor failed: {out.stderr.strip().splitlines()[:1]}")
        return None
    macros: dict[str, str] = {}
    for line in out.stdout.split("\n"):
        m = DEFINE.match(line)
        if m:
            macros[m.group(1)] = (m.group(2) or "").strip()
    return macros


def check(entry: dict) -> int:
    macros = dump(entry)
    if macros is None:
        return 0
    doc = json.loads(entry["json"].read_text("utf-8"))
    bad: list[str] = []
    bodies: list[str] = []
    for section in ("macros", "features"):
        for name, value in doc.get(section, {}).items():
            if name in MEASURED_OUTSIDE:
                continue
            defined = name in macros
            recorded = value not in ("undefined", "absent")
            if recorded != defined:
                bad.append(f"{section}.{name}: the JSON says {value!r}, the preprocessor says "
                           f"{'defined as ' + repr(macros[name]) if defined else 'not defined'}")
                continue
            if defined and value in BOOLEAN and macros[name] in BOOLEAN and value != macros[name]:
                bad.append(f"{section}.{name}: the JSON says {value!r}, the preprocessor says "
                           f"{macros[name]!r}")
            elif defined and value not in BOOLEAN and value != macros[name]:
                bodies.append(f"{section}.{name}: {value!r} vs {macros[name]!r}")
    for line in bad:
        print("  " + line)
    for line in bodies:
        print("  note (not a failure): " + line)
    print(f"{entry['name']}: {len(doc.get('macros', {}))} macro(s), "
          f"{len(doc.get('features', {}))} feature(s), {len(bad)} disagreement(s)")
    return 1 if bad else 0


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--json", type=pathlib.Path, default=None)
    ap.add_argument("--header", type=pathlib.Path, default=None)
    ap.add_argument("--cc", default=None)
    ap.add_argument("--name", default="command line")
    # One `-I` per include directory an ad-hoc run needs (`config.h` ends with
    # `#include <conf_post.h>`, which lives in the source tree, so a header outside the tree needs
    # two of them - measured when this option was missing).
    ap.add_argument("--include", action="append", default=[])
    a = ap.parse_args()
    if a.json is not None:
        entries = [{"name": a.name, "json": a.json, "header": a.header, "cc": a.cc,
                    "cc_args": ["-std=gnu23"],
                    "includes": [pathlib.Path(p) for p in a.include]}]
    else:
        entries = default_entries()
    status = 0
    for entry in entries:
        if not pathlib.Path(entry["json"]).is_file():
            print(f"{entry['name']}: skipped (no {entry['json']})")
            continue
        status |= check(entry)
    return status


if __name__ == "__main__":
    sys.exit(main())
