# emacs/lib/mktime-internal.h (83 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 11
- [x] __gmtime64_r  (line 50, body 2112e557abdb)
- [x] __gmtime_r  (line 69, body 9ee9db2bf979)
- [x] __libc_lock_lock  (line 22, body a29646b15239)
- [x] __libc_lock_unlock  (line 23, body a29646b15239)
- [x] __localtime64_r  (line 51, body af9bd6f1d877)
- [x] __localtime_r  (line 70, body 6e08437c5d11)
- [x] __mktime64  (line 52, body f09233e49161)
- [x] __mktime_internal  (line 72, body f1366fdc6788)
- [x] __time64_t  (line 49, body 325545ab4ce5)
- [x] __timegm64  (line 53, body 227ee5ccb914)
- [x] __tzset_unlocked  (line 24, body 333edb886263)

## Types and their fields — 0

## File-scope objects — 0

# ----------------------------------------------------------------------
## Decisions

`lib/include/mktime-internal.rs` mirrors `emacs/lib/mktime-internal.h`.  The
header is one of the four files `tools/check-cond-eval.py` cannot decide (its 5
conditions include the `<time.h>` template's `@…@` placeholders, so it is compared
blindly): the mirror is therefore written from the *frozen* configuration's text —
`_LIBC` undefined, `TIME_T_IS_SIGNED` 1, `__TIMESIZE` 64 — and every arm the checker
still demands is answered:

* `mktime_offset_t` and `__time64_t` are Rust type aliases (`time_t`), which is what the
  two live `typedef`/`#define` arms say.  `__time64_t` is additionally an
  `excluded-macros.txt` row (a type rename is not a Rust macro).
* `__libc_lock_lock`, `__libc_lock_unlock`, `__tzset_unlocked`, `__gmtime64_r`,
  `__localtime64_r`, `__gmtime_r`, `__localtime_r` and `__mktime_internal` are
  `macro_rules!` so a caller keeps C's spelling (`__tzset_unlocked` expands to
  `tzset! ()`, which is the *including* unit's macro — mktime.c's `#define tzset()
  my_tzset ()` — exactly as C's expansion works).
* `__mktime64` and `__timegm64` rename *definitions*; the port expresses each with
  `#[unsafe(export_name)]` on the definition (mktime.rs's `rpl_mktime`, and timegm.rs's
  `rpl_timegm` when that unit lands), so both are `excluded-macros.txt` rows.
* The `__mktime_internal` declaration is written under the name the rename produces
  (`mktime_internal`), which is the symbol the linker needs.  The unused-argument
  macro rows (`__libc_lock_*`) are in `macro-body-notes.tsv`.

Verification: `python3 tools/extract-abi.py --check emacs/lib/mktime-internal.h
lib/include/mktime-internal.rs` → OK; the mirror compiles as part of
`build-tu lib/mktime.c`.
