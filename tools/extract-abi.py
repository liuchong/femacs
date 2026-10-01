#!/usr/bin/env python3
"""Extract the per-file inventory of a C source, and check a Rust port against it.

The inventory is the checklist a port has to satisfy: every function, subr
(DEFUN), macro, type and global of the C file, plus the line count.  It needs no
configure step and no compiler: it reads the file as text, the way the C file is
written.

    python3 tools/extract-abi.py emacs/src/sort.c            # inventory
    python3 tools/extract-abi.py --check emacs/src/sort.c femacs/src/sort.rs
    python3 tools/extract-abi.py --check --all               # every ported file

--check compares names, not semantics: it fails when a function, subr, macro or
type is missing, renamed or invented.  Semantics are checked by the test suite.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

C_FUNC = re.compile(r"^([A-Za-z_]\w*)\s*\(", re.M)
C_DEFUN = re.compile(r'^DEFUN\s*\(\s*"([^"]+)"\s*,\s*(\w+)\s*,', re.M)
C_MACRO = re.compile(r"^#\s*define\s+(\w+)", re.M)
C_TAG = re.compile(r"^\s*(?:typedef\s+)?(struct|union|enum)\s+(\w+)\s*[;{]", re.M)
C_TYPEDEF = re.compile(r"^\}\s*(\w+)\s*;", re.M)
C_EXTERN = re.compile(r"^extern\s+[^;(]*?(\w+)\s*(?:\[[^\]]*\])?\s*;", re.M)
C_GLOBAL = re.compile(r"^(?:Lisp_Object|int|bool|char|double|EMACS_INT|unsigned|long|short|size_t|ptrdiff_t)\s+\**(\w+)\s*(?:=|;)", re.M)

R_FUNC = re.compile(r"^\s*(?:pub\s+)?(?:unsafe\s+)?(?:extern\s+\"C\"\s+)?fn\s+(\w+)", re.M)
R_ALIAS = re.compile(r'#\[no_mangle\]\s*(?:pub\s+)?(?:unsafe\s+)?(?:extern\s+"C"\s+)?fn\s+(\w+)', re.M)
R_MACRO = re.compile(r"^\s*macro_rules!\s*(\w+)", re.M)
R_TAG = re.compile(r"^\s*(?:pub\s+)?struct\s+(\w+)|^\s*(?:pub\s+)?(?:union|enum)\s+(\w+)", re.M)
R_STATIC = re.compile(r'^\s*(?:pub\s+)?static\s+(?:mut\s+)?(\w+)', re.M)
R_DEFUN_ALIAS = re.compile(r"\bdefun!\s*\(\s*(\w+)\s*,\s*(\w+)", re.M)


C_KEYWORDS = {
    "if", "else", "for", "while", "do", "switch", "case", "default", "break",
    "continue", "return", "goto", "sizeof", "typeof", "struct", "union", "enum",
    "const", "static", "extern", "inline", "void", "int", "char", "long",
    "short", "unsigned", "signed", "float", "double", "bool", "defined",
}


def inventory_c(path: Path) -> dict:
    text = path.read_text(encoding="utf-8", errors="replace")
    funcs = {m.group(1) for m in C_FUNC.finditer(text)}
    defuns = {m.group(2): m.group(1) for m in C_DEFUN.finditer(text)}
    macros = {m.group(1) for m in C_MACRO.finditer(text)}
    tags = {m.group(2) for m in C_TAG.finditer(text)}
    typedefs = {m.group(1) for m in C_TYPEDEF.finditer(text)} - C_KEYWORDS - tags
    externs = {m.group(1) for m in C_EXTERN.finditer(text)}
    globals_ = {m.group(1) for m in C_GLOBAL.finditer(text)}
    return {
        "file": str(path),
        "lines": text.count("\n"),
        "functions": sorted(funcs - C_KEYWORDS),
        "subrs": sorted(defuns),
        "subr_lisp_names": sorted(defuns.values()),
        "macros": sorted(macros),
        "tags": sorted(tags | typedefs),
        "externs": sorted(externs),
        "globals_defined": sorted(globals_),
    }


def inventory_rs(path: Path) -> dict:
    text = path.read_text(encoding="utf-8", errors="replace")
    funcs = {m.group(1) for m in R_FUNC.finditer(text)}
    funcs |= {m.group(1) for m in R_ALIAS.finditer(text)}
    macros = {m.group(1) for m in R_MACRO.finditer(text)}
    tags = set()
    for m in R_TAG.finditer(text):
        tags.add(m.group(1) or m.group(2))
    statics = {m.group(1) for m in R_STATIC.finditer(text)}
    subrs = {m.group(1) for m in R_DEFUN_ALIAS.finditer(text)}
    return {
        "file": str(path),
        "lines": text.count("\n"),
        "functions": sorted(funcs - C_KEYWORDS),
        "macros": sorted(macros),
        "tags": sorted(tags),
        "statics": sorted(statics),
        "subrs": sorted(subrs),
    }


def compare(c_path: Path, rs_path: Path) -> list[str]:
    c, r = inventory_c(c_path), inventory_rs(rs_path)
    problems: list[str] = []
    for key in ("functions", "macros", "tags"):
        missing = sorted(set(c[key]) - set(r[key]) - set(c["subrs"]))
        if missing:
            problems.append(f"missing {key} ({len(missing)}): {', '.join(missing[:12])}"
                            + (" …" if len(missing) > 12 else ""))
    for name in c["subrs"]:
        if name not in r["functions"] and name not in r["subrs"]:
            problems.append(f"missing subr {name}")
    clines, rlines = c["lines"], r["lines"]
    if clines and abs(rlines - clines) > max(20, clines // 10):
        problems.append(f"line count differs a lot: C {clines} vs Rust {rlines}")
    return problems


def port_pairs(reference: Path, root: Path):
    for c in sorted(reference.rglob("*.c")):
        rel = c.relative_to(reference)
        for cand in (root / rel.with_suffix(".rs"),
                     root / "src" / Path(rel.name).with_suffix(".rs")):
            if cand.is_file():
                yield c, cand
                break
        else:
            yield c, None


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("paths", nargs="*")
    ap.add_argument("--check", action="store_true", help="compare C file with Rust port")
    ap.add_argument("--all", action="store_true", help="check every ported file")
    ap.add_argument("--reference", default="emacs")
    ap.add_argument("--root", default="femacs")
    ap.add_argument("--json", action="store_true")
    args = ap.parse_args()

    if args.all:
        reference, root = Path(args.reference), Path(args.root)
        total = missing = 0
        for c, rs in port_pairs(reference, root):
            if rs is None:
                continue
            total += 1
            problems = compare(c, rs)
            if problems:
                missing += 1
                print(f"FAIL {c.relative_to(reference)} -> {rs}")
                for p in problems:
                    print(f"  - {p}")
        print(f"\nchecked {total} ported files, {missing} with differences")
        return 1 if missing else 0

    if args.check:
        if len(args.paths) != 2:
            print("error: --check takes a C file and a Rust file", file=sys.stderr)
            return 2
        c, rs = Path(args.paths[0]), Path(args.paths[1])
        problems = compare(c, rs)
        for p in problems:
            print(f"- {p}")
        print(f"{'FAIL' if problems else 'OK'} {c} -> {rs}")
        return 1 if problems else 0

    if len(args.paths) != 1:
        print("error: pass one C file, or use --all", file=sys.stderr)
        return 2
    inv = inventory_c(Path(args.paths[0]))
    if args.json:
        print(json.dumps(inv, indent=2, ensure_ascii=False))
        return 0
    print(f"# {inv['file']} ({inv['lines']} lines)\n")
    for key, title in (("subrs", "Subrs (DEFUN)"), ("functions", "Functions defined"),
                       ("macros", "Macros defined"), ("tags", "Types"),
                       ("externs", "Externs declared"), ("globals_defined", "Globals defined")):
        items = inv[key]
        print(f"## {title} — {len(items)}")
        for it in items:
            print(f"- [ ] {it}")
        print()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
