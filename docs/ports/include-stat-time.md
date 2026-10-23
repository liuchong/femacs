# emacs/lib/stat-time.h (266 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 10
- [x] get_stat_atime_ns  (line 71)
- [x] get_stat_ctime_ns  (line 84)
- [x] get_stat_mtime_ns  (line 97)
- [x] get_stat_birthtime_ns  (line 110)
- [x] _gl_make_timespec  (line 137, static)
- [x] get_stat_atime  (line 148)
- [x] get_stat_ctime  (line 159)
- [x] get_stat_mtime  (line 170)
- [x] get_stat_birthtime  (line 182)
- [x] stat_time_normalize  (line 226)

## Macros defined — 6
- [x] STAT_TIMESPEC  (line 61, body a480da412230)
- [x] STAT_TIMESPEC_NS  (line 66, body cf3ba08b866f)
- [x] STAT_TIMESPEC_OFFSETOF  (line 62, body 42d7c7703d6d)
- [x] STAT_TIME_H  (line 21, body 356a192b7913)
- [x] _GL_STAT_TIME_INLINE  (line 38, body a960df3ab827)
- [x] _gl_make_timespec  (line 128, body f0b597be7693)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/lib/stat-time.h` @ `fd7afced403`; port `lib/include/stat-time.rs`.  Fourteen of the
sixteen checklist items are ticked; the two unchecked ones need a row in a shared file that
this lane must not write (the integrator owns `docs/ports/excluded-macros.txt` and
`docs/ports/macro-body-notes.tsv`), and both rows are given verbatim below.  With them the
pair is `OK`.

**What the header is.**  Nine `_GL_STAT_TIME_INLINE` definitions (the C99 inline model), so
they are ordinary Rust items here and the `.c` wrapper `lib/stat-time.rs` keeps the
`_GL_STAT_TIME_INLINE`/`_GL_EXTERN_INLINE` relationship and re-exports the nine (PORTING.md
10.6/10.12, P-81).  The header is **not** in `docs/baseline/config/cond-eval-verified.txt`,
so `extract-abi.py` reads every arm as live (P-08's blind default) and the mirror carries the
dead arms as P-51 text - which is why the bodies below are longer than the frozen
configuration's own text.

**The configuration.**  `HAVE_STRUCT_STAT_ST_ATIMESPEC_TV_NSEC` and
`HAVE_STRUCT_STAT_ST_BIRTHTIMESPEC_TV_NSEC` are 1; `HAVE_STRUCT_STAT_ST_ATIM_TV_NSEC`,
`HAVE_STRUCT_STAT_ST_ATIMENSEC`, `HAVE_STRUCT_STAT_ST_ATIM_ST__TIM_TV_NSEC`,
`HAVE_STRUCT_STAT_ST_BIRTHTIM_TV_NSEC`, `HAVE_STRUCT_STAT_ST_BIRTHTIMENSEC` and
`_GL_WINDOWS_STAT_TIMESPEC` are `undef` (measured in `docs/baseline/config/darwin-arm64.macros.tsv`).
So C 58-67's arm in force is the `HAVE_STRUCT_STAT_ST_ATIMESPEC_TV_NSEC` one:
`STAT_TIMESPEC (st, st_xtim)` is `(st)->st_xtim##espec` and `STAT_TIMESPEC_OFFSETOF (st_xtim)`
is `offsetof (struct stat, st_xtim##espec)`.

**The two private macros are written out, not dropped.**  Rust cannot paste an identifier, so
`STAT_TIMESPEC!` and `STAT_TIMESPEC_OFFSETOF!` are `macro_rules!` with one arm per argument
the header's own functions pass (`st_atim`, `st_ctim`, `st_mtim`, `st_birthtim` - the four its
document comment names); that is the `NUMNAME`/`DYNARRAY_NAME` shape, and a function body then
keeps C's own text (`STAT_TIMESPEC! (st, st_atim)`).  The extractor compares a body's tokens,
so reading the member directly (the shape this mirror had while it was WIP) leaves
`STAT_TIMESPEC`/`STAT_TIMESPEC_OFFSETOF`/`STAT_TIMESPEC_NS` unmentioned and fails `--check`;
the existing rows for those three names in `docs/ports/excluded-macros.txt` keep their
meaning (the names are not *required*), but their reasons ("the port reads the st_*timespec
member directly at each of the nine use sites") are now stale - the integrator may want to
refresh them.  `_gl_make_timespec` is written as the function C's own **C++ arm** spells
(`static inline struct timespec _gl_make_timespec (time_t sec, long nsec)`); the C99
compound-literal arm is the same value, and the port's `let mut ts: timespec =
std::mem::zeroed ();` is PORTING.md 0.1-13's shape.

**The dead arms are P-51 text under two port-local feature names** (spelled out in the file's
own note, with a `const _: () = assert! (!cfg! (…))` for each, and off for every ported
unit): `stat_time_stat_timespec_ns` (C 70-80's `STAT_TIMESPEC_NS` arm) and
`stat_time_member_absent` (C 81-82's `# else`, and C 155/166/177's `_gl_make_timespec`
fallbacks), plus `stat_time_birthtime_nsec_arm`, `windowsnt` and `stat_time_no_birthtime` for
`get_stat_birthtime`'s arms.  The bodies of those arms are what keep `STAT_TIMESPEC_NS`,
`st_atime`/`st_ctime`/`st_mtime`, `st_birthtime`/`st_birthtimensec`/`st_ctim`/`st_ctime` and
the `return 0` arms' `0` in the comparison.

