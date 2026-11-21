// femacs: mirror of src/gnutls.h @ fd7afced403.
//
// Historical baseline: `HAVE_GNUTLS` is undefined in the frozen configuration (macros.tsv), so
// the whole body - C 25-75's `#ifdef HAVE_GNUTLS` block with the `<gnutls/gnutls.h>`/`x509.h`
// includes, `GNUTLS_EMACS_HANDSHAKES_LIMIT` (6000), the `gnutls_initstage_t` enum, the two
// error codes and the `GNUTLS_INITSTAGE` macro - is not compiled.  The live inventory is the
// guard alone (the extractor reports 0 subr / 0 function / 1 macro), so the mirror carries the
// guard and records the dead arms in docs/ports/include-gnutls.md instead of transcribing them
// (PORTING.md P-08).  src/gnutls.c, the only unit that would define anything for this header,
// is dead in the frozen build as well (batch F records it). P-401 restores the
// actual header/process path below and enables it in the owned macOS targets;
// functional acceptance remains separate from configuration availability.
//
// C 20: the include guard has no Rust counterpart (PORTING.md 0.1).
// [U1-MODULE-MIRROR]
use super::*;
pub const EMACS_GNUTLS_DEFINED: bool = true;

// C 98: `extern void syms_of_gnutls (void);` - **live**: it sits outside the
// `#ifdef HAVE_GNUTLS` block, and src/emacs.c 2462 calls it unconditionally.  src/gnutls.c
// defines it (src/gnutls.rs owns the definition), so it is declared in a glob-imported
// module whose import the unit's definition shadows (PORTING.md 0.1-10/P-21).
pub mod gnutls_decls {
  use super::*;

  unsafe extern "C" {
    pub fn syms_of_gnutls ();
    #[cfg(feature = "have_gnutls")]
    pub fn emacs_gnutls_write(p: *mut Lisp_Process, buf: *const c_char,
                              bytes: ptrdiff_t) -> ptrdiff_t;
    #[cfg(feature = "have_gnutls")]
    pub fn emacs_gnutls_read(p: *mut Lisp_Process, buf: *mut c_char,
                             bytes: ptrdiff_t) -> ptrdiff_t;
    #[cfg(feature = "have_gnutls")]
    pub fn emacs_gnutls_record_check_pending(state: process_gnutls_types::gnutls_session_t) -> ptrdiff_t;
    #[cfg(feature = "have_gnutls")]
    pub fn emacs_gnutls_deinit(process: Lisp_Object) -> Lisp_Object;
    #[cfg(feature = "have_gnutls")]
    pub fn gnutls_try_handshake(p: *mut Lisp_Process) -> c_int;
    #[cfg(feature = "have_gnutls")]
    pub fn gnutls_verify_boot(process: Lisp_Object, parameters: Lisp_Object) -> Lisp_Object;
  }

  #[cfg(feature = "have_gnutls")]
  #[inline(always)]
  pub unsafe fn GNUTLS_INITSTAGE(process: Lisp_Object) -> process_gnutls_types::gnutls_initstage_t {
    unsafe { (*XPROCESS(process)).gnutls_initstage }
  }

  #[cfg(feature = "have_gnutls")]
  #[inline(always)]
  pub unsafe fn GNUTLS_PROCESS_USABLE(process: Lisp_Object) -> bool {
    unsafe { GNUTLS_INITSTAGE(process) >= process_gnutls_types::GNUTLS_STAGE_READY }
  }
}
pub use self::gnutls_decls::*;
