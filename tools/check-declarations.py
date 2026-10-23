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
ported tree is scanned - `src` and `lib` plus batch F's own programs and
libraries (`lib-src`, `exec`, `lwlib`, `oldXMenu`, `nt`) and its non-C platform trees
(`platform`: the Swift/Kotlin/C++ targets of `src/*.m`, `java/`, `haiku_*.cc`);
`xtask` is this gate's own
source, scanned for the first three classes and exempt from the fourth.  What is read is
decided by *suffix*, not by tree (`SRC_SUFFIXES`): the platform trees are Swift/Kotlin/C++
today, and a tree named in `SCAN_DIRS` whose files the walk does not match is a tree the
gate only *claims* to read.  The one kind whose comments are not C's is the Kept assembly
(`exec/loader-*.s`, `docs/port-map.md`); `.s` is in `SRC_SUFFIXES` and its dialect
leaders are `LINE_COMMENTS`/`HASH_LINE_START` below (status 3.295).  3.250 found `platform` in exactly that state - seven
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

`docs/ports/` - the per-unit ledgers - is scanned too, for the `decl`, `version`
and `title` classes (`recital` never: a ledger is the *sanctioned* home for a
licence remark, status 3.90/3.100).  That walk was `rglob("*.md")` until the
sixteenth half (status 3.679), so the tables that sit beside the ledgers (`.txt`,
`.tsv`) and the one `.rs` code draft `tools/emit-mirror.py` writes there were read
by nobody - and no coverage line named that bound, which is the 3.250 failure shape
in the one scope that had no printed bound of its own.  The tables take the prose
model (a cell is prose, not a literal), the draft the source model.  A ledger quotes C, so it is exactly
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

Three of this repository's own files are read by name for the same reason (status
3.679): `LIFETIMES.tsv` and `.gitignore` sit at the root, where no `PROSE_DIRS`
walk reaches, and `docs/reviews/*.sha256` is a record whose header is our prose,
so the prose suffix set gained `.sha256`.

