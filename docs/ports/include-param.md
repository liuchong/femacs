# emacs/nt/inc/sys/param.h (11 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 2
- [x] NBPG  (line 8, body 6124cb94c33d)
- [x] _PARAM_H_  (line 2, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0
## Decisions

Mirror of nt/inc/sys/param.h, complete for this target: the guard and `NBPG` 4096 (the only macro: NT has no `sys/param.h`, so the C tree supplies one).

Nothing in the frozen configuration includes a `nt/inc/` header (no `nt/` or `msdos/` translation
unit is compiled - `docs/baseline/frozen-build-sources.txt`'s batch-F section records the
derivation - and no `src/` unit includes one), so `xtask layout-check`,
which compiles the `src/include` and `lib/include` mirrors, does not reach it - `spikes/f_inc_probe.rs`
compiles it instead, and that probe found three real defects that every text gate had passed
(docs/status.md 3.102).  Its values are C's own and need
no clang measurement: they are literals and typedefs from the Windows SDK's own spellings.

Evidence: `python3 tools/extract-abi.py --check emacs/nt/inc/sys/param.h
nt/inc/sys/include/param.rs` -> `OK` (2 ledger item(s)).
  `../spikes/f_inc_probe.rs` compiles this mirror: it is the only compiler it has.
