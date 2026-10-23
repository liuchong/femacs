# emacs/nt/inc/sys/time.h (21 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 3
- [x] ITIMER_PROF  (line 7, body 356a192b7913)
- [x] ITIMER_REAL  (line 6, body b6589fc6ab0d)
- [x] SYS_TIME_H_INCLUDED  (line 2, body da39a3ee5e6b)

## Types and their fields — 1
- [x] itimerval: struct  timeval it_interval, struct  timeval it_value

## File-scope objects — 0
## Decisions

Mirror of nt/inc/sys/time.h, complete for this target: the guard, `ITIMER_REAL`/`ITIMER_PROF`, the `struct itimerval` (two `struct timeval`s) and the two MSVC prototypes.  C's `#include_next <sys/time.h>` reaches **MinGW's** header, not the frozen target's: the two disagree (`ITIMER_VIRTUAL` 1 / `ITIMER_PROF` 2 there, `ITIMER_PROF` 1 here, and both define `itimerval` and `setitimer`), so including `lib/include/sys_time.rs` gave `E0428` four times; the mirror takes `struct timeval` from the including unit instead, exactly as C takes it from the system header.

Nothing in the frozen configuration includes a `nt/inc/` header (no `nt/` or `msdos/` translation
unit is compiled - `docs/baseline/frozen-build-sources.txt`'s batch-F section records the
derivation - and no `src/` unit includes one), so `xtask layout-check`,
which compiles the `src/include` and `lib/include` mirrors, does not reach it - `spikes/f_inc_probe.rs`
compiles it instead, and that probe found three real defects that every text gate had passed
(docs/status.md 3.102).  Its values are C's own and need
no clang measurement: they are literals and typedefs from the Windows SDK's own spellings.

Evidence: `python3 tools/extract-abi.py --check emacs/nt/inc/sys/time.h
nt/inc/sys/include/time.rs` -> `OK` (4 ledger item(s)).
  `../spikes/f_inc_probe.rs` compiles this mirror: it is the only compiler it has.
