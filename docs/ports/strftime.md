# emacs/lib/strftime.c (2446 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 7
- [x] memcpy_lowcase  (line 408, static)
- [x] memcpy_uppcase  (line 419, static)
- [x] iso_week_days  (line 870, static)
- [x] underlying_strftime  (line 944, static)
- [x] get_tm_zone  (line 1085, static)
- [x] my_strftime  (line 1137)
- [x] __strftime_internal  (line 1210, static)

## Macros defined — 51
- [x] ABALTMON_1  (line 142, body 97bf5d05b9d9)
- [x] CAL_ARGS  (line 191, body da39a3ee5e6b)
- [x] CHAR_T  (line 138, body 71fafc4e2fc1)
- [x] DO_MAYBE_SIGNED_NUMBER  (line 1478, body a1e78fee7f48)
- [x] DO_MULTIBYTE  (line 102, body ba1447bc6b31)
- [x] DO_NUMBER  (line 1466, body 6ac067adcf55)
- [x] DO_NUMBER_SPACEPAD  (line 1500, body e69fee274cfd)
- [x] DO_SIGNED_NUMBER  (line 1474, body 891a1f88bde1)
- [x] DO_TZ_OFFSET  (line 1491, body e74d1b0bab1a)
- [x] DO_YEARISH  (line 1476, body 964c96fa7814)
- [x] FAILURE  (line 229, body eb883130e83d)
- [x] FPRINTFTIME  (line 20, body b6589fc6ab0d)
- [x] HAVE_NATIVE_TIME_Z  (line 434, body f2337ca9aed4)
- [x] HAVE_ONLY_C_LOCALE  (line 50, body b6589fc6ab0d)
- [x] HELPER_LOCALE_ARG  (line 363, body da39a3ee5e6b)
- [x] ISDIGIT  (line 393, body fcfa890ac893)
- [x] ISO_WEEK1_WDAY  (line 866, body 1b6453892473)
- [x] ISO_WEEK_START_WDAY  (line 865, body 356a192b7913)
- [x] LOCALE_ARG  (line 359, body da39a3ee5e6b)
- [x] LOCALE_PARAM  (line 358, body da39a3ee5e6b)
- [x] L_  (line 140, body 5dfe08d5f0fd)
- [x] MEMCPY  (line 144, body c5d3fb3a32c0)
- [x] MULTIBYTE_IS_FORMAT_SAFE  (line 101, body 356a192b7913)
- [x] NLW  (line 141, body 50d6fddea6d8)
- [x] SHR  (line 159, body 8619c68aae39)
- [x] STREAM_OR_CHAR_T  (line 214, body 3bece736b3f1)
- [x] STRFTIME_ARG  (line 215, body 291885db8973)
- [x] STRLEN  (line 145, body 2d3571aeac9e)
- [x] TM_YEAR_BASE  (line 173, body 259e58e18997)
- [x] TOLOWER  (line 385, body 108ad3cf0f6e)
- [x] TOUPPER  (line 384, body f2d13032e3fc)
- [x] UCHAR_T  (line 139, body 3dc11840724e)
- [x] USE_C_LOCALE  (line 26, body b6589fc6ab0d)
- [x] YDAY_MINIMUM  (line 867, body 4ea3953420f9)
- [x] __isleap  (line 178, body 95de658df4f7)
- [x] add  (line 263, body afeb3b1275cb)
- [x] add1  (line 300, body 3773d130ee71)
- [x] advance  (line 260, body 09f8504f9344)
- [x] cpy  (line 307, body f8dc68d4e641)
- [x] extra_args  (line 911, body da39a3ee5e6b)
- [x] extra_args_spec  (line 912, body da39a3ee5e6b)
- [x] incr_overflow  (line 271, body 25edad550586)
- [x] memset_space  (line 253, body 8e1063c506af)
- [x] memset_zero  (line 254, body c6b1f937054c)
- [x] my_strftime  (line 908, body de093aaea82f)
- [x] nl_get_alt_digit  (line 909, body 9fd51e1dd6ef)
- [x] ns  (line 915, body b6589fc6ab0d)
- [x] tz  (line 914, body 356a192b7913)
- [x] width_add  (line 274, body f82539be670b)
- [x] width_add1  (line 304, body 1e67d593f538)
- [x] width_cpy  (line 326, body 64523a121b79)

