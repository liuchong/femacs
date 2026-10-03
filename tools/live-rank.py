#!/usr/bin/env python3
"""Rank the not-yet-ported reference files by how much of them this configuration compiles.

The port's cost is not a C file's length but its *live* length: `src/comp.c` is 5,857 lines and
45 of them are compiled (HAVE_NATIVE_COMP is undef), `src/gnutls.c` 3,196 and 124.  The
declaration gate and the line-count check already use `tools/cinventory.py`'s branch evaluator;
this asks it the planning question instead - "which files are cheapest next?" - so a round can
pick its units from evidence rather than from a directory listing.

    python3 tools/live-rank.py                # the 20 cheapest unported files
    python3 tools/live-rank.py --top 50
    python3 tools/live-rank.py --dir lib      # only lib/
    python3 tools/live-rank.py --ported       # also rank files that already have a port

`--ported` is the freshness view: a file whose port is much shorter than its live text has
dropped something, and the ratio is the first thing to look at (the per-file ledger is the
proof).
"""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import cinventory as ci  # noqa: E402

ROOT = Path(__file__).resolve().parent.parent
HERE = ROOT


def live_lines(path: Path, table_ok: bool) -> tuple[int, int]:
    """(live, total) lines of a reference file under the frozen configuration."""
    text = path.read_text(encoding="utf-8", errors="replace")
    total = text.count("\n") + 1
    live = sum(1 for flag in ci.dead_branch_map(text) if flag)
    return (live or total), total


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--top", type=int, default=20)
    ap.add_argument("--dir", default=None, help="only this reference directory (src, lib, ...)")
    ap.add_argument("--ported", action="store_true",
                    help="rank files that already have a port, by |live - port| / live")
    args = ap.parse_args()

    macros = HERE / "docs" / "baseline" / "config" / "darwin-arm64.macros.tsv"
    verified = HERE / "docs" / "baseline" / "config" / "cond-eval-verified.txt"
    if not macros.is_file():
        print(f"error: {macros} is missing (tools/gen-cond-macros.py writes it)", file=sys.stderr)
        return 1
    ci.set_condition_macros(ci.read_condition_macros(macros))
    if verified.is_file():
        ci.set_condition_paths([l.strip() for l in verified.read_text().splitlines()
                                if l.strip() and not l.startswith("#")],
                               reference=str(HERE / "emacs"))

    frozen = HERE / "docs" / "baseline" / "frozen-build-sources.txt"
    if not frozen.is_file():
        print(f"error: {frozen} is missing (docs/status.md 3.31 regenerates it)", file=sys.stderr)
        return 1
    sources = [l.strip() for l in frozen.read_text().splitlines()
               if l.strip() and not l.startswith("#")]

    rows = []
    for rel in sources:
        if args.dir and not rel.startswith(args.dir.rstrip("/") + "/"):
            continue
        c_path = HERE / "emacs" / rel
        rs_path = HERE / "femacs" / Path(rel).with_suffix(".rs")
        if not c_path.is_file():
            continue
        ported = rs_path.is_file()
        if ported != args.ported:
            continue
        live, total = live_lines(c_path, True)
        if args.ported:
            rust = len(rs_path.read_text(encoding="utf-8", errors="replace").split("\n"))
            score = abs(rust - live) / max(live, 1)
            rows.append((score, live, total, rust, rel))
        else:
            rows.append((live, total, 0, 0, rel))
    rows.sort()
    if not rows:
        print("nothing to rank")
        return 0
    if args.ported:
        print(f"{'drift':>7} {'live':>6} {'C':>6} {'rust':>6}  file")
        for score, live, total, rust, rel in rows[:args.top]:
            print(f"{score:7.1%} {live:6d} {total:6d} {rust:6d}  {rel}")
    else:
        print(f"{'live':>6} {'C':>6}  file  (unported, cheapest first)")
        for live, total, _, _, rel in rows[:args.top]:
            print(f"{live:6d} {total:6d}  {rel}")
    print(f"{len(rows)} files ranked", file=sys.stderr)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
