#!/usr/bin/env python3
"""Write the static reference baselines under docs/baseline/.

Everything comes from `tools/cinventory.py`, so the numbers here are the same
ones `tools/extract-abi.py --check` uses.  No compiler, no build.

    docs/baseline/inventory.tsv   per-file counts for every C source of the port
    docs/baseline/subrs.tsv       every live DEFUN with its signature and doc digest
    docs/baseline/totals.tsv      repo-wide totals (occurrences and unique names)

    python3 tools/gen-baseline.py [--reference emacs] [--out docs/baseline]
"""

from __future__ import annotations

import argparse
import importlib.util
from collections import Counter
from pathlib import Path

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("cinventory", HERE / "cinventory.py")
ci = importlib.util.module_from_spec(spec)
spec.loader.exec_module(ci)

C_DIRS = ("src", "lib", "lib-src", "lwlib", "oldXMenu", "exec", "nt", "msdos")


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--reference", default="emacs")
    ap.add_argument("--out", default="docs/baseline")
    args = ap.parse_args()
    ref, out = Path(args.reference), Path(args.out)
    out.mkdir(parents=True, exist_ok=True)

    rows, subrs = [], []
    counters = Counter()
    for d in C_DIRS:
        for path in sorted((ref / d).rglob("*")):
            if not path.is_file() or path.suffix not in (".c", ".h"):
                continue
            inv = ci.inventory(path)
            rel = str(path.relative_to(ref))
            rows.append((rel, inv["lines"], len(inv["functions"]), len(inv["defuns"]),
                         len(inv["macros"]), len(inv["constants"]),
                         len(inv["struct_fields"]), len(inv["globals"])))
            counters["functions"] += len(inv["functions"])
            counters["defuns"] += len(inv["defuns"])
            counters["macros"] += len(inv["macros"])
            counters["constants"] += len(inv["constants"])
            counters["struct_fields"] += len(inv["struct_fields"])
            counters["globals"] += len(inv["globals"])
            counters["lines"] += inv["lines"]
            counters["files"] += 1
            for d_ in inv["defuns"]:
                subrs.append((rel, d_["c_name"], d_["lisp_name"], d_["min"], d_["max"],
                              d_["n"], d_["doc_digest"]))

    with (out / "inventory.tsv").open("w", encoding="utf-8") as f:
        f.write("file\tlines\tfunctions\tsubrs\tmacros\tconstants\tstruct_fields\tglobals\n")
        for r in rows:
            f.write("\t".join(str(x) for x in r) + "\n")
    with (out / "subrs.tsv").open("w", encoding="utf-8") as f:
        f.write("file\tc_name\tlisp_name\tmin_args\tmax_args\tn\tldoc_digest\n")
        for r in subrs:
            f.write("\t".join(r) + "\n")
    with (out / "totals.tsv").open("w", encoding="utf-8") as f:
        f.write("metric\tvalue\n")
        for key in ("files", "lines", "functions", "defuns", "macros", "constants",
                    "struct_fields", "globals"):
            f.write(f"{key}\t{counters[key]}\n")
        f.write(f"unique_defun_c_names\t{len({r[1] for r in subrs})}\n")
        f.write(f"unique_defun_lisp_names\t{len({r[2] for r in subrs})}\n")
    for key in ("files", "lines", "functions", "defuns", "macros", "constants",
                "struct_fields", "globals"):
        print(f"{key:14s} {counters[key]:,}")
    print(f"unique c names {len({r[1] for r in subrs}):,}; "
          f"wrote {out}/inventory.tsv, subrs.tsv, totals.tsv")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
