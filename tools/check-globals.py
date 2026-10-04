#!/usr/bin/env python3
"""Every `DEFVAR_*` call site must have a definition of its variable in the same unit.

C's `DEFVAR_LISP`/`DEFVAR_BOOL`/`DEFVAR_INT`/`DEFVAR_KBOARD` (lisp.h 3495-3530) take the
*address* of `globals.f_<vname>`, and `struct emacs_globals globals` is defined once, in
`src/alloc.c`.  The port's `include/globals.rs` mirrors the names as `extern` declarations,
so the unit that owns one has to define it as well — otherwise the port has an object that is
referenced (through its `extern` declaration) and never defined, which no textual gate can see:
`extract-abi.py`'s "File-scope objects" section reads C's own object definitions, and
`DEFVAR_*`'s storage is not one of those (`src/cmds.c`'s checklist says "File-scope objects —
0" while the file owns `Vpost_self_insert_hook`).  The rule is status §3.29.

    python3 tools/check-globals.py            # 0 = every call site has a definition
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent

# The macrotag → the Rust type the port gives its storage (the C type of the field in
# `struct emacs_globals`, which mir.../globals.h generates).
KINDS = {"DEFVAR_LISP": "Lisp_Object", "DEFVAR_LISP_NOPRO": "Lisp_Object",
         "DEFVAR_BOOL": "bool", "DEFVAR_INT": "EMACS_INT"}
# `DEFVAR_KBOARD` names a *field of struct kboard*, not an object of its own
# (`DEFVAR_KBOARD ("last-kbd-macro", Vlast_kbd_macro)` is `offsetof (struct kboard, …)`), so
# the port keeps the offset form and there is nothing to define.
SKIP = {"DEFVAR_KBOARD", "DEFVAR_PER_BUFFER"}

CALL = re.compile(r'(DEFVAR_[A-Z_]+)\s*!\s*\((.*?)\)\s*;', re.S)
IDENT = re.compile(r'^[A-Za-z_][A-Za-z0-9_]*$')
DEFINITION = re.compile(r'static mut ([A-Za-z_][A-Za-z0-9_]*)\s*:[^;=]*=\s*[^;]+;')


def units() -> list[Path]:
    """Every ported unit under `femacs/src` and `femacs/lib`, mirrors excluded.

    The walk is recursive because a ported tree is not flat any more: the first gnulib
    library that lives in a subdirectory of the reference (`lib/malloc/`) put its units in
    `femacs/lib/malloc/`, where a single-level glob could not see them (status 3.121).  A
    path with an `include` component is a mirror, and a mirror is reached through the units
    that include it, so it is not a unit of its own.
    """
    out = []
    for d in ("src", "lib"):
        base = ROOT / "femacs" / d
        out += sorted(p for p in base.rglob("*.rs") if "include" not in p.relative_to(base).parts)
    return out


def check(path: Path) -> list[str]:
    text = path.read_text(encoding="utf-8", errors="replace")
    defined = set(DEFINITION.findall(text))
    problems = []
    for kind, args in CALL.findall(text):
        if kind in SKIP:
            continue
        if kind not in KINDS:
            problems.append(f"{path.name}: unknown macro {kind}")
            continue
        # The second argument is the variable (`&globals.f_<vname>` in C's macro); the third
        # is the docstring, whose prose contains words that are not declarations.
        fields = args.split(",")
        if len(fields) < 2:
            continue
        name = fields[1].strip()
        if not IDENT.match(name):
            continue                      # `DEFVAR_KBOARD`'s offset argument, or an expression
        if name not in defined:
            problems.append(f"{path}: {kind} names {name}, which this unit never defines")
    return problems


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--quiet", action="store_true")
    a = ap.parse_args()
    problems = []
    checked = 0
    for unit in units():
        if not unit.is_file():
            continue
        checked += 1
        problems += check(unit)
    for p in problems:
        print(p)
    if not a.quiet:
        print(f"check-globals: {checked} units, {len(problems)} DEFVAR_* variables without a "
              f"definition in their own unit")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
