#!/usr/bin/env python3
"""Compare a ported unit's symbol table with the reference object's.

The generated gnulib headers rename a replacement (`#define foo rpl_foo`) whenever the
frozen configuration replaces the system function, and a file may or may not `#undef` that
name between its declarator and its body — nanosleep.c 41 does, pipe2.c 49 does, mempcpy.c
does not.  Which side of the rename the *definition* lands on is therefore not visible in
the port's source text; the reference object's symbol table is the authority, and this check
makes it mechanical:

    python3 tools/check-symbols.py            # 0 = every built unit matches its reference

Rust's own codegen leaves global weak symbols behind (`__RNv…` monomorphizations of
`is_null`/`size_of_val` and friends).  They are an artifact of emitting one object per
translation unit rather than a difference in the C-visible interface, so they are filtered;
everything else must agree.
"""
from __future__ import annotations

import argparse
import re
import glob
import os
import re
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
RUST_NOISE = re.compile(r'^(__R|_R|_ZN)')

# gnulib's `mini-gmp-gnulib.c` is `#include "mini-gmp.c"` compiled under gnulib's allocator
# bindings, so the C build yields *one* object (`mini-gmp-gnulib.o`) and never `mini-gmp.o`.
# The port keeps the definitions in `mini-gmp.rs` (the includee) and the includer is the
# §3.25 empty unit, so the comparison has to follow the same edge: the port half of
# `lib/mini-gmp-gnulib` is `build/femacs/mini-gmp.o`.
ALIASES = {"lib/mini-gmp-gnulib": "lib/mini-gmp"}

# The port's one systematic symbol deviation from C (docs/status.md 3.29 and 3.60): C keeps a
# `DEFVAR_*` variable's storage inside the single `globals` object (`globals.f_<name>`,
# lisp.h 3495-3530), so the reference object exports no symbol for it, while the port gives
# each variable its own object - which is what makes `include/globals.rs`'s *external*
# declarations linkable, e.g. `keyboard.c` reading `Vprint_level`.  The extra symbols are
# counted and reported rather than failed on, so a reference object for a unit that owns
# variables still yields a real comparison of everything else.
DEFVAR = re.compile(r"DEFVAR_(?:LISP|BOOL|INT|LISP_NOPRO)\s*\(\s*\"[^\"]*\"\s*,\s*(\w+)")


def defvar_carriers(dir_name: str, stem: str) -> set[str]:
    """The `_V*`/`_print_*` symbols this unit's C file is expected to add over C's."""
    path = Path(ROOT) / "emacs" / dir_name / f"{stem}.c"
    if not path.is_file():
        return set()
    return {"_" + name for name in DEFVAR.findall(path.read_text(encoding="utf-8",
                                                                 errors="replace"))}


def syms(path: str) -> tuple[set[str], set[str]]:
    out = subprocess.run(["nm", "-g", path], capture_output=True, text=True).stdout
    exported, imported = set(), set()
    for line in out.splitlines():
        parts = line.split()
        if len(parts) < 2:
            continue
        kind, name = parts[-2], parts[-1]
        if RUST_NOISE.match(name):
            continue
        if kind == "U":
            imported.add(name)
        elif kind in ("T", "D", "B", "S", "C", "R"):
            exported.add(name)
    return exported, imported


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--dir", default="lib")
    ap.add_argument("--quiet", action="store_true")
    ap.add_argument("--pending", type=Path, default=None,
                    help="units whose mismatch is still open (a ratchet: an entry that has "
                         "become clean is an error, so the list can only shrink)")
    a = ap.parse_args()
    pending = set()
    if a.pending and a.pending.is_file():
        pending = {l.split("#")[0].strip() for l in a.pending.read_text(encoding="utf-8").splitlines()}
        pending.discard("")
    bad = []
    stale = []
    compared = 0
    carriers = 0
    for rs in sorted(glob.glob(f"{ROOT}/femacs/{a.dir}/*.rs")):
        stem = os.path.basename(rs)[:-3]
        mine_stem = ALIASES.get(f"{a.dir}/{stem}", f"{a.dir}/{stem}").split("/")[1]
        mine = f"{ROOT}/build/femacs/{mine_stem}.o"
        ref = f"{ROOT}/build/emacs-ref/{a.dir}/{stem}.o"
        if not (os.path.exists(mine) and os.path.exists(ref)):
            continue
        compared += 1
        me, _ = syms(mine)
        re_, _ = syms(ref)
        allowed = defvar_carriers(a.dir, stem) & me
        missing = sorted(re_ - me)
        invented = sorted(me - re_ - allowed)
        carriers += len(allowed)
        key = f"{a.dir}/{stem}"
        if missing or invented:
            if key in pending:
                continue
            bad.append((stem, missing, invented))
        elif key in pending:
            stale.append(key)
    for stem, missing, invented in bad:
        print(f"{a.dir}/{stem}: missing={missing} invented={invented}")
    for key in stale:
        print(f"{key}: listed in {a.pending} but its symbols now match — remove the entry")
    if not a.quiet:
        print(f"check-symbols: {compared} units compared, {len(bad)} mismatched, "
              f"{len(pending)} still pending"
              + (f"; {carriers} DEFVAR carrier symbol(s) are the port's documented "
                 f"deviation from C's single `globals` object (status 3.60)" if carriers else ""))
    return 1 if bad or stale else 0


if __name__ == "__main__":
    sys.exit(main())
