#!/usr/bin/env python3
"""Fail when the Android `libemacs.so` imports a name its own libraries cannot provide.

`xtask link-shared` builds the library and reports the linker's exit status, and that status is
not enough: a `-shared` link does **not** imply `-z defs`, so lld is happy to emit a library whose
undefined symbols nothing will ever satisfy.  The library then passes every structural gate
(member of the APK, 69 JNI exports, signature verified) and is refused by the device at the first
`dlopen`:

    dlopen failed: cannot locate symbol "malloc" referenced by ".../libemacs.so"

That was a real defect, not a hypothetical one (status 4.707): `link-shared` passes
`-nodefaultlibs`, which drops the driver's own `-lc`, and bionic resolves a `dlopen`ed library's
imports through that library's `DT_NEEDED` list alone.  This tool is the gate that would have
caught it without a device.

The measurement is differential, the way `tools/check-undef.py` is:

  * **imports** - the library's undefined dynamic symbols (`llvm-nm -D --undefined-only`);
  * **providers** - the dynamic symbols of the libraries the library itself declares
    (`llvm-readelf -d` -> `DT_NEEDED`), read from the NDK's stub libraries under
    `NDK/toolchains/llvm/prebuilt/*/sysroot/usr/lib/<target>/<api>/`.  A `DT_NEEDED` the stub
    directory does not have is reported as unknown rather than silently trusted;
  * **unresolved** = imports - providers.  These are names no library in the process provides.

Whether that *blocks loading* depends on the binding the link asked for.  With `-z now` (this
link used to pass it) every import is eager and one missing name fails the `dlopen`; with bionic's
default lazy binding - which is what C's own `libemacs.so` link has (`src/Makefile.in` 749-755,
no `-z now`) - the library loads and only a *call* to the missing name faults, which is C's own
behaviour for `backtrace` at API 21 (bionic added it in 33, `emacs_backtrace` calls it
unguarded).  The tool reads `DT_FLAGS`/`DT_BIND_NOW` and says which of the two applies.

The port is not finished on this target, so unresolved names are expected while the arms land.
`docs/baseline/android-imports-pending.txt` is therefore a **ratchet**: this tool fails on a name
that is *new* (a misspelling, an invented helper, a `#[no_mangle]` that went missing, a header
mirror that hard-codes another platform's symbol) and reports the entries that have become stale
so the file can shrink.  Regenerate it with `--write-pending` after reading what changed.

    python3 tools/check-android-imports.py
    python3 tools/check-android-imports.py --so build/port/aarch64-linux-android/libemacs.so
    python3 tools/check-android-imports.py --json
"""

from __future__ import annotations

import argparse
import json
import os
import pathlib
import re
import subprocess
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
SO = "build/port/aarch64-linux-android/libemacs.so"
TARGET = "aarch64-linux-android"
PENDING = "docs/baseline/android-imports-pending.txt"

NEEDED = re.compile(r"\(NEEDED\)\s+Shared library: \[(.+?)\]")


def ndk_bin() -> pathlib.Path:
    """The NDK's LLVM `bin` directory - the same discovery `xtask link-shared` uses."""
    root = os.environ.get("ANDROID_NDK_HOME") or os.environ.get("ANDROID_NDK_ROOT")
    if root:
        found = sorted(pathlib.Path(root).glob("toolchains/llvm/prebuilt/*/bin"))
        if found:
            return found[-1]
    home = os.environ.get("ANDROID_SDK_ROOT") or os.environ.get("ANDROID_HOME")
    sdk = pathlib.Path(home) if home else pathlib.Path.home() / "Library/Android/sdk"
    found = sorted(sdk.glob("ndk/*/toolchains/llvm/prebuilt/*/bin"))
    if not found:
        sys.exit("no NDK under $ANDROID_NDK_HOME / $ANDROID_SDK_ROOT / ~/Library/Android/sdk")
    return found[-1]


def stub_dir(bin_dir: pathlib.Path, target: str, api: str) -> pathlib.Path:
    sysroot = bin_dir.parent / "sysroot"
    return sysroot / "usr/lib" / target / api


def dynamic_symbols(nm: pathlib.Path, library: pathlib.Path, *, defined: bool) -> set[str]:
    flag = "--defined-only" if defined else "--undefined-only"
    out = subprocess.run([str(nm), "-D", flag, str(library)],
                         capture_output=True, text=True).stdout
    names = set()
    for line in out.splitlines():
        name = line.split()[-1] if line.split() else ""
        # bionic versions its exports (`__cxa_atexit@LIBC`); an import never carries the version.
        names.add(name.split("@")[0])
    return names


