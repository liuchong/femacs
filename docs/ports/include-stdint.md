# emacs/nt/inc/stdint.h (72 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 21
- [x] INT32_MAX  (line 53, body 75878664309b)
- [x] INT32_MIN  (line 54, body e5423fadb281)
- [x] INT64_MAX  (line 37, body 4328b4022f7d)
- [x] INT64_MIN  (line 38, body b3fdeb4cc5bf)
- [x] INTMAX_MAX  (line 60, body 94b7ff4d363a)
- [x] INTMAX_MIN  (line 61, body 58bb802332f9)
- [x] INTPTR_MAX  (line 55, body 94b7ff4d363a)
- [x] INTPTR_MIN  (line 56, body 58bb802332f9)
- [x] PTRDIFF_MAX  (line 66, body a8b82d9c6934)
- [x] PTRDIFF_MIN  (line 67, body ea1e73a8ba13)
- [x] SIZE_MAX  (line 68, body ce620a4e99af)
- [x] UINT32_MAX  (line 51, body 1d193a8b2ac3)
- [x] UINT32_MIN  (line 52, body b6589fc6ab0d)
- [x] UINT64_MAX  (line 34, body 9175872660d3)
- [x] UINT64_MIN  (line 35, body b6589fc6ab0d)
- [x] UINTMAX_MAX  (line 58, body 21b1af79b013)
- [x] UINTMAX_MIN  (line 59, body 2d1d15cb7351)
- [x] UINTPTR_MAX  (line 57, body 21b1af79b013)
- [x] _NT_STDINT_H_  (line 21, body da39a3ee5e6b)
- [x] intmax_t  (line 63, body bd3027fa569e)
- [x] uintmax_t  (line 62, body 48fd659982bc)

## Types and their fields — 0

## File-scope objects — 0
## Decisions

Mirror of nt/inc/stdint.h, complete for this target: the include guard, the MSVC arm's typedefs
(`intptr_t`, `uint32_t`, `uint64_t`), the two type renames (`uintmax_t`, `intmax_t`) as `pub type`
aliases, and all 18 limit constants.

**Which arm**: the file is not on `docs/baseline/config/cond-eval-verified.txt`, so the extractor
resolves no `#if` in it: `__GNUC__` (the `#include_next <stdint.h>` a MinGW build takes, since
that compiler *is* GCC), `!__GNUC__`, `_WIN64` and the 32-bit arm are all live text.  Nine names
exist in both of the MSVC arms (`intptr_t`, `uint32_t`, `uintmax_t`, `intmax_t`, `INTPTR_MAX`,
`INTPTR_MIN`, `UINTPTR_MAX`, `UINTMAX_MAX`, `UINTMAX_MIN`, `INTMAX_MAX`, `INTMAX_MIN`); where the
two arms differ, the mirror spells the 32-bit arm - the one the extractor records (`intmax_t` at
line 63, `uintmax_t` at line 62) - and names the `_WIN64` spelling in a `///` note.  The four
names only the `_WIN64` arm declares (`uint64_t`, `UINT64_MAX`, `UINT64_MIN`, `INT64_MAX`,
`INT64_MIN`) are marked `#[cfg (feature = "win64")]`, the way `msdos/inttypes.rs` marks its dead
arm.

**Names with no item**: `uintmax_t` and `intmax_t` are `#define`s that rename a type, so they also
have rows this round appends to `docs/ports/excluded-macros.txt` (the `file_offset` shape: the port
spells them `pub type uintmax_t = c_ulong;` / `pub type intmax_t = c_long;`, which the gate's item
scan - macros, constants, functions - does not read).  `ptrdiff_t` and `size_t` are not declared
here; PORTING.md 1.1 maps them to `isize`/`usize`, which is what `PTRDIFF_MAX`/`PTRDIFF_MIN`/
`SIZE_MAX` are typed with.

Nothing in the frozen configuration includes a `nt/inc/` header (no `nt/` or `msdos/` translation
unit is compiled - `docs/baseline/frozen-build-sources.txt`'s batch-F section records the
derivation - and no `src/` unit includes one), so `xtask layout-check`,
which compiles the `src/include` and `lib/include` mirrors, does not reach it; in a MinGW build
`nt/inc` is on the include path instead.
Evidence: `python3 tools/extract-abi.py --check emacs/nt/inc/stdint.h
nt/inc/include/stdint.rs` -> `OK` (with the expected notes that the two type renames have
recorded counterparts and seven names are defined in more than one arm).
