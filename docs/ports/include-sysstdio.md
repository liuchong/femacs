# emacs/src/sysstdio.h (47 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 3
- [x] EMACS_SYSSTDIO_H  (line 21, body da39a3ee5e6b)
- [x] FOPEN_BINARY  (line 39, body dd29ecf524b0)
- [x] FOPEN_TEXT  (line 40, body dd29ecf524b0)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

Mirror of src/sysstdio.h, complete.  Live text on the frozen target: the include guard, three
extern declarations, and the `#else` arm of `#if O_BINARY` - `O_BINARY` is 0 in the frozen
config.h, so `FOPEN_BINARY` and `FOPEN_TEXT` are the empty string (the two bodies the extractor
hashes as `dd29ecf524b0`, i.e. the literal `""`).  `#ifndef PIPE_BUF` (C 43-45) is dead: the
frozen `<limits.h>` defines PIPE_BUF (512, macros.tsv).

1. **The three functions are declarations only.**  C defines `errputc`, `errwrite` and
   `close_output_streams` in src/sysdep.c, which is ported - `src/sysdep.rs` 4147, 4161,
   4168 define them - so this mirror declares and never defines (PORTING.md 0.1-3), and the
   module plus glob import is the P-21 shape (`sysstdio_decls`, the atimer_decls precedent).
2. **String-literal macros are constants here.**  The extractor compares a string-literal macro
   body as a *constant*, so `FOPEN_BINARY`/`FOPEN_TEXT` must be `const`s, not `macro_rules!`
   (PORTING.md 2.1's rule: `&CStr` unless a use site needs the token inside `concat!`).  They are
   spelled `&core::ffi::CStr`, the `character.rs`/`config.rs`/`buildobj.rs` idiom, because a
   mirror cannot import `CStr` without risking `E0252` against a unit's own prelude.  The one
   use site that does need the token form - `src/xfaces.rs` 572 - keeps its local
   `macro_rules! FOPEN_TEXT` and records itself there; converging it would mean writing `c"r"`
   and losing C's `FOPEN_TEXT` token, so it stays a use-site deviation.
3. **`<unlocked-io.h>` (C 29) has no mirror.**  It is batch L work; the renames it performs on
   stdio calls are recorded at the use site by the units that need them (docs/ports/dispnew.md
   374, docs/ports/bidi.md 269, docs/ports/getdelim.md 30), and this mirror does not invent them.
4. **Why the module needs `use super::*;`.**  `c_int`, `c_void` and `ptrdiff_t` come from the
   including unit (lisp.rs's crate-root `use std::ffi::{…}` and its `pub type ptrdiff_t`);
   inside `pub mod sysstdio_decls` those names are not in scope without the glob import, which
   the first draft of this file lacked - `xtask layout-check` caught it as `E0425: cannot find
   type c_int`.  Recorded in docs/status.md 3.91.

Evidence: `extract-abi.py --check emacs/src/sysstdio.h src/include/sysstdio.rs` -> `OK`;
`xtask layout-check` passes with this file present (it is the probe's first `src/include` mirror
that declares externs, so the probe compiles the module path too).

## 2026-10-04: C 29's `#include <unlocked-io.h>` is carried (3.251)

The mirror's header used to say the `unlocked-io.h` mirror did not exist yet and that units
recorded those renames at their own use sites.  Batch L landed `lib/include/unlocked-io.rs`, so
the include is now the mirror's own line (C 29) - which is what makes the `*_unlocked` names
visible to every unit that includes `sysstdio.h`, exactly as in C.  Evidence: `build-tu` of the
two units that call them (`src/sysdep.c`, `src/callproc.c`) both built; `check-mirror-dupes.py`
still 1 duplicate declaration / 1 ratchet row / 0 failures.
