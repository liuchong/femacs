#!/usr/bin/env python3
"""Name a dump pair's byte differences: section, table, object, field offset.

`tools/check-verbatim.py`'s sibling for the *dump*: a dump pair differs by a few hundred
bytes per run and "302 bytes differ" is not a defect report until each byte has an owner.
Status 3.657 corrected 3.655 here: the *reference's* own dumps differ too, by the same
order (233/255/256/257 bytes over four pairs), at the same offsets and in the same shapes -
a per-run 32-bit-high value, a monotonically increasing counter, a rotated table of dump
offsets, and a cluster at the end of COLD - so "two dumps must compare equal" is not a
criterion, and this tool's job is to say *where* the two files differ, not that they must
not.  This reads the
dump's own header (`struct dump_header`, C 366-410: magic, fingerprint, three
`dump_relocs` locators, `object_starts`, `emacs_relocs`, `discardable_start`,
`cold_start`, `hash_list`), walks the `object_starts` table (each entry is a
`struct dump_reloc`: `raw_offset` in the low 27 bits, type in the high 5, so
`offset = (raw & 0x7ffffff) * DUMP_RELOCATION_ALIGNMENT` and the Lisp type is
`type - RELOC_DUMP_TO_DUMP_LV`), and reports, per differing byte run, which region it is
in, which recorded object contains it, and the aligned 8-byte words on both sides.

    python3 tools/decode-dump-diff.py A.pdmp B.pdmp [--max 40]

Readings are about the *files*: the tool never loads a dump.  A run inside a relocation
table is reported as such (those tables live in COLD too, right after the objects), so a
"difference at the end of the cold section" is not silently attributed to an object.
"""
import argparse
import struct
import sys
from pathlib import Path

MAGIC = b"DUMPEDGNUEMACS"
# `static const char dump_magic[16]` (C 116): the field is 16 bytes, the text only 14.
MAGIC_FIELD = 16
FINGERPRINT = 32
PHASES = 3
DUMP_RELOC_ALIGNMENT = 8
RELOC_DUMP_TO_DUMP_LV = 5
RELOC_DUMP_TO_EMACS_LV = 13
LISP_TYPE = {0: "Symbol", 1: "unused", 2: "Int0", 3: "Cons", 4: "String",
             5: "Vectorlike", 6: "Int1", 7: "Float"}


def parse_header(blob: bytes) -> dict:
    if blob[:len(MAGIC)] != MAGIC:
        sys.exit(f"not a dump: magic is {blob[:16]!r}")
    at = MAGIC_FIELD + FINGERPRINT
    out: dict = {"tables": {}}
    order = [f"dump_relocs[{i}]" for i in range(PHASES)] + ["object_starts", "emacs_relocs"]
    for name in order:
        offset, nr = struct.unpack_from("<II", blob, at)
        at += 8
        out["tables"][name] = (offset, nr)
    out["discardable_start"], out["cold_start"], out["hash_list"] = struct.unpack_from(
        "<III", blob, at)
    return out


# `struct dump_reloc` is a `dump_off` (4 bytes); `struct emacs_reloc` is 16, so the extent
# below is only used for the three dump tables and `object_starts` - `emacs_relocs` is
# reported by its locator alone.
ENTRY_BYTES = {"dump_relocs[0]": 4, "dump_relocs[1]": 4, "dump_relocs[2]": 4,
               "object_starts": 4, "emacs_relocs": 16}


def table_extent(name: str, offset: int, nr: int) -> tuple[int, int]:
    return offset, offset + ENTRY_BYTES[name] * nr


def object_starts(blob: bytes, table: tuple[int, int]) -> list[tuple[int, int]]:
    offset, nr = table
    out = []
    for i in range(nr):
        raw = struct.unpack_from("<I", blob, offset + 4 * i)[0]
        off = (raw & 0x7FFFFFF) * DUMP_RELOC_ALIGNMENT
        kind = raw >> 27
        out.append((off, kind - RELOC_DUMP_TO_DUMP_LV if kind >= RELOC_DUMP_TO_DUMP_LV
                    else -1))
    out.sort()
    return out


