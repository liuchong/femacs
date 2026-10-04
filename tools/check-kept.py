#!/usr/bin/env python3
"""Pin the whole of the Kept class: the reference's body, byte for byte.

`docs/port-map.md` classifies six files as `Kept` - the generator's words are "assembly
and data files that are used as they are", and its mode row says `path preserved`: the
target *is* the reference's path, the six `exec/loader-*.s` of the Android exec loader
(status 3.284 copied them and `cmp`ed them).  They are the one class with no port to
review, which is why they are also the one class that kept the reference's copied
declaration block for a day longer than the rest of the tree: `check-declarations.py`
reads by suffix, and `.s` was not in its list, so the 16-line block on each of the six
was invisible to every gate (status 3.295).

What makes them `Kept` is the *body* - the assembler sees exactly what the reference's
file holds - so the block was replaced by one origin line and the body stayed byte for
byte.  This gate is what keeps that true: the map says which files are Kept, the baseline
says what the reference is and which line the body starts at, and every reading is checked
against the files on disk.

    python3 tools/check-kept.py              # scan, compare with the baseline
    python3 tools/check-kept.py --selftest   # prove the readings fire
    python3 tools/check-kept.py --write      # rewrite docs/baseline/kept-files.tsv

`docs/baseline/kept-files.tsv`, columns
`<path>\\t<reference>\\t<ref_sha256>\\t<removed_lines>\\t<header>`: `ref_sha256` pins the
reference's whole file, `removed_lines` is the number of reference lines the origin line
replaces (the blank line that followed the block is the body's first line), and `header`
is the exact first line the target carries.  Per row three readings: the reference hashes
to `ref_sha256`; the target's first line is `header`, and `header` names the reference path
and a `<rev>` (`PATH_RULE`); the target's lines after the header are byte-identical to the
reference's lines after `removed_lines`.  The row set must also be exactly the map's Kept
section - a Kept file with no row, and a row naming a file the map does not classify as
Kept, are both failures.  Exit status 0 means all three hold.
"""

from __future__ import annotations

import argparse
import hashlib
import re
import shutil
import sys
import tempfile
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
REFERENCE = REPO / "emacs"
MAP = REPO / "docs/port-map.md"
BASELINE = REPO / "docs/baseline/kept-files.tsv"
# The origin line, in the dialect's own comment leader (`//` in the AArch64 file, `@` in
# the ARM one, `#` in the four that GNU as writes that way).
PATH_RULE = re.compile(r"^(?://|#|@|;) femacs: kept from (\S+) @ ([0-9a-f]{7,40})\.$")


def map_rows() -> list[tuple[str, str]]:
    """`(reference, target)` for the `## Kept` section of the generated port map."""
    out: list[tuple[str, str]] = []
    in_section = False
    for line in MAP.read_text(encoding="utf-8").splitlines():
        if line.startswith("## "):
            in_section = line.startswith("## Kept")
            continue
        if not in_section or not line.startswith("| `"):
            continue
        cells = [c.strip().strip("`") for c in line.split("|")[1:-1]]
        if len(cells) >= 3 and cells[0] and cells[2].startswith("femacs/"):
            out.append((cells[0], cells[2]))
    return out


def read_baseline() -> dict[str, tuple[str, str, int, str]]:
    """`target -> (reference, ref_sha256, removed_lines, header)`."""
    rows: dict[str, tuple[str, str, int, str]] = {}
    for raw in BASELINE.read_text(encoding="utf-8").splitlines():
        if not raw.strip() or raw.startswith("#") or raw.startswith("path\t"):
            continue
        parts = raw.split("\t")
        if len(parts) < 5 or not parts[3].strip().isdigit():
            print(f"BAD ROW {BASELINE.name}: {raw!r}")
            continue
        rows[parts[0]] = (parts[1], parts[2], int(parts[3]), parts[4])
    return rows


def check_one(root: Path, path: str, reference: str, ref_sha: str, removed: int,
              header: str) -> list[str]:
    """The readings for one row, as failure messages (empty when it holds)."""
    bad: list[str] = []
    ref = root / REFERENCE.name / reference
    target = root / path
    if not ref.is_file():
        return [f"{reference}: reference file is missing"]
    if not target.is_file():
        return [f"{path}: Kept file is missing"]
    ref_bytes = ref.read_bytes()
    got_sha = hashlib.sha256(ref_bytes).hexdigest()
    if ref_sha and got_sha != ref_sha:
        bad.append(f"{reference}: sha256 {got_sha[:16]}..., baseline says {ref_sha[:16]}...")
    m = PATH_RULE.match(header)
    if header and not m:
        bad.append(f"{path}: header {header!r} is not `\u2026 femacs: kept from \u2026 @ <rev>.`")
    elif header and m.group(1) != reference:
        bad.append(f"{path}: header names {m.group(1)}, the map says {reference}")
    ref_lines = ref_bytes.split(b"\n")
    target_lines = target.read_bytes().split(b"\n")
    if header and target_lines[0].decode("utf-8", "replace") != header:
        bad.append(f"{path}: first line is {target_lines[0]!r}, baseline says {header!r}")
    if removed < 0 or removed > len(ref_lines):
        return bad + [f"{path}: removed_lines {removed} is outside the reference"]
    if target_lines[1:] != ref_lines[removed:]:
        for i, (a, b) in enumerate(zip(target_lines[1:], ref_lines[removed:])):
            if a != b:
                bad.append(f"{path}: body line {i + 1} differs from {reference} "
                           f"line {removed + i + 1}: {a[:60]!r} != {b[:60]!r}")
                break
        else:
            bad.append(f"{path}: body is {len(target_lines) - 1} lines, the reference's "
                       f"body from line {removed + 1} is {len(ref_lines) - removed}")
    return bad


