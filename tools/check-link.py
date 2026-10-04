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



# ---------------------------------------------------------------- the executable level

# The partial link above answers "which names does the program reference and no member define".
# The *executable* link answers the sharper question - "does a program exist at all" - and it
# needs two things no unit owns (status 3.253):
#
#   * the Rust runtime seam: a `--emit obj` unit crate carries no `#[panic_handler]`, so the
#     program's link owes `__rust_begin_unwind` and `_rust_eh_personality`.  One small crate,
#     materialized here, provides them (plus `libcore`/`compiler_builtins` from the pinned
#     toolchain, which `rust-toolchain.toml` selects - the subprocess runs from `femacs/` for
#     exactly that reason);
#   * C's own library line.  `LIBXMENU`, `LIBX_BASE`, `PGTK_LIBS`, `LIBS_GNUSTEP`, `LIBS_SYSTEM`
#     and `EMACSRES` are empty in the frozen configuration, so C's recipe (`make -n temacs` in
#     `build/emacs-ref/src`, whose `temacs` rule is the authority) reduces the library set to
#     `-lncurses`; it is read from the recipe rather than written down here, so a configuration
#     change cannot silently diverge.
EXEC_PENDING = REPO / "docs/baseline/link-exec-pending.txt"
EXEC_OUT = REPO / "build/femacs/emacs"
RUNTIME_SRC = REPO / "build/femacs/rust-runtime-root.rs"
RUNTIME_OBJ = REPO / "build/femacs/rust-runtime-root.o"
RUNTIME_TEXT = """// femacs: the whole program's Rust runtime seam, materialized by `tools/check-link.py
// --executable` (docs/status.md 3.253, docs/ports/link.md).  A unit is compiled with
// `--crate-type lib --emit obj`, so no unit carries a `#[panic_handler]` and the link owes
// `rust_begin_unwind`; `_rust_eh_personality` is owed the same way.  Neither belongs to any C
// file, which is why it lives in `build/` and not in the ported tree.
#![no_std]

#[panic_handler]
fn panic (_info: &core::panic::PanicInfo) -> ! { loop {} }

#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality () {}
"""


def runtime_object() -> Path | None:
    """Compile (or reuse) the one object the program owes that no unit owns."""
    RUNTIME_SRC.parent.mkdir(parents=True, exist_ok=True)
    if not RUNTIME_SRC.is_file() or RUNTIME_SRC.read_text() != RUNTIME_TEXT:
        RUNTIME_SRC.write_text(RUNTIME_TEXT, encoding="utf-8")
    if RUNTIME_OBJ.is_file() and RUNTIME_OBJ.stat().st_mtime >= RUNTIME_SRC.stat().st_mtime:
        return RUNTIME_OBJ
    r = subprocess.run(["rustc", "--edition", "2024", "--crate-type", "lib", "--emit", "obj",
                        "--crate-name", "femacs_runtime", "-C", "panic=abort",
                        "-C", "codegen-units=1", "-C", "opt-level=0",
                        str(RUNTIME_SRC), "-o", str(RUNTIME_OBJ)],
                       cwd=REPO / "femacs", capture_output=True, text=True)
    if r.returncode != 0:
        print(r.stderr.strip()[:2000], file=sys.stderr)
        return None
    return RUNTIME_OBJ


def rust_runtime_libs() -> list[str]:
    """The pinned toolchain's `libcore` and `compiler_builtins` rlibs."""
    d = subprocess.run(["rustc", "--print", "target-libdir"], cwd=REPO / "femacs",
                       capture_output=True, text=True).stdout.strip()
    if not d:
        return []
    return (sorted(glob.glob(f"{d}/libcore-*.rlib"))
            + sorted(glob.glob(f"{d}/libcompiler_builtins-*.rlib")))


def c_link_libraries() -> tuple[list[str], str]:
    """C's own library flags, as C's own make expands them for the `temacs` rule."""
    makefile = REPO / "build/emacs-ref/src"
    if not (makefile / "Makefile").is_file():
        return [], "build/emacs-ref/src/Makefile is missing (the frozen build tree is needed)"
    r = subprocess.run(["make", "-n", "temacs"], cwd=makefile, capture_output=True, text=True)
    # The recipe is one command written across continued lines (`gcc ... -o temacs.tmp \\`),
    # so the flags come *after* the object list on later lines: join first, split after.
    joined = r.stdout.replace("\\\n", " ")
    for line in joined.split("\n"):
        if "-o temacs" not in line:
            continue
        toks, out, i = line.split(), [], 0
        while i < len(toks):
            t = toks[i]
            if re.match(r"-L\S+", t) or re.match(r"-l\S+", t) or t.startswith("-F"):
                out.append(t)
            elif t == "-framework" and i + 1 < len(toks):
                out += ["-framework", toks[i + 1]]
                i += 1
            i += 1
        seen, uniq = set(), []
        for t in out:
            if t not in seen:
                seen.add(t)
                uniq.append(t)
        return uniq, ""
    return [], "the `temacs` rule was not found in `make -n temacs`"


