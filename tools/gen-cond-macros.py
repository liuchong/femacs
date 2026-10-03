#!/usr/bin/env python3
"""Freeze the configuration macro table the extractor evaluates `#if`s against.

`tools/extract-abi.py` compares a mirror with the C text *the frozen configuration
compiles* (PORTING.md P-08), and for the display headers most of the file is inside
`#ifdef HAVE_WINDOW_SYSTEM` and friends: without the macro table it demanded the whole of
batch E/F before batch B's last file (P-66).  This writes that table:

    python3 tools/gen-cond-macros.py                     # uses build/emacs-ref/src/config.h
    python3 tools/gen-cond-macros.py --config PATH --out docs/baseline/config/darwin-arm64.macros.tsv

One `<name>\\t<value>` per line, where value is the `#define`'s body (possibly empty) or
`undef` for a commented-out `#undef`.  The file is generated from the same configure output
the other frozen baselines come from; regenerate it whenever that output is regenerated.
"""

from __future__ import annotations

import argparse
import re
import subprocess
from pathlib import Path


def read_macros(config: Path, include_dirs: list[str]) -> dict[str, str]:
    """Every macro the frozen configuration's preprocessor defines, plus config.h's `#undef`s.

    `clang -dM -E` is the only way to get the *whole* table: Emacs's `#if`s test library
    macros (`INTPTR_MAX`, `SIZE_MAX`, `__GNUC__`) and the platform's (`__APPLE__`) as well as
    config.h's, and an evaluator that guesses those would be wrong rather than blind —
    `USE_LSB_TAG (VAL_MAX / 2 < INTPTR_MAX)` (lisp.h 256) decides `GCALIGNMENT`, and with
    `INTPTR_MAX` unknown the gate demanded the wrong branch of lisp.h.
    """
    probe = Path("/tmp/femacs-cond-macros.c")
    # The Emacs headers test system macros too (`_STRUCT_TIMEVAL` from <sys/time.h>,
    # `__GLIBC__`, …), so the probe includes the C library and POSIX headers Emacs's own
    # headers pull in.
    probe.write_text(
        "#include <config.h>\n"
        "#include <stdint.h>\n#include <stddef.h>\n#include <limits.h>\n"
        "#include <float.h>\n#include <string.h>\n#include <stdlib.h>\n"
        "#include <sys/types.h>\n#include <sys/time.h>\n#include <sys/stat.h>\n"
        "#include <unistd.h>\n#include <signal.h>\n#include <setjmp.h>\n"
        "#include <time.h>\n#include <fcntl.h>\n"
        # The headers whose macros decide branches the port has hit so far: stdio's TMP_MAX,
        # locale/langinfo's CODESET, sysctl's HW_NCPU, the pwd/grp and dirent families, the
        # wchar/iconv pair, and the POSIX process/select headers.  Adding a header here can only
        # make the table know more; a file joins the verified list only when every one of its
        # conditions agrees with clang, so a wrong value shows up as a *disagreement* rather
        # than as a silently wrong branch (P-66).
        "#include <stdio.h>\n#include <errno.h>\n#include <locale.h>\n#include <langinfo.h>\n"
        "#include <wchar.h>\n#include <wctype.h>\n#include <iconv.h>\n#include <pwd.h>\n"
        "#include <grp.h>\n#include <dirent.h>\n#include <utime.h>\n#include <math.h>\n"
        "#include <pthread.h>\n#include <sys/param.h>\n#include <sys/mount.h>\n"
        "#include <sys/sysctl.h>\n#include <sys/select.h>\n#include <sys/wait.h>\n"
        "#include <sys/socket.h>\n#include <netinet/in.h>\n#include <sched.h>\n")
    cmd = ["clang", "-dM", "-E", "-std=gnu23", *include_dirs, str(probe)]
    out = subprocess.run(cmd, capture_output=True, text=True).stdout
    macros: dict[str, str] = {}
    for line in out.split("\n"):
        m = re.match(r"#\s*define\s+([A-Za-z_]\w*)\s*(.*)$", line)
        if m:
            macros[m.group(1)] = m.group(2).strip()
    # The generated config.h's `/* #undef X */` comments: `-dM` cannot see those, and a name
    # that is *absent* from the dump is undefined anyway, so they only make it explicit.
    for line in config.read_text(encoding="utf-8", errors="replace").split("\n"):
        m = re.match(r"/\*\s*#\s*undef\s+([A-Za-z_]\w*)\s*\*/", line.strip())
        if m:
            macros[m.group(1)] = "undef"
    return macros


