#!/usr/bin/env python3
"""Census: every *membership declaration* in the copied trees is renamed or sanctioned.

`tools/check-verbatim.py` answers "is the copy byte-identical?".  It cannot answer the
other half of PORTING.md 0.1-19: **which lines of the copy say who owns them**.  A file
re-copied from the reference passes the byte gate trivially and brings the reference's
membership declaration back with it, and a declaration the rule missed is invisible until
somebody reads four and a half million lines by hand.  That is the blind spot this gate
closes (status 3.656): the six copied trees are read by a *declaration* reader, and every
candidate line has to be one of

  forbidden-tree   it is under `test/`, which the goal text forbids modifying (the 651
                   files keep the reference's sentence);
  history          it is in a `ChangeLog*`/`NEWS*`/`HISTORY` file, which *quotes* the
                   sentence as a record of what the reference's tree said at the time;
  negation         the line denies the membership (`is not a part of …`), so renaming it
                   would turn a true statement into a false one;
  enumerated       `docs/baseline/verbatim-declaration-keeps.tsv` names it, line by line
                   (`<path>\t<line>\t<the line's text>\t<class>\t<reason>`), with the class
                   it falls in and the reason it stays (`language-name` for a title that
                   names the language rather than this program).

Anything else is a failure, printed with its path, line number and text - including the one
shape a byte gate cannot see: a candidate that a `docs/baseline/verbatim-substitutions.tsv`
row already covers.  That row says the line carries this repository's name, so a line that
still names the product (a file re-copied from the reference) contradicts it, and the row's
own text is checked against the tree by `tools/check-verbatim.py`.  The candidates
are the *shape* (a file being made part of something), not the word "part of" alone:

  <This|These|The following|They|…|NAME.el|NAME.rnc> … <is|are> … <part of|annex of>

and the line must also carry the product's readable name (bare `Emacs` included - that is
the spelling the manual-membership sentence uses).

The rule has a second shape (the thirty-second half, status 4.034): the file's own *opening
title line* - line 1, or the line after a directive that must stay first (`%&tex`/`%&mex`,
`#!`, `<?xml …?>`) - is a candidate whenever it carries the *spelled-out* product name.  A
copied file's title says what the file is (`;;; array.el --- array editing commands for GNU
Emacs`), and it sits three lines above a membership sentence that already carries this
repository's name, so a re-copy brings the product back in a line no reader read before.
Bare `Emacs` is out of this shape on purpose: in a title it is the language's name (`;;
cl-compat.el --- Common Lisp extensions for GNU Emacs Lisp`, kept and enumerated).  The bound is a model, like the other
gates' pattern tables: prose whose subject is not a thing of this repository (`Emacs
deletes the part of the file name that matched REGEXP`) is not a declaration and is out of
scope here, and `--selftest` has a case for exactly that line.

The statement may be **wrapped across two lines**: the subject ends one line and the verb
opens the next (`... the chapters included below` / `@c are part of the Emacs User
manual.`).  A line-shaped reader cannot see those - the continuation carries no subject and
the subject carries no membership phrase - so a wrapped declaration passes every earlier
half of the rule and comes back with a re-copy, unannounced.  When a line *opens* with the
verb (`is`/`are`/`was`/`were`, after whatever the file's line prefix is - `@c `, `;;`, `# `)
it is read together with the line above it: the continuation's prefix is dropped, the two
lines are joined with one space and the *joined* text is put through the shape above.  Such
a candidate is keyed at the continuation line, and the text the two tables carry for it is
that joined statement (status 3.666's fifteenth half).

Readings: exit 1 on any unclassified candidate, on a keep row that does not match the
tree, or on a keep row whose line is not a candidate (the table cannot collect prose).

    python3 tools/check-verbatim-declarations.py
    python3 tools/check-verbatim-declarations.py --selftest
"""

from __future__ import annotations

import argparse
import re
import subprocess
import sys
import tempfile
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
# `docs/port-map.md`'s VERBATIM_DIRS (`<reference dir>` -> `<same name>`).
TREES = ("lisp", "test", "etc", "doc", "leim", "admin")
TARGET = "."
SUBSTITUTIONS = REPO / "docs/baseline/verbatim-substitutions.tsv"
KEEPS = REPO / "docs/baseline/verbatim-declaration-keeps.tsv"

