#!/usr/bin/env python3
"""The work order for the window-system arms, should decision C be taken.

`android-arms.tsv` says which names the Android library imports and why nothing provides them;
40 of them sit behind the window-system family.  Taking decision C means writing those arms under
`#[cfg (feature = "have_window_system")]` rather than recording them as dead, so this tool turns
each name into a row a worker can act on: the C definition, the enclosing `#ifdef` region (start and
end lines), the port unit that will hold the arm, and whether the port already has a stub or a note
at that name.

It changes no code and decides nothing: it is the queue, prepared so that the decision does not also
have to pay for planning (docs/handoff.md).

    python3 tools/gen-window-arms-plan.py            # write docs/baseline/window-arms-plan.tsv
    python3 tools/gen-window-arms-plan.py --out -    # print
"""

from __future__ import annotations

import argparse
import pathlib
import re
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
ARMS = REPO / "docs/baseline/android-arms.tsv"
OUT = REPO / "docs/baseline/window-arms-plan.tsv"
WINDOW = ("WINDOW_SYSTEM", "HAVE_X_WINDOWS", "HAVE_NS", "HAVE_PGTK", "HAVE_HAIKU", "HAVE_NTGUI",
          "WINDOWSNT", "MSDOS")


# Indented directives count: reading only the left-margin ones ended the region at the first
# `#endif` of an *inner* block in files that indent their `# if`s (status 4.749, the same blind
# spot `tools/gen-android-arms.py` had).
DIRECTIVE = re.compile(r"^\s*#\s*(if|ifdef|ifndef|elif|else|endif)\b")


def region_end(lines: list[str], start: int) -> int:
    depth = 0
    for i in range(start, len(lines)):
        kind = DIRECTIVE.match(lines[i])
        if kind and kind.group(1) in ("if", "ifdef", "ifndef"):
            depth += 1
        elif kind and kind.group(1) == "endif":
            depth -= 1
            if depth == 0:
                return i + 1
    return len(lines)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--out", default=str(OUT))
    a = ap.parse_args()

    rows = []
    cache: dict[str, list[str]] = {}
    for line in ARMS.read_text(encoding="utf-8").splitlines():
        if line.startswith("#") or not line.strip():
            continue
        f = line.split("\t")
        if len(f) < 4:
            continue
        name, cls, where, conds = f[0], f[1], f[2], f[3]
        # `arm` rows carry the C guard; the Lisp globals and `x_*` accessors of the window front
        # ends classify as `system` but belong to the same decision (docs/android-readiness.py
        # counts them the same way).
        is_window = (cls == "arm" and any(w in conds for w in WINDOW)) or name.startswith(("Vx_", "x_")) \
            or name == "tty_draw_row_with_mouse_face"
        if not is_window:
            continue
        c_file, _, c_line = where.partition(":")
        if not c_file or not c_line.isdigit():
            # The Lisp globals and `x_*` accessors classify as `system` with no definition site;
            # their C origin is a `DEFVAR_*` (or a macro) somewhere in the window front ends.
            found = ""
            for src in sorted((REPO / "emacs/src").glob("*.c")):
                text = src.read_text(errors="replace")
                m = re.search(rf"DEFVAR_\w+ \([\s\S]{{0,200}}?\b{re.escape(name)}\b", text)
                if m:
                    found = f"{src.relative_to(REPO)}:{text[:m.start()].count(chr(10)) + 1}"
                    break
            rows.append((name, found or "(no DEFVAR found)", "", "", "", "global to define"))
            continue
        lines = cache.setdefault(c_file, (REPO / c_file).read_text(errors="replace").splitlines())
        # The enclosing `#if` for the definition line.
        depth, start = 0, None
        for i in range(int(c_line) - 1, -1, -1):
            if lines[i].startswith("#endif"):
                depth += 1
            elif lines[i].startswith("#if"):
                if depth == 0:
                    start = i
                    break
                depth -= 1
        span = f"{start + 1}-{region_end(lines, start)}" if start is not None else ""
        port = c_file.replace("emacs/src/", "src/").replace("emacs/lib/", "lib/")
        port = re.sub(r"\.c$", ".rs", port)
        has_port = (REPO / port).is_file()
        rows.append((name, where, span, port if has_port else "", conds,
                     "arm to write" if start is not None else "no enclosing #if found"))

    header = ("# The window-system arms as a work order, for decision C (tools/gen-window-arms-plan.py).\n"
              "# Each row: the name the Android library needs, its C definition, the enclosing\n"
              "# `#if` region (start-end in that C file), the port unit that will hold the arm, the\n"
              "# preprocessor conditions, and the action.  Nothing here changes code.\n"
              "#\n"
              "# name\tC definition\tC region\tport unit\tconditions\taction\n")
    text = header + "\n".join("\t".join(r) for r in rows) + "\n"
    if a.out == "-":
        sys.stdout.write(text)
    else:
        pathlib.Path(a.out).write_text(text, encoding="utf-8")
        print(f"wrote {pathlib.Path(a.out).name}: {len(rows)} row(s)")
    by_file: dict[str, int] = {}
    for r in rows:
        key = r[1].split(":")[0]
        by_file[key] = by_file.get(key, 0) + 1
    print("by C file: " + ", ".join(f"{k} {v}" for k, v in sorted(by_file.items(), key=lambda kv: -kv[1])[:6]))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
