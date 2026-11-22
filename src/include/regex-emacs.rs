// femacs: mirror of src/regex-emacs.h @ fd7afced403.
//
// Mirror (PORTING.md 0.1-2): types, constants, `macro_rules!` definitions and the function
// declarations, the latter in `regex_emacs_decls` + glob import (PORTING.md 0.1-10, the
// thread.rs/composite.rs shape) because `src/regex-emacs.c`'s port defines every one of
// them.  Nothing here emits a C-visible symbol.
//
// `re_wctype_t` is C's `typedef enum { RECC_ERROR = 0, ..., RECC_NUM_CLASSES = RECC_UNIBYTE }`
// (C 182-192).  `RECC_NUM_CLASSES` repeats a discriminant, which a Rust enum cannot spell, so the
// port keeps the type as an integer with one constant per C name; the values are C's, in order.
//
// `struct re_registers` (C 27-35) is declared before C's `#include "lisp.h"` because thread.h
// needs it: `src/thread.c` frees the two `struct re_registers` a thread_state embeds
// (`free_search_regs`).  Measured 24 bytes, alignment 8: `num_regs@0`, `start@8`, `end@16`.
//
// `struct re_pattern_buffer` (C 67-121) and the re_* API (C 123-179) land with
// `src/regex-emacs.c`, their definition unit; C 198-200's `print_compiled_pattern` is the
// checking-only third consumer.  C's two `bool_bf` runs and its `unsigned regs_allocated : 2`
// (C 98-120) are one storage unit, so the port packs them into `bits: u8` with one mask per C
// member (PORTING.md 1.3/P-80) and one accessor pair per member; `src/search.c`'s port already
// spells its reads and writes that way (`docs/ports/search.md`), and
// `docs/ports/token-aliases.tsv` carries the six families.  clang measured the frozen target:
// `sizeof` 64, alignment 8, `buffer`@0, `allocated`@8, `used`@16, `charset_unibyte`@24,
// `fastmap`@32, `translate`@40, `re_nsub`@48 and the bit run at 56; the `const _` assertions
// below pin those numbers (the `src/search.rs` and `src/regex-emacs.rs` copies of them go
// away with this change - two same-named definitions in one unit are an `E0428`).

// [U1-MODULE-MIRROR]
use super::*;
pub const EMACS_REGEX_H: c_int = 1;

