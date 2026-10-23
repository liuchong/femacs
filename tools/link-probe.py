#!/usr/bin/env python3
"""The linker's own reading: what a real link still cannot resolve.

Every other gate in this tree stops one level short of this one.  `build-tu` builds a single
object, `check-undef.py` reads each unit's undefined table, and `check-link.py` counts undefined
*symbols per object* against a ratchet.  None of them links: only the linker knows which of those
symbols the C library, `-lncurses` and the Rust runtime actually resolve, and 3.251's first real
link is what found four classes of invented symbol that no per-object gate could see (a libc
header's macro written as an `extern` declaration, and so on).

The recipe is the C build's own (`build/emacs-ref/src/Makefile`'s `temacs` rule, `make -n temacs`):
`cc -o <out> <ALLOBJS> ../lib/libgnu.a -lncurses`, plus this port's Rust runtime
(`libcore`/`libpanic_abort`/`libcompiler_builtins` from the pinned nightly's `target-libdir`).
3.251 measured the ladder 118 -> 106 (+ncurses) -> 88 (+runtime) -> 84 (after two fixes); the
remaining names are what the port still owes, so the count is the distance to load rung L0.

    python3 tools/link-probe.py              # link, compare with the ratchet
    python3 tools/link-probe.py --write      # rewrite docs/baseline/link-undefined.txt
    python3 tools/link-probe.py --selftest   # prove the comparison fires

`docs/baseline/link-undefined.txt` is the ratchet: `<symbol>\\t<reason>`, one per line.  A symbol
the linker reports that has no row is a **failure** (the drift got worse); a row whose symbol the
linker now resolves is **stale** (progress - update the ratchet, which is the point of writing it
down).  The object set is `check-link.py`'s, imported, so the two never disagree about what "the
program's objects" means.
"""

from __future__ import annotations

import argparse
import importlib.util
import re
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
RATCHET = REPO / "docs/baseline/link-undefined.txt"
RUST_LIBS = ("core", "panic_abort", "compiler_builtins")
UNDEFINED = re.compile(r'^  "(_?[^"]+)", referenced from:', re.M)


def load_check_link():
    spec = importlib.util.spec_from_file_location("check_link", REPO / "tools" / "check-link.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def link(out: Path) -> tuple[list[str], str]:
    """`(sorted undefined symbols, the linker's first error line)`."""
    cl = load_check_link()
    program, missing = cl.program_objects()
    if missing:
        return [], f"build/femacs is missing {len(missing)} program object(s), e.g. {missing[0]}.o"
    # The pinned runtime, not whatever `rustc` is on PATH: the channel comes from the workspace's
    # own `rust-toolchain.toml` (status 3.251 measured the three rlibs by hand).
    channel = "nightly-2026-10-01"
    toolchain = REPO / "rust-toolchain.toml"
    if toolchain.is_file():
        for line in toolchain.read_text(encoding="utf-8").splitlines():
            if line.strip().startswith("channel"):
                channel = line.split("=", 1)[1].strip().strip('"')
                break
    libdir = Path(subprocess.run(["rustup", "run", channel, "rustc", "--print", "target-libdir"],
                                 capture_output=True, text=True).stdout.strip())
    runtime = []
    for name in RUST_LIBS:
        cand = next(libdir.glob(f"lib{name}-*.rlib"), None)
        if cand is None:
            return [], f"the pinned nightly has no lib{name} in {libdir}"
        runtime.append(str(cand))
    cmd = (["cc", "-o", str(out), *[str(p) for p in program], *[str(p) for p in cl.lib_objects()],
            str(cl.LIBGNU), "-lncurses", *runtime])
    r = subprocess.run(cmd, capture_output=True, text=True)
    names = sorted(set(UNDEFINED.findall(r.stderr)))
    if not names and r.returncode != 0:
        first = next((l for l in r.stderr.splitlines() if l.strip()), "(no output)")
        return [], f"the link failed without an undefined-symbol list: {first}"
    return names, ""


def read_ratchet() -> dict[str, str]:
    rows: dict[str, str] = {}
    if not RATCHET.is_file():
        return rows
    for line in RATCHET.read_text(encoding="utf-8").splitlines():
        if not line or line.startswith("#"):
            continue
        parts = line.split("\t")
        if len(parts) >= 2:
            rows[parts[0]] = parts[1]
    return rows


def write_ratchet(names: list[str]) -> int:
    body = ["# Symbols a real link cannot resolve (tools/link-probe.py, the C build's own recipe).",
            "# One `<symbol>\\t<reason>` per line; a new name fails the gate and a row the linker",
            "# now resolves is stale - the count is the distance to load rung L0 (status 3.251, 3.337).",
            "# `_V...`/`_F...` are the port's generated Lisp-symbol carriers: the name after the",
            "# prefix is the C variable/function whose unit owes the definition."]
    for name in names:
        kind = ("a generated Lisp-variable carrier (`V` + the C variable's name): its unit has not "
                "defined the DEFVAR yet" if name.startswith("_V") else
                "a generated subr carrier (`F` + the C function's name): its unit has not defined "
                "the DEFUN yet" if name.startswith("_F") else
                "not a carrier: review whether the link line is short a library or the port is short a definition")
        body.append(f"{name}\t{kind}")
    RATCHET.write_text("\n".join(body) + "\n", encoding="utf-8")
    print(f"wrote {RATCHET.relative_to(REPO)}: {len(names)} row(s)")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--out", default="/tmp/link-probe.bin", help="where to write the linked image")
    ap.add_argument("--write", action="store_true", help="rewrite the ratchet from this link")
    ap.add_argument("--selftest", action="store_true", help="prove the comparison fires")
    args = ap.parse_args()
    names, error = link(Path(args.out))
    if error:
        print(f"FAIL {error}")
        return 1
    Path(args.out).unlink(missing_ok=True)
    if args.write:
        return write_ratchet(names)
    rows = read_ratchet()
    failures = 0
    for name in names:
        if name not in rows:
            print(f"FAIL {name}: the linker cannot resolve it and the ratchet has no row")
            failures += 1
    for name in sorted(set(rows) - set(names)):
        print(f"STALE {name}: the linker resolves it now - drop the row (that is progress)")
        failures += 1
    print(f"{len(names)} undefined symbol(s) after libc + ncurses + the Rust runtime, "
          f"{len(rows)} ratchet row(s), {failures} failure(s)")
    if args.selftest:
        # the comparison itself: one name in the link, one name only in the ratchet
        global RATCHET
        keep = RATCHET
        RATCHET = Path("/tmp/link-probe-selftest.txt")
        RATCHET.write_text("_only_in_the_ratchet\treason\n", encoding="utf-8")
        rows = read_ratchet()
        got = [n for n in names if n not in rows] + [n for n in sorted(set(rows) - set(names))]
        print(f"selftest: {len(got)} finding(s) on a ratchet that names one other symbol "
              f"(want {len(names) + 1})")
        RATCHET.unlink()
        RATCHET = keep
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
