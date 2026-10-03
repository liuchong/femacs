# emacs/nt/inc/sys/file.h (12 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 5
- [x] D_OK  (line 10, body fe5dbbcea5ce)
- [x] F_OK  (line 6, body b6589fc6ab0d)
- [x] R_OK  (line 9, body 1b6453892473)
- [x] W_OK  (line 8, body da4b9237bacc)
- [x] X_OK  (line 7, body 356a192b7913)

## Types and their fields — 0

## File-scope objects — 0
## Decisions

Mirror of nt/inc/sys/file.h, complete for this target: the five `*_OK` constants (4/2/1/8 with `F_OK` 0) - the whole file, since `sys/file.h` does not exist on NT.

Nothing in the frozen configuration includes a `nt/inc/` header (no `nt/` or `msdos/` translation
unit is compiled - `docs/baseline/frozen-build-sources.txt`'s batch-F section records the
derivation - and no `src/` unit includes one), so `xtask layout-check`,
which compiles the `src/include` and `lib/include` mirrors, does not reach it - `spikes/f_inc_probe.rs`
compiles it instead, and that probe found three real defects that every text gate had passed
(docs/status.md 3.102).  Its values are C's own and need
no clang measurement: they are literals and typedefs from the Windows SDK's own spellings.

Evidence: `python3 tools/extract-abi.py --check emacs/nt/inc/sys/file.h
femacs/nt/inc/sys/include/file.rs` -> `OK` (5 ledger item(s)).
  `../spikes/f_inc_probe.rs` compiles this mirror: it is the only compiler it has.
