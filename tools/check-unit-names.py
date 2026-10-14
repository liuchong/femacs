#!/usr/bin/env python3
"""Triage reading: capitalized names a unit's code uses that no item in `femacs/` defines.

Why this exists (docs/ports/w32fns.md, slice 8's Evidence): the `have_ntgui` arm probe is not a
name-resolution reading for `DEFUN!` bodies.  Measured 2026-10-14: a deliberately injected
`let _ = SM_CXVSCROLL_ZZZ_PROBE;` inside `Fx_create_frame`'s body is **absent** from the probe's
error list, while member errors (`E0609`) and initialization errors (`E0381`) from the very same
body are present.  Two real defects came out of that gap and are now fixed:

  * `SM_CXVSCROLL`/`SM_CXHSCROLL` and six `IDC_*` cursor ids - used since slices 5/7, defined
    nowhere (slice 8);
  * `validate_x_resource_name` - called since slice 6, declared nowhere (slice 10).

    python3 tools/check-unit-names.py femacs/src/w32fns.rs 10836 11019 [NAME ...]

The two trailing numbers are the 1-based line range to read; any further arguments are names to
skip (locals whose spelling looks capitalized, macro-generated subr statics, ...).  What it does:
strip comments and string literals, drop field accesses and macro invocation names, then require
an item (`fn`/`const`/`static`/`struct`/`type`/`enum`/`union`/`macro_rules!`) of that name anywhere
under `femacs/`.

Two known limitations, both of which produced false positives in practice and must be triaged by
hand rather than "fixed":

  * **nested block comments.**  The scanner ends a comment at the first `*/`, while Rust's block
    comments nest - so a slice that carries a C `#if 0` region as C's own text (w32fns.c
    7291-7388, 1278-1417) reports names that are inside that comment (measured: the comment depth
    at the C 1278-1417 site, `src/w32fns.rs` 2260, is 2);
  * **macro-generated names** (`DEFUN!`'s per-subr `Sname` statics) and C globals the port reaches
    through a mirrored record (the `KVAR` arm's `Vdefault_minibuffer_frame`) are not items;
  * **names that are not item references at all**: `#[derive (Clone, Copy)]`'s arguments, a struct
    definition's *field* names (`pub Flags: DWORD,` - only `.field` accesses are stripped), and the
    parameter names of a declaration (C's own `Msg: UINT` in a `SendMessageW` prototype).

Exit status is 0 either way: this is a reading for a lane's Evidence, not one of the ten gates.
"""
from __future__ import annotations

import pathlib
import re
import sys


def strip(text: str) -> str:
    """Drop `//` and `/* */` comments and string literals (a naive scanner; see the docstring)."""
    out: list[str] = []
    i, n = 0, len(text)
    while i < n:
        if text.startswith("//", i):
            j = text.find("\n", i)
            i = n if j < 0 else j
        elif text.startswith("/*", i):
            j = text.find("*/", i + 2)
            i = n if j < 0 else j + 2
        elif text[i] == '"':
            j = i + 1
            while j < n:
                if text[j] == "\\":
                    j += 2
                    continue
                if text[j] == '"':
                    break
                j += 1
            i = j + 1
        else:
            out.append(text[i])
            i += 1
    return "".join(out)


def item_pattern(name: str) -> re.Pattern[str]:
    return re.compile(r"\b(?:pub\s+)?(?:unsafe\s+)?(?:extern\s+\"[^\"]*\"\s+)?"
                      r"(?:fn|const|static(?:\s+mut)?|struct|type|enum|union|macro_rules!)\s+"
                      + re.escape(name) + r"\b")


def main() -> int:
    if len(sys.argv) < 4:
        print(__doc__.strip(), file=sys.stderr)
        return 2
    path, start, end = sys.argv[1], int(sys.argv[2]) - 1, int(sys.argv[3])
    skip = set(sys.argv[4:])

    lines = pathlib.Path(path).read_text(encoding="utf-8").splitlines()[start:end]
    code = strip("\n".join(lines))
    code = re.sub(r"\.\s*[A-Za-z_][A-Za-z0-9_]*", " . ", code)
    code = re.sub(r"\b[A-Za-z_][A-Za-z0-9_]*\s*!", " ", code)
    names = sorted({m for m in re.findall(r"\b[A-Z][A-Za-z0-9_]*\b", code)})

    corpus = [p.read_text(encoding="utf-8", errors="ignore")
              for p in pathlib.Path("femacs").rglob("*.rs")]
    missing = [n for n in names
               if n not in skip and not any(item_pattern(n).search(c) for c in corpus)]
    print(f"region {path}:{start + 1}-{end}: {len(names)} capitalized identifiers, "
          f"missing: {missing}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
