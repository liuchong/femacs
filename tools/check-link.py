#!/usr/bin/env python3
"""Measure what a partial link of the whole `emacs` program still owes, and ratchet it.

Why this exists (`docs/status.md` 3.244): every other gate stops at objects or text - `build-tu`
builds one unit, `layout-check` compiles probes, `check-undef.py` reads each unit's own undefined
list (so it cannot see a symbol that one unit defines and another references by the wrong name
once both are in the program), and `tools/check-swift-imports.py` covers the Swift side of the
C-ABI seam.  Nothing measured the program.  This does.

The inputs are C's own (`build/emacs-ref/src/Makefile`):

    temacs$(EXEEXT): $(LIBXMENU) $(ALLOBJS) $(LIBEGNU_ARCHIVE) $(EMACSRES) ... $(LIBES)
    LIBEGNU_ARCHIVE = $(lib)/libgnu.a

so the object set is the frozen build's sources mapped to `build/femacs/<stem>.o` (minus every
object that defines `_main` itself: that list also names the separate programs), plus the members
of `libgnu.a`.  Two rules were learnt by measuring:

  * `_main` must be matched exactly (`^\\S+ [Tt] _main$`).  A prefix match also drops
    `src/thread.c`, whose `_main_thread_p` starts with `_main`.
  * gnulib amalgamates sources with `#include` while this port makes one unit per file, so a
    member named `X-gnulib.o` also needs `X.o` when that unit exists (`mini-gmp-gnulib.o` carries
    no `mpz_*` at all; `build/femacs/mini-gmp.o` carries 127 of them).

The measurement is `ld -r` - a partial link, no runtime libraries, no successful executable - plus
`nm -u` on the result: exactly the names the program references and no member defines.  Each name
is classified against the reference objects (`nm -g build/emacs-ref/**/*.o`, split into
definitions and references) and the SDK's `.tbd`:

    rust-runtime      the `4core` panic entries the Rust runtime provides at link time
    port-owed         the reference *defines* it and no unit of ours does - the real debt
    libc-or-platform  the reference *references* it too, or an SDK `.tbd` exports it
    unknown           neither: C's generated `V...` carriers (the port's documented deviation),
                      a function of an unported unit, or a name worth investigating

`docs/baseline/link-pending.txt` is the ratchet: `<name>\\t<reason>`, one per line.  A name that is
new fails, a row nothing references any more is stale and is deleted, exactly as
`check-undef.py` and `check-swift-imports.py` do it.
"""

from __future__ import annotations

import argparse
import glob
import re
import subprocess
import sys
import tempfile
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
FROZEN = REPO / "docs/baseline/frozen-build-sources.txt"
LIBGNU = REPO / "build/emacs-ref/lib/libgnu.a"
OBJ_DIR = REPO / "build/femacs"
PENDING = REPO / "docs/baseline/link-pending.txt"
MAIN = re.compile(r"^\S+ [Tt] _main$")


def nm(args: list[str]) -> str:
    return subprocess.run(["nm", *args], capture_output=True, text=True).stdout


def program_objects() -> tuple[list[Path], list[str]]:
    objs, missing = [], []
    for line in FROZEN.read_text().split("\n"):
        line = line.strip()
        if not line or line.startswith("#"):
            continue
        stem = Path(line).name.rsplit(".", 1)[0]
        path = OBJ_DIR / f"{stem}.o"
        if not path.is_file():
            missing.append(stem)
            continue
        if stem != "emacs" and any(MAIN.match(l) for l in nm(["-g", str(path)]).split("\n")):
            continue                      # a separate program, not a member of this one
        objs.append(path)
    return objs, missing


def lib_objects() -> list[Path]:
    out = subprocess.run(["ar", "t", str(LIBGNU)], capture_output=True, text=True).stdout.split()
    objs = []
    for member in out:
        stem = Path(member).name.rsplit(".", 1)[0]
        path = OBJ_DIR / f"{stem}.o"
        if path.is_file():
            objs.append(path)
        if stem.endswith("-gnulib"):      # the amalgamation rule above
            base = OBJ_DIR / f"{stem[:-len('-gnulib')]}.o"
            if base.is_file():
                objs.append(base)
    return objs


def reference_symbols() -> tuple[set[str], set[str]]:
    defined, referenced = set(), set()
    objs = glob.glob(str(REPO / "build/emacs-ref/**/*.o"), recursive=True)
    if not objs:
        return defined, referenced
    for line in nm(["-g", *objs]).split("\n"):
        p = line.split()
        if len(p) >= 3:
            (defined if p[1] in "TtDBSRC" else referenced).add(p[2])
        elif len(p) == 2 and p[0] == "U":
            referenced.add(p[1])
    return defined, referenced


