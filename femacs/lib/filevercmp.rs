// femacs: port of lib/filevercmp.c @ fd7afced403.
//
// One unconditional body: the file has no `#if` arms at all, and every helper is `static`
// (so none of them carries `#[unsafe(no_mangle)]`).  C 20's <config.h> is the config mirror,
// C 23's <c-ctype.h> the c-ctype mirror (the three ASCII predicates are header-inline in C,
// hence `#[inline(always)]` functions in the port, P-12), C 24's <limits.h> the limits
// mirror for `UCHAR_MAX`/`INT_MAX`, and C 25's <idx.h> the idx mirror.
//
// Deviations (docs/ports/filevercmp.md): `static_assert` is the port's `static_assert!`
// (PORTING.md 2.5: `const _: () = assert! (…)`), `bool`/`false`/`true` stay Rust's, and the
// `switch`es over the ASCII case lists are `match`es with the same arms in the same order.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]

use std::ffi::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong};

include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/limits.in.rs");
include!("include/idx.rs");
include!("include/c-ctype.rs");
include!("include/filevercmp.rs");

/// PORTING.md 2.5: C's `static_assert` is `const _: () = assert! (cond);`.
macro_rules! static_assert {
    ($cond:expr) => { const _: () = assert! ($cond); };
}

/* Return the length of a prefix of S that corresponds to the suffix
   defined by this extended regular expression in the C locale:
     (\.[A-Za-z~][A-Za-z0-9~]*)*$
   Use the longest suffix matching this regular expression,
   except do not use all of S as a suffix if S is nonempty.
   If *LEN is -1, S is a string; set *LEN to S's length.
   Otherwise, *LEN should be nonnegative, S is a char array,
   and *LEN does not change.  */
unsafe extern "C" fn file_prefixlen (s: *const c_char, len: *mut isize) -> idx_t
{
  let n: usize = *len as usize;  /* SIZE_MAX if N == -1.  */
  let mut prefixlen: idx_t = 0;

  let mut i: idx_t = 0;
  loop
    {
      if if *len < 0 { *s.offset (i as isize) == 0 } else { i == n as idx_t }
        {
          *len = i;
          return prefixlen;
        }

      i += 1;
      prefixlen = i;
      while i + 1 < n as idx_t && *s.offset (i as isize) == b'.' as c_char
            && (c_isalpha (*s.offset ((i + 1) as isize) as c_int)
                || *s.offset ((i + 1) as isize) == b'~' as c_char)
        {
          i += 2;
          while i < n as idx_t && (c_isalnum (*s.offset (i as isize) as c_int)
                                   || *s.offset (i as isize) == b'~' as c_char)
            { i += 1; }
        }
    }
}

/* Return a version sort comparison value for S's byte at position POS.
   S has length LEN.  If POS == LEN, sort before all non-'~' bytes.  */

unsafe extern "C" fn order (s: *const c_char, pos: idx_t, len: idx_t) -> c_int
{
  if (pos == len)
    { return -1; }

  let c: c_uchar = *s.offset (pos as isize) as c_uchar;
  if (c_isdigit (c as c_int))
    { return 0; }
  else if (c_isalpha (c as c_int))
    { return c as c_int; }
  else if (c == b'~')
    { return -2; }
  else
    {
      static_assert! (UCHAR_MAX <= (INT_MAX - 1 - 2) / 2);
      return c as c_int + UCHAR_MAX + 1;
    }
}

/* slightly modified verrevcmp function from dpkg
   S1, S2 - compared char array
   S1_LEN, S2_LEN - length of arrays to be scanned

   This implements the algorithm for comparison of version strings
   specified by Debian and now widely adopted.  The detailed
   specification can be found in the Debian Policy Manual in the
   section on the 'Version' control field.  This version of the code
   implements that from s5.6.12 of Debian Policy v3.8.0.1
   https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-Version */
