#!/usr/bin/env python3
"""Check the `/// C: <file> <line(s)>` citations the ported files carry (docs/status.md 3.166).

Every mirror and unit quotes C's own text next to the line it came from (PORTING.md 0.1-2,
P-55), and a stale citation is worse than none: reviewers trust it, and this project has
already been bitten by wrong use-site notes three rounds running (3.153, 3.156, 3.157).
Nothing checked them - 998 citations live in the ported trees and `--check` only compares
items, not commentary.  This gate does the mechanical half, conservatively:

  range  the cited line range does not exist in the reference file
  external  the cited path is not in the tree at all (a system header - `jni.h`,
         `<sys/fileio.h>` - or a generated one); counted, never a failure
  drift  the first code-looking `backticked` snippet on the citation line does not appear
         within +-2 lines of the cited range, *but it does appear elsewhere in that same
         reference file* - so the file is right and the line is stale.  A snippet that
         appears nowhere in the file is silence: it is a paraphrase, a docstring, or a name
         from another file, and guessing about those would make the gate noisy.

A mechanical `--fix` was written and reverted in the same round (status 3.166): its
63 rewrites moved a cited range to the first line that quotes the same identifier, and
at least one changed the meaning - `// cm.c 23: lisp.h (globals.h, `emacs_abort`)`
cites an *include* line and only names an identifier, so replacing 23 with the first
line that says `emacs_abort` (122) is not a fix.  The list below is a review queue,
never a patch.

Deliberately not checked: prose paraphrases, quotes spread over several comment lines, and
citations of macros whose text C's preprocessor forms.  Those stay a reader's business.

    python3 tools/check-citations.py             # scan, print findings, fail if any
    python3 tools/check-citations.py --selftest
    python3 tools/check-citations.py --limit 40  # print at most N findings (scan is complete)
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
ALLOWLIST = REPO / "docs" / "baseline" / "citations-allowlist.tsv"

TREES = ["femacs/src", "femacs/lib", "femacs/lib-src", "femacs/platform", "femacs/exec",
         "femacs/lwlib", "femacs/oldXMenu", "femacs/nt", "femacs/msdos"]
SUFFIXES = {".rs", ".swift", ".kt", ".cc", ".cpp"}

CITE = re.compile(r"(?P<path>[A-Za-z0-9_./+-]+\.(?:in\.h|h|m|mm|cc|c))\s+"
                  r"(?P<start>\d+)(?:\s*-\s*(?P<end>\d+))?")
QUOTE = re.compile(r"`([^`]+)`")
# A snippet the gate can look for verbatim: words, underscores and stars only (so
# "struct font_metrics" and "FONT_SET_STYLE" qualify, while "FD_SET(n, p)" - whose C
# text the reader has to find by eye - does not).
CODE_QUOTE = re.compile(r"^[A-Za-z_*][A-Za-z0-9_ *]*$")
# ...but a *statement keyword* is never the thing a citation names, so pairing one with a citation
# is noise (the `for` of a FOR_EACH_TAIL row, the `int` of a UTF_16_HIGH_SURROGATE_P row).
KEYWORDS = {"for", "if", "while", "else", "return", "switch", "case", "break", "continue",
            "goto", "default", "do", "int", "char", "void", "unsigned", "signed", "short", "long",
            "struct", "union", "enum", "typedef", "static", "extern", "const", "volatile",
            "sizeof", "inline", "register", "auto",
            # Not C keywords, but they are what a sentence's prose uses and the picker used to
            # pair a citation with them (`pdumper.rs`'s `and`, `include/cm.rs`'s `let`).
            "and", "let"}


ROOTS = [REPO / "emacs", REPO / "emacs" / "src", REPO / "emacs" / "lib",
         REPO / "emacs" / "lib-src", REPO / "build" / "emacs-ref" / "src",
         REPO / "build" / "emacs-ref" / "lib", REPO / "build" / "emacs-ref"]


# C's line numbering is `split ("\n")`: Emacs's sources are full of `^L` (form feed, \x0c) page
# breaks, and Python's `str.splitlines ()` treats one as a line break too - which shifted every
# citation's line number by the number of form feeds above it.  The first version of this tool
# used splitlines (), and 124 of its 162 `drift` findings were that bug (found by the lane that
# used the tool; status 3.172).
def c_lines_text (text: str) -> list[str]:
    lines = text.split ("\n")
    if lines and lines[-1] == "":
        lines.pop ()
    return lines


def resolve(path: str) -> Path | None:
    """The reference file a citation names, or None when it is external to this tree.

    The roots cover the pinned checkout *and* the frozen build's generated headers
    (`build/emacs-ref/...`), because the port cites both (`config.h` is generated).  A path
    that matches neither is external: `jni.h`, `<sys/fileio.h>`, AppKit.  Those are counted
    and listed, never failed - the tree cannot be asked about a header it does not carry.
    """
    base = Path(path)
    for root in ROOTS:
        cand = root / base
        if cand.is_file():
            return cand
    for root in ROOTS:
        cand = root / base.name
        if cand.is_file():
            return cand
    return None


def findings_for(rel: str, text: str, limit: int | None = None) -> list[tuple[str, str]]:
    out: list[tuple[str, str]] = []
    cache: dict[Path, list[str]] = {}
    for lineno, line in enumerate(c_lines_text(text), 1):
        for m in CITE.finditer(line):
            ref = resolve(m.group("path"))
            if ref is None:
                out.append(f"external\t{rel}:{lineno}\t{m.group(0)}")
                continue
            if ref not in cache:
                cache[ref] = c_lines_text(ref.read_text(encoding="utf-8", errors="replace"))
            lines = cache[ref]
            start = int(m.group("start"))
            end = int(m.group("end") or start)
            if start < 1 or end > len(lines) or start > end:
                out.append(f"range\t{rel}:{lineno}\t{m.group(0)}\t"
                           f"the reference has {len(lines)} line(s)")
                continue
            quote = None
            for q in QUOTE.findall(line[m.end():]):
                if CODE_QUOTE.match(q.strip()) and q.strip() not in KEYWORDS:
                    quote = q.strip().rstrip("(").strip()
                    break
            if not quote:
                continue
            window = "\n".join(lines[max(0, start - 3): end + 2])
            if quote in window:
                continue
            actual = [i for i, l in enumerate(lines, 1) if quote in l]
            if actual:
                out.append(f"drift\t{rel}:{lineno}\t{m.group(0)}\t"
                           f"`{quote}` is at {ref.name}:{actual[0]}")
    return out[:limit] if limit else out


def fix_file(rel: str, text: str) -> tuple[str, list[str]]:
    """Rewrite the citations whose quoted C text lives somewhere else in the same file.

    Only the *unambiguous* cases are touched: the quote must occur exactly once in the
    reference file, so there is one place the citation can mean.  The span the author wrote
    is kept (`1198-1201` becomes `<actual>-<actual+3>`), the rest of the line is untouched,
    and every change is printed for review.  A quote with several occurrences is left alone
    and stays a finding - a wrong "fix" would be worse than a visibly stale line.
    """
    out_lines = text.split("\n")
    changes: list[str] = []
    for idx, line in enumerate(out_lines):
        for m in CITE.finditer(line):
            ref = resolve(m.group("path"))
            if ref is None:
                continue
            lines = ref.read_text(encoding="utf-8", errors="replace").splitlines()
            quote = None
            for q in QUOTE.findall(line[m.end():]):
                if CODE_QUOTE.match(q.strip()):
                    quote = q.strip().rstrip("(").strip()
                    break
            if not quote:
                continue
            actual = [i for i, l in enumerate(lines, 1) if quote in l]
            if len(actual) != 1:
                continue
            start = int(m.group("start"))
            end = int(m.group("end") or start)
            if start <= actual[0] <= end:
                continue
            span = end - start
            new_range = f"{actual[0]}" if span == 0 else f"{actual[0]}-{actual[0] + span}"
            old = m.group(0)
            new = m.group("path") + " " + new_range
            out_lines[idx] = out_lines[idx].replace(old, new, 1)
            changes.append(f"{rel}:{idx + 1}\t{old}\t{new}")
    return "\n".join(out_lines), changes


def read_allowlist () -> dict[str, str]:
    """`<port file>:<line>` -> reason, from docs/baseline/citations-allowlist.tsv.

    A citation can be *right* and still pair with a snippet the gate had to guess at (the
    include-line class: the quoted identifier is an example, not the cited text; or the range is
    right and the quote is the member's type).  Those rows carry a reason, and a row that no
    longer matches a finding is a failure - the same rule the other manifests use.
    """
    rows: dict[str, str] = {}
    if not ALLOWLIST.is_file ():
        return rows
    for line in ALLOWLIST.read_text (encoding="utf-8").splitlines ():
        if not line.strip () or line.startswith ("#"):
            continue
        parts = line.split ("\t")
        if len (parts) >= 2:
            rows[parts[0]] = parts[1]
    return rows


def selftest() -> int:
    ref = REPO / "emacs" / "src" / "font.h"
    cases = [
        ("a stale line is reported",
         '/// C: font.h 351-356: `struct font_metrics`\n', "drift"),
        ("the right line is silent",
         '/// C: font.h 414-417: `struct font_metrics`\n', None),
        ("a paraphrase is silent",
         '/// C: font.h 414-417: `the metrics record`\n', None),
        ("an impossible range is reported",
         '/// C: font.h 99999-100000: `x`\n', "range"),
        ("an unknown file is external, not a failure",
         '/// C: nope.h 1-2: `x`\n', "external"),
    ]
    bad = 0
    for name, src, want in cases:
        got = findings_for("selftest", src)
        klass = got[0].split("\t")[0] if got else None
        ok = klass == want
        print(f"{'ok  ' if ok else 'FAIL'} {name}: {klass}")
        bad += 0 if ok else 1
    print(f"{len(cases)} cases, {bad} failures")
    return 1 if bad else 0


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--selftest", action="store_true")
    ap.add_argument("--limit", type=int, default=0)
    ap.add_argument("--fix", action="store_true",
                    help="rewrite the unambiguous stale citations (tracked files only)")
    args = ap.parse_args()
    if args.selftest:
        return selftest()
    if args.fix:
        print("--fix was removed in status 3.166.  The 63 rewrites it made were reverted: a "
              "citation is a sentence, not a field, and one of them corrupted an "
              "`#include`-line citation whose quoted identifier was incidental.  Use this "
              "report as a review queue.")
        return 2
    files, found = 0, []
    for tree in TREES:
        for path in sorted((REPO / tree).rglob("*")):
            if not path.is_file() or path.suffix not in SUFFIXES:
                continue
            files += 1
            rel = str(path.relative_to(REPO))
            found += findings_for(rel, path.read_text(encoding="utf-8", errors="replace"))
    allow = read_allowlist ()
    def site (entry: str) -> str:
        return entry.split ("\t")[1]
    drift = [f for f in found if f.startswith ("drift") and site (f) not in allow]
    rng = [f for f in found if f.startswith ("range") and site (f) not in allow]
    ext = [f for f in found if f.startswith ("external")]
    allowed = [f for f in found if f.startswith (("drift", "range")) and site (f) in allow]
    stale = [k for k in allow if not any (site (f) == k for f in found)]
    for entry in (rng + drift + ext)[: args.limit or len (found)]:
        print (entry)
    for key in stale:
        print (f"stale\t{key}\t{allow[key]}\tthe row no longer matches a finding")
    print (f"check-citations: {files} file(s) scanned, {len(drift)} drift, {len(rng)} range, "
           f"{len(ext)} external, {len(allowed)} allowed, {len(stale)} stale")
    return 1 if drift or rng or stale else 0


if __name__ == "__main__":
    sys.exit(main())
