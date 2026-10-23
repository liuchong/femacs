# emacs/lib/time.in.h (646 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 26
- [x] GNULIB_defined_TIME_UTC  (line 123, body 356a192b7913)
- [x] GNULIB_defined_struct_time_t_must_be_integral  (line 116, body 356a192b7913)
- [x] GNULIB_defined_struct_timespec  (line 97, body 356a192b7913)
- [x] GNULIB_defined_timezone_t  (line 499, body 356a192b7913)
- [x] TIME_UTC  (line 122, body 356a192b7913)
- [x] _  (line 45, body 6564569645de)
- [x] _POSIX_THREAD_SAFE_FUNCTIONS  (line 52, body 6d19a880eaf5)
- [x] ctime  (line 437, body a9a50e1ed098)
- [x] gmtime  (line 400, body 9c780a8e1938)
- [x] gmtime_r  (line 341, body cd56458d6151)
- [x] localtime  (line 378, body bb9db16d3c93)
- [x] localtime_r  (line 319, body c6a96e77aa28)
- [x] localtime_rz  (line 539, body 51cb23a8ac47)
- [x] mktime  (line 295, body cfb74ca8ad18)
- [x] mktime_z  (line 568, body 1928894644b3)
- [x] nanosleep  (line 227, body 1c19a527b56d)
- [x] strftime  (line 460, body b8e523af4963)
- [x] time  (line 204, body 474ea98f747c)
- [x] timegm  (line 598, body b6b11795f8b7)
- [x] timespec  (line 91, body 556386842976)
- [x] timespec_get  (line 150, body f753e41133a6)
- [x] timespec_getres  (line 178, body f9b159d93189)
- [x] timezone_t  (line 497, body 85642344ef43)
- [x] tzfree  (line 525, body 45cbede3b328)
- [x] tzname  (line 140, body 7adeb914e850)
- [x] tzset  (line 277, body cf790664c255)

## Types and their fields — 1
- [x] timespec: time_t tv_sec, long int tv_nsec

## File-scope objects — 0

## Decisions

`emacs/lib/time.in.h` @ `fd7afced403`; port `lib/include/time.in.rs`.

**The template.**  The reference is a gnulib template (`@PRAGMA_SYSTEM_HEADER@`,
`@GUARD_PREFIX@`, `@INCLUDE_NEXT@`, `@NEXT_TIME_H@`, `@TIME_H_DEFINES_STRUCT_TIMESPEC@`,
`@TIME_H_DEFINES_TIME_UTC@`, `@GNULIB_*@`, `@REPLACE_*@`, `@HAVE_*@`), so
`tools/check-cond-eval.py` cannot decide its conditions and it stays off the verified list:
`extract-abi.py` reads every arm as live and the mirror must name every macro the template
can define (26 names).  The frozen substitution is the generated
`build/emacs-ref/lib/time.h` (1193 lines), and its values are the table in the mirror's own
header comment.  What that table means:

- **live** blocks (the port carries their declarations): `GNULIB_NANOSLEEP`/`REPLACE_NANOSLEEP`
  (C 224-243: `nanosleep` is `rpl_nanosleep`), `GNULIB_MDA_TZSET` (C 270-289: the system
  `tzset`), `GNULIB_MKTIME`/`REPLACE_MKTIME` (C 292-310: `rpl_mktime`),
  `GNULIB_TIME_R` (C 315-369: the system `gmtime_r`/`localtime_r`), `GNULIB_TIME_RZ` +
  `HAVE_TZALLOC`=0 + `HAVE_TIMEZONE_T`=0 (C 484-591: `typedef struct tm_zone *timezone_t;`
  and the `tzalloc`/`tzfree`/`localtime_rz`/`mktime_z` declarations), `GNULIB_TIMEGM`/
  `REPLACE_TIMEGM` (C 594-616: `rpl_timegm`), and C 107-117's
  `struct __time_t_must_be_integral` assertion;
