// femacs: port of src/commands.h @ fd7afced403 (GNU Emacs 32.0.50).
//
// WIP mirror (P-66): commands.h is 30 lines and has no conditional at all, so this is the
// whole header.  `#include "lisp.h"` (commands.h 24) has no counterpart here: a `src/`
// mirror never includes lisp.h (PORTING.md 0.1-9), so the including unit includes
// `lisp.rs` first, in C's order.

/* Definitions needed by most editing commands.
   Copyright (C) 1985, 1994, 2001-2026 Free Software Foundation, Inc.

This file is part of GNU Emacs.

GNU Emacs is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

GNU Emacs is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with GNU Emacs.  If not, see <https://www.gnu.org/licenses/>.  */

// commands.h 20: the include guard has no Rust counterpart (PORTING.md 0.1).
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
