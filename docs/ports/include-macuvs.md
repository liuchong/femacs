# emacs/src/macuvs.h (9218 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

Mirror of src/macuvs.h, complete: the header declares **no** subr, function, macro, type or
file-scope object - it is one data table.  C 9's `static const unsigned char
mac_uvs_table_adobe_japan1_bytes[] = { … };` (C 10-9218, 73,655 values, 8 per line) is mirrored
as `pub const mac_uvs_table_adobe_japan1_bytes: [u8; 73655]` in `femacs/src/include/macuvs.rs`.
Three points are decisions rather than transcription:

1. **`const`, not `pub static`.** C's `static` gives the array internal linkage: every
   including translation unit gets its own copy and the object emits no symbol.  A `pub static`
   definition in a mirror would emit one symbol per translation unit and break the link
   (PORTING.md 0.1-2 forbids definitions in mirrors); a `const` is the same per-unit copy and the
   same no-symbol property.  The element count C leaves implicit is what the initializer makes,
   so `[u8; 73655]` and C's `sizeof` agree.
2. **The value lines are C's own text.** Each line of the table is copied verbatim (same order,
   same 8-per-line grouping, same two-digit lowercase hex), so line *n* of the mirror's table is
   line *n* of C's - the 20 lines the mirror adds on top are the provenance note, C's own head
   comment (C 1-7, verbatim) and the `/// C 9:` quotation.
3. **Differential proof, since the gate has nothing to compare.** `tools/extract-abi.py --check
   emacs/src/macuvs.h femacs/src/include/macuvs.rs` -> `OK`, but the extractor counts zero items
   in a data-only header, so that `OK` alone would not prove the bytes.  The bytes were therefore
   compared the way C and Rust see them: a `/tmp` C probe that `#include "macuvs.h"` and
   `fwrite`s `sizeof mac_uvs_table_adobe_japan1_bytes` bytes, and a `/tmp` Rust probe that
   `include!`s the mirror and writes the array; both emit **73,655** bytes and
   `cmp`/`sha256` agree (`24a6073bcf4081d0cd025ab072ef61ee3288f93ae0042bfc710f5cf871a57d7e`).

Consumer: **none in the frozen configuration.**  `grep -rn macuvs emacs/src/*.c emacs/src/*.h`
finds no includer - the header is read by `src/macfont.m` (Objective-C, batch F: that file
becomes Swift), which is not in the frozen build's source list either.  So the mirror is
currently compiled only by the layout probe (`xtask layout-check` includes every
`src/include/*.rs` into one crate; it passes with this file present).  When the macOS font stage
lands, the open question is how the *Swift* side reaches these bytes (a Rust `#[unsafe(no_mangle)]`
definition emitted by whatever unit owns the table, or a generated Swift copy) - recorded here
rather than invented now, per PORTING.md 0.1-19's "no invented behaviour".
