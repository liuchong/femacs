#!/usr/bin/env python3
"""Differential test for the `#if` evaluator against the C preprocessor.

`tools/cinventory.py` decides which `#if` branches the frozen configuration compiles; the
inventory then compares a mirror with that text (PORTING.md P-66).  An evaluator that answers
*wrongly* is worse than one that answers "unknown", so every condition this tool finds in the
given headers is checked against clang: the file is copied with a unique marker
(`enum { EMACS_COND_k = 1 };`) inserted as the first line of each branch, the copy is
preprocessed with the frozen configuration, and the markers that survive say which branch the
C preprocessor took.  Any disagreement is printed.

    python3 tools/check-cond-eval.py src/lisp.h src/dispextern.h ...
    python3 tools/check-cond-eval.py --all-of src/dispextern.h    # plus everything it includes
"""

from __future__ import annotations

import argparse
import importlib.util
import re
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("cinventory", HERE / "cinventory.py")
ci = importlib.util.module_from_spec(spec)
spec.loader.exec_module(ci)


def conditions(text: str) -> list[tuple[int, str, str]]:
    """`(line_index, kind, expression)` for every `#if`/`#elif`/`#ifdef`/`#ifndef`."""
    out = []
    for index, (logical, physical) in enumerate(ci.logical_lines(text)):
        s = logical.strip()
        if not s.startswith("#"):
            continue
        body = s[1:].strip()
        word = body.split(maxsplit=1)[0] if body else ""
        if word in ("if", "elif", "ifdef", "ifndef"):
            out.append((logical, word, body[len(word):].strip()))
    return out


def marked_source(text: str) -> tuple[str, list[tuple[str, str, str]]]:
    """TEXT with a marker as the first line of every branch, plus the (marker, kind, expr) list."""
    out, marks = [], []
    counter = 0
    for logical, _ in ci.logical_lines(text):
        out.append(logical)
        s = logical.strip()
        if s.startswith("#"):
            body = s[1:].strip()
            word = body.split(maxsplit=1)[0] if body else ""
            if word in ("if", "elif", "ifdef", "ifndef", "else"):
                marker = f"EMACS_COND_{counter}"
                expr = body[len(word):].strip() if word != "else" else ""
                marks.append((marker, word, expr))
                out.append(f"enum {{ {marker} = 1 }};")
                counter += 1
    return "\n".join(out), marks


def verdicts(header: str, include_dirs: list[str]) -> dict[str, bool]:
    """Which markers clang keeps for HEADER's branches."""
    text = Path("emacs", header).read_text(encoding="utf-8", errors="replace")
    marked, marks = marked_source(text)
    probe = Path("/tmp/femacs-cond-eval") / Path(header).name
    probe.parent.mkdir(parents=True, exist_ok=True)
    # The copy must see the frozen configuration: without `<config.h>` every `#ifdef HAVE_…`
    # is false in the probe and the differential would compare against the wrong verdicts.
    probe.write_text("#include <config.h>\n" + marked, encoding="utf-8")
    cmd = ["clang", "-E", "-std=gnu23", *include_dirs, str(probe)]
    out = subprocess.run(cmd, capture_output=True, text=True).stdout
    kept = {name for name in re.findall(r"EMACS_COND_\d+", out)}
    by_name = {marker: (kind, expr) for marker, kind, expr in marks}
    result = {}
    for marker, (kind, expr) in by_name.items():
        result[marker] = marker in kept
    return result, marks


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("headers", nargs="+")
    ap.add_argument("--cond-macros", default="docs/baseline/config/darwin-arm64.macros.tsv")
    ap.add_argument("--configure", default="build/emacs-ref")
    args = ap.parse_args()
    table = ci.read_condition_macros(args.cond_macros) if Path(args.cond_macros).is_file() else {}
    include_dirs = ["-I", f"{args.configure}/src", "-I", f"{args.configure}/lib",
                    "-I", "emacs/src", "-I", "emacs/lib"]
    unknown = disagreements = total = 0
    for header in args.headers:
        ci.set_condition_macros(dict(table))
        kept, marks = verdicts(header, include_dirs)
        ci.set_condition_macros(dict(table))
        for marker, kind, expr in marks:
            if kind == "else":
                continue                     # an `#else` has no condition of its own
            want = kept[marker]
            if kind == "ifdef":
                got = ci.eval_condition(f"defined ({expr})")
            elif kind == "ifndef":
                got = ci.eval_condition(f"! defined ({expr})")
            else:
                got = ci.eval_condition(expr)
            total += 1
            if got is None:
                unknown += 1
            elif got != want:
                disagreements += 1
                print(f"DISAGREE {header}: `{kind} {expr}` — clang {want}, evaluator {got}")
    print(f"{total} conditions, {unknown} unknown to the evaluator, {disagreements} disagreements")
    return 1 if disagreements else 0


if __name__ == "__main__":
    raise SystemExit(main())
