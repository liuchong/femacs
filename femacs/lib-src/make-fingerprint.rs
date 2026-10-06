// femacs: port of lib-src/make-fingerprint.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names, the same `main`.
// This is a program of the C tree's lib-src directory (batch F), not a translation unit of the
// the reference core: it links against the port's gnulib mirrors and libSystem only.  The frozen
// configuration builds it (`make -n` in `build/emacs-ref/lib-src` lists make-fingerprint.c, and
// the reference directory holds the binary); docs/ports/make-fingerprint.md records that.
//
// **Dead text** (P-08): C 51-61's `WINDOWSNT` arm (the `fopen`->`sys_fopen` note, `<direct.h>`,
// the `fseeko`->`fseeko64` pair) and C 45-47's `#ifndef SSIZE_MAX` fallback (the frozen
// `<limits.h>` defines SSIZE_MAX) are recorded in the ledger, not transcribed.
//
// **Use-site handling**, because three C names have no mirror:
//  * `<intprops.h>`'s `TYPE_MAXIMUM (off_t)` (C 118): the port writes `off_t::MAX`, and
//    docs/ports/macro-body-notes.tsv carries the exemption row for the missing token;
//  * `FOPEN_BINARY` (C 113): the sysstdio.h mirror has it as a `&CStr` constant, and it is the
//    empty string on this target, so C's `"r" FOPEN_BINARY` is the literal `"r"` - a
//    concatenation cannot name a constant, and the same notes file records that;
//  * C 52's `<unlocked-io.h>`: the renames it performs on stdio calls are recorded at the use
//    site by every unit that needs them (docs/ports/dispnew.md 374, docs/ports/bidi.md 269), so
//    this unit calls `fread`/`fwrite`/`fseeko` as C's text spells them.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]

// The C names this unit spells.  It does not include the `lisp.h` mirror - C's
// make-fingerprint.c does not include lisp.h - so the prelude comes from here, the way
// `femacs/lib/regex.rs` carries `use std::ffi::c_int;`.
use std::ffi::{c_char, c_double, c_int, c_long, c_short, c_uchar, c_uint, c_ulong,
              c_void};

/// `<stddef.h>`'s `ptrdiff_t`, which this program gets from the system header (C's
/// `<stdio.h>` chain) rather than from a `lisp.h` mirror: the core units take the same
/// name from `femacs/src/include/lisp.rs`, and `femacs/src/include/sysstdio.rs` (C 38's
/// include) spells it, so this unit has to provide it (measured: `sizeof (ptrdiff_t)` 8).
pub type ptrdiff_t = isize;

include!("../src/include/config.rs");           // C 28: `#include <config.h>`
include!("../lib/include/limits.in.rs");        // C 30: `#include <limits.h>`
include!("../lib/include/stdint.in.rs");        // C 32: `#include <stdint.h>`
include!("../lib/include/stdio.rs");            // C 33: `#include <stdio.h>`
include!("../lib/include/stdlib.rs");           // C 34: `#include <stdlib.h>`
include!("../lib/include/string.rs");           // C 35: `#include <string.h>`
include!("../lib/include/sys_stat.in.rs");      // C 36: `#include <sys/stat.h>`
include!("../src/include/sysstdio.rs");         // C 38: `#include <sysstdio.h>`
include!("../lib/include/fingerprint.rs");      // C 40: `#include <fingerprint.h>`
include!("../lib/include/getopt.in.rs");        // C 41: `#include <getopt.h>`
include!("../lib/include/min-max.rs");          // C 43: `#include <min-max.h>`
include!("../lib/include/sha256.rs");           // C 44: `#include <sha256.h>`
include!("../lib/include/sys_types.in.rs");     // off_t, for C 118's `TYPE_MAXIMUM (off_t)`

// C 31's `#include <stdarg.h>` has no mirror: Rust passes a C variadic list through the
// declared `...` of the libc entry points (`stdio.rs`'s `printf`/`fprintf`), so nothing is
// included for it here.  C 42's `#include <intprops.h>` is likewise unused: its only tokens are
// the two `TYPE_MAXIMUM` uses handled below.

