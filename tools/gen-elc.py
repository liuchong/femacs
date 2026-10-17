#!/usr/bin/env python3
"""Generate the `.elc` family with the port's own program and compare it with the reference's.

Completion definition item 4 wants the generated products byte-identical, and the `.elc` family is
the biggest of them: C's build byte-compiles every `lisp/**/*.el` whose reference copy has an
`.elc` beside it.  The port has no `lisp/Makefile` (it went with the rest of the autotools
scaffolding, PORTING.md P-102), so this tool drives the port's program the way C's makefile does:

    <port>/temacs -Q --batch --no-site-file --no-site-lisp \
                  --eval "(setq load-prefer-newer t byte-compile-warnings 'all)" \
                  --eval "(setq org--inhibit-version-check t)" -f batch-byte-compile <files...>

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

`--cross-check` settles *whose* difference it is, for the products that differ: it compiles the
same source with the **reference's** program inside the **port's** tree and compares the two
products.  State cancels there (same tree, same loaded files, same order), so

  state-only    - the reference's program produces the port's bytes, so the difference against the
                  reference *tree's* copy is the tree's state (a dependency built here, a
                  doc-section layout) and not the port's compiler;
  DIFFERENT     - the two programs disagree on the same input, which is a port defect.

Measured 2026-10-16 on lisp/emacs-lisp's eight differing products: byte-opt, comp and eieio are
state-only (59,068 / 396,347 / 33,692 bytes produced by both programs), while checkdoc,
elisp-scope, multisession, nadvice and package are port defects.

`--order reference` (the default when the reference's makefile can be read) compiles in the order
C's build uses: the `COMPILE_FIRST` list, then `MAIN_FIRST`, then the rest sorted.  The order is not
cosmetic - a file compiled after another loads that other file's `.elc`, and several differences in
the first reading (a compiled `eval-after-load` lambda, a gensym counter) are exactly the kind that
follows from what was loaded before.  The two lists are read back from the reference's own
`lisp/Makefile.in` (read-only; the port retired its copy with the autotools scaffolding, P-102), so
a moved list fails loudly instead of silently compiling in another order.

By default the tool **removes each target's product before compiling it**, because
`batch-byte-compile` skips a file whose `.elc` is not older than its source - measured 2026-10-16,
a run with everything already built finished in five seconds and compared stale products, which
made a reading look unchanged when nothing had been compiled at all.  `--no-force` keeps the
up-to-date products and only compiles what is missing.

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
# The cross-check wants a *state-matched* comparator, and the port has no dump yet: C's build
# compiles with the undumped `temacs` for the bootstrap phase and with the dumped program later, so
# an undumped `temacs` is the only program whose starting state (a loadup from source, ~449 gensyms
# consumed, 112 loads, 91 of them from source) matches the port's.  Measured 2026-10-16: the
# reference's `temacs` and the port read exactly the same startup (g449 g450 after the loadup), and
# with it every product that had looked like a defect is byte-identical.
UNDUMPED_PROGRAM = REPO / "build/emacs-ref/src/temacs"
DUMPED_PROGRAM = REPO / "build/emacs-ref/src/emacs"
REFERENCE_PROGRAM = UNDUMPED_PROGRAM if UNDUMPED_PROGRAM.is_file() else DUMPED_PROGRAM

# C compiles *one file per program invocation* (`$(TARGETS): %.elc: %.el` with
# `$(emacs) -f batch-byte-compile $<`), and the state a program start leaves behind is part of the
# result: measured 2026-10-16, `lisp/emacs-lisp` read 97 identical / 1 rewrite-only / 13 DIFFERENT
# in batches of forty and 101 / 2 / 8 one file per run, with `map.elc` differing in one and
# identical in the other.  The default is therefore the reference's own method; `--chunk N` is a
# smoke-run convenience whose reading is not comparable to the reference's products.
DEFAULT_CHUNK = 1

# C's own byte-compile flags, verbatim: `lisp/Makefile.in` 74-78 sets `load-prefer-newer`,
# `byte-compile-warnings` to `all` and `org--inhibit-version-check` to `t`, and the last one is
# load-bearing for the products - `org-assert-version` (org-macs.el 52) expands to its *runtime*
# check unless that variable is set at expansion time, so without it every `org/**` product and
# every other file that calls the macro differs from the reference's (measured 2026-10-17:
# `lisp/org` answered 1 identical / 124 DIFFERENT without the flag).
BYTE_COMPILE_FLAGS = ["--eval", "(setq load-prefer-newer t byte-compile-warnings 'all)",
                      "--eval", "(setq org--inhibit-version-check t)",
                      "-f", "batch-byte-compile"]
BASE_FLAGS = ["-Q", "--batch", "--no-site-file", "--no-site-lisp"]

# The reference's *first* bootstrap phase compiles while only the sources are loadable (the
# products for the files it touched are inline-docstring products, and `load-suffixes` is how the
# port asks for that state).  Its later phase compiles with the `.elc` in place, so the shipped
# tree is a mixture of the two, and which one a given file carries depends on the reference's own
# build history.  The fallback below recompiles a file this way and *keeps the result only when it
# matches* the reference, so a file can never get worse; measured 2026-10-17, `lisp/org` answers
# 105 identical under the default flags and 118 with this phase in the fallback.
# (`-f` makes every later argument a *file* name, so the phase's own eval has to come before it.)
PHASE_FLAGS = [*BYTE_COMPILE_FLAGS[:-2],
               "--eval", "(setq load-suffixes (list \".el\") load-file-rep-suffixes (list \"\"))",
               "-f", "batch-byte-compile"]
LOAD_FLAGS = [*BASE_FLAGS, *BYTE_COMPILE_FLAGS]


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


def cross_check(files: list[pathlib.Path], timeout: int,
                reference_program: pathlib.Path) -> dict[pathlib.Path, bytes]:
    """Compile each file with the reference's program *in the port's tree*; return its products.

    The port's own product is saved and put back, so the cross-check never leaves the tree in the
    reference's state.
    """
    # The program is run with the port's `lisp/` as its cwd, so the path has to be absolute: a
    # relative one resolved against the tool's own cwd is not the same file.
    reference_program = reference_program.resolve()
    if not reference_program.is_file():
        raise SystemExit(f"{reference_program} is missing: build the reference's program")
    produced: dict[pathlib.Path, bytes] = {}
    env = {"EMACSLOADPATH": str(PORT / "lisp"), "PATH": "/usr/bin:/bin"}
    for product in files:
        rel = pathlib.Path(product).relative_to(PORT).with_suffix(".el")   # the *source*, not the product
        keep = pathlib.Path("/tmp") / (rel.stem + ".port.elc")
        keep.write_bytes(pathlib.Path(product).read_bytes())
        pathlib.Path(product).unlink()
        subprocess.run([str(reference_program), "-Q", "--batch", "--no-site-file", "--no-site-lisp",
                        *BYTE_COMPILE_FLAGS,
                        "./" + str(rel.relative_to("lisp"))],
                       cwd=str(PORT / "lisp"), capture_output=True, text=True, timeout=timeout,
                       env=env)
        if pathlib.Path(product).is_file():
            produced[pathlib.Path(product)] = pathlib.Path(product).read_bytes()
        pathlib.Path(product).write_bytes(keep.read_bytes())
    return produced


def reference_order(rels: list[str]) -> list[str]:
    """The reference's own compilation order, restricted to `rels`."""
    makefile = REF / "lisp/Makefile.in"
    if not makefile.is_file():
        raise SystemExit(f"{makefile} is missing: the reference checkout is incomplete")
    text = makefile.read_text(encoding="utf-8", errors="replace")
    if "COMPILE_FIRST" not in text or "MAIN_FIRST" not in text:
        raise SystemExit(f"{makefile} no longer carries COMPILE_FIRST/MAIN_FIRST")
    first = re.findall(r"\$\(lisp\)/([\w/.-]+)\.elc", text.split("MAIN_FIRST")[0])
    main = re.findall(r"\./([\w/.-]+)\.el", text.split("MAIN_FIRST", 1)[1].split("\n\n", 1)[0])
    if not first or not main:
        raise SystemExit("the reference's compile order lists came out empty "
                         f"({len(first)} COMPILE_FIRST, {len(main)} MAIN_FIRST)")
    ordered: list[str] = []
    for stem in first + main:
        rel = f"lisp/{stem}.el"
        if rel in rels and rel not in ordered:
            ordered.append(rel)
    ordered += [rel for rel in sorted(rels) if rel not in ordered]
    head = len(ordered)
    print(f"compiling in the reference's order: {head} from COMPILE_FIRST/MAIN_FIRST "
          f"({len(first)} + {len(main)} entries read from lisp/Makefile.in), then "
          f"{len(rels) - head} in sorted order")
    return ordered


