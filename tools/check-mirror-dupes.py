#!/usr/bin/env python3
"""Find declarations a unit repeats although a mirror in its own include graph carries them.

PORTING.md P-26 asks for one spelling per construct.  A header mirror that a unit `include!`s
puts its declarations into that unit's scope, so a second, identical `unsafe extern "C" { fn
name (); }` in the unit is a copy that a local item silently shadows - it compiles, it passes
every text gate, and it is exactly what turns into `clashing_extern_declarations` when the two
copies later disagree (the warning gate-fast's unit builds already print, status 3.75).

    python3 tools/check-mirror-dupes.py               # scan the tree against the ratchet
    python3 tools/check-mirror-dupes.py --write-ratchet
    python3 tools/check-mirror-dupes.py --selftest

Only *in-scope* mirrors count: the include graph is followed from the unit, and a mirror's
declarations count when they sit at the mirror's top level (where `include!` is textual, so they
land in the unit's crate root) or inside a module the mirror glob-imports (`use mod::*;`).
A module-local declaration in the unit is a different scope and is not a duplicate.  Declarations
with a body are definitions, not copies, and are never reported.

`docs/baseline/mirror-dupes.tsv` is the ratchet: `<unit>\t<mirror>\t<name>`, one per line.  A
hit with no row and a row with no hit are both failures; `--write-ratchet` rewrites the file from
the tree (rows carry the reason column for the human reader).
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
RATCHET = REPO / "docs/baseline/mirror-dupes.tsv"
UNIT_DIRS = ("femacs/src", "femacs/lib")
MIRROR_DIRS = ("femacs/src/include", "femacs/lib/include")

INCLUDE = re.compile(r'include!\(\s*"([^"]+)"\s*\)')
MODULE = re.compile(r"^\s*(?:pub\s+)?mod\s+([A-Za-z_]\w*)\s*\{")
GLOB_USE = re.compile(r"^\s*use\s+([A-Za-z_]\w*)\s*::\s*\*;")
FN_DECL = re.compile(r"^(?:pub\s+)?(?:unsafe\s+)?(?:extern\s+\"C\"\s+)?fn\s+([A-Za-z_]\w*)\s*\(")
# Constants and statics are the other half of the same question, and nothing else looks for
# them: a `const NAME` a unit keeps while a mirrored header declares it is an `E0428` the
# moment the mirror lands (status 3.80's sweep missed `EDOM` in two units, and the whole-tree
# build was what caught it; status 3.83 teaches this tool to see it first).
# The name must start with a letter: `const _: () = assert!(…)` is the layout-assertion
# shape every unit repeats on purpose, and it is not a declaration of anything.
ITEM_DECL = re.compile(r"^(?:pub\s+)?(?:static\s+mut\s+|const\s+)([A-Za-z]\w*)\s*[:=]")


def declarations_in(text: str, scope: str, out: dict[str, list[str]]) -> None:
    """Collect the bodyless `fn` declarations in one run of extern-block text."""
    for segment in text.split(";")[:-1]:          # only segments a `;` terminated
        if "}" in segment:
            continue                              # the block closed before this segment
        m = FN_DECL.match(segment.strip())
        if m:
            out.setdefault(scope, []).append(m.group(1))


EXTERN_OPEN = re.compile(r'(?:pub\s+)?unsafe\s+extern\s+"C"\s*\{')
MODULE_OPEN = re.compile(r"(?:^|\s)(?:pub\s+)?mod\s+([A-Za-z_]\w*)\s*\{")


def scan(path: Path) -> tuple[dict[str, list[str]], dict[str, list[str]]]:
    """`{scope: [declared names]}` for one file, `scope` being "" (top level) or a module name.

    Only bodyless declarations inside `unsafe extern "C"` blocks are collected: a `fn` with a
    body is a definition, and a definition shadowing a mirror's declaration is the intended
    shape (PORTING.md 0.1-10).  A block may open and hold declarations on one line, so the line
    is cut at the opener rather than assumed to be a lone `{` (the same for `mod`, whose body
    may also start on the opening line).
    """
    fns: dict[str, list[str]] = {}
    consts: dict[str, list[str]] = {}
    out = fns                      # `declarations_in` appends here
    text = path.read_text(encoding="utf-8", errors="replace")
    stack: list[tuple[str, int]] = []             # (module name, brace depth at entry)
    extern_depth: int | None = None
    depth = 0
    prev_item_line = ""
    for raw in text.split("\n"):
        line = raw.rstrip()
        # An item behind `#[cfg (…)]`/`#[cfg_attr (…)]` is a carried arm of the *other*
        # configuration, not an item of the frozen one: `lib/include/acl-internal.rs`'s
        # `#[cfg (feature = "no_fchmod")] pub const HAVE_FCHMOD` is the worked example (the
        # `dead-arms` pass is what exercises those arms, and it compiles the whole tree).
        gated = prev_item_line.startswith("#[cfg (") or prev_item_line.startswith("#[cfg_attr (")
        if line.strip() and not line.strip().startswith("//"):
            prev_item_line = line.strip()
        m = MODULE_OPEN.search(line)
        if m:
            stack.append((m.group(1), depth))
        scope = stack[-1][0] if stack else ""
        cut = 0
        if extern_depth is None:
            m_open = EXTERN_OPEN.search(line)
            if m_open:
                extern_depth = depth + line[:m_open.start()].count("{") - line[:m_open.start()].count("}")
                cut = m_open.end()
        if extern_depth is not None and not gated:
            declarations_in(line[cut:], scope, out)
        else:
            # A `const`/`static` item at this level: the mirror's copy is textually in
            # scope, so a unit keeping its own is the same duplicate (status 3.83).
            m_item = None if gated else ITEM_DECL.match(line.strip())
            if m_item and line.rstrip().endswith(";"):
                consts.setdefault(scope, []).append(m_item.group(1))
        depth += line.count("{") - line.count("}")
        while stack and depth <= stack[-1][1]:
            stack.pop()
        if extern_depth is not None and depth <= extern_depth:
            extern_depth = None
    return fns, consts


def visible_declarations(mirror: Path) -> tuple[set[str], set[str]]:
    """What an `include!` of `mirror` puts into the including unit's scope.

    Two sets, because the two kinds collide differently.  A *function* declaration in a module
    the mirror glob-imports is shadowed by a unit's own item - legal, and still a copy PORTING.md
    P-26 asks to drop - so functions count from the top level and from every glob-imported
    module.  A *constant* in a glob-imported module is a different case: shadowing it is the
    documented shape for a carrier unit (`lib/acl-internal.rs` defines `ACL_INTERNAL_INLINE`,
    `lib/set-permissions.rs` its own `HAVE_FCHMOD` - both legal, both still compile), while a
    *top-level* constant in the mirror is textually in the unit's scope and a second one there is
    an `E0428` (that is what status 3.80's `EDOM` sweep walked into).  So only top-level
    constants count.
    """
    fns, consts = scan(mirror)
    fn_names = set(fns.get("", []))
    text = mirror.read_text(encoding="utf-8", errors="replace")
    for line in text.split("\n"):
        m = GLOB_USE.match(line)
        if m:
            fn_names.update(fns.get(m.group(1), []))
    return fn_names, set(consts.get("", []))


def include_graph(unit: Path, seen: set[Path] | None = None) -> set[Path]:
    seen = seen if seen is not None else set()
    if unit in seen:
        return seen
    seen.add(unit)
    text = unit.read_text(encoding="utf-8", errors="replace")
    for line in text.split("\n"):
        m = INCLUDE.search(line)
        if not m:
            continue
        target = (unit.parent / m.group(1)).resolve()
        if target.is_file() and target not in seen:
            include_graph(target, seen)
    return seen


def findings(root: Path) -> list[tuple[str, str, str]]:
    mirrors = {p.resolve(): p for d in MIRROR_DIRS for p in (root / d).glob("*.rs")}
    visible = {p: visible_declarations(p) for p in mirrors}
    out: list[tuple[str, str, str]] = []
    for d in UNIT_DIRS:
        for unit in sorted((root / d).glob("*.rs")):
            graph = include_graph(unit.resolve())
            unit_fns, unit_consts = scan(unit)
            declared_fns = set(unit_fns.get("", []))
            declared_consts = set(unit_consts.get("", []))
            if not declared_fns and not declared_consts:
                continue
            for target in sorted(graph):
                if target not in visible:
                    continue
                mirror_fns, mirror_consts = visible[target]
                shared = (declared_fns & mirror_fns) | (declared_consts & mirror_consts)
                for name in sorted(shared):
                    out.append((unit.relative_to(root).as_posix(),
                                mirrors[target].relative_to(root).as_posix(), name))
    return out


SELFTEST = (
    # (name, unit source, mirror source, expected finding count)
    ("top-level copy counts",
     'include!("include/m.rs");\nunsafe extern "C" { fn foo (); }', 
     'unsafe extern "C" { pub fn foo (); }', 1),
    ("module-local declaration does not count",
     'include!("include/m.rs");\nmod inner { unsafe extern "C" { fn foo (); } }',
     'unsafe extern "C" { pub fn foo (); }', 0),
    ("a definition is not a copy",
     'include!("include/m.rs");\nunsafe extern "C" fn foo () { }',
     'unsafe extern "C" { pub fn foo (); }', 0),
    ("a mirror the unit does not include does not count",
     'unsafe extern "C" { fn foo (); }',
     'unsafe extern "C" { pub fn foo (); }', 0),
    ("glob-imported module declarations count",
     'include!("include/m.rs");\nunsafe extern "C" { fn foo (); }',
     'pub mod decls { unsafe extern "C" { pub fn foo (); } }\nuse decls::*;', 1),
    ("a module the mirror does not glob-import does not count",
     'include!("include/m.rs");\nunsafe extern "C" { fn foo (); }',
     'pub mod decls { unsafe extern "C" { pub fn foo (); } }', 0),
    # The constant class status 3.83 added: a top-level constant in the mirror is textually
    # in the unit's scope, so a second one is an E0428 (this is how `EDOM` bit); a module
    # constant and a cfg-gated one are not.
    ("top-level const counts", 'include!("include/m.rs");\npub const EDOM: c_int = 33;',
     "pub const EDOM: c_int = 33;", 1),
    ("module const does not count", 'include!("include/m.rs");\npub const EDOM: c_int = 33;',
     "pub mod d { pub const EDOM: c_int = 33; }\nuse d::*;", 0),
    ("cfg-gated mirror const does not count",
     'include!("include/m.rs");\npub const EDOM: c_int = 33;',
     '#[cfg (feature = "x")]\npub const EDOM: c_int = 33;', 0),
)


def selftest() -> int:
    import tempfile
    bad = 0
    for name, unit_src, mirror_src, want in SELFTEST:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            # The layout mirrors the repository's: the unit sits in `femacs/src/` and its
            # `include!("include/m.rs")` resolves to `femacs/src/include/m.rs`.
            for d in ("femacs/src/include", "femacs/lib", "femacs/lib/include"):
                (root / d).mkdir(parents=True, exist_ok=True)
            (root / "femacs/src/include/m.rs").write_text(mirror_src + "\n")
            (root / "femacs/src/u.rs").write_text(unit_src + "\n")
            got = len(findings(root))
        ok = got == want
        print(f"{'ok  ' if ok else 'FAIL'} {name}: {got} (want {want})")
        bad += 0 if ok else 1
    print(f"{len(SELFTEST)} cases, {bad} failures")
    return 1 if bad else 0


def read_ratchet(path: Path) -> set[tuple[str, str, str]]:
    rows = set()
    if not path.is_file():
        return rows
    for raw in path.read_text(encoding="utf-8").splitlines():
        if not raw.strip() or raw.startswith("#"):
            continue
        parts = raw.split("\t")
        if len(parts) < 3:
            print(f"BAD ROW {path.name}: {raw!r}")
            continue
        rows.add((parts[0], parts[1], parts[2]))
    return rows


def write_ratchet(path: Path, hits: list[tuple[str, str, str]]) -> None:
    # A row needs a reason a reader can act on; the generator ships an explicit TODO for the
    # reviewer to replace, the way `--write-pending` does for the undefined-reference ratchet.
    old = {}
    if path.is_file():
        for raw in path.read_text(encoding="utf-8").splitlines():
            parts = raw.split("\t")
            if len(parts) >= 4 and not raw.startswith("#"):
                old[(parts[0], parts[1], parts[2])] = parts[3]
    body = ["# Declarations a unit repeats although a mirror in its own include graph",
            "# already carries them (PORTING.md P-26; docs/status.md 3.75/3.76).  Written by",
            "# `python3 tools/check-mirror-dupes.py --write-ratchet`.  A new hit fails the gate;",
            "# deleting one is a two-step job per unit: drop the copy, `build-tu <unit>`, then",
            "# `extract-abi.py --check <c> <rs>` and re-run this tool.",
            "# <unit>\t<mirror>\t<name>\t<reason>"]
    for unit, mirror, name in sorted(hits):
        reason = old.get((unit, mirror, name), "TODO: drop the unit's copy and rebuild")
        body.append(f"{unit}\t{mirror}\t{name}\t{reason}")
    path.write_text("\n".join(body) + "\n", encoding="utf-8")


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--root", default=str(REPO))
    ap.add_argument("--write-ratchet", action="store_true")
    ap.add_argument("--selftest", action="store_true")
    args = ap.parse_args()
    if args.selftest:
        return selftest()

    root = Path(args.root)
    hits = findings(root)
    if args.write_ratchet:
        write_ratchet(RATCHET, hits)
        print(f"wrote {RATCHET.relative_to(REPO)}: {len(hits)} row(s)")
        return 0

    ratchet = read_ratchet(RATCHET)
    found = set(hits)
    failures = 0
    for unit, mirror, name in sorted(found - ratchet):
        print(f"FAIL {unit}: {name} is already declared by {mirror} (in the include graph)")
        failures += 1
    for unit, mirror, name in sorted(ratchet - found):
        print(f"STALE {unit}: {name} from {mirror} is gone - drop the row")
        failures += 1
    print(f"{len(hits)} duplicate declaration(s), {len(ratchet)} ratchet row(s), "
          f"{failures} failure(s)")
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
