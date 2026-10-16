#!/usr/bin/env python3
"""Verify that the verbatim trees are byte-identical to the reference checkout.

docs/port-map.md classifies six trees as `Verbatim` (`lisp`, `test`, `etc`, `doc`,
`leim`, `admin` -> `femacs/<top>/**`): the goal says they are *copied*, one byte at a
time, and that no test file may be modified, skipped or deleted.  Prose cannot prove
that, so this is the gate:

    python3 tools/check-verbatim.py            # all six trees
    python3 tools/check-verbatim.py lisp test  # only these
    python3 tools/check-verbatim.py --selftest # prove the readings fire

`docs/baseline/verbatim-absent.txt` lists the retired files (PORTING.md P-102), so those are
read as accepted absences rather than failures.  It compares the file *lists* (a missing file is
as much a failure as a changed one) and
then the contents, streamed in blocks so the four-and-a-half-million lines never sit in
memory.  Exit status 0 means exact equality; 1 lists what differs.

`docs/baseline/verbatim-substitutions.tsv` is the one way a line may differ.  Its subject
is what PORTING.md 0.1-19's tenth and eleventh halves are about: a *copied* tree may not be
rewritten, but (a) a string the reference program **prints** that names the reference product as
*this program's* own name, welcome, warranty line or generated-file identity, and (b) the
membership declaration every copied file opens with (`This file is part of <product>.`, whose
subject is *the file*, plus the three generators that write it into generated files and the
program's own buffer names), are this program's outward identity, so they carry this
repository's name (`status 3.613`'s criterion for the `.cc` bundle signature,
`carried-substitutions.tsv`'s shape; the eleventh half is `status 3.632`, the twelfth `status
# 3.641` - 1,815 sites).
`test/` is not among them - the goal text forbids modifying test files - and neither is a
`ChangeLog*`, which quotes the sentence as history.  Columns
`<repo-relative target path>\\t<line>\\t<the target's line>\\t<reason>`: every row is read
(the target's line `line` must be exactly the recorded text, the reference's line must
differ from it, and a row for a file the copy does not carry is a failure), and a
difference on any line with no row is a failure.  The default stays byte identity.  A line
whose text holds a tab or a backslash is written with `\t` / `\\` escapes, so a row is always
exactly four tab-separated columns.
"""

from __future__ import annotations

import argparse
import hashlib
import subprocess
import sys
from pathlib import Path

# docs/port-map.md's VERBATIM_DIRS: `<reference dir>` -> `femacs/<same name>`.
VERBATIM_DIRS = ("lisp", "test", "etc", "doc", "leim", "admin")
REPO = Path(__file__).resolve().parent.parent
REFERENCE = Path("emacs")
TARGET = Path("femacs")
BLOCK = 1 << 20
# The sanctioned identity lines (0.1-19's tenth half).  Hand-written, one row per line,
# each with the reason it cannot stay the reference's text; see the module docstring.
SUBSTITUTIONS = REPO / "docs/baseline/verbatim-substitutions.tsv"
# The accepted absences: files the reference tracks and this repository deliberately retired,
# one `<path>` TAB <reason> line each.  Their record is `docs/retired-scaffolding.md`
# (PORTING.md P-102); without the table an absence is a failure, with it a reading.
ABSENCES = REPO / "docs/baseline/verbatim-absent.txt"


def read_absences() -> dict[str, str]:
    out: dict[str, str] = {}
    if not ABSENCES.is_file():
        return out
    for line in ABSENCES.read_text(encoding="utf-8").splitlines():
        if not line or line.startswith("#"):
            continue
        path, _, reason = line.partition("\t")
        out[path] = reason
    return out


def files_under(repo: Path, prefix: str) -> dict[str, Path]:
    """The tree's **tracked** files, as `git` sees them, keyed by path relative to the tree.

    A build's generated outputs are not part of the copy.  C's own build writes `subdirs.el`, the
    `*loaddefs.el` files, ~1,600 `.elc`s and four `admin/` products *into the reference tree*, and
    the port's copies are of the reference's *source*: walking the filesystem (which is what this
    did until status 3.267) went red on the four `admin` products a rebuild recreated, and one of
    them, `charsets.stamp`, holds a timestamp - so it could never have stayed green across builds.
    `git ls-files` asks the question the rule actually means: does the port carry the reference's
    files, all of them, byte for byte.
    """
    r = subprocess.run(["git", "-C", str(repo), "ls-files", "-z", "--", prefix],
                       capture_output=True, text=True)
    out: dict[str, Path] = {}
    for line in r.stdout.split("\0"):
        if not line:
            continue
        rel = line[len(prefix) + 1:] if line.startswith(prefix + "/") else line
        out[rel] = repo / line
    return out


