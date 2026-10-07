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
Verbatim    copied unchanged, path preserved: the Lisp tree, tests, data, manuals.
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
             "haiku_support", "be_resources"}

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
        # headers never collide with their .c twin: they declare the interface
        if suffix == ".h":
            return "1:1 Rust", f"femacs/{rel.parent}/include/{stem}.rs" \
                if top != "src" else f"femacs/src/include/{stem}.rs"
        return "1:1 Rust", f"femacs/{rel.with_suffix('.rs')}"
    if suffix == ".m":
        return "Swift", f"femacs/platform/macos/{stem}.swift"
    if suffix == ".cc":
        # `lib-src/be_resources.cc` used to be sent to the macOS Swift tree on the reading that
        # it is "the macOS bundle's" C++ file; its own first line says "Haiku window system
        # support", it includes <SupportDefs.h>/<AppFileInfo.h>, and lib-src/Makefile.in builds
        # it only for Haiku (`be-resources`).  It belongs with the other Haiku C++ files
        # (status 3.147).
        return ("C++", f"femacs/platform/haiku/{stem}.cc") if stem in CPP_STEMS \
            else ("C++", f"femacs/platform/{top}/{stem}.cc")
    if suffix == ".java":
        return "Kotlin", f"femacs/platform/android/kotlin/{stem}.kt"
    if suffix in (".s", ".S"):
        return "Kept", f"femacs/{rel}"
    return "Replaced", "-"


def tracked_files(ref: Path) -> list[Path] | None:
    """Files tracked by git in the reference, or None when it is not a repo.

    The map must describe the pinned checkout, not build products that happen to
    sit in the working tree.
    """
    import subprocess
    out = subprocess.run(["git", "-C", str(ref), "ls-files", "-z"],
                         capture_output=True)
    if out.returncode != 0:
        return None
    names = out.stdout.decode("utf-8", "surrogateescape").split("\0")
    return [ref / n for n in names if n]


def collect(ref: Path):
    rows: list[dict] = []
    tracked = tracked_files(ref)
    if tracked is not None:
        return collect_tracked(ref, tracked)

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


def collect_tracked(ref: Path, tracked: list[Path]) -> list[dict]:
    rows: list[dict] = []
    seen_trees: dict[str, list[Path]] = {}
    for path in tracked:
        rel = path.relative_to(ref)
        top = rel.parts[0]
        if len(rel.parts) > 1 and top in VERBATIM_DIRS:
            seen_trees.setdefault(top, []).append(path)
            continue
        if len(rel.parts) > 1 and top in REPLACED_DIRS:
            seen_trees.setdefault(top, []).append(path)
            continue
        if len(rel.parts) > 1:
            mode, target = target_for(rel)
            if mode == "Replaced":
                seen_trees.setdefault(f"{top}/**", []).append(path)
                continue
            rows.append({"source": str(rel), "lines": lines_of(path), "mode": mode,
                         "target": target, "note": "", "target_lines": target_lines(target)})
            continue
        if top in VERBATIM_ROOT_FILES or top.startswith("ChangeLog"):
            rows.append({"source": top, "files": 1, "lines": lines_of(path),
                         "mode": "Verbatim", "target": "repository root",
                         "note": "documentation"})
        elif top in REPLACED_ROOT_FILES or top in ("AGENTS.md", "CLAUDE.md"):
            rows.append({"source": top, "files": 1, "lines": lines_of(path),
                         "mode": "Replaced", "target": "-",
                         "note": "Cargo and xtask replace it" if top in REPLACED_ROOT_FILES
                                 else "reference-only policy file"})
        else:
            mode, target = target_for(rel)
            rows.append({"source": top, "files": 1, "lines": lines_of(path), "mode": mode,
                         "target": target, "note": "", "target_lines": target_lines(target)})

    for tree, files in sorted(seen_trees.items()):
        mode = "Verbatim" if tree.rstrip("/**") in VERBATIM_DIRS else "Replaced"
        note = "copied unchanged" if mode == "Verbatim" else "not part of the program"
        if tree in REPLACED_DIRS:
            note = "Cargo and xtask replace it"
        ext = {"el": 0, "c": 0, "h": 0, "other": 0}
        for f in files:
            ext[{".el": "el", ".c": "c", ".h": "h"}.get(f.suffix, "other")] += 1
        rows.append({"source": tree, "files": len(files),
                     "lines": sum(lines_of(f) for f in files), "mode": mode,
                     "target": f"femacs/{tree}" if mode == "Verbatim" else "-",
                     "note": note + (f"  (.el {ext['el']}, .c {ext['c']}, .h {ext['h']}, other {ext['other']})"
                                     if mode == "Verbatim" else "")})
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
        print(f"error: {ref} does not look like the reference checkout", file=sys.stderr)
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
