# emacs/lib/mktime.c (582 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 11
- [x] my_tzset  (line 72, static)
- [x] shr  (line 142, static)
- [x] leapyear  (line 166, static)
- [x] isdst_differ  (line 192, static)
- [x] ydhms_diff  (line 208, static)
- [x] long_int_avg  (line 235, static)
- [x] tm_diff  (line 248, static)
- [x] convert_time  (line 261, static)
- [x] ranged_convert  (line 279, static)
- [x] __mktime_internal  (line 331)
- [x] __mktime64  (line 539)

## Macros defined — 6
- [x] EPOCH_YEAR  (line 160, body f9922496ec86)
- [x] LEAP_SECONDS_POSSIBLE  (line 41, body 356a192b7913)
- [x] NEED_MKTIME_WINDOWS  (line 59, body b6589fc6ab0d)
- [x] TM_YEAR_BASE  (line 161, body 259e58e18997)
- [x] __tz_convert  (line 269, body 3290cccd0132)
- [x] tzset  (line 105, body 4a0551a0882a)

## Types and their fields — 0

## File-scope objects — 0

# ----------------------------------------------------------------------
## Decisions

### Summary

`lib/mktime.rs` is the live text of `emacs/lib/mktime.c` for the frozen
configuration (`tools/check-cond-eval.py lib/mktime.c`: 16 conditions, 0 unknown,
0 disagreements).  The 11 functions and 6 macros all have a counterpart, and
`extract-abi.py --check` is OK.  The companion mirror is
`lib/include/mktime-internal.rs` (ledger `docs/ports/include-mktime-internal.md`).

### The two symbols this unit emits (measured, not guessed)

C 20-31's table says the `mktime` module's unit should define `rpl_mktime` and the
`mktime-internal` module's `mktime_internal`.  The frozen build confirms it:

    $ nm -g build/emacs-ref/lib/mktime.o
    T _mktime_internal
    T _rpl_mktime
    U _gmtime_r  U _localtime_r  U _tzset  U ___error

so `__mktime64` (renamed by the generated <time.h>'s `#define mktime rpl_mktime`;
REPLACE_MKTIME is 1) and `__mktime_internal` (renamed by mktime-internal.h) are
emitted through `#[unsafe(export_name = ...)]` while the Rust functions keep C's
identifiers (PORTING.md 10.13①, the mempcpy precedent).  `nm -g build/femacs/mktime.o`
prints the same two definitions and the same four undefined calls.

### Arms the frozen configuration deletes

| C lines | what | why dead |
|---|---|---|
| 33-35 | `#ifndef _LIBC`'s `<libc-config.h>` include | live, but has no Rust counterpart (the port's `config.rs`/`gnulib_config.rs` include chain is the same thing) |
| 74-99 | `my_tzset`'s `#if NEED_MKTIME_WINDOWS` body (`getenv ("TZ")` / `strchr (tz, '/')` / `_putenv`) | NEED_MKTIME_WINDOWS is 0 |
| 124-127 | the `#else` arm of the `long_int` typedef (`long long int`) | `INT_MAX <= LONG_MAX / 4 / 366 / 24 / 60 / 60` holds for a 64-bit `long` |
| 256-270 | the `#ifdef _LIBC` arm of `convert_time`/`__tz_convert` | `_LIBC` is undefined |
| 341-347 | the `#ifdef _LIBC` arm that skips the probe doubling | live as the `#ifndef` side |
| 544-550 | `__mktime64`'s `#else` arm (`mktime (tp)`) | `_LIBC`/NEED_MKTIME_WORKING decide the other arm |
| 557-581 | the `#if defined _LIBC && __TIMESIZE != 64` tail (the 32-bit `time_t` wrapper, `libc_hidden_def`, `weak_alias`, `libc_hidden_weak`) | `_LIBC` is undefined |

### Deviations

1. `struct tm tm;` (C 333), `struct tm oktm;` (C 294) and `struct tm otm;` (C 478) are
   declared without an initializer and then written field by field or by the callee;
   the port writes `std::mem::zeroed ()` first (PORTING.md 1.5, the canonicalize-lgpl
   precedent) and keeps C's assignments as separate statements.
2. `int negative_offset_guess;` (C 382) is set by `ckd_sub` before any read, so it is
   zero-initialized too (§1.5); that costs an "assigned value is never read" warning,
   which the rule accepts.
3. `ckd_add`/`ckd_sub` are the `overflowing_*` family (§2.6, P-15; the token-aliases
   families).  The calls whose result is *ignored* still store the wrapped value
   (C's `ckd_sub` always writes), and the ones used as a condition write the value and
   then test the overflow flag.  The port spells the three-argument `ckd_sub` as the
   associated-function form (`mktime_offset_t::overflowing_sub (0, off)`), which keeps
   C's arguments — including the `0` the family check compares.
4. `TYPE_SIGNED`/`TYPE_MINIMUM`/`TYPE_MAXIMUM` are unit-local `macro_rules!` over
   intprops.h's `_Generic` tables (every arm reduces to the type's own limit for the
   types this file names); `static_assert` is `static_assert!` (§2.5) and `__set_errno`
   is libc-config.h 46's macro (the canonicalize-lgpl precedent).
5. `goto offset_found` (C 429, C 492) is the labelled block `'offset_found` whose
   `break 'offset_found` is the same forward jump (§2.2, the getdelim/realpath_stk
   precedent); `while (true)` is kept as `while true` so the token survives.
6. C's `static` forward prototypes are dropped (§0.1-12), `static long_int const
   mktime_min/mktime_max` are Rust `static`s (the address is never taken), and the
   file-local `static mktime_offset_t localtime_offset;` (C 545) stays a *function-local*
   `static mut` (Rust allows items in a body), so its C scope is preserved.
7. The header's renames are split: the ones a call site spells (`__localtime64_r`,
   `__gmtime64_r`, `__libc_lock_lock`, `__libc_lock_unlock`, `__tzset_unlocked`,
   `__mktime_internal`) are `macro_rules!` in the mktime-internal mirror, and the ones
   that name a definition (`__mktime64`, `__timegm64`, `__time64_t`) are
   `excluded-macros.txt` rows because the port expresses them with `export_name` or a
   type alias.

### Notes recorded

* `docs/ports/macro-body-notes.tsv`: `__mktime_internal`'s `goto`, and the two
  `__libc_lock_lock`/`__libc_lock_unlock` rows (the `((void) 0)` body's one token has no
  Rust counterpart, and neither form evaluates the argument).
* `docs/ports/line-drift-tolerance.tsv`: `mktime 0.15` (C 582 vs the port's 513 — the
  deleted Windows and `_LIBC` arms are bigger than the port's own fixed costs).

### Verification

* `python3 tools/check-cond-eval.py lib/mktime.c` → `16 conditions, 0 unknown,
  0 disagreements [VERIFIED]`.
* `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target cargo run -q -p xtask -- build-tu lib/mktime.c`
  → `built lib/mktime.rs -> ../build/femacs/mktime.o`, 0 errors.
* `python3 tools/extract-abi.py --check emacs/lib/mktime.c lib/mktime.rs` → OK.
* Symbol-level check against the frozen reference object: `nm -g` on both objects
  (see above).  No differential *behaviour* probe was run (the round's workflow stops at
  `build-tu`/`--check`).
