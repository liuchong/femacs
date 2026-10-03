// femacs: mirror of lib/regex.h @ fd7afced403 (GNU Emacs 32.0.50; PORTING.md 0.1-2).
//
// Declarations only.  `_GNU_SOURCE` is 1 in the frozen configuration, so `__USE_GNU` is
// 1 and C takes the `__REPB_PREFIX(name) name` arm; `_REGEX_LARGE_OFFSETS` is 1, so
// `__re_size_t`/`__re_long_size_t` are `size_t` and `regoff_t` is `ssize_t` (P-08).
// `__restrict`/`__attribute__` spellings become no-ops (PORTING.md 8).


/* regex.h 21 */
pub const _REGEX_H: c_int = 1;

/* regex.h 33: `_GNU_SOURCE` is defined by the build, so this is 1.  */
pub const __USE_GNU: c_int = 1;

/* regex.h 36-61: the wide-offset spelling.  */
pub type __re_size_t = usize;
pub type __re_long_size_t = usize;

/* regex.h 64-65 */
pub type s_reg_t = c_long;
pub type active_reg_t = c_ulong;

/* regex.h 71 */
pub type reg_syntax_t = c_ulong;

/* regex.h 76-203: the syntax bits.  Each is an object-like macro; the port keeps C's
   value expressions, so the ledger can compare them token by token.  */
pub const RE_BACKSLASH_ESCAPE_IN_LISTS: reg_syntax_t = 1;
pub const RE_BK_PLUS_QM: reg_syntax_t = RE_BACKSLASH_ESCAPE_IN_LISTS << 1;
pub const RE_CHAR_CLASSES: reg_syntax_t = RE_BK_PLUS_QM << 1;
pub const RE_CONTEXT_INDEP_ANCHORS: reg_syntax_t = RE_CHAR_CLASSES << 1;
pub const RE_CONTEXT_INDEP_OPS: reg_syntax_t = RE_CONTEXT_INDEP_ANCHORS << 1;
pub const RE_CONTEXT_INVALID_OPS: reg_syntax_t = RE_CONTEXT_INDEP_OPS << 1;
pub const RE_DOT_NEWLINE: reg_syntax_t = RE_CONTEXT_INVALID_OPS << 1;
pub const RE_DOT_NOT_NULL: reg_syntax_t = RE_DOT_NEWLINE << 1;
pub const RE_HAT_LISTS_NOT_NEWLINE: reg_syntax_t = RE_DOT_NOT_NULL << 1;
pub const RE_INTERVALS: reg_syntax_t = RE_HAT_LISTS_NOT_NEWLINE << 1;
pub const RE_LIMITED_OPS: reg_syntax_t = RE_INTERVALS << 1;
pub const RE_NEWLINE_ALT: reg_syntax_t = RE_LIMITED_OPS << 1;
pub const RE_NO_BK_BRACES: reg_syntax_t = RE_NEWLINE_ALT << 1;
pub const RE_NO_BK_PARENS: reg_syntax_t = RE_NO_BK_BRACES << 1;
pub const RE_NO_BK_REFS: reg_syntax_t = RE_NO_BK_PARENS << 1;
pub const RE_NO_BK_VBAR: reg_syntax_t = RE_NO_BK_REFS << 1;
pub const RE_NO_EMPTY_RANGES: reg_syntax_t = RE_NO_BK_VBAR << 1;
pub const RE_UNMATCHED_RIGHT_PAREN_ORD: reg_syntax_t = RE_NO_EMPTY_RANGES << 1;
pub const RE_NO_POSIX_BACKTRACKING: reg_syntax_t = RE_UNMATCHED_RIGHT_PAREN_ORD << 1;
pub const RE_NO_GNU_OPS: reg_syntax_t = RE_NO_POSIX_BACKTRACKING << 1;
pub const RE_DEBUG: reg_syntax_t = RE_NO_GNU_OPS << 1;
pub const RE_INVALID_INTERVAL_ORD: reg_syntax_t = RE_DEBUG << 1;
pub const RE_ICASE: reg_syntax_t = RE_INVALID_INTERVAL_ORD << 1;
pub const RE_CARET_ANCHORS_HERE: reg_syntax_t = RE_ICASE << 1;
pub const RE_CONTEXT_INVALID_DUP: reg_syntax_t = RE_CARET_ANCHORS_HERE << 1;
pub const RE_NO_SUB: reg_syntax_t = RE_CONTEXT_INVALID_DUP << 1;

