#!/usr/bin/env python3
"""Triage the inventory's `wip` rows: which of them are actually finished?

Why this exists (docs/status.md 3.198/3.199): the inventory's state column is derived from the
*ledger*, and a ledger is prose written by hand, so it can disagree with what the tree proves.
`src/process.c` sat in WIP for rounds after it was finished (9,126 lines, `--check` OK, 235/235
boxes, builds) because the state test matched a `- [ ]` quoted inside its own Evidence; and three
mirrors were `--check`-clean while their boxes were stale.  Neither reading is wrong on its own -
the fix is to *compare* them, which is what this tool does:

  for every `wip` row of docs/port-inventory.md
    * run `tools/extract-abi.py --check <reference> <port>` and count its `- ` rows
    * count the ledger's unchecked boxes (`^\\s*- \\[ \\]`)

and group the rows:

  A  FAIL and unchecked boxes        - genuinely in progress; nothing to do
  B  OK and unchecked boxes          - read the ledger's `## Decisions`: either a *deliberate*
                                       deferral (its own text says so) or a stale box that should
                                       be ticked with the check as its evidence
  C  OK and no unchecked boxes       - the inventory is wrong: this unit is done.  This is an
                                       accounting bug, and the exit status is non-zero so a
                                       wrapper can catch it.

Exit status: 0 when group C is empty, 1 otherwise (group B is a judgement call, not a failure).
"""

from __future__ import annotations

import pathlib
import re
import subprocess
import sys

HERE = pathlib.Path(__file__).resolve().parent
REPO = HERE.parent
INVENTORY = REPO / "docs" / "port-inventory.md"


def port_path_for(ref: str) -> pathlib.Path | None:
    """The port of `ref`, by the path rule the map generator uses."""
    rel = pathlib.Path(ref)
    stem = rel.stem
    if rel.suffix == ".h":
        cands = [REPO / "femacs" / rel.parent / "include" / (stem + ".rs"),
                 REPO / "femacs" / rel.parent / (stem + ".rs")]
        cands += [REPO / "femacs/src/include" / (stem + ".rs"),
                  REPO / "femacs/lib/include" / (stem + ".rs")]
    else:
        cands = [REPO / "femacs" / rel.with_suffix(".rs")]
    return next((p for p in cands if p.is_file()), None)


def ledger_for(ref: str) -> pathlib.Path | None:
    rel = pathlib.Path(ref)
    name = ("include-" + rel.stem + ".md") if rel.suffix == ".h" else (rel.stem + ".md")
    for cand in (REPO / "docs" / "ports" / name,
                 REPO / "docs" / "ports" / ("include-" + rel.stem + ".md")):
        if cand.is_file():
            return cand
    return None


def main() -> int:
    rows: list[tuple[str, pathlib.Path, int, int]] = []
    for line in INVENTORY.read_text(encoding="utf-8").splitlines():
        if not line.startswith("| `") or not line.rstrip().endswith("| wip |"):
            continue
        cells = [c.strip() for c in line.split("|")[1:-1]]
        ref = cells[0].strip("` ")
        port = next((c.strip("`") for c in cells if c.startswith("`") and c.endswith(".rs`")), None)
        if port is None:
            continue                                   # the per-batch summary table
        p = REPO / port
        if not p.is_file():
            rows.append((ref, p, -1, -1)); continue
        out = subprocess.run([sys.executable, "tools/extract-abi.py", "--check",
                              "emacs/" + ref, port],
                             cwd=REPO, capture_output=True, text=True).stdout
        problems = sum(1 for l in out.splitlines() if l.startswith("- "))
        ledger = ledger_for(ref)
        unchecked = 0
        if ledger:
            unchecked = sum(1 for l in ledger.read_text(encoding="utf-8", errors="replace")
                            .splitlines() if re.match(r"^\s*- \[ \]", l))
        rows.append((ref, p, problems, unchecked))

    groups = {"A": [], "B": [], "C": [], "?": []}
    for ref, p, problems, unchecked in rows:
        if problems < 0:
            groups["?"].append((ref, p, problems, unchecked))
        elif problems:
            groups["A"].append((ref, p, problems, unchecked))
        elif unchecked:
            groups["B"].append((ref, p, problems, unchecked))
        else:
            groups["C"].append((ref, p, problems, unchecked))

    def show(title: str, items, note: str):
        print(f"\n{title} ({len(items)}) - {note}")
        for ref, p, problems, unchecked in items:
            print(f"  {ref:28s} {str(p.relative_to(REPO)):36s} "
                  f"{problems if problems >= 0 else '?':>4} problem row(s), {unchecked} unchecked")

    show("A. in progress (check fails, boxes left)",
         groups["A"], "genuinely unfinished; nothing to do")
    show("B. check OK, boxes left",
         groups["B"], "read `## Decisions`: a deliberate deferral, or a stale box to tick")
    show("C. check OK, no boxes left",
         groups["C"], "THE INVENTORY IS WRONG - this unit is done")
    if groups["?"]:
        show("?. no port file", groups["?"], "the inventory lists a path that does not exist")
    return 1 if groups["C"] else 0


if __name__ == "__main__":
    sys.exit(main())
