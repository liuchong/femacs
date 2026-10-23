# emacs/lib/fingerprint.c  →  lib/fingerprint.rs (66 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- The file is on the verified list and the frozen table decides every condition in it; nothing
  is demanded of the port because the reference compiles no definition from it.
- the file is the loader for Emacs's build fingerprint; the frozen configuration keeps only its two includes, so no function, macro, type or object survives (the symbol itself is emitted by lib-src/make-fingerprint.c, which is a separate unit).
- Policy: docs/status.md 3.25 (a file the frozen configuration strips to its includes is
  replicated as an empty unit); the tolerance row records the line relation.

- **PORTING.md P-81**: this is one of gnulib's extern-inline carriers - its C text is
  the `#define <qualifier> _GL_EXTERN_INLINE` line and its includes, with the bodies
  in the header - so the reference object exports the *header's* functions and the unit
  re-exports them under C's own names (`#[unsafe(export_name = "…")]` inside
  `mod <stem>_unit`, each calling `super::…` so the module's own item cannot shadow the
  mirror's and recurse).  tools/check-symbols.py compares the result with the reference
  object symbol by symbol; this entry was on docs/baseline/symbol-pending.txt until it
  matched.
  C 33-66 is `volatile unsigned char fingerprint[] = { … };` - 32 bytes the reference
  object exports as `D _fingerprint` - so the port defines exactly that array under
  C's name; `volatile` qualifies C's accesses (a Rust reader spells `read_volatile` at
  the access).  C 20's `#include "fingerprint.h"` has no mirror: that header only
  declares the object this unit defines, so a mirror would collide with it (E0428) and
  no ported unit includes it yet (the region-cache.h reading, PORTING.md 0.1-12).

  The extractor's object scan reads single-line declarations, so it reports 0 file-scope
  objects for this C file even though the reference object has one (the multi-line
  initializer); tools/check-symbols.py is the gate that sees it, and it is what closed
  this unit's docs/baseline/symbol-pending.txt entry.
