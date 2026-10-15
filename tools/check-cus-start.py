#!/usr/bin/env python3
"""Compare the port's `cus-start.el` effects with the reference's.

`cus-start.el` gives every variable that C defines (`builtin-cus-vars`, 208 entries) its
Custom properties, and - via `:standard`/`:initialize custom-initialize-delay` - its *value*:
the C code binds such a variable to nil and the value is computed at startup from
`custom-delayed-init-variables` (`custom-reevaluate-setting` in `startup.el`).  A port whose
`cus-start.el` does not take effect therefore has nil values for a whole family of variables
(`temporary-file-directory`, `create-lockfiles`, `exec-path`, ...), which is a symptom far from
its cause, so this tool reads the family directly:

    python3 tools/check-cus-start.py                       # reference vs port, default binaries
    python3 tools/check-cus-start.py --port build/port/src/temacs
    python3 tools/check-cus-start.py --names               # just print the symbol list

The readings are per-symbol `standard-value` presence, taken by loading one generated probe file
into each binary's *post-startup* state; the port is expected to match the reference entry for
entry.  A divergence is printed as `symbol: reference=<sv|-> port=<sv|->` for the first
`--show` (default 20) entries plus the total count; the exit status is 0 only when the two
readings agree everywhere.
"""

from __future__ import annotations

import argparse
import pathlib
import re
import subprocess
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
REF = REPO / "emacs"
BUILD = REPO / "build"

# The list is introduced by `(builtin-cus-vars` and holds one `(SYMBOL GROUP TYPE VERSION REST...)`
# group per variable; comments, strings and the `,`/`,@` unquotes inside the groups have to be
# skipped when walking it.
LIST_START = "(builtin-cus-vars"
GROUP_HEAD = re.compile(r"\(\s*([^\s()]+)")


def _walk_balanced(text: str, start: int) -> int:
    """Return the index of the paren matching `text[start] == '('`."""
    depth, i, n = 0, start, len(text)
    while i < n:
        c = text[i]
        if c == "\\":
            i += 2
            continue
        if c == '"':
            i += 1
            while text[i] != '"':
                i += 2 if text[i] == "\\" else 1
            i += 1
            continue
        if c == ";":
            nl = text.find("\n", i)
            i = n if nl < 0 else nl + 1
            continue
        if c == "(":
            depth += 1
        elif c == ")":
            depth -= 1
            if depth == 0:
                return i
        i += 1
    raise ValueError("unbalanced parentheses")


def symbols(path: pathlib.Path) -> list[str]:
    """The `builtin-cus-vars` symbol names of `cus-start.el`."""
    text = path.read_text()
    start = text.index(LIST_START)
    i = text.index("`(", start)
    body = text[i + 2:_walk_balanced(text, i + 1)]

    names: list[str] = []
    depth, j, n = 0, 0, len(body)
    while j < n:
        c = body[j]
        if c == "\\":
            j += 2
            continue
        if c == '"':
            j += 1
            while body[j] != '"':
                j += 2 if body[j] == "\\" else 1
            j += 1
            continue
        if c == ";":
            nl = body.find("\n", j)
            j = n if nl < 0 else nl + 1
            continue
        if c == "(":
            depth += 1
            if depth == 1:
                head = GROUP_HEAD.match(body, j)
                if head:
                    names.append(head.group(1))
        elif c == ")":
            depth -= 1
        j += 1
    return names


def write_probe(names: list[str], path: pathlib.Path) -> None:
    """Write the probe file that prints one `NAME=<sv|->` line per symbol."""
    with path.open("w") as f:
        f.write(";;; -*- lexical-binding: t -*-\n")
        for name in names:
            f.write(f'(princ (format "{name}=%s\\n" '
                    f'(if (get (quote {name}) (quote standard-value)) "sv" "-")))\n')


def read_probe(binary: pathlib.Path, probe: pathlib.Path, timeout: int) -> dict[str, str]:
    """Load the probe file into `binary` and collect its `NAME=sv|-` readings."""
    if not binary.is_file():
        raise SystemExit(f"{binary} is missing: build it first")
    result = subprocess.run([str(binary), "-Q", "--batch", "-l", str(probe)],
                            capture_output=True, text=True, timeout=timeout)
    readings: dict[str, str] = {}
    for line in result.stdout.splitlines():
        m = re.fullmatch(r"([a-z0-9-]+)=(sv|-)", line)
        if m:
            readings[m.group(1)] = m.group(2)
    if not readings:
        sys.stderr.write(f"{binary}: the probe produced no readings "
                         f"(exit {result.returncode})\n")
        sys.stderr.write(result.stderr[-2000:])
        raise SystemExit(1)
    return readings


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--reference", type=pathlib.Path,
                        default=BUILD / "emacs-ref/src/emacs",
                        help="the reference binary (default: the frozen build's emacs)")
    parser.add_argument("--port", type=pathlib.Path, default=BUILD / "port/src/temacs",
                        help="the port binary (default: build/port/src/temacs)")
    parser.add_argument("--reference-lisp", type=pathlib.Path, default=REF / "lisp/cus-start.el",
                        help="the reference's cus-start.el (the symbol list's source)")
    parser.add_argument("--probe", type=pathlib.Path, default=BUILD / "tools/cus-start-probe.el",
                        help="where to write the generated probe file")
    parser.add_argument("--names", action="store_true", help="print the symbol list and exit")
    parser.add_argument("--show", type=int, default=20, help="how many divergences to print")
    parser.add_argument("--timeout", type=int, default=600, help="seconds per binary")
    args = parser.parse_args()

    names = symbols(args.reference_lisp)
    if args.names:
        for name in names:
            print(name)
        return 0

    args.probe.parent.mkdir(parents=True, exist_ok=True)
    write_probe(names, args.probe)

    reference = read_probe(args.reference, args.probe, args.timeout)
    port = read_probe(args.port, args.probe, args.timeout)

    missing = [n for n in names if n not in port]
    if missing:
        print(f"the port's probe did not report {len(missing)} symbol(s): {missing[:8]}")
        return 1

    divergences = [n for n in names if reference.get(n) != port.get(n)]
    sv_reference = sum(1 for n in names if reference.get(n) == "sv")
    sv_port = sum(1 for n in names if port.get(n) == "sv")
    print(f"{len(names)} built-in variable(s); standard-value present: "
          f"reference {sv_reference}, port {sv_port}, divergent {len(divergences)}")
    for name in divergences[:args.show]:
        print(f"  {name}: reference={reference.get(name, '?')} port={port.get(name, '?')}")
    if len(divergences) > args.show:
        print(f"  ... {len(divergences) - args.show} more")
    return 1 if divergences else 0


if __name__ == "__main__":
    sys.exit(main())
