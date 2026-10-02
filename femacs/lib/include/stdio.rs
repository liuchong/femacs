// femacs: mirror of the frozen target's <stdio.h> (system header, P-26/P-50).
//
// lisp.h includes <stdio.h> (lisp.h 38) and, because the frozen darwin-arm64
// configuration leaves HAVE_ANDROID undefined, lisp.h 5316-5318 takes
// `# define emacs_fclose fclose` instead of declaring a wrapper.  The port needs
// `FILE` and `fclose` where C gets them, so this mirror carries them and every unit
// includes it in lisp.h's own include order (config.rs -> stdio.rs -> lisp.rs).

/// <stdio.h>'s opaque `FILE`; the reference only ever passes `FILE *` (P-26).
pub enum FILE {}

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
  }
}
use stdio_decls::*;
