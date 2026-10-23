// femacs: port of src/fontset.h @ fd7afced403.
//
// The fontset interface: the ten entry points the rest of the C tree calls to ask which face a
// character renders in (`face_for_char`, `face_for_font`, `font_for_char`), to build and name
// fontsets (`make_fontset_for_ascii_face`, `fontset_from_font`, `fontset_name`, `fontset_ascii`,
// `list_fontsets`, `fs_query_fontset`) and to release one (`free_face_fontset`).  C 31
// forward-declares `struct face`; `struct frame` and `Lisp_Object` come from `lisp.h` (C 29),
// which every translation unit reaches through its own includes (PORTING.md 0.1-9).
//
// `#if` inventory: the header has one condition, C 26-27's include guard
// (`python3 tools/check-cond-eval.py src/fontset.h` -> `1 conditions, 0 unknown, 0
// disagreements [VERIFIED]`, and the file is in docs/baseline/config/cond-eval-verified.txt).
// C 29-47 is live text - there is no `#ifdef` anywhere else in the file - so all ten prototypes
// are live text and are mirrored.
//
// Nothing on this target compiles it, but that is the *unit*'s fact, not the header's: the
// frozen configuration builds no unit that includes `fontset.h` (the `HAVE_WINDOW_SYSTEM` arms
// that reach it are dead, PORTING.md P-08, and `fontset.c` itself is not in
// `docs/baseline/frozen-build-sources.txt`, docs/status.md 3.31), so `src/fontset.rs` is
// the empty-shape note and this mirror's compiler is `xtask layout-check` plus the
// `spikes/sfnt_inc_probe.rs` probe that its sibling `src/sfnt.h` mirror brought - both orphan
// mirrors of the same cause (the `w32inevt.rs` precedent, status 3.159).
//
// Degradations (P-55/P-72): C names only three types here besides `Lisp_Object` - `struct frame`
// and `struct face` behind pointers, and `ptrdiff_t` by value.  Neither record is declared by
// this header (C 31's `struct face` is only a forward declaration, and `struct frame` is not
// mentioned outside the prototypes), and Rust's type identity is global: naming either would
// make every unit that takes this mirror without `frame.h`/`dispextern.h` fail (that is exactly
// what P-72 rule 3 records).  Both pointer parameters are therefore the untyped `*mut c_void`
// with the `/// C:` line P-55 requires, and the use site casts - the `struct buffer *`
// convention.  `ptrdiff_t` is `isize` (the token check folds the two spellings, and the alias
// is lisp.rs's either way).  The two uses of `font.c`'s port that cite
// `dispextern.h` 3811/`fontset.h` 88-89 today (`font.rs` 627-634, 713-724) keep their own
// copies until that unit includes this mirror; docs/ports/include-fontset.md records them for
// the integrator (the `struct font` use-site-copy precedent, status 3.158).

/* fontset.h 26-27: the include guard has no Rust counterpart (PORTING.md 0.1-4).  The
   extractor's item list for this header names it, so the mirror spells it the way
   `include/w32select.rs` spells its own guard.  */
pub const EMACS_FONTSET_H: bool = true;

// fontset.h 29: `#include "lisp.h"` - `Lisp_Object` is the enclosing unit's (PORTING.md 0.1-9:
// lisp.rs is the documented case, and `layout-check` forces it ahead of every other mirror), so
// this mirror carries no `include!` for it.
// fontset.h 31: `struct face;` - a forward declaration with no mirror in this tree (seven units
// carry their own `pub struct face`): see the file note above.

/* The ten entry points, in C's order.  They sit in a module the mirror glob-imports
   because `src/fontset.rs` **defines** nine of them (C 861-1234, 1266, 1798): a crate-root
   copy is `E0428` in that unit, which is why its C 37 `#include "fontset.h"` could not be
   spelled until this shape landed (P-39, the `font_log_decls` / `xterm_decls` shape;
   status 3.526).  */
pub mod fontset_decls {
  use super::*;

  unsafe extern "C" {
    /// C: void free_face_fontset (struct frame *, struct face *);
    pub fn free_face_fontset (arg0: *mut c_void, arg1: *mut c_void);
    /// C: int face_for_char (struct frame *, struct face *, int, ptrdiff_t, Lisp_Object);
    pub fn face_for_char (arg0: *mut c_void, arg1: *mut c_void, arg2: c_int, arg3: isize,
                          arg4: Lisp_Object) -> c_int;
    /// C: Lisp_Object font_for_char (struct face *, int, ptrdiff_t, Lisp_Object);
    pub fn font_for_char (arg0: *mut c_void, arg1: c_int, arg2: isize,
                          arg3: Lisp_Object) -> Lisp_Object;
    /// C: int make_fontset_for_ascii_face (struct frame *, int, struct face *);
    pub fn make_fontset_for_ascii_face (arg0: *mut c_void, arg1: c_int,
                                        arg2: *mut c_void) -> c_int;
    /// C: int fontset_from_font (Lisp_Object);
    pub fn fontset_from_font (arg0: Lisp_Object) -> c_int;
    /// C: int fs_query_fontset (Lisp_Object, int);
    pub fn fs_query_fontset (arg0: Lisp_Object, arg1: c_int) -> c_int;
    /// C: Lisp_Object list_fontsets (struct frame *, Lisp_Object, int);
    pub fn list_fontsets (arg0: *mut c_void, arg1: Lisp_Object, arg2: c_int) -> Lisp_Object;
    /// C: Lisp_Object fontset_name (int);
    pub fn fontset_name (arg0: c_int) -> Lisp_Object;
    /// C: Lisp_Object fontset_ascii (int);
    pub fn fontset_ascii (arg0: c_int) -> Lisp_Object;
    /// C: int face_for_font (struct frame *, Lisp_Object, struct face *);
    pub fn face_for_font (arg0: *mut c_void, arg1: Lisp_Object, arg2: *mut c_void) -> c_int;
  }
}
use fontset_decls::*;