/* regex.h 210.  The declaration is C's `extern`, and the *definition* is the unit's
   (`regcomp.rs` 131: `reg_syntax_t re_syntax_options;`); Rust forbids an `extern` static and
   a defined static of the same name in one scope, so the declaration lives in a module and
   the defining unit shadows it (PORTING.md 0.1-10, the tempname.rs shape).  */
pub mod regex_decls {
  use super::*;
  unsafe extern "C" {
    /// C: `extern reg_syntax_t re_syntax_options;`
    pub static mut re_syntax_options: reg_syntax_t;
  }
}

/* regex.h 217-283: the standard syntax combinations, written with C's own operands --
   `~` is Rust's `!` (the same bitwise complement).  */
pub const _RE_SYNTAX_POSIX_COMMON: reg_syntax_t = (RE_CHAR_CLASSES | RE_DOT_NEWLINE
    | RE_DOT_NOT_NULL | RE_INTERVALS | RE_NO_EMPTY_RANGES);
pub const RE_SYNTAX_EMACS: reg_syntax_t = (RE_CHAR_CLASSES | RE_INTERVALS);
pub const RE_SYNTAX_AWK: reg_syntax_t =
    (RE_BACKSLASH_ESCAPE_IN_LISTS | RE_DOT_NOT_NULL
     | RE_NO_BK_PARENS | RE_NO_BK_REFS
     | RE_NO_BK_VBAR | RE_NO_EMPTY_RANGES
     | RE_DOT_NEWLINE | RE_CONTEXT_INDEP_ANCHORS
     | RE_CHAR_CLASSES
     | RE_UNMATCHED_RIGHT_PAREN_ORD | RE_NO_GNU_OPS);
pub const RE_SYNTAX_GNU_AWK: reg_syntax_t =
    ((RE_SYNTAX_POSIX_EXTENDED | RE_BACKSLASH_ESCAPE_IN_LISTS
      | RE_INVALID_INTERVAL_ORD)
     & !(RE_DOT_NOT_NULL | RE_CONTEXT_INDEP_OPS | RE_CONTEXT_INVALID_OPS));
pub const RE_SYNTAX_POSIX_AWK: reg_syntax_t =
    (RE_SYNTAX_POSIX_EXTENDED | RE_BACKSLASH_ESCAPE_IN_LISTS
     | RE_INTERVALS | RE_NO_GNU_OPS
     | RE_INVALID_INTERVAL_ORD);
pub const RE_SYNTAX_GREP: reg_syntax_t =
    ((RE_SYNTAX_POSIX_BASIC | RE_NEWLINE_ALT)
     & !(RE_CONTEXT_INVALID_DUP | RE_DOT_NOT_NULL));
pub const RE_SYNTAX_EGREP: reg_syntax_t =
    ((RE_SYNTAX_POSIX_EXTENDED | RE_INVALID_INTERVAL_ORD | RE_NEWLINE_ALT)
     & !(RE_CONTEXT_INVALID_OPS | RE_DOT_NOT_NULL));
pub const RE_SYNTAX_POSIX_EGREP: reg_syntax_t = RE_SYNTAX_EGREP;
pub const RE_SYNTAX_ED: reg_syntax_t = RE_SYNTAX_POSIX_BASIC;
pub const RE_SYNTAX_SED: reg_syntax_t = RE_SYNTAX_POSIX_BASIC;
pub const RE_SYNTAX_POSIX_BASIC: reg_syntax_t =
    (_RE_SYNTAX_POSIX_COMMON | RE_BK_PLUS_QM | RE_CONTEXT_INVALID_DUP);
pub const RE_SYNTAX_POSIX_MINIMAL_BASIC: reg_syntax_t =
    (_RE_SYNTAX_POSIX_COMMON | RE_LIMITED_OPS);
