#!/usr/bin/env python3
"""Fail when a Swift unit imports a C-ABI symbol the ported core neither exports nor owns.

Why this exists (docs/status.md 3.231 item 4): the Swift lanes speak to the Rust port only
through the C ABI - `@_cdecl("<name>")` out of the unit, `@_silgen_name("<name>")` into it - and
`tools/check-swift-units.py` counts those imports without asking whether anything provides them.
`ns_use_srgb_colorspace` sat in that gap: `nsterm.swift` and `nsimage.swift` import it, no Rust
unit exports it, and no gate read it.  `check-undef.py` cannot cover it either: its universe is
the built *Rust* unit objects, so a Swift-only symbol there would be a row nothing can ever match
(recorded when the nsterm lane asked to add sixteen of them).

The measurement is the same shape as `check-undef.py`'s:

  * the **imports** are every `@_silgen_name("X")` in `femacs/platform/**/*.swift`;
  * the **providers** are the symbols the built Rust units export (`nm -g build/femacs/**/*.o`,
    the same objects `layout-check` and `check-symbols.py` read) plus the `@_cdecl` names the
    Swift units themselves define;
  * a name that neither provides is a finding, and `docs/baseline/swift-import-pending.txt` is
    the ratchet: `<name>\t<reason>`, one per line, appended only by the integrator.

    python3 tools/check-swift-imports.py                # scan, compare with the ratchet
    python3 tools/check-swift-imports.py --write-pending # rewrite the ratchet from the tree
    python3 tools/check-swift-imports.py --selftest

System symbols (libobjc's `objc_autoreleasePoolPush`, libSystem's `uname`, AppKit's classes) do
appear here: they are C-ABI entries this program gets from the platform, not from the core, and
each one carries that reason in the ratchet rather than being filtered out by a hand-written list
of prefixes.
"""

from __future__ import annotations

import argparse
import glob
import re
import subprocess
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
SWIFT_DIRS = ("femacs/platform",)
OBJECT_GLOBS = ("build/femacs/**/*.o",)
PENDING = REPO / "docs/baseline/swift-import-pending.txt"

REF_SYMBOLS: set[str] = set()
SDK_SYMBOLS: set[str] = set()

SILGEN = re.compile(r'@_silgen_name\(\s*"([^"]+)"\s*\)')
CDECL = re.compile(r'@_cdecl\(\s*"([^"]+)"\s*\)')


def swift_sources() -> list[Path]:
    out: list[Path] = []
    for base in SWIFT_DIRS:
        out += sorted((REPO / base).rglob("*.swift"))
    return out


def load_universes() -> None:
    global REF_SYMBOLS, SDK_SYMBOLS
    REF_SYMBOLS = nm_names(REF_OBJECT_GLOBS)
    SDK_SYMBOLS = sdk_symbols()


def imports() -> dict[str, set[str]]:
    """name -> the Swift files that import it with `@_silgen_name`."""
    found: dict[str, set[str]] = {}
    for path in swift_sources():
        rel = path.relative_to(REPO).as_posix()
        for m in SILGEN.finditer(path.read_text(encoding="utf-8", errors="replace")):
            found.setdefault(m.group(1), set()).add(rel)
    return found


def swift_exports() -> dict[str, set[str]]:
    found: dict[str, set[str]] = {}
    for path in swift_sources():
        rel = path.relative_to(REPO).as_posix()
        for m in CDECL.finditer(path.read_text(encoding="utf-8", errors="replace")):
            found.setdefault(m.group(1), set()).add(rel)
    return found


def core_exports() -> set[str]:
    """Every symbol the built Rust units export, in `nm` spelling (`_name`)."""
    names: set[str] = set()
    objects = [p for g in OBJECT_GLOBS for p in glob.glob(str(REPO / g), recursive=True)]
    if not objects:
        print(f"no objects under {OBJECT_GLOBS}; run `xtask build-tu` (or gate-fast) first")
        raise SystemExit(2)
    out = subprocess.run(["nm", "-g", *objects], capture_output=True, text=True).stdout
    for line in out.split("\n"):
        parts = line.split()
        if len(parts) >= 3 and parts[1] in ("T", "t", "D", "B", "S", "R", "C"):
            names.add(parts[2])
        elif len(parts) == 2 and parts[0] in ("U",):
            continue
    return names


SDK_FRAMEWORKS = ("AppKit", "Foundation", "CoreGraphics", "CoreText", "CoreFoundation",
                  "QuartzCore", "CoreImage", "UniformTypeIdentifiers", "CoreServices",
                  "Carbon", "ApplicationServices")
