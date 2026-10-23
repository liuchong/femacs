#!/usr/bin/env python3
"""Generate and compare src/lisp.mk with the reference's own recipe.

The reference's `src/Makefile.in` 555-560 builds it with:

    printf 'shortlisp = \\\n'
    sed -n 's/^[ \t]*(load "\([^"]*\)".*/\1/p' loadup.el |
      sed -e 's/$/.elc \\/' -e 's/\.el\.elc/.el/'
    echo ""

This tool runs the same text transformations on the port's verbatim `lisp/loadup.el`,
writes `build/port/src/lisp.mk`, and compares it byte-for-byte with the reference build's
`build/emacs-ref/src/lisp.mk` when that file is present.  It also compares the checked-in
`lisp/ldefs-boot.el` pair, the other generated Lisp artifact named by the plan.

    python3 tools/gen-lisp-mk.py            # generate and compare
    python3 tools/gen-lisp-mk.py --list     # print the extracted names only
"""
from __future__ import annotations

import argparse
import pathlib
import re
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
PORT_LOADUP = REPO / "lisp/loadup.el"
OUT = REPO / "build/port/src/lisp.mk"
REF = REPO / "build/emacs-ref/src/lisp.mk"
PORT_LDEFS = REPO / "lisp/ldefs-boot.el"
REF_LDEFS = REPO / "emacs/lisp/ldefs-boot.el"
LOAD = re.compile(r'^[ \t]*\(load "([^"]*)"')


def names() -> list[str]:
    found: list[str] = []
    for line in PORT_LOADUP.read_text(encoding="utf-8", errors="surrogateescape").splitlines():
        m = LOAD.match(line)
        if m:
            found.append(m.group(1))
    return found


def render(found: list[str]) -> str:
    out = ["shortlisp = \\"]
    for name in found:
        item = name + ".elc \\"
        item = item.replace(".el.elc", ".el", 1)
        out.append(item)
    out.append("")
    return "\n".join(out) + "\n"


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--list", action="store_true", help="print the extracted load names and exit")
    args = ap.parse_args()
    if not PORT_LOADUP.is_file():
        print(f"gen-lisp-mk: missing {PORT_LOADUP.relative_to(REPO)}", file=sys.stderr)
        return 2
    found = names()
    if args.list:
        print("\n".join(found))
        return 0
    OUT.parent.mkdir(parents=True, exist_ok=True)
    text = render(found)
    with OUT.open("w", encoding="utf-8", newline="\n") as fh:
        fh.write(text)
    print(f"wrote {OUT.relative_to(REPO)} ({len(text)} bytes, {len(found)} load(s))")
    failed = 0
    if REF.is_file():
        ref = REF.read_bytes()
        got = OUT.read_bytes()
        if ref == got:
            print(f"identical    {REF.relative_to(REPO)} ({len(ref)} bytes)")
        else:
            print(f"DIFFERENT    {REF.relative_to(REPO)} ({len(ref)} bytes) vs {OUT.relative_to(REPO)} ({len(got)} bytes)")
            failed = 1
    else:
        print(f"reference    {REF.relative_to(REPO)} is absent; generated only")
    if PORT_LDEFS.is_file() and REF_LDEFS.is_file():
        got = PORT_LDEFS.read_bytes()
        ref = REF_LDEFS.read_bytes()
        if ref == got:
            print(f"identical    {PORT_LDEFS.relative_to(REPO)} ({len(ref)} bytes)")
        else:
            print(f"DIFFERENT    {REF_LDEFS.relative_to(REPO)} ({len(ref)} bytes) vs {PORT_LDEFS.relative_to(REPO)} ({len(got)} bytes)")
            failed = 1
    elif PORT_LDEFS.is_file() or REF_LDEFS.is_file():
        print(f"missing      one side of the {PORT_LDEFS.relative_to(REPO)} pair")
        failed = 1
    return failed


if __name__ == "__main__":
    raise SystemExit(main())
