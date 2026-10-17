#!/usr/bin/env python3
"""Generate the unidata family (`unidata.txt`, `lisp/international/uni-*.el`, `charprop.el`).

`admin/unidata/Makefile` derives everything in `lisp/international/` that describes Unicode
character properties from the Unicode data files it ships: `unidata.txt` is a `sed` of
`UnicodeData.txt`, one `unidata-gen-file` run per `uni-*.el` (the target list comes from
`unidata-gen.el` itself), and `charprop.el`, `uni-scripts.el`, `uni-confusable.el` and
`idna-mapping.el` are one run each of their own generator.  As in `tools/gen-charsets.py` and
`tools/gen-leim.py`, this tool asks the *frozen reference build* what it would run
(`make -B -n all`) and runs exactly that, with three substitutions:

  * the program becomes the **port's** binary - the files it writes name the port's own program
    (PORTING.md 0.1-19);
  * `-L` and the generator's data-directory argument become the port's `admin/unidata`, whose
    data files are the reference's verbatim copies and whose `unidata.txt` this tool writes first;
  * output paths become the port's `lisp/international/`.

A recipe that would write into the reference tree is refused and named (the `uvs`/`macuvs.h` run
is one: its output is `${top_srcdir}/src/macuvs.h`); the `gawk` and `emoji--generate-file`
recipes are not this tool's (their outputs, `charscript.el` and the two emoji files, come from
`tools/gen-charsets.py`).  Every produced file is then compared with the reference's own copy,
which is the reading: a difference is printed with the differing line count.

    python3 tools/gen-unidata.py                 # the whole family, then the comparisons
    python3 tools/gen-unidata.py --list          # print the plan and exit
    python3 tools/gen-unidata.py --only charprop # unidata.txt plus the named targets
    python3 tools/gen-unidata.py --compare-only  # only compare what is already there
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
REF_BUILD = REPO / "build/emacs-ref/admin/unidata"
PORT_EMACS = REPO / "build/port/src/temacs"

REF_PROGRAM = json_program = "../../src/emacs"
GENERATORS = ("unidata-gen-file", "unidata-gen-charprop", "unidata-gen-scripts",
              "unidata-gen-confusable", "unidata-gen-idna-mapping",
              "emoji--generate-file")
NOT_OURS = ("uvs", )          # the `macuvs.h` run writes into the reference's source tree


def recipes() -> list[list[str]]:
    """The invocations this tool owns, as argument lists (continuations joined)."""
    out = subprocess.run(["make", "-B", "-n", "all"], cwd=REF_BUILD,
                         capture_output=True, text=True)
    if out.returncode != 0:
        raise SystemExit(f"make -B -n all failed in {REF_BUILD}: {out.stderr.strip()}")
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
        if REF_PROGRAM not in command:
            continue
        if any(f"-f {gen}" in command for gen in GENERATORS):
            found.append(shlex.split(command))
        elif " -l uvs" in command:
            print(f"skipped (writes into the reference tree): {command[:80]}...")
    return found


def map_path(arg: str) -> str:
    """Map a path argument that names something inside the reference tree to the port's copy.

    The recipes run with `build/emacs-ref/admin/unidata` as their working directory and name
    their inputs and outputs **relatively** (`../../../../emacs/lisp/international/charprop.el`),
    so an argument has to be resolved against that directory before it can be recognised as a
    reference-tree path - comparing the raw strings is what let one run write the port's
    `charprop.el` into the reference's own tree (found by the port-name check below, which is why
    this function exists and why `run()` refuses such an output *before* executing).
    """
    if arg.startswith("-") or arg in ("", "."):
        return arg
    path = pathlib.Path(arg)
    if not path.is_absolute():
        path = REF_BUILD / path
    try:
        resolved = path.resolve()
        rel = resolved.relative_to(REF.resolve())
    except (OSError, ValueError):
        return arg                      # not a reference-tree path: leave it alone
    return str(PORT / rel)


def rewrite(argv: list[str]) -> list[str]:
    """The same invocation, pointed at the port's program, data directory and outputs."""
    result = []
    for arg in argv:
        if arg in (REF_PROGRAM, f"'{REF_PROGRAM}'"):
            result.append(str(PORT_EMACS))
        else:
            result.append(map_path(arg))
    return result


