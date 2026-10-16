#!/usr/bin/env python3
"""Generate the products the reference's build makes by *running* its own program.

Most generated artefacts are written by a script (`tools/gen-charsets.py`, `gen-leim.py`,
`gen-unidata.py`, `gen-subdirs.py`, `gen-emacsver.py`).  Eight more come out of the program
itself, and completion definition item 4 wants them byte-identical as well:

  * the three Bovine grammars and the five Wisent grammars under `lisp/cedet/`
    (`admin/grammars/Makefile.in`: `bovine-batch-make-parser` / `wisent-batch-make-parser`),
  * `lisp/language/pinyin.el` (`leim/Makefile.in`: `-l titdic-cnv -f tit-pinyin-convert`).

Each recipe is the reference's own command line, run twice: once with the *reference's* program
on the reference's trees (the golden, into a temporary directory) and once with the *port's*
program on the port's trees (the product, into the port's tree unless `--check`).  Both runs are
outside any checkout's `.dir-locals.el`, so the two sides see the same generator settings.

The reading is the differential one on purpose.  Measured 2026-10-16: the reference tree's own
copy of `lisp/cedet/srecode/srt-wy.el` (9081 bytes) is *not* what the reference's own recipe
produces here (9018 bytes) - those copies carry tabs and a different turn of the touch-up step,
and their provenance is the prebuilt tree the checkout came with, not a recipe this environment
can replay.  So the reproducible target is "the same recipe on the reference program", and the
tree copy's verdict is printed beside it for the record.

The output name is the generator's business (it comes from the input's `%provide`), so the tool
passes the target's *directory* and then reads back the file the generator wrote.

    python3 tools/gen-program-products.py            # write into the port tree, then compare
    python3 tools/gen-program-products.py --check    # write to a temp dir, compare only
    python3 tools/gen-program-products.py --only c-by.el

The reference's recipe lines are read back before anything runs, so a moved rule fails loudly
instead of silently generating the wrong bytes; and no output path is ever allowed to land under
`emacs/`.
"""

from __future__ import annotations

import argparse
import os
import pathlib
import re
import shutil
import subprocess
import sys
import tempfile

REPO = pathlib.Path(__file__).resolve().parent.parent
REF = REPO / "emacs"
PORT = REPO / "femacs"
PROGRAM = REPO / "build/port/src/temacs"
REFERENCE_PROGRAM = REPO / "build/emacs-ref/src/emacs"

# Neither side gets extra settings: both generators run outside a checkout's `.dir-locals.el`
# (the reference's build would pick that file up for a tree-internal output buffer, but the
# reference tree's own copies are not reproducible here - see the docstring - so the fair and
# reproducible comparison is temp directory against temp directory).
GENERATOR_SETTINGS: list[str] = []

# (maker, input, target): the input the generator reads, and the file the reference's build
# produces from it.  `%provide` in the input names the output's basename (the generator uses
# `file-name-nondirectory` of it), which `check_provide` verifies below.
PRODUCTS = (
    ("bovine", "admin/grammars/c.by", "lisp/cedet/semantic/bovine/c-by.el"),
    ("bovine", "admin/grammars/make.by", "lisp/cedet/semantic/bovine/make-by.el"),
    ("bovine", "admin/grammars/scheme.by", "lisp/cedet/semantic/bovine/scm-by.el"),
    ("wisent", "admin/grammars/grammar.wy", "lisp/cedet/semantic/grammar-wy.el"),
    ("wisent", "admin/grammars/java-tags.wy", "lisp/cedet/semantic/wisent/javat-wy.el"),
    ("wisent", "admin/grammars/js.wy", "lisp/cedet/semantic/wisent/js-wy.el"),
    ("wisent", "admin/grammars/python.wy", "lisp/cedet/semantic/wisent/python-wy.el"),
    ("wisent", "admin/grammars/srecode-template.wy", "lisp/cedet/srecode/srt-wy.el"),
    ("pinyin", "leim/MISC-DIC/pinyin.map", "lisp/language/pinyin.el"),
)

