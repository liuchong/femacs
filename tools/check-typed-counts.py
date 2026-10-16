#!/usr/bin/env python3
"""The `memcpy`/`memset` families' third argument: bytes in C, *elements* in Rust.

C's `memcpy (dst, src, n_bytes)` and `memset (p, 0, n_bytes)` count **bytes**, while
`std::ptr::copy_nonoverlapping (src, dst, n)` and `std::ptr::write_bytes (p, val, n)` count
**`T`s** and multiply by `size_of::<T>()` themselves.  A ported call that keeps C's byte count
and hands it to a typed pointer therefore writes `size_of::<T>()` times too much.

Two rulings cover it: P-93 (the `memcpy` family, found three times - `maybe_resize_hash_table`,
`Fcopy_sequence`, and `copy_hash_table`'s four copies) and P-114 (the `memset` family, where the
dump died writing 1,572,864 bytes into a 6144-byte array).  The port's shape is C's own: cast
the pointer to `u8` and keep C's byte expression.

This gate looks for the shape that caused both: a count argument that mentions `size_of` or
`size_of_val` while the pointer argument is *not* byte-typed.  That is exactly "a byte count
given to a typed pointer", and it needs no type inference to see.

Usage
-----
    python3 tools/check-typed-counts.py            # every port source
    python3 tools/check-typed-counts.py --selftest
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent

# (call, index of the count argument, indexes of the pointer arguments)
FAMILIES = {
    "copy_nonoverlapping": (2, (0, 1)),
    "copy": (2, (0, 1)),
    "write_bytes": (2, (0,)),
    "read_volatile": None,
}

BYTE_TYPED = re.compile(r"\*\s*(?:mut|const)\s+(?:u8|c_char|i8|c_void)\b|\b(?:u8|c_char|c_void)\b")
# `c_void` is one byte in Rust (`size_of::<c_void> () == 1`), so C's byte count is the element
# count for it too - and the C sources reach for it exactly when the bytes are untyped.
SIZE_OF = re.compile(r"\bsize_of(?:_val)?\b")

# Roots that hold ported units (the mirrors included: a macro there expands into a unit).
ROOTS = ("femacs/src", "femacs/src/include", "femacs/lib", "femacs/lib/include",
         "femacs/lib-src", "femacs/lwlib", "femacs/oldXMenu", "femacs/exec", "femacs/nt",
         "femacs/msdos", "femacs/platform")


def split_args(text: str, start: int) -> tuple[list[str], int] | None:
    """Top-level argument texts of the call whose `(` is at `start`, and the index after `)`."""
    depth = 0
    i = start
    n = len(text)
    args: list[str] = []
    arg_start = start + 1
    while i < n:
        c = text[i]
        if c == '"':
            i += 1
            while i < n and text[i] != '"':
                i += 2 if text[i] == "\\" else 1
            i += 1
            continue
        if c in "([{":
            depth += 1
        elif c in ")]}":
            depth -= 1
            if depth == 0:
                args.append(text[arg_start:i])
                return args, i + 1
        elif c == "," and depth == 1:
            args.append(text[arg_start:i])
            arg_start = i + 1
        i += 1
    return None


def scan(path: Path) -> list[tuple[int, str]]:
    """(line, reason) for every site whose pointer/count pairing cannot be right."""
    text = path.read_text(encoding="utf-8", errors="replace")
    out: list[tuple[int, str]] = []
    for name in ("copy_nonoverlapping", "copy", "write_bytes"):
        for m in re.finditer(r"\bstd::ptr::" + name + r"\s*\(", text):
            got = split_args(text, text.index("(", m.start()))
            if got is None:
                continue
            args, _ = got
            if name == "copy":
                count_i, ptrs = 2, (0, 1)
            elif name == "copy_nonoverlapping":
                count_i, ptrs = 2, (0, 1)
            else:
                count_i, ptrs = 2, (0,)
            if count_i >= len(args):
                continue
            count = args[count_i]
            if not SIZE_OF.search(count):
                continue
            ptrs_text = " ".join(args[i] for i in ptrs if i < len(args))
            if BYTE_TYPED.search(ptrs_text):
                continue
            line = text.count("\n", 0, m.start()) + 1
            out.append((line, f"{name}: count {count.strip()[:60]!r} with pointer "
                              f"{ptrs_text.strip()[:60]!r}"))
    return out


def selftest() -> int:
    bad = [
        'std::ptr::write_bytes (rows, 0, n as usize * std::mem::size_of::<glyph_row> ());',
        'std::ptr::copy_nonoverlapping (src, dst, n * core::mem::size_of::<Lisp_Object> ());',
    ]
    good = [
        'std::ptr::write_bytes (rows as *mut u8, 0, n as usize * std::mem::size_of::<glyph_row> ());',
        'std::ptr::copy_nonoverlapping (src as *const u8, dst as *mut u8, n * std::mem::size_of::<T> ());',
        'std::ptr::write_bytes (row_table, 0, size as usize);',
    ]
    tmp = REPO / "build/load/typed-counts-selftest.rs"
    tmp.parent.mkdir(parents=True, exist_ok=True)
    failures = 0
    for i, case in enumerate(bad):
        tmp.write_text(case + "\n", encoding="utf-8")
        if not scan(tmp):
            print(f"  selftest: case {i} should fail but passed: {case}")
            failures += 1
    for i, case in enumerate(good):
        tmp.write_text(case + "\n", encoding="utf-8")
        if scan(tmp):
            print(f"  selftest: case {i} should pass but failed: {case}")
            failures += 1
    print(f"  selftest: {len(bad)} bad and {len(good)} good case(s), {failures} failure(s)")
    return failures


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--selftest", action="store_true")
    ap.add_argument("--limit", type=int, default=30)
    a = ap.parse_args()

    if a.selftest:
        return 1 if selftest() else 0

    files = 0
    sites = 0
    bad: list[tuple[str, int, str]] = []
    seen: set[Path] = set ()
    for root in ROOTS:
        base = REPO / root
        if not base.is_dir():
            continue
        for p in sorted(base.rglob("*.rs")):
            if p in seen:
                continue
            seen.add(p)
            files += 1
            found = scan(p)
            text = p.read_text(encoding="utf-8", errors="replace")
            sites += len(re.findall(r"\bstd::ptr::(?:copy_nonoverlapping|copy|write_bytes)\s*\(", text))
            for line, why in found:
                bad.append((str(p.relative_to(REPO)), line, why))
    print(f"{sites} site(s) in {files} file(s); {len(bad)} with a byte count on a typed pointer")
    for path, line, why in bad[:a.limit]:
        print(f"  {path}:{line}  {why}")
    if len(bad) > a.limit:
        print(f"  … {len(bad) - a.limit} more")
    if not bad:
        print("typed counts: every byte count goes through a byte pointer (P-93/P-114)")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
