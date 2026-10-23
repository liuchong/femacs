// femacs: port of lib-src/make-docfile.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names, the same
// `main`.  This is a host program of the C tree's lib-src directory (batch F): the frozen
// configuration builds it (docs/baseline/frozen-build-sources.txt, the reference
// `lib-src/Makefile`'s `DONT_INSTALL` list), and it links against the port's gnulib mirrors
// and libSystem only.
//
// It generates the doc-string file (and, under `-g`, the globals header) from the C sources
// named on its command line; `docs/port-plan.md` 7.1's L1+ needs the file it generates.
//
// **Dead text** (P-08/P-51): `WINDOWSNT` and `DOS_NT` are undefined in the frozen
// configuration, so C 53-58 (`#undef fopen` plus `<direct.h>`) and C 60-67 (`#undef chdir`)
// are preprocessor-only arms with no compiled statement of their own; they are recorded in
// docs/ports/make-docfile.md, not transcribed.  The file's own conditions are decided by
// `python3 tools/check-cond-eval.py lib-src/make-docfile.c`.
//
// **Use-site handling** (P-26/P-72 (3)), because C names these from headers the port has no
// mirror for (or has a mirror that does not carry them): `freopen`, `ungetc`, `fscanf`,
// `vfprintf`, `qsort`, `strstr` and `exit` are declared here where C's translation unit gets
// them; `<stdarg.h>` has no mirror (the `...` parameter *is* the list, P-04);
// `<stddef.h>`'s `ptrdiff_t` and `<stdint.h>`'s `intmax_t` are written out as the frozen
// target's aliases; `<intprops.h>` is unused by this file; `<unlocked-io.h>`'s renames on
// stdio calls are recorded at the use site by every unit that needs them
// (docs/ports/dispnew.md 374, docs/ports/bidi.md 269), so this program calls
// `getc`/`putchar`/`fputs` as C's text spells them; and `DIRECTORY_SEP`/`IS_DIRECTORY_SEP`
// are generated `<config.h>` 120/2097, which the config mirror does not carry (the
// `src/dired.rs` 99-104 and `src/callproc.rs` 292-297 precedent).
//
// **Forward declarations dropped** (PORTING.md 0.1-12): C 69-73 declares five helpers ahead
// of their definitions; Rust items are order-independent, so those declaration lines are
// gone and the definitions stay where C has them.
//
// `ATTRIBUTE_FORMAT_PRINTF`, `ATTRIBUTE_MALLOC`, `NO_INLINE` and `_Noreturn` are attributes
// with no Rust counterpart (PORTING.md 8/P-13); the C names are kept in the comments at
// their definitions, and `_Noreturn` is the port's `-> !`.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]

use std::ffi::{c_char, c_double, c_int, c_long, c_short, c_uchar, c_uint, c_ulong,
              c_void};

// C gets `va_list` from <stdarg.h>; the port's equivalent is core::ffi::VaList.
use core::ffi::VaList;

/// `<stddef.h>`: `typedef ... ptrdiff_t;` — the frozen target's `long` (measured with
/// clang: `sizeof (ptrdiff_t)` is 8).  This program has no `lisp.h` mirror, so it provides
/// the name the way `lib-src/make-fingerprint.rs` does.
pub type ptrdiff_t = isize;

/// `<stdint.h>`: `typedef ... intmax_t;` — the frozen target's 64-bit signed type
/// (measured: `sizeof (intmax_t)` is 8).
pub type intmax_t = i64;

include!("../src/include/config.rs");        // C 37: `#include <config.h>`
include!("../lib/include/stdio.rs");         // C's `<stdio.h>` (`<config.h>`'s conf_post.h 402 reaches it)
include!("../lib/include/stdcountof.rs");    // C 40: `#include <stdcountof.h>`
include!("../lib/include/stdint.in.rs");     // C 42: `#include <stdint.h>`
include!("../lib/include/stdlib.rs");        // C 43: `#include <stdlib.h>`
include!("../lib/include/string.in.rs");        // C 44: `#include <string.h>`
include!("../lib/include/attribute.rs");     // C 46: `#include <attribute.h>`
include!("../lib/include/binary-io.rs");     // C 47: `#include <binary-io.h>` (`set_binary_mode`)
include!("../lib/include/c-ctype.rs");       // C 48: `#include <c-ctype.h>`
include!("../lib/include/min-max.rs");       // C 50: `#include <min-max.h>`
include!("../lib/include/unistd.in.rs");     // C 75: `#include <unistd.h>` (`chdir`)

/* C 39's `#include <stdarg.h>`: Rust passes a C variadic list through the `...` of the
   definitions below and through `va_list` parameters as `core::ffi::VaList` (P-04), so
   nothing is included for it.  C 49's `#include <intprops.h>` names no token this file
   uses.  C 51's `#include <unlocked-io.h>` is the use-site note in the file header.  */

