#!/usr/bin/env python3
"""Pin the two classes whose target is the reference's own text, byte for byte.

`docs/port-map.md` names two classes the port does not rewrite:

    Kept    assembly and data files that are used as they are - the six
            `exec/loader-*.s`, "path preserved" (status 3.284);
    C++     code that must talk to a C++ API (Haiku) - the five `*.cc`, whose
            target moves to `platform/haiku/NAME.cc`.

Their target language *is* the reference's language or the file is data, so the port's
deliverable is the text itself - which is why they are also the two classes that kept the
reference's copied declaration block for a day longer than the rest of the tree:
`check-declarations.py` reads by suffix, and `.s` was not in its list (status 3.295) while
the `.cc` tree had no file in it yet (status 3.299).  Both blocks are gone; what this gate
keeps true is the part that has to stay: the body.

    python3 tools/check-carried.py              # scan, compare with the baseline
    python3 tools/check-carried.py --selftest   # prove the readings fire
    python3 tools/check-carried.py --write      # rewrite docs/baseline/carried-files.tsv

`docs/baseline/carried-substitutions.tsv` is the one way a body line may differ, and its
subject is *this program's outward identity*: the Haiku bundle signature is the name the OS
registers the running program under, and the reference's value names the reference product
(PORTING.md 0.1-19's ninth half, status 3.613).  Columns `<path>\t<line>\t<target line>\t<reason>`;
each row is read - the target's line `line` must be the recorded text - and a row for a file the
map does not carry, or a difference with no row, is a failure.  The default stays byte identity.

`docs/baseline/carried-files.tsv`, columns
`<class>\\t<path>\\t<reference>\\t<ref_sha256>\\t<removed_lines>\\t<header>`:
`ref_sha256` pins the reference's whole file, `removed_lines` is the number of reference
lines the origin line replaces (the blank line that followed the block is the body's first
line), and `header` is the exact first line the target carries.  Per row four readings: the
reference hashes to `ref_sha256`; the target's first line is `header`; `header` names the
reference path and a `<rev>`, in the phrase its class uses (`kept from` when the target
keeps the reference's path, `port of` when it moves); and the target's lines after the
header are byte-identical to the reference's lines after `removed_lines`.  The row set must
also be exactly the two sections of the map - a file the map calls Kept or C++ with no row,
and a row for a file the map does not, are both failures.  Exit status 0 means all of it
holds.
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
BASELINE = REPO / "docs/baseline/carried-files.tsv"
# The lines whose text is this program's own name rather than the reference's (0.1-19's
# ninth half).  Hand-written, with a reason per row; see the module docstring.
SUBSTITUTIONS = REPO / "docs/baseline/carried-substitutions.tsv"
# The two sections of the generated map this gate owns.
CLASSES = ("Kept", "C++")
# The origin line, in the file's own comment leader (`//` in the Swift/Kotlin and most C++
# targets, `#`/`@` in the assembly dialects).  The phrase says which class the file is.
PATH_RULE = re.compile(r"^(?://|#|@|;) femacs: (kept from|port of) (\S+) @ ([0-9a-f]{7,40})\.$")


def map_rows() -> list[tuple[str, str, str]]:
    """`(class, reference, target)` for the map's Kept and C++ sections."""
    out: list[tuple[str, str, str]] = []
    klass = ""
    for line in MAP.read_text(encoding="utf-8").splitlines():
        if line.startswith("## "):
            klass = next((c for c in CLASSES if line.startswith(f"## {c}")), "")
            continue
        if not klass or not line.startswith("| `"):
            continue
        cells = [c.strip().strip("`") for c in line.split("|")[1:-1]]
        if len(cells) >= 3 and cells[0] and cells[2] and not Path(cells[2]).is_absolute() and ".." not in Path(cells[2]).parts:
            out.append((klass, cells[0], cells[2]))
    return out


def read_baseline() -> dict[str, tuple[str, str, str, int, str]]:
    """`target -> (class, reference, ref_sha256, removed_lines, header)`."""
    rows: dict[str, tuple[str, str, str, int, str]] = {}
    for raw in BASELINE.read_text(encoding="utf-8").splitlines():
        if not raw.strip() or raw.startswith("#") or raw.startswith("class\t"):
            continue
        parts = raw.split("\t")
        if len(parts) < 6 or not parts[4].strip().isdigit():
            print(f"BAD ROW {BASELINE.name}: {raw!r}")
            continue
        rows[parts[1]] = (parts[0], parts[2], parts[3], int(parts[4]), parts[5])
    return rows