## Types and their fields — 0

## File-scope objects — 0

# ----------------------------------------------------------------------
## Decisions

### Summary

`femacs/lib/strftime.rs` is the live text of `emacs/lib/strftime.c` for the frozen
configuration: `tools/check-cond-eval.py lib/strftime.c` reports 115 conditions, 0 unknown,
0 disagreements, so C's dead arms are not written.  The seven live functions, the 51 live
macros/constants and the two live types (`pad_style`, the byte-count aliases) all have a
counterpart, and `extract-abi.py --check` reports OK.

### Arms the frozen configuration deletes (each is a one-line note where C's `#if` stood)

| C lines | what | why dead |
|---|---|---|
| 30-33, 43, 59-60, 76-77, 79-82, 86, 105-106, 119-121, 128-136, 183-189, 196-203, 209-212, 225-226, 233-251, 258, 268, 302, 309-324, 337-340, 345-356, 360-363, 368-378 | the `_LIBC`, `FPRINTFTIME`, `COMPILE_WIDE`, `USE_IN_EXTENDED_LOCALE_MODEL`, `SUPPORT_NON_GREG_CALENDARS`, NetBSD/sun and Win32 arms | `_LIBC` undefined; `FPRINTFTIME` 0; `COMPILE_WIDE` undefined; `USE_C_LOCALE` 0; `HAVE_ONLY_C_LOCALE` 0; `SUPPORT_NON_GREG_CALENDARS_IN_STRFTIME` false |
| 438-455 | `c_locale` (the C-locale cache) | `USE_C_LOCALE && HAVE_STRFTIME_L` is 0 |
| 458-478 | `utc_timezone` | `HAVE_NATIVE_TIME_Z` is 0 |
| 483-829 | `should_remove_ampm` (347 lines of NetBSD/sun locale-name tables) | the arm's own condition |
| 833-856 | `tm_diff` | `!HAVE_STRUCT_TM_TM_GMTOFF` is false (the frozen `struct tm` has `tm_gmtoff`) |
| 1200-1284 | the `_NL_CURRENT`/`c_weekday_names` arms of the `a_wkday`-family macros | `_NL_CURRENT` undefined and both alternative arms false |
| in the switch | the `_NL_CURRENT`/`HAVE_ONLY_C_LOCALE` conversions | those cases are live as `goto underlying_strftime` |

P-51 text is optional for a file whose conditions are verified; the port keeps the *note*
because a reviewer still has to be able to see that C's arm was considered (the round-42
`gettimeofday.c` Windows-arm precedent is the opposite case: there the arm is compiled-in
text under a feature).

### `my_strftime`'s name and symbol (round 42; **superseded by "Round 52" below**)

Round 42 read this file as a standalone unit and recorded that `my_strftime` emits the
symbol `strftime`; round 52 replaced that with the parameterized body the shim needs.  The
two facts the port still rests on are kept here:

* The frozen strip of the *file* cannot be compiled as C at all: `#define tz 1` (C 914)
  rewrites `underlying_strftime`'s `timezone_t tz` parameter to `1`
  (`clang -fsyntax-only -DHAVE_CONFIG_H -I build/emacs-ref/lib … emacs/lib/strftime.c`
  reports ``expected parameter declarator``).  The arm that makes `tz`/`ns` parameters is
  `nstrftime.c`'s `#define my_strftime nstrftime`.
* The real Emacs build never links a `strftime.o`: `lib/Makefile` builds `nstrftime.o`, and
  `build/emacs-ref/lib` has no `strftime.o` (its `nstrftime.o` defines `_nstrftime` only).

