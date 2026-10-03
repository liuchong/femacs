# emacs/lib/nproc.h (46 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

Mirror of lib/nproc.h, complete: `enum nproc_query` (the three members, no explicit values in
C and none in the port) and the `num_processors` prototype in a glob-imported module
(`pub mod nproc_decls` + `use`, PORTING.md 0.1-10, the include/fsusage.rs shape).  The header
has no include guard (it opens with the license comment and the `#ifdef __cplusplus` pair,
which PORTING.md 0.1 drops), so there is no guard marker constant; `unsigned long int` is the
port's `c_ulong`.  `python3 tools/extract-abi.py --check emacs/lib/nproc.h
femacs/lib/include/nproc.rs` -> `OK`; the enum is really gated (deleting
`NPROC_CURRENT_OVERRIDABLE` -> `missing constant`, setting `NPROC_ALL = 5` -> `constant
NPROC_ALL: C 0 vs Rust enum 5`).
