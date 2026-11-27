// femacs: port of src/keymap.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1): types, constants, macros and the declarations C spells
// `extern`.  Nothing is defined here, so any unit may include it.  It needs `lisp.h` only,
// which is where `CHARACTERBITS` (lisp.h 3233) lives; C's keymap.h includes lisp.h and
// nothing else, so every includer of this mirror includes `lisp.rs` too.

// The include guard has no Rust counterpart (PORTING.md 0.1).
// [U1-MODULE-MIRROR]
use super::*;
pub const KEYMAP_H: bool = true;

/* The maximum byte size consumed by push_key_description.
   All callers should assure that at least this size of memory is
   allocated at the place pointed by the second argument.

   There are 6 modifiers, each consumes 2 chars.
   The octal form of a character code consumes
   (1 + CHARACTERBITS / 3 + 1) chars (including backslash at the head).
   We need one more byte for string terminator `\0'.  */
pub const KEY_DESCRIPTION_SIZE: c_int = (2 * 6) + 1 + (CHARACTERBITS / 3) + 1 + 1;

/// `KEYMAPP (m)` is C's macro (keymap.h 34), and the port keeps both its name and its macro
/// shape (PORTING.md 0.2: a C macro keeps its name).  Its argument is a plain Lisp_Object
/// expression that both languages evaluate exactly once, so a Rust `fn` would be
/// behaviourally equivalent; the macro is what C's `KEYMAPP` *is*, and the expansion below is
/// C's text (`(!NILP (get_keymap (m, false, false)))`).
macro_rules! KEYMAPP { ($m:expr) => { !NILP (get_keymap ($m, false, false)) }; }

/// C: `typedef void (*map_keymap_function_t)
///      (Lisp_Object key, Lisp_Object val, Lisp_Object args, void *data);`
/// (`Option<…>` is the port's NULL-able C function pointer, PORTING.md 5.2.)
pub type map_keymap_function_t = Option<unsafe extern "C-unwind" fn (key: Lisp_Object,
                                                             val: Lisp_Object,
                                                             args: Lisp_Object,
                                                             data: *mut c_void)>;

// Every object and function in this block is **defined by keymap.c itself** (its nine
// non-`static` functions plus `current_global_map`), so they live in a module + glob
// import: an explicit item shadows a glob import, where a same-named declaration at this
// scope would be `E0428` against the definition (PORTING.md 0.1-10/P-21, the
// composite.rs/`intervals_decls` shape).  `src/keymap.rs` is what needs them.
pub mod keymap_decls {
  use super::*;

  unsafe extern "C-unwind" {
  pub safe static current_global_map: VmGlobal<Lisp_Object>;

  // Parameter names come from the definitions in keymap.c (keymap.h names no parameter
  // except `map_keymap_canonical`'s).
  pub fn push_key_description (ch: EMACS_INT, p: *mut c_char) -> *mut c_char;
  pub safe fn access_keymap (map: Lisp_Object, idx: Lisp_Object, t_ok: bool,
                             noinherit: bool) -> Lisp_Object;
  pub safe fn get_keymap (object: Lisp_Object, error_if_not_keymap: bool,
                          autoload: bool) -> Lisp_Object;
  pub fn current_minor_maps (modeptr: *mut *mut Lisp_Object,
                             mapptr: *mut *mut Lisp_Object) -> ptrdiff_t;
  pub fn initial_define_lispy_key (keymap: Lisp_Object, keyname: *const c_char,
                                   defname: *const c_char);
  pub safe fn syms_of_keymap ();
  pub fn map_keymap (map: Lisp_Object, fun: map_keymap_function_t, args: Lisp_Object,
                     data: *mut c_void, autoload: bool);
  pub fn map_keymap_canonical (map: Lisp_Object, fun: map_keymap_function_t,
                               args: Lisp_Object, data: *mut c_void);
  }
}
pub use self::keymap_decls::*;

#[inline(always)]
pub fn push_key_description_str (ch: EMACS_INT) -> Lisp_Object {
  let mut tem: [c_char; KEY_DESCRIPTION_SIZE as usize] = [0; KEY_DESCRIPTION_SIZE as usize];
  /* SAFETY: `tem` has `KEY_DESCRIPTION_SIZE` bytes as required by `push_key_description`. */
  unsafe {
    let p = push_key_description (ch, tem.as_mut_ptr ());
    *p = 0;
    let nbytes = p.offset_from (tem.as_ptr ());
    crate::alloc::make_specified_string (tem.as_ptr (), -1, nbytes, true)
  }
}

#[inline(always)]
pub fn error_single_key_description_type () -> ! {
  /* SAFETY: static NUL-terminated message with zero format arguments. */
  unsafe { error (c"KEY must be an integer, cons, symbol, or string".as_ptr ()) }
}


