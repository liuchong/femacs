#!/usr/bin/env python3
"""Fail when a real C `do { ... } while (COND);` was transcribed without negating COND.

C's `do`/`while` loops while COND holds; the port's shape is `loop { BODY; if !COND { break; } }`
(PORTING.md P-95).  status 3.635 found one transcription that broke on COND itself - the fast scan
in `scan_lists`' whitespace skip left the loop after the first character whenever syntax properties
are off, and the unguarded `update_syntax_table_forward` that follows asked about a charpos whose
interval cache was already NULL.  A mechanical transcription keeps C's tokens, so an un-negated
`break` is recognisable: its condition is textually *equal* to the C condition, while a negated one
is that condition inside `! (...)`.  This gate compares the two after stripping whitespace,
`unsafe`, braces, semicolons and call parentheses.

    python3 tools/check-do-while.py            # every ported unit that has a C file
    python3 tools/check-do-while.py --selftest # prove the reading fires (the 3.635 case)

Sound but not complete: a *rewritten* condition (`*++p != '\\0'` carried as `p.add (1); if *p == 0`)
does not match textually, so it is counted as unchecked rather than as a failure.  The coverage line
prints how many C conditions matched a port condition at all.
"""

from __future__ import annotations

import argparse
import re
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
REFERENCE = REPO / "emacs"
# Reviewed candidates.  A candidate that is not an un-negated trampoline still *looks* like one
# (the same condition text belongs to a different loop in the same unit), so it is listed here with
# the reason; a candidate without a row, and a row without a candidate, are both failures.
# Columns: <port path>\t<line>\t<C path>\t<C line>\t<reason>.
ALLOWLIST = REPO / "docs/baseline/do-while-allowlist.tsv"


def norm(text: str) -> str:
    text = re.sub(r"\bunsafe\b", " ", text)
    text = re.sub(r"[\s{}();]", "", text)
    text = text.replace("_", "")
    return text


def c_do_while(text: str) -> list[tuple[int, str]]:
    """`(line, normalized condition)` for every `do { ... } while (COND);` in C source."""
    out = []
    for m in re.finditer(r"\bdo\s*\{", text):
        i, depth = m.end(), 1
        while i < len(text) and depth:
            if text[i] == "{":
                depth += 1
            elif text[i] == "}":
                depth -= 1
            i += 1
        tail = text[i:i + 400]
        w = re.match(r"\s*while\s*\((.*?)\)\s*;", tail, re.S)
        if w:
            out.append((text.count("\n", 0, m.start()) + 1, norm(w.group(1))))
    return out


def strip_rust(text: str) -> str:
    """Drop comments and string/char literals so prose cannot look like an `if` statement."""
    # keep the newline count of every comment, so the line numbers stay exact
    text = re.sub(r"/\*.*?\*/", lambda m: "\n" * m.group(0).count("\n"), text, flags=re.S)
    text = re.sub(r"//[^\n]*", "", text)
    text = re.sub(r'"([^"\\\n]|\\.)*"', '""', text)
    text = re.sub(r"r#\".*?\"#", '""', text, flags=re.S)
    text = re.sub(r"'(\\.|[^'\\\n])'", "''", text)
    return text


def rust_break_conditions(text: str) -> list[tuple[int, str]]:
    """`(line, normalized condition)` for every `if COND { break; }` in ported Rust.

    Brace-aware: a condition may itself hold blocks (`unsafe { x }`), so the block that counts is
    the first one whose balanced body is exactly `break;`.
    """
    text = strip_rust(text)
    out = []
    for m in re.finditer(r"\bif\b", text):
        start, i = m.end(), m.end()
        while i < len(text) and i - start < 800:
            ch = text[i]
            if ch == "{":
                j, depth = i + 1, 1
                while j < len(text) and depth:
                    if text[j] == "{":
                        depth += 1
                    elif text[j] == "}":
                        depth -= 1
                    j += 1
                if text[i + 1:j - 1].strip() in ("break;", "break"):
                    out.append((text.count("\n", 0, m.start()) + 1, norm(text[start:i])))
                    break
                i = j
                continue
            if ch in ";{}":
                break
            i += 1
    return out


