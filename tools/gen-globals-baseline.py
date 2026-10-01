#!/usr/bin/env python3
"""Freeze the Lisp symbol table as a baseline: docs/baseline/globals/symbols.tsv.

The reference build generates src/globals.h by running make-docfile over its
inputs, which turns every DEFSYM into `#define iQfoo N` plus
`DEFINE_LISP_SYMBOL (Qfoo)`.  The port generates its own globals.rs from its own
sources (PORTING.md R7); this baseline fixes the *set* of symbols and their Lisp
names so the port can be compared against the reference.

Index assignment in the port: Qnil is 0 (the reference requires iQnil == 0, see
NIL_IS_ZERO in src/lisp.h), Qt is 1, then the remaining symbols in this file's
order.  The reference's own order is an internal detail of its build; the port's
generator owns its order and this file records the set.
"""
import re
import sys
from pathlib import Path

PAT = re.compile(r'DEFSYM\s*\(\s*(Q\w+)\s*,\s*"([^"]*)"\s*\)')


def main() -> int:
    ref, out = Path(sys.argv[1] if len(sys.argv) > 1 else "emacs"), \
        Path(sys.argv[2] if len(sys.argv) > 2 else "docs/baseline/globals/symbols.tsv")
    seen: dict[str, tuple[str, str]] = {}
    for f in sorted(list(ref.glob("src/*.c")) + list(ref.glob("src/*.h"))):
        text = f.read_text(encoding="utf-8", errors="replace")
        for m in PAT.finditer(text):
            seen.setdefault(m.group(1), (m.group(2), f.name))
    builtins = ["Qnil", "Qt"]
    for name in builtins:
        seen.setdefault(name, (name[1:].lower() if name == "Qnil" else "t", "<builtin>"))
    order = builtins + [n for n in sorted(seen) if n not in builtins]
    out.parent.mkdir(parents=True, exist_ok=True)
    with out.open("w", encoding="utf-8") as f:
        f.write("index\tq_name\tlisp_name\tfirst_defined_in\n")
        for i, n in enumerate(order):
            lisp, where = seen[n]
            f.write(f"{i}\t{n}\t{lisp}\t{where}\n")
    print(f"wrote {out}: {len(order)} symbols ({len(seen)} known)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