def sdk_symbols() -> set[str]:
    sdk = subprocess.run(["xcrun", "--show-sdk-path"], capture_output=True, text=True).stdout.strip()
    names: set[str] = set()
    if not sdk:
        return names
    for path in [f"{sdk}/usr/lib/libSystem.B.tbd"]:
        try:
            text = Path(path).read_text(errors="ignore")
        except OSError:
            continue
        names |= {"_" + m.group(1) for m in re.finditer(r"^\s*-\s*_(\w+)\s*$", text, re.M)}
    return names


def undefined_symbols(objs: list[Path]) -> set[str]:
    with tempfile.NamedTemporaryFile(suffix=".o", delete=True) as tmp:
        r = subprocess.run(["ld", "-r", "-o", tmp.name, *[str(o) for o in objs]],
                           capture_output=True, text=True)
        if r.returncode != 0:
            print(r.stderr.strip()[:2000], file=sys.stderr)
            print(f"ld -r failed (exit {r.returncode})", file=sys.stderr)
            raise SystemExit(2)
        out = nm(["-u", tmp.name])
    return {line.split()[-1] for line in out.split("\n") if line.split()}


def classify(name: str, ref_def: set[str], ref_use: set[str], sdk: set[str]) -> str:
    if "4core" in name or name.startswith("_RN") or "rust" in name.lower():
        return "rust-runtime"
    if name in ref_def:
        return "port-owed"
    if name in ref_use or name in sdk:
        return "libc-or-platform"
    return "unknown"


REASON = {
    "port-owed": "the reference defines it and no unit of this port does - the definition is owed",
    "unknown": ("neither the reference nor the SDK explains it: a generated `V...` carrier (the port's "
                "documented deviation), a function of an unported unit, or a name to investigate"),
    "libc-or-platform": "the reference references it too (or an SDK .tbd exports it): provided at link time",
    "rust-runtime": "a `4core` entry the Rust runtime provides; not this port's debt",
}


def read_pending() -> dict[str, str]:
    rows: dict[str, str] = {}
    if PENDING.is_file():
        for raw in PENDING.read_text().splitlines():
            if raw.strip() and not raw.startswith("#"):
                parts = raw.split("\t")
                if parts: rows[parts[0]] = parts[1] if len(parts) > 1 else ""
    return rows


def write_pending(rows: dict[str, str]) -> None:
    body = ["# Symbols a partial link of the whole emacs program leaves undefined, with the class",
            "# that explains them.  Written by `python3 tools/check-link.py --write-pending`",
            "# (docs/status.md 3.244, docs/ports/link.md).  One `<name>\\t<reason>` per line.  A name",
            "# that is new fails, and a row nothing references any more is stale and is deleted - the",
            "# same ratchet `check-undef.py` and `check-swift-imports.py` keep."]
    for name in sorted(rows):
        body.append(f"{name}\t{rows[name]}")
    PENDING.write_text("\n".join(body) + "\n", encoding="utf-8")


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--write-pending", action="store_true")
    ap.add_argument("--report", action="store_true", help="print every undefined name with its class")
    args = ap.parse_args()

    prog, missing = program_objects()
    libs = lib_objects()
    if missing:
        print(f"{len(missing)} frozen source(s) have no built object, so the link set is incomplete; "
              f"run `cargo run -q -p xtask -- gate-fast` (or build-tu) first:")
        print("  " + " ".join(sorted(missing)[:20]))
        return 2
    objs = prog + libs
    names = undefined_symbols(objs)
    ref_def, ref_use = reference_symbols()
    sdk = sdk_symbols()
    klass = {n: classify(n, ref_def, ref_use, sdk) for n in names}

    if args.write_pending:
        write_pending({n: REASON[klass[n]] for n in sorted(names)})
        print(f"wrote {PENDING.relative_to(REPO)}: {len(names)} row(s) from "
              f"{len(prog)} program + {len(libs)} libgnu object(s)")
        return 0

    rows = read_pending()
    failures = 0
    for n in sorted(names - set(rows)):
        print(f"FAIL {n} ({klass[n]}): referenced and defined by nothing")
        failures += 1
    for n in sorted(set(rows) - names):
        print(f"STALE {n}: no longer undefined")
        failures += 1
    counts: dict[str, int] = {}
    for n in names:
        counts[klass[n]] = counts.get(klass[n], 0) + 1
    if args.report:
        for n in sorted(names):
            print(f"{klass[n]}\t{n}")
    print(" ".join(f"{k}={v}" for k, v in sorted(counts.items())))
    print(f"{len(names)} undefined symbol(s) from {len(prog)} program + {len(libs)} libgnu "
          f"object(s), {len(rows)} ratchet row(s), {failures} failure(s)")
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
