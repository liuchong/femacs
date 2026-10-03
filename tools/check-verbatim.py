#!/usr/bin/env python3
"""Verify that the verbatim trees are byte-identical to the reference checkout.

docs/port-map.md classifies six trees as `Verbatim` (`lisp`, `test`, `etc`, `doc`,
`leim`, `admin` -> `femacs/<top>/**`): the goal says they are *copied*, one byte at a
time, and that no test file may be modified, skipped or deleted.  Prose cannot prove
that, so this is the gate:

    python3 tools/check-verbatim.py            # all six trees
    python3 tools/check-verbatim.py lisp test  # only these

It compares the file *lists* (a missing file is as much a failure as a changed one) and
then the contents, streamed in blocks so the four-and-a-half-million lines never sit in
memory.  Exit status 0 means exact equality; 1 lists what differs.
"""

from __future__ import annotations

import argparse
import hashlib
import sys
from pathlib import Path

# docs/port-map.md's VERBATIM_DIRS: `<reference dir>` -> `femacs/<same name>`.
VERBATIM_DIRS = ("lisp", "test", "etc", "doc", "leim", "admin")
REFERENCE = Path("emacs")
TARGET = Path("femacs")
BLOCK = 1 << 20


def files_under(root: Path) -> dict[str, Path]:
    return {str(p.relative_to(root)): p for p in sorted(root.rglob("*")) if p.is_file()}


def digest(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as fh:
        while chunk := fh.read(BLOCK):
            h.update(chunk)
    return h.hexdigest()


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("trees", nargs="*", default=list(VERBATIM_DIRS),
                    help=f"trees to check (default: {' '.join(VERBATIM_DIRS)})")
    args = ap.parse_args()

    failures = 0
    for tree in args.trees:
        src, dst = REFERENCE / tree, TARGET / tree
        if not src.is_dir():
            print(f"MISSING {src}: the reference tree is not checked out here")
            failures += 1
            continue
        if not dst.is_dir():
            print(f"TODO    {tree}: {dst} has not been copied yet")
            failures += 1
            continue
        a, b = files_under(src), files_under(dst)
        missing = sorted(set(a) - set(b))
        extra = sorted(set(b) - set(a))
        changed = []
        for rel in sorted(set(a) & set(b)):
            if a[rel].stat().st_size != b[rel].stat().st_size:
                changed.append(rel)
            elif digest(a[rel]) != digest(b[rel]):
                changed.append(rel)
        if missing or extra or changed:
            failures += 1
            print(f"FAIL    {tree}: {len(missing)} missing, {len(extra)} extra, "
                  f"{len(changed)} changed")
            for rel in (missing + extra + changed)[:10]:
                print(f"          {rel}")
        else:
            print(f"OK      {tree}: {len(a)} files, byte-identical")
    total = sum(len(files_under(REFERENCE / t)) for t in args.trees
                if (REFERENCE / t).is_dir())
    print(f"{len(args.trees)} trees, {total} files, {failures} failures")
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