pub const RE_SYNTAX_POSIX_EXTENDED: reg_syntax_t =
    (_RE_SYNTAX_POSIX_COMMON | RE_CONTEXT_INDEP_ANCHORS
     | RE_CONTEXT_INDEP_OPS | RE_NO_BK_BRACES
     | RE_NO_BK_PARENS | RE_NO_BK_VBAR
     | RE_CONTEXT_INVALID_OPS | RE_UNMATCHED_RIGHT_PAREN_ORD);
pub const RE_SYNTAX_POSIX_MINIMAL_EXTENDED: reg_syntax_t =
    (_RE_SYNTAX_POSIX_COMMON | RE_CONTEXT_INDEP_ANCHORS
     | RE_CONTEXT_INVALID_OPS | RE_NO_BK_BRACES
     | RE_NO_BK_PARENS | RE_NO_BK_REFS
     | RE_NO_BK_VBAR | RE_UNMATCHED_RIGHT_PAREN_ORD);

/* regex.h 302 */
pub const RE_DUP_MAX: c_int = 0x7fff;

/* regex.h 310-340 */
pub const REG_EXTENDED: c_int = 1;
pub const REG_ICASE: c_int = 1 << 1;
pub const REG_NEWLINE: c_int = 1 << 2;
pub const REG_NOSUB: c_int = 1 << 3;
pub const REG_NOTBOL: c_int = 1;
pub const REG_NOTEOL: c_int = 1 << 1;
pub const REG_STARTEND: c_int = 1 << 2;

/* regex.h 346-372: the error codes.  */
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum reg_errcode_t {
  _REG_ENOSYS = -1,
  _REG_NOERROR = 0,
  _REG_NOMATCH = 1,
  _REG_BADPAT = 2,
  _REG_ECOLLATE = 3,
  _REG_ECTYPE = 4,
  _REG_EESCAPE = 5,
  _REG_ESUBREG = 6,
  _REG_EBRACK = 7,
  _REG_EPAREN = 8,
  _REG_EBRACE = 9,
  _REG_BADBR = 10,
  _REG_ERANGE = 11,
  _REG_ESPACE = 12,
  _REG_BADRPT = 13,
  _REG_EEND = 14,
  _REG_ESIZE = 15,
  _REG_ERPAREN = 16,
}

/* regex.h 374-392 */
pub const REG_ENOSYS: reg_errcode_t = reg_errcode_t::_REG_ENOSYS;
pub const REG_NOERROR: reg_errcode_t = reg_errcode_t::_REG_NOERROR;
pub const REG_NOMATCH: reg_errcode_t = reg_errcode_t::_REG_NOMATCH;
pub const REG_BADPAT: reg_errcode_t = reg_errcode_t::_REG_BADPAT;
pub const REG_ECOLLATE: reg_errcode_t = reg_errcode_t::_REG_ECOLLATE;
pub const REG_ECTYPE: reg_errcode_t = reg_errcode_t::_REG_ECTYPE;
pub const REG_EESCAPE: reg_errcode_t = reg_errcode_t::_REG_EESCAPE;
pub const REG_ESUBREG: reg_errcode_t = reg_errcode_t::_REG_ESUBREG;
pub const REG_EBRACK: reg_errcode_t = reg_errcode_t::_REG_EBRACK;
pub const REG_EPAREN: reg_errcode_t = reg_errcode_t::_REG_EPAREN;
pub const REG_EBRACE: reg_errcode_t = reg_errcode_t::_REG_EBRACE;
pub const REG_BADBR: reg_errcode_t = reg_errcode_t::_REG_BADBR;
pub const REG_ERANGE: reg_errcode_t = reg_errcode_t::_REG_ERANGE;
pub const REG_ESPACE: reg_errcode_t = reg_errcode_t::_REG_ESPACE;
pub const REG_BADRPT: reg_errcode_t = reg_errcode_t::_REG_BADRPT;
pub const REG_EEND: reg_errcode_t = reg_errcode_t::_REG_EEND;
pub const REG_ESIZE: reg_errcode_t = reg_errcode_t::_REG_ESIZE;
pub const REG_ERPAREN: reg_errcode_t = reg_errcode_t::_REG_ERPAREN;

