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
source, scanned for the first three classes and exempt from the fourth.  What is read is
decided by *suffix*, not by tree (`SRC_SUFFIXES`): the platform trees are Swift/Kotlin/C++
today, and a tree named in `SCAN_DIRS` whose files the walk does not match is a tree the
gate only *claims* to read.  The one kind whose comments are not C's is the Kept assembly
(`femacs/exec/loader-*.s`, `docs/port-map.md`); `.s` is in `SRC_SUFFIXES` and its dialect
leaders are `LINE_COMMENTS`/`HASH_LINE_START` below (status 3.295).  3.250 found `femacs/platform` in exactly that state - seven
`.swift` files, a `*.rs` glob that matched none of them, while those same files transcribe
the comments of `src/*.m`, whose first twenty lines are the product-name prose this gate
exists to keep out:

  decl     the copied ownership/licence prose ("Free Software Foundation",
           "General Public License", "is free software", "along with GNU",
           "Copyright (C)") inside a comment.  Every one of the 268 blocks 3.58
           deleted was this class.  A block whose only line is the title ("This
           file is part of GNU Emacs.") holds none of the five and is `title`.
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

Our own tooling, probes and CI are a scope of their own too (status 3.462).  They were
written from the first round and no walk ever read them: `SCAN_DIRS` is the *ported*
trees, `DOC_DIRS`/`PROSE_DIRS` are the ledgers and the plan set, so `tools/`, `spikes/`
and `.github/` were read by nobody - which is how 3.413's two identity-line writers had
to be found by hand, and one scope out from that is every probe and gate in the tree.
`.py`/`.sh`/TOML/YAML comments are `#`, not C's, so `HASH_LINE_ANYWHERE` teaches the
scanner that; the `spikes/**` files that are `.c`/`.rs`/`.s` go through the C rules
unchanged.  One file is exempt by class: `tools/check-declarations.py` is the rule's own
table, so `literal` is the pattern it has to write down and `version` is the form it
exists to find - being notation, not a statement about this repository, while `decl`,
`title` and `recital` still bind it.

This repository's own prose is a scope of its own (status 3.340).  The walk above is
bounded by *suffix* over the ported trees and by `DOC_DIRS` over the ledgers, so the
documents that say what this repository *is* - the root README, the plan set, the
translations, the manifests - were read by nobody; every run printed that bound as
`walked past ... femacs/xtask/Cargo.toml`.  A prose file has no literals and no comments,
so the classes that bind it are `version` and `title`: naming *our own* licence is not a
finding, but a document here may not declare this repository to be the reference product.

The two acceptance goldens are read by name (status 3.413).  `docs/baseline/` is
exempt as *measured values*, and `docs/baseline/l{0,1}-reference.txt` are two of
those - but each opens with a header this repository writes, and
`tools/acceptance-l{0,1}.py --golden` rewrites it, so a version line naming the
product can come back in a file no walk of ours read (they are `.txt`, outside
`PROSE_SUFFIXES`, and the blanket exemption covered them).  `BASELINE_PROSE` reads
their leading `#` block and stops there: the records below it are the reference
program's own output, the same kind of data as the rest of `docs/baseline/`.

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
# What the walk reads.  Keyed on the suffix, so a tree named in SCAN_DIRS cannot be bound
# by the *name* alone: `femacs/platform` is the Swift target of `src/*.m` and holds seven
# `.swift` files and no `.rs` at all, so a `*.rs` glob read none of the tree whose comments
# are the ones most likely to carry the reference's opening block (3.250).  `//` and
# `/* */` are the same syntax in every language here and the scanner already models raw and
# `c"` strings, so the classes carry over unchanged.  The `.h`/`.c`/`.m` members are what a
# copied file of the reference would arrive as if one is ever dropped into a scanned tree
# (a verbatim tree is not scanned at all: `tools/check-verbatim.py` owns those).
SRC_SUFFIXES = (".rs", ".swift", ".kt", ".kts", ".java", ".cpp", ".cc", ".cxx", ".m", ".mm",
                ".h", ".hpp", ".c", ".s", ".S")
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

# This repository's own prose and manifests (status 3.340).  `PROSE_FILES` is named file by
# file so a manifest that a walk would miss is still read; `PROSE_DIRS` is walked by suffix.
PROSE_FILES = ("README.md", "PORTING.md", "LICENSE", "femacs/README.md",
               "femacs/Cargo.toml", "femacs/xtask/Cargo.toml", "femacs/rust-toolchain.toml",
               "femacs/Cargo.lock")
# `spikes` is here for the reason the run's own last line exists (status 3.476): the tool walk
# binds `TOOL_SUFFIXES`, which has no `.md` and no `.lock`, so the two files the coverage line
# had been naming as walked past - `spikes/shim/README.md` and `spikes/miri/Cargo.lock` - were
# read by nobody.  A prose file has no literals and no comments, so the suffix walk of this
# scope is the right reader for them, and the `walked past` line then reports their count as
# read rather than as a hole (the machinery already subtracts what this scope read).
PROSE_DIRS = ("docs", "spikes")
# The two files whose *header* is our own prose inside an exempt tree (status 3.413).  Read by
# name, for the reason in the docstring; `header_only` is what keeps the measured records below
# the header out of the rule.
BASELINE_PROSE = ("docs/baseline/l0-reference.txt", "docs/baseline/l1-reference.txt")
PROSE_SUFFIXES = (".md", ".toml", ".lock")
PROSE_CLASSES = ("version", "title")
# The sanctioned homes, each with the reason it is left out.  They are counted and named in the
# run output so an exemption is a reading rather than a silence: the log quotes the user's
# instructions and the reference's measured output by necessity; `acceptance.md` records what
# the *reference program* prints (`--version`, `PACKAGE_STRING`), which is evidence about the
# reference and not a declaration by this repository; `docs/ports` is already the DOC_DIRS
# scope above.
PROSE_EXEMPT = (
    ("docs/status.md", "the running log: quotes the instructions and the reference's output"),
    ("docs/acceptance.md", "records the reference program's own --version output as evidence"),
    ("docs/baseline/", "measured values of the frozen reference build (the two acceptance "
                       "goldens' own headers are read by name - BASELINE_PROSE)"),
    ("docs/ports/", "the per-unit ledgers (already the DOC_DIRS scope)"),
)

# The prose every copied declaration block contains, and that no port file may
# carry in any form (a comment or a literal - the classes below say which).
DECL_PROSE = (
    "Free Software Foundation",
    "General Public License",
    "is free software",
    "along with GNU",
    "Copyright (C)",
)
# The product's *other* registered names, added by the seventh half (status 3.503): the
# Java/Kotlin package the app was shipped under, the JNI symbol prefix it pastes, the
# AppUserModelID C sets with a wide-string literal, the reference's bug address and the Haiku
# bundle signature.  Each of them was an identity declaration of *this* program before 3.503 -
# the Kotlin package and the JNI prefix above all, since the app is what they name.  What
# stays is recounted in the allowlist with a reason: the reference's own text inside a carried
# `.cc`, an address that points upstream, or a literal the frozen configuration prints.
# The *reference path* form the port headers and the port map use
# (`java/org/gnu/emacs/X.java`) is deliberately not a pattern: it names where a unit was
# copied from, which is the other half of PORTING.md 0.1-19.
PRODUCT = ("GNU Emacs", "gnu emacs", "org.gnu.emacs", "org_gnu_emacs", "GNU.Emacs",
           "bug-gnu-emacs", "x-vnd.GNU-emacs")

# The Kept assembly files are the one kind in a scanned tree whose comments are not C's,
# and the *suffix* is all the walk has (the target is not in the path: `loader-x86.s` and
# `loader-aarch64.s` share one).  Two leaders, each measured on this tree's six files
# (status 3.295):
#   * `@` is the ARM dialects' comment character *anywhere* - `loader-armeabi.s` writes 188
#     lines of it and the other five files carry no `@` at all;
#   * `#` is a comment *only at the start of a line*, because in the ARM/AArch64 dialects it
#     is also the immediate prefix (`mov x1, #0`, `cmp r12, #4` - 56 and 82 lines) while the
#     x86/MIPS dialects write their comments with it and have **0** mid-line occurrences
#     (`loader-{x86,x86_64,mips64el,mipsel}.s`).  A `#`-anywhere model would move every ARM
#     immediate's operand into the comment half, and a column-zero-only model would miss an
#     indented `#` comment; both are testable, and both are in `--selftest`.
LINE_COMMENTS = {".s": ("@",), ".S": ("@",)}
HASH_LINE_START = (".s", ".S")

# This repository's own tooling, probes and CI (status 3.462).  Bound by *suffix* for the
# reason 3.250 wrote down: a tree named in a list whose files no suffix matches is a tree the
# gate only claims to read, and this one spans four languages (Python, shell, the probes'
# Rust/C, the CI's YAML).
TOOL_DIRS = ("tools", "spikes", ".github")
TOOL_SUFFIXES = (".py", ".sh", ".rs", ".c", ".h", ".cc", ".s", ".S", ".toml", ".yml", ".yaml")
# Suffixes whose comment leader is `#` *anywhere* in the line, not only at column zero (which
# is the assembly dialects' rule above).  Python has no other use of `#`; shell's is a comment
# at a word start (`${x#y}` and `a#b` are the two exceptions, and a finding still has to
# contain the product name); TOML and YAML have no `#` operator at all.  The bound is written
# down because it is a model, and the two shell exceptions are what it does not model.
HASH_LINE_ANYWHERE = (".py", ".sh", ".toml", ".yml", ".yaml")
# What binds a tool: the four classes a ported unit gets, minus `recital`.  A gate or a probe
# has to say what it enforces (`tools/check-declarations.py`'s own docstring is the largest
# instance) and to quote C's prose it strips, which is the `femacs/xtask` shape - that tree is
# out of `RECITAL_TREES` for the same reason.
TOOL_CLASSES = ("decl", "version", "title", "literal")
# One exemption rule, by file and class: **a tool whose job is to recognise or strip the
# reference's licence prose has to write that prose down** - `tools/check-declarations.py` is
# the rule's own table (`literal` is the pattern list and the selftest table, `version` is the
# form it exists to find; status 3.226 named this file as one of the boundary's homes) and
# `tools/cinventory.py` says which phrase its extractor strips.  Writing it down there is
# notation, not a statement about this repository; `decl`, `title` and `recital` still bind
# both files, and every run prints what was left rather than dropping it silently.
TOOL_EXEMPT = {"tools/check-declarations.py": ("literal", "version"),
               "tools/cinventory.py": ("literal",)}
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


def scan(text: str, suffix: str = "") -> tuple[list[str], list[str]]:
    """Split source into per-line code and per-line comment text.

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
    extra_leaders = LINE_COMMENTS.get(suffix, ())
    hash_at_line_start = suffix in HASH_LINE_START
    hash_anywhere = suffix in HASH_LINE_ANYWHERE
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
            if extra_leaders and ch in extra_leaders:
                state = "line"
                i += 1
                continue
            if ch == "#" and (hash_anywhere
                              or (hash_at_line_start and not "".join(code_line).strip())):
                state = "line"
                i += 1
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


def scanned_files(root: Path) -> list[tuple[str, Path]]:
    """`(rel path, path)` for every file of every scanned tree the gate reads.

    Trees are named by `SCAN_DIRS`, files by `SRC_SUFFIXES`; both are needed, because a
    tree can hold only another language (3.250) and a suffix list is what keeps a
    language-specific glob from deciding whether the rule binds.
    """
    out: list[tuple[str, Path]] = []
    for base in SCAN_DIRS:
        for path in sorted((root / base).rglob("*")):
            if path.is_file() and path.suffix in SRC_SUFFIXES:
                out.append((path.relative_to(root).as_posix(), path))
    return out


def unread_files(root: Path) -> list[str]:
    """Files under a scanned tree that the gate does not read, as `rel path` strings.

    Never empty by accident: `femacs/xtask/Cargo.toml` is the standing example.  It is a
    reading, printed every run, because the failure mode it catches is a *silent* one -
    a tree the docstring says is scanned while every file in it is skipped.
    """
    out: list[str] = []
    for base in SCAN_DIRS:
        for path in sorted((root / base).rglob("*")):
            if path.is_file() and path.suffix not in SRC_SUFFIXES:
                out.append(path.relative_to(root).as_posix())
    # The tool scope's own bound, printed the same way (status 3.462): a `.md` or a
    # `Cargo.lock` under `tools/`/`spikes/` is not read by the tool walk either.
    for base in TOOL_DIRS:
        for path in sorted((root / base).rglob("*")):
            if path.is_file() and path.suffix not in TOOL_SUFFIXES:
                out.append(path.relative_to(root).as_posix())
    return out


def tool_files(root: Path) -> list[tuple[str, Path]]:
    """`(rel path, path)` for this repository's own tools, probes and CI (status 3.462)."""
    out: list[tuple[str, Path]] = []
    for base in TOOL_DIRS:
        for path in sorted((root / base).rglob("*")):
            if path.is_file() and path.suffix in TOOL_SUFFIXES:
                out.append((path.relative_to(root).as_posix(), path))
    return out


def tool_findings(root: Path) -> tuple[list[tuple[str, str, str]], list[tuple[str, str, int]]]:
    """The tool scope's findings, and what `TOOL_EXEMPT` left behind (counted, not dropped).

    The exemption is reported as `(rel path, class, count)` so a run says which file is
    speaking in the rule's own notation, the way `PROSE_EXEMPT` reports the size it did not
    read - an exemption that is a reading rather than a silence.
    """
    out: list[tuple[str, str, str]] = []
    left: dict[tuple[str, str], int] = {}
    for rel, path in tool_files(root):
        text = path.read_text(encoding="utf-8", errors="replace")
        code, comment = scan(text, path.suffix)
        raw_lines = text.split("\n")
        exempt = TOOL_EXEMPT.get(rel, ())
        for line_no in range(max(len(code), len(comment))):
            c = comment[line_no] if line_no < len(comment) else ""
            k = code[line_no] if line_no < len(code) else ""
            klass = classify(k, c, recital=False)
            if klass not in TOOL_CLASSES:
                continue
            if klass in exempt:
                left[(rel, klass)] = left.get((rel, klass), 0) + 1
                continue
            snippet = " ".join(raw_lines[line_no].split())[:240] if line_no < len(raw_lines) else ""
            out.append((rel, klass, snippet))
    return out, [(rel, k, n) for (rel, k), n in sorted(left.items())]


def prose_files(root: Path) -> tuple[list[tuple[str, Path]], dict[str, tuple[int, str]]]:
    """The repository's own prose/manifests, and the sanctioned homes that were left out.

    `read, exempt`: `read` is `(rel path, path)`; `exempt` maps each sanctioned prefix to
    the number of files it held and its reason, so a run reports the size it did not read.
    """
    read: list[tuple[str, Path]] = []
    seen: set[str] = set()
    exempt: dict[str, tuple[int, str]] = {prefix: (0, reason) for prefix, reason in PROSE_EXEMPT}
    for rel in PROSE_FILES:
        path = root / rel
        if path.is_file():
            read.append((rel, path))
            seen.add(rel)
    for base in PROSE_DIRS:
        for path in sorted((root / base).rglob("*")):
            if not path.is_file() or path.suffix not in PROSE_SUFFIXES:
                continue
            rel = path.relative_to(root).as_posix()
            if rel in seen:
                continue
            for prefix in exempt:
                if rel == prefix or rel.startswith(prefix):
                    exempt[prefix] = (exempt[prefix][0] + 1, exempt[prefix][1])
                    break
            else:
                read.append((rel, path))
    return read, exempt


def prose_findings_text(text: str) -> list[tuple[str, str]]:
    """`(class, line)` for the prose classes this gate binds (`PROSE_CLASSES`)."""
    out: list[tuple[str, str]] = []
    for raw in text.split("\n"):
        klass = classify("", raw, recital=False)
        if klass in PROSE_CLASSES:
            out.append((klass, " ".join(raw.split())[:240]))
    return out


def prose_findings(path: Path) -> list[tuple[str, str]]:
    return prose_findings_text(path.read_text(encoding="utf-8", errors="replace"))


def header_only(text: str) -> str:
    """The leading `#` comment block of a file, up to the first line that is not one.

    The goldens open with a header this repository writes and then hold the reference program's
    measured output; only the header is prose of ours, so the scan stops at the blank line that
    separates them (a record can quote the reference's diagnostics, product name included).
    """
    out: list[str] = []
    for line in text.split("\n"):
        if not line.startswith("#"):
            break
        out.append(line)
    return "\n".join(out)


def baseline_prose_findings(path: Path) -> list[tuple[str, str]]:
    return prose_findings_text(header_only(path.read_text(encoding="utf-8", errors="replace")))


def findings(root: Path) -> list[tuple[str, str, str]]:
    """`(path, class, line)` for every declaration this gate knows about."""
    out: list[tuple[str, str, str]] = []
    for rel, path in scanned_files(root):
        text = path.read_text(encoding="utf-8", errors="replace")
        code, comment = scan(text, path.suffix)
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
    for rel, path in prose_files(root)[0]:
        for klass, snippet in prose_findings(path):
            out.append((rel, klass, snippet))
    out.extend(tool_findings(root)[0])
    for rel in BASELINE_PROSE:
        path = root / rel
        if not path.is_file():
            continue
        for klass, snippet in baseline_prose_findings(path):
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
    # The other languages of a scanned tree - the shapes `femacs/platform` writes.  The
    # scanner is syntax-driven (comments, `"` strings), so nothing language-specific is
    # needed; these cases exist because the *walk* decides whether they are reached at all
    # (3.250), and a case per language is what makes a walk that skips them visible.
    ("copied block in a Swift comment",
     "// This file is part of GNU Emacs.\n// Copyright (C) 2026 Free Software Foundation, Inc.",
     ["decl", "title"]),
    ("Swift citation of C's docstring literal",
     'let s = "Directory of machine-readable files that come with GNU Emacs."', ["literal"]),
    ("Kotlin port header keeps the revision only",
     "// femacs: port of src/x.java @ fd7afced403.", []),
    ("C++ platform title comment",
     "/* Support for the Haiku window system, for GNU Emacs.  */", ["title"]),
    ("licence recital in a Swift comment",
     "// the reference's licence block is not carried.", ["recital"]),
)


