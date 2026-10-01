#!/usr/bin/env python3
"""Generate docs/port-map.md: the file-by-file port map for femacs.

Generated data, never hand edited: re-run the script when the tree changes.

Policy
------
1:1 Rust    every C source (`*.c`, `*.h`) in the reference tree is ported to
            Rust one to one: `src/NAME.c` -> `femacs/src/NAME.rs`,
            `src/NAME.h` -> `femacs/src/include/NAME.rs`, other C directories
            keep their directory name under `femacs/`.
Swift       macOS backends written in Objective-C (`src/*.m`) become Swift.
Kotlin      the Android app layer (`java/**/*.java`) becomes Kotlin.
C++         code that must talk to a C++ API (Haiku) stays modern C++.
Kept        assembly and data files that are used as they are.
Verbatim    copied unchanged, path preserved: Emacs Lisp, tests, data, manuals.
Replaced    build scaffolding, which is not part of the program: Cargo and the
            Rust `xtask` replace it.

Usage
-----
    python3 tools/gen-port-map.py [--reference DIR] [--out FILE]
"""

from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path

# Objective-C files of the macOS backend -> Swift
SWIFT_STEMS = {"nsfns", "nsfont", "nsimage", "nsmenu", "nsselect", "nsterm",
               "nsxwidget", "macfont"}
# C++ files that speak the Haiku API -> modern C++
CPP_STEMS = {"haiku_draw_support", "haiku_font_support", "haiku_select",
             "haiku_support"}
# C++ file of the macOS bundle -> Swift
SWIFT_EXTRA_STEMS = {"be_resources"}

C_SOURCE_DIRS = ("src", "lib", "lib-src", "lwlib", "oldXMenu", "exec", "nt",
                 "msdos")
VERBATIM_DIRS = ("lisp", "test", "etc", "doc", "leim", "admin")
REPLACED_DIRS = ("m4", "build-aux", "cross", "nextstep")
VERBATIM_ROOT_FILES = ("INSTALL", "INSTALL.REPO", "CONTRIBUTE", "BUGS",
                       "COPYING", "README")
REPLACED_ROOT_FILES = ("configure.ac", "Makefile.in", "GNUmakefile", "make-dist",
                       "autogen.sh", "config.bat")
CHANGELOG_GLOB = "ChangeLog*"

MODE_ORDER = ["1:1 Rust", "Swift", "Kotlin", "C++", "Kept", "Verbatim",
              "Replaced"]

TARGET_RULE = {
    "1:1 Rust": "`femacs/<same path>.rs` (`src/*.h` -> `femacs/src/include/*.rs`)",
    "Swift": "`femacs/platform/macos/NAME.swift`",
    "Kotlin": "`femacs/platform/android/kotlin/NAME.kt`",
    "C++": "`femacs/platform/haiku/NAME.cc`",
    "Kept": "path preserved",
    "Verbatim": "path preserved",
    "Replaced": "-",
}


def lines_of(path: Path) -> int:
    return path.read_bytes().count(b"\n")


def human(n: int) -> str:
    return f"{n:,}"


def target_lines(target: str) -> int | None:
    p = Path(target)
    return lines_of(p) if p.exists() else None


DOC_NAMES = ("README", "COPYING", "INSTALL", "CONTRIBUTE", "BUGS", "NEWS",
             "AUTHORS", "TODO", "ChangeLog")


def target_for(rel: Path) -> tuple[str, str]:
    """Return (mode, target) for one reference-relative file."""
    stem, suffix = rel.stem, rel.suffix
    if suffix in (".md", ".txt") or stem.startswith("ChangeLog") or stem in DOC_NAMES:
        return "Verbatim", f"femacs/{rel}"
    top = rel.parts[0]
    if suffix in (".c", ".h") and top in C_SOURCE_DIRS:
        sub = f"include/{stem}.rs" if suffix == ".h" and top == "src" else f"{rel.with_suffix('.rs')}"
        return "1:1 Rust", f"femacs/{sub}"
    if suffix == ".m":
        return "Swift", f"femacs/platform/macos/{stem}.swift"
    if suffix == ".cc":
        if stem in SWIFT_EXTRA_STEMS:
            return "Swift", f"femacs/platform/macos/{stem}.swift"
        return "C++", f"femacs/platform/haiku/{stem}.cc"
    if suffix == ".java":
        return "Kotlin", f"femacs/platform/android/kotlin/{stem}.kt"
    if suffix in (".s", ".S"):
        return "Kept", f"femacs/{rel}"
    return "Replaced", "-"