# The product's readable names, as in `tools/check-declarations.py`'s PRODUCT table; the
# literals below are why this file is on that gate's tool exemption list (the tool has to
# write the name it looks for).  The bare `Emacs` is wanted here and *not* there: the
# manual-membership sentence names the manual, and a scan that only knew the spelled-out
# name would miss the manual-membership sentence (`@c This is part of the Emacs manual.`
# and its Lisp-reference sibling: 116 rows the rule's thirtieth half renamed, status 4.008 -
# the spelling stays in the pattern because a re-copy brings the sentence back).
PRODUCT = re.compile(r"\bGNU Emacs\b|\bgnu emacs\b|\bGNU\.Emacs\b|\bGnu Emacs\b|(?<!GNU )\bEmacs\b")
MEMBERSHIP = re.compile(r"\bpart of\b|\bannex of\b|\bbelongs to\b", re.I)
# The subject has to be a thing this repository carries: this document, these files, or a
# file named on the line.  `[A-Za-z0-9_.-]+\.(el|rnc|xml|txt)` is the measured set of
# subjects the copied trees write that way (`locate.rnc, … are part of …`).
SUBJECT = (r"(?:This|These|The following|The (?:file|files|chapters|icons|schemas?)|They|"
           r"[A-Za-z0-9_][A-Za-z0-9_.-]*\.(?:el|rnc|xml|txt))")
DECLARATION = re.compile(
    r"(?:^|[\s#;*%@/\[(])" + SUBJECT + r"(?:\s+[A-Za-z0-9_'\"\-.,]+){0,8}?\s+(?:is|are|was|were)\b"
    r"(?:\s+[A-Za-z0-9_'\"\-.,]+){0,6}?\s+(?:part of|an annex of|a part of)\b", re.I)
NEGATION = re.compile(r"\bnot\b[^.;]{0,24}?\bpart of\b|\bnever\b[^.;]{0,24}?\bpart of\b|"
                      r"\bisn't\b[^.;]{0,24}?\bpart of\b|\bno longer\b[^.;]{0,24}?\bpart of\b", re.I)
# The files that *quote* the sentence as history (`lisp/ChangeLog.17`, `etc/NEWS.22`,
# `etc/HISTORY`); the classes and their readings are in the module docstring.
HISTORY = re.compile(r"(?:^|/)ChangeLog(?:\.|$)|(?:^|/)(?:NEWS|NEWS\.[0-9][0-9.-]*|HISTORY|"
                     r"MH-E-NEWS|ORG-NEWS|PROBLEMS|DEBUG)$")
# A continuation line: the file's line prefix and then the statement's verb.  The prefixes
# below are the measured ones (`@c ` in `doc/`, `;;` in `lisp/`, `# ` in the shell scripts);
# a prefix with a *letter* in it (`@c `) is why this is a list and not `\W+`, which stops at
# the `c` and makes the line open with `c are part of ...`.
PREFIX = re.compile(r"^(?:\s*(?:@c|@comment|;;+|;|#+|//+|\*+|%)\s+|\s*)")
CONTINUATION = re.compile(r"(?:is|are|was|were)\b", re.I)
CLASSES = ("forbidden-tree", "history", "negation", "enumerated")
# The file's own *opening title line*: what a copied file calls itself (`;;; foo.el --- …`,
# `% Reference Card for …`, `#+title: …`), or - when the file opens with a directive that has
# to stay first (`%&tex`, `%&mex`, `#!`, `<?xml …?>`) - the line right after it (the 32nd half).
# A file re-copied from the reference brings that line back the same way it brings a
# membership sentence back, and until this reader existed nothing read it: the byte gate
# compares bytes, and the membership shape above wants a "part of" phrase.
TITLE_DIRECTIVE = re.compile(r"^(?:%&(?:tex|mex|latex|plain)\b|#!|<\?xml)")
# The spelled-out product name only.  The bare `Emacs` the membership shape needs is the
# *language's* name in the titles these trees write (`Emacs Lisp`, `An Emacs mode`, the
# `emacs-lisp-intro` manual the fourteenth and thirty-first halves keep), so the title shape
# bounds itself to the spelled-out forms the substitutions table carries.
PRODUCT_SPELLED = re.compile(r"\bGNU Emacs\b|\bgnu emacs\b|\bGNU\.Emacs\b|\bGnu Emacs\b")


