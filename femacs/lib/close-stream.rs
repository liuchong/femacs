#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]
use std::ffi::{c_char, c_double, c_int, c_long, c_short, c_uint, c_ulong, c_void};
// femacs: port of lib/close-stream.c @ fd7afced403.  Blind file:
// `USE_UNLOCKED_IO` expands to `GNULIB_STDIO_SINGLE_THREAD`, which is 1 (macros.tsv 1160),
// so C 26-28's `#include "unlocked-io.h"` is live.  That header renames the unlocked stdio
// entries to the locked ones, and on this target `ferror` is `<stdio.h>`'s `__sferror`
// macro (`(p)->_flags & __SERR`), which is why the reference object has no `_ferror`
// reference (`nm -u build/emacs-ref/lib/close-stream.o` shows `___error`, `___fpending` and
// `_fclose` only).  The port keeps C's text `ferror (stream)` and calls the libSystem
// function of that name, which returns the same flag for the same stream (its `_ferror`
// symbol is linkable: measured with `nm -u` on a probe that spells `(ferror) (stdin)`).
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/errno.rs");       // <errno.h>
include!("include/stdio.rs");       // <stdio.h>
include!("include/fpending.rs");    // "fpending.h" (`__fpending`)
include!("include/close-stream.rs");  // "close-stream.h"

/* Close STREAM.  Return 0 if successful, EOF (setting errno)
   otherwise.  A failure might set errno to 0 if the error number
   cannot be determined.

   A failure with errno set to EPIPE may or may not indicate an error
   situation worth signaling to the user.  See the documentation of the
   close_stdout_set_ignore_EPIPE function for details.

   If a program writes *anything* to STREAM, that program should close
   STREAM and make sure that it succeeds before exiting.  Otherwise,
   suppose that you go to the extreme of checking the return status
   of every function that does an explicit write to STREAM.  The last
   printf can succeed in writing to the internal stream buffer, and yet
   the fclose(STREAM) could still fail (due e.g., to a disk full error)
   when it tries to write out that buffered data.  Thus, you would be
   left with an incomplete output file and the offending program would
   exit successfully.  Even calling fflush is not always sufficient,
   since some file systems (NFS and CODA) buffer written/flushed data
   until an actual close call.

   Besides, it's wasteful to check the return value from every call
   that writes to STREAM -- just let the internal stream state record
   the failure.  That's what the ferror test is checking below.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn close_stream (stream: *mut FILE) -> c_int
{
  let some_pending: bool = (__fpending (stream) != 0);
  let prev_fail: bool = (ferror (stream) != 0);
  let fclose_fail: bool = (fclose (stream) != 0);

  /* Return an error indication if there was a previous failure or if
     fclose failed, with one exception: ignore an fclose failure if
     there was no previous error, no data remains to be flushed, and
     fclose failed with EBADF.  That can happen when a program like cp
     is invoked like this 'cp a b >&-' (i.e., with standard output
     closed) and doesn't generate any output (hence no previous error
     and nothing to be flushed).  */

  if prev_fail || (fclose_fail && (some_pending || *errno () != EBADF))
    {
      if ! fclose_fail
        { *errno () = 0; }
      return EOF;
    }

  return 0;
}
