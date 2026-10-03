# emacs/src/atimer.h (84 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] EMACS_ATIMER_H  (line 20, body da39a3ee5e6b)

## Types and their fields — 1
- [x] atimer: enum atimer_type type, struct timespec expiration, struct timespec interval, atimer_callback fn, void* client_data, struct atimer* next

## File-scope objects — 0

## Decisions

- **`enum atimer_type` is the C type plus one constant per member** (PORTING.md 1.3's shape;
  `dispextern.rs`/`syntax.rs` write their enums the same way), so `ATIMER_ABSOLUTE` … are `0..2`
  and the field keeps the type.
- **The `atimer_callback` typedef is `Option<unsafe extern "C" fn (…)>`** (P-27): C's function
  pointers are nullable and the header's own users store null in them.
- **The member C spells `fn` becomes the raw identifier `r#fn`**, so the C name survives
  verbatim — the same device `r#type` uses in dispextern.rs/lisp.rs.
- **`#ifdef HAVE_TIMERFD`** (C 81-83) is dead on this target (docs/baseline/config's table
  records `HAVE_TIMERFD` as `undef`), so `timerfd_callback` keeps its text behind
  `#[cfg (feature = "timerfd")]` (P-51) and a build that enables it gets the declaration.
- **Compile evidence**: the mirror has no unit of its own yet (`atimer.c` is next), so it was
  compiled inside a two-file probe (`config.rs` + `lisp.rs` + this mirror) with
  `rustc --edition 2024 --crate-type lib --emit metadata` — RC=0.
