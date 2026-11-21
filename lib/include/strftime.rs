// femacs: port of lib/strftime.h @ fd7afced403 (gnulib).
//
// Header mirror (PORTING.md 0.1).  The header declares no macro and no type, only the two
// entry points over `timezone_t` (the time.h mirror) and `struct tm` (also the time.h
// mirror); its long `%N`/conversion documentation is the function comment of
// `nstrftime` below.  `nstrftime`/`c_nstrftime` are defined by lib/nstrftime.c, which is a
// separate unit (it `#include`s strftime.c with `my_strftime` renamed).

// [U1-MODULE-MIRROR]
use super::*;
// [U1] time.in.rs imported via super

unsafe extern "C" {
  /* ptrdiff_t nstrftime (char *restrict, size_t, char const *restrict,
                          struct tm const *restrict, timezone_t, int);  */
  pub fn nstrftime (s: *mut c_char, maxsize: usize, format: *const c_char, tp: *const tm,
                    tz: timezone_t, ns: c_int) -> isize;
  /* ptrdiff_t c_nstrftime (…same…);  */
  pub fn c_nstrftime (s: *mut c_char, maxsize: usize, format: *const c_char, tp: *const tm,
                      tz: timezone_t, ns: c_int) -> isize;
}
