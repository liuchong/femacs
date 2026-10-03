# emacs/lib/ftoastr.c (152 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 2
- [x] ftoastr_snprintf  (line 89, static)
- [x] FTOASTR  (line 112)

## Macros defined — 10
- [x] FLOAT  (line 61, body 685e80366130)
- [x] FLOAT_DIG  (line 62, body b1aba7feca7a)
- [x] FLOAT_MIN  (line 63, body 9ac7510b92ab)
- [x] FLOAT_PREC_BOUND  (line 64, body a5820064c626)
- [x] FTOASTR  (line 65, body 85e190300f4b)
- [x] LENGTH  (line 60, body 356a192b7913)
- [x] PREFIX  (line 41, body 6ae999552a0d)
- [x] PROMOTED_FLOAT  (line 66, body bdb36bb22deb)
- [x] STRTOF  (line 76, body c152c9dc92dc)
- [x] snprintf  (line 87, body 035aad84cdf5)

## Types and their fields — 0

## File-scope objects — 0


# ----------------------------------------------------------------------
## Decisions

`emacs/lib/ftoastr.c` @ `fd7afced403`; port `femacs/lib/ftoastr.rs`; the gates are
`build-tu lib/ftoastr.c` -> `built` and `python3 tools/extract-abi.py --check
emacs/lib/ftoastr.c femacs/lib/ftoastr.rs` -> `OK`.  The file is a *template*: the frozen
build never compiles it as its own object (`build/emacs-ref/lib/` carries `dtoastr.o` and no
`ftoastr.o`), and `dtoastr.c` is the only shim this target compiles.

- **The 10.13② shape is a prelude `macro_rules!`.**  PORTING.md 10.13② asks the shared body to
  land first as a prelude `macro_rules!` (or a parameterised unit) with the instantiations
  after.  `femacs/lib/ftoastr.rs` is that prelude: its body is `macro_rules! ftoastr_body`,
  parameterised on C's `LENGTH` and the arm's bindings (`$float`/`$promoted`/`$dig`/`$min`/
  `$prec_bound`/`$strtof`) plus the entry's `$name`/`$export` - the `strftime.rs` shape, whose
  `nstrftime.c` shim is its only instantiation.  C's three arms (C 44-70) become the
  `$length == 1`/`$length == 2`/else selection.

- **The arm/parameter mapping.**  `femacs/lib/dtoastr.rs` (the `LENGTH == 2` shim C 52-58's
  arm selects, and the only instantiation) passes `$length` 2, `$name` `dtoastr`, `$export`
  `"dtoastr"`, `$float` `c_double`, `$promoted` `c_double`, `$dig` `DBL_DIG`, `$min` `DBL_MIN`,
  `$prec_bound` `_GL_DBL_PREC_BOUND`, `$strtof` `strtod`.  C's standalone reading (the one this
  file's own `--check` compares against) is the `LENGTH == 1` arm (C 59-70); `mod standalone`
  records that arm's names - `FLOAT` `float`, `FLOAT_DIG` `FLT_DIG`, `FLOAT_MIN` `FLT_MIN`,
  `FLOAT_PREC_BOUND` `_GL_FLT_PREC_BOUND`, `FTOASTR` `PREFIX (ftoastr)`, `PROMOTED_FLOAT`
  `double`, `STRTOF` `PREFIX (strtod)`, `snprintf` `ftoastr_snprintf`, `PREFIX` `name`,
  `LENGTH` 1 - as the `macro_rules!` items the macro check compares.  It sits in a module
  because its `LENGTH` would clash with the shim's own `LENGTH` in the one crate that includes
  this file (the `strftime.rs` standalone decision).

- **The `!GNULIB_SNPRINTF` fallback is the live definition.**  The frozen build resolves
  `! GNULIB_SNPRINTF` true (C 85-87), and `nm -g build/emacs-ref/lib/dtoastr.o` confirms it -
  the object imports `_sprintf` and not `_snprintf` - so `ftoastr_snprintf` (C 88-108) is the
  body's `snprintf` and the main function calls it.  The `#ifdef C_LOCALE` arm (C 36-42/82-84)
  is dead, so C 39-41 leave `PREFIX (name)` as `name`.  C 44-51's `LENGTH == 3` arm and C 67-69's
  `HAVE_STRTOF` arm are dead too (`HAVE_STRTOF` is undefined, so C 75-77 make `STRTOF` `strtod`).

- **The round-trip semantics preserved.**  The loop (C 144-151) keeps C's shape: `prec` starts
  at `abs_x < FLOAT_MIN ? 1 : FLOAT_DIG`, calls `snprintf`, and returns when `n < 0 ||
  FLOAT_PREC_BOUND <= prec || (n < bufsize && STRTOF (buf, NULL) == x)` - the `strtod`
  round-trip verification - incrementing `prec` otherwise.  `(n as usize) < bufsize` spells
  C's int-to-`size_t` conversion, and `$strtof (buf, core::ptr::null_mut ())` is C's
  `STRTOF (buf, NULL)`.  The `snprintf` fallback (C 88-108) keeps C's empty-first-`sprintf`
  shape: it sizes `width_0_buffer` by `LENGTH == 1 ? FLT_BUFSIZE_BOUND : LENGTH == 2 ?
  DBL_BUFSIZE_BOUND : LDBL_BUFSIZE_BOUND`, probes with width 0 when the caller's buffer is
  smaller, clamps the probe to `width`, and only writes the real buffer when it fits.

- **No `#![allow (…)]` here.**  An inner attribute inside an included file is E0753 (the
  `strftime.rs` decision), so `femacs/lib/dtoastr.rs` - the unit that compiles the body -
  carries the list; this file's own `build-tu` therefore compiles with warnings, not errors.

- **No `ftoastr.h` mirror yet; the header's constants are carried here.**  `ftoastr.h` is not on
  the port map (P-26), so the flag enum (`FTOASTR_*`), the `_GL_*_PREC_BOUND` bounds and the
  `*_BUFSIZE_BOUND` sizes are defined in this file with the values the frozen darwin-arm64
  compiler reports (`_GL_FLT/DBL/LDBL_PREC_BOUND` 9/17/17, `FLT/DBL/LDBL_BUFSIZE_BOUND`
  21/30/30), and `DBL_MIN` comes from the system `<float.h>` (which also has no mirror).
  `MB_LEN_MAX` is `include/limits.in.rs`'s.

- **Line drift** is inside the default 5% (C 152 vs Rust 147); no
  `docs/ports/line-drift-tolerance.tsv` row is needed.

- **Symbol evidence.**  There is no reference `ftoastr.o` to compare, so this unit is not in
  `tools/check-symbols.py`'s comparison; the shim `lib/dtoastr` is, and its object now exports
  `dtoastr` (see `docs/ports/dtoastr.md`).
