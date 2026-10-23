# emacs/nt/inc/mingw_time.h (32 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 6
- [x] _MINGW_TIME_H  (line 4, body da39a3ee5e6b)
- [x] _USE_32BIT_TIME_T  (line 29, body da39a3ee5e6b)
- [x] __MINGW_MAJOR_VERSION  (line 21, body 77de68daecd8)
- [x] __MINGW_MINOR_VERSION  (line 23, body 0716d9708d32)
- [x] __MINGW_PATCHLEVEL  (line 25, body b6589fc6ab0d)
- [x] __MINGW_VERSION  (line 19, body b759f390eb3f)

## Types and their fields — 0

## File-scope objects — 0
## Decisions

Mirror of nt/inc/mingw_time.h, complete for this target: the include guard, the four version
constants C re-derives (`__MINGW_VERSION`, `__MINGW_MAJOR_VERSION`, `__MINGW_MINOR_VERSION`,
`__MINGW_PATCHLEVEL`) and `_USE_32BIT_TIME_T`.

**Which arm**: the file is not on `docs/baseline/config/cond-eval-verified.txt`, so the extractor
resolves no `#if` in it and all of `__MINGW32__` / `__MINGW64_VERSION_MAJOR` / `__MINGW_VERSION`
/ `__MINGW_MAJOR_VERSION >= 4` is live text.  The mirror carries the names once each, which is
what the file is for: C's own `#undef`/`#define` pairs exist so that the rest of the tree sees a
single trusted set.

**Names with no mirror**: `_mingw.h` (the MinGW runtime's own header) is named in the head note
rather than pulled in; the port has no `nt/`-side mirror for it.

Nothing in the frozen configuration includes a `nt/inc/` header (no `nt/` or `msdos/` translation
unit is compiled - `docs/baseline/frozen-build-sources.txt`'s batch-F section records the
derivation - and no `src/` unit includes one), so `xtask layout-check`,
which compiles the `src/include` and `lib/include` mirrors, does not reach it; in a MinGW build
`nt/inc` is on the include path instead.
Evidence: `python3 tools/extract-abi.py --check emacs/nt/inc/mingw_time.h
nt/inc/include/mingw_time.rs` -> `OK`.
