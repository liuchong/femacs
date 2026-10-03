#!/usr/bin/env python3
"""Verify that no ported file carries a product-name declaration (PORTING.md 0.1-19).

The version/copyright declaration a C file opens with says "this file is part of
<product>".  Copied into this repository it is a false statement about *this*
repository, and a version line naming the product ("... @ <rev> (<product> x.y.z)")
reads as a claim that this program *is* that product.  Two user instructions
(2026-10-03) removed both from the tree: `2ee1d1d`/`4810bb0` rewrote 231 port
headers down to the revision, and `fd9cf89` deleted the 268 copied declaration
blocks.  Prose cannot keep that from coming back: every unit ported afterwards
copies its C file's comments, and `src/emacs.c`'s own product-name literals are
the worked example of the one class that *stays*.

    python3 tools/check-declarations.py                  # scan, compare with the allowlist
    python3 tools/check-declarations.py --selftest       # prove the scanner's cases
    python3 tools/check-declarations.py --write-allowlist  # rewrite the allowlist from the tree

Three classes are reported, each by `<path>\t<class>\t<count>\t<line>\t<reason>`:

  decl     the copied ownership/licence prose (FSF, "General Public License",
           "part of GNU", "is free software", "Copyright (C)") inside a comment.
           Every one of the 268 blocks 3.58 deleted was this class.
  version  the parenthetical 0.1-19 removed: `@ <rev> (<product> x.y.z)`.
  title    the product name in any *other* comment - a copied first-line title
           ("... subroutines for GNU Emacs.") or a doc comment quoting one of
           C's lines.
  literal  the product name inside a *string literal*.  These are ported
           semantics - C's own literals, which the docstring gate compares with
           the reference - so they are the class that is expected to exist.

Every site, of every class, must be named in
`docs/baseline/declarations-allowlist.tsv` with a reason, the way
`symbol-pending.txt` counts a deviation instead of hiding it; the three classes
that are statements about *this* file are expected to have no rows at all.

Comments and strings are separated by the scanner below (nestable `/* */`, raw
and `c"` strings, lifetimes versus char literals), because a rule that cannot tell
a comment from a literal would either miss the copied blocks or fire on every
docstring.  Exit status 0 means the tree matches the allowlist exactly.
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
SCAN_DIRS = ("femacs/src", "femacs/lib", "femacs/xtask")
ALLOWLIST = REPO / "docs/baseline/declarations-allowlist.tsv"

# The prose every copied declaration block contains, and that no port file may
# carry in any form (a comment or a literal - the classes below say which).
DECL_PROSE = (
    "Free Software Foundation",
    "General Public License",
    "is free software",
    "along with GNU",
    "Copyright (C)",
)
PRODUCT = ("GNU Emacs", "gnu emacs")
# `// femacs: port of src/buffer.c @ fd7afced403 (GNU Emacs 32.0.50).` - the form
# 0.1-19 removed.  The rule keeps the revision and drops the parenthetical.
# The removed form: the revision followed by a parenthetical that names the
# product.  `@ fd7afced403 (gnulib)` / `(PORTING.md P-26)` are provenance notes
# about where the file came from, not version declarations, and stay.
VERSION_PAREN = re.compile(r"@\s*[0-9a-f]{7,40}\s*\([^)]*(?:GNU Emacs|gnu emacs)")


def scan(text: str) -> tuple[list[str], list[str]]:
    """Split Rust source into per-line code and per-line comment text.

    One entry per physical line in both lists - the whole point of the split is
    that the gate can say *which* line of *which* file carries the product name.
    Only the two things it asks about are modelled: what is inside a comment and
    what is inside a literal.  `'a` (a lifetime) is code, `'a'` (a char literal)
    is a literal; `r#"…"#` and `c"…"` are literals too.
    """
    code: list[str] = []
    comment: list[str] = []
    code_line: list[str] = []
    comment_line: list[str] = []
    i, n = 0, len(text)
    state = "code"          # code | line | block | string | char | raw
    raw_hashes = 0
    block_depth = 0
    while i < n:
        ch = text[i]
        if ch == "\n":
            code.append("".join(code_line))
            comment.append("".join(comment_line))
            code_line, comment_line = [], []
            if state in ("line", "char"):
                state = "code"      # neither spans a line
            i += 1
            continue
        if state == "code":
            if ch == "/" and text[i + 1:i + 2] == "/":
                state = "line"
                i += 2
                continue
            if ch == "/" and text[i + 1:i + 2] == "*":
                state, block_depth = "block", 1
                i += 2
                continue
            if ch == "r" or (ch == "b" and text[i + 1:i + 2] == "r"):
                j = i + 1 if ch == "r" else i + 2
                hashes = 0
                while text[j:j + 1] == "#":
                    hashes += 1
                    j += 1
                if text[j:j + 1] == '"':
                    state, raw_hashes = "raw", hashes
                    i = j + 1
                    continue
            if ch == '"' or (ch == "c" and text[i + 1:i + 2] == '"'):
                state = "string"
                i += 2 if ch == "c" else 1
                continue
            if ch == "'" or (ch == "b" and text[i + 1:i + 2] == "'"):
                j = i + (2 if ch == "b" else 1)
                k, closed = j, False
                while k < n and text[k] != "\n" and k - j < 12:
                    if text[k] == "\\":
                        k += 2
                        continue
                    if text[k] == "'":
                        closed = True
                        break
                    k += 1
                if closed:
                    state = "char"
            code_line.append(ch)
            i += 1
            continue
        if state == "line":
            comment_line.append(ch)
            i += 1
            continue
        if state == "block":
            if ch == "/" and text[i + 1:i + 2] == "*":
                block_depth += 1
                i += 2
                continue
            if ch == "*" and text[i + 1:i + 2] == "/":
                block_depth -= 1
                i += 2
                if block_depth == 0:
                    state = "code"
                continue
            comment_line.append(ch)
            i += 1
            continue
        if state == "raw":
            if ch == '"' and text[i + 1:i + 1 + raw_hashes] == "#" * raw_hashes:
                i += 1 + raw_hashes
                state = "code"
                continue
            code_line.append(ch)
            i += 1
            continue
        # string | char
        code_line.append(ch)
        if ch == "\\":
            code_line.append(text[i + 1:i + 2])
            i += 2
            continue
        if (state == "string" and ch == '"') or (state == "char" and ch == "'"):
            state = "code"
        i += 1
    code.append("".join(code_line))
    comment.append("".join(comment_line))
    return code, comment


def classify(code: str, comment: str) -> str | None:
    """The class of the first declaration on this line, or None.

    Position decides: prose inside a *literal* is ported semantics (C's own
    string, compared with the reference by the docstring gate) and is classed
    `literal` even when it is the copyright line, while prose inside a comment is
    a statement *about this file* and is classed `decl` (the ownership block),
    `version` (the removed parenthetical) or `title` (any other product name).
    """
    if any(p in code for p in DECL_PROSE) or any(p in code for p in PRODUCT):
        return "literal"
    if VERSION_PAREN.search(comment):
        return "version"
    if any(p in comment for p in DECL_PROSE):
        return "decl"
    if any(p in comment for p in PRODUCT):
        return "title"
    return None


def findings(root: Path) -> list[tuple[str, str, str]]:
    """`(path, class, line)` for every declaration this gate knows about."""
    out: list[tuple[str, str, str]] = []
    for base in SCAN_DIRS:
        for path in sorted((root / base).rglob("*.rs")):
            rel = path.relative_to(root).as_posix()
            text = path.read_text(encoding="utf-8", errors="replace")
            code, comment = scan(text)
            raw_lines = text.split("\n")
            for line_no in range(max(len(code), len(comment))):
                c = comment[line_no] if line_no < len(comment) else ""
                k = code[line_no] if line_no < len(code) else ""
                # The raw line is the key, not the code half: the report has to show
                # the line a reader will open in an editor, quotes and all.
                snippet = " ".join(raw_lines[line_no].split())[:240] if line_no < len(raw_lines) else ""
                klass = classify(k, c)
                if klass:
                    out.append((rel, klass, snippet))
    return out


def tally(rows: list[tuple[str, str, str]]) -> dict[tuple[str, str, str], int]:
    counted: dict[tuple[str, str, str], int] = {}
    for path, klass, line in rows:
        key = (path, klass, line)
        counted[key] = counted.get(key, 0) + 1
    return counted


def read_allowlist(path: Path) -> dict[tuple[str, str, str], int]:
    allowed: dict[tuple[str, str, str], int] = {}
    if not path.is_file():
        return allowed
    for raw in path.read_text(encoding="utf-8").splitlines():
        if not raw.strip() or raw.startswith("#"):
            continue
        parts = raw.split("\t")
        if len(parts) < 4 or not parts[2].strip().isdigit():
            if not parts[0] == "path":       # the column-name row
                print(f"BAD ROW {path.name}: {raw!r}")
            continue
        allowed[(parts[0], parts[1], parts[3])] = int(parts[2])
    return allowed


def write_allowlist(path: Path, counted: dict[tuple[str, str, str], int]) -> None:
    rows = sorted(counted.items(), key=lambda kv: (kv[0][0], kv[0][1], kv[0][2]))
    body = ["# Product-name occurrences inside ported string literals, one per line",
            "# (docs/status.md 3.73).  PORTING.md 0.1-19 forbids the product name in a",
            "# comment, a version line or a copied declaration block (`tools/check-declarations.py`",
            "# fails on those outright); a *literal* is ported semantics - C's own string -",
            "# and the docstring gate compares it with the reference, so it is counted here,",
            "# with the reason the literal cannot be changed.  Columns:",
            "# <path>\\t<class>\\t<count>\\t<line>\\t<reason>.  A count that no longer matches,",
            "# a row nothing matches, and a finding with no row are all failures.",
            "path\tclass\tcount\tline\treason"]
    for (p, klass, line), n in rows:
        body.append(f"{p}\t{klass}\t{n}\t{line}\tTODO")
    path.write_text("\n".join(body) + "\n", encoding="utf-8")


SELFTEST = (
    # (name, source, expected classes)
    # A copied block reports both classes: its first line names the product (title)
    # and its body carries the ownership prose (decl).
    ("copied block", "/* This file is part of GNU Emacs.\n   Copyright (C) 2026 */",
     ["decl", "title"]),
    ("version line", "// femacs: port of src/x.c @ fd7afced403 (GNU Emacs 32.0.50).", ["version"]),
    ("title comment", "/* X window system support.  */\n/* Cursor motion subroutines for GNU Emacs.  */",
     ["title"]),
    ("literal", 'const PACKAGE_NAME: &CStr = c"GNU Emacs";', ["literal"]),
    ("copyright literal", 'const COPYRIGHT: &CStr = c"Copyright (C) 2026 Free Software Foundation, Inc.";',
     ["literal"]),
    ("provenance note is not a version line", "// femacs: port of lib/x.c @ fd7afced403 (gnulib).", []),
    ("rule note is not a version line", "// femacs: mirror of lib/regex.h @ fd7afced403 (PORTING.md 0.1-2).", []),
    ("docstring literal", 'defsubr! (Sfoo, Foo, 0, 1, doc: "Report GNU Emacs bugs.");',
     ["literal"]),
    ("lifetime is not a char literal", "fn f<'a> (x: &'a str) -> &'a str { x }", []),
    ("raw string is code", 'let s = r#"no product here"#;', []),
    ("nested block comment", "/* a /* b */ /* c */ */\n/* for GNU Emacs.  */", ["title"]),
    ("plain port line", "// femacs: port of src/buffer.c @ fd7afced403.", []),
    ("emacs path is not the product", "// femacs: mirror of emacs/src/lisp.h", []),
)


def selftest() -> int:
    bad = 0
    for name, source, want in SELFTEST:
        got = sorted({k for _, k, _ in findings_text(source)})
        ok = got == sorted(want)
        print(f"{'ok  ' if ok else 'FAIL'} {name}: {got or 'none'}")
        bad += 0 if ok else 1
    print(f"{len(SELFTEST)} cases, {bad} failures")
    return 1 if bad else 0


def findings_text(text: str) -> list[tuple[str, str, str]]:
    code, comment = scan(text)
    raw_lines = text.split("\n")
    out = []
    for line_no in range(max(len(code), len(comment))):
        c = comment[line_no] if line_no < len(comment) else ""
        k = code[line_no] if line_no < len(code) else ""
        snippet = " ".join(raw_lines[line_no].split())[:240] if line_no < len(raw_lines) else ""
        klass = classify(k, c)
        if klass:
            out.append(("<test>", klass, snippet))
    return out


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--root", default=str(REPO), help="repository root to scan")
    ap.add_argument("--write-allowlist", action="store_true",
                    help="rewrite docs/baseline/declarations-allowlist.tsv from the tree")
    ap.add_argument("--selftest", action="store_true", help="run the scanner's own cases")
    args = ap.parse_args()
    if args.selftest:
        return selftest()

    root = Path(args.root)
    counted = tally(findings(root))
    if args.write_allowlist:
        write_allowlist(ALLOWLIST, counted)
        print(f"wrote {ALLOWLIST.relative_to(REPO)}: {len(counted)} rows")
        return 0

    allowed = read_allowlist(ALLOWLIST)
    failures = 0
    for key in sorted(set(counted) - set(allowed)):
        path, klass, line = key
        print(f"FAIL {path}: {klass} x{counted[key]}: {line}")
        failures += 1
    for key in sorted(set(allowed) - set(counted)):
        path, klass, line = key
        print(f"STALE {path}: {klass}: {line} (nothing in the tree matches it)")
        failures += 1
    for key in sorted(set(allowed) & set(counted)):
        if allowed[key] != counted[key]:
            path, klass, line = key
            print(f"FAIL {path}: {klass} x{counted[key]}, allowlist says x{allowed[key]}: {line}")
            failures += 1
    decls = sum(n for (_, k, _), n in counted.items() if k in ("decl", "version", "title"))
    literals = sum(n for (_, k, _), n in counted.items() if k == "literal")
    print(f"{len(counted)} declaration sites ({decls} forbidden, {literals} allowed literals), "
          f"{failures} failures")
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
