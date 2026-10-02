#!/usr/bin/env python3
"""Generate LIFETIMES.tsv: the ownership and lifetime ledger for the port.

Rows come from the reference sources through `tools/cinventory.py`:

  struct_field   every field of every struct/union in the C headers
  global         every file-scope object in the C sources

Columns marked `auto:` are mechanical hints and must be checked.  The decision
columns (owner, freed_by, crosses_setjmp, in_dump, note) are filled in by the
porter of the file and cross-checked adversarially; a row is decided once `owner`
and `freed_by` are non-empty.

    python3 tools/gen-lifetimes.py [--reference emacs] [--out LIFETIMES.tsv]
"""

from __future__ import annotations

import argparse
import importlib.util
import re
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("cinventory", HERE / "cinventory.py")
ci = importlib.util.module_from_spec(spec)
spec.loader.exec_module(ci)

C_DIRS = ("src", "lib", "lib-src", "lwlib", "oldXMenu", "exec", "nt", "msdos")
DECISION = ["owner", "freed_by", "crosses_setjmp", "in_dump", "note"]
GC_TYPES = ("Lisp_Object", "Lisp_", "INTERVAL", "struct interval", "Lisp_Word")


def flat(text: str) -> str:
    return re.sub(r"[\t\n\r]+", " ", str(text)).strip()


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--reference", default="emacs")
    ap.add_argument("--out", default="LIFETIMES.tsv")
    args = ap.parse_args()
    ref = Path(args.reference)

    staticpros = set()
    for c in ref.glob("src/*.c"):
        staticpros |= set(re.findall(r"staticpro\s*\(\s*&+\s*([A-Za-z_]\w*)\s*\)",
                                     c.read_text(encoding="utf-8", errors="replace")))
    pdumper = (ref / "src/pdumper.c").read_text(encoding="utf-8", errors="replace")
    dmpstruct = (ref / "src/dmpstruct.h").read_text(encoding="utf-8", errors="replace") \
        if (ref / "src/dmpstruct.h").is_file() else ""
    dump_text = pdumper + dmpstruct

    rows = []
    headers = [p for d in C_DIRS for p in sorted((ref / d).rglob("*.h"))]
    sources = [p for d in C_DIRS for p in sorted((ref / d).rglob("*.c"))]
    for h in headers:
        text = h.read_text(encoding="utf-8", errors="replace")
        for f in ci.struct_fields(text):
            rel = str(h.relative_to(ref))
            gc = "yes" if any(t in f["c_type"] for t in GC_TYPES) else ""
            dump = "yes" if re.search(rf"\b{re.escape(f['container'])}_OFFSET\b", dump_text) else ""
            rows.append([rel, "struct_field", f["container"], f["container_kind"],
                         f["c_type"], str(f["pointer_depth"]), f["array"],
                         f["bitfield"], f["name"], f["nesting"], gc, dump]
                        + [""] * len(DECISION))
    for c in sources:
        text = c.read_text(encoding="utf-8", errors="replace")
        for g in ci.globals_(text):
            rel = str(c.relative_to(ref))
            gc = "yes" if any(t in g["c_type"] for t in GC_TYPES) else ""
            root = "yes" if g["name"] in staticpros else ""
            rows.append([rel, "global", "-", "-", g["c_type"], str(g["pointer_depth"]),
                         g["array"], "", g["name"], "-", gc,
                         "root" if root else ""] + [""] * len(DECISION))

    header = ["file", "kind", "container", "container_kind", "c_type", "pointer_depth",
              "array", "bitfield", "name", "auto:nesting", "auto:gc_managed",
              "auto:root_or_dump"] + DECISION
    key_len = len(header) - len(DECISION)
    out = Path(args.out)

    # This file is a *record*: `owner`/`freed_by`/... are filled in by the porter of each
    # file and cross-checked adversarially, and this generator is what the "Generated
    # artifacts are fresh" CI step re-runs.  Regenerating therefore has to carry the
    # decisions over, keyed by the row's own non-decision columns (the extractor may add or
    # reorder rows but does not renumber them); a decision whose row is gone is reported and
    # fails the run rather than being dropped silently.
    # The key is (the twelve non-decision columns, the row's occurrence among rows with
    # that key): rows are duplicated on purpose (one anon struct under three `#if`
    # variants of lib/ieee754.in.h is three rows with identical columns), so a decision
    # belongs to the n-th such row, not to "the first one that matches".
    carried: dict[tuple, list[str]] = {}
    occurrence: dict[tuple, int] = {}
    malformed = 0
    if out.is_file():
        for line in out.read_text(encoding="utf-8").splitlines():
            if not line.strip() or line.lstrip().startswith("#"):
                continue
            cells = line.split("\t")
            if cells[0] == "file":
                continue
            if len(cells) != len(header):
                malformed += 1
                continue
            key = (tuple(cells[:key_len]),
                   occurrence.get(tuple(cells[:key_len]), 0))
            occurrence[tuple(cells[:key_len])] = key[1] + 1
            if any(cells[key_len:]):
                carried[key] = cells[key_len:]
    flat_rows = [[flat(x) for x in row[:key_len]] for row in rows]
    seen: dict[tuple, int] = {}
    for row, key in zip(rows, flat_rows):
        k = tuple(key)
        lookup = (k, seen.get(k, 0))
        seen[k] = lookup[1] + 1
        decisions = carried.pop(lookup, None)
        if decisions is not None:
            row[key_len:] = decisions
    if malformed or carried:
        print(f"{malformed} line(s) do not have the ledger's {len(header)} columns and "
              f"{len(carried)} recorded decision(s) have no row any more:",
              file=sys.stderr)
        for key in sorted(carried):
            print("  " + "\t".join(key[0]) + f"\t[occurrence {key[1]}]", file=sys.stderr)
    with out.open("w", encoding="utf-8") as f:
        f.write("# LIFETIMES.tsv - ownership and lifetime ledger for the femacs port.\n")
        f.write("# Generated by tools/gen-lifetimes.py from the C sources via "
                "tools/cinventory.py (comments stripped, #if 0 branches skipped).\n")
        f.write("# auto:gc_managed and auto:root_or_dump are hints: check them. "
                "Fill owner and freed_by when the file is ported;\n")
        f.write("# set crosses_setjmp=yes wherever a value of the type can live "
                "across sys_setjmp/sys_longjmp (a Drop value there is forbidden);\n")
        f.write("# in_dump is layout-frozen by pdumper. A row is decided when owner "
                "and freed_by are both filled.\n")
        f.write("\t".join(header) + "\n")
        for row in rows:
            f.write("\t".join(flat(x) for x in row) + "\n")
    # The header defines a decided row as `owner` *and* `freed_by` filled (reviewer A's
    # round-9 note); this count used to look at `owner` alone.
    decided = sum(1 for r in rows if r[len(header) - len(DECISION)]
                  and r[len(header) - len(DECISION) + 1])
    bad = sum(1 for r in rows if len(r) != len(header))
    print(f"wrote {out}: {len(rows):,} rows "
          f"({sum(1 for r in rows if r[1] == 'struct_field'):,} fields, "
          f"{sum(1 for r in rows if r[1] == 'global'):,} globals), "
          f"{decided:,} decided, malformed rows {bad}")
    # A decision that cannot be carried over is a *loss*, not a warning: the porter's
    # answer for a field the extractor no longer produces has to be looked at by hand.
    if malformed or carried:
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
