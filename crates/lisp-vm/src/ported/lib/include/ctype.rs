// femacs: mirror of the frozen target's <ctype.h> (system header, P-26).
//
// The regex cluster is the first port to use the byte classification macros
// regex_internal.h 511-513 names (`isalnum`) and the `isblank` arm it keeps for
// targets without the function (regex_internal.h 70-72 — dead on this one).  The
// frozen target's glibc implements them as functions, so the mirror declares
// functions and lets the libc provide them; the C's macro-vs-function distinction
// is invisible at the call sites.


// [U1-MODULE-MIRROR]
use super::*;
unsafe extern "C" {
  /* int isalnum (int c);  */
  pub fn isalnum (c: c_int) -> c_int;
  /* int isalpha (int c);  */
  pub fn isalpha (c: c_int) -> c_int;
  /* int isblank (int c);  */
  pub fn isblank (c: c_int) -> c_int;
  /* int isdigit (int c);  */
  pub fn isdigit (c: c_int) -> c_int;
  /* int islower (int c);  */
  pub fn islower (c: c_int) -> c_int;
  /* int isspace (int c);  */
  pub fn isspace (c: c_int) -> c_int;
  /* int iscntrl (int c);  */
  pub fn iscntrl (c: c_int) -> c_int;
  /* int isgraph (int c);  */
  pub fn isgraph (c: c_int) -> c_int;
  /* int isprint (int c);  */
  pub fn isprint (c: c_int) -> c_int;
  /* int ispunct (int c);  */
  pub fn ispunct (c: c_int) -> c_int;
  /* int isxdigit (int c);  */
  pub fn isxdigit (c: c_int) -> c_int;
  /* int isupper (int c);  */
  pub fn isupper (c: c_int) -> c_int;
  /* int toupper (int c);  */
  pub fn toupper (c: c_int) -> c_int;
  /* int tolower (int c);  */
  pub fn tolower (c: c_int) -> c_int;
}
