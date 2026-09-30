#!/usr/bin/env python3
"""Generate docs/port-map.md: the mechanical port map for femacs.

The port map is the correspondence between the pinned reference checkout and
this repository.  It is generated data, never hand edited: re-run this script
whenever the tree changes.

Modes
-----
1:1 Rust     src/NAME.c -> femacs/src/NAME.rs, statement by statement
             src/NAME.h -> femacs/src/include/NAME.rs
Modern Rust  same C name, reimplemented in Rust on current APIs and crates
Swift        macOS backend (src/NAME.m and friends) on AppKit/CoreText
C++          Haiku backend, whose OS API is C++
Kotlin       Android app layer (java/org/gnu/emacs/NAME.java)
Drop         superseded legacy code, replaced by the modern stack
Verbatim     copied unchanged, path preserved (Emacs Lisp, tests, data)
Generated    produced by the build, never ported

Usage
-----
    python3 tools/gen-port-map.py [--reference DIR] [--out FILE]
"""

from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path

# --- the modernization policy, by file name stem -------------------------

# macOS: AppKit, CoreText, WKWebView and Swift concurrency, bridged to the
# Rust core over the C ABI.  No Objective-C.
SWIFT_STEMS = {
    "nsfns", "nsfont", "nsgui", "nsimage", "nsmenu", "nsselect", "nsterm",
    "nsxwidget", "macfont",
}

# Haiku: the OS API itself is C++, so the backend stays C++ (modern dialect).
CPP_STEMS = {
    "haiku", "haiku_io", "haikufns", "haikufont", "haikuimage", "haikumenu",
    "haikuselect", "haikuterm", "haiku_support", "haiku_draw_support",
    "haiku_font_support", "haiku_select", "haikugui",
}

# Naming rules for whole areas.
SWIFT_PREFIXES = ("ns", "macfont")
CPP_PREFIXES = ("haiku",)
WINDOWS_PREFIXES = ("w32", "cygw32")          # Rust + windows-rs (DirectWrite)
GTK_PREFIXES = ("pgtk",)                      # Rust + GTK4 / Wayland
ANDROID_PREFIXES = ("android",)               # Rust + NDK/JNI

# Ported core whose implementation is replaced by modern Rust crates instead of
# being transliterated: thin bindings to third-party libraries.
MODERN_RUST_STEMS = {
    "bignum",        # libgmp        -> pure Rust big integers
    "sysdep",        # POSIX shims   -> Rust std and nix
    "systhread",     # pthreads      -> Rust std threads
    "dynlib",        # dlopen        -> libloading
    "decompress",    # zlib          -> flate2
    "gnutls",        # GnuTLS        -> rustls
    "sqlite",        # SQLite        -> rusqlite
    "json",          # jansson       -> serde_json
    "xml",           # libxml2       -> quick-xml
    "dbusbind",      # libdbus       -> zbus
    "lcms",          # lcms2         -> lcms2 crate
    "image",         # ImageMagick   -> image crate and system codecs
    "sound",         # ALSA/OSS      -> modern Rust audio backends
    "inotify",       # inotify       -> notify crate
    "kqueue",        # kqueue        -> notify crate
    "gfilenotify",   # GLib GIO      -> notify crate
    # font parsing, shaping and rasterization -> HarfBuzz, FreeType and the
    # platform text APIs; Emacs' own TrueType rasterizer is retired
    "sfnt", "sfntfont", "sfntfont-android", "ftfont", "ftcrfont", "hbfont",
    "xftfont", "w32font", "w32dwrite", "w32uniscribe",
    # JIT native compilation -> Rust backend over LLVM or Cranelift
    "comp",
    # web widget -> WebKitGTK / WKWebView
    "xwidget",
}

# Superseded legacy code, replaced instead of ported.
DROP_STEMS = {
    "w16select",     # 16-bit Windows clipboard
    "msdos", "dosfns", "w32heap",   # DOS and unexec-era Windows heap
    "macuvs",        # CoreText handles variation sequences
    "emacs-icon",    # replaced by platform asset catalogs
    "be_resources",  # macOS bundle resources: SwiftPM resources replace them
}
# whole trees dropped as one row (the C trees below are enumerated file by file)
DROP_DIRS = {
    "msdos",         # DOS build
    "m4", "build-aux",  # autoconf machinery: cargo and xtask replace it
    "cross",         # cross-build configs: cargo targets replace them
    "nextstep",      # Cocoa/GNUstep makefiles: SwiftPM replaces them
}
X11_STEMS = {        # Xlib/Xt backend: superseded by the GTK4 path
    "xterm", "xfns", "xfont", "xgselect", "xmenu", "xrdb", "xselect",
    "xsettings", "xsmfns",
}
DROP_ROOT_FILES = {
    "configure.ac", "Makefile.in", "GNUmakefile", "make-dist", "autogen.sh",
    "config.bat",
}

# --- copied unchanged ----------------------------------------------------

VERBATIM_DIRS = ("lisp", "test", "etc", "doc", "leim", "admin")
VERBATIM_ROOT_FILES = (
    "INSTALL", "INSTALL.REPO", "CONTRIBUTE", "BUGS", "COPYING", "README",
)
CHANGELOG_GLOB = "ChangeLog*"

