#!/usr/bin/env python3
"""Generate the `*loaddefs.el` family the way the reference's build does (the `autoloads` recipe).

`lisp/Makefile.in` 202-205 is the recipe: byte-compile `lisp/emacs-lisp/loaddefs-gen.el` (the
`autoloads` prerequisite, line 101's `COMPILE_FIRST`) and then run

    ${emacs} -l <lisp>/emacs-lisp/loaddefs-gen.elc -f loaddefs-generate--emacs-batch ${SUBDIRS_ALMOST}

`SUBDIRS_ALMOST` is `find ${srcdir} -type d` minus `obsolete` and `term` (125).  This tool asks the
frozen reference build for that command line (`make -B -n autoloads` in `build/emacs-ref/lisp`),
rewrites the program to the port's binary and every path into the port's tree, and runs it - so the
port's `loaddefs.el` is *generated*, not copied or text-rewritten.  That distinction is the whole
point: `docs/status.md` 4.556 found the port's copy carrying two characters mangled by a rewrite
(`回复` -> `回\15`, the low byte of the codepoint) and that is not a state any generator produces.

The prerequisite matters for the same reason `docs/status.md` 4.556/P-187 found it for
`admin/unidata`: `make -n` prints commands, not order-only prerequisites, and generating with the
*source* generator instead of the compiled one changes the output.  This tool compiles it first,
with the port's own program.

    python3 tools/gen-loaddefs.py              # generate into the port tree, then compare
    python3 tools/gen-loaddefs.py --list       # print the commands and exit
    python3 tools/gen-loaddefs.py --compare-only

**Read this before running it.**  The tool is a *diagnostic*, not a build step: measured
2026-10-17, the port's own generation of this family is not yet equivalent to the reference's, so
its product must not be shipped.

  * the port's product mangles exactly two characters - `复` (U+590D) comes out as `\15` and `’`
    (U+2019) as `\31`, each the *low byte* of the codepoint in octal - while the **reference
    program**, given the same generator `.elc` and byte-identical sources, writes them correctly.
    That is a defect in the port's generation path (status 4.560).
  * the port's product makes `xtask dump` fail (`Attempt to autoload rx while preparing to dump`),
    so nothing that loads `loaddefs` can be rebuilt from it.

Two operational facts the runs cost: `loaddefs-generate--emacs-batch` writes into the running
program's **own** tree (so a reference-program run with the port's arguments edits the reference
tree, which is how one of these runs did), and it *merges* into an existing file instead of
replacing it - which is why the Makefile's `autoloads-force` removes the file first and why this
tool does the same.
"""

from __future__ import annotations

import argparse
import pathlib
import re
import shlex
import subprocess
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
REF = REPO / "emacs"
PORT = REPO / "femacs"
REF_LISP_BUILD = REPO / "build/emacs-ref/lisp"
PORT_PROG = REPO / "build/port/src/emacs"
REF_PROG = "../src/emacs"
PORT_NAME = "femacs"
NAME_LINE = re.compile(r"This file is part of ([^.\n]+)\.")


def product_name() -> str:
    for rel in ("lisp/loaddefs.el", "lisp/subdirs.el"):
        path = REF / rel
        if path.is_file():
            m = NAME_LINE.search(path.read_text("utf-8", "surrogateescape"))
            if m:
                return m.group(1)
    raise SystemExit("cannot read the reference's product name")


def recipe() -> list[list[str]]:
    """The two command lines this tool owns: the compile prerequisite and the generator."""
    out = subprocess.run(["make", "-B", "-n", "autoloads"], cwd=REF_LISP_BUILD,
                         capture_output=True, text=True)
    if out.returncode != 0:
        raise SystemExit(f"make -B -n autoloads failed: {out.stderr.strip()[:400]}")
    joined, pending = [], ""
    for line in out.stdout.split("\n"):
        line = line.rstrip()
        if line.endswith("\\"):
            pending += line[:-1]
            continue
        joined.append(pending + line)
        pending = ""
    found = []
    for line in joined:
        # Make prints `echo "  ELC     " file; '<prog>' …` - two shell commands on one line, so
        # split on `;` and keep the parts that actually run the program (otherwise `echo` is taken
        # for a path argument and mapped into the port tree).
        for command in line.split(";"):
            if REF_PROG not in command:
                continue
            if "loaddefs-generate--emacs-batch" in command or (
                    "batch-byte-compile" in command and "loaddefs-gen.el" in command):
                found.append(shlex.split(command.strip()))
    return found


def map_path(arg: str) -> str:
    """Re-root a path argument at the port's tree, keeping its *shape*.

    The shape matters: `loaddefs-generate--emacs-batch` builds each output path as
    `<argument>/loaddefs.el` and resolves it against the *current directory*, so an absolute
    argument makes it write `../../../../../Users/…/lisp/loaddefs.el` - the right file by
    accident, and twice for the same directory (measured: an 83,836-line file, i.e. the content
    written into itself).  Keeping `../../../femacs/lisp` in the same relative shape as the
    reference's `../../../emacs/lisp` avoids both.

    A plain word is not mapped at all: it is a function name (`-f loaddefs-generate--emacs-batch`)
    or a Lisp form (`--eval "(setq …)"`), and resolving it against the build directory turns it
    into `<port>/lisp/loaddefs-generate--emacs-batch`, which the program then treats as a function
    name (measured).
    """
    if arg.startswith("-") or arg in ("", ".") or not (
            arg.startswith((".", "/", "~")) or "/" in arg):
        return arg
    if not arg.startswith("/"):
        parts = pathlib.PurePath(arg).parts
        return str(pathlib.PurePath(*[(PORT.name if p == REF.name else p) for p in parts]))
    path = pathlib.Path(arg)
    try:
        resolved = path.resolve()
    except OSError:
        return arg
    for root, target in ((REF, PORT), (REF_LISP_BUILD, PORT / "lisp")):
        try:
            return str(target / resolved.relative_to(root.resolve()))
        except ValueError:
            continue
    return arg


