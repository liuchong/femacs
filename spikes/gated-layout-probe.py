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

Arms: `frozen` (the baseline's own configuration), `x11` (`HAVE_WINDOW_SYSTEM` +
`HAVE_X_WINDOWS`, through the machine's own X11 headers), `x11_ext` (the same plus the
X11 extension macros this machine has headers for - `HAVE_XSYNC`, `HAVE_X_I18N`; see the
arm's comment for the four whose headers are absent), `ns_cocoa` (what `configure
--with-ns` emits on macOS), and any further arm `--define` builds (`--define` builds it
from the *frozen* configuration plus those macros, so it is not the way to reach an
X11-extension arm).  A type that the
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

**How to read `parse=no`** (3.318, after the xterm.h tables): for a header that includes
another target's header unconditionally, an arm can fail for a reason that has nothing to do
with the *type* - `xterm.h`'s line 23 is `#include <X11/Xlib.h>`, and `-I <X11>` is given only
to an arm whose defines contain `HAVE_X_WINDOWS`, so the `frozen` and `ns_cocoa` arms both
report `fatal error: 'X11/Xlib.h' file not found` and a `parse=no` row.  That is *not*
"this configuration has no `struct x_display_info`": the NS configuration has one, in
`nsterm.h`, and `femacs/src/include/nsterm.rs` mirrors it.  The reason is in the run's own
`unavailable: N error(s)` block; the TSV keeps `parse=no` as its machine-readable half.
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
    # The two window systems this machine can parse: X11 through Homebrew's headers (measured
    # present), and Cocoa through the SDK.  A shape is a *configuration*: HAVE_WINDOW_SYSTEM
    # alone leaves dispextern.h without Emacs_Cursor/Display_Info and parses in neither arm.
    ("x11", ("-DHAVE_WINDOW_SYSTEM=1", "-DHAVE_X_WINDOWS=1")),
    ("ns_cocoa", ("-DHAVE_WINDOW_SYSTEM=1", "-DHAVE_NS=1", "-DNS_IMPL_COCOA=1")),
    # X11's *optional* libraries change the same records again (`struct x_display_info`'s
    # HAVE_XFIXES / HAVE_XRANDR / HAVE_XINPUT2 / HAVE_XSYNC / HAVE_XINERAMA members, xterm.h
    # 406/496/700).  This arm turns on as many as this machine's headers allow, measured 3.309:
    # `X11/Xlib.h`, `X11/Xlocale.h` and `X11/extensions/{Xext.h,sync.h}` are present, while
    # `X11/extensions/{Xfixes.h,Xrandr.h,XInput2.h,Xinerama.h}` are not - so HAVE_XSYNC (which
    # needs sync.h + Xext.h) and HAVE_X_I18N (Xlocale.h) can be read here and the other four
    # macros cannot, which is a limit of the machine rather than a choice (the frozen tree does
    # not build xterm.h at all: xterm.h's line 23 includes <X11/Xlib.h> unconditionally).
    ("x11_ext", ("-DHAVE_WINDOW_SYSTEM=1", "-DHAVE_X_WINDOWS=1",
                 "-DHAVE_XSYNC=1", "-DHAVE_X_I18N=1")),
)
# X11's headers are not part of the frozen configuration; they come from the machine
# (`/opt/homebrew/include/X11/Xlib.h` was measured here).  Absent, the x11 arm reports
# `parse=no` and the other arms still read.
X11_INCLUDE = Path("/opt/homebrew/include")