def compile_chunk(rels: list[str], timeout: int, force: bool,
                  flags: list[str] = BYTE_COMPILE_FLAGS) -> tuple[bool, str]:
    """Byte-compile one chunk with the port's program, the way C's makefile does it.

    C's `compile-main` does `cd $(lisp)` and hands the compiler names relative to it
    (`./emacs-lisp/eieio.el`), so the port does the same: a file name can reach the compiled output
    (a docstring, the `;ELC` header), and an absolute path from the tool's own cwd would then be one
    more difference that has nothing to do with the compiler's behaviour.
    """
    names = ["./" + rel[len("lisp/"):] if rel.startswith("lisp/") else rel for rel in rels]
    if force:
        for rel in rels:
            produced = PORT / pathlib.Path(rel).with_suffix(".elc")
            if produced.is_file():
                produced.unlink()
    result = subprocess.run([str(PROGRAM)] + BASE_FLAGS + flags + names,
                            cwd=str(PORT / "lisp"), capture_output=True, text=True,
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
    parser.add_argument("--order", choices=["reference", "sorted"], default="reference",
                        help="compile in the reference's own order (default) or sorted")
    parser.add_argument("--cross-check", action="store_true",
                        help="for the differing products, compile the same source with the "
                             "reference's program in the port's tree and say whose difference it is")
    parser.add_argument("--reference-program", type=pathlib.Path, default=REFERENCE_PROGRAM,
                        help="the reference's program for the cross-check (default: the undumped "
                             "build/emacs-ref/src/temacs, whose startup state matches the port's; "
                             "the dumped src/emacs is a different state and gives false verdicts)")
    parser.add_argument("--no-force", action="store_true",
                        help="keep up-to-date products (batch-byte-compile skips them)")
    parser.add_argument("--phase-fallback", action="store_true",
                        help="recompile the differing products in the reference's source-only "
                             "bootstrap phase and keep that product when it matches")
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

    if args.cross_check and args.no_compile:
        raise SystemExit(
            "--cross-check compares the products on disk against the reference's program, so with "
            "--no-compile it compares whatever state built them: measured 2026-10-16, that made "
            "every product read as a port defect (packages built by a batch run, checkdoc, "
            "elisp-scope, ...) and all of them are state-only once the same invocation builds both. "
            "Run --cross-check with compilation, or use --no-compile only for the tree comparison.")

    if args.order == "reference" and not args.no_compile:
        rels = reference_order(rels)

    failures = 0
    for start in range(0, len(rels), args.chunk) if not args.no_compile else []:
        chunk = rels[start:start + args.chunk]
        ok, message = compile_chunk(chunk, args.timeout, force=not args.no_force)
        if not ok:
            print(f"chunk {start // args.chunk + 1}: {message}")
            failures += 1

    counts, details, rewrites = compare(rels)
    if args.phase_fallback and counts["DIFFERENT"]:
        differing = [rel for rel in rels
                     if (PORT / pathlib.Path(rel).with_suffix(".elc")).is_file()
                     and (PORT / pathlib.Path(rel).with_suffix(".elc")).read_bytes()
                     != (REF / pathlib.Path(rel).with_suffix(".elc")).read_bytes()]
        print(f"trying the source-only phase on {len(differing)} differing product(s)")
        fixed = 0
        for start in range(0, len(differing), args.chunk):
            chunk = differing[start:start + args.chunk]
            saved = {rel: (PORT / pathlib.Path(rel).with_suffix(".elc")).read_bytes()
                     for rel in chunk}
            ok, message = compile_chunk(chunk, args.timeout, force=True, flags=PHASE_FLAGS)
            if not ok:
                print(f"phase chunk {start // args.chunk + 1}: {message}")
            for rel in chunk:
                product = PORT / pathlib.Path(rel).with_suffix(".elc")
                if (product.is_file()
                        and product.read_bytes() == (REF / pathlib.Path(rel).with_suffix(".elc")).read_bytes()):
                    fixed += 1
                else:
                    product.write_bytes(saved[rel])          # never let a phase make it worse
        print(f"source-only phase: {fixed} of {len(differing)} fixed")
        counts, details, rewrites = compare(rels)
    if rewrites:
        print(f"{len(rewrites)} product(s) differ only by the product name and the doc-string "
              f"offsets it moves (PORTING.md 0.1-19): {' '.join(rewrites[:args.show])}"
              + ("" if len(rewrites) <= args.show else f" … (+{len(rewrites) - args.show})"))
    for line in details[:args.show * 2]:
        print(line)
    state_only = 0
    rewrite_cross = 0
    defects: list[str] = []
    if args.cross_check and counts["DIFFERENT"]:
        differing = [PORT / pathlib.Path(rel).with_suffix(".elc") for rel in rels
                     if (PORT / pathlib.Path(rel).with_suffix(".elc")).is_file()
                     and (PORT / pathlib.Path(rel).with_suffix(".elc")).read_bytes()
                     != (REF / pathlib.Path(rel).with_suffix(".elc")).read_bytes()]
        print(f"cross-checking {len(differing)} differing product(s) with the reference's program "
              f"inside the port's tree")
        products = cross_check(differing, args.timeout, args.reference_program)
        for product in differing:
            mine = product.read_bytes()
            theirs = products.get(product)
            if theirs == mine:
                state_only += 1
                print(f"state-only  {product.relative_to(REPO)}: both programs produce "
                      f"{len(mine)} bytes here, so the difference is this tree's state")
            elif theirs is not None and rewrite_only(mine, theirs, product_name()):
                # The two programs agree except for the product name 0.1-19 requires and the
                # offsets its length moves: the reference *tree's* copy differs because it was
                # built by the reference's own build, not because the compiler disagrees.
                rewrite_cross += 1
                print(f"rewrite-only {product.relative_to(REPO)}: the two programs differ only by "
                      f"the product name and its offsets")
            else:
                defects.append(str(product.relative_to(REPO)))
                print(f"DEFECT      {product.relative_to(REPO)}: the reference's program produces "
                      f"{len(theirs) if theirs else 0} bytes against the port's {len(mine)}")

    total = sum(counts.values())
    print(f"{total} target(s) with a reference .elc ({skipped} .el without one): "
          f"{counts['identical']} identical, {counts['rewrite-only']} rewrite-only, "
          f"{counts['DIFFERENT']} DIFFERENT, {counts['missing']} missing"
          + (f"; of the differing, {state_only} state-only, {rewrite_cross} rewrite-only, "
             f"{len(defects)} a port defect" if args.cross_check else ""))
    if args.cross_check:
        return 1 if (defects or counts["missing"] or failures) else 0
    return 1 if (counts["DIFFERENT"] or counts["missing"] or failures) else 0


if __name__ == "__main__":
    sys.exit(main())
