// femacs: port of src/regex-emacs.h @ fd7afced403 (GNU Emacs 32.0.50).
//
// Partial header mirror: only the character-class API that src/syntax.c reads, because
// regex-emacs.h's main body describes `struct re_pattern_buffer`/`struct re_registers` and the
// whole regex engine, whose port (regex-emacs.c) has not started.  docs/ports/include-regex-emacs.md
// lists what is still unchecked; this file carries what the first consumer needs, exactly as
// docs/ports/include-stdio-impl.md records for that mirror.
//
// `re_wctype_t` is C's `typedef enum { RECC_ERROR = 0, ..., RECC_NUM_CLASSES = RECC_UNIBYTE }`
// (C 182-192).  `RECC_NUM_CLASSES` repeats a discriminant, which a Rust enum cannot spell, so the
// port keeps the type as an integer with one constant per C name; the values are C's, in order.

pub const EMACS_REGEX_H: bool = true;

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
// (re_pattern_buffer, re_registers, the re_* API) are not mirrored yet.
