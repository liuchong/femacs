#!/usr/bin/env python3
"""Turn the Android import ratchet into a per-file work queue.

`tools/check-android-imports.py` answers "which names does the Android `libemacs.so` import
that nothing in the process provides" - 214 of them, listed one per line in
`docs/baseline/android-imports-pending.txt`.  That list is a set of *names*, and the work to
close it is not per name but per C region: the port transcribes C in the configuration it
freezes, and a name is missing here because the region that defines it is an arm the frozen
(darwin) configuration does not compile - the `HAVE_WINDOW_SYSTEM` arms the port records as
"dead on this target (P-08)", for example - while the authorised Android configuration
(`HAVE_ANDROID`, `HAVE_WINDOW_SYSTEM`) does.

So this tool locates every unresolved name's definition in the reference tree, computes the
preprocessor conditions it sits under, and groups the result by C file.  The output is the
queue, not a verdict: `docs/status.md` 4.707 records the class counts, and the reading it
points at (`check-android-imports.py`'s unresolved count) is what has to reach zero.

    python3 tools/gen-android-arms.py                    # write docs/baseline/android-arms.tsv
    python3 tools/gen-android-arms.py --out -            # print it
    python3 tools/gen-android-arms.py --class window_system   # one class only
"""

from __future__ import annotations

import argparse
import collections
import pathlib
import re
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
PENDING = REPO / "docs/baseline/android-imports-pending.txt"
OUT = REPO / "docs/baseline/android-arms.tsv"

# The conditions that make a region a *platform* arm rather than ordinary C: a definition under
# one of these is compiled by some configuration the repository can name, and skipped by the
# frozen one.
PLATFORM = ("HAVE_WINDOW_SYSTEM", "HAVE_X_WINDOWS", "HAVE_NS", "HAVE_ANDROID", "HAVE_HAIKU",
            "HAVE_PGTK", "HAVE_NTGUI", "HAVE_W32", "HAVE_ANDROID_STUBIFY")

SOURCES = sorted(list((REPO / "emacs/src").glob("*.c"))
                 + list((REPO / "emacs/lib").rglob("*.c"))
                 + list((REPO / "emacs/lib-src").glob("*.c")))


def conditions(lines: list[str], index: int) -> list[str]:
    """The `#if*` conditions enclosing line `index`, outermost first."""
    stack: list[list[str]] = []
    for line in lines[: index + 1]:
        if line.startswith("#if"):
            stack.append([line.rstrip()])
        elif line.startswith("#endif"):
            if stack:
                stack.pop()
        elif line.startswith(("#else", "#elif")) and stack:
            stack[-1] = [line.rstrip()]  # a sibling arm; the enclosing block is what matters
    return [c for group in stack for c in group]


def definition_lines(lines: list[str], name: str) -> list[int]:
    """Line numbers where NAME is defined (a column-0 function or object)."""
    fn = re.compile(rf"^{re.escape(name)}\s*\(")
    obj = re.compile(rf"^{re.escape(name)}\s*(?:\[[^\]]*\])?\s*(?:=|;)")
    out = []
    for i, line in enumerate(lines):
        if line.startswith(("#", " ", "\t", "}")):
            continue
        if fn.match(line) or obj.match(line):
            out.append(i)
    return out


def port_file(c_file: pathlib.Path) -> str:
    rel = c_file.relative_to(REPO / "emacs")
    stem = rel.with_suffix(".rs")
    for candidate in (REPO / "femacs" / stem, REPO / "femacs/src" / rel.name.replace(".c", ".rs")):
        if candidate.is_file():
            return str(candidate.relative_to(REPO))
    return ""


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--pending", type=pathlib.Path, default=PENDING)
    ap.add_argument("--out", default=str(OUT))
    ap.add_argument("--class", dest="cls", default=None,
                    help="only rows of this class (window_system | other | platform_name)")
    a = ap.parse_args()

    names = [l.strip() for l in a.pending.read_text(encoding="utf-8").splitlines()
             if l.strip() and not l.startswith("#")]
    if not names:
        print("gen-android-arms: the ratchet is empty: nothing is missing")
        return 0

    rows: list[tuple[str, str, int, str, str, str]] = []
    cache: dict[pathlib.Path, list[str]] = {}
    for name in names:
        if name.startswith(("_R", "__")):
            # A Rust symbol or a platform interface name: no C region defines it, so it is its
            # own row rather than an arm (the panic entry point, bionic's `__errno`).
            rows.append((name, "", 0, "", "platform_name", ""))
            continue
        found = False
        for path in SOURCES:
            lines = cache.setdefault(path, path.read_text(errors="replace").splitlines())
            for index in definition_lines(lines, name)[:3]:
                conds = conditions(lines, index)
                cls = "window_system" if any(p in c for c in conds for p in PLATFORM) else "other"
                guard = " && ".join(c.strip() for c in conds)
                rows.append((name, str(path.relative_to(REPO)), index + 1, guard, cls,
                             port_file(path)))
                found = True
            if found:
                break
        if not found:
            rows.append((name, "", 0, "", "not_found", ""))

    if a.cls:
        rows = [r for r in rows if r[4] == a.cls]

    header = ("# The Android import ratchet as a per-file work queue (tools/gen-android-arms.py).\n"
              "# The reading this queue exists to drive to zero is\n"
              "# `python3 tools/check-android-imports.py` (docs/status.md 4.707).\n"
              "#\n"
              "# name\tC file\tline\tpreprocessor conditions\tclass\tport unit\n")
    text = header + "\n".join("\t".join(str(f) for f in row) for row in rows) + "\n"
    if a.out == "-":
        sys.stdout.write(text)
    else:
        pathlib.Path(a.out).write_text(text, encoding="utf-8")
        print(f"wrote {pathlib.Path(a.out).name}: {len(rows)} row(s)")

    classes = collections.Counter(r[4] for r in rows)
    files = collections.Counter(r[1] for r in rows if r[1])
    print("classes: " + ", ".join(f"{k} {v}" for k, v in classes.most_common()))
    print("top C files: " + ", ".join(f"{k} {v}" for k, v in files.most_common(8)))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
