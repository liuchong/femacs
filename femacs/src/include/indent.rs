// femacs: port of src/indent.h @ fd7afced403.
//
// This mirror lands completed with `src/indent.c`'s port (the charset.h/intervals.h
// precedent): the record `struct position`, its five prototypes and the one `extern`
// object.  This unit defines all five prototypes, so they moved into `indent_decls` +
// glob import (PORTING.md 0.1-10) and the object into `indent_objs` (P-21); the unit's own
// definitions shadow both globs.
//
// `indent.h` includes only `lisp.h` (C 22), so the prototypes that name `struct window`
// (C 34-39) and `struct buffer` (C 57) spell them the ABI-equal `*mut c_void` with a
// `/// C:` line above (PORTING.md 0.1-2/P-72 rule 3, the buffer.rs `overlay_strings`
// shape); the defining unit spells the real type.  `lisp.h` itself forward-declares
// `struct window` (lisp.h 3927) but no mirror reaches it from this header.
//
// `struct position`'s size and offsets are measured with clang against the frozen target
// and pinned by the `const _` assertions below (`tools/gen-layout-fields.py`'s STRUCTS list
// does not name it yet; docs/ports/indent.md records the numbers).

// C 19-20's include guard has no Rust counterpart (PORTING.md 0.1).
pub const EMACS_INDENT_H: bool = true;

// C 22: `#include "lisp.h"` is the including unit's `include!("include/lisp.rs")`.

// indent.h 24-32.  Measured: 48 bytes, alignment 8.  `position` is a plain record, so the
// port derives `Copy` (PORTING.md 0.1-9: a C struct is trivially copyable, and both
// `compute_motion`'s and `vmotion`'s callers copy the struct out of the returned pointer).
#[repr(C)]
#[derive (Clone, Copy)]
pub struct position
{
  pub bufpos: ptrdiff_t,                // indent.h 26
  pub bytepos: ptrdiff_t,               // 27
  pub hpos: EMACS_INT,        // 28
  pub vpos: EMACS_INT,        // 29
  pub prevhpos: EMACS_INT,    // 30
  pub contin: c_int,                    // 31
}

const _: () = core::assert! (std::mem::size_of::<position> () == 48);
const _: () = core::assert! (std::mem::align_of::<position> () == 8);
const _: () = core::assert! (std::mem::offset_of! (position, bufpos) == 0);
const _: () = core::assert! (std::mem::offset_of! (position, bytepos) == 8);
const _: () = core::assert! (std::mem::offset_of! (position, hpos) == 16);
const _: () = core::assert! (std::mem::offset_of! (position, vpos) == 24);
const _: () = core::assert! (std::mem::offset_of! (position, prevhpos) == 32);
const _: () = core::assert! (std::mem::offset_of! (position, contin) == 40);

/// The prototypes `src/indent.c` defines (PORTING.md 0.1-10): a glob import, so the unit's
/// own `#[unsafe(no_mangle)] pub unsafe extern "C" fn` definitions are not a second
/// declaration of the same name at the crate root (E0428).
pub mod indent_decls {
  use super::*;

  unsafe extern "C" {
    /// C: indent.h 34-38: `struct position *compute_motion (ptrdiff_t from, ptrdiff_t
    /// frombyte, EMACS_INT fromvpos, EMACS_INT fromhpos, bool did_motion, ptrdiff_t to,
    /// EMACS_INT tovpos, EMACS_INT tohpos, EMACS_INT width, ptrdiff_t hscroll, int
    /// tab_offset, struct window *);` — `indent.h` includes only `lisp.h`, so the window
    /// is the ABI-equal `*mut c_void` (PORTING.md 0.1-2/P-72 rule 3).
    pub fn compute_motion (from: ptrdiff_t, frombyte: ptrdiff_t, fromvpos: EMACS_INT,
                           fromhpos: EMACS_INT, did_motion: bool, to: ptrdiff_t,
                           tovpos: EMACS_INT, tohpos: EMACS_INT, width: EMACS_INT,
                           hscroll: ptrdiff_t, tab_offset: c_int, win: *mut c_void)
                           -> *mut position;
    /// C: indent.h 39-40: `struct position *vmotion (ptrdiff_t from, ptrdiff_t from_byte,
    /// EMACS_INT vtarget, struct window *);` — same `*mut c_void` rule as above.
    pub fn vmotion (from: ptrdiff_t, from_byte: ptrdiff_t, vtarget: EMACS_INT,
                    w: *mut c_void) -> *mut position;
    /// C: indent.h 41-42: `ptrdiff_t skip_invisible (ptrdiff_t pos, ptrdiff_t
    /// *next_boundary_p, ptrdiff_t to, Lisp_Object window);`
    pub fn skip_invisible (pos: ptrdiff_t, next_boundary_p: *mut ptrdiff_t, to: ptrdiff_t,
                           window: Lisp_Object) -> ptrdiff_t;
    /// C: indent.h 50-51: `bool disptab_matches_widthtab (struct Lisp_Char_Table *disptab,
    /// struct Lisp_Vector *widthtab);`
    pub fn disptab_matches_widthtab (disptab: *mut Lisp_Char_Table,
                                     widthtab: *mut Lisp_Vector) -> bool;
    /// C: indent.h 54-55: `void recompute_width_table (struct buffer *buf, struct
    /// Lisp_Char_Table *disptab);` — `struct buffer` is not reachable from this header
    /// (indent.h includes only lisp.h), so it is the ABI-equal `*mut c_void`.
    pub fn recompute_width_table (buf: *mut c_void,
                                  disptab: *mut Lisp_Char_Table);
  }
}
use indent_decls::*;

/// The file-scope object the header declares (`extern`).  `src/indent.c` defines it, so it
/// lives here + glob import (PORTING.md P-21/0.1-10).
pub mod indent_objs {
  use super::*;

  unsafe extern "C" {
    /// C: indent.h 46: `extern ptrdiff_t last_known_column_point;`
    pub static mut last_known_column_point: ptrdiff_t;
  }
}
use indent_objs::*;
