# emacs/lib/sys_types.in.h (138 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 12
- [x] GNULIB_defined_blkcnt_t  (line 132, body 356a192b7913)
- [x] GNULIB_defined_blksize_t  (line 124, body 356a192b7913)
- [x] GNULIB_defined_dev_t  (line 84, body 356a192b7913)
- [x] GNULIB_defined_ino_t  (line 104, body 356a192b7913)
- [x] GNULIB_defined_off64_t  (line 68, body 356a192b7913)
- [x] _  (line 49, body 1ff62f6206d8)
- [x] _GL_INCLUDING_SYS_TYPES_H  (line 44, body da39a3ee5e6b)
- [x] _GL_WINDOWS_64_BIT_OFF_T  (line 60, body 356a192b7913)
- [x] _GL_WINDOWS_STAT_INODES  (line 110, body 92f878d71f0f)
- [x] dev_t  (line 83, body 16a951da420a)
- [x] ino_t  (line 103, body 11a0d16dc1c7)
- [x] off_t  (line 57, body c7a965515c55)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/lib/sys_types.in.h` @ `fd7afced403`; port `femacs/lib/include/sys_types.in.rs`.

**The template.**  The reference is a gnulib template (`@PRAGMA_SYSTEM_HEADER@`,
`@GUARD_PREFIX@`, `@INCLUDE_NEXT@`, `@NEXT_SYS_TYPES_H@`, `@WINDOWS_64_BIT_OFF_T@`,
`@HAVE_OFF64_T@`, `@WINDOWS_STAT_INODES@`, `@HAVE_BLKSIZE_T@`, `@HAVE_BLKCNT_T@`), so
`tools/check-cond-eval.py` cannot decide its conditions and it stays off the verified list;
`extract-abi.py` therefore reads **every** arm as live and the mirror must name every macro
the template can define.  The frozen substitution is the generated header
`build/emacs-ref/lib/sys/types.h` (gnulib emits it under `lib/sys/`, not `lib/`), and its
substitutions are:

| placeholder | frozen value | effect on the generated header |
| --- | --- | --- |
| `@GUARD_PREFIX@` | `_GL` | guards are `_GL_SYS_TYPES_H` |
| `@WINDOWS_64_BIT_OFF_T@` | 0 | C 51-61 becomes `#if 0` (dead) |
| `@HAVE_OFF64_T@` | 1 | C 63-70 becomes `#if !1` (dead) |
| `@WINDOWS_STAT_INODES@` | 0 | C 72-112 becomes `#if 0` (dead) |
| `@HAVE_BLKSIZE_T@` | 1 | C 120-126 becomes `#if !1` (dead) |
| `@HAVE_BLKCNT_T@` | 1 | C 128-134 becomes `#if !1` (dead) |

So **no** type the template defines is live on this target: every type comes from the
system `<sys/types.h>` the template includes next (the port has no `#include_next`, so the
mirror's aliases are the measured system types - the `sizeof` probe in the file's header
comment, `2 2 4 8 8 4 8`).  `off64_t` therefore has no alias: `#if !1` is dead and the
system header does not define the name.

