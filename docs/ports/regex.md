# emacs/lib/regex.c (82 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 2
- [x] _GL_USE_STDLIB_ALLOC  (line 23, body 356a192b7913)
- [x] __STDC_WANT_IEC_60559_BFP_EXT__  (line 20, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/lib/regex.c` @ `fd7afced403`; port `lib/regex.rs`; the gates are
`build-tu lib/regex.c` -> `built`, `python3 tools/extract-abi.py --check emacs/lib/regex.c
lib/regex.rs` -> `OK`, and the symbol ratchet
(`tools/check-symbols.py --pending docs/baseline/symbol-pending.txt`, where this unit is listed).

- **What this file is.**  The aggregate translation unit: C 57-59 `#include` `regex_internal.c`,
  `regcomp.c` and `regexec.c`, which is why the reference has a single `regex.o` and no separate
  objects for the three.  The inventory of *this* file is two macros, because the extractor does
  not follow those includes (they are `.c` files, not headers).
- **Where the aggregate lives in the port.**  PORTING.md 0.1's "one `.c` = one `.rs`" rule
  applies to the three files C writes, so the port keeps `lib/regex_internal.rs`,
  `lib/regcomp.rs` and `lib/regexec.rs` as units (each with its own gate and
  ledger, docs/ports/regex_internal.md, /regcomp.md, /regexec.md), and their objects carry the
  13 `rpl_*` definitions the reference `regex.o` exports.  That is why `lib/regex` is listed in
  `docs/baseline/symbol-pending.txt`: this object's own table is empty by construction.  Closing
  it would mean teaching the build to link the three into one object, which the dump and the
  module ABI have not needed so far.
- **Live text.**  C 20's `#define __STDC_WANT_IEC_60559_BFP_EXT__` (empty body) and C 23's
  `# define _GL_USE_STDLIB_ALLOC 1` (the live arm: `_LIBC` is undefined) are the two macros the
  ledger lists; C 21-31's `#ifndef _LIBC` arm (the `libc-config.h` include and the three
  `#pragma GCC diagnostic` lines), C 34-37's `#if defined __cplusplus && defined _LIBC` error and
  C 39-55's `#ifdef _LIBC` rename block are dead, and C 60-75's `<limits.h>`/`<regex.h>`/
  `regex_internal.h` includes belong to the three sibling units.  C 75-82's `#if _LIBC`
  `re_max_failures` block is dead too.
- **Line-count basis.**  The `regex` row in `docs/ports/line-drift-tolerance.tsv` uses `total`
  with 0.50 (C 82 vs the port's 45): 45 of C's lines are its opening notice block and the dead/linked
  regions, and this file's inventory is complete at two macros.
