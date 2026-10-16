#!/usr/bin/env python3
"""Give the Swift units the docstring carriers P-106 asks for (and P-110's shape).

The reference carries a DEFUN's docstring as an argument of its own invocation, and
`make-docfile` reads it from there; the port's Rust units keep that shape (`doc: "…"`), and
the Rust side is already complete.  The Swift units register their subrs through
`make_aligned_subr (…)` and their variables through `defvar_lisp`/`defvar_bool`, and those
calls carry no docstring at all - so `etc/DOC` can have no record for `nsfns.o`,
`nsterm.o`, `nsmenu.o`, `nsselect.o` or `nsfont.o` (96 records).

This tool reads each unit's own `.m` (the file the unit was ported from), takes C's
docstring text the way `make-docfile` reads it (PORTING.md P-109) and C's argument names,
and appends

    docstring: "…", args: "PARMS"        to a `make_aligned_subr` call
    docstring: "…"                       to a `defvar_*` call

keeping every other byte of the call.  It also widens the definitions of those helpers, which
accept the carriers and ignore them - exactly as C's macro does with its docstring argument.
`tools/gen-doc.py` reads them back and renders C's record.

Usage
-----
    python3 tools/add-swift-doc.py            # dry run
    python3 tools/add-swift-doc.py --write
"""

from __future__ import annotations

import argparse
import importlib.util
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent


def load_gen_doc():
    spec = importlib.util.spec_from_file_location("gen_doc", REPO / "tools/gen-doc.py")
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


gd = load_gen_doc()


def rust_literal(text: str) -> str:
    out = ['"']
    for ch in text:
        if ch == "\\":
            out.append("\\\\")
        elif ch == '"':
            out.append('\\"')
        elif ch == "\n":
            out.append("\\n")
        elif ch == "\t":
            out.append("\\t")
        else:
            out.append(ch)
    out.append('"')
    return "".join(out)


def closing_paren(text: str, open_at: int) -> int:
    """Index of the `)` that closes the `(` at `open_at`, string literals skipped."""
    depth = 0
    i = open_at
    while i < len(text):
        c = text[i]
        if c == '"':
            _, i = gd.string_literal(text, i)
            continue
        if c == "(":
            depth += 1
        elif c == ")":
            depth -= 1
            if depth == 0:
                return i
        i += 1
    raise ValueError("unbalanced parenthesis")


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--write", action="store_true")
    ap.add_argument("--limit", type=int, default=40)
    a = ap.parse_args()

    ref = gd.DEFAULT_REFERENCE.read_bytes()
    reference = {(o, n): d for o, recs in gd.split_doc(ref) for _, n, d in recs}
    total = 0
    skipped: list[str] = []
    for stem in sorted(gd.SWIFT_STEMS):
        swift = gd.SWIFT_DIR / f"{stem}.swift"
        if not swift.exists():
            continue
        mfile = gd.REFERENCE_SRC / f"{stem}.m"
        if not mfile.exists():
            skipped.append(f"{stem}: no reference source {mfile.name}")
            continue
        text = swift.read_text(encoding="utf-8")
        names = dict(re.findall(r"fileprivate let (lname_[A-Za-z_0-9]+): StaticString =\s*\"([^\"]*)\"",
                                text, re.S))
        edits: list[tuple[int, str]] = []          # (insert position, text)

        # subr registrations
        for m in re.finditer(r"make_aligned_subr\(lname_([A-Za-z_0-9]+),\s*(\d+),\s*(\d+)", text):
            lname = names.get(f"lname_{m.group(1)}")
            if not lname:
                skipped.append(f"{stem}.swift: lname_{m.group(1)} has no literal")
                continue
            got = gd.read_c_docstring(mfile, lname, "F", args_wanted=True)
            if got is None:
                skipped.append(f"{stem}.swift {lname}: no DEFUN in {mfile.name}")
                continue
            doc, cline, args = got
            minargs, maxargs = int(m.group(2)), int(m.group(3))
            want = reference.get((f"{stem}.o", lname))
            probe = gd.Record("F", lname, doc, minargs, maxargs, [gd.c_arg_name(a) for a in args],
                              f"{mfile.name}:{cline}")
            if want is None:
                skipped.append(f"{stem}.swift {lname}: no record in the reference DOC")
                continue
            if gd.render_doc(probe, []) != want:
                skipped.append(f"{stem}.swift {lname}: reference text does not render to the record")
                continue
            close = closing_paren(text, text.index("(", m.start()))
            if "docstring:" in text[m.start():close]:
                continue
            edits.append((close, f", docstring: {rust_literal(doc)}, args: {rust_literal(' '.join(args))}"))

        # variable registrations
        for m in re.finditer(r"\bdefvar_(lisp|bool|int|kboard)\(([^;]*?)\)\s*\n", text):
            inside = m.group(2)
            name_m = re.search(r'"([^"]*)"', inside)
            if not name_m:
                continue
            name = name_m.group(1)
            got = gd.read_c_docstring(mfile, name, "V")
            if got is None:
                skipped.append(f"{stem}.swift {name}: no DEFVAR_* in {mfile.name}")
                continue
            doc, cline, _ = got
            want = reference.get((f"{stem}.o", name))
            if want is None or gd.render_doc(gd.Record("V", name, doc), []) != want:
                skipped.append(f"{stem}.swift {name}: reference text does not render to the record")
                continue
            close = m.end(2)
            if "docstring:" in inside:
                continue
            edits.append((close, f", docstring: {rust_literal(doc)}"))

        # the helper definitions take the carriers and ignore them
        defs = 0
        for m in re.finditer(r"fileprivate func make_aligned_subr\(([^)]*)\)", text):
            sig = m.group(1)
            if "docstring:" in sig:
                continue
            edits.append((m.end(1), ',\n                                   docstring: StaticString, args: StaticString'))
            defs += 1
        for m in re.finditer(r"func defvar_(lisp|bool|int|kboard)\(([^)]*)\)", text):
            sig = m.group(2)
            if "docstring:" in sig:
                continue
            edits.append((m.end(2), ', docstring: StaticString'))
            defs += 1

        if not edits:
            continue
        out = text
        for pos, ins in sorted(edits, reverse=True):
            out = out[:pos] + ins + out[pos:]
        if a.write:
            swift.write_text(out, encoding="utf-8")
        total += len(edits)
        print(f"  {swift.name}: {len(edits)} carrier edit(s), {defs} helper definition(s)"
              f"{' [written]' if a.write else ''}")

    print(f"swift carrier edits: {total}; {'written' if a.write else 'dry run'}")
    for s in skipped[:a.limit]:
        print(f"  skip: {s}")
    if len(skipped) > a.limit:
        print(f"  … {len(skipped) - a.limit} more skipped")
    return 0


if __name__ == "__main__":
    sys.exit(main())
