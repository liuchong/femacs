# emacs/lib/minmax.h (65 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 3
- [x] MAX  (line 57, body 54a3705d26d6)
- [x] MIN  (line 62, body 7c03c63086ce)
- [x] _MINMAX_H  (line 19, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/lib/minmax.h` @ `fd7afced403`; port `lib/include/minmax.rs`.  `MAX`/`MIN` are value
macros whose arguments C evaluates twice; the port's `macro_rules!` keep that (the callers pass
plain variables).  The `#ifndef MAX`/`#ifndef MIN` guards are dead on this target and the
`HAVE_MINMAX_IN_LIMITS_H`/`HAVE_MINMAX_IN_SYS_PARAM_H` includes are only needed for building the
system definitions, which the port does not.

## Evidence

```
$ python3 tools/extract-abi.py --check emacs/lib/minmax.h lib/include/minmax.rs
OK emacs/lib/minmax.h -> lib/include/minmax.rs            (0 problem rows)
$ cd femacs && cargo run -q -p xtask -- build-tu lib/careadlinkat.c
built lib/careadlinkat.rs -> ../build/femacs/careadlinkat.o
```

`minmax.h` has no translation unit of its own; its consumers carry the compile evidence, and
`lib/careadlinkat.rs` `include!`s this mirror (as do `lib/getrandom.rs` and
`lib/nproc.rs`), so that unit's build is this mirror's build.
