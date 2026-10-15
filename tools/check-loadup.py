#!/usr/bin/env python3
"""Compare the two `loadup.el` runs file for file.

The port has no dump, so its `temacs` equivalent loads `loadup.el` at every start (lread.c's
`--no-loadup` aside), which is why the same file list can be read from both sides: the reference
is run as `temacs -batch -l loadup` (undumped, so it takes the same path) and the port as
`build/port/src/temacs -Q --batch`.  Both print `Loading NAME...` once per file, and the *set* of
those names is the reading:

    python3 tools/check-loadup.py                  # reference vs port, default binaries
    python3 tools/check-loadup.py --show 20        # how many divergences to print

Names are normalised (the path prefix is dropped, the `.el`/`.elc` suffix and the
`(source)`/`(compiled ...)` marker are stripped), so the reference's compiled files and the port's
sources compare equal; files that are loaded by only one side are printed in order.  The exit
status is 0 only when both runs reach the end of `loadup.el` and load the same files.
"""

from __future__ import annotations

import argparse
import pathlib
import re
import subprocess
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
BUILD = REPO / "build"
LOADING = re.compile(r"^Loading (.+?)\.\.\.$")
MARKER = "LOADUP-REACHED-END"


def normalise(name: str) -> str:
    """Strip the directory, the load marker and the `.el`/`.elc`/`.eln` suffix from a name."""
    name = re.sub(r"\s*\((source|compiled[^)]*|module|native-compiled[^)]*)\)\s*$", "", name)
    if "/lisp/" in name:
        name = name.split("/lisp/", 1)[1]
    for suffix in (".eln", ".elc", ".el"):
        if name.endswith(suffix):
            name = name[:-len(suffix)]
            break
    return name


def run(cmd: list[str], env: dict[str, str] | None, timeout: int) -> tuple[list[str], int, str]:
    """Run one loadup and return (loaded names in order, exit status, stderr tail)."""
    result = subprocess.run(cmd, capture_output=True, text=True, timeout=timeout, env=env)
    names = [normalise(m.group(1)) for m in
             (LOADING.match(line) for line in result.stderr.splitlines()) if m]
    reached = MARKER in result.stdout
    return names, result.returncode, ("" if reached else result.stderr[-1500:])


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--reference", type=pathlib.Path, default=BUILD / "emacs-ref/src/temacs")
    parser.add_argument("--port", type=pathlib.Path, default=BUILD / "port/src/temacs")
    parser.add_argument("--lisp", type=pathlib.Path, default=REPO / "femacs/lisp",
                        help="the port's Lisp tree (its load-path root)")
    parser.add_argument("--reference-lisp", type=pathlib.Path, default=REPO / "emacs/lisp",
                        help="the reference's Lisp tree (its load-path root)")
    parser.add_argument("--show", type=int, default=12, help="how many divergences to print")
    parser.add_argument("--timeout", type=int, default=600, help="seconds per binary")
    args = parser.parse_args()

    for binary in (args.reference, args.port):
        if not binary.is_file():
            raise SystemExit(f"{binary} is missing: build it first")

    import os
    src = args.lisp.resolve()
    dirs = [src] + [src / d for d in ("emacs-lisp", "progmodes", "language", "international",
                                      "textmodes", "vc")]
    env = dict(os.environ, EMACSLOADPATH=":".join(str(d) for d in dirs))
    ref_cmd = [str(args.reference), "-batch", "-l", "loadup",
               "--eval", f'(princ "{MARKER}\\n")']
    ref_env = dict(env, EMACSLOADPATH=str(args.reference_lisp.resolve()))
    port_cmd = [str(args.port), "-Q", "--batch", "--eval", f'(princ "{MARKER}\\n")']

    ref_names, ref_status, ref_tail = run(ref_cmd, ref_env, args.timeout)
    port_names, port_status, port_tail = run(port_cmd, env, args.timeout)

    print(f"loaded files: reference {len(ref_names)}, port {len(port_names)}")
    only_reference = [n for n in ref_names if n not in set(port_names)]
    only_port = [n for n in port_names if n not in set(ref_names)]
    print(f"only in the reference {len(only_reference)}, only in the port {len(only_port)}")
    for name in only_reference[:args.show]:
        print(f"  missing from the port: {name}")
    if len(only_reference) > args.show:
        print(f"  ... {len(only_reference) - args.show} more missing")
    for name in only_port[:args.show]:
        print(f"  extra in the port: {name}")

    for label, tail in (("reference", ref_tail), ("port", port_tail)):
        if tail:
            print(f"{label} did not reach the end of loadup.el:")
            for line in tail.strip().splitlines()[-6:]:
                print(f"  {line}")

    print(f"exit status: reference {ref_status}, port {port_status}")
    if not ref_tail and not port_tail and ref_names == port_names:
        print("the two loadups agree file for file")
        return 0
    return 1


if __name__ == "__main__":
    sys.exit(main())
