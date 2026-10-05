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
                   it falls in and the reason it stays.

Anything else is a failure, printed with its path, line number and text - including the one
shape a byte gate cannot see: a candidate that a `docs/baseline/verbatim-substitutions.tsv`
row already covers.  That row says the line carries this repository's name, so a line that
still names the product (a file re-copied from the reference) contradicts it, and the row's
own text is checked against the tree by `tools/check-verbatim.py`.  The candidates
are the *shape* (a file being made part of something), not the word "part of" alone:

  <This|These|The following|They|…|NAME.el|NAME.rnc> … <is|are> … <part of|annex of>

and the line must also carry the product's readable name (bare `Emacs` included - that is
the spelling the manual-membership sentence uses).  The bound is a model, like the other
gates' pattern tables: prose whose subject is not a thing of this repository (`Emacs
deletes the part of the file name that matched REGEXP`) is not a declaration and is out of
scope here, and `--selftest` has a case for exactly that line.

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
# `docs/port-map.md`'s VERBATIM_DIRS (`<reference dir>` -> `femacs/<same name>`).
TREES = ("lisp", "test", "etc", "doc", "leim", "admin")
TARGET = "femacs"
SUBSTITUTIONS = REPO / "docs/baseline/verbatim-substitutions.tsv"
KEEPS = REPO / "docs/baseline/verbatim-declaration-keeps.tsv"

# The product's readable names, as in `tools/check-declarations.py`'s PRODUCT table; the
# literals below are why this file is on that gate's tool exemption list (the tool has to
# write the name it looks for).  The bare `Emacs` is wanted here and *not* there: the
# manual-membership sentence names the manual, and a scan that only knew the spelled-out
# name would miss the 54 `the Emacs manual` rows.
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
HISTORY = re.compile(r"(?:^|/)ChangeLog(?:\.|$)|(?:^|/)(?:NEWS|NEWS\.[0-9][0-9.]*|HISTORY|"
                     r"MH-E-NEWS|ORG-NEWS|PROBLEMS|DEBUG)$")
CLASSES = ("forbidden-tree", "history", "negation", "enumerated")


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
           trees: tuple[str, ...] = TREES) -> tuple[dict[str, int], list[str], int, int]:
    """`(counts by class, failures, candidates, sanctioned)`."""
    counts = {c: 0 for c in CLASSES}
    failures: list[str] = []
    candidates = 0
    sanctioned = 0
    used_keeps: set[str] = set()
    for tree in trees:
        prefix = f"{TARGET}/{tree}"
        for rel in tracked_files(repo, prefix):
            p = repo / rel
            sub_rows = {ln: txt for ln, txt in subs.get(rel, [])}
            keep_rows = {ln: txt for ln, txt in keeps.get(rel, [])}
            try:
                with p.open("r", encoding="utf-8", errors="replace") as fh:
                    for line_no, line in enumerate(fh, 1):
                        text = line.rstrip("\n")
                        if not (PRODUCT.search(text) and MEMBERSHIP.search(text)
                                and DECLARATION.search(text)):
                            continue
                        candidates += 1
                        if line_no in sub_rows:
                            failures.append(
                                f"{rel}:{line_no} a substitution row says this line carries this"
                                f" repository's name, but the line still names the product:"
                                f" {text!r} (row: {sub_rows[line_no]!r})")
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
                                failures.append(f"{rel}:{line_no} the keep row's text is"
                                                f" {keep_rows[line_no]!r}, the line is {text!r}")
                                continue
                            counts["enumerated"] += 1
                            sanctioned += 1
                            continue
                        failures.append(f"{rel}:{line_no} unclassified declaration: {text!r}")
            except OSError as exc:
                failures.append(f"{rel}: {exc}")
    scanned = tuple(f"{TARGET}/{tree}/" for tree in trees)
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
    return counts, failures, candidates, sanctioned


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--selftest", action="store_true", help="prove each class fires")
    args = ap.parse_args()
    if args.selftest:
        return selftest()
    subs = read_table(SUBSTITUTIONS)
    keeps = read_table(KEEPS)
    counts, failures, candidates, sanctioned = census(REPO, subs, keeps)
    for name in CLASSES:
        print(f"{name:15s} {counts[name]:6,d}")
    rows = sum(len(v) for v in subs.values())
    print(f"read {len(TREES)} copied tree(s): {candidates} candidate(s), {sanctioned} "
          f"enumerated keep(s), {sum(counts.values())} classified, {len(failures)} failure(s)")
    print(f"the substitution table carries {rows} row(s) for these trees - the lines the tenth"
          f" to fourteenth halves renamed")
    for line in failures[:20]:
        print(f"          {line}")
    return 1 if failures else 0


def selftest() -> int:
    """Nine readings with the fixed shape: the clean line passes, each mistake fires once."""
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
            counts, failures, cand, _ = census(root, sub_fix, keep_fix, trees=(tree,))
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
    counts, failures, cand, _ = census(REPO, read_table(SUBSTITUTIONS), read_table(KEEPS),
                                      trees=("lisp",))
    print(f"{'ok  ' if not failures else 'FAIL'} the real lisp/ tree: {cand} candidate(s), "
          f"{len(failures)} unclassified")
    for line in failures[:5]:
        print(f"       {line}")
    bad += 0 if not failures else 1
    print(f"selftest: {len(cases) + 1} case(s), {bad} failure(s)")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
