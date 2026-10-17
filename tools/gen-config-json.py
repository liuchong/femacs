#!/usr/bin/env python3
"""Turn a reference `config.h` into the frozen configuration `xtask gen-config` reads.

`docs/baseline/config/*.json` is the frozen input of PORTING.md P-08: for each macro the port
cares about it records one of three states, because the port's condition evaluator needs the
*difference* between "the header does not mention it" and "the header says `#undef`":

    "1"          `#define NAME 1`           (any value, verbatim)
    "undefined"  `/* #undef NAME */`        (the header knows the name and disabled it)
    "absent"     the header says nothing    (configure never tested it)

So a second target - the Android one the platform rows need - is one `configure` run away,
provided the header is turned into this file with the same rules:

    python3 tools/gen-config-json.py build/emacs-ref/src/config.h \
        --target "darwin-arm64 ..." --out /tmp/check.json     # reproduce the frozen one
    python3 tools/gen-config-json.py <android build>/src/config.h \
        --target "android-arm64 ..." --out docs/baseline/config/android-arm64.json

`--keys-from` (default: the committed darwin file) fixes the key set, so two targets are
comparable key for key.
"""

from __future__ import annotations

import argparse
import json
import pathlib
import re
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
DEFAULT_KEYS = REPO / "docs/baseline/config/darwin-arm64.json"

DEFINE = re.compile(r"^#\s*define\s+([A-Za-z_][A-Za-z0-9_]*)(?:\s+(.*?))?\s*$")
UNDEF = re.compile(r"^/\*\s*#\s*undef\s+([A-Za-z_][A-Za-z0-9_]*)\s*\*/")


def read_header(path: pathlib.Path) -> dict[str, str]:
    defines: dict[str, str] = {}
    undefs: set[str] = set()
    for line in path.read_text("utf-8", "surrogateescape").split("\n"):
        m = UNDEF.match(line.strip())
        if m:
            undefs.add(m.group(1))
            continue
        m = DEFINE.match(line.strip())
        if m:
            defines[m.group(1)] = (m.group(2) or "1").strip()
    out: dict[str, str] = {}
    for name, value in defines.items():
        out[name] = value
    for name in undefs:
        out.setdefault(name, "undefined")
    return out


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("header", type=pathlib.Path)
    ap.add_argument("--target", default="unnamed target")
    ap.add_argument("--keys-from", type=pathlib.Path, default=DEFAULT_KEYS)
    ap.add_argument("--out", type=pathlib.Path, default=None)
    # `--macro NAME=VALUE` records a value that is *not* a config.h macro.  `HAVE_SIGNBIT` is the
    # case that needed it: floatfns.c tests `#ifndef signbit`, which is a preprocessor fact about
    # the compiler's <math.h>, and the darwin JSON's note has always said it was measured that
    # way - but nothing carried the measurement into a second target, so Android's configuration
    # lacked the constant and `floatfns.rs` would not compile there (P-227).
    ap.add_argument("--macro", action="append", default=[], metavar="NAME=VALUE")
    # The key list used to come from darwin's JSON alone, so a macro only another target defines
    # (`HAVE_ANDROID`) could not appear anywhere - and the port's `#ifdef` gate for it could never
    # be opened (P-229).  `--keys-from-header` unions in every name this header defines or undefs.
    ap.add_argument("--keys-from-header", action="store_true")
    a = ap.parse_args()

    header = read_header(a.header)
    keys = json.loads(a.keys_from.read_text("utf-8"))["macros"]
    # Header macros beyond the shared key list are *cfg features*, never constants: `gen-config`
    # emits `pub const <name>` for every key, and the mirrors define plenty of these names
    # themselves (`CHECK_LISP_OBJECT_TYPE`, `__STDC_CONSTANT_MACROS`...), so putting them in
    # `macros` made the port define them twice (status 4.633).  They go in their own section,
    # which `config_features()` reads and `gen-config` ignores.
    features = {}
    if a.keys_from_header:
        import re as _re
        for name, value in header.items():
            if not _re.fullmatch(r"[A-Z_][A-Z0-9_]*", name):
                continue
            if name in keys or name in features:
                continue
            features[name] = value
    macros = {k: header.get(k, "absent") for k in keys}
    for spec in a.macro:
        name, _, value = spec.partition("=")
        if not name or not value:
            raise SystemExit(f"--macro wants NAME=VALUE, not {spec!r}")
        keys = [*keys, name]
        macros[name] = value
    doc = {"target": a.target, "source": str(a.header), "macros": macros}
    if features:
        doc["features"] = features
    text = json.dumps(doc, indent=2) + "\n"
    if a.out is None:
        sys.stdout.write(text)
        return 0
    a.out.parent.mkdir(parents=True, exist_ok=True)
    a.out.write_text(text, encoding="utf-8")
    differing = sum(1 for k in keys if macros[k] != json.loads(a.keys_from.read_text("utf-8"))["macros"][k])
    print(f"wrote {a.out} ({len(macros)} macro(s), {differing} differ from {a.keys_from.name})")
    return 0


if __name__ == "__main__":
    sys.exit(main())