def include_flags(lang: str, defs: tuple[str, ...]) -> list[str]:
    """The frozen configuration's generated headers first, then the reference's sources.

    The platform include paths come with the platform: `-I <X11>` only for an arm that defines
    `HAVE_X_WINDOWS`, `-I nextstep` only for an Objective-C arm.  Handing X11's headers to the
    frozen arm made `xterm.h` parse and *report a layout for a struct the frozen configuration
    never compiles* (measured 3.282: `struct x_display_info`, 207 members that a `have_x_windows`
    build reaches and the frozen one does not) - a reading that looks like a mirror gap and is an
    arm's own.
    """
    out: list[str] = []
    for d in (BUILD / "src", BUILD / "lib", REFERENCE / "src", REFERENCE / "lib"):
        out += ["-I", str(d)]
    if lang == "objc":
        out += ["-I", str(REFERENCE / "nextstep")]
    if any(d.startswith("-DHAVE_X_WINDOWS") for d in defs) and X11_INCLUDE.is_dir():
        out += ["-I", str(X11_INCLUDE)]
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
        cmd += include_flags(lang, defs) + list(defs) + [str(unit)]
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


def bit_rows(type_name: str, arm: str, block: str) -> list[tuple[str, str, str, str, str]]:
    """`(arm, member, unit, shift, width)` for every bitfield member of one arm.

    The derivation a mirror needs: C packs the flags into four-byte units, and a member's unit
    and shift are `(byte - base) // 4` and the bit within it.  Serialising the *units* rather
    than only the raw `byte:bits` is what makes a configuration change visible - the frozen arm
    has `no_special_glyphs` in unit 1 at bit 27 and both window-system arms have it in unit 2 at
    bit 12, because C inserts the window-system flags ahead of it.
    """
    bits = [line for line in block.splitlines() if re.match(r"^\s*\d+:\d+-\d+\s*\|", line)]
    if not bits:
        return []
    first = int(re.match(r"^\s*(\d+):", bits[0]).group(1))
    base = first - (first % 4)
    out: list[tuple[str, str, str, str, str]] = []
    for line in bits:
        m = re.match(r"^\s*(\d+):(\d+)-(\d+)\s*\|\s+(\S.*?)\s*$", line)
        byte, lo, hi, decl = int(m.group(1)), int(m.group(2)), int(m.group(3)), m.group(4)
        name = decl.split()[-1]
        unit = (byte - base) // 4 + 1
        shift = (byte - base - (unit - 1) * 4) * 8 + lo
        out.append((arm, name, str(unit), str(shift), str(hi - lo + 1)))
    return out


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("type_name")
    ap.add_argument("header")
    ap.add_argument("lang", nargs="?", default="c", choices=("c", "objc"))
    ap.add_argument("--bits", action="store_true",
                    help="derive each arm's (unit, shift, width) per flag member instead of "
                         "writing the raw byte:bits table")
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
    bit_table: list[tuple[str, str, str, str, str]] = []
    any_ok = False
    for arm, defs in arms:
        block, errors = run_clang(args.type_name, args.header, args.lang, defs)
        print(f"## {arm}  {' '.join(defs)}".rstrip())
        if block:
            print(block)
            table += rows(args.type_name, arm, block)
            bit_table += bit_rows(args.type_name, arm, block)
            any_ok = True
        else:
            table += [(arm, "parse", "no")]
            print(f"unavailable: {len(errors.splitlines())} error(s)")
            for line in errors.splitlines()[:4]:
                print(f"  {line.strip()}")
        print()
    if args.write and args.bits:
        if not bit_table:
            print("refusing to write: no arm had flag members", file=sys.stderr)
            return 1
        head = [f"# {args.type_name} from {args.header}: each arm's flag members as C packs them",
                "# (four-byte units; UNIT is 1-based, SHIFT is the bit within that unit).",
                "# Derived from `spikes/gated-layout-probe.py --bits`, whose raw byte:bits reading",
                "# is what the compiler itself printed - a member can move *unit* between",
                "# configurations, which is why the pair is what a mirror needs.",
                "arm\tmember\tunit\tshift\twidth"]
        Path(args.write).write_text("\n".join(head + ["\t".join(r) for r in bit_table]) + "\n",
                                    encoding="utf-8")
        print(f"wrote {args.write}: {len(bit_table)} row(s)")
        return 0
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
