// femacs: port of lib/c-strcasecmp.c @ fd7afced403.  The file is
// blind; its only conditions are the two `#if`s of <limits.h> (C 22) and c-ctype.h (C 24),
// which contribute no body text.  `UCHAR_MAX <= INT_MAX` is live on this target (both are
// in lib/include/limits.in.rs), and C's `else` arm is carried as text so the checker sees
// `_GL_CMP` (PORTING.md P-51 / lib/include/gnulib_config.rs).
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]
use std::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};

include!("../src/include/config.rs");
include!("include/gnulib_config.rs");   // _GL_CMP
include!("include/limits.in.rs");       // <limits.h> (UCHAR_MAX, INT_MAX)
include!("include/c-ctype.rs");         // "c-ctype.h" (c_tolower)


/* int c_strcasecmp (const char *s1, const char *s2)  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn c_strcasecmp (s1: *const c_char, s2: *const c_char) -> c_int
{
  let mut p1: *const u8 = s1 as *const u8;
  let mut p2: *const u8 = s2 as *const u8;

  if p1 == p2
    { return 0; }

  /* C 35's `for (;; p1++, p2++)`: the two steps move to the loop's tail, where C's
     increment expression runs them.  */
  loop
    {
      let c1: u8 = c_tolower (*p1 as c_int) as u8;
      let c2: u8 = c_tolower (*p2 as c_int) as u8;

      if c1 == 0 || c1 != c2
        {
          if UCHAR_MAX <= INT_MAX
            { return c1 as c_int - c2 as c_int; }
          else
            /* On machines where 'char' and 'int' are types of the same size,
               the difference of two 'unsigned char' values - including the
               sign bit - doesn't fit in an 'int'.  */
            { return _GL_CMP! (c1, c2); }
        }
      p1 = p1.add (1);
      p2 = p2.add (1);
    }
}
