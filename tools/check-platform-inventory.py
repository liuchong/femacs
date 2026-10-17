#!/usr/bin/env python3
"""Every platform target the port map names must exist (Swift, Kotlin, C++).

`docs/port-map.md` is generated from the reference checkout, and its non-C rows are the port's
promise that the Objective-C, Java and Haiku C++ sources have counterparts in the new
languages (`docs/goal.md`: `src/*.m` -> Swift, `java/**/*.java` -> Kotlin, `src/haiku_*.cc` ->
modern C++).  The C rows are gated by `extract-abi.py --all`; the platform rows had no gate at
all, so a target could be absent from the tree while the map merely printed `-` for it - which
is exactly what status 4.571 found for `src/nsxwidget.m`.

    python3 tools/check-platform-inventory.py            # every non-C family
    python3 tools/check-platform-inventory.py --show 20
"""

from __future__ import annotations

import argparse
import pathlib
import re
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
MAP = REPO / "docs/port-map.md"
FAMILIES = ("Swift", "Kotlin", "C++")
ROW = re.compile(r"^\|\s*`(?P<source>[^`]+)`\s*\|\s*(?P<lines>[\d,]+)\s*\|\s*`(?P<target>[^`]+)`\s*\|\s*(?P<tlines>[-\d,]*)\s*\|")


def rows(family: str) -> list[tuple[str, str, str]]:
    """(source, target, target-lines cell) for one family's section of the map."""
    out, inside = [], False
    for line in MAP.read_text("utf-8").split("\n"):
        if line.startswith("## "):
            inside = line.startswith(f"## {family} ")
            continue
        if not inside:
            continue
        m = ROW.match(line)
        if m:
            out.append((m.group("source"), m.group("target"), m.group("tlines")))
    return out


def android_data() -> list[tuple[str, str]]:
    """(reference, target) pairs for the Android data the Kotlin shell needs (P-199).

    `emacs/java/AndroidManifest.xml.in` and `emacs/java/res/**` are what `aapt2` turns into the
    `R` class the Kotlin sources reference; the port has neither, so `kotlinc` reports
    `unresolved reference 'R'` on every use (measured 2026-10-17).
    """
    out = []
    manifest = pathlib.PurePath("java/AndroidManifest.xml.in")
    if (REPO / "emacs" / manifest).is_file():
        out.append((str(manifest), "femacs/platform/android/AndroidManifest.xml"))
    for ref in sorted((REPO / "emacs/java/res").rglob("*")):
        if ref.is_file():
            rel = ref.relative_to(REPO / "emacs")
            out.append((str(rel), str(pathlib.PurePath("femacs/platform/android") / rel.parts[-2:][0] / rel.parts[-2:][1]) if False else str(pathlib.PurePath("femacs/platform/android") / pathlib.PurePath(*rel.relative_to("java").parts))))
    return out


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--show", type=int, default=10)
    # Kept for one purpose only: a diagnostic run on a tree where a target is known to be
    # absent (P-198 removed the only use `gate-fast` had for it).  It is *not* wired in.
    ap.add_argument("--warn", action="store_true",
                    help="report gaps but exit 0 (diagnostic only; not wired into gate-fast)")
    a = ap.parse_args()
    missing = 0
    total = 0
    for family in FAMILIES:
        entries = rows(family)
        bad = []
        for source, target, tlines in entries:
            total += 1
            path = REPO / target
            lines = sum(1 for _ in path.open("rb")) if path.is_file() else 0
            if not path.is_file() or lines == 0:
                bad.append((source, target, tlines, lines))
        print(f"{family:7s}: {len(entries)} entry(ies), {len(bad)} without a target file")
        for source, target, tlines, lines in bad[:a.show]:
            print(f"    MISSING {source} -> {target} (map says {tlines or '-'}, file has {lines} lines)")
        if len(bad) > a.show:
            print(f"    … {len(bad) - a.show} more")
        missing += len(bad)
    data = android_data()
    data_missing = [(ref, tgt) for ref, tgt in data if not (REPO / tgt).is_file()]
    print(f"Android data: {len(data)} entry(ies), {len(data_missing)} without a target file")
    for ref, tgt in data_missing[:a.show]:
        print(f"    MISSING {ref} -> {tgt}")
    if len(data_missing) > a.show:
        print(f"    … {len(data_missing) - a.show} more")
    print(f"{total} platform entry(ies): {missing} without a target file")
    if (missing or data_missing) and a.warn:
        print("(warning only: P-197 is open; remove --warn when the gap is closed)")
        return 0
    return 1 if (missing or data_missing) else 0


if __name__ == "__main__":
    sys.exit(main())
