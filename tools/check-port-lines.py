#!/usr/bin/env python3
"""A target file that is far shorter than its reference is work nobody can see.

`docs/port-map.md` pairs every reference file with its target, and the checklist gates
(`extract-abi.py --check`) compare *the frozen configuration's live lines*.  For a file whose body
sits entirely under a condition the frozen `config.h` leaves false - every `android*.c`,
`haiku*.c`, `w32*.c`, `pgtk*.c`, `msdos*.c`, `lwlib*`, `oldXMenu`, `exec/exec.c` - that live-line
filter (P-73) yields an *empty* checklist, so `--check` passes vacuously and the ledger shows no
unchecked box while the port's target is a three-line header comment.  Measured 2026-10-05: 98 of
590 targets under ten percent of their reference's size, covering 280,076 C lines - a third of the
whole task, invisible to every gate.

This is the reading that sees it.  For every `*.c`/`*.h` target:

    python3 tools/check-port-lines.py                 # compare with docs/baseline/port-lines-pending.tsv
    python3 tools/check-port-lines.py --write-pending # record the list from the tree

A row is `<reference path>\t<C lines>\t<target lines>\t<threshold>\t<reason>`.  A deficit not in the
file fails; a row the tree no longer matches is stale (informational, like the other ratchets).  The
threshold is ten percent of the reference's line count, and a reference under 200 lines is never
counted (small files are small for honest reasons).
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
MAP = REPO / "docs/port-map.md"
PENDING = REPO / "docs/baseline/port-lines-pending.tsv"
FRACTION = 0.10
FLOOR = 200


def map_rows() -> list[tuple[str, int, str]]:
    """`(reference path, reference lines, target path)` for the 1:1 Rust table."""
    out: list[tuple[str, int, str]] = []
    section = None
    for line in MAP.read_text(encoding="utf-8").split("\n"):
        m = re.match(r"^## (.+?) — ", line)
        if m:
            section = m.group(1)
            continue
        m = re.match(r"^\| `([^`]+)` \| ([\d,]+) \| `([^`]+)` \| ([\d,\-]+) \|", line)
        if m and section == "1:1 Rust":
            out.append((m.group(1), int(m.group(2).replace(",", "")), m.group(3)))
    return out


def target_lines(path: str) -> int:
    p = REPO / path
    if not p.exists():
        return 0
    return sum(1 for _ in p.open(encoding="utf-8", errors="replace"))


def deficits() -> list[tuple[str, int, int, int]]:
    out = []
    for ref, c_lines, target in map_rows():
        if c_lines < FLOOR:
            continue
        n = target_lines(target)
        threshold = int(c_lines * FRACTION)
        if n < threshold:
            out.append((ref, c_lines, n, threshold))
    return out


def read_pending() -> dict[str, tuple[int, int, int]]:
    out = {}
    for line in PENDING.read_text(encoding="utf-8").split("\n"):
        if not line or line.startswith("#"):
            continue
        parts = line.split("\t")
        if len(parts) >= 4 and parts[1].isdigit():
            out[parts[0]] = (int(parts[1]), int(parts[2]), int(parts[3]))
    return out


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--write-pending", action="store_true", help="record the tree's reading")
    args = ap.parse_args()
    rows = deficits()
    if args.write_pending:
        head = ["# Targets under ten percent of their reference's line count (docs/status.md 3.285).",
                "# Written by `python3 tools/check-port-lines.py --write-pending`.  A file whose body",
                "# is entirely under a false configuration condition has an *empty* checklist, so the",
                "# unit gates cannot see that it is unported; this list is what sees it.  Columns:",
                "# <reference>\t<C lines>\t<target lines>\t<threshold>\t<reason>.",
                "reference\tc_lines\ttarget_lines\tthreshold\treason"]
        PENDING.write_text("\n".join(head + [f"{r}\t{c}\t{n}\t{t}\tknown-unported target (plan batch F)"
                                             for r, c, n, t in rows]) + "\n", encoding="utf-8")
        total = sum(c for _, c, _, _ in rows)
        print(f"wrote {PENDING.relative_to(REPO)}: {len(rows)} file(s), {total:,} reference line(s)")
        return 0
    pending = read_pending()
    new = [r for r in rows if r[0] not in pending]
    stale = [ref for ref in pending if ref not in {r[0] for r in rows}]
    for ref, c, n, _ in new:
        print(f"NEW      {ref}: {c} lines of C, {n} in the target")
    for ref in stale:
        print(f"stale    {ref}: no longer under the threshold")
    total = sum(c for _, c, _, _ in rows)
    print(f"{len(rows)} file(s) under ten percent, {total:,} reference line(s); "
          f"{len(pending)} ratchet row(s), {len(new)} new, {len(stale)} stale")
    return 1 if new else 0


if __name__ == "__main__":
    raise SystemExit(main())