def write_baseline(root: Path) -> int:
    rows = map_rows()
    body = ["# The Kept class of the generated port map (docs/port-map.md): the reference's",
            "# own file, body byte for byte, with the copied declaration block replaced by one",
            "# origin line (PORTING.md 0.1-19, 3.295).  Generated by `tools/check-kept.py",
            "# --write`; checked by `tools/check-kept.py`.  Columns:",
            "# <path>\\t<reference>\\t<ref_sha256>\\t<removed_lines>\\t<header>",
            "path\treference\tref_sha256\tremoved_lines\theader"]
    for reference, target in rows:
        ref_bytes = (root / REFERENCE.name / reference).read_bytes()
        target_lines = (root / target).read_bytes().split(b"\n")
        removed = len(ref_bytes.split(b"\n")) - len(target_lines) + 1
        header = target_lines[0].decode("utf-8")
        if not PATH_RULE.match(header):
            print(f"REFUSED {target}: first line {header!r} is not the kept-file header")
            return 1
        body.append(f"{target}\t{reference}\t{hashlib.sha256(ref_bytes).hexdigest()}"
                    f"\t{removed}\t{header}")
    BASELINE.write_text("\n".join(body) + "\n", encoding="utf-8")
    print(f"wrote {BASELINE.relative_to(REPO)}: {len(rows)} row(s)")
    return 0


def selftest() -> int:
    """Three readings, each with the fixed shape: clean passes, each mutation fires."""
    bad = 0
    source = Path(__file__).resolve().parent.parent
    row = map_rows()[0]
    reference, target = row
    with tempfile.TemporaryDirectory() as tmp:
        root = Path(tmp)
        (root / REFERENCE.name / Path(reference).parent).mkdir(parents=True)
        (root / Path(target).parent).mkdir(parents=True)
        shutil.copyfile(source / REFERENCE.name / reference, root / REFERENCE.name / reference)
        shutil.copyfile(source / target, root / target)
        ref_sha = hashlib.sha256((root / REFERENCE.name / reference).read_bytes()).hexdigest()
        header = (root / target).read_bytes().split(b"\n")[0].decode()
        target_lines = (root / target).read_bytes().split(b"\n")
        removed = len((root / REFERENCE.name / reference).read_bytes().split(b"\n")) \
            - len(target_lines) + 1
        cases = [("the kept file", lambda r: None, 0)]
        def flip(r: Path) -> None:
            data = (r / target).read_bytes()
            (r / target).write_bytes(data[:-2] + bytes([data[-2] ^ 0x20]) + data[-1:])
        def unheader(r: Path) -> None:
            lines = (r / target).read_bytes().split(b"\n")
            (r / target).write_bytes(b"\n".join(lines[1:]))
        # Removing the origin line fires twice, on purpose: the first line is no longer the
        # recorded header, *and* the body it left behind is now shifted against the
        # reference's - the two readings are independent, which is the point of having both.
        cases += [("one body byte flipped", flip, 1), ("the origin line removed", unheader, 2)]
        for name, mutate, want in cases:
            mutate(root)
            got = len(check_one(root, target, reference, ref_sha, removed, header))
            ok = got == want
            print(f"{'ok  ' if ok else 'FAIL'} {name}: {got} failure(s), wanted {want}")
            bad += 0 if ok else 1
            shutil.copyfile(source / target, root / target)
            shutil.copyfile(source / REFERENCE.name / reference, root / REFERENCE.name / reference)
    print(f"{len(cases)} cases, {bad} failures")
    return 1 if bad else 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--root", default=str(REPO), help="repository root to scan")
    ap.add_argument("--selftest", action="store_true", help="run this gate's own cases")
    ap.add_argument("--write", action="store_true",
                    help="rewrite docs/baseline/kept-files.tsv from the tree")
    args = ap.parse_args()
    if args.selftest:
        return selftest()
    root = Path(args.root)
    if args.write:
        return write_baseline(root)
    rows = read_baseline()
    failures = 0
    for reference, target in map_rows():
        if target not in rows:
            print(f"FAIL {target}: the map calls it Kept and the baseline has no row")
            failures += 1
            continue
        got_ref, ref_sha, removed, header = rows[target]
        if got_ref != reference:
            print(f"FAIL {target}: baseline says {got_ref}, the map says {reference}")
            failures += 1
            continue
        for message in check_one(root, target, reference, ref_sha, removed, header):
            print(f"FAIL {message}")
            failures += 1
    kept = {target for _, target in map_rows()}
    for target in sorted(set(rows) - kept):
        print(f"STALE {target}: a baseline row for a file the map does not call Kept")
        failures += 1
    print(f"{len(kept)} kept file(s), {len(rows)} baseline row(s), {failures} failure(s)")
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
