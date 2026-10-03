#!/usr/bin/env python3
"""Fail when a C file-scope object the reference exports has no symbol in the port's object.

`search.c`'s lane found the hole by hand (docs/status.md 3.60): `re_match_object` was written
without `#[unsafe(no_mangle)]`, so the unit emitted `__RNv…_6search15re_match_object` where C
emits `_re_match_object`, and **every** gate stayed green — `extract-abi.py`'s object rule
compares *names* in the text, `check-symbols.py` compares export tables only for the units whose
reference object exists (79 of them, all in `lib/`), and `check-undef.py` sees only what a unit
*references*, never what it fails to *export*.

This tool closes the one direction that needs no reference object: for every built unit, every
file-scope object `cinventory` finds in the C source and that C does *not* mark `static` must
appear as an exported symbol of the port's object.  (The other direction — a symbol the port
exports that C does not — is `check-symbols.py`'s job where a reference object exists, and
`tools/check-symbols.py`'s DEFVAR-carrier rule documents the systematic exception.)

`DEFVAR_*` storage is not in `cinventory`'s object list (C keeps it inside `globals`), so the
§3.29 deviation does not reach this check.
"""

from __future__ import annotations

import argparse
import glob
import os
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
sys.path.insert(0, str(HERE))
import re  # noqa: E402
import cinventory  # noqa: E402  (the same scanner every other gate uses)
import importlib.util  # noqa: E402

# `tools/extract-abi.py` cannot be imported by name (a hyphen), but its module scope is what
# carries the frozen `#if` evaluator, so it is loaded from its path.
_spec = importlib.util.spec_from_file_location("extract_abi", HERE / "extract-abi.py")
extract_abi = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(extract_abi)

# The same exemption the other gates use (docs/status.md 3.31): an `src/*.c` the frozen build's
# source list excludes, ported as the empty shape (no `fn` at all - the note file), has nothing
# to export and is skipped.
FROZEN = set()
_sources = ROOT / "docs" / "baseline" / "frozen-build-sources.txt"
if _sources.is_file():
    FROZEN = {l.strip() for l in _sources.read_text(encoding="utf-8").splitlines()
              if l.strip() and not l.startswith("#")}


def empty_unbuilt(c_path: Path, dir_name: str, rs_path: Path) -> bool:
    if dir_name != "src" or f"{dir_name}/{c_path.name}" in FROZEN:
        return False
    return not rs_path.is_file() or not re.search(r"\bfn\s+\w+",
                                                   rs_path.read_text(encoding="utf-8",
                                                                     errors="replace"))


def frozen_conditions() -> None:
    """Load the same condition machinery every other gate uses."""
    verified = [l.strip() for l in (ROOT / "docs/baseline/config/cond-eval-verified.txt")
                .read_text(encoding="utf-8").split("\n") if l.strip() and not l.startswith("#")]
    extract_abi.ci.set_condition_paths(verified, reference="emacs")
    extract_abi.ci.set_condition_macros(extract_abi.ci.read_condition_macros(
        ROOT / "docs/baseline/config/darwin-arm64.macros.tsv"))
    return set(verified)


VERIFIED: set[str] = set()


def want_objects(c_path: Path, rel: str) -> set[str]:
    """The `_name` symbols C's frozen object is expected to export for this file's objects.

    Three filters, each of which had to be measured rather than guessed (status 3.61):
    `static` objects are not exported; an `extern` declaration is not a definition (cinventory
    lists both); and a line the frozen configuration does not compile defines nothing.  Names
    are still the *source* spelling - configure-time renames (`#define optarg rpl_optarg`) are
    invisible here, which is why a unit whose reference object exists is left to
    check-symbols.py instead.
    """
    text = c_path.read_text(encoding="utf-8", errors="replace")
    lines = text.split("\n")
    live = extract_abi.ci.dead_branch_map(text) if rel in VERIFIED else None
    if live is not None and len(live) != len(lines):
        live = None
    out = set()
    for g in cinventory.inventory(c_path).get("globals", []):
        if g.get("static"):
            continue
        line_no = g.get("line") or 0
        source = lines[line_no - 1] if 0 < line_no <= len(lines) else ""
        if "extern" in source:
            continue
        if live is not None and not live[line_no - 1]:
            continue
        out.add("_" + g["name"])
    return out


def exported(path: str) -> set[str]:
    out = subprocess.run(["nm", "-g", path], capture_output=True, text=True).stdout
    names = set()
    for line in out.splitlines():
        parts = line.split()
        if len(parts) >= 2 and parts[-2] in ("T", "D", "B", "S", "C", "R"):
            names.add(parts[-1])
    return names


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--dirs", default="src,lib",
                    help="reference directories whose units are checked, in order")
    ap.add_argument("--quiet", action="store_true")
    a = ap.parse_args()

    global VERIFIED
    VERIFIED = frozen_conditions()
    missing: list[tuple[str, list[str]]] = []
    verified: list[tuple[str, list[str]]] = []
    unverified: list[tuple[str, list[str]]] = []
    checked = 0
    for dir_name in a.dirs.split(","):
        for obj in sorted(glob.glob(f"{ROOT}/build/femacs/*.o")):
            stem = os.path.basename(obj)[:-2]
            # A mirror's object does not exist; a unit's object is named after its source, and
            # the two directories are searched in order so `src/` wins for a shared stem.
            c_path = ROOT / "emacs" / dir_name / f"{stem}.c"
            if not c_path.is_file():
                continue
            if empty_unbuilt(c_path, dir_name, ROOT / "femacs" / dir_name / f"{stem}.rs"):
                continue
            # A `src/*.c` the frozen build does not compile is not part of the link, so its
            # objects answer to nobody (docs/status.md 3.31): `tparam.c` - not in the source
            # list - defines `BC`/`UP` in C, while the frozen build takes them from
            # `terminfo.c`, which is the unit the port defines them in.
            if dir_name == "src" and f"src/{c_path.name}" not in FROZEN:
                continue
            rel = f"{dir_name}/{c_path.name}"
            want = want_objects(c_path, rel)
            if not want:
                continue
            if (ROOT / "build" / "emacs-ref" / dir_name / f"{stem}.o").is_file():
                continue          # check-symbols.py compares the whole table for these
            checked += 1
            have = exported(obj)
            # A configure-time rename is invisible to a source scan (`#define optarg
            # rpl_optarg` lives in the generated config), so a symbol that *ends* with the C
            # name counts as present: `_rpl_re_syntax_options` satisfies `re_syntax_options`.
            # The names are long and distinctive, so the relaxation does not hide a miss.
            gap = sorted(n for n in want if not any(s.endswith(n) for s in have))
            if gap:
                (verified if rel in VERIFIED else unverified).append(
                    (f"{dir_name}/{stem}", gap))
    for unit, gap in verified + unverified:
        print(f"{unit}: the reference defines these non-static objects, the port exports none: "
              f"{gap}")
    for unit, gap in unverified:
        print(f"  ^ {unit}.c is outside docs/baseline/config/cond-eval-verified.txt, so its "
              f"`#if`s are unknown here and the finding needs triage (a dead arm's object is "
              f"not a defect - ccl.c's two are inside `#ifdef CCL_DEBUG`).")
    if not a.quiet:
        total = sum(len(g) for _, g in verified)
        print(f"check-linkage: {checked} unit object(s) compared against the non-static objects "
              f"of the C sources the frozen build links, {len(verified)} with {total} missing "
              f"symbol(s)" +
              (f", {len(unverified)} whose conditions are unverified" if unverified else ""))
    return 1 if verified else 0


if __name__ == "__main__":
    raise SystemExit(main())