unsafe extern "C" {
  /// `<stdlib.h>`: `void exit (int);` — no mirror carries it yet (the emacs.rs/sysdep.rs
  /// precedent keeps it at the use site).
  pub fn exit (status: c_int) -> !;
  /// `<stdio.h>`: `FILE *freopen (const char *restrict, const char *restrict,
  /// FILE *restrict);`
  pub fn freopen (path: *const c_char, mode: *const c_char, stream: *mut FILE) -> *mut FILE;
  /// `<stdio.h>`: `int ungetc (int c, FILE *stream);`
  pub fn ungetc (c: c_int, stream: *mut FILE) -> c_int;
  /// `<stdio.h>`: `int fscanf (FILE *restrict, const char *restrict, ...);`
  pub fn fscanf (stream: *mut FILE, format: *const c_char, ...) -> c_int;
  /// `<stdio.h>`: `int vfprintf (FILE *restrict, const char *restrict, va_list);` — the
  /// `va_list` parameter is the port's `VaList<'_>` (P-04; the `src/pdumper.rs` 122
  /// precedent).
  pub fn vfprintf (stream: *mut FILE, format: *const c_char, ap: VaList<'_>) -> c_int;
  /// `<string.h>`: `char *strstr (const char *haystack, const char *needle);`
  pub fn strstr (haystack: *const c_char, needle: *const c_char) -> *mut c_char;
}

unsafe extern "C" {
  /// `<stdlib.h>`: `void qsort (void *base, size_t nmemb, size_t size,
  /// int (*compar) (const void *, const void *));` — the comparison function is C's ABI,
  /// so `compare_globals` below is `unsafe extern "C"` too (PORTING.md 1.4).
  pub fn qsort (base: *mut c_void, nmemb: usize, size: usize,
                compar: unsafe extern "C" fn (*const c_void, *const c_void) -> c_int);
}

/// `<config.h>` 120: `#define DIRECTORY_SEP '/'` (the `src/dired.rs` 99 precedent).
const DIRECTORY_SEP: c_char = b'/' as c_char;

/// `<config.h>` 2097: `#define IS_DIRECTORY_SEP(_c_) ((_c_) == DIRECTORY_SEP)`.
#[inline(always)]
fn IS_DIRECTORY_SEP (c: c_char) -> bool { c == DIRECTORY_SEP }

// C 77-78: `static char *progname;`  (the name this program was invoked with)
static mut progname: *mut c_char = std::ptr::null_mut ();

// C 80-81: `static bool generate_globals;`  (true if this invocation generates globals.h)
static mut generate_globals: bool = false;

/* Print error message.  Args are like vprintf.  */

// C 85-86: `static void ATTRIBUTE_FORMAT_PRINTF (1, 0) verror (char const *m, va_list ap)`
// - the attribute has no Rust counterpart (PORTING.md 8), and C's `va_list` parameter is
// the port's `VaList<'_>` (P-04).
unsafe fn verror (m: *const c_char, ap: VaList<'_>)
{
  fprintf (stderr, c"%s: ".as_ptr (), progname);
  vfprintf (stderr, m, ap);
  fprintf (stderr, c"\n".as_ptr ());
}

/* Print error message.  Args are like printf.  */

/* C 95-96: `static void ATTRIBUTE_FORMAT_PRINTF (1, 2) error (char const *m, ...)` - C's
   `va_list ap; va_start (ap, m); va_end (ap);` have no port site: the `...` parameter *is*
   the list (P-04).  */
unsafe extern "C" fn error (m: *const c_char, mut ap: ...)
{
  verror (m, ap);
}

/* Print error message and exit.  Args are like printf.  */

/* C 106-107: `static _Noreturn void ATTRIBUTE_FORMAT_PRINTF (1, 2) fatal (char const *m,
   ...)` - `_Noreturn` is the port's `-> !`, and the `...` parameter is the list (P-04).  */
unsafe extern "C" fn fatal (m: *const c_char, mut ap: ...) -> !
{
  verror (m, ap);
  exit (EXIT_FAILURE);
}

// C 116-117: `static _Noreturn void memory_exhausted (void)`
unsafe fn memory_exhausted () -> !
{
  fatal (c"virtual memory exhausted".as_ptr ());
}

/* Like malloc but get fatal error if memory is exhausted.  */

// C 124-125: `static void * ATTRIBUTE_MALLOC xmalloc (ptrdiff_t size)`
unsafe fn xmalloc (size: ptrdiff_t) -> *mut c_void
{
  let result: *mut c_void = malloc (size as usize);
  if result.is_null ()
    {
      memory_exhausted ();
    }
  return result;
}

/* Like realloc but get fatal error if memory is exhausted.  */

// C 135-136: `static void * xrealloc (void *arg, ptrdiff_t size)`
unsafe fn xrealloc (arg: *mut c_void, size: ptrdiff_t) -> *mut c_void
{
  let result: *mut c_void = realloc (arg, size as usize);
  if result.is_null ()
    {
      memory_exhausted ();
    }
  return result;
}

// C 145: `int main (int argc, char **argv)`.  C's `main` has external linkage, so the port
// keeps the name and the C signature (the `src/emacs.rs` precedent).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main (argc: c_int, argv: *mut *mut c_char) -> c_int
{
  let mut i: c_int;

  progname = *argv;

  /* If first two args are -o FILE, output to FILE.  */
  i = 1;
  if argc > i + 1 && streq (*argv.offset (i as isize), c"-o".as_ptr ())
    {
      if freopen (*argv.offset ((i + 1) as isize), c"w".as_ptr (), stdout).is_null ()
	{
	  perror (*argv.offset ((i + 1) as isize));
	  return EXIT_FAILURE;
	}
      i += 2;
    }
  if argc > i + 1 && streq (*argv.offset (i as isize), c"-a".as_ptr ())
    {
      if freopen (*argv.offset ((i + 1) as isize), c"a".as_ptr (), stdout).is_null ()
	{
	  perror (*argv.offset ((i + 1) as isize));
	  return EXIT_FAILURE;
	}
      i += 2;
    }
  if argc > i + 1 && streq (*argv.offset (i as isize), c"-d".as_ptr ())
    {
      if chdir (*argv.offset ((i + 1) as isize)) != 0
	{
	  perror (*argv.offset ((i + 1) as isize));
	  return EXIT_FAILURE;
	}
      i += 2;
    }
  if argc > i && streq (*argv.offset (i as isize), c"-g".as_ptr ())
    {
      generate_globals = true;
      i += 1;
    }

  set_binary_mode (fileno (stdout), O_BINARY);

  if generate_globals
    {
      start_globals ();
    }

  if argc <= i
    {
      scan_c_stream (stdin);
    }
  else
    {
      let first_infile: c_int = i;
      while i < argc
	{
	  let mut j: c_int;
	  /* Don't process one file twice.  */
	  j = first_infile;
	  while j < i
	    {
	      if streq (*argv.offset (i as isize), *argv.offset (j as isize))
		{
		  break;
		}
	      j += 1;
	    }
	  if j == i
	    {
	      scan_file (*argv.offset (i as isize));
	    }
	  i += 1;
	}
    }

  if generate_globals
    {
      write_globals ();
    }

  if ferror (stdout) != 0 || fclose (stdout) != 0
    {
      fatal (c"write error".as_ptr ());
    }

  return EXIT_SUCCESS;
}

/* Add a source file name boundary marker in the output file.  */

// C 219-220: `static void put_filename (char *filename)`
unsafe fn put_filename (filename: *mut c_char)
{
  let mut filename: *mut c_char = filename;
  let mut tmp: *mut c_char;

  tmp = filename;
  while *tmp != 0
    {
      if IS_DIRECTORY_SEP (*tmp)
	{
	  filename = tmp.offset (1);
	}
      tmp = tmp.offset (1);
    }

  printf (c"\x1fS%s\n".as_ptr (), filename);
}

/* Read file FILENAME and output its doc strings to stdout.
   Return true if file is found, false otherwise.  */

// C 236-237: `static void scan_file (char *filename)`
unsafe fn scan_file (filename: *mut c_char)
{
  if ! generate_globals
    {
      put_filename (filename);
    }
  scan_c_file (filename, c"r".as_ptr ());
}

// C 244-245: `static void start_globals (void)`
unsafe fn start_globals ()
{
  puts (c"/* This file was auto-generated by make-docfile.  */".as_ptr ());
  puts (c"/* DO NOT EDIT.  */".as_ptr ());
  puts (c"struct emacs_globals {".as_ptr ());
}

// C 252: `static char input_buffer[128];`
static mut input_buffer: [c_char; 128] = [0; 128];

/* Some state during the execution of `read_c_string_or_comment'.  */

/* C 255-278: `struct rcsoc_state`.  `intmax_t` is the frozen target's `long` and `bool` is
   one byte (measured: the record is 72 bytes, see the assertions below).  */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcsoc_state
{
  /* A count of spaces and newlines that have been read, but not output.  */
  pub pending_spaces: intmax_t,
  pub pending_newlines: intmax_t,

  /* Where we're reading from.  */
  pub in_file: *mut FILE,

  /* If non-zero, a buffer into which to copy characters.  */
  pub buf_ptr: *mut c_char,
  /* If non-zero, one past the buffer's last byte.  */
  pub buf_lim: *mut c_char,
  /* If non-zero, a file into which to copy characters.  */
  pub out_file: *mut FILE,

  /* A keyword we look for at the beginning of lines.  If found, it is not copied, and
     SAW_KEYWORD is set to true.  */
  pub keyword: *const c_char,
  /* The current point we've reached in an occurrence of KEYWORD in the input stream.  */
  pub cur_keyword_ptr: *const c_char,
  /* Set to true if we saw an occurrence of KEYWORD.  */
  pub saw_keyword: bool,
}

/* Output CH to the file or buffer in STATE.  Any pending newlines or spaces are output
   first.  */

// C 283-284: `static void put_char (char ch, struct rcsoc_state *state)`
unsafe fn put_char (ch: c_char, state: *mut rcsoc_state)
{
  let mut out_ch: c_char;
  loop
    {
      if (*state).pending_newlines > 0
	{
	  (*state).pending_newlines -= 1;
	  out_ch = b'\n' as c_char;
	}
      else if (*state).pending_spaces > 0
	{
	  (*state).pending_spaces -= 1;
	  out_ch = b' ' as c_char;
	}
      else
	{
	  out_ch = ch;
	}

      if ! (*state).out_file.is_null ()
	{
	  putc (out_ch as c_int, (*state).out_file);
	}
      if ! (*state).buf_ptr.is_null ()
	{
	  *(*state).buf_ptr = out_ch;
	  (*state).buf_ptr = (*state).buf_ptr.offset (1);
	  if (*state).buf_lim <= (*state).buf_ptr
	    {
	      fatal (c"state buffer exhausted".as_ptr ());
	    }
	}

      if ! (out_ch != ch)
	{
	  break;
	}
    }
}

/* If in the middle of scanning a keyword, continue scanning with character CH, otherwise
   output CH to the file or buffer in STATE.  ...  */

// C 320-321: `static void scan_keyword_or_put_char (char ch, struct rcsoc_state *state)`
unsafe fn scan_keyword_or_put_char (ch: c_char, state: *mut rcsoc_state)
{
  if ! (*state).keyword.is_null ()
      && *(*state).cur_keyword_ptr == ch
      && ((*state).cur_keyword_ptr > (*state).keyword
	  || (*state).pending_newlines > 0)
    /* We might be looking at STATE->keyword at some point.
       Keep looking until we know for sure.  */
    {
      (*state).cur_keyword_ptr = (*state).cur_keyword_ptr.offset (1);
      if *(*state).cur_keyword_ptr == 0
	/* Saw the whole keyword.  Set SAW_KEYWORD flag to true.  */
	{
	  (*state).saw_keyword = true;

	  /* Reset the scanning pointer.  */
	  (*state).cur_keyword_ptr = (*state).keyword;

	  /* Canonicalize whitespace preceding a usage string.  */
	  (*state).pending_newlines = 2;
	  (*state).pending_spaces = 0;

	  /* Skip any spaces and newlines between the keyword and the usage string.  */
	  let mut c: c_int;
	  loop
	    {
	      c = getc ((*state).in_file);
	      if ! (c == b' ' as c_int || c == b'\n' as c_int)
		{
		  break;
		}
	    }

	  /* Output the open-paren we just read.  */
	  if c != b'(' as c_int
	    {
	      fatal (c"Missing '(' after keyword".as_ptr ());
	    }
	  put_char (c as c_char, state);

	  /* Skip the function name and replace it with `fn'.  */
	  loop
	    {
	      c = getc ((*state).in_file);
	      if c == EOF
		{
		  fatal (c"Unexpected EOF after keyword".as_ptr ());
		}
	      if ! (c != b' ' as c_int && c != b')' as c_int)
		{
		  break;
		}
	    }

	  put_char (b'f' as c_char, state);
	  put_char (b'n' as c_char, state);

	  /* Put back the last character.  */
	  ungetc (c, (*state).in_file);
	}
    }
  else
    {
      if ! (*state).keyword.is_null () && (*state).cur_keyword_ptr > (*state).keyword
	/* We scanned the beginning of a potential usage keyword, but it was a false
	   alarm.  Output the part we scanned.  */
	{
	  let mut p: *const c_char;

	  p = (*state).keyword;
	  while p < (*state).cur_keyword_ptr
	    {
	      put_char (*p, state);
	      p = p.offset (1);
	    }

	  (*state).cur_keyword_ptr = (*state).keyword;
	}

      put_char (ch, state);
    }
}

/* Skip a C string or C-style comment from INFILE, and return the byte that follows, or EOF.
   COMMENT means skip a comment.  Leading '"' or '/','*' have already been read.  ...  */

// C 400-402
unsafe fn read_c_string_or_comment (infile: *mut FILE, printflag: c_int, comment: bool,
				    saw_usage: *mut bool) -> c_int
{
  let mut c: c_int;
  /* C 405's `struct rcsoc_state state;` is assigned field by field below; Rust has no
     partial initialization, so the port starts it zeroed and keeps every assignment
     (PORTING.md 0.1-13/1.5).  */
  let mut state: rcsoc_state = std::mem::zeroed ();

  state.in_file = infile;
  state.buf_ptr = if printflag < 0 { input_buffer.as_mut_ptr () } else { std::ptr::null_mut () };
  state.buf_lim = if printflag < 0
    { input_buffer.as_mut_ptr ().add (std::mem::size_of_val (&input_buffer)) }
    else { std::ptr::null_mut () };
  state.out_file = if printflag <= 0 { std::ptr::null_mut () } else { stdout };
  state.pending_spaces = 0;
  state.pending_newlines = 0;
  state.keyword = if ! saw_usage.is_null () { c"usage:".as_ptr () } else { std::ptr::null () };
  state.cur_keyword_ptr = state.keyword;
  state.saw_keyword = false;

  c = getc (infile);
  if comment
    {
      while c_isspace (c)
	{
	  c = getc (infile);
	}
    }

  while c != EOF
    {
      while c != EOF && (if comment { c != b'*' as c_int } else { c != b'"' as c_int })
	{
	  if c == b'\\' as c_int
	    {
	      c = getc (infile);
	      /* C 429-436's `switch (c)` with `case '\n': case '\r'`, `case 'n'` and
		 `case 't'`: a byte literal is not a Rust pattern, so the arms spell C's
		 characters as their values (the `src/font.rs`/`lib-src/make-fingerprint.rs`
		 precedent).  */
	      match c
		{
		  10 /* C 431: `case '\n':` */ | 13 /* C 431: `case '\r':` */ =>
		    {
		      c = getc (infile);
		      continue;
		    }
		  110 /* C 434: `case 'n':` */ =>
		    {
		      c = b'\n' as c_int;
		    }
		  116 /* C 435: `case 't':` */ =>
		    {
		      c = b'\t' as c_int;
		    }
		  _ =>
		    {
		    }
		}
	    }

	  if c == b' ' as c_int
	    {
	      state.pending_spaces += 1;
	    }
	  else if c == b'\n' as c_int
	    {
	      state.pending_newlines += 1;
	      state.pending_spaces = 0;
	    }
	  else
	    {
	      scan_keyword_or_put_char (c as c_char, &mut state);
	    }

	  c = getc (infile);
	}

      if c != EOF
	{
	  c = getc (infile);
	}

      if comment
	{
	  if c == b'/' as c_int
	    {
	      c = getc (infile);
	      break;
	    }

	  scan_keyword_or_put_char (b'*' as c_char, &mut state);
	}
      else
	{
	  if c != b'"' as c_int
	    {
	      break;
	    }

	  /* If we had a "", concatenate the two strings.  */
	  c = getc (infile);
	}
    }

  if printflag < 0
    {
      *state.buf_ptr = 0;
    }

  if ! saw_usage.is_null ()
    {
      *saw_usage = state.saw_keyword;
    }

  return c;
}

/* Write to stdout the argument names of function FUNC, whose text is in BUF.
   MINARGS and MAXARGS are the minimum and maximum number of arguments.  */

// C 489-490: `static void write_c_args (char *buf, int minargs, int maxargs)`
unsafe fn write_c_args (buf: *mut c_char, minargs: c_int, maxargs: c_int)
{
  let mut buf: *mut c_char = buf;
  let mut minargs: c_int = minargs;
  let mut maxargs: c_int = maxargs;
  let mut p: *mut c_char;
  let mut in_ident: bool = false;
  /* C 494's `char *ident_start UNINIT;`: `UNINIT` is `<conf_post.h>`'s marker, empty in
     the frozen configuration, so the port writes the initialized pointer it stands for
     (PORTING.md 1.5).  */
  let mut ident_start: *mut c_char = std::ptr::null_mut ();
  let mut ident_length: ptrdiff_t = 0;

  fputs (c"(fn".as_ptr (), stdout);

  if *buf == b'(' as c_char
    {
      buf = buf.offset (1);
    }

  p = buf;
  while *p != 0
    {
      let mut c: c_char = *p;

      /* Notice when a new identifier starts.  */
      if (c_isalnum (c as c_int) || c == b'_' as c_char) != in_ident
	{
	  if ! in_ident
	    {
	      in_ident = true;
	      ident_start = p;
	    }
	  else
	    {
	      in_ident = false;
	      ident_length = p.offset_from (ident_start);
	    }
	}

      /* Found the end of an argument, write out the last seen identifier.  */
      if c == b',' as c_char || c == b')' as c_char
	{
	  if ident_length == 0
	    {
	      p = p.offset (1);
	      continue;
	    }

	  if ident_length == 4
	      && memeq (ident_start as *const c_void, c"void".as_ptr () as *const c_void, 4)
	    {
	      p = p.offset (1);
	      continue;
	    }

	  putchar (b' ' as c_int);

	  if minargs == 0 && maxargs > 0
	    {
	      fputs (c"&optional ".as_ptr (), stdout);
	    }

	  minargs -= 1;
	  maxargs -= 1;

	  /* In C code, `default' is a reserved word, so we spell it `defalt'; demangle that
	     here.  */
	  if ident_length == 6
	      && memcmp (ident_start as *const c_void, c"defalt".as_ptr () as *const c_void, 6)
		 == 0
	    {
	      fputs (c"DEFAULT".as_ptr (), stdout);
	    }
	  else
	    {
	      /* C's `while (ident_length-- > 0)` decrements on the failing test too, and the
		 trailing `ident_length = 0;` puts it back.  */
	      loop
		{
		  let old: ptrdiff_t = ident_length;
		  ident_length -= 1;
		  if ! (old > 0)
		    {
		      break;
		    }
		  c = c_toupper (*ident_start as c_int) as c_char;
		  ident_start = ident_start.offset (1);
		  if c == b'_' as c_char
		    /* Print underscore as hyphen.  */
		    {
		      c = b'-' as c_char;
		    }
		  putchar (c as c_int);
		}
	    }

	  ident_length = 0;
	}

      p = p.offset (1);
    }

  putchar (b')' as c_int);
}

/* The types of globals.  These are sorted roughly in decreasing alignment order to avoid
   allocation gaps, except that symbols and functions are last.  */

// C 564-572: `enum global_type`
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum global_type
{
  INVALID,
  LISP_OBJECT,
  EMACS_INTEGER,
  BOOLEAN,
  SYMBOL,
  FUNCTION,
}

/* A single global.  */

// C 575-585: `struct global`
#[repr(C)]
#[derive(Copy, Clone)]
pub struct global
{
  pub r#type: global_type,
  pub name: *mut c_char,
  pub flags: c_int,
  pub v: global_v,
}

/* C 580-584: the anonymous `union { int value; char const *svalue; } v` member; C's
   anonymous union is named here because Rust needs the type (PORTING.md 1.3).  */
#[repr(C)]
#[derive(Copy, Clone)]
pub union global_v
{
  pub value: c_int,
  pub svalue: *const c_char,
}

/* Bit values for FLAGS field from the above.  Applied for DEFUNs only.  */

// C 588-594: `enum { DEFUN_noreturn = 1, DEFUN_const = 2, DEFUN_noinline = 4,
// DEFUN_pure = 8, };`
pub const DEFUN_noreturn: c_int = 1;
pub const DEFUN_const: c_int = 2;
pub const DEFUN_noinline: c_int = 4;
pub const DEFUN_pure: c_int = 8;

/* The two records above are file-local (no header baseline holds them), so the layout is
   measured: `clang` compiled the same declarations and printed
   `global 32 {type 0, name 8, flags 16, v 24}` and
   `rcsoc_state 72 {pending_spaces 0, pending_newlines 8, in_file 16, buf_ptr 24,
   buf_lim 32, out_file 40, keyword 48, cur_keyword_ptr 56, saw_keyword 64}`
   (docs/ports/make-docfile.md records the probe).  */
const _: () = core::assert! (std::mem::size_of::<global> () == 32);
const _: () = core::assert! (std::mem::offset_of! (global, r#type) == 0);
const _: () = core::assert! (std::mem::offset_of! (global, name) == 8);
const _: () = core::assert! (std::mem::offset_of! (global, flags) == 16);
const _: () = core::assert! (std::mem::offset_of! (global, v) == 24);
const _: () = core::assert! (std::mem::size_of::<global_v> () == 8);
const _: () = core::assert! (std::mem::size_of::<rcsoc_state> () == 72);
const _: () = core::assert! (std::mem::offset_of! (rcsoc_state, pending_spaces) == 0);
const _: () = core::assert! (std::mem::offset_of! (rcsoc_state, pending_newlines) == 8);
const _: () = core::assert! (std::mem::offset_of! (rcsoc_state, in_file) == 16);
const _: () = core::assert! (std::mem::offset_of! (rcsoc_state, buf_ptr) == 24);
const _: () = core::assert! (std::mem::offset_of! (rcsoc_state, buf_lim) == 32);
const _: () = core::assert! (std::mem::offset_of! (rcsoc_state, out_file) == 40);
const _: () = core::assert! (std::mem::offset_of! (rcsoc_state, keyword) == 48);
const _: () = core::assert! (std::mem::offset_of! (rcsoc_state, cur_keyword_ptr) == 56);
const _: () = core::assert! (std::mem::offset_of! (rcsoc_state, saw_keyword) == 64);

/* All the variable names we saw while scanning C sources in `-g' mode.  */

// C 598-600
static mut num_globals: ptrdiff_t = 0;
static mut num_globals_allocated: ptrdiff_t = 0;
static mut globals: *mut global = std::ptr::null_mut ();

// C 602-604: `static struct global *add_global (enum global_type type, char const *name,
// int value, char const *svalue)`
unsafe fn add_global (r#type: global_type, name: *const c_char, value: c_int,
		      svalue: *const c_char) -> *mut global
{
  /* Ignore the one non-symbol that can occur.  */
  if ! streq (name, c"...".as_ptr ())
    {
      if num_globals == num_globals_allocated
	{
	  let num_globals_max: ptrdiff_t =
	    (min! (PTRDIFF_MAX as usize, SIZE_MAX) / std::mem::size_of::<global> ())
	    as ptrdiff_t;
	  if num_globals_allocated == num_globals_max
	    {
	      memory_exhausted ();
	    }
	  if num_globals_allocated < num_globals_max / 2
	    {
	      num_globals_allocated = 2 * num_globals_allocated + 1;
	    }
	  else
	    {
	      num_globals_allocated = num_globals_max;
	    }
	  globals = xrealloc (globals as *mut c_void,
			      num_globals_allocated * std::mem::size_of::<global> ()
			      as ptrdiff_t) as *mut global;
	}

      num_globals += 1;

      let namesize: ptrdiff_t = strlen (name) as ptrdiff_t + 1;
      let buf: *mut c_char = xmalloc (namesize + (if ! svalue.is_null ()
						     { strlen (svalue) as ptrdiff_t + 1 }
						   else { 0 })) as *mut c_char;
      (*globals.add ((num_globals - 1) as usize)).r#type = r#type;
      (*globals.add ((num_globals - 1) as usize)).name = strcpy (buf, name);
      if ! svalue.is_null ()
	{
	  (*globals.add ((num_globals - 1) as usize)).v.svalue =
	    strcpy (buf.offset (namesize), svalue);
	}
      else
	{
	  (*globals.add ((num_globals - 1) as usize)).v.value = value;
	}
      (*globals.add ((num_globals - 1) as usize)).flags = 0;
      return globals.add ((num_globals - 1) as usize);
    }
  return std::ptr::null_mut ();
}

/* C 638-639: `static int compare_globals (const void *a, const void *b)`.  C 691 passes its
   address to `qsort`, so it is `unsafe extern "C"` (PORTING.md 1.4).  */
unsafe extern "C" fn compare_globals (a: *const c_void, b: *const c_void) -> c_int
{
  let ga: *const global = a as *const global;
  let gb: *const global = b as *const global;

  if (*ga).r#type != (*gb).r#type
    {
      return (*ga).r#type as c_int - (*gb).r#type as c_int;
    }

  /* Consider "nil" to be the least, so that iQnil is zero.  That way, Qnil's internal
     representation is zero, which is a bit faster.  Similarly, consider "t" to be the
     second-least, and so forth.  */
  if (*ga).r#type == global_type::SYMBOL
    {
      /* Common symbols in decreasing popularity order.  */
      static commonsym: [[u8; 8]; 5] =
	[ *b"nil\0\0\0\0\0", *b"t\0\0\0\0\0\0\0", *b"unbound\0", *b"error\0\0\0",
	  *b"lambda\0\0" ];
      let ncommonsym: c_int = countof! (commonsym) as c_int;
      let mut ai: c_int = ncommonsym;
      let mut bi: c_int = ncommonsym;
      let mut i: c_int = 0;
      while i < ncommonsym
	{
	  if *(*ga).name == b'Q' as c_char
	      && streq ((*ga).name.offset (1),
			commonsym[i as usize].as_ptr () as *const c_char)
	    {
	      ai = i;
	    }
	  if *(*gb).name == b'Q' as c_char
	      && streq ((*gb).name.offset (1),
			commonsym[i as usize].as_ptr () as *const c_char)
	    {
	      bi = i;
	    }
	  i += 1;
	}
      if ! (ai == ncommonsym && bi == ncommonsym)
	{
	  return ai - bi;
	}
    }

  return strcmp ((*ga).name, (*gb).name);
}

// C 671-672: `static void close_emacs_globals (ptrdiff_t num_symbols)`
unsafe fn close_emacs_globals (num_symbols: ptrdiff_t)
{
  printf (c"};\nextern struct emacs_globals globals;\n\nextern struct Lisp_Symbol lispsym[%td];\n#ifdef DEFINE_SYMBOLS\nstruct Lisp_Symbol lispsym[%td];\n#endif\n".as_ptr (),
	  num_symbols, num_symbols);
}

// C 683-685: `static void write_globals (void)`
unsafe fn write_globals ()
{
  let mut i: ptrdiff_t;
  let mut j: ptrdiff_t;
  let mut seen_defun: bool = false;
  let mut symnum: ptrdiff_t = 0;
  let mut num_symbols: ptrdiff_t = 0;
  qsort (globals as *mut c_void, num_globals as usize, std::mem::size_of::<global> (),
	 compare_globals);

  j = 0;
  i = 0;
  while i < num_globals
    {
      while i + 1 < num_globals
	  && streq ((*globals.add (i as usize)).name,
		    (*globals.add ((i + 1) as usize)).name)
	{
	  if (*globals.add (i as usize)).r#type == global_type::FUNCTION
	      && (*globals.add (i as usize)).v.value
		 != (*globals.add ((i + 1) as usize)).v.value
	    {
	      error (c"function '%s' defined twice with differing signatures".as_ptr (),
		     (*globals.add (i as usize)).name);
	    }
	  free ((*globals.add (i as usize)).name as *mut c_void);
	  i += 1;
	}
      num_symbols += ((*globals.add (i as usize)).r#type == global_type::SYMBOL) as ptrdiff_t;
      *globals.add (j as usize) = *globals.add (i as usize);
      j += 1;
      i += 1;
    }
  num_globals = j;

  i = 0;
  while i < num_globals
    {
      let mut r#type: *const c_char = std::ptr::null ();

      match (*globals.add (i as usize)).r#type
	{
	  global_type::EMACS_INTEGER =>
	    {
	      r#type = c"intmax_t".as_ptr ();
	    }
	  global_type::BOOLEAN =>
	    {
	      r#type = c"bool".as_ptr ();
	    }
	  global_type::LISP_OBJECT =>
	    {
	      r#type = c"Lisp_Object".as_ptr ();
	    }
	  global_type::SYMBOL | global_type::FUNCTION =>
	    {
	      if ! seen_defun
		{
		  close_emacs_globals (num_symbols);
		  putchar (b'\n' as c_int);
		  seen_defun = true;
		}
	    }
	  _ =>
	    {
	      fatal (c"not a recognized DEFVAR_".as_ptr ());
	    }
	}

      if ! r#type.is_null ()
	{
	  printf (c"  %s f_%s;\n".as_ptr (), r#type, (*globals.add (i as usize)).name);
	  printf (c"#define %s globals.f_%s\n".as_ptr (), (*globals.add (i as usize)).name,
		  (*globals.add (i as usize)).name);
	}
      else if (*globals.add (i as usize)).r#type == global_type::SYMBOL
	{
	  printf (c"#define i%s %td\nDEFINE_LISP_SYMBOL (%s)\n".as_ptr (),
		  (*globals.add (i as usize)).name, symnum,
		  (*globals.add (i as usize)).name);
	  symnum += 1;
	}
      else
	{
	  if ((*globals.add (i as usize)).flags & DEFUN_noreturn) != 0
	    {
	      fputs (c"_Noreturn ".as_ptr (), stdout);
	    }
	  if ((*globals.add (i as usize)).flags & DEFUN_noinline) != 0
	    {
	      fputs (c"NO_INLINE ".as_ptr (), stdout);
	    }

	  printf (c"EXFUN (%s, ".as_ptr (), (*globals.add (i as usize)).name);
	  if (*globals.add (i as usize)).v.value == -1
	    {
	      fputs (c"MANY".as_ptr (), stdout);
	    }
	  else if (*globals.add (i as usize)).v.value == -2
	    {
	      fputs (c"UNEVALLED".as_ptr (), stdout);
	    }
	  else
	    {
	      printf (c"%d".as_ptr (), (*globals.add (i as usize)).v.value);
	    }
	  putchar (b')' as c_int);

	  if ((*globals.add (i as usize)).flags & DEFUN_noreturn) != 0
	    {
	      fputs (c" ATTRIBUTE_COLD".as_ptr (), stdout);
	    }
	  if ((*globals.add (i as usize)).flags & DEFUN_const) != 0
	    {
	      fputs (c" ATTRIBUTE_CONST".as_ptr (), stdout);
	    }
	  if ((*globals.add (i as usize)).flags & DEFUN_pure) != 0
	    {
	      fputs (c" ATTRIBUTE_PURE".as_ptr (), stdout);
	    }

	  puts (c";".as_ptr ());
	}

      i += 1;
    }

  if ! seen_defun
    {
      close_emacs_globals (num_symbols);
    }

  puts (c"#ifdef DEFINE_SYMBOLS".as_ptr ());
  puts (c"static char const *const defsym_name[] = {".as_ptr ());
  let mut i: ptrdiff_t = 0;
  while i < num_globals
    {
      if (*globals.add (i as usize)).r#type == global_type::SYMBOL
	{
	  printf (c"\t\"%s\",\n".as_ptr (), (*globals.add (i as usize)).v.svalue);
	}
      i += 1;
    }
  puts (c"};".as_ptr ());
  puts (c"#endif".as_ptr ());

  puts (c"#define Qnil builtin_lisp_symbol (0)".as_ptr ());
  puts (c"#if DEFINE_NON_NIL_Q_SYMBOL_MACROS".as_ptr ());
  num_symbols = 0;
  let mut i: ptrdiff_t = 0;
  while i < num_globals
    {
      if (*globals.add (i as usize)).r#type == global_type::SYMBOL
	{
	  let old: ptrdiff_t = num_symbols;
	  num_symbols += 1;
	  if old != 0
	    {
	      printf (c"# define %s builtin_lisp_symbol (%td)\n".as_ptr (),
		      (*globals.add (i as usize)).name, num_symbols - 1);
	    }
	}
      i += 1;
    }
  puts (c"#endif".as_ptr ());
}

/* Read through a c file.  If a .o file is named, the corresponding .c or .m file is read
   instead.  Looks for DEFUN constructs such as are defined in ../src/lisp.h.  Accepts any
   word starting DEF... so it finds DEFSIMPLE and DEFPRED.  */

// C 803-804: `static void scan_c_file (char *filename, const char *mode)`
unsafe fn scan_c_file (filename: *mut c_char, mode: *const c_char)
{
  let mut infile: *mut FILE;
  let extension: c_char = *filename.offset (strlen (filename) as isize - 1);

  if extension == b'o' as c_char
    {
      *filename.offset (strlen (filename) as isize - 1) = b'c' as c_char;
    }

  infile = fopen (filename, mode);

  if infile.is_null () && extension == b'o' as c_char
    {
      /* Try .m.  */
      *filename.offset (strlen (filename) as isize - 1) = b'm' as c_char;
      infile = fopen (filename, mode);
      if infile.is_null ()
	{
	  *filename.offset (strlen (filename) as isize - 1) = b'c' as c_char;
	}
    }

  if infile.is_null ()
    {
      perror (filename);
      exit (EXIT_FAILURE);
    }

  /* Reset extension to be able to detect duplicate files.  */
  *filename.offset (strlen (filename) as isize - 1) = extension;
  scan_c_stream (infile);
}

/* Return 1 if next input from INFILE is equal to P, -1 if EOF, 0 if input doesn't match.  */

// C 837-838: `static int stream_match (FILE *infile, const char *p)`
unsafe fn stream_match (infile: *mut FILE, p: *const c_char) -> c_int
{
  let mut p: *const c_char = p;
  while *p != 0
    {
      let c: c_int = getc (infile);
      if c == EOF
	{
	  return -1;
	}
      if c != *p as c_int
	{
	  return 0;
	}
      p = p.offset (1);
    }
  return 1;
}

/* C 852-1231: the DEFVAR_/DEFUN scanner.  C's `eof:` label and the forward `goto eof` that
   leaves the loop nest for the cleanup at the end become the labelled `'eof` loop
   (PORTING.md 2.2: a forward cleanup jump is a labelled block, never an early `return` that
   would skip `fclose`/`ferror`); the `continue` statements of C's `while (!feof (infile))`
   are `continue 'eof`.  */
unsafe fn scan_c_stream (infile: *mut FILE)
{
  // C 854-855: `int commas, minargs, maxargs;` - zero-initialized (PORTING.md 1.5); every
  // use below assigns them first.
  let mut commas: c_int = 0;
  let mut minargs: c_int = 0;
  let mut maxargs: c_int = 0;
  let mut c: c_int = b'\n' as c_int;

  'eof: loop
    {
      if feof (infile) != 0
	{
	  break 'eof;
	}

      let mut doc_keyword: bool = false;
      let mut defunflag: bool = false;
      let mut defvarperbufferflag: bool = false;
      let mut defvarflag: bool = false;
      let mut r#type: global_type = global_type::INVALID;
      /* C 869: `static char name[sizeof input_buffer];` - the port names the measured size
	 (128), because a `static`'s length cannot come from another item's `size_of`, which
	 is not a const fn.  */
      static mut name: [c_char; 128] = [0; 128];

      if c != b'\n' as c_int && c != b'\r' as c_int
	{
	  c = getc (infile);
	  continue 'eof;
	}
      c = getc (infile);
      if c == b' ' as c_int
	{
	  while c == b' ' as c_int
	    {
	      c = getc (infile);
	    }
	  if c != b'D' as c_int
	    {
	      continue 'eof;
	    }
	  c = getc (infile);
	  if c != b'E' as c_int
	    {
	      continue 'eof;
	    }
	  c = getc (infile);
	  if c != b'F' as c_int
	    {
	      continue 'eof;
	    }
	  c = getc (infile);
	  if c == b'S' as c_int
	    {
	      c = getc (infile);
	      if c != b'Y' as c_int
		{
		  continue 'eof;
		}
	      c = getc (infile);
	      if c != b'M' as c_int
		{
		  continue 'eof;
		}
	      c = getc (infile);
	      if c != b' ' as c_int && c != b'\t' as c_int && c != b'(' as c_int
		{
		  continue 'eof;
		}
	      r#type = global_type::SYMBOL;
	    }
	  else if c == b'V' as c_int
	    {
	      c = getc (infile);
	      if c != b'A' as c_int
		{
		  continue 'eof;
		}
	      c = getc (infile);
	      if c != b'R' as c_int
		{
		  continue 'eof;
		}
	      c = getc (infile);
	      if c != b'_' as c_int
		{
		  continue 'eof;
		}

	      defvarflag = true;

	      c = getc (infile);
	      defvarperbufferflag = c == b'P' as c_int;
	      if generate_globals
		{
		  if c == b'I' as c_int
		    {
		      r#type = global_type::EMACS_INTEGER;
		    }
		  else if c == b'L' as c_int
		    {
		      r#type = global_type::LISP_OBJECT;
		    }
		  else if c == b'B' as c_int
		    {
		      r#type = global_type::BOOLEAN;
		    }
		}

	      c = getc (infile);
	      /* We need to distinguish between DEFVAR_BOOL and DEFVAR_BUFFER_DEFAULTS.  */
	      if generate_globals && r#type == global_type::BOOLEAN && c != b'O' as c_int
		{
		  r#type = global_type::INVALID;
		}
	    }
	  else
	    {
	      continue 'eof;
	    }
	}
      else if c == b'D' as c_int
	{
	  c = getc (infile);
	  if c != b'E' as c_int
	    {
	      continue 'eof;
	    }
	  c = getc (infile);
	  if c != b'F' as c_int
	    {
	      continue 'eof;
	    }
	  c = getc (infile);
	  defunflag = c == b'U' as c_int;
	}
      else
	{
	  continue 'eof;
	}

      if generate_globals
	  && (! defvarflag || defvarperbufferflag || r#type == global_type::INVALID)
	  && ! defunflag && r#type != global_type::SYMBOL
	{
	  continue 'eof;
	}

      while c != b'(' as c_int
	{
	  if c < 0
	    {
	      break 'eof;
	    }
	  c = getc (infile);
	}

      if r#type != global_type::SYMBOL
	{
	  /* Lisp variable or function name.  */
	  c = getc (infile);
	  if c != b'"' as c_int
	    {
	      continue 'eof;
	    }
	  c = read_c_string_or_comment (infile, -1, false, std::ptr::null_mut ());
	}

      if generate_globals
	{
	  let mut i: ptrdiff_t = 0;
	  let mut svalue: *const c_char = std::ptr::null ();

	  /* Skip "," and whitespace.  */
	  loop
	    {
	      c = getc (infile);
	      if ! (c == b',' as c_int || c_isspace (c))
		{
		  break;
		}
	    }

	  /* Read in the identifier.  */
	  loop
	    {
	      if c < 0
		{
		  break 'eof;
		}
	      *input_buffer.as_mut_ptr ().offset (i as isize) = c as c_char;
	      i += 1;
	      if std::mem::size_of_val (&input_buffer) <= i as usize
		{
		  fatal (c"identifier too long".as_ptr ());
		}
	      c = getc (infile);
	      if c == b',' as c_int || c_isspace (c)
		{
		  break;
		}
	    }

	  *input_buffer.as_mut_ptr ().offset (i as isize) = 0;
	  memcpy (name.as_mut_ptr () as *mut c_void, input_buffer.as_ptr () as *const c_void,
		  (i + 1) as usize);

	  if r#type == global_type::SYMBOL
	    {
	      loop
		{
		  c = getc (infile);
		  if ! c_isspace (c)
		    {
		      break;
		    }
		}

	      if c != b'"' as c_int
		{
		  continue 'eof;
		}
	      c = read_c_string_or_comment (infile, -1, false, std::ptr::null_mut ());
	      svalue = input_buffer.as_mut_ptr ();
	    }

	  if ! defunflag
	    {
	      add_global (r#type, name.as_mut_ptr (), 0, svalue);
	      continue 'eof;
	    }
	}

      if r#type == global_type::SYMBOL
	{
	  continue 'eof;
	}

      /* DEFVAR_LISP ("name", addr, "doc")
	 DEFVAR_LISP ("name", addr /\* doc *\/)
	 DEFVAR_LISP ("name", addr, doc: /\* doc *\/)  */

      if defunflag
	{
	  commas = if generate_globals { 4 } else { 5 };
	}
      else if defvarperbufferflag
	{
	  commas = 3;
	}
      else if defvarflag
	{
	  commas = 1;
	}
      else  /* For DEFSIMPLE and DEFPRED.  */
	{
	  commas = 2;
	}

      while commas != 0
	{
	  if c == b',' as c_int
	    {
	      commas -= 1;

	      if defunflag && (commas == 1 || commas == 2)
		{
		  let mut scanned: c_int = 0;
		  loop
		    {
		      c = getc (infile);
		      if ! c_isspace (c)
			{
			  break;
			}
		    }

		  if c < 0
		    {
		      break 'eof;
		    }
		  ungetc (c, infile);
		  if commas == 2  /* Pick up minargs.  */
		    {
		      scanned = fscanf (infile, c"%d".as_ptr (), &mut minargs);
		    }
		  else  /* Pick up maxargs.  */
		    {
		      if c == b'M' as c_int || c == b'U' as c_int  /* MANY || UNEVALLED */
			{
			  if generate_globals
			    {
			      maxargs = if c == b'M' as c_int { -1 } else { -2 };
			    }
			  else
			    {
			      maxargs = -1;
			    }
			}
		      else
			{
			  scanned = fscanf (infile, c"%d".as_ptr (), &mut maxargs);
			}
		    }
		  if scanned < 0
		    {
		      break 'eof;
		    }
		}
	    }

	  if c == EOF
	    {
	      break 'eof;
	    }
	  c = getc (infile);
	}

      if generate_globals
	{
	  let g: *mut global = add_global (global_type::FUNCTION, name.as_mut_ptr (), maxargs,
					   std::ptr::null ());
	  if g.is_null ()
	    {
	      continue 'eof;
	    }

	  /* The following code tries to recognize function attributes specified after the
	     docstring, e.g.:

	     DEFUN ("foo", Ffoo, Sfoo, X, Y, Z,
		   doc: /\* doc *\/
		   attributes: attribute1 attribute2 ...)
	       (Lisp_Object arg...)

	     Now only 'const', 'noinline', 'noreturn', and 'pure' attributes are used.  */

	  /* Advance to the end of docstring.  */
	  c = getc (infile);
	  if c == EOF
	    {
	      break 'eof;
	    }
	  let mut d: c_int = getc (infile);
	  if d == EOF
	    {
	      break 'eof;
	    }
	  loop
	    {
	      if c == b'*' as c_int && d == b'/' as c_int
		{
		  break;
		}
	      c = d;
	      d = getc (infile);
	      if d == EOF
		{
		  break 'eof;
		}
	    }
	  /* Skip spaces, if any.  */
	  loop
	    {
	      c = getc (infile);
	      if c == EOF
		{
		  break 'eof;
		}
	      if ! c_isspace (c)
		{
		  break;
		}
	    }

	  /* Check for 'attributes:' token.  */
	  if c == b'a' as c_int && stream_match (infile, c"ttributes:".as_ptr ()) != 0
	    {
	      let mut p: *mut c_char = input_buffer.as_mut_ptr ();
	      /* Collect attributes up to ')'.  */
	      loop
		{
		  c = getc (infile);
		  if c == EOF
		    {
		      break 'eof;
		    }
		  if c == b')' as c_int
		    {
		      break;
		    }
		  if input_buffer.as_mut_ptr ().add (std::mem::size_of_val (&input_buffer)) <= p
		    {
		      fatal (c"attribute buffer exhausted".as_ptr ());
		    }
		  *p = c as c_char;
		  p = p.offset (1);
		}
	      *p = 0;
	      if ! strstr (input_buffer.as_ptr (), c"noreturn".as_ptr ()).is_null ()
		{
		  (*g).flags |= DEFUN_noreturn;
		}
	      if ! strstr (input_buffer.as_ptr (), c"const".as_ptr ()).is_null ()
		{
		  (*g).flags |= DEFUN_const;
		}
	      if ! strstr (input_buffer.as_ptr (), c"pure".as_ptr ()).is_null ()
		{
		  (*g).flags |= DEFUN_pure;
		}

	      /* Although the noinline attribute is no longer used, leave its support in, in
		 case it's needed later.  */
	      if ! strstr (input_buffer.as_ptr (), c"noinline".as_ptr ()).is_null ()
		{
		  (*g).flags |= DEFUN_noinline;
		}
	    }
	  continue 'eof;
	}

      while c_isspace (c)
	{
	  c = getc (infile);
	}

      if c == b'"' as c_int
	{
	  c = read_c_string_or_comment (infile, 0, false, std::ptr::null_mut ());
	}

      while c != EOF && c != b',' as c_int && c != b'/' as c_int
	{
	  c = getc (infile);
	}
      if c == b',' as c_int
	{
	  loop
	    {
	      c = getc (infile);
	      if ! c_isspace (c)
		{
		  break;
		}
	    }

	  while c_isalpha (c)
	    {
	      c = getc (infile);
	    }
	  if c == b':' as c_int
	    {
	      doc_keyword = true;
	      loop
		{
		  c = getc (infile);
		  if ! c_isspace (c)
		    {
		      break;
		    }
		}
	    }
	}

      let comment: bool = c == b'/' as c_int && { c = getc (infile); c == b'*' as c_int };
      if comment || c == b'"' as c_int
	{
	  let mut saw_usage: bool = false;

	  printf (c"\x1f%c%s\n".as_ptr (),
		  if defvarflag { b'V' as c_int } else { b'F' as c_int },
		  input_buffer.as_ptr ());

	  c = read_c_string_or_comment (infile, 1, comment, &mut saw_usage);

	  /* If this is a defun, find the arguments and print them.  If this function takes
	     MANY or UNEVALLED args, then the C source won't give the names of the
	     arguments, so we shouldn't bother trying to find them.

	     Various doc-string styles:
	      0: DEFUN (..., "DOC") (args)            [!comment]
	      1: DEFUN (..., /\* DOC *\/ (args))      [comment && !doc_keyword]
	      2: DEFUN (..., doc: /\* DOC *\/) (args) [comment && doc_keyword]
	  */
	  if defunflag && maxargs != -1 && ! saw_usage
	    {
	      let mut argbuf: [c_char; 1024] = [0; 1024];
	      let mut p: *mut c_char = argbuf.as_mut_ptr ();

	      if ! comment || doc_keyword
		{
		  while c != b')' as c_int
		    {
		      if c < 0
			{
			  break 'eof;
			}
		      c = getc (infile);
		    }
		}

	      /* Skip into arguments.  */
	      while c != b'(' as c_int
		{
		  if c < 0
		    {
		      break 'eof;
		    }
		  c = getc (infile);
		}
	      /* Copy arguments into ARGBUF.  */
	      let mut nested: ptrdiff_t = 0;
	      loop
		{
		  *p = c as c_char;
		  p = p.offset (1);
		  if argbuf.as_mut_ptr ().add (std::mem::size_of_val (&argbuf)) <= p
		    {
		      fatal (c"argument buffer exhausted".as_ptr ());
		    }
		  nested += (c == b'(' as c_int) as ptrdiff_t
			    - (c == b')' as c_int) as ptrdiff_t;
		  if c == b')' as c_int && nested == 0
		    {
		      break;
		    }
		  c = getc (infile);
		  if c < 0
		    {
		      break 'eof;
		    }
		}
	      *p = 0;
	      /* Output them.  */
	      fputs (c"\n\n".as_ptr (), stdout);
	      write_c_args (argbuf.as_mut_ptr (), minargs, maxargs);
	    }
	  else if defunflag && maxargs == -1 && ! saw_usage
	    /* The DOC should provide the usage form.  */
	    {
	      fprintf (stderr, c"Missing 'usage' for function '%s'.\n".as_ptr (),
		       input_buffer.as_ptr ());
	    }
	}
    }

  // C 1230-1231 (`eof:`): `if (ferror (infile) || fclose (infile) != 0) fatal ("read error");`
  if ferror (infile) != 0 || fclose (infile) != 0
    {
      fatal (c"read error".as_ptr ());
    }
}
