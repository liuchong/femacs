#!/usr/bin/env python3
"""Compare the four copied trees with the reference's, accounting for the required name rewrite.

DoD item 5 is "`diff -r emacs/lisp femacs/lisp`, `test/`, `etc/`, `doc/` with no differences" -
but PORTING.md 0.1-19 *requires* one class of difference: the program's name, which the port's
copies carry where the reference's carry the reference's.  A raw `diff -r` therefore reports
thousands of lines that are not defects, and a difference that *is* one hides among them.

This tool does the comparison the DoD means:

  * every file in the reference tree must exist in the port's (a missing file is a failure);
  * a byte-identical file is `identical`;
  * a text file whose only differences are the name rewrite (both the full product name and the
    short form docstrings use) is `name-only`;
  * anything else is `DIFFERENT`, with the first differing lines printed.

It also lists files the port has and the reference does not (they are not failures by themselves:
the port's build writes its generated artifacts into the tree, and the reference's does too, so
the *set* difference is reported for reading rather than judged).

    python3 tools/check-trees.py                 # lisp, test, etc, doc
    python3 tools/check-trees.py --tree lisp     # one tree
    python3 tools/check-trees.py --show 3        # differing lines per file
    python3 tools/check-trees.py --limit 40      # stop after N DIFFERENT files (0 = no limit)
"""

from __future__ import annotations

import argparse
import pathlib
import re
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
REF = REPO / "emacs"
PORT = REPO / "femacs"
TREES = ["lisp", "test", "etc", "doc"]
PORT_NAME = "femacs"
NAME_LINE = re.compile(r"This file is part of ([^.\n]+)\.")
CANDIDATES = ("lisp/international/charprop.el", "lisp/leim/leim-list.el", "lisp/loaddefs.el")

# Extensions whose files are text and may therefore carry the rewrite.  Everything else is
# compared byte for byte (a binary difference is a real one).
def is_text(data: bytes) -> bool:
    """A file is text when it has no NUL byte (the rewrite only ever touches text)."""
    return b"\0" not in data


def retired_paths() -> set[str]:
    """The reference-tree paths `docs/retired-scaffolding.md` records as dropped on purpose.

    PORTING.md P-102 drops the reference's autotools scaffolding, so those files are *expected* to be
    absent from the port: a missing file is only a failure when it is not in that log.
    """
    log = REPO / "docs/retired-scaffolding.md"
    if not log.is_file():
        return set()
    import re as _re
    return set(_re.findall(r"`femacs/([^`]+)`", log.read_text()))


def product_name() -> str:
    for rel in CANDIDATES:
        path = REF / rel
        if path.is_file():
            m = NAME_LINE.search(path.read_text("utf-8", "surrogateescape"))
            if m:
                return m.group(1)
    raise SystemExit("cannot read the reference's product name from its generated files")


def compare(tree: str, product: str, show: int, limit: int) -> tuple[dict[str, int], list[str]]:
    counts = {"identical": 0, "name-only": 0, "DIFFERENT": 0, "missing": 0, "retired": 0,
              "extra": 0}
    failures: list[str] = []
    reference = REF / tree
    retired = retired_paths()
    for src in sorted(reference.rglob("*")):
        if not src.is_file():
            continue
        rel = src.relative_to(reference)
        produced = PORT / tree / rel
        if not produced.is_file():
            if str(src.relative_to(REF)) in retired:
                counts["retired"] += 1
                continue
            counts["missing"] += 1
            failures.append(f"MISSING {tree}/{rel}")
            continue
        want, got = src.read_bytes(), produced.read_bytes()
        if want == got:
            counts["identical"] += 1
            continue
        if not is_text(want) or not is_text(got):
            counts["DIFFERENT"] += 1
            failures.append(f"DIFFERENT {tree}/{rel}: binary, {len(want)} vs {len(got)} bytes")
            continue
        wl = want.decode("utf-8", "surrogateescape").split("\n")
        gl = got.decode("utf-8", "surrogateescape").split("\n")
        short = product.split()[-1]

        def mapped(line: str) -> str:
            return line.replace(product, PORT_NAME).replace(short, PORT_NAME)

        if len(wl) == len(gl) and all(x == y or y == mapped(x) for x, y in zip(wl, gl)):
            counts["name-only"] += 1
            continue
        counts["DIFFERENT"] += 1
        # Only the differences the rewrite cannot explain: the rest are PORTING.md 0.1-19's own
        # class, and printing them buries the ones that are real (the first sample taken this way
        # showed nothing but explained lines).
        unmapped = [(n, x, y) for n, (x, y) in enumerate(zip(wl, gl), 1)
                    if x != y and y != mapped(x)]
        failures.append(f"DIFFERENT {tree}/{rel}: {len(unmapped)} unexplained line(s) "
                        f"of {sum(1 for x, y in zip(wl, gl) if x != y)} differing, "
                        f"{len(wl)} vs {len(gl)} lines")
        if len(wl) != len(gl):
            # A line-count difference is itself the reading: show the tail that has no partner.
            longer, label = ((wl, "reference") if len(wl) > len(gl) else (gl, "port"))
            for line in longer[min(len(wl), len(gl)):][:max(show, 1)]:
                failures.append(f"    {tree}/{rel}: only in the {label}: {line.strip()[:70]!r}")
        if limit and counts["DIFFERENT"] >= limit:
            return counts, failures
        for n, x, y in unmapped[:max(show, 0)]:
            failures.append(f"    {tree}/{rel}:{n}  ref  {x.strip()[:78]!r}")
            failures.append(f"    {tree}/{rel}:{n}  port {y.strip()[:78]!r}")
    produced_tree = PORT / tree
    if produced_tree.is_dir():
        known = {src.relative_to(reference) for src in reference.rglob("*") if src.is_file()}
        for produced in sorted(produced_tree.rglob("*")):
            if produced.is_file() and produced.relative_to(produced_tree) not in known:
                counts["extra"] += 1
    return counts, failures


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--tree", nargs="*", default=TREES)
    parser.add_argument("--show", type=int, default=0)
    parser.add_argument("--limit", type=int, default=40)
    args = parser.parse_args()

    product = product_name()
    totals: dict[str, int] = {}
    all_failures: list[str] = []
    for tree in args.tree:
        counts, failures = compare(tree, product, args.show, args.limit)
        print(f"{tree}: {counts['identical']} identical, {counts['name-only']} name-only, "
              f"{counts['DIFFERENT']} DIFFERENT, {counts['missing']} missing, "
              f"{counts['retired']} retired, {counts['extra']} extra in the port")
        for key, value in counts.items():
            totals[key] = totals.get(key, 0) + value
        all_failures += failures
    print(f"total: " + ", ".join(f"{k} {v}" for k, v in totals.items()))
    # The DIFFERENT details come first and the missing list is summarised: with 1,600+ missing
    # `.elc` files, a single cap on the combined list printed nothing but "MISSING" (4.395's own
    # reporting bug, found while producing that round's punch list).
    different = [line for line in all_failures if "MISSING" not in line]
    missing = [line for line in all_failures if "MISSING" in line]
    for line in different[:300]:
        print(line)
    if len(different) > 300:
        print(f"... {len(different) - 300} more DIFFERENT line(s)")
    for line in missing[:10]:
        print(line)
    if len(missing) > 10:
        print(f"... {len(missing) - 10} more MISSING line(s)")
    return 1 if (totals.get("DIFFERENT") or totals.get("missing")) else 0


if __name__ == "__main__":
    sys.exit(main())
