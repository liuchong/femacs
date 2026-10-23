#!/usr/bin/env python3
"""Every string handed to C through `.as_ptr ()` must be NUL-terminated, as C's literals are.

Two defects of this shape have already been found by hand: `lib/strftime.rs`'s `L_!` lost C's
terminator (status 4.398: the subformat pointer read `"%H:%M:%S%Y-%m-%d%m/%d/%y"`, the literals
being packed back to back in rodata), and `src/region-cache.rs`'s two `fprintf` format strings
(4.399) were plain Rust literals with no terminator at all.  Both are invisible to the ABI, symbol
and token gates, and both are *text* defects, which is what this gate reads.

The rule, read off the tree's own house style:

  * `c"...".as_ptr ()` - accepted; the `c` literal carries the terminator.  This is the form the
    port uses almost everywhere (11,000+ sites).
  * `b"...\\0".as_ptr ()` - accepted; the terminator is written out.
  * `"...".as_ptr ()` with no trailing `\\0` - a defect: the pointer is a `&str`'s, and whatever
    byte follows the literal in rodata is read as part of the C string.

A site that genuinely wants a Rust `&str`'s pointer (a length-bounded call, say) is registered in
`docs/baseline/c-string-literals.tsv` with a reason, and the gate prints how many are registered.

    python3 tools/check-c-string-literals.py            # every unit under the port's trees
    python3 tools/check-c-string-literals.py --list     # every site, classified
    python3 tools/check-c-string-literals.py --selftest
"""

from __future__ import annotations

import argparse
import pathlib
import re
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
# The port's own source trees (the copied trees carry no Rust).
ROOTS = ("src", "lib", "lib-src", "lwlib", "oldXMenu",
         "exec", "nt", "msdos", "java")
BASELINE = REPO / "docs/baseline/c-string-literals.tsv"

# A literal immediately before `.as_ptr ()`: the `c`/`b` prefix is captured, and a literal that is
# not directly followed by `.as_ptr` (a `concat!`, a macro argument, a variable) is not a match -
# those are the forms the two accepted spellings expand to.
SITE = re.compile(r'(?<![A-Za-z0-9_])([cb]?)("(?:[^"\\]|\\.)*")(\s*)\.as_ptr\s*\(\s*\)')


def classify(prefix: str, literal: str) -> str:
    """`c-string` / `byte-string` / `terminated` / `defect` for one site."""
    if prefix == "c":
        return "c-string"
    if literal.endswith('\\0"'):
        return "terminated"
    if prefix == "b":
        return "byte-string-defect"
    return "defect"


def baseline() -> dict[str, str]:
    """The registered exceptions: `path:line` -> reason (the file's third column)."""
    rows: dict[str, str] = {}
    if not BASELINE.is_file():
        return rows
    for line in BASELINE.read_text(encoding="utf-8").splitlines():
        if not line.strip() or line.startswith("#") or line.startswith("path\t"):
            continue
        parts = line.split("\t")
        if len(parts) >= 3:
            rows[parts[0]] = parts[2]
    return rows


def scan(units: list[pathlib.Path]) -> list[tuple[str, int, str, str]]:
    """(path, line, kind, literal) for every site, in file and line order."""
    found: list[tuple[str, int, str, str]] = []
    for unit in units:
        text = unit.read_text(encoding="utf-8", errors="replace")
        for m in SITE.finditer(text):
            line = text[:m.start()].count("\n") + 1
            rel = str(unit.relative_to(REPO))
            found.append((rel, line, classify(m.group(1), m.group(2)), m.group(2)))
    return found


def units() -> list[pathlib.Path]:
    """Every `.rs` unit under the port's own trees, in a stable order."""
    found: list[pathlib.Path] = []
    for root in ROOTS:
        path = REPO / root
        if path.is_dir():
            found.extend(sorted(path.rglob("*.rs")))
    return found


def selftest() -> int:
    """The classifier, on the shapes the gate has to tell apart."""
    cases = [
        ('fprintf (stderr, c"%s\\n".as_ptr (), x);', "c-string"),
        ('write (1, b"abc\\0".as_ptr (), 4);', "terminated"),
        ('write (1, b"abc".as_ptr (), 3);', "byte-string-defect"),
        ('fprintf (stderr, "%s\\n".as_ptr (), x);', "defect"),
        ('let p = buffer.as_ptr ();', None),          # a variable: no site at all
        ('let p = concat! ("a", "\\0").as_ptr ();', None),
    ]
    failures = 0
    for text, want in cases:
        m = SITE.search(text)
        got = classify(m.group(1), m.group(2)) if m else None
        if got != want:
            print(f"selftest FAIL: {text!r}: expected {want}, got {got}")
            failures += 1
    print(f"selftest: {len(cases)} case(s), {failures} failure(s)")
    return 1 if failures else 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--selftest", action="store_true", help="check the classifier and exit")
    parser.add_argument("--list", action="store_true", help="list every site, classified")
    parser.add_argument("--show", type=int, default=20, help="defects to print")
    args = parser.parse_args()

    if args.selftest:
        return selftest()

    sites = scan(units())
    counts: dict[str, int] = {}
    for _, _, kind, _ in sites:
        counts[kind] = counts.get(kind, 0) + 1

    allowed = baseline()
    defects = [(p, n, lit) for p, n, kind, lit in sites
               if kind in ("defect", "byte-string-defect") and f"{p}:{n}" not in allowed]

    if args.list:
        for path, line, kind, literal in sites:
            print(f"{kind:20} {path}:{line}: {literal[:70]}")

    scale = ", ".join(f"{k} {v}" for k, v in sorted(counts.items()))
    print(f"{len(sites)} site(s): {scale}; registered in the baseline: {len(allowed)}")
    if defects:
        print(f"{len(defects)} site(s) hand a C string a literal with no terminator:")
        for path, line, literal in defects[:args.show]:
            print(f"  {path}:{line}: {literal[:80]}")
        return 1
    print("c string literals: every site is terminated (c\"...\" or an explicit \\0)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
