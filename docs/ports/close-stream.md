# emacs/lib/close-stream.c (77 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] close_stream  (line 55)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- Blind file.  `USE_UNLOCKED_IO` expands to `GNULIB_STDIO_SINGLE_THREAD` = 1 (macros.tsv
  1160), so C 26-28's `#include "unlocked-io.h"` is live; that header's renames are why the
  reference object calls no `_ferror` (`nm -u build/emacs-ref/lib/close-stream.o` lists
  `___error`, `___fpending`, `_fclose`), because `ferror` is `<stdio.h>`'s `__sferror` macro
  on this target.  The port keeps C's text `ferror (stream)` and calls the libSystem function
  of that name — its `_ferror` symbol is linkable (measured) and returns the same flag for the
  same stream.  This is the one behavioural-fidelity note of this file: the C compiler inlines
  a bit test, the port makes a call.
- `close_stream` is a non-`static` definition → `#[unsafe(no_mangle)]` (P-52); the reference
  object prints `T _close_stream` and so does the port's.
- C's four `const bool` locals are the port's `let … : bool`; `errno` is the errno mirror's
  accessor (`*errno ()`), and `EOF` comes from the stdio mirror.
- **Mirror edits**: `lib/include/close-stream.rs` (the one prototype, in a `pub mod` so this
  unit's definition shadows it) and `lib/include/fpending.rs`/`stream-impl`/`stdio.rs` as in
  docs/ports/fpending.md.
- Line-count drift (C 77 vs the port's 84) is declared in docs/ports/line-drift-tolerance.tsv.
- Call-relationship differences (D7): none.