def opening_title(lines: list[str]) -> int | None:
    """The index of the file's own opening title line, or None when it has none."""
    if not lines or not lines[0].strip():
        return None
    idx = 1 if TITLE_DIRECTIVE.match(lines[0]) and len(lines) > 1 else 0
    return idx if idx < len(lines) and lines[idx].strip() else None


def continuation_body(text: str) -> str | None:
    """`text` minus its line prefix if it opens with a wrapped statement's verb, else None."""
    m = PREFIX.match(text)
    body = text[m.end():] if m else text
    return body if CONTINUATION.match(body) else None


def membership_object(text: str) -> str:
    """The words the statement's first membership phrase points at (`part of <object>`).

    The shape's second bound, and the one the manual-style prose needed: the phrase has to
    point *at the product* (`part of GNU Emacs`, `part of the Emacs manual`), not merely sit
    on a line that mentions it (`... (@pxref{Display Custom,,, emacs, The GNU Emacs
    Manual}) do not count as decorations either; they are part of the window's body too.`).
    Eight words is the measured width of the objects the trees write (`part of the standard
    distribution of Emacs.` is five)."""
    m = DECLARATION.search(text)
    return " ".join(text[m.end():].split()[:8]) if m else ""


def is_declaration(text: str) -> bool:
    return bool(PRODUCT.search(text) and MEMBERSHIP.search(text) and DECLARATION.search(text)
                and PRODUCT.search(membership_object(text)))


def escape(text: str) -> str:
    return text.replace("\\", "\\\\").replace("\t", "\\t")


def unescape(text: str) -> str:
    out: list[str] = []
    i = 0
    while i < len(text):
        if text[i] == "\\" and i + 1 < len(text) and text[i + 1] in "\\t":
            out.append("\\" if text[i + 1] == "\\" else "\t")
            i += 2
            continue
        out.append(text[i])
        i += 1
    return "".join(out)


def read_table(path: Path) -> dict[str, list[tuple[int, str]]]:
    """`<repo-relative path> -> [(line, cell 3)]`, for a table whose columns 1-3 identify a line."""
    rows: dict[str, list[tuple[int, str]]] = {}
    if not path.is_file():
        return rows
    for raw in path.read_text(encoding="utf-8").splitlines():
        if not raw.strip() or raw.startswith("#") or raw.startswith("path\t"):
            continue
        parts = raw.split("\t")
        if len(parts) < 4 or not parts[1].strip().isdigit():
            print(f"BAD ROW {path.name}: {raw!r}")
            continue
        rows.setdefault(parts[0], []).append((int(parts[1]), unescape(parts[2])))
    return rows


def tracked_files(repo: Path, prefix: str) -> list[str]:
    r = subprocess.run(["git", "-C", str(repo), "ls-files", "-z", "--", prefix],
                       capture_output=True, text=True)
    return [p for p in r.stdout.split("\0") if p]


