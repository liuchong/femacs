# emacs/nt/inc/stdalign.h (20 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 5
- [x] _Alignas  (line 14, body 3ffe166228d5)
- [x] _Alignof  (line 9, body 1ee517052b09)
- [x] _NT_STDALIGN_H_  (line 2, body da39a3ee5e6b)
- [x] alignas  (line 17, body 3cb3417ca913)
- [x] alignof  (line 11, body bda6adb87ec6)

## Types and their fields — 0

## File-scope objects — 0
## Decisions

Mirror of nt/inc/stdalign.h, complete for this target: the include guard and `_Alignof`/`alignof`
as `macro_rules!` whose body is the Rust spelling (`core::mem::align_of::<T> ()`).

**Which arm**: `_Alignof` is the one name C defines twice (the `__cplusplus` template helper, then
C's `offsetof (struct { char __a; type __b; }, __b)`); C keeps the second, and the extractor leaves
the body uncompared because there are two definitions.  The file is not on
`docs/baseline/config/cond-eval-verified.txt`, so no `#if` in it is resolved and `__cplusplus`,
`__GNUC__` and `_Alignas` are all live text.

**Names with no item**: `_Alignas` and `alignas` (C 14 and 17) are recorded in
`docs/ports/excluded-macros.txt` - the `alignas` row has been there since an earlier round, and
this round appends `_Alignas` next to it.  Their Rust counterpart is the `#[repr(align(N))]`
attribute, which a `macro_rules!` cannot produce, so neither is declared here.  `offsetof` (from
C 4's `#include <stddef.h>`) has no mirror either: the port writes `core::mem::offset_of!`.

Nothing in the frozen configuration includes a `nt/inc/` header (no `nt/` or `msdos/` translation
unit is compiled - `docs/baseline/frozen-build-sources.txt`'s batch-F section records the
derivation - and no `src/` unit includes one), so `xtask layout-check`,
which compiles the `src/include` and `lib/include` mirrors, does not reach it; in a MinGW build
`nt/inc` is on the include path instead.
Evidence: `python3 tools/extract-abi.py --check emacs/nt/inc/stdalign.h
femacs/nt/inc/include/stdalign.rs` -> `OK` (with the expected notes that `alignas` already has an
exclusion row and `_Alignof` is defined twice).