`docs/baseline/` is exempt as *measured values*, and that exemption was whole-file until the
seventeenth half (status 3.692): the tables there also carry comment lines this repository writes,
and `docs/baseline/verbatim-substitutions.tsv` puts two of them several lines *below* its leading
block - exactly where the reader the two goldens got in 3.413 walks past them (a leading `#` block,
stopped at the first line that is not one).  The payload below the comments is still the
reference's, so the walk reads every `#` line of every file in the directory, keeps the two goldens
header-only (their records are the reference program's own output), and prints what it leaves as
the measured payload lines that name the product, counted rather than dropped.  The
eighteenth half read that count one layer further: 109 of its 126 lines were the *reason*
column of the two registration tables - prose this repository writes, not a measured value -
so that column is read with the prose model too, beside the copied sentence and the ported
line it justifies, which stay quoted and are what the remaining count is.

Two sanctioned homes were left out whole until the nineteenth half (status 3.750): the running
log (status.md) and the acceptance record, on the ground that they "quote the instructions and
the reference's output by necessity".  That is a claim *about* a file, and the whole point of the
halves above is that such a claim becomes a reading.  So `LOG_SCOPE` reads both files and
classifies every line that carries the product's name: a citation when the name sits in a fenced
code block, in a code span (including the one the log's hard wrap creates, a span the previous
line left open), in a quote pair, or on a line carrying the instruction's own words - and a
`title` finding when the log speaks in its own voice.  The run prints the reading and how each
citation kind contributed, so the log may quote the reference's output but may not declare this
program to be the reference product.

This repository's own prose is a scope of its own (status 3.340).  The walk above is
bounded by *suffix* over the ported trees and by `DOC_DIRS` over the ledgers, so the
documents that say what this repository *is* - the root README, the plan set, the
translations, the manifests - were read by nobody; every run printed that bound as
`walked past ... xtask/Cargo.toml`.  A prose file has no literals and no comments,
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
# reference block's provenance sentence and no gate read it (status 3.100).  `xtask`
# is this gate's own source, not a port.
SCAN_DIRS = ("src", "lib", "lib-src", "exec", "lwlib",
             "oldXMenu", "nt", "msdos",
             # batch F's non-C platform trees (src/*.m -> Swift, java/ -> Kotlin,
             # haiku_*.cc -> C++).  They do not exist yet; naming them here is what makes
             # the rule bind the first file written into them rather than the first review.
             "platform", "xtask")
# What the walk reads.  Keyed on the suffix, so a tree named in SCAN_DIRS cannot be bound
# by the *name* alone: `platform` is the Swift target of `src/*.m` and holds seven
# `.swift` files and no `.rs` at all, so a `*.rs` glob read none of the tree whose comments
# are the ones most likely to carry the reference's opening block (3.250).  `//` and
# `/* */` are the same syntax in every language here and the scanner already models raw and
# `c"` strings, so the classes carry over unchanged.  The `.h`/`.c`/`.m` members are what a
# copied file of the reference would arrive as if one is ever dropped into a scanned tree
# (a verbatim tree is not scanned at all: `tools/check-verbatim.py` owns those).
SRC_SUFFIXES = (".rs", ".swift", ".kt", ".kts", ".java", ".cpp", ".cc", ".cxx", ".m", ".mm",
                ".h", ".hpp", ".c", ".s", ".S")
# The recital rule binds the ported units and mirrors; `xtask` explains the rule in
# its own comments and is outside it (a licence remark about this repository belongs in
# docs/status.md and the ledger, never in a ported unit).
RECITAL_TREES = SCAN_DIRS[:-1]
# The ledgers of our own (see the module docstring).  Markdown has no literals, so only the
# three prose classes can be re-introduced there, and a quoted C string that carries the
# product name is prose in this tree, not ported semantics - the ported unit is where the
# literal lives and where it is allowlisted.
DOC_DIRS = ("docs/ports",)
# The ledger walk bound *two* suffixes from the first round (`rglob("*.md")`), and the
# file kinds it left are the ones this scope could not say it was reading (status 3.679).
# The tables take the prose model - they have no literals, and a cell that names the
# product is a `decl`/`version`/`title` the same way a ledger line is - while the one
# `.rs` draft takes the source model, so a product-name literal in it is counted and
# needs an allowlist row like any other.
DOC_SUFFIXES = (".md", ".txt", ".tsv")
DOC_SRC_SUFFIXES = (".rs",)
DOC_CLASSES = ("decl", "version", "title")
ALLOWLIST = REPO / "docs/baseline/declarations-allowlist.tsv"

# This repository's own prose and manifests (status 3.340).  `PROSE_FILES` is named file by
# file so a manifest that a walk would miss is still read; `PROSE_DIRS` is walked by suffix.
# `.gitignore` and `LIFETIMES.tsv` are the two root files no walk reached (status 3.679):
# the first has no suffix at all and the second is a table of this repository's own - both
# are manifests of ours, so they are read by name rather than by a suffix that would have
# to be invented for them.
PROSE_FILES = ("README.md", "PORTING.md", "LICENSE", "README.md",
               ".gitignore", "LIFETIMES.tsv",
               "Cargo.toml", "xtask/Cargo.toml", "rust-toolchain.toml",
               "Cargo.lock")
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
# `docs/baseline/` is a scope of its own (status 3.692).  Its *payload* is the reference's
# measured values, which is why `PROSE_EXEMPT` leaves it out; its `#` lines are comments this
# repository writes, and one of them names the product several lines below the leading block the
# goldens' reader stops at.  So the directory is walked for its comment band - every `#` line,
# wherever it sits - and the payload is left, counted, on the run's own line.
BASELINE_DIR = "docs/baseline"
# `.sha256` is `docs/reviews/`'s record kind (status 3.679): a hash list whose header is
# this repository's own prose.
PROSE_SUFFIXES = (".md", ".toml", ".lock", ".sha256")
PROSE_CLASSES = ("version", "title")
# The sanctioned homes, each with the reason it is left out.  They are counted and named in the
# run output so an exemption is a reading rather than a silence: `docs/ports` is already the
# DOC_DIRS scope above.  The running log and the acceptance record stood here until the
# nineteenth half (status 3.750) with the reason that they quote the user's instructions and the
# reference's measured output; they moved to `LOG_SCOPE` below, which reads that reason.
PROSE_EXEMPT = (
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

# The *fragment* forms, added by the thirteenth half (status 3.648).  The eighth half (3.599)
# recorded this one class as a known gap: the table collected the product's readable names, and
# these are *path and alias fragments* instead, so a site inside a **comment** was counted
# nowhere - the ported `nt/`/`w32` units quote C's own `#define` for the registry key the
# reference program's settings live under, and 3.599's rule says a comment carries no product
# name.  A defect inside a blind spot of the gate that claims to read the tree, in other words.
# The bytes themselves stay: each is an *external* name - that registry key, which other
# programs and the user's own registry settings live under (written twice: prose's single
# backslashes and a C/Rust literal's doubled ones), the Start Menu folder and shortcut file
# name, the MCI device alias handed to the sound system, and the window class the X clipboard
# negotiates - so every site of this class is allowlisted with that reason.
PRODUCT_FRAGMENTS = ("SOFTWARE\GNU\Emacs", "SOFTWARE\\\\GNU\\\\Emacs",
                     "GNUEmacs_", "Gnu Emacs", "Emacs.lnk", "Emacs Clipboard")

# The two sanctioned log files, read by their *citation band* rather than exempted whole
# (status 3.750).  The exemption above says the log and the acceptance record quote the
# instructions and the reference's output; this scope reads that claim.  A product-name line is a
# citation when
#   * it sits inside a fenced code block - the reference's measured output, pasted verbatim;
#   * the name sits inside a code span: the line's own backticks, plus the one case the log's
#     hard wrap creates, a span the previous line left open (counted as `carried`), since one C
#     comment can be quoted across two wrapped lines;
#   * the name sits inside a quote pair (`quoted_span`: the ASCII, CJK and corner pairs) - the
#     log quotes C, the copied trees and its own tooling's output that way;
#   * the line carries the user's instruction's own words (`INSTRUCTION_QUOTES`): the directive
#     sentence is a quotation wherever it appears.
# Anything else is this repository's prose, and on these two files it is a `title` finding: the
# log may quote the reference's output, but it may not speak in its own voice as the product.
LOG_SCOPE = ("docs/status.md", "docs/acceptance.md")
INSTRUCTION_QUOTES = ("版本声明不要再说是", "把之前的代码中的这种声明也去掉")
LOG_PRODUCT = PRODUCT + PRODUCT_FRAGMENTS
LOG_PATTERN = re.compile("|".join(re.escape(name) for name in LOG_PRODUCT))
SPAN_MARK = "`"

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
# instance) and to quote C's prose it strips, which is the `xtask` shape - that tree is
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
               "tools/cinventory.py": ("literal",),
               # 3.656: the census over the six copied trees filters on the product's
               # readable names, so its pattern table and its selftest fixture write them
               # down; `decl`, `title` and `recital` still bind the file (the run below
               # showed both - the two literals are this row, the comment was reworded).
               "tools/check-verbatim-declarations.py": ("literal",)}
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
    product-name handling instead of carrying one; it is off for `xtask`,
    which is the gate's own source and names the classes in prose on purpose.
    """
    if (any(p in code for p in DECL_PROSE) or any(p in code for p in PRODUCT)
            or any(p in code for p in PRODUCT_FRAGMENTS)):
        return "literal"
    if VERSION_PAREN.search(comment):
        return "version"
    if any(p in comment for p in DECL_PROSE):
        return "decl"
    if any(p in comment for p in PRODUCT) or any(p in comment for p in PRODUCT_FRAGMENTS):
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

    Never empty by accident: `xtask/Cargo.toml` is the standing example.  It is a
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
    # The ledger and prose scopes' bound, printed from 3.679 on: their walks bind a suffix
    # set too, so a file of another kind under them was read by nobody *and* was named by no
    # coverage line - `docs/ports/lisp-draft.rs` stood in that hole for the rule's whole
    # history.  The sanctioned prefixes are subtracted by the caller, not here: this function
    # reports what the walk passed over, and the run says which of it is an exemption.
    seen = set(out)
    for base in DOC_DIRS + PROSE_DIRS:
        for path in sorted((root / base).rglob("*")):
            if not path.is_file():
                continue
            rel = path.relative_to(root).as_posix()
            if rel in PROSE_FILES or rel in seen:
                continue
            # `docs/baseline/` is read by its own scope above (status 3.692): its comment band is
            # our prose, its payload is counted on that scope's own line, and the two goldens are
            # read by name.  Leaving them here would report them as a hole the run has closed.
            if rel in BASELINE_PROSE or rel.startswith(BASELINE_DIR + "/"):
                continue
            # The two sanctioned logs are read by `LOG_SCOPE` below (status 3.750), so they are
            # no longer part of what the prose walk passed over.
            if rel in LOG_SCOPE:
                continue
            # Each scope's suffix set is scoped to its own directories: `.tsv` is read
            # under `docs/ports/` (DOC_DIRS) and is *not* read under `docs/baseline/`
            # (sanctioned), so a global suffix test would mark the second kind read when
            # nothing reads it - the same silence this walk exists to break.
            if (any(rel == d or rel.startswith(d + "/") for d in DOC_DIRS)
                    and path.suffix in DOC_SUFFIXES + DOC_SRC_SUFFIXES):
                continue
            if (any(rel == d or rel.startswith(d + "/") for d in PROSE_DIRS)
                    and path.suffix in PROSE_SUFFIXES):
                continue
            # `spikes/` is in both scopes; its 22 `.rs`/17 `.sh`/6 `.py`/6 `.c` members are
            # the tool walk's (status 3.462), not this one's hole.
            if (any(rel == t or rel.startswith(t + "/") for t in TOOL_DIRS)
                    and path.suffix in TOOL_SUFFIXES):
                continue
            seen.add(rel)
            out.append(rel)
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
            # The sanctioned logs are read by their own scope (status 3.750); reading them here
            # as well would report every quoted product name as this repository's prose.
            if rel in LOG_SCOPE:
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


def quoted_span(line: str, start: int) -> bool:
    """Is the position `start` inside a quote pair the log cites with?

    Three marks, all measured on these two files: the ASCII double quote (symmetric, so an odd
    count before the name means it sits between two of them), the CJK double quote, and the
    corner quotes (both asymmetric, so more openers than closers before the name means the same).
    """
    before = line[:start]
    return (before.count('"') % 2 == 1
            or before.count("“") > before.count("”")
            or before.count("「") > before.count("」"))


def log_files(root: Path) -> list[tuple[str, Path]]:
    """`(rel path, path)` for the sanctioned logs `LOG_SCOPE` names."""
    return [(rel, root / rel) for rel in LOG_SCOPE if (root / rel).is_file()]


def log_findings_text(text: str) -> tuple[list[tuple[str, str]],
                                         tuple[int, int, int, int, int, int, int]]:
    """`(findings, stats)` for one sanctioned log file.

    `findings` is `(class, line)` for every line that carries the product's name in this
    repository's own voice (`PROSE_CLASSES`).  `stats` is `(lines, fenced, spanned, carried,
    quoted, instruction, own)`: the product-name lines, how many names each citation kind
    covered, and how many were left as our prose.  `carried` is printed because it is the span
    model's weak spot - a name covered only by a span the *previous* line left open; the log's
    lines are hard-wrapped, so that is the normal shape of a quoted multi-line C comment, and a
    reader can see how much of the reading rests on it.
    """
    findings: list[tuple[str, str]] = []
    lines = fenced = spanned = carried = quoted = instruction = 0
    in_fence = open_span = False
    for raw in text.split("\n"):
        if raw.lstrip().startswith("```"):
            in_fence = not in_fence
            open_span = False
            continue
        hits = list(LOG_PATTERN.finditer(raw)) if LOG_PATTERN.search(raw) else []
        if hits:
            lines += 1
            if in_fence:
                fenced += len(hits)
            elif any(phrase in raw for phrase in INSTRUCTION_QUOTES):
                instruction += len(hits)
            else:
                first_span = raw.find(SPAN_MARK)
                uncited: list[str] = []
                for match in hits:
                    before = raw[:match.start()]
                    if before.count(SPAN_MARK) % 2:
                        spanned += 1
                    elif open_span and (first_span < 0 or match.start() < first_span):
                        carried += 1
                    elif quoted_span(raw, match.start()):
                        quoted += 1
                    else:
                        uncited.append(match.group(0))
                if uncited:
                    findings.append(("title", " ".join(raw.split())[:240]))
        # A span left open at the end of a line continues on the next one (the wrap case); a
        # blank line ends it, so a stray backtick cannot carry a citation across a paragraph.
        open_span = bool(raw.strip()) and raw.count(SPAN_MARK) % 2 == 1
    return findings, (lines, fenced, spanned, carried, quoted, instruction, len(findings))


