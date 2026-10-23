#!/usr/bin/env python3
"""Generate the leim family (the quail dictionaries and `leim-list.el`).

`leim/Makefile` turns the input dictionaries under `leim/CXTERM-DIC/` and `leim/MISC-DIC/` into
`lisp/leim/quail/*.el` with `titdic-cnv`, and then scans the result into `lisp/leim/leim-list.el`.
Nineteen `titdic-cnv` runs are one program invocation each, so this tool asks the *frozen
reference build* what it would run (`make -B -n all`, the `-B` making already-existing targets
print their recipe too) and runs exactly those, rewriting only

  * the program: the reference's `../src/emacs` becomes the **port's** binary - the files it
    writes carry the port's own program name (PORTING.md 0.1-19), so the port cannot borrow the
    reference's;
  * the output directory: `-dir …/lisp/leim/quail` becomes the port's `<port>/lisp/leim/quail`;
  * the input paths: the port's own copies of the dictionaries when it has them.

Then it runs the `leim-list.el` recipe (`update-leim-list-file` plus the `sed` that appends
`leim-ext.el`) and compares every produced file with the reference's own copy - the comparison is
the reading, and a difference is printed with the differing line count (the program-name lines of
the generated headers are expected to differ, exactly as `PORTING.md` 0.1-19 requires).

    python3 tools/gen-leim.py                 # every target, then the comparisons
    python3 tools/gen-leim.py --list          # print the 19 conversions and exit
    python3 tools/gen-leim.py --show 20       # how many differing lines to print per file
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
PORT = REPO
REF_BUILD_LEIM = REPO / "build/emacs-ref/leim"
PORT_EMACS = REPO / "build/port/src/temacs"

# The reference build's own `PROGRAM` placeholder and the `-dir` argument it passes.
REF_PROGRAM = "../src/emacs"          # `shlex.split` drops the recipe's quotes
REF_QUAIL = "lisp/leim/quail"
REF_LEIM_INPUT = "/leim/"
REF_LEIMDIR = re.compile(r"-dir\s+(\S+)")


def recipes() -> list[list[str]]:
    """The `titdic-cnv` invocations the reference build would run, as argument lists."""
    out = subprocess.run(["make", "-B", "-n", "all"], cwd=REF_BUILD_LEIM,
                         capture_output=True, text=True)
    if out.returncode != 0:
        raise SystemExit(f"make -B -n all failed in {REF_BUILD_LEIM}: {out.stderr.strip()}")
    joined, pending = [], ""
    for line in out.stdout.split("\n"):
        line = line.rstrip()
        if line.endswith("\\"):
            pending += line[:-1]
            continue
        joined.append(pending + line)
        pending = ""
    found = []
    for command in joined:
        # The same Makefile also generates `lisp/language/pinyin.el` through the same library
        # (`tit-pinyin-convert`), with an output path outside `leim/`, so the three `-f` names
        # are matched instead of a name prefix.
        if REF_PROGRAM not in command or not any(
                f"-f {fn}" in command for fn in ("batch-tit-dic-convert",
                                                 "batch-tit-miscdic-convert",
                                                 "tit-pinyin-convert")):
            continue
        found.append(command)
    if not found:
        raise SystemExit("no `batch-tit-*` recipe found: the frozen build moved")
    return [shlex.split(c) for c in found]


def map_path(arg: str) -> str:
    """Map a reference-tree path argument to the port's copy (the two are not always inside
    `leim/`: `lisp/language/pinyin.el` is written by a recipe of this Makefile)."""
    if arg.startswith("-") or arg in ("", "."):
        return arg
    path = pathlib.Path(arg)
    if not path.is_absolute():
        path = REF_BUILD_LEIM / path
    try:
        rel = path.resolve().relative_to(REF.resolve())
    except (OSError, ValueError):
        return arg
    return str(PORT / rel)


def rewrite(argv: list[str]) -> list[str]:
    """The same invocation, pointed at the port's program, output directory and inputs."""
    result: list[str] = []
    i = 0
    while i < len(argv):
        arg = argv[i]
        if arg in (REF_PROGRAM, f"'{REF_PROGRAM}'"):
            result.append(str(PORT_EMACS))
        elif arg == "-dir":
            result += ["-dir", str(PORT / "lisp/leim/quail")]
            i += 1                      # skip the recipe's own directory argument
        else:
            result.append(map_path(arg))
        i += 1
    return result


