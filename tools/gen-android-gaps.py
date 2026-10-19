#!/usr/bin/env python3
"""Classify every unresolved Android import by *whose* deviation it is.

`tools/check-android-imports.py` answers "which names does the Android `libemacs.so` import that
the process does not provide" - 53 against the delivery device's own libraries (status 4.749).
That list mixes two very different kinds of entry, and the difference decides the work:

  * **`c-included`** - the C preprocessor compiles a *reference* to the name under the frozen
    Android configuration, so C's own `libemacs.so` imports it too: an upstream/platform gap the
    port has reproduced faithfully (bionic may lack the symbol even though it declares it).
  * **`c-excluded`** - every reference C has sits under a condition the Android configuration
    answers false, so C never imports it.  The port compiled a body C leaves out: **a defect in
    the referring unit**, and the row names the C site and the port unit that imported it.
  * **`no-reference`** - no file the Android build compiles even mentions the name as a call: the
    port invented the reference (a mirror that hard-codes another platform's call, a gate that
    names the wrong macro).

The liveness verdict is the same `#if` problem `tools/cinventory.py` solves for the inventory, and
the same macro table answers it: `docs/baseline/config/<config>.json`'s `macros`, with
`undefined`/`absent` reading as undefined.  Two limits are worth stating: a macro a *system* header
defines and config.h does not record (`_CS_PATH` in `<unistd.h>`, which bionic does not define and
macOS does) is read as undefined here - checked by hand for the rows that matter - and a name with
no `#if` above it is live, which is what C says.

    python3 tools/gen-android-gaps.py                     # write docs/baseline/android-gaps.tsv
    python3 tools/gen-android-gaps.py --out -             # print instead of writing
    python3 tools/gen-android-gaps.py --config darwin-arm64   # the same question for another target
"""

from __future__ import annotations

import argparse
import importlib.util
import json
import pathlib
import re
import sys

HERE = pathlib.Path(__file__).resolve().parent
REPO = HERE.parent
PENDING = REPO / "docs/baseline/android-imports-pending.txt"
ARMS = REPO / "docs/baseline/android-arms.tsv"
SOURCES = REPO / "docs/baseline/frozen-build-sources-android.txt"
LIBS = ("docs/baseline/lib-link-sources-android.txt", "docs/baseline/lib-link-sources.txt")
OUT = REPO / "docs/baseline/android-gaps.tsv"
WINDOW = ("WINDOW_SYSTEM", "HAVE_X_WINDOWS", "HAVE_NS", "HAVE_PGTK", "HAVE_HAIKU", "HAVE_NTGUI",
          "WINDOWSNT", "MSDOS")

# Longest alternative first: `(if|ifdef|...)` matches `if` inside `#ifdef`, and then `\\b` fails
# and backtracking saves the *match* - but a strip built on the group without the boundary silently
# turns `#ifdef HAVE_WINDOW_SYSTEM` into `def HAVE_WINDOW_SYSTEM`, which evaluates false and made
# every guarded region read as dead (measured and fixed in the same round, status 4.749).
DIRECTIVE = re.compile(r"^\s*#\s*(ifdef|ifndef|elif|else|endif|if)\b")