def log_findings(root: Path) -> list[tuple[str, str, str]]:
    out: list[tuple[str, str, str]] = []
    for rel, path in log_files(root):
        found, _ = log_findings_text(path.read_text(encoding="utf-8", errors="replace"))
        out.extend((rel, klass, snippet) for klass, snippet in found)
    return out


def log_stats(root: Path) -> tuple[int, int, int, int, int, int, int, int]:
    """`(files, lines, fenced, spanned, carried, quoted, instruction, own)` for the logs."""
    files = lines = fenced = spanned = carried = quoted = instruction = own = 0
    for _, path in log_files(root):
        _, stat = log_findings_text(path.read_text(encoding="utf-8", errors="replace"))
        files += 1
        lines += stat[0]
        fenced += stat[1]
        spanned += stat[2]
        carried += stat[3]
        quoted += stat[4]
        instruction += stat[5]
        own += stat[6]
    return files, lines, fenced, spanned, carried, quoted, instruction, own


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


# The two registration tables under `docs/baseline/` are read in two halves (status 3.692
# read the first).  Their *last* column is a reason this repository writes - the columns before
# it quote the copied sentence (`verbatim-declaration-keeps.tsv`) or the ported line that
# carries the literal (`declarations-allowlist.tsv`), so they are the registration's measured
# side and stay - while a reason is prose of ours, exactly like a `docs/ports/` ledger's, which
# is the sanctioned home for a remark about the reference's declaration.  The run's line used to
# read "126 payload line(s) naming the product"; 109 of them were this column (108 rows carried
# one shared reason string, one more cited the reference's schema file), i.e. the exemption's
# own bound was counting our prose as a measured value.  The eighteenth half reads the column and
# rewrote those reasons; the columns it quotes keep theirs.
BASELINE_OWN_COLUMN = {"docs/baseline/declarations-allowlist.tsv": 4,
                       "docs/baseline/verbatim-declaration-keeps.tsv": 4}