SDK_LIBS = ("libSystem.B", "libobjc.A")
REF_OBJECT_GLOBS = ("build/emacs-ref/**/*.o",)
# The C side of the reference: a name that appears anywhere here is the reference's own construct
# (a header macro/INLINE/enumerator, a .c/.m file's DEFSYM or its own helper), so the core owes a
# C-ABI entry for it under C's own name (the lanes' rule, status 3.144).
REF_SOURCE_GLOBS = ("emacs/src/*.h", "emacs/lib/*.h", "emacs/src/*.c", "emacs/lib/*.c",
                    "emacs/src/*.m", "emacs/lib-src/*.h", "emacs/lib-src/*.c")


def nm_names(globs: tuple[str, ...]) -> set[str]:
    objects = [p for g in globs for p in glob.glob(str(REPO / g), recursive=True)]
    if not objects:
        return set()
    out = subprocess.run(["nm", "-g", *objects], capture_output=True, text=True).stdout
    return {line.split()[-1] for line in out.split("\n") if len(line.split()) >= 2}


def sdk_symbols() -> set[str]:
    sdk = subprocess.run(["xcrun", "--show-sdk-path"], capture_output=True, text=True).stdout.strip()
    if not sdk:
        return set()
    paths = [f"{sdk}/usr/lib/{l}.tbd" for l in SDK_LIBS] + [
        f"{sdk}/System/Library/Frameworks/{f}.framework/{f}.tbd" for f in SDK_FRAMEWORKS]
    names: set[str] = set()
    for path in paths:
        try:
            txt = Path(path).read_text(errors="ignore")
        except OSError:
            continue
        names |= {"_" + m.group(1) for m in re.finditer(r"^\s*-\s*_(\w+)\s*$", txt, re.M)}
        for m in re.finditer(r"symbols:\s*\[([^\]]*)\]", txt):
            names |= {"_" + x for x in re.findall(r"_(\w+)", m.group(1))}
    return names


def referenced_in_headers(name: str) -> bool:
    """Is `name` a construct of the reference's own C text (a macro, an INLINE, an enum, a DEFSYM)?"""
    pattern = re.compile(r"(?<![A-Za-z0-9_])" + re.escape(name) + r"(?![A-Za-z0-9_])")
    for g in REF_SOURCE_GLOBS:
        for path in glob.glob(str(REPO / g)):
            if pattern.search(Path(path).read_text(encoding="utf-8", errors="replace")):
                return True
    return False


def classify(name: str) -> str:
    """`reference` | `system` | `reference-source` | `rust-side` | `unclassified` (status 3.234).

    `rust-side` is the class the lanes' ABI rule creates: a record member the unit reads through an
    accessor the mirror names in Rust (`ns_output_view`, `window_current_matrix`), which the core
    must export as a C-ABI entry before the Swift unit can link to it.
    """
    if f"_{name}" in REF_SYMBOLS:
        return "reference"
    if f"_{name}" in SDK_SYMBOLS:
        return "system"
    if referenced_in_headers(name):
        return "reference-source"
    if rust_side_names() and name in rust_side_names():
        return "rust-side"
    if name in RUST_SIDE_GATED:
        return "rust-side-gated"
    return "unclassified"


RUST_SIDE: set[str] = set()
# A name the tree declares behind a feature gate is a *different* thing from one it declares
# unconditionally: the export exists, and no configuration this repository builds turns that arm
# on, so the row can only be cleared by a build with the feature on (status 3.262 measured what
# that costs for `have_ns`; 3.264 recorded the shim's 43 guarded bodies as exactly this class).
# The lane that reported it is right that calling them `rust-side` - "the core must export it" -
# asks for work that is already done.
RUST_SIDE_GATED: dict[str, str] = {}
GATE = re.compile(r'#\[cfg\s*\(\s*feature\s*=\s*"([a-z0-9_]+)"\s*\)\]')


def rust_items() -> tuple[set[str], dict[str, str]]:
    """`(unconditional names, gated name -> feature)` the Rust tree spells C-ABI-fashion.

    The gate is the nearest `#[cfg (feature = "...")]` in the attributes directly above the
    declaration, which is how this tree writes them (`#[unsafe(no_mangle)]` then `#[cfg (...)`
    then `pub ... fn NAME`).
    """
    names: set[str] = set()
    gated: dict[str, str] = {}
    for g in ("femacs/src/*.rs", "femacs/src/include/*.rs", "femacs/lib/*.rs",
              "femacs/lib/include/*.rs"):
        for path in glob.glob(str(REPO / g)):
            txt = Path(path).read_text(encoding="utf-8", errors="replace")
            lines = txt.split("\n")
            for i, line in enumerate(lines):
                found = re.findall(r"\bpub\s+(?:unsafe\s+)?(?:extern\s+\"C\"\s+)?fn\s+(\w+)", line)
                found += re.findall(r"\bmacro_rules!\s+(\w+)", line)
                if not found:
                    continue
                feature = None
                for j in range(i - 1, max(-1, i - 5), -1):
                    stripped = lines[j].strip()
                    if stripped.startswith("#["):
                        m = GATE.search(stripped)
                        if m:
                            feature = m.group(1)
                            break
                    elif stripped and not stripped.startswith("//"):
                        break
                for name in found:
                    if feature:
                        gated[name] = feature
                    else:
                        names.add(name)
    return names, gated


