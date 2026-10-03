# emacs/lib/time_r.c (49 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 3
- [x] copy_tm_result  (line 30, static)
- [x] gmtime_r  (line 40)
- [x] localtime_r  (line 46)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- The file has no preprocessing conditions at all: all three C functions are live text.
  `gmtime_r`/`localtime_r` are non-`static` definitions, so both export their C names
  (`#[unsafe(no_mangle)]`, P-52); `copy_tm_result` is `static` and must not.
- **Mirror edit**: this unit defines `gmtime_r`/`localtime_r`, which `<time.h>`'s mirror
  declares.  `femacs/lib/include/time.in.rs` now keeps its declarations in a uniquely-named
  `pub mod time_decls` + glob import (E0428; PORTING.md 0.1-10, the utimens.rs/stdio.rs
  shape).  `time_decls`, not `decls`, because two mirrors in one unit must not share the
  module name (docs/status.md 3.18).
- `if (! src) return 0;` is `if src.is_null () { return 0 as *mut tm; }` — the `0` is C's
  null-pointer constant for this type, written as the literal the checker compares.  The
  `restrict` qualifiers and `register` have no Rust spelling (PORTING.md 0.3).
- Line-count drift (C 49 vs the port's 58) is declared in docs/ports/line-drift-tolerance.tsv.
- Call-relationship differences (D7): none.
