# emacs/lib/timespec.h (102 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 4
- [x] make_timespec  (line 56)
- [x] timespec_cmp  (line 64)
- [x] timespec_sign  (line 72)
- [x] timespectod  (line 86)

## Macros defined — 2
- [x] TIMESPEC_H  (line 20, body da39a3ee5e6b)
- [x] _GL_TIMESPEC_INLINE  (line 32, body a960df3ab827)

## Types and their fields — 0

## File-scope objects — 0

# ----------------------------------------------------------------------
## Notes

### Deviations (recorded, not missing work)

- The include guard has no Rust counterpart (PORTING.md 0.1): `pub const TIMESPEC_H: bool = true`.
- C's `#if !_GL_CONFIG_H_INCLUDED / #error "Please include config.h first." / #endif` is mirrored as a
  `const _: () = assert! (_GL_CONFIG_H_INCLUDED, …)`.  `_GL_CONFIG_H_INCLUDED` is a property of the
  *generated* config header (src/config.h 38 of the frozen build), so it was added to
  `docs/baseline/config/darwin-arm64.json` and `xtask gen-config` writes it into `config.rs` — the
  port's config.rs is generated and must not be hand-edited.
- `#include "arg-nonnull.h"` is dropped: the only thing this header takes from it is `_GL_ARG_NONNULL`,
  an attribute.  `_GL_TIMESPEC_INLINE`, `_GL_INLINE_HEADER_BEGIN`/`END`, `_GL_INLINE`,
  `_GL_ATTRIBUTE_CONST` and `_GL_ARG_NONNULL` are recorded in `docs/ports/excluded-macros.txt`
  (kind=syntax): the port spells the four `_GL_TIMESPEC_INLINE` functions as
  `#[inline(always)] pub fn`, which is what those macros mean after preprocessing.
- `_GL_CMP` is a *value* macro generated into gnulib's config header (`m4/gnulib-common.m4`
  4746-4754, `(((n1) > (n2)) - ((n1) < (n2)))`); it is a macro in the port too, declared once in
  `lib/include/gnulib_config.rs` (PORTING.md 0.1-5, one home per macro) and included here,
  which is where C's expansion site is.
- `time_t` and `struct timespec` come from `<time.h>` in C, i.e. from `lib/include/time.in.rs`
  (the mirror of gnulib's `time.in.h`) in the port, and this mirror includes it itself — C's
  `#include <time.h>` is right there.  `buffer.rs` reaches the same mirror directly, so a unit
  including both would include it twice; `xtask`'s include-graph expansion de-duplicates that
  (PORTING.md 0.1-4/P-61), which is why the mirror keeps its own includes instead of asking every
  translation unit to list the closure.
- `settime` and `gettime_res` have no definition anywhere in the reference checkout (they are gnulib
  modules Emacs does not build), so their parameter names in the `extern` block are the port's; the
  five others come from the definitions in `lib/timespec-add.c`, `lib/timespec-sub.c`,
  `lib/dtotimespec.c` and `lib/gettime.c`.

### Call-relationship differences (D7)

- None.  Same four functions, same seven declarations, same order.
