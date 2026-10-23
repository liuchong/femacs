# emacs/lib/strtol.c (410 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 17
- [x] GROUP_PARAM_PROTO  (line 141, body da39a3ee5e6b)
- [x] INT  (line 47, body 641941a0007a)
- [x] INTERNAL  (line 194, body c032adc1ff62)
- [x] ISALPHA  (line 184, body 4c9d42282664)
- [x] ISSPACE  (line 183, body 243d553c4c30)
- [x] LOCALE_PARAM  (line 155, body da39a3ee5e6b)
- [x] LOCALE_PARAM_PROTO  (line 156, body da39a3ee5e6b)
- [x] LONG  (line 131, body bd3027fa569e)
- [x] L_  (line 175, body f71fb785330f)
- [x] STRING_TYPE  (line 177, body 71fafc4e2fc1)
- [x] STRTOL_LONG_MAX  (line 133, body f5c01bcbc752)
- [x] STRTOL_LONG_MIN  (line 132, body afdafb6274ed)
- [x] STRTOL_ULONG_MAX  (line 134, body 7531eda0d2d7)
- [x] TOUPPER  (line 185, body 4e30308d0b80)
- [x] UCHAR_TYPE  (line 176, body 3dc11840724e)
- [x] UNSIGNED  (line 46, body b6589fc6ab0d)
- [x] __set_errno  (line 31, body 3db31e808293)

## Types and their fields — 0

## File-scope objects — 0

# ----------------------------------------------------------------------
## Decisions

`emacs/lib/strtol.c` @ `fd7afced403`; port `lib/strtol.rs` (255 lines); the gates are
`build-tu lib/strtol.c` -> `built` (`build/femacs/strtol.o`, no C-visible exports - the
template defines no function of its own, matching the ledger's 0 functions) and
`python3 tools/extract-abi.py --check emacs/lib/strtol.c lib/strtol.rs` -> `OK`.  The
frozen build compiles neither `strtol.o` nor `strtoll.o` (`build/emacs-ref/lib` carries
neither), so `tools/check-symbols.py` does not compare this unit; its instantiation is
`lib/strtoll.rs`.

- **The 10.13② shape is a prelude `macro_rules!`.**  PORTING.md 10.13② names `lib/strtoll.c`
  as an inclusion shim of this source, so the shared body lands as
  `macro_rules! strtol_body` and each shim instantiates it once; `lib/strtoll.rs` is
  the only instantiation (the `ftoastr.c`/`strftime.c` shape).  The frozen configuration never
  compiles this file on its own, so this file's own `--check` compares C's *standalone*
  reading - the one C's text has with `QUAD`/`UNSIGNED`/`USE_WIDE_CHAR`/
  `USE_IN_EXTENDED_LOCALE_MODEL`/`USE_NUMBER_GROUPING` all undefined.

- **What is parameterised.**  Only `QUAD` is controlled by the includer (C 19-21:
  `#define QUAD 1` + `#include <strtol.c>`), and it selects just the entry's name (C 108-112
  makes it `strtoll`) and C 130-135's `LONG`/`STRTOL_LONG_MIN`/`STRTOL_LONG_MAX`/
  `STRTOL_ULONG_MAX` bindings.  The body therefore takes `$name`/`$export` plus
  `$long` (C's `INT`/`LONG`, `long long` under `QUAD`), `$ulong` (C's `unsigned LONG int`) and
  the three limits as parameters; `lib/strtoll.rs` passes the `QUAD` arm as
  `c_longlong`/`c_ulonglong`/`c_longlong::MIN`/`c_longlong::MAX`/`c_ulonglong::MAX`.

- **The standalone reading and where its names live.**  `mod standalone` records the five
  names C 47/131-135's `#else` arm leaves (`INT`, `LONG`, `STRTOL_LONG_MIN`,
  `STRTOL_LONG_MAX`, `STRTOL_ULONG_MAX`), because the body takes them as parameters and they
  have no other Rust item.  The other twelve ledger names are C's *own* `#define`s (C 30-32,
  43-47, 136-157, 159-195), which in C are shared by every includer, so the port keeps them as
  top-level `macro_rules!`/aliases that `strtol_body` and any includer use: `__set_errno`
  (C 31), `UNSIGNED` (C 46, the ledger's constant), `GROUP_PARAM_PROTO` (C 141),
  `LOCALE_PARAM`/`LOCALE_PARAM_PROTO` (C 155-156), `L_` (C 175), `UCHAR_TYPE`/`STRING_TYPE`
  (C 176-177), `ISSPACE`/`ISALPHA`/`TOUPPER` (C 183-185) and `INTERNAL` (C 194).

- **The type macros are two items each.**  C 176-177's `#define UCHAR_TYPE unsigned char` /
  `STRING_TYPE char` are `pub type` aliases (so the signatures read like C's) plus
  `macro_rules!` records (so the ledger's macro check sees the names) - the `strftime.rs`
  `CHAR_T`/`UCHAR_T` shape.

- **The dead arms are recorded, not carried.**  `_LIBC` is undefined, so C 22-26 take the
  `# include <config.h>` arm; `USE_WIDE_CHAR`, `USE_IN_EXTENDED_LOCALE_MODEL` and
  `USE_NUMBER_GROUPING` are undefined, so the `wchar.h`/`wctype.h` body, the `../locale/
  localeinfo.h` `_NL_CURRENT` model, C 138-142's `grouping.h` include, C 215-244/C 299-315's
  number-grouping blocks, C 367-368's `#if UNSIGNED` return and C 398-410's `#ifdef
  USE_NUMBER_GROUPING` external entry point are dead text.  Each is a one-line note at its
  site here and is not transcribed (the `strftime.c` treatment); the port carries C's live
  text only, and the line-count row is against that live text.

- **Deviations worth a note.**  `register` is a C storage hint with no Rust counterpart
  (dropped); `int negative;` is zero-initialized (PORTING.md 1.5); the two `goto noconv`
  jumps are the labelled block `'noconv` (PORTING.md 2.2, C's `noconv:` label follows the
  block); `ISSPACE`/`ISALPHA`/`TOUPPER` keep C's `(unsigned char)` cast and return the libc
  `int`; C 175's identity `L_(Ch)` is recorded as `$ch` and the body's `L_! (b'-')` spells
  Rust's `u8` byte literal cast to the `char` C's `'x'` promotes to; `STRTOL_ULONG_MAX %
  (unsigned LONG int) base` is C's `unsigned int` `cutlim` after the explicit cast; `i *=
  base; i += c;` are `wrapping_mul`/`wrapping_add` (PORTING.md 1.1's modular arithmetic), and
  `-i` is `i.wrapping_neg ()` before C's conversion to the signed return type.

- **No `#![allow (…)]` here.**  An inner attribute inside an included file is E0753 (the
  `strftime.rs` decision), so `lib/strtoll.rs` - the unit that compiles the body -
  carries the list; this file's own `build-tu` compiles with warnings, not errors.

- **Line drift** is against C's live text: C 410 with about 165 dead lines, 245 live, vs the
  port's 255 (4.1%); the `strtol	0.08	…	live` row in `docs/ports/line-drift-tolerance.tsv`
  records it.  The 17-macro inventory is the real guard.
