#!/usr/bin/env python3
"""Fail on a bare `0`-prefixed integer literal in the ported Rust.

C writes octal as a leading zero (`040`, `0300`, `0644`); Rust reads those digits as *decimal*
(`040` is forty), so a literal copied across keeps its digits and changes its value.  status
3.633 found the whole 168-entry bytecode opcode table that way (160 values wrong: `Bcall` was
44 instead of 32, so every call dispatched to the wrong arm, leaked a stack slot and finally
wrote a Lisp_Object over a frame's `saved_fp`).  The port's spelling is `0o` + the same digits
(`0o40`), which is what `PORTING.md` P-94 requires; this gate is what says so.

    python3 tools/check-octal-literals.py            # scan the ported trees
    python3 tools/check-octal-literals.py --selftest # prove the scanner's cases

A site is a *code* literal: comments (`//`, `/* */`, nesting like C++), string and char literals,
`r#"..."#` and `c"..."` are skipped, so prose about `P-05` and dates in comments do not count.
Exit 1 lists the sites; 0 prints the coverage line.
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
TREES = ("femacs/src", "femacs/lib", "femacs/lib-src", "femacs/lwlib", "femacs/oldXMenu",
         "femacs/nt", "femacs/exec", "femacs/xtask")
LITERAL = re.compile(r"(?<![0-9A-Za-z_.])0[0-7]+\b")


def code_spans(text: str):
    """Yield `(start, end)` of every comment/string/char span, so literals can be found in code."""
    i, n = 0, len(text)
    while i < n:
        c = text[i]
        if c == "/" and i + 1 < n and text[i + 1] == "/":
            j = text.find("\n", i)
            j = n if j < 0 else j
            yield i, j
            i = j
        elif c == "/" and i + 1 < n and text[i + 1] == "*":
            depth, j = 1, i + 2
            while j < n and depth:
                if text.startswith("/*", j):
                    depth, j = depth + 1, j + 2
                elif text.startswith("*/", j):
                    depth, j = depth - 1, j + 2
                else:
                    j += 1
            yield i, j
            i = j
        elif c in "\"'" or (c in "rc" and i + 1 < n and text[i + 1] in "\"#"):
            q = '"' if text[i] == 'c' or c == '"' else c
            if c in "rc" and (text[i + 1] == "#" or text[i + 1] == '"'):
                hashes = 0
                j = i + 1
                if text[j] == "#":
                    while j < n and text[j] == "#":
                        hashes, j = hashes + 1, j + 1
                if j < n and text[j] == '"':
                    closer = '"' + "#" * hashes
                    j = text.find(closer, j + 1)
                    j = n if j < 0 else j + len(closer)
                    yield i, j
                    i = j
                    continue
            j = i + 1
            while j < n:
                if text[j] == "\\":
                    j += 2
                    continue
                if text[j] == q:
                    j += 1
                    break
                if text[j] == "\n" and q == "'":      # a lifetime, not a char literal
                    break
                j += 1
            if j <= i + 1:
                i += 1
                continue
            yield i, j
            i = j
        else:
            i += 1


def scan(path: Path) -> list[tuple[int, str]]:
    text = path.read_text(encoding="utf-8", errors="replace")
    masked = list(text)
    for a, b in code_spans(text):
        for k in range(a, min(b, len(masked))):
            masked[k] = " "
    code = "".join(masked)
    out = []
    for m in LITERAL.finditer(code):
        line = text.count("\n", 0, m.start()) + 1
        out.append((line, text.splitlines()[line - 1].strip()[:100]))
    return out


SELFTEST = [
    ("let x = 0644;\n", 1),
    ("// 0644 in a comment\n", 0),
    ("/* P-05 and 0300 */\n", 0),
    ('let s = "0600";\n', 0),
    ('let s = c"0300";\n', 0),
    ('let s = r#"0077"#;\n', 0),
    ("let x = 0o644;\n", 0),
    ("let x = 0;\n", 0),
    ("let x = 0x40;\n", 0),
    ('let c = \'a\';\nlet y = 077;\n', 1),
]


def selftest() -> int:
    bad = 0
    for src, want in SELFTEST:
        got = len(scan_text(src))
        if got != want:
            print(f"FAIL  {src!r}: {got} site(s), want {want}")
            bad += 1
    print(f"selftest: {len(SELFTEST)} case(s), {bad} failure(s)")
    return 1 if bad else 0


def scan_text(text: str) -> list[tuple[int, str]]:
    masked = list(text)
    for a, b in code_spans(text):
        for k in range(a, min(b, len(masked))):
            masked[k] = " "
    code = "".join(masked)
    out = []
    for m in LITERAL.finditer(code):
        line = text.count("\n", 0, m.start()) + 1
        out.append((line, text.splitlines()[line - 1].strip()[:100]))
    return out


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--selftest", action="store_true")
    ap.add_argument("trees", nargs="*", default=list(TREES))
    args = ap.parse_args()
    if args.selftest:
        return selftest()
    files = sites = 0
    for tree in args.trees:
        root = REPO / tree
        if not root.is_dir():
            continue
        for p in sorted(root.rglob("*.rs")):
            files += 1
            hits = scan(p)
            for line, text in hits:
                sites += 1
                print(f"{p.relative_to(REPO)}:{line}: {text}")
    print(f"read {files} .rs file(s): {sites} bare 0-prefixed literal site(s)")
    return 1 if sites else 0


if __name__ == "__main__":
    sys.exit(main())
