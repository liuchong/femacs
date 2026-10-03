// femacs: port of lib/c-ctype.h @ fd7afced403 (GNU Emacs 32.0.50).
//
// The frozen target takes the `C_CTYPE_ASCII` arm (C 49-74's big `(' ' == 32) && …`
// test), so the case lists below are the ASCII ones, and `lib/c-ctype.c` is only the
// `C_CTYPE_INLINE` wrapper (femacs/lib/c-ctype.rs).
//
// The definitions are header-inline in C, so they are `#[inline(always)]` functions here
// (PORTING.md P-12) and emit no symbol; the fifteen of them are re-exported by the unit
// (PORTING.md P-81).
//
// C spells each predicate as a `switch` over `case` labels assembled from the
// `_C_CTYPE_*` macro family; Rust's `match` takes the same sets as (or-)patterns, which is
// what the macros below are.  The predicates match on `c as u8`: every C case label is
// ASCII (< 128) and a negative `c` (EOF) wraps to a byte no pattern names, so it falls
// through to `false` exactly as C's `default` does.  The `\xNN` escapes are C's spellings
// of the control characters (`\a`/`\b`/`\f`/`\n`/`\r`/`\t`/`\v` are 0x07-0x0d).

/* C 49-74: `# define C_CTYPE_ASCII 1`  */
pub const C_CTYPE_ASCII: c_int = 1;

/* The include guard has no Rust counterpart (PORTING.md 0.1).  */
pub const C_CTYPE_H: bool = true;

/* # define _C_CTYPE_DIGIT  */
macro_rules! _C_CTYPE_DIGIT { () => { b'0'..=b'9' }; }
/* # define _C_CTYPE_LOWER  */
macro_rules! _C_CTYPE_LOWER { () => { b'a'..=b'z' }; }
/* # define _C_CTYPE_UPPER  */
macro_rules! _C_CTYPE_UPPER { () => { b'A'..=b'Z' }; }
/* `_C_CTYPE_LOWER_A_THRU_F_N (N)` / `_C_CTYPE_LOWER_N (N)` are the two parameterised
   families the lists are assembled from; `_C_CTYPE_A_THRU_F` is the only user, so the port
   writes that one out and the `_N` pair stays the C text above (PORTING.md 0.1).  */
/* # define _C_CTYPE_A_THRU_F  */
macro_rules! _C_CTYPE_A_THRU_F { () => { b'a'..=b'f' | b'A'..=b'F' }; }
/* # define _C_CTYPE_PUNCT  */
macro_rules! _C_CTYPE_PUNCT { () => { 0x21..=0x2f | 0x3a..=0x40 | 0x5b..=0x60 | 0x7b..=0x7e }; }
/* C 94-110's ASCII arm: `'\a' '\b' '\f' '\n' '\r' '\t' '\v'` (0x07..=0x0d) followed by
   `_C_CTYPE_OTHER_CNTRL`, which is `'\x00'-'\x06'`, `'\x0e'-'\x1f'` and `'\x7f'`.  */
/* # define _C_CTYPE_CNTRL  */
macro_rules! _C_CTYPE_CNTRL { () => { _C_CTYPE_OTHER_CNTRL! () | 0x07..=0x0d }; }
/* # define _C_CTYPE_OTHER_CNTRL  */
macro_rules! _C_CTYPE_OTHER_CNTRL { () => { 0x00..=0x06 | 0x0e..=0x1f | 0x7f }; }

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

/* C_CTYPE_INLINE bool c_isascii (int c)  */
#[inline(always)]
pub unsafe fn c_isascii (c: c_int) -> bool
{
  match c as u8
    {
    b' ' => { return true; }
    _C_CTYPE_CNTRL! () => { return true; }
    _C_CTYPE_DIGIT! () => { return true; }
    _C_CTYPE_LOWER! () => { return true; }
    _C_CTYPE_PUNCT! () => { return true; }
    _C_CTYPE_UPPER! () => { return true; }
    _ => { return false; }
    }
}