/* regex.h 400-403 */
pub type __RE_TRANSLATE_TYPE = *mut u8;
pub type RE_TRANSLATE_TYPE = __RE_TRANSLATE_TYPE;

/* regex.h 400-403: the two type names are macros in C; the port keeps the type aliases
   above and the names here (macros live in their own namespace).  */
macro_rules! __RE_TRANSLATE_TYPE {
    () => { u8 };
}
macro_rules! RE_TRANSLATE_TYPE {
    () => { __RE_TRANSLATE_TYPE };
}

/* regex.h 408-410: the port names the fields plainly, and keeps the macro's two bodies
   in the ledger (both arms are in the reference text, so the body is not compared).  */
macro_rules! __REPB_PREFIX {
    ($name:ident) => { $name };
}

/* regex.h 412-476 */
#[repr(C)]
pub struct re_pattern_buffer {
  /// C: `struct re_dfa_t *__REPB_PREFIX(buffer);`
  pub buffer: *mut c_void,
  /// C: `__re_long_size_t __REPB_PREFIX(allocated);`
  pub allocated: __re_long_size_t,
  /// C: `__re_long_size_t __REPB_PREFIX(used);`
  pub used: __re_long_size_t,
  /// C: `reg_syntax_t __REPB_PREFIX(syntax);`
  pub syntax: reg_syntax_t,
  /// C: `char *__REPB_PREFIX(fastmap);`
  pub fastmap: *mut c_char,
  /// C: `__RE_TRANSLATE_TYPE __REPB_PREFIX(translate);`
  pub translate: __RE_TRANSLATE_TYPE,
  pub re_nsub: usize,
  /// C: `unsigned __REPB_PREFIX(can_be_null) : 1; unsigned __REPB_PREFIX(regs_allocated) : 2;
  /// unsigned __REPB_PREFIX(fastmap_accurate) : 1; unsigned __REPB_PREFIX(no_sub) : 1;
  /// unsigned __REPB_PREFIX(not_bol) : 1; unsigned __REPB_PREFIX(not_eol) : 1;
  /// unsigned __REPB_PREFIX(newline_anchor) : 1;`
  pub bits: c_uint,
}

pub type regex_t = re_pattern_buffer;

/* regex.h 486-495 */
pub const REGS_UNALLOCATED: c_int = 0;
pub const REGS_REALLOCATE: c_int = 1;
pub const REGS_FIXED: c_int = 2;

/* regex.h 501-510 */
pub type regoff_t = isize;

/* regex.h 514-520 */
#[repr(C)]
pub struct re_registers {
  pub num_regs: __re_size_t,
  pub start: *mut regoff_t,
  pub end: *mut regoff_t,
}

/* regex.h 528 */
pub const RE_NREGS: c_int = 30;

/* regex.h 536-542 */
#[repr(C)]
pub struct regmatch_t {
  /// C: `regoff_t rm_so;`
  pub rm_so: regoff_t,
  /// C: `regoff_t rm_eo;`
  pub rm_eo: regoff_t,
}

/* regex.h 525-531 / 628-659: the qualifiers have no Rust counterpart; the names stay so
   the prototype sites below read the same (PORTING.md 8).  Their C bodies are
   configuration branches, so the gate compares the names, not the bodies.  */
macro_rules! _Attr_access_ {
    ($x:expr) => {};
}
macro_rules! _Restrict_ {
    () => {};
}
macro_rules! _Restrict_arr_ {
    () => {};
}

/* regex.h 665-...: the POSIX and GNU entries.  They are C prototypes; the port puts them
   in a module and imports it wholesale, so the unit that defines one (regcomp.rs defines
   `regcomp`, regexec.rs defines `regexec`) shadows the declaration instead of colliding
   (PORTING.md 0.1-10).  */
