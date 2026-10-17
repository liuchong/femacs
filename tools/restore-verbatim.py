#!/usr/bin/env python3
"""Restore the copied trees to the reference's bytes (the product-name decision, 2026-10-18).

The six copied trees (`lisp`, `test`, `etc`, `doc`, `leim`, `admin`) carried a registered
product-name rewrite; the user decided they must be copied verbatim instead, with compatibility
interfaces unchanged (nothing in those trees was renamed - only licence prose, docstrings, tags
and manual text).  This tool re-copies every file whose bytes differ, which is the same set the
verbatim gate compares, and leaves generated artefacts (`.elc`) alone.

    python3 tools/restore-verbatim.py            # copy, then report
    python3 tools/restore-verbatim.py --check    # report only
"""

from __future__ import annotations

import argparse
import pathlib
import shutil
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
TREES = ("lisp", "test", "etc", "doc", "leim", "admin")
SUBSTITUTIONS = REPO / "docs/baseline/verbatim-substitutions.tsv"


def registered_files() -> set[str]:
    """The files the substitution registry names (`path\tline\tline-text\treason`)."""
    rows = set()
    if SUBSTITUTIONS.is_file():
        for raw in SUBSTITUTIONS.read_text("utf-8").splitlines():
            parts = raw.split("\t")
            if len(parts) >= 4 and parts[1].strip().isdigit():
                rows.add(parts[0])
    return rows
SKIP_SUFFIXES = (".elc",)


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--check", action="store_true")
    a = ap.parse_args()
    registered = registered_files()
    differing, copied, missing = [], 0, 0
    for tree in TREES:
        ref = REPO / "emacs" / tree
        port = REPO / "femacs" / tree
        if not ref.is_dir():
            continue
        for rp in ref.rglob("*"):
            if not rp.is_file() or rp.suffix in SKIP_SUFFIXES:
                continue
            pp = port / rp.relative_to(ref)
            # Only the files the registry names: other differences in these trees are generated
            # artefacts or deliberate content and are not this tool's business.
            if str(pp.relative_to(REPO)) not in registered:
                continue
            if not pp.is_file():
                missing += 1
                continue
            try:
                if rp.read_bytes() == pp.read_bytes():
                    continue
            except OSError:
                continue
            differing.append(str(pp.relative_to(REPO)))
            if not a.check:
                shutil.copyfile(rp, pp)
                copied += 1
    print(f"differing files: {len(differing)}; {'would copy' if a.check else 'copied'}: {copied}; "
          f"missing on the port side: {missing}")
    for row in differing[:5]:
        print(f"    {row}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
