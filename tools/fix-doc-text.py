#!/usr/bin/env python3
"""Rewrite a port docstring carrier so that its rendered record matches the reference byte
for byte (PORTING.md P-109, which the DOC gate of P-108 clause 4 measures).

For every record whose text drifts, this tool

1. finds the reference's own macro invocation for that name and reads its docstring the
   way `make-docfile` does (P-109: the comment's text with its escapes applied),
2. renders it and checks that the result *is* the reference DOC's record for that
   (section, name); if not, the case is skipped and reported, never guessed at,
3. rewrites the port's string literal with the reference's text, leaving every other byte
   of the invocation alone, and refuses any span that does not decode to the carrier text
   it is about to replace.

The port's literal therefore holds C's text after `make-docfile`'s unescaping (P-109
clause 1), which is what a Rust string literal can carry exactly.

Usage
-----
    python3 tools/fix-doc-text.py              # dry run: report what would change
    python3 tools/fix-doc-text.py --write      # apply
    python3 tools/fix-doc-text.py --section lread.o --write    # one section
"""

from __future__ import annotations

import argparse
import importlib.util
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent


def load_gen_doc():
    spec = importlib.util.spec_from_file_location("gen_doc", REPO / "tools/gen-doc.py")
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


gd = load_gen_doc()


def rust_literal(text: str) -> str:
    out = ['"']
    for ch in text:
        if ch == "\\":
            out.append("\\\\")
        elif ch == '"':
            out.append('\\"')
        elif ch == "\n":
            out.append("\\n")
        elif ch == "\t":
            out.append("\\t")
        elif ch == "\r":
            out.append("\\r")
        else:
            out.append(ch)
    out.append('"')
    return "".join(out)


def port_literal_span(text: str, current: str) -> tuple[int, int] | None:
    """The absolute span of the port's docstring literal that decodes to `current`."""
    macro = re.compile(r"\b(DEFUN|DEFVAR_[A-Z_0-9]+)!\s*\(")
    m = macro.search(text)
    while m:
        paren = text.index("(", m.start())
        args, end = gd.split_args(text, paren)
        cursor = paren
        for a in args:
            start = text.find(a, cursor)
            if start < 0:
                break
            cursor = start + len(a)
            t = a.strip()
            if t.startswith("doc:"):
                t = t[4:].strip()
            if not (t.startswith('"') or t.startswith('c"') or t.startswith('r"')):
                continue
            quote = start + a.index('"')
            try:
                value, lit_end = gd.string_literal(text, quote)
            except ValueError:
                continue
            if value == current:
                return quote, lit_end
        m = macro.search(text, end)
    return None


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--write", action="store_true")
    ap.add_argument("--section")
    ap.add_argument("--limit", type=int, default=200)
    a = ap.parse_args()

    ref = gd.DEFAULT_REFERENCE.read_bytes()
    built, _, _ = gd.build(ref)
    want: dict[tuple[str, str], bytes] = {}
    kinds: dict[tuple[str, str], str] = {}
    for o, recs in gd.split_doc(ref):
        for k, n, d in recs:
            want[(o, n)] = d
            kinds[(o, n)] = k
    got = {(o, n): d for o, recs in gd.split_doc(built) for _, n, d in recs}
    drift = [(o, n) for o, n in want if (o, n) in got and want[o, n] != got[o, n]]
    if a.section:
        drift = [k for k in drift if k[0] == a.section]

    planned: dict[Path, list[tuple[str, str, str]]] = {}
    skipped: list[str] = []
    for obj, name in drift:
        stem = gd.section_stem(obj)
        src = gd.port_source(stem)
        if src.suffix == ".swift":
            skipped.append(f"{obj} {name}: Swift carrier (P-106)")
            continue
        kind = kinds[(obj, name)]
        cfile, _, _ = gd.c_origin(stem, name, kind)
        if cfile is None:
            skipped.append(f"{obj} {name}: no reference invocation found")
            continue
        got_c = gd.read_c_docstring(cfile, name, kind)
        if got_c is None:
            skipped.append(f"{obj} {name}: no docstring read from {cfile.name}")
            continue
        text, cline = got_c[0], got_c[1]
        rec = next((r for r in gd.scan_rust(src) if r.name == name and r.kind == kind), None)
        if rec is None:
            skipped.append(f"{obj} {name}: carrier not scanned in {src.name}")
            continue
        probe = gd.Record(rec.kind, rec.name, text, rec.minargs, rec.maxargs, rec.args,
                          f"{cfile.name}:{cline}")
        rendered = gd.render_doc(probe, [])
        if rendered != want[obj, name]:
            skipped.append(f"{obj} {name}: reference text does not render to the record "
                           f"({len(rendered)} vs {len(want[obj, name])} bytes)")
            continue
        planned.setdefault(src, []).append((name, rust_literal(text), rec.doc))

    changed = 0
    for src, items in sorted(planned.items()):
        text = src.read_text(encoding="utf-8")
        for name, literal, current in items:
            span = port_literal_span(text, current)
            if span is None:
                skipped.append(f"{src.name} {name}: literal span not located")
                continue
            if span[1] - span[0] == len(literal) and text[span[0] + 1:span[1] - 1] == literal[1:-1]:
                continue
            before = text[span[0]:span[1]]
            text = text[:span[0]] + literal + text[span[1]:]
            changed += 1
            print(f"  {src.name} {name}: {len(before)} -> {len(literal)} bytes")
        if a.write:
            src.write_text(text, encoding="utf-8")

    print(f"records drifting: {len(drift)}; literal rewrites: {changed}; "
          f"{'written' if a.write else 'dry run'}")
    for s in skipped[:a.limit]:
        print(f"  skip: {s}")
    if len(skipped) > a.limit:
        print(f"  … {len(skipped) - a.limit} more skipped")
    return 0 if not skipped else 1


if __name__ == "__main__":
    sys.exit(main())