# --- generated -----------------------------------------------------------

GENERATED = (
    "src/config.h", "src/lisp.mk", "src/emacs-module.h", "etc/DOC",
    "lisp/loaddefs.el", "lisp/ldefs-boot.el", "admin/unidata/*",
    "admin/charsets/*",
)

MODE_ORDER = ["1:1 Rust", "Modern Rust", "Swift", "C++", "Kotlin", "Drop",
              "Verbatim", "Generated"]


def lines_of(path: Path) -> int:
    return path.read_bytes().count(b"\n")


def human(n: int) -> str:
    return f"{n:,}"


def target_lines(target: str) -> int | None:
    p = Path(target)
    return lines_of(p) if p.exists() else None


def classify_src(stem: str, suffix: str):
    """Return (mode, target, note) for a src/ file.

    Headers stay Rust: they declare the interface and the layouts shared with
    the core, whichever language implements the backend behind them.
    """
    if stem in DROP_STEMS:
        return "Drop", "-", "superseded"
    if suffix == ".h":
        return "1:1 Rust", f"femacs/src/include/{stem}.rs", "shared interface"
    if stem.startswith(CPP_PREFIXES) or stem in CPP_STEMS:
        return "C++", f"femacs/platform/haiku/{stem}.cc", "Haiku API is C++"
    if stem.startswith(SWIFT_PREFIXES) or stem in SWIFT_STEMS:
        return "Swift", f"femacs/platform/macos/{stem}.swift", "AppKit/CoreText"
    if stem.startswith(WINDOWS_PREFIXES):
        return "Modern Rust", f"femacs/src/{stem}.rs", "windows-rs"
    if stem.startswith(GTK_PREFIXES):
        return "Modern Rust", f"femacs/src/{stem}.rs", "GTK4/Wayland"
    if stem.startswith(ANDROID_PREFIXES):
        return "Modern Rust", f"femacs/src/{stem}.rs", "NDK/JNI"
    if stem in X11_STEMS:
        return "Drop", "-", "GTK4 replaces Xlib/Xt"
    if stem in MODERN_RUST_STEMS:
        return "Modern Rust", f"femacs/src/{stem}.rs", "modern crate"
    sub = "" if suffix == ".c" else "include/"
    return "1:1 Rust", f"femacs/src/{sub}{stem}.rs", ""


def collect(ref: Path):
    rows: list[dict] = []

    for p in sorted((ref / "src").iterdir()):
        if p.suffix not in (".c", ".h", ".m"):
            continue
        mode, target, note = classify_src(p.stem, p.suffix)
        if p.suffix == ".m" and mode != "Swift":
            mode, target, note = "Modern Rust", f"femacs/src/{p.stem}.rs", "was Objective-C"
        rows.append({"source": f"src/{p.name}", "lines": lines_of(p),
                     "mode": mode, "target": target, "note": note,
                     "target_lines": None if target == "-" else target_lines(target)})

    for d, mode in (("lib-src", "1:1 Rust"), ("lib", "Drop"),
                    ("lwlib", "Drop"), ("oldXMenu", "Drop"), ("exec", "Drop"),
                    ("nt", "Drop")):
        base = ref / d
        if not base.is_dir():
            continue
        for p in sorted(base.rglob("*")):
            if not p.is_file() or p.suffix not in (".c", ".h", ".cc"):
                continue
            if mode == "Drop" or p.stem in DROP_STEMS:
                note = {"lib": "gnulib: Rust std replaces it",
                        "lwlib": "Lucid widgets: GTK4 replaces them",
                        "oldXMenu": "Lucid menus: GTK4 replaces them",
                        "exec": "Android execve trampolines: NDK/JNI replaces them",
                        "nt": "NMake project files: cargo replaces them"}.get(d, "superseded")
                rows.append({"source": f"{d}/{p.relative_to(base)}",
                             "lines": lines_of(p), "mode": "Drop", "target": "-",
                             "note": note, "target_lines": None})
            else:
                target = f"femacs/{d}/{p.relative_to(base).with_suffix('.rs')}"
                rows.append({"source": f"{d}/{p.relative_to(base)}",
                             "lines": lines_of(p), "mode": mode, "target": target,
                             "note": "", "target_lines": target_lines(target)})

    java = ref / "java"
    if java.is_dir():
        for p in sorted(java.rglob("*.java")):
            target = f"femacs/platform/android/kotlin/{p.stem}.kt"
            rows.append({"source": f"java/{p.relative_to(java)}", "lines": lines_of(p),
                         "mode": "Kotlin", "target": target, "note": "Android app layer",
                         "target_lines": target_lines(target)})

    for d in sorted(DROP_DIRS):
        base = ref / d
        if not base.is_dir():
            continue
        files = [p for p in base.rglob("*") if p.is_file()]
        if not files:
            continue
        rows.append({"source": f"{d}/**", "lines": sum(lines_of(p) for p in files),
                     "mode": "Drop", "target": "-", "note": "superseded",
                     "target_lines": None, "files": len(files)})

    roots = [ref / n for n in sorted(DROP_ROOT_FILES) if (ref / n).is_file()]
    if roots:
        rows.append({"source": "configure.ac, Makefile.in, GNUmakefile, …",
                     "lines": sum(lines_of(p) for p in roots), "mode": "Drop",
                     "target": "-", "note": "cargo and xtask replace them",
                     "target_lines": None, "files": len(roots)})

    for d in VERBATIM_DIRS:
        base = ref / d
        if not base.is_dir():
            continue
        files = [p for p in base.rglob("*") if p.is_file()]
        if not files:
            continue
        rows.append({"source": f"{d}/**", "lines": sum(lines_of(p) for p in files),
                     "mode": "Verbatim", "target": f"femacs/{d}/**",
                     "note": "copied unchanged", "target_lines": None,
                     "files": len(files)})

    roots = [ref / n for n in VERBATIM_ROOT_FILES if (ref / n).is_file()]
    logs = [p for p in sorted(ref.glob(CHANGELOG_GLOB)) if p.is_file()]
    if roots or logs:
        files = roots + logs
        rows.append({"source": "INSTALL, README, COPYING, ChangeLogs, …",
                     "lines": sum(lines_of(p) for p in files), "mode": "Verbatim",
                     "target": "repository root", "note": "copied unchanged",
                     "target_lines": None, "files": len(files)})

    rows.append({"source": "config.h, lisp.mk, emacs-module.h, DOC, "
                           "loaddefs.el, unidata, charsets",
                 "lines": None, "mode": "Generated", "target": "-",
                 "note": "produced by the build", "target_lines": None})
    return rows


