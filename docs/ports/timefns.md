# emacs/src/timefns.c (2144 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 14
- [x] time-add  (Ftime_add, 2/2, doc 1aa3818e65c5, line 1220)
- [x] time-subtract  (Ftime_subtract, 2/2, doc 98761f725387, line 1229)
- [x] time-less-p  (Ftime_less_p, 2/2, doc 1a4522e58f11, line 1287)
- [x] time-equal-p  (Ftime_equal_p, 2/2, doc e40e1ce26f0b, line 1296)
- [x] float-time  (Ffloat_time, 0/1, doc 1451ee237e2e, line 1307)
- [x] format-time-string  (Fformat_time_string, 1/3, doc e53a62da2dda, line 1415)
- [x] decode-time  (Fdecode_time, 0/3, doc 30c8c20fa2dd, line 1503)
- [x] encode-time  (Fencode_time, 1/MANY, doc 1b55496165de, line 1657)
- [x] time-convert  (Ftime_convert, 1/2, doc 63e5be4eff28, line 1778)
- [x] current-time  (Fcurrent_time, 0/0, doc d6c8d79bfa24, line 1816)
- [x] current-cpu-time  (Fcurrent_cpu_time, 0/0, doc 1cd53a5673d9, line 1833)
- [x] current-time-string  (Fcurrent_time_string, 0/2, doc 9436ca126ae4, line 1844)
- [x] current-time-zone  (Fcurrent_time_zone, 0/2, doc baad03fb9b60, line 1893)
- [x] set-time-zone-rule  (Fset_time_zone_rule, 1/1, doc 528b07fce252, line 1962)

## Functions defined — 48
- [x] trillion_factor  (line 97, static)
- [x] make_timeval  (line 115)
- [x] monotonic_coarse_timespec  (line 145)
- [x] tm_gmtoff  (line 161, static)
- [x] tm_diff  (line 173, static)
- [x] emacs_localtime_rz  (line 200, static)
- [x] invalid_time_zone_specification  (line 219, static)
- [x] xtzfree  (line 227, static)
- [x] tzlookup  (line 238, static)
- [x] init_timefns  (line 341)
- [x] time_overflow  (line 352, static)
- [x] time_spec_invalid  (line 358, static)
- [x] time_error  (line 364, static)
- [x] invalid_hz  (line 375, static)
- [x] hi_time  (line 382, static)
- [x] lo_time  (line 389, static)
- [x] frac_to_double  (line 406, static)
- [x] mpz_time  (line 490, static)
- [x] s_ns_to_timespec  (line 507, static)
- [x] ticks_hz_to_timespec  (line 528, static)
- [x] decode_ticks_hz  (line 592, static)
- [x] decode_float_time  (line 610, static)
- [x] ticks_hz_list4  (line 663, static)
- [x] mpz_set_time  (line 699, static)
- [x] timespec_mpz  (line 710, static)
- [x] timespec_ticks  (line 720, static)
- [x] emacs_gcd  (line 736, static)
- [x] ticks_hz_hz_ticks  (line 746, static)
- [x] ticks_hz_seconds  (line 799, static)
- [x] make_lisp_time  (line 820)
- [x] timespec_to_lisp  (line 835)
- [x] decode_time_components  (line 853, static)
- [x] current_time_in_cform  (line 939, static)
- [x] decode_lisp_time  (line 957, static)
- [x] float_time  (line 1046)
- [x] list4_to_timespec  (line 1054)
- [x] lisp_time_argument  (line 1067)
- [x] lisp_seconds_argument  (line 1078, static)
- [x] lispint_arith  (line 1090, static)
- [x] time_arith  (line 1125, static)
- [x] time_cmp  (line 1248, static)
- [x] emacs_nmemftime  (line 1337, static)
- [x] format_time_string  (line 1371, static)
- [x] check_tm_member  (line 1636, static)
- [x] emacs_getenv_TZ  (line 1995)
- [x] emacs_setenv_TZ  (line 2013)
- [x] syms_of_timefns_for_pdumper  (line 2089, static)
- [x] syms_of_timefns  (line 2097)

## Macros defined — 8
- [x] FASTER_TIMEFNS  (line 66, body 356a192b7913)
- [x] HAVE_TM_GMTOFF  (line 60, body 7cb6efb98ba5)
- [x] HAVE_TZALLOC_BUG  (line 54, body 7cb6efb98ba5)
- [x] NEED_ZTRILLION_INIT  (line 91, body da39a3ee5e6b)
- [x] TRILLION  (line 80, body 05355f30cfce)
- [x] timespec_hz  (line 75, body f7136b95cc56)
- [x] trillion  (line 82, body 63e8c3fe45bf)
- [x] ztrillion  (line 88, body e58b0540439d)

