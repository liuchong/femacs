// femacs: port of src/regex-emacs.h @ fd7afced403.
//
// Partial header mirror: the character-class API that src/syntax.c reads plus the one record
// thread.h embeds (`struct re_registers`, C 28-35, which regex-emacs.h deliberately declares
// before including lisp.h).  regex-emacs.h's main body still describes `struct
// re_pattern_buffer` and the whole regex engine, whose port (regex-emacs.c) has not started.
// docs/ports/include-regex-emacs.md lists what is still unchecked; this file carries what the
// first consumers need, exactly as docs/ports/include-stdio-impl.md records for that mirror.
//
// `re_wctype_t` is C's `typedef enum { RECC_ERROR = 0, ..., RECC_NUM_CLASSES = RECC_UNIBYTE }`
// (C 182-192).  `RECC_NUM_CLASSES` repeats a discriminant, which a Rust enum cannot spell, so the
// port keeps the type as an integer with one constant per C name; the values are C's, in order.

pub const EMACS_REGEX_H: bool = true;

/* This is the structure we store register match data in.
   Declare this before including lisp.h, since lisp.h (via thread.h)
   uses struct re_registers.  */
// regex-emacs.h 28-35.  It is declared before C's `#include "lisp.h"` because thread.h needs
// it: `src/thread.c` frees the two `struct re_registers` a thread_state embeds
// (`free_search_regs`), so the record lands here with that first consumer (the
// docs/ports/include-regex-emacs.md row).  Measured 24 bytes, alignment 8: `num_regs@0`,
// `start@8`, `end@16`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct re_registers
{
  pub num_regs: ptrdiff_t,
  pub start: *mut ptrdiff_t,
  pub end: *mut ptrdiff_t,
}
const _: () = assert! (std::mem::size_of::<re_registers> () == 24);
const _: () = assert! (std::mem::offset_of! (re_registers, start) == 8);
const _: () = assert! (std::mem::offset_of! (re_registers, end) == 16);

pub type re_wctype_t = c_int;

pub const RECC_ERROR: re_wctype_t = 0;
pub const RECC_ALNUM: re_wctype_t = 1;
pub const RECC_ALPHA: re_wctype_t = 2;
pub const RECC_WORD: re_wctype_t = 3;
pub const RECC_GRAPH: re_wctype_t = 4;
pub const RECC_PRINT: re_wctype_t = 5;
pub const RECC_LOWER: re_wctype_t = 6;
pub const RECC_UPPER: re_wctype_t = 7;
pub const RECC_PUNCT: re_wctype_t = 8;
pub const RECC_CNTRL: re_wctype_t = 9;
pub const RECC_DIGIT: re_wctype_t = 10;
pub const RECC_XDIGIT: re_wctype_t = 11;
pub const RECC_BLANK: re_wctype_t = 12;
pub const RECC_SPACE: re_wctype_t = 13;
pub const RECC_MULTIBYTE: re_wctype_t = 14;
pub const RECC_NONASCII: re_wctype_t = 15;
pub const RECC_ASCII: re_wctype_t = 16;
pub const RECC_UNIBYTE: re_wctype_t = 17;
pub const RECC_NUM_CLASSES: re_wctype_t = RECC_UNIBYTE;

unsafe extern "C" {
  /// C: regex-emacs.h 194: `extern bool re_iswctype (int ch, re_wctype_t cc);`
  pub fn re_iswctype (ch: c_int, cc: re_wctype_t) -> bool;
  /// C: regex-emacs.h 195-196: `extern re_wctype_t re_wctype_parse (const unsigned char **strp,
  /// ptrdiff_t limit);`
  pub fn re_wctype_parse (strp: *mut *const u8, limit: ptrdiff_t) -> re_wctype_t;
}

// C 198-206 (`#if ENABLE_CHECKING`'s print_compiled_pattern) and the rest of the header
// (re_pattern_buffer, the re_* API) are not mirrored yet.
