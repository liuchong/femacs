#!/usr/bin/env python3
"""Fail when a `DEFVAR_*` carrier's declared type does not match how the macro dereferences it.

The port's `DEFVAR_INT!`/`DEFVAR_BOOL!`/`DEFVAR_LISP!` hand `defvar_int`/`defvar_bool`/`defvar_lisp`
the *address* of the variable, cast to the pointee C uses (`*mut i64`, `*mut bool`,
`*mut Lisp_Object`).  C's own text takes the address of a field of `globals`, which has the right
type by construction; where the port lets a unit own the object instead (status 3.29's deviation),
the *declaration* has to agree - status 3.638's `process_error_pause_time: c_int` was 4-byte aligned
and the 8-byte read panicked with `misaligned pointer dereference`.  PORTING.md P-97 is the rule.

    python3 tools/check-defvar-carriers.py            # every unit under femacs/src
    python3 tools/check-defvar-carriers.py --selftest # prove the reading fires (the 3.638 case)

For each site the variable's declaration is looked up in the unit itself and then in
`femacs/src/include/*.rs`; a declaration that is *not* found is counted as unchecked (the name may be
a macro or a field), while a found declaration whose type is not one of the C-equivalent spellings is
a failure.
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
SRC = REPO / "femacs/src"
MIRRORS = SRC / "include"
# macro -> (C's forward kind, the spellings that have C's size and alignment)
MACROS = {
    "DEFVAR_INT": ("Lisp_Fwd_Int", {"EMACS_INT", "intmax_t", "i64", "isize", "ptrdiff_t"}),
    "DEFVAR_BOOL": ("Lisp_Fwd_Bool", {"bool", "c_bool"}),
    "DEFVAR_LISP": ("Lisp_Fwd_Obj", {"Lisp_Object"}),
}
SITE = re.compile(r"\b(DEFVAR_INT|DEFVAR_BOOL|DEFVAR_LISP)!\s*\(\s*[^,]+,\s*([A-Za-z_][A-Za-z_0-9]*)")
DECL = re.compile(r"(?:^|\n)\s*(?:pub\s+)?static\s+mut\s+([A-Za-z_][A-Za-z_0-9]*)\s*:\s*([^;=]+)")


def declarations(text: str) -> dict[str, str]:
    out = {}
    for name, typ in DECL.findall(text):
        out.setdefault(name, typ.strip())
    return out


def last_segment(typ: str) -> str:
    typ = typ.split("<")[0]
    return typ.split("::")[-1].strip()


def check_text(text: str, lookup: dict[str, str]) -> tuple[int, int, list[str]]:
    sites = found = 0
    bad: list[str] = []
    for macro, var in SITE.findall(text):
        sites += 1
        typ = lookup.get(var)
        if typ is None:
            continue
        found += 1
        allowed = MACROS[macro][1]
        if last_segment(typ) not in allowed:
            bad.append(f"{macro}! uses `{var}: {typ}` - {MACROS[macro][0]} reads it as "
                       f"{'/'.join(sorted(allowed))} (PORTING.md P-97)")
    return sites, found, bad


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--selftest", action="store_true")
    args = ap.parse_args()
    if args.selftest:
        cases = [
            ("DEFVAR_INT! (c\"x\", carrier, doc);", {"carrier": "c_int"}, 1),
            ("DEFVAR_INT! (c\"x\", carrier, doc);", {"carrier": "EMACS_INT"}, 0),
            ("DEFVAR_BOOL! (c\"x\", flag, doc);", {"flag": "bool"}, 0),
            ("DEFVAR_BOOL! (c\"x\", flag, doc);", {"flag": "c_int"}, 1),
            ("DEFVAR_LISP! (c\"x\", obj, doc);", {"obj": "Lisp_Object"}, 0),
            ("DEFVAR_LISP! (c\"x\", obj, doc);", {"obj": "EMACS_INT"}, 1),
            ("DEFVAR_INT! (c\"x\", unknown, doc);", {}, 0),
        ]
        bad = 0
        for text, lookup, want in cases:
            _s, _f, found = check_text(text, lookup)
            if len(found) != want:
                print(f"FAIL  {text!r} / {lookup}: {len(found)} hit(s), want {want}")
                bad += 1
        print(f"selftest: {len(cases)} case(s), {bad} failure(s)")
        return 1 if bad else 0

    mirror_decls: dict[str, str] = {}
    for m in sorted(MIRRORS.glob("*.rs")):
        for name, typ in declarations(m.read_text(encoding="utf-8", errors="replace")).items():
            mirror_decls.setdefault(name, typ)

    total_sites = total_found = 0
    failures: list[str] = []
    files = 0
    for p in sorted(SRC.rglob("*.rs")):
        if p.parent == MIRRORS:
            continue
        files += 1
        text = p.read_text(encoding="utf-8", errors="replace")
        if "DEFVAR_INT!" not in text and "DEFVAR_BOOL!" not in text and "DEFVAR_LISP!" not in text:
            continue
        lookup = dict(mirror_decls)
        lookup.update(declarations(text))
        sites, found, bad = check_text(text, lookup)
        total_sites += sites
        total_found += found
        failures += [f"{p.relative_to(REPO)}: {b}" for b in bad]
    for f in failures:
        print(f)
    print(f"read {files} unit file(s): {total_sites} DEFVAR_* site(s), {total_found} with a "
          f"declaration whose type is checked, {len(failures)} failure(s)")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
