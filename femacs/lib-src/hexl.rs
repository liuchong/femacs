// femacs: port of lib-src/hexl.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names, the same `main`,
// and the two static helpers (`output_error`, `hexchar`) stay plain `unsafe fn` except that the
// one C marks `_Noreturn` says `-> !`.  This is a *program* of the C tree's lib-src directory
// (batch F), built by the frozen configuration (it is in the reference `lib-src/Makefile`'s
// `UTILITIES` list) and linked against the port's gnulib mirrors and libSystem only.
//
// Two C names have no mirror and are handled at the use site (P-26/P-72 (3)):
//  * `<inttypes.h>`'s `PRIxMAX` (C 163): the frozen target's `<inttypes.h>` defines it as `"jx"`
//    (measured with clang: `printf ("%s\n", PRIxMAX)` prints `jx`, so `uintmax_t` is the 64-bit
//    type and C's format is `%08jx`), and the port writes that format;
//  * `<unlocked-io.h>` (C 29): the renames it performs on stdio calls are recorded at the use
//    site by every unit that needs them (docs/ports/dispnew.md 374, docs/ports/bidi.md 269), so
//    this program calls `getc`/`putchar`/`fputs` as C's text spells them.
// `uintmax_t` itself comes from the unit, the way `ptrdiff_t` does in `make-fingerprint.rs`:
// the Emacs core takes it from the `lisp.h` mirror, which this program does not include.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]

use std::ffi::{c_char, c_double, c_int, c_long, c_short, c_uchar, c_uint, c_ulong,
              c_void};

/// `<stdint.h>`: `typedef ... uintmax_t;` - measured as the target's 64-bit unsigned type
/// (`PRIxMAX` is `"jx"`, and `sizeof (uintmax_t)` is 8).
pub type uintmax_t = u64;

include!("../src/include/config.rs");        // C 22: `#include <config.h>`
include!("../lib/include/stdint.in.rs");     // C 24's `<inttypes.h>` half: the int types
include!("../lib/include/stdlib.rs");        // C 25: `#include <stdlib.h>`
include!("../lib/include/string.rs");        // C 26: `#include <string.h>`
include!("../lib/include/binary-io.rs");     // C 28: `#include <binary-io.h>` (`set_binary_mode`)

/// `<stdlib.h>`: `void exit (int);` - no mirror carries it yet (the emacs.rs/sysdep.rs
/// precedent keeps it at the use site).
unsafe extern "C" {
  pub fn exit (status: c_int) -> !;
}

// C 31: `static char *progname;`
static mut progname: *mut c_char = std::ptr::null_mut ();

// C 33-38: `static _Noreturn void output_error (void)`
unsafe fn output_error () -> !
{
  fprintf (stderr, c"%s: write error\n".as_ptr (), progname);
  exit (EXIT_FAILURE);
}

// C 40-44: `static int hexchar (int c)`
unsafe fn hexchar (c: c_int) -> c_int
{
  return c - (if b'0' as c_int <= c && c <= b'9' as c_int
	      { b'0' as c_int }
	      else
	      { b'a' as c_int - 10 });
}