def executable_link(objs: list[Path]) -> tuple[list[str], str, str]:
    """Link the program.  Returns `(undefined names, stderr, note)`."""
    rt = runtime_object()
    if rt is None:
        return [], "", "the Rust runtime seam did not compile"
    c_libs, note = c_link_libraries()
    cmd = ["cc", "-o", str(EXEC_OUT), *[str(o) for o in objs], str(rt),
           *c_libs, *rust_runtime_libs()]
    r = subprocess.run(cmd, capture_output=True, text=True)
    if r.returncode == 0:
        return [], r.stderr, ""
    return sorted(set(re.findall(r'"(_[^"]+)"', r.stderr))), r.stderr, note


def write_exec_pending(rows: dict[str, str]) -> None:
    body = ["# Names an *executable* link of the whole program leaves undefined, with the class",
            "# that explains them.  Written by `python3 tools/check-link.py --executable",
            "# --write-pending` (docs/status.md 3.253, docs/ports/link.md reading four).  The",
            "# list is the program's own debt - the partial link's ratchet",
            "# (`link-pending.txt`) is the rung below it.  One `<name>\t<reason>` per line.",
            "#",
            "# A moment in time: taken with lane objects in flight, so a clean worktree at HEAD",
            "# reconciles it (`--executable --strict`; `--write-pending` when only the stale",
            "# rows differ).  A *new* name fails in either mode; stale rows are informational",
            "# unless `--strict`."]
    for name in sorted(rows):
        body.append(f"{name}\t{rows[name]}")
    EXEC_PENDING.write_text("\n".join(body) + "\n", encoding="utf-8")


def read_rows(path: Path) -> dict[str, str]:
    rows: dict[str, str] = {}
    if path.is_file():
        for raw in path.read_text().splitlines():
            if raw.strip() and not raw.startswith("#"):
                parts = raw.split("\t")
                if parts:
                    rows[parts[0]] = parts[1] if len(parts) > 1 else ""
    return rows


def executable() -> int:
    prog, missing = program_objects()
    if missing:
        print(f"{len(missing)} frozen source(s) have no built object; build them first: "
              + " ".join(sorted(missing)[:20]))
        return 2
    objs = prog + lib_objects()
    names, stderr, note = executable_link(objs)
    ref_def, ref_use = reference_symbols()
    sdk = sdk_symbols()
    klass = {n: classify(n, ref_def, ref_use, sdk) for n in names}
    if args_write_pending:
        write_exec_pending({n: REASON[klass[n]] for n in names})
        print(f"wrote {EXEC_PENDING.relative_to(REPO)}: {len(names)} row(s) from "
              f"{len(objs)} object(s)")
        return 0
    rows = read_rows(EXEC_PENDING)
    failures = 0
    for n in sorted(set(names) - set(rows)):
        print(f"FAIL {n} ({klass[n]}): the executable link leaves it undefined")
        failures += 1
    stale = sorted(set(rows) - set(names))
    for n in stale:
        print(f"STALE {n}: the executable link no longer leaves it undefined")
    if args_strict:
        failures += len(stale)
    counts: dict[str, int] = {}
    for n in names:
        counts[klass[n]] = counts.get(klass[n], 0) + 1
    if args_report:
        for n in sorted(names):
            print(f"{klass[n]}\t{n}")
    if not names:
        print(f"linked {EXEC_OUT.relative_to(REPO)} - the program has no undefined reference")
        return 0
    if note:
        print(f"note: {note}")
    print(" ".join(f"{k}={v}" for k, v in sorted(counts.items())))
    tail = f", {len(stale)} stale (informational)" if stale and not args_strict else ""
    print(f"{len(names)} undefined symbol(s) in the executable link from {len(objs)} object(s), "
          f"{len(rows)} ratchet row(s), {failures} failure(s){tail}")
    return 1 if failures else 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--write-pending", action="store_true")
    ap.add_argument("--report", action="store_true", help="print every undefined name with its class")
    ap.add_argument("--strict", action="store_true",
                    help="a ratchet row that no longer holds is a failure too (the mode a clean "
                         "worktree at HEAD is reconciled in)")
    ap.add_argument("--executable", action="store_true",
                    help="link the whole program (Rust runtime seam + C's own library line) "
                         "and compare with docs/baseline/link-exec-pending.txt")
    args = ap.parse_args()
    global args_write_pending, args_report, args_strict
    args_write_pending, args_report = args.write_pending, args.report
    args_strict = args.strict
    if args.executable:
        return executable()

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
    # A row that no longer holds is the ratchet *shrinking*, which is what a lane's object in
    # flight does to it (the unit defines ten more of its own names, and the ratchet is a moment
    # behind).  Reported always, failed on only under `--strict`, which is the shape a clean
    # worktree at HEAD is reconciled in: there the file must be exact.  A *new* name still fails
    # in both modes - that is the regression this gate exists for.
    stale = sorted(set(rows) - names)
    for n in stale:
        print(f"STALE {n}: no longer undefined")
    if args_strict:
        failures += len(stale)
    counts: dict[str, int] = {}
    for n in names:
        counts[klass[n]] = counts.get(klass[n], 0) + 1
    if args.report:
        for n in sorted(names):
            print(f"{klass[n]}\t{n}")
    print(" ".join(f"{k}={v}" for k, v in sorted(counts.items())))
    tail = f"{len(stale)} stale (informational)" if stale and not args_strict else ""
    print(f"{len(names)} undefined symbol(s) from {len(prog)} program + {len(libs)} libgnu "
          f"object(s), {len(rows)} ratchet row(s), {failures} failure(s)"
          + (f", {tail}" if tail else ""))
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
