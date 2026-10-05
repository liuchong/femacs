#!/usr/bin/env python3
"""L1 of the acceptance ladder: byte-compile a Lisp subtree with a binary, and compare.

`docs/port-plan.md` 7.1's L1 is "every `lisp/emacs-lisp/*.elc` loads, with no error and no warning
difference".  The rung has two halves and this tool measures the first one - the *compilation*: it
copies a subtree into a scratch directory under `build/`, byte-compiles every `.el` in it with
`<binary> -Q --batch -L <scratch> -f batch-byte-compile <file>`, and records each file's exit code
and diagnostics.  Loading what it produced is the second half (L1's own check), and the `.elc`
*bytes* are a later rung's criterion.

    python3 tools/acceptance-l1.py --golden build/emacs-ref/src/emacs
    python3 tools/acceptance-l1.py --check <binary>

The scratch directory is fixed (`build/lisp-l1`) so that a diagnostic which mentions a path mentions
the *same* path on both sides; the golden records the reference binary's identity, and the
comparison is on the per-file records, byte for byte.
"""

from __future__ import annotations

import argparse
import hashlib
import re
import shutil
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
GOLDEN = REPO / "docs/baseline/l1-reference.txt"
SCRATCH = REPO / "build/lisp-l1"
SUBTREE = "lisp/emacs-lisp"
# The tree to copy.  The reference's own (`emacs/lisp/emacs-lisp`, whose `subdirs.el`, `loaddefs`
# and `.elc`s C's build generated) is the golden side; the port's (`femacs/lisp/emacs-lisp`) is the
# side that needs its *own* build pass first, which is why the comparison is differential.
TREE = REPO / "femacs/lisp"


def prepare() -> list[Path]:
    """The subtree, copied into the fixed scratch directory, and its `.el` files in order."""
    src = TREE / SUBTREE.split("/", 1)[1]
    if not src.is_dir():
        raise SystemExit(f"{src.relative_to(REPO)} is missing")
    if SCRATCH.is_dir():
        shutil.rmtree(SCRATCH)
    shutil.copytree(src, SCRATCH)
    files = sorted(p for p in SCRATCH.rglob("*.el") if p.is_file())
    if not files:
        raise SystemExit(f"no .el file under {SCRATCH.relative_to(REPO)}")
    return files


def dump_lisp_dirs(binary: Path) -> list[str]:
    """The `lisp` directories the binary's own dump carries, in the order it searches them.

    Measured at 3.268, because the order decides which file a `require` finds.  The reference's
    dump lists its tree root, then 24 subdirectories in *reverse* `make -C lisp` `SUBDIRS` order
    (`vc use-package url textmodes progmodes play org nxml net mh-e mail leim language
    international image gnus eshell erc emulation emacs-lisp cedet calendar calc obsolete`).
    `progmodes` ahead of `cedet` is the whole difference: `bytecomp.el`'s
    `(eval-when-compile (require 'compile))` finds `progmodes/compile.elc`, while any order that
    reaches `cedet/srecode` first finds a file that does not provide the feature.

    Returns the entries under that root, as relative paths, in that order.
    """
    r = subprocess.run([str(binary), "-Q", "--batch", "--eval", "(prin1 load-path)"],
                       capture_output=True, text=True)
    paths = re.findall(r'"([^"]*)"', r.stdout)
    if not paths:
        raise SystemExit(f"{binary} did not print a load-path")
    def under(p: str) -> int:
        return sum(1 for q in paths if q.startswith(p.rstrip("/") + "/"))
    roots = [p for p in paths if p.rstrip("/").endswith("/lisp") and under(p) > 1]
    if not roots:
        raise SystemExit(f"{binary}'s load-path has no lisp directory: {paths}")
    root = max(roots, key=under)
    return [p[len(root) + 1:] for p in paths if p.startswith(root.rstrip("/") + "/")]


def load_flags(binary: Path) -> list[str]:
    """`-L` for the scratch copy, the tree's root, and the tree's subdirectories in dump order.

    C's own build compiles with the load path of the dumped program (`make -C lisp` invokes
    `$(emacs) -f batch-byte-compile`), because the libraries a file `require`s live in
    `lisp/progmodes`, `lisp/net`, ...  This harness has to name that path itself, and Emacs
    searches `-L` directories in *command-line order*, ahead of the path its dump carries
    (`-L /tmp/a -L /tmp/b` gives `load-path` head `(/tmp/a /tmp/b ...)`, measured 3.268).

    Two consequences fix this list's shape, and both are measurements rather than choices: the
    tree's own root has to be on it, and the subdirectories have to come in the dumped program's
    own order rather than sorted.  Without either, `(require 'compile)` in `bytecomp.el` finds a
    subdirectory's `cedet/srecode/compile.elc` - a property of the harness, not of C (3.268).  The
    scratch copy stays first so that both sides' diagnostics name the same paths; the tree root
    follows it; the subdirectories come in the order the measured binary searches them, which is
    the order C's own builds search too.  The list is built from the tree being measured - the
    port's and the reference's `lisp` trees differ only in the 24 sanctioned identity lines
    (`tools/check-verbatim.py`, `docs/baseline/verbatim-substitutions.tsv`) - so it does not depend on
    which binary is used beyond that order.
    """
    dirs = [SCRATCH, TREE]
    for rel in dump_lisp_dirs(binary):
        d = TREE / rel
        if d.is_dir():
            dirs.append(d)
    out: list[str] = []
    for d in dirs:
        out += ["-L", str(d)]
    return out


