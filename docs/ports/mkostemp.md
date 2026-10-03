# emacs/lib/mkostemp.c (46 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] mkostemp  (line 43)

## Macros defined — 2
- [x] __GT_FILE  (line 28, body 05e9fe679987)
- [x] __gen_tempname  (line 26, body 131bc2b1ae9e)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- `_LIBC` is undefined, so C 18-20's `<config.h>` include is live and C 24-30's `#if !_LIBC`
  arm is too: it defines `__gen_tempname` as `gen_tempname` (the port's nullary
  `macro_rules!`, called as `__gen_tempname! () (...)`, the strtoimax.rs `Strtoimax` shape)
  and `__GT_FILE` as `GT_FILE`.
- `__GT_FILE` has **two** definitions (C 28's `GT_FILE` and C 35's `0`, the latter inside the
  dead `#ifndef __GT_FILE`); the extractor keeps the last one and the branch-macro rule
  compares presence, not the body (PORTING.md P-08).  Both spell 0 for this target, and the
  port's constant is the mirror's `GT_FILE` (tempname.h 30-32), which is the live arm's value.
- `mkostemp` is a non-`static` definition → `#[unsafe(no_mangle)]` (P-52).  The reference's
  `<stdlib.h>`/`<stdio.h>`/`"tempname.h"` includes are the stdlib/stdio/tempname mirrors
  (`gen_tempname` is declared there and defined in lib/tempname.c, PORTING.md 0.1-3).
- Line-count drift (C 46 vs the port's 51) is declared in docs/ports/line-drift-tolerance.tsv.
- Call-relationship differences (D7): none.