### Structural deviations

1. `goto`/label → stage constants.  C's format dispatch is `switch`/`case` with `goto`s
   into a shared tail.  The port keeps the shared statements once and enters them at a
   `mod stage` constant named after C's label (`stage::do_number_body` for
   `do_number_body:`), then runs the `if stage == …` blocks in the order that lets each
   stage's block follow every block that can set it.  `stage::do_z_conversion` is
   therefore *before* `bad_format` (C's `default: goto bad_format` is a backward jump),
   and `stage::bad_percent` exists because C's `bad_percent:` is the one label that runs
   `--f` first.  Two labels of the inner colons switch (`tz_hh_mm`, `tz_hh_mm_ss`) are
   selected through C's `case 3:` test written as a local `zstage`; the ledger notes them.
2. C's `L_`/`L_C` split.  C has one `L_` macro for both string and character literals
   because `L_('x')` is an `int` and `L_("…")` a pointer.  Rust's `'x'` is a `char`/u8, so
   the port keeps `L_!` for the strings and adds `L_C!` for the character constants,
   whose value the port types as `c_char` (C's character constants promote to `int` in
   every comparison it makes, and the port writes that promotion out at each use).
3. `format_char`/`modifier` are `int` in C (`int modifier; int format_char;`), so the port
   declares them `c_int` and converts `*f` (a `char`) with `as c_int`, which is exactly
   C's sign-extending conversion.
4. `memcpy_lowcase`/`memcpy_uppcase`'s C forward declarations are dropped (PORTING.md
   0.1-12); the definitions stand where C defines them.  `while (len-- > 0)` is kept as a
   `loop` whose decrement precedes the test (the `len--` wraps; `len` is dead afterwards).
5. `countof` is the C23 system header's `#define countof _Countof`; the port uses the
   existing `countof!` from the `<stdcountof.h>` mirror.
6. `INT_STRLEN_BOUND (time_t)` has no mirror: the `buf` declaration carries the single
   instantiation, C's own arithmetic (intprops.h 97/108-114) on
   `TYPE_WIDTH (time_t) - signed` (the md5 `INT_BUFSIZE_BOUND` decision).