/* `PTRDIFF_MAX` and `SIZE_MAX` come from the `stdint.in.rs` mirror that C 32 includes
   (`isize::MAX` and `usize::MAX`), and `SSIZE_MAX` from the `limits.in.rs` mirror C 30
   includes: C 45-47's `#ifndef` is dead on the frozen target, whose `<limits.h>` defines it
   as `LONG_MAX` (`clang` against the frozen SDK prints `9223372036854775807`).  */

/// `<stdio.h>`'s `#define SEEK_SET 0` (measured with clang).  The port's `unistd.in.rs` mirror
/// carries the same three constants, but this program has no `<unistd.h>` include, so the use
/// site spells the one it needs (P-72 (3)).
const SEEK_SET: c_int = 0;

/// `<stdio.h>`: `int fseeko (FILE *, off_t, int);` - no mirror carries it: `stdio.rs` is
/// included by units that do not all have `off_t` in scope, so the declaration belongs at the
/// one use site that calls it (P-72 (3)).
unsafe extern "C" {
  pub fn fseeko (stream: *mut FILE, offset: off_t, whence: c_int) -> c_int;
}

// C 69: `static char *buf;`  (`/* Static (instead of being local to 'main') to pacify
// LeakSanitizer.  */`)
static mut buf: *mut c_char = std::ptr::null_mut ();

