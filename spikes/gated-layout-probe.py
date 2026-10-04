#!/usr/bin/env python3
"""Read a C type's layout in the frozen configuration *and* in a gated arm.

The port mirrors the frozen configuration against clang's own readings
(`-Xclang -fdump-record-layouts`; PORTING.md P-80 requires that proof for bitfields, and
`spikes/compare_layout.py` compares the two sides).  The *gated* arms - the ones
`HAVE_WINDOW_SYSTEM`, `HAVE_NS`, `NS_IMPL_COCOA` switch on - never had a reading:
docs/status.md 3.264 ⑥ and 3.257 ② record them as "a wall no compile checks", because the
port's gated Rust cannot be compiled before its platform mirrors exist.

That wall is on the *Rust* side only.  The C side reads fine, because the frozen
generated `config.h` parses when the feature macros configure would emit are passed on
the command line, and the reference's own headers are configuration-independent: this
probe asks clang for one type's record layout in the arms below and prints what it says.

    spikes/gated-layout-probe.py 'struct frame' frame.h
    spikes/gated-layout-probe.py 'struct ns_display_info' nsterm.h objc
    spikes/gated-layout-probe.py 'struct frame' frame.h --write docs/baseline/frame-layout.tsv

Arms: `frozen` (the baseline's own configuration), `ns_cocoa` (what `configure
--with-ns` emits on macOS), and any second arm `--define` builds.  A type that the
frozen configuration only forward-declares (`struct ns_display_info`, every NS type)
reads `parse=no` in that arm and is complete in the gated one, so the TSV keeps a
`parse` row per arm and only an all-arms failure is an error.  A window-system arm
needs a window system: `HAVE_WINDOW_SYSTEM` alone leaves `dispextern.h` without
`Emacs_Cursor`/`Display_Info` and does not parse (measured), which is why the arms are
named configurations rather than single macros.

Nothing is written into `emacs/`: the probe translation unit lives in a temporary
directory and the reference tree is read through `-I` only.  An arm that does not parse
is recorded as `parse=no` (with its errors printed) rather than silently dropped, and the
frozen arm failing is an error - the baseline of every comparison is that arm.
"""

from __future__ import annotations

import argparse
import re
import subprocess
import sys
import tempfile
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
REFERENCE = REPO / "emacs"
BUILD = REPO / "build" / "emacs-ref"
ARMS = (
    ("frozen", ()),
    ("ns_cocoa", ("-DHAVE_WINDOW_SYSTEM=1", "-DHAVE_NS=1", "-DNS_IMPL_COCOA=1")),
)


def include_flags(lang: str) -> list[str]:
    """The frozen configuration's generated headers first, then the reference's sources."""
    out: list[str] = []
    for d in (BUILD / "src", BUILD / "lib", REFERENCE / "src", REFERENCE / "lib"):
        out += ["-I", str(d)]
    if lang == "objc":
        out += ["-I", str(REFERENCE / "nextstep")]
    return out


def run_clang(type_name: str, header: str, lang: str, defs: tuple[str, ...]) -> tuple[str, str]:
    """`(layout-or-empty, errors)` for one arm."""
    src = (f'#include <config.h>\n#include "{header}"\nint probe_size (void);\n'
           f"int probe_size (void) {{ return (int) sizeof ({type_name}); }}\n")
    with tempfile.TemporaryDirectory(prefix="gated-layout-") as tmp:
        unit = Path(tmp) / ("probe.m" if lang == "objc" else "probe.c")
        unit.write_text(src, encoding="utf-8")
        cmd = ["clang", "-std=gnu23"]
        if lang == "objc":
            cmd += ["-x", "objective-c"]
        # `-complete` prints every complete record, not only the ones this translation unit
        # forces a layout for: `sizeof` alone does not make clang dump a record (measured on
        # `struct ns_display_info`), and a *type* query has no other reason to mention it.
        cmd += ["-fsyntax-only", "-Xclang", "-fdump-record-layouts-complete"]
        cmd += include_flags(lang) + list(defs) + [str(unit)]
        r = subprocess.run(cmd, capture_output=True, text=True)
        out = r.stdout + r.stderr
    errors = [line for line in out.splitlines() if re.search(r"error:", line)]
    want = re.escape(type_name)
    m = re.search(rf"^ +\d+ \| {want}$.*?^ +\| \[sizeof=\d+, align=\d+\]", out, re.S | re.M)
    return (m.group(0) if m else ""), "\n".join(errors)


def rows(type_name: str, arm: str, block: str) -> list[tuple[str, str, str]]:
    """`(arm, key, value)` for sizeof/align and every member line of one arm's dump."""
    out: list[tuple[str, str, str]] = []
    m = re.search(r"\[sizeof=(\d+), align=(\d+)\]", block)
    if m:
        out += [(arm, f"sizeof {type_name}", m.group(1)),
                (arm, f"align {type_name}", m.group(2))]
    for line in block.splitlines():
        mm = re.match(r"^\s*([\d:+-]+)\s*\|\s+(\S.*?)\s*$", line)
        if mm and mm.group(2) != type_name:
            out.append((arm, mm.group(2), mm.group(1)))
    return out


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("type_name")
    ap.add_argument("header")
    ap.add_argument("lang", nargs="?", default="c", choices=("c", "objc"))
    ap.add_argument("--define", action="append", default=[], metavar="MACRO[=VALUE]",
                    help="add a third arm built from the frozen configuration plus these")
    ap.add_argument("--write", metavar="FILE", help="write the readings as a TSV")
    args = ap.parse_args()
    if not (BUILD / "src" / "config.h").exists():
        print(f"missing {BUILD / 'src' / 'config.h'}: build the reference configuration first",
              file=sys.stderr)
        return 1
    arms = list(ARMS)
    if args.define:
        arms.append(("custom", tuple(f"-D{d}" for d in args.define)))
    table: list[tuple[str, str, str]] = []
    any_ok = False
    for arm, defs in arms:
        block, errors = run_clang(args.type_name, args.header, args.lang, defs)
        print(f"## {arm}  {' '.join(defs)}".rstrip())
        if block:
            print(block)
            table += rows(args.type_name, arm, block)
            any_ok = True
        else:
            table += [(arm, "parse", "no")]
            print(f"unavailable: {len(errors.splitlines())} error(s)")
            for line in errors.splitlines()[:4]:
                print(f"  {line.strip()}")
        print()
    if args.write:
        if not any_ok:
            print("refusing to write: no arm parsed", file=sys.stderr)
            return 1
        lines = [f"# {args.type_name} from {args.header} ({args.lang}), read by",
                 f"# `spikes/gated-layout-probe.py {args.type_name} {args.header} {args.lang}`:",
                 "# the frozen build's generated config.h, plus the gated macros configure would",
                 "# emit, on the command line.  Columns: arm\\tmember\\tbyte-offset-or-byte:bits.",
                 "# `parse=no` marks an arm this machine cannot read, with the reason in the run.",
                 "arm\tmember\toffset-or-bits"]
        lines += [f"{a}\t{m}\t{v}" for a, m, v in table]
        Path(args.write).write_text("\n".join(lines) + "\n", encoding="utf-8")
        print(f"wrote {args.write}: {len(table)} row(s)")
    return 0 if any_ok else 1


if __name__ == "__main__":
    raise SystemExit(main())
