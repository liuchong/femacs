// femacs: port of src/w32select.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1).  The header has no platform guard of its own: C 20-21 is the
// include guard, C 22 is `#include <windows.h>` (whose names this header never mentions, so the
// mirror names none of them either - a unit that needs them on the Windows target takes them from
// that target's own headers), and C 24-26 are the three entry points the source file defines.
//
// `tools/check-cond-eval.py src/w32select.h` reports 1 condition, 0 unknown, 0 disagreements - and
// that one condition is the `#ifndef` guard itself.  The *unit* is nevertheless an empty shape
// (`femacs/src/w32select.rs`), because the frozen configuration does not compile `src/w32select.c`
// at all (docs/baseline/frozen-build-sources.txt: this target's platform object lists are empty);
// the declarations below are the header's live text, so the mirror carries them, and their
// definitions are deferred to the Windows target the way the unit's own note records.

// C 20-21: the include guard has no Rust counterpart (PORTING.md 0.1-4).
pub const W32SELECT_H: bool = true;

/// C 24-26: the three entry points `src/w32select.c` defines.  They are declarations only - the
/// port's frozen build has no such translation unit (the empty unit records the exclusion).
/// C: void syms_of_w32select (void);
/// C: void globals_of_w32select (void);
/// C: void term_w32select (void);
unsafe extern "C" {
  pub fn syms_of_w32select ();
  pub fn globals_of_w32select ();
  pub fn term_w32select ();
}
