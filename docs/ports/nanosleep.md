# emacs/lib/nanosleep.c (191 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 2
- [x] nanosleep  (line 96)
- [x] nanosleep  (line 186)

## Macros defined — 1
- [x] WIN32_LEAN_AND_MEAN  (line 86, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0


## Decisions

`emacs/lib/nanosleep.c` @ `fd7afced403`; port `femacs/lib/nanosleep.rs`.  All checklist items
ticked; `build-tu` → `built nanosleep.rs -> build/femacs/nanosleep.o`, `--check` → `OK`.

- **Live arm.**  The frozen configuration sets `HAVE_BUG_BIG_NANOSLEEP`, so C 39-80's arm is
  the live text (the 24-day-chunk loop); C 82-177's Windows arm and C 180-191's `pselect`
  arm are the non-frozen sides.
- **One function, not two**: C defines `nanosleep` in two arms, and the checklist names both,
  so both of the port's `#[cfg]` blocks sit inside the *same* `nanosleep` function (never a
  renamed twin, the malloc.c/cloexec.c shape).  The `pselect` arm is kept as real text; the
  Windows arm needs `<windows.h>`'s `LARGE_INTEGER`/`LONGLONG` and the `QueryPerformance*`/
  `Sleep` entries, so its tokens are recorded in `docs/ports/macro-body-notes.tsv` (the
  gettimeofday.c Windows-arm precedent) and its two statics are kept in the cfg'd block.
- `WIN32_LEAN_AND_MEAN` (a dead-arm macro) is in `docs/ports/excluded-macros.txt`;
  `static_assert`/`TYPE_MAXIMUM` are the port's `static_assert!` and a unit-local
  `TYPE_MAXIMUM!` (PORTING.md 2.5); `pselect`'s three `fd_set *` parameters are `*mut c_void`
  (only the dead arm passes `NULL`, the P-55 shape).
- Line drift: `line-drift-tolerance.tsv` records `nanosleep 0.35` (22%: the Windows arm's
  ~90 lines are not carried as code).
- Not verified: gate-fast/gate-probes, ERT, and any behavioral test of the sleep loop.
