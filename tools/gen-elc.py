#!/usr/bin/env python3
"""Generate the `.elc` family with the port's own program and compare it with the reference's.

Completion definition item 4 wants the generated products byte-identical, and the `.elc` family is
the biggest of them: C's build byte-compiles every `lisp/**/*.el` whose reference copy has an
`.elc` beside it.  The port has no `lisp/Makefile` (it went with the rest of the autotools
scaffolding, PORTING.md P-102), so this tool drives the port's program the way C's makefile does:

    <port>/temacs -Q --batch --no-site-file --no-site-lisp
                  --eval '(setq load-prefer-newer t)' -f batch-byte-compile <files...>

and then compares every product with the reference tree's copy, byte for byte.  Target files are
handed over in chunks, so one program start (the port's loadup, ~25 s) is amortised over many
compilations, and the scope is always explicit - a directory or a glob - so no run walks the whole
tree by accident.

    python3 tools/gen-elc.py lisp/emacs-lisp               # one directory
    python3 tools/gen-elc.py lisp/emacs-lisp --chunk 40    # files per program start
    python3 tools/gen-elc.py lisp/emacs-lisp --list        # the targets, no compilation
    python3 tools/gen-elc.py lisp/emacs-lisp --skip-matching   # only what is not identical yet

Products are written beside their sources in the port's tree (that is where the reference's build
writes them too, and `*.elc` is in the port's `.gitignore`); nothing is ever written under the
reference tree, and a target whose reference copy is missing is skipped and counted.

A product can differ from the reference's copy in three ways, and the verdict says which:

  identical     - byte for byte the same;
  rewrite-only  - the only differences are the product name (PORTING.md 0.1-19 requires the port's
                  build to write its own name where the reference's wrote its own) and the
                  `#@<n>` doc-string offsets that name's length moves - the same accounting
                  tools/check-generated.py does for the script-written families, in bytes.  The
                  masking of `#@<n>` is deliberate and its risk is bounded: an offset that moves
                  for any *other* reason still shows as a difference in the doc string's own
                  bytes, because the offsets are derived from them;
  DIFFERENT     - something else differs: the first differing offset and its neighbourhood.

`--no-compile` re-reads the products already on disk, which is how a classification (rather than a
fresh compilation) is repeated.
"""

from __future__ import annotations

import argparse
import pathlib
import re
import subprocess
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
REF = REPO / "emacs"
PORT = REPO / "femacs"
PROGRAM = REPO / "build/port/src/temacs"

# The loadup is what makes a program start expensive, so files are compiled in batches.
DEFAULT_CHUNK = 40
LOAD_FLAGS = ["-Q", "--batch", "--no-site-file", "--no-site-lisp",
              "--eval", "(setq load-prefer-newer t)", "-f", "batch-byte-compile"]


def targets(pattern: str) -> tuple[list[str], int]:
    """(reference-relative `.el` paths with an `.elc` beside them, count without one)."""
    found: list[str] = []
    skipped = 0
    for path in sorted(REF.glob(pattern)):
        if path.suffix != ".el":
            continue
        if (REF / path.relative_to(REF).with_suffix(".elc")).is_file():
            found.append(str(path.relative_to(REF)))
        else:
            skipped += 1
    return found, skipped


# The port's program name, and the reference's product name read out of the reference's own
# generated files (neither is spelled as a *reference* name here), the same accounting
# tools/check-generated.py applies to the script-written families.
PORT_NAME = b"femacs"
# `#@<n>` is a doc-string offset and `#$ . <n>` a shared-string offset; both are derived from the
# string data, so the name rewrite's length change moves them.  Masking them is deliberate and its
# risk is bounded: a difference in the strings themselves still shows, because the offsets are
# computed from those very bytes.
DOC_OFFSET = re.compile(rb"#@\d+")
STRING_OFFSET = re.compile(rb"#\$ \. \d+")


def product_name() -> bytes:
    """The reference's product name, from the reference's own generated files."""
    for rel in ("lisp/international/charprop.el", "lisp/leim/leim-list.el", "lisp/loaddefs.el"):
        path = REF / rel
        if not path.is_file():
            continue
        m = re.search(rb"This file is part of ([^.\n]+)\.", path.read_bytes())
        if m:
            return m.group(1)
    raise SystemExit("cannot read the reference's product name from its generated files")


def rewrite_only(got: bytes, want: bytes, product: bytes) -> bool:
    """True when the product name's rewrite and its doc-string offsets explain every difference."""
    def normalise(data: bytes) -> bytes:
        data = DOC_OFFSET.sub(b"#@#", data)
        data = STRING_OFFSET.sub(b"#$ . #", data)
        data = data.replace(product, b"product")
        data = data.replace(PORT_NAME, b"product")
        data = data.replace(product.split()[-1], b"product")
        return data
    return normalise(got) == normalise(want)


def first_difference(a: bytes, b: bytes) -> str:
    """Describe where two byte strings first differ (offset and the surrounding bytes)."""
    for i, (x, y) in enumerate(zip(a, b)):
        if x != y:
            lo = max(0, i - 40)
            return (f"first difference at offset {i}: "
                    f"received {a[lo:i + 40]!r} vs expected {b[lo:i + 40]!r}")
    return f"equal over the common prefix; lengths {len(a)} vs {len(b)}"