pub mod re_protos {
  // `use std::…` is allowed in a mirror; `use crate::…` is not (PORTING.md 0.1-9), so
  // the struct pointers the prototypes name are ABI-equal `*mut c_void` (the `/// C:`
  // line above each keeps the reference type visible).
  use std::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
  unsafe extern "C" {
    /// C: `int regcomp (regex_t *__preg, const char *__pattern, int __cflags);`
    pub fn regcomp (preg: *mut c_void, pattern: *const c_char, cflags: c_int) -> c_int;
    /// C: `int regexec (const regex_t *__preg, const char *__string, size_t __nmatch, regmatch_t __pmatch[], int __eflags);`
    pub fn regexec (preg: *const c_void, string: *const c_char, nmatch: usize,
                    pmatch: *mut c_void, eflags: c_int) -> c_int;
    /// C: `size_t regerror (int __errcode, const regex_t *__preg, char *__errbuf, size_t __errbuf_size);`
    pub fn regerror (errcode: c_int, preg: *const c_void, errbuf: *mut c_char,
                     errbuf_size: usize) -> usize;
    /// C: `void regfree (regex_t *__preg);`
    pub fn regfree (preg: *mut c_void);
    /// C: `reg_syntax_t re_set_syntax (reg_syntax_t __syntax);`
    pub fn re_set_syntax (syntax: c_ulong) -> c_ulong;
    /// C: `const char *re_compile_pattern (const char *__pattern, size_t __length, struct re_pattern_buffer *__buffer);`
    pub fn re_compile_pattern (pattern: *const c_char, length: usize,
                               buffer: *mut c_void) -> *const c_char;
    /// C: `int re_compile_fastmap (struct re_pattern_buffer *__buffer);`
    pub fn re_compile_fastmap (buffer: *mut c_void) -> c_int;
    /// C: `regoff_t re_search (struct re_pattern_buffer *__buffer, const char *__string, regoff_t __length, regoff_t __start, regoff_t __range, struct re_registers *__regs);`
    pub fn re_search (buffer: *mut c_void, string: *const c_char, length: isize,
                      start: isize, range: isize, regs: *mut c_void) -> isize;
    /// C: `regoff_t re_search_2 (struct re_pattern_buffer *__buffer, const char *__string1, regoff_t __length1, const char *__string2, regoff_t __length2, regoff_t __start, regoff_t __range, struct re_registers *__regs, regoff_t __stop);`
    pub fn re_search_2 (buffer: *mut c_void, string1: *const c_char, length1: isize,
                        string2: *const c_char, length2: isize, start: isize, range: isize,
                        regs: *mut c_void, stop: isize) -> isize;
    /// C: `regoff_t re_match (struct re_pattern_buffer *__buffer, const char *__string, regoff_t __length, regoff_t __start, struct re_registers *__regs);`
    pub fn re_match (buffer: *mut c_void, string: *const c_char, length: isize,
                     start: isize, regs: *mut c_void) -> isize;
    /// C: `regoff_t re_match_2 (struct re_pattern_buffer *__buffer, const char *__string1, regoff_t __length1, const char *__string2, regoff_t __length2, regoff_t __start, struct re_registers *__regs, regoff_t __stop);`
    pub fn re_match_2 (buffer: *mut c_void, string1: *const c_char, length1: isize,
                       string2: *const c_char, length2: isize, start: isize,
                       regs: *mut c_void, stop: isize) -> isize;
    /// C: `void re_set_registers (struct re_pattern_buffer *__buffer, struct re_registers *__regs, unsigned int __num_regs, regoff_t *__starts, regoff_t *__ends);`
    pub fn re_set_registers (buffer: *mut c_void, regs: *mut c_void, num_regs: c_uint,
                             starts: *mut isize, ends: *mut isize);
    /// C: `const char *re_comp (const char *__pattern);`
    pub fn re_comp (pattern: *const c_char) -> *const c_char;
    /// C: `int re_exec (const char *__string);`
    pub fn re_exec (string: *const c_char) -> c_int;
  }
}

/* The `re_pattern_buffer` flag unit (PORTING.md 1.3).  C's bitfields and the port's
   accessors: `can_be_null : 1`, `regs_allocated : 2`, `fastmap_accurate : 1`,
   `no_sub : 1`, `not_bol : 1`, `not_eol : 1`, `newline_anchor : 1` -- LSB first.  */