def digest(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as fh:
        while chunk := fh.read(BLOCK):
            h.update(chunk)
    return h.hexdigest()


def escape(text: str) -> str:
    """The table's one-line form: a backslash and a tab would break the four columns."""
    return text.replace("\\", "\\\\").replace("\t", "\\t")


def unescape(text: str) -> str:
    """Inverse of `escape` for the one row of a table that carries a tab (module docstring)."""
    out: list[str] = []
    i = 0
    while i < len(text):
        if text[i] == "\\" and i + 1 < len(text) and text[i + 1] in "\\t":
            out.append("\\" if text[i + 1] == "\\" else "\t")
            i += 2
            continue
        out.append(text[i])
        i += 1
    return "".join(out)


def read_substitutions() -> dict[str, list[tuple[int, str, str]]]:
    """`repo-relative target path -> [(line, the target's line, reason)]`."""
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
        rows.setdefault(parts[0], []).append((int(parts[1]), unescape(parts[2]), parts[3]))
    return rows


def line_readings(ref: Path, target: Path,
                  rows: list[tuple[int, str, str]]) -> tuple[list[str], int]:
    """The readings for one file that carries substitution rows: `(failures, sanctioned)`."""
    bad: list[str] = []
    ref_lines = ref.read_bytes().split(b"\n")
    target_lines = target.read_bytes().split(b"\n")
    allowed: set[int] = set()
    for line_no, target_text, _reason in rows:
        if line_no < 1 or line_no > len(target_lines) or line_no > len(ref_lines):
            bad.append(f"line {line_no} is outside the file ({len(target_lines)} lines)")
            continue
        got = target_lines[line_no - 1].decode("utf-8", "replace")
        if got != target_text:
            bad.append(f"line {line_no} is {got!r}, the table says {target_text!r}")
            continue
        if ref_lines[line_no - 1] == target_lines[line_no - 1]:
            bad.append(f"line {line_no} already matches the reference - the row is stale")
            continue
        allowed.add(line_no)
    if not bad and len(ref_lines) != len(target_lines):
        bad.append(f"{len(target_lines)} lines, the reference has {len(ref_lines)}")
    if bad:
        return bad, 0
    for i, (x, y) in enumerate(zip(ref_lines, target_lines)):
        if x != y and (i + 1) not in allowed:
            bad.append(f"line {i + 1} differs with no row: {y[:60]!r} != {x[:60]!r}")
            break
    return bad, len(allowed)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("trees", nargs="*", default=list(VERBATIM_DIRS),
                    help=f"trees to check (default: {' '.join(VERBATIM_DIRS)})")
    ap.add_argument("--selftest", action="store_true",
                    help="run the six readings against a temporary pair of files")
    args = ap.parse_args()
    if args.selftest:
        return selftest()

    subs = read_substitutions()
    absent = read_absences()
    used: set[str] = set()
    failures = 0
    total_sanctioned = 0
    for tree in args.trees:
        src, dst = REFERENCE / tree, TARGET / tree
        if not src.is_dir():
            print(f"MISSING {src}: the reference tree is not checked out here")
            failures += 1
            continue
        if not (src / ".git").exists() and not (REPO / ".git").exists():
            print(f"MISSING git in {src} or {REPO}: the file lists come from the repositories")
            failures += 1
            continue
        if not dst.is_dir():
            print(f"TODO    {tree}: {dst} has not been copied yet")
            failures += 1
            continue
        a = files_under(src if (src / ".git").is_dir() else src.parent, tree)
        b = files_under(REPO, f"{TARGET}/{tree}")
        missing = sorted(rel for rel in set(a) - set(b)
                         if f"{TARGET}/{tree}/{rel}" not in absent)
        extra = sorted(set(b) - set(a))
        changed: list[str] = []
        sanctioned = 0
        for rel in sorted(set(a) & set(b)):
            rows = subs.get(f"{TARGET}/{tree}/{rel}")
            if rows:
                used.add(f"{TARGET}/{tree}/{rel}")
                bad, n = line_readings(a[rel], b[rel], rows)
                changed += [f"{rel}: {m}" for m in bad]
                sanctioned += n
            elif a[rel].stat().st_size != b[rel].stat().st_size or digest(a[rel]) != digest(b[rel]):
                changed.append(rel)
        total_sanctioned += sanctioned
        if missing or extra or changed:
            failures += 1
            print(f"FAIL    {tree}: {len(missing)} missing, {len(extra)} extra, "
                  f"{len(changed)} changed")
            for rel in (missing + extra + changed)[:10]:
                print(f"          {rel}")
        else:
            note = f", {sanctioned} sanctioned line(s)" if sanctioned else ""
            print(f"OK      {tree}: {len(a)} files, byte-identical{note}")
    stale = sorted(set(subs) - used - set(absent))
    if stale:
        failures += 1
        print(f"FAIL    {SUBSTITUTIONS.name}: {len(stale)} row(s) for a file no tree carries")
        for path in stale[:10]:
            print(f"          {path}")
    total = sum(len(files_under(REFERENCE / t if (REFERENCE / t / ".git").is_dir()
                                else REFERENCE, t))
                for t in args.trees if (REFERENCE / t).is_dir())
    print(f"{len(args.trees)} trees, {total} files, {total_sanctioned} sanctioned line(s), "
          f"{len(absent)} accepted absence(s), {failures} failures")
    return 1 if failures else 0


def selftest() -> int:
    """Six readings with the fixed shape: the clean pair passes, each mutation fires once."""
    import tempfile

    # The fixture's product token is neutral on purpose: it is the *mechanism* that is under
    # test here, and a product name would make the declaration gate read this file (0.1-19).
    ref_text = "a\n;; This file is part of Elsewhere.\nlast\n"
    tgt_text = "a\n;; This file is part of femacs.\nlast\n"
    rows = [(2, ";; This file is part of femacs.",
             "selftest: the second line is the one row the table sanctions")]
    cases: list[tuple[str, str, str, list[tuple[int, str, str]], int]] = [
        ("identical pair", ref_text, ref_text, [], 0),
        ("one line flipped", ref_text, "a\nb\nlast\n", [], 1),
        ("the sanctioned line", ref_text, tgt_text, rows, 0),
        ("the reference's text under a row", ref_text, ref_text, rows, 1),
        ("a row whose text is wrong", ref_text, tgt_text,
         [(2, ";; This file is part of femacs!", rows[0][2])], 1),
        ("a row outside the file", ref_text, tgt_text,
         [(9, "x", rows[0][2])], 1),
        # The two lines of the table whose text starts with a tab (calc-keypd.el, forms.el) are
        # written with `\t`; the pair below is the reading that fails without the escape.
        ("a row whose text starts with a tab", "\ta\n", "\tb\n",
         [(1, "\tb", "selftest: the escape round-trip")], 0),
    ]
    assert unescape(escape("\ta\n")) == "\ta\n" and unescape(escape("a\\b")) == "a\\b" 
    bad = 0
    with tempfile.TemporaryDirectory() as tmp:
        root = Path(tmp)
        for name, a, b, r, want in cases:
            (root / "ref").write_text(a, encoding="utf-8")
            (root / "tgt").write_text(b, encoding="utf-8")
            got, n = line_readings(root / "ref", root / "tgt", r)
            ok = (len(got) == want) and (n == (1 if name.startswith("the sanctioned line")
                                                 or name == "a row whose text starts with a tab"
                                                 else 0))
            bad += 0 if ok else 1
            print(f"{'ok  ' if ok else 'FAIL'} {name}: {len(got)} failure(s), {n} sanctioned "
                  f"(want {want})")
    print(f"selftest: {len(cases)} case(s), {bad} failure(s)")
    return 1 if bad else 0


if __name__ == "__main__":
    raise SystemExit(main())