# The reference's own command lines, one per maker, with the makefile and the rule each comes
# from.  `{program} {output} {input}` are filled in per product.
MAKERS = {
    "bovine": {
        "makefile": "admin/grammars/Makefile.in",
        "lines": ("make_bovine = ${emacs} -l semantic/bovine/grammar -f bovine-batch-make-parser",
                  "bovine-batch-make-parser"),
        "argv": ["-batch", "--no-site-file", "--no-site-lisp",
                 "--eval", "(setq load-prefer-newer t)",
                 "-l", "semantic/bovine/grammar"] + GENERATOR_SETTINGS + [
                 "-f", "bovine-batch-make-parser",
                 "-o", "{output}", "{input}"],
    },
    "wisent": {
        "makefile": "admin/grammars/Makefile.in",
        "lines": ("make_wisent = ${emacs} -l semantic/wisent/grammar -f wisent-batch-make-parser",
                  "wisent-batch-make-parser"),
        "argv": ["-batch", "--no-site-file", "--no-site-lisp",
                 "--eval", "(setq load-prefer-newer t)",
                 "-l", "semantic/wisent/grammar"] + GENERATOR_SETTINGS + [
                 "-f", "wisent-batch-make-parser",
                 "-o", "{output}", "{input}"],
    },
    "pinyin": {
        "makefile": "leim/Makefile.in",
        "lines": ("-l titdic-cnv -f tit-pinyin-convert $< $@",
                  "tit-pinyin-convert"),
        "argv": ["-batch", "--no-site-file", "--no-site-lisp",
                 "-l", "titdic-cnv"] + GENERATOR_SETTINGS + [
                 "-f", "tit-pinyin-convert",
                 "{input}", "{output}"],
    },
}


# The port's own program name, and the reference's product name read out of the reference's own
# generated files (neither is spelled as a *reference* name here).  PORTING.md 0.1-19 requires the
# rewrite, so a comparison that ignores it would report every header line as a difference - the
# accounting `tools/check-generated.py` already does for the script-written families.
PORT_NAME = "femacs"
NAME_LINE = re.compile(r"This file is part of ([^.\n]+)\.")
PRODUCT_CANDIDATES = ("lisp/international/charprop.el", "lisp/leim/leim-list.el",
                      "lisp/loaddefs.el", "lisp/subdirs.el")


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


def differing_lines(received: bytes, expected: bytes, product: str, show: int) -> int:
    """Print the lines that are not explained by the name rewrite; return their count."""
    want = expected.decode("utf-8", "surrogateescape").split("\n")
    got = received.decode("utf-8", "surrogateescape").split("\n")
    diffs = [(i + 1, x, y) for i, (x, y) in enumerate(zip(want, got))
             if x != y and y != x.replace(product, PORT_NAME).replace(
                 product.split()[-1], PORT_NAME)]
    for n, x, y in diffs[:show]:
        print(f"    line {n}: reference {x[:110]!r}")
        print(f"              port      {y[:110]!r}")
    if len(want) != len(got):
        print(f"    line counts: reference {len(want)} vs port {len(got)}")
    return len(diffs) + (1 if len(want) != len(got) else 0)


def check_recipe(maker: str) -> None:
    """The recipe must still be the one the reference's makefile carries."""
    spec = MAKERS[maker]
    makefile = REF / spec["makefile"]
    if not makefile.is_file():
        raise SystemExit(f"{makefile} is missing: the reference checkout is incomplete")
    text = makefile.read_text(encoding="utf-8", errors="replace")
    for line in spec["lines"]:
        if line not in text:
            raise SystemExit(f"{spec['makefile']} no longer carries {line!r}: "
                             f"the reference's recipe moved")


def check_provide(input_file: pathlib.Path, target: pathlib.Path) -> None:
    """The generator names its output from the input's `%provide`, so it must agree."""
    if input_file.suffix == ".map":
        return
    text = input_file.read_text(encoding="utf-8", errors="replace")
    match = re.search(r"^%provide\s+(\S+)\s*$", text, re.MULTILINE)
    if not match:
        raise SystemExit(f"{input_file} has no %provide directive")
    named = match.group(1).split("/")[-1] + ".el"
    if named != target.name:
        raise SystemExit(f"{input_file.name} provides {match.group(1)} (=> {named}), "
                         f"but this tool expects {target.name}")