/* This is the structure we store register match data in.
   Declare this before including lisp.h, since lisp.h (via thread.h)
   uses struct re_registers.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct re_registers
{
  pub num_regs: ptrdiff_t,
  pub start: *mut ptrdiff_t,
  pub end: *mut ptrdiff_t,
}
const _: () = core::assert! (std::mem::size_of::<re_registers> () == 24);
const _: () = core::assert! (std::mem::offset_of! (re_registers, start) == 8);
const _: () = core::assert! (std::mem::offset_of! (re_registers, end) == 16);

/* This data structure represents a compiled pattern.  Before calling
   the pattern compiler, the fields 'buffer', 'allocated', 'fastmap',
   and 'translate' can be set.  After the pattern has been
   compiled, the 're_nsub' field is available.  All other fields are
   private to the regex routines.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct re_pattern_buffer
{
	/* Space that holds the compiled pattern.  It is declared as
          'unsigned char *' because its elements are
           sometimes used as array indexes.  */
  pub buffer: *mut u8,

	/* Number of bytes to which 'buffer' points.  */
  pub allocated: ptrdiff_t,

	/* Number of bytes actually used in 'buffer'.  */
  pub used: ptrdiff_t,

        /* Charset of unibyte characters at compiling time.  */
  pub charset_unibyte: c_int,

        /* Pointer to a fastmap, if any, otherwise zero.  re_search uses
           the fastmap, if there is one, to skip over impossible
           starting points for matches.  */
  pub fastmap: *mut c_char,

        /* Either a translate table to apply to all characters before
           comparing them, or zero for no translation.  The translation
           applies to a pattern when it is compiled and to a string
           when it is matched.  */
  pub translate: Lisp_Object,

	/* Number of subexpressions found by the compiler.  */
  pub re_nsub: ptrdiff_t,

        /* True if and only if this pattern can match the empty string.
           Well, in truth it's used only in 're_search_2', to see
           whether or not we should use the fastmap, so we don't set
           this absolutely perfectly; see 're_compile_fastmap'.  */
        /* If REGS_UNALLOCATED, allocate space in the 'regs' structure
             for at least (re_nsub + 1) groups.
           If REGS_REALLOCATE, reallocate space if necessary.
           If REGS_FIXED, use what's there.  */
        /* Set to false when 'regex_compile' compiles a pattern; set to true
           by 're_compile_fastmap' if it updates the fastmap.  */
  /* If true, the compilation of the pattern had to look up the syntax table,
     so the compiled pattern is valid for the current syntax table only.  */
  /* If true, multi-byte form in the regexp pattern should be
     recognized as a multibyte character.  */
  /* If true, multi-byte form in the target of match should be
     recognized as a multibyte character.  */
  // C 98-120: `bool_bf can_be_null : 1; unsigned regs_allocated : 2; bool_bf
  // fastmap_accurate : 1; bool_bf used_syntax : 1; bool_bf multibyte : 1; bool_bf
  // target_multibyte : 1;` share one storage unit, packed here as `bits: u8` with one mask
  // per C member (PORTING.md 1.3/P-80); the `regs_allocated` value set is
  // `regex-emacs.c`'s `enum { REGS_UNALLOCATED, REGS_REALLOCATE, REGS_FIXED }` (C 862).
  pub bits: u8,
}
const _: () = core::assert! (std::mem::size_of::<re_pattern_buffer> () == 64);
const _: () = core::assert! (std::mem::align_of::<re_pattern_buffer> () == 8);
const _: () = core::assert! (std::mem::offset_of! (re_pattern_buffer, buffer) == 0);
const _: () = core::assert! (std::mem::offset_of! (re_pattern_buffer, allocated) == 8);
const _: () = core::assert! (std::mem::offset_of! (re_pattern_buffer, used) == 16);
const _: () = core::assert! (std::mem::offset_of! (re_pattern_buffer, charset_unibyte) == 24);
const _: () = core::assert! (std::mem::offset_of! (re_pattern_buffer, fastmap) == 32);
const _: () = core::assert! (std::mem::offset_of! (re_pattern_buffer, translate) == 40);
const _: () = core::assert! (std::mem::offset_of! (re_pattern_buffer, re_nsub) == 48);
const _: () = core::assert! (std::mem::offset_of! (re_pattern_buffer, bits) == 56);

// C 98-120's bit members, in C's declaration order.
pub const RE_PATTERN_CAN_BE_NULL: u8 = 1 << 0;
pub const RE_PATTERN_REGS_ALLOCATED: u8 = 3 << 1;
pub const RE_PATTERN_REGS_ALLOCATED_SHIFT: u8 = 1;
pub const RE_PATTERN_FASTMAP_ACCURATE: u8 = 1 << 3;
pub const RE_PATTERN_USED_SYNTAX: u8 = 1 << 4;
pub const RE_PATTERN_MULTIBYTE: u8 = 1 << 5;
pub const RE_PATTERN_TARGET_MULTIBYTE: u8 = 1 << 6;

/* The bit members of `struct re_pattern_buffer` this file's consumers read or write, one
   accessor per C member (PORTING.md 1.3/P-80): C spells them as bit-field accesses, so
   every read and every write is occurrence-checked through the token-aliases families
   (`can_be_null`, `regs_allocated`, `fastmap_accurate`, `used_syntax`, `multibyte`,
   `target_multibyte`).  `#[inline(always)]` keeps them per-unit copies, as lisp.rs's
   accessors are.  */
#[inline(always)]
pub unsafe fn can_be_null_ (bufp: *const re_pattern_buffer) -> bool
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*bufp).bits & RE_PATTERN_CAN_BE_NULL != 0 
  }
}
#[inline(always)]
pub unsafe fn set_can_be_null_ (bufp: *mut re_pattern_buffer, value: bool)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*bufp).bits = if value { (*bufp).bits | RE_PATTERN_CAN_BE_NULL } else { (*bufp).bits & ! RE_PATTERN_CAN_BE_NULL }; 
  }
}

