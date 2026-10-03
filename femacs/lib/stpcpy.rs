// femacs: port of lib/stpcpy.c @ fd7afced403.  C 30-32's
// `#ifndef weak_alias / # define __stpcpy stpcpy / #endif` is the live arm (this target
// has no weak_alias), so the definition C writes as `__stpcpy` leaves the preprocessor as
// `stpcpy`: the port keeps C's source name on the item and emits the real symbol with
// `#[unsafe(export_name = ...)]` (PORTING.md 10.13, the mempcpy precedent).  C 26-28's
// `#ifdef _LIBC` arm and C 47-49's `#ifdef weak_alias` arm are dead.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]
use std::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};

include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/string.rs");     // <string.h>


/* C 30-32's rename macro, which the ledger compares (its body is C's `stpcpy`).  */
macro_rules! __stpcpy { () => { stpcpy }; }

/* Copy SRC to DEST, returning the address of the terminating '\0' in DEST.  */
#[unsafe(export_name = "stpcpy")]
pub unsafe extern "C" fn __stpcpy (dest: *mut c_char, src: *const c_char) -> *mut c_char
{
  let mut d: *mut c_char = dest;
  let mut s: *const c_char = src;

  /* C 41-43's `do *d++ = *s; while (*s++ != '\0');`: the post-increments become the
     explicit steps C's `++` performs them in.  */
  loop
    {
      *d = *s;
      d = d.add (1);
      let c: c_char = *s;
      s = s.add (1);
      if ! (c != 0 as c_char)
        { break; }
    }

  return d.offset (-1);
}
