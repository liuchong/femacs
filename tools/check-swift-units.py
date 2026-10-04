#!/usr/bin/env python3
"""C-side coverage for the Swift units (docs/status.md 3.164).

`tools/extract-abi.py` has no Swift support at all, so before this gate every `.swift`
target of batch F read as if *every* C function of its `.m` source were missing
(`--check emacs/src/nsfont.m femacs/platform/macos/nsfont.swift` reports all 36), and the
units' real coverage lived only in hand-written ledger boxes (status 3.160 recorded the
gap).  This tool is the missing half, kept separate from `extract-abi.py` on purpose: it
compares **function-name sets** - C definitions (from `tools/cinventory.py`, the same
reader the ledger checklists use) against Swift `func` definitions - and says nothing
about signatures, bodies, layout or ABI, all of which stay where they were (the per-file
`swiftc -c` compile, `nm` for the exported symbols, and each ledger's Decisions).

Two classes are *failures*, the same way `docs/baseline/declarations-allowlist.tsv`
counts a deviation instead of hiding it:

  new    a C definition with no Swift `func` that is not named in the manifest
  stale  a manifest row whose C definition now *has* a Swift `func`

`@_silgen_name("…") func x (…)` with no body is C's `extern` declaration in Swift's
spelling - it imports a symbol, it does not port one - so those names are reported as
*imports* (informational) and never counted as coverage.  A Swift `func` with no C
counterpart is reported as `extra` (informational): helpers like `lispString` are
legitimate, and `nm` on the built file is what decides the exported surface.

    python3 tools/check-swift-units.py                 # compare with the manifest
    python3 tools/check-swift-units.py --write-manifest
    python3 tools/check-swift-units.py --selftest
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(REPO / "tools"))
import cinventory  # noqa: E402  (the C-side reader the ledger checklists use)

MANIFEST = REPO / "docs" / "baseline" / "swift-units.tsv"

# The Swift table of docs/port-map.md: `| `src/nsfont.m` | 1,765 | `…/nsfont.swift` | … |`
PAIR = re.compile(r"^\|\s*`(?P<c>src/[^`]+\.m)`\s*\|\s*[\d,]+\s*\|\s*"
                  r"`(?P<s>femacs/platform/macos/[^`]+\.swift)`\s*\|", re.M)

_FUNC = re.compile(r"\bfunc\s+([A-Za-z_]\w*)\s*(?:<[^>{}]*>)?\s*\(")
_SILGEN = re.compile(r'@_silgen_name\s*\(\s*"([^"]+)"\s*\)')


def strip_swift(text: str) -> str:
    """Blank comments and string literals, keeping the line structure.

    Swift has `//`, nestable `/* */` and three string forms (`"…"`, `\"\"\"…\"\"\"`, and
    `\\(…)` interpolation).  Interpolation bodies are *code*, but a name inside one cannot
    be a definition, so blanking the whole literal is the safe reading for this gate.
    """
    out = list(text)
    i, n = 0, len(text)
    while i < n:
        c = text[i]
        if c == "/" and i + 1 < n and text[i + 1] == "/":
            while i < n and text[i] != "\n":
                out[i] = " "
                i += 1
        elif c == "/" and i + 1 < n and text[i + 1] == "*":
            depth = 0
            while i < n:
                if text.startswith("/*", i):
                    depth += 1
                    out[i] = out[i + 1] = " "
                    i += 2
                elif text.startswith("*/", i):
                    depth -= 1
                    out[i] = out[i + 1] = " "
                    i += 2
                    if depth == 0:
                        break
                else:
                    if text[i] != "\n":
                        out[i] = " "
                    i += 1
        elif text.startswith('"""', i):
            i += 3
            while i < n and not text.startswith('"""', i):
                if text[i] != "\n":
                    out[i] = " "
                i += 1
            for k in range(i, min(i + 3, n)):
                out[k] = " "
            i += 3
        elif c == '"':
            i += 1
            while i < n and text[i] != '"':
                if text[i] == "\\":
                    out[i] = " "
                    i += 1
                    if i < n:
                        out[i] = " "
                        i += 1
                    continue
                if text[i] != "\n":
                    out[i] = " "
                i += 1
            if i < n:
                out[i] = " "
            i += 1
        else:
            i += 1
    return "".join(out)


