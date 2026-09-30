#!/usr/bin/env python3
"""Generate docs/port-map.md: the mechanical port map for femacs.

The port map is the correspondence between the pinned reference checkout and
this repository's Rust sources.  It is generated data, never hand edited:
re-run this script whenever the tree changes.

Mapping rules
-------------
src/NAME.c      -> femacs/src/NAME.rs            1:1 Rust port
src/NAME.h      -> femacs/src/include/NAME.rs    1:1 Rust port
src/NAME.m      -> deferred (Objective-C, recorded only)
lib-src/NAME.c  -> femacs/lib-src/NAME.rs        1:1 Rust port
lib/NAME.c|h    -> femacs/lib/NAME.rs            1:1 Rust port
lwlib/NAME.c|h  -> femacs/lwlib/NAME.rs          1:1 Rust port
oldXMenu/NAME.c -> femacs/oldXMenu/NAME.rs       1:1 Rust port
exec/NAME.c     -> femacs/exec/NAME.rs           1:1 Rust port
nt/NAME.c|h     -> femacs/nt/NAME.rs             1:1 Rust port
lisp/**, test/**, etc/**, doc/**, ...            verbatim copy, path preserved

Usage
-----
    python3 tools/gen-port-map.py [--reference DIR] [--out FILE]
"""

from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path

# src/ files that belong to a platform backend rather than the portable core.
PLATFORM_PREFIXES = (
    "ns", "w32", "haiku", "android", "msdos", "dosfns", "pgtk",
    "cygw32", "gtkutil", "emacsgtkfixed", "macfont", "macuvs",
)
PLATFORM_STEMS = {
    "xterm", "xfns", "xfont", "xftfont", "xgselect", "xmenu", "xrdb",
    "xselect", "xsettings", "xsmfns", "xwidget",
}

# directories holding C sources ported 1:1, mapped onto femacs/<dir>/
PORT_DIRS = ("lib-src", "lwlib", "oldXMenu", "exec", "nt")

# trees copied into the repository unchanged, path preserved
VERBATIM_DIRS = (
    "lisp", "test", "etc", "doc", "leim", "admin", "m4", "build-aux",
    "nextstep", "java", "msdos", "cross", "modules", "info",
)
VERBATIM_ROOT_FILES = (
    "configure.ac", "Makefile.in", "GNUmakefile", "make-dist",
    "autogen.sh", "config.bat", "INSTALL", "INSTALL.REPO", "CONTRIBUTE",
    "BUGS", "COPYING", "README",
)
CHANGELOG_GLOB = "ChangeLog*"

AREA_ORDER = ["core", "platform", "objc (deferred)", "lib-src", "lwlib",
              "oldXMenu", "exec", "nt", "lib (gnulib)"]


def lines_of(path: Path) -> int:
    return path.read_bytes().count(b"\n")


def rust_lines(target: str) -> int | None:
    p = Path(target)
    return lines_of(p) if p.exists() else None


def in_scope(target: str) -> bool:
    """Only look for Rust counterparts inside this repository."""
    return not target.startswith("..")


def classify_src(name: str) -> str:
    stem = name.rsplit(".", 1)[0]
    if stem.startswith(PLATFORM_PREFIXES) or stem in PLATFORM_STEMS:
        return "platform"
    return "core"


def collect(ref: Path):
    ports: list[dict] = []
    trees: list[dict] = []

    for p in sorted((ref / "src").iterdir()):
        if p.suffix not in (".c", ".h", ".m"):
            continue
        stem = p.stem
        if p.suffix == ".m":
            ports.append({"source": f"src/{p.name}", "lines": lines_of(p),
                          "target": "-", "area": "objc (deferred)",
                          "rust_lines": None})
            continue
        kind = "src-c" if p.suffix == ".c" else "src-h"
        sub = "" if kind == "src-c" else "include/"
        target = f"femacs/src/{sub}{stem}.rs"
        ports.append({"source": f"src/{p.name}", "lines": lines_of(p),
                      "target": target, "area": classify_src(p.name),
                      "rust_lines": rust_lines(target)})

    for d in PORT_DIRS:
        base = ref / d
        if not base.is_dir():
            continue
        for p in sorted(base.rglob("*")):
            if not p.is_file() or p.suffix not in (".c", ".h"):
                continue
            rel = p.relative_to(base)
            target = f"femacs/{d}/{rel.with_suffix('.rs')}"
            ports.append({"source": f"{d}/{rel}", "lines": lines_of(p),
                          "target": target, "area": d,
                          "rust_lines": rust_lines(target)})

    gnulib = ref / "lib"
    if gnulib.is_dir():
        for p in sorted(gnulib.rglob("*")):
            if not p.is_file() or p.suffix not in (".c", ".h"):
                continue
            rel = p.relative_to(gnulib)
            target = f"femacs/lib/{rel.with_suffix('.rs')}"
            ports.append({"source": f"lib/{rel}", "lines": lines_of(p),
                          "target": target, "area": "lib (gnulib)",
                          "rust_lines": rust_lines(target)})

    for d in VERBATIM_DIRS:
        base = ref / d
        if not base.is_dir():
            continue
        ext = {"el": [0, 0], "c": [0, 0], "h": [0, 0], "other": [0, 0]}
        files = 0
        for p in base.rglob("*"):
            if not p.is_file():
                continue
            files += 1
            key = {".el": "el", ".c": "c", ".h": "h"}.get(p.suffix, "other")
            ext[key][0] += 1
            ext[key][1] += lines_of(p)
        trees.append({"source": f"{d}/**", "files": files,
                      "lines": sum(v[1] for v in ext.values()),
                      "ext": ext, "target": f"femacs/{d}/**"})

    roots = [ref / n for n in VERBATIM_ROOT_FILES if (ref / n).is_file()]
    if roots:
        trees.append({"source": "root build files", "files": len(roots),
                      "lines": sum(lines_of(p) for p in roots),
                      "ext": None, "target": "repository root"})
    logs = sorted(ref.glob(CHANGELOG_GLOB)) + sorted(ref.glob("*/ChangeLog.1"))
    logs = [p for p in logs if p.is_file()]
    if logs:
        trees.append({"source": "ChangeLog.*", "files": len(logs),
                      "lines": sum(lines_of(p) for p in logs),
                      "ext": None, "target": "femacs/**"})
    return ports, trees


