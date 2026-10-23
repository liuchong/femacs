# emacs/nt/inc/dirent.h (59 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 3
- [x] DIRBLKSIZ  (line 9, body ce09b127d48f)
- [x] MAXNAMLEN  (line 13, body f1abd670358e)
- [x] rewinddir  (line 59, body bbbdd2ecc648)

## Types and their fields — 1
- [x] dirent: long d_ino, unsigned short d_reclen, unsigned short d_namlen, unsigned d_type, __int64 d_time_create, __int64 d_time_access, __int64 d_time_write, _fsize_t d_size, char d_name

## File-scope objects — 0
## Decisions

Mirror of nt/inc/dirent.h, complete for this target: the two block-size constants, `struct
dirent` (including the five MinGW 4.x fields the extractor reads as live text), the `DIR`
stream struct and the four directory calls, in C's order.

**Which arm**: this file is not on `docs/baseline/config/cond-eval-verified.txt`, so the
extractor resolves no `#if` in it - both the `WINDOWSNT` and the `__MINGW_MAJOR_VERSION >= 4`
arms are live text and the mirror carries both.  `MAXNAMLEN` is the one name C defines twice
(255 under `WINDOWSNT`, 15 otherwise); C keeps the last definition, the extractor records line
13, and the mirror spells 15 - a real NT build gets 255, because `nt/inc/ms-w32.h` defines
`WINDOWSNT` for it.

**Names with no mirror**: `_fsize_t` is the MinGW runtime's own typedef and stays as C spells it,
named from the including unit (the `ssize_t` shape of `nt/inc/sys/include/acl.rs`); the `__int64`
fields are spelled `c_longlong`, PORTING.md 1.1's mapping for MSVC's 64-bit builtin, and are the
same fields `nt/inc/sys/include/stat.rs` carries.  The frozen target's `<sys/types.h>` mirror
describes darwin's types, not MinGW's, so it is deliberately not pulled in here (the same choice
`nt/inc/sys/include/time.rs` records for `<sys/time.h>`).

Nothing in the frozen configuration includes a `nt/inc/` header (no `nt/` or `msdos/` translation
unit is compiled - `docs/baseline/frozen-build-sources.txt`'s batch-F section records the
derivation - and no `src/` unit includes one), so `xtask layout-check`,
which compiles the `src/include` and `lib/include` mirrors, does not reach it; in a MinGW build
`nt/inc` is on the include path instead.
Evidence: `python3 tools/extract-abi.py --check emacs/nt/inc/dirent.h
nt/inc/include/dirent.rs` -> `OK` (with the expected note that C's one duplicated macro
has its body left uncompared).
