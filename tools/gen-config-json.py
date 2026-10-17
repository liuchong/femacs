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
    a = ap.parse_args()

    header = read_header(a.header)
    keys = json.loads(a.keys_from.read_text("utf-8"))["macros"]
    macros = {k: header.get(k, "absent") for k in keys}
    doc = {"target": a.target, "source": str(a.header), "macros": macros}
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
