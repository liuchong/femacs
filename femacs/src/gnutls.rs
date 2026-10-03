// femacs: port of src/gnutls.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names.
//
// `HAVE_GNUTLS` is undefined (docs/baseline/config/darwin-arm64.macros.tsv), so all but 124
// lines are dead text: C 30-2336's `#ifdef HAVE_GNUTLS` region (the `<gnutls/gnutls.h>`
// include, the Lisp_GnuTLS_* records, the callback and error-mapping machinery,
// `init_gnutls_functions`'s dynlib loading and the four DEFUNs `gnutls-format-certificate`,
// `gnutls-ciphers`, `gnutls-digests` and `gnutls-hash-mac`) and C 2970-3013's and C 3036-3142's
// regions inside the two live functions.  They are recorded in docs/ports/gnutls.md, not
// transcribed (P-08).
//
// The live text is C 19-28's nine includes, the three statements of
// `Fgnutls_available_p` (`capabilities`' initialization and its return) and
// `syms_of_gnutls`'s `DEFVAR_LISP`, its assignment, one `defsubr` and the fifty-odd `DEFSYM`s.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn)]

include!("include/config.rs");
include!("../lib/include/errno.rs");	// C 20: <errno.h>
include!("../lib/include/stdio.rs");	// C 21: <stdio.h>
include!("include/lisp.rs");
include!("include/gnutls.rs");	// C 25: `#include "gnutls.h"`

// The `DEFVAR_*` variables this C file owns: C's storage is `globals.f_<vname>`
// (lisp.h 3495-3530) and `struct emacs_globals globals` is defined once, in alloc.c; the port's
// `include/globals.rs` mirrors the names as declarations, so the unit that owns one defines it
// here (status 3.29, the callint.rs shape).

#[unsafe(no_mangle)]
pub static mut Vlibgnutls_version: Lisp_Object = lisp_h_Qnil ();

// C 24-28's `process.h`, `coding.h`, `buffer.h` and `pdumper.h` are not pulled in: the names
// the live text could take from them (the `Lisp_Process` accessors, `struct Lisp_GnuTLS_*`,
// the coding-system calls, `BVAR` and the four `PDUMPER_*` hooks) all sit inside the dead
// region (the atimer.rs precedent; docs/ports/gnutls.md).  C 25's own header *is* included now
// that `include/gnutls.rs` exists - the unit defines `syms_of_gnutls`, and the mirror's
// `gnutls_decls` glob import is what that definition shadows (P-21).

DEFUN! ("gnutls-available-p", Fgnutls_available_p, Sgnutls_available_p, 0, 0, 0,
       doc: "Return list of capabilities if GnuTLS is available in this instance of Emacs.\n\n...if supported         : then...\nGnuTLS 3 or higher      : the list will contain `gnutls3'.\nGnuTLS MACs             : the list will contain `macs'.\nGnuTLS digests          : the list will contain `digests'.\nGnuTLS symmetric ciphers: the list will contain `ciphers'.\nGnuTLS AEAD ciphers     : the list will contain `AEAD-ciphers'.\n%DUMBFW                 : the list will contain `ClientHello\\ Padding'.\nAny GnuTLS extension with ID up to 100\n                        : the list will contain its name.  ",
       (),
       {
  let mut capabilities: Lisp_Object = Qnil ();

  // C 2965-3013's `#ifdef HAVE_GNUTLS` region is dead text (docs/ports/gnutls.md).

  return capabilities;
});

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_gnutls ()
{
  DEFVAR_LISP! (c"libgnutls-version", Vlibgnutls_version,
		"The version of libgnutls that Emacs was compiled with.\nThe version number is encoded as an integer with the major version in\nthe ten thousands place, minor version in the hundreds, and patch\nlevel in the ones.  For builds without libgnutls, the value is -1.");
  // C 3026-3034's `#ifdef HAVE_GNUTLS` arm is dead; the live arm is `(-1)`.
  Vlibgnutls_version = make_fixnum (-1);

  // C 3036-3142's `#ifdef HAVE_GNUTLS` region is dead text (docs/ports/gnutls.md).

  defsubr (&raw mut Sgnutls_available_p);

  DEFSYM! (QAEAD_ciphers, "AEAD-ciphers");
  DEFSYM! (QCcertificate, ":certificate");
  DEFSYM! (QCcertificate_id, ":certificate-id");
  DEFSYM! (QCcertificate_security_level, ":certificate-security-level");
  DEFSYM! (QCcertificates, ":certificates");
  DEFSYM! (QCcipher, ":cipher");
  DEFSYM! (QCcompression, ":compression");
  DEFSYM! (QCdiffie_hellman_prime_bits, ":diffie-hellman-prime-bits");
  DEFSYM! (QCencrypt_then_mac, ":encrypt-then-mac");
  DEFSYM! (QCexpired, ":expired");
  DEFSYM! (QCinsecure, ":insecure");
  DEFSYM! (QCinvalid, ":invalid");
  DEFSYM! (QCinvalid_ocsp_status, ":invalid-ocsp-status");
  DEFSYM! (QCissuer, ":issuer");
  DEFSYM! (QCissuer_unique_id, ":issuer-unique-id");
  DEFSYM! (QCkey_exchange, ":key-exchange");
  DEFSYM! (QClientHello_Padding, "ClientHello Padding");
  DEFSYM! (QCmac, ":mac");
  DEFSYM! (QCmissing_ocsp_status, ":missing-ocsp-status");
  DEFSYM! (QCno_host_match, ":no-host-match");
  DEFSYM! (QCnot_activated, ":not-activated");
  DEFSYM! (QCnot_ca, ":not-ca");
  DEFSYM! (QCpem, ":pem");
  DEFSYM! (QCprotocol, ":protocol");
  DEFSYM! (QCpublic_key_algorithm, ":public-key-algorithm");
  DEFSYM! (QCpublic_key_id, ":public-key-id");
  DEFSYM! (QCpublic_key_id_sha256, ":public-key-id-sha256");
  DEFSYM! (QCpurpose_mismatch, ":purpose-mismatch");
  DEFSYM! (QCrevocation_data_issued_in_future,
	  ":revocation-data-issued-in-future");
  DEFSYM! (QCrevocation_data_superseded, ":revocation-data-superseded");
  DEFSYM! (QCrevoked, ":revoked");
  DEFSYM! (QCsafe_renegotiation, ":safe-renegotiation");
  DEFSYM! (QCself_signed, ":self-signed");
  DEFSYM! (QCserial_number, ":serial-number");
  DEFSYM! (QCsignature_algorithm, ":signature-algorithm");
  DEFSYM! (QCsignature_failure, ":signature-failure");
  DEFSYM! (QCsigner_constraints_failure, ":signer-constraints-failure");
  DEFSYM! (QCsubject, ":subject");
  DEFSYM! (QCsubject_unique_id, ":subject-unique-id");
  DEFSYM! (QCunknown_ca, ":unknown-ca");
  DEFSYM! (QCvalid_from, ":valid-from");
  DEFSYM! (QCvalid_to, ":valid-to");
  DEFSYM! (QCversion, ":version");
  DEFSYM! (QCwarnings, ":warnings");
  DEFSYM! (Qciphers, "ciphers");
  DEFSYM! (Qdigests, "digests");
  DEFSYM! (Qgnutls, "gnutls");
  DEFSYM! (Qgnutls3, "gnutls3");
  DEFSYM! (Qmacs, "macs");
}