def baseline_prose_findings(path: Path) -> list[tuple[str, str]]:
    return prose_findings_text(header_only(path.read_text(encoding="utf-8", errors="replace")))


def baseline_table_files(root: Path) -> list[tuple[str, Path]]:
    """`(rel path, path)` for the tables and records under `docs/baseline/`.

    The two goldens are `BASELINE_PROSE` and are read by name, header only: their record body is
    the reference program's own output.  Everything else in the directory is read here (status
    3.692) - its comment band is prose of ours, its payload is the reference's measured values.
    """
    out: list[tuple[str, Path]] = []
    for path in sorted((root / BASELINE_DIR).rglob("*")):
        if not path.is_file():
            continue
        rel = path.relative_to(root).as_posix()
        if rel in BASELINE_PROSE:
            continue
        out.append((rel, path))
    return out


def baseline_findings_text(text: str) -> list[tuple[str, str]]:
    """`(class, line)` for one baseline file's comment band (`PROSE_CLASSES`).

    Every `#` line counts, not only the leading block: the file's comments are ours wherever they
    sit, and a payload line is not read at all (it is a measured value, not a statement by us).
    """
    out: list[tuple[str, str]] = []
    for raw in text.split("\n"):
        if not raw.strip().startswith("#"):
            continue
        klass = classify("", raw, recital=False)
        if klass in PROSE_CLASSES:
            out.append((klass, " ".join(raw.split())[:240]))
    return out