#[inline(always)]
pub unsafe fn regs_allocated_ (bufp: *const re_pattern_buffer) -> u8
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { ((*bufp).bits & RE_PATTERN_REGS_ALLOCATED) >> RE_PATTERN_REGS_ALLOCATED_SHIFT 
  }
}
#[inline(always)]
pub unsafe fn set_regs_allocated_ (bufp: *mut re_pattern_buffer, value: u8)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  (*bufp).bits = ((*bufp).bits & ! RE_PATTERN_REGS_ALLOCATED)
    | ((value << RE_PATTERN_REGS_ALLOCATED_SHIFT) & RE_PATTERN_REGS_ALLOCATED);

  }
}

#[inline(always)]
pub unsafe fn fastmap_accurate_ (bufp: *const re_pattern_buffer) -> bool
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*bufp).bits & RE_PATTERN_FASTMAP_ACCURATE != 0 
  }
}
#[inline(always)]
pub unsafe fn set_fastmap_accurate_ (bufp: *mut re_pattern_buffer, value: bool)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*bufp).bits = if value { (*bufp).bits | RE_PATTERN_FASTMAP_ACCURATE } else { (*bufp).bits & ! RE_PATTERN_FASTMAP_ACCURATE }; 
  }
}

#[inline(always)]
pub unsafe fn used_syntax_ (bufp: *const re_pattern_buffer) -> bool
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*bufp).bits & RE_PATTERN_USED_SYNTAX != 0 
  }
}
#[inline(always)]
pub unsafe fn set_used_syntax_ (bufp: *mut re_pattern_buffer, value: bool)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*bufp).bits = if value { (*bufp).bits | RE_PATTERN_USED_SYNTAX } else { (*bufp).bits & ! RE_PATTERN_USED_SYNTAX }; 
  }
}

#[inline(always)]
pub unsafe fn multibyte_ (bufp: *const re_pattern_buffer) -> bool
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*bufp).bits & RE_PATTERN_MULTIBYTE != 0 
  }
}
#[inline(always)]
pub unsafe fn set_multibyte_ (bufp: *mut re_pattern_buffer, value: bool)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*bufp).bits = if value { (*bufp).bits | RE_PATTERN_MULTIBYTE } else { (*bufp).bits & ! RE_PATTERN_MULTIBYTE }; 
  }
}

#[inline(always)]
pub unsafe fn target_multibyte_ (bufp: *const re_pattern_buffer) -> bool
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*bufp).bits & RE_PATTERN_TARGET_MULTIBYTE != 0 
  }
}
#[inline(always)]
pub unsafe fn set_target_multibyte_ (bufp: *mut re_pattern_buffer, value: bool)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*bufp).bits = if value { (*bufp).bits | RE_PATTERN_TARGET_MULTIBYTE } else { (*bufp).bits & ! RE_PATTERN_TARGET_MULTIBYTE }; 
  }
}

/* Character classes.  */
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

// C 50-56 and 123-206: the three file-scope objects and the eight function declarations.
// They sit in `pub mod regex_emacs_decls` + glob import because `src/regex-emacs.c`'s port
// defines all of them (PORTING.md 0.1-10: a same-named declaration *and* definition in one
// unit is an `E0428`, and the definition shadows the glob import, which is C's
// prototype-plus-definition).
pub mod regex_emacs_decls {
  use super::*;

