// femacs: port of src/haikuselect.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1): declarations only.  `haikuselect.h` is the Haiku window
// system's selection interface - the one type it defines (`enum haiku_clipboard`), the entry
// point `src/haikuselect.c` implements, the clipboard and message helpers `src/haiku_select.cc`
// implements, and the one notification call.  The frozen target (darwin-arm64, no window
// system) compiles no Haiku unit - `docs/baseline/frozen-build-sources.txt` lists no
// `src/haiku*.c` - so the *unit* is the recorded empty shape (`src/haikuselect.rs`);
// the header's own text is unconditional, so the mirror carries it (the `src/w32select.h`
// reading, status 3.158: the unit and the header have two different answers).
//
// Dead text under the frozen configuration (PORTING.md P-08): `__cplusplus` is undefined, so
// the dead arms are C 22-24 (the C++ half of the include pair - `<cstdio>`/`<cstdint>`; the
// live half is C 26-27's `<stdio.h>`/`<stdint.h>`), C 39-42 and C 82-84 (the `extern "C" { … }`
// braces, which are C++ linkage decoration with no Rust counterpart).  C 19-20 is the include
// guard; everything else is live text.  `tools/check-cond-eval.py src/haikuselect.h` ->
// `4 conditions, 0 unknown to the evaluator, 0 disagreements`.
//
// Names from the system (PORTING.md 0.1-9: the translation unit supplies them, and this
// header's own includes are system headers with no mirror): `int32`/`int64`/`uint32` are
// Haiku's `SupportDefs.h` spellings (C 30), written here as the `<stdint.h>` names of the live
// include at C 27 with the same widths (`int32_t`/`int64_t`/`uint32_t`, carried by
// `lib/include/stdint.in.rs`) - `haiku_support.rs` already declares the `int32` alias for that
// same toolchain header, and a second one here would be an `E0428` in the crate both mirrors
// land in (`spikes/haiku_inc_probe.rs`).  `ssize_t` comes from `lib/include/unistd.in.rs` (the
// `SupportDefs.h` chain reaches `<sys/types.h>`, the reading `include-haiku_support.md`
// records); `intmax_t` (C 78-79) is spelled `i64` - PORTING.md 1.1's mapping - because the
// port's alias lives in `lisp.rs`, which this header does not include.
//
// No parameter in this header is a pointer to a type the port cannot name: `void *` is C's own
// untyped pointer and stays `*mut c_void`, and every other pointer is to a scalar or to
// `char`.  The C text of each declaration is carried in its `/// C:` line, so nothing is
// degraded here (PORTING.md P-55/P-72 do not apply to this file).

include!("../../lib/include/stdint.in.rs");   // C 27: `#include <stdint.h>`
include!("../../lib/include/unistd.in.rs");    // `ssize_t` (C 30's `SupportDefs.h` chain)

// C 19-20: the include guard has no Rust counterpart (PORTING.md 0.1-4).
pub const _HAIKU_SELECT_H_: bool = true;

/* C 32-37: `enum haiku_clipboard`.  Declared as C's own type plus one constant per member -
   the shape PORTING.md 1.3 fixes for an enum whose members C compares as integers
   (`haiku_support.rs`'s `haiku_cursor`, `termhooks.rs`, `dispextern.rs`).  The members have no
   explicit initialisers, so C's rule gives 0, 1, 2.  */
pub type haiku_clipboard = c_int;
pub const CLIPBOARD_PRIMARY: haiku_clipboard = 0;
pub const CLIPBOARD_SECONDARY: haiku_clipboard = 1;
pub const CLIPBOARD_CLIPBOARD: haiku_clipboard = 2;

/* C 43-79: what `src/haikuselect.c` and `src/haiku_select.cc` define.  The block sits in a
   module with a glob import (PORTING.md 0.1-10/P-17/P-21: `haiku_support.rs`'s
   `haiku_support_decls` is the same shape), so the defining unit's own definitions shadow
   these declarations instead of colliding with them.  Unnamed C parameters become `arg0`,
   `arg1`, … (the `pdumper.rs`/`haiku_support.rs` convention).  */
pub mod haikuselect_decls {
  use super::*;

