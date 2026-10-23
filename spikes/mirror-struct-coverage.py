#!/usr/bin/env python3
"""Which mirrored `struct`s are missing members, per configuration.

`src/include/*.rs` carries one mirror per C header, and each mirror's structs are written
against the *frozen* configuration's member list.  This tool checks that claim over every mirrored
struct at once: it finds the C header each struct comes from, asks `gated-layout-probe.py` for the
struct's layout in each arm, and reports the members that appear in an arm's reading but have no
`pub <member>: <type>` (or `pub r#<member>:`) in the mirror.

Two readings matter:

* **frozen-arm gaps** are defects - every member the frozen configuration has must be in the mirror
  (the layout gate asserts offsets, which catches a *moved* member but a missing one only if a
  later offset or the size moves).  Measured 3.282: **0** across 285 structs / 752 arm readings.
* **window-system-arm gaps** are the port's remaining work list for the gated surface.  They are
  not defects while the arm cannot be compiled, but each one is a member C declares and the port
  does not carry.

    spikes/mirror-struct-coverage.py                 # both readings, one line per gap
    spikes/mirror-struct-coverage.py --arm frozen    # only the defect reading

An arm that could not be read (`unavailable`, i.e. the configuration does not define the type or
the platform headers are absent) is reported as such, never as a gap.
"""

from __future__ import annotations

import argparse
import glob
import pathlib
import re
import subprocess
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
PROBE = REPO / "spikes" / "gated-layout-probe.py"
# A struct's header is the mirror's own stem when a C header of that name exists; these few names
# live in a header whose stem differs from the mirror's.
OVERRIDES = {"display": "dispextern.h", "xterm": "xterm.h", "nsterm": "nsterm.h"}
OBJC = {"nsterm", "nsgui", "nsfns", "macfont"}


def port_structs() -> dict[str, tuple[str, set[str]]]:
    """`struct name -> (mirror path, member names)` for every struct in every mirror."""
    out: dict[str, tuple[str, set[str]]] = {}
    for path in sorted(glob.glob(str(REPO / "src/include/*.rs"))):
        text = pathlib.Path(path).read_text(encoding="utf-8")
        for m in re.finditer(r"^pub struct ([a-z_0-9]+)\b.*?^}", text, re.S | re.M):
            body = m.group(0)
            # Member names are C's, so they can start with a capital (`Wcm`, `TS_ins_line`,
            # `Voverriding_terminal_local_map_`) - a lower-case-only pattern reported every
            # one of those as a gap (measured 3.282).
            fields = {g[1] for g in re.findall(r"^\s*pub (r#)?([A-Za-z_0-9]+)\s*:", body, re.M)}
            out[m.group(1)] = (path, fields)
    return out


def header_for(stem: str, pool: dict[str, str]) -> str | None:
    if stem in OVERRIDES:
        return OVERRIDES[stem]
    return pool.get(stem)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--arm", default=None, help="report only this arm (e.g. frozen)")
    args = ap.parse_args()
    pool = {pathlib.Path(h).stem: h for h in
            glob.glob(str(REPO / "emacs/src/*.h")) + glob.glob(str(REPO / "emacs/lib/*.h"))}
    structs = port_structs()
    arms = {}
    unreadable = attempted = 0
    for name, (path, fields) in sorted(structs.items()):
        stem = pathlib.Path(path).stem
        header = header_for(stem, pool)
        if not header:
            print(f"??      struct {name}: no C header guessed (mirror {stem})", file=sys.stderr)
            continue
        header_arg = pathlib.Path(header).name
        lang = "objc" if stem in OBJC else "c"
        r = subprocess.run(["python3", str(PROBE), f"struct {name}", header_arg, lang],
                           capture_output=True, text=True, cwd=REPO)
        for b in re.split(r"^## ", r.stdout, flags=re.M)[1:]:
            arm = b.split()[0]
            attempted += 1
            if "unavailable" in b:
                unreadable += 1
                continue
            for line in b.splitlines():
                m = re.match(r"^\s*(\d+)\s*\|\s{3}(\S.*?)\s*$", line)
                if not m or m.group(2).split()[-1] == name:
                    continue
                member = m.group(2).split()[-1]
                if member not in fields:
                    arms.setdefault(arm, []).append((name, member, header_arg))
    print(f"{len(structs)} mirrored struct(s): {attempted} arm(s) attempted, "
          f"{unreadable} unreadable (the configuration does not define the type, or the platform "
          f"headers are absent), {sum(len(v) for v in arms.values())} gap(s)")
    for arm in sorted(arms):
        if args.arm and arm != args.arm:
            continue
        gaps = arms[arm]
        names = sorted({n for n, _, _ in gaps})
        print(f"\n## {arm}: {len(gaps)} gap(s) over {len(names)} struct(s)")
        for n in names:
            sample = [m for s, m, _ in gaps if s == n]
            print(f"   struct {n:24} {len(sample):3} missing e.g. {sample[:4]}")
    if args.arm == "frozen":
        print(f"\nfrozen-arm gaps: {len(arms.get('frozen', []))}")
    return 1 if (arms.get("frozen") and not args.arm) or args.arm == "frozen" and arms.get("frozen") else 0


if __name__ == "__main__":
    raise SystemExit(main())
