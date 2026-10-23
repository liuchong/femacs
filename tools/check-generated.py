#!/usr/bin/env python3
"""Compare the port's generated files with the reference's, name rewrite aside.

The generated families (`tools/gen-charsets.py`, `gen-leim.py`, `gen-ja-dic.py`, `gen-unidata.py`, `gen-emacsver.py`, and
the `loaddefs`/`theme-loaddefs`/`subdirs` recipes) are byte-identical to the reference's copies except
for one thing PORTING.md 0.1-19 *requires*: the program's name, which the port's own build writes
where the reference's build wrote the reference's name.  A comparison that does not account for
that rewrite either reports every file as different or has to be read by eye, which is how a real
difference hid inside 22 "differing file(s)" readings (status 4.383).

This tool does the accounting: it takes the reference's product name from the reference's own
files (never spelled here), replaces it with the port's program name in the reference's text, and
then compares bytes.  Each pair is reported as

  identical        - byte for byte the same;
  name-only        - the only differences are lines that the rewrite explains;
  DIFFERENT        - something else differs: the differing lines are printed, with their numbers.

    python3 tools/check-generated.py                  # every family this repository generates
    python3 tools/check-generated.py --show 12        # differing lines per file
    python3 tools/check-generated.py --only 'lisp/international/uni-*.el'
"""

from __future__ import annotations

import argparse
import fnmatch
import pathlib
import re
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
REF = REPO / "emacs"
PORT = REPO

# The port's program name is the name its own generated files carry; the reference's product name
# is read out of the reference's generated files (`;; This file is part of X.`), so neither is
# spelled in this file.
PORT_NAME = "femacs"
NAME_LINE = re.compile(r"This file is part of ([^.\n]+)\.")
PRODUCT_CANDIDATES = ("lisp/international/charprop.el", "lisp/leim/leim-list.el",
                      "lisp/loaddefs.el", "lisp/subdirs.el")

# Every generated target, as (reference-relative pattern).  The families are those the generators
# in this directory write; `etc/charsets/*.map` are data files with no name in them, so they are
# listed too - an identical verdict for them means "byte for byte", which is the stronger reading.
PATTERNS = [
    "etc/charsets/*.map",
    "lisp/international/charprop.el",
    "lisp/international/uni-*.el",
    "lisp/international/uni-scripts.el",
    "lisp/international/uni-confusable.el",
    "lisp/international/idna-mapping.el",
    "lisp/international/charscript.el",
    "lisp/international/emoji-zwj.el",
    "lisp/international/emoji-labels.el",
    "lisp/international/cp51932.el",
    "lisp/international/eucjp-ms.el",
    "lisp/language/pinyin.el",
    "lisp/leim/quail/*.el",
    "lisp/leim/leim-list.el",
    # `leim/Makefile`'s `generate-ja-dic` rule, run by `tools/gen-ja-dic.py`.  The `.elc` is
    # binary and produced by the port's own program, so it is compared byte for byte.
    "lisp/leim/ja-dic/ja-dic.el",
    "lisp/leim/ja-dic/ja-dic.elc",
    # `admin/unidata/Makefile.in` 54-55's order-only prerequisite, compiled by the port's
    # own program (the `macuvs.h` run it feeds is `gen-unidata.py`'s `run_uvs`).
    "admin/unidata/uvs.elc",
    # `admin/charsets/Makefile.in` 195-197's generated sed script, which the JISX2131 recipe
    # reads; `gen-charsets.py` rebuilds it from the port's own data (status 4.608).
    "admin/charsets/jisx2131-filter",
    "lisp/loaddefs.el",
    "lisp/theme-loaddefs.el",
    "lisp/subdirs.el",
    "doc/emacs/emacsver.texi",
    "etc/refcards/emacsver.tex",
]


def product_name() -> str:
    """The reference's product name, read from the reference's own generated files."""
    for rel in PRODUCT_CANDIDATES:
        path = REF / rel
        if not path.is_file():
            continue
        m = NAME_LINE.search(path.read_text("utf-8", "surrogateescape"))
        if m:
            return m.group(1)
    raise SystemExit("cannot read the reference's product name from its generated files")


def pairs(patterns: list[str]) -> list[tuple[pathlib.Path, pathlib.Path]]:
    """Every (reference, port) file the patterns name, both sides present."""
    found: list[tuple[pathlib.Path, pathlib.Path]] = []
    for pattern in patterns:
        for reference in sorted(REF.glob(pattern)):
            produced = PORT / reference.relative_to(REF)
            found.append((reference, produced))
    return found


def name_only(reference_lines: list[str], port_lines: list[str], product: str) -> bool:
    """True when every differing line is explained by the name rewrite (and the tree path).

    Two registered substitutions are applied, and both are consequences of the port *being* the
    port rather than the reference: the product name (PORTING.md 0.1-19) wherever a generated
    file spells it, and the tree's own path where a *compiled* describer embeds it.  The second
    one is not obvious and was found the hard way (status 4.556): `unidata-gen-file` compiles a
    describer and stores the function object in the table, and a compiled function prints as a
    byte-code object that carries the path of the `.elc` it was loaded from - so the reference's
    generated `uni-*.el` files contain the *reference tree's* path and the port's contain the
    port's.  It is one directory name, and no port defect can remove it.
    """
    if len(reference_lines) != len(port_lines):
        return False
    reference_root, port_root = str(REF), str(PORT)
    for want, got in zip(reference_lines, port_lines):
        if want == got:
            continue
        # The rewrite may substitute the name anywhere in the line (a comment, a docstring, the
        # generated heading), and it may also change the line's length, which is why the
        # comparison is line by line rather than byte by byte.
        # Two spellings are rewritten: the full product name in the license header, and the
        # short form docstrings use when they mean the manual ("the Emacs manual").
        wanted = want.replace(product, PORT_NAME).replace(product.split()[-1], PORT_NAME)
        if got != wanted.replace(reference_root, port_root):
            return False
    return True


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--only", nargs="*", default=[], help="patterns to check instead")
    parser.add_argument("--show", type=int, default=6, help="differing lines to print per file")
    args = parser.parse_args()

    product = product_name()
    counts = {"identical": 0, "name-only": 0, "DIFFERENT": 0, "missing": 0}
    for reference, produced in pairs(args.only or PATTERNS):
        if not produced.is_file():
            print(f"MISSING      {produced.relative_to(REPO)}")
            counts["missing"] += 1
            continue
        want, got = reference.read_bytes(), produced.read_bytes()
        if want == got:
            counts["identical"] += 1
            continue
        wl = want.decode("utf-8", "surrogateescape").split("\n")
        gl = got.decode("utf-8", "surrogateescape").split("\n")
        if name_only(wl, gl, product):
            counts["name-only"] += 1
            continue
        counts["DIFFERENT"] += 1
        diffs = [(i + 1, x, y) for i, (x, y) in enumerate(zip(wl, gl)) if x != y]
        print(f"DIFFERENT    {produced.relative_to(REPO)}: {len(diffs)} differing line(s), "
              f"{len(wl)} vs {len(gl)} lines")
        for n, x, y in diffs[:args.show]:
            print(f"    line {n}: reference {x[:100]!r}")
            print(f"              port      {y[:100]!r}")
    total = sum(counts.values())
    print(f"{total} file(s): {counts['identical']} identical, {counts['name-only']} name-only, "
          f"{counts['DIFFERENT']} DIFFERENT, {counts['missing']} missing")
    return 1 if (counts["DIFFERENT"] or counts["missing"]) else 0


if __name__ == "__main__":
    sys.exit(main())