  unsafe extern "C-unwind" {
    pub static mut re_match_object: Lisp_Object;        // regex-emacs.h 50
    pub static mut emacs_re_max_failures: ptrdiff_t;    // regex-emacs.h 53
    pub static mut emacs_re_safe_alloca: ptrdiff_t;     // regex-emacs.h 56

    /// C: regex-emacs.h 128-131: `extern const char *re_compile_pattern (const char
    /// *pattern, ptrdiff_t length, bool posix_backtracking, const char
    /// *whitespace_regexp, struct re_pattern_buffer *buffer);`
    // C: src/config.h 3642-3663 renames this identifier to `rpl_re_compile_pattern`, which is the symbol the reference object exports.
    #[link_name = "rpl_re_compile_pattern"]
    pub fn re_compile_pattern (pattern: *const c_char, length: ptrdiff_t,
			       posix_backtracking: bool, whitespace_regexp: *const c_char,
			       buffer: *mut re_pattern_buffer) -> *const c_char;
    /// C: regex-emacs.h 139-142: `extern ptrdiff_t re_search (struct re_pattern_buffer
    /// *buffer, const char *string, ptrdiff_t length, ptrdiff_t start, ptrdiff_t range,
    /// struct re_registers *regs);`
    // C: src/config.h 3642-3663 renames this identifier to `rpl_re_search`, which is the symbol the reference object exports.
    #[link_name = "rpl_re_search"]
    pub fn re_search (buffer: *mut re_pattern_buffer, string: *const c_char,
		      length: ptrdiff_t, start: ptrdiff_t, range: ptrdiff_t,
		      regs: *mut re_registers) -> ptrdiff_t;
    /// C: regex-emacs.h 147-152: `extern ptrdiff_t re_search_2 (struct re_pattern_buffer
    /// *buffer, const char *string1, ptrdiff_t length1, const char *string2,
    /// ptrdiff_t length2, ptrdiff_t start, ptrdiff_t range, struct re_registers *regs,
    /// ptrdiff_t stop);`
    // C: src/config.h 3642-3663 renames this identifier to `rpl_re_search_2`, which is the symbol the reference object exports.
    #[link_name = "rpl_re_search_2"]
    pub fn re_search_2 (buffer: *mut re_pattern_buffer, string1: *const c_char,
			length1: ptrdiff_t, string2: *const c_char, length2: ptrdiff_t,
			start: ptrdiff_t, range: ptrdiff_t, regs: *mut re_registers,
			stop: ptrdiff_t) -> ptrdiff_t;
    /// C: regex-emacs.h 157-161: `extern ptrdiff_t re_match_2 (struct re_pattern_buffer
    /// *buffer, const char *string1, ptrdiff_t length1, const char *string2,
    /// ptrdiff_t length2, ptrdiff_t start, struct re_registers *regs, ptrdiff_t stop);`
    // C: src/config.h 3642-3663 renames this identifier to `rpl_re_match_2`, which is the symbol the reference object exports.
    #[link_name = "rpl_re_match_2"]
    pub fn re_match_2 (buffer: *mut re_pattern_buffer, string1: *const c_char,
		       length1: ptrdiff_t, string2: *const c_char, length2: ptrdiff_t,
		       start: ptrdiff_t, regs: *mut re_registers, stop: ptrdiff_t) -> ptrdiff_t;
    /// C: regex-emacs.h 176-179: `extern void re_set_registers (struct re_pattern_buffer
    /// *buffer, struct re_registers *regs, ptrdiff_t num_regs, ptrdiff_t *starts,
    /// ptrdiff_t *ends);`
    // C: src/config.h 3642-3663 renames this identifier to `rpl_re_set_registers`, which is the symbol the reference object exports.
    #[link_name = "rpl_re_set_registers"]
    pub fn re_set_registers (buffer: *mut re_pattern_buffer, regs: *mut re_registers,
			     num_regs: ptrdiff_t, starts: *mut ptrdiff_t,
			     ends: *mut ptrdiff_t);
    /// C: regex-emacs.h 194: `extern bool re_iswctype (int ch, re_wctype_t cc);`
    pub fn re_iswctype (ch: c_int, cc: re_wctype_t) -> bool;
    /// C: regex-emacs.h 195-196: `extern re_wctype_t re_wctype_parse (const unsigned char
    /// **strp, ptrdiff_t limit);`
    pub fn re_wctype_parse (strp: *mut *const u8, limit: ptrdiff_t) -> re_wctype_t;
    /// C: regex-emacs.h 199: `extern void print_compiled_pattern (FILE *dest, struct
    /// re_pattern_buffer *bufp);` (under `ENABLE_CHECKING`; the checking-only entry).
    #[cfg (enable_checking)]
    pub fn print_compiled_pattern (dest: *mut FILE, bufp: *mut re_pattern_buffer);
  }
}
pub use self::regex_emacs_decls::*;