def baseline_own_findings(rel: str, text: str) -> list[tuple[str, str]]:
    """`(class, cell)` for a registration table's reason column (`BASELINE_OWN_COLUMN`).

    A file with no reason column (the frozen configuration's macro dump) is not read here: its
    payload *is* the reference's measured value, which is the exemption's reason for existing.
    """
    column = BASELINE_OWN_COLUMN.get(rel)
    if column is None:
        return []
    out: list[tuple[str, str]] = []
    for raw in text.split("\n"):
        if not raw or raw.startswith("#"):
            continue
        cells = raw.split("\t")
        if len(cells) <= column:
            continue
        klass = classify("", cells[column], recital=False)
        if klass in PROSE_CLASSES:
            out.append((klass, " ".join(cells[column].split())[:240]))
    return out


def baseline_stats(root: Path) -> tuple[int, int, int, int, int, int]:
    """The reading the new scope prints: `(files, comment lines, lines below the leading block,
    payload lines naming the product, files with one)`.

    The third number is the bound the goldens' reader cannot see - the reader stops at the first
    line that is not `#`, and the substitution table's two product-name sentences sit in the band
    that reader never reaches.  The last three are what the exemption leaves: the reference's
    measured payload, counted so the run says what it did not read, and the reason column of the
    two registration tables (`BASELINE_OWN_COLUMN`), which is this repository's prose and is
    bound by the rule where it sits.
    """
    files = lines = deep = payload_hits = payload_files = own_hits = 0
    for rel, path in baseline_table_files(root):
        text = path.read_text(encoding="utf-8", errors="replace")
        raw_lines = text.split("\n")
        lead = 0
        for raw in raw_lines:
            if raw.startswith("#"):
                lead += 1
            else:
                break
        n_comment = sum(1 for raw in raw_lines if raw.strip().startswith("#"))
        files += 1
        lines += n_comment
        deep += n_comment - lead
        hits = sum(1 for raw in raw_lines
                   if not raw.strip().startswith("#")
                   and any(pat in raw for pat in PRODUCT + PRODUCT_FRAGMENTS))
        payload_hits += hits
        payload_files += 1 if hits else 0
        own_hits += len(baseline_own_findings(rel, text))
    return files, lines, deep, payload_hits, payload_files, own_hits