def compile_chunk(rels: list[str], timeout: int) -> tuple[bool, str]:
    """Byte-compile one chunk with the port's program (cwd is the port's tree)."""
    paths = [str(PORT / rel) for rel in rels]
    result = subprocess.run([str(PROGRAM)] + LOAD_FLAGS + paths,
                            cwd=str(REPO / "femacs"), capture_output=True, text=True,
                            timeout=timeout,
                            env={"EMACSLOADPATH": str(PORT / "lisp"), "PATH": "/usr/bin:/bin"})
    if result.returncode != 0:
        tail = (result.stderr or result.stdout).strip().splitlines()[-4:]
        return False, f"the compiler exited {result.returncode}: {' | '.join(tail)}"
    return True, ""


def compare(rels: list[str]) -> tuple[dict[str, int], list[str]]:
    """Compare each product with the reference's copy; return the counts and the detail lines."""
    counts = {"identical": 0, "rewrite-only": 0, "DIFFERENT": 0, "missing": 0}
    details: list[str] = []
    rewrites: list[str] = []
    product = product_name()
    for rel in rels:
        produced = PORT / pathlib.Path(rel).with_suffix(".elc")
        expected = REF / pathlib.Path(rel).with_suffix(".elc")
        if not produced.is_file():
            counts["missing"] += 1
            details.append(f"MISSING      {produced.relative_to(REPO)}")
            continue
        got, want = produced.read_bytes(), expected.read_bytes()
        if got == want:
            counts["identical"] += 1
            continue
        if rewrite_only(got, want, product):
            counts["rewrite-only"] += 1
            rewrites.append(str(produced.relative_to(REPO)))
            continue
        counts["DIFFERENT"] += 1
        details.append(f"DIFFERENT    {produced.relative_to(REPO)} ({len(got)} vs {len(want)} bytes)")
        details.append(f"  {first_difference(got, want)}")
    return counts, details, rewrites


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("scope", help="a glob under the reference tree, e.g. lisp/emacs-lisp or "
                                      "'lisp/emacs-lisp/*.el'")
    parser.add_argument("--chunk", type=int, default=DEFAULT_CHUNK,
                        help=f"files per program start (default {DEFAULT_CHUNK})")
    parser.add_argument("--timeout", type=int, default=1800, help="seconds per chunk")
    parser.add_argument("--list", action="store_true", help="print the targets and exit")
    parser.add_argument("--skip-matching", action="store_true",
                        help="leave out the targets whose product already matches")
    parser.add_argument("--show", type=int, default=6, help="differing files to describe")
    parser.add_argument("--no-compile", action="store_true",
                        help="re-read the products already on disk instead of compiling")
    args = parser.parse_args()

    if not args.list and not args.no_compile and not PROGRAM.is_file():
        raise SystemExit(f"{PROGRAM} is missing: build the port's program "
                         f"(xtask gate-fast, then gate-load)")

    pattern = args.scope if any(ch in args.scope for ch in "*?[") else f"{args.scope.rstrip('/')}/**/*.el"
    rels, skipped = targets(pattern)
    if not rels:
        raise SystemExit(f"{pattern} matched no .el with a reference .elc beside it")

    if args.list:
        for rel in rels:
            print(rel)
        print(f"{len(rels)} target(s), {skipped} .el without a reference .elc")
        return 0

    if args.skip_matching:
        counts, _, _ = compare(rels)
        if counts["identical"]:
            matching = {rel for rel in rels
                        if (PORT / pathlib.Path(rel).with_suffix(".elc")).is_file()
                        and (PORT / pathlib.Path(rel).with_suffix(".elc")).read_bytes()
                        == (REF / pathlib.Path(rel).with_suffix(".elc")).read_bytes()}
            rels = [rel for rel in rels if rel not in matching]
            print(f"skipping {len(matching)} already-matching product(s)")
            if not rels:
                print("nothing left to compile")
                return 0

    failures = 0
    for start in range(0, len(rels), args.chunk) if not args.no_compile else []:
        chunk = rels[start:start + args.chunk]
        ok, message = compile_chunk(chunk, args.timeout)
        if not ok:
            print(f"chunk {start // args.chunk + 1}: {message}")
            failures += 1

    counts, details, rewrites = compare(rels)
    if rewrites:
        print(f"{len(rewrites)} product(s) differ only by the product name and the doc-string "
              f"offsets it moves (PORTING.md 0.1-19): {' '.join(rewrites[:args.show])}"
              + ("" if len(rewrites) <= args.show else f" … (+{len(rewrites) - args.show})"))
    for line in details[:args.show * 2]:
        print(line)
    total = sum(counts.values())
    print(f"{total} target(s) with a reference .elc ({skipped} .el without one): "
          f"{counts['identical']} identical, {counts['rewrite-only']} rewrite-only, "
          f"{counts['DIFFERENT']} DIFFERENT, {counts['missing']} missing")
    return 1 if (counts["DIFFERENT"] or counts["missing"] or failures) else 0


if __name__ == "__main__":
    sys.exit(main())