def region(offset: int, header: dict) -> str:
    for name, (off, nr) in header["tables"].items():
        lo, hi = table_extent(name, off, nr)
        if lo <= offset < hi:
            return f"table:{name}"
    if offset < header["discardable_start"]:
        return "HOT"
    if offset < header["cold_start"]:
        return "discardable"
    return "COLD"


def owner(starts: list[tuple[int, int]], offset: int) -> tuple[int, int]:
    lo, hi = 0, len(starts)
    while lo < hi:
        mid = (lo + hi) // 2
        if starts[mid][0] <= offset:
            lo = mid + 1
        else:
            hi = mid
    if lo == 0:
        return -1, -1
    return starts[lo - 1]


def runs(a: bytes, b: bytes) -> list[tuple[int, int]]:
    out = []
    start = None
    for i in range(min(len(a), len(b))):
        if a[i] != b[i]:
            if start is None:
                start = i
        elif start is not None:
            out.append((start, i - start))
            start = None
    if start is not None:
        out.append((start, len(a) - start))
    return out


def word(blob: bytes, offset: int) -> str:
    base = offset & ~7
    if base + 8 > len(blob):
        return "-"
    return f"0x{struct.unpack_from('<Q', blob, base)[0]:016x}"


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("a")
    ap.add_argument("b")
    ap.add_argument("--max", type=int, default=40, help="runs to print (default 40)")
    args = ap.parse_args()
    a = Path(args.a).read_bytes()
    b = Path(args.b).read_bytes()
    if len(a) != len(b):
        print(f"sizes differ: {len(a)} vs {len(b)}")
    ha, hb = parse_header(a), parse_header(b)
    if ha["tables"] != hb["tables"] or ha["cold_start"] != hb["cold_start"]:
        print("headers differ:")
        for k in ("discardable_start", "cold_start", "hash_list"):
            if ha[k] != hb[k]:
                print(f"  {k}: {ha[k]:#x} vs {hb[k]:#x}")
        for k in ha["tables"]:
            if ha["tables"][k] != hb["tables"][k]:
                print(f"  {k}: {ha['tables'][k]} vs {hb['tables'][k]}")
    starts = object_starts(a, ha["tables"]["object_starts"])
    print(f"header: {len(a)} byte(s), discardable_start={ha['discardable_start']:#x}, "
          f"cold_start={ha['cold_start']:#x}, hash_list={ha['hash_list']:#x}, "
          f"{len(starts)} object start(s)")
    rs = runs(a, b)
    total = sum(n for _, n in rs)
    print(f"{total} differing byte(s) in {len(rs)} run(s)")
    by_region: dict[str, int] = {}
    by_type: dict[str, int] = {}
    for off, n in rs:
        r = region(off, ha)
        by_region[r] = by_region.get(r, 0) + n
        s, t = owner(starts, off)
        by_type[f"{r}/{LISP_TYPE.get(t, t if t >= 0 else '?')}"] = \
            by_type.get(f"{r}/{LISP_TYPE.get(t, t if t >= 0 else '?')}", 0) + n
    print("by region: " + ", ".join(f"{k}={v}" for k, v in sorted(by_region.items())))
    print("by region/type: " + ", ".join(f"{k}={v}" for k, v in sorted(by_type.items())))
    print()
    for off, n in rs[:args.max]:
        r = region(off, ha)
        s, t = owner(starts, off)
        name = LISP_TYPE.get(t, t if t >= 0 else "?")
        where = (f"obj {s:#x} ({name}) +{off - s}" if s >= 0 else "no object")
        print(f"{off:#010x} len={n:<4} {r:<22} {where:<34} "
              f"{word(a, off)} -> {word(b, off)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
