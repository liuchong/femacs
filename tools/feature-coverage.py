#!/usr/bin/env python3
"""Which `#[cfg (feature = "...")]` arms does no configuration ever compile?

PORTING.md P-51 carries the arms the frozen configuration does not take, and `xtask dead-arms`
exists to compile *some* of them by flipping a feature together with its configuration constant.
That list is short (six entries) while the tree mentions dozens of features, so a large part of the
carried text has never been through rustc - which is how 3.262 found eleven bare `c_float`
spellings that only the flip could see, and how the shim lane's guarded bodies turned out to be
"recorded text" rather than compiled text.

This tool measures the surface rather than asserting it:

    python3 tools/feature-coverage.py                 # the table
    python3 tools/feature-coverage.py --write         # write docs/baseline/feature-coverage.tsv

Columns: `<feature>\\t<sites>\\t<files>\\t<enabled by>\\t<config constant>`.  "Enabled by" is where
a configuration turns the feature on: `Cargo.toml`'s `[features]` table (all off today), a
`xtask dead-arms` flip, or "-" for a feature nothing enables.  The constant column names the
`config.rs` constant a flip would have to move with it, derived by upper-casing the feature name
and checked against the generated file - `-` means there is none, which is a thing to know before
flipping.
"""

from __future__ import annotations

import argparse
import re
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
# Every *ported* tree, not just the two the first 350 units came from: batch F's programs and
# libraries (`lib-src`, `exec`, `lwlib`, `oldXMenu`, `nt`, `msdos`) carry `#[cfg]` sites of their
# own - `nt/runemacs.rs`'s `choose_newest_exe` was the first one this table could not see, because
# the walk stopped at the two core trees (status 3.469; `check-declarations.py`'s `SCAN_DIRS` lists
# the same eight trees for the same reason).  `xtask` stays out: it is the tooling, not a port.
TREES = ("src", "lib", "lib-src", "exec", "lwlib",
         "oldXMenu", "nt", "msdos")
FEATURE = re.compile(r'feature\s*=\s*"([a-z0-9_]+)"')
FLIP = re.compile(r'\(\s*"([a-z0-9_]*)"\s*,\s*"([A-Z0-9_]*)"\s*,\s*"(\w+)"\s*\)')
CONFIG = REPO / "src/include/config.rs"
CARGO = REPO / "Cargo.toml"
XTASK = REPO / "xtask/src/main.rs"
OUT = REPO / "docs/baseline/feature-coverage.tsv"


def scanner():
    """`tools/check-declarations.py`'s comment/string splitter, so a `feature = "..."` inside a
    doc comment or a C docstring quoted in a literal is not counted as a site (the ported units
    quote C's text, and that text mentions features)."""
    import importlib.util
    spec = importlib.util.spec_from_file_location("decl", REPO / "tools/check-declarations.py")
    mod = importlib.util.module_from_spec(spec)
    assert spec.loader is not None
    spec.loader.exec_module(mod)
    return mod.scan


def sites() -> dict[str, tuple[int, set[str]]]:
    scan = scanner()
    out: dict[str, tuple[int, set[str]]] = {}
    for tree in TREES:
        for path in sorted((REPO / tree).rglob("*.rs")):
            rel = path.relative_to(REPO).as_posix()
            text = path.read_text(encoding="utf-8", errors="replace")
            _, comment = scan(text)
            # `scan()`'s code half has the string delimiters removed (it models what is *inside* a
            # literal), so the regex runs on the raw line with the comment half cut out of it:
            # comments are where this tree talks about features most, and a literal is left whole.
            for i, raw in enumerate(text.split("\n")):
                half = comment[i] if i < len(comment) else ""
                line = raw.replace(half, "", 1) if half else raw
                for name in FEATURE.findall(line):
                    n, files = out.get(name, (0, set()))
                    out[name] = (n + 1, files | {rel})
    return out


def enabled() -> dict[str, str]:
    """feature -> where a configuration turns it on."""
    out: dict[str, str] = {}
    if CARGO.is_file():
        body = CARGO.read_text(encoding="utf-8")
        if "[features]" in body:
            table = body.split("[features]", 1)[1].split("\n[", 1)[0]
            for line in table.split("\n"):
                name = line.split("=")[0].strip()
                if name and not name.startswith("#") and not name.startswith("["):
                    out[name] = "Cargo.toml [features]"
    if XTASK.is_file():
        for feature, _, _ in FLIP.findall(XTASK.read_text(encoding="utf-8")):
            if feature:
                out[feature] = "xtask dead-arms"
    return out


def constants() -> set[str]:
    if not CONFIG.is_file():
        return set()
    return set(re.findall(r"pub const ([A-Z0-9_]+)\s*:", CONFIG.read_text(encoding="utf-8")))


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--write", action="store_true")
    args = ap.parse_args()
    used = sites()
    on = enabled()
    const = constants()
    rows = []
    for name, (n, files) in sorted(used.items(), key=lambda kv: (-kv[1][0], kv[0])):
        cname = name.upper()
        rows.append((name, n, len(files), on.get(name, "-"),
                     cname if cname in const else "-"))
    never = [r for r in rows if r[3] == "-"]
    if args.write:
        body = ["# `#[cfg (feature = \"...\")]` sites in the ported trees and whether any",
                "# configuration enables them.  Written by `python3 tools/feature-coverage.py",
                "# --write` (docs/status.md 3.263).  Columns: feature, sites, files, enabled by,",
                "# config constant (the `config.rs` constant a flip of this feature must move",
                "# with it; `-` means there is none).  A feature nothing enables is text that has",
                "# never been through rustc (PORTING.md P-51 carries it on purpose).",
                "feature\tsites\tfiles\tenabled_by\tconfig_constant"]
        body += ["\t".join(str(x) for x in r) for r in rows]
        OUT.write_text("\n".join(body) + "\n", encoding="utf-8")
        print(f"wrote {OUT.relative_to(REPO)}: {len(rows)} feature(s), {len(never)} never enabled")
        return 0
    print(f"{len(rows)} feature(s); {len(never)} never enabled by any configuration:")
    for name, n, files, _, cname in never:
        print(f"  {name:28s} {n:5d} site(s) in {files:4d} file(s), constant {cname}")
    print(f"enabled: " + ", ".join(sorted(on)))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