unsafe extern "C" fn verrevcmp (s1: *const c_char, s1_len: idx_t, s2: *const c_char,
                                s2_len: idx_t) -> c_int
{
  let mut s1_pos: idx_t = 0;
  let mut s2_pos: idx_t = 0;
  while (s1_pos < s1_len || s2_pos < s2_len)
    {
      let mut first_diff: c_int = 0;
      while ((s1_pos < s1_len && ! c_isdigit (*s1.offset (s1_pos as isize) as c_int))
             || (s2_pos < s2_len
                 && ! c_isdigit (*s2.offset (s2_pos as isize) as c_int)))
        {
          let s1_c: c_int = order (s1, s1_pos, s1_len);
          let s2_c: c_int = order (s2, s2_pos, s2_len);
          if (s1_c != s2_c)
            { return s1_c - s2_c; }
          s1_pos += 1;
          s2_pos += 1;
        }
      while (s1_pos < s1_len && *s1.offset (s1_pos as isize) == b'0' as c_char)
        { s1_pos += 1; }
      while (s2_pos < s2_len && *s2.offset (s2_pos as isize) == b'0' as c_char)
        { s2_pos += 1; }
      while (s1_pos < s1_len && s2_pos < s2_len
             && c_isdigit (*s1.offset (s1_pos as isize) as c_int)
             && c_isdigit (*s2.offset (s2_pos as isize) as c_int))
        {
          if first_diff == 0
            { first_diff = *s1.offset (s1_pos as isize) as c_int
                           - *s2.offset (s2_pos as isize) as c_int; }
          s1_pos += 1;
          s2_pos += 1;
        }
      if (s1_pos < s1_len && c_isdigit (*s1.offset (s1_pos as isize) as c_int))
        { return 1; }
      if (s2_pos < s2_len && c_isdigit (*s2.offset (s2_pos as isize) as c_int))
        { return -1; }
      if (first_diff != 0)
        { return first_diff; }
    }
  return 0;
}

/* Compare version strings S1 and S2.
   See filevercmp.h for function description.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn filevercmp (s1: *const c_char, s2: *const c_char) -> c_int
{
  return filenvercmp (s1, -1, s2, -1);
}

/* Compare versions A (of length ALEN) and B (of length BLEN).
   See filevercmp.h for function description.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn filenvercmp (a: *const c_char, mut alen: isize, b: *const c_char,
                                      mut blen: isize) -> c_int
{
  /* Special case for empty versions.  */
  let aempty: bool = if alen < 0 { *a == 0 } else { alen == 0 };
  let bempty: bool = if blen < 0 { *b == 0 } else { blen == 0 };
  if (aempty)
    { return -((!bempty) as c_int); }
  if (bempty)
    { return 1; }

  /* Special cases for leading ".": "." sorts first, then "..", then
     other names with leading ".", then other names.  */
  if (*a == b'.' as c_char)
    {
      if (*b != b'.' as c_char)
        { return -1; }

      let adot: bool = if alen < 0 { *a.offset (1) == 0 } else { alen == 1 };
      let bdot: bool = if blen < 0 { *b.offset (1) == 0 } else { blen == 1 };
      if (adot)
        { return -((!bdot) as c_int); }
      if (bdot)
        { return 1; }

      let adotdot: bool = *a.offset (1) == b'.' as c_char
                          && (if alen < 0 { *a.offset (2) == 0 } else { alen == 2 });
      let bdotdot: bool = *b.offset (1) == b'.' as c_char
                          && (if blen < 0 { *b.offset (2) == 0 } else { blen == 2 });
      if (adotdot)
        { return -((!bdotdot) as c_int); }
      if (bdotdot)
        { return 1; }
    }
  else if (*b == b'.' as c_char)
    { return 1; }

  /* Cut file suffixes.  */
  let aprefixlen: idx_t = file_prefixlen (a, &mut alen);
  let bprefixlen: idx_t = file_prefixlen (b, &mut blen);

  /* If both suffixes are empty, a second pass would return the same thing.  */
  let one_pass_only: bool = aprefixlen == alen && bprefixlen == blen;

  let result: c_int = verrevcmp (a, aprefixlen, b, bprefixlen);

  /* Return the initial result if nonzero, or if no second pass is needed.
     Otherwise, restore the suffixes and try again.  */
  return if result != 0 || one_pass_only { result } else { verrevcmp (a, alen, b, blen) };
}