// C 72: `int main (int argc, char **argv)`.  C's `main` has external linkage, so the port keeps
// the name and the C signature (`femacs/src/emacs.rs`'s `main` is the precedent).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main (argc: c_int, argv: *mut *mut c_char) -> c_int
{
  let mut c: c_int;
  let mut raw: bool = false;

  /* C 76: `while (0 <= (c = getopt (argc, argv, "rh")))` - the assignment inside the condition
     becomes a `loop` with the same test (PORTING.md 2.2's statement-shape rule).  */
  loop
    {
      c = getopt (argc, argv, c"rh".as_ptr ());
      if ! (0 <= c)
	{
	  break;
	}

      /* C 79-91: `switch (c)` with `case 'r'`, `case 'h'` and `default` (PORTING.md 2.2).  */
      match c
	{
	  114 /* C 82: `case 'r':` */ =>
	    {
	      raw = true;
	    }
	  104 /* C 85: `case 'h':` */ =>
	    {
	      printf (c"make-fingerprint [-r] FILE: replace or compute a hash\n".as_ptr ());
	      return EXIT_SUCCESS;
	    }
	  _ =>
	    {
	      return EXIT_FAILURE;
	    }
	}
    }

  // C 94-95: `struct sha256_ctx ctx; sha256_init_ctx (&ctx);`
  let mut ctx: sha256_ctx = std::mem::zeroed ();
  sha256_init_ctx (&mut ctx);

  // C 97-98: `char *prog = argv[0]; char *file = argv[optind];`
  let prog: *mut c_char = *argv;
  let file: *mut c_char = *argv.offset (optind as isize);
  if argc - optind != 1
    {
      fprintf (stderr, c"%s: missing or extra file operand\n".as_ptr (), prog);
      return EXIT_FAILURE;
    }

  /* C 113: `FILE *f = fopen (file, raw ? "r" FOPEN_BINARY : "r+" FOPEN_BINARY);` - FOPEN_BINARY
     is the empty string in the frozen configuration, so the two modes are C's literals.  */
  let f: *mut FILE = fopen (file, if raw { c"r".as_ptr () } else { c"r+".as_ptr () });

  // C 114: `struct stat st;`
  let mut st: stat = std::mem::zeroed ();

  // C 115-120: `if (!f || fstat (fileno (f), &st) != 0) { perror (file); return EXIT_FAILURE; }`
  if f.is_null () || fstat (fileno (f), &mut st) != 0
    {
      perror (file);
      return EXIT_FAILURE;
    }

  // C 122-127: `if (!S_ISREG (st.st_mode)) { … }`
  if ! S_ISREG! (st.st_mode)
    {
      fprintf (stderr, c"%s: Error: %s is not a regular file\n".as_ptr (), prog, file);
      return EXIT_FAILURE;
    }

  /* C 129-130: `ptrdiff_t maxlen = min (min (TYPE_MAXIMUM (off_t), PTRDIFF_MAX),
                                           min (SIZE_MAX, SSIZE_MAX));`
     `TYPE_MAXIMUM (off_t)` is `off_t::MAX`; C compares `SIZE_MAX` (unsigned long) with
     `SSIZE_MAX` (long) by converting the latter to unsigned, which the port keeps explicit.  */
  // C's `ptrdiff_t maxlen` (this unit has no lisp.h mirror, so the Rust spelling of that
  // type is written out; PORTING.md 2.2's `int`-family rule).
  let maxlen: isize = min! (min! (off_t::MAX as isize, PTRDIFF_MAX),
				min! (SIZE_MAX, SSIZE_MAX as usize) as isize);

  // C 131-136: `if (maxlen <= st.st_size) { … }`
  if maxlen <= st.st_size as isize
    {
      fprintf (stderr, c"%s: %s: file too big\n".as_ptr (), prog, file);
      return EXIT_FAILURE;
    }

  // C 138-143: `buf = malloc (st.st_size + 1); if (!buf) { perror ("malloc"); … }`
  buf = malloc (st.st_size as usize + 1) as *mut c_char;
  if buf.is_null ()
    {
      perror (c"malloc".as_ptr ());
      return EXIT_FAILURE;
    }

  // C 145-151: `size_t chunksz = fread (buf, 1, st.st_size + 1, f); if (ferror (f) || …)`
  let chunksz: usize = fread (buf as *mut c_void, 1, st.st_size as usize + 1, f);
  if ferror (f) != 0 || chunksz != st.st_size as usize
    {
      fprintf (stderr, c"%s: Error: could not read %s\n".as_ptr (), prog, file);
      return EXIT_FAILURE;
    }

  // C 153: `sha256_process_bytes (buf, chunksz, &ctx);`
  sha256_process_bytes (buf as *const c_void, chunksz, &mut ctx);

  // C 155-156: `unsigned char digest[32]; sha256_finish_ctx (&ctx, digest);`
  let mut digest: [c_uchar; 32] = [0; 32];
  sha256_finish_ctx (&mut ctx, digest.as_mut_ptr () as *mut c_void);

  // C 158-164: `if (raw) { for (int i = 0; i < 32; ++i) printf ("%02X", digest[i]); }`
  if raw
    {
      for i in 0..32
	{
	  printf (c"%02X".as_ptr (), digest[i] as c_int);
	}
    }
  else
    {
      // C 166: `bool fingered = false;`
      let mut fingered: bool = false;

      /* C 168-182: the search loop
	   `for (char *finger = buf; (finger = memmem (finger, buf + chunksz - finger,
	                                               (unsigned char *) fingerprint,
	                                               sizeof fingerprint)); finger++)`  */
      let mut finger: *mut c_char = buf;
      loop
	{
	  finger = memmem (finger as *const c_void,
			   buf.add (chunksz).offset_from (finger) as usize,
			   fingerprint.as_ptr () as *const c_void,
			   std::mem::size_of_val (&fingerprint)) as *mut c_char;
	  if finger.is_null ()
	    {
	      break;
	    }

	  /* C 174-180: `if (! (fseeko (f, finger - buf, SEEK_SET) == 0 && fwrite (…) == sizeof
	     digest)) { perror (file); return EXIT_FAILURE; } fingered = true;`  */
	  if ! (fseeko (f, finger.offset_from (buf) as off_t, SEEK_SET) == 0
		&& fwrite (digest.as_ptr () as *const c_void, 1,
			   std::mem::size_of_val (&digest), f)
		     == std::mem::size_of_val (&digest))
	    {
	      perror (file);
	      return EXIT_FAILURE;
	    }
	  fingered = true;

	  finger = finger.add (1);
	}

      // C 184-188: `if (!fingered) { fprintf (stderr, "%s: %s: missing fingerprint\n", …); … }`
      if ! fingered
	{
	  fprintf (stderr, c"%s: %s: missing fingerprint\n".as_ptr (), prog, file);
	  return EXIT_FAILURE;
	}
    }

  // C 191-195: `if (fclose (f) != 0) { perror (file); return EXIT_FAILURE; }`
  if fclose (f) != 0
    {
      perror (file);
      return EXIT_FAILURE;
    }

  // C 197: `return EXIT_SUCCESS;`
  return EXIT_SUCCESS;
}