## Types and their fields — 5
- [x] c_time: struct ticks_hz th, struct timespec ts, double d
- [x] err_time: int err, union c_time time
- [x] ticks_hz: Lisp_Object ticks, Lisp_Object hz
- [x] timespec: struct timespec ts
- [x] timeval: struct timeval tv

## File-scope objects — 5
- [x] static Lisp_Object timespec_hz  (line 77, static)
- [x] static Lisp_Object trillion  (line 84, static)
- [x] static mpz_t ztrillion  (line 90, static)
- [x] static timezone_t local_tz  (line 196, static)
- [x] static Lisp_Object flt_radix_power  (line 401, static)

# ----------------------------------------------------------------------
## Decisions

`emacs/src/timefns.c` @ `fd7afced403`; port `femacs/src/timefns.rs` (2,172 lines vs C's
2,144, +1.3%, inside the default 5% - the row in `docs/ports/line-drift-tolerance.tsv`
spells out why the **`total`** basis is the right one here).  `src/timefns.c` is in
`docs/baseline/frozen-build-sources.txt`, so the frozen configuration compiles it and the
full comparison applies (no 3.31 exemption).  The gates:

    build-tu src/timefns.c      -> built src/timefns.rs -> ../build/femacs/timefns.o
    extract-abi.py --check      -> OK emacs/src/timefns.c -> femacs/src/timefns.rs
    check-symbols.py --pending  -> check-symbols: 78 units compared, 0 mismatched, 1 still pending
    check-globals.py            -> check-globals: 240 units, 0 DEFVAR_* variables without a definition in their own unit
    ledger                      -> 80 rows, 0 unchecked
    (extra) rustc --cfg enable_checking on the materialized unit -> exit 0, no error

### Carried arms (the integrator's 2026-10-03 decision)

`src/timefns.c` is not in `docs/baseline/config/cond-eval-verified.txt`; the evaluator's
table cannot decide `FIXNUM_OVERFLOW_P (…)` (a function-like macro that is not in the
frozen `-dM` dump at all) or `TRILLION <= ULONG_MAX`, so `--check` reports
`30 conditions, 7 unknown, 4 disagreements [NOT VERIFIED]` and, through
`tools/extract-abi.py`'s `blind_conditions`, keeps *both* arms of every `#if` live.  The
port therefore **carries** the arms the frozen build does not compile, each under a
`#[cfg (feature = "…")]` naming the C condition's non-frozen side (PORTING.md P-51, the
`femacs/lib/boot-time.rs` precedent).  None of the features is set by any build tool
(`xtask dead-arms` can only flip the keys `config.rs` carries - status 3.9), so the
compiled build is exactly what clang compiles.  The arms, by C line:

- C 74-78 / 81-85: the `#else` objects `static Lisp_Object timespec_hz` / `trillion`
  (`no_fixnum_timespec_hz` / `no_fixnum_trillion`); the live arm is C 75/82's object-like
  macro, which the port writes `macro_rules!` + `timespec_hz! ()` / `trillion! ()`.
- C 86-93: `! (FASTER_TIMEFNS && TRILLION <= ULONG_MAX)` is false, so clang compiles
  nothing there; both inner arms are carried (`ztrillion_bignum_val` for C 87-88,
  `ztrillion_mpz` for C 89-91 - the latter also carrying `NEED_ZTRILLION_INIT`,
  `syms_of_timefns_for_pdumper` and the `pdumper_do_now_and_after_load` call,
  C 2087-2094 / 2141-2143).
- C 667-671 / 676-684 / 915-923: the `ztrillion` halves of the `TRILLION <= ULONG_MAX`
  shortcut (`no_trillion_shortcut`).
- C 148-154: the `CLOCK_MONOTONIC_COARSE` arm (`clock_monotonic_coarse`; the frozen target
  has no such clock).
- C 47-52 (`have_timezone_t`), C 43-45 / 202-205 / 328-330 (`windowsnt`),
  C 1997-2001 / 2015-2084 (`msdos`).
- C 59-61, 2099-2102, 2103-2106: the three `#ifndef` blocks that are dead on the frozen
  target (`no_have_tm_gmtoff`, `no_fixnum_timespec_hz`, `no_fixnum_trillion`).
- C 23-25's `#pragma GCC diagnostic` (clang's `__GNUC__` is 4, so it is dead) has no Rust
  counterpart and is recorded here instead (the `__FINITE_MATH_ONLY__` precedent).

