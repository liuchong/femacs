# emacs/lib-src/make-fingerprint.c (178 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] main  (line 66)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 1
- [x] static char buf  (line 63, static)

## Decisions

Port of lib-src/make-fingerprint.c, complete: one function (`main`, C 72-177) and one file-scope
object (`static char *buf`, C 69) are the whole inventory, and the ledger's two boxes are ticked.

**This is the first unit of the C tree's `lib-src/` directory** (batch F), and the first file that
`xtask` could not have built before: `build-tu` and the whole-tree walks hard-coded `["src",
"lib"]` until docs/status.md 3.94 taught them the eight ported directories.  The frozen
configuration does build this program (`(cd build/emacs-ref/lib-src && make -n) | grep -oE
'[a-zA-Z0-9_-]+\.c'` lists `make-fingerprint.c`, and `build/emacs-ref/lib-src/make-fingerprint`
is the built binary), so it is a live file, not a dead one.

Deviations, all recorded rather than hidden:

1. **Dead arms (P-08).**  C 45-47's `#ifndef SSIZE_MAX` fallback is dead (the frozen `<limits.h>`
   defines `SSIZE_MAX` = `isize::MAX`, measured), as is C 51-61's `WINDOWSNT` arm (the
   `fopen`->`sys_fopen` note, `<direct.h>`, the `fseeko`->`fseeko64` pair).
2. **Two tokens the port cannot name** - both in `main`, both with rows in
   `docs/ports/macro-body-notes.tsv`: C 113's `"r" FOPEN_BINARY` is a *concatenation*, and the
   sysstdio.h mirror carries `FOPEN_BINARY` as a `&CStr` constant (empty on this target), so the
   port writes the literals `c"r"` / `c"r+"`; C 118's `TYPE_MAXIMUM (off_t)` is `off_t::MAX`
   (`<intprops.h>` has no mirror, and the port names the type's own maximum - P-26).
3. **Use-site declarations (P-72 (3)).**  `SEEK_SET` and `fseeko` come from `<stdio.h>`, but the
   `stdio.rs` mirror cannot carry `fseeko` without putting `off_t` in every consumer's scope and
   `unistd.in.rs` already owns the `SEEK_*` constants, so this program declares the two it needs.
   `ptrdiff_t` is provided by the unit itself, the way `femacs/lib/regex.rs` carries
   `use std::ffi::c_int;` - C gets the name from `<stddef.h>` through `<stdio.h>`, and the core
   units get it from the `lisp.h` mirror, which this program does not include.
4. **`<unlocked-io.h>` (C 52) has no mirror**: the renames it performs on stdio calls are recorded
   at the use site by every unit that needs them (docs/ports/dispnew.md 374, docs/ports/bidi.md
   269), so this unit calls `fread`/`fwrite`/`fseeko` as C's text spells them.

Evidence: `cargo run -q -p xtask -- build-tu lib-src/make-fingerprint.c` ->
`built lib-src/make-fingerprint.rs -> ../build/femacs/make-fingerprint.o`;
`python3 tools/extract-abi.py --check emacs/lib-src/make-fingerprint.c
femacs/lib-src/make-fingerprint.rs` -> `OK`; `rustc --cfg enable_checking` on the materialized
unit -> exit 0; `check-globals` 273 units / 0 missing; `check-mirror-dupes` 0 failures;
`check-declarations` 0.  There is **no behavioural evidence** yet: the port has no way to run this
program (it needs the whole build chain), so the ledger claims text and object agreement only.