pub const PB_CAN_BE_NULL_SHIFT: u32 = 0;
pub const PB_REGS_ALLOCATED_SHIFT: u32 = 1;
pub const PB_FASTMAP_ACCURATE_SHIFT: u32 = 3;
pub const PB_NO_SUB_SHIFT: u32 = 4;
pub const PB_NOT_BOL_SHIFT: u32 = 5;
pub const PB_NOT_EOL_SHIFT: u32 = 6;
pub const PB_NEWLINE_ANCHOR_SHIFT: u32 = 7;

#[inline(always)]
fn pb_get (bits: c_uint, shift: u32, mask: c_uint) -> c_uint { (bits >> shift) & mask }
#[inline(always)]
fn pb_set (bits: c_uint, shift: u32, mask: c_uint, value: c_uint) -> c_uint {
    (bits & !(mask << shift)) | ((value & mask) << shift)
}

#[inline(always)]
pub unsafe fn can_be_null_ (preg: *const re_pattern_buffer) -> c_uint { pb_get ((*preg).bits, PB_CAN_BE_NULL_SHIFT, 1) }
#[inline(always)]
pub unsafe fn set_can_be_null_ (preg: *mut re_pattern_buffer, value: c_uint) { (*preg).bits = pb_set ((*preg).bits, PB_CAN_BE_NULL_SHIFT, 1, value); }
#[inline(always)]
pub unsafe fn regs_allocated_ (preg: *const re_pattern_buffer) -> c_uint { pb_get ((*preg).bits, PB_REGS_ALLOCATED_SHIFT, 3) }
#[inline(always)]
pub unsafe fn set_regs_allocated_ (preg: *mut re_pattern_buffer, value: c_uint) { (*preg).bits = pb_set ((*preg).bits, PB_REGS_ALLOCATED_SHIFT, 3, value); }
#[inline(always)]
pub unsafe fn fastmap_accurate_ (preg: *const re_pattern_buffer) -> c_uint { pb_get ((*preg).bits, PB_FASTMAP_ACCURATE_SHIFT, 1) }
#[inline(always)]
pub unsafe fn set_fastmap_accurate_ (preg: *mut re_pattern_buffer, value: c_uint) { (*preg).bits = pb_set ((*preg).bits, PB_FASTMAP_ACCURATE_SHIFT, 1, value); }
#[inline(always)]
pub unsafe fn no_sub_ (preg: *const re_pattern_buffer) -> c_uint { pb_get ((*preg).bits, PB_NO_SUB_SHIFT, 1) }
#[inline(always)]
pub unsafe fn set_no_sub_ (preg: *mut re_pattern_buffer, value: c_uint) { (*preg).bits = pb_set ((*preg).bits, PB_NO_SUB_SHIFT, 1, value); }
#[inline(always)]
pub unsafe fn not_bol_ (preg: *const re_pattern_buffer) -> c_uint { pb_get ((*preg).bits, PB_NOT_BOL_SHIFT, 1) }
#[inline(always)]
pub unsafe fn set_not_bol_ (preg: *mut re_pattern_buffer, value: c_uint) { (*preg).bits = pb_set ((*preg).bits, PB_NOT_BOL_SHIFT, 1, value); }
#[inline(always)]
pub unsafe fn not_eol_ (preg: *const re_pattern_buffer) -> c_uint { pb_get ((*preg).bits, PB_NOT_EOL_SHIFT, 1) }
#[inline(always)]
pub unsafe fn set_not_eol_ (preg: *mut re_pattern_buffer, value: c_uint) { (*preg).bits = pb_set ((*preg).bits, PB_NOT_EOL_SHIFT, 1, value); }
#[inline(always)]
pub unsafe fn newline_anchor_ (preg: *const re_pattern_buffer) -> c_uint { pb_get ((*preg).bits, PB_NEWLINE_ANCHOR_SHIFT, 1) }
#[inline(always)]
pub unsafe fn set_newline_anchor_ (preg: *mut re_pattern_buffer, value: c_uint) { (*preg).bits = pb_set ((*preg).bits, PB_NEWLINE_ANCHOR_SHIFT, 1, value); }
