#!/usr/bin/env python3
"""Check the *quoted text* of the ported files' `/// C: <file> <line>: `<quote>`` citations.

tools/check-citations.py (docs/status.md 3.166) checks citations structurally: the cited
range exists, and a backticked *type name* appears somewhere in the reference file.  It
deliberately never compares the quoted C text against the cited line, so a citation whose
line number drifted - or whose quote was copied from a different declaration - passes it
untouched.  This tool does the other half, mechanically:

  checked     a single-line citation whose quoted text was compared against the cited line
  mismatched  the quoted text does not occur on the cited line (a wrong line or a wrong quote)
  ranges      citations that name a line range (`4344-4345`), which cannot be line-compared
  unresolved  the reference file was not found in `emacs/{,src,lib,lib-src}`, the cited line
              is outside the file, or the citation carries no backticked quote

Comparison is whitespace-insensitive and one-directional on purpose: whitespace runs collapse
to one space, and the quote counts as present when it is a substring of the cited line.  The
tool only reports; fixing a citation is a human (or reviewed) decision - the rewrite queue
below is printed as `path:line  cited-file line  quoted  actual`.

Two deliberate refusals keep the report honest:

  * A quote that spans several C lines (a long prototype) is reported `mismatched`: the
    comparison is per line, so a citation that points at the declaration's *first* line while
    quoting the whole declaration is flagged even though the line is right.  A range citation
    (`4344-4345`) sidesteps this and is counted under `ranges` instead of line-checked.
  * A citation whose line number is a placeholder (`lisp.h 4xxx:`) is not matched at all:
    the digits must be a standalone token, so `4xxx` is not read as line 4.

Because the fix is out of scope for range citations, the `mismatched` count is a *review
queue*: each row has to be triaged into a wrong line/file (fixable) or a cross-line /
generated-header / `#if`-duplicate citation (reported, not guessed at).

    python3 tools/check-citation-text.py femacs/src femacs/lib
    python3 tools/check-citation-text.py --all          # also print matching citations
    python3 tools/check-citation-text.py --limit 40
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
SUFFIXES = {".rs", ".swift"}
DEFAULT_TREES = ["femacs/src", "femacs/lib"]

# The roots the citation text may live under, in the task's order.
ROOTS = ["emacs", "emacs/src", "emacs/lib", "emacs/lib-src"]

# `/// C: <file> <line>[-<end>][: ...]`.  Only citations that name at least one line are the
# subject here; the `<stdlib.h> `...`` form carries no line and is out of this tool's scope.
CITE = re.compile(
    r"^\s*//[/!]{0,2}\s*C:\s*"
    r"(?P<path>[A-Za-z0-9_./+-]+\.(?:in\.h|h|m|mm|cc|c))\s+"
    r"(?P<start>\d+)(?:\s*-\s*(?P<end>\d+))?(?![0-9A-Za-z_])\s*:?(?P<rest>.*)$"
)
BACKTICK = re.compile(r"`([^`]+)`")
WS = re.compile(r"\s+")


def c_lines_text(text: str) -> list[str]:
    """C's own line numbering: split on `\\n`, so an Emacs `^L` form feed is not a line break."""
    lines = text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    return lines


def norm(s: str) -> str:
    return WS.sub(" ", s).strip()


def resolve(name: str) -> Path | None:
    for root in ROOTS:
        cand = REPO / root / name
        if cand.is_file():
            return cand
    return None


def citation_quote(rest: str) -> str | None:
    """The first backticked span in the citation tail, or None."""
    spans = BACKTICK.findall(rest)
    if not spans:
        return None
    return spans[0]


class Finding:
    __slots__ = ("kind", "path", "lineno", "cited", "quoted", "actual")

    def __init__(self, kind, path, lineno, cited, quoted, actual=""):
        self.kind = kind
        self.path = path
        self.lineno = lineno
        self.cited = cited
        self.quoted = quoted
        self.actual = actual


def scan_file(rel: str, text: str) -> list[Finding]:
    out: list[Finding] = []
    cache: dict[Path, list[str]] = {}
    for lineno, line in enumerate(c_lines_text(text), 1):
        m = CITE.match(line)
        if not m:
            continue
        name = m.group("path")
        start = int(m.group("start"))
        end = m.group("end")
        if end is not None:
            out.append(Finding("range", rel, lineno, f"{name} {start}-{end}", ""))
            continue
        ref = resolve(name)
        if ref is None:
            out.append(Finding("unresolved", rel, lineno, f"{name} {start}", "no reference file"))
            continue
        if ref not in cache:
            cache[ref] = c_lines_text(ref.read_text(encoding="utf-8", errors="replace"))
        lines = cache[ref]
        if start < 1 or start > len(lines):
            out.append(Finding("unresolved", rel, lineno, f"{name} {start}",
                               f"file has {len(lines)} line(s)"))
            continue
        quoted = citation_quote(m.group("rest"))
        if quoted is None:
            out.append(Finding("unresolved", rel, lineno, f"{name} {start}", "no backticked quote"))
            continue
        actual = lines[start - 1]
        if norm(quoted) in norm(actual):
            out.append(Finding("checked", rel, lineno, f"{name} {start}", quoted, actual.strip()))
        else:
            out.append(Finding("mismatched", rel, lineno, f"{name} {start}", quoted, actual.strip()))
    return out


def collect(trees: list[str]) -> list[Finding]:
    findings: list[Finding] = []
    seen: set[Path] = set()
    for tree in trees:
        base = REPO / tree
        if base.is_file():
            files = [base]
        else:
            files = sorted(p for p in base.rglob("*") if p.suffix in SUFFIXES)
        for path in files:
            if path in seen:
                continue
            seen.add(path)
            rel = str(path.relative_to(REPO))
            findings.extend(scan_file(rel, path.read_text(encoding="utf-8", errors="replace")))
    return findings


def main() -> int:
    ap = argparse.ArgumentParser(description="Compare `/// C: <file> <line>: `<quote>`` text.")
    ap.add_argument("trees", nargs="*", default=DEFAULT_TREES,
                    help=f"directories or files to scan (default: {' '.join(DEFAULT_TREES)})")
    ap.add_argument("--all", action="store_true", help="print matching citations too")
    ap.add_argument("--limit", type=int, default=0, help="print at most N findings (0 = all)")
    args = ap.parse_args()
    trees = args.trees or DEFAULT_TREES

    findings = collect(trees)
    checked = [f for f in findings if f.kind == "checked"]
    mismatched = [f for f in findings if f.kind == "mismatched"]
    ranges = [f for f in findings if f.kind == "range"]
    unresolved = [f for f in findings if f.kind == "unresolved"]

    rows = (checked if args.all else []) + mismatched
    if args.limit:
        rows = rows[: args.limit]
    for f in rows:
        if f.kind == "mismatched":
            print(f"{f.path}:{f.lineno}\t{f.cited}\t`{f.quoted}`\t{f.actual}")
        else:
            print(f"{f.path}:{f.lineno}\t{f.cited}\tok")

    print(f"checked {len(checked)} / mismatched {len(mismatched)} / "
          f"ranges {len(ranges)} / unresolved {len(unresolved)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