def output_of(argv: list[str]) -> pathlib.Path:
    """The path an invocation writes: the argument after the generator name."""
    i = argv.index("-f")
    return pathlib.Path(argv[i + 2])


def targets() -> list[str]:
    """The target name of every invocation (the argument after the `-f <generator>` pair)."""
    names = []
    for argv in recipes():
        try:
            i = argv.index("-f")
        except ValueError:
            continue
        names.append(argv[i + 2] if len(argv) > i + 2 else "")
    return names


def write_unidata_txt() -> int:
    """`sed -e 's/\\([^;]*\\);.../'` over UnicodeData.txt, into the port's admin/unidata."""
    source = PORT / "admin/unidata/UnicodeData.txt"
    target = PORT / "admin/unidata/unidata.txt"
    if not source.is_file():
        print(f"MISSING {source.relative_to(REPO)}")
        return 1
    text = source.read_text("utf-8", "surrogateescape")
    lines = []
    for line in text.split("\n"):
        if not line:
            lines.append("")
            continue
        head, _, rest = line.partition(";")
        lines.append(f'(#x{head} "{rest.replace(";", chr(34) + " " + chr(34))}")')
    target.write_text("\n".join(lines), encoding="utf-8", errors="surrogateescape")
    print(f"wrote {target.relative_to(REPO)} ({target.stat().st_size} bytes)")
    return 0


def remove_stale_charprop() -> None:
    """Delete `charprop.el` when the `uni-*.el` files it names are not all there yet.

    The order matters and the failure is not obvious: with `charprop.el` present but even one
    `uni-*.el` missing, the port's *own* start-up breaks - `characters.el` asks
    `unicode-property-table-internal` for a property, `uniprop_table`'s load branch fails (the
    data file is not there), and the function returns the **file name** the property's alist entry
    holds, so `(map-char-table ...)` signals `wrong-type-argument (char-table-p "uni-category.el")`
    and `loadup.el` aborts.  Every generator run then exits 255, which is how this was found
    (2026-10-16, status 4.383): the family has to be generated *before* `charprop.el`, exactly as
    `admin/unidata/Makefile`'s `all` order does it.
    """
    unifiles = [pathlib.Path(n).name for n in targets() if pathlib.Path(n).name.startswith("uni-")]
    charprop = PORT / "lisp/international/charprop.el"
    if charprop.is_file() and any(not (PORT / "lisp/international" / n).is_file() for n in unifiles):
        charprop.unlink()
        print("removed the stale lisp/international/charprop.el (its uni-*.el files are not all "
              "present yet; it is regenerated by this run)")


def compile_generators() -> int:
    """Byte-compile every generator the recipes load, exactly as the Makefile's rule does.

    `build/emacs-ref/admin/unidata/Makefile` 60-61 compiles any `%.el` the recipes need
    (`${emacs} -f batch-byte-compile $<`) and 70-81 makes `${srcdir}/unidata-gen.elc` an
    order-only prerequisite of the `uni-*.el` files.  Skipping that prerequisite changes the
    *output*: `unidata-gen-file` byte-compiles a describer and stores the function object in the
    table, and a compiled function prints as a byte-code object that embeds the path of the
    `.elc` it came from, while an interpreted one prints as a lambda.  The port's tree therefore
    has to compile the generator first, or its files can never look like the reference's
    (found 2026-10-17, status 4.556).
    """
    names = []
    for argv in recipes():
        if "-l" in argv:
            i = argv.index("-l")
            if len(argv) > i + 1 and argv[i + 1] not in names:
                names.append(argv[i + 1])
    directory = PORT / "admin/unidata"
    failures = 0
    for name in names:
        source = directory / f"{name}.el"
        if not source.is_file():
            print(f"no {source.relative_to(REPO)} to compile (not this tool's generator)")
            continue
        result = subprocess.run([str(PORT_EMACS), "-batch", "--no-site-file", "--no-site-lisp",
                                 "-L", str(directory), "-f", "batch-byte-compile", str(source)],
                                cwd=str(directory), capture_output=True, text=True)
        elc = source.with_suffix(".elc")
        status = "ok" if result.returncode == 0 else f"FAILED ({result.returncode})"
        print(f"compiled {source.name} -> {elc.name} ({elc.stat().st_size if elc.is_file() else 0} "
              f"bytes): {status}")
        if result.returncode != 0:
            failures += 1
            sys.stderr.write(result.stderr[-1200:])
    return failures


