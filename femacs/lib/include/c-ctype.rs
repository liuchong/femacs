// femacs: port of lib/c-ctype.h @ fd7afced403 (GNU Emacs 32.0.50).
//
// WIP mirror: it carries the ASCII arm's case-macro set and the three predicates the ported
// units need so far (filevercmp.c's `c_isalpha`/`c_isalnum`/`c_isdigit`); the remaining
// `c_is*`/`c_to*` entries are tracked in docs/ports/include-c-ctype.md.  The frozen target
// takes the `C_CTYPE_ASCII` arm (C 56-74's big `(' ' == 32) && …` test), so the case lists
// below are the ASCII ones, and `lib/c-ctype.c` is only the `_GL_CTYPE_INLINE` wrapper
// (femacs/lib/c-ctype.rs).
//
// The definitions are header-inline in C, so they are `#[inline(always)]` functions here
// (PORTING.md P-12) and emit no symbol; the scalar names come from the including unit.

/* # define C_CTYPE_ASCII 1  */
pub const C_CTYPE_ASCII: c_int = 1;

/* The three case lists the ported predicates use; C spells them as `case '0': … case '9':`
   chains, and the port's byte-range patterns are the same sets in the same order.  The
   predicates match on `c as u8`: every case label is ASCII (< 128) and a negative `c` (EOF)
   wraps to a byte no label names, so it falls through to `false` exactly as C's `default`
   does.  */
/* # define _C_CTYPE_DIGIT case '0': … case '9':  */
macro_rules! _C_CTYPE_DIGIT { () => { b'0'..=b'9' }; }
/* # define _C_CTYPE_LOWER case 'a': … case 'z':  */
macro_rules! _C_CTYPE_LOWER { () => { b'a'..=b'z' }; }
/* # define _C_CTYPE_UPPER case 'A': … case 'Z':  */
macro_rules! _C_CTYPE_UPPER { () => { b'A'..=b'Z' }; }

/* C_CTYPE_INLINE bool c_isalnum (int c)  */
#[inline(always)]
pub unsafe fn c_isalnum (c: c_int) -> bool
{
  match c as u8
    {
    _C_CTYPE_DIGIT! () => { return true; }
    _C_CTYPE_LOWER! () => { return true; }
    _C_CTYPE_UPPER! () => { return true; }
    _ => { return false; }
    }
}

/* C_CTYPE_INLINE bool c_isalpha (int c)  */
#[inline(always)]
pub unsafe fn c_isalpha (c: c_int) -> bool
{
  match c as u8
    {
    _C_CTYPE_LOWER! () => { return true; }
    _C_CTYPE_UPPER! () => { return true; }
    _ => { return false; }
    }
}

/* C_CTYPE_INLINE bool c_isdigit (int c)  */
#[inline(always)]
pub unsafe fn c_isdigit (c: c_int) -> bool
{
  match c as u8
    {
    _C_CTYPE_DIGIT! () => { return true; }
    _ => { return false; }
    }
}

/* The include guard has no Rust counterpart (PORTING.md 0.1).  */
pub const C_CTYPE_H: bool = true;