def source_header_macros(include_dirs: list[str], table_file: Path) -> dict[str, str]:
    """The macros `docs/baseline/config/source-header-macros.tsv` lists, measured with clang.

    config.h cannot decide a macro that Emacs' own headers define (`PROFILER_CPU_SUPPORT` is
    syssignal.h 54's `#if defined SIGPROF && !defined PROFILING && (defined HAVE_SETITIMER ||
    defined HAVE_ITIMERSPEC)`), and the frozen build does define it, so the table has to
    carry it or the extractor calls a live region dead (a ported `defsubr` then reads as
    "unexpected").  A row is `<header>\t<macro>`; its value is whatever `clang -dM -E` says
    after `#include <config.h>` + `#include "<header>"` on the frozen include paths, so a
    stale row is reported instead of freezing a wrong branch.
    """
    if not table_file.is_file():
        return {}
    out: dict[str, str] = {}
    for line in table_file.read_text(encoding="utf-8").split("\n"):
        if not line.strip() or line.startswith("#"):
            continue
        header, _, macro = line.partition("\t")
        header, macro = header.strip(), macro.strip()
        if not header or not macro:
            continue
        probe = Path("/tmp/femacs-cond-macros-header.c")
        probe.write_text(f'#include <config.h>\n#include "{header}"\n', encoding="utf-8")
        dump = subprocess.run(["clang", "-dM", "-E", "-std=gnu23", *include_dirs, str(probe)],
                              capture_output=True, text=True).stdout
        value = None
        for dump_line in dump.split("\n"):
            m = re.match(rf"#\s*define\s+{re.escape(macro)}\s*(.*)$", dump_line)
            if m:
                value = m.group(1).strip()
                break
        if value is None:
            print(f"warning: {header} does not define {macro} on the frozen configuration; "
                  f"recording it as undef")
            value = "undef"
        out[macro] = value
    return out


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--config", default="build/emacs-ref/src/config.h")
    ap.add_argument("--out", default="docs/baseline/config/darwin-arm64.macros.tsv")
    ap.add_argument("--source-headers",
                    default="docs/baseline/config/source-header-macros.tsv",
                    help="macros Emacs' own headers define, which config.h cannot decide")
    args = ap.parse_args()
    config = Path(args.config)
    if not config.is_file():
        print(f"error: {config} does not exist — configure the reference tree first "
              "(docs/spikes/2026-10-02-three-spikes.md, Reproduction)")
        return 1
    include_dirs = ["-I", str(config.parent), "-I", "emacs/src", "-I", "emacs/lib",
                    "-I", str(config.parent.parent / "lib")]
    macros = read_macros(config, include_dirs)
    extra = source_header_macros(include_dirs, Path(args.source_headers))
    macros.update(extra)
    out = Path(args.out)
    header = ("# Generated by tools/gen-cond-macros.py from the frozen configure output "
              f"({config}).\n"
              "# The macro table `tools/extract-abi.py` evaluates `#if`/`#ifdef` with, so the\n"
              "# inventory compares the frozen configuration's live text (PORTING.md P-08/P-66).\n"
              "# <name>\\t<value>, or <name>\\tundef for a disabled macro.\n")
    out.write_text(header + "".join(f"{name}\t{value}\n" for name, value in sorted(macros.items())),
                   encoding="utf-8")
    defined = sum(1 for v in macros.values() if v != "undef")
    print(f"wrote {out}: {len(macros)} macros ({defined} defined, {len(macros) - defined} undef)"
          + (f", {len(extra)} of them from {args.source_headers}" if extra else ""))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