def run_all(only: list[str] = ()) -> int:
    """Run every conversion, then the `leim-list.el` recipe, with the port's program."""
    if not PORT_EMACS.is_file():
        raise SystemExit(f"{PORT_EMACS} is missing: build the port first")
    quail = PORT / "lisp/leim/quail"
    quail.mkdir(parents=True, exist_ok=True)

    failures = 0
    todo = recipes()
    if only:
        todo = [a for a in todo if pathlib.Path(a[-1]).name in only]
    for n, argv in enumerate(todo, 1):
        real = rewrite(argv)
        out = pathlib.Path(real[-1])
        try:
            out.resolve().relative_to(REF.resolve())
        except ValueError:
            pass                                # good: outside the reference tree
        else:
            print(f"[{n}/{len(todo)}] {name}: REFUSED (would write into the reference tree)")
            failures += 1
            continue
        result = subprocess.run(real, cwd=REF_BUILD_LEIM, capture_output=True, text=True)
        name = pathlib.Path(argv[-1]).name
        status = "ok" if result.returncode == 0 else f"FAILED ({result.returncode})"
        print(f"[{n}/{len(todo)}] {name}: {status}")
        if result.returncode != 0:
            failures += 1
            sys.stderr.write(result.stderr[-1200:])

    # The `leim-list.el` recipe: the program part, then the `sed` that appends `leim-ext.el`.
    leim_list = PORT / "lisp/leim/leim-list.el"
    leim_list.unlink(missing_ok=True)
    program = subprocess.run(
        [str(PORT_EMACS), "-batch", "--no-site-file", "--no-site-lisp",
         "-l", "international/quail",
         "--eval", f'(update-leim-list-file "{PORT}/lisp/leim")'],
        capture_output=True, text=True)
    print(f"leim-list.el (update-leim-list-file): "
          f"{'ok' if program.returncode == 0 else f'FAILED ({program.returncode})'}")
    if program.returncode != 0:
        failures += 1
        sys.stderr.write(program.stderr[-1200:])
    if leim_list.is_file():
        with leim_list.open("a") as out:
            sed = subprocess.run(
                ["sed", "-n", "-e", "/^[^;]/p", "-e", "s/^;\\(;*\\)inc /;\\1 /p",
                 str(PORT / "leim/leim-ext.el")],
                capture_output=True, text=True)
            out.write(sed.stdout)
    print(f"leim-list.el: {'appended leim-ext.el' if leim_list.is_file() else 'NOT WRITTEN'}")
    return failures


def compare(show: int) -> int:
    """Compare every quail `.el` and `leim-list.el` with the reference's own copy."""
    pairs: list[tuple[pathlib.Path, pathlib.Path]] = []
    for produced in sorted((PORT / "lisp/leim/quail").glob("*.el")):
        pairs.append((produced, REF / "lisp/leim/quail" / produced.name))
    pairs.append((PORT / "lisp/leim/leim-list.el", REF / "lisp/leim/leim-list.el"))

    identical = different = missing = 0
    for produced, reference in pairs:
        if not produced.is_file():
            print(f"MISSING {produced.relative_to(REPO)}")
            missing += 1
            continue
        if not reference.is_file():
            print(f"no reference copy for {produced.name}")
            missing += 1
            continue
        got, want = produced.read_bytes(), reference.read_bytes()
        if got == want:
            identical += 1
            continue
        different += 1
        a, b = got.decode("utf-8", "replace").split("\n"), want.decode("utf-8", "replace").split("\n")
        diffs = [f"received {x!r} expected {y!r}"
                 for x, y in zip(a, b) if x != y]
        print(f"DIFFERENT {produced.name}: {len(diffs)} differing line(s)")
        for line in diffs[:show]:
            print(f"    {line[:160]}")
    print(f"{len(pairs)} file(s): {identical} identical, {different} different, {missing} missing")
    return 1 if (different or missing) else 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--list", action="store_true", help="print the conversions and exit")
    parser.add_argument("--compare-only", action="store_true", help="skip the runs")
    parser.add_argument("--only", nargs="*", default=[],
                        help="run only the recipes whose last argument names these files")
    parser.add_argument("--show", type=int, default=10, help="differing lines to print per file")
    args = parser.parse_args()

    if args.list:
        for argv in recipes():
            print(" ".join(argv))
        return 0

    failures = 0 if args.compare_only else run_all(args.only)
    compare_status = compare(args.show)
    return 1 if (failures or compare_status) else 0


if __name__ == "__main__":
    if "--compare-reference" not in sys.argv:
        raise SystemExit(subprocess.call([sys.executable, str(REPO / "tools/bootstrap-resources.py"), "--family", "leim", *sys.argv[1:]]))
    sys.argv.remove("--compare-reference")
    sys.exit(main())