def _after_parens(text: str, i: int) -> int | None:
    """Index just past the `)` matching the `(` at `i`, or None."""
    depth = 0
    while i < len(text):
        if text[i] == "(":
            depth += 1
        elif text[i] == ")":
            depth -= 1
            if depth == 0:
                return i + 1
        i += 1
    return None


def swift_items(text: str) -> tuple[list[str], list[str], list[str]]:
    """(definitions, imports, extras-shaped) - see the module docstring.

    Returns `(definitions, imports_of_c_symbols, opaque)` where `opaque` lists the Swift
    names whose body could not be found on this line or the next (kept visible rather than
    silently counted either way).
    """
    flat = strip_swift(text)
    # `strip_swift` blanks string literals, `@_silgen_name("…")`'s argument among them, so the
    # import test runs on the *original* text: a `func` is an import when the only thing between
    # the attribute's `)` and the `func` keyword is whitespace (positions are identical - the
    # stripper keeps the text's length).
    silgen = [(m.start(), m.end()) for m in _SILGEN.finditer(text)]

    def is_import(pos: int) -> bool:
        for _, end in silgen:
            if end <= pos and text[end:pos].strip() == "":
                return True
        return False

    defs: list[str] = []
    imports: list[str] = []
    opaque: list[str] = []
    for m in _FUNC.finditer(flat):
        name = m.group(1)
        end = _after_parens(flat, flat.index("(", m.start()))
        if end is None:
            opaque.append(name)
            continue
        k, body = end, False
        while k < len(flat):
            ch = flat[k]
            if ch == "{":
                body = True
                break
            if ch == "\n":
                nxt = re.compile(r"[ \t]*(\S)").search(flat, k)
                if nxt is None:
                    break
                nline = flat.count("\n", 0, nxt.start())
                if nline - flat.count("\n", 0, k) > 1 or nxt.group(1) in "@f}#":
                    break
            k += 1
        if body:
            defs.append(name)
        elif is_import(m.start()):
            imports.append(name)
        else:
            opaque.append(name)
    return defs, imports, opaque


def pairs() -> list[tuple[str, str]]:
    text = (REPO / "docs" / "port-map.md").read_text(encoding="utf-8")
    return [(m.group("c"), m.group("s")) for m in PAIR.finditer(text)]


def survey() -> dict[str, tuple[list[str], list[str], list[str], list[str]]]:
    """`swift target -> (c_functions, c_missing, imports, opaque)` per landed pair."""
    out: dict[str, tuple[list[str], list[str], list[str], list[str]]] = {}
    for c_rel, s_rel in pairs():
        s_path = REPO / s_rel
        if not s_path.is_file():
            continue
        c_funcs = [f["name"] for f in cinventory.inventory(REPO / "emacs" / c_rel)["functions"]]
        defs, imports, opaque = swift_items(s_path.read_text(encoding="utf-8", errors="replace"))
        missing = [n for n in c_funcs if n not in set(defs)]
        out[s_rel] = (c_funcs, missing, imports, opaque)
    return out


def read_manifest() -> dict[str, set[str]]:
    rows: dict[str, set[str]] = {}
    if not MANIFEST.is_file():
        return rows
    for line in MANIFEST.read_text(encoding="utf-8").splitlines():
        if not line.strip() or line.startswith("#"):
            continue
        parts = line.split("\t")
        if len(parts) < 2:
            continue
        rows.setdefault(parts[0], set()).add(parts[1])
    return rows


def read_manifest_reasons() -> dict[tuple[str, str], str]:
    """`(target, name) -> reason` from the manifest's optional fourth column."""
    out: dict[tuple[str, str], str] = {}
    if not MANIFEST.is_file():
        return out
    for line in MANIFEST.read_text(encoding="utf-8").splitlines():
        if not line.strip() or line.startswith("#"):
            continue
        parts = line.split("\t")
        if len(parts) >= 4 and parts[3].strip():
            out[(parts[0], parts[1])] = parts[3]
    return out