# The Kept assembly files (`femacs/exec/loader-*.s`, status 3.295): a copied block in the
# x86/MIPS dialect and in the ARM one.  These are scanned with `suffix=".s"`, because that
# is what decides the leaders - the same text without it is code, not a comment (which is
# exactly the finding the two cases would report).
SELFTEST_ASM = (
    ("copied block in an assembly comment",
     "# This file is part of GNU Emacs.\n# GNU Emacs is free software: you can redistribute it.",
     ["decl", "title"]),
    ("ARM comment character carries the block",
     "@ along with GNU Emacs.  If not, see <https://www.gnu.org/licenses/>.", ["decl"]),
)


# The assembly dialect is a *splitting* rule, not a class, so it is checked on the split
# (`scan`) rather than through the classes: `#` at the start of a line is a comment, a
# mid-line `#` is an operand of the ARM/AArch64 immediate syntax and stays code (3.295).
SELFTEST_ASM_SPLIT = (
    ("#", "\tmov\tx0, #1\t\t// x0 = 1", "\tmov\tx0, #1", " x0 = 1"),
    ("#", "\t# for GNU Emacs", "", " for GNU Emacs"),
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


SELFTEST_PROSE = (
    ("prose declares the replication",
     "[femacs](../README.md) is a one-to-one replication of GNU Emacs: every C source",
     ["title"]),
    ("prose carries the removed version line", "`// femacs: port of src/x.c @ fd7afced403 (GNU Emacs 32.0.50).`",
     ["version"]),
    ("our own licence sentence is not a finding",
     "femacs is distributed under the **GNU General Public License, version 3 or later**.",
     []),
    ("golden header names the product", "# version: GNU Emacs 32.0.50", ["title"]),
    ("golden header keeps the number", "# version: 32.0.50", []),
    ("prose that records the revision in a title", "# GOAL: femacs - the C source of the reference tree, one to one",
     []),
)


# The header-only reading the two goldens get (status 3.413): the header is ours, the records
# below it are measured output, and the third case is what proves the scan stops in between.
SELFTEST_BASELINE = (
    ("golden header names the product",
     "# L0 golden: the self-check expressions' outputs on the reference program.\n"
     "# version: GNU Emacs 32.0.50\n# sha256[0:16]: a959f1d7a8bb4e11\n\n@@ arith\n",
     ["title"]),
    ("golden header keeps the number only",
     "# binary: build/emacs-ref/src/emacs\n# version: 32.0.50\n"
     "#   the number only; PORTING.md 0.1-19: the name the reference prints is quoted in\n"
     "# sha256[0:16]: a959f1d7a8bb4e11\n\n@@ arith\n",
     []),
    ("a record that recites the product name is the reference's output, not our prose",
     "# version: 32.0.50\n\n@@ advice.el\n--- exit: 0\n--- diagnostics:\n"
     "Warning: GNU Emacs 32.0.50 reports this\n--- end diagnostics\n",
     []),
)


# The tool scope (status 3.462).  The `#` cases are also the split's proof: if a `#` were not
# read as a comment leader for these suffixes the first two would come back `literal`/nothing,
# and if it were read inside a string the last one would come back `title` instead of `literal`.
SELFTEST_TOOL = (
    ("a Python comment naming the product", "# the reference's GNU Emacs build\n", ["title"]),
    ("a Python comment carrying the removed form",
     "# was `// femacs: port of src/x.c @ fd7afced403 (GNU Emacs 32.0.50).`\n", ["version"]),
    ("a shell comment carrying the copied block",
     "#!/bin/sh\n# GNU Emacs is free software: you can redistribute it.\n", ["decl"]),
    ("a literal in a tool is this repository speaking",
     'IDENTITY = "GNU Emacs 32.0.50"\n', ["literal"]),
    ("a `#` inside a Python string stays a literal",
     'P = r"# GNU Emacs"\n', ["literal"]),
)


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
    for name, source, want in SELFTEST_ASM:
        got = sorted({k for _, k, _ in findings_text(source, ".s")})
        ok = got == sorted(want)
        print(f"{'ok  ' if ok else 'FAIL'} {name}: {got or 'none'}")
        bad += 0 if ok else 1
    for name, source, want in SELFTEST_PROSE:
        got = sorted({k for k, _ in prose_findings_text(source)})
        ok = got == sorted(want)
        print(f"{'ok  ' if ok else 'FAIL'} {name}: {got or 'none'}")
        bad += 0 if ok else 1
    for name, source, want in SELFTEST_BASELINE:
        got = sorted({k for k, _ in prose_findings_text(header_only(source))})
        ok = got == sorted(want)
        print(f"{'ok  ' if ok else 'FAIL'} {name}: {got or 'none'}")
        bad += 0 if ok else 1
    for name, source, want in SELFTEST_TOOL:
        got = sorted({k for _, k, _ in findings_text(source, ".py")})
        ok = got == sorted(want)
        print(f"{'ok  ' if ok else 'FAIL'} {name}: {got or 'none'}")
        bad += 0 if ok else 1
    for leader, source, want_code, want_comment in SELFTEST_ASM_SPLIT:
        code, comment = scan(source + "\n", ".s")
        ok = code[0].strip() == want_code.strip() and comment[0] == want_comment
        print(f"{'ok  ' if ok else 'FAIL'} assembly `{leader}` split: "
              f"code={code[0]!r} comment={comment[0]!r}")
        bad += 0 if ok else 1
    total = (len(SELFTEST) + len(SELFTEST_DOC) + len(SELFTEST_PROSE) + len(SELFTEST_ASM)
             + len(SELFTEST_ASM_SPLIT) + len(SELFTEST_BASELINE) + len(SELFTEST_TOOL))
    print(f"{total} cases, {bad} failures")
    return 1 if bad else 0


def findings_text(text: str, suffix: str = "") -> list[tuple[str, str, str]]:
    code, comment = scan(text, suffix)
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
    read = scanned_files(root)
    skip = unread_files(root)
    prose, exempt = prose_files(root)
    tool = tool_files(root)
    _, tool_left = tool_findings(root)
    by_suffix: dict[str, int] = {}
    for _, path in read:
        by_suffix[path.suffix] = by_suffix.get(path.suffix, 0) + 1
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
    # The coverage line is the gate's own bound: what it read, and what it walked past.
    # 3.250 added it because `femacs/platform` was "scanned" by name while its seven
    # `.swift` files - the ones transcribing `src/*.m` - were read by nobody.
    spans = " ".join(f"{n} {ext}" for ext, n in sorted(by_suffix.items(), key=lambda kv: -kv[1]))
    print(f"read {len(read)} files ({spans})")
    kinds: dict[str, int] = {}
    for _, path in prose:
        kinds[path.suffix or "(no suffix)"] = kinds.get(path.suffix or "(no suffix)", 0) + 1
    span = " ".join(f"{n} {ext}" for ext, n in sorted(kinds.items(), key=lambda kv: -kv[1]))
    print(f"read {len(prose)} own prose/manifest file(s) ({span})")
    print(f"read {len(BASELINE_PROSE)} baseline header(s) by name "
          f"({', '.join(BASELINE_PROSE)}), records below the header left to the exemption")
    if exempt:
        print("left out (sanctioned, counted): "
              + "; ".join(f"{prefix} {n} file(s) - {reason}"
                          for prefix, (n, reason) in exempt.items()))
    tkinds: dict[str, int] = {}
    for _, path in tool:
        tkinds[path.suffix or "(no suffix)"] = tkinds.get(path.suffix or "(no suffix)", 0) + 1
    tspan = " ".join(f"{n} {ext}" for ext, n in sorted(tkinds.items(), key=lambda kv: -kv[1]))
    print(f"read {len(tool)} own tool/probe file(s) ({tspan})"
          + ("; left to the rule's own notation: "
             + ", ".join(f"{rel} {k} x{n}" for rel, k, n in tool_left)
             if tool_left else ""))
    # The bound the walk prints last: `skip` is what the *suffix* walk passed over, and a
    # file the prose scope reads is no longer part of that hole (3.340 closed its one member,
    # `femacs/xtask/Cargo.toml`), so the line stays printed with the count it now reads.
    covered = {rel for rel, _ in prose}
    rest = [s for s in skip if s not in covered]
    print(f"walked past {len(rest)} file(s) of an unscanned kind: "
          + (", ".join(sorted(rest)[:8]) + (" ..." if len(rest) > 8 else "") if rest else "none")
          + (f" ({len(skip) - len(rest)} of them read above)" if len(rest) != len(skip) else ""))
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
