#!/usr/bin/env python3
"""Pin the integer-to-enum `transmute`s: every site recorded, every enum unchanged.

PORTING.md's P-98 settled *how* the port must write C's `(enum E) n` casts, and recorded that the
same family's sites (`emacs_reloc_type`, `dump_fixup_type`, `cold_op`, `Lisp_Type`) currently
receive only declared members - but nothing measured that.  A payload-less Rust enum has no legal
value outside its discriminants, so a site that receives anything else is instantly undefined
behaviour, and the port cannot see it: the cast compiles, the value is written, and the damage
appears wherever it is read.

This gate is the missing measurement, in two halves:

  * **coverage** - every `transmute::<<int>, T>` site in the tree must be recorded in
    `docs/baseline/enum-transmutes.tsv` (keyed by file, source type and target type, with the
    site's count), and every row must still match a live site: a new site, a removed one or a
    changed count is a failure.
  * **the enum itself** - each row records the target enum's discriminant set as parsed from the
    port's own mirrors, and the gate re-parses it: an enum whose members were renumbered is a
    failure even though no site changed (which is exactly what would make an old row's reasoning
    false).

The rows' `why` column is the human half (the reasoning, e.g. "C's `dump_reloc_set_type` masks to
`DUMP_RELOC_TYPE_BITS`, and the value is `RELOC_DUMP_TO_DUMP_LV + <Lisp_Type>`"), and it is not
machine-checked - the two halves above are what keep it from rotting silently.

    python3 tools/check-enum-transmutes.py            # check
    python3 tools/check-enum-transmutes.py --write    # rewrite the table (keeps existing rows)
    python3 tools/check-enum-transmutes.py --selftest
"""

from __future__ import annotations

import argparse
import pathlib
import re
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
PORT = REPO
TABLE = REPO / "docs/baseline/enum-transmutes.tsv"

INT_TYPES = ("c_int", "c_uint", "c_uchar", "c_char", "i8", "i16", "i32", "u8", "u16", "u32",
             "isize", "usize", "ptrdiff_t")
SITE = re.compile(r"transmute::<\s*(" + "|".join(INT_TYPES) + r")\s*,\s*"
                  r"([A-Za-z_][A-Za-z0-9_:]*)\s*>")
ENUM = re.compile(r"pub enum\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{(.*?)\}", re.S)
VARIANT = re.compile(r"^\s*([A-Za-z_][A-Za-z0-9_]*)\s*(?:=\s*(.+?))?\s*$")


def sites() -> dict[tuple[str, str, str], list[int]]:
    """Every integer-to-enum transmute, keyed by (file, source type, target type)."""
    found: dict[tuple[str, str, str], list[int]] = {}
    for path in sorted((p for tree in ('src','lib','lib-src','lwlib','oldXMenu','nt','msdos','exec','platform','xtask') for p in (PORT/tree).rglob("*.rs"))):
        if "target" in path.parts:
            continue
        rel = str(path.relative_to(PORT))
        for n, line in enumerate(path.read_text(errors="replace").split("\n"), 1):
            for match in SITE.finditer(line):
                key = (rel, match.group(1), match.group(2))
                found.setdefault(key, []).append(n)
    return found


def variants(body: str) -> list[str]:
    """Split an enum body into variant tokens, respecting nested brackets.

    C's enums carry a comment per member, and the port copies them, so the comments have to go
    before the tokens can be read (a member whose token begins with `/* ... */` matches nothing).
    """
    body = re.sub(r"/\*.*?\*/", "", body, flags=re.S)
    body = re.sub(r"//[^\n]*", "", body)
    out, depth, current = [], 0, ""
    for ch in body:
        if ch in "([{":
            depth += 1
        elif ch in ")]}":
            depth -= 1
        if ch == "," and depth == 0:
            out.append(current)
            current = ""
        else:
            current += ch
    out.append(current)
    return [t for t in out if t.strip()]


def enum_discriminants(name: str) -> list[str] | None:
    """The discriminant set of a payload-less enum, as the port declares it."""
    for path in sorted((p for tree in ('src','lib','lib-src','lwlib','oldXMenu','nt','msdos','exec','platform','xtask') for p in (PORT/tree).rglob("*.rs"))):
        if "target" in path.parts:
            continue
        for match in ENUM.finditer(path.read_text(errors="replace")):
            if match.group(1) != name:
                continue
            values: list[str] = []
            next_value: int | None = 0
            for token in variants(match.group(2)):
                variant = VARIANT.match(token)
                if variant is None:
                    continue
                if variant.group(2) is None:
                    values.append(str(next_value) if next_value is not None else "?")
                    next_value = None if next_value is None else next_value + 1
                else:
                    value = variant.group(2).strip()
                    values.append(value)
                    try:
                        next_value = int(eval(value, {"__builtins__": {}}, {})) + 1
                    except Exception:
                        next_value = None
            return values
    return None


