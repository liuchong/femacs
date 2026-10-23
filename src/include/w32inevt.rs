// femacs: port of src/w32inevt.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1).  The header is unconditional - C 20-21 is the include guard and
// the three declarations below are live text - but nothing on this target compiles it: the frozen
// configuration's platform object lists are empty, so `src/w32inevt.c`'s unit is an empty shape
// (docs/baseline/frozen-build-sources.txt, status 3.31) and only the Windows target links these
// names.  The mirror carries them anyway, because the header's *text* is what a mirror mirrors.
//
// The header includes nothing of its own; C's translation units reach `struct terminal`,
// `struct input_event`, `struct frame`, `enum scroll_bar_part`, `Lisp_Object` and `Time` through
// their own includes (PORTING.md 0.1-9), which is why the mirror names them without including
// their mirrors.

// C 20-21: the include guard has no Rust counterpart (PORTING.md 0.1-4).
pub const EMACS_W32INEVT_H: bool = true;

/* The two blocks below are the header's three declarations, and all three are `src/w32inevt.c`'s
   *definitions*.  They sit in a module this mirror glob-imports for the reason `include/w32term.rs`
   gives for its own: a crate-root definition shadows a glob import (PORTING.md 0.1-10 / P-21), so
   the unit's item stays the live one - at this mirror's crate root the three read
   `E0428 ... is defined multiple times` in the `windowsnt` arm (docs/status.md 3.575's class;
   corrected 2026-10-05, status 3.589).  `check-mirror-dupes.py` counts a mirror's declarations
   inside a module it glob-imports, so the count is unchanged.  */
pub mod w32inevt_decls {
  use super::*;

  /// C 23: the console reader's Unicode flag (`src/w32inevt.c` defines it).
  /// C: extern int w32_console_unicode_input;
  unsafe extern "C" { pub static mut w32_console_unicode_input: c_int; }

  /// C 25-34: the two entry points `src/w32inevt.c` defines.  `Time` is systime.h's, and
  /// `scroll_bar_part` is termhooks.h's (the header's callers include both).
  /// C: extern int w32_console_read_socket (struct terminal *, struct input_event *);
  /// C: extern void w32_console_mouse_position (struct frame **, int, Lisp_Object *,
  ///                                            enum scroll_bar_part *, Lisp_Object *,
  ///                                            Lisp_Object *, Time *);
  unsafe extern "C" {
    pub fn w32_console_read_socket (term: *mut c_void, hold_quit: *mut c_void) -> c_int;
    pub fn w32_console_mouse_position (f: *mut *mut c_void, insist: c_int,
                                       bar_window: *mut Lisp_Object, part: *mut c_int,
                                       x: *mut Lisp_Object, y: *mut Lisp_Object,
                                       time: *mut c_void);
  }
}
use w32inevt_decls::*;