**The template's own half, and why it is carried anyway.**  The mirror also carries the
names of the dead arms, because P-26's criterion for a `.in.h` is the whole template: the
guard `_GL_SYS_TYPES_H` (the frozen substitution of `_@GUARD_PREFIX@_SYS_TYPES_H`; the
template's own spelling reads as `_` to the extractor, which
`docs/ports/excluded-macros.txt` already records), `_GL_INCLUDING_SYS_TYPES_H` (C 44/46's
define/undef around the include-next - a `pub const`, the `min-eloop-threshold.rs`
`_MIN_ELOOP_THRESHOLD_H` shape), the five `GNULIB_defined_*` markers and
`_GL_WINDOWS_64_BIT_OFF_T` (all `pub const … : c_int = 1;`, the `endian.in.rs` shape, and
exactly the value C's preprocessor holds in the arm), and the three rename/indicator macros
of the Windows arms.

**The Windows renames.**  `off_t`/`dev_t`/`ino_t` are the one class the mirror cannot spell
as a type alias plus a value: C's `#define dev_t rpl_dev_t` is a *name renaming*, and the
port's answer for the frozen target is the system type alias.  The extracts live under the
feature that names the non-frozen side (P-51, the `acl-internal.rs` `GETACLCNT` shape) so
they are never compiled on this target:

```
#[cfg (feature = "windows_64_bit_off_t")]  macro_rules! off_t { () => { long long int } }
#[cfg (feature = "windows_stat_inodes")]   macro_rules! dev_t { () => { rpl_dev_t } }
#[cfg (feature = "windows_stat_inodes")]   macro_rules! ino_t { () => { rpl_ino_t } }
```

`extract-abi.py` names `rpl_dev_t` for `dev_t` and, because `off_t`/`ino_t` have several
definitions (the `_MSC_VER` sibling and the two `WINDOWS_STAT_INODES` arms), it does not
compare their bodies; `dev_t`'s body is compared and the macro's body mentions
`rpl_dev_t`, so the pair is `OK`.  `_GL_WINDOWS_STAT_INODES` (C 110) is the same shape: its
template body is the m4 placeholder `@WINDOWS_STAT_INODES@`, which the extractor reads as
the token `WINDOWS_STAT_INODES`, so the mirror keeps that token as the macro's body and
records the frozen substitution (0) in the comment - a value constant would lose the token
the template writes, and would then be reported as a body the extractor cannot find.

**Every ticked box.**  `_` is the guard name read through `@GUARD_PREFIX@` (the
tree-wide `excluded-macros.txt` row); the other eleven are `--check`-verified counterparts
as described above.

## Evidence

```
$ python3 tools/extract-abi.py --check emacs/lib/sys_types.in.h femacs/lib/include/sys_types.in.rs
  (1 C macros have a recorded Rust counterpart: _)
  (3 C macros have several definitions (configuration branches), body not compared:
   GNULIB_defined_ino_t, ino_t, off_t)
OK emacs/lib/sys_types.in.h -> femacs/lib/include/sys_types.in.rs

$ cd /tmp/femacs-probe2 && rustc --edition 2024 --crate-type=lib --emit=obj -C panic=abort \
    sys_types_probe.rs -o sys_types_probe.o
exit=0        # the probe `use std::ffi::{c_int, c_uint};` + include!s the mirror and asserts
              # size_of for off_t 8 / pid_t 4 / mode_t 2 / nlink_t 2 / blksize_t 4 /
              # blkcnt_t 8 / ino_t 8 / dev_t 4 / uid_t 4 and the value 1 of
              # GNULIB_defined_off64_t / _dev_t / _blksize_t and _GL_WINDOWS_64_BIT_OFF_T

$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-mir2 cargo run -q -p xtask -- build-tu lib/fstatat.c
built lib/fstatat.rs -> ../build/femacs/fstatat.o        # exit 0
$ python3 tools/extract-abi.py --check emacs/lib/fstatat.c femacs/lib/fstatat.rs
OK emacs/lib/fstatat.c -> femacs/lib/fstatat.rs          # a real consumer of this mirror

$ python3 tools/check-mirror-dupes.py
1 duplicate declaration(s), 1 ratchet row(s), 0 failure(s)
$ python3 tools/check-declarations.py
13 declaration sites (0 forbidden, 13 allowed literals), 0 failures
$ python3 tools/check-verbatim.py
6 trees, 4607 files, 0 failures
$ python3 tools/check-globals.py
check-globals: 283 units, 0 DEFVAR_* variables without a definition in their own unit
```

No other unit spells any of the names this mirror adds (`GNULIB_defined_*`,
`_GL_WINDOWS*`, `_GL_INCLUDING_SYS_TYPES_H`, `_GL_SYS_TYPES_H`), so 0.1-3 had no use-site
copy to delete in the same batch.
