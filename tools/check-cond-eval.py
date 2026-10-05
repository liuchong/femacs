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
import os
import re
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("cinventory", HERE / "cinventory.py")
ci = importlib.util.module_from_spec(spec)
spec.loader.exec_module(ci)


# The probe directory.  It is a fixed path because two ledgers cite it as the place a run's
# probe file lived (docs/ports/nsterm.md 2174, docs/ports/include-readutmp.md 76), and the probe
# text is deterministic - the same condition regenerates the same file.  What is *not* kept is
# the file itself: by status 3.681 the directory held **6,128 files / 891 MB**, because nothing
# ever removed them and the disk filled up twice (the second time mid-run, `OSError: [Errno 28]`).
# One probe is removed as soon as clang has answered; `FEMACS_COND_EVAL_KEEP=1` keeps them for a
# failing run's post-mortem.
PROBE_DIR = Path("/tmp/femacs-cond-eval")
KEEP_PROBES = bool(os.environ.get("FEMACS_COND_EVAL_KEEP"))


def clang_verdict(header: str, prefix: str, expr: str, kind: str,
                  include_dirs: list[str], tag: str) -> bool:
    """What the C preprocessor says for one condition, with the file's own state in front.

    The probe is `#include <config.h>`, then the header's text up to the condition, then the
    condition wrapped around a `#define` — so clang answers with exactly the macros the frozen
    configuration has at that point (the header's own `#define`s and its live-branch
    resolution included), and the answer is a *macro* rather than an output line: a marker
    inside a dead branch would simply not be defined, which also means an `#elif` (whose chain
    is still open in the prefix) tests cleanly.  `-dM` only, so the prefix is never parsed.
    """
    marker = f"EMACS_COND_TAKEN_{tag}"
    probe = PROBE_DIR / f"probe_{tag}.c"
    probe.parent.mkdir(parents=True, exist_ok=True)
    # Close the conditionals the prefix left open: an `#elif`'s condition is only reached
    # when every earlier branch of its chain was false, so the defines in those bodies do not
    # apply and the probe belongs at the chain's own level.  Without the `#endif`s the probe
    # sits inside the still-open chain and clang answers "dead" for the wrong reason — that
    # was the last disagreement (lisp.h:93).
    # An `#elif`'s own chain is still open in the prefix, so it is closed with exactly one
    # `#endif`; anything else would close the *enclosing* chains too and make clang answer
    # `true` for a condition the real file never reaches (`systime.h`'s HPUX block).
    probe.write_text(
        "#include <config.h>\n" + prefix + "\n" + ("#endif\n" if kind == "elif" else "")
        + (f"#if defined ({expr})" if kind == "ifdef"
           else f"#if ! defined ({expr})" if kind == "ifndef"
           else f"#if {expr}") + "\n"
        + f"#define {marker} 1\n#endif\n", encoding="utf-8")
    out = subprocess.run(["clang", "-dM", "-E", "-std=gnu23", *include_dirs, str(probe)],
                         capture_output=True, text=True).stdout
    taken = f"#define {marker} " in out
    if not KEEP_PROBES:
        probe.unlink(missing_ok=True)
    return taken


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("headers", nargs="*")
    ap.add_argument("--cond-macros", default="docs/baseline/config/darwin-arm64.macros.tsv")
    ap.add_argument("--configure", default="build/emacs-ref")
    ap.add_argument("--show-unknown", action="store_true",
                    help="also list the conditions the evaluator answers `unknown`")
    ap.add_argument("--write-verified", metavar="PATH",
                    help="write the headers that raised zero disagreements to PATH; that "
                         "file is what `tools/extract-abi.py --cond-macro-paths` consults, so "
                         "the macro table is never used for a file the evaluator gets wrong")
    ap.add_argument("--verified", metavar="PATH",
                    help="read such a file and require every listed header to still raise "
                         "zero disagreements (this is the gate step in `xtask gate-fast`)")
    args = ap.parse_args()
    table = ci.read_condition_macros(args.cond_macros) if Path(args.cond_macros).is_file() else {}
    include_dirs = ["-I", f"{args.configure}/src", "-I", f"{args.configure}/lib",
                    "-I", "emacs/src", "-I", "emacs/lib"]
    known: list[str] = []
    if args.verified:
        # The list is the gate's input, not a separate assertion: `xtask gate-fast` runs
        # `check-cond-eval.py --verified <list>` with no headers of its own.
        known = [line.strip() for line in Path(args.verified).read_text(
            encoding="utf-8").split("\n") if line.strip() and not line.startswith("#")]
        args.headers = known + [h for h in args.headers if h not in known]
    if not args.headers:
        print("no headers to check: pass headers, or --verified <list>", file=sys.stderr)
        return 1
    total_all = unknown_all = disagree_all = 0
    clean: list[str] = []
    regressed: list[str] = []
    for header in args.headers:
        text = Path("emacs", header).read_text(encoding="utf-8", errors="replace")
        lines = text.split("\n")
        ci.set_condition_macros(dict(table))
        live = ci.dead_branch_map(text, header)       # fills ci.CONDITION_LOG
        unknown = disagreements = total = 0
        for line_index, kind, expr, got in ci.CONDITION_LOG:
            # `CONDITION_LOG` keeps the reference line's own text, so a trailing block comment
            # is still in `expr`.  C removes comments in translation phase 3, *before* a
            # directive is read, so the condition clang must be asked about is the comment-free
            # one: `#ifdef INLCR /* ... */` made the probe ill-formed (`unterminated /* comment`
            # and then `missing ')' after 'defined'`), and clang answered "dead" for a branch
            # the frozen build compiles - two false disagreements in src/sysdep.c, found by that
            # lane and fixed here (status 3.63).
            expr = re.sub(r"/\*.*$", "", expr).rstrip()
            prefix = "\n".join(lines[:line_index])
            # The tag becomes a *C identifier* (`EMACS_COND_TAKEN_<tag>`), so a stem with a
            # hyphen or a dot has to be sanitised: `intprops-internal_17` made clang read the
            # marker as `EMACS_COND_TAKEN_intprops` with the body `-internal_17 1`, so the
            # gate's `"#define <marker> " in out` test never matched and clang was recorded as
            # dead for *every* branch the frozen build takes (measured, status 3.201 - the
            # text is what pointed at it: `ISO C99 requires whitespace after the macro name`).
            tag = re.sub(r"[^A-Za-z0-9_]", "_", Path(header).stem) + f"_{line_index}"
            want = clang_verdict(header, prefix, expr, kind, include_dirs, tag)
            # What matters is whether the *branch* is the one the frozen configuration
            # compiles, which is what `live` says: for an `#elif` whose chain already took a
            # branch, or a condition nested in a dead one, the condition's own truth is not
            # the answer.  An `unknown` verdict keeps the branch live (conservative), which
            # disagrees with clang whenever clang knows the branch is dead — that is counted,
            # because the gate would then demand items from a dead branch.
            got_live = bool(live[line_index])
            total += 1
            if got is None:
                unknown += 1
                if args.show_unknown:
                    print(f"UNKNOWN {header}:{line_index + 1} `{kind} {expr}`")
                if not want:
                    disagreements += 1
                    print(f"DISAGREE(unknown) {header}:{line_index + 1} `{kind} {expr}` "
                          f"— clang {want}, evaluator keeps the branch live")
            elif got_live != want:
                disagreements += 1
                print(f"DISAGREE {header}:{line_index + 1} `{kind} {expr}` "
                      f"— clang {want}, evaluator {got_live}")
        total_all += total
        unknown_all += unknown
        disagree_all += disagreements
        if disagreements:
            regressed.append(header)
        else:
            clean.append(header)
        mark = "VERIFIED" if not disagreements else "NOT VERIFIED"
        print(f"{header}: {total} conditions, {unknown} unknown, {disagreements} disagreements"
              f" [{mark}]")
    if args.write_verified:
        Path(args.write_verified).write_text(
            "# Reference files whose `#if` conditions the frozen macro table decides exactly:\n"
            "# every condition here agreed with clang (tools/check-cond-eval.py).  Only these\n"
            "# files may consult the table (tools/cinventory.py CONDITION_MACROS_PATHS); a\n"
            "# gnulib `*.in.h` template stays out until it agrees, because the table cannot\n"
            "# decide its `@PLACEHOLDER@`s (PORTING.md P-66).\n"
            + "".join(f"{h}\n" for h in sorted(clean)), encoding="utf-8")
        print(f"wrote {args.write_verified}: {len(clean)} verified, {len(regressed)} not")
    if args.verified and regressed:
        print(f"verified list regressed: {' '.join(regressed)}", file=sys.stderr)
        return 1
    print(f"{total_all} conditions, {unknown_all} unknown to the evaluator, "
          f"{disagree_all} disagreements")
    return 1 if args.verified is None and disagree_all else 0


if __name__ == "__main__":
    raise SystemExit(main())
