// femacs: port of src/commands.h @ fd7afced403.
//
// WIP mirror (P-66): commands.h is 30 lines and has no conditional at all, so this is the
// whole header.  `#include "lisp.h"` (commands.h 24) has no counterpart here: a `src/`
// mirror never includes lisp.h (PORTING.md 0.1-9), so the including unit includes
// `lisp.rs` first, in C's order.

// commands.h 20: the include guard has no Rust counterpart (PORTING.md 0.1).
// [U1-MODULE-MIRROR]
use super::*;
pub const EMACS_COMMANDS_H: bool = true;

// C: `#define Ctl(c) ((c)&037)`.  Rust has no octal literal, so the constant is `0o37`
// (31, the same value); the token check's exemption is recorded in
// docs/ports/macro-body-notes.tsv (the `Ctl` row).
macro_rules! Ctl { ($c:expr) => { ($c) & 0o37 }; }

/* Nonzero if input is coming from the keyboard.  */

// C: `#define INTERACTIVE (NILP (Vexecuting_kbd_macro) && !noninteractive)`.  The frozen
// C global `noninteractive` is `noninteractive1` in the generated globals
// (docs/baseline/globals/variables.tsv 711, P-17), which the `INTERACTIVE` row of
// docs/ports/macro-body-notes.tsv records.
macro_rules! INTERACTIVE {
    () => { NILP (unsafe { Vexecuting_kbd_macro }) && !unsafe { noninteractive1 } };
}

// EMACS_COMMANDS_H's `#endif` has no Rust counterpart (PORTING.md 0.1).
