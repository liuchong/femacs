#!/usr/bin/env python3
"""The "copied verbatim" screen: the port's `lisp/ leim/ test/ etc/ doc/ admin/` against the reference's.

PORTING.md's rule 7 and the goal's hard constraint are that these trees are *copied*, one word
unchanged, with a single recorded exception: the GPL declaration's product name, which the
project rewrites for legal reasons (`GNU Emacs` -> `femacs`).  The screen therefore accepts
exactly three outcomes per file:

    identical            - byte for byte the same as the reference
    declaration-only      - equal after the rewrite, applied *on the declaration lines only*
    other                 - anything else, printed with its first differing lines

The middle case is the one that needs the anchor: measured 2026-10-16, the rewrite is not a
plain `GNU Emacs` -> `femacs` over the whole file (2,003 files differ, and 362 of them are
neither the identity nor that substitution - some keep `GNU Emacs` in a declaration-looking
line, some rewrite a bare `Emacs` in prose), so the anchor is what makes the claim checkable
at all.  The exclusions are the nine retired with `Makefile
.in` scaffolding files, whose record is `docs/retired-scaffolding.md` (PORTING.md P-102).

    python3 tools/check-copied-trees.py                 # counts by directory + samples
    python3 tools/check-copied-trees.py --all           # every "other" file, one per line
"""

from __future__ import annotations

import argparse
import pathlib
import subprocess
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
REF = REPO / "emacs"
PORT = REPO / "femacs"
TREES = ["lisp", "leim", "test", "etc", "doc", "admin"]

# The nine files the port deliberately does not carry (docs/retired-scaffolding.md).
RETIRED = {
    "lisp/Makefile.in", "leim/Makefile.in", "admin/unidata/Makefile.in",
    "admin/charsets/Makefile.in", "admin/grammars/Makefile.in", "doc/emacs/Makefile.in",
    "doc/lispintro/Makefile.in", "doc/lispref/Makefile.in", "doc/misc/Makefile.in",
}

# The declaration anchors: the GPL header sentences, in the comment syntaxes the trees use.
ANCHORS = ("This file is part of", "is free software: you can redistribute",
           "is distributed in the hope that it will be useful",
           "along with", "This is part of")


def rewrite(line: str) -> str:
    if any(a in line for a in ANCHORS):
        return line.replace("GNU Emacs", "femacs").replace(
            "This is part of the Emacs manual", "This is part of the femacs manual")
    return line


def classify(a: pathlib.Path, b: pathlib.Path) -> tuple[str, str]:
    if not b.is_file():
        return "missing", "absent from the port"
    at, bt = a.read_text(errors="replace"), b.read_text(errors="replace")
    if at == bt:
        return "identical", ""
    rewritten = "\n".join(rewrite(l) for l in at.split("\n"))
    if rewritten == bt:
        return "declaration-only", ""
    for i, (x, y) in enumerate(zip(rewritten.split("\n"), bt.split("\n"))):
        if x != y:
            return "other", f"line {i + 1}: {x[:70]!r} vs {y[:70]!r}"
    return "other", "length differs"


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--all", action="store_true", help="print every file in the 'other' class")
    ap.add_argument("--limit", type=int, default=20, help="samples per directory")
    a = ap.parse_args()

    tracked = subprocess.run(["git", "-C", str(REF), "ls-files", *TREES],
                             capture_output=True, text=True, check=True).stdout.split()
    counts = {"identical": 0, "declaration-only": 0, "other": 0, "missing": 0}
    per_dir: dict[str, dict[str, int]] = {}
    others: list[tuple[str, str]] = []
    for rel in tracked:
        if rel in RETIRED:
            continue
        kind, why = classify(REF / rel, PORT / rel)
        counts[kind] += 1
        d = per_dir.setdefault(rel.split("/")[0], dict.fromkeys(counts, 0))
        d[kind] += 1
        if kind in ("other", "missing"):
            others.append((rel, why))
    print(f"tracked files (minus the 9 retired): {sum(counts.values())}")
    print(f"  byte-identical to the reference: {counts['identical']}")
    print(f"  equal after the declaration-line rewrite: {counts['declaration-only']}")
    print(f"  other differences: {counts['other']}   missing: {counts['missing']}")
    for d in sorted(per_dir):
        c = per_dir[d]
        print(f"    {d:6s} identical={c['identical']:5d} declaration-only={c['declaration-only']:5d} "
              f"other={c['other']:4d} missing={c['missing']}")
    shown: dict[str, int] = {}
    for rel, why in others:
        d = rel.split("/")[0]
        if not a.all:
            if shown.get(d, 0) >= a.limit:
                continue
            shown[d] = shown.get(d, 0) + 1
        print(f"    {rel}: {why}")
    # A reading, not a verdict: the 'other' class is what the next pass has to account for.
    return 0


if __name__ == "__main__":
    sys.exit(main())
