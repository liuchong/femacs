#!/usr/bin/env python3
"""Verify that no ported file carries a product-name declaration (PORTING.md 0.1-19).

The version/copyright declaration a C file opens with says "this file is part of
<product>".  Copied into this repository it is a false statement about *this*
repository, and a version line naming the product ("... @ <rev> (<product> x.y.z)")
reads as a claim that this program *is* that product.  User instructions
(2026-10-03/04) removed both from the tree: `2ee1d1d`/`4810bb0` rewrote 231 port
headers down to the revision, and `fd9cf89` deleted the 268 copied declaration
blocks.  Prose cannot keep that from coming back: every unit ported afterwards
copies its C file's comments, and `src/emacs.c`'s own product-name literals are
the worked example of the one class that *stays*.

    python3 tools/check-declarations.py                  # scan, compare with the allowlist
    python3 tools/check-declarations.py --selftest       # prove the scanner's cases
    python3 tools/check-declarations.py --write-allowlist  # rewrite the allowlist from the tree

Four classes are reported, each by `<path>\t<class>\t<count>\t<line>\t<reason>`.  Every
ported tree is scanned - `femacs/src` and `femacs/lib` plus batch F's own programs and
libraries (`lib-src`, `exec`, `lwlib`, `oldXMenu`, `nt`) and its non-C platform trees
(`femacs/platform`: the Swift/Kotlin/C++ targets of `src/*.m`, `java/`, `haiku_*.cc`);
`femacs/xtask` is this gate's own
source, scanned for the first three classes and exempt from the fourth:

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
  recital  a comment that *recites* what was done with the reference's opening
           block or its product name - "C's LGPL header", "the licence block is
           not carried", "the product name out of every comment".  A licence
           remark about this repository is a statement about this repository
           wherever it is written, so it belongs in docs/status.md and the
           file's ledger, never in the ported unit (status 3.90, 3.100).

`docs/ports/*.md` - the per-unit ledgers - are scanned too, for the `decl`,
`version` and `title` classes (`recital` never: a ledger is the *sanctioned* home
for a licence remark, status 3.90/3.100).  A ledger quotes C, so it is exactly
where the reference's opening block or its version parenthetical can come back
without a ported unit ever carrying it; 3.100 found `lib-src/hexl.rs` doing that
in a tree no gate read, and a ledger is that tree's successor.

Every site, of every class, must be named in
`docs/baseline/declarations-allowlist.tsv` with a reason, the way
`symbol-pending.txt` counts a deviation instead of hiding it; the four classes
that are statements about *this* repository are expected to have no rows at all.

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
# Every ported tree, not just the two the first 350 units came from: batch F ports
# programs and libraries of their own (`lib-src`, `exec`, `lwlib`, `oldXMenu`, `nt`),
# and 3.98's `lib-src/hexl.rs` showed the omission mattered - that file recited the
# reference block's provenance sentence and no gate read it (status 3.100).  `femacs/xtask`
# is this gate's own source, not a port.
SCAN_DIRS = ("femacs/src", "femacs/lib", "femacs/lib-src", "femacs/exec", "femacs/lwlib",
             "femacs/oldXMenu", "femacs/nt", "femacs/msdos",
             # batch F's non-C platform trees (src/*.m -> Swift, java/ -> Kotlin,
             # haiku_*.cc -> C++).  They do not exist yet; naming them here is what makes
             # the rule bind the first file written into them rather than the first review.
             "femacs/platform", "femacs/xtask")
# The recital rule binds the ported units and mirrors; `femacs/xtask` explains the rule in
# its own comments and is outside it (a licence remark about this repository belongs in
# docs/status.md and the ledger, never in a ported unit).
RECITAL_TREES = SCAN_DIRS[:-1]
# The ledgers of our own (see the module docstring).  Markdown has no literals, so only the
# three prose classes can be re-introduced there, and a quoted C string that carries the
# product name is prose in this tree, not ported semantics - the ported unit is where the
# literal lives and where it is allowlisted.
DOC_DIRS = ("docs/ports",)
DOC_CLASSES = ("decl", "version", "title")
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
# 3.90 emptied the comments that *recite* what was done with the reference's opening block
# ("C's LGPL header", "the licence block is not carried", "the product name out of every
# comment"), and the class returned within two rounds: 3.98's `lib-src/hexl.rs` was written
# with a four-line justification of exactly that kind, in a tree no gate scanned.  The words
# are the signature, so the rule reads them - a licence/copyright word, or talk about the
# product name, inside a comment of a ported file is a finding.
#
# What keeps the rule usable is what it refuses to match.  `emacs_copyright`,
# `emacs-copyright`, `` `COPYRIGHT` `` and `canonicalize-lgpl.rs` are ported *names*, not
# prose, so the words are matched case-sensitively (C's prose writes "Copyright", C's object
# is `COPYRIGHT`) and carry an identifier guard on both sides.  `LGPL`/`GPL` are acronyms and
# uppercase-only, and count only as bare words: `GPLv3` is a version number, and
# `canonicalize-lgpl.rs` is a file name.  `COPYING` is not matched at all - it would fire on
# the ordinary verb "copying".
RECITAL = re.compile(
    r"(?<![-\w.`])(?:[Ll]icen[cs]e[sd]?|[Ll]icen[cs]ing|[Cc]opyright|[Cc]opyleft"
    r"|[Pp]roduct[- ][Nn]ame)(?![-`\w.])"
    r"|(?<![-\w.`])(?:L?GPL)(?!\S)|产品名|许可|版权")


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


def classify(code: str, comment: str, recital: bool = True) -> str | None:
    """The class of the first declaration on this line, or None.

    Position decides: prose inside a *literal* is ported semantics (C's own
    string, compared with the reference by the docstring gate) and is classed
    `literal` even when it is the copyright line, while prose inside a comment is
    a statement *about this file* and is classed `decl` (the ownership block),
    `version` (the removed parenthetical) or `title` (any other product name).
    `recital` is the fourth class - a comment that talks about the licence or the
    product-name handling instead of carrying one; it is off for `femacs/xtask`,
    which is the gate's own source and names the classes in prose on purpose.
    """
    if any(p in code for p in DECL_PROSE) or any(p in code for p in PRODUCT):
        return "literal"
    if VERSION_PAREN.search(comment):
        return "version"
    if any(p in comment for p in DECL_PROSE):
        return "decl"
    if any(p in comment for p in PRODUCT):
        return "title"
    if recital and RECITAL.search(comment):
        return "recital"
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
                klass = classify(k, c, recital=not rel.startswith("femacs/xtask/"))
                if klass:
                    out.append((rel, klass, snippet))
    for base in DOC_DIRS:
        for path in sorted((root / base).rglob("*.md")):
            rel = path.relative_to(root).as_posix()
            text = path.read_text(encoding="utf-8", errors="replace")
            for raw_line in text.split("\n"):
                klass = classify("", raw_line, recital=False)
                if klass in DOC_CLASSES:
                    out.append((rel, klass, " ".join(raw_line.split())[:240]))
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
    ("licence recital", "// C's LGPL header survives as the macro.", ["recital"]),
    ("recital names no product", "// the licence block is not carried.", ["recital"]),
    ("ported identifier is not a licence word", "/// C 160: `static const char emacs_copyright[] = COPYRIGHT;`", []),
    ("hyphenated identifier is not a licence word", "// file names (`PACKAGE_*`, `COPYRIGHT`, `emacs-copyright`)", []),
    ("licence recital in Chinese", "/* 参照的许可声明未抄。  */", ["recital"]),
    ("provenance note is not a recital", "// femacs: port of lib/stdbit.c @ fd7afced403 (gnulib): the macro.", []),
    ("module file name is not a licence word", "// (P-26, the canonicalize-lgpl.rs shape).", []),
    ("declaration block reports decl, not recital", "/* Copyright (C) 2026 Free Software Foundation, Inc. */", ["decl"]),
    ("licence prose in a literal is ported semantics", 'let s = "under the terms of the GNU General Public License";', ["literal"]),
    ("emacs path is not the product", "// femacs: mirror of emacs/src/lisp.h", []),
)


SELFTEST_DOC = (
    # The doc shape: Markdown has no literals, so a quoted C string is prose here.
    ("ledger quotes the copied block",
     "> This file is part of GNU Emacs.\n> Copyright (C) 2026 Free Software Foundation, Inc.",
     ["decl", "title"]),
    ("ledger quotes the removed version line",
     "`// femacs: port of src/x.c @ fd7afced403 (GNU Emacs 32.0.50).`", ["version"]),
    ("ledger quotes a C docstring", "- doc-directory: `Directory containing the DOC file that comes with GNU Emacs.`",
     ["title"]),
    ("ledger provenance note", "`// femacs: port of lib/x.c @ fd7afced403 (gnulib).`", []),
    ("ledger licence remark stays a ledger's own", "| licence | the reference block is not carried |", []),
    ("ledger names the reference's revision", "the checkout at `fd7afced403`", []),
)


def findings_markdown(text: str) -> list[tuple[str, str, str]]:
    out = []
    for raw_line in text.split("\n"):
        klass = classify("", raw_line, recital=False)
        if klass in DOC_CLASSES:
            out.append(("<md>", klass, " ".join(raw_line.split())[:240]))
    return out


def selftest() -> int:
    bad = 0
    for name, source, want in SELFTEST:
        got = sorted({k for _, k, _ in findings_text(source)})
        ok = got == sorted(want)
        print(f"{'ok  ' if ok else 'FAIL'} {name}: {got or 'none'}")
        bad += 0 if ok else 1
    for name, source, want in SELFTEST_DOC:
        got = sorted({k for _, k, _ in findings_markdown(source)})
        ok = got == sorted(want)
        print(f"{'ok  ' if ok else 'FAIL'} {name}: {got or 'none'}")
        bad += 0 if ok else 1
    print(f"{len(SELFTEST) + len(SELFTEST_DOC)} cases, {bad} failures")
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
    decls = sum(n for (_, k, _), n in counted.items()
                if k in ("decl", "version", "title", "recital"))
    literals = sum(n for (_, k, _), n in counted.items() if k == "literal")
    print(f"{len(counted)} declaration sites ({decls} forbidden, {literals} allowed literals), "
          f"{failures} failures")
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