def read_allowlist() -> dict[tuple[str, int], tuple[str, int, str]]:
    rows: dict[tuple[str, int], tuple[str, int, str]] = {}
    if not ALLOWLIST.is_file():
        return rows
    for raw in ALLOWLIST.read_text(encoding="utf-8").splitlines():
        if not raw.strip() or raw.startswith("#") or raw.startswith("port path\t"):
            continue
        parts = raw.split("\t")
        if len(parts) < 5 or not parts[1].strip().isdigit():
            print(f"BAD ROW {ALLOWLIST.name}: {raw!r}")
            continue
        rows[(parts[0], int(parts[1]))] = (parts[2], int(parts[3]) if parts[3].strip().isdigit() else 0, parts[4])
    return rows


def check_unit(c: Path, r: Path) -> tuple[int, int, list[str]]:
    ctext, rtext = c.read_text(encoding="utf-8", errors="replace"), r.read_text(encoding="utf-8", errors="replace")
    conds = c_do_while(ctext)
    breaks = rust_break_conditions(rtext)
    checked, bad = 0, []
    for cline, cond in conds:
        if not cond or cond in ("0", "false"):
            continue
        for rline, bcond in breaks:
            if bcond == cond:
                checked += 1
                bad.append((str(r.relative_to(REPO)), rline,
                            f"emacs/{c.relative_to(REFERENCE)}", cline))
            elif bcond == "!" + cond or bcond == "!(" + cond + ")":
                checked += 1
    return len(conds), checked, bad


SELFTEST = [
    ("do { x++; } while (a || b);", "let _ = x;\nif a || b { break; }\n", 1),
    ("do { x++; } while (a || b);", "if !(a || b) { break; }\n", 0),
    ("do { x++; } while (!p || q < e);", "if !(!p || q < unsafe { e }) { break; }\n", 0),
    ("do { x++; } while (0);", "if 0 { break; }\n", 0),   # the `while (0)` idiom is skipped
    ("do { x++; } while (0);", "if !(0) { break; }\n", 0),
]


def selftest() -> int:
    bad = 0
    for ctext, rtext, want in SELFTEST:
        conds = c_do_while(ctext)
        breaks = rust_break_conditions(rtext)
        hits = sum(1 for _c, cond in conds
                   if cond not in ("0", "false") and any(b == cond for _r, b in breaks))
        if hits != want:
            print(f"FAIL  {ctext!r} / {rtext!r}: {hits} hit(s), want {want}")
            bad += 1
    print(f"selftest: {len(SELFTEST)} case(s), {bad} failure(s)")
    return 1 if bad else 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--selftest", action="store_true")
    ap.add_argument("units", nargs="*", help="stem of a src/<unit>.c pair (default: all)")
    args = ap.parse_args()
    if args.selftest:
        return selftest()
    if args.units:
        pairs = [(REFERENCE / "src" / (u + ".c"), REPO / "src" / (u + ".rs")) for u in args.units]
    else:
        pairs = [(c, REPO / "src" / (c.stem + ".rs"))
                 for c in sorted((REFERENCE / "src").glob("*.c"))
                 if (REPO / "src" / (c.stem + ".rs")).is_file()]
    allow = read_allowlist()
    used: set[tuple[str, int]] = set()
    total_c = total_checked = 0
    failures: list[str] = []
    allowed = 0
    for c, r in pairs:
        n, checked, bad = check_unit(c, r)
        total_c += n
        total_checked += checked
        for path, line, cpath, cline in bad:
            row = allow.get((path, line))
            if row and row[0] == cpath:
                used.add((path, line))
                allowed += 1
            else:
                failures.append(f"{path}:{line}: breaks on C's own condition ({cpath}:{cline}) - "
                                f"it must be negated (P-95), or be a reviewed row of "
                                f"{ALLOWLIST.name}")
    stale = sorted(set(allow) - used)
    for path, line in stale:
        failures.append(f"{ALLOWLIST.name}: review row for {path}:{line} matches no candidate - "
                        f"the row is stale")
    for f in failures:
        print(f)
    print(f"read {len(pairs)} unit(s): {total_c} C do/while condition(s), {total_checked} matched by a "
          f"port `if ... {{ break; }}`, {allowed} reviewed, {len(failures)} failure(s)")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
