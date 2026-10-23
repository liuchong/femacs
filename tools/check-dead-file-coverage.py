#!/usr/bin/env python3
"""A P-86 file's ledger must account for every non-blank line of its reference.

Why this exists: for a file whose body sits entirely under conditions the frozen configuration
leaves false, the checklist `extract-abi.py` generates is empty (P-86 clause 2), so the ledger
records a table of C line ranges instead of ticked boxes, and `check-port-lines.py`'s ratchet is
the only other reading - which compares totals and therefore cannot see "I stopped halfway through
a 3,000-line file".  This gate reads the table.

    python3 tools/check-dead-file-coverage.py             # scan every ledger with the section
    python3 tools/check-dead-file-coverage.py --selftest

The scope is self-selecting: a ledger is bound when it carries the `## C line ranges` section (the
marker P-86 clause 2 names), and a file with no table is not bound - `check-port-lines.py` is what
sees those.  For a bound ledger the reference path and its line count come from
`docs/port-map.md`, and the reference's own text is read to classify a line the table does not
cover: a *blank* line between two blocks needs no row (every file in the class has them, and
`androidmenu.c` has twenty-two), while any other uncovered line is a gap and fails.  The rows must
also ascend without overlapping, so the table stays a table rather than an unordered list.

The first P-86 file is the worked example: `docs/ports/androidmenu.md`'s 22 rows cover C 20-860
with C 1-18 marked `dropped` (the licence head, PORTING.md 0.1-19), and the gate's reading is what
says so mechanically rather than by reading the prose.
"""

from __future__ import annotations

import argparse
import pathlib
import re
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
MAP = REPO / "docs/port-map.md"
LEDGERS = REPO / "docs/ports"
SECTION = "## C line ranges"
MAP_ROW = re.compile(r"^\| `([^`]+)` \| ([\d,]+) \| `([^`]+)` \| ([\d,]+) \|$")
TABLE_ROW = re.compile(r"^\|\s*([^|]+?)\s*\|")
RANGE = re.compile(r"^(\d+)(?:\s*-\s*(\d+))?$")


def ledger_for(target: str, refs: set[str]) -> pathlib.Path | None:
    """The ledger a port-map target uses (the rule `check-ledger-ticks.py` shares)."""
    rel = pathlib.PurePosixPath(target)
    stem, base = rel.stem, rel.parent
    if base.name == "include":
        base = base.parent
    prefix = "" if base.as_posix() == "." else base.as_posix().replace("/", "-") + "-"
    for name in dict.fromkeys([f"{prefix}{stem}.md", f"{stem}.md"]):
        candidate = LEDGERS / name
        if candidate.is_file():
            return candidate
    return None


def parse_ranges(text: str) -> tuple[list[tuple[int, int, str]], str | None]:
    """`(ranges, error)` for a ledger's C range table.

    A row's first cell holds one or more ranges separated by `/` or `,` (a file can carry the same
    construct at several positions), and the whole row's text is kept as the arm description.
    """
    if SECTION not in text:
        return [], None
    body = text.split(SECTION, 1)[1]
    body = body.split("\n## ", 1)[0]
    out: list[tuple[int, int, str]] = []
    for line in body.split("\n"):
        if not line.startswith("|"):
            continue
        cell = TABLE_ROW.match(line)
        if not cell:
            continue
        first = cell.group(1).strip()
        if first in ("C", "") or set(first) <= set("-: "):
            continue
        for part in re.split(r"[/,]", first):
            part = part.strip()
            m = RANGE.match(part)
            if not m:
                return [], f"row {first!r} is not one or more `<start>`/`<start>-<end>` ranges"
            start = int(m.group(1))
            end = int(m.group(2)) if m.group(2) else start
            out.append((start, end, line))
    return out, None


