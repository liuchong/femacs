#!/usr/bin/env python3
"""Turn a unit sweep's unresolved names into the `include!` lines that would supply them.

The Android arm is where the port's `#[cfg (feature = …)]` arms compile for the first time
(`docs/baseline/config/darwin-arm64.json` carries no `features` section, so the darwin build
sets almost no features and the window-system arms are dead there).  An arm that resolves
compiles; an arm that names something no edge in the unit's graph supplies reads

    error[E0425]: cannot find type `frame` in this scope
    error[E0432]: unresolved imports `dispextern_unit_names::get_overlay_p`

`tools/check-all-units.sh` reports the first line of each such unit.  This tool reads that
sweep's `FEMACS_CHECK_VERBOSE=1` output and answers, per unit, which mirror (or unit) already
declares each missing name - the input an edge-completion pass needs.  It reads a log; it does
not compile anything and it edits nothing.

    FEMACS_CHECK_VERBOSE=1 tools/check-all-units.sh /tmp/units.txt > /tmp/sweep.log 2>&1
    python3 tools/suggest-unit-includes.py /tmp/sweep.log              # per-unit suggestions
    python3 tools/suggest-unit-includes.py /tmp/sweep.log --no-home    # names nothing declares
"""

from __future__ import annotations

import argparse
import collections
import pathlib
import re
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
PORT = REPO
MANGLE = "_Users_liu_projects_src_github_com_liuchong_femacs_"

# A name's *home* is the file that would have to be in scope for it to resolve.  The order of the
# classes is the preference: a type or a macro must come from a mirror that declares it, and a
# `pub fn` at a mirror's top level is a declaration of a C `extern`, i.e. the same thing a unit
# gets from its own text.
CLASSES = (
    ("struct", re.compile(r"^[ \t]*pub struct ([A-Za-z_][A-Za-z0-9_]*)", re.M)),
    ("enum", re.compile(r"^[ \t]*pub enum ([A-Za-z_][A-Za-z0-9_]*)", re.M)),
    ("type", re.compile(r"^[ \t]*pub type ([A-Za-z_][A-Za-z0-9_]*)", re.M)),
    ("const", re.compile(r"^[ \t]*pub const ([A-Za-z_][A-Za-z0-9_]*)", re.M)),
    ("static", re.compile(r"^[ \t]*pub static mut ([A-Za-z_][A-Za-z0-9_]*)", re.M)),
    ("macro", re.compile(r"^[ \t]*macro_rules! ([A-Za-z_][A-Za-z0-9_]*)", re.M)),
    ("extern", re.compile(r"^[ \t]*pub fn ([A-Za-z_][A-Za-z0-9_]*) *\(", re.M)),
    ("fn", re.compile(r"^[ \t]*pub (?:unsafe )?(?:extern \"C\" )?fn ([A-Za-z_][A-Za-z0-9_]*)",
                      re.M)),
)
ERROR = re.compile(r"^  \| error(?:\[([A-Z0-9]+)\])?: (.*)$")


def scan() -> tuple[dict[str, list[str]], dict[str, str]]:
    homes: dict[str, list[str]] = collections.defaultdict(list)
    seen: dict[str, str] = {}
    for path in sorted((p for tree in ('src','lib','lib-src','lwlib','oldXMenu','nt','msdos','exec','platform','xtask') for p in (PORT/tree).rglob("*.rs"))):
        if "target" in path.parts:
            continue
        text = path.read_text(encoding="utf-8", errors="surrogateescape")
        rel = path.relative_to(REPO).as_posix()
        for kind, rx in CLASSES:
            for name in rx.findall(text):
                if name not in seen:
                    seen[name] = kind
                if rel not in homes[name]:
                    homes[name].append(rel)
    return homes, seen


def unit_of(blob: str) -> str:
    """`build/femacs/units/<stem>/<mangled>.rs` -> the mirror a span belongs to."""
    if MANGLE not in blob:
        return ""
    tail = blob.split(MANGLE, 1)[1]
    if tail.startswith("build"):
        return ""
    return "" + tail.replace("_rs.rs", ".rs").replace("_", "/")


def read_log(path: pathlib.Path) -> dict[str, list[tuple[str, str, str]]]:
    """unit -> [(code, name, span file)]."""
    out: dict[str, list[tuple[str, str, str]]] = collections.defaultdict(list)
    unit = ""
    lines = path.read_text(encoding="utf-8", errors="surrogateescape").splitlines()
    for i, line in enumerate(lines):
        if line.startswith("FAIL "):
            unit = line.split()[1].rstrip(":")
        m = ERROR.match(line)
        if not m or not unit:
            continue
        code, msg = m.group(1) or "-", m.group(2)
        names = re.findall(r"`([^`]+)`", msg)
        if not names or "failed for" in msg or msg.startswith("aborting"):
            continue
        span = ""
        for probe in lines[i + 1:i + 8]:
            hit = re.search(r"([A-Za-z0-9_/.\-]+\.rs):(\d+):", probe)
            if hit:
                span = unit_of(hit.group(1)) or hit.group(1).rsplit("", 1)[-1]
                break
        for name in names:
            out[unit].append((code, name.split("::")[-1], span))
    return out


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("log", type=pathlib.Path)
    ap.add_argument("--no-home", action="store_true",
                    help="only names no port file declares (real porting gaps)")
    args = ap.parse_args()

    homes, _ = scan()
    per_unit = read_log(args.log)
    needed = sorted({name for rows in per_unit.values() for _, name, _ in rows})
    missing = [n for n in needed if n not in homes]

    if args.no_home:
        for name in missing:
            units = sorted({u for u, rows in per_unit.items() if any(n == name for _, n, _ in rows)})
            print(f"{name}\t{' '.join(units)}")
        print(f"{len(missing)} of {len(needed)} name(s) have no port declaration")
        return 0

    for unit in sorted(per_unit):
        rows = per_unit[unit]
        want: dict[str, list[str]] = collections.defaultdict(list)
        unknown: dict[str, str] = {}
        for code, name, span in rows:
            found = homes.get(name)
            if not found:
                unknown.setdefault(name, span or "-")
                continue
            mirrors = [h for h in found if "/include/" in h]
            pick = (mirrors or found)[0]
            want[pick].append(f"{name}{' (from ' + span.split('/')[-1] + ')' if span else ''}")
        print(f"== {unit}")
        for home, names in sorted(want.items()):
            short = home.split("")[-1]
            print(f'include!("{short}");   // ' + ", ".join(sorted(names)[:6]))
        for name, span in sorted(unknown.items()):
            print(f"NO HOME: {name}   (used at {span})")
    print(f"{len(per_unit)} unit(s), {len(needed)} name(s); "
          f"{len(missing)} have no port declaration")
    return 0


if __name__ == "__main__":
    sys.exit(main())