// C 46: `int main (int argc, char **argv)`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main (argc: c_int, argv: *mut *mut c_char) -> c_int
{
  let mut argv: *mut *mut c_char = argv;
  let _ = argc;

  // C 48-51: `int status = EXIT_SUCCESS; int DEFAULT_GROUPING = 0x01; int group_by =
  // DEFAULT_GROUPING; bool un_flag = false, iso_flag = false;`
  let mut status: c_int = EXIT_SUCCESS;
  let DEFAULT_GROUPING: c_int = 0x01;
  let mut group_by: c_int = DEFAULT_GROUPING;
  let mut un_flag: bool = false;
  let mut iso_flag: bool = false;

  // C 53: `progname = *argv++;`
  progname = *argv;
  argv = argv.add (1);

  /* C 69's switch list is C 70-73's comment; the loop below is C 69:
     `for (; *argv && *argv[0] == '-' && (*argv)[1]; argv++)`.  */
  while ! (*argv).is_null () && **argv == b'-' as c_char && *(*argv).offset (1) != 0
    {
      /* A switch! */
      if strcmp (*argv, c"--".as_ptr ()) == 0
	{
	  // C 75-79: `argv++; break;`
	  argv = argv.add (1);
	  break;
	}
      else if strcmp (*argv, c"-un".as_ptr ()) == 0 || strcmp (*argv, c"-de".as_ptr ()) == 0
	{
	  un_flag = true;
	  set_binary_mode (fileno (stdout), O_BINARY);
	}
      else if strcmp (*argv, c"-hex".as_ptr ()) == 0
	{
	  /* Hex is the default and is only base supported.  */
	}
      else if strcmp (*argv, c"-iso".as_ptr ()) == 0
	{
	  iso_flag = true;
	}
      else if strcmp (*argv, c"-group-by-8-bits".as_ptr ()) == 0
	{
	  group_by = 0x00;
	}
      else if strcmp (*argv, c"-group-by-16-bits".as_ptr ()) == 0
	{
	  group_by = 0x01;
	}
      else if strcmp (*argv, c"-group-by-32-bits".as_ptr ()) == 0
	{
	  group_by = 0x03;
	}
      else if strcmp (*argv, c"-group-by-64-bits".as_ptr ()) == 0
	{
	  group_by = 0x07;
	}
      else
	{
	  fprintf (stderr, c"%s: invalid switch: \"%s\".\n".as_ptr (), progname, *argv);
	  fprintf (stderr, c"usage: %s [-de] [-iso]\n".as_ptr (), progname);
	  return EXIT_FAILURE;
	}

      argv = argv.add (1);
    }

  // C 114: `char const *filename = *argv ? *argv++ : "-";`
  let mut filename: *const c_char = if ! (*argv).is_null ()
    {
      let f: *const c_char = *argv;
      argv = argv.add (1);
      f
    }
    else
    {
      c"-".as_ptr ()
    };

  // C 116: `do { … } while (filename);`
  loop
    {
      // C 118-120: `FILE *fp;`
      let mut fp: *mut FILE;

      if strcmp (filename, c"-".as_ptr ()) == 0
	{
	  fp = stdin;
	  if ! un_flag
	    {
	      set_binary_mode (fileno (stdin), O_BINARY);
	    }
	}
      else
	{
	  fp = fopen (filename, if un_flag { c"r".as_ptr () } else { c"rb".as_ptr () });
	  if fp.is_null ()
	    {
	      perror (filename);
	      status = EXIT_FAILURE;
	      // C 139-140: `continue;` - the do/while's tail is the `filename` update below.
	      filename = *argv;
	      argv = argv.add (1);
	      if filename.is_null ()
		{
		  break;
		}
	      continue;
	    }
	}

      if un_flag
	{
	  // C 145-147: `for (int c; 0 <= (c = getc (fp)); )`
	  let mut c: c_int;
	  loop
	    {
	      c = getc (fp);
	      if ! (0 <= c)
		{
		  break;
		}

	      /* C 149-151: skip the address at the start of the line.  */
	      if c != b' ' as c_int
		{
		  continue;
		}

	      // C 153: `for (int i = 0; i < 16; i++)`
	      for i in 0..16
		{
		  c = getc (fp);
		  if c < 0 || c == b' ' as c_int
		    {
		      break;
		    }

		  let hc: c_int = hexchar (c);
		  c = getc (fp);
		  if c < 0
		    {
		      break;
		    }
		  putchar (hc * 0x10 + hexchar (c));

		  if (i & group_by) == group_by
		    {
		      c = getc (fp);
		      if c < 0
			{
			  break;
			}
		    }
		}

	      while 0 <= c && c != b'\n' as c_int
		{
		  c = getc (fp);
		}
	      if c < 0
		{
		  break;
		}
	      if ferror (stdout) != 0
		{
		  output_error ();
		}
	    }
	}
      else
	{
	  // C 182-185: `int c = 0; char string[18]; string[0] = ' '; string[17] = '\0';`
	  let mut c: c_int = 0;
	  let mut string: [c_char; 18] = [0; 18];
	  string[0] = b' ' as c_char;
	  string[17] = 0;

	  // C 186: `for (uintmax_t address = 0; 0 <= c; address += 0x10)`
	  let mut address: uintmax_t = 0;
	  while 0 <= c
	    {
	      /* C 189 declares `int i;` outside the loop and reads it after it (C 224's `if (i)`);
		 the port initializes it, because Rust requires a value before use and the loop's
		 sixteen iterations always assign one (PORTING.md 1.5).  */
	      let mut i: c_int = 0;
	      for i_ in 0..16
		{
		  i = i_;
		  if 0 <= c
		    {
		      c = getc (fp);
		    }
		  if c < 0
		    {
		      if i == 0
			{
			  break;
			}

		      fputs (c"  ".as_ptr (), stdout);
		      string[(i + 1) as usize] = 0;
		    }
		  else
		    {
		      if i == 0
			{
			  // C 199: `printf ("%08"PRIxMAX": ", address);` - PRIxMAX is `"jx"` here.
			  printf (c"%08jx: ".as_ptr (), address);
			}

		      string[(i + 1) as usize] = (if c < 0x20
						     || (0x7F <= c && (! iso_flag || c < 0xa0))
						  { b'.' as c_char }
						  else { c as c_char });

		      printf (c"%02x".as_ptr (), c as c_uint + 0u32);
		    }

		  if (i & group_by) == group_by
		    {
		      putchar (b' ' as c_int);
		    }
		}

	      if i != 0
		{
		  puts (string.as_ptr ());
		}

	      if ferror (stdout) != 0
		{
		  output_error ();
		}

	      address += 0x10;
	    }
	}

      // C 231-233: `bool trouble = ferror (fp) != 0; trouble |= fp != stdin && fclose (fp) != 0;`
      let mut trouble: bool = ferror (fp) != 0;
      trouble |= fp != stdin && fclose (fp) != 0;
      if trouble
	{
	  fprintf (stderr, c"%s: read error\n".as_ptr (), progname);
	  status = EXIT_FAILURE;
	}

      // C 241: `filename = *argv++;`
      filename = *argv;
      argv = argv.add (1);
      if filename.is_null ()
	{
	  break;
	}
    }

  // C 245-246: `if (ferror (stdout) || fclose (stdout) != 0) output_error ();`
  if ferror (stdout) != 0 || fclose (stdout) != 0
    {
      output_error ();
    }

  // C 247: `return status;`
  return status;
}