def read_substitutions() -> dict[str, list[tuple[int, str, str]]]:
    """`target -> [(line, target line, reason)]` for the sanctioned differences."""
    rows: dict[str, list[tuple[int, str, str]]] = {}
    if not SUBSTITUTIONS.is_file():
        return rows
    for raw in SUBSTITUTIONS.read_text(encoding="utf-8").splitlines():
        if not raw.strip() or raw.startswith("#") or raw.startswith("path\t"):
            continue
        parts = raw.split("\t")
        if len(parts) < 4 or not parts[1].strip().isdigit():
            print(f"BAD ROW {SUBSTITUTIONS.name}: {raw!r}")
            continue
        rows.setdefault(parts[0], []).append((int(parts[1]), parts[2], parts[3]))
    return rows


def phrase_of(klass: str, reference: str, target: str) -> str:
    """`kept from` when the path is preserved, `port of` when it moves."""
    return "kept from" if target == f"{reference}" else "port of"


def check_one(root: Path, klass: str, path: str, reference: str, ref_sha: str,
              removed: int, header: str,
              substitutions: dict[str, list[tuple[int, str, str]]] | None = None) -> list[str]:
    """The readings for one row, as failure messages (empty when it holds)."""
    bad: list[str] = []
    ref = root / REFERENCE.name / reference
    target = root / path
    if not ref.is_file():
        return [f"{reference}: reference file is missing"]
    if not target.is_file():
        return [f"{path}: carried file is missing"]
    ref_bytes = ref.read_bytes()
    got_sha = hashlib.sha256(ref_bytes).hexdigest()
    if ref_sha and got_sha != ref_sha:
        bad.append(f"{reference}: sha256 {got_sha[:16]}..., baseline says {ref_sha[:16]}...")
    want_phrase = phrase_of(klass, reference, path)
    m = PATH_RULE.match(header)
    if header and not m:
        bad.append(f"{path}: header {header!r} is not `\u2026 femacs: <phrase> \u2026 @ <rev>.`")
    elif header:
        if m.group(1) != want_phrase:
            bad.append(f"{path}: header says {m.group(1)!r}, a {klass} file says {want_phrase!r}")
        if m.group(2) != reference:
            bad.append(f"{path}: header names {m.group(2)}, the map says {reference}")
    ref_lines = ref_bytes.split(b"\n")
    target_lines = target.read_bytes().split(b"\n")
    if header and target_lines[0].decode("utf-8", "replace") != header:
        bad.append(f"{path}: first line is {target_lines[0]!r}, baseline says {header!r}")
    if removed < 0 or removed > len(ref_lines):
        return bad + [f"{path}: removed_lines {removed} is outside the reference"]
    subs = read_substitutions() if substitutions is None else substitutions
    allowed: set[int] = set()
    for line_no, target_text, _reason in subs.get(path, ()):
        i = line_no - 2                      # the body index of the target's line `line_no`
        if i < 0 or line_no > len(target_lines):
            bad.append(f"{path}: substitution line {line_no} is outside the file")
            continue
        got = target_lines[line_no - 1].decode("utf-8", "replace")
        if got != target_text:
            bad.append(f"{path}: substitution line {line_no} is {got!r}, "
                       f"baseline says {target_text!r}")
            continue
        allowed.add(i)
    body, want_body = target_lines[1:], ref_lines[removed:]
    diff = next(((i, a, b) for i, (a, b) in enumerate(zip(body, want_body))
                 if a != b and i not in allowed), None)
    if diff is not None:
        i, a, b = diff
        bad.append(f"{path}: body line {i + 1} differs from {reference} "
                   f"line {removed + i + 1}: {a[:60]!r} != {b[:60]!r}")
    elif len(body) != len(want_body):
        bad.append(f"{path}: body is {len(body)} lines, the reference's "
                   f"body from line {removed + 1} is {len(want_body)}")
    return bad


def write_baseline(root: Path) -> int:
    rows = map_rows()
    body = ["# The Kept and C++ classes of the generated port map (docs/port-map.md): the",
            "# reference's own text, body byte for byte, with the copied declaration block",
            "# replaced by one origin line (PORTING.md 0.1-19/P-84/P-85; status 3.295, 3.299).",
            "# Generated by `tools/check-carried.py --write`; checked by the same tool.  Columns:",
            "# <class>\\t<path>\\t<reference>\\t<ref_sha256>\\t<removed_lines>\\t<header>",
            "class\tpath\treference\tref_sha256\tremoved_lines\theader"]
    for klass, reference, target in rows:
        ref_bytes = (root / REFERENCE.name / reference).read_bytes()
        target_lines = (root / target).read_bytes().split(b"\n")
        removed = len(ref_bytes.split(b"\n")) - len(target_lines) + 1
        header = target_lines[0].decode("utf-8")
        if not PATH_RULE.match(header):
            print(f"REFUSED {target}: first line {header!r} is not a carried-file header")
            return 1
        body.append(f"{klass}\t{target}\t{reference}\t{hashlib.sha256(ref_bytes).hexdigest()}"
                    f"\t{removed}\t{header}")
    BASELINE.write_text("\n".join(body) + "\n", encoding="utf-8")
    print(f"wrote {BASELINE.relative_to(REPO)}: {len(rows)} row(s)")
    return 0


