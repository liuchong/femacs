#!/usr/bin/env python3
"""Generate `lisp/subdirs.el` with the reference's own generator.

`build-aux/update-subdirs` writes `subdirs.el` into a directory: it lists that directory's
subdirectories so that `normal-top-level-add-subdirs-to-load-path` can find them.  It is pure
shell, its input is the directory listing itself, and the port has to produce the same file
(completion definition item 4: the generated artefacts are byte-identical), so this tool runs
*the reference's script* against the port's tree - the only thing that differs between the two
runs is which directory the script is pointed at.

    python3 tools/gen-subdirs.py            # every subdirs.el the reference tree has
    python3 tools/gen-subdirs.py --list     # the target list only

The script must be run *in* the target directory (it does `cd "$1"` itself), so this tool passes
the port's directory and never the reference's: `lisp/subdirs.el` is written and then
compared with the reference's own `emacs/lisp/subdirs.el`.  The comparison is the reading; a byte
difference is printed with the first differing offset.
"""

from __future__ import annotations

import argparse
import pathlib
import subprocess
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
REF = REPO / "emacs"
PORT = REPO

# The reference's generator, and the one directory whose subdirs.el its build writes (the
# `update-subdirs` rule in `lisp/Makefile` runs over ${SUBDIRS_SUBDIRS}, which is just `lisp`).
SCRIPT = REF / "build-aux/update-subdirs"
TARGETS = ["lisp"]


def targets() -> list[str]:
    """The directories whose `subdirs.el` the reference's build produces."""
    if not SCRIPT.is_file():
        raise SystemExit(f"{SCRIPT} is missing: the reference checkout is incomplete")
    found = sorted(str(p.parent.relative_to(REF))
                   for p in REF.glob("**/subdirs.el"))
    if found != TARGETS:
        raise SystemExit(f"the reference's subdirs.el set moved: {found} != {TARGETS}")
    return TARGETS


def first_difference(a: bytes, b: bytes) -> str:
    """Describe where two byte strings first differ (offset and the surrounding text)."""
    for i, (x, y) in enumerate(zip(a, b)):
        if x != y:
            lo = max(0, i - 40)
            return (f"first difference at offset {i}: "
                    f"received {a[lo:i + 40]!r} vs expected {b[lo:i + 40]!r}")
    return f"equal over the common prefix; lengths {len(a)} vs {len(b)}"


def generate(target: str) -> int:
    """Run the reference's generator on the port's directory, compare with the reference's file."""
    port_dir = PORT / target
    ref_file = REF / target / "subdirs.el"
    port_file = port_dir / "subdirs.el"
    if not port_dir.is_dir():
        raise SystemExit(f"{port_dir} is missing")
    if not ref_file.is_file():
        raise SystemExit(f"{ref_file} is missing")

    result = subprocess.run(["/bin/sh", str(SCRIPT), str(port_dir)],
                            cwd=port_dir, capture_output=True, text=True)
    if result.returncode != 0:
        print(f"{target}/subdirs.el: generator failed ({result.returncode}): {result.stderr.strip()}")
        return 1

    if not port_file.is_file():
        # The script removes subdirs.el when the directory has no subdirectories to list.
        print(f"{target}/subdirs.el: not written; the reference has {ref_file.stat().st_size} bytes")
        return 1

    received = port_file.read_bytes()
    expected = ref_file.read_bytes()
    if received == expected:
        print(f"{target}/subdirs.el: identical ({len(received)} bytes)")
        return 0
    print(f"{target}/subdirs.el: DIFFERENT")
    print(f"  {first_difference(received, expected)}")
    return 1


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("targets", nargs="*", help="directory names (default: every target)")
    parser.add_argument("--list", action="store_true", help="print the target list and exit")
    args = parser.parse_args()

    names = args.targets or targets()
    if args.list:
        for name in names:
            print(name)
        return 0

    failures = 0
    for name in names:
        failures += generate(name) != 0

    print(f"{len(names)} target(s), {failures} failure(s)")
    return 1 if failures else 0


if __name__ == "__main__":
    if "--compare-reference" not in sys.argv:
        raise SystemExit(subprocess.call([sys.executable, str(REPO / "tools/bootstrap-resources.py"), "--family", "subdirs", *sys.argv[1:]]))
    sys.argv.remove("--compare-reference")
    sys.exit(main())
