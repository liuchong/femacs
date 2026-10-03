// femacs: mirror of the frozen target's <stdio.h> (system header, P-26/P-50).
//
// lisp.h includes <stdio.h> (lisp.h 38) and, because the frozen darwin-arm64
// configuration leaves HAVE_ANDROID undefined, lisp.h 5316-5318 takes
// `# define emacs_fclose fclose` instead of declaring a wrapper.  The port needs
// `FILE` and `fclose` where C gets them, so this mirror carries them and every unit
// includes it in lisp.h's own include order (config.rs -> stdio.rs -> lisp.rs).

/* <stdio.h>'s `FILE`.  It is Darwin's `struct __sFILE`, whose fields three gnulib units read
   through lib/stdio-impl.h's `fp_` (lib/fpending.c, lib/fseterr.c, lib/close-stream.c), so
   the mirror carries the measured layout in lib/include/stdio-impl.rs and names it `FILE`
   here - every unit keeps writing `*mut FILE` as C does.  */
include!("stdio-impl.rs");

unsafe extern "C" {
  /* int fclose (FILE *stream);  */
  pub fn fclose (stream: *mut FILE) -> c_int;
}

/// The four stdio entries lib/md5-stream.c names (the eof/error predicates, the read and the
/// write), in a whole-import module: a unit that declares one of them itself (lib/mini-gmp.c
/// declares `fwrite`) then shadows the glob instead of clashing with it (PORTING.md 0.1-10).
pub mod stdio_decls {
  use super::*;

  unsafe extern "C" {
    pub fn feof (stream: *mut FILE) -> c_int;
    pub fn ferror (stream: *mut FILE) -> c_int;
    pub fn fread (ptr: *mut core::ffi::c_void, size: usize, n: usize, stream: *mut FILE)
      -> usize;
    pub fn fwrite (ptr: *const core::ffi::c_void, size: usize, n: usize, stream: *mut FILE)
      -> usize;
    /* The two writes lib/stdio-consolesafe.c uses instead of fwrite (msvcrt's fwrite is the
       bug that file works around).  */
    pub fn fputs (s: *const c_char, stream: *mut FILE) -> c_int;
    pub fn fputc (c: c_int, stream: *mut FILE) -> c_int;
    /* int sprintf (char *s, const char *format, ...); - src/character.c is the first unit
       that formats into a caller-supplied buffer (`string_escape_byte8`'s "\\%03o"); the
       declaration is C-variadic, like lisp.rs's `error` (PORTING.md P-04).  */
    pub fn sprintf (s: *mut c_char, format: *const c_char, ...) -> c_int;

    /* The entries the debug/dump paths print with.  Thirteen units declared `fprintf`
       themselves before this landed (status 3.79); the module + glob import is what lets any
       of them keep a copy without an `E0428` (0.1-10), and their copies are gone now.  */
    /// C: <stdio.h>: `int printf (const char *format, ...);`  (`src/emacs.c` declared
    /// this itself until the mirror was completed; lib-src/make-fingerprint.c is the second
    /// unit that prints without a stream).
    pub fn printf (format: *const c_char, ...) -> c_int;
    /// C: <stdio.h>: `FILE *fopen (const char *restrict, const char *restrict);`
    pub fn fopen (path: *const c_char, mode: *const c_char) -> *mut FILE;
    /// C: <stdio.h>: `void perror (const char *);`
    pub fn perror (s: *const c_char);
    /// C: <stdio.h>: `int fprintf (FILE *stream, const char *format, ...);`
    pub fn fprintf (stream: *mut FILE, format: *const c_char, ...) -> c_int;
    /// C: <stdio.h>: `int putc (int c, FILE *stream);`
    pub fn putc (c: c_int, stream: *mut FILE) -> c_int;
    /// C: <stdio.h>: `int fflush (FILE *stream);`
    pub fn fflush (stream: *mut FILE) -> c_int;
    /// C: <stdio.h>: `int fileno (FILE *stream);`
    pub fn fileno (stream: *mut FILE) -> c_int;
    /// C: <stdio.h>: `void clearerr (FILE *stream);`  (the port of `src/lread.c` spelled it
    /// `-> c_int` until this landed - C returns void, and nothing used the value.)
    pub fn clearerr (stream: *mut FILE);
    /// C: <stdio.h>: `int getc (FILE *stream);`
    pub fn getc (stream: *mut FILE) -> c_int;
  }
}
use stdio_decls::*;

/// <stdio.h>'s three standard streams.  Darwin exports them as `__stdoutp`/`__stdinp`/
/// `__stderrp` (the same `link_name` spelling the units used at their use sites), and they
/// live in a whole-import module so a unit that declares one itself shadows the glob instead
/// of clashing with it (PORTING.md 0.1-10).  Twenty-four units carried a copy (status 3.79).
pub mod stdio_objs {
  use super::*;

  unsafe extern "C" {
    #[cfg_attr(target_os = "macos", link_name = "__stdoutp")]
    pub static mut stdout: *mut FILE;
    #[cfg_attr(target_os = "macos", link_name = "__stdinp")]
    pub static mut stdin: *mut FILE;
    #[cfg_attr(target_os = "macos", link_name = "__stderrp")]
    pub static mut stderr: *mut FILE;
  }
}
use stdio_objs::*;

/// <stdio.h>: `#define EOF (-1)` (the same value lib/getdelim.c declares in-unit).
pub const EOF: c_int = -1;
