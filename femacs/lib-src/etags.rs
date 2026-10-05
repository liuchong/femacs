// femacs: port of lib-src/etags.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names, the same
// `main`.  This is a program of the C tree's lib-src directory (batch F), built by the frozen
// configuration (the reference `lib-src/Makefile` installs it) and linked against the port's
// gnulib mirrors and libSystem only -- it is not a translation unit of the editor core, so it
// does not include the `lisp.h` mirror.
//
// **Dead text (P-08)**, recorded here and in docs/ports/etags.md, not transcribed: `DEBUG`
// (C 81-86) sits in its `#else` arm, so the macro is `false`; `MSDOS` (C 90-96) likewise;
// `WINDOWSNT` (C 98-105) is undefined, so that block (`<direct.h>`, the three `#undef`s and
// `#define O_CLOEXEC O_NOINHERIT`) is dead; `GNUC_PREREQ (14, 0, 0)` (C 139-141) is a
// preprocessor-only arm.  The file's conditions are decided by
// `python3 tools/check-cond-eval.py lib-src/etags.c` (18 conditions, 0 unknown,
// 0 disagreements [VERIFIED]), and every later dead arm is recorded at its window.
//
// **Use-site handling** (P-26/P-72 (3)), because C names these from headers the port has no
// mirror for: `mempcpy` under its own name (the generated `<string.h>`'s `rpl_mempcpy` rename is
// inside `# if 0`; PORTING.md
// 10.13), `c_strcasecmp`/`c_strncasecmp` (`<c-strcase.h>`), `assert` (Rust's `assert!`, the
// `lib/mini-gmp.rs` 8 precedent: `NDEBUG` is undefined in the frozen build).  `<limits.h>`'s
// `CHAR_MAX` used to be declared here too; the `lib/include/limits.in.rs` mirror now carries
// it (measured 127 on this target with clang), so the use-site copy is gone and the option
// table reads the mirror's constant.
//
// `ATTRIBUTE_*`, `_Noreturn` and `NO_INLINE` have no Rust counterpart (PORTING.md 8/P-13);
// the C names stay in the comments at their definitions, and `_Noreturn` is the port's `-> !`.
//
// **Forward declarations dropped** (PORTING.md 0.1-12): C 334-417 declares the language
// functions, the helpers and the allocators ahead of their definitions; Rust items are
// order-independent, so those lines are gone and every definition stays where C has it.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]

use std::ffi::{c_char, c_double, c_int, c_long, c_short, c_uchar, c_uint, c_ulong,
              c_ushort, c_void, CStr};

// C gets `va_list` from <stdarg.h>; the port's equivalent is core::ffi::VaList (P-04).
use core::ffi::VaList;

/// `<stddef.h>`: `typedef ... ptrdiff_t;` -- the frozen target's 8-byte signed type
/// (measured with clang: `sizeof (ptrdiff_t)` is 8).  This program has no `lisp.h` mirror to
/// take it from.
pub type ptrdiff_t = isize;
/// `<stddef.h>`'s `size_t` (measured: 8 bytes).
pub type size_t = usize;
/// `<stdint.h>`'s `intmax_t` (measured: 8 bytes).
pub type intmax_t = i64;
/// `<stdint.h>`'s `uintmax_t` (measured: 8 bytes).
pub type uintmax_t = u64;

include!("../src/include/config.rs");        // C 88: `#include <config.h>`
include!("../lib/include/attribute.rs");     // C 107: `#include <attribute.h>`
include!("../lib/include/stdint.in.rs");     // C 108: `#include <inttypes.h>`
include!("../lib/include/limits.in.rs");     // C 109: `#include <limits.h>`
include!("../lib/include/unistd.in.rs");     // C 110: `#include <unistd.h>`
// C 111's `#include <stdarg.h>` has no mirror: Rust spells the C variadic list as `VaList`.
include!("../lib/include/stdcountof.rs");    // C 113: `#include <stdcountof.h>`
include!("../lib/include/stdlib.rs");        // C 114: `#include <stdlib.h>`
include!("../lib/include/string.in.rs");        // C 115: `#include <string.h>`
include!("../src/include/sysstdio.rs");      // C 116: `#include <sysstdio.h>`
include!("../lib/include/errno.rs");         // C 117: `#include <errno.h>`
include!("../lib/include/fcntl.in.rs");      // C 118: `#include <fcntl.h>`
include!("../lib/include/binary-io.rs");     // C 119: `#include <binary-io.h>`
include!("../lib/include/verify.rs");        // C 122: `#include <verify.h>`
include!("../lib/include/c-ctype.rs");       // C 123: `#include <c-ctype.h>`
include!("../lib/include/getopt.in.rs");     // C 127: `#include <getopt.h>`
include!("../lib/include/regex.rs");         // C 128: `#include <regex.h>`
// The regex.h prototypes live in the mirror's `re_protos` module; it prescribes a wholesale
// import so that a unit defining one shadows the declaration (PORTING.md 0.1-10).
use re_protos::*;

/// C 81-86: `#ifdef DEBUG` is false in the frozen configuration, so C's `#else` arm stands:
/// `# define DEBUG false`.  It is read at C 3431/3481/8207 as a value.
pub const DEBUG: bool = false;

/// C 90-96: `#ifdef MSDOS` is false here, so C's `#else` arm stands: `# define MSDOS false`.
/// It is read at C 1465/1566/1727 and in the `#if MSDOS || defined (DOS_NT)` conditions.
pub const MSDOS: bool = false;

/// C 133-135: `#ifndef MERCURY_HEURISTICS_RATIO` / `# define MERCURY_HEURISTICS_RATIO 0.5`.
pub const MERCURY_HEURISTICS_RATIO: f32 = 0.5;

/* C's `mempcpy`, under its own name: the generated `<string.h>`'s `rpl_mempcpy` rename sits
   inside `# if 0` (status 3.205), and `lib/mempcpy.rs` emits `mempcpy`.  The reference object
   has no mempcpy reference at all (clang inlines the call); the port keeps the call.  */
unsafe extern "C" {
  /* void *mempcpy (void *dest, const void *src, size_t n);  */
  pub fn mempcpy (dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
}

/* `<stdlib.h>`: `void exit (int);` -- no mirror carries it yet (the emacsclient.rs / sysdep.rs
   precedent keeps it at the use site), and `<config.h>`'s `EXIT_SUCCESS` is in the stdlib
   mirror.  */
unsafe extern "C" {
  pub fn exit (status: c_int) -> !;
}

/* `<c-strcase.h>` has no mirror yet; `lib/c-strcasecmp.rs` and `lib/c-strncasecmp.rs` own
   the two definitions.  */
unsafe extern "C" {
  /* int c_strcasecmp (const char *s1, const char *s2);  */
  pub fn c_strcasecmp (s1: *const c_char, s2: *const c_char) -> c_int;
  /* int c_strncasecmp (const char *s1, const char *s2, size_t n);  */
  pub fn c_strncasecmp (s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
}

/* COPY to DEST from SRC (containing LEN bytes), and append a NUL byte.  */
unsafe fn memcpyz (dest: *mut c_void, src: *const c_void, len: ptrdiff_t)
{
  let e: *mut c_char = mempcpy (dest, src, len as size_t) as *mut c_char;
  *e = 0;
}

unsafe fn strcaseeq (s: *const c_char, t: *const c_char) -> bool
{
  return c_strcasecmp (s, t) == 0;
}

unsafe fn strneq (s: *const c_char, t: *const c_char, n: size_t) -> bool
{
  return strncmp (s, t, n) == 0;
}

unsafe fn strncaseeq (s: *const c_char, t: *const c_char, n: size_t) -> bool
{
  return c_strncasecmp (s, t, n) == 0;
}

/* C is not in a name.  */
unsafe fn notinname (c: c_uchar) -> bool
{
  /* Look at make_tag before modifying!  */
  static table: [bool; UCHAR_MAX as usize + 1] = {
    let mut t = [false; UCHAR_MAX as usize + 1];
    t[b'\0' as usize] = true; t[b'\t' as usize] = true; t[b'\n' as usize] = true;
    t[b'\x0c' as usize] = true; t[b'\r' as usize] = true; t[b' ' as usize] = true;
    t[b'(' as usize] = true; t[b')' as usize] = true; t[b',' as usize] = true;
    t[b';' as usize] = true; t[b'=' as usize] = true;
    t
  };
  return table[c as usize];
}

/* C can start a token.  */
unsafe fn begtoken (c: c_uchar) -> bool
{
  static table: [bool; UCHAR_MAX as usize + 1] = {
    let mut t = [false; UCHAR_MAX as usize + 1];
    t[b'$' as usize] = true; t[b'@' as usize] = true;
    t[b'A' as usize] = true; t[b'B' as usize] = true; t[b'C' as usize] = true;
    t[b'D' as usize] = true; t[b'E' as usize] = true; t[b'F' as usize] = true;
    t[b'G' as usize] = true; t[b'H' as usize] = true; t[b'I' as usize] = true;
    t[b'J' as usize] = true; t[b'K' as usize] = true; t[b'L' as usize] = true;
    t[b'M' as usize] = true; t[b'N' as usize] = true; t[b'O' as usize] = true;
    t[b'P' as usize] = true; t[b'Q' as usize] = true; t[b'R' as usize] = true;
    t[b'S' as usize] = true; t[b'T' as usize] = true; t[b'U' as usize] = true;
    t[b'V' as usize] = true; t[b'W' as usize] = true; t[b'X' as usize] = true;
    t[b'Y' as usize] = true; t[b'Z' as usize] = true;
    t[b'_' as usize] = true;
    t[b'a' as usize] = true; t[b'b' as usize] = true; t[b'c' as usize] = true;
    t[b'd' as usize] = true; t[b'e' as usize] = true; t[b'f' as usize] = true;
    t[b'g' as usize] = true; t[b'h' as usize] = true; t[b'i' as usize] = true;
    t[b'j' as usize] = true; t[b'k' as usize] = true; t[b'l' as usize] = true;
    t[b'm' as usize] = true; t[b'n' as usize] = true; t[b'o' as usize] = true;
    t[b'p' as usize] = true; t[b'q' as usize] = true; t[b'r' as usize] = true;
    t[b's' as usize] = true; t[b't' as usize] = true; t[b'u' as usize] = true;
    t[b'v' as usize] = true; t[b'w' as usize] = true; t[b'x' as usize] = true;
    t[b'y' as usize] = true; t[b'z' as usize] = true;
    t[b'~' as usize] = true;
    t
  };
  return table[c as usize];
}

/* C can be in the middle of a token.  */
unsafe fn intoken (c: c_uchar) -> bool
{
  static table: [bool; UCHAR_MAX as usize + 1] = {
    let mut t = [false; UCHAR_MAX as usize + 1];
    t[b'$' as usize] = true;
    t[b'0' as usize] = true; t[b'1' as usize] = true; t[b'2' as usize] = true;
    t[b'3' as usize] = true; t[b'4' as usize] = true; t[b'5' as usize] = true;
    t[b'6' as usize] = true; t[b'7' as usize] = true; t[b'8' as usize] = true;
    t[b'9' as usize] = true;
    t[b'A' as usize] = true; t[b'B' as usize] = true; t[b'C' as usize] = true;
    t[b'D' as usize] = true; t[b'E' as usize] = true; t[b'F' as usize] = true;
    t[b'G' as usize] = true; t[b'H' as usize] = true; t[b'I' as usize] = true;
    t[b'J' as usize] = true; t[b'K' as usize] = true; t[b'L' as usize] = true;
    t[b'M' as usize] = true; t[b'N' as usize] = true; t[b'O' as usize] = true;
    t[b'P' as usize] = true; t[b'Q' as usize] = true; t[b'R' as usize] = true;
    t[b'S' as usize] = true; t[b'T' as usize] = true; t[b'U' as usize] = true;
    t[b'V' as usize] = true; t[b'W' as usize] = true; t[b'X' as usize] = true;
    t[b'Y' as usize] = true; t[b'Z' as usize] = true;
    t[b'_' as usize] = true;
    t[b'a' as usize] = true; t[b'b' as usize] = true; t[b'c' as usize] = true;
    t[b'd' as usize] = true; t[b'e' as usize] = true; t[b'f' as usize] = true;
    t[b'g' as usize] = true; t[b'h' as usize] = true; t[b'i' as usize] = true;
    t[b'j' as usize] = true; t[b'k' as usize] = true; t[b'l' as usize] = true;
    t[b'm' as usize] = true; t[b'n' as usize] = true; t[b'o' as usize] = true;
    t[b'p' as usize] = true; t[b'q' as usize] = true; t[b'r' as usize] = true;
    t[b's' as usize] = true; t[b't' as usize] = true; t[b'u' as usize] = true;
    t[b'v' as usize] = true; t[b'w' as usize] = true; t[b'x' as usize] = true;
    t[b'y' as usize] = true; t[b'z' as usize] = true;
    t
  };
  return table[c as usize];
}

/* C can end a token.  */
unsafe fn endtoken (c: c_uchar) -> bool
{
  static table: [bool; UCHAR_MAX as usize + 1] = {
    let mut t = [false; UCHAR_MAX as usize + 1];
    t[b'\0' as usize] = true; t[b'\t' as usize] = true; t[b'\n' as usize] = true;
    t[b'\r' as usize] = true; t[b' ' as usize] = true;
    t[b'!' as usize] = true; t[b'"' as usize] = true; t[b'#' as usize] = true;
    t[b'%' as usize] = true; t[b'&' as usize] = true; t[b'\'' as usize] = true;
    t[b'(' as usize] = true; t[b')' as usize] = true;
    t[b'*' as usize] = true; t[b'+' as usize] = true; t[b',' as usize] = true;
    t[b'-' as usize] = true; t[b'.' as usize] = true; t[b'/' as usize] = true;
    t[b':' as usize] = true; t[b';' as usize] = true;
    t[b'<' as usize] = true; t[b'=' as usize] = true; t[b'>' as usize] = true;
    t[b'?' as usize] = true; t[b'[' as usize] = true; t[b']' as usize] = true;
    t[b'^' as usize] = true;
    t[b'{' as usize] = true; t[b'|' as usize] = true; t[b'}' as usize] = true;
    t[b'~' as usize] = true;
    t
  };
  return table[c as usize];
}

/*
 *	xrnew -- reallocate storage
 *
 * SYNOPSIS:	void xrnew (OldPointer, ptrdiff_t n, int multiplier);
 */
/* C 241: `#define xrnew(op, n, m) ((op) = xnrealloc (op, n, (m) * sizeof *(op)))` -- the
   assignment expression is the macro body (PORTING.md 2.1), and `sizeof *(op)` is the size of
   the pointed-to type, which the generic helper below names.  */
unsafe fn __sizeof_deref<T> (_p: *mut T) -> ptrdiff_t
{
  return std::mem::size_of::<T> () as ptrdiff_t;
}
macro_rules! xrnew {
    ($op:expr, $n:expr, $m:expr) => {
        $op = xnrealloc ($op as *mut c_void, $n, ($m) * __sizeof_deref ($op)) as _
    };
}

/* C 243: `typedef void Lang_function (FILE *);` -- the parse-function type the language table
   holds.  C takes these functions' addresses, so the port declares them
   `unsafe extern "C" fn` (PORTING.md 1.4).  */
pub type Lang_function = unsafe extern "C" fn (*mut FILE);

/* C 245-249.  */
#[repr(C)]
pub struct compressor
{
  pub suffix: *const c_char,           /* file name suffix for this compressor */
  pub command: *const c_char,		/* takes one arg and decompresses to stdout */
}

/* C 251-260.  */
#[repr(C)]
pub struct language
{
  pub name: *const c_char,             /* language name */
  pub help: *const c_char, 		/* detailed help for the language */
  pub function: Option<Lang_function>,	/* parse function */
  pub suffixes: *const *const c_char,        /* name suffixes of this language's files */
  pub filenames: *const *const c_char,       /* names of this language's files */
  pub interpreters: *const *const c_char,    /* interpreters for this language */
  pub metasource: bool,		/* source used to generate other sources */
}

/* C 262-273.  The C tag is `struct fdesc`, which the code then names `fdesc`.  C copies this
   record as a whole (`*fdhead = *curfdp` in `readline`), so the port derives `Copy` (which
   does not change the `#[repr(C)]` layout).  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct fdesc
{
  pub next: *mut fdesc,		/* for the linked list */
  pub infname: *mut c_char,		/* uncompressed input file name */
  pub infabsname: *mut c_char,		/* absolute uncompressed input file name */
  pub infabsdir: *mut c_char,		/* absolute dir of input file */
  pub taggedfname: *mut c_char,		/* file name to write in tagfile */
  pub lang: *mut language,		/* language of file */
  pub prop: *mut c_char,			/* file properties to write in tagfile */
  pub usecharno: bool,		/* etags tags shall contain char number */
  pub written: bool,			/* entry written in the tags file */
}

/* C 275-286.  The C tag is `struct node_st`, which the code then names `node`.  */
#[repr(C)]
pub struct node
{				/* sorting structure */
  pub left: *mut node, pub right: *mut node,	/* left and right sons */
  pub fdp: *mut fdesc,			/* description of file to whom tag belongs */
  pub name: *mut c_char, 			/* tag name */
  pub regex: *mut c_char,			/* search regexp */
  pub valid: bool,			/* write this tag on the tag file */
  pub is_func: bool,			/* function tag: use regexp in CTAGS mode */
  pub been_warned: bool,		/* warning already given for duplicated tag */
  pub lno: intmax_t,			/* line number tag is on */
  pub cno: intmax_t,			/* character number line starts on */
}

/*
 * A `linebuffer' is a structure which holds a line of text.
 * `readline_internal' reads a line from a stream into a linebuffer
 * and works regardless of the length of the line.
 * SIZE is the size of BUFFER, LEN is the length of the string in
 * BUFFER after readline reads it.
 */
/* C 295-300.  */
#[repr(C)]
pub struct linebuffer
{
  pub size: ptrdiff_t,
  pub len: ptrdiff_t,
  pub buffer: *mut c_char,
}

/* Used to support mixing of --lang and file names.  C 303-315; the anonymous enum's members
   are the constants below.  */
pub const at_language: c_int = 0;		/* a language specification */
pub const at_regexp: c_int = 1;			/* a regular expression */
pub const at_filename: c_int = 2;		/* a file name */
pub const at_stdin: c_int = 3;			/* read from stdin here */
pub const at_end: c_int = 4;			/* stop parsing the list */
#[repr(C)]
pub struct argument
{
  pub arg_type: c_int,			/* argument type */
  pub lang: *mut language,		/* language associated with the argument */
  pub what: *mut c_char,			/* the argument itself */
}

/* Structure defining a regular expression.  C 317-330.  */
#[repr(C)]
pub struct regexp
{
  pub p_next: *mut regexp,	/* pointer to next in list */
  pub lang: *mut language,		/* if set, use only for this language */
  pub pattern: *mut c_char,		/* the regexp pattern */
  pub name: *mut c_char,			/* tag name */
  pub pat: *mut re_pattern_buffer, /* the compiled pattern */
  pub regs: re_registers,	/* re registers */
  pub error_signaled: bool,		/* already signaled for this regexp */
  pub ignore_case: bool,		/* ignore case when matching */
  pub multi_line: bool,		/* do a multi-line match on the whole file */
}


/* The C compiler's layout for the seven types above, measured with clang against the frozen
   configuration; every one of them is `#[repr(C)]` (PORTING.md 1.3, the `const _` shape the
   lib-src programs carry).  */
const _: () = core::assert! (std::mem::size_of::<compressor> () == 16);
const _: () = core::assert! (std::mem::size_of::<language> () == 56);
const _: () = core::assert! (std::mem::size_of::<fdesc> () == 64);
const _: () = core::assert! (std::mem::size_of::<node> () == 64);
const _: () = core::assert! (std::mem::size_of::<linebuffer> () == 24);
const _: () = core::assert! (std::mem::size_of::<argument> () == 24);
const _: () = core::assert! (std::mem::size_of::<regexp> () == 72);
const _: () = core::assert! (std::mem::offset_of! (language, help) == 8);
const _: () = core::assert! (std::mem::offset_of! (language, function) == 16);
const _: () = core::assert! (std::mem::offset_of! (language, suffixes) == 24);
const _: () = core::assert! (std::mem::offset_of! (language, metasource) == 48);
const _: () = core::assert! (std::mem::offset_of! (fdesc, infabsname) == 16);
const _: () = core::assert! (std::mem::offset_of! (fdesc, lang) == 40);
const _: () = core::assert! (std::mem::offset_of! (fdesc, usecharno) == 56);
const _: () = core::assert! (std::mem::offset_of! (fdesc, written) == 57);
const _: () = core::assert! (std::mem::offset_of! (node, fdp) == 16);
const _: () = core::assert! (std::mem::offset_of! (node, regex) == 32);
const _: () = core::assert! (std::mem::offset_of! (node, valid) == 40);
const _: () = core::assert! (std::mem::offset_of! (node, lno) == 48);
const _: () = core::assert! (std::mem::offset_of! (node, cno) == 56);
const _: () = core::assert! (std::mem::offset_of! (linebuffer, len) == 8);
const _: () = core::assert! (std::mem::offset_of! (linebuffer, buffer) == 16);
const _: () = core::assert! (std::mem::offset_of! (argument, lang) == 8);
const _: () = core::assert! (std::mem::offset_of! (argument, what) == 16);
const _: () = core::assert! (std::mem::offset_of! (regexp, lang) == 8);
const _: () = core::assert! (std::mem::offset_of! (regexp, pat) == 32);
const _: () = core::assert! (std::mem::offset_of! (regexp, regs) == 40);
const _: () = core::assert! (std::mem::offset_of! (regexp, error_signaled) == 64);
const _: () = core::assert! (std::mem::offset_of! (regexp, multi_line) == 66);

static mut searchar: c_char = b'/' as c_char;	/* use /.../ searches */

static mut tagfile: *mut c_char = std::ptr::null_mut ();		/* output file */
static mut progname: *mut c_char = std::ptr::null_mut ();		/* name this program was invoked with */
static mut cwd: *mut c_char = std::ptr::null_mut ();		/* current working directory */
static mut tagfiledir: *mut c_char = std::ptr::null_mut ();	/* directory of tagfile */
static mut tagf: *mut FILE = std::ptr::null_mut ();		/* ioptr for tags file */
static mut whatlen_max: ptrdiff_t = 0;	/* maximum length of any 'what' member */

static mut fdhead: *mut fdesc = std::ptr::null_mut ();		/* head of file description list */
static mut curfdp: *mut fdesc = std::ptr::null_mut ();		/* current file description */
static mut infilename: *mut c_char = std::ptr::null_mut ();	/* current input file name */
static mut lineno: intmax_t = 0;		/* line number of current line */
static mut charno: intmax_t = 0;		/* current character number */
static mut linecharno: intmax_t = 0;	/* charno of start of current line */
static mut dbp: *mut c_char = std::ptr::null_mut ();		/* pointer to start of current tag */

static invalidcharno: intmax_t = -1;

static mut nodehead: *mut node = std::ptr::null_mut ();		/* the head of the binary tree of tags */
static mut last_node: *mut node = std::ptr::null_mut ();		/* the last node created */

static mut lb: linebuffer = linebuffer { size: 0, len: 0, buffer: std::ptr::null_mut () };		/* the current line */
static mut filebuf: linebuffer = linebuffer { size: 0, len: 0, buffer: std::ptr::null_mut () };	/* a buffer containing the whole file */
static mut token_name: linebuffer = linebuffer { size: 0, len: 0, buffer: std::ptr::null_mut () };	/* a buffer containing a tag name */

static mut append_to_tagfile: bool = false;	/* -a: append to tags */
/* The next five default to true in C and derived languages.  */
static mut typedefs: bool = false;		/* -t: create tags for C and Ada typedefs */
static mut typedefs_or_cplusplus: bool = false; /* -T: create tags for C typedefs, level */
				/* 0 struct/enum/union decls, and C++ */
				/* member functions. */
static mut constantypedefs: bool = false;	/* -d: create tags for C #define, enum */
				/* constants and variables. */
				/* -D: opposite of -d.  Default if ctags. */
static mut globals: c_int = 0;		/* create tags for global variables */
static mut members: c_int = 0;		/* create tags for C member variables */
static mut declarations: c_int = 0;	/* --declarations: tag them and extern in C&Co*/
static mut no_line_directive: c_int = 0;	/* ignore #line directives (undocumented) */
static mut no_duplicates: c_int = 0;	/* no duplicate tags if ctags (undocumented) */
static mut update: bool = false;		/* -u: update tags */
static mut vgrind_style: bool = false;	/* -v: create vgrind style index output */
static mut no_warnings: bool = false;	/* -w: suppress warnings (undocumented) */
static mut cxref_style: bool = false;	/* -x: create cxref style output */
static mut cplusplus: bool = false;		/* .[hc] means C++, not C (undocumented) */
static mut ignoreindent: bool = false;	/* -I: ignore indentation in C */
static mut packages_only: c_int = 0;	/* --packages-only: in Ada, only tag packages*/
static mut class_qualify: c_int = 0;	/* -Q: produce class-qualified tags in C++/Java */
static mut ctags: bool = false;		/* --ctags */
static mut debug: c_int = 0;		/* --debug */
static mut fallback_lang: c_int = 0;	/* --(no-)fallback-lang: Fortran/C fallbacks */
static mut empty_files: c_int = 0;	/* --(no-)empty-file-entries */
static mut parsing_stdin: bool = false;	/* --parse-stdin used */

/* For long options that have no equivalent short option, use a
   non-character as a pseudo short option, starting with CHAR_MAX + 1.  */
/* C 473-479: `enum { CTAGS_OPTION = CHAR_MAX + 1, STDIN_OPTION };`  */
pub const CTAGS_OPTION: c_int = CHAR_MAX + 1;
pub const STDIN_OPTION: c_int = CHAR_MAX + 2;

static mut p_head: *mut regexp = std::ptr::null_mut ();		/* list of all regexps */
static mut need_filebuf: bool = false;	/* some regexes are multi-line */

/* C 484-527: `static struct option longopts[]`, in C's order; the last element is C's `{ NULL }`
   terminator.  C's array is not `const` (getopt writes nothing, but the pointer table is
   mutable storage in C), and Rust statics holding raw pointers are not `Sync`, so the port
   writes `static mut` as the other pointer tables do.  */
static mut longopts: [option; 38] =
[
  option { name: c"append".as_ptr (),             has_arg: no_argument,       flag: std::ptr::null_mut (), val: 'a' as c_int   },
  option { name: c"packages-only".as_ptr (),      has_arg: no_argument,       flag: &raw mut packages_only, val: 1     },
  option { name: c"c++".as_ptr (),                has_arg: no_argument,       flag: std::ptr::null_mut (), val: 'C' as c_int   },
  option { name: c"ctags".as_ptr (),              has_arg: no_argument,       flag: std::ptr::null_mut (), val: CTAGS_OPTION },
  option { name: c"debug".as_ptr (),              has_arg: no_argument,       flag: &raw mut debug, val: 1     },
  option { name: c"declarations".as_ptr (),       has_arg: no_argument,       flag: &raw mut declarations, val: 1     },
  option { name: c"no-line-directive".as_ptr (),  has_arg: no_argument,       flag: &raw mut no_line_directive, val: 1     },
  option { name: c"no-duplicates".as_ptr (),      has_arg: no_argument,       flag: &raw mut no_duplicates, val: 1     },
  option { name: c"help".as_ptr (),               has_arg: no_argument,       flag: std::ptr::null_mut (), val: 'h' as c_int   },
  option { name: c"help".as_ptr (),               has_arg: no_argument,       flag: std::ptr::null_mut (), val: 'H' as c_int   },
  option { name: c"ignore-indentation".as_ptr (), has_arg: no_argument,       flag: std::ptr::null_mut (), val: 'I' as c_int   },
  option { name: c"language".as_ptr (),           has_arg: required_argument, flag: std::ptr::null_mut (), val: 'l' as c_int   },
  option { name: c"members".as_ptr (),            has_arg: no_argument,       flag: &raw mut members, val: 1     },
  option { name: c"no-members".as_ptr (),         has_arg: no_argument,       flag: &raw mut members, val: 0     },
  option { name: c"output".as_ptr (),             has_arg: required_argument, flag: std::ptr::null_mut (), val: 'o' as c_int   },
  option { name: c"class-qualify".as_ptr (),      has_arg: no_argument,       flag: &raw mut class_qualify, val: 'Q' as c_int   },
  option { name: c"regex".as_ptr (),              has_arg: required_argument, flag: std::ptr::null_mut (), val: 'r' as c_int   },
  option { name: c"no-regex".as_ptr (),           has_arg: no_argument,       flag: std::ptr::null_mut (), val: 'R' as c_int   },
  option { name: c"ignore-case-regex".as_ptr (),  has_arg: required_argument, flag: std::ptr::null_mut (), val: 'c' as c_int   },
  option { name: c"parse-stdin".as_ptr (),        has_arg: required_argument, flag: std::ptr::null_mut (), val: STDIN_OPTION },
  option { name: c"version".as_ptr (),            has_arg: no_argument,       flag: std::ptr::null_mut (), val: 'V' as c_int   },

  /* ctags options */
  option { name: c"backward-search".as_ptr (),    has_arg: no_argument,       flag: std::ptr::null_mut (), val: 'B' as c_int   },
  option { name: c"cxref".as_ptr (),              has_arg: no_argument,       flag: std::ptr::null_mut (), val: 'x' as c_int   },
  option { name: c"defines".as_ptr (),            has_arg: no_argument,       flag: std::ptr::null_mut (), val: 'd' as c_int   },
  option { name: c"globals".as_ptr (),            has_arg: no_argument,       flag: &raw mut globals, val: 1     },
  option { name: c"typedefs".as_ptr (),           has_arg: no_argument,       flag: std::ptr::null_mut (), val: 't' as c_int   },
  option { name: c"typedefs-and-c++".as_ptr (),   has_arg: no_argument,       flag: std::ptr::null_mut (), val: 'T' as c_int   },
  option { name: c"update".as_ptr (),             has_arg: no_argument,       flag: std::ptr::null_mut (), val: 'u' as c_int   },
  option { name: c"vgrind".as_ptr (),             has_arg: no_argument,       flag: std::ptr::null_mut (), val: 'v' as c_int   },
  option { name: c"no-warn".as_ptr (),            has_arg: no_argument,       flag: std::ptr::null_mut (), val: 'w' as c_int   },

  /* etags options */
  option { name: c"no-defines".as_ptr (),         has_arg: no_argument,       flag: std::ptr::null_mut (), val: 'D' as c_int   },
  option { name: c"no-globals".as_ptr (),         has_arg: no_argument,       flag: &raw mut globals, val: 0     },
  option { name: c"include".as_ptr (),            has_arg: required_argument, flag: std::ptr::null_mut (), val: 'i' as c_int   },
  option { name: c"no-fallback-lang".as_ptr (),   has_arg: no_argument,       flag: &raw mut fallback_lang, val: 0     },
  option { name: c"fallback-lang".as_ptr (),      has_arg: no_argument,       flag: &raw mut fallback_lang, val: 1     },
  option { name: c"no-empty-file-entries".as_ptr (), has_arg: no_argument,    flag: &raw mut empty_files, val: 0     },
  option { name: c"empty-file-entries".as_ptr (), has_arg: no_argument,       flag: &raw mut empty_files, val: 1     },

  option { name: std::ptr::null (), has_arg: 0, flag: std::ptr::null_mut (), val: 0 },
];

/* C 531-546.  */
static mut compressors: [compressor; 8] =
[
  compressor { suffix: c"z".as_ptr (), command: c"gzip -d -c".as_ptr () },
  compressor { suffix: c"Z".as_ptr (), command: c"gzip -d -c".as_ptr () },
  compressor { suffix: c"gz".as_ptr (), command: c"gzip -d -c".as_ptr () },
  compressor { suffix: c"GZ".as_ptr (), command: c"gzip -d -c".as_ptr () },
  compressor { suffix: c"bz2".as_ptr (), command: c"bzip2 -d -c".as_ptr () },
  compressor { suffix: c"xz".as_ptr (), command: c"xz -d -c".as_ptr () },
  compressor { suffix: c"zst".as_ptr (), command: c"zstd -d -c".as_ptr () },
  compressor { suffix: std::ptr::null (), command: std::ptr::null () },
];

/* Ada code */

static mut Ada_suffixes: [*const c_char; 4] =
  [ c"ads".as_ptr (),
    c"adb".as_ptr (),
    c"ada".as_ptr (),
    std::ptr::null () ];

static Ada_help: &CStr = c"In Ada code, functions, procedures, packages, tasks and types are\ntags.  Use the '--packages-only' option to create tags for\npackages only.\nAda tag names have suffixes indicating the type of entity:\n\tEntity type:\tQualifier:\n\t------------\t----------\n\tfunction\t/f\n\tprocedure\t/p\n\tpackage spec\t/s\n\tpackage body\t/b\n\ttype\t\t/t\n\ttask\t\t/k\nThus, 'M-x find-tag <RET> bidule/b <RET>' will go directly to the\nbody of the package 'bidule', while 'M-x find-tag <RET> bidule <RET>'\nwill just search for any tag 'bidule'.";

/* Assembly code */

static mut Asm_suffixes: [*const c_char; 10] =
  [ c"a".as_ptr (),
    c"asm".as_ptr (),
    c"def".as_ptr (),
    c"inc".as_ptr (),
    c"ins".as_ptr (),
    c"s".as_ptr (),
    c"sa".as_ptr (),
    c"S".as_ptr (),
    c"src".as_ptr (),
    std::ptr::null () ];

static Asm_help: &CStr = c"In assembler code, labels appearing at the beginning of a line,\nfollowed by a colon, are tags.";

/* Note that .c and .h can be considered C++, if the --c++ flag was
   given, or if the `class' or `template' keywords are met inside the file.
   That is why default_C_entries is called for these. */

static mut default_C_suffixes: [*const c_char; 3] =
  [ c"c".as_ptr (),
    c"h".as_ptr (),
    std::ptr::null () ];

/* C help for ctags */

static ctags_default_C_help: &CStr = c"In C code, any C function is a tag.  Use -t to tag typedefs.\nUse -T to tag definitions of 'struct', 'union' and 'enum'.\nUse -d to tag '#define' macro definitions and 'enum' constants.\nUse --globals to tag global variables.\nYou can tag function declarations and external variables by\nusing '--declarations', and struct members by using '--members'.";

/* C help for etags */

static default_C_help: &CStr = c"In C code, any C function or typedef is a tag, and so are\ndefinitions of 'struct', 'union' and 'enum'.  '#define' macro\ndefinitions and 'enum' constants are tags unless you specify\n'--no-defines'.  Global variables are tags unless you specify\n'--no-globals' and so are struct members unless you specify\n'--no-members'.  Use of '--no-globals', '--no-defines' and\n'--no-members' can make the tags table file much smaller.\nYou can tag function declarations and external variables by\nusing '--declarations'.";

static mut Cplusplus_suffixes: [*const c_char; 13] =
  [ c"C".as_ptr (),
    c"c++".as_ptr (),
    c"cc".as_ptr (),
    c"cpp".as_ptr (),
    c"cxx".as_ptr (),
    c"H".as_ptr (),
    c"h++".as_ptr (),
    c"hh".as_ptr (),
    c"hpp".as_ptr (),
    c"hxx".as_ptr (),
    c"M".as_ptr (),
    c"pdb".as_ptr (),
    std::ptr::null () ];

static Cplusplus_help: &CStr = c"In C++ code, all the tag constructs of C code are tagged.  (Use\n--help --lang=c --lang=c++ for full help.)\nIn addition to C tags, member functions are also recognized.  Member\nvariables are recognized unless you use the '--no-members' option.\nTags for variables and functions in classes are named 'CLASS::VARIABLE'\nand 'CLASS::FUNCTION'.  'operator' definitions have tag names like\n'operator+'.";

static mut Cjava_suffixes: [*const c_char; 2] =
  [ c"java".as_ptr (),
    std::ptr::null () ];

static Cjava_help: &CStr = c"In Java code, all the tags constructs of C and C++ code are\ntagged.  (Use --help --lang=c --lang=c++ --lang=java for full help.)";

static mut Cobol_suffixes: [*const c_char; 3] =
  [ c"COB".as_ptr (),
    c"cob".as_ptr (),
    std::ptr::null () ];

static Cobol_help: &CStr = c"In Cobol code, tags are paragraph names; that is, any word\nstarting in column 8 and followed by a period.";

static mut Cstar_suffixes: [*const c_char; 3] =
  [ c"cs".as_ptr (),
    c"hs".as_ptr (),
    std::ptr::null () ];

static mut Erlang_suffixes: [*const c_char; 3] =
  [ c"erl".as_ptr (),
    c"hrl".as_ptr (),
    std::ptr::null () ];

static Erlang_help: &CStr = c"In Erlang code, the tags are the functions, records and macros\ndefined in the file.";

static mut Erlang_interpreters: [*const c_char; 2] =
  [ c"escript".as_ptr (),
    std::ptr::null () ];

static mut Forth_suffixes: [*const c_char; 3] =
  [ c"fth".as_ptr (),
    c"tok".as_ptr (),
    std::ptr::null () ];

static Forth_help: &CStr = c"In Forth code, tags are words defined by ':',\nconstant, code, create, defer, value, variable, buffer:, field.";

static mut Fortran_suffixes: [*const c_char; 8] =
  [ c"F".as_ptr (),
    c"f".as_ptr (),
    c"for".as_ptr (),
    c"f90".as_ptr (),
    c"f95".as_ptr (),
    c"f03".as_ptr (),
    c"f08".as_ptr (),
    std::ptr::null () ];

static Fortran_help: &CStr = c"In Fortran code, modules, subroutines, functions, entries\nand block data are tags.";

static mut Go_suffixes: [*const c_char; 2] =
  [ c"go".as_ptr (),
    std::ptr::null () ];

static Go_help: &CStr = c"In Go code, functions, interfaces and packages are tags.";

static mut HTML_suffixes: [*const c_char; 4] =
  [ c"htm".as_ptr (),
    c"html".as_ptr (),
    c"shtml".as_ptr (),
    std::ptr::null () ];

static HTML_help: &CStr = c"In HTML input files, the tags are the 'title' and the 'h1', 'h2',\n'h3' headers.  Also, tags are 'name=' in anchors and all\noccurrences of 'id='.";

static mut Lisp_suffixes: [*const c_char; 9] =
  [ c"cl".as_ptr (),
    c"clisp".as_ptr (),
    c"el".as_ptr (),
    c"l".as_ptr (),
    c"lisp".as_ptr (),
    c"LSP".as_ptr (),
    c"lsp".as_ptr (),
    c"ml".as_ptr (),
    std::ptr::null () ];

static Lisp_help: &CStr = c"In Lisp code, any function defined with 'defun', any variable\ndefined with 'defvar' or 'defconst', and in general the first\nargument of any expression that starts with '(def' in column zero\nis a tag.\nThe '--declarations' option tags \"(defvar foo)\" constructs too.";

static mut Lua_suffixes: [*const c_char; 3] =
  [ c"lua".as_ptr (),
    c"LUA".as_ptr (),
    std::ptr::null () ];

static Lua_help: &CStr = c"In Lua scripts, all functions are tags.";

static mut Lua_interpreters: [*const c_char; 2] =
  [ c"lua".as_ptr (),
    std::ptr::null () ];

static mut Makefile_filenames: [*const c_char; 6] =
  [ c"Makefile".as_ptr (),
    c"makefile".as_ptr (),
    c"GNUMakefile".as_ptr (),
    c"Makefile.in".as_ptr (),
    c"Makefile.am".as_ptr (),
    std::ptr::null () ];

static Makefile_help: &CStr = c"In makefiles, targets are tags; additionally, variables are tags\nunless you specify '--no-globals'.";

static mut Mercury_suffixes: [*const c_char; 2] =
  [ c"m".as_ptr (),
    std::ptr::null () ];

static Mercury_help: &CStr = c"In Mercury code, tags are all declarations beginning a line with ':-'\nand optionally Prolog-like definitions (first rule for a predicate or function).\nTo enable this behavior, run etags using --declarations.";

static mut with_mercury_definitions: bool = false;
static mut mercury_heuristics_ratio: f32 = MERCURY_HEURISTICS_RATIO;

static mut Objc_suffixes: [*const c_char; 3] =
  [ c"lm".as_ptr (),
    c"m".as_ptr (),
    std::ptr::null () ];

static Objc_help: &CStr = c"In Objective C code, tags include Objective C definitions for classes,\nclass categories, methods and protocols.  Tags for variables and\nfunctions in classes are named 'CLASS::VARIABLE' and 'CLASS::FUNCTION'.\n(Use --help --lang=c --lang=objc --lang=java for full help.)";

static mut Pascal_suffixes: [*const c_char; 3] =
  [ c"p".as_ptr (),
    c"pas".as_ptr (),
    std::ptr::null () ];

static Pascal_help: &CStr = c"In Pascal code, the tags are the functions and procedures defined\nin the file.";

/* " // this is for working around an Emacs highlighting bug... */

static mut Perl_suffixes: [*const c_char; 3] =
  [ c"pl".as_ptr (),
    c"pm".as_ptr (),
    std::ptr::null () ];

static mut Perl_interpreters: [*const c_char; 3] =
  [ c"perl".as_ptr (),
    c"@PERL@".as_ptr (),
    std::ptr::null () ];

static Perl_help: &CStr = c"In Perl code, the tags are the packages, subroutines and variables\ndefined by the 'package', 'sub', 'my' and 'local' keywords.  Use\n'--globals' if you want to tag global variables.  Tags for\nsubroutines are named 'PACKAGE::SUB'.  The name for subroutines\ndefined in the default package is 'main::SUB'.";

static mut PHP_suffixes: [*const c_char; 4] =
  [ c"php".as_ptr (),
    c"php3".as_ptr (),
    c"php4".as_ptr (),
    std::ptr::null () ];

static PHP_help: &CStr = c"In PHP code, tags are functions, classes and defines.  Unless you use\nthe '--no-members' option, vars are tags too.";

static mut plain_C_suffixes: [*const c_char; 2] =
  [ c"pc".as_ptr (),
    std::ptr::null () ];

/* C 744-745: the C 745 line carries the trailing note ".psw is for PSWrap". */

static mut PS_suffixes: [*const c_char; 3] =
  [ c"ps".as_ptr (),
    c"psw".as_ptr (),
    std::ptr::null () ];

static PS_help: &CStr = c"In PostScript code, the tags are the functions.";

static mut Prolog_suffixes: [*const c_char; 2] =
  [ c"prolog".as_ptr (),
    std::ptr::null () ];

static Prolog_help: &CStr = c"In Prolog code, tags are predicates and rules at the beginning of\nline.";

static mut Prolog_interpreters: [*const c_char; 6] =
  [ c"gprolog".as_ptr (),
    c"pl".as_ptr (),
    c"yap".as_ptr (),
    c"swipl".as_ptr (),
    c"prolog".as_ptr (),
    std::ptr::null () ];

static mut Python_suffixes: [*const c_char; 2] =
  [ c"py".as_ptr (),
    std::ptr::null () ];

static Python_help: &CStr = c"In Python code, 'def' or 'class' at the beginning of a line\ngenerate a tag.";

static mut Python_interpreters: [*const c_char; 2] =
  [ c"python".as_ptr (),
    std::ptr::null () ];

static mut Ruby_suffixes: [*const c_char; 4] =
  [ c"rb".as_ptr (),
    c"ru".as_ptr (),
    c"rbw".as_ptr (),
    std::ptr::null () ];

static mut Ruby_filenames: [*const c_char; 3] =
  [ c"Rakefile".as_ptr (),
    c"Thorfile".as_ptr (),
    std::ptr::null () ];

static Ruby_help: &CStr = c"In Ruby code, 'def' or 'class' or 'module' at the beginning of\na line generate a tag.  Constants also generate a tag.";

static mut Ruby_interpreters: [*const c_char; 2] =
  [ c"ruby".as_ptr (),
    std::ptr::null () ];

static mut Rust_suffixes: [*const c_char; 2] =
  [ c"rs".as_ptr (),
    std::ptr::null () ];

static Rust_help: &CStr = c"In Rust code, tags anything defined with 'fn', 'enum', \n'struct' or 'macro_rules!'.";

/* Can't do the `SCM' or `scm' prefix with a version number. */

static mut Scheme_suffixes: [*const c_char; 11] =
  [ c"oak".as_ptr (),
    c"rkt".as_ptr (),
    c"sch".as_ptr (),
    c"scheme".as_ptr (),
    c"SCM".as_ptr (),
    c"scm".as_ptr (),
    c"SM".as_ptr (),
    c"sm".as_ptr (),
    c"ss".as_ptr (),
    c"t".as_ptr (),
    std::ptr::null () ];

static Scheme_help: &CStr = c"In Scheme code, tags include anything defined with 'def' or with a\nconstruct whose name starts with 'def'.  They also include\nvariables set with 'set!' at top level in the file.";

static mut TeX_suffixes: [*const c_char; 8] =
  [ c"bib".as_ptr (),
    c"clo".as_ptr (),
    c"cls".as_ptr (),
    c"ltx".as_ptr (),
    c"sty".as_ptr (),
    c"TeX".as_ptr (),
    c"tex".as_ptr (),
    std::ptr::null () ];

static TeX_help: &CStr = c"In LaTeX text, the argument of the commands '\\chapter', '\\section',\n'\\subsection', '\\subsubsection', '\\eqno', '\\label', '\\ref',\n'\\Ref', '\\footref', '\\cite', '\\bibitem', '\\part', '\\appendix',\n'\\entry', '\\index', '\\def', '\\edef', '\\gdef', '\\xdef',\n'\\newcommand', '\\renewcommand', '\\newrobustcmd', '\\renewrobustcmd',\n'\\newenvironment', '\\renewenvironment', '\\DeclareRobustCommand',\n'\\providecommand', '\\providerobustcmd', '\\NewDocumentCommand',\n'\\RenewDocumentCommand', '\\ProvideDocumentCommand',\n'\\DeclareDocumentCommand', '\\NewExpandableDocumentCommand',\n'\\RenewExpandableDocumentCommand', '\\ProvideExpandableDocumentCommand',\n'\\DeclareExpandableDocumentCommand', '\\NewDocumentEnvironment',\n'\\RenewDocumentEnvironment', '\\ProvideDocumentEnvironment',\n'\\DeclareDocumentEnvironment','\\csdef', '\\csedef', '\\csgdef',\n'\\csxdef', '\\csletcs', '\\cslet', '\\letcs', '\\let',\n'\\cs_new_protected_nopar', '\\cs_new_protected', '\\cs_new_nopar',\n'\\cs_new_eq', '\\cs_new', '\\cs_set_protected_nopar',\n'\\cs_set_protected', '\\cs_set_nopar', '\\cs_set_eq', '\\cs_set',\n'\\cs_gset_protected_nopar', '\\cs_gset_protected', '\\cs_gset_nopar',\n'\\cs_gset_eq', '\\cs_gset', '\\cs_generate_from_arg_count', or\n'\\cs_generate_variant' is a tag.  So is the argument of any starred\nvariant of these commands.\n\nOther commands can be specified by setting the environment variable\n'TEXTAGS' to a colon-separated list like, for example,\n     TEXTAGS=\"mycommand:myothercommand\".";

static mut Texinfo_suffixes: [*const c_char; 4] =
  [ c"texi".as_ptr (),
    c"texinfo".as_ptr (),
    c"txi".as_ptr (),
    std::ptr::null () ];

static Texinfo_help: &CStr = c"for texinfo files, lines starting with @node are tagged.";

static mut Yacc_suffixes: [*const c_char; 6] =
  [ c"y".as_ptr (),
    c"y++".as_ptr (),
    c"ym".as_ptr (),
    c"yxx".as_ptr (),
    c"yy".as_ptr (),
    std::ptr::null () ];

static Yacc_help: &CStr = c"In Bison or Yacc input files, each rule defines as a tag the\nnonterminal it constructs.  The portions of the file that contain\nC code are parsed as C code (use --help --lang=c --lang=yacc\nfor full help).";

static auto_help: &CStr = c"'auto' is not a real language, it indicates to use\na default language for files base on file name suffix and file contents.";

static none_help: &CStr = c"'none' is not a real language, it indicates to only do\nregexp processing on files.";

static no_lang_help: &CStr = c"No detailed help available for this language.";


/* C 855: `#define C_LANG_NAMES_INDEX 2` -- the index of the "c" entry in `lang_names`, which
   `main` reads.  */
pub const C_LANG_NAMES_INDEX: c_int = 2;

/*
 * Table of languages.
 *
 * It is ok for a given function to be listed under more than one
 * name.  I just didn't.
 */

static mut lang_names: [language; 33] =
[
  language { name: c"ada".as_ptr (), help: Ada_help.as_ptr (),
             function: Some (Ada_funcs), suffixes: &raw const Ada_suffixes as *const *const c_char,
             filenames: std::ptr::null (), interpreters: std::ptr::null (), metasource: false },
  language { name: c"asm".as_ptr (), help: Asm_help.as_ptr (),
             function: Some (Asm_labels), suffixes: &raw const Asm_suffixes as *const *const c_char,
             filenames: std::ptr::null (), interpreters: std::ptr::null (), metasource: false },
  language { name: c"c".as_ptr (), help: default_C_help.as_ptr (),
             function: Some (default_C_entries), suffixes: &raw const default_C_suffixes as *const *const c_char,
             filenames: std::ptr::null (), interpreters: std::ptr::null (), metasource: false },
  language { name: c"c++".as_ptr (), help: Cplusplus_help.as_ptr (),
             function: Some (Cplusplus_entries), suffixes: &raw const Cplusplus_suffixes as *const *const c_char,
             filenames: std::ptr::null (), interpreters: std::ptr::null (), metasource: false },
  language { name: c"c*".as_ptr (), help: no_lang_help.as_ptr (),
             function: Some (Cstar_entries), suffixes: &raw const Cstar_suffixes as *const *const c_char,
             filenames: std::ptr::null (), interpreters: std::ptr::null (), metasource: false },
  language { name: c"cobol".as_ptr (), help: Cobol_help.as_ptr (),
             function: Some (Cobol_paragraphs), suffixes: &raw const Cobol_suffixes as *const *const c_char,
             filenames: std::ptr::null (), interpreters: std::ptr::null (), metasource: false },
  language { name: c"erlang".as_ptr (), help: Erlang_help.as_ptr (),
             function: Some (Erlang_functions), suffixes: &raw const Erlang_suffixes as *const *const c_char,
             filenames: std::ptr::null (), interpreters: &raw const Erlang_interpreters as *const *const c_char,
             metasource: false },
  language { name: c"forth".as_ptr (), help: Forth_help.as_ptr (),
             function: Some (Forth_words), suffixes: &raw const Forth_suffixes as *const *const c_char,
             filenames: std::ptr::null (), interpreters: std::ptr::null (), metasource: false },
  language { name: c"fortran".as_ptr (), help: Fortran_help.as_ptr (),
             function: Some (Fortran_functions), suffixes: &raw const Fortran_suffixes as *const *const c_char,
             filenames: std::ptr::null (), interpreters: std::ptr::null (), metasource: false },
  language { name: c"go".as_ptr (), help: Go_help.as_ptr (),
             function: Some (Go_functions), suffixes: &raw const Go_suffixes as *const *const c_char,
             filenames: std::ptr::null (), interpreters: std::ptr::null (), metasource: false },
  language { name: c"html".as_ptr (), help: HTML_help.as_ptr (),
             function: Some (HTML_labels), suffixes: &raw const HTML_suffixes as *const *const c_char,
             filenames: std::ptr::null (), interpreters: std::ptr::null (), metasource: false },
  language { name: c"java".as_ptr (), help: Cjava_help.as_ptr (),
             function: Some (Cjava_entries), suffixes: &raw const Cjava_suffixes as *const *const c_char,
             filenames: std::ptr::null (), interpreters: std::ptr::null (), metasource: false },
  language { name: c"lisp".as_ptr (), help: Lisp_help.as_ptr (),
             function: Some (Lisp_functions), suffixes: &raw const Lisp_suffixes as *const *const c_char,
             filenames: std::ptr::null (), interpreters: std::ptr::null (), metasource: false },
  language { name: c"lua".as_ptr (), help: Lua_help.as_ptr (),
             function: Some (Lua_functions), suffixes: &raw const Lua_suffixes as *const *const c_char,
             filenames: std::ptr::null (), interpreters: &raw const Lua_interpreters as *const *const c_char,
             metasource: false },
  language { name: c"makefile".as_ptr (), help: Makefile_help.as_ptr (),
             function: Some (Makefile_targets), suffixes: std::ptr::null (),
             filenames: &raw const Makefile_filenames as *const *const c_char, interpreters: std::ptr::null (),
             metasource: false },
  /* objc listed before mercury as it is a better default for .m extensions.  */
  language { name: c"objc".as_ptr (), help: Objc_help.as_ptr (),
             function: Some (plain_C_entries), suffixes: &raw const Objc_suffixes as *const *const c_char,
             filenames: std::ptr::null (), interpreters: std::ptr::null (), metasource: false },
  language { name: c"mercury".as_ptr (), help: Mercury_help.as_ptr (),
             function: Some (Mercury_functions), suffixes: &raw const Mercury_suffixes as *const *const c_char,
             filenames: std::ptr::null (), interpreters: std::ptr::null (), metasource: false },
  language { name: c"pascal".as_ptr (), help: Pascal_help.as_ptr (),
             function: Some (Pascal_functions), suffixes: &raw const Pascal_suffixes as *const *const c_char,
             filenames: std::ptr::null (), interpreters: std::ptr::null (), metasource: false },
  language { name: c"perl".as_ptr (), help: Perl_help.as_ptr (),
             function: Some (Perl_functions), suffixes: &raw const Perl_suffixes as *const *const c_char,
             filenames: std::ptr::null (), interpreters: &raw const Perl_interpreters as *const *const c_char,
             metasource: false },
  language { name: c"php".as_ptr (), help: PHP_help.as_ptr (),
             function: Some (PHP_functions), suffixes: &raw const PHP_suffixes as *const *const c_char,
             filenames: std::ptr::null (), interpreters: std::ptr::null (), metasource: false },
  language { name: c"postscript".as_ptr (), help: PS_help.as_ptr (),
             function: Some (PS_functions), suffixes: &raw const PS_suffixes as *const *const c_char,
             filenames: std::ptr::null (), interpreters: std::ptr::null (), metasource: false },
  language { name: c"proc".as_ptr (), help: no_lang_help.as_ptr (),
             function: Some (plain_C_entries), suffixes: &raw const plain_C_suffixes as *const *const c_char,
             filenames: std::ptr::null (), interpreters: std::ptr::null (), metasource: false },
  language { name: c"prolog".as_ptr (), help: Prolog_help.as_ptr (),
             function: Some (Prolog_functions), suffixes: &raw const Prolog_suffixes as *const *const c_char,
             filenames: std::ptr::null (), interpreters: &raw const Prolog_interpreters as *const *const c_char,
             metasource: false },
  language { name: c"python".as_ptr (), help: Python_help.as_ptr (),
             function: Some (Python_functions), suffixes: &raw const Python_suffixes as *const *const c_char,
             filenames: std::ptr::null (), interpreters: &raw const Python_interpreters as *const *const c_char,
             metasource: false },
  language { name: c"ruby".as_ptr (), help: Ruby_help.as_ptr (),
             function: Some (Ruby_functions), suffixes: &raw const Ruby_suffixes as *const *const c_char,
             filenames: &raw const Ruby_filenames as *const *const c_char, interpreters: &raw const Ruby_interpreters as *const *const c_char,
             metasource: false },
  language { name: c"rust".as_ptr (), help: Rust_help.as_ptr (),
             function: Some (Rust_entries), suffixes: &raw const Rust_suffixes as *const *const c_char,
             filenames: std::ptr::null (), interpreters: std::ptr::null (), metasource: false },
  language { name: c"scheme".as_ptr (), help: Scheme_help.as_ptr (),
             function: Some (Scheme_functions), suffixes: &raw const Scheme_suffixes as *const *const c_char,
             filenames: std::ptr::null (), interpreters: std::ptr::null (), metasource: false },
  language { name: c"tex".as_ptr (), help: TeX_help.as_ptr (),
             function: Some (TeX_commands), suffixes: &raw const TeX_suffixes as *const *const c_char,
             filenames: std::ptr::null (), interpreters: std::ptr::null (), metasource: false },
  language { name: c"texinfo".as_ptr (), help: Texinfo_help.as_ptr (),
             function: Some (Texinfo_nodes), suffixes: &raw const Texinfo_suffixes as *const *const c_char,
             filenames: std::ptr::null (), interpreters: std::ptr::null (), metasource: false },
  language { name: c"yacc".as_ptr (), help: Yacc_help.as_ptr (),
             function: Some (Yacc_entries), suffixes: &raw const Yacc_suffixes as *const *const c_char,
             filenames: std::ptr::null (), interpreters: std::ptr::null (), metasource: true },
  language { name: c"auto".as_ptr (), help: auto_help.as_ptr (),        /* default guessing scheme */
             function: None, suffixes: std::ptr::null (),
             filenames: std::ptr::null (), interpreters: std::ptr::null (), metasource: false },
  language { name: c"none".as_ptr (), help: none_help.as_ptr (),
             function: Some (just_read_file), suffixes: std::ptr::null (),
             filenames: std::ptr::null (), interpreters: std::ptr::null (), metasource: false },
  language { name: std::ptr::null (), help: std::ptr::null (), function: None,   /* end of list */
             suffixes: std::ptr::null (), filenames: std::ptr::null (),
             interpreters: std::ptr::null (), metasource: false },
];

unsafe fn print_language_names ()
{
  let mut lang: *mut language = &raw mut lang_names[0];
  let mut name: *const *const c_char = std::ptr::null ();
  let mut ext: *const *const c_char = std::ptr::null ();

  puts (c"\nThese are the currently supported languages, along with the\ndefault file names and dot suffixes:".as_ptr ());
  while (*lang).name != std::ptr::null ()
    {
      printf (c"  %-*s".as_ptr (), 10, (*lang).name);
      if (*lang).filenames != std::ptr::null ()
	{
	  name = (*lang).filenames;
	  while *name != std::ptr::null ()
	    { printf (c" %s".as_ptr (), *name); name = name.offset (1); }
	}
      if (*lang).suffixes != std::ptr::null ()
	{
	  ext = (*lang).suffixes;
	  while *ext != std::ptr::null ()
	    { printf (c" .%s".as_ptr (), *ext); ext = ext.offset (1); }
	}
      puts (c"".as_ptr ());
      lang = lang.offset (1);
    }
  puts (c"where 'auto' means use default language for files based on file\nname suffix, and 'none' means only do regexp processing on files.\nIf no language is specified and no matching suffix is found,\nthe first line of the file is read for a sharp-bang (#!) sequence\nfollowed by the name of an interpreter.  If no such sequence is found,\nFortran is tried first; if no tags are found, C is tried next.\nWhen parsing any C file, a \"class\" or \"template\" keyword\nswitches to C++.".as_ptr ());
  puts (c"Compressed files are supported using gzip, bzip2, xz, and zstd.\n\nFor detailed help on a given language use, for example,\netags --help --lang=ada.".as_ptr ());
}

/// `<config.h>`'s `PACKAGE_NAME`, carried by this unit the way `src/emacs.rs` carries the
/// core's; `print_version` prints it, so it is the port's own name for the same reason
/// (PORTING.md 0.1-19's name half, status 3.433).  The frozen configuration's own value is a
/// measurement of the reference build and stays in `docs/baseline/config/`.
pub const PACKAGE_NAME: &CStr = c"femacs";
/// `<config.h>`'s `PACKAGE_VERSION` (measured: `32.0.50`).
pub const PACKAGE_VERSION: &CStr = c"32.0.50";
/// `<config.h>`'s `COPYRIGHT` (measured: the frozen configuration's value).
pub const COPYRIGHT: &CStr = c"Copyright (C) 2026 Free Software Foundation, Inc.";

unsafe extern "C" fn print_version () -> !
{
  fputs (c"etags (femacs 32.0.50)\nCopyright (C) 2026 Free Software Foundation, Inc.\nThis program is distributed under the terms in ETAGS.README\n".as_ptr (),
	 stdout);
  exit (EXIT_SUCCESS);
}

/* C 938-940: `#ifndef PRINT_UNDOCUMENTED_OPTIONS_HELP` / `# define
   PRINT_UNDOCUMENTED_OPTIONS_HELP false` / `#endif`.  */
pub const PRINT_UNDOCUMENTED_OPTIONS_HELP: bool = false;

unsafe extern "C" fn print_help (mut argbuffer: *mut argument) -> !
{
  let mut help_for_lang: bool = false;

  while (*argbuffer).arg_type != at_end
    {
      if (*argbuffer).arg_type == at_language
	{
	  if help_for_lang
	    { puts (c"".as_ptr ()); }
	  puts ((*(*argbuffer).lang).help);
	  help_for_lang = true;
	}
      argbuffer = argbuffer.offset (1);
    }

  if help_for_lang
    { exit (EXIT_SUCCESS); }

  printf (c"Usage: %s [options] [[regex-option ...] file-name] ...\n\nThese are the options accepted by %s.\n".as_ptr (), progname, progname);
  puts (c"You may use unambiguous abbreviations for the long option names.".as_ptr ());
  puts (c"  A - as file name means read names from stdin (one per line).\nAbsolute names are stored in the output file as they are.\nRelative ones are stored relative to the output file's directory.\n".as_ptr ());

  puts (c"-a, --append\n        Append tag entries to existing tags file.".as_ptr ());

  puts (c"--packages-only\n        For Ada files, only generate tags for packages.".as_ptr ());

  if (ctags)
  {
      puts (c"-B, --backward-search\n        Write the search commands for the tag entries using '?', the\n        backward-search command instead of '/', the forward-search command.".as_ptr ());
  }

  /* This option is mostly obsolete, because etags can now automatically
     detect C++.  Retained for backward compatibility and for debugging and
     experimentation.  In principle, we could want to tag as C++ even
     before any "class" or "template" keyword.
  puts (c"-C, --c++\n        Treat files whose name suffix defaults to C language as C++ files.".as_ptr ());
  */

  if (PRINT_UNDOCUMENTED_OPTIONS_HELP && !ctags)
  {
      puts (c"--ctags\n\tImplement ctags behavior.".as_ptr ());
  }

  puts (c"--declarations\n\tIn C and derived languages, create tags for function declarations,".as_ptr ());
  if (ctags)
  {
      puts (c"\tand create tags for extern variables if --globals is used.".as_ptr ());
  }
  else
  {
      puts (c"\tand create tags for extern variables unless --no-globals is used.".as_ptr ());
  }

  puts (c"\tIn Mercury, tag both declarations starting a line with ':-' and\n        first predicates or functions in clauses.".as_ptr ());

  if (ctags)
  {
      puts (c"-d, --defines\n        Create tag entries for C #define constants and enum constants, too.".as_ptr ());
  }
  else
  {
      puts (c"-D, --no-defines\n        Don't create tag entries for C #define constants and enum constants.\n\tThis makes the tags file smaller.".as_ptr ());
  }

  if (!ctags)
  {
      puts (c"-i FILE, --include=FILE\n        Include a note in tag file indicating that, when searching for\n        a tag, one should also consult the tags file FILE after\n        checking the current file.".as_ptr ());
  }

  puts (c"-l LANG, --language=LANG\n        Force the following files to be considered as written in the\n\tnamed language up to the next --language=LANG option.".as_ptr ());

  if (ctags)
  {
      puts (c"--globals\n\tCreate tag entries for global variables in some languages.".as_ptr ());
  }
  else
  {
      puts (c"--no-globals\n\tDo not create tag entries for global variables in some\n\tlanguages.  This makes the tags file smaller.".as_ptr ());
  }

  puts (c"--no-line-directive\n        Ignore #line preprocessor directives in C and derived languages.".as_ptr ());

  if (ctags)
  {
      puts (c"--members\n\tCreate tag entries for members of structures in some languages.".as_ptr ());
  }
  else
  {
      puts (c"--no-members\n\tDo not create tag entries for members of structures\n\tin some languages.".as_ptr ());
  }

  if (!ctags)
    {
      puts (c"--fallback-lang\n\tIf a file's language could not be determined, try to parse\n\tit as Fortran and C/C++.".as_ptr ());
      puts (c"--no-fallback-lang\n\tDo not fall back to Fortran and C/C++ if a file's language\n\tcould not be determined.".as_ptr ());
      puts (c"--empty-file-entries\n\tProduce file entries for files with no tags.".as_ptr ());
      puts (c"--no-empty-file-entries\n\tDo not output file entries for files with no tags.".as_ptr ());
    }

  puts (c"-Q, --class-qualify\n        Qualify tag names with their class name in C++, ObjC, Java, and Perl.\n        This produces tag names of the form \"class::member\" for C++,\n        \"class(category)\" for Objective C, and \"class.member\" for Java.\n        For Objective C, this also produces class methods qualified with\n        their arguments, as in \"foo:bar:baz:more\".\n        For Perl, this produces \"package::member\".".as_ptr ());
  puts (c"-r REGEXP, --regex=REGEXP or --regex=@regexfile\n        Make a tag for each line matching a regular expression pattern\n\tin the following files.  {LANGUAGE}REGEXP uses REGEXP for LANGUAGE\n\tfiles only.  REGEXFILE is a file containing one REGEXP per line.\n\tREGEXP takes the form /TAGREGEXP/TAGNAME/MODS, where TAGNAME/ is\n\toptional.  The TAGREGEXP pattern is anchored (as if preceded by ^).".as_ptr ());
  puts (c"\tIf TAGNAME/ is present, the tags created are named.\n\tFor example Tcl named tags can be created with:\n\t  --regex=\"/proc[ \\t]+\\([^ \\t]+\\)/\\1/.\".\n\tMODS are optional one-letter modifiers: 'i' means to ignore case,\n\t'm' means to allow multi-line matches, 's' implies 'm' and\n\tcauses dot to match any character, including newline.".as_ptr ());

  puts (c"-R, --no-regex\n        Don't create tags from regexps for the following files.".as_ptr ());

  puts (c"-I, --ignore-indentation\n        In C and C++ do not assume that a closing brace in the first\n        column is the final brace of a function or structure definition.".as_ptr ());

  puts (c"-o FILE, --output=FILE\n        Write the tags to FILE.".as_ptr ());

  puts (c"--parse-stdin=NAME\n        Read from standard input and record tags as belonging to file NAME.".as_ptr ());

  if (ctags)
    {
      puts (c"-t, --typedefs\n        Generate tag entries for C and Ada typedefs.".as_ptr ());
      puts (c"-T, --typedefs-and-c++\n        Generate tag entries for C typedefs, C struct/enum/union tags,\n        and C++ member functions.".as_ptr ());
    }

  if (ctags)
  {
      puts (c"-u, --update\n        Update the tag entries for the given files, leaving tag\n        entries for other files in place.  Currently, this is\n        implemented by deleting the existing entries for the given\n        files and then rewriting the new entries at the end of the\n        tags file.  It is often faster to simply rebuild the entire\n        tag file than to use this.".as_ptr ());
  }

  if (ctags)
    {
      puts (c"-v, --vgrind\n        Print on the standard output an index of items intended for\n        human consumption, similar to the output of vgrind.  The index\n        is sorted, and gives the page number of each item.".as_ptr ());

      if (PRINT_UNDOCUMENTED_OPTIONS_HELP)
      {
   puts (c"-w, --no-duplicates\n        Do not create duplicate tag entries, for compatibility with\n\ttraditional ctags.".as_ptr ());
      }

      if (PRINT_UNDOCUMENTED_OPTIONS_HELP)
      {
   puts (c"-w, --no-warn\n        Suppress warning messages about duplicate tag entries.".as_ptr ());
      }

      puts (c"-x, --cxref\n        Like --vgrind, but in the style of cxref, rather than vgrind.\n        The output uses line numbers instead of page numbers, but\n        beyond that the differences are cosmetic; try both to see\n        which you like.".as_ptr ());
    }

  puts (c"-V, --version\n        Print the version of the program.\n-h, --help\n        Print this help message.\n        Followed by one or more '--language' options prints detailed\n        help about tag generation for the specified languages.".as_ptr ());

  print_language_names ();

  puts (c"".as_ptr ());
  puts (c"Report bugs to https://github.com/liuchong/femacs/issues".as_ptr ());

  exit (EXIT_SUCCESS);
}

/* `<stdio.h>`: `int snprintf (char *s, size_t maxlen, const char *format, ...);` -- no mirror
   carries the `snprintf` family yet (the make-docfile.rs / emacsclient.rs precedent keeps
   them at the use site).  */
unsafe extern "C" {
  pub fn snprintf (s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;
}

/* `STDOUT_FILENO` comes from the unistd mirror now (the use-site const is gone: the
   mirror landed it, PORTING.md 0.1-3; `build-tu` found the duplicate as an E0428).  */

/* `<stdlib.h>`: `int system (const char *command);` -- the sysdep.rs 953 declaration is for
   the core; this program has no `lisp.h` mirror, so it declares it here.  */
unsafe extern "C" {
  pub fn system (command: *const c_char) -> c_int;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main (argc: c_int, argv: *mut *mut c_char) -> c_int
{
  let mut i: c_int;
  let mut nincluded_files: c_int;
  let mut included_files: *mut *mut c_char = std::ptr::null_mut ();
  let mut argbuffer: *mut argument = std::ptr::null_mut ();
  let mut current_arg: c_int; let mut file_count: c_int;
  let mut filename_lb: linebuffer = linebuffer { size: 0, len: 0, buffer: std::ptr::null_mut () };
  let mut help_asked: bool = false;
  let mut len: ptrdiff_t;
  let mut opt: c_int;

  progname = *argv;
  nincluded_files = 0;
  included_files = xnmalloc (argc as ptrdiff_t, __sizeof_deref (included_files)) as *mut *mut c_char;
  current_arg = 0;
  file_count = 0;

  /* Allocate enough no matter what happens.  Overkill, but each one
     is small. */
  argbuffer = xnmalloc (argc as ptrdiff_t, __sizeof_deref (argbuffer)) as *mut argument;

  /*
   * Always find typedefs and structure tags.
   * Also default to find macro constants, enum constants, struct
   * members and global variables.  Do it for both etags and ctags.
   */
  constantypedefs = true;
  typedefs_or_cplusplus = constantypedefs;
  typedefs = typedefs_or_cplusplus;
  members = true as c_int;
  globals = members;

  /* By default, fall back to Fortran/C/C++ if no language is detected by the
     file's name.  This could be reversed in a future version, but only for
     ETAGS.  */
  fallback_lang = true as c_int;

  /* By default, output file entries for files that have no tags.  This affects
     only ETAGS. */
  empty_files = true as c_int;

  /* When the optstring begins with a '-' getopt_long does not rearrange the
     non-options arguments to be at the end, but leaves them alone. */
  static optstring: &CStr = c"-aBc:CdDf:hHi:Il:o:Qr:RStTuvVwx";

  loop
    {
      opt = getopt_long (argc, argv, optstring.as_ptr (), &raw const longopts as *const option,
			 std::ptr::null_mut ());
      if !(opt != EOF)
	{ break; }
      match opt
	{
	  0 =>
	    {
	    /* If getopt returns 0, then it has already processed a
       long-named option.  We should do nothing.  */
	    }

	  1 =>
	    {
	    /* This means that a file name has been seen.  Record it. */
	    (*argbuffer.offset (current_arg as isize)).arg_type = at_filename;
	    (*argbuffer.offset (current_arg as isize)).what     = optarg;
	    len = strlen (optarg) as ptrdiff_t;
	    if whatlen_max < len
	      { whatlen_max = len; }
	    current_arg += 1;
	    file_count += 1;
	    }

	  CTAGS_OPTION =>
	    {
	    /* Operate as ctags, not etags.  */
	    ctags = true;
	    (*&raw mut lang_names[C_LANG_NAMES_INDEX as usize]).help = ctags_default_C_help.as_ptr ();
	    }

	  STDIN_OPTION =>
	    {
	    /* Parse standard input.  Idea by Vivek <vivek@etla.org>. */
	    (*argbuffer.offset (current_arg as isize)).arg_type = at_stdin;
	    (*argbuffer.offset (current_arg as isize)).what     = optarg;
	    len = strlen (optarg) as ptrdiff_t;
	    if whatlen_max < len
	      { whatlen_max = len; }
	    current_arg += 1;
	    file_count += 1;
	    if parsing_stdin
	      { fatal (c"cannot parse standard input more than once".as_ptr ()); }
	    parsing_stdin = true;
	    }

	/* Common options. */
	  97 /* 'a' */ => { append_to_tagfile = true; }
	  67 /* 'C' */ => { cplusplus = true; }
	  102 /* 'f' */ | 111 /* 'o' */ =>
	    {
	    /* for compatibility with old makefiles */
	    if tagfile != std::ptr::null_mut ()
	      {
		error (c"-o option may only be given once.".as_ptr ());
		suggest_asking_for_help ();
	      }
	    tagfile = optarg;
	    }
	  73 /* 'I' */ | 83 /* 'S' */ =>
	    {
	    /* for backward compatibility */
	    ignoreindent = true;
	    }
	  108 /* 'l' */ =>
	    {
	      let lang: *mut language = get_language_from_langname (optarg);
	      if lang != std::ptr::null_mut ()
		{
		  (*argbuffer.offset (current_arg as isize)).lang = lang;
		  (*argbuffer.offset (current_arg as isize)).arg_type = at_language;
		  current_arg += 1;
		}
	    }
	  99 /* 'c' */ =>
	    {
	    /* Backward compatibility: support obsolete --ignore-case-regexp. */
	    optarg = concat (optarg, c"i".as_ptr (), c"".as_ptr ()); /* memory leak here */
	    /* C's `FALLTHROUGH;` sends this arm on into the `case 'r':` arm's statements
	       (PORTING.md 2.2).  */
	    (*argbuffer.offset (current_arg as isize)).arg_type = at_regexp;
	    (*argbuffer.offset (current_arg as isize)).what = optarg;
	    len = strlen (optarg) as ptrdiff_t;
	    if whatlen_max < len
	      { whatlen_max = len; }
	    current_arg += 1;
	    }
	  114 /* 'r' */ =>
	    {
	    (*argbuffer.offset (current_arg as isize)).arg_type = at_regexp;
	    (*argbuffer.offset (current_arg as isize)).what = optarg;
	    len = strlen (optarg) as ptrdiff_t;
	    if whatlen_max < len
	      { whatlen_max = len; }
	    current_arg += 1;
	    }
	  82 /* 'R' */ =>
	    {
	    (*argbuffer.offset (current_arg as isize)).arg_type = at_regexp;
	    (*argbuffer.offset (current_arg as isize)).what = std::ptr::null_mut ();
	    current_arg += 1;
	    }
	  86 /* 'V' */ =>
	    {
	    print_version ();
	    }
	  104 /* 'h' */ | 72 /* 'H' */ =>
	    {
	    help_asked = true;
	    }
	  81 /* 'Q' */ =>
	    {
	    class_qualify = 1;
	    }

	/* Etags options */
	  68 /* 'D' */ => { constantypedefs = false; }
	  105 /* 'i' */ =>
	    {
	      *included_files.offset (nincluded_files as isize) = optarg;
	      nincluded_files += 1;
	    }

	/* Ctags options. */
	  66 /* 'B' */ => { searchar = b'?' as c_char; }
	  100 /* 'd' */ => { constantypedefs = true; }
	  116 /* 't' */ => { typedefs = true; }
	  84 /* 'T' */ =>
	    {
	      typedefs = true;
	      typedefs_or_cplusplus = true;
	    }
	  117 /* 'u' */ => { update = true; }
	  118 /* 'v' */ =>
	    {
	      vgrind_style = true;
	      /* C's `FALLTHROUGH;` sends this arm on into the `case 'x':` arm's statement
		 (PORTING.md 2.2).  */
	      cxref_style = true;
	    }
	  120 /* 'x' */ => { cxref_style = true; }
	  119 /* 'w' */ => { no_warnings = true; }
	  _ =>
	    {
	    suggest_asking_for_help ();
	    }
	}
    }

  /* No more options.  Store the rest of arguments. */
  while optind < argc
    {
      (*argbuffer.offset (current_arg as isize)).arg_type = at_filename;
      (*argbuffer.offset (current_arg as isize)).what = *argv.offset (optind as isize);
      len = strlen (*argv.offset (optind as isize)) as ptrdiff_t;
      if whatlen_max < len
	{ whatlen_max = len; }
      current_arg += 1;
      file_count += 1;
      optind += 1;
    }

  (*argbuffer.offset (current_arg as isize)).arg_type = at_end;

  if help_asked
    { print_help (argbuffer); }

  if nincluded_files == 0 && file_count == 0
    {
      error (c"no input files specified.".as_ptr ());
      suggest_asking_for_help ();
    }

  if tagfile == std::ptr::null_mut ()
    {
      tagfile = savestr (if ctags { c"tags".as_ptr () } else { c"TAGS".as_ptr () });
    }
  cwd = etags_getcwd ();	/* the current working directory */
  if *cwd.offset (strlen (cwd) as isize - 1) != b'/' as c_char
    {
      let oldcwd: *mut c_char = cwd;
      cwd = concat (oldcwd, c"/".as_ptr (), c"".as_ptr ());
      free (oldcwd as *mut c_void);
    }

  /* Compute base directory for relative file names. */
  if streq (tagfile, c"-".as_ptr ()) || strneq (tagfile, c"/dev/".as_ptr (), 5)
    { tagfiledir = cwd; }		 /* relative file names are relative to cwd */
  else
    {
      canonicalize_filename (tagfile);
      tagfiledir = absolute_dirname (tagfile, cwd);
    }

  linebuffer_init (&raw mut lb);
  linebuffer_init (&raw mut filename_lb);
  linebuffer_init (&raw mut filebuf);
  linebuffer_init (&raw mut token_name);

  if !ctags
    {
      if streq (tagfile, c"-".as_ptr ())
	{
	  tagf = stdout;
	  set_binary_mode (STDOUT_FILENO, O_BINARY);
	}
      else
	{ tagf = fopen (tagfile, if append_to_tagfile { c"ab".as_ptr () } else { c"wb".as_ptr () }); }
      if tagf == std::ptr::null_mut ()
	{ pfatal (tagfile); }
    }

  /*
   * Loop through files finding functions.
   */
  i = 0;
  while i < current_arg
    {
      static mut lang: *mut language = std::ptr::null_mut ();	/* non-NULL if language is forced */
      let mut this_file: *mut c_char;

      match (*argbuffer.offset (i as isize)).arg_type
	{
	at_language =>
	  {
	  lang = (*argbuffer.offset (i as isize)).lang;
	  }
	at_regexp =>
	  {
	  analyze_regex ((*argbuffer.offset (i as isize)).what);
	  }
	at_filename =>
	  {
	      this_file = (*argbuffer.offset (i as isize)).what;
	      /* Input file named "-" means read file names from stdin
		 (one per line) and use them. */
	      if streq (this_file, c"-".as_ptr ())
		{
		  if parsing_stdin
		    { fatal (c"cannot parse standard input AND read file names from it".as_ptr ()); }
		  while readline_internal (&raw mut filename_lb, stdin, c"-".as_ptr (), false) > 0
		    { process_file_name (filename_lb.buffer, lang); }
		}
	      else
		{ process_file_name (this_file, lang); }
	  }
	at_stdin =>
	  {
	    this_file = (*argbuffer.offset (i as isize)).what;
	    process_file (stdin, this_file, lang);
	  }
	_ =>
	  {
	  error (c"internal error: arg_type".as_ptr ());
	  }
	}
      i += 1;
    }

  free_regexps ();
  free (lb.buffer as *mut c_void);
  free (filebuf.buffer as *mut c_void);
  free (token_name.buffer as *mut c_void);

  if !ctags || cxref_style
    {
      /* Write the remaining tags to tagf (ETAGS) or stdout (CXREF). */
      put_entries (nodehead);
      free_tree (nodehead);
      nodehead = std::ptr::null_mut ();
      if !ctags
	{
	  let mut fdp: *mut fdesc;

	  /* Output file entries that have no tags, unless disabled. */
	  if empty_files != 0
	    {
	      fdp = fdhead;
	      while fdp != std::ptr::null_mut ()
		{
		  if !(*fdp).written
		    { fprintf (tagf, c"\x0c\n%s,0\n".as_ptr (), (*fdp).taggedfname); }
		  fdp = (*fdp).next;
		}
	    }

	  loop
	    {
	      let old: c_int = nincluded_files;
	      nincluded_files -= 1;
	      if !(old > 0)
		{ break; }
	      fprintf (tagf, c"\x0c\n%s,include\n".as_ptr (), *included_files);
	      included_files = included_files.offset (1);
	    }

	  if fclose (tagf) == EOF
	    { pfatal (tagfile); }
	}

      return EXIT_SUCCESS;
    }

  /* From here on, we are in (CTAGS && !cxref_style) */
  if update
    {
      i = 0;
      while i < current_arg
	{
	  match (*argbuffer.offset (i as isize)).arg_type
	    {
	    at_filename | at_stdin => {}
	    _ =>
	      {
		i += 1;
		continue;		/* the for loop */
	      }
	    }
	  cleanup_tags_file (tagfile, (*argbuffer.offset (i as isize)).what);
	  i += 1;
	}
      append_to_tagfile = true;
    }

  tagf = fopen (tagfile, if append_to_tagfile { c"ab".as_ptr () } else { c"wb".as_ptr () });
  if tagf == std::ptr::null_mut ()
    { pfatal (tagfile); }
  put_entries (nodehead);	/* write all the tags (CTAGS) */
  free_tree (nodehead);
  nodehead = std::ptr::null_mut ();
  if fclose (tagf) == EOF
    { pfatal (tagfile); }

  if ctags
    {
      if append_to_tagfile || update
	{
	  /* Maybe these should be used:
	     setenv ("LC_COLLATE", "C", 1);
	     setenv ("LC_ALL", "C", 1); */
	  let mut cmd: *mut c_char = xmalloc (8 * strlen (tagfile) as ptrdiff_t
					      + 17 /* sizeof "sort -u -o '' ''" */) as *mut c_char;
	  /* C 1465-1484: `#if defined WINDOWSNT || MSDOS` is dead in the frozen configuration
	     (recorded, not transcribed); the live arm is the `#else` below.  */
	  /* Quote 'like this', and escape the apostrophe in the file name.  */
	  let mut z: *mut c_char = stpcpy (cmd, c"sort -u -o '".as_ptr ());
	  let escaped_tagfile: *mut c_char = z;
	  while *tagfile != 0
	    {
	      if *tagfile == b'\'' as c_char
		{ z = stpcpy (z, c"'\\'".as_ptr ()); }
	      *z = *tagfile;
	      z = z.offset (1);
	      tagfile = tagfile.offset (1);
	    }
	  let escaped_tagfile_len: ptrdiff_t = z.offset_from (escaped_tagfile);
	  z = stpcpy (z, c"' '".as_ptr ());
	  z = mempcpy (z as *mut c_void, escaped_tagfile as *const c_void,
			   escaped_tagfile_len as size_t) as *mut c_char;
	  strcpy (z, c"'".as_ptr ());
	  return system (cmd);
	}
    }
  return EXIT_SUCCESS;
}

/*
 * Equivalent to: mv tags OTAGS;grep -Fv ' filename ' OTAGS >tags;rm OTAGS
 */
/* The C parameter `tagfile` shadows the file-scope object of that name; Rust refuses a
   parameter that shadows a `static` (E0530), so the port spells it with the trailing
   underscore the ledger records (docs/ports/keyboard.md 503's convention).  */
unsafe fn cleanup_tags_file (tagfile_: *const c_char, match_file_name: *const c_char)
{
  let mut otags_f: *mut FILE = fopen (c"OTAGS".as_ptr (), c"wb".as_ptr ());
  let mut tag_f: *mut FILE = fopen (tagfile_, c"rb".as_ptr ());

  if otags_f == std::ptr::null_mut ()
    { pfatal (c"OTAGS".as_ptr ()); }

  if tag_f == std::ptr::null_mut ()
    { pfatal (tagfile_); }

  let mut buf_len: c_int = strlen (match_file_name) as c_int + 4 /* sizeof ("\t\t ") */ + 1;
  let mut buf: *mut c_char = xmalloc (buf_len as ptrdiff_t) as *mut c_char;
  snprintf (buf, buf_len as size_t, c"\t%s\t".as_ptr (), match_file_name);

  let mut line: linebuffer = linebuffer { size: 0, len: 0, buffer: std::ptr::null_mut () };
  linebuffer_init (&raw mut line);
  while readline_internal (&raw mut line, tag_f, tagfile_, true) > 0
    {
      if ferror (tag_f) != 0
	{ pfatal (tagfile_); }

      if strstr (line.buffer, buf) == std::ptr::null_mut ()
	{
	  fprintf (otags_f, c"%s\n".as_ptr (), line.buffer);
	  if ferror (tag_f) != 0
	    { pfatal (tagfile_); }
	}
    }
  free (buf as *mut c_void);
  free (line.buffer as *mut c_void);

  if fclose (otags_f) == EOF
    { pfatal (c"OTAGS".as_ptr ()); }

  if fclose (tag_f) == EOF
    { pfatal (tagfile_); }

  do_move_file (c"OTAGS".as_ptr (), tagfile_);
  return;
}

/*
 * Return a compressor given the file name.  If EXTPTR is non-zero,
 * return a pointer into FILE where the compressor-specific
 * extension begins.  If no compressor is found, NULL is returned
 * and EXTPTR is not significant.
 * Idea by Vladimir Alexiev <vladimir@cs.ualberta.ca> (1998)
 */
unsafe fn get_compressor_from_suffix (file: *mut c_char, extptr: *mut *mut c_char) -> *mut compressor
{
  let mut compr: *mut compressor;
  let mut slash: *mut c_char; let mut suffix: *mut c_char;

  /* File has been processed by canonicalize_filename,
     so we don't need to consider backslashes on DOS_NT.  */
  slash = strrchr (file, b'/' as c_int);
  suffix = strrchr (file, b'.' as c_int);
  if suffix == std::ptr::null_mut () || suffix < slash
    { return std::ptr::null_mut (); }
  if extptr != std::ptr::null_mut ()
    { *extptr = suffix; }
  suffix = suffix.offset (1);
  /* Let those poor souls who live with DOS 8+3 file name limits get
     some solace by treating foo.cgz as if it were foo.c.gz, etc.
     Only the first do loop is run if not MSDOS */
  loop
    {
      /* C 1563-1565: `for (compr = compressors; compr->suffix != NULL; compr++)`  */
      compr = &raw mut compressors[0];
      while (*compr).suffix != std::ptr::null ()
	{
	  if streq ((*compr).suffix, suffix)
	    { return compr; }
	  compr = compr.offset (1);
	}
      if !MSDOS
	{ break; }			/* do it only once: not really a loop */
      if extptr != std::ptr::null_mut ()
	{
	  suffix = suffix.offset (1);
	  *extptr = suffix;
	}
      if !(*suffix != 0)
	{ break; }
    }
  return std::ptr::null_mut ();
}



/*
 * Return a language given the name.
 */
unsafe fn get_language_from_langname (name: *const c_char) -> *mut language
{
  let mut lang: *mut language;

  if name == std::ptr::null ()
    { error (c"empty language name".as_ptr ()); }
  else
    {
      lang = &raw mut lang_names[0];
      while (*lang).name != std::ptr::null ()
	{
	  if streq (name, (*lang).name)
	    { return lang; }
	  lang = lang.offset (1);
	}
      error (c"unknown language \"%s\"".as_ptr (), name);
    }

  return std::ptr::null_mut ();
}


/*
 * Return a language given the interpreter name.
 */
unsafe fn get_language_from_interpreter (interpreter: *mut c_char) -> *mut language
{
  let mut lang: *mut language;
  let mut iname: *const *const c_char;

  if interpreter == std::ptr::null_mut ()
    { return std::ptr::null_mut (); }
  lang = &raw mut lang_names[0];
  while (*lang).name != std::ptr::null ()
    {
      if (*lang).interpreters != std::ptr::null ()
	{
	  iname = (*lang).interpreters;
	  while *iname != std::ptr::null ()
	    {
	      if streq (*iname, interpreter)
		{ return lang; }
	      iname = iname.offset (1);
	    }
	}
      lang = lang.offset (1);
    }

  return std::ptr::null_mut ();
}



/*
 * Return a language given the file name.
 */
unsafe fn get_language_from_filename (mut file: *mut c_char, case_sensitive: bool) -> *mut language
{
  let mut lang: *mut language;
  let mut name: *const *const c_char; let mut ext: *const *const c_char;
  let mut suffix: *const c_char;
  let mut slash: *mut c_char;

  /* Try whole file name first. */
  slash = strrchr (file, b'/' as c_int);
  if slash != std::ptr::null_mut ()
    { file = slash.offset (1); }
  /* C 1634-1637: `#ifdef DOS_NT` is dead in the frozen configuration (recorded, not
     transcribed): the `else if (file[0] && file[1] == ':') file += 2;` arm is not compiled.  */
  lang = &raw mut lang_names[0];
  while (*lang).name != std::ptr::null ()
    {
      if (*lang).filenames != std::ptr::null ()
	{
	  name = (*lang).filenames;
	  while *name != std::ptr::null ()
	    {
	      if if case_sensitive { streq (*name, file) } else { strcaseeq (*name, file) }
		{ return lang; }
	      name = name.offset (1);
	    }
	}
      lang = lang.offset (1);
    }

  /* If not found, try suffix after last dot. */
  suffix = strrchr (file, b'.' as c_int);
  if suffix == std::ptr::null_mut ()
    { return std::ptr::null_mut (); }
  suffix = suffix.offset (1);
  lang = &raw mut lang_names[0];
  while (*lang).name != std::ptr::null ()
    {
      if (*lang).suffixes != std::ptr::null ()
	{
	  ext = (*lang).suffixes;
	  while *ext != std::ptr::null ()
	    {
	      if if case_sensitive { streq (*ext, suffix) } else { strcaseeq (*ext, suffix) }
		{ return lang; }
	      ext = ext.offset (1);
	    }
	}
      lang = lang.offset (1);
    }
  return std::ptr::null_mut ();
}


/* `<string.h>`: `char *strstr (const char *haystack, const char *needle);` -- the
   `string.rs` mirror does not carry it (the make-docfile.rs 87 precedent).  */
unsafe extern "C" {
  pub fn strstr (haystack: *const c_char, needle: *const c_char) -> *mut c_char;
}

/* `<stdio.h>`: `int fseek (FILE *stream, long offset, int whence);` -- no mirror carries it
   yet (the emacsclient.rs / sysdep.rs precedent keeps it at the use site).  */
unsafe extern "C" {
  pub fn fseek (stream: *mut FILE, offset: c_long, whence: c_int) -> c_int;
}

/* `<stdio.h>`: `int remove (const char *filename);` -- the `sysstdio.rs` mirror does not
   carry it.  */
unsafe extern "C" {
  pub fn remove (filename: *const c_char) -> c_int;
}

/*
 * This routine is called on each file argument.
 */
unsafe fn process_file_name (file: *mut c_char, lang: *mut language)
{
  let mut inf: *mut FILE;
  let mut fdp: *mut fdesc;
  let mut compr: *mut compressor;
  let mut compressed_name: *mut c_char;
  let mut uncompressed_name: *mut c_char;
  let mut ext: *mut c_char = std::ptr::null_mut ();
  /* C's `char *real_name UNINIT, *tmp_name UNINIT;` -- `UNINIT` is `<conf_post.h>`'s zero
     marker in this configuration (PORTING.md 1.5).  */
  let mut real_name: *mut c_char = std::ptr::null_mut ();
  let mut tmp_name: *mut c_char = std::ptr::null_mut ();
  let mut retval: c_int;

  canonicalize_filename (file);
  if streq (file, tagfile) && !streq (tagfile, c"-".as_ptr ())
    {
      error (c"skipping inclusion of %s in self.".as_ptr (), file);
      return;
    }
  compr = get_compressor_from_suffix (file, &raw mut ext);
  if compr != std::ptr::null_mut ()
    {
      compressed_name = file;
      uncompressed_name = savenstr (file, ext.offset_from (file));
    }
  else
    {
      compressed_name = std::ptr::null_mut ();
      uncompressed_name = file;
    }

  /* If the canonicalized uncompressed name
     has already been dealt with, skip it silently.  */
  /* C's `cleanup:` label below is the port's labelled block (PORTING.md 2.2); every C
     `goto cleanup` is a `break 'cleanup`, and the block ends where the label stands.  */
  'cleanup: {
  fdp = fdhead;
  while fdp != std::ptr::null_mut ()
    {
      assert! ((*fdp).infname != std::ptr::null_mut ());
      if streq (uncompressed_name, (*fdp).infname)
	{ break 'cleanup; }
      fdp = (*fdp).next;
    }

  inf = fopen (file, c"r".as_ptr ());		/* C: `"r" FOPEN_BINARY` */
  if inf != std::ptr::null_mut ()
    { real_name = file; }
  else
    {
      let mut file_errno: c_int = *errno ();
      if compressed_name != std::ptr::null_mut ()
	{
	  /* Try with the given suffix.  */
	  inf = fopen (uncompressed_name, c"r".as_ptr ());
	  if inf != std::ptr::null_mut ()
	    { real_name = uncompressed_name; }
	}
      else
	{
	  /* Try all possible suffixes.  */
	  compr = &raw mut compressors[0];
	  while (*compr).suffix != std::ptr::null ()
	    {
	      compressed_name = concat (file, c".".as_ptr (), (*compr).suffix);
	      inf = fopen (compressed_name, c"r".as_ptr ());
	      if inf != std::ptr::null_mut ()
		{
		  real_name = compressed_name;
		  break;
		}
	      if MSDOS
		{
		  let mut suf: *mut c_char =
		    compressed_name.offset (strlen (file) as isize);
		  let mut suflen: size_t = strlen ((*compr).suffix) + 1;
		  while *suf.offset (1) != 0
		    {
		      memmove (suf as *mut c_void, suf.offset (1) as *const c_void, suflen);
		      inf = fopen (compressed_name, c"r".as_ptr ());
		      if inf != std::ptr::null_mut ()
			{
			  real_name = compressed_name;
			  break;
			}
		      suf = suf.offset (1);
		      suflen -= 1;
		    }
		  if inf != std::ptr::null_mut ()
		    { break; }
		}
	      free (compressed_name as *mut c_void);
	      compressed_name = std::ptr::null_mut ();
	      compr = compr.offset (1);
	    }
	}
      if !(inf != std::ptr::null_mut ())
	{
	  *errno () = file_errno;
	  perror (file);
	  break 'cleanup;
	}
    }

  if real_name == compressed_name
    {
      fclose (inf);
      tmp_name = etags_mktmp ();
      if tmp_name == std::ptr::null_mut ()
	{ inf = std::ptr::null_mut (); }
      else
	{
	  /* C 1764-1783: `#if MSDOS || defined (DOS_NT)` is dead in the frozen configuration
	     (recorded, not transcribed); the live arm is the `#else` below.  */
	  let mut new_real_name: *mut c_char = escape_shell_arg_string (real_name);
	  let mut new_tmp_name: *mut c_char = escape_shell_arg_string (tmp_name);
	  let mut buf_len: c_int =
	    (strlen ((*compr).command) + strlen (c"  > ".as_ptr ()) + strlen (new_real_name)
	     + strlen (new_tmp_name) + 1) as c_int;
	  let mut cmd: *mut c_char = xmalloc (buf_len as ptrdiff_t) as *mut c_char;
	  snprintf (cmd, buf_len as size_t, c"%s %s > %s".as_ptr (),
		    (*compr).command, new_real_name, new_tmp_name);
	  free (new_real_name as *mut c_void);
	  free (new_tmp_name as *mut c_void);
	  inf = if system (cmd) == -1
	    { std::ptr::null_mut () }
	  else
	    { fopen (tmp_name, c"r".as_ptr ()) };
	  free (cmd as *mut c_void);
	}

      if !(inf != std::ptr::null_mut ())
	{
	  perror (real_name);
	  break 'cleanup;
	}
    }

  process_file (inf, uncompressed_name, lang);

  retval = fclose (inf);
  if real_name == compressed_name
    {
      remove (tmp_name);
      free (tmp_name as *mut c_void);
    }
  if retval < 0
    { pfatal (file); }

  }	/* C's `cleanup:` label */

  if compressed_name != file
    { free (compressed_name as *mut c_void); }
  if uncompressed_name != file
    { free (uncompressed_name as *mut c_void); }
  last_node = std::ptr::null_mut ();
  curfdp = std::ptr::null_mut ();
  return;
}

unsafe fn process_file (fh: *mut FILE, fn_: *mut c_char, lang: *mut language)
{
  /* C's function-scope `static const fdesc emptyfdesc;` is a zero-filled object with no
     symbol; the port keeps it as a `const` item whose bytes are C's (PORTING.md 0.2: C's
     `static` gets no symbol, and this one is never written).  */
  const emptyfdesc: fdesc =
    fdesc { next: std::ptr::null_mut (), infname: std::ptr::null_mut (),
	    infabsname: std::ptr::null_mut (), infabsdir: std::ptr::null_mut (),
	    taggedfname: std::ptr::null_mut (), lang: std::ptr::null_mut (),
	    prop: std::ptr::null_mut (), usecharno: false, written: false };
  let mut fdp: *mut fdesc;

  infilename = fn_;
  /* Create a new input file description entry. */
  fdp = xmalloc (std::mem::size_of::<fdesc> () as ptrdiff_t) as *mut fdesc;
  *fdp = emptyfdesc;
  (*fdp).next = fdhead;
  (*fdp).infname = savestr (fn_);
  (*fdp).lang = lang;
  (*fdp).infabsname = absolute_filename (fn_, cwd);
  (*fdp).infabsdir = absolute_dirname (fn_, cwd);
  if filename_is_absolute (fn_)
    {
      /* An absolute file name.  Canonicalize it. */
      (*fdp).taggedfname = absolute_filename (fn_, std::ptr::null_mut ());
    }
  else
    {
      /* A file name relative to cwd.  Make it relative
	 to the directory of the tags file. */
      (*fdp).taggedfname = relative_filename (fn_, tagfiledir);
    }
  (*fdp).usecharno = true;	/* use char position when making tags */
  (*fdp).prop = std::ptr::null_mut ();
  (*fdp).written = false;	/* not written on tags file yet */

  fdhead = fdp;
  curfdp = fdhead;		/* the current file description */

  find_entries (fh);

  /* If not Ctags, and if this is not metasource and if it contained no #line
     directives, we can write the tags and free all nodes pointing to
     curfdp. */
  if !ctags
    && (*curfdp).usecharno	/* no #line directives in this file */
    && !(*(*curfdp).lang).metasource
    {
      let mut np: *mut node;
      let mut prev: *mut node;

      /* Look for the head of the sublist relative to this file.  See add_node
	 for the structure of the node tree. */
      prev = std::ptr::null_mut ();
      np = nodehead;
      while np != std::ptr::null_mut ()
	{
	  if (*np).fdp == curfdp
	    { break; }
	  prev = np;
	  np = (*np).left;
	}

      /* If we generated tags for this file, write and delete them. */
      if np != std::ptr::null_mut ()
	{
	  /* This is the head of the last sublist, if any.  The following
	     instructions depend on this being true. */
	  assert! ((*np).left == std::ptr::null_mut ());

	  assert! (fdhead == curfdp);
	  assert! ((*last_node).fdp == curfdp);
	  put_entries (np);	/* write tags for file curfdp->taggedfname */
	  free_tree (np);	/* remove the written nodes */
	  if prev == std::ptr::null_mut ()
	    { nodehead = std::ptr::null_mut (); }	/* no nodes left */
	  else
	    { (*prev).left = std::ptr::null_mut (); }	/* delete the pointer to the sublist */
	}
    }
}

unsafe fn reset_input (inf: *mut FILE)
{
  if fseek (inf, 0, SEEK_SET) != 0
    { perror (infilename); }
}

/*
 * This routine opens the specified file and calls the function
 * which finds the function and type definitions.
 */
unsafe fn find_entries (inf: *mut FILE)
{
  let mut cp: *mut c_char;
  let mut lang: *mut language = (*curfdp).lang;
  let mut parser: Option<Lang_function> = None;

  /* If user specified a language, use it. */
  if lang != std::ptr::null_mut () && (*lang).function.is_some ()
    {
      parser = (*lang).function;
    }

  /* Else try to guess the language given the file name. */
  if parser.is_none ()
    {
      lang = get_language_from_filename ((*curfdp).infname, true);

      /* Disambiguate file names between Objc and Mercury. */
      if lang != std::ptr::null_mut () && strcmp ((*lang).name, c"objc".as_ptr ()) == 0
	{ test_objc_is_mercury ((*curfdp).infname, &raw mut lang); }

      if lang != std::ptr::null_mut () && (*lang).function.is_some ()
	{
	  (*curfdp).lang = lang;
	  parser = (*lang).function;
	}
    }

  /* Else look for sharp-bang as the first two characters. */
  if parser.is_none ()
    && readline_internal (&raw mut lb, inf, infilename, false) > 0
    && lb.len >= 2
    && *lb.buffer == b'#' as c_char
    && *lb.buffer.offset (1) == b'!' as c_char
    {
      let mut lp: *mut c_char;

      /* Set lp to point at the first char after the last slash in the
         line or, if no slashes, at the first nonblank.  Then set cp to
	 the first successive blank and terminate the string. */
      lp = strrchr (lb.buffer.offset (2), b'/' as c_int);
      if lp != std::ptr::null_mut ()
	{ lp = lp.offset (1); }
      else
	{ lp = skip_spaces (lb.buffer.offset (2)); }
      cp = skip_non_spaces (lp);
      /* If the "interpreter" turns out to be "env", the real interpreter is
	 the next word.  */
      if cp > lp && strneq (lp, c"env".as_ptr (), cp.offset_from (lp) as size_t)
	{
	  lp = skip_spaces (cp);
	  cp = skip_non_spaces (lp);
	}
      *cp = 0;

      if *lp != 0
	{
	  lang = get_language_from_interpreter (lp);
	  if lang != std::ptr::null_mut () && (*lang).function.is_some ()
	    {
	      (*curfdp).lang = lang;
	      parser = (*lang).function;
	    }
	}
    }

  reset_input (inf);

  /* Else try to guess the language given the case insensitive file name. */
  if parser.is_none ()
    {
      lang = get_language_from_filename ((*curfdp).infname, false);
      if lang != std::ptr::null_mut () && (*lang).function.is_some ()
	{
	  (*curfdp).lang = lang;
	  parser = (*lang).function;
	}
    }

  /* Else try Fortran or C if that fallback is not disabled. */
  if parser.is_none ()
    {
      if fallback_lang != 0
	{
	  let mut old_last_node: *mut node = last_node;

	  (*curfdp).lang = get_language_from_langname (c"fortran".as_ptr ());
	  find_entries (inf);

	  if old_last_node == last_node
	    /* No Fortran entries found.  Try C. */
	    {
	      reset_input (inf);
	      (*curfdp).lang = get_language_from_langname (
		if cplusplus { c"c++".as_ptr () } else { c"c".as_ptr () });
	      find_entries (inf);
	    }
	  return;
	}
      /* If fallbacks are disabled, treat files without a language as if
	 '--language=none' was specified for them. */
      (*curfdp).lang = get_language_from_langname (c"none".as_ptr ());
      parser = (*(*curfdp).lang).function;
    }

  if no_line_directive == 0
    && (*curfdp).lang != std::ptr::null_mut () && (*(*curfdp).lang).metasource
    /* It may be that this is a bingo.y file, and we already parsed a bingo.c
       file, or anyway we parsed a file that is automatically generated from
       this one.  If this is the case, the bingo.c file contained #line
       directives that generated tags pointing to this file.  Let's delete
       them all before parsing this file, which is the real source. */
    {
      let mut fdpp: *mut *mut fdesc = &raw mut fdhead;
      while *fdpp != std::ptr::null_mut ()
	{
	  if *fdpp != curfdp
	    && streq ((**fdpp).taggedfname, (*curfdp).taggedfname)
	  /* We found one of those!  We must delete both the file description
	     and all tags referring to it. */
	    {
	      let mut badfdp: *mut fdesc = *fdpp;

	      /* Delete the tags referring to badfdp->taggedfname
		 that were obtained from badfdp->infname. */
	      invalidate_nodes (badfdp, &raw mut nodehead);

	      *fdpp = (*badfdp).next; /* remove the bad description from the list */
	      free_fdesc (badfdp);
	    }
	  else
	    { fdpp = &raw mut (**fdpp).next; } /* advance the list pointer */
	}
    }

  assert! (parser.is_some ());

  /* Generic initializations before reading from file. */
  linebuffer_setlen (&raw mut filebuf, 0); /* reset the file buffer */

  /* Generic initializations before parsing file with readline. */
  lineno = 0;		       /* reset global line number */
  charno = 0;		       /* reset global char number */
  linecharno = 0;	       /* reset global char number of line start */

  (parser.unwrap ()) (inf);

  regex_tag_multiline ();
}

/* `<string.h>`: `size_t strnlen (const char *, size_t);` -- the `string.rs` mirror does not
   carry it (the emacsclient.rs precedent keeps it at the use site).  */
unsafe extern "C" {
  pub fn strnlen (s: *const c_char, maxlen: size_t) -> size_t;
}

/*
 * Check whether an implicitly named tag should be created,
 * then call `pfnote'.
 * NAME is a string that is internally copied by this function.
 *
 * TAGS format specification
 * Idea by Sam Kendall <kendall@mv.mv.com> (1997)
 * The following is explained in some more detail in etc/ETAGS.EBNF.
 *
 * make_tag creates tags with "implicit tag names" (unnamed tags)
 * if the following are all true, assuming NONAM=" \f\t\n\r()=,;":
 *  1. NAME does not contain any of the characters in NONAM;
 *  2. LINESTART contains name as either a rightmost, or rightmost but
 *     one character, substring;
 *  3. the character, if any, immediately before NAME in LINESTART must
 *     be a character in NONAM;
 *  4. the character, if any, immediately after NAME in LINESTART must
 *     also be a character in NONAM.
 *
 * The implementation uses the notinname() macro, which recognizes the
 * characters stored in the string `nonam'.
 * etags.el needs to use the same characters that are in NONAM.
 */
unsafe fn make_tag (name: *const c_char,	/* tag name, or NULL if unnamed */
		    namelen: ptrdiff_t,	/* tag length */
		    is_func: bool,		/* tag is a function */
		    linestart: *mut c_char,	/* start of the line where tag is */
		    linelen: ptrdiff_t,	/* length of the line where tag is */
		    lno: intmax_t,		/* line number */
		    cno: intmax_t)		/* character number */
{
  let mut named: bool = name != std::ptr::null () && namelen > 0;
  let mut nname: *mut c_char = std::ptr::null_mut ();

  if debug != 0
    { fprintf (stderr, c"%s on %s:%jd: %s\n".as_ptr (),	/* C: `"%"PRIdMAX` */
	       if named { name } else { c"(unnamed)".as_ptr () },
	       (*curfdp).taggedfname, lno, linestart); }

  if !ctags && named		/* maybe set named to false */
    /* Let's try to make an implicit tag name, that is, create an unnamed tag
       such that etags.el can guess a name from it. */
    {
      let mut i: ptrdiff_t;
      let mut cp: *const c_char = name;

      i = 0;
      while i < namelen
	{
	  let ch: c_uchar = *cp as c_uchar;
	  cp = cp.offset (1);
	  if notinname (ch)
	    { break; }
	  i += 1;
	}
      if i == namelen				/* rule #1 */
	{
	  cp = linestart.offset (linelen - namelen);
	  if notinname (*linestart.offset (linelen - 1) as c_uchar)
	    { cp = cp.offset (-1); }		/* rule #4 */
	  if cp >= linestart			/* rule #2 */
	      && (cp == linestart
		  || notinname (*cp.offset (-1) as c_uchar))	/* rule #3 */
	      && strneq (name, cp, namelen as size_t)	/* rule #2 */
	    { named = false; }	/* use implicit tag name */
	}
    }

  if named
    { nname = savenstr (name, namelen); }

  pfnote (nname, is_func, linestart, linelen, lno, cno);
}

/* Record a tag. */
unsafe fn pfnote (name: *mut c_char,	/* tag name, or NULL if unnamed */
		  is_func: bool,		/* tag is a function */
		  linestart: *mut c_char,	/* start of the line where tag is */
		  linelen: ptrdiff_t,	/* length of the line where tag is */
		  lno: intmax_t,		/* line number */
		  cno: intmax_t)		/* character number */

{
  let mut np: *mut node;

  if (ctags && name == std::ptr::null_mut ())
      /* We used to have an assertion here for the case below, but if we hit
	 that case, it just means our parser got confused, and there's nothing
	 to do about such empty "tags".  */
      || (!ctags && name != std::ptr::null_mut () && *name == 0)
    { return; }

  np = xmalloc (std::mem::size_of::<node> () as ptrdiff_t) as *mut node;

  /* If ctags mode, change name "main" to M<thisfilename>. */
  if ctags && !cxref_style && streq (name, c"main".as_ptr ())
    {
      let mut fp: *mut c_char = strrchr ((*curfdp).taggedfname, b'/' as c_int);
      (*np).name = concat (c"M".as_ptr (),
			   if fp == std::ptr::null_mut () { (*curfdp).taggedfname }
			   else { fp.offset (1) },
			   c"".as_ptr ());
      fp = strrchr ((*np).name, b'.' as c_int);
      if fp != std::ptr::null_mut () && *fp.offset (1) != 0 && *fp.offset (2) == 0
	{ *fp = 0; }
    }
  else
    { (*np).name = name; }
  (*np).valid = true;
  (*np).been_warned = false;
  (*np).fdp = curfdp;
  (*np).is_func = is_func;
  (*np).lno = lno;
  if (*(*np).fdp).usecharno
    /* Our char numbers are 0-base, because of C language tradition?
       ctags compatibility?  old versions compatibility?   I don't know.
       Anyway, since emacs's are 1-base we expect etags.el to take care
       of the difference.  If we wanted to have 1-based numbers, we would
       uncomment the +1 below. */
    { (*np).cno = cno /* + 1 */ ; }
  else
    { (*np).cno = invalidcharno; }
  /* C's `np->left = np->right = NULL;`: the port writes the two assignments in C's
     right-to-left order (PORTING.md 0.1-13, the chained-assignment deviation).  */
  (*np).right = std::ptr::null_mut ();
  (*np).left = std::ptr::null_mut ();
  if ctags && !cxref_style
    {
      if strnlen (linestart, 50) < 50
	{ (*np).regex = concat (linestart, c"$".as_ptr (), c"".as_ptr ()); }
      else
	{ (*np).regex = savenstr (linestart, 50); }
    }
  else
    { (*np).regex = savenstr (linestart, linelen); }

  add_node (np, &raw mut nodehead);
}

/*
 * Utility functions and data to avoid recursion.
 */

/* C 2213-2217: `typedef struct stack_entry { node *np; struct stack_entry *next; }
   stkentry;`  */
#[repr(C)]
pub struct stkentry
{
  pub np: *mut node,
  pub next: *mut stkentry,
}

unsafe fn push_node (np: *mut node, stack_top: *mut *mut stkentry)
{
  if np != std::ptr::null_mut ()
    {
      let mut new: *mut stkentry =
	xmalloc (std::mem::size_of::<stkentry> () as ptrdiff_t) as *mut stkentry;

      (*new).np = np;
      (*new).next = *stack_top;
      *stack_top = new;
    }
}

unsafe fn pop_node (stack_top: *mut *mut stkentry) -> *mut node
{
  let mut ret: *mut node = std::ptr::null_mut ();

  if *stack_top != std::ptr::null_mut ()
    {
      let mut old_start: *mut stkentry = *stack_top;

      ret = (**stack_top).np;
      *stack_top = (**stack_top).next;
      free (old_start as *mut c_void);
    }
  return ret;
}

/*
 * free_tree ()
 *	emulate recursion on left children, iterate on right children.
 */
unsafe fn free_tree (mut np: *mut node)
{
  let mut stack: *mut stkentry = std::ptr::null_mut ();

  while np != std::ptr::null_mut ()
    {
      /* Descent on left children.  */
      while (*np).left != std::ptr::null_mut ()
	{
	  push_node (np, &raw mut stack);
	  np = (*np).left;
	}
      /* Free node without left children.  */
      let mut node_right: *mut node = (*np).right;
      free ((*np).name as *mut c_void);
      free ((*np).regex as *mut c_void);
      free (np as *mut c_void);
      if node_right == std::ptr::null_mut ()
	{
	  /* Backtrack to find a node with right children, while freeing nodes
	     that don't have right children.  */
	  while node_right == std::ptr::null_mut ()
	    {
	      np = pop_node (&raw mut stack);
	      if np == std::ptr::null_mut ()
		{ break; }
	      node_right = (*np).right;
	      free ((*np).name as *mut c_void);
	      free ((*np).regex as *mut c_void);
	      free (np as *mut c_void);
	    }
	}
      /* Free right children.  */
      np = node_right;
    }
}

/*
 * free_fdesc ()
 *	delete a file description
 */
unsafe fn free_fdesc (fdp: *mut fdesc)
{
  free ((*fdp).infname as *mut c_void);
  free ((*fdp).infabsname as *mut c_void);
  free ((*fdp).infabsdir as *mut c_void);
  free ((*fdp).taggedfname as *mut c_void);
  free ((*fdp).prop as *mut c_void);
  free (fdp as *mut c_void);
}

/*
 * add_node ()
 *	Adds a node to the tree of nodes.  In etags mode, sort by file
 *  	name.  In ctags mode, sort by tag name.  Make no attempt at
 *    	balancing.
 *
 *	add_node is the only function allowed to add nodes, so it can
 *	maintain state.
 */
unsafe fn add_node (np: *mut node, cur_node_p: *mut *mut node)
{
  let mut cur_node: *mut node = *cur_node_p;

  /* Make the first node.  */
  if cur_node == std::ptr::null_mut ()
    {
      *cur_node_p = np;
      last_node = np;
      return;
    }

  if !ctags
    /* Etags Mode */
    {
      /* For each file name, tags are in a linked sublist on the right
	 pointer.  The first tags of different files are a linked list
	 on the left pointer.  last_node points to the end of the last
	 used sublist. */
      if last_node != std::ptr::null_mut () && (*last_node).fdp == (*np).fdp
	{
	  /* Let's use the same sublist as the last added node. */
	  assert! ((*last_node).right == std::ptr::null_mut ());
	  (*last_node).right = np;
	  last_node = np;
	}
      else
	{
	   while (*cur_node).fdp != (*np).fdp
	     {
	       if (*cur_node).left == std::ptr::null_mut ()
		 { break; }
	       /* The head of this sublist is not good for us.  Let's try the
		  next one. */
	       cur_node = (*cur_node).left;
	     }
	   if (*cur_node).left != std::ptr::null_mut ()
	     {
	       /* Scanning the list we found the head of a sublist which is
		  good for us.  Let's scan this sublist. */
	       if (*cur_node).right != std::ptr::null_mut ()
		 {
		   cur_node = (*cur_node).right;
		   while (*cur_node).right != std::ptr::null_mut ()
		     { cur_node = (*cur_node).right; }
		 }
	       /* Make a new node in this sublist.  */
	       (*cur_node).right = np;
	     }
	   else
	     {
	       /* Make a new sublist.  */
	       (*cur_node).left = np;
	     }
	   last_node = np;
	}
    } /* if ETAGS mode */
  else
    {
      /* Ctags Mode */
      let mut next_node: *mut *mut node = &raw mut cur_node;

      loop
	{
	  cur_node = *next_node;
	  if cur_node == std::ptr::null_mut ()
	    { break; }
	  let mut dif: c_int = strcmp ((*np).name, (*cur_node).name);
	  /*
	   * If this tag name matches an existing one, then
	   * do not add the node, but maybe print a warning.
	   */
	  if dif == 0 && no_duplicates != 0
	    {
	      if (*np).fdp == (*cur_node).fdp
		{
		  if no_warnings == false
		    {
		      /* C: `"Duplicate entry in file %s, " "line %"PRIdMAX": %s\n"`.  */
		      fprintf (stderr,
			       c"Duplicate entry in file %s, line %jd: %s\n".as_ptr (),
			       (*(*np).fdp).infname, lineno, (*np).name);
		      fprintf (stderr, c"Second entry ignored\n".as_ptr ());
		    }
		}
	      else if (*cur_node).been_warned == false && no_warnings == false
		{
		  fprintf
		    (stderr,
		     c"Duplicate entry in files %s and %s: %s (Warning only)\n".as_ptr (),
		     (*(*np).fdp).infname, (*(*cur_node).fdp).infname, (*np).name);
		  (*cur_node).been_warned = true;
		}
	      return;
	    }
	  else
	    { next_node = if dif < 0 { &raw mut (*cur_node).left }
			  else { &raw mut (*cur_node).right }; }
	}
      *next_node = np;
      last_node = np;
    } /* if CTAGS mode */
}

/*
 * invalidate_nodes ()
 *	Scan the node tree and invalidate all nodes pointing to the
 *	given file description (CTAGS case) or free them (ETAGS case).
 */
unsafe fn invalidate_nodes (badfdp: *mut fdesc, npp: *mut *mut node)
{
  let mut np: *mut node = *npp;
  let mut stack: *mut stkentry = std::ptr::null_mut ();

  if ctags
    {
      while np != std::ptr::null_mut ()
	{
	  /* Push all the left children on the stack.  */
	  while (*np).left != std::ptr::null_mut ()
	    {
	      push_node (np, &raw mut stack);
	      np = (*np).left;
	    }
	  /* Invalidate this node.  */
	  if (*np).fdp == badfdp
	    { (*np).valid = false; }
	  if (*np).right == std::ptr::null_mut ()
	    {
	      /* Pop nodes from stack, invalidating them, until we find one
		 with a right child.  */
	      loop
		{
		  np = pop_node (&raw mut stack);
		  if np == std::ptr::null_mut ()
		    { break; }
		  if (*np).fdp == badfdp
		    { (*np).valid = false; }
		  if (*np).right != std::ptr::null_mut ()
		    { break; }
		}
	    }
	  /* Process the right child, if any.  */
	  if np != std::ptr::null_mut ()
	    { np = (*np).right; }
	}
    }
  else
    {
      /* C's `node super_root, *np_parent = NULL;` -- the two fields assigned below are set
	 on a zero-filled record (PORTING.md 1.5/0.1-13).  */
      let mut super_root: node = std::mem::zeroed ();
      let mut np_parent: *mut node = std::ptr::null_mut ();

      super_root.left = np;
      super_root.fdp = (-1 as isize) as *mut fdesc;
      np = &raw mut super_root;

      while np != std::ptr::null_mut ()
	{
	  /* Descent on left children until node with BADFP.  */
	  while np != std::ptr::null_mut () && (*np).fdp != badfdp
	    {
	      assert! ((*np).fdp != std::ptr::null_mut ());
	      np_parent = np;
	      np = (*np).left;
	    }
	  if np != std::ptr::null_mut ()
	    {
	      (*np_parent).left = (*np).left; /* detach subtree from the tree */
	      (*np).left = std::ptr::null_mut ();	  /* isolate it */
	      free_tree (np);		  /* free it */

	      /* Continue with rest of tree.  */
	      np = (*np_parent).left;
	    }
	}
      *npp = super_root.left;
    }
}



/* Length of a non-negative number's decimal representation. */
unsafe fn number_len (mut num: intmax_t) -> c_int
{
  let mut len: c_int = 1;
  loop
    {
      num /= 10;
      if !(num > 0)
	{ break; }
      len += 1;
    }
  return len;
}

unsafe fn total_size_of_entries (mut np: *mut node) -> ptrdiff_t
{
  let mut total: ptrdiff_t = 0;

  while np != std::ptr::null_mut ()
    {
      if (*np).valid
	{
	  total += strlen ((*np).regex) as ptrdiff_t + 1;		/* pat\177 */
	  if (*np).name != std::ptr::null_mut ()
	    { total += strlen ((*np).name) as ptrdiff_t + 1; }	/* name\001 */
	  total += number_len ((*np).lno) as ptrdiff_t + 1;		/* lno, */
	  if (*np).cno != invalidcharno				/* cno */
	    { total += number_len ((*np).cno) as ptrdiff_t; }
	  total += 1;					/* newline */
	}
      np = (*np).right;
    }

  return total;
}

unsafe fn put_entry (np: *mut node)
{
  let mut sp: *mut c_char;
  static mut fdp: *mut fdesc = std::ptr::null_mut ();

  /* Output this entry */
  if (*np).valid
    {
      if !ctags
	{
	  /* Etags mode */
	  if fdp != (*np).fdp
	    {
	      fdp = (*np).fdp;
	      fprintf (tagf, c"\x0c\n%s,%ld\n".as_ptr (),	/* C: `"%"PRIdPTR` */
		       (*fdp).taggedfname, total_size_of_entries (np));
	      (*fdp).written = true;
	    }
	  fputs ((*np).regex, tagf);
	  fputc (127 /* '\177' */, tagf);
	  if (*np).name != std::ptr::null_mut ()
	    {
	      fputs ((*np).name, tagf);
	      fputc (1 /* '\001' */, tagf);
	    }
	  fprintf (tagf, c"%jd,".as_ptr (), (*np).lno);	/* C: `"%"PRIdMAX","` */
	  if (*np).cno != invalidcharno
	    { fprintf (tagf, c"%jd".as_ptr (), (*np).cno); }	/* C: `"%"PRIdMAX` */
	  fputs (c"\n".as_ptr (), tagf);
	}
      else
	{
	  /* Ctags mode */
	  if (*np).name == std::ptr::null_mut ()
	    { error (c"internal error: NULL name in ctags mode.".as_ptr ()); }

	  if cxref_style
	    {
	      if vgrind_style
		{ fprintf (stdout, c"%s %s %jd\n".as_ptr (),	/* C: `"%"PRIdMAX` */
			   (*np).name, (*(*np).fdp).taggedfname, ((*np).lno + 63) / 64); }
	      else
		{ fprintf (stdout, c"%-16s %3jd %-16s %s\n".as_ptr (),	/* C: `"%"PRIdMAX` */
			   (*np).name, (*np).lno, (*(*np).fdp).taggedfname, (*np).regex); }
	    }
	  else
	    {
	      fprintf (tagf, c"%s\t%s\t".as_ptr (), (*np).name, (*(*np).fdp).taggedfname);

	      if (*np).is_func
		{		/* function or #define macro with args */
		  putc (searchar as c_int, tagf);
		  putc (b'^' as c_int, tagf);

		  sp = (*np).regex;
		  while *sp != 0
		    {
		      if *sp == b'\\' as c_char || *sp == searchar
			{ putc (b'\\' as c_int, tagf); }
		      putc (*sp as c_int, tagf);
		      sp = sp.offset (1);
		    }
		  putc (searchar as c_int, tagf);
		}
	      else
		{		/* anything else; text pattern inadequate */
		  fprintf (tagf, c"%jd".as_ptr (), (*np).lno);	/* C: `"%"PRIdMAX` */
		}
	      putc (b'\n' as c_int, tagf);
	    }
	}
    } /* if this node contains a valid tag */
}

unsafe fn put_entries (mut np: *mut node)
{
  let mut stack: *mut stkentry = std::ptr::null_mut ();

  if np == std::ptr::null_mut ()
    { return; }

  if ctags
    {
      while np != std::ptr::null_mut ()
	{
	  /* Stack subentries that precede this one.  */
	  while (*np).left != std::ptr::null_mut ()
	    {
	      push_node (np, &raw mut stack);
	      np = (*np).left;
	    }
	  /* Output this subentry.  */
	  put_entry (np);
	  /* Stack subentries that follow this one.  */
	  while (*np).right == std::ptr::null_mut ()
	    {
	      /* Output subentries that precede the next one.  */
	      np = pop_node (&raw mut stack);
	      if np == std::ptr::null_mut ()
		{ break; }
	      put_entry (np);
	    }
	  if np != std::ptr::null_mut ()
	    { np = (*np).right; }
	}
    }
  else
    {
      push_node (np, &raw mut stack);
      loop
	{
	  np = pop_node (&raw mut stack);
	  if np == std::ptr::null_mut ()
	    { break; }
	  /* Output this subentry.  */
	  put_entry (np);
	  while (*np).right != std::ptr::null_mut ()
	    {
	      /* Output subentries that follow this one.  */
	      put_entry ((*np).right);
	      /* Stack subentries from the following files.  */
	      push_node ((*np).left, &raw mut stack);
	      np = (*np).right;
	    }
	  push_node ((*np).left, &raw mut stack);
	}
    }
}


/* C extensions.  */
pub const C_EXT: c_int = 0x00fff;		/* C extensions */
pub const C_PLAIN: c_int = 0x00000;		/* C */
pub const C_PLPL: c_int = 0x00001;		/* C++ */
pub const C_STAR: c_int = 0x00003;		/* C* */
pub const C_JAVA: c_int = 0x00005;		/* JAVA */
pub const C_AUTO: c_int = 0x01000;		/* C, but switch to C++ if `class' is met */
pub const YACC: c_int = 0x10000;		/* yacc file */

/*
 * The C symbol tables.
 */
/* C 2628-2639: `enum sym_type { ... }`.  C names the type in `C_stab_entry`'s third field,
   in `C_symtype`'s return type and in the two function-scope statics of `consider_token`,
   so the port keeps it as a `#[repr(C)]` enum with C's member names (PORTING.md 1.3) and
   the `use` below makes them visible unqualified the way C's enumerators are.  */
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum sym_type
{
  st_none,
  st_C_objprot, st_C_objimpl, st_C_objend,
  st_C_gnumacro,
  st_C_ignore, st_C_attribute, st_C_enum_bf,
  st_C_javastruct,
  st_C_operator,
  st_C_class, st_C_template,
  st_C_struct, st_C_extern, st_C_enum, st_C_define, st_C_typedef
}
use sym_type::*;

/* Feed stuff between (but not including) %[ and %] lines to:
     gperf -m 5
%[
%compare-strncmp
%enum
%struct-type
struct C_stab_entry { char *name; int c_ext; enum sym_type type; }
%%
if,		0,			st_C_ignore
for,		0,			st_C_ignore
while,		0,			st_C_ignore
switch,		0,			st_C_ignore
return,		0,			st_C_ignore
__attribute__,	0,			st_C_attribute
GTY,            0,                      st_C_attribute
@interface,	0,			st_C_objprot
@protocol,	0,			st_C_objprot
@implementation,0,			st_C_objimpl
@end,		0,			st_C_objend
import,		(C_JAVA & ~C_PLPL),	st_C_ignore
package,	(C_JAVA & ~C_PLPL),	st_C_ignore
friend,		C_PLPL,			st_C_ignore
extends,	(C_JAVA & ~C_PLPL),	st_C_javastruct
implements,	(C_JAVA & ~C_PLPL),	st_C_javastruct
interface,	(C_JAVA & ~C_PLPL),	st_C_struct
class,		0,			st_C_class
namespace,	C_PLPL,			st_C_struct
domain,		C_STAR,			st_C_struct
union,		0,			st_C_struct
struct,		0,			st_C_struct
extern,		0,			st_C_extern
enum,		0,			st_C_enum
typedef,	0,			st_C_typedef
define,		0,			st_C_define
undef,		0,			st_C_define
operator,	C_PLPL,			st_C_operator
template,	0,			st_C_template
# DEFUN used in emacs, the next three used in glibc (SYSCALL only for mach).
DEFUN,		0,			st_C_gnumacro
SYSCALL,	0,			st_C_gnumacro
ENTRY,		0,			st_C_gnumacro
PSEUDO,		0,			st_C_gnumacro
ENUM_BF,	0,			st_C_enum_bf
@SuppressWarnings,	(C_JAVA & ~C_PLPL),	st_C_attribute
# These are defined inside C functions, so currently they are not met.
# EXFUN used in glibc, DEFVAR_* in emacs.
#EXFUN,		0,			st_C_gnumacro
#DEFVAR_,	0,			st_C_gnumacro
%]
and replace lines between %< and %> with its output, then:
 - remove the #if characterset check
 - remove any #line directives
 - make in_word_set static and not inline
 - remove any 'register' qualifications from variable decls. */
/*%<*/
/* ANSI-C code produced by gperf version 3.1 */
/* Command-line: gperf -m 5 gperf.inp  */
/* Computed positions: -k'2-3' */

/* C 2690: `struct C_stab_entry { const char *name; int c_ext; enum sym_type type; };`  */
#[repr(C)]
pub struct C_stab_entry
{
  pub name: *const c_char,
  pub c_ext: c_int,
  pub r#type: sym_type
}
const _: () = core::assert! (std::mem::size_of::<C_stab_entry> () == 16);
/* maximum key range = 36, duplicates = 0 */

unsafe fn hash (str: *const c_char, len: c_int) -> c_int
{
  static asso_values: [c_char; 256] =
    [
      38, 38, 38, 38, 38, 38, 38, 38, 38, 38,
      38, 38, 38, 38, 38, 38, 38, 38, 38, 38,
      38, 38, 38, 38, 38, 38, 38, 38, 38, 38,
      38, 38, 38, 38, 38, 38, 38, 38, 38, 38,
      38, 38, 38, 38, 38, 38, 38, 38, 38, 38,
      38, 38, 38, 38, 38, 38, 38, 38, 38, 38,
      38, 38, 38, 38, 38, 38, 38, 38, 38, 29,
       3, 38, 38, 38, 38, 38, 38, 38, 23, 38,
      38, 38, 38,  0,  5,  4, 38, 38, 38, 24,
      38, 38, 38, 38, 38,  1, 38, 16, 38,  6,
      23,  0,  0, 38, 22,  0, 38, 38,  5,  0,
       0, 15,  1, 38,  6, 38,  8, 19, 38, 16,
       4,  5, 38, 38, 38, 38, 38, 38, 38, 38,
      38, 38, 38, 38, 38, 38, 38, 38, 38, 38,
      38, 38, 38, 38, 38, 38, 38, 38, 38, 38,
      38, 38, 38, 38, 38, 38, 38, 38, 38, 38,
      38, 38, 38, 38, 38, 38, 38, 38, 38, 38,
      38, 38, 38, 38, 38, 38, 38, 38, 38, 38,
      38, 38, 38, 38, 38, 38, 38, 38, 38, 38,
      38, 38, 38, 38, 38, 38, 38, 38, 38, 38,
      38, 38, 38, 38, 38, 38, 38, 38, 38, 38,
      38, 38, 38, 38, 38, 38, 38, 38, 38, 38,
      38, 38, 38, 38, 38, 38, 38, 38, 38, 38,
      38, 38, 38, 38, 38, 38, 38, 38, 38, 38,
      38, 38, 38, 38, 38, 38, 38, 38, 38, 38,
      38, 38, 38, 38, 38, 38
    ];
  let mut hval: c_int = len;

  match hval
    {
      2 =>
	{
	  hval += asso_values[*str.offset (1) as u8 as usize] as c_int;
	}
      /* C's `default:` falls through into `case 2:` (FALLTHROUGH, PORTING.md 2.2): the
	 arm carries the following arm's statement too, so the `2` arm above is the one
	 that skips the first statement.  */
      _ =>
	{
	  hval += asso_values[*str.offset (2) as u8 as usize] as c_int;
	  hval += asso_values[*str.offset (1) as u8 as usize] as c_int;
	}
    }
  return hval;
}

unsafe fn in_word_set (str: *const c_char, len: ptrdiff_t) -> *mut C_stab_entry
{
  /* C's function-scope `enum { TOTAL_KEYWORDS = 35, ... };` -- the anonymous enum's members
     are the constants below (the `at_language` precedent above).  */
  const TOTAL_KEYWORDS: c_int = 35;
  const MIN_WORD_LENGTH: c_int = 2;
  const MAX_WORD_LENGTH: c_int = 17;
  const MIN_HASH_VALUE: c_int = 2;
  const MAX_HASH_VALUE: c_int = 37;

  /* C's `static struct C_stab_entry wordlist[]` -- `static mut` because `C_stab_entry`
     carries a raw pointer (PORTING.md 0.2: Rust's `static` needs `Sync`).  */
  static mut wordlist: [C_stab_entry; 38] =
    [
      C_stab_entry { name: c"".as_ptr (), c_ext: 0, r#type: st_none },
      C_stab_entry { name: c"".as_ptr (), c_ext: 0, r#type: st_none },
      C_stab_entry { name: c"if".as_ptr (), c_ext: 0, r#type: st_C_ignore },
      C_stab_entry { name: c"".as_ptr (), c_ext: 0, r#type: st_none },
      C_stab_entry { name: c"@end".as_ptr (), c_ext: 0, r#type: st_C_objend },
      C_stab_entry { name: c"union".as_ptr (), c_ext: 0, r#type: st_C_struct },
      C_stab_entry { name: c"define".as_ptr (), c_ext: 0, r#type: st_C_define },
      C_stab_entry { name: c"import".as_ptr (), c_ext: (C_JAVA & !C_PLPL), r#type: st_C_ignore },
      C_stab_entry { name: c"template".as_ptr (), c_ext: 0, r#type: st_C_template },
      C_stab_entry { name: c"operator".as_ptr (), c_ext: C_PLPL, r#type: st_C_operator },
      C_stab_entry { name: c"@interface".as_ptr (), c_ext: 0, r#type: st_C_objprot },
      C_stab_entry { name: c"implements".as_ptr (), c_ext: (C_JAVA & !C_PLPL), r#type: st_C_javastruct },
      C_stab_entry { name: c"friend".as_ptr (), c_ext: C_PLPL, r#type: st_C_ignore },
      C_stab_entry { name: c"typedef".as_ptr (), c_ext: 0, r#type: st_C_typedef },
      C_stab_entry { name: c"return".as_ptr (), c_ext: 0, r#type: st_C_ignore },
      C_stab_entry { name: c"@implementation".as_ptr (), c_ext: 0, r#type: st_C_objimpl },
      C_stab_entry { name: c"@protocol".as_ptr (), c_ext: 0, r#type: st_C_objprot },
      C_stab_entry { name: c"interface".as_ptr (), c_ext: (C_JAVA & !C_PLPL), r#type: st_C_struct },
      C_stab_entry { name: c"extern".as_ptr (), c_ext: 0, r#type: st_C_extern },
      C_stab_entry { name: c"extends".as_ptr (), c_ext: (C_JAVA & !C_PLPL), r#type: st_C_javastruct },
      C_stab_entry { name: c"struct".as_ptr (), c_ext: 0, r#type: st_C_struct },
      C_stab_entry { name: c"domain".as_ptr (), c_ext: C_STAR, r#type: st_C_struct },
      C_stab_entry { name: c"switch".as_ptr (), c_ext: 0, r#type: st_C_ignore },
      C_stab_entry { name: c"enum".as_ptr (), c_ext: 0, r#type: st_C_enum },
      C_stab_entry { name: c"for".as_ptr (), c_ext: 0, r#type: st_C_ignore },
      C_stab_entry { name: c"namespace".as_ptr (), c_ext: C_PLPL, r#type: st_C_struct },
      C_stab_entry { name: c"class".as_ptr (), c_ext: 0, r#type: st_C_class },
      C_stab_entry { name: c"while".as_ptr (), c_ext: 0, r#type: st_C_ignore },
      C_stab_entry { name: c"undef".as_ptr (), c_ext: 0, r#type: st_C_define },
      C_stab_entry { name: c"package".as_ptr (), c_ext: (C_JAVA & !C_PLPL), r#type: st_C_ignore },
      C_stab_entry { name: c"__attribute__".as_ptr (), c_ext: 0, r#type: st_C_attribute },
      C_stab_entry { name: c"SYSCALL".as_ptr (), c_ext: 0, r#type: st_C_gnumacro },
      C_stab_entry { name: c"GTY".as_ptr (), c_ext: 0, r#type: st_C_attribute },
      C_stab_entry { name: c"ENTRY".as_ptr (), c_ext: 0, r#type: st_C_gnumacro },
      C_stab_entry { name: c"ENUM_BF".as_ptr (), c_ext: 0, r#type: st_C_enum_bf },
      C_stab_entry { name: c"PSEUDO".as_ptr (), c_ext: 0, r#type: st_C_gnumacro },
      C_stab_entry { name: c"@SuppressWarnings".as_ptr (), c_ext: (C_JAVA & !C_PLPL), r#type: st_C_attribute },
      C_stab_entry { name: c"DEFUN".as_ptr (), c_ext: 0, r#type: st_C_gnumacro }
    ];

  if len <= MAX_WORD_LENGTH as ptrdiff_t && len >= MIN_WORD_LENGTH as ptrdiff_t
    {
      let mut key: c_int = hash (str, len as c_int);

      if key <= MAX_HASH_VALUE && key >= 0
        {
          let s: *const c_char = wordlist[key as usize].name;

          if *str == *s && strncmp (str.offset (1), s.offset (1), (len - 1) as usize) == 0
	      && *s.offset (len) == 0
            { return &raw mut wordlist[key as usize]; }
        }
    }
  return std::ptr::null_mut ();
}
/*%>*/

unsafe fn C_symtype (str: *mut c_char, len: ptrdiff_t, c_ext: c_int) -> sym_type
{
  let mut se: *mut C_stab_entry = in_word_set (str, len);

  if se == std::ptr::null_mut () || ((*se).c_ext != 0 && (c_ext & (*se).c_ext) == 0)
    { return st_none; }
  return (*se).r#type;
}


/*
 * Ignoring __attribute__ ((list))
 */
static mut inattribute: bool = false;	/* looking at an __attribute__ construct */

/* Ignoring ENUM_BF (type)
 *
 */
static mut in_enum_bf: bool = false;		/* inside parentheses following ENUM_BF */

/*
 * C functions and variables are recognized using a simple
 * finite automaton.  fvdef is its state variable.
 */
/* C's anonymous `enum { fvnone, ... } fvdef;` -- the members are the constants below and the
   variable keeps C's `int` width (the `at_language` precedent above).  */
pub const fvnone: c_int = 0;			/* nothing seen */
pub const fdefunkey: c_int = 1;			/* Emacs DEFUN keyword seen */
pub const fdefunname: c_int = 2;			/* Emacs DEFUN name seen */
pub const foperator: c_int = 3;			/* func: operator keyword seen (cplpl) */
pub const fvnameseen: c_int = 4;			/* function or variable name seen */
pub const fstartlist: c_int = 5;			/* func: just after open parenthesis */
pub const finlist: c_int = 6;			/* func: in parameter list */
pub const flistseen: c_int = 7;			/* func: after parameter list */
pub const fignore: c_int = 8;			/* func: before open brace */
pub const vignore: c_int = 9;			/* var-like: ignore until ';' */
static mut fvdef: c_int = fvnone;

static mut fvextern: bool = false;		/* func or var: extern keyword seen; */

/*
 * typedefs are recognized using a simple finite automaton.
 * typdef is its state variable.
 */
pub const tnone: c_int = 0;			/* nothing seen */
pub const tkeyseen: c_int = 1;			/* typedef keyword seen */
pub const ttypeseen: c_int = 2;			/* defined type seen */
pub const tinbody: c_int = 3;			/* inside typedef body */
pub const tend: c_int = 4;			/* just before typedef tag */
pub const tignore: c_int = 5;			/* junk after typedef tag */
static mut typdef: c_int = tnone;

/*
 * struct-like structures (enum, struct and union) are recognized
 * using another simple finite automaton.  `structdef' is its state
 * variable.
 */
pub const snone: c_int = 0;			/* nothing seen yet,
				   or in struct body if bracelev > 0 */
pub const skeyseen: c_int = 1;			/* struct-like keyword seen */
pub const stagseen: c_int = 2;			/* struct-like tag seen */
pub const scolonseen: c_int = 3;			/* colon seen after struct-like tag */
static mut structdef: c_int = snone;

/*
 * When objdef is different from onone, objtag is the name of the class.
 */
static mut objtag: *const c_char = c"<uninited>".as_ptr ();

/*
 * Yet another little state machine to deal with preprocessor lines.
 */
pub const dnone: c_int = 0;			/* nothing seen */
pub const dsharpseen: c_int = 1;			/* '#' seen as first char on line */
pub const ddefineseen: c_int = 2;			/* '#' and 'define' seen */
pub const dignorerest: c_int = 3;			/* ignore rest of line */
static mut definedef: c_int = dnone;

/*
 * State machine for Objective C protocols and implementations.
 * Idea by Tom R.Hageman <tom@basil.icce.rug.nl> (1995)
 */
pub const onone: c_int = 0;			/* nothing seen */
pub const oprotocol: c_int = 1;			/* @interface or @protocol seen */
pub const oimplementation: c_int = 2;		/* @implementations seen */
pub const otagseen: c_int = 3;			/* class name seen */
pub const oparenseen: c_int = 4;			/* parenthesis before category seen */
pub const ocatseen: c_int = 5;			/* category name seen */
pub const oinbody: c_int = 6;			/* in @implementation body */
pub const omethodsign: c_int = 7;			/* in @implementation body, after +/- */
pub const omethodtag: c_int = 8;			/* after method name */
pub const omethodcolon: c_int = 9;			/* after method colon */
pub const omethodparm: c_int = 10;			/* after method parameter */
pub const oignore: c_int = 11;			/* wait for @end */
static mut objdef: c_int = onone;


/*
 * Use this structure to keep info about the token read, and how it
 * should be tagged.  Used by the make_C_tag function to build a tag.
 */
/* C 2940-2958: `static struct tok { ... } token;`  C copies this record as a whole
   (`token = savetoken` / `savetoken = token`), so the port derives `Copy`.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct tok
{
  pub line: *mut c_char,			/* string containing the token */
  pub offset: ptrdiff_t,		/* where the token starts in LINE */
  pub length: ptrdiff_t,		/* token length */
  /*
    The previous members can be used to pass strings around for generic
    purposes.  The following ones specifically refer to creating tags.  In this
    case the token contained here is the pattern that will be used to create a
    tag.
  */
  pub valid: bool,			/* do not create a tag; the token should be
				   invalidated whenever a state machine is
				   reset prematurely */
  pub named: bool,			/* create a named tag */
  pub lineno: intmax_t,		/* source line number of tag */
  pub linepos: intmax_t		/* source char number of tag */
}
static mut token: tok =
  tok { line: std::ptr::null_mut (), offset: 0, length: 0, valid: false, named: false,
	lineno: 0, linepos: 0 };		/* latest token read */

/*
 * Variables and functions for dealing with nested structures.
 * Idea by Mykola Dzyuba <mdzyuba@yahoo.com> (2001)
 */
/* C 2955-2960: the anonymous `static struct { char **cname; ptrdiff_t *bracelev; ptrdiff_t nl;
   ptrdiff_t size; } cstack;` -- Rust needs a name for the type (the make-docfile.rs
   `global_v` precedent).  */
#[repr(C)]
pub struct cstack_record
{
  pub cname: *mut *mut c_char,		/* nested class names */
  pub bracelev: *mut ptrdiff_t,		/* nested class brace level */
  pub nl: ptrdiff_t,			/* class nesting level (elements used) */
  pub size: ptrdiff_t			/* length of the array */
}
static mut cstack: cstack_record =
  cstack_record { cname: std::ptr::null_mut (), bracelev: std::ptr::null_mut (),
		  nl: 0, size: 0 };	/* stack for nested declaration tags */
/* Current struct nesting depth (namespace, class, struct, union, enum). */
macro_rules! nestlev { () => { cstack.nl } }
/* After struct keyword or in struct body, not inside a nested function.  (`instruct' reads
   C_entries' `bracelev' local, so Rust's macro hygiene forces its definition into that
   function; see the note there.)  */

unsafe fn pushclass_above (bracelev: ptrdiff_t, str: *mut c_char, len: ptrdiff_t)
{
  let mut nl: ptrdiff_t;

  popclass_above (bracelev);
  nl = cstack.nl;
  if nl >= cstack.size
    {
      xrnew! (cstack.cname, cstack.size, 2);
      xrnew! (cstack.bracelev, cstack.size, 2);
      cstack.size *= 2;
    }
  assert! (nl == 0 || *cstack.bracelev.offset (nl - 1) < bracelev);
  *cstack.cname.offset (nl) = if str == std::ptr::null_mut () { std::ptr::null_mut () }
			      else { savenstr (str, len) };
  *cstack.bracelev.offset (nl) = bracelev;
  cstack.nl = nl + 1;
}

unsafe fn popclass_above (bracelev: ptrdiff_t)
{
  let mut nl: ptrdiff_t = cstack.nl - 1;
  while nl >= 0 && *cstack.bracelev.offset (nl) >= bracelev
    {
      free (*cstack.cname.offset (nl) as *mut c_void);
      cstack.nl = nl;
      nl -= 1;
    }
}

unsafe fn write_classname (cn: *mut linebuffer, qualifier: *const c_char)
{
  let mut len: ptrdiff_t;

  if cstack.nl == 0 || *cstack.cname == std::ptr::null_mut ()
    {
      len = 0;
      (*cn).len = 0;
      *(*cn).buffer = 0;
    }
  else
    {
      len = strlen (*cstack.cname) as ptrdiff_t;
      linebuffer_setlen (cn, len);
      strcpy ((*cn).buffer, *cstack.cname);
    }
  let mut i: ptrdiff_t = 1;
  while i < cstack.nl
    {
      let mut s: *mut c_char = *cstack.cname.offset (i);
      if s != std::ptr::null_mut ()
	{
	  let qlen: c_int = strlen (qualifier) as c_int;
	  let slen: ptrdiff_t = strlen (s) as ptrdiff_t;
	  linebuffer_setlen (cn, len + qlen as ptrdiff_t + slen);
	  memcpyz (stpcpy ((*cn).buffer.offset (len), qualifier) as *mut c_void,
		   s as *const c_void, slen);
	  len += qlen as ptrdiff_t + slen;
	}
      i += 1;
    }
}


/*
 * consider_token ()
 *	checks to see if the current token is at the start of a
 *	function or variable, or corresponds to a typedef, or
 * 	is a struct/union/enum tag, or #define, or an enum constant.
 *
 *	*IS_FUNC_OR_VAR gets true if the token is a function or #define macro
 *	with args.  C_EXTP points to which language we are looking at.
 *
 * Globals
 *	fvdef			IN OUT
 *	structdef		IN OUT
 *	definedef		IN OUT
 *	typdef			IN OUT
 *	objdef			IN OUT
 */
unsafe fn consider_token (str: *mut c_char,	/* IN: token pointer */
			  len: ptrdiff_t,	/* IN: token length */
			  c: c_int,		/* IN: first char after the token */
			  c_extp: *mut c_int,	/* IN, OUT: C extensions mask */
			  bracelev: ptrdiff_t,	/* IN: brace level */
			  parlev: ptrdiff_t,	/* IN: parenthesis level */
			  is_func_or_var: *mut bool) -> bool	/* OUT: function or variable found */
{
  /* When structdef is stagseen, scolonseen, or snone with bracelev > 0,
     structtype is the type of the preceding struct-like keyword, and
     structbracelev is the brace level where it has been seen. */
  static mut structtype: sym_type = st_none;
  static mut structbracelev: ptrdiff_t = 0;
  static mut toktype: sym_type = st_none;

  /* C 2965-2966 defines `instruct' at file scope over `structdef', `cstack' and the
     enclosing function's `bracelev'; `macro_rules!` hygiene resolves identifiers at the
     definition site, so the port defines the macro inside each function that uses it, after
     its locals (the `bytecode.rs` FETCH/POP precedent, PORTING.md P-57).  `nestlev'
     (C 2963) only reads `cstack', so it stays at file scope above.  */
  macro_rules! instruct {
      () => { structdef == snone && nestlev! () > 0
	      && bracelev == *cstack.bracelev.offset (nestlev! () - 1) + 1 };
  }

  toktype = C_symtype (str, len, *c_extp);

  /*
   * Skip __attribute__
   */
  if toktype == st_C_attribute
    {
      inattribute = true;
      return false;
     }

  /*
   * Skip ENUM_BF
   */
  if toktype == st_C_enum_bf && definedef == dnone
    {
      in_enum_bf = true;
      return false;
    }

   /*
    * Advance the definedef state machine.
    */
   match definedef
     {
     dnone =>
       {
	 /* We're not on a preprocessor line. */
	 if toktype == st_C_gnumacro
	   {
	     fvdef = fdefunkey;
	     return false;
	   }
       }
     dsharpseen =>
       {
	 if toktype == st_C_define
	   {
	     definedef = ddefineseen;
	   }
	 else
	   {
	     definedef = dignorerest;
	   }
	 return false;
       }
     ddefineseen =>
       {
	 /*
	  * Make a tag for any macro, unless it is a constant
	  * and constantypedefs is false.
	  */
	 definedef = dignorerest;
	 *is_func_or_var = c == b'(' as c_int;
	 if !*is_func_or_var && !constantypedefs
	   { return false; }
	 else
	   { return true; }
       }
     dignorerest =>
       {
	 return false;
       }
     _ =>
       {
	 error (c"internal error: definedef value.".as_ptr ());
       }
     }

   /*
    * Now typedefs
    */
   match typdef
     {
     tnone =>
       {
	 if toktype == st_C_typedef
	   {
	     if typedefs
	       { typdef = tkeyseen; }
	     fvextern = false;
	     fvdef = fvnone;
	     return false;
	   }
       }
     tkeyseen =>
       {
	 match toktype
	   {
	   st_none | st_C_class | st_C_struct | st_C_enum =>
	     {
	       typdef = ttypeseen;
	     }
	   _ => {}
	   }
       }
     ttypeseen =>
       {
	 if structdef == snone && fvdef == fvnone
	   {
	     fvdef = fvnameseen;
	     return true;
	   }
       }
     tend =>
       {
	 match toktype
	   {
	   st_C_class | st_C_struct | st_C_enum =>
	     {
	       return false;
	     }
	   _ =>
	     {
	       return true;
	     }
	   }
       }
     _ => {}
     }

   'toktype_switch: {
   match toktype
     {
     st_C_javastruct =>
       {
	 if structdef == stagseen
	   { structdef = scolonseen; }
	 return false;
       }
     /* C's `case st_C_template: case st_C_class:` runs the C_AUTO test and then either
	 `break`s out of the switch (template) or falls through into the
	 `case st_C_struct: case st_C_enum:` body (FALLTHROUGH, PORTING.md 2.2); the port
	 carries the shared tail in the one arm.  */
     st_C_template | st_C_class | st_C_struct | st_C_enum =>
       {
	 if toktype == st_C_template || toktype == st_C_class
	   {
	     if (*c_extp & C_AUTO) != 0	/* automatic detection of C++ language */
		 && bracelev == 0
		 && definedef == dnone && structdef == snone
		 && typdef == tnone && fvdef == fvnone
	       { *c_extp = (*c_extp | C_PLPL) & !C_AUTO; }
	     if toktype == st_C_template
	       { break 'toktype_switch; }
	   }
	 if parlev == 0
	     && fvdef != vignore
	     && (typdef == tkeyseen
		 || (typedefs_or_cplusplus && structdef == snone))
	   {
	     structdef = skeyseen;
	     structtype = toktype;
	     structbracelev = bracelev;
	     if fvdef == fvnameseen
	       { fvdef = fvnone; }
	   }
	 return false;
       }
     _ => {}
     }
   }

   if structdef == skeyseen
     {
       structdef = stagseen;
       return true;
     }

   if typdef != tnone
     { definedef = dnone; }

   /* Detect Objective C constructs. */
   match objdef
     {
     onone =>
       {
	 match toktype
	   {
	   st_C_objprot =>
	     {
	       objdef = oprotocol;
	       return false;
	     }
	   st_C_objimpl =>
	     {
	       objdef = oimplementation;
	       return false;
	     }
	   _ => {}
	   }
       }
     oimplementation =>
       {
	 /* Save the class tag for functions or variables defined inside. */
	 objtag = savenstr (str, len);
	 objdef = oinbody;
	 return false;
       }
     oprotocol =>
       {
	 /* Save the class tag for categories. */
	 objtag = savenstr (str, len);
	 objdef = otagseen;
	 *is_func_or_var = true;
	 return true;
       }
     oparenseen =>
       {
	 objdef = ocatseen;
	 *is_func_or_var = true;
	 return true;
       }
     oinbody => {}
     omethodsign =>
       {
	 if parlev == 0
	   {
	     fvdef = fvnone;
	     objdef = omethodtag;
	     linebuffer_setlen (&raw mut token_name, len);
	     memcpyz (token_name.buffer as *mut c_void, str as *const c_void, len);
	     return true;
	   }
	 return false;
       }
     omethodcolon =>
       {
	 if parlev == 0
	   { objdef = omethodparm; }
	 return false;
       }
     omethodparm =>
       {
	 if parlev == 0
	   {
	     objdef = omethodtag;
	     if class_qualify != 0
	       {
		 let mut oldlen: ptrdiff_t = token_name.len;
		 fvdef = fvnone;
		 linebuffer_setlen (&raw mut token_name, oldlen + len);
		 memcpyz (token_name.buffer.offset (oldlen) as *mut c_void,
			  str as *const c_void, len);
	       }
	     return true;
	   }
	 return false;
       }
     oignore =>
       {
	 if toktype == st_C_objend
	   {
	     /* Memory leakage here: the string pointed by objtag is
		never released, because many tests would be needed to
		avoid breaking on incorrect input code.  The amount of
		memory leaked here is the sum of the lengths of the
		class tags.
	     free (objtag); */
	     objdef = onone;
	   }
	 return false;
       }
     _ => {}
     }

   /* A function, variable or enum constant? */
   'toktype_switch2: {
   match toktype
     {
     st_C_extern =>
       {
	 fvextern = true;
	 match fvdef
	   {
	   finlist | flistseen | fignore | vignore => {}
	   _ =>
	     {
	       fvdef = fvnone;
	     }
	   }
	 return false;
       }
     st_C_ignore =>
       {
	 fvextern = false;
	 fvdef = vignore;
	 return false;
       }
     st_C_operator =>
       {
	 fvdef = foperator;
	 *is_func_or_var = true;
	 return true;
       }
     st_none =>
       {
	 if constantypedefs
	     && structdef == snone
	     && structtype == st_C_enum && bracelev > structbracelev
	     /* Don't tag tokens in expressions that assign values to enum
		constants.  */
	     && fvdef != vignore
	   { return true; }		/* enum constant */
	 'fvdef: {
	 match fvdef
	   {
	   fdefunkey =>
	     {
	       if bracelev > 0
		 { break 'fvdef; }
	       fvdef = fdefunname;	/* GNU macro */
	       *is_func_or_var = true;
	       return true;
	     }
	   /* C's `case fvnone:` runs its body and then falls through into `case
	      fvnameseen:` (FALLTHROUGH, PORTING.md 2.2).  */
	   fvnone | fvnameseen =>
	     {
	       if fvdef == fvnone
		 {
		   match typdef
		     {
		     ttypeseen =>
		       {
			 return false;
		       }
		     tnone =>
		       {
			 if (strneq (str, c"asm".as_ptr (), 3)
			     && endtoken (*str.offset (3) as c_uchar))
			     || (strneq (str, c"__asm__".as_ptr (), 7)
				 && endtoken (*str.offset (7) as c_uchar))
			   {
			     fvdef = vignore;
			     return false;
			   }
		       }
		     _ => {}
		     }
		 }
	       if len >= 10 && strneq (str.offset (len - 10), c"::operator".as_ptr (), 10)
		 {
		   if (*c_extp & C_AUTO) != 0 /* automatic detection of C++ */
		     { *c_extp = (*c_extp | C_PLPL) & !C_AUTO; }
		   fvdef = foperator;
		   *is_func_or_var = true;
		   return true;
		 }
	       if bracelev > 0 && !instruct! ()
		 { break 'fvdef; }
	       fvdef = fvnameseen;	/* function or variable */
	       *is_func_or_var = true;
	       return true;
	     }
	   _ => {}
	   }
	 }
	 break 'toktype_switch2;
       }
     _ => {}
     }
   }

  return false;
}

/*
 * C_entries often keeps pointers to tokens or lines which are older than
 * the line currently read.  By keeping two line buffers, and switching
 * them at end of line, it is possible to use those pointers.
 */
/* C 3386-3390: the anonymous `static struct { intmax_t linepos; linebuffer lb; } lbs[2];` --
   Rust needs a name for the type.  */
#[repr(C)]
pub struct lbs_record
{
  pub linepos: intmax_t,
  pub lb: linebuffer
}
static mut lbs: [lbs_record; 2] =
  [ lbs_record { linepos: 0, lb: linebuffer { size: 0, len: 0, buffer: std::ptr::null_mut () } },
    lbs_record { linepos: 0, lb: linebuffer { size: 0, len: 0, buffer: std::ptr::null_mut () } } ];

/* C 3390-3415 defines `current_lb_is_new', `switch_line_buffers', `curlb', `newlb',
   `curlinepos', `newlinepos', `plainc', `cplpl', `cjava', `CNL_SAVE_DEFINEDEF' and `CNL' at
   file scope over C_entries' own locals; `macro_rules!` hygiene resolves identifiers at the
   definition site, so the port defines all of them inside `C_entries`, after those locals
   (PORTING.md P-57; the `bytecode.rs` FETCH/POP precedent).  C's `switch_line_buffers ()
   (curndx = 1 - curndx)` is the port's boolean negation.  */

unsafe fn make_C_tag (isfun: bool)
{
  /* This function is never called when token.valid is false, but
     we must protect against invalid input or internal errors. */
  if token.valid
    { make_tag (token_name.buffer, token_name.len, isfun, token.line,
		token.offset + token.length + 1, token.lineno, token.linepos); }
  else if DEBUG
    {				  /* this branch is optimized away if !DEBUG */
      make_tag (concat (c"INVALID TOKEN:-->".as_ptr (), token_name.buffer, c"".as_ptr ()),
		token_name.len + 17, isfun, token.line,
		token.offset + token.length + 1, token.lineno, token.linepos);
      error (c"INVALID TOKEN".as_ptr ());
    }

  token.valid = false;
}

unsafe fn perhaps_more_input (inf: *mut FILE) -> bool
{
  return feof (inf) == 0 && ferror (inf) == 0;
}


/*
 * C_entries ()
 *	This routine finds functions, variables, typedefs,
 * 	#define's, enum constants and struct/union/enum definitions in
 *	C syntax and adds them to the list.
 */
unsafe fn C_entries (c_ext_arg: c_int,		/* extension of C */
		     inf: *mut FILE)		/* input file */
{
  let mut c: c_char;			/* latest char read; '\0' for end of line */
  let mut lp: *mut c_char;			/* pointer one beyond the character `c' */
  let mut curndx: bool; let mut newndx: bool;		/* indices for current and new lb */
  let mut tokoff: ptrdiff_t;		/* offset in line of start of current token */
  let mut toklen: ptrdiff_t;		/* length of current token */
  let qualifier: *const c_char;        /* string used to qualify names */
  let mut qlen: c_int;			/* length of qualifier */
  let mut bracelev: ptrdiff_t;		/* current brace level */
  let mut bracketlev: ptrdiff_t;		/* current bracket level */
  let mut parlev: ptrdiff_t;		/* current parenthesis level */
  let mut attrparlev: ptrdiff_t;		/* __attribute__ parenthesis level */
  let mut templatelev: ptrdiff_t;	/* current template level */
  let mut typdefbracelev: ptrdiff_t;	/* bracelev where a typedef struct body begun */
  let mut incomm: bool; let mut inquote: bool; let mut inchar: bool;
  let mut quotednl: bool; let mut midtoken: bool;
  let mut yacc_rules: bool;		/* in the rules part of a yacc file */
  let mut savetoken: tok =
    tok { line: std::ptr::null_mut (), offset: 0, length: 0, valid: false, named: false,
	  lineno: 0, linepos: 0 };	/* token saved during preprocessor handling */

  /* `c_ext' is C's parameter, written by `consider_token' through `&c_ext'; the macros
     below read it, and `macro_rules!` hygiene resolves identifiers at the definition site,
     so the local must be declared before them.  */
  let mut c_ext: c_int = c_ext_arg;

  /* C 3390-3415's file-scope macros, which read these locals (see the note at `lbs`).  */
  macro_rules! current_lb_is_new { () => { newndx == curndx } }
  macro_rules! switch_line_buffers { () => { curndx = !curndx } }
  macro_rules! curlb { () => { lbs[curndx as usize].lb } }
  macro_rules! newlb { () => { lbs[newndx as usize].lb } }
  macro_rules! curlinepos { () => { lbs[curndx as usize].linepos } }
  macro_rules! newlinepos { () => { lbs[newndx as usize].linepos } }
  macro_rules! plainc { () => { (c_ext & C_EXT) == C_PLAIN } }
  macro_rules! cplpl { () => { (c_ext & C_PLPL) != 0 } }
  macro_rules! cjava { () => { (c_ext & C_JAVA) == C_JAVA } }
  macro_rules! CNL_SAVE_DEFINEDEF {
      () => {
	  {
	    curlinepos! () = charno;
	    readline (&raw mut curlb! (), inf);
	    lp = curlb! ().buffer;
	    quotednl = false;
	    newndx = curndx;
	  }
      };
  }
  macro_rules! CNL {
      () => {
	  {
	    CNL_SAVE_DEFINEDEF! ();
	    if savetoken.valid
	      {
		token = savetoken;
		savetoken.valid = false;
	      }
	    definedef = dnone;
	  }
      };
  }
  macro_rules! instruct {
      () => { structdef == snone && nestlev! () > 0
	      && bracelev == *cstack.bracelev.offset (nestlev! () - 1) + 1 };
  }

  linebuffer_init (&raw mut lbs[0].lb);
  linebuffer_init (&raw mut lbs[1].lb);
  if cstack.size == 0
    {
      cstack.size = if DEBUG { 1 } else { 4 };
      cstack.nl = 0;
      cstack.cname = xnmalloc (cstack.size, __sizeof_deref (cstack.cname)) as *mut *mut c_char;
      cstack.bracelev = xnmalloc (cstack.size, __sizeof_deref (cstack.bracelev)) as *mut ptrdiff_t;
    }

  typdefbracelev = 0;			/* keep compiler quiet */
  toklen = 0;
  tokoff = 0;
  newndx = false;
  curndx = false;
  lp = curlb! ().buffer;
  *lp = 0;

  fvdef = fvnone; fvextern = false; typdef = tnone;
  structdef = snone; definedef = dnone; objdef = onone;
  yacc_rules = false;
  midtoken = false; inquote = false; inchar = false; incomm = false; quotednl = false;
  savetoken.valid = false;
  token.valid = false;
  templatelev = 0; attrparlev = 0; parlev = 0; bracketlev = 0; bracelev = 0;
  if cjava! ()
    { qualifier = c".".as_ptr (); qlen = 1; }
  else
    { qualifier = c"::".as_ptr (); qlen = 2; }


  while perhaps_more_input (inf)
    {
      c = *lp;
      lp = lp.offset (1);
      if c == b'\\' as c_char
	{
	  /* If we are at the end of the line, the next character is a
	     '\0'; do not skip it, because it is what tells us
	     to read the next line.  */
	  if *lp == 0
	    {
	      quotednl = true;
	      continue;
	    }
	  lp = lp.offset (1);
	  c = b' ' as c_char;
	}
      else if incomm
	{
	  match c
	    {
	    42 /* '*' */ =>
	      {
		if *lp == b'/' as c_char
		  {
		    c = *lp;
		    lp = lp.offset (1);
		    incomm = false;
		  }
	      }
	    0 /* '\0' */ =>
	      {
		/* Newlines inside comments do not end macro definitions in
		   traditional cpp. */
		CNL_SAVE_DEFINEDEF! ();
	      }
	    _ => {}
	    }
	  continue;
	}
      else if inquote
	{
	  match c
	    {
	    34 /* '"' */ =>
	      {
		inquote = false;
	      }
	    0 /* '\0' */ =>
	      {
		/* Newlines inside strings do not end macro definitions
		   in traditional cpp, even though compilers don't
		   usually accept them. */
		CNL_SAVE_DEFINEDEF! ();
	      }
	    _ => {}
	    }
	  continue;
	}
      else if inchar
	{
	  match c
	    {
	    0 /* '\0' */ =>
	      {
		/* Hmmm, something went wrong. */
		CNL! ();
		inchar = false;
	      }
	    39 /* '\'' */ =>
	      {
		inchar = false;
	      }
	    _ => {}
	    }
	  continue;
	}
      else
	{
	  match c
	    {
	    34 /* '"' */ =>
	      {
		inquote = true;
		if bracketlev > 0
		  { continue; }
		if inattribute
		  {
		    /* C's `break` leaves `switch (c)` and reaches the token
		       consideration below; the arm ends here.  */
		  }
		else
		  {
		    match fvdef
		      {
		      fdefunkey | fstartlist | finlist | fignore | vignore => {}
		      _ =>
			{
			  fvextern = false;
			  fvdef = fvnone;
			}
		      }
		    continue;
		  }
	      }
	    39 /* '\'' */ =>
	      {
		inchar = true;
		if bracketlev > 0
		  { continue; }
		if inattribute
		  {
		    /* C's `break` leaves `switch (c)` and reaches the token
		       consideration below; the arm ends here.  */
		  }
		else
		  {
		    if fvdef != finlist && fvdef != fignore && fvdef != vignore
		      {
			fvextern = false;
			fvdef = fvnone;
		      }
		    continue;
		  }
	      }
	    47 /* '/' */ =>
	      {
		if *lp == b'*' as c_char
		  {
		    incomm = true;
		    lp = lp.offset (1);
		    c = b' ' as c_char;
		    if bracketlev > 0
		      { continue; }
		  }
		else if /* cplpl && */ *lp == b'/' as c_char
		  {
		    c = 0;
		  }
	      }
	    37 /* '%' */ =>
	      {
		if (c_ext & YACC) != 0 && *lp == b'%' as c_char
		  {
		    /* Entering or exiting rules section in yacc file. */
		    lp = lp.offset (1);
		    definedef = dnone; fvdef = fvnone; fvextern = false;
		    typdef = tnone; structdef = snone;
		    midtoken = false; inquote = false; inchar = false; incomm = false;
		    quotednl = false;
		    bracelev = 0;
		    yacc_rules = !yacc_rules;
		    continue;
		  }
	      }
	    35 /* '#' */ =>
	      {
		if definedef == dnone
		  {
		    let mut cp: *mut c_char;
		    let mut cpptoken: bool = true;

		    /* Look back on this line.  If all blanks, or nonblanks
		       followed by an end of comment, this is a preprocessor
		       token. */
		    cp = newlb! ().buffer;
		    while cp < lp.offset (-1)
		      {
			if !c_isspace (*cp as c_int)
			  {
			    if *cp == b'*' as c_char && *cp.offset (1) == b'/' as c_char
			      {
				cp = cp.offset (1);
				cpptoken = true;
			      }
			    else
			      { cpptoken = false; }
			  }
			cp = cp.offset (1);
		      }
		    if cpptoken
		      {
			definedef = dsharpseen;
			/* This is needed for tagging enum values: when there are
			   preprocessor conditionals inside the enum, we need to
			   reset the value of fvdef so that the next enum value is
			   tagged even though the one before it did not end in a
			   comma.  */
			if fvdef == vignore && instruct! () && parlev == 0
			  {
			    if strneq (cp, c"#if".as_ptr (), 3)
				|| strneq (cp, c"#el".as_ptr (), 3)
			      { fvdef = fvnone; }
			  }
		      }
		  } /* if (definedef == dnone) */
		continue;
	      }
	    91 /* '[' */ =>
	      {
		bracketlev += 1;
		continue;
	      }
	    _ =>
	      {
		if bracketlev > 0
		  {
		    if c == 93 /* ']' */
		      { bracketlev -= 1; }
		    else if c == 0 /* '\0' */
		      { CNL_SAVE_DEFINEDEF! (); }
		    continue;
		  }
	      }
	    } /* switch (c) */
	}

      /* Consider token only if some involved conditions are satisfied. */
      if typdef != tignore
	  && definedef != dignorerest
	  && fvdef != finlist
	  && templatelev == 0
	  && (definedef != dnone
	      || structdef != scolonseen)
	  && !inattribute
	  && !in_enum_bf
	{
	  if midtoken
	    {
	      if endtoken (c as c_uchar)
		{
		  if c == b':' as c_char && *lp == b':' as c_char
		      && begtoken (*lp.offset (1) as c_uchar)
		    /* This handles :: in the middle,
		       but not at the beginning of an identifier.
		       Also, space-separated :: is not recognized. */
		    {
		      if (c_ext & C_AUTO) != 0 /* automatic detection of C++ */
			{ c_ext = (c_ext | C_PLPL) & !C_AUTO; }
		      lp = lp.offset (2);
		      toklen += 2;
		      c = *lp.offset (-1);
		      /* C: `goto still_in_token;` -- the labelled block's two statements
			 are written here (PORTING.md 2.2).  */
		      toklen += 1;
		      continue;
		    }
		  else
		    {
		      let mut funorvar: bool = false;

		      if yacc_rules
			  || consider_token (newlb! ().buffer.offset (tokoff), toklen,
					     c as c_int,
					     &raw mut c_ext, bracelev, parlev,
					     &raw mut funorvar)
			{
			  if fvdef == foperator
			    {
			      let mut oldlp: *mut c_char = lp;
			      lp = skip_spaces (lp.offset (-1));
			      if *lp != 0
				{ lp = lp.offset (1); }
			      while *lp != 0
				     && !c_isspace (*lp as c_int) && *lp != b'(' as c_char
				{ lp = lp.offset (1); }
			      c = *lp;
			      lp = lp.offset (1);
			      toklen += lp.offset_from (oldlp);
			    }
			  token.named = false;
			  if !plainc! ()
			      && nestlev! () > 0 && definedef == dnone
			    /* in struct body */
			    {
			      if class_qualify != 0
				{
				  write_classname (&raw mut token_name, qualifier);
				  let mut len: ptrdiff_t = token_name.len;
				  linebuffer_setlen (&raw mut token_name, len + qlen as ptrdiff_t + toklen);
				  memcpyz (stpcpy (token_name.buffer.offset (len),
						   qualifier) as *mut c_void,
					   newlb! ().buffer.offset (tokoff) as *const c_void,
					   toklen);
				}
			      else
				{
				  linebuffer_setlen (&raw mut token_name, toklen);
				  memcpyz (token_name.buffer as *mut c_void,
					   newlb! ().buffer.offset (tokoff) as *const c_void,
					   toklen);
				}
			      token.named = true;
			    }
			  else if objdef == ocatseen
			    /* Objective C category */
			    {
			      if class_qualify != 0
				{
				  let mut len: ptrdiff_t = strlen (objtag) as ptrdiff_t + 2 + toklen;
				  linebuffer_setlen (&raw mut token_name, len);
				  let mut p1: *mut c_char =
				    stpcpy (token_name.buffer, objtag);
				  let mut p2: *mut c_char = stpcpy (p1, c"(".as_ptr ());
				  let mut p3: *mut c_char =
				    mempcpy (p2 as *mut c_void,
						 newlb! ().buffer.offset (tokoff) as *const c_void,
						 toklen as size_t) as *mut c_char;
				  strcpy (p3, c")".as_ptr ());
				}
			      else
				{
				  linebuffer_setlen (&raw mut token_name, toklen);
				  memcpyz (token_name.buffer as *mut c_void,
					   newlb! ().buffer.offset (tokoff) as *const c_void,
					   toklen);
				}
			      token.named = true;
			    }
			  else if objdef == omethodtag
				   || objdef == omethodparm
			    /* Objective C method */
			    {
			      token.named = true;
			    }
			  else if fvdef == fdefunname
			    /* GNU DEFUN and similar macros */
			    {
			      let mut defun: bool =
				*newlb! ().buffer.offset (tokoff) == b'F' as c_char;
			      let mut off: ptrdiff_t = tokoff;
			      let mut len: ptrdiff_t = toklen;

			      if defun
				{
				  off += 1;
				  len -= 1;

				  /* First, tag it as its C name */
				  linebuffer_setlen (&raw mut token_name, toklen);
				  memcpyz (token_name.buffer as *mut c_void,
					   newlb! ().buffer.offset (tokoff) as *const c_void,
					   toklen);
				  token.named = true;
				  token.lineno = lineno;
				  token.offset = tokoff;
				  token.length = toklen;
				  token.line = newlb! ().buffer;
				  token.linepos = newlinepos! ();
				  token.valid = true;
				  make_C_tag (funorvar);
				}
			      /* Rewrite the tag so that emacs lisp DEFUNs
				 can be found also by their elisp name */
			      linebuffer_setlen (&raw mut token_name, len);
			      memcpyz (token_name.buffer as *mut c_void,
				       newlb! ().buffer.offset (off) as *const c_void,
				       len);
			      if defun
				{
				  loop
				    {
				      len -= 1;
				      if !(len >= 0)
					{ break; }
				      if *token_name.buffer.offset (len) == b'_' as c_char
					{ *token_name.buffer.offset (len) = b'-' as c_char; }
				    }
				}
			      token.named = defun;
			    }
			  else
			    {
			      linebuffer_setlen (&raw mut token_name, toklen);
			      memcpyz (token_name.buffer as *mut c_void,
				       newlb! ().buffer.offset (tokoff) as *const c_void,
				       toklen);
			      /* Name macros and members. */
			      token.named = structdef == stagseen
					     || typdef == ttypeseen
					     || typdef == tend
					     || (funorvar
						 && definedef == dignorerest)
					     || (funorvar
						 && definedef == dnone
						 && structdef == snone
						 && bracelev > 0);
			    }
			  token.lineno = lineno;
			  token.offset = tokoff;
			  token.length = toklen;
			  token.line = newlb! ().buffer;
			  token.linepos = newlinepos! ();
			  token.valid = true;

			  if definedef == dnone
			      && (fvdef == fvnameseen
				  || fvdef == foperator
				  || structdef == stagseen
				  || typdef == tend
				  || typdef == ttypeseen
				  || objdef != onone)
			    {
			      if current_lb_is_new! ()
				{ switch_line_buffers! (); }
			    }
			  else if definedef != dnone
				   || fvdef == fdefunname
				   || instruct! ()
			    { make_C_tag (funorvar); }
			}
		      else /* not yacc and consider_token failed */
			{
			  if inattribute && fvdef == fignore
			    {
			      /* We have just met __attribute__ after a
				 function parameter list: do not tag the
				 function again. */
			      fvdef = fvnone;
			    }
			}
		      midtoken = false;
		    }
		} /* if (endtoken (c)) */
	      else if intoken (c as c_uchar)
		/* C's `still_in_token:` label is the `toklen++; continue;` block at the end of
		   this arm (PORTING.md 2.2).  */
		{
		  toklen += 1;
		  continue;
		}
	    } /* if (midtoken) */
	  else if begtoken (c as c_uchar)
	    {
	      match definedef
		{
		dnone =>
		  {
		    match fvdef
		      {
		      fstartlist =>
			{
			  /* This prevents tagging fb in
			     void (__attribute__ ((noreturn)) *fb) (void);
			     Fixing this is not easy and not very important. */
			  fvdef = finlist;
			  continue;
			}
		      flistseen =>
			{
			  if plainc! () || declarations != 0
			    {
			      make_C_tag (true); /* a function */
			      fvdef = fignore;
			    }
			}
		      _ => {}
		      }
		    if structdef == stagseen && !cjava! ()
		      {
			popclass_above (bracelev);
			structdef = snone;
		      }
		  }
		dsharpseen =>
		  {
		    savetoken = token;
		  }
		_ => {}
		}
	      if !yacc_rules || lp == newlb! ().buffer.offset (1)
		{
		  tokoff = lp.offset (-1).offset_from (newlb! ().buffer);
		  toklen = 1;
		  midtoken = true;
		}
	      continue;
	    } /* if (begtoken) */
	} /* if must look at token */


      /* Detect end of line, colon, comma, semicolon and various braces
	 after having handled a token.*/
      match c
	{
	58 /* ':' */ =>
	  {
	    'colon: {
	    if inattribute
	      { break 'colon; }
	    if yacc_rules && token.offset == 0 && token.valid
	      {
		make_C_tag (false); /* a yacc function */
		break 'colon;
	      }
	    if definedef != dnone
	      { break 'colon; }
	    match objdef
	      {
	      otagseen =>
		{
		  objdef = oignore;
		  make_C_tag (true); /* an Objective C class */
		}
	      omethodtag | omethodparm =>
		{
		  objdef = omethodcolon;
		  if class_qualify != 0
		    {
		      /* C's inner `ptrdiff_t toklen` shadows the function's.  */
		      let mut toklen: ptrdiff_t = token_name.len;
		      linebuffer_setlen (&raw mut token_name, toklen + 1);
		      strcpy (token_name.buffer.offset (toklen), c":".as_ptr ());
		    }
		}
	      _ => {}
	      }
	    if structdef == stagseen
	      {
		structdef = scolonseen;
		break 'colon;
	      }
	    /* Should be useless, but may be work as a safety net. */
	    if cplpl! () && fvdef == flistseen
	      {
		make_C_tag (true); /* a function */
		fvdef = fignore;
		break 'colon;
	      }
	    }
	  }
	59 /* ';' */ =>
	  {
	    if definedef == dnone && !inattribute
	      {
		match typdef
		  {
		  tend | ttypeseen =>
		    {
		      make_C_tag (false); /* a typedef */
		      typdef = tnone;
		      fvdef = fvnone;
		    }
		  _ =>
		    {
		      /* C's `case tnone: case tinbody: case tignore:` runs its body and
			 then falls through into `default:` (FALLTHROUGH, PORTING.md 2.2).  */
		      if typdef == tnone || typdef == tinbody || typdef == tignore
			{
			  match fvdef
			    {
			    fignore =>
			      {
				if typdef == tignore || cplpl! ()
				  { fvdef = fvnone; }
			      }
			    fvnameseen =>
			      {
				if (globals != 0 && bracelev == 0
				    && (!fvextern || declarations != 0))
				    || (members != 0 && instruct! ())
				  { make_C_tag (false); } /* a variable */
				fvextern = false;
				fvdef = fvnone;
				token.valid = false;
			      }
			    _ =>
			      {
				/* C's `case flistseen:` runs its body and then falls
				   through into `default:` (FALLTHROUGH, PORTING.md 2.2).  */
				if fvdef == flistseen
				  {
				    if (declarations != 0
					&& (cplpl! () || !instruct! ())
					&& (typdef == tnone
					    || (typdef != tignore && instruct! ())))
					|| (members != 0
					    && plainc! () && instruct! ())
				      { make_C_tag (true); }  /* a function */
				  }
				fvextern = false;
				fvdef = fvnone;
				if declarations != 0
				    && cplpl! () && structdef == stagseen
				  { make_C_tag (false); }	/* forward declaration */
				else
				  { token.valid = false; }
			      }
			    }
			}
		      if !instruct! ()
			{ typdef = tnone; }
		    }
		  }
	      }
	    if structdef == stagseen
	      { structdef = snone; }
	  }
	44 /* ',' */ =>
	  {
	    if definedef == dnone && !inattribute
	      {
		match objdef
		  {
		  omethodtag | omethodparm =>
		    {
		      make_C_tag (true); /* an Objective C method */
		      objdef = oinbody;
		    }
		  _ => {}
		  }
		match fvdef
		  {
		  fdefunkey | foperator | fstartlist | finlist | fignore => {}
		  vignore =>
		    {
		      if instruct! () && parlev == 0
			{ fvdef = fvnone; }
		    }
		  fdefunname =>
		    {
		      fvdef = fignore;
		    }
		  fvnameseen =>
		    {
		      if parlev == 0
			  && ((globals != 0
			       && bracelev == 0
			       && templatelev == 0
			       && (!fvextern || declarations != 0))
			      || (members != 0 && instruct! ()))
			{ make_C_tag (false); } /* a variable */
		    }
		  flistseen =>
		    {
		      if (declarations != 0 && typdef == tnone && !instruct! ())
			  || (members != 0 && typdef != tignore && instruct! ())
			{
			  make_C_tag (true); /* a function */
			  fvdef = fvnameseen;
			}
		      else if declarations == 0
			{ fvdef = fvnone; }
		      token.valid = false;
		    }
		  _ =>
		    {
		      fvdef = fvnone;
		    }
		  }
		if structdef == stagseen
		    /* class Foo<K,V,T>...  */
		    && !(cjava! () && templatelev > 0)
		  { structdef = snone; }
	      }
	  }
	93 /* ']' */ =>
	  {
	    if definedef == dnone && !inattribute
	      {
		if structdef == stagseen
		  { structdef = snone; }
		match typdef
		  {
		  ttypeseen | tend =>
		    {
		      typdef = tignore;
		      make_C_tag (false);	/* a typedef */
		    }
		  _ =>
		    {
		      if typdef == tnone || typdef == tinbody
			{
			  match fvdef
			    {
			    foperator | finlist | fignore | vignore => {}
			    _ =>
			      {
				/* C's `case fvnameseen:` falls through into `default:`
				   (FALLTHROUGH, PORTING.md 2.2).  */
				if fvdef == fvnameseen
				  {
				    if (members != 0 && bracelev == 1)
					|| (globals != 0 && bracelev == 0
					    && (!fvextern || declarations != 0))
				      { make_C_tag (false); } /* a variable */
				  }
				fvdef = fvnone;
			      }
			    }
			}
		    }
		  }
	      }
	  }
	40 /* '(' */ =>
	  {
	    if inattribute
	      {
		attrparlev += 1;
	      }
	    else if definedef != dnone
	      {
	      }
	    else
	      {
		if objdef == otagseen && parlev == 0
		  { objdef = oparenseen; }
		match fvdef
		  {
		  fvnameseen =>
		    {
		      if typdef == ttypeseen
			  && *lp != b'*' as c_char
			  && !instruct! ()
			{
			  /* This handles constructs like:
			     typedef void OperatorFun (int fun); */
			  make_C_tag (false);
			  typdef = tignore;
			  fvdef = fignore;
			}
		      else
			{
			  /* C's FALLTHROUGH: `case fvnameseen` continues in
			     `case foperator:`'s body.  */
			  fvdef = fstartlist;
			}
		    }
		  foperator =>
		    {
		      fvdef = fstartlist;
		    }
		  flistseen =>
		    {
		      fvdef = finlist;
		    }
		  _ => {}
		  }
		parlev += 1;
	      }
	  }
	41 /* ')' */ =>
	  {
	    if inattribute
	      {
		attrparlev -= 1;
		if attrparlev == 0
		  { inattribute = false; }
	      }
	    else if in_enum_bf
	      {
		parlev -= 1;
		if parlev == 0
		  { in_enum_bf = false; }
	      }
	    else if definedef != dnone
	      {
	      }
	    else
	      {
		if objdef == ocatseen && parlev == 1
		  {
		    make_C_tag (true); /* an Objective C category */
		    objdef = oignore;
		  }
		parlev -= 1;
		if parlev == 0
		  {
		    match fvdef
		      {
		      fstartlist | finlist =>
			{
			  fvdef = flistseen;
			}
		      _ => {}
		      }
		    if !instruct! ()
			&& (typdef == tend
			    || typdef == ttypeseen)
		      {
			typdef = tignore;
			make_C_tag (false); /* a typedef */
		      }
		  }
		else if parlev < 0	/* can happen due to ill-conceived #if's. */
		  { parlev = 0; }
	      }
	  }
	123 /* '{' */ =>
	  {
	    if definedef == dnone
	      {
		if typdef == ttypeseen
		  {
		    /* Whenever typdef is set to tinbody (currently only
		       here), typdefbracelev should be set to bracelev. */
		    typdef = tinbody;
		    typdefbracelev = bracelev;
		  }
		match fvdef
		  {
		  flistseen | fignore =>
		    {
		      /* C's `case flistseen:` runs its body and then falls through into
			 `case fignore:` (FALLTHROUGH, PORTING.md 2.2).  */
		      if fvdef == flistseen
			{
			  if cplpl! () && class_qualify == 0
			    {
			      /* Remove class and namespace qualifiers from the token,
				 leaving only the method/member name.  */
			      let mut cc: *mut c_char;
			      let mut uqname: *mut c_char = token_name.buffer;
			      let tok_end: *mut c_char =
				token_name.buffer.offset (token_name.len);

			      cc = token_name.buffer;
			      while cc < tok_end
				{
				  if *cc == b':' as c_char
				      && *cc.offset (1) == b':' as c_char
				    {
				      uqname = cc.offset (2);
				      cc = cc.offset (1);
				    }
				  cc = cc.offset (1);
				}
			      if uqname > token_name.buffer
				{
				  let uqlen: ptrdiff_t = strlen (uqname) as ptrdiff_t;
				  linebuffer_setlen (&raw mut token_name, uqlen);
				  memmove (token_name.buffer as *mut c_void,
					   uqname as *const c_void,
					   (uqlen + 1) as size_t);
				}
			    }
			  make_C_tag (true);    /* a function */
			}
		      fvdef = fvnone;
		    }
		  fvnone =>
		    {
		      match objdef
			{
			otagseen =>
			  {
			    make_C_tag (true); /* an Objective C class */
			    objdef = oignore;
			  }
			omethodtag | omethodparm =>
			  {
			    make_C_tag (true); /* an Objective C method */
			    objdef = oinbody;
			  }
			_ =>
			  {
			    /* Neutralize `extern "C" {' grot. */
			    if bracelev == 0 && structdef == snone
				&& nestlev! () == 0
				&& typdef == tnone
			      { bracelev = -1; }
			  }
			}
		    }
		  _ => {}
		  }
		match structdef
		  {
		  skeyseen =>	   /* unnamed struct */
		    {
		      pushclass_above (bracelev, std::ptr::null_mut (), 0);
		      structdef = snone;
		    }
		  stagseen | scolonseen =>	   /* named struct or enum / a class */
		    {
		      pushclass_above (bracelev,
				       token.line.offset (token.offset), token.length);
		      structdef = snone;
		      make_C_tag (false);  /* a struct or enum */
		    }
		  _ => {}
		  }
		bracelev += 1;
	      }
	  }
	42 /* '*' */ =>
	  {
	    if definedef == dnone
	      {
		if fvdef == fstartlist
		  {
		    fvdef = fvnone;	/* avoid tagging `foo' in `foo (*bar()) ()' */
		    token.valid = false;
		  }
	      }
	  }
	125 /* '}' */ =>
	  {
	    if definedef == dnone
	      {
		bracelev -= 1;
		/* If we see a closing brace in column zero, and we weren't told to
		   ignore indentation, we assume this the final brace of a function
		   or struct definition, and reset bracelev to zero.  */
		if !ignoreindent && lp == newlb! ().buffer.offset (1)
		  {
		    if bracelev != 0
		      { token.valid = false; } /* unexpected value, token unreliable */
		    bracelev = 0;	/* reset brace level if first column */
		    parlev = 0;	/* also reset paren level, just in case... */
		  }
		else if bracelev < 0
		  {
		    token.valid = false; /* something gone amiss, token unreliable */
		    bracelev = 0;
		  }
		if bracelev == 0 && fvdef == vignore
		  { fvdef = fvnone; }		/* end of function */
		popclass_above (bracelev);
		structdef = snone;
		/* Only if typdef == tinbody is typdefbracelev significant. */
		if typdef == tinbody && bracelev <= typdefbracelev
		  {
		    /* If we forcibly reset bracelevel to zero above, let's
		       not shoot ourself in the foot and assert that we didn't.  */
		    if !(!ignoreindent && lp == newlb! ().buffer.offset (1))
		      { assert! (bracelev == typdefbracelev); }
		    typdef = tend;
		  }
	      }
	  }
	61 /* '=' */ =>
	  {
	    if definedef == dnone
	      {
		match fvdef
		  {
		  foperator | finlist | fignore | vignore => {}
		  _ =>
		    {
		      /* C's `case fvnameseen:` falls through into `default:`
			 (FALLTHROUGH, PORTING.md 2.2).  */
		      if fvdef == fvnameseen
			{
			  if (members != 0 && bracelev == 1)
			      || (globals != 0 && bracelev == 0
				  && (!fvextern || declarations != 0))
			    { make_C_tag (false); } /* a variable */
			}
		      fvdef = vignore;
		    }
		  }
	      }
	  }
	60 /* '<' */ =>
	  {
	    if cplpl! ()
		&& (structdef == stagseen || fvdef == fvnameseen)
	      {
		templatelev += 1;
	      }
	    else
	      {
		/* C: `goto resetfvdef;` (PORTING.md 2.2).  */
		if definedef == dnone
		  {
		    match fvdef
		      {
		      foperator | finlist | fignore | vignore => {}
		      _ =>
			{
			  fvdef = fvnone;
			}
		      }
		  }
	      }
	  }
	62 /* '>' */ =>
	  {
	    if templatelev > 0
	      {
		templatelev -= 1;
	      }
	    else
	      {
		/* C: `goto resetfvdef;` (PORTING.md 2.2).  */
		if definedef == dnone
		  {
		    match fvdef
		      {
		      foperator | finlist | fignore | vignore => {}
		      _ =>
			{
			  fvdef = fvnone;
			}
		      }
		  }
	      }
	  }
	43 /* '+' */ | 45 /* '-' */ =>
	  {
	    if objdef == oinbody && bracelev == 0
	      {
		objdef = omethodsign;
	      }
	    else
	      {
		/* C's FALLTHROUGH sends these into the `resetfvdef:` group.  */
		if definedef == dnone
		  {
		    match fvdef
		      {
		      foperator | finlist | fignore | vignore => {}
		      _ =>
			{
			  fvdef = fvnone;
			}
		      }
		  }
	      }
	  }
	35 /* '#' */ | 126 /* '~' */ | 38 /* '&' */ | 37 /* '%' */ | 47 /* '/' */
	| 124 /* '|' */ | 94 /* '^' */ | 33 /* '!' */ | 46 /* '.' */ | 63 /* '?' */ =>
	  {
	    /* C's `resetfvdef:` label.  */
	    if definedef == dnone
	      {
		/* These surely cannot follow a function tag in C. */
		match fvdef
		  {
		  foperator | finlist | fignore | vignore => {}
		  _ =>
		    {
		      fvdef = fvnone;
		    }
		  }
	      }
	  }
	0 /* '\0' */ =>
	  {
	    if objdef == otagseen
	      {
		make_C_tag (true); /* an Objective C class */
		objdef = oignore;
	      }
	    /* If a macro spans multiple lines don't reset its state. */
	    if quotednl
	      { CNL_SAVE_DEFINEDEF! (); }
	    else
	      { CNL! (); }
	  }
	_ => {}
	} /* switch (c) */

    } /* while not eof */

  free (lbs[0].lb.buffer as *mut c_void);
  free (lbs[1].lb.buffer as *mut c_void);
}

/*
 * Process either a C++ file or a C file depending on the setting
 * of a global flag.
 */
unsafe extern "C" fn default_C_entries (inf: *mut FILE)
{
  C_entries (if cplusplus { C_PLPL } else { C_AUTO }, inf);
}

/* Always do plain C. */
unsafe extern "C" fn plain_C_entries (inf: *mut FILE)
{
  C_entries (0, inf);
}

/* Always do C++. */
unsafe extern "C" fn Cplusplus_entries (inf: *mut FILE)
{
  C_entries (C_PLPL, inf);
}

/* Always do Java. */
unsafe extern "C" fn Cjava_entries (inf: *mut FILE)
{
  C_entries (C_JAVA, inf);
}

/* Always do C*. */
unsafe extern "C" fn Cstar_entries (inf: *mut FILE)
{
  C_entries (C_STAR, inf);
}

/* Always do Yacc. */
unsafe extern "C" fn Yacc_entries (inf: *mut FILE)
{
  C_entries (YACC, inf);
}


/* Useful macros. */

/* C's `LOOP_ON_INPUT_LINES (fp, lb, cp)` is a `while` header that the caller's block becomes
   the body of; Rust macros cannot expand into an unfinished statement, so the body is the
   fourth argument (the `bytecode.rs` FIRST! precedent).  The comma expression inside C's
   condition is the port's block expression.  */
macro_rules! LOOP_ON_INPUT_LINES {
    ($file_pointer:expr, $line_buffer:expr, $char_pointer:ident, $body:block) => {
	while perhaps_more_input ($file_pointer)
	    && {
		readline (&raw mut $line_buffer, $file_pointer);
		$char_pointer = $line_buffer.buffer;
		true
	    }
	  $body
    };
}

/* C's `assert ("" kw)` (a compile-time check that kw is a literal string) has no runtime
   counterpart and is dropped; `sizeof (kw) - 1` is the literal's length, which the port
   takes from the `&CStr`'s bytes.  */
macro_rules! LOOKING_AT {
    ($cp:expr, $kw:expr) => {
	(strneq ($cp, $kw.as_ptr (), $kw.to_bytes ().len () as size_t)
	 && notinname (*$cp.offset ($kw.to_bytes ().len () as isize) as c_uchar)
	 && { $cp = skip_spaces ($cp.offset ($kw.to_bytes ().len () as isize)); true })
    };
}

/* Similar to LOOKING_AT but does not use notinname, does not skip */
macro_rules! LOOKING_AT_NOCASE {
    ($cp:expr, $kw:expr) => {
	(strncaseeq ($cp, $kw.as_ptr (), $kw.to_bytes ().len () as size_t)
	 && { $cp = $cp.offset ($kw.to_bytes ().len () as isize); true })
    };
}

/*
 * Read a file, but do no processing.  This is used to do regexp
 * matching on files that have no language defined.
 */
unsafe extern "C" fn just_read_file (inf: *mut FILE)
{
  while perhaps_more_input (inf)
    { readline (&raw mut lb, inf); }
}


/* Fortran parsing */

unsafe fn F_takeprec ()
{
  dbp = skip_spaces (dbp);
  if *dbp != b'*' as c_char
    { return; }
  dbp = dbp.offset (1);
  dbp = skip_spaces (dbp);
  if strneq (dbp, c"(*)".as_ptr (), 3)
    {
      dbp = dbp.offset (3);
      return;
    }
  if !c_isdigit (*dbp as c_int)
    {
      dbp = dbp.offset (-1);		/* force failure */
      return;
    }
  loop
    {
      dbp = dbp.offset (1);
      if !c_isdigit (*dbp as c_int)
	{ break; }
    }
}

unsafe fn F_getit (inf: *mut FILE)
{
  let mut cp: *mut c_char;

  dbp = skip_spaces (dbp);
  if *dbp == 0
    {
      readline (&raw mut lb, inf);
      dbp = lb.buffer;
      if *dbp.offset (5) != b'&' as c_char
	{ return; }
      dbp = dbp.offset (6);
      dbp = skip_spaces (dbp);
    }
  if !c_isalpha (*dbp as c_int) && *dbp != b'_' as c_char && *dbp != b'$' as c_char
    { return; }
  cp = dbp.offset (1);
  while *cp != 0 && intoken (*cp as c_uchar)
    { cp = cp.offset (1); }
  make_tag (dbp, cp.offset_from (dbp), true,
	    lb.buffer, cp.offset_from (lb.buffer) + 1, lineno, linecharno);
}


unsafe extern "C" fn Fortran_functions (inf: *mut FILE)
{
  LOOP_ON_INPUT_LINES! (inf, lb, dbp,
    {
      if *dbp == b'%' as c_char
	{ dbp = dbp.offset (1); }		/* Ratfor escape to fortran */
      dbp = skip_spaces (dbp);
      if *dbp == 0
	{ continue; }

      if LOOKING_AT_NOCASE! (dbp, c"recursive")
	{ dbp = skip_spaces (dbp); }

      if LOOKING_AT_NOCASE! (dbp, c"pure")
	{ dbp = skip_spaces (dbp); }

      if LOOKING_AT_NOCASE! (dbp, c"elemental")
	{ dbp = skip_spaces (dbp); }

      match c_tolower (*dbp as c_int)
	{
	105 /* 'i' */ =>
	  {
	    if nocase_tail (c"integer".as_ptr ())
	      { F_takeprec (); }
	  }
	114 /* 'r' */ =>
	  {
	    if nocase_tail (c"real".as_ptr ())
	      { F_takeprec (); }
	  }
	108 /* 'l' */ =>
	  {
	    if nocase_tail (c"logical".as_ptr ())
	      { F_takeprec (); }
	  }
	99 /* 'c' */ =>
	  {
	    if nocase_tail (c"complex".as_ptr ()) || nocase_tail (c"character".as_ptr ())
	      { F_takeprec (); }
	  }
	100 /* 'd' */ =>
	  {
	    if nocase_tail (c"double".as_ptr ())
	      {
		dbp = skip_spaces (dbp);
		if *dbp == 0
		  { continue; }
		if nocase_tail (c"precision".as_ptr ())
		  { break; }
		continue;
	      }
	  }
	_ => {}
	}
      dbp = skip_spaces (dbp);
      if *dbp == 0
	{ continue; }
      match c_tolower (*dbp as c_int)
	{
	109 /* 'm' */ =>
	  {
	    if nocase_tail (c"module".as_ptr ())
	      { F_getit (inf); }
	    continue;
	  }
	102 /* 'f' */ =>
	  {
	    if nocase_tail (c"function".as_ptr ())
	      { F_getit (inf); }
	    continue;
	  }
	115 /* 's' */ =>
	  {
	    if nocase_tail (c"subroutine".as_ptr ())
	      { F_getit (inf); }
	    continue;
	  }
	101 /* 'e' */ =>
	  {
	    if nocase_tail (c"entry".as_ptr ())
	      { F_getit (inf); }
	    continue;
	  }
	98 /* 'b' */ =>
	  {
	    if nocase_tail (c"blockdata".as_ptr ()) || nocase_tail (c"block data".as_ptr ())
	      {
		dbp = skip_spaces (dbp);
		if *dbp == 0	/* assume un-named */
		  { make_tag (c"blockdata".as_ptr (), 9, true,
			      lb.buffer, dbp.offset_from (lb.buffer), lineno, linecharno); }
		else
		  { F_getit (inf); }	/* look for name */
	      }
	    continue;
	  }
	_ => {}
	}
    })
}

/*
 * Go language support
 * Original code by Xi Lu <lx@shellcodes.org> (2016)
 */
unsafe extern "C" fn Go_functions (inf: *mut FILE)
{
  let mut cp: *mut c_char;
  let mut name: *mut c_char;

  LOOP_ON_INPUT_LINES! (inf, lb, cp,
    {
      cp = skip_spaces (cp);

      if LOOKING_AT! (cp, c"package")
	{
	  name = cp;
	  while !notinname (*cp as c_uchar) && *cp != 0
	    { cp = cp.offset (1); }
	  make_tag (name, cp.offset_from (name), false, lb.buffer,
		    cp.offset_from (lb.buffer) + 1, lineno, linecharno);
	}
      else if LOOKING_AT! (cp, c"func")
	{
	  /* Go implementation of interface, such as:
	     func (n *Integer) Add(m Integer) ...
	     skip `(n *Integer)` part.
	  */
	  if *cp == b'(' as c_char
	    {
	      while *cp != b')' as c_char
		{ cp = cp.offset (1); }
	      cp = skip_spaces (cp.offset (1));
	    }

	  if *cp != 0
	    {
	      name = cp;

	      while !notinname (*cp as c_uchar)
		{ cp = cp.offset (1); }

	      make_tag (name, cp.offset_from (name), true, lb.buffer,
			cp.offset_from (lb.buffer) + 1, lineno, linecharno);
	    }
	}
      else if members != 0 && LOOKING_AT! (cp, c"type")
	{
	  name = cp;

	  /* Ignore the likes of the following:
	     type (
	            A
	     )
	   */
	  if *cp == b'(' as c_char
	    { return; }

	  while !notinname (*cp as c_uchar) && *cp != 0
	    { cp = cp.offset (1); }

	  make_tag (name, cp.offset_from (name), false, lb.buffer,
		    cp.offset_from (lb.buffer) + 1, lineno, linecharno);
	}
    })
}

/*
 * Ada parsing
 * Original code by
 * Philippe Waroquiers (1998)
 */

/* Once we are positioned after an "interesting" keyword, let's get
   the real tag value necessary. */
unsafe fn Ada_getit (inf: *mut FILE, mut name_qualifier: *const c_char)
{
  let mut cp: *mut c_char;
  let mut name: *mut c_char;
  let mut c: c_char;

  while perhaps_more_input (inf)
    {
      dbp = skip_spaces (dbp);
      if *dbp == 0
	  || (*dbp == b'-' as c_char && *dbp.offset (1) == b'-' as c_char)
	{
	  readline (&raw mut lb, inf);
	  dbp = lb.buffer;
	}
      match c_tolower (*dbp as c_int)
        {
        98 /* 'b' */ =>
          {
          if nocase_tail (c"body".as_ptr ())
            {
              /* Skipping body of   procedure body   or   package body or ....
		 resetting qualifier to body instead of spec. */
              name_qualifier = c"/b".as_ptr ();
              continue;
            }
          }
        116 /* 't' */ =>
          {
          /* Skipping type of   task type   or   protected type ... */
          if nocase_tail (c"type".as_ptr ())
            { continue; }
          }
        _ => {}
        }
      if *dbp == b'"' as c_char
	{
	  dbp = dbp.offset (1);
	  cp = dbp;
	  while *cp != 0 && *cp != b'"' as c_char
	    { cp = cp.offset (1); }
	}
      else
	{
	  dbp = skip_spaces (dbp);
	  cp = dbp;
	  while c_isalnum (*cp as c_int) || *cp == b'_' as c_char || *cp == b'.' as c_char
	    { cp = cp.offset (1); }
	  if cp == dbp
	    { return; }
	}
      c = *cp;
      *cp = 0;
      name = concat (dbp, name_qualifier, c"".as_ptr ());
      *cp = c;
      make_tag (name, strlen (name) as ptrdiff_t, true,
		lb.buffer, cp.offset_from (lb.buffer) + 1, lineno, linecharno);
      free (name as *mut c_void);
      if c == b'"' as c_char
	{ dbp = cp.offset (1); }
      return;
    }
}

unsafe extern "C" fn Ada_funcs (inf: *mut FILE)
{
  let mut inquote: bool = false;
  let mut skip_till_semicolumn: bool = false;

  LOOP_ON_INPUT_LINES! (inf, lb, dbp,
    {
      'ada_char: while *dbp != 0
	{
	  /* Skip a string i.e. "abcd". */
	  if inquote || (*dbp == b'"' as c_char)
	    {
	      dbp = strchr (dbp.offset (if inquote { 0 } else { 1 }), b'"' as c_int);
	      if dbp != std::ptr::null_mut ()
		{
		  inquote = false;
		  dbp = dbp.offset (1);
		  continue;	/* advance char */
		}
	      else
		{
		  inquote = true;
		  break;	/* advance line */
		}
	    }

	  /* Skip comments. */
	  if *dbp == b'-' as c_char && *dbp.offset (1) == b'-' as c_char
	    { break; }		/* advance line */

	  /* Skip character enclosed in single quote i.e. 'a'
	     and skip single quote starting an attribute i.e. 'Image. */
	  if *dbp == b'\'' as c_char
	    {
	      dbp = dbp.offset (1);
	      if *dbp != 0
		{ dbp = dbp.offset (1); }
	      continue;
	    }

	  if skip_till_semicolumn
	    {
	      if *dbp == b';' as c_char
		{ skip_till_semicolumn = false; }
	      dbp = dbp.offset (1);
	      continue;         /* advance char */
	    }

	  /* Search for beginning of a token.  */
	  if !begtoken (*dbp as c_uchar)
	    {
	      dbp = dbp.offset (1);
	      continue;		/* advance char */
	    }

	  /* We are at the beginning of a token. */
	  'ada_switch: {
	  match c_tolower (*dbp as c_int)
	    {
	    102 /* 'f' */ =>
	      {
		if packages_only == 0 && nocase_tail (c"function".as_ptr ())
		  { Ada_getit (inf, c"/f".as_ptr ()); }
		else
		  { break 'ada_switch; }	/* from switch */
		continue 'ada_char;		/* advance char */
	      }
	    112 /* 'p' */ =>
	      {
		if packages_only == 0 && nocase_tail (c"procedure".as_ptr ())
		  { Ada_getit (inf, c"/p".as_ptr ()); }
		else if nocase_tail (c"package".as_ptr ())
		  { Ada_getit (inf, c"/s".as_ptr ()); }
		else if nocase_tail (c"protected".as_ptr ()) /* protected type */
		  { Ada_getit (inf, c"/t".as_ptr ()); }
		else
		  { break 'ada_switch; }	/* from switch */
		continue 'ada_char;		/* advance char */
	      }

	    117 /* 'u' */ | 116 /* 't' */ =>
	      {
		if c_tolower (*dbp as c_int) == 117 /* 'u' */
		  {
		    if typedefs && packages_only == 0 && nocase_tail (c"use".as_ptr ())
		      {
			/* when tagging types, avoid tagging  use type Pack.Typename;
			   for this, we will skip everything till a ; */
			skip_till_semicolumn = true;
			continue 'ada_char;     /* advance char */
		      }
		    /* C's `case 'u':` with a false condition falls through into
		       `case 't':` (C has no FALLTHROUGH marker there).  */
		  }
		if packages_only == 0 && nocase_tail (c"task".as_ptr ())
		  { Ada_getit (inf, c"/k".as_ptr ()); }
		else if typedefs && packages_only == 0 && nocase_tail (c"type".as_ptr ())
		  {
		    Ada_getit (inf, c"/t".as_ptr ());
		    while *dbp != 0
		      { dbp = dbp.offset (1); }
		  }
		else
		  { break 'ada_switch; }	/* from switch */
		continue 'ada_char;		/* advance char */
	      }
	    _ => {}
	    }
	  }

	  /* Look for the end of the token. */
	  while !endtoken (*dbp as c_uchar)
	    { dbp = dbp.offset (1); }

	} /* advance char */
    }) /* advance line */
}

/*
 * Unix and microcontroller assembly tag handling
 * Labels:  /^[a-zA-Z_.$][a-zA_Z0-9_.$]*[: ^I^J]/
 * Idea by Bob Weiner, Motorola Inc. (1994)
 */
unsafe extern "C" fn Asm_labels (inf: *mut FILE)
{
  let mut cp: *mut c_char;

  LOOP_ON_INPUT_LINES! (inf, lb, cp,
    {
      /* If first char is alphabetic or one of [_.$], test for colon
	 following identifier. */
      if c_isalpha (*cp as c_int) || *cp == b'_' as c_char || *cp == b'.' as c_char
	  || *cp == b'$' as c_char
 	{
 	  /* Read past label. */
	  cp = cp.offset (1);
	  while c_isalnum (*cp as c_int) || *cp == b'_' as c_char || *cp == b'.' as c_char
		|| *cp == b'$' as c_char
	    { cp = cp.offset (1); }
	  if *cp == b':' as c_char || c_isspace (*cp as c_int)
	    /* Found end of label, so copy it and add it to the table. */
	    { make_tag (lb.buffer, cp.offset_from (lb.buffer), true,
			lb.buffer, cp.offset_from (lb.buffer) + 1, lineno, linecharno); }
 	}
    })
}

/*
 * Perl support
 * Perl sub names: /^sub[ \t\n]+[^ \t\n{]+/
 *                 /^use constant[ \t\n]+[^ \t\n{=,;]+/
 * Perl variable names: /^(my|local).../
 * Original code by Bart Robinson <lomew@cs.utah.edu> (1995)
 * Additions by Michael Ernst <mernst@alum.mit.edu> (1997)
 * Ideas by Kai Grossjohann <Kai.Grossjohann@CS.Uni-Dortmund.DE> (2001)
 */
unsafe extern "C" fn Perl_functions (inf: *mut FILE)
{
  let mut package: *mut c_char = savestr (c"main".as_ptr ()); /* current package name */
  let mut cp: *mut c_char;

  LOOP_ON_INPUT_LINES! (inf, lb, cp,
    {
      cp = skip_spaces (cp);

      /* C's `goto subr;` from the `use constant` arm enters the `sub` arm's body at the
	 `subr:` label below; the port selects that body with a flag (PORTING.md 2.2).  */
      if LOOKING_AT! (cp, c"package")
	{
	  free (package as *mut c_void);
	  get_tag (cp, &raw mut package);
	}
      else
	{
	  let mut use_subr: bool = false;

	  if LOOKING_AT! (cp, c"sub")
	    { use_subr = true; }
	  else if LOOKING_AT! (cp, c"use constant")
	       || LOOKING_AT! (cp, c"use constant::defer")
	    {
	      /* For hash style multi-constant like
		    use constant { FOO => 123,
		                   BAR => 456 };
		 only the first FOO is picked up.  Parsing across the value
		 expressions would be difficult in general, due to possible nested
		 hashes, here-documents, etc.  */
	      if *cp == b'{' as c_char
		{ cp = skip_spaces (cp.offset (1)); }
	      use_subr = true;
	    }
	  if use_subr
	    {
	      let mut pos: *mut c_char;
	      let mut sp: *mut c_char;

	    /* subr: */
	      sp = cp;
	      while !notinname (*cp as c_uchar)
		{ cp = cp.offset (1); }
	      if cp == sp
		{ continue; }		/* nothing found */
	      pos = strchr (sp, b':' as c_int);
	      if pos != std::ptr::null_mut () && pos < cp
		  && *pos.offset (1) == b':' as c_char
		{
		  /* The name is already qualified. */
		  if class_qualify == 0
		    {
		      let mut q: *mut c_char = pos.offset (2);
		      let mut qpos: *mut c_char;
		      loop
			{
			  qpos = strchr (q, b':' as c_int);
			  if !(qpos != std::ptr::null_mut ()
			       && qpos < cp
			       && *qpos.offset (1) == b':' as c_char)
			    { break; }
			  q = qpos.offset (2);
			}
		      sp = q;
		    }
		  make_tag (sp, cp.offset_from (sp), true,
			    lb.buffer, cp.offset_from (lb.buffer) + 1, lineno, linecharno);
		}
	      else if class_qualify != 0
		/* Qualify it. */
		{
		  let mut savechar: c_char;
		  let mut name: *mut c_char;

		  savechar = *cp;
		  *cp = 0;
		  name = concat (package, c"::".as_ptr (), sp);
		  *cp = savechar;
		  make_tag (name, strlen (name) as ptrdiff_t, true,
			    lb.buffer, cp.offset_from (lb.buffer) + 1, lineno, linecharno);
		  free (name as *mut c_void);
		}
	      else
		{ make_tag (sp, cp.offset_from (sp), true,
			    lb.buffer, cp.offset_from (lb.buffer) + 1, lineno, linecharno); }
	    }
	  else if globals != 0	/* only if we are tagging global vars */
 	    {
	      /* Skip a qualifier, if any. */
	      let qual: bool = LOOKING_AT! (cp, c"my") || LOOKING_AT! (cp, c"local");
	      /* After "my" or "local", but before any following paren or space. */
	      let mut varstart: *mut c_char = cp;

	      if qual		/* should this be removed?  If yes, how? */
		  && (*cp == b'$' as c_char || *cp == b'@' as c_char
		      || *cp == b'%' as c_char)
		{
		  varstart = varstart.offset (1);
		  loop
		    {
		      cp = cp.offset (1);
		      if !(c_isalnum (*cp as c_int) || *cp == b'_' as c_char)
			{ break; }
		    }
		}
	      else if qual
		{
		  /* Should be examining a variable list at this point;
		     could insist on seeing an open parenthesis. */
		  while *cp != 0 && *cp != b';' as c_char && *cp != b'=' as c_char
			&& *cp != b')' as c_char
		    { cp = cp.offset (1); }
		}
	      else
		{ continue; }

	      make_tag (varstart, cp.offset_from (varstart), false,
			lb.buffer, cp.offset_from (lb.buffer) + 1, lineno, linecharno);
	    }
	}
    });
  free (package as *mut c_void);
}


/*
 * Python support
 * Look for /^[\t]*def[ \t\n]+[^ \t\n(:]+/ or /^class[ \t\n]+[^ \t\n(:]+/
 * Idea by Eric S. Raymond <esr@thyrsus.com> (1997)
 * More ideas by seb bacon <seb@jamkit.com> (2002)
 */
unsafe extern "C" fn Python_functions (inf: *mut FILE)
{
  let mut cp: *mut c_char;

  LOOP_ON_INPUT_LINES! (inf, lb, cp,
    {
      cp = skip_spaces (cp);
      if LOOKING_AT! (cp, c"def") || LOOKING_AT! (cp, c"class")
	{
	  let mut name: *mut c_char = cp;
	  while !notinname (*cp as c_uchar) && *cp != b':' as c_char
	    { cp = cp.offset (1); }
	  make_tag (name, cp.offset_from (name), true,
		    lb.buffer, cp.offset_from (lb.buffer) + 1, lineno, linecharno);
	}
    })
}

/*
 * Ruby support
 * Original code by Xi Lu <lx@shellcodes.org> (2015)
 */
unsafe extern "C" fn Ruby_functions (inf: *mut FILE)
{
  let mut cp: *mut c_char = std::ptr::null_mut ();
  let mut reader: bool = false; let mut writer: bool = false;
  let mut alias: bool = false; let mut continuation: bool = false;

  LOOP_ON_INPUT_LINES! (inf, lb, cp,
    {
      let mut is_class: bool = false;
      let mut is_method: bool = false;
      let mut name: *mut c_char;

      cp = skip_spaces (cp);
      if !continuation
	  /* Constants.  */
	  && c_isalpha (*cp as c_int) && c_isupper (*cp as c_int)
	{
	  let mut bp: *mut c_char;
	  let mut colon: *mut c_char = std::ptr::null_mut ();

	  name = cp;

	  cp = cp.offset (1);
	  while c_isalnum (*cp as c_int) || *cp == b'_' as c_char || *cp == b':' as c_char
	    {
	      if *cp == b':' as c_char
		{ colon = cp; }
	      cp = cp.offset (1);
	    }
	  if cp > name.offset (1)
	    {
	      bp = skip_spaces (cp);
	      if *bp == b'=' as c_char && !(*bp.offset (1) == b'=' as c_char
					    || *bp.offset (1) == b'>' as c_char)
		{
		  if colon != std::ptr::null_mut () && !c_isspace (*colon.offset (1) as c_int)
		    { name = colon.offset (1); }
		  make_tag (name, cp.offset_from (name), false,
			    lb.buffer, cp.offset_from (lb.buffer) + 1, lineno, linecharno);
		}
	    }
	}
      else if !continuation
	       /* Modules, classes, methods.  */
	       && {
		    is_method = LOOKING_AT! (cp, c"def");
		    is_method
		      || {
			   is_class = LOOKING_AT! (cp, c"class");
			   is_class
			 }
		      || LOOKING_AT! (cp, c"module")
		  }
	{
	  /* C 5064-5065: `const char self_name[] = "self."; const size_t self_size1 =
	     sizeof (self_name) - 1;`  */
	  const self_name: &CStr = c"self.";
	  const self_size1: size_t = 5;

	  name = cp;

	 /* Ruby method names can end in a '='.  Also, operator overloading can
	    define operators whose names include '='.  */
	  while !notinname (*cp as c_uchar) || *cp == b'=' as c_char
	    {
	      cp = cp.offset (1);
	      if *cp.offset (-1) == b':' as c_char
		{ name = cp; }
	    }

	  /* Remove "self." from the method name.  */
	  if cp.offset_from (name) as size_t > self_size1
	      && strneq (name, self_name.as_ptr (), self_size1)
	    { name = name.offset (self_size1 as isize); }

	  /* Remove the class/module qualifiers from method names.  */
	  if is_method
	    {
	      let mut q: *mut c_char;

	      q = name;
	      while q < cp && *q != b'.' as c_char
		{ q = q.offset (1); }
	      if q < cp.offset (-1)	/* punt if we see just "FOO." */
		{ name = q.offset (1); }
	    }

	  /* Don't tag singleton classes.  */
	  if is_class && strneq (name, c"<<".as_ptr (), 2) && cp == name.offset (2)
	    { continue; }

	  make_tag (name, cp.offset_from (name), true,
		    lb.buffer, cp.offset_from (lb.buffer) + 1, lineno, linecharno);
	}
      else
	{
	  /* Tag accessors and aliases.  */

	  if !continuation
	    {
	      /* C's `reader = writer = alias = false;` in C's right-to-left order.  */
	      alias = false;
	      writer = false;
	      reader = false;
	    }

	  while *cp != 0 && *cp != b'#' as c_char
	    {
	      if !continuation
		{
		  alias = false;
		  writer = false;
		  reader = false;
		  if LOOKING_AT! (cp, c"attr_reader")
		    { reader = true; }
		  else if LOOKING_AT! (cp, c"attr_writer")
		    { writer = true; }
		  else if LOOKING_AT! (cp, c"attr_accessor")
		    {
		      reader = true;
		      writer = true;
		    }
		  else if LOOKING_AT! (cp, c"alias_method")
		    { alias = true; }
		}
	      if reader || writer || alias
		{
		  loop
		    {
		      'do_body: {
		      let mut np: *mut c_char;

		      cp = skip_spaces (cp);
		      if *cp == b'(' as c_char
			{ cp = skip_spaces (cp.offset (1)); }
		      np = cp;
		      cp = skip_name (cp);
		      if *np != b':' as c_char
			{ break 'do_body; }	/* C's `continue` (to the do-while test) */
		      np = np.offset (1);
		      if reader
			{
			  make_tag (np, cp.offset_from (np), true,
				    lb.buffer, cp.offset_from (lb.buffer) + 1,
				    lineno, linecharno);
			  continuation = false;
			}
		      if writer
			{
			  let name_len: size_t = (cp.offset_from (np) + 1) as size_t;
			  let mut wr_name: *mut c_char =
			    xmalloc ((name_len + 1) as ptrdiff_t) as *mut c_char;

			  strcpy (mempcpy (wr_name as *mut c_void,
					       np as *const c_void,
					       name_len - 1) as *mut c_char,
				  c"=".as_ptr ());
			  pfnote (wr_name, true, lb.buffer,
				  cp.offset_from (lb.buffer) + 1, lineno, linecharno);
			  if debug != 0
			    { fprintf (stderr, c"%s on %s:%jd: %s\n".as_ptr (),	/* C: `"%"PRIdMAX` */
				       wr_name, (*curfdp).taggedfname, lineno, lb.buffer); }
			  continuation = false;
			}
		      if alias
			{
			  if !continuation
			    { make_tag (np, cp.offset_from (np), true,
					lb.buffer, cp.offset_from (lb.buffer) + 1,
					lineno, linecharno); }
			  continuation = false;
			  while *cp != 0 && *cp != b'#' as c_char && *cp != b';' as c_char
			    {
			      if *cp == b',' as c_char
				{ continuation = true; }
			      else if !c_isspace (*cp as c_int)
				{ continuation = false; }
			      cp = cp.offset (1);
			    }
			  if *cp == b';' as c_char
			    { continuation = false; }
			}
		      cp = skip_spaces (cp);
		      }
		      /* C's `do ... while` test.  */
		      let mut go: bool;
		      if alias
			{ go = *cp == b',' as c_char; }
		      else
			{
			  continuation = *cp == b',' as c_char;
			  go = continuation;
			}
		      if go
			{
			  cp = skip_spaces (cp.offset (1));
			  go = *cp != 0 && *cp != b'#' as c_char;
			}
		      if !go
			{ break; }
		    }
		}
	      if *cp != b'#' as c_char
		{ cp = skip_name (cp); }
	      while *cp != 0 && *cp != b'#' as c_char && notinname (*cp as c_uchar)
		{ cp = cp.offset (1); }
	    }
	}
    })
}

/*
 * Rust support
 * Look for:
 *  - fn: Function
 *  - struct: Structure
 *  - enum: Enumeration
 *  - macro_rules!: Macro
 */
unsafe extern "C" fn Rust_entries (inf: *mut FILE)
{
  let mut cp: *mut c_char; let mut name: *mut c_char;
  let mut is_func: bool = false;

  LOOP_ON_INPUT_LINES! (inf, lb, cp,
    {
      cp = skip_spaces (cp);
      name = cp;

      /* Skip 'pub' keyword.  */
      let _ = LOOKING_AT! (cp, c"pub");

      /* Look for define.  */
      let found: bool = {
	  is_func = LOOKING_AT! (cp, c"fn");
	  is_func
	      || LOOKING_AT! (cp, c"enum")
	      || LOOKING_AT! (cp, c"struct")
	      || {
		   is_func = LOOKING_AT! (cp, c"macro_rules!");
		   is_func
		 }
      };
      if found
	{
	  cp = skip_spaces (cp);
	  name = cp;

	  while !notinname (*cp as c_uchar)
	    { cp = cp.offset (1); }

	  make_tag (name, cp.offset_from (name), is_func,
		    lb.buffer, cp.offset_from (lb.buffer) + 1,
		    lineno, linecharno);
	  is_func = false;
	}
    })
}

/*
 * PHP support
 * Look for:
 *  - /^[ \t]*function[ \t\n]+[^ \t\n(]+/
 *  - /^[ \t]*class[ \t\n]+[^ \t\n]+/
 *  - /^[ \t]*define\(\"[^\"]+/
 * Only with --members:
 *  - /^[ \t]*var[ \t\n]+\$[^ \t\n=;]/
 * Idea by Diez B. Roggisch (2001)
 */
unsafe extern "C" fn PHP_functions (inf: *mut FILE)
{
  let mut cp: *mut c_char; let mut name: *mut c_char;
  let mut search_identifier: bool = false;

  LOOP_ON_INPUT_LINES! (inf, lb, cp,
    {
      cp = skip_spaces (cp);
      name = cp;
      if search_identifier
	  && *cp != 0
	{
	  while !notinname (*cp as c_uchar)
	    { cp = cp.offset (1); }
	  make_tag (name, cp.offset_from (name), true,
		    lb.buffer, cp.offset_from (lb.buffer) + 1, lineno, linecharno);
	  search_identifier = false;
	}
      else if LOOKING_AT! (cp, c"function")
	{
	  if *cp == b'&' as c_char
	    { cp = skip_spaces (cp.offset (1)); }
	  if *cp != 0
	    {
	      name = cp;
	      while !notinname (*cp as c_uchar)
		{ cp = cp.offset (1); }
	      make_tag (name, cp.offset_from (name), true,
			lb.buffer, cp.offset_from (lb.buffer) + 1, lineno, linecharno);
	    }
	  else
	    { search_identifier = true; }
	}
      else if LOOKING_AT! (cp, c"class")
	{
	  if *cp != 0
	    {
	      name = cp;
	      while *cp != 0 && !c_isspace (*cp as c_int)
		{ cp = cp.offset (1); }
	      make_tag (name, cp.offset_from (name), false,
			lb.buffer, cp.offset_from (lb.buffer) + 1, lineno, linecharno);
	    }
	  else
	    { search_identifier = true; }
	}
      else if strneq (cp, c"define".as_ptr (), 6)
	       && { cp = skip_spaces (cp.offset (6)); cp != std::ptr::null_mut () }
	       && { let ch: c_char = *cp; cp = cp.offset (1); ch == b'(' as c_char }
	       && (*cp == b'"' as c_char || *cp == b'\'' as c_char)
	{
	  let mut quote: c_char = *cp;
	  cp = cp.offset (1);
	  name = cp;
	  while *cp != quote && *cp != 0
	    { cp = cp.offset (1); }
	  make_tag (name, cp.offset_from (name), false,
		    lb.buffer, cp.offset_from (lb.buffer) + 1, lineno, linecharno);
	}
      else if members != 0
	       && LOOKING_AT! (cp, c"var")
	       && *cp == b'$' as c_char
	{
	  name = cp;
	  while !notinname (*cp as c_uchar)
	    { cp = cp.offset (1); }
	  make_tag (name, cp.offset_from (name), false,
		    lb.buffer, cp.offset_from (lb.buffer) + 1, lineno, linecharno);
	}
    })
}

/*
 * Cobol tag functions
 * We could look for anything that could be a paragraph name.
 * i.e. anything that starts in column 8 is one word and ends in a full stop.
 * Idea by Corny de Souza (1993)
 */
unsafe extern "C" fn Cobol_paragraphs (inf: *mut FILE)
{
  let mut bp: *mut c_char; let mut ep: *mut c_char;

  LOOP_ON_INPUT_LINES! (inf, lb, bp,
    {
      if lb.len < 9
	{ continue; }
      bp = bp.offset (8);

      /* If eoln, compiler option or comment ignore whole line. */
      if *bp.offset (-1) != b' ' as c_char || !c_isalnum (*bp as c_int)
        { continue; }

      ep = bp;
      while c_isalnum (*ep as c_int) || *ep == b'-' as c_char
	{ ep = ep.offset (1); }
      { let ch: c_char = *ep; ep = ep.offset (1);
        if ch == b'.' as c_char
	  { make_tag (bp, ep.offset_from (bp), true,
		      lb.buffer, ep.offset_from (lb.buffer) + 1, lineno, linecharno); } }
    })
}

/*
 * Makefile support
 * Ideas by Assar Westerlund <assar@sics.se> (2001)
 */
unsafe extern "C" fn Makefile_targets (inf: *mut FILE)
{
  let mut bp: *mut c_char;

  LOOP_ON_INPUT_LINES! (inf, lb, bp,
    {
      if *bp == b'\t' as c_char || *bp == b'#' as c_char
	{ continue; }
      while *bp != 0 && *bp != b'=' as c_char && *bp != b':' as c_char
	{ bp = bp.offset (1); }
      if *bp == b':' as c_char || (globals != 0 && *bp == b'=' as c_char)
	{
	  /* We should detect if there is more than one tag, but we do not.
	     We just skip initial and final spaces. */
	  let mut namestart: *mut c_char = skip_spaces (lb.buffer);
	  loop
	    {
	      bp = bp.offset (-1);
	      if !(bp > namestart)
		{ break; }
	      if !notinname (*bp as c_uchar)
		{ break; }
	    }
	  make_tag (namestart, bp.offset_from (namestart) + 1, true,
		    lb.buffer, bp.offset_from (lb.buffer) + 2, lineno, linecharno);
	}
    })
}

/*
 * Pascal parsing
 * Original code by Mosur K. Mohan (1989)
 *
 *  Locates tags for procedures & functions.  Doesn't do any type- or
 *  var-definitions.  It does look for the keyword "extern" or
 *  "forward" immediately following the procedure statement; if found,
 *  the tag is skipped.
 */
unsafe extern "C" fn Pascal_functions (inf: *mut FILE)
{
  let mut tline: linebuffer;		/* mostly copied from C_entries */
  let mut save_lcno: intmax_t; let mut save_lineno: intmax_t;
  let mut namelen: ptrdiff_t; let mut taglen: ptrdiff_t;
  let mut c: c_char; let mut name: *mut c_char;

  let mut incomment: bool;		/* each of these flags is true if: */
  let mut inquote: bool;		/* point is inside a comment */
  let mut get_tagname: bool;		/* point is inside '..' string */
  let mut found_tag: bool;		/* point is after PROCEDURE/FUNCTION
				   keyword, so next item = potential tag */
  let mut inparms: bool;		/* point is after a potential tag */
  let mut verify_tag: bool;		/* point is within parameter-list */

  save_lcno = 0; save_lineno = 0; namelen = 0; taglen = 0; /* keep compiler quiet */
  name = std::ptr::null_mut ();		/* keep compiler quiet */
  dbp = lb.buffer;
  *dbp = 0;
  tline = linebuffer { size: 0, len: 0, buffer: std::ptr::null_mut () };
  linebuffer_init (&raw mut tline);

  incomment = false; inquote = false;
  found_tag = false;		/* have a proc name; check if extern */
  get_tagname = false;		/* found "procedure" keyword	     */
  inparms = false;		/* found '(' after "proc"            */
  verify_tag = false;		/* check if "extern" is ahead        */


  while perhaps_more_input (inf) /* long main loop to get next char */
    {
      c = *dbp;
      dbp = dbp.offset (1);
      if c == 0		/* if end of line */
	{
	  readline (&raw mut lb, inf);
	  dbp = lb.buffer;
	  if *dbp == 0
	    { continue; }
	  if !((found_tag && verify_tag)
		|| get_tagname)
	    { c = *dbp; dbp = dbp.offset (1); }	/* only if don't need *dbp pointing
				   to the beginning of the name of
				   the procedure or function */
	}
      if incomment
	{
	  if c == b'}' as c_char		/* within { } comments */
	    { incomment = false; }
	  else if c == b'*' as c_char && *dbp == b')' as c_char /* within (* *) comments */
	    {
	      dbp = dbp.offset (1);
	      incomment = false;
	    }
	  continue;
	}
      else if inquote
	{
	  if c == b'\'' as c_char
	    { inquote = false; }
	  continue;
	}
      else
	{
	  match c
	    {
	    39 /* '\'' */ =>
	      {
		inquote = true;	/* found first quote */
		continue;
	      }
	    123 /* '{' */ =>		/* found open { comment */
	      {
		incomment = true;
		continue;
	      }
	    40 /* '(' */ =>
	      {
		if *dbp == b'*' as c_char	/* found open (* comment */
		  {
		    incomment = true;
		    dbp = dbp.offset (1);
		  }
		else if found_tag	/* found '(' after tag, i.e., parm-list */
		  { inparms = true; }
		continue;
	      }
	    41 /* ')' */ =>		/* end of parms list */
	      {
		if inparms
		  { inparms = false; }
		continue;
	      }
	    59 /* ';' */ =>
	      {
		if found_tag && !inparms /* end of proc or fn stmt */
		  {
		    verify_tag = true;
		    /* C's `break` leaves the switch for the check below.  */
		  }
		else
		  { continue; }
	      }
	    _ => {}
	    }
	}
      if found_tag && verify_tag && (*dbp != b' ' as c_char)
	{
	  /* Check if this is an "extern" declaration. */
	  if *dbp == 0
	    { continue; }
	  if c_tolower (*dbp as c_int) == b'e' as c_int
	    {
	      if nocase_tail (c"extern".as_ptr ()) /* superfluous, really! */
		{
		  found_tag = false;
		  verify_tag = false;
		}
	    }
	  else if c_tolower (*dbp as c_int) == b'f' as c_int
	    {
	      if nocase_tail (c"forward".as_ptr ()) /* check for forward reference */
		{
		  found_tag = false;
		  verify_tag = false;
		}
	    }
	  if found_tag && verify_tag /* not external proc, so make tag */
	    {
	      found_tag = false;
	      verify_tag = false;
	      make_tag (name, namelen, true,
			tline.buffer, taglen, save_lineno, save_lcno);
	      continue;
	    }
	}
      if get_tagname		/* grab name of proc or fn */
	{
	  let mut cp: *mut c_char;

	  if *dbp == 0
	    { continue; }

	  /* Find block name. */
	  cp = dbp.offset (1);
	  while *cp != 0 && !endtoken (*cp as c_uchar)
	    { cp = cp.offset (1); }

	  /* Save all values for later tagging. */
	  linebuffer_setlen (&raw mut tline, lb.len);
	  strcpy (tline.buffer, lb.buffer);
	  save_lineno = lineno;
	  save_lcno = linecharno;
	  name = tline.buffer.offset (dbp.offset_from (lb.buffer));
	  namelen = cp.offset_from (dbp);
	  taglen = cp.offset_from (lb.buffer) + 1;

	  dbp = cp;		/* set dbp to e-o-token */
	  get_tagname = false;
	  found_tag = true;
	  continue;

	  /* And proceed to check for "extern". */
	}
      else if !incomment && !inquote && !found_tag
	{
	  /* Check for proc/fn keywords. */
	  match c_tolower (c as c_int)
	    {
	    112 /* 'p' */ =>
	      {
		if nocase_tail (c"rocedure".as_ptr ()) /* c = 'p', dbp has advanced */
		  { get_tagname = true; }
		continue;
	      }
	    102 /* 'f' */ =>
	      {
		if nocase_tail (c"unction".as_ptr ())
		  { get_tagname = true; }
		continue;
	      }
	    _ => {}
	    }
	}
    } /* while not eof */

  free (tline.buffer as *mut c_void);
}


/*
 * Lisp tag functions
 *  look for (def or (DEF, quote or QUOTE
 */

unsafe fn L_getit ()
{
  if *dbp == b'\'' as c_char		/* Skip prefix quote */
    { dbp = dbp.offset (1); }
  else if *dbp == b'(' as c_char
  {
    dbp = dbp.offset (1);
    /* Try to skip "(quote " */
    if !LOOKING_AT! (dbp, c"quote") && !LOOKING_AT! (dbp, c"QUOTE")
      /* Ok, then skip "(" before name in (defstruct (foo)) */
      { dbp = skip_spaces (dbp); }
  }
  get_lispy_tag (dbp);
}

unsafe extern "C" fn Lisp_functions (inf: *mut FILE)
{
  LOOP_ON_INPUT_LINES! (inf, lb, dbp,
    {
      if *dbp != b'(' as c_char
	{ continue; }

      /* "(defvar foo)" is a declaration rather than a definition.  */
      if declarations == 0
	{
	  let mut p: *mut c_char = dbp.offset (1);
	  if LOOKING_AT! (p, c"defvar")
	    {
	      p = skip_name (p); /* past var name */
	      p = skip_spaces (p);
	      if *p == b')' as c_char
		{ continue; }
	    }
	}

      if strneq (dbp.offset (1), c"cl-".as_ptr (), 3)
	  || strneq (dbp.offset (1), c"CL-".as_ptr (), 3)
	{ dbp = dbp.offset (3); }

      if strneq (dbp.offset (1), c"def".as_ptr (), 3)
	  || strneq (dbp.offset (1), c"DEF".as_ptr (), 3)
	{
	  dbp = skip_non_spaces (dbp);
	  dbp = skip_spaces (dbp);
	  L_getit ();
	}
      else
	{
	  /* Check for (foo::defmumble name-defined ... */
	  loop
	    {
	      dbp = dbp.offset (1);
	      if !(!notinname (*dbp as c_uchar) && *dbp != b':' as c_char)
		{ break; }
	    }
	  if *dbp == b':' as c_char
	    {
	      loop
		{
		  dbp = dbp.offset (1);
		  if !(*dbp == b':' as c_char)
		    { break; }
		}

	      if strneq (dbp, c"def".as_ptr (), 3) || strneq (dbp, c"DEF".as_ptr (), 3)
		{
		  dbp = skip_non_spaces (dbp);
		  dbp = skip_spaces (dbp);
		  L_getit ();
		}
	    }
	}
    })
}

/*
 * Lua script language parsing
 * Original code by David A. Capello <dacap@users.sourceforge.net> (2004)
 *
 *  "function" and "local function" are tags if they start at column 1.
 */
unsafe extern "C" fn Lua_functions (inf: *mut FILE)
{
  let mut bp: *mut c_char;

  LOOP_ON_INPUT_LINES! (inf, lb, bp,
    {
      bp = skip_spaces (bp);
      if *bp != b'f' as c_char && *bp != b'l' as c_char
	{ continue; }

      let _ = LOOKING_AT! (bp, c"local"); /* skip possible "local" */

      if LOOKING_AT! (bp, c"function")
	{
	  let mut tag_name: *mut c_char = std::ptr::null_mut ();
	  let mut tp_dot: *mut c_char; let mut tp_colon: *mut c_char;

	  get_tag (bp, &raw mut tag_name);
	  /* If the tag ends with ".foo" or ":foo", make an additional tag for
	     "foo".  */
	  tp_dot = strrchr (tag_name, b'.' as c_int);
	  tp_colon = strrchr (tag_name, b':' as c_int);
	  if tp_dot != std::ptr::null_mut () || tp_colon != std::ptr::null_mut ()
	    {
	      let mut p: *mut c_char =
		if tp_dot > tp_colon { tp_dot } else { tp_colon };
	      let mut len_add: ptrdiff_t = p.offset_from (tag_name) + 1;

	      get_tag (bp.offset (len_add), std::ptr::null_mut ());
	    }
	}
    })
}

/*
 * PostScript tags
 * Just look for lines where the first character is '/'
 * Also look at "defineps" for PSWrap
 * Ideas by:
 *   Richard Mlynarik <mly@adoc.xerox.com> (1997)
 *   Masatake Yamato <masata-y@is.aist-nara.ac.jp> (1999)
 */
unsafe extern "C" fn PS_functions (inf: *mut FILE)
{
  let mut bp: *mut c_char; let mut ep: *mut c_char;

  LOOP_ON_INPUT_LINES! (inf, lb, bp,
    {
      if *bp == b'/' as c_char
	{
	  ep = bp.offset (1);
	  while *ep != 0 && *ep != b' ' as c_char && *ep != b'{' as c_char
	    { ep = ep.offset (1); }
	  make_tag (bp, ep.offset_from (bp), true,
		    lb.buffer, ep.offset_from (lb.buffer) + 1, lineno, linecharno);
	}
      else if LOOKING_AT! (bp, c"defineps")
	{ get_tag (bp, std::ptr::null_mut ()); }
    })
}

/*
 * Forth tags
 * Ignore anything after \ followed by space or in ( )
 * Look for words defined by :
 * Look for constant, code, create, defer, value, and variable
 * OBP extensions:  Look for buffer:, field,
 * Ideas by Eduardo Horvath <eeh@netbsd.org> (2004)
 */
unsafe extern "C" fn Forth_words (inf: *mut FILE)
{
  let mut bp: *mut c_char;

  LOOP_ON_INPUT_LINES! (inf, lb, bp,
    { while { bp = skip_spaces (bp); *bp != 0 }
      {
      if *bp == b'\\' as c_char && c_isspace (*bp.offset (1) as c_int)
	{ break; }			/* read next line */
      else if *bp == b'(' as c_char && c_isspace (*bp.offset (1) as c_int)
	{
	  loop			/* skip to ) or eol */
	    {
	      bp = bp.offset (1);
	      if !(*bp != b')' as c_char && *bp != 0)
		{ break; }
	    }
	}
      else
	{
	  let mut matched: bool = {
	      if *bp == b':' as c_char && c_isspace (*bp.offset (1) as c_int)
		{ bp = bp.offset (1); true }
	      else
		{ false }
	  };
	  if !matched
	    {
	      matched = LOOKING_AT_NOCASE! (bp, c"constant")
		|| LOOKING_AT_NOCASE! (bp, c"2constant")
		|| LOOKING_AT_NOCASE! (bp, c"fconstant")
		|| LOOKING_AT_NOCASE! (bp, c"code")
		|| LOOKING_AT_NOCASE! (bp, c"create")
		|| LOOKING_AT_NOCASE! (bp, c"defer")
		|| LOOKING_AT_NOCASE! (bp, c"value")
		|| LOOKING_AT_NOCASE! (bp, c"2value")
		|| LOOKING_AT_NOCASE! (bp, c"fvalue")
		|| LOOKING_AT_NOCASE! (bp, c"variable")
		|| LOOKING_AT_NOCASE! (bp, c"2variable")
		|| LOOKING_AT_NOCASE! (bp, c"fvariable")
		|| LOOKING_AT_NOCASE! (bp, c"buffer:")
		|| LOOKING_AT_NOCASE! (bp, c"field:")
		|| LOOKING_AT_NOCASE! (bp, c"+field")
		|| LOOKING_AT_NOCASE! (bp, c"field") /* not standard? */
		|| LOOKING_AT_NOCASE! (bp, c"begin-structure")
		|| LOOKING_AT_NOCASE! (bp, c"synonym")
		;
	    }
	  if matched && c_isspace (*bp as c_int)
	    {
	      /* Yay!  A definition! */
	      let mut name_start: *mut c_char = skip_spaces (bp);
	      let mut name_end: *mut c_char = skip_non_spaces (name_start);
	      if name_start < name_end
		{ make_tag (name_start, name_end.offset_from (name_start),
			    true, lb.buffer, name_end.offset_from (lb.buffer),
			    lineno, linecharno); }
	      bp = name_end;
	    }
	  else
	    { bp = skip_non_spaces (bp); }
	}
      }
    })
}

/*
 * Scheme tag functions
 * look for (def... xyzzy
 *          (def... (xyzzy
 *          (def ... ((...(xyzzy ....
 *          (set! xyzzy
 * Original code by Ken Haase (1985?)
 */
unsafe extern "C" fn Scheme_functions (inf: *mut FILE)
{
  let mut bp: *mut c_char;

  LOOP_ON_INPUT_LINES! (inf, lb, bp,
    {
      if strneq (bp, c"(def".as_ptr (), 4) || strneq (bp, c"(DEF".as_ptr (), 4)
	{
	  bp = skip_non_spaces (bp.offset (4));
	  /* Skip over open parens and white space.
	     Don't continue past '\0' or '='. */
	  while *bp != 0 && notinname (*bp as c_uchar) && *bp != b'=' as c_char
	    { bp = bp.offset (1); }
	  get_lispy_tag (bp);
	}
      if LOOKING_AT! (bp, c"(SET!") || LOOKING_AT! (bp, c"(set!")
	{ get_lispy_tag (bp); }
    })
}

/* Find tags in TeX and LaTeX input files.  */

/* TEX_toktab is a table of TeX control sequences that define tags.
 * Each entry records one such control sequence.
 *
 * Original code from who knows whom.
 * Ideas by:
 *   Stefan Monnier (2002)
 */

static mut TEX_toktab: *mut linebuffer = std::ptr::null_mut (); /* Table with tag tokens */

/* Default set of control sequences to put into TEX_toktab.
   The value of environment var TEXTAGS is prepended to this.  C 5807-5821's one literal with
   backslash-newline continuations is the port's one literal (PORTING.md 5.1).  */
static TEX_defenv: &CStr = c":label:ref:Ref:footref:chapter:section:subsection:subsubsection:eqno:cite:bibitem:part:appendix:entry:index:def:edef:gdef:xdef:newcommand:renewcommand:newenvironment:renewenvironment:DeclareRobustCommand:renewrobustcmd:newrobustcmd:providecommand:providerobustcmd:NewDocumentCommand:RenewDocumentCommand:ProvideDocumentCommand:DeclareDocumentCommand:NewExpandableDocumentCommand:RenewExpandableDocumentCommand:ProvideExpandableDocumentCommand:DeclareExpandableDocumentCommand:NewDocumentEnvironment:RenewDocumentEnvironment:ProvideDocumentEnvironment:DeclareDocumentEnvironment:csdef:csedef:csgdef:csxdef:csletcs:cslet:letcs:let:cs_new_protected_nopar:cs_new_protected:cs_new_nopar:cs_new_eq:cs_new:cs_set_protected_nopar:cs_set_protected:cs_set_nopar:cs_set_eq:cs_set:cs_gset_protected_nopar:cs_gset_protected:cs_gset_nopar:cs_gset_eq:cs_gset:cs_generate_from_arg_count:cs_generate_variant";

/*
 * TeX/LaTeX scanning loop.
 */
unsafe extern "C" fn TeX_commands (inf: *mut FILE)
{
  let mut cp: *mut c_char;
  let mut key: *mut linebuffer;

  let mut TEX_esc: c_char = 0;
  /* C 5839's `char TEX_opgrp UNINIT, TEX_clgrp UNINIT;` -- the port's zero initialiser is
     the UNINIT class (PORTING.md 1.5); both are assigned before any read.  */
  let mut TEX_opgrp: c_char = 0;
  let mut TEX_clgrp: c_char = 0;

  /* Initialize token table once from environment. */
  if TEX_toktab == std::ptr::null_mut ()
    { TEX_decode_env (c"TEXTAGS".as_ptr (), TEX_defenv.as_ptr ()); }

  LOOP_ON_INPUT_LINES! (inf, lb, cp,
    {
      /* Look at each TEX keyword in line. */
      'tex_next_line: loop
	{
	  /* Look for a TEX escape. */
	  loop
	    {
	      let mut c: c_char = *cp;
	      cp = cp.offset (1);
	      if c == 0 || c == b'%' as c_char
		{ break 'tex_next_line; }

	      /* Select either \ or ! as escape character, whichever comes
		 first outside a comment.  */
	      if TEX_esc == 0
		{
		  match c
		    {
		    92 /* '\\' */ =>
		      {
			TEX_esc = c;
			TEX_opgrp = b'{' as c_char;
			TEX_clgrp = b'}' as c_char;
		      }
		    33 /* '!' */ =>
		      {
			TEX_esc = c;
			TEX_opgrp = b'<' as c_char;
			TEX_clgrp = b'>' as c_char;
		      }
		    _ => {}
		    }
		}

	      if c == TEX_esc
		{ break; }
	    }

	  /* C 5970: `for (key = TEX_toktab; key->buffer != NULL; key++)`, whose `key->` is
	     `(*key).`  */
	  key = TEX_toktab;
	  while (*key).buffer != std::ptr::null_mut ()
	    {
	      if strneq (cp, (*key).buffer, (*key).len as size_t)
		{
		  let mut p: *mut c_char;
		  let mut namelen: ptrdiff_t; let mut linelen: ptrdiff_t;
		  let mut opgrp: bool = false; let mut one_esc: bool = false;
		  let mut is_explthree: bool = false;

		  cp = skip_spaces (cp.offset ((*key).len));

		  /* 1. The canonical expl3 syntax looks something like this:
		     \cs_new:Npn \__hook_tl_gput:Nn { \ERROR }.  First, if we
		     want to tag any such commands, we include only the part
		     before the colon (cs_new) in TEX_defenv or TEXTAGS.  Second,
		     etags skips the argument specifier (including the colon)
		     after the tag token, so that it doesn't become the tag name.
		     Third, we set the boolean 'is_explthree' to true so that we
		     can remove the argument specifier from the actual tag name
		     (__hook_tl_gput).  This all allows us to include expl3
		     constructs in TEX_defenv or in the environment variable
		     TEXTAGS without requiring a change of separator, and it also
		     allows us to find the definition of variant commands (with
		     different argument specifiers) defined using, for example,
		     \cs_generate_variant:Nn.  Please note that the expl3 spec
		     requires etags to pay more attention to whitespace in the
		     code.

		     2. We also automatically remove the asterisk from starred
		     variants of all commands, without the need to include the
		     starred commands explicitly in TEX_defenv or TEXTAGS. */
		  if *cp == b':' as c_char
		    {
		      while !c_isspace (*cp as c_int) && *cp != TEX_opgrp
			{ cp = cp.offset (1); }
		      cp = skip_spaces (cp);
		      is_explthree = true;
		    }
		  else if *cp == b'*' as c_char
		    { cp = cp.offset (1); }

		  /* Skip the optional arguments to commands in the tags list so
		     that these arguments don't end up as the name of the tag.
		     The name will instead come from the argument in curly braces
		     that follows the optional ones.  The '\let' command gets
		     special treatment. */
		  while *cp != 0 && *cp != b'%' as c_char
			 && !streq ((*key).buffer, c"let".as_ptr ())
		    {
		      if *cp == b'[' as c_char
			{
			  while *cp != b']' as c_char && *cp != 0
				 && *cp != b'%' as c_char
			    { cp = cp.offset (1); }
			}
		      else if *cp == b'(' as c_char
			{
			  while *cp != b')' as c_char && *cp != 0
				 && *cp != b'%' as c_char
			    { cp = cp.offset (1); }
			}
		      else if *cp == b']' as c_char || *cp == b')' as c_char
			{ cp = cp.offset (1); }
		      else
			{ break; }
		    }
		  if *cp == TEX_opgrp
		    {
		      opgrp = true;
		      cp = cp.offset (1);
		      cp = skip_spaces (cp); /* For expl3 code. */
		    }

		  /* Removing the TeX escape character from tag names simplifies
		     things for editors finding tagged commands in TeX buffers.
		     This applies to Emacs but also to the tag-finding behavior
		     of at least some of the editors that use ctags, though in
		     the latter case this will remain suboptimal.  The
		     undocumented ctags option '--no-duplicates' may help. */
		  if *cp == TEX_esc
		    {
		      cp = cp.offset (1);
		      one_esc = true;
		    }

		  /* Testing !c_isspace && !c_ispunct is simpler, but halts
		     processing at too many places.  The list as it stands tries
		     both to ensure that tag names will derive from macro names
		     rather than from optional parameters to those macros, and
		     also to return findable names while still allowing for
		     unorthodox constructs. */
		  p = cp;
		  while !c_isspace (*p as c_int) && *p != b'#' as c_char && *p != b'=' as c_char
			&& *p != b'[' as c_char && *p != b'(' as c_char && *p != TEX_opgrp
			&& *p != TEX_clgrp && *p != b'"' as c_char && *p != b'\'' as c_char
			&& *p != b'%' as c_char && *p != b',' as c_char
			&& *p != b'|' as c_char && *p != b'$' as c_char
		    /* In expl3 code we remove the argument specification from
		       the tag name.  More generally we allow only one (deleted)
		       escape char in a tag name, which (primarily) enables
		       tagging a TeX command's different, possibly temporary,
		       '\let' bindings. */
		    {
		      if is_explthree && *p == b':' as c_char
			{ break; }
		      else if *p == TEX_esc
			{ /* Second part of test is for, e.g., \cslet. */
			  if !one_esc && !opgrp
			    {
			      one_esc = true;
			      p = p.offset (1);
			      continue;
			    }
			  else
			    { break; }
			}
		      else
			{
			  p = p.offset (1);
			  continue;
			}
		    }
		  /* For TeX files, tags without a name are basically cruft, and
		     in some situations they can produce spurious and confusing
		     matches.  Try to catch as many cases as possible where a
		     command name is of the form '\(', but avoid, as far as
		     possible, the spurious matches. */
		  if p == cp
		    {
		      match *p
			{ /* Include =? */
			40 /* '(' */ | 91 /* '[' */ | 34 /* '"' */ | 39 /* '\'' */
			| 92 /* '\\' */ | 33 /* '!' */ | 61 /* '=' */ | 44 /* ',' */
			| 124 /* '|' */ | 36 /* '$' */ =>
			  {
			    p = p.offset (1);
			  }
			123 /* '{' */ | 125 /* '}' */ | 60 /* '<' */ | 62 /* '>' */ =>
			  {
			    if !opgrp
			      {
				p = p.offset (1);
				if *p == 0 || *p == b'%' as c_char
				  { break 'tex_next_line; }
			      }
			  }
			_ => {}
			}
		    }
		  namelen = p.offset_from (cp);
		  linelen = lb.len;
		  if !opgrp || *p == TEX_clgrp
		    {
		      while *p != 0 && *p != TEX_opgrp && *p != TEX_clgrp
			{ p = p.offset (1); }
		      linelen = p.offset_from (lb.buffer) + 1;
		    }
		  if namelen != 0
		    { make_tag (cp, namelen, true,
				lb.buffer, linelen, lineno, linecharno); }
		  /* Lines with more than one \def or \let are surprisingly
		     common in TeX files, especially in the system files that
		     form the basis of the various TeX formats.  This tags them
		     all. */
		  /* goto tex_next_line; /\* We only tag a line once *\/ */
		  while *cp != 0 && *cp != b'%' as c_char && *cp != TEX_esc
		    { cp = cp.offset (1); }
		  if *cp != TEX_esc
		    { break 'tex_next_line; }
		}
	      key = key.offset (1);
	    }
	}
    })
}

/* Read environment and prepend it to the default string.
   Build token table. */
unsafe fn TEX_decode_env (evarname: *const c_char, defenv: *const c_char)
{
  let mut env: *const c_char; let mut p: *const c_char;
  let mut len: ptrdiff_t = 1;

  /* Append default string to environment. */
  env = getenv (evarname);
  if env == std::ptr::null ()
    { env = defenv; }
  else
    { env = concat (env, defenv, c"".as_ptr ()); }

  /* If the environment variable doesn't start with a colon, increase
     the length of the token table.  */
  if *env != b':' as c_char
    { len += 1; }

  /* Allocate a token table */
  p = env;
  loop
    {
      p = strchr (p, b':' as c_int);
      if p == std::ptr::null ()
	{ break; }
      p = p.offset (1);
      if *p != 0
	{ len += 1; }
    }
  TEX_toktab = xnmalloc (len, __sizeof_deref (TEX_toktab)) as *mut linebuffer;

  /* Unpack environment string into token table. Be careful about */
  /* zero-length strings (leading ':', "::" and trailing ':') */
  let mut i: ptrdiff_t = 0;
  while *env != 0
    {
      p = strchr (env, b':' as c_int);
      if p == std::ptr::null ()		/* End of environment string. */
	{ p = env.offset (strlen (env) as isize); }
      if p.offset_from (env) > 0
	{			/* Only non-zero strings. */
	  (*TEX_toktab.offset (i)).buffer = savenstr (env, p.offset_from (env));
	  (*TEX_toktab.offset (i)).len = p.offset_from (env);
	  i += 1;
	}
      if *p != 0
	{ env = p.offset (1); }
      else
	{
	  (*TEX_toktab.offset (i)).buffer = std::ptr::null_mut (); /* Mark end of table. */
	  (*TEX_toktab.offset (i)).len = 0;
	  break;
	}
    }
}


/* Texinfo support.  Dave Love, Mar. 2000.  */
unsafe extern "C" fn Texinfo_nodes (inf: *mut FILE)
{
  let mut cp: *mut c_char; let mut start: *mut c_char;
  LOOP_ON_INPUT_LINES! (inf, lb, cp,
    {
      if LOOKING_AT! (cp, c"@node")
	{
	  start = cp;
	  while *cp != 0 && *cp != b',' as c_char
	    { cp = cp.offset (1); }
	  make_tag (start, cp.offset_from (start), true,
		    lb.buffer, cp.offset_from (lb.buffer) + 1, lineno, linecharno);
	}
    })
}

/*
 * HTML support.
 * Contents of <title>, <h1>, <h2>, <h3> are tags.
 * Contents of <a name=xxx> are tags with name xxx.
 *
 * Francesco Potorti, 2002.
 */
unsafe extern "C" fn HTML_labels (inf: *mut FILE)
{
  let mut getnext: bool = false;		/* next text outside of HTML tags is a tag */
  let mut skiptag: bool = false;		/* skip to the end of the current HTML tag */
  let mut intag: bool = false;		/* inside an html tag, looking for ID= */
  let mut inanchor: bool = false;	/* when INTAG, is an anchor, look for NAME= */
  let mut end: *mut c_char;


  linebuffer_setlen (&raw mut token_name, 0); /* no name in buffer */

  LOOP_ON_INPUT_LINES! (inf, lb, dbp,
    { loop			/* loop on the same line */
      {
	if skiptag		/* skip HTML tag */
	  {
	    while *dbp != 0 && *dbp != b'>' as c_char
	      { dbp = dbp.offset (1); }
	    if *dbp == b'>' as c_char
	      {
		dbp = dbp.offset (1);
		skiptag = false;
		continue;	/* look on the same line */
	      }
	    break;		/* go to next line */
	  }

	else if intag	/* look for "name=" or "id=" */
	  {
	    while *dbp != 0 && *dbp != b'>' as c_char
		   && c_tolower (*dbp as c_int) != b'n' as c_int
		   && c_tolower (*dbp as c_int) != b'i' as c_int
	      { dbp = dbp.offset (1); }
	    if *dbp == 0
	      { break; }		/* go to next line */
	    if *dbp == b'>' as c_char
	      {
		dbp = dbp.offset (1);
		intag = false;
		continue;	/* look on the same line */
	      }
	    if (inanchor && LOOKING_AT_NOCASE! (dbp, c"name="))
		|| LOOKING_AT_NOCASE! (dbp, c"id=")
	      {
		let quoted: bool = *dbp == b'"' as c_char;

		if quoted
		  {
		    dbp = dbp.offset (1);
		    end = dbp;
		    while *end != 0 && *end != b'"' as c_char
		      { end = end.offset (1); }
		  }
		else
		  {
		    end = dbp;
		    while *end != 0 && intoken (*end as c_uchar)
		      { end = end.offset (1); }
		  }
		linebuffer_setlen (&raw mut token_name, end.offset_from (dbp));
		memcpyz (token_name.buffer as *mut c_void,
			 dbp as *const c_void, end.offset_from (dbp));

		dbp = end;
		intag = false;	/* we found what we looked for */
		skiptag = true; /* skip to the end of the tag */
		getnext = true;	/* then grab the text */
		continue;	/* look on the same line */
	      }
	    dbp = dbp.offset (1);
	  }

	else if getnext	/* grab next tokens and tag them */
	  {
	    dbp = skip_spaces (dbp);
	    if *dbp == 0
	      { break; }		/* go to next line */
	    if *dbp == b'<' as c_char
	      {
		intag = true;
		inanchor = c_tolower (*dbp.offset (1) as c_int) == b'a' as c_int
			   && !intoken (*dbp.offset (2) as c_uchar);
		continue;	/* look on the same line */
	      }

	    end = dbp.offset (1);
	    while *end != 0 && *end != b'<' as c_char
	      { end = end.offset (1); }
	    make_tag (token_name.buffer, token_name.len, true,
		      dbp, end.offset_from (dbp), lineno, linecharno);
	    linebuffer_setlen (&raw mut token_name, 0);	/* no name in buffer */
	    getnext = false;
	    break;		/* go to next line */
	  }

	else			/* look for an interesting HTML tag */
	  {
	    while *dbp != 0 && *dbp != b'<' as c_char
	      { dbp = dbp.offset (1); }
	    if *dbp == 0
	      { break; }		/* go to next line */
	    intag = true;
	    if c_tolower (*dbp.offset (1) as c_int) == b'a' as c_int
		&& !intoken (*dbp.offset (2) as c_uchar)
	      {
		inanchor = true;
		continue;	/* look on the same line */
	      }
	    else if LOOKING_AT_NOCASE! (dbp, c"<title>")
		     || LOOKING_AT_NOCASE! (dbp, c"<h1>")
		     || LOOKING_AT_NOCASE! (dbp, c"<h2>")
		     || LOOKING_AT_NOCASE! (dbp, c"<h3>")
	      {
		intag = false;
		getnext = true;
		continue;	/* look on the same line */
	      }
	    dbp = dbp.offset (1);
	  }
      }
    })
}

/*
 * Prolog support
 *
 * Assumes that the predicate or rule starts at column 0.
 * Only the first clause of a predicate or rule is added.
 * Original code by Sunichirou Sugou (1989)
 * Rewritten by Anders Lindgren (1996)
 */
unsafe extern "C" fn Prolog_functions (inf: *mut FILE)
{
  let mut cp: *mut c_char; let mut last: *mut c_char = std::ptr::null_mut ();
  let mut lastlen: ptrdiff_t = 0; let mut allocated: ptrdiff_t = 0;

  LOOP_ON_INPUT_LINES! (inf, lb, cp,
    {
      if *cp == 0	/* Empty line */
	{ continue; }
      else if c_isspace (*cp as c_int) /* Not a predicate */
	{ continue; }
      else if *cp == b'/' as c_char && *cp.offset (1) == b'*' as c_char /* comment. */
	{ prolog_skip_comment (&raw mut lb, inf); }
      else
	{
	  let mut len: ptrdiff_t = prolog_pr (cp, last, lastlen);
	  if 0 < len
	    {
	      /* Store the predicate name to avoid generating duplicate
		 tags later.  */
	      if allocated <= len
		{
		  xrnew! (last, len + 1, 1);
		  allocated = len + 1;
		}
	      memcpyz (last as *mut c_void, cp as *const c_void, len);
	      lastlen = len;
	    }
	}
    });
  free (last as *mut c_void);
}


unsafe fn prolog_skip_comment (plb: *mut linebuffer, inf: *mut FILE)
{
  let mut cp: *mut c_char;

  loop
    {
      cp = (*plb).buffer;
      while *cp != 0
	{
	  if *cp == b'*' as c_char && *cp.offset (1) == b'/' as c_char
	    { return; }
	  cp = cp.offset (1);
	}
      readline (plb, inf);
      if !perhaps_more_input (inf)
	{ break; }
    }
}

/*
 * A predicate or rule definition is added if it matches:
 *     <beginning of line><Prolog Atom><whitespace>(
 * or  <beginning of line><Prolog Atom><whitespace>:-
 *
 * It is added to the tags database if it doesn't match the
 * name of the previous clause header.
 *
 * Return the size of the name of the predicate or rule, or 0 if no
 * header was found.
 */
unsafe fn prolog_pr (s: *mut c_char, last: *mut c_char, lastlen: ptrdiff_t) -> ptrdiff_t
{
  let mut len: ptrdiff_t = prolog_atom (s, 0) as ptrdiff_t;
  if len == 0
    { return 0; }
  let mut pos: ptrdiff_t = skip_spaces (s.offset (len)).offset_from (s);

  /* Save only the first clause.  */
  let mut matched: bool = false;
  if *s.offset (pos) == b'.' as c_char
    { matched = true; }
  else if *s.offset (pos) == b'(' as c_char
    { pos += 1; matched = true; }
  else if *s.offset (pos) == b':' as c_char
	   && *s.offset (pos + 1) == b'-' as c_char
    { pos += 2; matched = true; }
  if matched
      && !(lastlen == len
	   && memcmp (s as *const c_void, last as *const c_void, len as usize) == 0)
    {
      make_tag (s, len, true, s, pos, lineno, linecharno);
      return len;
    }

  return 0;
}

/*
 * Consume a Prolog atom.
 * Return the number of bytes consumed, or 0 if there was an error.
 *
 * A prolog atom, in this context, could be one of:
 * - An alphanumeric sequence, starting with a lower case letter.
 * - A quoted arbitrary string. Single quotes can escape themselves.
 *   Backslash quotes everything.
 */
unsafe fn prolog_atom (s: *mut c_char, mut pos: size_t) -> size_t
{
  let origpos: size_t;

  origpos = pos;

  if c_islower (*s.offset (pos as isize) as c_int) || *s.offset (pos as isize) == b'_' as c_char
    {
      /* The atom is unquoted. */
      pos += 1;
      while c_isalnum (*s.offset (pos as isize) as c_int)
	    || *s.offset (pos as isize) == b'_' as c_char
	{
	  pos += 1;
	}
      return pos - origpos;
    }
  else if *s.offset (pos as isize) == b'\'' as c_char
    {
      pos += 1;

      loop
	{
	  if *s.offset (pos as isize) == b'\'' as c_char
	    {
	      pos += 1;
	      if *s.offset (pos as isize) != b'\'' as c_char
		{ break; }
	      pos += 1;		/* A double quote  */
	    }
	  else if *s.offset (pos as isize) == 0
	    /* Multiline quoted atoms are ignored.  */
	    { return 0; }
	  else if *s.offset (pos as isize) == b'\\' as c_char
	    {
	      if *s.offset ((pos + 1) as isize) == 0
		{ return 0; }
	      pos += 2;
	    }
	  else
	    { pos += 1; }
	}
      return pos - origpos;
    }
  else
    { return 0; }
}

/*
 * Support for Mercury
 *
 * Assumes that the declarations start at column 0.
 * Original code by Sunichirou Sugou (1989) for Prolog.
 * Rewritten by Anders Lindgren (1996) for Prolog.
 * Adapted by Fabrice Nicol (2021) for Mercury.
 * Note: Prolog-support behavior is preserved if
 * --declarations is used, corresponding to
 * with_mercury_definitions=true.
 */

static mut is_mercury_type: bool = false;
static mut is_mercury_quantifier: bool = false;
static mut is_mercury_declaration: bool = false;
/* C 6396-6401: `typedef struct { size_t pos; size_t namelength; size_t totlength; }
   mercury_pos_t;`  */
#[repr(C)]
pub struct mercury_pos_t
{
  pub pos: size_t,          /* Position reached in parsing tag name.  */
  pub namelength: size_t,   /* Length of tag name  */
  pub totlength: size_t     /* Total length of parsed tag: this field is currently
			  reserved for control and debugging.   */
}

/*
 * Objective-C and Mercury have identical file extension .m.
 * To disambiguate between Objective C and Mercury, parse file
 * with the following heuristics hook:
 *   - if line starts with :-, choose Mercury unconditionally;
 *   - if line starts with #, @, choose Objective-C;
 *   - otherwise compute the following ratio:
 *
 *     r = (number of lines with :-
 *          or % in non-commented parts or . at trimmed EOL)
 *         / (number of lines - number of lines starting by any amount
 *                        of whitespace, optionally followed by comment(s))
 *
 * Note: strings are neglected in counts.
 *
 * If r > mercury_heuristics_ratio, choose Mercury.
 * Experimental tests show that a possibly optimal default value for
 * this floor value is around 0.5.  This is the default value for
 * MERCURY_HEURISTICS_RATIO, defined in the first lines of this file.
 * The closer r is to 0.5, the closer the source code to pure Prolog.
 * Idiomatic Mercury is scored either with r = 1.0 or higher.
 * Objective-C is scored with r = 0.0.  When this fails, the r-score
 * never rose above 0.1 in Objective-C tests.
 */

/* `<stdio.h>`: `int fgetc (FILE *stream);` -- the stdio mirror does not carry it.  */
unsafe extern "C" {
  pub fn fgetc (stream: *mut FILE) -> c_int;
}

unsafe fn test_objc_is_mercury (this_file: *mut c_char, lang: *mut *mut language)
{
  if this_file == std::ptr::null_mut () { return; }
  let mut fp: *mut FILE = fopen (this_file, c"r".as_ptr ());
  if fp == std::ptr::null_mut ()
    { pfatal (this_file); }

  let mut blank_line: bool = false; /* Line starting with any amount of white space
			      followed by optional comment(s).  */
  let mut commented_line: bool = false;
  let mut found_dot: bool = false;
  let mut only_space_before: bool = true;
  let mut start_of_line: bool = true;
  let mut c: c_int;
  let mut lines: intmax_t = 1;
  let mut mercury_dots: intmax_t = 0;
  let mut percentage_signs: intmax_t = 0;
  let mut rule_signs: intmax_t = 0;
  let mut ratio: f32 = 0.0;

  'out: {
  loop
    {
      c = fgetc (fp);
      if !(c != EOF)
	{ break; }
      match c
	{
	10 /* '\n' */ =>
	  {
	    if !blank_line
	      { lines += 1; }
	    blank_line = true;
	    commented_line = false;
	    start_of_line = true;
	    if found_dot
	      { mercury_dots += 1; }
	    found_dot = false;
	    only_space_before = true;
	  }
	46 /* '.' */ =>
	  {
	    found_dot = !commented_line;
	    only_space_before = false;
	  }
	37 /* '%' */ => /* More frequent in Mercury.  May be modulo in Obj.-C.  */
	  {
	    if !commented_line
	      {
		percentage_signs += 1;
		/* Cannot tell if it is a comment or modulo yet for sure.
                   Yet works for heuristic purposes.  */
		commented_line = true;
	      }
	    found_dot = false;
	    start_of_line = false;
	    only_space_before = false;
	  }
	47 /* '/' */ =>
	  {
	    let mut d: c_int = fgetc (fp);
	    found_dot = false;
	    only_space_before = false;
	    if !commented_line
	      {
		if d == b'*' as c_int
		  { commented_line = true; }
		else
		  /* If d == '/', cannot tell if it is an Obj.-C comment:
		     may be Mercury integ. division.  */
		  { blank_line = false; }
	      }
	    /* FALLTHROUGH into the ' '/'\t' arm (PORTING.md 2.2).  */
	    start_of_line = false;
	  }
        32 /* ' ' */ | 9 /* '\t' */ =>
	  {
	    start_of_line = false;
	  }
        58 /* ':' */ =>
	  {
	    c = fgetc (fp);
	    if start_of_line
	      {
		if c == b'-' as c_int
		  {
		    ratio = 1.0; /* Failsafe, not an operator in Obj.-C.  */
		    break 'out;
		  }
		start_of_line = false;
	      }
	    else
	      {
		/* p :- q.  Frequent in Mercury.
		   Rare or in quoted exprs in Obj.-C.  */
		if c == b'-' as c_int && !commented_line
		  { rule_signs += 1; }
	      }
	    blank_line = false;
	    found_dot = false;
	    only_space_before = false;
	  }
	64 /* '@' */ | 35 /* '#' */ =>
	  {
	    if start_of_line || only_space_before
	      {
		ratio = 0.0;
		break 'out;
	      }
	    /* FALLTHROUGH into `default:` (PORTING.md 2.2).  */
	    start_of_line = false;
	    blank_line = false;
	    found_dot = false;
	    only_space_before = false;
	  }
	_ =>
	  {
	    start_of_line = false;
	    blank_line = false;
	    found_dot = false;
	    only_space_before = false;
	  }
	}
    }

  /* Fallback heuristic test.  Not failsafe but errless in practice.  */
  ratio = ((rule_signs as f32) + (percentage_signs as f32) + (mercury_dots as f32))
	  / (lines as f32);
  }

  /* C's `out:` label.  */
  if fclose (fp) == EOF
    { pfatal (this_file); }

  if ratio > mercury_heuristics_ratio
    {
      /* Change the language from Objective-C to Mercury.  */
      static mut lang0: language =
	language { name: c"mercury".as_ptr (), help: Mercury_help.as_ptr (),
		   function: Some (Mercury_functions),
		   suffixes: &raw const Mercury_suffixes as *const *const c_char,
		   filenames: std::ptr::null (), interpreters: std::ptr::null (),
		   metasource: false };
      *lang = &raw mut lang0;
    }
}

unsafe extern "C" fn Mercury_functions (inf: *mut FILE)
{
  let mut cp: *mut c_char; let mut last: *mut c_char = std::ptr::null_mut ();
  let mut lastlen: ptrdiff_t = 0; let mut allocated: ptrdiff_t = 0;
  if declarations != 0
    { with_mercury_definitions = true; }

  LOOP_ON_INPUT_LINES! (inf, lb, cp,
    {
      if *cp == 0   /* Empty line.  */
	{ continue; }
      else if c_isspace (*cp as c_int) || *cp == b'%' as c_char
	/*  A Prolog-type comment or anything other than a declaration.  */
	{ continue; }
      else if *cp == b'/' as c_char && *cp.offset (1) == b'*' as c_char /* Mercury C-type comment.  */
        { mercury_skip_comment (&raw mut lb, inf); }
      else
	{
	  is_mercury_declaration = *cp == b':' as c_char && *cp.offset (1) == b'-' as c_char;

          if is_mercury_declaration
	      || with_mercury_definitions
	    {
	      let mut len: ptrdiff_t = mercury_pr (cp, last, lastlen);
	      if 0 < len
		{
		  /* Store the declaration to avoid generating duplicate
		     tags later.  */
		  if allocated <= len
		    {
		      xrnew! (last, len + 1, 1);
		      allocated = len + 1;
		    }
		  memcpyz (last as *mut c_void, cp as *const c_void, len);
		  lastlen = len;
		}
	    }
	}
    });
  free (last as *mut c_void);
}

unsafe fn mercury_skip_comment (plb: *mut linebuffer, inf: *mut FILE)
{
  let mut cp: *mut c_char;

  loop
    {
      cp = (*plb).buffer;
      while *cp != 0
	{
	  if *cp == b'*' as c_char && *cp.offset (1) == b'/' as c_char
	    { return; }
	  cp = cp.offset (1);
	}
      readline (plb, inf);
      if !perhaps_more_input (inf)
	{ break; }
    }
}

/*
 * A declaration is added if it matches:
 *     <beginning of line>:-<whitespace><Mercury Term><whitespace>(
 * If with_mercury_definitions == true, we also add:
 *     <beginning of line><Mercury item><whitespace>(
 * or  <beginning of line><Mercury item><whitespace>:-
 * As for Prolog support, different arities and types are not taken into
 * consideration.
 * Item is added to the tags database if it doesn't match the
 * name of the previous declaration.
 *
 * Consume a Mercury declaration.
 * Return the number of bytes consumed, or 0 if there was an error.
 *
 * A Mercury declaration must be one of:
 *  :- type
 *  :- solver type
 *  :- pred
 *  :- func
 *  :- inst
 *  :- mode
 *  :- typeclass
 *  :- instance
 *  :- pragma
 *  :- promise
 *  :- initialise
 *  :- finalise
 *  :- mutable
 *  :- module
 *  :- interface
 *  :- implementation
 *  :- import_module
 *  :- use_module
 *  :- include_module
 *  :- end_module
 * followed on the same line by an alphanumeric sequence, starting with a lower
 * case letter or by a single-quoted arbitrary string.
 * Single quotes can escape themselves.  Backslash quotes everything.
 *
 * Return the size of the name of the declaration or 0 if no header was found.
 * As quantifiers may precede functions or predicates, we must list them too.
 */

static mut Mercury_decl_tags: [*const c_char; 22] =
  [c"type".as_ptr (), c"solver type".as_ptr (), c"pred".as_ptr (),
   c"func".as_ptr (), c"inst".as_ptr (), c"mode".as_ptr (),
   c"typeclass".as_ptr (), c"instance".as_ptr (), c"pragma".as_ptr (),
   c"promise".as_ptr (), c"initialise".as_ptr (), c"finalise".as_ptr (),
   c"mutable".as_ptr (), c"module".as_ptr (), c"interface".as_ptr (),
   c"implementation".as_ptr (), c"import_module".as_ptr (), c"use_module".as_ptr (),
   c"include_module".as_ptr (), c"end_module".as_ptr (), c"some".as_ptr (),
   c"all".as_ptr ()];

/* Return true if array of char BUF, of length LEN, equals STR.  */

unsafe fn memstreq (buf: *const c_char, len: ptrdiff_t, str: *const c_char) -> bool
{
  return strlen (str) == len as size_t && memeq (buf as *const c_void, str as *const c_void,
						 len as usize);
}

unsafe fn mercury_decl (s: *mut c_char, mut pos: size_t) -> mercury_pos_t
{
  let mut null_pos: mercury_pos_t =
    mercury_pos_t { pos: 0, namelength: 0, totlength: 0 };

  if s == std::ptr::null_mut () { return null_pos; }

  let origpos: size_t = pos;
  let mut decl_type: *mut c_char = s.offset (origpos as isize);

  while c_isalnum (*s.offset (pos as isize) as c_int) || *s.offset (pos as isize) == b'_' as c_char
    { pos += 1; }

  let mut decl_type_length: ptrdiff_t = pos as ptrdiff_t - origpos as ptrdiff_t;

  /* Mercury declaration tags.  Consume them, then check the declaration item
     following :- is legitimate, then go on as in the prolog case.  */

  let mut found_decl_tag: bool = false;

  if is_mercury_quantifier
    {
      if !(memstreq (decl_type, decl_type_length, c"pred".as_ptr ())
	   || memstreq (decl_type, decl_type_length, c"func".as_ptr ()))
	{ return null_pos; } /* Bad syntax.  */

      is_mercury_quantifier = false; /* Reset to base value.  */
      found_decl_tag = true;
    }
  else
    {
      let mut j: c_int = 0;
      while j < countof! (Mercury_decl_tags) as c_int
	{
	  if memstreq (decl_type, decl_type_length,
		       Mercury_decl_tags[j as usize])
	    {
	      found_decl_tag = true;
	      if memstreq (decl_type, decl_type_length, c"type".as_ptr ())
		{ is_mercury_type = true; }

	      if memstreq (decl_type, decl_type_length, c"some".as_ptr ())
		  || memstreq (decl_type, decl_type_length, c"all".as_ptr ())
		{
		  is_mercury_quantifier = true;
		}

	      break;  /* Found declaration tag of rank j.  */
	    }
	  else
	    /* 'solver type' has a blank in the middle,
	       so this is the hard case.  */
	    {
	      if memstreq (decl_type, decl_type_length, c"solver".as_ptr ())
		{
		  loop
		    {
		      pos += 1;
		      if !(c_isalnum (*s.offset (pos as isize) as c_int)
			   || *s.offset (pos as isize) == b'_' as c_char)
			{ break; }
		    }

		  decl_type_length = pos as ptrdiff_t - origpos as ptrdiff_t;

		  if memstreq (decl_type, decl_type_length, c"solver type".as_ptr ())
		    {
		      found_decl_tag = false;
		      break;  /* Found declaration tag of rank j.  */
		    }
		}
	    }
	  j += 1;
	}
    }

  /* If with_mercury_definitions == false
   * this is a Mercury syntax error, ignoring... */

  if with_mercury_definitions
    {
      if found_decl_tag
	{ pos = skip_spaces (s.offset (pos as isize)).offset_from (s) as size_t; } /* Skip len blanks again.  */
      else
	/* Prolog-like behavior
	 * we have parsed the predicate once, yet inappropriately
	 * so restarting again the parsing step.  */
	{ pos = 0; }
    }
  else
    {
      if found_decl_tag
	{ pos = skip_spaces (s.offset (pos as isize)).offset_from (s) as size_t; } /* Skip len blanks again.  */
      else
	{ return null_pos; }
    }

  /* From now on it is the same as for Prolog except for module dots.  */

  let mut start_of_name: size_t = pos;

  if c_islower (*s.offset (pos as isize) as c_int)
      || *s.offset (pos as isize) == b'_' as c_char
    {
      /* The name is unquoted.
         Do not confuse module dots with end-of-declaration dots.  */
      let mut module_dot_pos: c_int = 0;

      loop
	{
	  if c_isalnum (*s.offset (pos as isize) as c_int)
	      || *s.offset (pos as isize) == b'_' as c_char
	    { pos += 1; }
	  else if *s.offset (pos as isize) == b'.' as c_char /* A module dot.  */
	      && (c_isalnum (*s.offset ((pos + 1) as isize) as c_int)
		  || *s.offset ((pos + 1) as isize) == b'_' as c_char)
	      && { module_dot_pos = pos as c_int;	/* Record module dot position.
				                 Erase module from name.  */
		   module_dot_pos != 0 }
	    { pos += 1; }
	  else
	    { break; }
	}

      if module_dot_pos != 0
	{
	  start_of_name = (module_dot_pos as size_t) + 2;
	  pos += 1;
        }

      let position: mercury_pos_t =
	mercury_pos_t { pos: pos, namelength: pos - start_of_name + 1,
			totlength: pos - origpos };
      return position;
    }
  else if *s.offset (pos as isize) == b'\'' as c_char
    {
      pos += 1;
      loop
	{
	  if *s.offset (pos as isize) == b'\'' as c_char
	    {
	      pos += 1;
	      if *s.offset (pos as isize) != b'\'' as c_char
		{ break; }
	      pos += 1; /* A double quote.  */
	    }
	  else if *s.offset (pos as isize) == 0  /* Multiline quoted atoms are ignored.  */
	    { return null_pos; }
	  else if *s.offset (pos as isize) == b'\\' as c_char
	    {
	      if *s.offset ((pos + 1) as isize) == 0
		{ return null_pos; }
	      pos += 2;
	    }
	  else
	    { pos += 1; }
	}

      let position: mercury_pos_t =
	mercury_pos_t { pos: pos, namelength: pos - start_of_name + 1,
			totlength: pos - origpos };
      return position;
    }
  else if is_mercury_quantifier && *s.offset (pos as isize) == b'[' as c_char   /* :- some [T] pred/func.  */
    {
      let mut close_bracket: *mut c_char =
	strchr (s.offset ((pos + 1) as isize), b']' as c_int);
      if close_bracket == std::ptr::null_mut ()
	{ return null_pos; }
      pos = skip_spaces (close_bracket.offset (1)).offset_from (s) as size_t;
      let mut position: mercury_pos_t = mercury_decl (s, pos);
      position.totlength += pos - origpos;
      return position;
    }
  else if *s.offset (pos as isize) == b'.' as c_char  /* as in ':- interface.'  */
    {
      let position: mercury_pos_t =
	mercury_pos_t { pos: pos, namelength: pos - origpos + 1, totlength: pos - origpos };
      return position;
    }
  else
    { return null_pos; }
}

unsafe fn mercury_pr (s: *mut c_char, last: *mut c_char, lastlen: ptrdiff_t) -> ptrdiff_t
{
  let mut len0: size_t = 0;
  is_mercury_type = false;
  is_mercury_quantifier = false;
  let mut stop_at_rule: bool = false;

  if is_mercury_declaration
    {
      /* Skip len0 blanks only for declarations.  */
      len0 = skip_spaces (s.offset (2)).offset_from (s) as size_t;
    }

  let mut position: mercury_pos_t = mercury_decl (s, len0);
  let mut pos: size_t = position.pos;
  let mut offset: c_int = 0;  /* may be < 0  */
  if pos == 0 { return 0; }

  /* Skip white space for:
     a. rules in definitions before :-
     b. 0-arity predicates with inlined modes.
     c. possibly multiline type definitions  */

  while c_isspace (*s.offset (pos as isize) as c_int) { pos += 1; offset += 1; }

  let mut case_1: bool = false;
  if *s.offset (pos as isize) == b'.' as c_char
      && { pos += 1; true }     /* case 1
                                   This is a statement dot,
                                   not a module dot. */
    { case_1 = true; }
  else if c_isalnum (*s.offset (pos as isize) as c_int)              /* 0-arity procedures  */
    { case_1 = true; }
  else if *s.offset (pos as isize) == b'(' as c_char
      && { pos += 1; true }  /* case 2: arity > 0   */
    { case_1 = true; }
  else if *s.offset (pos as isize) == b':' as c_char               /* case 3: rules  */
      && *s.offset ((pos + 1) as isize) == b'-' as c_char
      && { stop_at_rule = true; true }
    { case_1 = true; }
  if (case_1
      && ((lastlen as size_t) != pos
	  || memcmp (s as *const c_void, last as *const c_void, pos) != 0))
      /* Types are often declared on several lines so keeping just
	 the first line.  */

      || is_mercury_type  /* When types are implemented.  */
    {
      let mut namelength: size_t = position.namelength;
      if stop_at_rule && offset != 0 { offset -= 1; }

      /* Left-trim type definitions.  */

      while pos > namelength + offset as size_t
	    && c_isspace (*s.offset ((pos - namelength - offset as size_t) as isize) as c_int)
	{ offset -= 1; }

      make_tag (s.offset ((pos - namelength - offset as size_t) as isize),
		namelength as ptrdiff_t - 1, true,
		s, (pos - offset as size_t - 1) as ptrdiff_t, lineno, linecharno);
      return pos as ptrdiff_t;
    }

  return 0;
}

/*
 * Support for Erlang
 *
 * Generates tags for functions, defines, and records.
 * Assumes that Erlang functions start at column 0.
 * Original code by Anders Lindgren (1996)
 */
unsafe extern "C" fn Erlang_functions (inf: *mut FILE)
{
  let mut cp: *mut c_char; let mut last: *mut c_char = std::ptr::null_mut ();
  let mut lastlen: ptrdiff_t = 0; let mut allocated: ptrdiff_t = 0;

  LOOP_ON_INPUT_LINES! (inf, lb, cp,
    {
      if *cp == 0	/* Empty line */
	{ continue; }
      else if c_isspace (*cp as c_int) /* Not function nor attribute */
	{ continue; }
      else if *cp == b'%' as c_char	/* comment */
	{ continue; }
      else if *cp == b'"' as c_char	/* Sometimes, strings start in column one */
	{ continue; }
      else if *cp == b'-' as c_char 	/* attribute, e.g. "-define" */
	{
	  erlang_attribute (cp);
	  if last != std::ptr::null_mut ()
	    {
	      free (last as *mut c_void);
	      last = std::ptr::null_mut ();
	      allocated = 0; lastlen = 0;
	    }
	}
      else
	{
	  let mut name_offset: ptrdiff_t = 0;
	  let mut len: ptrdiff_t = erlang_func (cp, last, lastlen, &raw mut name_offset);
	  if 0 < len
	    {
	      /* Store the function name to avoid generating duplicate
		 tags later.  */
	      if allocated <= len
		{
		  xrnew! (last, len + 1, 1);
		  allocated = len + 1;
		}
	      memcpyz (last as *mut c_void, cp.offset (name_offset) as *const c_void, len);
	      lastlen = len;
	    }
	}
    });
  free (last as *mut c_void);
}


/*
 * A function definition is added if it matches:
 *     <beginning of line><Erlang Atom><whitespace>(
 *
 * It is added to the tags database if it doesn't match the
 * name of the previous clause header.
 *
 * Return the size of the name of the function, or 0 if no function
 * was found.
 */
unsafe fn erlang_func (s: *mut c_char, last: *mut c_char, lastlen: ptrdiff_t,
		       name_offset: *mut ptrdiff_t) -> ptrdiff_t
{
  let mut name: *mut c_char = s;
  let mut len: ptrdiff_t = erlang_atom (s);
  if len == 0
    { return 0; }
  let mut pos: ptrdiff_t = skip_spaces (s.offset (len)).offset_from (s);

  /* If the name is quoted, the quotes are not part of the name. */
  let quoted: bool = 2 < len && *name == b'\'' as c_char
    && *name.offset (len - 1) == b'\'' as c_char;
  name = name.offset (quoted as isize);
  len -= 2 * (quoted as ptrdiff_t);

  /* Save only the first clause. */
  let ch: c_char = *s.offset (pos);
  pos += 1;
  if ch == b'(' as c_char
      && !(lastlen == len && memcmp (name as *const c_void, last as *const c_void,
				     len as usize) == 0)
    {
      make_tag (s, len, true, s, pos, lineno, linecharno);
      *name_offset = quoted as ptrdiff_t;
      return len;
    }

  return 0;
}


/*
 * Handle attributes.  Currently, tags are generated for defines
 * and records.
 *
 * They are on the form:
 * -define(foo, bar).
 * -define(Foo(M, N), M+N).
 * -record(graph, {vtab = notable, cyclic = true}).
 */
unsafe fn erlang_attribute (s: *mut c_char)
{
  let mut cp: *mut c_char = s;

  let ch: c_char = *cp;
  if (LOOKING_AT! (cp, c"-define") || LOOKING_AT! (cp, c"-record"))
      && { let c: c_char = *cp; cp = cp.offset (1); c == b'(' as c_char }
    {
      cp = skip_spaces (cp);
      let mut len: ptrdiff_t = erlang_atom (cp);
      let mut pos: ptrdiff_t = cp.offset (len).offset_from (s);
      if len > 0
	{
	  /* If the name is quoted, the quotes are not part of the name. */
	  if len > 2 && *cp == b'\'' as c_char && *cp.offset (len - 1) == b'\'' as c_char
	    {
	      cp = cp.offset (1);
	      len -= 2;
	    }
	  make_tag (cp, len, true, s, pos, lineno, linecharno);
	}
    }
  let _ = ch;
  return;
}


/*
 * Consume an Erlang atom (or variable).
 * Return the number of bytes consumed, or -1 if there was an error.
 */
unsafe fn erlang_atom (s: *mut c_char) -> ptrdiff_t
{
  let mut pos: ptrdiff_t = 0;

  if c_isalpha (*s as c_int) || *s == b'_' as c_char
    {
      /* The atom is unquoted. */
      loop
	{
	  pos += 1;
	  if !(c_isalnum (*s.offset (pos) as c_int) || *s.offset (pos) == b'_' as c_char)
	    { break; }
	}
    }
  else if *s == b'\'' as c_char
    {
      pos += 1;
      while *s.offset (pos) != b'\'' as c_char
	{
	  if *s.offset (pos) == 0	/* multiline quoted atoms are ignored */
	      || (*s.offset (pos) == b'\\' as c_char
		  && { pos += 1; *s.offset (pos) == 0 })
	    { return 0; }
	  pos += 1;
	}
      pos += 1;
    }

  return pos;
}


/* `<stdio.h>`: `int rename (const char *old, const char *new);` -- no mirror carries it.  */
unsafe extern "C" {
  pub fn rename (old: *const c_char, new: *const c_char) -> c_int;
}

/* `<stdio.h>`: `int vfprintf (FILE *restrict, const char *restrict, va_list);` -- the
   `va_list` parameter is the port's `VaList<'_>` (P-04; the make-docfile.rs 111 precedent).  */
unsafe extern "C" {
  pub fn vfprintf (stream: *mut FILE, format: *const c_char, ap: VaList<'_>) -> c_int;
}

/* `<inttypes.h>`: `intmax_t strtoimax (const char *restrict, char **restrict, int);` --
   `lib/strtoimax.rs` owns the definition (the emacsclient.rs 307 precedent).  */
unsafe extern "C" {
  pub fn strtoimax (nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> intmax_t;
}

/* `<stdlib.h>`/`<fcntl.h>`: `int mkostemp (char *xtemplate, int flags);` --
   `lib/mkostemp.rs` owns the definition.  */
unsafe extern "C" {
  pub fn mkostemp (xtemplate: *mut c_char, flags: c_int) -> c_int;
}

/*
 * Take a string like "/blah/" and turn it into "blah", verifying
 * that the first and last characters are the same, and handling
 * quoted separator characters.  Actually, stops on the occurrence of
 * an unquoted separator.  Also process \t, \n, etc. and turn into
 * appropriate characters. Works in place.  Null terminates name string.
 * Returns pointer to terminating separator, or NULL for
 * unterminated regexps.
 */
unsafe fn scan_separators (mut name: *mut c_char) -> *mut c_char
{
  let sep: c_char = *name;
  let mut copyto: *mut c_char = name;
  let mut quoted: bool = false;

  name = name.offset (1);
  while *name != 0
    {
      if quoted
	{
	  match *name
	    {
	    97 /* 'a' */ => { *copyto = 7 /* '\007' */; copyto = copyto.offset (1); } /* BEL (bell)		 */
	    98 /* 'b' */ => { *copyto = 8 /* '\b' */; copyto = copyto.offset (1); }	 /* BS (back space)	 */
	    100 /* 'd' */ => { *copyto = 127 /* 0177 */; copyto = copyto.offset (1); }	 /* DEL (delete)	 */
	    101 /* 'e' */ => { *copyto = 27 /* 033 */; copyto = copyto.offset (1); }	 /* ESC (delete)	 */
	    102 /* 'f' */ => { *copyto = 12 /* '\f' */; copyto = copyto.offset (1); }	 /* FF (form feed)	 */
	    110 /* 'n' */ => { *copyto = 10 /* '\n' */; copyto = copyto.offset (1); }	 /* NL (new line)	 */
	    114 /* 'r' */ => { *copyto = 13 /* '\r' */; copyto = copyto.offset (1); }	 /* CR (carriage return) */
	    116 /* 't' */ => { *copyto = 9 /* '\t' */; copyto = copyto.offset (1); }	 /* TAB (horizontal tab) */
	    118 /* 'v' */ => { *copyto = 11 /* '\v' */; copyto = copyto.offset (1); }	 /* VT (vertical tab)    */
	    _ =>
	      {
		if *name == sep
		  { *copyto = sep; copyto = copyto.offset (1); }
		else
		  {
		    /* Something else is quoted, so preserve the quote. */
		    *copyto = b'\\' as c_char; copyto = copyto.offset (1);
		    *copyto = *name; copyto = copyto.offset (1);
		  }
	      }
	    }
	  quoted = false;
	}
      else if *name == b'\\' as c_char
	{ quoted = true; }
      else if *name == sep
	{ break; }
      else
	{ *copyto = *name; copyto = copyto.offset (1); }
      name = name.offset (1);
    }
  if *name != sep
    { name = std::ptr::null_mut (); }		/* signal unterminated regexp */

  /* Terminate copied string. */
  *copyto = 0;
  return name;
}

/* Look at the argument of --regex or --no-regex and do the right
   thing.  Same for each line of a regexp file. */
unsafe fn analyze_regex (regex_arg: *mut c_char)
{
  if regex_arg == std::ptr::null_mut ()
    {
      free_regexps ();		/* --no-regex: remove existing regexps */
      return;
    }

  /* A real --regexp option or a line in a regexp file. */
  match *regex_arg
    {
      /* Comments in regexp file or null arg to --regex. */
    0 /* '\0' */ | 32 /* ' ' */ | 9 /* '\t' */ => {}

      /* Read a regex file.  This is recursive and may result in a
	 loop, which will stop when the file descriptors are exhausted. */
    64 /* '@' */ =>
      {
	let mut regexfp: *mut FILE;
	let mut regexbuf: linebuffer;
	let mut regexfile: *mut c_char = regex_arg.offset (1);

	/* regexfile is a file containing regexps, one per line. */
	regexfp = fopen (regexfile, c"r".as_ptr ());	/* C: `"r" FOPEN_BINARY` */
	if regexfp == std::ptr::null_mut ()
	  { pfatal (regexfile); }
	regexbuf = linebuffer { size: 0, len: 0, buffer: std::ptr::null_mut () };
	linebuffer_init (&raw mut regexbuf);
	while readline_internal (&raw mut regexbuf, regexfp, regexfile, false) > 0
	  { analyze_regex (regexbuf.buffer); }
	free (regexbuf.buffer as *mut c_void);
	if fclose (regexfp) != 0
	  { pfatal (regexfile); }
      }

      /* Regexp to be used for a specific language only. */
    123 /* '{' */ =>
      {
	let mut lang: *mut language;
	let mut lang_name: *mut c_char = regex_arg.offset (1);
	let mut cp: *mut c_char;

	cp = lang_name;
	while *cp != b'}' as c_char
	  {
	    if *cp == 0
	      {
		error (c"unterminated language name in regex: %s".as_ptr (), regex_arg);
		return;
	      }
	    cp = cp.offset (1);
	  }
	*cp = 0;
	cp = cp.offset (1);
	lang = get_language_from_langname (lang_name);
	if lang == std::ptr::null_mut ()
	  { return; }
	add_regex (cp, lang);
      }

      /* Regexp to be used for any language. */
    _ =>
      {
	add_regex (regex_arg, std::ptr::null_mut ());
      }
    }
}

/* Separate the regexp pattern, compile it,
   and care for optional name and modifiers. */
unsafe fn add_regex (regexp_pattern: *mut c_char, lang: *mut language)
{
  /* C's function-scope `static struct re_pattern_buffer zeropattern;` is a zero-filled
     object that is only read (`*patbuf = zeropattern`); the port keeps it as a `const` item
     whose bytes are C's (the `emptyfdesc` note above).  */
  const zeropattern: re_pattern_buffer =
    re_pattern_buffer { buffer: std::ptr::null_mut (), allocated: 0, used: 0, syntax: 0,
			fastmap: std::ptr::null_mut (), translate: std::ptr::null_mut (),
			re_nsub: 0, bits: 0 };
  let mut sep: c_char; let mut pat: *mut c_char; let mut name: *mut c_char;
  let mut modifiers: *mut c_char;
  let mut empty: c_char = 0;
  let mut err: *const c_char;
  let mut patbuf: *mut re_pattern_buffer;
  let mut rp: *mut regexp;
  let mut ignore_case: bool = false;		/* case is significant */
  let mut multi_line: bool = false;		/* matches are done one line at a time */
  let mut single_line: bool = false;		/* dot does not match newline */


  if strnlen (regexp_pattern, 3) < 3
    {
      error (c"null regexp".as_ptr ());
      return;
    }
  sep = *regexp_pattern;
  name = scan_separators (regexp_pattern);
  if name == std::ptr::null_mut ()
    {
      error (c"%s: unterminated regexp".as_ptr (), regexp_pattern);
      return;
    }
  if *name.offset (1) == sep
    {
      error (c"null name for regexp \"%s\"".as_ptr (), regexp_pattern);
      return;
    }
  modifiers = scan_separators (name);
  if modifiers == std::ptr::null_mut ()	/* no terminating separator --> no name */
    {
      modifiers = name;
      name = &raw mut empty;
    }
  else
    { modifiers = modifiers.offset (1); }		/* skip separator */

  /* Parse regex modifiers. */
  while *modifiers != 0
    {
      match *modifiers
	{
	78 /* 'N' */ =>
	  {
	    if modifiers == name
	      { error (c"forcing explicit tag name but no name, ignoring".as_ptr ()); }
	    /* This option has no effect and is present only for backward
	       compatibility.  */
	  }
	105 /* 'i' */ =>
	  {
	    ignore_case = true;
	  }
	115 /* 's' */ =>
	  {
	    single_line = true;
	    /* FALLTHROUGH into the 'm' arm (PORTING.md 2.2).  */
	    multi_line = true;
	    need_filebuf = true;
	  }
	109 /* 'm' */ =>
	  {
	    multi_line = true;
	    need_filebuf = true;
	  }
	_ =>
	  {
	    error (c"invalid regexp modifier '%c', ignoring".as_ptr (),
		   *modifiers as c_int);
	  }
	}
      modifiers = modifiers.offset (1);
    }

  patbuf = xmalloc (std::mem::size_of::<re_pattern_buffer> () as ptrdiff_t)
	as *mut re_pattern_buffer;
  *patbuf = zeropattern;
  if ignore_case
    {
      /* C's function-scope `static unsigned char lc_trans[UCHAR_MAX + 1];` -- a persistent
	 zero-filled table the port keeps as a `static mut` (PORTING.md 0.2).  */
      static mut lc_trans: [c_uchar; 256] = [0; 256];
      let mut i: c_int;
      i = 0;
      while i < 256 /* UCHAR_MAX + 1 */
	{ lc_trans[i as usize] = c_tolower (i) as c_uchar; i += 1; }
      (*patbuf).translate = &raw mut lc_trans as *mut u8;	/* translation table to fold case  */
    }

  if multi_line
    { pat = concat (c"^".as_ptr (), regexp_pattern, c"".as_ptr ()); } /* anchor to beginning of line */
  else
    { pat = regexp_pattern; }

  if single_line
    { re_set_syntax ((RE_SYNTAX_EMACS | RE_DOT_NEWLINE) as c_ulong); }
  else
    { re_set_syntax (RE_SYNTAX_EMACS as c_ulong); }

  err = re_compile_pattern (pat, strlen (pat), patbuf as *mut c_void);
  if multi_line
    { free (pat as *mut c_void); }
  if err != std::ptr::null ()
    {
      error (c"%s while compiling pattern".as_ptr (), err);
      return;
    }

  rp = p_head;
  p_head = xmalloc (__sizeof_deref (p_head)) as *mut regexp;
  (*p_head).pattern = savestr (regexp_pattern);
  (*p_head).p_next = rp;
  (*p_head).lang = lang;
  (*p_head).pat = patbuf;
  (*p_head).name = savestr (name);
  (*p_head).error_signaled = false;
  (*p_head).ignore_case = ignore_case;
  (*p_head).multi_line = multi_line;
}

/*
 * Do the substitutions indicated by the regular expression and
 * arguments.
 */
unsafe fn substitute (input: *mut c_char, mut out: *mut c_char,
		      regs: *mut re_registers) -> *mut c_char
{
  let mut result: *mut c_char; let mut t: *mut c_char;

  result = std::ptr::null_mut ();
  let mut size: ptrdiff_t = strlen (out) as ptrdiff_t;

  /* Pass 1: figure out how much to allocate by finding all \N strings. */
  if *out.offset (size - 1) == b'\\' as c_char
    { fatal (c"pattern error in \"%s\"".as_ptr (), out); }
  t = strchr (out, b'\\' as c_int);
  while t != std::ptr::null_mut ()
    {
      if c_isdigit (*t.offset (1) as c_int)
	{
	  let dig: c_int = *t.offset (1) as c_int - b'0' as c_int;
	  let diglen: ptrdiff_t = *(*regs).end.offset (dig as isize)
	    - *(*regs).start.offset (dig as isize);
	  size += diglen - 2;
	}
      else
	{ size -= 1; }
      t = strchr (t.offset (2), b'\\' as c_int);
    }

  /* Allocate space and do the substitutions. */
  assert! (size >= 0);
  result = xmalloc (size + 1) as *mut c_char;

  t = result;
  while *out != 0
    {
      if *out == b'\\' as c_char
	  && { out = out.offset (1); c_isdigit (*out as c_int) }
	{
	  let dig: c_int = *out as c_int - b'0' as c_int;
	  let diglen: ptrdiff_t = *(*regs).end.offset (dig as isize)
	    - *(*regs).start.offset (dig as isize);
	  memcpy (t as *mut c_void,
		  input.offset (*(*regs).start.offset (dig as isize)) as *const c_void,
		  diglen as usize);
	  t = t.offset (diglen);
	}
      else
	{ *t = *out; t = t.offset (1); }
      out = out.offset (1);
    }
  *t = 0;

  assert! (t <= result.offset (size));
  assert! (t == result.offset (strlen (result) as isize));

  return result;
}

/* Deallocate all regexps. */
unsafe fn free_regexps ()
{
  let mut rp: *mut regexp;
  while p_head != std::ptr::null_mut ()
    {
      rp = (*p_head).p_next;
      free ((*p_head).pattern as *mut c_void);
      free ((*p_head).name as *mut c_void);
      free (p_head as *mut c_void);
      p_head = rp;
    }
  return;
}

/*
 * Reads the whole file as a single string from `filebuf' and looks for
 * multi-line regular expressions, creating tags on matches.
 * readline already dealt with normal regexps.
 *
 * Idea by Ben Wing <ben@666.com> (2002).
 */
unsafe fn regex_tag_multiline ()
{
  let mut buffer: *mut c_char = filebuf.buffer;
  let mut rp: *mut regexp;
  let mut name: *mut c_char;

  rp = p_head;
  while rp != std::ptr::null_mut ()
    {
      let mut match_: ptrdiff_t = 0;

      if !(*rp).multi_line
	{
	  rp = (*rp).p_next;
	  continue;		/* skip normal regexps */
	}

      /* Generic initializations before parsing file from memory. */
      lineno = 1;		/* reset global line number */
      charno = 0;		/* reset global char number */
      linecharno = 0;		/* reset global char number of line start */

      /* Only use generic regexps or those for the current language. */
      if (*rp).lang != std::ptr::null_mut () && (*rp).lang != (*curfdp).lang
	{
	  rp = (*rp).p_next;
	  continue;
	}

      while match_ >= 0 && match_ < filebuf.len
	{
	  match_ = re_search ((*rp).pat as *mut c_void, buffer, filebuf.len,
			      charno as isize, filebuf.len - match_,
			      &raw mut (*rp).regs as *mut c_void);
	  match match_
	    {
	    -2 =>
	      {
		/* Some error. */
		if !(*rp).error_signaled
		  {
		    error (c"regexp stack overflow while matching \"%s\"".as_ptr (),
			   (*rp).pattern);
		    (*rp).error_signaled = true;
		  }
	      }
	    -1 => {}		/* No match. */
	    _ =>
	      {
		if match_ == *(*rp).regs.end.offset (0)
		  {
		    if !(*rp).error_signaled
		      {
			error (c"regexp matches the empty string: \"%s\"".as_ptr (),
			       (*rp).pattern);
			(*rp).error_signaled = true;
		      }
		    match_ = -3;	/* exit from while loop */
		  }
		else
		  {
		    /* Match occurred.  Construct a tag. */
		    while charno < *(*rp).regs.end.offset (0) as intmax_t
		      {
			let ch: c_char = *buffer.offset (charno as isize);
			charno += 1;
			if ch == b'\n' as c_char
			  { lineno += 1; linecharno = charno; }
		      }
		    name = (*rp).name;
		    if *name == 0
		      { name = std::ptr::null_mut (); }
		    else /* make a named tag */
		      { name = substitute (buffer, (*rp).name, &raw mut (*rp).regs); }

		    /* Force explicit tag name, if a name is there. */
		    pfnote (name, true, buffer.offset (linecharno as isize),
			    (charno - linecharno) as ptrdiff_t, lineno, linecharno);

		    if debug != 0
		      { fprintf (stderr, c"%s on %s:%jd: %s\n".as_ptr (),	/* C: `"%"PRIdMAX` */
				 if name != std::ptr::null_mut () { name }
				 else { c"(unnamed)".as_ptr () },
				 (*curfdp).taggedfname, lineno,
				 buffer.offset (linecharno as isize)); }
		  }
	      }
	    }
	}
      rp = (*rp).p_next;
    }
}


unsafe fn nocase_tail (mut cp: *const c_char) -> bool
{
  let mut len: ptrdiff_t = 0;

  while *cp != 0 && c_tolower (*cp as c_int) == c_tolower (*dbp.offset (len) as c_int)
    { cp = cp.offset (1); len += 1; }
  if *cp == 0 && !intoken (*dbp.offset (len) as c_uchar)
    {
      dbp = dbp.offset (len);
      return true;
    }
  return false;
}

unsafe fn get_tag (bp: *mut c_char, namepp: *mut *mut c_char)
{
  let mut cp: *mut c_char = bp;

  if *bp != 0
    {
      /* Go till you get to white space or a syntactic break */
      cp = bp.offset (1);
      while !notinname (*cp as c_uchar)
	{ cp = cp.offset (1); }
      make_tag (bp, cp.offset_from (bp), true,
		lb.buffer, cp.offset_from (lb.buffer) + 1, lineno, linecharno);
    }

  if namepp != std::ptr::null_mut ()
    { *namepp = savenstr (bp, cp.offset_from (bp)); }
}

/* Similar to get_tag, but include '=' as part of the tag. */
unsafe fn get_lispy_tag (bp: *mut c_char)
{
  let mut cp: *mut c_char = bp;

  if *bp != 0
    {
      /* Go till you get to white space or a syntactic break */
      cp = bp.offset (1);
      while !notinname (*cp as c_uchar) || *cp == b'=' as c_char
	{ cp = cp.offset (1); }
      make_tag (bp, cp.offset_from (bp), true,
		lb.buffer, cp.offset_from (lb.buffer) + 1, lineno, linecharno);
    }
}

/*
 * Read a line of text from `stream' into `lbp', excluding the
 * newline or CR-NL (if `leave_cr' is false), if any.  Return the
 * number of characters read from `stream', which is the length
 * of the line including the newline.
 *
 * On DOS or Windows, if `leave_cr` is false, we do not count the
 * CR character, if any before the NL, in the returned length;
 * this mirrors the behavior of Emacs on those
 * platforms (for text files, it translates CR-NL to NL as it reads in the
 * file).
 *
 * If multi-line regular expressions are requested, each line read is
 * appended to `filebuf'.
 */
unsafe fn readline_internal (lbp: *mut linebuffer, stream: *mut FILE,
			     filename: *const c_char, leave_cr: bool) -> ptrdiff_t
{
  let mut buffer: *mut c_char = (*lbp).buffer;
  let mut p: *mut c_char = (*lbp).buffer;
  let mut pend: *mut c_char;
  let mut chars_deleted: c_int;

  pend = p.offset ((*lbp).size);		/* Separate to avoid 386/IX compiler bug.  */

  loop
    {
      let mut c: c_int = getc (stream);
      if p == pend
	{
	  /* We're at the end of linebuffer: expand it. */
	  xrnew! (buffer, (*lbp).size, 2);
	  p = buffer.offset ((*lbp).size);
	  (*lbp).size *= 2;
	  pend = buffer.offset ((*lbp).size);
	  (*lbp).buffer = buffer;
	}
      if c == EOF
	{
	  if ferror (stream) != 0
	    { perror (filename); }
	  *p = 0;
	  chars_deleted = 0;
	  break;
	}
      if c == b'\n' as c_int
        {
          if !leave_cr && p > buffer && *p.offset (-1) == b'\r' as c_char
            {
              p = p.offset (-1);
              chars_deleted = 2;
            }
          else
            {
              chars_deleted = 1;
            }
          *p = 0;
          break;
        }
      *p = c as c_char;
      p = p.offset (1);
    }
  (*lbp).len = p.offset_from (buffer);

  if need_filebuf		/* we need filebuf for multi-line regexps */
      && chars_deleted > 0	/* not at EOF */
    {
      while filebuf.size <= filebuf.len + (*lbp).len + 1 /* +1 for \n */
	{
	  /* Expand filebuf. */
	  xrnew! (filebuf.buffer, filebuf.size, 2);
	  filebuf.size *= 2;
	}
      strcpy (mempcpy (filebuf.buffer.offset (filebuf.len) as *mut c_void,
			   (*lbp).buffer as *const c_void,
			   (*lbp).len as size_t) as *mut c_char,
	      c"\n".as_ptr ());
      filebuf.len += (*lbp).len + 1;
    }

  return (*lbp).len + chars_deleted as ptrdiff_t;
}

/*
 * Like readline_internal, above, but in addition try to match the
 * input line against relevant regular expressions and manage #line
 * directives.
 */
unsafe fn readline (lbp: *mut linebuffer, stream: *mut FILE)
{
  linecharno = charno;		/* update global char number of line start */
  let mut result: ptrdiff_t = readline_internal (lbp, stream, infilename, false);
  lineno += 1;			/* increment global line number */
  charno += result as intmax_t;		/* increment global char number */

  /* Honor #line directives. */
  if no_line_directive == 0
    {
      static mut discard_until_line_directive: bool = false;

      /* Check whether this is a #line directive. */
      if result > 12 && strneq ((*lbp).buffer, c"#line ".as_ptr (), 6)
	{
	  let mut lno_start: *mut c_char = (*lbp).buffer.offset (6);
	  let mut lno_end: *mut c_char = std::ptr::null_mut ();
	  let mut lno: intmax_t = strtoimax (lno_start, &raw mut lno_end, 10);
	  let mut quoted_filename: *mut c_char =
	    if lno_start < lno_end { skip_spaces (lno_end) }
	    else { std::ptr::null_mut () };

	  if quoted_filename != std::ptr::null_mut () && *quoted_filename == b'"' as c_char
	    {
	      let mut endp: *mut c_char = quoted_filename;
	      loop
		{
		  endp = endp.offset (1);
		  if !(*endp != 0 && *endp != b'"' as c_char)
		    { break; }
		  endp = endp.offset ((*endp == b'\\' as c_char
				       && *endp.offset (1) != 0) as isize);
		}

	      if *endp != 0
		/* Ok, this is a real #line directive.  Let's deal with it. */
		{
		  let mut taggedabsname: *mut c_char;	/* absolute name of original file */
		  let mut taggedfname: *mut c_char;	/* name of original file as given */
		  let mut name: *mut c_char = quoted_filename.offset (1);

		  discard_until_line_directive = false; /* found it */
		  *endp = 0;
		  canonicalize_filename (name);
		  taggedabsname = absolute_filename (name, tagfiledir);
		  if filename_is_absolute (name)
		      || filename_is_absolute ((*curfdp).infname)
		    { taggedfname = savestr (taggedabsname); }
		  else
		    { taggedfname = relative_filename (taggedabsname, tagfiledir); }

		  if streq ((*curfdp).taggedfname, taggedfname)
		    /* The #line directive is only a line number change.  We
		       deal with this afterwards. */
		    { free (taggedfname as *mut c_void); }
		  else
		    /* The tags following this #line directive should be
		       attributed to taggedfname.  In order to do this, set
		       curfdp accordingly. */
		    {
		      let mut fdp: *mut fdesc; /* file description pointer */

		      /* Go look for a file description already set up for the
			 file indicated in the #line directive.  If there is
			 one, use it from now until the next #line
			 directive. */
		      fdp = fdhead;
		      while fdp != std::ptr::null_mut ()
			{
			  if streq ((*fdp).infname, (*curfdp).infname)
			      && streq ((*fdp).taggedfname, taggedfname)
			    /* If we remove the second test above (after the &&)
			       then all entries pertaining to the same file are
			       coalesced in the tags file.  If we use it, then
			       entries pertaining to the same file but generated
			       from different files (via #line directives) will
			       go into separate sections in the tags file.  These
			       alternatives look equivalent.  The first one
			       destroys some apparently useless information. */
			    {
			      curfdp = fdp;
			      free (taggedfname as *mut c_void);
			      break;
			    }
			  fdp = (*fdp).next;
			}
		      /* Else, if we already tagged the real file, skip all
			 input lines until the next #line directive. */
		      if fdp == std::ptr::null_mut () /* not found */
			{
			  fdp = fdhead;
			  while fdp != std::ptr::null_mut ()
			    {
			      if streq ((*fdp).infabsname, taggedabsname)
				{
				  discard_until_line_directive = true;
				  free (taggedfname as *mut c_void);
				  break;
				}
			      fdp = (*fdp).next;
			    }
			}
		      /* Else create a new file description and use that from
			 now on, until the next #line directive. */
		      if fdp == std::ptr::null_mut () /* not found */
			{
			  fdp = fdhead;
			  fdhead = xmalloc (__sizeof_deref (fdhead)) as *mut fdesc;
			  *fdhead = *curfdp; /* copy curr. file description */
			  (*fdhead).next = fdp;
			  (*fdhead).infname = savestr ((*curfdp).infname);
			  (*fdhead).infabsname = savestr ((*curfdp).infabsname);
			  (*fdhead).infabsdir = savestr ((*curfdp).infabsdir);
			  (*fdhead).taggedfname = taggedfname;
			  (*fdhead).usecharno = false;
			  (*fdhead).prop = std::ptr::null_mut ();
			  (*fdhead).written = false;
			  curfdp = fdhead;
			}
		    }
		  free (taggedabsname as *mut c_void);
		  lineno = lno - 1;
		  readline (lbp, stream);
		  return;
		} /* if a real #line directive */
	    } /* if #line is followed by a number */
	} /* if line begins with "#line " */

      /* If we are here, no #line directive was found. */
      if discard_until_line_directive
	{
	  if result > 0
	    {
	      /* Do a tail recursion on ourselves, thus discarding the contents
		 of the line buffer. */
	      readline (lbp, stream);
	      return;
	    }
	  /* End of file. */
	  discard_until_line_directive = false;
	  return;
	}
    } /* if #line directives should be considered */

  {
    let mut match_: ptrdiff_t;
    let mut rp: *mut regexp;
    let mut name: *mut c_char;

    /* Match against relevant regexps. */
    if (*lbp).len > 0
      {
	rp = p_head;
	while rp != std::ptr::null_mut ()
	  {
	    /* Only use generic regexps or those for the current language.
	       Also do not use multiline regexps, which is the job of
	       regex_tag_multiline. */
	    if ((*rp).lang != std::ptr::null_mut () && (*rp).lang != (*fdhead).lang)
		|| (*rp).multi_line
	      {
		rp = (*rp).p_next;
		continue;
	      }

	    match_ = re_match ((*rp).pat as *mut c_void, (*lbp).buffer, (*lbp).len,
			       0, &raw mut (*rp).regs as *mut c_void);
	    match match_
	      {
	      -2 =>
		{
		  /* Some error. */
		  if !(*rp).error_signaled
		    {
		      error (c"regexp stack overflow while matching \"%s\"".as_ptr (),
			     (*rp).pattern);
		      (*rp).error_signaled = true;
		    }
		}
	      -1 => {}		/* No match. */
	      0 =>
		{
		  /* Empty string matched. */
		  if !(*rp).error_signaled
		    {
		      error (c"regexp matches the empty string: \"%s\"".as_ptr (),
			     (*rp).pattern);
		      (*rp).error_signaled = true;
		    }
		}
	      _ =>
		{
		  /* Match occurred.  Construct a tag. */
		  name = (*rp).name;
		  if *name == 0
		    { name = std::ptr::null_mut (); }
		  else /* make a named tag */
		    { name = substitute ((*lbp).buffer, (*rp).name, &raw mut (*rp).regs); }

		  /* Force explicit tag name, if a name is there. */
		  pfnote (name, true, (*lbp).buffer, match_, lineno, linecharno);

		  if debug != 0
		    { fprintf (stderr, c"%s on %s:%jd: %s\n".as_ptr (),	/* C: `"%"PRIdMAX` */
			       if name != std::ptr::null_mut () { name }
			       else { c"(unnamed)".as_ptr () },
			       (*curfdp).taggedfname, lineno, (*lbp).buffer); }
		}
	      }
	    rp = (*rp).p_next;
	  }
      }
  }
}

/*
 * Return a pointer to a space of size strlen(cp)+1 allocated
 * with xmalloc where the string CP has been copied.
 */
unsafe fn savestr (cp: *const c_char) -> *mut c_char
{
  return savenstr (cp, strlen (cp) as ptrdiff_t);
}

/*
 * Return a pointer to a space of size LEN+1 allocated with xmalloc
 * with a copy of CP (containing LEN bytes) followed by a NUL byte.
 */
unsafe fn savenstr (cp: *const c_char, len: ptrdiff_t) -> *mut c_char
{
  let mut dp: *mut c_char = xmalloc (len + 1) as *mut c_char;
  *dp.offset (len) = 0;
  return memcpy (dp as *mut c_void, cp as *const c_void, len as usize) as *mut c_char;
}

/* Skip spaces (end of string is not space), return new pointer. */
unsafe fn skip_spaces (mut cp: *mut c_char) -> *mut c_char
{
  while c_isspace (*cp as c_int)
    { cp = cp.offset (1); }
  return cp;
}

/* Skip non spaces, except end of string, return new pointer. */
unsafe fn skip_non_spaces (mut cp: *mut c_char) -> *mut c_char
{
  while *cp != 0 && !c_isspace (*cp as c_int)
    { cp = cp.offset (1); }
  return cp;
}

/* Skip any chars in the "name" class.*/
unsafe fn skip_name (mut cp: *mut c_char) -> *mut c_char
{
  /* '\0' is a notinname() so loop stops there too */
  while ! notinname (*cp as c_uchar)
    { cp = cp.offset (1); }
  return cp;
}

/* Print error message and exit.  */
/* C 7814: `static void ATTRIBUTE_FORMAT_PRINTF (1, 2) fatal (char const *format, ...)` - the
   attribute has no Rust counterpart (PORTING.md 8), and C's `va_list ap; va_start (ap,
   format); va_end (ap);` have no port site: the `...` parameter *is* the list (P-04).  */
unsafe extern "C" fn fatal (format: *const c_char, mut ap: ...)
{
  verror (format, ap);
  exit (EXIT_FAILURE);
}

unsafe fn pfatal (s1: *const c_char)
{
  perror (s1);
  exit (EXIT_FAILURE);
}

unsafe extern "C" fn suggest_asking_for_help () -> !
{
  fprintf (stderr, c"\tTry '%s --help' for a complete list of options.\n".as_ptr (),
	   progname);
  exit (EXIT_FAILURE);
}

/* Output a diagnostic with printf-style FORMAT and args.  */
/* C 7840: `static void ATTRIBUTE_FORMAT_PRINTF (1, 2) error (const char *format, ...)`  */
unsafe extern "C" fn error (format: *const c_char, mut ap: ...)
{
  verror (format, ap);
}

unsafe fn verror (format: *const c_char, ap: VaList<'_>)
{
  fprintf (stderr, c"%s: ".as_ptr (), progname);
  vfprintf (stderr, format, ap);
  fprintf (stderr, c"\n".as_ptr ());
}

/* Return a newly-allocated string whose contents
   concatenate those of s1, s2, s3.  */
unsafe fn concat (s1: *const c_char, s2: *const c_char, s3: *const c_char) -> *mut c_char
{
  let mut len1: ptrdiff_t = strlen (s1) as ptrdiff_t;
  let mut len2: ptrdiff_t = strlen (s2) as ptrdiff_t;
  let mut len3: ptrdiff_t = strlen (s3) as ptrdiff_t;
  let mut result: *mut c_char = xmalloc (len1 + len2 + len3 + 1) as *mut c_char;
  strcpy (stpcpy (stpcpy (result, s1), s2), s3);
  return result;
}


/* Does the same work as the system V getcwd, but does not need to
   guess the buffer size in advance. */
unsafe fn etags_getcwd () -> *mut c_char
{
  let mut bufsize: ptrdiff_t = 200;
  let mut path: *mut c_char = xmalloc (bufsize) as *mut c_char;

  while getcwd (path, bufsize as size_t) == std::ptr::null_mut ()
    {
      if *errno () != ERANGE
	{ pfatal (c"getcwd".as_ptr ()); }
      free (path as *mut c_void);
      path = xnmalloc (bufsize, 2 * __sizeof_deref (path)) as *mut c_char;
      bufsize *= 2;
    }

  canonicalize_filename (path);
  return path;
}

/* Return a newly allocated string containing a name of a temporary file.  */
unsafe fn etags_mktmp () -> *mut c_char
{
  let mut tmpdir: *const c_char = getenv (c"TMPDIR".as_ptr ());
  let mut slash: *const c_char = c"/".as_ptr ();

  /* C 7911-7921's `#if MSDOS || defined (DOS_NT)` arm is dead in the frozen configuration
     (recorded, not transcribed); the live arm is the `#else` below.  */
  if tmpdir == std::ptr::null ()
    { tmpdir = c"/tmp".as_ptr (); }
  if *tmpdir.offset (strlen (tmpdir) as isize - 1) == b'/' as c_char
    { slash = c"".as_ptr (); }

  let mut templt: *mut c_char = concat (tmpdir, slash, c"etXXXXXX".as_ptr ());
  let mut fd: c_int = mkostemp (templt, O_CLOEXEC);
  if fd < 0 || close (fd) != 0
    {
      free (templt as *mut c_void);
      templt = std::ptr::null_mut ();
    }
  /* C 7920-7930's `#if defined (DOS_NT)` arm is dead in the frozen configuration
     (recorded, not transcribed).  */

  return templt;
}

/* C 7935: `#if !MSDOS && !defined (DOS_NT)` is live in the frozen configuration.  */
/*
 * Add single quotes around a string, and escape any single quotes.
 * Return a newly-allocated string.
 *
 * For example:
 * escape_shell_arg_string ("test.txt")  => "'test.txt'"
 * escape_shell_arg_string ("'test.txt") => "''\''test.txt'"
 */
unsafe fn escape_shell_arg_string (str_: *mut c_char) -> *mut c_char
{
  let mut p: *mut c_char = str_;
  let mut need_space: c_int = 2;		/* ' at begin and end */

  while *p != 0
    {
      if *p == b'\'' as c_char
	{ need_space += 4; }	/* ' to '\'', length is 4 */
      else
	{ need_space += 1; }

      p = p.offset (1);
    }

  let mut new_str: *mut c_char = xmalloc (need_space as ptrdiff_t + 1) as *mut c_char;
  *new_str = b'\'' as c_char;
  *new_str.offset (need_space as isize - 1) = b'\'' as c_char;

  let mut i: c_int = 1;			/* skip first byte */
  p = str_;
  while *p != 0
    {
      *new_str.offset (i as isize) = *p;
      if *p == b'\'' as c_char
	{
	  *new_str.offset (i as isize + 1) = b'\\' as c_char;
	  *new_str.offset (i as isize + 2) = b'\'' as c_char;
	  *new_str.offset (i as isize + 3) = b'\'' as c_char;
	  i += 3;
	}

      i += 1;
      p = p.offset (1);
    }

  *new_str.offset (need_space as isize) = 0;
  return new_str;
}

unsafe fn do_move_file (src_file: *const c_char, dst_file: *const c_char)
{
  if rename (src_file, dst_file) == 0
    { return; }

  let mut src_f: *mut FILE = fopen (src_file, c"rb".as_ptr ());
  let mut dst_f: *mut FILE = fopen (dst_file, c"wb".as_ptr ());

  if src_f == std::ptr::null_mut ()
    { pfatal (src_file); }

  if dst_f == std::ptr::null_mut ()
    { pfatal (dst_file); }

  let mut c: c_int;
  c = fgetc (src_f);
  while c != EOF
    {
      if ferror (src_f) != 0
        { pfatal (src_file); }

      if ferror (dst_f) != 0
        { pfatal (dst_file); }

      if fputc (c, dst_f) == EOF
        { pfatal (c"cannot write".as_ptr ()); }
      c = fgetc (src_f);
    }

  if fclose (src_f) == EOF
    { pfatal (src_file); }

  if fclose (dst_f) == EOF
    { pfatal (dst_file); }

  if unlink (src_file) < 0 && *errno () != ENOENT
    { pfatal (c"unlink error".as_ptr ()); }

  return;
}

/* Return a newly allocated string containing the file name of FILE
   relative to the absolute directory DIR (which should end with a slash). */
unsafe fn relative_filename (file: *mut c_char, dir: *mut c_char) -> *mut c_char
{
  let mut fp: *mut c_char; let mut dp: *mut c_char;
  let mut afn: *mut c_char; let mut res: *mut c_char;
  let mut i: ptrdiff_t;
  /* C's `char *dir_last_slash UNINIT;` -- the port's zero initialiser is the UNINIT class
     (PORTING.md 1.5).  */
  let mut dir_last_slash: *mut c_char = std::ptr::null_mut ();

  /* Find the common root of file and dir (with a trailing slash). */
  afn = absolute_filename (file, cwd);
  fp = afn;
  dp = dir;
  loop
    {
      let a: c_char = *fp;
      let b: c_char = *dp;
      fp = fp.offset (1);
      dp = dp.offset (1);
      if !(a == b)
	{ break; }
      if *dp.offset (-1) == b'/' as c_char
	{ dir_last_slash = dp.offset (-1); }
    }
  /* C 8042-8045's `#ifdef DOS_NT` arm is dead in the frozen configuration (recorded, not
     transcribed).  */
  fp = fp.offset (- dp.offset_from (dir_last_slash));
  dp = dir_last_slash;

  /* Build a sequence of "../" strings for the resulting relative file name. */
  i = 0;
  loop
    {
      dp = strchr (dp.offset (1), b'/' as c_int);
      if dp == std::ptr::null_mut ()
	{ break; }
      i += 1;
    }
  res = xmalloc (3 * i + strlen (fp.offset (1)) as ptrdiff_t + 1) as *mut c_char;
  let mut z: *mut c_char = res;
  loop
    {
      let old: ptrdiff_t = i;
      i -= 1;
      if !(old > 0)
	{ break; }
      z = stpcpy (z, c"../".as_ptr ());
    }

  /* Add the file name relative to the common root of file and dir. */
  strcpy (z, fp.offset (1));
  free (afn as *mut c_void);

  return res;
}

/* Return a newly allocated string containing the absolute file name
   of FILE given DIR (which should end with a slash). */
unsafe fn absolute_filename (file: *mut c_char, dir: *mut c_char) -> *mut c_char
{
  let mut slashp: *mut c_char; let mut cp: *mut c_char; let mut res: *mut c_char;

  if filename_is_absolute (file)
    { res = savestr (file); }
  /* C 8067-8071's `#ifdef DOS_NT` arm is dead in the frozen configuration (recorded, not
     transcribed).  */
  else
    { res = concat (dir, file, c"".as_ptr ()); }

  /* Delete the "/dirname/.." and "/." substrings. */
  slashp = strchr (res, b'/' as c_int);
  while slashp != std::ptr::null_mut () && *slashp != 0
    {
      if *slashp.offset (1) == b'.' as c_char
	{
	  if *slashp.offset (2) == b'.' as c_char
	      && (*slashp.offset (3) == b'/' as c_char || *slashp.offset (3) == 0)
	    {
	      cp = slashp;
	      loop
		{
		  cp = cp.offset (-1);
		  if !(cp >= res && !filename_is_absolute (cp))
		    { break; }
		}
	      if cp < res
		{ cp = slashp; }	/* the absolute name begins with "/.." */
	      /* C 8099-8105's `#ifdef DOS_NT` arm is dead in the frozen configuration
		 (recorded, not transcribed).  */
              memmove (cp as *mut c_void, slashp.offset (3) as *const c_void,
		       strlen (slashp.offset (2)));
	      slashp = cp;
	      continue;
	    }
	  else if *slashp.offset (2) == b'/' as c_char || *slashp.offset (2) == 0
	    {
	      memmove (slashp as *mut c_void, slashp.offset (2) as *const c_void,
		       strlen (slashp.offset (1)));
	      continue;
	    }
	}

      slashp = strchr (slashp.offset (1), b'/' as c_int);
    }

  if *res == 0		/* just a safety net: should never happen */
    {
      free (res as *mut c_void);
      return savestr (c"/".as_ptr ());
    }
  else
    { return res; }
}

/* Return a newly allocated string containing the absolute
   file name of dir where FILE resides given DIR (which should
   end with a slash). */
unsafe fn absolute_dirname (file: *mut c_char, dir: *mut c_char) -> *mut c_char
{
  let mut slashp: *mut c_char; let mut res: *mut c_char;
  let mut save: c_char;

  slashp = strrchr (file, b'/' as c_int);
  if slashp == std::ptr::null_mut ()
    { return savestr (dir); }
  save = *slashp.offset (1);
  *slashp.offset (1) = 0;
  res = absolute_filename (file, dir);
  *slashp.offset (1) = save;

  return res;
}

/* Whether the argument string is an absolute file name.  The argument
   string must have been canonicalized with canonicalize_filename. */
unsafe fn filename_is_absolute (fn_: *mut c_char) -> bool
{
  return *fn_ == b'/' as c_char
  /* C 8155-8157's `#ifdef DOS_NT` arm is dead in the frozen configuration (recorded, not
     transcribed).  */
	  ;
}

/* Downcase DOS drive letter and collapse separators into single slashes.
   Works in place. */
unsafe fn canonicalize_filename (mut fn_: *mut c_char)
{
  let mut cp: *mut c_char;

  /* C 8166-8186's `#ifdef DOS_NT` arm is dead in the frozen configuration (recorded, not
     transcribed); the live arm is the `#else` below.  */

  /* Collapse multiple slashes into a single slash. */
  cp = fn_;
  while *cp != 0
    {
      if *cp == b'/' as c_char
	{
	  *fn_ = b'/' as c_char;
	  while *cp.offset (1) == b'/' as c_char
	    { cp = cp.offset (1); }
	}
      else
	{ *fn_ = *cp; }
      cp = cp.offset (1);
      fn_ = fn_.offset (1);
    }

  *fn_ = 0;
}


/* Initialize a linebuffer for use. */
unsafe fn linebuffer_init (lbp: *mut linebuffer)
{
  (*lbp).size = if DEBUG { 3 } else { 200 };
  (*lbp).buffer = xmalloc ((*lbp).size) as *mut c_char;
  *(*lbp).buffer = 0;
  (*lbp).len = 0;
}

/* Set the minimum size of a string contained in a linebuffer. */
unsafe fn linebuffer_setlen (lbp: *mut linebuffer, toksize: ptrdiff_t)
{
  if (*lbp).size <= toksize
    {
      let mut multiplier: ptrdiff_t = toksize / (*lbp).size + 1;
      xrnew! ((*lbp).buffer, (*lbp).size, multiplier);
      (*lbp).size *= multiplier;
    }
  (*lbp).len = toksize;
}

/* Memory allocators with a fatal error if memory is exhausted.  */

unsafe fn memory_full ()
{
  fatal (c"virtual memory exhausted".as_ptr ());
}

unsafe fn xmalloc (size: ptrdiff_t) -> *mut c_void
{
  if SIZE_MAX < size as size_t
    { memory_full (); }
  let mut result: *mut c_void = malloc (size as usize);
  if result == std::ptr::null_mut ()
    { memory_full (); }
  return result;
}

unsafe fn xnmalloc (nitems: ptrdiff_t, item_size: ptrdiff_t) -> *mut c_void
{
  let mut nbytes: ptrdiff_t;
  assume! (0 <= nitems);
  assume! (0 < item_size);
  /* C's `ckd_mul (&nbytes, nitems, item_size)`: the store happens only when the
     multiplication did not overflow (P-15, the ccl.rs shape).  */
  let (mul_value, mul_overflow) = nitems.overflowing_mul (item_size);
  nbytes = mul_value;
  if mul_overflow
    { memory_full (); }
  return xmalloc (nbytes);
}

unsafe fn xnrealloc (pa: *mut c_void, nitems: ptrdiff_t, item_size: ptrdiff_t)
  -> *mut c_void
{
  let mut nbytes: ptrdiff_t;
  assume! (0 <= nitems);
  assume! (0 < item_size);
  let (mul_value, mul_overflow) = nitems.overflowing_mul (item_size);
  nbytes = mul_value;
  if mul_overflow || SIZE_MAX < nbytes as size_t
    { memory_full (); }
  let mut result: *mut c_void = realloc (pa, nbytes as usize);
  if result == std::ptr::null_mut ()
    { memory_full (); }
  return result;
}

/*
 * Local Variables:
 * indent-tabs-mode: t
 * tab-width: 8
 * fill-column: 79
 * c-font-lock-extra-types: ("FILE" "bool" "language" "linebuffer" "fdesc" "node" "regexp")
 * c-file-style: "gnu"
 * End:
 */

/* etags.c ends here */