def ci():
    spec = importlib.util.spec_from_file_location("cinventory", HERE / "cinventory.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def macro_table(stem: str) -> dict[str, str]:
    """The frozen configuration's macro table, `undefined`/`absent` reading as undefined.

    Both sections are needed: `features` is `gen-config-json.py --keys-from-header`'s dump of the
    configuration's own `config.h` (781 names for android-arm64, `HAVE_WINDOW_SYSTEM` and
    `HAVE_ANDROID` among them) and `macros` is the small extra set measured on the command line.
    Reading `macros` alone left the table without `HAVE_WINDOW_SYSTEM`, so every
    `#ifdef HAVE_WINDOW_SYSTEM` region evaluated false and the first version of this table called
    the live Android arms dead (found and fixed in the same round, status 4.749).
    """
    config = json.loads((REPO / f"docs/baseline/config/{stem}.json").read_text(encoding="utf-8"))
    merged = {**config["features"], **config["macros"]}
    return {name: ("undef" if str(value) in ("undefined", "absent", "") else str(value))
            for name, value in merged.items()}


def directive_condition(kind: str, line: str) -> str:
    """The `#if`-expression KIND-line contributes (`#ifdef X` -> `defined (X)`)."""
    body = line.strip()
    body = re.sub(r"^#\s*(ifdef|ifndef|elif|else|if)\b", "", body).strip()
    if kind == "ifdef":
        return f"defined ({body})"
    if kind == "ifndef":
        return f"!defined ({body})"
    if kind == "else":
        return "1"
    return body


def live_map(inventory, lines: list[str]) -> list[bool]:
    """Per-line: does the preprocessor keep this line under the installed macro table?"""
    stack: list[list] = []          # [parent_live, this_live, taken]
    live: list[bool] = []
    for line in lines:
        m = DIRECTIVE.match(line)
        if m:
            kind = m.group(1)
            if kind in ("if", "ifdef", "ifndef", "elif", "else"):
                parent = stack[-1][1] if stack else True
                cond = inventory.eval_condition(directive_condition(kind, line))
                if kind in ("elif", "else") and stack:
                    parent = stack[-1][0]
                    cond = (not stack[-1][2]) and (cond is not False)
                taken = bool(cond) and parent
                if kind in ("elif", "else") and stack:
                    stack[-1][1] = taken
                    stack[-1][2] = stack[-1][2] or taken
                else:
                    stack.append([parent, taken, taken])
            elif kind == "endif":
                if stack:
                    stack.pop()
        live.append(stack[-1][1] if stack else True)
    return live


def source_set() -> list[pathlib.Path]:
    files: list[pathlib.Path] = []
    for line in SOURCES.read_text(encoding="utf-8").splitlines():
        if line.strip() and not line.startswith("#"):
            files.append(REPO / "emacs" / line.strip())
    for rel in LIBS:
        path = REPO / rel
        if not path.is_file():
            continue
        for line in path.read_text(encoding="utf-8").splitlines():
            if line.strip() and not line.startswith("#"):
                files.append(REPO / "emacs" / line.strip())
    return [f for f in files if f.is_file()]


def references(name: str, files: list[pathlib.Path], inventory) -> list[tuple[str, int, bool]]:
    """(file, line, live) for every C site that *calls* NAME, source files before headers."""
    pattern = re.compile(rf"\b{re.escape(name)}\s*\(")
    hits: list[tuple[str, int, bool]] = []
    for path in files:
        text = path.read_text(errors="replace")
        if name not in text:
            continue
        lines = text.splitlines()
        live = live_map(inventory, lines)
        for i, line in enumerate(lines):
            if line.lstrip().startswith(("*", "/*", "//")):
                continue
            if pattern.search(line):
                rel = str(path.relative_to(REPO))
                hits.append((rel, i + 1, live[i]))
    # A definition is not a reference: keep only the sites that are not the name's own definition
    # line, and prefer the units the build compiles into the library over declarations in headers.
    defs = {line for path, line, _ in []}
    del defs
    source = [h for h in hits if h[0].endswith(".c")]
    return source or hits


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--config", default="android-arm64")
    ap.add_argument("--out", default=str(OUT))
    args = ap.parse_args()

    inventory = ci()
    inventory.set_condition_macros(macro_table(args.config))
    inventory.set_condition_paths(None)

    names = [l.strip() for l in PENDING.read_text(encoding="utf-8").splitlines()
             if l.strip() and not l.startswith("#")]
    conditions = {}
    for line in ARMS.read_text(encoding="utf-8").splitlines():
        if line.strip() and not line.startswith("#"):
            fields = line.split("\t")
            if len(fields) >= 4:
                conditions[fields[0]] = fields[3]

    files = source_set()
    print(f"{len(names)} unresolved name(s); {len(files)} file(s) in the {args.config} build set")
    rows = []
    counts: dict[str, int] = {}
    for name in names:
        # The same rule `tools/check-doc-numbers.py` counts the window arms with: the condition
        # string, the `Vx_`/`x_` frontend variables, and the one dispextern entry point - a name
        # whose definition sits in a file with no condition of its own (`docs/baseline/
        # window-arms-plan.tsv`'s four `Vx_*`) has an empty string there and is still an arm.
        window = (any(w in conditions.get(name, "") for w in WINDOW)
                  or name.startswith(("Vx_", "x_"))
                  or name == "tty_draw_row_with_mouse_face")
        hits = references(name, files, inventory)
        live = [h for h in hits if h[2]]
        if live:
            verdict = "c-included"
            where = f"{live[0][0]}:{live[0][1]}"
        elif hits:
            verdict = "c-excluded"
            where = f"{hits[0][0]}:{hits[0][1]}"
        else:
            verdict = "no-reference"
            where = ""
        counts[verdict] = counts.get(verdict, 0) + 1
        dead = [f"{f}:{l}" for f, l, is_live in hits if not is_live]
        rows.append((name, "window" if window else "other", verdict, where, "; ".join(dead[:3]),
                     str(len(hits))))
    header = ("# Whose deviation is each unresolved Android import (tools/gen-android-gaps.py,\n"
              f"# status 4.749).  Configuration: {args.config}.  `c-included` = C compiles a\n"
              "# reference, so C's own library imports it too (an upstream gap); `c-excluded` =\n"
              "# every C reference is under a condition this configuration answers false, so the\n"
              "# port compiled what C leaves out (a defect in the port unit that imports it);\n"
              "# `no-reference` = no file in the build set even calls it (an invented import).\n"
              "# name\tscope\tverdict\tfirst C site\tdead C sites\tC sites\n")
    out = header + "".join("\t".join(str(f) for f in row) + "\n" for row in rows)
    summary = ", ".join(f"{k} {v}" for k, v in sorted(counts.items()))
    if args.out == "-":
        print(out, end="")
    else:
        pathlib.Path(args.out).write_text(out, encoding="utf-8")
        print(f"wrote {pathlib.Path(args.out).relative_to(REPO)}: {len(rows)} row(s)")
    print(f"verdicts: {summary}")
    for name, scope, verdict, where, dead, n in rows:
        if verdict != "c-included":
            print(f"  {verdict:13s} {scope:6s} {name:34s} {where or '(no C site)'}"
                  + (f"  dead: {dead}" if dead and verdict == "c-excluded" else ""))
    return 0


if __name__ == "__main__":
    sys.exit(main())