def doc_files(root: Path) -> list[tuple[str, Path]]:
    """`(rel path, path)` for the ledger directory's files (status 3.679).

    Both kinds: the ledgers and tables by `DOC_SUFFIXES`, the code drafts by
    `DOC_SRC_SUFFIXES`.  The suffix set, not the directory name, is what decides whether a
    file in a directory this gate claims to read is read at all - the 3.250/3.462 lesson,
    applied to the one scope whose bound the run did not print.
    """
    out: list[tuple[str, Path]] = []
    for base in DOC_DIRS:
        for path in sorted((root / base).rglob("*")):
            if path.is_file() and path.suffix in DOC_SUFFIXES + DOC_SRC_SUFFIXES:
                out.append((path.relative_to(root).as_posix(), path))
    return out


def ledger_findings_text(text: str, suffix: str) -> list[tuple[str, str]]:
    """`(class, line)` for one ledger-scope file body: the tables' prose model, the
    drafts' source model (status 3.679)."""
    out: list[tuple[str, str]] = []
    if suffix in DOC_SRC_SUFFIXES:
        code, comment = scan(text, suffix)
        raw_lines = text.split("\n")
        for line_no in range(max(len(code), len(comment))):
            c = comment[line_no] if line_no < len(comment) else ""
            k = code[line_no] if line_no < len(code) else ""
            snippet = (" ".join(raw_lines[line_no].split())[:240]
                       if line_no < len(raw_lines) else "")
            klass = classify(k, c, recital=False)
            if klass:
                out.append((klass, snippet))
        return out
    for raw_line in text.split("\n"):
        klass = classify("", raw_line, recital=False)
        if klass in DOC_CLASSES:
            out.append((klass, " ".join(raw_line.split())[:240]))
    return out


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
            klass = classify(k, c, recital=not rel.startswith("xtask/"))
            if klass:
                out.append((rel, klass, snippet))
    for rel, path in doc_files(root):
        text = path.read_text(encoding="utf-8", errors="replace")
        for klass, snippet in ledger_findings_text(text, path.suffix):
            out.append((rel, klass, snippet))
    for rel, path in prose_files(root)[0]:
        for klass, snippet in prose_findings(path):
            out.append((rel, klass, snippet))
    out.extend(tool_findings(root)[0])
    out.extend(log_findings(root))
    for rel in BASELINE_PROSE:
        path = root / rel
        if not path.is_file():
            continue
        for klass, snippet in baseline_prose_findings(path):
            out.append((rel, klass, snippet))
    for rel, path in baseline_table_files(root):
        text = path.read_text(encoding="utf-8", errors="replace")
        for klass, snippet in baseline_findings_text(text):
            out.append((rel, klass, snippet))
        for klass, snippet in baseline_own_findings(rel, text):
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
    # The other languages of a scanned tree - the shapes `platform` writes.  The
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


# The Kept assembly files (`exec/loader-*.s`, status 3.295): a copied block in the
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


# The tables' own reading (status 3.692): the comment band is ours wherever it sits, the payload
# is the reference's measured values.  The first case is why the second reader exists - the
# leading-block reading above (`header_only`, the goldens') reaches the block and stops, and this
# sentence is below it.
SELFTEST_BASELINE_TABLE = (
    ("a table's comment band names the product below the leading block",
     "# The table's leading comment, our prose.\n#\n\n"
     "# and later, below the leading block:\n"
     "# `This file is part of GNU Emacs.` - the copied sentence.\n\nrow\tcell\n",
     ["title"]),
    ("a table's leading block names the product",
     "# version: GNU Emacs 32.0.50\n\nrow\tcell\n", ["title"]),
    ("a table's leading block keeps the number",
     "# version: 32.0.50\n\nrow\tcell\n", []),
    ("a table's payload naming the product is a measured value",
     'PACKAGE_NAME\t"GNU Emacs"\nPACKAGE_STRING\t"GNU Emacs 32.0.50"\n', []),
    ("a table's comment carrying the removed version form",
     "# femacs: port of src/x.c @ fd7afced403 (GNU Emacs 32.0.50).\n\nrow\n", ["version"]),
    ("a comment quoting the reference's macro line",
     "# the header quotes it: `# define PACKAGE_NAME \"GNU Emacs\"`\n\nrow\n", ["title"]),
)