- **dead** blocks (the port keeps the system entry and/or the arm's names): the
  `struct timespec` arm (C 76-105), `TIME_UTC` (C 119-125), `tzname` (C 127-142),
  `timespec_get`/`timespec_getres` (C 146-198), `time` (C 201-219), `localtime` (C 374-394),
  `gmtime` (C 396-409), `strptime` (C 414-430), `ctime` (C 434-453), `strftime` (C 457-482).

**The two halves.**  The port has no `#include_next`, so the mirror's declarations are the
*system* `<time.h>` / `<sys/time.h>` entries (plus the two `rpl_` definitions the reference
objects call), and the template's own names are carried the way the endian/byteswap mirrors
carry theirs:

| item | shape | why |
| --- | --- | --- |
| `_GL_TIME_H` | `pub const … = 1` | the guard, the frozen substitution of `_@GUARD_PREFIX@_TIME_H` (the template's own spelling is `_`, an `excluded-macros.txt` row) |
| `_POSIX_THREAD_SAFE_FUNCTIONS` | `#[cfg (feature = "mingw32")] pub const … : core::ffi::c_long = 200112;` | C 49-53's mingw arm, dead here; C's value `200112L` |
| `timespec` | `#[cfg (feature = "no_time_h_defines_struct_timespec")] macro_rules! … { () => { rpl_timespec } }` | the dead arm's rename; the extractor compares the body, which is the single token `rpl_timespec` |
| `GNULIB_defined_struct_timespec` | `pub const … = 1` | C 89-97's guard |
| `GNULIB_defined_struct_time_t_must_be_integral` | `pub const … = 1` | C 107-117, the live block; the struct itself is the compiler-level `time_t`-is-integral assertion |
| `TIME_UTC`, `GNULIB_defined_TIME_UTC` | `pub const … = 1` | C 119-125, the dead arm's constant and guard |
| `tzname` | `#[cfg (feature = "cygwin_win32")] macro_rules! … { () => { _tzname } }` | C 139-140's Windows rename; the live declaration is the `pub static mut tzname` the mirror already had |
| `timezone_t` | `#[cfg (feature = "have_timezone_t")] macro_rules! … { () => { rpl_timezone_t } }` | C 496-497's dead `#else` rename; the live arm is the `timezone_t` alias the mirror already had |
| `GNULIB_defined_timezone_t` | `pub const … = 1` | C 492-499's guard |
| `ctime`, `time`, `timespec_get`, `timespec_getres`, `tzalloc`, `tzfree` | declarations in `time_decls` | the frozen header's live system entries; the four the extractor names as macros are declared *functions*, which is what C's `_GL_CXXALIAS_SYS` gives |

**`struct __time_t_must_be_integral`** (C 113-115) is not carried as a struct: its only
member is a one-bit bitfield of width `(time_t) 1`, whose purpose is to make the compiler
reject a floating `time_t`.  The port's `time_t` is `i64` (the alias in the mirror), so the
assertion holds by construction, and PORTING.md 1.3's bitfield rule leaves no member to
mirror; `cinventory` reports no field for it either (a bitfield-only struct contributes
nothing to "Types and their fields", which lists only `timespec`).

**`timespec` the struct** keeps its layout assertions (56-byte-free `timespec` is 16 bytes:
`i64 tv_sec` + `c_long tv_nsec`), and the `tm` struct keeps the measured 56-byte layout.

**Convergence (PORTING.md 0.1-3, same batch).**  The declarations this mirror added had two
use-site copies, both deleted with the consumer left reading the mirror:

| unit | copy | now |
| --- | --- | --- |
| `src/timefns.rs` 111 | `fn tzalloc (name: *const c_char) -> timezone_t;` | from `time_decls` (the mirror) |
| `src/timefns.rs` 113 | `fn tzfree (tz: timezone_t);` | from `time_decls` (the mirror) |

`check-mirror-dupes.py` reported exactly those two before the edit and 0 failures after.
No unit spells `ctime`, `time`, `timespec_get`, `timespec_getres`, `TIME_UTC`,
`GNULIB_defined_*`, `_POSIX_THREAD_SAFE_FUNCTIONS` or `_GL_TIME_H` at top level, and
`nt/inc/include/ms-w32.rs`'s own `tzname`/`ctime` macro_rules live outside
`src`/`lib`, which is the gate's scope.

## Evidence

```
$ python3 tools/extract-abi.py --check emacs/lib/time.in.h lib/include/time.in.rs
  (1 C macros have a recorded Rust counterpart: _)
  (1 C macros have several definitions (configuration branches), body not compared: tzset)
OK emacs/lib/time.in.h -> lib/include/time.in.rs

$ cd /tmp/femacs-probe2 && rustc --edition 2024 --crate-type=lib --emit=obj -C panic=abort \
    time_probe.rs -o time_probe.o
exit=0        # the probe use-imports the ffi scalars + include!s the mirror, asserts
              # size_of::<timespec>() == 16 and size_of::<tm>() == 56 and the value 1 of
              # TIME_UTC / GNULIB_defined_TIME_UTC / _struct_timespec /
              # _struct_time_t_must_be_integral / _timezone_t, and calls ctime, time,
              # timespec_get, timespec_getres, tzalloc, tzfree (6 warnings, no error)

$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-mir2 cargo run -q -p xtask -- build-tu <unit.c>
built src/timefns.rs -> ../build/femacs/timefns.o    # exit 0 (the edited consumer)
built lib/mktime.rs  -> ../build/femacs/mktime.o     # exit 0
built lib/time_rz.rs -> ../build/femacs/time_rz.o    # exit 0 (defines tzalloc/tzfree)
built lib/boot-time.rs -> ../build/femacs/boot-time.o # exit 0

$ python3 tools/extract-abi.py --check emacs/<unit.c> <unit.rs>
OK emacs/src/timefns.c -> src/timefns.rs
OK emacs/lib/mktime.c -> lib/mktime.rs
OK emacs/lib/time_rz.c -> lib/time_rz.rs
OK emacs/lib/boot-time.c -> lib/boot-time.rs

$ python3 tools/check-mirror-dupes.py
1 duplicate declaration(s), 1 ratchet row(s), 0 failure(s)
$ python3 tools/check-declarations.py
13 declaration sites (0 forbidden, 13 allowed literals), 0 failures
$ python3 tools/check-verbatim.py
6 trees, 4607 files, 0 failures
$ python3 tools/check-globals.py
check-globals: 283 units, 0 DEFVAR_* variables without a definition in their own unit
```