def census(repo: Path, subs: dict[str, list[tuple[int, str]]],
           keeps: dict[str, list[tuple[int, str]]],
           trees: tuple[str, ...] = TREES) -> tuple[dict[str, int], list[str], int, int, int]:
    """`(counts by class, failures, candidates, sanctioned, wrapped candidates)`."""
    counts = {c: 0 for c in CLASSES}
    failures: list[str] = []
    candidates = 0
    sanctioned = 0
    wrapped = 0
    used_keeps: set[str] = set()
    for tree in trees:
        prefix = str(Path(TARGET) / tree)
        for rel in tracked_files(repo, prefix):
            p = repo / rel
            sub_rows = {ln: txt for ln, txt in subs.get(rel, [])}
            keep_rows = {ln: txt for ln, txt in keeps.get(rel, [])}
            try:
                lines = p.read_text(encoding="utf-8", errors="replace").split("\n")
            except OSError as exc:
                failures.append(f"{rel}: {exc}")
                continue
            title_idx = opening_title(lines)
            for idx, line in enumerate(lines):
                # A candidate is one line's text, or - when this line opens with the
                # continuation's verb - the line above joined with it.
                line_no = idx + 1
                where = f"{rel}:{line_no}"
                text = line
                if idx == title_idx and PRODUCT_SPELLED.search(line):
                    # The file's own opening title line (the 32nd half): a candidate as it
                    # stands - no join, no membership phrase needed.
                    pass
                elif not is_declaration(text):
                    # A wrapped statement: the line above must *not* carry the phrase
                    # already, or the join would merely glue two statements together and
                    # report the one above a second time (`etc/images/README:45-46`).
                    body = (continuation_body(line)
                            if idx > 0 and not MEMBERSHIP.search(lines[idx - 1]) else None)
                    if body is None:
                        continue
                    text = lines[idx - 1] + " " + body
                    where = f"{rel}:{line_no - 1}-{line_no}"
                    if not is_declaration(text):
                        continue
                    wrapped += 1
                candidates += 1
                if line_no in sub_rows:
                    failures.append(
                        f"{where} a substitution row says this line carries this repository's"
                        f" name, but the statement still names the product: {text!r}"
                        f" (row: {sub_rows[line_no]!r})")
                    continue
                if tree == "test":
                    counts["forbidden-tree"] += 1
                    continue
                if HISTORY.search(rel):
                    counts["history"] += 1
                    continue
                if NEGATION.search(text):
                    counts["negation"] += 1
                    continue
                if line_no in keep_rows:
                    # The row is *read* whatever its text says, so a mismatch is one
                    # failure (the text) and not two (the text and "it went unread").
                    used_keeps.add(f"{rel}:{line_no}")
                    if keep_rows[line_no] != text:
                        failures.append(f"{where} the keep row's text is"
                                        f" {keep_rows[line_no]!r}, the statement is {text!r}")
                        continue
                    counts["enumerated"] += 1
                    sanctioned += 1
                    continue
                failures.append(f"{where} unclassified declaration: {text!r}")
    scanned = tuple(str(Path(TARGET) / tree) + "/" for tree in trees)
    for rel, rows in keeps.items():
        if not rel.startswith(scanned):
            continue
        if not (repo / rel).is_file():
            failures.append(f"{rel}: a keep row for a file the copy does not carry")
            continue
        for line_no, _text in rows:
            if f"{rel}:{line_no}" not in used_keeps:
                failures.append(f"{rel}:{line_no} the keep row is stale (or its line is not a"
                                f" candidate)")
    return counts, failures, candidates, sanctioned, wrapped

def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--selftest", action="store_true", help="prove each class fires")
    args = ap.parse_args()
    if args.selftest:
        return selftest()
    subs = read_table(SUBSTITUTIONS)
    keeps = read_table(KEEPS)
    counts, failures, candidates, sanctioned, wrapped = census(REPO, subs, keeps)
    for name in CLASSES:
        print(f"{name:15s} {counts[name]:6,d}")
    rows = sum(len(v) for v in subs.values())
    print(f"read {len(TREES)} copied tree(s): {candidates} candidate(s) of which {wrapped} "
          f"wrapped across two lines, {sanctioned} enumerated keep(s), {sum(counts.values())} "
          f"classified, {len(failures)} failure(s)")
    print(f"the substitution table carries {rows} row(s) for these trees - the lines the tenth"
          f" to thirty-second halves renamed")
    for line in failures[:20]:
        print(f"          {line}")
    return 1 if failures else 0


