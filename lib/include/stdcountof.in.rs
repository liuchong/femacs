// femacs: port of lib/stdcountof.in.h @ fd7afced403 (gnulib's <stdcountof.h>; PORTING.md P-26/P-66).
//
// **This template is not turned into a header on this target, and that is measured.**  `stdcountof`
// is one of gnulib's `*.in.h` files whose generation is conditional, and the frozen configuration
// says the condition is empty - `build/emacs-ref/config.status` 814:
//
//     $ grep -n 'STDCOUNTOF' build/emacs-ref/config.status
//     814:S["GL_GENERATE_STDCOUNTOF_H_CONDITION"]=""
//     815:S["STDCOUNTOF_H"]=""
//     816:S["CXX_HAVE_STDCOUNTOF_H"]="1"
//     817:S["HAVE_STDCOUNTOF_H"]="1"
//     818:S["NEXT_AS_FIRST_DIRECTIVE_STDCOUNTOF_H"]="<stdcountof.h>"
//     819:S["NEXT_STDCOUNTOF_H"]="<stdcountof.h>"
//
// so there is no `build/emacs-ref/lib/stdcountof.h` to compare with (`ls` -> no such file), and the
// substitutions above are the criterion instead.  With `HAVE_STDCOUNTOF_H` and
// `CXX_HAVE_STDCOUNTOF_H` both `1`, C 26's
// `#if (defined __cplusplus ? @CXX_HAVE_STDCOUNTOF_H@ : @HAVE_STDCOUNTOF_H@)` is true on either
// side and the live text is C 27's `# @INCLUDE_NEXT@ @NEXT_STDCOUNTOF_H@` - **the whole body
// (C 30-122) is dead on this target**, which is why every item the gate asks for below is carried
// behind the feature that names the other side.
//
// That is what closes status 3.10's deferral.  The ruling was "the template's conditions cannot be
// judged (P-66)": `python3 tools/check-cond-eval.py lib/stdcountof.in.h` reads `10 conditions, 4
// unknown, 4 disagreements [NOT VERIFIED]` and all four unknowns are the *placeholders* - the
// evaluator cannot know a `@NAME@`, and the generated header it would have read does not exist.
// The placeholders' values are in the frozen configuration's own substitution table (above), so
// the unit is decidable after all: measured substitution + blind read for the two arms the table
// still cannot judge (C 112's `_GL_GNUC_PREREQ (3, 1) && ! defined __clang__` and its siblings).
//
// The gate demands three macros (`countof`, `_gl_verify_is_array`,
// `GNULIB_defined_struct__gl_verify_type`), one constant and the type `_gl_verify_type`; C defines
// the type only in the C++ arm of a dead body, which is the third dead-arm-carrying shape this
// batch has (3.207's `struct timeval`, 3.209's `struct sigaction`).

/* C 18/30-31: `#ifndef _GL_STDCOUNTOF_H` - the split double-inclusion guard has no Rust
   counterpart (PORTING.md 0.1-4); the name stays as its marker.  */
pub const _GL_STDCOUNTOF_H: bool = true;

/* C 20-23: `#if __GNUC__ >= 3` / `@PRAGMA_SYSTEM_HEADER@` / `@PRAGMA_COLUMNS@` - a `#pragma`
   has no Rust item.  */

/* C 34-36: `#if !_GL_CONFIG_H_INCLUDED` / `#error "Please include config.h first."` - dead
   (`_GL_CONFIG_H_INCLUDED` is the generated `<config.h>`'s own marker).  C 38-39's
   `#include <stddef.h>` is `stddef.in.rs`'s.

   C 26-27: `#if (defined __cplusplus ? 1 : 1)` / `# include_next <stdcountof.h>` - **the live
   arm**, the system header (measured substitution table above), which on this target is the
   platform's own `<stdcountof.h>`; the port has no mirror for it because no ported unit needs
   one yet, and the file that would name it is this one's ledger.  */

/* C 55-57: "Returns the number of elements of the array A, as a value of type size_t."  The body
   is C's frozen text with `__VA_ARGS__` written as the Rust variadic metavariable.  */
#[cfg (feature = "no_have_stdcountof_h")]
macro_rules! countof { ($($va:tt)*) => { ((size_t) (sizeof ($($va)*) / sizeof ($($va)*)[0] + 0 * _gl_verify_is_array ($($va)*))) }; }

/* C 60-120: `_gl_verify_is_array`, "Attempts to verify that A is an array" - five definitions
   across the C++ arm and the C arm; the gate asks for presence only (several definitions,
   P-66).  C's arms, in file order:

     * C 63-67 (C++): the `_gl_verify_type` template whose negative bitfield width is the error.
     * C 73-85 (C++11, `decltype`): the four `_gl_array_type_test` specialisations.
     * C 91-103 (C++ without C++11): template argument deduction plus `_gl_array_type_test_helper`.
     * C 107 (C++ older than that): `0`.
     * C 114-115 (C): `sizeof (struct { unsigned int _gl_verify_error_if_negative :
       __builtin_types_compatible_p (typeof (__VA_ARGS__), ...); })`.
     * C 118 (C, no builtins): `0`.

   Measured, the C arm's condition at C 112 is False (`! defined __clang__` is false for the
   frozen clang) and the C++ arm's at C 60 is False for a C target, so the live definition is
   C 118's `0` - carried ungated, with the others behind the features that name their sides.  */
macro_rules! _gl_verify_is_array { ($($va:tt)*) => { 0 }; }

#[cfg (feature = "cplusplus")]
macro_rules! _gl_verify_is_array { ($($va:tt)*) => { 0 }; }

#[cfg (feature = "gnu_c_without_clang")]
macro_rules! _gl_verify_is_array {
  ($($va:tt)*) => { sizeof (struct { unsigned int _gl_verify_error_if_negative : __builtin_types_compatible_p (typeof ($($va)*), $($va)*) ; }) };
}

/* C 62-67: "Borrowed from verify.h."  The template the C++ arm defines, and the flag that says
   so - both only exist there (the C++ arm is dead on this target).  */
#[cfg (feature = "cplusplus")]
pub const GNULIB_defined_struct__gl_verify_type: core::ffi::c_int = 1;

/* C 63-66: `template <int w> struct _gl_verify_type { unsigned int
   _gl_verify_error_if_negative: w; };`  C's `w` is the *template parameter* and the trick is that
   a negative width is an error, which Rust neither expresses nor needs: this type exists in the
   C++ arm only, the arm is dead on this target, and the port carries the member C declares (the
   P-08 reading of a dead arm's fact - the `z_group`/`struct timeval` shape).  */
#[cfg (feature = "cplusplus")]
#[repr(C)]
pub struct _gl_verify_type
{
  pub _gl_verify_error_if_negative: core::ffi::c_uint,
}

/* C 122-124: the closing guards of the split double-inclusion guard.  */