def write_manifest(state: dict) -> None:
    # The reason column is free text: a row that can never clear says why.  Rewriting keeps the
    # rows that stay *and* their reasons - dropping them was a defect this writer had until status
    # 3.182, found by the lane whose notes it ate.
    header = [
        "# The Swift units' unported C definitions, one row per name (docs/status.md 3.164).",
        "# Written by `python3 tools/check-swift-units.py --write-manifest`.  A name that",
        "# leaves this file must have landed as a Swift `func` (the gate reports a row that",
        "# no longer matches as `stale`, and a missing C definition with no row as `new`).",
        "# Columns: <swift target>, <C definition>, <C source>, <reason>.",
        "swift\tmissing_c_definition\tc_source\treason",
    ]
    kept = read_manifest_reasons()
    c_of = {s: c for c, s in pairs()}
    body = []
    for target in sorted(state):
        for name in sorted(state[target][1]):
            reason = kept.get((target, name), "")
            row = f"{target}\t{name}\t{c_of[target]}"
            body.append(row + (f"\t{reason}" if reason else ""))
    MANIFEST.write_text("\n".join(header + body) + "\n", encoding="utf-8")


def selftest() -> int:
    cases = [
        ("a definition and an import are told apart",
         'fileprivate func ns_get_family (_ spec: Lisp_Object) -> NSString? {\n  return nil\n}\n'
         '@_silgen_name("xfree") func xfree(_ p: UnsafeMutableRawPointer?)\n',
         (["ns_get_family"], ["xfree"], [])),
        ("a body on the next line still counts",
         'private func helper (_ a: Int)\n  -> Bool\n{\n  return true\n}\n'
         'private func declared (_ a: Int) -> Bool\n',
         (["helper"], [], ["declared"])),
        ("strings and comments cannot invent a definition",
         '/* func not_a_function () { } */\nlet s = "func also_not () {"\n'
         'func real () { }\n// func commented_out () { }\n',
         (["real"], [], [])),
    ]
    bad = 0
    for name, src, want in cases:
        got = swift_items(src)
        ok = got == want
        print(f"{'ok  ' if ok else 'FAIL'} {name}: {got}" + ("" if ok else f" (want {want})"))
        bad += 0 if ok else 1
    print(f"{len(cases)} cases, {bad} failures")
    return 1 if bad else 0


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--write-manifest", action="store_true")
    ap.add_argument("--selftest", action="store_true")
    args = ap.parse_args()
    if args.selftest:
        return selftest()
    state = survey()
    if args.write_manifest:
        write_manifest(state)
        print(f"wrote {MANIFEST.relative_to(REPO)}: "
              f"{sum(len(v[1]) for v in state.values())} row(s), {len(state)} Swift unit(s)")
        return 0
    manifest = read_manifest()
    new, stale = [], []
    for target in sorted(state):
        c_funcs, missing, imports, opaque = state[target]
        rows = manifest.get(target, set())
        new += [(target, n) for n in missing if n not in rows]
        stale += [(target, n) for n in sorted(rows) if n not in missing]
        extra = [n for n in imports]
        print(f"{target}: {len(c_funcs)} C definition(s), {len(missing)} unported, "
              f"{len(rows)} row(s); {len(extra)} C symbol import(s)"
              + (f", {len(opaque)} declaration(s) with no body" if opaque else ""))
    for target, name in new:
        print(f"new\t{target}\t{name}\tC definition has no Swift func and no manifest row")
    for target, name in stale:
        print(f"stale\t{target}\t{name}\tmanifest row no longer matches (it landed?)")
    print(f"{len(new)} new, {len(stale)} stale, {len(state)} Swift unit(s) landed")
    return 1 if new or stale else 0


if __name__ == "__main__":
    sys.exit(main())