# The registration tables' reason column (the eighteenth half).  The second case is the split's
# proof: the copied sentence in the column before the reason is a quotation, so a reader that
# bound the whole row would report it, and one that read no column at all would miss the first.
SELFTEST_BASELINE_OWN = (
    ("a registration reason naming the product",
     "docs/baseline/verbatim-declaration-keeps.tsv",
     "doc/emacs/abbrevs.texi\t1\t@c This is part of the Emacs manual.\t"
     "manual-membership\tthe titles are narrative (934 `GNU Emacs` lines in `doc/`)\n",
     ["title"]),
    ("a quoted sentence in the column before it is a measured value",
     "docs/baseline/verbatim-declaration-keeps.tsv",
     "doc/lispref/abbrevs.texi\t1\t@c This is part of the GNU Emacs Lisp Reference "
     "Manual.\tmanual-membership\tthe copied sentence stays\n", []),
    ("a registration reason carrying the removed version form",
     "docs/baseline/declarations-allowlist.tsv",
     "src/x.rs\tliteral\t1\tconst P: &CStr = c" + chr(34) + "x" + chr(34) + ";\t"
     "the row above was `// femacs: port of src/x.c @ fd7afced403 (GNU Emacs 32.0.50).`\n",
     ["version"]),
    ("a measured-value table has no reason column to read",
     "docs/baseline/config/darwin-arm64.macros.tsv",
     'PACKAGE_NAME\t"GNU Emacs"\n', []),
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
    ("the registry key spelled as prose is a finding in a comment",
     "// C 78: `#define REG_ROOT " + chr(34) + "SOFTWARE\\GNU\\Emacs" + chr(34) + "`\n", ["title"]),
    ("the registry key spelled as a literal is a finding in code",
     'const R: &CStr = c"SOFTWARE\\\\GNU\\\\Emacs";\n', ["literal"]),
)


# The ledger scope's other kinds (status 3.679).  One case per reader the suffix selects:
# the tables' three classes and the draft's source model, plus the provenance note that must
# stay a note - if the walk or the dispatch were dropped, the first four would come back empty.
SELFTEST_LEDGER = (
    ("a table row carrying the copied title", ".tsv",
     "# This file is part of GNU Emacs.\n", ["title"]),
    ("a table row carrying the removed version form", ".txt",
     "# femacs: port of src/x.c @ fd7afced403 (GNU Emacs 32.0.50).\n", ["version"]),
    ("a table row carrying the ownership prose", ".tsv",
     "C 1: GNU Emacs is free software: you can redistribute it.\n", ["decl"]),
    ("a draft's product-name literal", ".rs",
     'const P: &CStr = c"GNU Emacs";\n', ["literal"]),
    ("a draft's provenance note is not a version line", ".rs",
     "// femacs: port of src/lisp.h @ fd7afced403.\n", []),
)


