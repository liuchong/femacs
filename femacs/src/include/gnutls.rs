// femacs: mirror of src/gnutls.h @ fd7afced403.
//
// Dead by construction: `HAVE_GNUTLS` is undefined in the frozen configuration (macros.tsv), so
// the whole body - C 25-75's `#ifdef HAVE_GNUTLS` block with the `<gnutls/gnutls.h>`/`x509.h`
// includes, `GNUTLS_EMACS_HANDSHAKES_LIMIT` (6000), the `gnutls_initstage_t` enum, the two
// error codes and the `GNUTLS_INITSTAGE` macro - is not compiled.  The live inventory is the
// guard alone (the extractor reports 0 subr / 0 function / 1 macro), so the mirror carries the
// guard and records the dead arms in docs/ports/include-gnutls.md instead of transcribing them
// (PORTING.md P-08).  src/gnutls.c, the only unit that would define anything for this header,
// is dead in the frozen build as well (batch F records it).
//
// C 20: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const EMACS_GNUTLS_DEFINED: bool = true;

// C 98: `extern void syms_of_gnutls (void);` - **live**: it sits outside the
// `#ifdef HAVE_GNUTLS` block, and src/emacs.c 2462 calls it unconditionally.  src/gnutls.c
// defines it (femacs/src/gnutls.rs owns the definition), so it is declared in a glob-imported
// module whose import the unit's definition shadows (PORTING.md 0.1-10/P-21).
pub mod gnutls_decls {
  use super::*;

  unsafe extern "C" {
    pub fn syms_of_gnutls ();
  }
}
use gnutls_decls::*;