def collect(ref: Path):
    rows: list[dict] = []

    for top in sorted(p.name for p in ref.iterdir()):
        if top.startswith("."):
            continue
        entry = ref / top
        if entry.is_dir() and top in VERBATIM_DIRS:
            files = [p for p in entry.rglob("*") if p.is_file()]
            ext = {"el": 0, "c": 0, "h": 0, "other": 0}
            for p in files:
                ext[{".el": "el", ".c": "c", ".h": "h"}.get(p.suffix, "other")] += 1
            rows.append({"source": f"{top}/**", "files": len(files),
                         "lines": sum(lines_of(p) for p in files),
                         "mode": "Verbatim", "target": f"femacs/{top}/**",
                         "note": f".el {ext['el']}, .c {ext['c']}, .h {ext['h']}, other {ext['other']}"})
            continue
        if entry.is_dir() and top in REPLACED_DIRS:
            files = [p for p in entry.rglob("*") if p.is_file()]
            rows.append({"source": f"{top}/**", "files": len(files),
                         "lines": sum(lines_of(p) for p in files),
                         "mode": "Replaced", "target": "-",
                         "note": "Cargo and xtask replace it"})
            continue
        if entry.is_file():
            if top in VERBATIM_ROOT_FILES or any(top.startswith(g.rstrip("*")) for g in (CHANGELOG_GLOB,)):
                rows.append({"source": top, "files": 1, "lines": lines_of(entry),
                             "mode": "Verbatim", "target": "repository root",
                             "note": "documentation"})
            elif top in REPLACED_ROOT_FILES:
                rows.append({"source": top, "files": 1, "lines": lines_of(entry),
                             "mode": "Replaced", "target": "-",
                             "note": "Cargo and xtask replace it"})
            else:
                rows.append({"source": top, "files": 1, "lines": lines_of(entry),
                             "mode": "Replaced", "target": "-", "note": "not part of the program"})
            continue
        if not entry.is_dir():
            continue
        # a source directory: classify every file
        other: list[Path] = []
        for p in sorted(entry.rglob("*")):
            if not p.is_file():
                continue
            mode, target = target_for(p.relative_to(ref))
            if mode == "Replaced":
                other.append(p)
                continue
            rows.append({"source": str(p.relative_to(ref)), "lines": lines_of(p),
                         "mode": mode, "target": target,
                         "note": "", "target_lines": target_lines(target)})
        if other:
            rows.append({"source": f"{top}/** (build and data files)",
                         "files": len(other), "lines": sum(lines_of(p) for p in other),
                         "mode": "Replaced", "target": "-",
                         "note": "not part of the program"})
    return rows


def totals(rows, mode):
    sel = [r for r in rows if r["mode"] == mode]
    return (sum(r.get("files", 1) for r in sel),
            sum(r["lines"] for r in sel if r["lines"] is not None),
            sel)


def render(ref: Path, rows: list[dict]) -> str:
    sha = subprocess.run(["git", "-C", str(ref), "rev-parse", "--short", "HEAD"],
                         capture_output=True, text=True).stdout.strip()
    out = ["# Port map\n",
           "Generated by `tools/gen-port-map.py` from the pinned reference checkout "
           f"(`{ref.name}`, `{sha}`). Generated data: do not edit by hand. "
           "Plan and rules: [port-plan.md](port-plan.md).\n",
           "## Summary\n",
           "| Mode | Files | Lines | Target |",
           "| --- | ---: | ---: | --- |"]
    for mode in MODE_ORDER:
        files, lines, sel = totals(rows, mode)
        if not sel:
            continue
        out.append(f"| {mode} | {human(files)} | {human(lines)} | {TARGET_RULE[mode]} |")
    out.append("")

    for mode in ("1:1 Rust", "Swift", "Kotlin", "C++", "Kept"):
        files, lines, sel = totals(rows, mode)
        if not sel:
            continue
        out.append(f"\n## {mode} — {len(sel)} entries, {human(files)} files, "
                   f"{human(lines)} lines\n")
        out.append("| Reference source | Lines | Target | Target lines |")
        out.append("| --- | ---: | --- | ---: |")
        for r in sorted(sel, key=lambda x: x["source"]):
            tl = "-" if r.get("target_lines") is None else human(r["target_lines"])
            out.append(f"| `{r['source']}` | {human(r['lines'])} | `{r['target']}` | {tl} |")

    for mode in ("Verbatim", "Replaced"):
        out.append(f"\n## {mode}\n")
        out.append("| Reference tree | Files | Lines | Note |")
        out.append("| --- | ---: | ---: | --- |")
        for r in sorted([x for x in rows if x["mode"] == mode], key=lambda x: x["source"]):
            out.append(f"| `{r['source']}` | {human(r.get('files', 1))} | "
                       f"{human(r['lines']) if r['lines'] is not None else '-'} | {r['note']} |")
    return "\n".join(out) + "\n"


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--reference", default="emacs")
    ap.add_argument("--out", default="docs/port-map.md")
    args = ap.parse_args()
    ref = Path(args.reference)
    if not (ref / "src").is_dir():
        print(f"error: {ref} does not look like an Emacs checkout", file=sys.stderr)
        return 1
    rows = collect(ref)
    out = Path(args.out)
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(render(ref, rows), encoding="utf-8")
    print(f"wrote {out}")
    for mode in MODE_ORDER:
        files, lines, sel = totals(rows, mode)
        if sel:
            print(f"  {mode:10s} {len(sel):4d} entries {files:5d} files {lines:>10,} lines")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
