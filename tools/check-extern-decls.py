#!/usr/bin/env python3
"""Find `extern` declarations whose definition lives in a *unit* the declaring mirror cannot see.

The Android sweep's remaining failures are all one shape (P-262): a name declared in a mirror
(`unsafe extern "C" { pub fn X (...); }`) whose definition sits in a unit (`src/image.rs`,
`src/fringe.rs`, ...).  C allows the declaration and the definition to coexist in one
translation unit; Rust does not - the unit that contains both reports the name defined twice,
and a unit that contains only the declaration reports it missing.

This tool is the inventory that a merged-source pass needs: for every such declaration it names
the defining file, so the generator can decide what to pull in.

    python3 tools/check-extern-decls.py            # summary
    python3 tools/check-extern-decls.py --list     # one row per declaration
"""

from __future__ import annotations

import argparse
import pathlib
import re
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
PORT = REPO

DECL = re.compile(r"^\s*pub fn ([A-Za-z_][A-Za-z0-9_]*) *\(", re.MULTILINE)
DEF = re.compile(r"^(?:pub )?(?:unsafe )?(?:extern \"C(?:-unwind)?\" )?fn ([A-Za-z_][A-Za-z0-9_]*) *[(<]", re.MULTILINE)


def sources() -> list[pathlib.Path]:
    return sorted(p for p in (p for tree in ('src','lib','lib-src','lwlib','oldXMenu','nt','msdos','exec','platform','xtask') for p in (PORT/tree).rglob("*.rs")) if "target" not in p.parts)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--list", action="store_true")
    ap.add_argument("--out", type=pathlib.Path, help="write the candidate rows as a TSV")
    args = ap.parse_args()

    files = sources()
    mirrors: dict[pathlib.Path, list[str]] = {}
    definitions: dict[str, list[pathlib.Path]] = {}
    for path in files:
        text = path.read_text(encoding="utf-8", errors="surrogateescape")
        if 'extern "C' in text:
            # declarations live inside `unsafe extern "C" / "C-unwind" { ... }` blocks
            decls = []
            for block in re.findall(r'unsafe extern "C(?:-unwind)?" \{(.*?)\n\}', text, re.S):
                decls.extend(DECL.findall(block))
            if decls:
                mirrors[path] = decls
        for name in DEF.findall(text):
            definitions.setdefault(name, []).append(path)

    rows = []
    for path, decls in mirrors.items():
        for name in decls:
            where = definitions.get(name, [])
            units = [w for w in where if w.parent == PORT / "src"]
            rows.append((path, name, units))

    unbacked = [r for r in rows if not r[2]]
    cross = [r for r in rows if r[2]]
    print(f"{len(rows)} extern declaration(s) in {len(mirrors)} mirror file(s)")
    print(f"  {len(cross)} defined in a unit (the merged-source candidates)")
    print(f"  {len(unbacked)} with no unit definition (system symbols, or not yet ported)")
    lines = [
        f"{path.relative_to(REPO)}\t{name}\t{', '.join(u.relative_to(REPO).as_posix() for u in units)}"
        for path, name, units in sorted(cross, key=lambda r: (r[0].as_posix(), r[1]))
    ]
    if args.out is not None:
        args.out.write_text(
            "mirror\tdeclaration\tdefining_units\n" + "\n".join(lines) + "\n", encoding="utf-8"
        )
        print(f"wrote {args.out} ({len(lines)} row(s))")
    if args.list:
        for line in lines:
            print(f"  {line}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
