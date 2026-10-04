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
import shutil
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
GOLDEN = REPO / "docs/baseline/l1-reference.txt"
SCRATCH = REPO / "build/lisp-l1"
SUBTREE = "lisp/emacs-lisp"


def prepare() -> list[Path]:
    """The subtree, copied into the fixed scratch directory, and its `.el` files in order."""
    src = REPO / "femacs" / SUBTREE
    if not src.is_dir():
        raise SystemExit(f"{src.relative_to(REPO)} is missing")
    if SCRATCH.is_dir():
        shutil.rmtree(SCRATCH)
    shutil.copytree(src, SCRATCH)
    files = sorted(p for p in SCRATCH.rglob("*.el") if p.is_file())
    if not files:
        raise SystemExit(f"no .el file under {SCRATCH.relative_to(REPO)}")
    return files


def load_flags() -> list[str]:
    """`-L` for the scratch copy and for every directory of the tree it came from.

    C's own build passes one `-L` per lisp subdirectory (`make -C lisp`'s
    `BYTE_COMPILE_FLAGS`), because the libraries a file `require`s live in `lisp/progmodes`,
    `lisp/net`, ... and the dumped binary's own load path names only the tree root.  The list comes
    from the port's tree for both sides - it is byte-identical to the reference's
    (`tools/check-verbatim.py`) - so the invocation does not depend on which binary is being
    measured.
    """
    tree = REPO / "femacs/lisp"
    dirs = [SCRATCH] + sorted(p for p in tree.rglob("*") if p.is_dir())
    out: list[str] = []
    for d in dirs:
        out += ["-L", str(d)]
    return out


def compile_one(binary: Path, file: Path) -> tuple[int, str]:
    # The load path is the scratch copy plus the tree it was copied from, for both sides: the
    # binaries' *own* load paths differ (the reference's points at its build tree, the port's will
    # point at its own), and a rung that compared those would measure the installation, not the
    # compiler.  The tree itself is byte-identical between the two sides (check-verbatim), so
    # naming the port's copy on the reference side too keeps the invocation identical.
    r = subprocess.run([str(binary), "-Q", "--batch", *load_flags(),
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


def write_golden(binary: Path) -> int:
    first, digest = binary_identity(binary)
    rows = records(binary)
    lines = ["# L1 golden: byte-compiling `lisp/emacs-lisp/*.el` with the reference program.",
             "# Written by `python3 tools/acceptance-l1.py --golden <binary>` (docs/status.md 3.265,",
             "# docs/acceptance.md).  `--check <binary>` re-runs the same 110 compilations in",
             "# `build/lisp-l1` and compares (exit code, diagnostics) per file, byte for byte.",
             f"# binary: {binary}",
             f"# version: {first}",
             f"# sha256[0:16]: {digest}",
             ""]
    for name, code, diag in rows:
        lines += [f"@@ {name}", f"--- exit: {code}", "--- diagnostics:", diag, "--- end diagnostics"]
    GOLDEN.write_text("\n".join(lines), encoding="utf-8")
    errors = sum(1 for _, code, _ in rows if code != 0)
    warned = sum(1 for _, _, d in rows if d.strip())
    print(f"wrote {GOLDEN.relative_to(REPO)}: {len(rows)} file(s), {errors} non-zero exit(s), "
          f"{warned} file(s) with diagnostics (reference {first!r})")
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
    print(f"binary: {binary}\n  version: {first}\n  sha256[0:16]: {digest}")
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
    args = ap.parse_args()
    if args.golden:
        return write_golden(Path(args.golden))
    if args.check:
        return check(Path(args.check))
    ap.print_help()
    return 2


if __name__ == "__main__":
    raise SystemExit(main())
