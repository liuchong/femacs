# emacs/nt/inc/sys/resource.h (51 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 5
- [x] INC_SYS_RESOURCE_H_  (line 21, body da39a3ee5e6b)
- [x] RLIMIT_INFINITY  (line 41, body aee03bd4c206)
- [x] RLIMIT_NLIMITS  (line 32, body 927dbc59f7d4)
- [x] RLIMIT_NOFILE  (line 29, body 135abdc7b303)
- [x] RLIMIT_STACK  (line 26, body 713ccc73d0ea)

## Types and their fields — 1
- [x] rlimit: rlim_t rlim_cur, rlim_t rlim_max

## File-scope objects — 0
## Decisions

Mirror of nt/inc/sys/resource.h, complete for this target: the include guard, the three-member `enum rlimit_resource` (with the self-referential `#define RLIMIT_STACK RLIMIT_STACK` spellings C adds so the names work inside `#if` - recorded per name in `docs/ports/macro-body-notes.tsv`), `rlimit_resource_t`, `rlim_t`, `RLIMIT_INFINITY`, `struct rlimit` and the two prototypes.

Nothing in the frozen configuration includes a `nt/inc/` header (no `nt/` or `msdos/` translation
unit is compiled - `docs/baseline/frozen-build-sources.txt`'s batch-F section records the
derivation - and no `src/` unit includes one), so `xtask layout-check`,
which compiles the `src/include` and `lib/include` mirrors, does not reach it - `spikes/f_inc_probe.rs`
compiles it instead, and that probe found three real defects that every text gate had passed
(docs/status.md 3.102).  Its values are C's own and need
no clang measurement: they are literals and typedefs from the Windows SDK's own spellings.
Evidence: `python3 tools/extract-abi.py --check emacs/nt/inc/sys/resource.h
femacs/nt/inc/sys/include/resource.rs` -> `OK`.
  `../spikes/f_inc_probe.rs` compiles this mirror: it is the only compiler it has.

## 2026-10-05: `getrlimit`/`setrlimit` move into `resource_decls` and the mirror names its own types (3.513)

C 50-51's two entry points are defined by `src/w32heap.c` (C 483-554), so their top-level copies
were `E0428` there, and the file's `rlim_t = c_longlong` named a type nothing imported (`E0412`/
`E0425` in the same flip).  Both fixes land together: the two prototypes sit in a
`resource_decls` module the mirror glob-imports (P-39), and the file's `use core::ffi::*;` supplies
C's own scalar names (P-70's rule for a mirror whose header the port does not carry).
