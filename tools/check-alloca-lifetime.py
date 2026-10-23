#!/usr/bin/env python3
"""The `alloca` lifetime screen: which `SAFE_ALLOCA` buffers are still used after `SAFE_FREE`.

`PORTING.md` P-117 ⑧ makes the port's `alloca` substitute release a request's bytes at the
scope's `SAFE_FREE` (C's own fallback lifetime, `record_xmalloc`), which is shorter than C's
`alloca` lifetime: C's bytes live until the *frame* returns, so C code may use them after
`SAFE_FREE` and still be correct.  This screen is the ledger for that claim - it walks every
unit, pairs each `SAFE_ALLOCA!` (and its `SAFE_ALLOCA_LISP!`/`SAFE_NALLOCA!`/
`SAFE_ALLOCA_STRING!` siblings) with the first `SAFE_FREE!` after it inside the same block,
and reports the sites whose variable is still referenced further down.

It is a *screen*, not a proof: a reference after the free can sit on a control-flow path that
does not reach the use (an early return frees and leaves), so the reading has to be read with
the C nearby - which is what status 4.436 did for the ten sites it reports on the frozen
checkout.

    python3 tools/check-alloca-lifetime.py             # both lists, by unit
"""

from __future__ import annotations

import pathlib
import re
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
ROOT = REPO

ITEM = re.compile(r'^(pub\s+)?(unsafe\s+)?(extern\s+"C"\s+)?fn\s+([A-Za-z0-9_]+)\s*[<(]')
LHS = re.compile(r'(?:let\s+(?:mut\s+)?([A-Za-z0-9_]+)\s*(?::[^=]*)?=\s*|([A-Za-z0-9_]+)\s*=\s*)SAFE_')
ARGS = re.compile(r'SAFE_(ALLOCA_LISP|NALLOCA|ALLOCA_STRING)!\s*\(([^)]*)\)')
FREE = re.compile(r'SAFE_FREE\w*!\s*\(([^)]*)\)')


def ident(text: str | None) -> str | None:
    names = re.findall(r'[A-Za-z_][A-Za-z0-9_]*', text) if text else []
    return names[-1] if names else None


def block_end(lines: list[str], start: int) -> int:
    depth = 0
    for i in range(start, len(lines)):
        for ch in lines[i]:
            if ch == '{':
                depth += 1
            elif ch == '}':
                depth -= 1
                if depth < 0:
                    return i
    return len(lines)


def screen(path: pathlib.Path) -> tuple[list[str], list[str]]:
    """(allocations whose block never frees, allocations used after the free)."""
    lines = path.read_text(encoding='utf-8', errors='replace').split('\n')
    bounds = [i for i, line in enumerate(lines) if ITEM.match(line)] + [len(lines)]
    no_free: list[str] = []
    after: list[str] = []
    for b in range(len(bounds) - 1):
        start, end = bounds[b], bounds[b + 1]
        name = ITEM.match(lines[start]).group(4)
        body = lines[start:end]
        allocs: list[tuple[int, str | None, str]] = []
        frees: list[int] = []
        for j, line in enumerate(body):
            if 'SAFE_FREE!' in line or 'SAFE_FREE_UNBIND_TO!' in line:
                frees.append(j)
                continue
            if 'macro_rules!' in line:
                continue
            if 'SAFE_ALLOCA!' in line:
                m = LHS.search(line)
                if m:
                    allocs.append((j, m.group(1) or m.group(2), line.strip()))
            else:
                for m in ARGS.finditer(line):
                    args = m.group(2).split(',')
                    var = args[1].strip().lstrip('$') if len(args) >= 2 else ''
                    if re.match(r'^[A-Za-z_][A-Za-z0-9_]*$', var):
                        allocs.append((j, var, line.strip()))
        for j, var, text in allocs:
            bend = block_end(body, j)
            later = [f for f in frees if j < f <= bend]
            where = f"{path.relative_to(REPO)}:{start + j + 1} [{name}] {text[:78]}"
            if not later:
                no_free.append(where)
                continue
            if not var:
                continue
            pattern = re.compile(r'\b' + re.escape(var) + r'\b')
            uses = [start + k + 1 for k in range(later[0] + 1, min(bend + 1, len(body)))
                    if pattern.search(body[k])]
            if uses:
                after.append(f"{where}\n      ... still used at {uses[:5]}")
    return no_free, after


def main() -> int:
    no_free: list[str] = []
    after: list[str] = []
    for path in sorted((p for tree in ('src','lib','lib-src','lwlib','oldXMenu','nt','msdos','exec','platform','xtask') for p in (ROOT/tree).rglob("*.rs"))):
        a, b = screen(path)
        no_free += a
        after += b
    print(f"allocations whose enclosing block has no later SAFE_FREE: {len(no_free)}")
    for row in no_free:
        print(f"  {row}")
    print()
    print(f"allocations whose variable is still referenced after SAFE_FREE: {len(after)}")
    for row in after:
        print(f"  {row}")
    # A reading, not a verdict: the sites above have to be read with the C (status 4.436).
    return 0


if __name__ == "__main__":
    sys.exit(main())