def rust_side_names() -> set[str]:
    global RUST_SIDE, RUST_SIDE_GATED
    if not RUST_SIDE and not RUST_SIDE_GATED:
        RUST_SIDE, RUST_SIDE_GATED = rust_items()
    return RUST_SIDE


def read_pending() -> dict[str, str]:
    rows: dict[str, str] = {}
    if not PENDING.is_file():
        return rows
    for raw in PENDING.read_text(encoding="utf-8").splitlines():
        if not raw.strip() or raw.startswith("#"):
            continue
        parts = raw.split("\t")
        if len(parts) >= 2:
            rows[parts[0]] = parts[1]
    return rows


def write_pending(rows: dict[str, str]) -> None:
    body = ["# C-ABI symbols the Swift units import and the ported core does not provide, yet.",
            "# Written by `python3 tools/check-swift-imports.py --write-pending` (docs/status.md",
            "# 3.233).  One `<name>\\t<reason>` per line, `nm` spelling.  A new name here is either",
            "# work the core still owes (a Rust unit must export it) or a system symbol, which says",
            "# so in its reason; an entry whose import disappeared is stale and must go."]
    for name in sorted(rows):
        body.append(f"{name}\t{rows[name]}")
    PENDING.write_text("\n".join(body) + "\n", encoding="utf-8")


def scan(verbose: bool = True) -> tuple[dict[str, set[str]], set[str], dict[str, set[str]]]:
    imp = imports()
    sx = swift_exports()
    core = core_exports()
    # A name an SDK framework's .tbd exports is provided at link time like libc is, so it is not a
    # finding: the ratchet is the list of C-ABI entries *the core* still owes (status 3.247).
    missing = {n: fs for n, fs in imp.items()
               if n not in sx and n not in core and f"_{n}" not in core
               and classify(n) != "system"}
    if verbose:
        for name in sorted(missing):
            print(f"unprovided\t{classify(name)}\t{name}\t{', '.join(sorted(missing[name]))}")
    return imp, core, missing


SELFTEST = (
    ("silgen import", '@_silgen_name("Fcons") func Fcons()', ["Fcons"]),
    ("cdecl export", '@_cdecl("macfont_driver") let macfont_driver = 0', []),
    ("no annotation", "func plain()", []),
    ("two on one line", '@_silgen_name("a") var a: Int\n@_silgen_name("b") var b: Int', ["a", "b"]),
)


def selftest() -> int:
    bad = 0
    for name, source, want in SELFTEST:
        got = sorted(SILGEN.findall(source))
        ok = got == sorted(want)
        print(f"{'ok  ' if ok else 'FAIL'} {name}: {got or 'none'}")
        bad += 0 if ok else 1
    print(f"{len(SELFTEST)} cases, {bad} failures")
    return 1 if bad else 0


def reason(name: str) -> str:
    klass = classify(name)
    if klass == "reference":
        return "the reference objects define or reference it (`nm` over build/emacs-ref); the core must export it"
    if klass == "system":
        return "provided by the platform SDK (a .tbd exports it), not by the core"
    if klass == "reference-source":
        return ("the reference's own C text declares or defines it (a header macro/INLINE/enumerator, or a "
                ".c/.m DEFSYM), imported by the C-ABI-by-C-name rule (status 3.144); the core must export it")
    if klass == "rust-side":
        return ("an accessor the mirror names in Rust for a record member the unit reads; the core must export "
                "it as a C-ABI entry (the lane's rule) or the Swift unit cannot link")
    if klass == "rust-side-gated":
        feature = RUST_SIDE_GATED.get(name, "?")
        return (f"the core does export it, but only behind `feature = \"{feature}\"`: no configuration this "
                f"repository builds turns that arm on, so only a build with {feature} can clear this row "
                f"(docs/status.md 3.262/3.264) - the work is done, the row is a record")
    # The class the universes cannot explain is *two* things, and 3.257 asked for the reason to say
    # which instead of suggesting a typo: a C-ABI spelling a Swift unit chose for a C construct
    # (the unit's own `## ABI` table records that decision, and the core owes the same-name entry -
    # status 3.255's ruling), or a name to fix.  What is checked here is only what is *true*: the
    # tail is an identifier of the reference's C text, which is what a member accessor looks like.
    ev = tail_evidence(name)
    if ev:
        tail, path = ev
        return (f"no universe explains the name itself; `{tail}` is an identifier of the reference's own "
                f"C text ({path}), which is the shape of a member accessor - if this is the importing "
                f"Swift unit's C-ABI spelling of that construct, the core owes the same-name entry "
                f"(docs/status.md 3.255 A) or the unit expands it ((B)); otherwise it is a name to fix")
    return ("nothing in the reference objects, the SDK or the reference's C and Rust text explains it: "
            "a name to investigate (a C-ABI spelling only a lane knows, or a spelling to fix)")


