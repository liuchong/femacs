# emacs/lib/strftime.c (2446 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 11
- [ ] memcpy_lowcase  (line 408, static)
- [ ] memcpy_uppcase  (line 419, static)
- [ ] c_locale  (line 448, static)
- [ ] utc_timezone  (line 471, static)
- [ ] should_remove_ampm  (line 486, static)
- [ ] tm_diff  (line 838, static)
- [ ] iso_week_days  (line 870, static)
- [ ] underlying_strftime  (line 944, static)
- [ ] get_tm_zone  (line 1085, static)
- [ ] my_strftime  (line 1137)
- [ ] __strftime_internal  (line 1210, static)

## Macros defined — 81
- [ ] ABALTMON_1  (line 142, body 97bf5d05b9d9)
- [ ] CAL_ARGS  (line 191, body da39a3ee5e6b)
- [ ] CHAR_T  (line 138, body 71fafc4e2fc1)
- [ ] DO_MAYBE_SIGNED_NUMBER  (line 1478, body a1e78fee7f48)
- [ ] DO_MULTIBYTE  (line 102, body ba1447bc6b31)
- [ ] DO_NUMBER  (line 1466, body 6ac067adcf55)
- [ ] DO_NUMBER_SPACEPAD  (line 1500, body e69fee274cfd)
- [ ] DO_SIGNED_NUMBER  (line 1474, body 891a1f88bde1)
- [ ] DO_TZ_OFFSET  (line 1491, body e74d1b0bab1a)
- [ ] DO_YEARISH  (line 1476, body 964c96fa7814)
- [ ] FAILURE  (line 229, body eb883130e83d)
- [ ] FPRINTFTIME  (line 20, body b6589fc6ab0d)
- [ ] FPUTC  (line 233, body 7fcb69046e09)
- [ ] HAVE_NATIVE_TIME_Z  (line 434, body f2337ca9aed4)
- [ ] HAVE_ONLY_C_LOCALE  (line 50, body b6589fc6ab0d)
- [ ] HAVE_STRUCT_ERA_ENTRY  (line 31, body 356a192b7913)
- [ ] HAVE_STRUCT_TM_TM_GMTOFF  (line 32, body 356a192b7913)
- [ ] HAVE_STRUCT_TM_TM_ZONE  (line 33, body 356a192b7913)
- [ ] HELPER_LOCALE_ARG  (line 363, body da39a3ee5e6b)
- [ ] ISDIGIT  (line 393, body fcfa890ac893)
- [ ] ISO_WEEK1_WDAY  (line 866, body 1b6453892473)
- [ ] ISO_WEEK_START_WDAY  (line 865, body 356a192b7913)
- [ ] LOCALE_ARG  (line 359, body da39a3ee5e6b)
- [ ] LOCALE_PARAM  (line 358, body da39a3ee5e6b)
- [ ] L_  (line 140, body 5dfe08d5f0fd)
- [ ] MEMCPY  (line 144, body c5d3fb3a32c0)
- [ ] MULTIBYTE_IS_FORMAT_SAFE  (line 101, body 356a192b7913)
- [ ] NLW  (line 141, body 50d6fddea6d8)
- [ ] REQUIRE_GNUISH_STRFTIME_AM_PM  (line 63, body 7cb6efb98ba5)
- [ ] SHR  (line 159, body 8619c68aae39)
- [ ] STREAM_OR_CHAR_T  (line 214, body 3bece736b3f1)
- [ ] STRFTIME_ARG  (line 215, body 291885db8973)
- [ ] STRLEN  (line 145, body 2d3571aeac9e)
- [ ] SUPPORT_NON_GREG_CALENDARS_IN_STRFTIME  (line 82, body 7cb6efb98ba5)
- [ ] TM_YEAR_BASE  (line 173, body 259e58e18997)
- [ ] TOLOWER  (line 385, body 108ad3cf0f6e)
- [ ] TOUPPER  (line 384, body f2d13032e3fc)
- [ ] UCHAR_T  (line 139, body 3dc11840724e)
- [ ] USE_C_LOCALE  (line 26, body b6589fc6ab0d)
- [ ] USE_IN_EXTENDED_LOCALE_MODEL  (line 30, body 356a192b7913)
- [ ] YDAY_MINIMUM  (line 867, body 4ea3953420f9)
- [ ] _NL_CURRENT  (line 352, body 4cca9ba0aa1d)
- [ ] __gmtime_r  (line 201, body 09b6aef5407c)
- [ ] __isleap  (line 178, body 95de658df4f7)
- [ ] __mbsrtowcs_l  (line 339, body 153058c388e0)
- [ ] a_altmonth  (line 1249, body 8b28cb2d253c)
- [ ] a_month  (line 1273, body 479eaad5886e)
- [ ] a_wkday  (line 1266, body 810c31dc65ed)
- [ ] aam_len  (line 1261, body 9592e652215f)
- [ ] add  (line 263, body afeb3b1275cb)
- [ ] add1  (line 300, body 3773d130ee71)
- [ ] advance  (line 260, body 09f8504f9344)
- [ ] am_len  (line 1275, body 77de68daecd8)
- [ ] ampm  (line 1279, body ea0169ec69c9)
- [ ] ap_len  (line 1280, body da4b9237bacc)
- [ ] aw_len  (line 1268, body 77de68daecd8)
- [ ] cpy  (line 307, body f8dc68d4e641)
- [ ] extra_args  (line 911, body da39a3ee5e6b)
- [ ] extra_args_spec  (line 912, body da39a3ee5e6b)
- [ ] f_altmonth  (line 1252, body 470f0e9736ee)
- [ ] f_month  (line 1276, body 479eaad5886e)
- [ ] f_wkday  (line 1269, body 810c31dc65ed)
- [ ] incr_overflow  (line 271, body 25edad550586)
- [ ] memset_byte  (line 242, body 6ac6c9fc6517)
- [ ] memset_space  (line 253, body 8e1063c506af)
- [ ] memset_zero  (line 254, body c6b1f937054c)
- [ ] mktime  (line 202, body 7d555aeaf8ae)
- [ ] mktime_z  (line 196, body 25537d99b389)
- [ ] my_strftime  (line 908, body de093aaea82f)
- [ ] nl_get_alt_digit  (line 909, body 9fd51e1dd6ef)
- [ ] ns  (line 915, body b6589fc6ab0d)
- [ ] strftime  (line 349, body 09afddd0c73a)
- [ ] time_t  (line 200, body 4bc9aebb49e0)
- [ ] tm_diff  (line 835, body 56b8caaab899)
- [ ] tz  (line 914, body 356a192b7913)
- [ ] tzname  (line 197, body e5810cd2c06c)
- [ ] tzset  (line 198, body 3af6d925ab03)
- [ ] wcsftime  (line 350, body 5be8d030b25d)
- [ ] width_add  (line 274, body f82539be670b)
- [ ] width_add1  (line 304, body 1e67d593f538)
- [ ] width_cpy  (line 326, body 64523a121b79)

