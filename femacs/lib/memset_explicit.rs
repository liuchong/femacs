/* Erase sensitive data from memory.
   Copyright 2022-2026 Free Software Foundation, Inc.

   This file is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as
   published by the Free Software Foundation; either version 2.1 of the
   License, or (at your option) any later version.

   This file is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]
use std::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};
// femacs: port of lib/memset_explicit.c @ fd7afced403.  Blind file.
// The frozen configuration gives the `HAVE_MEMSET_S` arm (macros.tsv 1432 is 1, with
// HAVE_EXPLICIT_MEMSET undefined and HAVE_MEMSET_S_SUPPORTS_ZERO 1), so C 16's
// `(void) memset_s (s, len, c, len);` and C 17's `return s;` are the live text; the other
// three arms are named at their sites.  The reference object confirms the name:
// `nm -g build/emacs-ref/lib/memset_explicit.o` prints `T _memset_explicit`.
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/string.rs");     // <string.h> (`memset_s`, `memset`)

/* Set S's bytes to C, where S has LEN bytes.  The compiler will not
   optimize effects away, even if S is dead after the call.  */

/* void *memset_explicit (void *s, int c, size_t len)  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset_explicit (s: *mut c_void, c: c_int, len: usize) -> *mut c_void
{
  /* C 10-11's `#if HAVE_EXPLICIT_MEMSET` arm (`return explicit_memset (s, c, len);`),
     C 13-15's `# if !HAVE_MEMSET_S_SUPPORTS_ZERO` guard (`if (len > 0)`), C 18-22's
     `#elif defined __GNUC__ && !defined __clang__` arm and C 23-36's two remaining arms
     (`__asm__ volatile ("" ::: "memory")`, the `g`-constraint form and the
     `volatile_memset` function pointer) are dead here and named at this site (P-51).  */

  /* C 16-17 (the live `HAVE_MEMSET_S` arm).  */
  let _ = memset_s (s, len, c, len);
  return s;
}
