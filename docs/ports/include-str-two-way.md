# emacs/lib/str-two-way.h (455 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 3
- [x] critical_factorization  (line 108, static)
- [x] two_way_short_needle  (line 238, static)
- [x] two_way_long_needle  (line 331, static)

## Macros defined — 4
- [x] CANON_ELEMENT  (line 75, body 84a516841ba7)
- [x] CMP_FUNC  (line 78, body f7d1d081ce68)
- [x] LONG_NEEDLE_THRESHOLD  (line 67, body 256c9f9a067e)
- [x] MAX  (line 71, body eab8082c0680)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **`--check lib/str-two-way.h` is OK** (note printed by the gate: `LONG_NEEDLE_THRESHOLD` has
  two definitions, so only its presence is compared — the branch-macro rule, P-08/P-66).
- The three functions are `static` in C: each translation unit that includes the header gets
  its own copy and no exported symbol.  The port's `#[inline(always)] pub unsafe fn` in this
  mirror is that same shape (the timespec.rs/c-ctype.rs precedent for a header's definitions),
  which is why they are neither `extern "C"` nor `#[unsafe(no_mangle)]` (PORTING.md 0.1-2).
- `RETURN_TYPE` and `AVAILABLE` are *not* defined here: C's header requires the including unit
  to define them before C 29's `#include`, and the port keeps that contract (the unit's
  `macro_rules!` come first — PORTING.md 0.1-5/P-57).  `CANON_ELEMENT`/`CMP_FUNC`/`MAX` are the
  header's own `#ifndef` defaults and live here.
- C's `SIZE_MAX` wrap-around arithmetic (`max_suffix + k`, `--i` from 0, `i + 1` at SIZE_MAX) is
  written with `wrapping_add`/`wrapping_sub` where C relies on the wrap (PORTING.md 1.1/2.6);
  the `NULL` returns are `std::ptr::null_mut ()`.
- Two deviations are recorded: the mirror carries no `#![allow]` (an inner attribute is illegal
  in an `include!`d file, PORTING.md 0.1-9), and `period` is initialised to 0 because Rust
  requires the whole initialisation up front (PORTING.md 0.1-13) where C leaves it unset on the
  `needle_len < 3` path.  The two `#[inline(always)]`-annotated helpers are otherwise a
  statement-by-statement transcription.
