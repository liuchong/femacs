#!/usr/bin/env python3
"""Audit the lib/ units whose gnulib module the frozen build does not use.

Thirteen `lib/` files port to objects that define a *system* function's own name (`open`,
`fcntl`, `getdelim`, ...).  C never links them: `temacs` links `libgnu.a`'s member set, and the
frozen configuration does not build those modules at all.  The port's *archive* used to include
every ported `lib/` unit instead, which shadowed libSystem's definitions - measured 2026-10-16,
the program died inside the port's own `open` before running any Lisp, and an object-set audit
found 14 such definitions across 11 units (docs/status.md 4.383).

The rule that fixed it lives in `xtask gate_load` (the link takes the *reference archive's* member
set).  What was missing is the per-unit reading, which is what this gate is:

  * every one of the thirteen units' objects exists (building it with `xtask build-tu` first);
  * none of those objects is named in the link's response file (`build/load/link-args.rsp`);
  * **and** none of the global text symbols those objects define is defined in the linked program
    - a name that shows up in both would be a shadow that reached the binary.

    python3 tools/check-libshadows.py            # build what is missing, then audit
    python3 tools/check-libshadows.py --no-build # audit only (missing objects fail)
"""

from __future__ import annotations

import argparse
import pathlib
import subprocess
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
PORT_TREE = REPO / "femacs"
BUILD = REPO / "build/femacs"
PROGRAM = REPO / "build/port/src/temacs"
LINK_ARGS = REPO / "build/load/link-args.rsp"

# The thirteen files, and the C path each is built from.
UNITS = ["open", "fcntl", "getdelim", "getline", "getgroups", "mkostemp", "stpcpy", "strnlen",
         "strtoimax", "strtoll", "time_r", "fchmodat", "cloexec"]


def build(unit: str) -> bool:
    """Build one unit with the project's own builder."""
    import os
    env = dict(os.environ, CARGO_TARGET_DIR="/tmp/xt-cargo")
    result = subprocess.run(
        ["cargo", "run", "-q", "-p", "xtask", "--", "build-tu", f"lib/{unit}.c"],
        cwd=PORT_TREE, capture_output=True, text=True, env=env)
    if result.returncode != 0:
        sys.stderr.write(result.stderr[-400:])
    return result.returncode == 0


def defined_text_symbols(obj: pathlib.Path) -> list[str]:
    """The global text symbols an object defines, without the leading underscore."""
    out = subprocess.run(["nm", "-g", str(obj)], capture_output=True, text=True).stdout
    names = []
    for line in out.split("\n"):
        parts = line.split()
        if len(parts) == 3 and parts[1] == "T":
            names.append(parts[2].lstrip("_"))
    return names


def program_text_symbols() -> set[str]:
    """The global text symbols the linked program defines."""
    out = subprocess.run(["nm", "-g", str(PROGRAM)], capture_output=True, text=True).stdout
    names = set()
    for line in out.split("\n"):
        parts = line.split()
        if len(parts) == 3 and parts[1] in ("T", "t"):
            names.add(parts[2].lstrip("_"))
    return names


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--no-build", action="store_true", help="do not build missing objects")
    args = parser.parse_args()

    if not PROGRAM.is_file():
        raise SystemExit(f"{PROGRAM} is missing: link the port first (xtask gate-load)")
    if not LINK_ARGS.is_file():
        raise SystemExit(f"{LINK_ARGS} is missing: run `xtask gate-load` first")
    linked = LINK_ARGS.read_text()
    program = program_text_symbols()

    failures = 0
    shadowing: list[str] = []
    for unit in UNITS:
        obj = BUILD / f"{unit}.o"
        if not obj.is_file():
            if args.no_build or not build(unit):
                print(f"MISSING {obj.relative_to(REPO)}")
                failures += 1
                continue
        if f"/{unit}.o" in linked:
            print(f"LINKED  lib/{unit}.c's object is in the link's object list")
            failures += 1
        names = defined_text_symbols(obj)
        for name in names:
            if name in program:
                shadowing.append(f"{unit}: {name}")
        print(f"ok      lib/{unit}.c -> {obj.name}: defines {len(names)} symbol(s)"
              f"{': ' + ', '.join(names) if names else ''}")

    if shadowing:
        for row in shadowing:
            print(f"SHADOW  {row}: defined by a port object *and* by the linked program")
        failures += len(shadowing)
    print(f"{len(UNITS)} unit(s), {failures} failure(s), {len(shadowing)} shadowing symbol(s)")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
