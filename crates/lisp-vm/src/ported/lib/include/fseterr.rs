// femacs: port of lib/fseterr.h @ fd7afced403 ; PORTING.md 0.1.
//
// `HAVE___FSETERR` is undefined on this target (macros.tsv 1696), so C 36-45's `#else` arm is
// live: `extern void fseterr (FILE *fp);`.  The `#if HAVE___FSETERR` arm (the musl `stdio_ext.h`
// rename) and its inner `__HAIKU__` declaration are dead.
//
// The declaration lives in `pub mod fseterr_decls` + glob import, because lib/fseterr.c
// *defines* `fseterr` (E0428; PORTING.md 0.1-10).
// [U1-MODULE-MIRROR]
use super::*;
// [U1] gnulib_config.rs imported via super
// [U1] stdio.rs imported via super

/* C 20-21's include guard.  */
pub const _FSETERR_H: bool = true;

/* C 23-26's `#if !_GL_CONFIG_H_INCLUDED / # error "Please include config.h first." / #endif`.  */
const _: () = core::assert! (_GL_CONFIG_H_INCLUDED, "Please include config.h first.");

pub mod fseterr_decls {
  use super::*;

  unsafe extern "C" {
    /* extern void fseterr (FILE *fp);  */
    pub fn fseterr (fp: *mut FILE);
  }
}
pub use self::fseterr_decls::*;
