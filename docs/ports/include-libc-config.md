# emacs/lib/libc-config.h (209 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 16
- [x] SHLIB_COMPAT  (line 207, body b6589fc6ab0d)
- [x] _FEATURES_H  (line 89, body 356a192b7913)
- [x] __GNUC_PREREQ  (line 60, body b6589fc6ab0d)
- [x] __GNULIB_CDEFS  (line 93, body da39a3ee5e6b)
- [x] __glibc_clang_prereq  (line 78, body b6589fc6ab0d)
- [x] __inline  (line 190, body da39a3ee5e6b)
- [x] __set_errno  (line 46, body 988357e59c7f)
- [x] attribute_hidden  (line 198, body da39a3ee5e6b)
- [x] compat_symbol  (line 208, body c9112b5fae3e)
- [x] libc_hidden_def  (line 200, body da39a3ee5e6b)
- [x] libc_hidden_proto  (line 199, body da39a3ee5e6b)
- [x] libc_hidden_ver  (line 202, body da39a3ee5e6b)
- [x] libc_hidden_weak  (line 201, body da39a3ee5e6b)
- [x] strong_alias  (line 203, body da39a3ee5e6b)
- [x] versioned_symbol  (line 209, body c9112b5fae3e)
- [x] weak_alias  (line 204, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

- **Status: the unit is PORTED** (`lib/include/libc-config.rs`, 2026-10-04).

## Decisions

Criterion: the file is only ever included as a *first* header (`#include <config.h>` then
`<errno.h>`), so its own text decides the arms - there is no `@PLACEHOLDER@` and no
`#include_next`, and `check-cond-eval` reports `9 conditions, 1 unknown, 2 disagreements
[NOT VERIFIED]` whose two disagreements are the `defined __clang__` predicates the evaluator keeps
live (conservative, P-66).

Live arms, measured per item in the mirror's comments: `__set_errno` is the fallback (this
target's `<errno.h>` has none), `__GNUC_PREREQ`/`__glibc_clang_prereq` are the `0` bodies,
`_FEATURES_H` is 1, and `__GNULIB_CDEFS`, `attribute_hidden`, the four `libc_hidden_*`,
`strong_alias`, `weak_alias` and `__inline` all have empty bodies (markers or no-ops).
`SHLIB_COMPAT` is gnulib's `0`.

Recorded instead of carried, in `docs/ports/excluded-macros.txt`: `compat_symbol` and
`versioned_symbol`, whose C body is `extern int dummy` - a declaration for glibc's compat-symbol
machinery that the port has nothing for.

## Evidence

```
$ python3 tools/extract-abi.py --check emacs/lib/libc-config.h lib/include/libc-config.rs
OK emacs/lib/libc-config.h -> lib/include/libc-config.rs            (0 problem rows)
$ cd femacs && cargo run -q -p xtask -- build-tu lib/libc-config.h
built lib/include/libc-config.rs -> ../build/femacs/include_libc-config.o
```

The mirror needs no name from an includer (everything is absolute `core::ffi::*`), which is why
`build-tu lib/libc-config.h` is its compile evidence; the five units that include this header in C
(`tempname.c`, `canonicalize-lgpl.c`, `strftime.c`, `mktime.c`, `timegm.c`) still carry their own
`__set_errno!`/`weak_alias!` copies, and re-pointing them at this mirror is scheduled separately
each one at a time (the `string.in.rs`/`ftoastr.rs` shape).