def compile_one(binary: Path, file: Path) -> tuple[int, str]:
    # The load path is the scratch copy plus the tree it was copied from, for both sides: the
    # binaries' *own* load paths differ (the reference's points at its build tree, the port's will
    # point at its own), and a rung that compared those would measure the installation, not the
    # compiler.  The tree is the same on both sides here (check-verbatim: identical apart from the
    # 24 sanctioned identity lines), so naming the same tree on the reference side too keeps the
    # invocation identical.
    r = subprocess.run([str(binary), "-Q", "--batch", *load_flags(binary),
                        "-f", "batch-byte-compile", str(file)],
                       capture_output=True, text=True)
    return r.returncode, r.stdout + r.stderr


def binary_identity(binary: Path) -> tuple[str, str]:
    version = subprocess.run([str(binary), "--version"], capture_output=True, text=True).stdout
    return (version.split("\n")[0] if version else "",
            hashlib.sha256(binary.read_bytes()).hexdigest()[:16])


def records(binary: Path) -> list[tuple[str, int, str]]:
    out = []
    for file in prepare():
        code, diag = compile_one(binary, file)
        out.append((file.relative_to(SCRATCH).as_posix(), code, diag))
    return out


def version_number(first: str) -> str:
    """The reference's version *number*, out of its `--version` first line.

    `PORTING.md` 0.1-19: a version declaration this repository writes carries the revision and
    the version number, never the product's name - the reference prints `<name> <version>`, so
    the last whitespace-separated token is the number.  A line with no separate token (a build
    that prints the number alone) stays as it is, so the golden shows the difference instead of
    an empty field.
    """
    tokens = first.split()
    return tokens[-1] if len(tokens) > 1 else first


def version_header_lines(first: str) -> list[str]:
    """The `# version:` header line this repository writes into the golden, and why it is bare.

    The reference's own banner is quoted in `docs/acceptance.md`, where it is evidence *about*
    the reference; repeating the name it carries in a file of ours is the declaration 0.1-19
    removes.  The revision and the sha256 below identify the measured binary without it.
    """
    return [f"# version: {version_number(first)}",
            "#   the number only; PORTING.md 0.1-19: the name the reference prints is quoted in",
            "#   docs/acceptance.md as evidence about the reference, and is not repeated here"]


def write_golden(binary: Path) -> int:
    first, digest = binary_identity(binary)
    rows = records(binary)
    lines = ["# L1 golden: byte-compiling `lisp/emacs-lisp/*.el` with the reference program.",
             "# Written by `python3 tools/acceptance-l1.py --golden <binary>` (docs/status.md 3.265,",
             "# docs/acceptance.md).  `--check <binary>` re-runs the same 110 compilations in",
             "# `build/lisp-l1` and compares (exit code, diagnostics) per file, byte for byte.",
             f"# binary: {binary}",
             *version_header_lines(first),
             f"# sha256[0:16]: {digest}",
             ""]
    for name, code, diag in rows:
        lines += [f"@@ {name}", f"--- exit: {code}", "--- diagnostics:", diag, "--- end diagnostics"]
    GOLDEN.write_text("\n".join(lines), encoding="utf-8")
    errors = sum(1 for _, code, _ in rows if code != 0)
    warned = sum(1 for _, _, d in rows if d.strip())
    print(f"wrote {GOLDEN.relative_to(REPO)}: {len(rows)} file(s), {errors} non-zero exit(s), "
          f"{warned} file(s) with diagnostics (reference {version_number(first)!r})")
    return 0


def read_golden() -> dict[str, tuple[int, str]]:
    out: dict[str, tuple[int, str]] = {}
    label, code, mode, diag = None, 0, None, []
    def flush():
        if label is not None:
            out[label] = (code, "\n".join(diag))
    for line in GOLDEN.read_text(encoding="utf-8").split("\n"):
        if line.startswith("@@"):
            flush(); label, code, mode, diag = line[3:].strip(), 0, None, []
        elif label is None:
            continue
        elif line.startswith("--- exit: "):
            code = int(line[len("--- exit: "):])
        elif line == "--- diagnostics:":
            mode = "d"
        elif line == "--- end diagnostics":
            mode = None
        elif mode == "d":
            diag.append(line)
    flush()
    return out


def check(binary: Path) -> int:
    golden = read_golden()
    first, digest = binary_identity(binary)
    print(f"binary: {binary}\n  version: {version_number(first)}\n  sha256[0:16]: {digest}")
    failures = 0
    for name, code, diag in records(binary):
        want = golden.get(name)
        if want is None:
            print(f"GONE  {name}: the golden has no record under this name")
            failures += 1
            continue
        if (code, diag) == want:
            print(f"ok    {name}")
            continue
        failures += 1
        print(f"FAIL  {name}")
        if code != want[0]:
            print(f"      exit: golden {want[0]}, binary {code}")
        if diag != want[1]:
            print(f"      diagnostics differ:\n        golden: {want[1][:400]!r}\n"
                  f"        binary: {diag[:400]!r}")
    print(f"{len(golden)} file(s), {failures} failure(s)")
    return 1 if failures else 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--golden", metavar="BINARY")
    ap.add_argument("--check", metavar="BINARY")
    ap.add_argument("--tree", metavar="DIR", default=None,
                    help="the lisp tree to copy (default femacs/lisp; the reference's own "
                         "emacs/lisp is the golden side)")
    args = ap.parse_args()
    global TREE
    if args.tree:
        TREE = (REPO / args.tree).resolve() if not Path(args.tree).is_absolute() else Path(args.tree)
    if args.golden:
        return write_golden(Path(args.golden))
    if args.check:
        return check(Path(args.check))
    ap.print_help()
    return 2


if __name__ == "__main__":
    raise SystemExit(main())