def tail_evidence(name: str) -> tuple[str, str] | None:
    """`(tail, path)` when the name's tail is an identifier of the reference's C text.

    `frame_set_no_split` -> `no_split` (frame.h's member) is the shape this looks for; a tail
    shorter than three characters is skipped because it would match anything.
    """
    m = re.match(r"^[a-z]+_(?:set_)?(.+)$", name)
    if not m:
        return None
    tail = m.group(1)
    if len(tail) < 3:
        return None
    pattern = re.compile(r"(?<![A-Za-z0-9_])" + re.escape(tail) + r"(?![A-Za-z0-9_])")
    for g in REF_SOURCE_GLOBS:
        for path in sorted(glob.glob(str(REPO / g))):
            if pattern.search(Path(path).read_text(encoding="utf-8", errors="replace")):
                return tail, str(Path(path).relative_to(REPO))
    return None


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--write-pending", action="store_true")
    ap.add_argument("--report", action="store_true", help="print the review list")
    ap.add_argument("--selftest", action="store_true")
    args = ap.parse_args()
    if args.selftest:
        return selftest()
    load_universes()
    imp, core, missing = scan()
    if args.write_pending:
        write_pending({n: reason(n) for n in sorted(missing)})
        print(f"wrote {PENDING.relative_to(REPO)}: {len(missing)} row(s)")
        return 0
    rows = read_pending()
    failures = 0
    for name in sorted(set(missing) - set(rows)):
        print(f"FAIL {name} ({classify(name)}): imported by {', '.join(sorted(missing[name]))} "
              f"and provided by nothing")
        failures += 1
    # A row leaves the ratchet for one of two reasons, and they ask for different actions, so the
    # gate names them apart (`status` 3.247's ambiguity): the name is *provided* now (delete the
    # row), or nothing imports it any more (check the call site - the import may have moved or the
    # row may have been added for a spelling that was never used).
    for name in sorted(set(rows) - set(missing)):
        if name in imp:
            print(f"STALE (satisfied) {name}: provided now, so the row goes")
        else:
            print(f"STALE (unimported) {name}: no Swift unit imports it any more")
        failures += 1
    # A ratcheted name that no universe explains is a *review* item, not a failure: the import is
    # real (the unit compiles), so it is either a C-ABI entry the core still owes under a name only
    # a lane knows, or a symbol another Swift unit provides.  The ratchet row is where the ruling
    # goes; `--report` prints the list.  A name that is *new* still fails, above.
    # The review list is one *reporting* class, not one kind of thing, and 3.258 measured what is
    # in it: of the 82 rows nothing explains, 18 are member accessors (their tail is an identifier
    # of the reference's C text, so `reason()` names it) and the rest are the Objective-C side -
    # `EmacsFileDelegate_new`, `EmacsImage_allocInitFromFile`, `GSFontEnumerator_*` - classes and
    # selectors another Swift unit of this port owns, which the universities here (the reference
    # objects, the SDK, the reference's C text, the Rust items) cannot see by construction.  So a
    # row in this list is either a core debt to export, an Objective-C name to attribute to its
    # Swift unit, or a spelling to fix; the ratchet row's reason is where that is written down.
    review = sorted(n for n in set(missing) & set(rows) if classify(n) == "unclassified")
    if args.report:
        for name in review:
            print(f"REVIEW {name}: imported by {', '.join(sorted(missing[name]))}")
    counts = {k: 0 for k in ("reference", "reference-source", "rust-side", "rust-side-gated",
                             "system", "unclassified")}
    for name in missing:
        counts[classify(name)] += 1
    print(" ".join(f"{k}={v}" for k, v in counts.items()))
    if review:
        print(f"({len(review)} ratcheted name(s) need a ruling - `--report` lists them)")
    print(f"{len(imp)} import(s) in {len(swift_sources())} Swift file(s), "
          f"{len(core)} core export(s), {len(missing)} unprovided, "
          f"{len(rows)} ratchet row(s), {failures} failure(s)")
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
