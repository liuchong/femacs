// femacs: mirror of lib/dynarray.h @ fd7afced403.
//
// The reference header is the gnulib wrapper around the shared glibc definitions.  It has
// two readings and the frozen build takes the `#else` one everywhere:
//
//   * `#if defined DYNARRAY_STRUCT || defined DYNARRAY_ELEMENT || defined DYNARRAY_PREFIX`
//     (C 258) - the reading that defines an instantiation: `_GL_LIKELY`/`_GL_UNLIKELY`, then
//     the generated `malloc/dynarray.gl.h` and `malloc/dynarray-skeleton.gl.h`.  No file in
//     this build includes the wrapper with those three names defined (`regexec.c` includes
//     `malloc/dynarray-skeleton.c` directly at C 1441 and reaches the wrapper through
//     `regex_internal.h` 35 with none of them defined), so this arm's text is recorded in
//     `lib/malloc/dynarray-skeleton.rs` and its ledger, not instantiated here.
//   * `#else` (C 279-281) - "This file is being included from one of the malloc/dynarray_*.c
//     files", i.e. `#include <malloc/dynarray.h>`: the `include!` below.
//
// Both arms reach the same header text (the generated `dynarray.gl.h` is `malloc/dynarray.h`
// plus its generated-marker line, minus the five `libc_hidden_proto` lines), so the port
// includes the shared mirror once; `lib/malloc/dynarray.rs` explains that split.
//
// The generated header also renames the five entries (C 250-256), and the source of the
// rename is what the objects show: the reference `regex.o` undefineds
// `_gl_dynarray_resize` (nm prints `__gl_dynarray_resize`).  The port keeps C's source name
// and spells the renamed symbol on the declaration `#[link_name]`
// (lib/malloc/include/dynarray.rs) and on the definition `#[unsafe(export_name)]`
// (the units), so these five macros are the rename record PORTING.md 10.13(1) asks for.

/* Avoid possible conflicts with symbols exported by the GNU libc.  (C 250-256)  */
macro_rules! __libc_dynarray_at_failure {
  ($size:expr, $index:expr) => { _gl_dynarray_at_failure ($size, $index) };
}
macro_rules! __libc_dynarray_emplace_enlarge {
  ($list:expr, $scratch:expr, $element_size:expr) => {
    _gl_dynarray_emplace_enlarge ($list, $scratch, $element_size)
  };
}
macro_rules! __libc_dynarray_finalize {
  ($list:expr, $scratch:expr, $element_size:expr, $result:expr) => {
    _gl_dynarray_finalize ($list, $scratch, $element_size, $result)
  };
}
macro_rules! __libc_dynarray_resize_clear {
  ($list:expr, $size:expr, $scratch:expr, $element_size:expr) => {
    _gl_dynarray_resize_clear ($list, $size, $scratch, $element_size)
  };
}
macro_rules! __libc_dynarray_resize {
  ($list:expr, $size:expr, $scratch:expr, $element_size:expr) => {
    _gl_dynarray_resize ($list, $size, $scratch, $element_size)
  };
}

/* Rely on __builtin_expect, as provided by the module 'builtin-expect'.  (C 259-264).
   `__builtin_expect` is a branch hint, not a layout hint (PORTING.md 2.6), so the port keeps
   the identity-plus-hint pair the scratch_buffer mirror uses.  The macros come before their
   users, which PORTING.md 0.1-18 requires of a mirror.  */
macro_rules! __builtin_expect {
  ($cond:expr, $hint:expr) => { $cond };
}
macro_rules! _GL_LIKELY {
  ($cond:expr) => { __builtin_expect! ($cond, 1) };
}
macro_rules! _GL_UNLIKELY {
  ($cond:expr) => { __builtin_expect! ($cond, 0) };
}

/* C 266-276's `#if defined DYNARRAY_STRUCT || ...` arm's two generated includes are the
   skeleton instantiation; see the note above and lib/malloc/dynarray-skeleton.rs.  */

/* C 278-281's `#else` arm: `<malloc/dynarray.h>`.  */
include!("../malloc/include/dynarray.rs");

/* The include guard has no Rust counterpart (PORTING.md 0.1).  */
pub const _GL_DYNARRAY_H: bool = true;