def human(n: int) -> str:
    return f"{n:,}"


def render(ref: Path, ports: list[dict], trees: list[dict]) -> str:
    sha = subprocess.run(["git", "-C", str(ref), "rev-parse", "--short", "HEAD"],
                         capture_output=True, text=True).stdout.strip()
    areas: dict[str, list[dict]] = {}
    for row in ports:
        areas.setdefault(row["area"], []).append(row)

    out: list[str] = []
    out.append("# Port map\n")
    out.append("Generated by `tools/gen-port-map.py` from the pinned reference "
               f"checkout (`{ref.name}`, `{sha}`). This file is generated data: "
               "do not edit it by hand, re-run the script instead. "
               "Narrative and rules: [plan.md](plan.md).\n")

    out.append("## C sources ported 1:1\n")
    out.append("| Area | Files | Lines | Rust target | Mode |")
    out.append("| --- | ---: | ---: | --- | --- |")
    for area in AREA_ORDER:
        rows = areas.get(area)
        if not rows:
            continue
        if area.startswith("objc"):
            target, mode = "-", "deferred"
        elif area in ("core", "platform"):
            target, mode = "`femacs/src/*.rs`", "1:1 Rust"
        else:
            target, mode = f"`femacs/{area.split(' ')[0]}/*.rs`", "1:1 Rust"
        out.append(f"| `{area}` | {len(rows)} | {human(sum(r['lines'] for r in rows))} "
                   f"| {target} | {mode} |")
    ported = [r for r in ports if not r["area"].startswith("objc")]
    out.append(f"| **total** | **{len(ported)}** | "
               f"**{human(sum(r['lines'] for r in ported))}** | | |")

    out.append("\n## Verbatim trees\n")
    out.append("Copied unchanged, path preserved. `Lines` counts every file in the "
               "tree; the per-extension columns count files.\n")
    out.append("| Tree | Files | Lines | .el | .c | .h | other | Target |")
    out.append("| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |")
    for t in trees:
        if t["ext"] is None:
            cells = "| — | — | — | — |"
        else:
            cells = "|" + "|".join(f" {t['ext'][k][0]}" for k in ("el", "c", "h", "other")) + "|"
        out.append(f"| `{t['source']}` | {t['files']} | {human(t['lines'])} "
                   f"{cells} `{t['target']}` |")
    out.append(f"| **total** | **{sum(t['files'] for t in trees)}** | "
               f"**{human(sum(t['lines'] for t in trees))}** | | | | | |")

    out.append("\n## Per-file detail\n")
    for area in AREA_ORDER:
        rows = areas.get(area)
        if not rows:
            continue
        out.append(f"\n### {area} — {len(rows)} files, "
                   f"{human(sum(r['lines'] for r in rows))} lines\n")
        out.append("| C source | Lines | Rust target | Rust lines | Status |")
        out.append("| --- | ---: | --- | ---: | --- |")
        for r in sorted(rows, key=lambda x: x["source"]):
            if r["area"].startswith("objc"):
                out.append(f"| `{r['source']}` | {human(r['lines'])} | - | - | deferred |")
            elif r["rust_lines"] is None:
                out.append(f"| `{r['source']}` | {human(r['lines'])} "
                           f"| `{r['target']}` | - | pending |")
            else:
                out.append(f"| `{r['source']}` | {human(r['lines'])} "
                           f"| `{r['target']}` | {human(r['rust_lines'])} | ported |")
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

    ports, trees = collect(ref)
    out = Path(args.out)
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(render(ref, ports, trees), encoding="utf-8")

    print(f"wrote {out}")
    for area in AREA_ORDER:
        rows = [r for r in ports if r["area"] == area]
        if rows:
            print(f"  {area:18s} {len(rows):4d} files {sum(r['lines'] for r in rows):>9,} lines")
    ported = [r for r in ports if not r["area"].startswith("objc")]
    print(f"  {'TOTAL (ported)':18s} {len(ported):4d} files {sum(r['lines'] for r in ported):>9,} lines")
    for t in trees:
        print(f"  verbatim {t['source']:16s} {t['files']:5d} files {t['lines']:>9,} lines")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