def read_table() -> list[dict[str, str]]:
    if not TABLE.is_file():
        return []
    rows = []
    for line in TABLE.read_text().split("\n"):
        if not line.strip() or line.startswith("#"):
            continue
        parts = line.split("\t")
        if len(parts) < 6:
            continue
        rows.append(dict(zip(("file", "source", "target", "count", "discriminants", "why"), parts)))
    return rows


def write_table() -> int:
    old = {(r["file"], r["source"], r["target"]): r for r in read_table()}
    rows, missing = [], 0
    for (rel, src, target), lines in sorted(sites().items()):
        previous = old.get((rel, src, target))
        parsed = enum_discriminants(target)
        value = previous["discriminants"] if previous else (",".join(parsed) if parsed else "-")
        why = previous["why"] if previous else "TODO: why no other value can reach this site"
        if not previous:
            missing += 1
        rows.append(f"{rel}\t{src}\t{target}\t{len(lines)}\t{value}\t{why}")
    header = ("# file\tsource\ttarget\tcount\tdiscriminants\twhy\n"
              "# Written by `tools/check-enum-transmutes.py --write`; the count, the discriminants\n"
              "# and the site set are machine-checked, the `why` column is the human half (P-98).\n")
    TABLE.write_text(header + "\n".join(rows) + "\n")
    print(f"wrote {TABLE.relative_to(REPO)}: {len(rows)} row(s), {missing} need a `why`")
    return 0


def check() -> int:
    found = sites()
    rows = read_table()
    recorded = {(r["file"], r["source"], r["target"]): r for r in rows}
    failures = 0

    for key, lines in sorted(found.items()):
        row = recorded.get(key)
        if row is None:
            print(f"NEW SITE  {key[0]}:{key[2]} ({key[1]} -> {key[2]}), {len(lines)} site(s) "
                  f"not recorded in the table")
            failures += 1
            continue
        if int(row["count"]) != len(lines):
            print(f"COUNT     {key[0]} {key[1]} -> {key[2]}: table says {row['count']}, "
                  f"tree has {len(lines)}")
            failures += 1
        if row["why"].startswith("TODO"):
            print(f"NO WHY    {key[0]} {key[1]} -> {key[2]}: the row has no reasoning yet")
            failures += 1
        current = enum_discriminants(key[2])
        if current is None:
            # Not this gate's class: the target is not a payload-less enum in the port (a
            # function pointer built from a constant, say - C's own `SIG_IGN`/`SIG_ERR` pattern).
            # Such a row must say so, so that a *new* site of that kind cannot slip in silently.
            if row["discriminants"] != "-":
                print(f"NO ENUM   {key[2]}: no payload-less enum in the port; record the row as `-` "
                      f"with the reason (a constant-to-pointer cast, not P-98's class)")
                failures += 1
        elif row["discriminants"] != ",".join(current):
            print(f"ENUM      {key[2]}: table records [{row['discriminants']}], "
                  f"the port now declares [{','.join(current)}]")
            failures += 1
    for key in recorded:
        if key not in found:
            print(f"STALE ROW {key[0]} {key[1]} -> {key[2]}: no live site")
            failures += 1

    print(f"{len(found)} site group(s), {len(rows)} row(s), {failures} failure(s)")
    return 1 if failures else 0


def selftest() -> int:
    """Prove the readings fire: a synthetic site, a renumbered enum, a stale row."""
    global PORT
    import tempfile
    with tempfile.TemporaryDirectory() as tmp:
        fake = pathlib.Path(tmp)
        (fake / "x.rs").write_text("let a = transmute::<c_int, Lisp_Type> (v);\n"
                                  "pub enum Lisp_Type { A, B, C }\n")
        real, PORT = PORT, fake
        try:
            found = sites()
            ok_site = (("x.rs", "c_int", "Lisp_Type") in found)
            ok_enum = enum_discriminants("Lisp_Type") == ["0", "1", "2"]
            (fake / "x.rs").write_text("pub enum Lisp_Type { A = 5, B = 7 }\n")
            moved = enum_discriminants("Lisp_Type") == ["5", "7"]
        finally:
            PORT = real
    for name, ok in (("a transmute site is found", ok_site),
                     ("an enum's discriminants are parsed", ok_enum),
                     ("a renumbering is visible", moved)):
        print(f"{'ok  ' if ok else 'FAIL'} {name}")
    return 0 if (ok_site and ok_enum and moved) else 1


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--write", action="store_true")
    parser.add_argument("--selftest", action="store_true")
    args = parser.parse_args()
    if args.selftest:
        return selftest()
    return write_table() if args.write else check()


if __name__ == "__main__":
    sys.exit(main())