/* C_CTYPE_INLINE bool c_isblank (int c)  */
#[inline(always)]
pub unsafe fn c_isblank (c: c_int) -> bool
{
  return c == ' ' as c_int || c == '\t' as c_int;
}

/* C_CTYPE_INLINE bool c_iscntrl (int c)  */
#[inline(always)]
pub unsafe fn c_iscntrl (c: c_int) -> bool
{
  match c as u8
    {
    _C_CTYPE_CNTRL! () => { return true; }
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

/* C_CTYPE_INLINE bool c_isgraph (int c)  */
#[inline(always)]
pub unsafe fn c_isgraph (c: c_int) -> bool
{
  match c as u8
    {
    _C_CTYPE_DIGIT! () => { return true; }
    _C_CTYPE_LOWER! () => { return true; }
    _C_CTYPE_PUNCT! () => { return true; }
    _C_CTYPE_UPPER! () => { return true; }
    _ => { return false; }
    }
}

/* C_CTYPE_INLINE bool c_islower (int c)  */
#[inline(always)]
pub unsafe fn c_islower (c: c_int) -> bool
{
  match c as u8
    {
    _C_CTYPE_LOWER! () => { return true; }
    _ => { return false; }
    }
}

/* C_CTYPE_INLINE bool c_isprint (int c)  */
#[inline(always)]
pub unsafe fn c_isprint (c: c_int) -> bool
{
  match c as u8
    {
    b' ' => { return true; }
    _C_CTYPE_DIGIT! () => { return true; }
    _C_CTYPE_LOWER! () => { return true; }
    _C_CTYPE_PUNCT! () => { return true; }
    _C_CTYPE_UPPER! () => { return true; }
    _ => { return false; }
    }
}

/* C_CTYPE_INLINE bool c_ispunct (int c)  */
#[inline(always)]
pub unsafe fn c_ispunct (c: c_int) -> bool
{
  match c as u8
    {
    _C_CTYPE_PUNCT! () => { return true; }
    _ => { return false; }
    }
}

/* C_CTYPE_INLINE bool c_isspace (int c)  */
#[inline(always)]
pub unsafe fn c_isspace (c: c_int) -> bool
{
  /* C 302-303's `case ' ': case '\t': case '\n': case '\v': case '\f': case '\r':` - the
     header spells these six characters out rather than through a `_C_CTYPE_*` macro, so the
     port does too.  A `match` cannot take them: Rust has no `\v`/`\f` escapes (the port
     writes `\x0b`/`\x0c`) and a char literal in a pattern is a `char`, not the `int` C
     compares, so C's switch is an `if` chain of the same six comparisons.  */
  if c == ' ' as c_int || c == '\t' as c_int || c == '\n' as c_int
      || c == '\x0b' as c_int || c == '\x0c' as c_int || c == '\r' as c_int
    {
      return true;
    }
  return false;
}

/* C_CTYPE_INLINE bool c_isupper (int c)  */
#[inline(always)]
pub unsafe fn c_isupper (c: c_int) -> bool
{
  match c as u8
    {
    _C_CTYPE_UPPER! () => { return true; }
    _ => { return false; }
    }
}

/* C_CTYPE_INLINE bool c_isxdigit (int c)  */
#[inline(always)]
pub unsafe fn c_isxdigit (c: c_int) -> bool
{
  match c as u8
    {
    _C_CTYPE_DIGIT! () => { return true; }
    _C_CTYPE_A_THRU_F! () => { return true; }
    _ => { return false; }
    }
}

/* C_CTYPE_INLINE int c_tolower (int c)  */
#[inline(always)]
pub unsafe fn c_tolower (c: c_int) -> c_int
{
  match c as u8
    {
    _C_CTYPE_UPPER! () => { return c - b'A' as c_int + b'a' as c_int; }
    _ => { return c; }
    }
}

/* C_CTYPE_INLINE int c_toupper (int c)  */
#[inline(always)]
pub unsafe fn c_toupper (c: c_int) -> c_int
{
  match c as u8
    {
    _C_CTYPE_LOWER! () => { return c - b'a' as c_int + b'A' as c_int; }
    _ => { return c; }
    }
}