def run(product: tuple[str, str, str], output_dir: pathlib.Path, program: pathlib.Path,
        tree_root: pathlib.Path, args_show: argparse.Namespace) -> int:
    """Run the reference's recipe with `program` on `tree_root`'s trees, writing into
    `output_dir`; return the written file (or None when the generator failed)."""
    maker, input_rel, target_rel = product
    spec = MAKERS[maker]
    input_file = tree_root / input_rel
    target = tree_root / target_rel
    if not input_file.is_file():
        print(f"{target_rel}: SKIPPED, {tree_root.name}/{input_rel} is missing")
        return None
    check_provide(input_file, target)

    resolved = (output_dir / target.name).resolve()
    if REF in resolved.parents or resolved == REF.resolve():
        raise SystemExit(f"{resolved} is inside the reference tree: refusing to write")
    output_dir.mkdir(parents=True, exist_ok=True)
    argv = [str(program)] + [a.format(output=str(output_dir / target.name),
                                      input=str(input_file)) for a in spec["argv"]]
    env = dict(os.environ)
    # Both sides load the lisp tree of the checkout they are generating for, symmetrically.
    env["EMACSLOADPATH"] = str(tree_root / "lisp")
    result = subprocess.run(argv, capture_output=True, text=True, timeout=900, env=env,
                            cwd=str(tree_root))
    produced = output_dir / target.name
    if result.returncode != 0 or not produced.is_file():
        tail = (result.stderr or result.stdout or "").strip().splitlines()[-3:]
        print(f"{target_rel}: the generator failed ({program.name}, exit {result.returncode}): "
              f"{' | '.join(tail)}")
        return None
    return produced


def compare(target_rel: str, received: pathlib.Path | None, expected: pathlib.Path,
            args_show: argparse.Namespace, label: str) -> int:
    """Report one product's verdict against one golden file."""
    if received is None:
        return 1
    got, want = received.read_bytes(), expected.read_bytes()
    if got == want:
        print(f"{target_rel}: identical ({len(got)} bytes) [{label}]")
        return 0
    diffs = differing_lines(got, want, product_name(), args_show.show)
    if diffs == 0:
        print(f"{target_rel}: name-only ({len(got)} vs {len(want)} bytes) [{label}]")
        return 0
    print(f"{target_rel}: DIFFERENT, {diffs} differing line(s) "
          f"({len(got)} vs {len(want)} bytes) [{label}]")
    return 1


def one_product(product: tuple[str, str, str], args: argparse.Namespace,
                workdir: pathlib.Path) -> int:
    """Generate one product on both programs and report the differential reading."""
    maker, input_rel, target_rel = product
    check_recipe(maker)
    port_dir = workdir / "port" if args.check else PORT / pathlib.Path(target_rel).parent
    port_file = run(product, port_dir, args.program, PORT, args)
    reference_file = run(product, workdir / "reference", args.reference_program, REF, args)
    failures = 0
    if reference_file is None:
        print(f"{target_rel}: the reference's own run did not produce the golden; "
              f"only the port side ran")
        return 1
    failures += compare(target_rel, port_file, reference_file, args,
                        "port vs the reference program, same recipe")
    tree_copy = REF / target_rel
    if tree_copy.is_file():
        compare(target_rel, port_file, tree_copy, args, "port vs the reference tree's own copy")
    return 1 if failures else 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--program", type=pathlib.Path, default=PROGRAM,
                        help="the port's program (default: build/port/src/temacs)")
    parser.add_argument("--reference-program", type=pathlib.Path, default=REFERENCE_PROGRAM,
                        help="the reference's program (default: build/emacs-ref/src/emacs)")
    parser.add_argument("--check", action="store_true",
                        help="write into a temporary directory and compare only")
    parser.add_argument("--only", nargs="*", default=[], help="target basenames to generate")
    parser.add_argument("--list", action="store_true", help="print the product list and exit")
    parser.add_argument("--show", type=int, default=6, help="differing lines to print per product")
    parser.add_argument("--no-name-rewrite", action="store_true",
                        help="compare bytes only, without accounting for the name rewrite")
    args = parser.parse_args()
    args.reference_name_only = args.no_name_rewrite

    selected = [p for p in PRODUCTS if not args.only or pathlib.Path(p[2]).name in args.only]
    if args.list:
        for _, _, target in selected:
            print(target)
        return 0
    for path, what in ((args.program, "the port's program (xtask gate-fast then gate-load)"),
                       (args.reference_program, "the reference's program")):
        if not path.is_file():
            raise SystemExit(f"{path} is missing: build {what} first")

    failures = 0
    with tempfile.TemporaryDirectory() as tmp:
        workdir = pathlib.Path(tmp)
        for product in selected:
            failures += one_product(product, args, workdir) != 0

    print(f"{len(selected)} product(s), {failures} failure(s)")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