def selftest() -> int:
    """Readings with the fixed shape: the clean line passes, each mistake fires once."""
    declaration = ";; This file is part of GNU Emacs."
    prose = "Emacs deletes the part of the file name that matched REGEXP"
    cases: list[tuple[str, str, dict, dict, int, int]] = [
        # name, line, substitutions, keeps, want failures, want candidates
        ("a declaration with no row and no keep", declaration, {}, {}, 1, 1),
        ("a line a substitution row already covers", declaration,
         {"x.el": [(1, ";; This file is part of femacs.")]}, {}, 1, 1),
        ("a negated declaration",
         ";; This file is not part of GNU Emacs.", {}, {}, 0, 1),
        ("an enumerated keep", declaration, {},
         {"x.el": [(1, declaration)]}, 0, 1),
        ("a keep row whose text is wrong", declaration, {},
         {"x.el": [(1, declaration + "!")]}, 1, 1),
        ("a keep row that is not a candidate", prose, {}, {"x.el": [(1, prose)]}, 1, 0),
        ("prose that is not a declaration", prose, {}, {}, 0, 0),
        ("a declaration under test/", declaration, {}, {}, 0, 1),
        ("a declaration wrapped across two lines",
         "the chapters included below\n@c are part of the Emacs User manual.", {}, {}, 1, 1),
        ("a wrapped declaration a keep row enumerates",
         "the chapters included below\n@c are part of the Emacs User manual.", {},
         {"x.el": [(2, "the chapters included below are part of the Emacs User manual.")]},
         0, 1),
        ("a wrapped declaration that already carries this repository's name",
         "the chapters included below\n@c are part of the femacs User manual.", {}, {}, 0, 0),
        ("a wrapped continuation that names no product",
         "the chapters included below\n@c are part of the manual.", {}, {}, 0, 0),
        ("a wrapped line whose own line above already made the declaration",
         ";; This file is part of GNU Emacs.\nare part of the manual.", {}, {}, 1, 1),
        ("a statement whose object is not the product",
         ";; like Emacs, they are part of the window's body", {}, {}, 0, 0),
        ("a file's opening title line that names the product",
         ";;; x.el --- a mode for GNU Emacs", {}, {}, 1, 1),
        ("an opening title line a keep row enumerates",
         ";;; x.el --- a mode for GNU Emacs", {}, {"x.el": [(1, ";;; x.el --- a mode for GNU Emacs")]},
         0, 1),
        ("an opening title line under test/",
         ";;; x.el --- a mode for GNU Emacs", {}, {}, 0, 1),
        ("an opening title line after a format directive",
         "%&tex\n%% Reference Card for GNU Emacs", {}, {}, 1, 1),
        ("an opening title line whose token is the language's name",
         ";;; x.el --- Common Lisp extensions for GNU Emacs Lisp", {},
         {"x.el": [(1, ";;; x.el --- Common Lisp extensions for GNU Emacs Lisp")]}, 0, 1),
        ("a title line that already says this repository's name",
         ";;; x.el --- a mode for femacs", {}, {}, 0, 0),
    ]
    bad = 0
    with tempfile.TemporaryDirectory() as tmp:
        root = Path(tmp)
        for i, (name, line, subs, keeps, want_fail, want_cand) in enumerate(cases):
            tree = "test" if name.endswith("under test/") else "lisp"
            blob = f"{line}\n".encode()
            subprocess.run(["git", "init", "-q", str(root)], check=True)
            path = root / TARGET / tree / "x.el"
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(blob)
            subprocess.run(["git", "-C", str(root), "add", "-f",
                            f"{TARGET}/{tree}/x.el"], check=True)
            rel = f"{TARGET}/{tree}/x.el"
            sub_fix = {rel: rows for rows in subs.values()}
            keep_fix = {rel: rows for rows in keeps.values()}
            counts, failures, cand, _, _ = census(root, sub_fix, keep_fix, trees=(tree,))
            got_fail = len(failures)
            ok = got_fail == want_fail and cand == want_cand
            bad += 0 if ok else 1
            print(f"{'ok  ' if ok else 'FAIL'} {name}: {got_fail} failure(s), {cand} candidate(s)"
                  f" (want {want_fail}, {want_cand})")
            (root / TARGET).rename(root / f"{TARGET}.{i}")
    # The end-to-end reading: the real `lisp/` tree against the committed tables.  It is
    # here because a fixture cannot carry the two tables' own shapes (a `\t`-escaped row, a
    # 108-row keep class), and because the failure this gate exists for - a declaration that
    # survived a re-copy - is only visible on the real tree.
    counts, failures, cand, _, wrapped = census(REPO, read_table(SUBSTITUTIONS),
                                                read_table(KEEPS), trees=("lisp",))
    print(f"{'ok  ' if not failures else 'FAIL'} the real lisp/ tree: {cand} candidate(s) "
          f"({wrapped} wrapped), {len(failures)} unclassified")
    for line in failures[:5]:
        print(f"       {line}")
    bad += 0 if not failures else 1
    print(f"selftest: {len(cases) + 1} case(s), {bad} failure(s)")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