## Types and their fields — 0

## File-scope objects — 0

# ----------------------------------------------------------------------
## Port plan (round 42, Kierkegaard) - reconnaissance only, no `.rs` yet

**Status: not ported.**  This round stops at the checklist plus the arm-liveness map and the
three constructions that need adjudication before ~900 lines are written; PORTING.md's
"未知项一律不得自行发挥" (record first, adjudicate, then write) is the reason.  No
`femacs/lib/strftime.rs` exists yet, so `extract-abi.py --all` does not pair this file.

### Arm liveness (measured, not guessed)

`_LIBC` is undefined on the frozen target and the other conditions come from the frozen
macro table; the preprocessor settles every one of them.  Command (the include paths are the
frozen build's own):

    clang -dM -E -DHAVE_CONFIG_H -I build/emacs-ref/lib -I build/emacs-ref/src \
          -I emacs/lib -I emacs/src emacs/lib/strftime.c > /tmp/strftime.macros

The values that decide this file: `FPRINTFTIME 0`, `USE_C_LOCALE 0`,
`HAVE_ONLY_C_LOCALE 0`, `REQUIRE_GNUISH_STRFTIME_AM_PM false`,
`SUPPORT_NON_GREG_CALENDARS_IN_STRFTIME false`, `MULTIBYTE_IS_FORMAT_SAFE 1`,
`DO_MULTIBYTE (!1)`, `HAVE_STRUCT_TM_TM_ZONE 1`, `HAVE_STRUCT_TM_TM_GMTOFF 1`,
`HAVE_STRFTIME_Z undef` (so `HAVE_NATIVE_TIME_Z` is 0), `_NL_CURRENT` undefined,
`_GL_GNUC_PREREQ(7,0) 1` with `__OPTIMIZE__` undefined.

Live: the `#else` arms of every `#ifdef _LIBC` / `#ifdef COMPILE_WIDE` / `#if FPRINTFTIME` /
`#if DO_MULTIBYTE` / `#if SUPPORT_NON_GREG_CALENDARS_IN_STRFTIME` / `#if HAVE_ONLY_C_LOCALE`
test, i.e. `CHAR_T char`, `L_(Str) Str`, `NLW(Sym) Sym`, `MEMCPY memcpy`, `STRLEN strlen`,
`retval_t = sbyte_count_t = ptrdiff_t`, `FAILURE (-1)`, `TOUPPER/TOUPPER` = `toupper`/`tolower`,
`my_strftime strftime`, `tz 1`, `ns 0`, `CAL_ARGS`/`STRFTIME_ARG`/`LOCALE_PARAM` empty.

Dead (their text is **not** compiled by the frozen target, but `--check` is blind for this
file, so their tokens are still demanded - see the plan below):

| lines | what | why |
|---|---|---|
| 30-33 | `_LIBC`'s three `HAVE_STRUCT_*`/`USE_IN_EXTENDED_LOCALE_MODEL` defines and `localeinfo.h` | `_LIBC` undefined |
| 37-38, 59-60, 76-77, 79-82, 86, 105-106, 119-121, 128-136, 183-189, 196-203, 209-212, 225-226, 233-251, 258, 268, 302, 309-324, 337-340, 345-356, 360-363, 368-378 | the `FPRINTFTIME`, `COMPILE_WIDE`, `SUPPORT_NON_GREG_CALENDARS`, `USE_IN_EXTENDED_LOCALE_MODEL`, `_LIBC`, NetBSD/sun and Win32 arms | the config values above |
| 438-455 | `c_locale` (the C-locale cache) | needs `USE_C_LOCALE && HAVE_STRFTIME_L` (0) |
| 458-478 | `utc_timezone` | needs `HAVE_NATIVE_TIME_Z` (0) |
| 483-829 | `should_remove_ampm` (347 lines of NetBSD/sun locale-name tables) | the arm's own condition |
| 833-856 | `tm_diff` | `!HAVE_STRUCT_TM_TM_GMTOFF` is false (the frozen `struct tm` has `tm_gmtoff`) |
| 1200-1284 | the `_NL_CURRENT`/`c_weekday_names` locale-data arms of the `a_wkday`-family macros | `_NL_CURRENT` undefined and both alternative arms false |
| in the switch | the `#if defined _NL_CURRENT || HAVE_ONLY_C_LOCALE || (USE_C_LOCALE && !HAVE_STRFTIME_L)` arms of the locale conversions | same: those cases are live as `goto underlying_strftime;` (49 cases, only 11 of them have a live body of their own) |

Live functions: `memcpy_lowcase`, `memcpy_uppcase`, `iso_week_days`, `underlying_strftime`,
`get_tm_zone`, `my_strftime`, `__strftime_internal` (7).  The other four inventory entries
(`c_locale`, `utc_timezone`, `should_remove_ampm`, `tm_diff`) are dead text whose *bodies*
`--check` still compares, because the checker keeps both branches for this file.

### Mirrors the live code needs (P-26)

* `lib/include/time.in.rs` - `struct tm` (the frozen target's layout, with `tm_gmtoff` and
  `tm_zone`, which `HAVE_STRUCT_TM_TM_GMTOFF/_ZONE 1` makes live), `timezone_t`
  (`typedef struct tm_zone *timezone_t;` is time.in.h 494), and the declarations the code
  names: `gmtime`, `localtime`, `tzset`, `tzname`, `mktime`, `timegm`, `localtime_rz`,
  `mktime_z`, `strftime` (the *underlying* one `underlying_strftime` calls), `getenv`.
* `lib/include/time-internal.rs` (new, mirrors lib/time-internal.h): `struct tm_zone`,
  `set_tz`, `revert_tz`, `local_tz` (`((timezone_t) 1)`), `tzalloc`, `tzfree`.
* `lib/include/strftime.rs` (new, mirrors lib/strftime.h): `nstrftime`, `c_nstrftime`.
* `lib/include/locale.rs` - only if the dead `c_locale` arm is carried: `locale_t`,
  `LC_ALL_MASK`, `newlocale`.
* `stdcountof`: the C token in the live text is the C23 keyword `_Countof` (gnulib's
  `stdcountof.h` has `#define countof _Countof`), so the port needs a `_Countof` macro
  (or a `countof,_Countof` token-aliases row).
* `INT_STRLEN_BOUND (time_t)` and `INT_STRLEN_BOUND (int)` in the `buf` declaration: a
  recorded instantiation constant (the md5 `INT_BUFSIZE_BOUND` decision).
* `ckd_add`/`ckd_mul` in the width parser: the `overflowing_add/overflowing_mul` families
  (PORTING.md 2.6/P-15), already registered in token-aliases.tsv.

No new `#if` in this file is structural enough for a `#[cfg(feature = …)]` of its own beyond
the `FPRINTFTIME`/`COMPILE_WIDE`/`USE_IN_EXTENDED_LOCALE_MODEL`/`sun&netbsd`/`openssl`-class
features the other units already use.