def render(ref: Path, rows: list[dict]) -> str:
    sha = subprocess.run(["git", "-C", str(ref), "rev-parse", "--short", "HEAD"],
                         capture_output=True, text=True).stdout.strip()
    out = ["# Port map\n",
           "Generated by `tools/gen-port-map.py` from the pinned reference "
           f"checkout (`{ref.name}`, `{sha}`). This file is generated data: do not "
           "edit it by hand, re-run the script instead. Rules and batches: "
           "[plan.md](plan.md).\n",
           "Modes: `1:1 Rust` is a statement-by-statement translation; "
           "`Modern Rust`, `Swift`, `C++` and `Kotlin` keep the same file name but "
           "are written against current APIs; `Drop` is superseded legacy code; "
           "`Verbatim` is copied unchanged.\n",
           "## Summary\n",
           "| Mode | Files | Lines | Target |",
           "| --- | ---: | ---: | --- |"]

    targets = {
        "1:1 Rust": "`femacs/src/NAME.rs`, `femacs/src/include/NAME.rs`",
        "Modern Rust": "`femacs/src/NAME.rs` (same name, modern API)",
        "Swift": "`femacs/platform/macos/NAME.swift`",
        "C++": "`femacs/platform/haiku/NAME.cc`",
        "Kotlin": "`femacs/platform/android/kotlin/NAME.kt`",
        "Drop": "-",
        "Verbatim": "path preserved",
        "Generated": "-",
    }
    for mode in MODE_ORDER:
        sel = [r for r in rows if r["mode"] == mode]
        if not sel:
            continue
        files = sum(r.get("files", 1) for r in sel)
        lines = sum(r["lines"] for r in sel if r["lines"] is not None)
        partial = any(r["lines"] is None for r in sel)
        out.append(f"| {mode} | {human(files)} | {human(lines)}"
                   f"{' (partial)' if partial else ''} | {targets[mode]} |")
    out.append("")

    for mode in [m for m in MODE_ORDER if m not in ("Verbatim", "Generated", "Drop")]:
        sel = [r for r in rows if r["mode"] == mode]
        if not sel:
            continue
        out.append(f"\n## {mode} — {len(sel)} files, "
                   f"{human(sum(r['lines'] for r in sel))} lines\n")
        out.append("| Reference source | Lines | Target | Target lines | Note |")
        out.append("| --- | ---: | --- | ---: | --- |")
        for r in sorted(sel, key=lambda x: x["source"]):
            tl = "-" if r["target_lines"] is None else human(r["target_lines"])
            out.append(f"| `{r['source']}` | {human(r['lines'])} | `{r['target']}` "
                       f"| {tl} | {r['note']} |")

    for mode in ("Drop", "Verbatim", "Generated"):
        sel = [r for r in rows if r["mode"] == mode]
        if not sel:
            continue
        out.append(f"\n## {mode}\n")
        out.append("| Reference tree | Files | Lines | Note |")
        out.append("| --- | ---: | ---: | --- |")
        for r in sorted(sel, key=lambda x: x["source"]):
            files = human(r.get("files", 1))
            lines = "-" if r["lines"] is None else human(r["lines"])
            out.append(f"| `{r['source']}` | {files} | {lines} | {r['note']} |")
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
        sel = [r for r in rows if r["mode"] == mode]
        if not sel:
            continue
        files = sum(r.get("files", 1) for r in sel)
        lines = sum(r["lines"] for r in sel if r["lines"] is not None)
        print(f"  {mode:12s} {files:5d} files {lines:>10,} lines")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