### Residue (the integrator-authorized ledger entries)

1. **`timespec_hz` / `trillion` / `ztrillion` are one name with two spellings.**  The
   ledger lists each as *both* an object macro (C 75/82/88) and a `static` object
   (C 77/84/90) - that is the `#if`/`#else` pair of one object, and the port keeps both
   spellings, one live and one inside the feature block (the token check is the real
   guard).  `ztrillion`'s macro body is `xbignum_val (trillion! ())` (C's array decays to
   the pointer the `mpz_mul` calls take) and the `static` is `ztrillion`'s `mpz_t`.
2. **`HAVE_TM_GMTOFF`.**  `<config.h>` defines it as 1, but the blind evaluator records
   C 60's `#define HAVE_TM_GMTOFF false` and demands that spelling.  The port carries the
   dead `false` const **first** (the extractor's `constants_of` reads the first occurrence)
   and the frozen `true` const second, so `--check` sees `false` and the compiled build
   uses 1 - exactly the "one live, one in the feature block" shape the integrator asked
   for.
3. `HAVE_TZALLOC_BUG` is the only C macro with several definitions (`branch_macros`), so
   its body is not compared; the port defines the frozen `false`.

### Deviations recorded

1. **Object-like macros whose body is a call are `macro_rules!` with no parameters**
   (`timespec_hz`, `trillion`, `ztrillion`, `NEED_ZTRILLION_INIT`), invoked as `x! ()`
   (the `doc.rs`/`euidaccess.rs` precedent); C's `x` is the same text without the bang.
2. **`switch` becomes `match`** in `time_error`, `decode_ticks_hz` (PORTING.md 2.2); both
   are recorded in `macro-body-notes.tsv` because `switch`/`case`/`default` have no Rust
   spelling.  `time_error`'s fall-through arms are `noreturn`, so the `match`'s `_` arm is
   the port's `default`.
3. **C's `UNINIT` is the `= {0,}` marker** of the frozen `conf_post.h`, so `Lisp_Object
   abbr UNINIT` (C 277) and `struct ticks_hz th UNINIT` (C 1543) become zero-initialised
   locals and the macro name is exempted in `macro-body-notes.tsv` (the data.c
   `Fmake_variable_buffer_local` row).
4. **`ckd_add`/`ckd_mul`/`ckd_sub` → `overflowing_*`** with C's store-then-test shape
   (P-15): `make_timeval`, `mpz_time`, `s_ns_to_timespec`, `timespec_ticks`,
   `ticks_hz_hz_ticks`, `decode_time_components`, `check_tm_member`, `emacs_nmemftime`.
   `check_tm_member`'s `ckd_sub (&i, n, offset)` computes in `EMACS_INT` and stores when
   the difference fits in `int`, which is gnulib's rule.
5. **C's comma-expression statement** (`s--, ns += TIMESPEC_HZ;` in
   `ticks_hz_to_timespec`) is two statements; `emacs_gcd`'s empty `for` body keeps C's
   `continue;`, and `for (EMACS_INT r; (r = a % b) != 0; a = b, b = r)` becomes a `loop`
   whose condition is the port's explicit test.
6. **C's `?:` chains become `if`/`else if`/`else`** (`ticks_hz_list4`'s round-to-even,
   `time_arith`'s return, `decode_lisp_time`'s `(TICKS . HZ)` test, `Fcurrent_time_zone`,
   `Fcurrent_time_string`'s sign byte, `Fencode_time`'s two returns, `Ftime_convert`).
7. **`&"-"[XFIXNUM (zone) < 0]`** (C 296/308) is the same index into a two-byte
   `b"-\0"`, the address of the indexed byte being the `char *` `sprintf` takes.
8. **String-literal concatenation.**  `pI` is `"l"` in `lisp.rs`, so
   `tzbuf_format` / `trailing_tzbuf_format` are `b"<%+.*ld>%s%ld:%02d:%02d\0"` and the
   offset is `size_of_val (b"<%+.*ld") - 1`; `PRIdMAX` is `"jd"`, so
   `Fcurrent_time_string`'s format is `"%s %s%3d %02d:%02d:%02d %jd"` (both recorded in
   `macro-body-notes.tsv`, the `lock_file_1` precedent).
9. **`TYPE_SIGNED`/`EXPR_SIGNED`** (gnulib intprops) are local `macro_rules!` (the
   `regcomp.rs`/`dired.rs` shape); `time_t` is signed here, so both `mpz_time` arms and
   both `mpz_set_time` arms compile with the signed one taken.
10. **C's uninitialised locals are zero-initialised** (PORTING.md 1.5): `struct tm tm`,
    `struct ticks_hz th`, `struct timespec ts`, `time_t sec`, `int ns`, `intmax_t n`,
    `int incr`, each named in the port's comment where C just declares it.
11. **`emacs_setenv_TZ`'s `static ptrdiff_t tzvalbufsize`** is a function-local
    `static mut`; `tzvalbuf` is a file-scope `static mut` (C 1989).  The `#ifdef MSDOS`
    arm's `SAFE_ALLOCA` block is carried, so the unit needs `USE_SAFE_ALLOCA!`'s two
    locals only in that arm.
12. **`Fformat_time_string`'s parameter reassignment** (C 1500) is a shadowing `let`,
    because the `DEFUN!` macro's parameters are immutable.
13. **`utc_tz`** (C 197) is C's `static timezone_t const utc_tz = 0;`; raw pointers are
    not `Sync`, so the port uses `static mut` for a cell it never writes.

### Names written at their use site

The mirrors this unit includes do not carry these, so each is declared where it is called
(the terminal.rs/filelock.rs shape), with C's prototype in a comment: `tzalloc`, `tzfree`
(time.in.h 512/520), `clock_gettime`, `clock` (+ the `clock_t`/`CLOCKS_PER_SEC` pair),
`synchronize_system_time_locale` (lisp.h 5143), `xputenv` (lisp.h 5579), and the
`PRIdMAX`/`EDOM` constants.  `localtime_rz`/`mktime_z`/`tzset`/`gmtime_r` come from the
`time.in.rs` mirror through `systime.rs`.

### Macro-body notes

`docs/ports/macro-body-notes.tsv` gains eleven rows for this unit: `tzlookup` and
`Fdecode_time` (`uninit`), `time_error` and `decode_ticks_hz`
(`switch,case,default`), `ticks_hz_list4`/`ticks_hz_hz_ticks`/`time_cmp` (`mpz_t`, the
P-49 decay that the existing `rescale_for_division`/`bignum_integer` rows record),
`emacs_getenv_TZ` (`0`, C's null `char *`), `syms_of_timefns` (`doc`, the
`syms_of_filelock` row), `Fformat_time_string` (`1` -> the mirror's `bool`, the casetab
precedent) and `Fcurrent_time_string` (`pridmax` -> the literal `%jd`, the `lock_file_1`
row).  No `token-aliases.tsv` family and no `excluded-macros.txt` row was needed.

### Condition table

Every `#if` is decided by `docs/baseline/config/darwin-arm64.macros.tsv`, but this file is
*not* in `cond-eval-verified.txt`, so `tools/extract-abi.py` uses the blind evaluator for
it (see "Carried arms"): the whole conflict is the `FIXNUM_OVERFLOW_P` / `ULONG_MAX`
pair.  `python3 tools/check-cond-eval.py --show-unknown src/timefns.c` reports
`30 conditions, 7 unknown, 4 disagreements [NOT VERIFIED]` (C 74, 81, 86, 87, 667, 676,
915 unknown; the four disagreements are C 86, 87, 2087 and 2141).  The integrator's
decision replaces the stop-and-report of the previous round.

### Line count

`docs/ports/line-drift-tolerance.tsv`'s `timefns` row declares the default **`total`**
basis at 0.05: the dead text is carried, not recorded, so C's whole line count is the
baseline and the port's +1.3% is the fixed costs (the `// femacs:` note, the `#![allow]`
block, the include block, the use-site declarations, the `#[cfg]` attributes and the
comments naming each carried arm) less what the fourteen `DEFUN!` docstrings save by
collapsing C's comment lines into literals.

### Changes outside the unit (for integrator review)

1. `femacs/src/include/systime.rs`: the ten declarations the mirror carries were moved
   into a `pub mod systime_decls { use super::*; … } use systime_decls::*;` (the
   `time.in.rs` `time_decls` / `bignum_externs` shape, PORTING.md 0.1-10).  `timefns.c` is
   the *defining* unit of seven of them, and a definition at the crate root collided with
   the crate-root declaration (`E0428`, measured).  The two callers (`atimer.rs`,
   `profiler.rs`) are unaffected: the glob import keeps the same names in scope.  The
   mirror's ledger (`docs/ports/include-systime.md`) records the change.
2. `docs/ports/macro-body-notes.tsv`: the eleven rows above (appended).
3. `docs/ports/line-drift-tolerance.tsv`: the `timefns` row above (appended).