def selftest() -> int:
    """Three readings, each with the fixed shape: clean passes, each mutation fires."""
    bad = 0
    source = Path(__file__).resolve().parent.parent
    # One row of each class: the phrase rule is what the pair proves.
    rows = [next(r for r in map_rows() if r[0] == "Kept"),
            next(r for r in map_rows() if r[0] == "C++")]
    cases_run = 0
    for klass, reference, target in rows:
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

            def flip(r: Path) -> None:
                data = (r / target).read_bytes()
                (r / target).write_bytes(data[:-2] + bytes([data[-2] ^ 0x20]) + data[-1:])

            def unheader(r: Path) -> None:
                lines = (r / target).read_bytes().split(b"\n")
                (r / target).write_bytes(b"\n".join(lines[1:]))

            def rephrase(r: Path) -> None:
                lines = (r / target).read_bytes().split(b"\n")
                other = "port of" if klass == "Kept" else "kept from"
                lines[0] = lines[0].replace(phrase_of(klass, reference, target).encode(),
                                            other.encode(), 1)
                (r / target).write_bytes(b"\n".join(lines))

            # Removing the origin line fires twice, on purpose: the first line is no longer the
            # recorded header, *and* the body it left behind is shifted against the reference's.
            def unreplace(r: Path, line_no: int) -> None:
                """Put the reference's value back on a line a substitution row covers."""
                lines = (r / target).read_bytes().split(b"\n")
                lines[line_no - 1] = lines[line_no - 1].replace(b"femacs", b"GNU-emacs")
                (r / target).write_bytes(b"\n".join(lines))

            # The rows a file may carry (`carried-substitutions.tsv`).  They change two of the
            # expectations below: removing the origin line shifts the body, so every sanctioned
            # row stops matching as well (one reading each), and putting the reference's value
            # back leaves the body *equal* to the reference, so only the row fires.
            sub_rows = read_substitutions().get(target, ())
            cases = [("the carried file", lambda r: None, 0),
                     ("one body byte flipped", flip, 1),
                     ("the origin line removed", unheader, 2 + len(sub_rows)),
                     ("the phrase of the other class", rephrase, 1)]
            if sub_rows:
                cases.append(("the sanctioned line put back",
                              lambda r: unreplace(r, sub_rows[0][0]), 1))
            for name, mutate, want in cases:
                cases_run += 1
                mutate(root)
                got = len(check_one(root, klass, target, reference, ref_sha, removed, header))
                ok = got == want
                print(f"{'ok  ' if ok else 'FAIL'} {klass}: {name}: {got} failure(s), wanted {want}")
                bad += 0 if ok else 1
                shutil.copyfile(source / target, root / target)
                shutil.copyfile(source / REFERENCE.name / reference,
                                root / REFERENCE.name / reference)
    print(f"{cases_run} cases, {bad} failures")
    return 1 if bad else 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--root", default=str(REPO), help="repository root to scan")
    ap.add_argument("--selftest", action="store_true", help="run this gate's own cases")
    ap.add_argument("--write", action="store_true",
                    help="rewrite docs/baseline/carried-files.tsv from the tree")
    args = ap.parse_args()
    if args.selftest:
        return selftest()
    root = Path(args.root)
    if args.write:
        return write_baseline(root)
    rows = read_baseline()
    subs = read_substitutions()
    failures = 0
    wanted = map_rows()
    for klass, reference, target in wanted:
        if target not in rows:
            print(f"FAIL {target}: the map calls it {klass} and the baseline has no row")
            failures += 1
            continue
        got_class, got_ref, ref_sha, removed, header = rows[target]
        if got_ref != reference or got_class != klass:
            print(f"FAIL {target}: baseline says {got_class} {got_ref}, the map says {klass} {reference}")
            failures += 1
            continue
        for message in check_one(root, klass, target, reference, ref_sha, removed, header, subs):
            print(f"FAIL {message}")
            failures += 1
    carried = {target for _, _, target in wanted}
    for target in sorted(set(rows) - carried):
        print(f"STALE {target}: a baseline row for a file the map does not carry")
        failures += 1
    for target in sorted(set(subs) - carried):
        print(f"STALE {target}: a substitution row for a file the map does not carry")
        failures += 1
    n_subs = sum(len(v) for v in subs.values())
    print(f"{len(carried)} carried file(s), {len(rows)} baseline row(s), "
          f"{n_subs} sanctioned substitution(s), {failures} failure(s)")
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