  unsafe extern "C" {
    /// C: extern void haiku_selection_disowned (enum haiku_clipboard, int64);
    pub fn haiku_selection_disowned (arg0: haiku_clipboard, arg1: int64_t);

    /// C: extern void be_clipboard_init (void);
    pub fn be_clipboard_init ();
    /// C: extern char *be_find_clipboard_data (enum haiku_clipboard, const char *,
    ///                                          ssize_t *);
    pub fn be_find_clipboard_data (arg0: haiku_clipboard, arg1: *const c_char,
                                   arg2: *mut ssize_t) -> *mut c_char;
    /// C: extern void be_set_clipboard_data (enum haiku_clipboard, const char *,
    ///                                       const char *, ssize_t, bool);
    pub fn be_set_clipboard_data (arg0: haiku_clipboard, arg1: *const c_char,
                                  arg2: *const c_char, arg3: ssize_t, arg4: bool);
    /// C: extern bool be_clipboard_owner_p (enum haiku_clipboard);
    pub fn be_clipboard_owner_p (arg0: haiku_clipboard) -> bool;
    /// C: extern void be_update_clipboard_count (enum haiku_clipboard);
    pub fn be_update_clipboard_count (arg0: haiku_clipboard);

    /// C: extern int be_enum_message (void *, int32 *, int32, int32 *, const char **);
    pub fn be_enum_message (arg0: *mut c_void, arg1: *mut int32_t, arg2: int32_t,
                            arg3: *mut int32_t, arg4: *mut *const c_char) -> c_int;
    /// C: extern int be_get_message_data (void *, const char *, int32, int32,
    ///                                    const void **, ssize_t *);
    pub fn be_get_message_data (arg0: *mut c_void, arg1: *const c_char, arg2: int32_t,
                                arg3: int32_t, arg4: *mut *const c_void,
                                arg5: *mut ssize_t) -> c_int;
    /// C: extern int be_get_refs_data (void *, const char *, int32, char **);
    pub fn be_get_refs_data (arg0: *mut c_void, arg1: *const c_char, arg2: int32_t,
                             arg3: *mut *mut c_char) -> c_int;
    /// C: extern int be_get_point_data (void *, const char *, int32, float *, float *);
    pub fn be_get_point_data (arg0: *mut c_void, arg1: *const c_char, arg2: int32_t,
                              arg3: *mut f32, arg4: *mut f32) -> c_int;
    /// C: extern uint32 be_get_message_type (void *);
    pub fn be_get_message_type (arg0: *mut c_void) -> uint32_t;
    /// C: extern void be_set_message_type (void *, uint32);
    pub fn be_set_message_type (arg0: *mut c_void, arg1: uint32_t);
    /// C: extern void *be_get_message_message (void *, const char *, int32);
    pub fn be_get_message_message (arg0: *mut c_void, arg1: *const c_char,
                                   arg2: int32_t) -> *mut c_void;
    /// C: extern void *be_create_simple_message (void);
    pub fn be_create_simple_message () -> *mut c_void;
    /// C: extern int be_add_message_data (void *, const char *, int32, const void *,
    ///                                    ssize_t);
    pub fn be_add_message_data (arg0: *mut c_void, arg1: *const c_char, arg2: int32_t,
                                arg3: *const c_void, arg4: ssize_t) -> c_int;
    /// C: extern int be_add_refs_data (void *, const char *, const char *);
    pub fn be_add_refs_data (arg0: *mut c_void, arg1: *const c_char,
                             arg2: *const c_char) -> c_int;
    /// C: extern int be_add_point_data (void *, const char *, float, float);
    pub fn be_add_point_data (arg0: *mut c_void, arg1: *const c_char, arg2: f32,
                              arg3: f32) -> c_int;
    /// C: extern int be_add_message_message (void *, const char *, void *);
    pub fn be_add_message_message (arg0: *mut c_void, arg1: *const c_char,
                                   arg2: *mut c_void) -> c_int;
    /// C: extern int be_lock_clipboard_message (enum haiku_clipboard, void **, bool);
    pub fn be_lock_clipboard_message (arg0: haiku_clipboard, arg1: *mut *mut c_void,
                                      arg2: bool) -> c_int;
    /// C: extern void be_unlock_clipboard (enum haiku_clipboard, bool);
    pub fn be_unlock_clipboard (arg0: haiku_clipboard, arg1: bool);
    /// C: extern void be_handle_clipboard_changed_message (void);
    pub fn be_handle_clipboard_changed_message ();
    /// C: extern void be_start_watching_selection (enum haiku_clipboard);
    pub fn be_start_watching_selection (arg0: haiku_clipboard);
    /// C: extern bool be_selection_outdated_p (enum haiku_clipboard, int64);
    pub fn be_selection_outdated_p (arg0: haiku_clipboard, arg1: int64_t) -> bool;
    /// C: extern int64 be_get_clipboard_count (enum haiku_clipboard);
    pub fn be_get_clipboard_count (arg0: haiku_clipboard) -> int64_t;

    /// C: extern intmax_t be_display_notification (const char *, const char *,
    ///                                             intmax_t, int, const char *);
    pub fn be_display_notification (arg0: *const c_char, arg1: *const c_char, arg2: i64,
                                    arg3: c_int, arg4: *const c_char) -> i64;
  }
}
use haikuselect_decls::*;

// C 86-88, the file's own local-variables block, carried verbatim (PORTING.md 0.3).
// Local Variables:
// eval: (setf (alist-get 'inextern-lang c-offsets-alist) 0)
// End:
