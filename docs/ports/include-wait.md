# emacs/nt/inc/sys/wait.h (33 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 6
- [x] INC_SYS_WAIT_H_  (line 21, body da39a3ee5e6b)
- [x] WCONTINUED  (line 27, body fe5dbbcea5ce)
- [x] WEXITED  (line 26, body 1b6453892473)
- [x] WNOHANG  (line 23, body 356a192b7913)
- [x] WSTOPPED  (line 25, body da4b9237bacc)
- [x] WUNTRACED  (line 24, body da4b9237bacc)

## Types and their fields — 0

## File-scope objects — 0
## Decisions

Mirror of nt/inc/sys/wait.h, complete for this target: the include guard, the five `W*` constants (`WSTOPPED` is `WUNTRACED`, as C's own comment says) and the `waitpid` prototype; the `WIF*` macros C's comment points at live in the `src/syswait.h` mirror.

Nothing in the frozen configuration includes a `nt/inc/` header (no `nt/` or `msdos/` translation
unit is compiled - `docs/baseline/frozen-build-sources.txt`'s batch-F section records the
derivation - and no `src/` unit includes one), so `xtask layout-check`,
which compiles the `src/include` and `lib/include` mirrors, does not reach it - `spikes/f_inc_probe.rs`
compiles it instead, and that probe found three real defects that every text gate had passed
(docs/status.md 3.102).  Its values are C's own and need
no clang measurement: they are literals and typedefs from the Windows SDK's own spellings.
Evidence: `python3 tools/extract-abi.py --check emacs/nt/inc/sys/wait.h
nt/inc/sys/include/wait.rs` -> `OK`.
  `../spikes/f_inc_probe.rs` compiles this mirror: it is the only compiler it has.