7. `TOUPPER`/`TOLOWER` are `toupper (Ch)`/`tolower (Ch)` here (C 384-385's `#else`), so the
   second argument C's call sites pass (`loc`) is unused; the port keeps the two-argument
   macro shape for the ledger.  `nl_get_alt_digit` (`_nl_get_alt_digit`) has no live user.
8. The unit carries the standard `#![allow (…, unsafe_op_in_unsafe_fn)]` crate header the
   other units use (a unit file is a crate root; mirrors may not have inner attributes).
9. Every C signed↔unsigned conversion that can wrap is written with `wrapping_*`
   (PORTING.md 1.1): `%G`'s `tm_year + (unsigned int) TM_YEAR_BASE + year_adjust`,
   `%j`'s `tm_yday + 1U`, `%m`'s `tm_mon + 1U`, `u_number_value`'s `-`, the `i += padding`
   of the `SPACE_PAD` arm (C's `ckd_add` stores the wrapped sum whatever the condition, see
   below), `width -= padding`, and `incr_overflow`'s `i += incr`.
10. C's `byte_count_t shortage = width - !!sign_char - number_digits;` converts a signed
    difference to the unsigned `byte_count_t`, so C's `shortage <= 0` is an *unsigned*
    comparison; the port writes it with `byte_count_t` for the same reason.

### Defects found while writing this port (each is fixed in the file)

* C's `bad_percent:` runs `--f` before the shared `bad_format` body; the port's first
  draft reused one stage, so `"%"`/`"%5"`/`"%-%"`/`"%::::z"` copied one byte too many (or
  the `default: goto bad_format` of the colons switch copied nothing at all).  Split into
  `stage::bad_percent` (which decrements) and `stage::bad_format`.
* `%P` must fall through into `%p` (`to_lowcase = true`, `format_char = 'p'`, then the
  `change_case` block); the first draft had a separate branch that skipped the fallthrough.
* `width_add1`'s `if (ckd_add (&i, i, padding) && FPRINTFTIME)` still stores into `i`; the
  first draft discarded the sum, which loses the padded width from the return value.
* `%G`'s and `%q`'s conversions were applied before C's `int` arithmetic instead of after
  it (C converts the whole `+ 1`/`+ TM_YEAR_BASE` expression to `unsigned int`).

### Verification

* `python3 tools/check-cond-eval.py lib/strftime.c` → `115 conditions, 0 unknown, 0 disagreements [VERIFIED]`.
* `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target cargo run -q -p xtask -- build-tu lib/strftime.c`
  → `built lib/strftime.rs -> ../build/femacs/strftime.o`, 0 errors.
* `python3 tools/extract-abi.py --check emacs/lib/strftime.c femacs/lib/strftime.rs` → OK.
* Line drift: C 2446 vs the port's ~1309 (the dead arms account for it); the row is in
  `docs/ports/line-drift-tolerance.tsv`, and the token exemptions are the ten
  `strftime.c` rows appended to `docs/ports/macro-body-notes.tsv`.
* No differential probe was run (the round's workflow stops at `build-tu`/`--check`).

### Round 52: the shared body is a macro, and the shim is its only instantiation

`lib/nstrftime.c` is an including shim (`#define my_strftime nstrftime` + `#include
"strftime.c"`), so PORTING.md 10.13② applies: the body is written once, as

    macro_rules! strftime_body { ($name:ident, $export:literal) => { … } }

and C's `#ifdef my_strftime` bindings become the macro's two arguments.  Decisions and
consequences:

* **This file does not instantiate the macro.**  The shim reaches it through
  `include!("strftime.rs")`, which splices this file's whole text into the shim's crate, so
  an instantiation here would define the body (and its symbol) a *second* time in that
  crate: E0428 for every function, and a `strftime` definition in an object the reference
  has only as `nstrftime`.  `build-tu lib/strftime.c` therefore compiles the macro and its
  includes and nothing else — which is what the frozen build compiles of this file
  (nothing: `lib/Makefile` builds `nstrftime.o` only, and C's standalone text does not even
  compile) — and 0 errors is still what the gate sees.
* The entry point lives inside the macro as `fn $name` with
  `#[unsafe(export_name = $export)]`: at the shim's instantiation the Rust identifier is
  C's `my_strftime` and the symbol is `nstrftime` (the mempcpy precedent, PORTING.md
  10.13①).  The token gate cannot see a `fn $name` as a definition, so the entry's body is
  not text-matched here; `my_strftime`'s counterpart is C 908-916's rename macro, which is
  still in the file.
* `tz`/`ns` are `__strftime_internal`'s **parameters** now, not the constants round 42
  wrote: the frozen build compiles the shim's arm, in which C 896-899's `extra_args_spec`
  makes them parameters of every function in the body and C 914-915's `#define`s belong to
  the *other* arm.  Those two `#define`s are recorded as `mod standalone`'s
  `pub const tz`/`pub const ns` — a module, because a Rust parameter cannot shadow a
  constant of the same name (E0530) while the body's parameters are named `tz`/`ns`.  The
  calls that used those constants (`underlying_strftime`, `get_tm_zone`, `mktime_z`, `%N`)
  now pass the parameters, which is C's own text in this arm.
* This file has **no crate-level `#![allow (…)]`**: an inner attribute inside an included
  file is E0753, so the shim carries the allow list for the unit that compiles the body
  (docs/ports/nstrftime.md).  This unit's own build instantiates nothing, so it emits the
  mirrors' warnings and the port's conventions' absence here is recorded rather than
  hidden.
* `docs/ports/nstrftime.md` carries the shim's ledger, the order note (C's two lines are
  swapped so the shim's rename shadows the body's) and the `nm` comparison of the two
  objects.