def coverage_failures(rel: str, ranges: list[tuple[int, int, str]], blanks: set[int],
                      ref_lines: int) -> tuple[int, list[str], int]:
    """`(failures, messages)` for one table against a reference of `ref_lines` lines.

    `blanks` are the reference's own empty lines, which need no row: every file in this class has
    them between blocks.  Everything else must be inside a range, the ranges must ascend without
    overlapping, and the last one must reach the reference's final line.
    """
    msgs: list[str] = []
    failures = 0
    covered: set[int] = set()
    prev_end = 0
    for start, end, _ in sorted(ranges):
        if start <= prev_end:
            failures += 1
            msgs.append(f"FAIL {rel}: range {start}-{end} does not follow the previous one")
        if end < start:
            failures += 1
            msgs.append(f"FAIL {rel}: range {start}-{end} is inverted")
        covered.update(range(start, end + 1))
        prev_end = max(prev_end, end)
    gaps = [str(n) for n in range(1, ref_lines + 1) if n not in covered and n not in blanks]
    if gaps:
        failures += 1
        shown = ", ".join(gaps[:12]) + (" ..." if len(gaps) > 12 else "")
        msgs.append(f"FAIL {rel}: {len(gaps)} reference line(s) with text are not in any range "
                    f"({shown})")
    if prev_end < ref_lines:
        failures += 1
        msgs.append(f"FAIL {rel}: the last range ends at {prev_end}, the reference has {ref_lines} "
                    f"line(s)")
    # the blanks no range covers are the ones the table legitimately leaves out
    return failures, msgs, len([n for n in blanks if n not in covered])


def check_one(ref: str, ref_lines: int, ledger: pathlib.Path) -> tuple[int, list[str]]:
    """`(failures, messages)` for one bound ledger, reading the reference for its blank lines."""
    text = ledger.read_text(encoding="utf-8")
    ranges, error = parse_ranges(text)
    rel = ledger.relative_to(REPO).as_posix()
    if error:
        return 1, [f"FAIL {rel}: {error}"]
    if not ranges:
        return 1, [f"FAIL {rel}: has the `{SECTION}` heading but no rows"]
    reference = (REPO / "emacs" / ref).read_text(encoding="utf-8", errors="replace").split("\n")
    blanks = {n for n in range(1, min(ref_lines, len(reference)) + 1)
              if not reference[n - 1].strip()}
    failures, msgs, slack = coverage_failures(rel, ranges, blanks, ref_lines)
    msgs.append(f"info {rel}: {len(ranges)} range row(s) cover {ref} ({ref_lines} lines; {slack} "
                f"blank line(s) between blocks need no row)")
    return failures, msgs


def selftest() -> int:
    bad = 0
    table = lambda rows: SECTION + "\n\n| C | block | arm |\n| --- | --- | --- |\n" + rows
    cases = [
        ("a complete table passes",
         table("| 1-18 | head | dropped |\n| 20 | body | gate |\n"), {19}, 20, 0),
        ("a line with text outside the ranges fails",
         table("| 1-5 | a | gate |\n"), set(), 5, 0),
        ("a gap fails",
         table("| 1-4 | a | gate |\n| 6-20 | b | gate |\n"), set(), 20, 1),
        ("an overlap fails",
         table("| 1-6 | a | gate |\n| 6-20 | b | gate |\n"), set(), 20, 1),
        ("a short tail fails",
         table("| 1-8 | a | gate |\n"), {n for n in range(9, 21)}, 20, 1),
        ("several ranges in one row are read as several",
         table("| 1-5 / 9-20 | a | gate |\n"), {6, 7, 8}, 20, 0),
        ("an unparsable row is an error",
         table("| the whole file | a | gate |\n"), set(), 20, -1),
    ]
    for name, text, blanks, ref_lines, want in cases:
        ranges, error = parse_ranges(text)
        if want == -1:
            ok = error is not None
            got = "error" if error else "no error"
        else:
            failures, _, _ = coverage_failures("<test>", ranges, blanks, ref_lines)
            ok = failures == want
            got = f"{failures} failure(s)"
        print(f"{'ok  ' if ok else 'FAIL'} {name}: {got}")
        bad += 0 if ok else 1
    print(f"{len(cases)} cases, {bad} failures")
    return 1 if bad else 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--selftest", action="store_true")
    args = ap.parse_args()
    if args.selftest:
        return selftest()

    rows = []
    for line in MAP.read_text(encoding="utf-8").split("\n"):
        m = MAP_ROW.match(line)
        if m and m.group(3).endswith(".rs"):
            ref = m.group(1)
            rows.append((ref, int(m.group(2).replace(",", "")), m.group(3)))
    bound = 0
    failures = 0
    for ref, ref_lines, target in rows:
        ledger = ledger_for(target, {r for r, _, _ in rows})
        if ledger is None or SECTION not in ledger.read_text(encoding="utf-8"):
            continue
        bound += 1
        f, msgs = check_one(ref, ref_lines, ledger)
        failures += f
        for msg in msgs:
            print(msg)
    print(f"{bound} ledger(s) bound (a C range table), {failures} failure(s)")
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