# The log scope's cases (status 3.750).  One per citation kind, and one for the shape the whole
# scope exists to catch: the log speaking in its own voice.  The second case is the `carried`
# model - the span opens at the end of the first line and closes after the name on the second.
SELFTEST_LOG = (
    ("the reference's measured output pasted into a fence",
     "```\n$ build/emacs-ref/src/emacs --version\nGNU Emacs 32.0.50\n```\n", []),
    ("a code span the log wraps across two lines",
     "**not rewritten**: the first line (`/* Cursor motion\n"
     "subroutines for GNU Emacs.  */`) - it is the copied text, GPL requires it.\n", []),
    ("a name inside a quote pair",
     'the sentence ("a one-to-one replication of GNU Emacs") stays quoted\n', []),
    ("a name inside the corner quotes",
     "\u8bd1\u6587\u5904\uff081056\u300c\u672c\u6587\u4ef6\u4e3a GNU Emacs \u7684\u4e00\u90e8\u5206\u300d\uff09\n", []),
    ("the instruction's own words",
     '- \u7528\u6237\u7684\u957f\u671f\u6307\u4ee4\uff08"\u7248\u672c\u58f0\u660e\u4e0d\u8981\u518d\u8bf4\u662f GNU Emacs \u4e86"\uff09\n', []),
    ("the log speaking in its own voice",
     "- \u672c\u7a0b\u5e8f\u662f GNU Emacs \u7684\u91cd\u5199\u3002\n", ["title"]),
    ("a stray backtick does not carry across a blank line",
     "- `stray\n\n- \u672c\u7a0b\u5e8f\u662f GNU Emacs \u7684\u91cd\u5199\u3002\n", ["title"]),
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
    for name, source, want in SELFTEST_BASELINE_TABLE:
        got = sorted({k for k, _ in baseline_findings_text(source)})
        ok = got == sorted(want)
        print(f"{'ok  ' if ok else 'FAIL'} {name}: {got or 'none'}")
        bad += 0 if ok else 1
    for name, rel, source, want in SELFTEST_BASELINE_OWN:
        got = sorted({k for k, _ in baseline_own_findings(rel, source)})
        ok = got == sorted(want)
        print(f"{'ok  ' if ok else 'FAIL'} {name}: {got or 'none'}")
        bad += 0 if ok else 1
    for name, source, want in SELFTEST_LOG:
        got = sorted({k for k, _ in log_findings_text(source)[0]})
        ok = got == sorted(want)
        print(f"{'ok  ' if ok else 'FAIL'} {name}: {got or 'none'}")
        bad += 0 if ok else 1
    for name, source, want in SELFTEST_TOOL:
        got = sorted({k for _, k, _ in findings_text(source, ".py")})
        ok = got == sorted(want)
        print(f"{'ok  ' if ok else 'FAIL'} {name}: {got or 'none'}")
        bad += 0 if ok else 1
    for name, suffix, source, want in SELFTEST_LEDGER:
        got = sorted({k for k, _ in ledger_findings_text(source, suffix)})
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
             + len(SELFTEST_ASM_SPLIT) + len(SELFTEST_BASELINE) + len(SELFTEST_BASELINE_TABLE)
             + len(SELFTEST_TOOL) + len(SELFTEST_LEDGER) + len(SELFTEST_BASELINE_OWN)
             + len(SELFTEST_LOG))
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
    doc = doc_files(root)
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
    # 3.250 added it because `platform` was "scanned" by name while its seven
    # `.swift` files - the ones transcribing `src/*.m` - were read by nobody.
    spans = " ".join(f"{n} {ext}" for ext, n in sorted(by_suffix.items(), key=lambda kv: -kv[1]))
    print(f"read {len(read)} files ({spans})")
    kinds: dict[str, int] = {}
    for _, path in prose:
        kinds[path.suffix or "(no suffix)"] = kinds.get(path.suffix or "(no suffix)", 0) + 1
    span = " ".join(f"{n} {ext}" for ext, n in sorted(kinds.items(), key=lambda kv: -kv[1]))
    print(f"read {len(prose)} own prose/manifest file(s) ({span})")
    dkinds: dict[str, int] = {}
    for _, path in doc:
        dkinds[path.suffix] = dkinds.get(path.suffix, 0) + 1
    dspan = " ".join(f"{n} {ext}" for ext, n in sorted(dkinds.items(), key=lambda kv: -kv[1]))
    print(f"read {len(doc)} ledger-scope file(s) ({dspan})")
    bfiles, blines, bdeep, bpayload, bpayload_files, bown = baseline_stats(root)
    print(f"read {blines} comment line(s) in {bfiles} docs/baseline file(s) "
          f"({bdeep} of them below the leading block the goldens' reader stops at); "
          f"read the reason column of {len(BASELINE_OWN_COLUMN)} registration table(s) as "
          f"prose ({bown} cell(s) naming the product); "
          f"left as measured values: {bpayload} payload line(s) naming the product in "
          f"{bpayload_files} file(s)")
    print(f"read {len(BASELINE_PROSE)} baseline header(s) by name "
          f"({', '.join(BASELINE_PROSE)}), records below the header left to the exemption")
    lfiles, llines, lfence, lspan, lcarry, lquote, linstr, lown = log_stats(root)
    print(f"read {lfiles} sanctioned log file(s) by their citation band "
          f"({', '.join(LOG_SCOPE)}): {llines} line(s) carry the product's name - "
          f"{lfence} in a fenced block, {lspan} in a code span ({lcarry} of them by a span the "
          f"previous line left open), {lquote} in a quote pair, {linstr} the instruction's own "
          f"words, {lown} as our own prose")
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
    # `xtask/Cargo.toml`), so the line stays printed with the count it now reads.
    covered = ({rel for rel, _ in prose} | {rel for rel, _ in doc}
               | {rel for rel, _ in baseline_table_files(root)} | set(BASELINE_PROSE))
    sanctioned = tuple(prefix for prefix, _ in PROSE_EXEMPT)
    sanctioned_left = [s for s in skip
                       if any(s == p or s.startswith(p) for p in sanctioned)]
    rest = [s for s in skip if s not in covered and s not in sanctioned_left]
    print(f"walked past {len(rest)} file(s) of an unscanned kind: "
          + (", ".join(sorted(rest)[:8]) + (" ..." if len(rest) > 8 else "") if rest else "none")
          + (f" ({len(skip) - len(rest) - len(sanctioned_left)} of them read above)"
             if len(rest) != len(skip) - len(sanctioned_left) else ""))
    if sanctioned_left:
        print(f"left under a sanctioned prefix (the reference's measured values, the "
              f"per-unit ledgers - not a hole): {len(sanctioned_left)} file(s) "
              f"- e.g. {sorted(sanctioned_left)[0]}")
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
