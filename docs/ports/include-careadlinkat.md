# emacs/lib/careadlinkat.h (82 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 2
- [x] AT_FDCWD  (line 73, body c999ee6dae0f)
- [x] _GL_CAREADLINKAT_H  (line 21, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/lib/careadlinkat.h` @ `fd7afced403`; port `lib/include/careadlinkat.rs`.  The header
is `struct allocator`'s forward declaration (the allocator mirror carries the type), one
prototype - in a whole-import module because `lib/careadlinkat.rs` defines it - and C
72-80's `AT_FDCWD` arm, which is live only when HAVE_READLINKAT is undefined: the dead arm's
`-3041965` is kept as the cfg'd constant C would have defined (P-51; the header is not on the
verified list, so the checker still wants the token).

## Evidence

```
$ python3 tools/extract-abi.py --check emacs/lib/careadlinkat.h lib/include/careadlinkat.rs
OK emacs/lib/careadlinkat.h -> lib/include/careadlinkat.rs            (0 problem rows)
$ cd femacs && cargo run -q -p xtask -- build-tu lib/careadlinkat.c
built lib/careadlinkat.rs -> ../build/femacs/careadlinkat.o
```