**The two unchecked boxes.**

1. `_GL_STAT_TIME_INLINE` (C 38-40's `#ifndef` fallback): the unit
   `lib/stat-time.rs` 11 defines the name to `_GL_EXTERN_INLINE` before including this
   mirror, exactly as `lib/stat-time.c` does, and Rust has no macro shadowing - so the mirror
   **cannot** hold the name without an `E0428`.  This is the same construct as the six
   `*_INLINE` rows already in `docs/ports/excluded-macros.txt`, and the row to add is:

```
_GL_STAT_TIME_INLINE	syntax	stat-time.h 38-40's `#ifndef _GL_STAT_TIME_INLINE / # define _GL_STAT_TIME_INLINE _GL_INLINE` spells gnulib's `static inline`; the header's nine functions are `#[inline(always)] pub fn` in the mirror, and the unit `lib/stat-time.c` carries the qualifier itself (`#define _GL_STAT_TIME_INLINE _GL_EXTERN_INLINE`, the timespec/issymlink shape), so the mirror cannot hold the name without an E0428
```

2. `get_stat_birthtime` needs two tokens of C's `#if` **condition text**:
   `HAVE_STRUCT_STAT_ST_BIRTHTIM_TV_NSEC` and `HAVE_STRUCT_STAT_ST_BIRTHTIMENSEC` sit on the
   *continuation* lines of C 190-191 and 212-214 (`#if (defined … \` then `|| defined …)`),
   which the extractor's line-based `#` filter does not drop, so they are compared as body
   tokens although C's own `#if` line is not.  The port reads the configuration through
   `#[cfg (feature = …)]` (whose string is blanked by the same tokenizer), so the names have
   no Rust counterpart - the `network_interface_info` row is the same class.  The row to add
   is:

```
get_stat_birthtime	have_struct_stat_st_birthtim_tv_nsec,have_struct_stat_st_birthtimensec	stat-time.h 190-191/212-214's `#if` condition text (`defined HAVE_STRUCT_STAT_ST_BIRTHTIM_TV_NSEC`, `defined HAVE_STRUCT_STAT_ST_BIRTHTIMENSEC`), which has no Rust counterpart: the port reads the configuration through `#[cfg (feature = …)]` (the network_interface_info row's class)
```

**Convergence**: no unit or mirror declared any of this header's names itself (the four
private macros were already recorded in `docs/ports/excluded-macros.txt`, and the nine
functions live only here), so there was nothing to delete; `check-mirror-dupes.py` stays at
its one ratchet row.

## Evidence

```
$ python3 tools/extract-abi.py --check emacs/lib/stat-time.h lib/include/stat-time.rs
  (4 C macros have a recorded Rust counterpart: STAT_TIMESPEC, STAT_TIMESPEC_NS,
   STAT_TIMESPEC_OFFSETOF, _gl_make_timespec)
- missing macro _GL_STAT_TIME_INLINE
- function get_stat_birthtime does not mention have_struct_stat_st_birthtim_tv_nsec,
  have_struct_stat_st_birthtimensec
FAIL emacs/lib/stat-time.h -> lib/include/stat-time.rs
# those two lines are exactly what the two rows above answer; nothing else is reported.

$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-mir-limits \
    cargo run -q -p xtask -- probe /tmp/femacs-probe/stat_time_wrap.rs \
    /tmp/femacs-probe/stat_time_probe.o --crate-type=lib --emit=obj -C panic=abort
exit=0        # the wrapper is the unit's `#![allow (…)]`, `use std::ffi::{…}`, then
              # `include! ("…/femacs/lib/include/stat-time.rs")` and one use of every item;
              # `xtask probe` materializes it (the mirror's own `include!`s are deduped by
              # path, P-61), which a bare `rustc` on the mirror would not do (its `time.in.rs`
              # reaches it twice, through `sys_stat.in.rs` as well).

$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-mir-limits \
    cargo run -q -p xtask -- build-tu <unit.c>          # each printed "built <unit.rs>", exit 0
lib/lstat.c    lib/fstatat.c   (the two `stat_time_normalize` callers)
lib/utimens.c  src/lread.c     (units whose mirrors include stat-time.rs)

$ python3 tools/check-mirror-dupes.py
1 duplicate declaration(s), 1 ratchet row(s), 0 failure(s)
$ python3 tools/check-declarations.py
13 declaration sites (0 forbidden, 13 allowed literals), 0 failures
$ python3 tools/check-verbatim.py
6 trees, 4607 files, 0 failures
$ python3 tools/check-globals.py
check-globals: 283 units, 0 DEFVAR_* variables without a definition in their own unit
```

**Measured, not predicted** (the two rows above are the integrator's to add, so they were
proved with copies): `extract-abi.py` takes `--excluded` and `--macro-notes`, so both rows
were appended to copies in `/tmp` and

```
$ python3 tools/extract-abi.py --check --excluded /tmp/mem-excl.txt \
    --macro-notes /tmp/mem-notes.tsv emacs/lib/stat-time.h lib/include/stat-time.rs
OK emacs/lib/stat-time.h -> lib/include/stat-time.rs
```

so the mirror plus those two rows is the complete pair.  Nothing in either shared file was
written.