def rewrite(argv: list[str]) -> list[str]:
    return [str(PORT_PROG) if a in (REF_PROG, f"'{REF_PROG}'") else map_path(a) for a in argv]


def check_dirs_outside_the_port(argv: list[str]) -> list[str]:
    """Directories the generator would scan that are not inside the port's tree.

    The tool must refuse those.  `loaddefs-generate--emacs-batch` writes each output as
    `<argument>/loaddefs.el` *and* maintains its own top-level `loaddefs.el`, so a directory
    outside the port (a temp tree, the reference's own `lisp/`) would either be written to or
    would make the port's file grow a second copy of itself - both measured on 2026-10-17, the
    second one twice.  The rule from P-186 and P-189 is the same one: only the port's tree is ever
    a target.
    """
    offenders = []
    for arg in argv[7:]:                     # everything after `-f loaddefs-generate--emacs-batch`
        if arg.startswith("-"):
            continue
        path = pathlib.Path(arg)
        if not path.is_absolute():
            path = REF_LISP_BUILD / path
        try:
            path.resolve().relative_to(PORT.resolve())
        except (OSError, ValueError):
            offenders.append(arg)
    return offenders


def run_command(argv: list[str], what: str) -> int:
    result = subprocess.run(argv, cwd=REF_LISP_BUILD, capture_output=True, text=True)
    status = "ok" if result.returncode == 0 else f"FAILED ({result.returncode})"
    print(f"{what}: {status}")
    if result.returncode != 0:
        sys.stderr.write(result.stderr[-1500:])
    return 0 if result.returncode == 0 else 1


def compare(show: int) -> int:
    product = product_name()
    reference_root, port_root = str(REF), str(PORT)
    produced = sorted((PORT / "lisp").rglob("*loaddefs.el"))
    identical = name_only = different = missing = 0
    for port_file in produced:
        rel = port_file.relative_to(PORT)
        ref_file = REF / rel
        if not ref_file.is_file():
            print(f"no reference copy for {rel}")
            missing += 1
            continue
        want, got = ref_file.read_bytes(), port_file.read_bytes()
        if want == got:
            identical += 1
            continue
        wl = want.decode("utf-8", "surrogateescape").split("\n")
        gl = got.decode("utf-8", "surrogateescape").split("\n")
        diffs = []
        if len(wl) != len(gl):
            diffs.append(f"line counts differ: {len(wl)} vs {len(gl)}")
        for n, (x, y) in enumerate(zip(wl, gl), 1):
            if x == y:
                continue
            wanted = x.replace(product, PORT_NAME).replace(product.split()[-1], PORT_NAME)
            if y != wanted and y != wanted.replace(reference_root, port_root):
                diffs.append(f"line {n}: reference {x[:110]!r} / port {y[:110]!r}")
        if not diffs:
            name_only += 1
            continue
        different += 1
        print(f"DIFFERENT {rel}: {len(diffs)} line(s)")
        for line in diffs[:show]:
            print(f"    {line}")
    print(f"{len(produced)} file(s): {identical} identical, {name_only} name-only, "
          f"{different} DIFFERENT, {missing} missing")
    return 1 if (different or missing) else 0


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--list", action="store_true")
    ap.add_argument("--compare-only", action="store_true")
    ap.add_argument("--show", type=int, default=6)
    a = ap.parse_args()

    commands = recipe()
    if not commands:
        raise SystemExit("the autoloads recipe did not name loaddefs-gen")
    if a.list:
        for argv in commands:
            print(" ".join(argv)[:200])
        return 0
    failures = 0
    if not a.compare_only:
        if not PORT_PROG.is_file():
            raise SystemExit(f"{PORT_PROG} is missing: build the port first")
        # The Makefile's own `autoloads-force` (208-211) removes the file first, and that is the
        # difference between "the generator ran" and "the file was rewritten": `loaddefs-generate`
        # leaves a `loaddefs.el` that is newer than its sources alone (measured: the port's copy
        # stayed byte for byte what it was, mtime and all, while the command reported success).
        stale = PORT / "lisp/loaddefs.el"
        if stale.is_file():
            stale.unlink()
            print(f"removed {stale.relative_to(REPO)} first (the Makefile's autoloads-force)")
        for argv in commands:
            real = rewrite(argv)
            what = "compiled loaddefs-gen.el" if "batch-byte-compile" in argv \
                else "generated the *loaddefs.el family"
            offenders = check_dirs_outside_the_port(real)
            if offenders:
                print(f"REFUSED {what}: these directories are not in the port's tree: "
                      f"{' '.join(offenders[:3])}")
                failures += 1
                continue
            failures += run_command(real, what)
    status = compare(a.show)
    return 1 if (failures or status) else 0


if __name__ == "__main__":
    sys.exit(main())