def measure(so: pathlib.Path, api: str) -> tuple[set[str], set[str], list[str], dict[str, set[str]]]:
    """(unresolved, provided, unknown-needed, providers-of-each-name)."""
    bin_dir = ndk_bin()
    readelf = bin_dir / "llvm-readelf"
    nm = bin_dir / "llvm-nm"
    for tool in (readelf, nm):
        if not tool.exists():
            sys.exit(f"{tool} is missing: the NDK is incomplete")
    text = subprocess.run([str(readelf), "-d", str(so)], capture_output=True, text=True).stdout
    needed = NEEDED.findall(text)
    imports = dynamic_symbols(nm, so, defined=False)
    stubs = stub_dir(bin_dir, TARGET, api)
    provided: set[str] = set()
    origins: dict[str, set[str]] = {}
    unknown: list[str] = []
    for name in needed:
        stub = stubs / name
        if not stub.is_file():
            unknown.append(name)
            continue
        names = dynamic_symbols(nm, stub, defined=True)
        provided |= names
        for symbol in names:
            origins.setdefault(symbol, set()).add(name)
    unresolved = imports - provided
    return unresolved, imports, unknown, {n: origins.get(n, set()) for n in unresolved}


def binds_eagerly(so: pathlib.Path) -> bool:
    """Whether the library asks for eager binding (`-z now`) - then a missing import blocks load."""
    readelf = ndk_bin() / "llvm-readelf"
    text = subprocess.run([str(readelf), "-d", str(so)],
                          capture_output=True, text=True).stdout
    # DF_BIND_NOW is bit 0x8 of DT_FLAGS; DT_BIND_NOW is the pre-DF_FLAGS spelling.
    if "BIND_NOW" in text:
        return True
    for line in text.splitlines():
        if "FLAGS" in line and "0x" in line:
            for token in line.split():
                if token.startswith("0x"):
                    try:
                        if int(token, 16) & 0x8:
                            return True
                    except ValueError:
                        pass
    return False


def read_pending(path: pathlib.Path) -> list[str]:
    if not path.is_file():
        return []
    return [line.strip() for line in path.read_text(encoding="utf-8").splitlines()
            if line.strip() and not line.startswith("#")]


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--so", default=SO)
    ap.add_argument("--api", default=os.environ.get("ANDROID_NDK_API", "21"),
                    help="the API level's stub libraries (default: 21, the link's --min-api)")
    ap.add_argument("--pending", default=PENDING)
    ap.add_argument("--write-pending", action="store_true")
    ap.add_argument("--json", action="store_true")
    args = ap.parse_args()

    so = REPO / args.so
    if not so.is_file():
        sys.exit(f"{args.so} does not exist: build it first (`xtask link-shared "
                 f"{TARGET}`)")
    unresolved, imports, unknown, origins = measure(so, args.api)
    eager = binds_eagerly(so)
    path = REPO / args.pending
    if args.write_pending:
        header = ("# Names the Android `libemacs.so` imports that no library in the process\n"
                  "# provides.  Written by `python3 tools/check-android-imports.py\n"
                  "# --write-pending` (docs/status.md 4.707, PORTING.md P-300).  One symbol per\n"
                  "# line, `nm` spelling.  An entry disappears when the arm that defines it is\n"
                  "# transcribed for this target; a NEW symbol here is a defect (a misspelling,\n"
                  "# a missing #[no_mangle], or a header mirror hard-coding another platform's\n"
                  "# symbol).  A non-empty file means the library cannot be dlopen'ed.\n")
        path.write_text(header + "\n".join(sorted(unresolved)) + "\n")
        print(f"wrote {path.relative_to(REPO)}: {len(unresolved)} unresolved import(s)")
        return 0

    pending = set(read_pending(path))
    new = sorted(unresolved - pending)
    stale = sorted(pending - unresolved)
    if args.json:
        print(json.dumps({"unresolved": sorted(unresolved), "new": new, "stale": stale,
                          "imports": len(imports), "unknown_needed": unknown}, indent=2))
        return 1 if new else 0
    if unknown:
        print(f"check-android-imports: DT_NEEDED without a stub library, so its symbols cannot "
              f"be checked: {' '.join(unknown)}")
    if stale:
        print(f"check-android-imports: {len(stale)} ratchet entr"
              f"{'y' if len(stale) == 1 else 'ies'} are stale (now provided): "
              f"{' '.join(stale[:8])}" + (" …" if len(stale) > 8 else ""))
    if new:
        print(f"check-android-imports: {len(new)} NEW import(s) that no library provides:")
        for name in new[:40]:
            who = ", ".join(sorted(origins.get(name, ()))) or "nothing in DT_NEEDED"
            print(f"  - {name:38s} searched: {who}")
        if len(new) > 40:
            print(f"  … {len(new) - 40} more")
        print("  A new entry is either an arm that is not transcribed for this target (then run\n"
              "  `--write-pending` after reading the list) or a defect in the referring unit.")
        return 1
    tail = "; the library cannot be dlopen'ed until that list is empty" if unresolved else ""
    binding = ("eager (-z now): the library will not load" if eager
               else "lazy (bionic default): it loads, and a call to a missing name faults")
    print(f"check-android-imports: {len(imports)} import(s); {len(unresolved)} unresolved "
          f"(0 new); binding {binding}{tail if eager else ''}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
