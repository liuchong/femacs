# emacs/lib/getopt.in.h (66 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 4
- [x] _  (line 42, body ae6d67cb7a1e)
- [x] _GL_SYSTEM_GETOPT  (line 32, body da39a3ee5e6b)
- [x] __need_system_stdlib_h  (line 52, body da39a3ee5e6b)
- [x] option  (line 36, body 79d11a3493c0)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

**Status: the unit is PORTED** (`femacs/lib/include/getopt.in.rs`, 2026-10-04).

Criterion: the generated header (`build/emacs-ref/lib/getopt.h`), whose measured substitutions make
`HAVE_GETOPT_H` 1 and `__GETOPT_PREFIX` `rpl_`.  The four rows above are all *recorded* names:

- `_GL_SYSTEM_GETOPT` and `__need_system_stdlib_h` are transient preprocessor markers (`#define`
  then `#undef` around an `#include`), with no Rust counterpart; the second one's arm is dead on
  this target anyway.  Both are `excluded-macros.txt` rows (kind=config).
- `option` is the include-time rename (`# define option sys_option`) that keeps the system
  header's `struct option` from colliding for the duration of C 36's `#include_next`; the port has
  no include-time rename and keeps gnulib's single `struct option` in `getopt-ext.rs`.  Recorded
  as a `syntax` row.
- `_` is the split guard's placeholder spelling (`_@GUARD_PREFIX@_GETOPT_H`), silenced by the
  shared `_` row.

## Evidence

```
$ python3 tools/extract-abi.py --check emacs/lib/getopt.in.h femacs/lib/include/getopt.in.rs
OK emacs/lib/getopt.in.h -> femacs/lib/include/getopt.in.rs            (0 problem rows)
$ cd femacs && cargo run -q -p xtask -- build-tu lib/getopt1.c
built lib/getopt1.rs -> ../build/femacs/getopt1.o
```
