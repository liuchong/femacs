#!/usr/bin/env python3
"""List the C regions a *second* configuration compiles that the frozen one does not.

`docs/baseline/android-arms.tsv` says which names the Android library imports and nothing
provides.  This tool says where the C text for them lives: every guarded region in the sources the
Android configuration compiles, with its condition, its line range, and the C items inside it.
That is the unit of work - `term.c`'s 22 `#ifndef HAVE_ANDROID` regions are excluded by the
Android build entirely (their callees live in `CM_OBJ`, which `src/Makefile.in` 344-345 leaves
empty for Android), while its 2 `#ifdef HAVE_ANDROID` regions are the text C compiles instead and
the port still records as dead (status 4.710).

The rows are measurements, not judgements: a region's port text may already be right (many
`HAVE_WINDOW_SYSTEM` regions are written under their `cfg`), the tool only says how much C text
each condition covers and which of the frozen build's own conditions select it.

    python3 tools/gen-platform-arms.py                    # write docs/baseline/platform-arms.tsv
    python3 tools/gen-platform-arms.py --condition HAVE_ANDROID
"""

from __future__ import annotations

import argparse
import collections
import pathlib
import re
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
OUT = REPO / "docs/baseline/platform-arms.tsv"
# The conditions worth counting: the ones a second configuration of this repository can name.
WANTED = ("HAVE_ANDROID", "HAVE_WINDOW_SYSTEM", "HAVE_X_WINDOWS", "HAVE_NS", "HAVE_PGTK",
          "HAVE_HAIKU", "HAVE_NTGUI", "WINDOWSNT", "MSDOS", "HAVE_MODULES", "ANDROID_STUBIFY")

ITEM = re.compile(r"^(?:[A-Za-z_][\w \t*]*[\s*])?"
                  r"([A-Za-z_][A-Za-z_0-9]*)\s*(\[[^\]]*\])?\s*[=(]")


def regions(path: pathlib.Path) -> list[tuple[str, int, int, list[str]]]:
    """(condition, first_line, last_line, items) for each guarded region in PATH."""
    lines = path.read_text(errors="replace").splitlines()
    out: list[tuple[str, int, int, list[str]]] = []
    for start, line in enumerate(lines):
        if not line.startswith("#if") or not any(w in line for w in WANTED):
            continue
        depth = 0
        for end in range(start, len(lines)):
            if lines[end].startswith("#if"):
                depth += 1
            elif lines[end].startswith("#endif"):
                depth -= 1
                if depth == 0:
                    break
        else:
            continue
        items = []
        for body in lines[start + 1:end]:
            m = ITEM.match(body)
            if m:
                items.append(m.group(1))
        out.append((" ".join(line.split()), start + 1, end + 1, items))
    return out


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--out", default=str(OUT))
    ap.add_argument("--condition", default=None, help="only regions whose condition names this")
    a = ap.parse_args()

    sources = []
    for pattern in ("src/*.c", "lib/*.c", "lib-src/*.c"):
        sources += sorted((REPO / "emacs").glob(pattern))
    rows = []
    for path in sources:
        for condition, start, end, items in regions(path):
            if a.condition and a.condition not in condition:
                continue
            rows.append((str(path.relative_to(REPO)), condition, start, end, end - start,
                         len(items), " ".join(items[:6])))
    rows.sort(key=lambda r: (-r[4], r[0]))

    header = ("# C regions a second configuration compiles (tools/gen-platform-arms.py):\n"
              "# the conditions the Android/other targets select, where the frozen configuration\n"
              "# does not, with the C items each region carries.  The work queue behind\n"
              "# docs/baseline/android-arms.tsv (docs/status.md 4.710, PORTING.md P-302/P-303).\n"
              "#\n"
              "# C file\tcondition\tlines\t\tspan\titems\tfirst items\n")
    text = header + "\n".join(
        f"{r[0]}\t{r[1]}\t{r[2]}-{r[3]}\t{r[4]}\t{r[5]}\t{r[6]}" for r in rows) + "\n"
    if a.out == "-":
        sys.stdout.write(text)
    else:
        pathlib.Path(a.out).write_text(text, encoding="utf-8")
        print(f"wrote {pathlib.Path(a.out).name}: {len(rows)} region(s)")

    conditions = collections.Counter(r[1] for r in rows)
    print("conditions: " + ", ".join(f"{k} {v}" for k, v in conditions.most_common(6)))
    files = collections.Counter(r[0] for r in rows)
    print("top files: " + ", ".join(f"{k} {v}" for k, v in files.most_common(6)))
    print(f"total C lines guarded: {sum(r[4] for r in rows)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
