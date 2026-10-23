# emacs/lib/getdelim.c (150 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 2
- [x] alloc_failed  (line 47, static)
- [x] getdelim  (line 63)

## Macros defined — 4
- [x] _GL_ARG_NONNULL  (line 22, body da39a3ee5e6b)
- [x] flockfile  (line 39, body a29646b15239)
- [x] funlockfile  (line 40, body a29646b15239)
- [x] getc_maybe_unlocked  (line 43, body d0f089ee976c)

## Types and their fields — 0

## File-scope objects — 0


## Decisions

`emacs/lib/getdelim.c` @ `fd7afced403`; port `lib/getdelim.rs` (175 lines).  All
checklist items ticked; `build-tu` → `built getdelim.rs -> build/femacs/getdelim.o`,
`--check` → `OK`.

- **Live arm.**  `USE_UNLOCKED_IO`'s `GNULIB_STDIO_SINGLE_THREAD` is undefined and the frozen
  configuration has neither `HAVE_FLOCKFILE` nor `HAVE_FUNLOCKFILE`, so C 34-42's `#elif` arm
  is the live text: `flockfile`/`funlockfile` are the no-op macros (their C body's `0` is the
  port's `let _ = ($x, 0);`) and `getc_maybe_unlocked (fp)` is `getc (fp)`.  The
  `USE_UNLOCKED_IO` arm and the `getc_unlocked` spelling are the non-frozen sides (P-24 does
  not compare the duplicated `getc_maybe_unlocked` body).
- **`alloc_failed`** is empty on this target; its only statement is the Windows arm's
  `errno = ENOMEM`, kept as `#[cfg (feature = "windows_native")]` text (P-51).
- **`goto unlock_return`** is the labelled block `'unlock_return` (PORTING.md 2.2);
  `macro-body-notes.tsv` records its `goto` and the `__GLIBC__` of the `fp == NULL` guard
  (C 69's condition only decides whether that comparison is compiled, and the frozen target
  is not glibc).
- **`ssize_t result;`** is zero-initialized (PORTING.md 1.5); `getc`/`EOF`/`SSIZE_MAX` are
  declared in the unit with their C prototypes because the stdio/limits mirrors do not carry
  them yet.
- Line drift: `line-drift-tolerance.tsv` records `getdelim 0.20` (14.7%).
- Not verified: gate-fast/gate-probes, ERT, and any behavioral test of the read loop.