def run(only: list[str]) -> int:
    if not PORT_EMACS.is_file():
        raise SystemExit(f"{PORT_EMACS} is missing: build the port first")
    (PORT / "lisp/international").mkdir(parents=True, exist_ok=True)
    remove_stale_charprop()
    failures = compile_generators()

    todo = []
    for argv in recipes():
        i = argv.index("-f")
        if len(argv) <= i + 2:
            continue
        name = pathlib.Path(argv[i + 2]).name
        if only and name not in only:
            continue
        todo.append((name, argv))

    for n, (name, argv) in enumerate(todo, 1):
        real = rewrite(argv)
        out = output_of(real)
        try:
            out.resolve().relative_to(REF.resolve())
        except ValueError:
            pass                                # good: outside the reference tree
        else:
            print(f"[{n}/{len(todo)}] {name}: REFUSED (would write into the reference tree)")
            failures += 1
            continue
        if not out.is_absolute():
            out = REF_BUILD / out
        result = subprocess.run(real, cwd=REF_BUILD, capture_output=True, text=True)
        status = "ok" if result.returncode == 0 else f"FAILED ({result.returncode})"
        print(f"[{n}/{len(todo)}] {name}: {status}")
        if result.returncode != 0:
            failures += 1
            sys.stderr.write(result.stderr[-1500:])
    return failures


def compare(show: int) -> int:
    """Compare each `lisp/international` file this family produces with the reference's copy."""
    names = []
    for argv in recipes():
        i = argv.index("-f")
        if len(argv) > i + 2:
            names.append(pathlib.Path(argv[i + 2]).name)
    pairs = [(PORT / "lisp/international" / n, REF / "lisp/international" / n) for n in names]

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
        a = got.decode("utf-8", "replace").split("\n")
        b = want.decode("utf-8", "replace").split("\n")
        # `zip` used to hide the tail: when the two files have different line counts, the extra
        # lines of the longer one were simply not compared, so a file that *lost* half its
        # content could still read as "1 differing line" (found 2026-10-17, status 4.555: a
        # generated `uni-*.el` family whose tails had been cut short).
        pairs = list(zip(a, b))
        diffs = [f"line {i}: received {x!r} expected {y!r}"
                 for i, (x, y) in enumerate(pairs, 1) if x != y]
        if len(a) != len(b):
            diffs.append(f"line count: {len(a)} vs {len(b)} lines "
                         f"({abs(len(a) - len(b))} line(s) have no counterpart)")
        print(f"DIFFERENT {produced.name}: {len(diffs)} differing line(s) "
              f"({len(a)} vs {len(b)} lines)")
        for line in diffs[:show]:
            print(f"    {line[:150]}")
    print(f"{len(pairs)} file(s): {identical} identical, {different} different, {missing} missing")
    return 1 if (different or missing) else 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--list", action="store_true", help="print the plan and exit")
    parser.add_argument("--only", nargs="*", default=[], help="generator target names to run")
    parser.add_argument("--compare-only", action="store_true")
    parser.add_argument("--show", type=int, default=8)
    args = parser.parse_args()

    if args.list:
        for name, argv in zip(targets(), recipes()):
            print(f"{pathlib.Path(name).name}: {' '.join(argv)[:150]}")
        return 0

    failures = 0
    if not args.compare_only:
        failures += write_unidata_txt()
        failures += run(args.only)
    status = compare(args.show)
    return 1 if (failures or status) else 0


if __name__ == "__main__":
    sys.exit(main())
