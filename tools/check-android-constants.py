#!/usr/bin/env python3
"""Compare the port's header-mirror constants against the *delivery target's* headers.

Three defects of one class have already reached the device through this gap:

  * `RENAME_EXCL` and `_CS_PATH` - macros bionic does not have at all (status 4.749);
  * `O_CLOEXEC` - present on both platforms with *different* values (Darwin 0x1000000,
    bionic 02000000), which made `pipe2` fail with EINVAL and took C's
    `if (pipe2 (...) < 0) emacs_abort ()` arm (status 4.780).

`docs/baseline/config/*.json` cannot express this: those entries say whether a macro is
defined, not what it expands to.  This tool asks the target's own preprocessor for every
numeric constant the mirrors declare and reports the ones whose value differs, so the
"freeze one platform's header and ship it everywhere" mistake is caught by a reading instead
of by a tombstone.

    python3 tools/check-android-constants.py [--ndk CC]

Exit status is 0 when every comparable constant agrees, 1 when any differs.
"""

from __future__ import annotations

import argparse
import glob
import pathlib
import re
import subprocess
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
HEADERS = ["fcntl.h", "unistd.h", "signal.h", "errno.h", "stdio.h", "string.h", "stdlib.h",
           "sys/stat.h", "sys/types.h", "sys/socket.h", "sys/ioctl.h", "sys/mman.h",
           "sys/wait.h", "sys/resource.h", "sys/time.h", "time.h", "dirent.h", "termios.h",
           "pwd.h", "grp.h", "limits.h", "poll.h"]
CONST = re.compile(r"pub const (\w+): (?:c_int|c_long|c_uint|c_ulong) = ([^;]+);")
TYPES = ("c_int", "c_long", "c_uint", "c_ulong")


def find_cc(explicit: str | None) -> str:
    if explicit:
        return explicit
    found = sorted(glob.glob(str(pathlib.Path.home() /
                                "Library/Android/sdk/ndk/*/toolchains/llvm/prebuilt/*/bin/clang")))
    if not found:
        sys.exit("check-android-constants: no NDK clang found; pass --ndk CC")
    return found[-1]


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--ndk", default=None)
    a = ap.parse_args()
    cc = find_cc(a.ndk)
    prelude = "\n".join(f"#include <{h}>" for h in HEADERS) + "\n"

    def expand(expr: str) -> int | None:
        out = subprocess.run([cc, "--target=aarch64-linux-android24", "-E", "-P", "-x", "c", "-"],
                             input=prelude + expr + "\n", capture_output=True, text=True,
                             errors="replace").stdout
        # Only a name whose whole expansion is one integer can be compared: anything else is
        # either not a macro at all (the identifier survives, `no_argument`) or expands to an
        # expression this tool does not evaluate.  The earlier "take the last numeric token"
        # form read unrelated numbers out of the header text and reported 1,149 false
        # mismatches (status 4.781).
        lines = [l.strip() for l in out.splitlines() if not l.startswith("#") and l.strip()]
        if not lines:
            return None
        last = lines[-1]
        if not re.fullmatch(r"[-+]?(0[xX][0-9a-fA-F]+|\d+)", last):
            return None
        try:
            return int(last, 0)
        except ValueError:
            return None

    consts: dict[str, set[tuple[str, str]]] = {}
    for path in (sorted(glob.glob(str(REPO / "lib/include/*.rs"))) +
                 sorted(glob.glob(str(REPO / "src/include/*.rs")))):
        for m in CONST.finditer(pathlib.Path(path).read_text(errors="replace")):
            consts.setdefault(m.group(1), set()).add(
                (pathlib.Path(path).name, m.group(2).strip()))

    compared = 0
    bad: list[tuple[str, int, int, str, str]] = []
    for name, sites in sorted(consts.items()):
        # A name declared more than once is a hand-written platform pair (`#[cfg (feature =
        # "have_android")]` beside the frozen arm); the tool cannot see which arm the target
        # compiles, so it leaves those to the cfg and reports only the single-declaration
        # names - the class `O_CLOEXEC` was in before it got its pair.
        if len(sites) != 1:
            continue
        target = expand(name)
        if target is None:
            continue
        compared += 1
        for site, expr in sorted(sites):
            try:
                port = int(eval(expr, {"__builtins__": {}}, {}))  # noqa: S307 - our own text
            except Exception:
                continue
            if port != target:
                bad.append((name, port, target, site, expr))

    for name, port, target, site, expr in bad:
        print(f"{name:28s} port={port:<12} target={target:<12} {site}  ({expr})")
    print(f"check-android-constants: {compared} constant(s) compared, {len(bad)} mismatch(es)")
    return 1 if bad else 0


if __name__ == "__main__":
    raise SystemExit(main())
