#!/usr/bin/env python3
"""Generate `finder-inf.el`, the keyword-to-package map, the reference's own way.

The reference's rule (`lisp/Makefile.in` 166-170) is:

    emacs -l finder \\
      --eval '(setq generated-finder-keywords-file "<srcdir>/finder-inf.el")' \\
      -f finder-compile-keywords-make-dist <SUBDIRS_FINDER>

with `SUBDIRS = $(sort $(shell find ${srcdir} -type d -print))` minus `obsolete`, `term` and
`leim*` (lines 121-129).  The port runs the same recipe with its own dumped program over its own
lisp tree, so the map is built from this repository's sources; the file is a *product*, never
copied or tracked (the reference does not track it either, and it is in `.gitignore`).

The reading is the reference's own file with the sanctioned product-name rewrite applied: the
generator reads the copied trees, whose declaration and summary lines carry this repository's
name (docs/baseline/verbatim-substitutions.tsv), so those lines come out rewritten.

    python3 tools/gen-finder.py                 # write femacs/lisp/finder-inf.el
    python3 tools/gen-finder.py --check        # also diff against the reference's own product
"""

from __future__ import annotations

import argparse
import pathlib
import subprocess
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
PORT_LISP = REPO / "femacs/lisp"
PROGRAM = REPO / "build/port/src/emacs"
REFERENCE = REPO / "emacs/lisp/finder-inf.el"


def dirs() -> list[str]:
    """SUBDIRS_FINDER: every directory under the lisp tree except leim* (and the retired pair)."""
    out = []
    for p in sorted(PORT_LISP.rglob("*")):
        if not p.is_dir():
            continue
        rel = p.relative_to(PORT_LISP).as_posix()
        if rel.split("/")[0] in ("leim", "obsolete", "term"):
            continue
        out.append(str(p))
    return [str(PORT_LISP)] + out


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--check", action="store_true",
                    help="diff the product against the reference's own finder-inf.el")
    a = ap.parse_args()
    if not PROGRAM.is_file():
        print(f"missing {PROGRAM.relative_to(REPO)}; `xtask gate-load` links it", file=sys.stderr)
        return 2
    out = PORT_LISP / "finder-inf.el"
    argv = [str(PROGRAM), "-Q", "-batch", "-L", str(PORT_LISP), "-l", "finder",
            "--eval", f'(setq generated-finder-keywords-file "{out}")',
            "-f", "finder-compile-keywords-make-dist", *dirs()]
    proc = subprocess.run(argv, cwd=str(PORT_LISP), capture_output=True, text=True,
                          env={"EMACSLOADPATH": str(PORT_LISP), "PATH": "/usr/bin:/bin",
                               "LC_ALL": "C"})
    if proc.returncode != 0 or not out.is_file():
        print(proc.stdout + proc.stderr, file=sys.stderr)
        return 1
    print(f"wrote {out.relative_to(REPO)} ({out.stat().st_size} bytes)")
    if not a.check:
        return 0
    if not REFERENCE.is_file():
        print(f"no reference product at {REFERENCE.relative_to(REPO)} to compare with")
        return 0
    ref = REFERENCE.read_text().split("\n")
    port = out.read_text().split("\n")
    import difflib
    sm = difflib.SequenceMatcher(a=ref, b=port, autojunk=False)
    explained = unexplained = 0
    samples = []
    for tag, i1, i2, j1, j2 in sm.get_opcodes():
        if tag == "equal":
            continue
        if tag == "replace" and (i2 - i1) == (j2 - j1):
            for x, y in zip(ref[i1:i2], port[j1:j2]):
                if y == x.replace("GNU Emacs", "femacs"):
                    explained += 1
                else:
                    unexplained += 1
                    samples.append((x, y))
        else:
            unexplained += max(i2 - i1, j2 - j1)
            samples.append((f"<{tag}: {i2-i1} line(s)>", f"<{j2-j1} line(s)>"))
    print(f"reference {len(ref)} lines, port {len(port)} lines; "
          f"sanctioned rewrite(s): {explained}; other: {unexplained}")
    for x, y in samples[:12]:
        print(f"  - {x[:100]}\n  + {y[:100]}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
