// femacs: port of src/gnutls.c @ fd7afced403.
// P-401: real certificate, boot, verification, record and crypto bodies and
// registrations are now present. Foreign API/version/dependency integration
// remains in progress; enabling a target is not a runtime acceptance claim.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model. The original first-slice ledger is historical; P-401 restores the
// remaining actual body below. Current enablement and acceptance are recorded
// separately from that original disabled baseline.
//
// `HAVE_GNUTLS` is `undef` in the frozen configuration
// (docs/baseline/config/darwin-arm64.macros.tsv 1381), so C's whole body (C 30-2336) is dead text;
// this is a P-86 unit and **both arms of every `#if` are carried**, each item under the gate that
// names C's own macro:
//
//   * the file gate (`#ifdef HAVE_GNUTLS`, C 30) is `feature = "have_gnutls"`;
//   * C 82-86's and C 104-589's `#ifdef WINDOWSNT` arms are `feature = "windowsnt"` (nested inside
//     the file gate), the name `src/vm-limit.rs` 95 already uses for that configuration;
//   * C 32-80's ten version conditions `#define` the eleven `HAVE_GNUTLS_*` macros the rest of the
//     file tests.  A `#define` has no Rust counterpart (P-86 ①), so every `#ifdef HAVE_GNUTLS_X`
//     site carries `feature = "have_gnutls_x"`, and the C 32-80 block below records which version
//     condition feeds each -- `docs/ports/gnutls.md`'s `## Decisions` lists the eleven with their
//     frozen verdicts (`tools/check-cond-eval.py src/gnutls.c` -> 104 conditions, 0 unknown,
//     0 disagreements) plus the two sub-gates that *are* defined in the frozen table
//     (`EMSGSIZE` 40, `ECONNRESET` 54).
//
// `<gnutls/gnutls.h>`, `<gnutls/x509.h>` and `<gnutls/crypto.h>` have no file in the reference
// tree, so this unit carries the declarations it uses itself (P-88 ①; every layout number and
// constant in that segment is a clang reading against this machine's header -- gnutls 3.8.13 --
// and docs/ports/gnutls.md's `## Evidence` records the probe).  The reference header those three
// reach this file through, `src/gnutls.h`, *does* have a mirror
// (`src/include/gnutls.rs`), but it is still the P-08 stub: its own `#ifdef HAVE_GNUTLS`
// arm is not carried, so the handful of its names this slice needs (`GNUTLS_STAGE_*`, the
// `GNUTLS_LOG*` macros, `GNUTLS_INITSTAGE`) are declared at this use site under the same gate and
// are **owed back to that mirror** (docs/ports/gnutls.md's `## Owed`; the P-72 rule-4 shape
// `src/font.rs` 380-381 uses for `nconc2`).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_unsafe, unused_assignments, unused_parens, unused_mut, unreachable_code, unused_labels, while_true, unused_comparisons, overflowing_literals, unused_must_use, clippy::all)]
// [U1-MODULE-UNIT]
use crate::common::*;

const _: () = ();
/* C 20's `<errno.h>`: the frozen configuration has no `build/emacs-ref/lib/errno.h` (gnulib's
   `errno-h` module removes it; `lib/include/errno.in.rs` 3-20 records the measurement), so
   the header C sees is the *system* one, and the port's home for the frozen target's values is
   `errno.in.rs` -- the template mirror that carries both halves (`errno.rs`'s `#include_next` plus
   the frozen fallbacks ENOTCONN 57 / ECONNRESET 54 the gated text below needs).
   `exec/mipsfpu.rs` 27 includes it for the same C line.  */
// [U1] ../lib/include/errno.in.rs imported via common
// [U1] ../lib/include/stdio.rs imported via common

/* The unconditional capability/registration path needs lisp and this unit's
   header. P-401 restores the actual process, file encoding, buffer extraction
   and dumper inputs for the enabled body below. */
// [U1] include/lisp.rs imported via common
// [U1] include/gnutls.rs imported via common

#[cfg (feature = "have_gnutls")]
const _: () = ();
#[cfg(feature = "have_gnutls")]
const _: () = ();
#[cfg(feature = "have_gnutls")]
const _: () = ();
#[cfg(feature = "have_gnutls")]
const _: () = ();

/* The system headers the gated body reaches through C's own includes: `<string.h>` (lisp.h 30;
   `strcpy`/`strlen`/`sprintf`), `<limits.h>` (lisp.h 32; `INT_MIN`/`INT_MAX`), `ssize_t`
   (`<sys/types.h>`/`<unistd.h>`), `<time.h>` (`time_t`, `nanosleep`) and C23 `<stdckdint.h>`
   (the `ckd_mul`/`ckd_add` pair at C 1089-1090, P-15).  */
#[cfg (feature = "have_gnutls")]
const _: () = ();
#[cfg (feature = "have_gnutls")]
const _: () = ();
#[cfg (feature = "have_gnutls")]
const _: () = ();
#[cfg (feature = "have_gnutls")]
const _: () = ();
#[cfg (feature = "have_gnutls")]
const _: () = ();

/* ----------------------------------------------------------------------------------------------
   C 25: `#include "gnutls.h"` -> gnutls.h 21: `#include <gnutls/gnutls.h>`, gnutls.h 26:
   `#include <gnutls/x509.h>`, gnutls.h 29: `#include <gnutls/crypto.h>` -- the third-party
   surface the reference tree does not carry (P-88 ①: the declarations stand in the unit, not in
   `src/include/`, which mirrors the *reference* headers; `check-target-names.py` reads that
   directory by the reference header's name).  Types are `#[repr(C)]` aliases with the header's own
   line in `/// C:` (P-55), the enums are the `pub type = c_int` shape the port uses for every C
   enum (`src/coding.rs` 413), and the values are clang readings.  Every name the header
   declares only for a *later* slice (the rest of the file's API) is not declared here -- this
   segment carries exactly the surface C 30-1130 names.  */

/// C: <gnutls/gnutls.h> 1276: `typedef void *gnutls_transport_ptr_t;`
#[cfg (feature = "have_gnutls")]
pub type gnutls_transport_ptr_t = *mut c_void;
/// C: <gnutls/gnutls.h> 1290-1294: `typedef struct {` / `unsigned char *data;` / `unsigned int size;` /
/// `} gnutls_datum_t;` -- `sizeof` 16, `data` at 0, `size` at 8 (clang).
#[cfg (feature = "have_gnutls")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct gnutls_datum_t
{
  pub data: *mut c_uchar,       /* C: `unsigned char *data;` */
  pub size: c_uint,             /* C: `unsigned int size;` */
}
/// C: <gnutls/gnutls.h> 1614-1630: `typedef enum { GNUTLS_NAME_DNS = 1 } gnutls_server_name_type_t;`
#[cfg (feature = "have_gnutls")]
pub type gnutls_server_name_type_t = c_uint;
#[cfg (feature = "have_gnutls")]
pub const GNUTLS_NAME_DNS: gnutls_server_name_type_t = 1;
/// C: <gnutls/gnutls.h> 2413-2415 ... the `GNUTLS_X509_FMT_*` enum, <gnutls/gnutls.h> 856-859:
/// `typedef enum { GNUTLS_X509_FMT_DER = 0, GNUTLS_X509_FMT_PEM = 1 } gnutls_x509_crt_fmt_t;`
/// (values measured: DER 0, PEM 1).
#[cfg (feature = "have_gnutls")]
pub type gnutls_x509_crt_fmt_t = c_uint;
/// C: <gnutls/gnutls.h> 791-794: `typedef enum { GNUTLS_SHUT_RDWR = 0, GNUTLS_SHUT_WR = 1 }
/// gnutls_close_request_t;` (sizeof 4).
#[cfg (feature = "have_gnutls")]
pub type gnutls_close_request_t = c_uint;
/// C: <gnutls/gnutls.h> 614-648: `typedef enum { ... } gnutls_alert_description_t;` (sizeof 4).
#[cfg (feature = "have_gnutls")]
pub type gnutls_alert_description_t = c_uint;
/// C: <gnutls/gnutls.h> 287-293: `typedef enum { ... } gnutls_credentials_type_t;` (sizeof 4).
#[cfg (feature = "have_gnutls")]
pub type gnutls_credentials_type_t = c_uint;
/// C: <gnutls/gnutls.h> 159-219: `typedef enum gnutls_cipher_algorithm { ... } gnutls_cipher_algorithm_t;`
/// (sizeof 4).
#[cfg (feature = "have_gnutls")]
pub type gnutls_cipher_algorithm_t = c_uint;
/// C: <gnutls/gnutls.h> 335-371: `typedef enum { ... } gnutls_mac_algorithm_t;` (sizeof 4).
#[cfg (feature = "have_gnutls")]
pub type gnutls_mac_algorithm_t = c_uint;
/// C: <gnutls/gnutls.h> 398-421: `typedef enum { ... } gnutls_digest_algorithm_t;` (sizeof 4).
#[cfg (feature = "have_gnutls")]
pub type gnutls_digest_algorithm_t = c_uint;
/// C: <gnutls/gnutls.h> 243-261: `typedef enum { ... } gnutls_kx_algorithm_t;` (sizeof 4).
#[cfg (feature = "have_gnutls")]
pub type gnutls_kx_algorithm_t = c_uint;
/// C: <gnutls/gnutls.h> 813-828: `typedef enum { ... } gnutls_protocol_t;` (sizeof 4).
#[cfg (feature = "have_gnutls")]
pub type gnutls_protocol_t = c_uint;
/// C: <gnutls/gnutls.h> 440-447: `typedef enum { ... } gnutls_compression_method_t;` (sizeof 4).
#[cfg (feature = "have_gnutls")]
pub type gnutls_compression_method_t = c_uint;
/// C: <gnutls/gnutls.h> 841-847: `typedef enum { ... } gnutls_certificate_type_t;` (sizeof 4).
#[cfg (feature = "have_gnutls")]
pub type gnutls_certificate_type_t = c_uint;
/// C: <gnutls/gnutls.h> 1002-1061: `typedef enum { ... } gnutls_sign_algorithm_t;` (sizeof 4).
#[cfg (feature = "have_gnutls")]
pub type gnutls_sign_algorithm_t = c_uint;
/// C: <gnutls/gnutls.h> 1203-1216: `typedef enum { ... } gnutls_sec_param_t;` (sizeof 4).
#[cfg (feature = "have_gnutls")]
pub type gnutls_sec_param_t = c_uint;
/// C: <gnutls/gnutls.h> 908-933: `typedef enum { ... } gnutls_pk_algorithm_t;` (sizeof 4).
#[cfg (feature = "have_gnutls")]
pub type gnutls_pk_algorithm_t = c_uint;
/// C: <gnutls/gnutls.h> 871-877: `typedef enum gnutls_certificate_print_formats { ... }
/// gnutls_certificate_print_formats_t;` (sizeof 4).
#[cfg (feature = "have_gnutls")]
pub type gnutls_certificate_print_formats_t = c_uint;
/// C: crypto.h 32: `typedef struct api_cipher_hd_st *gnutls_cipher_hd_t;`
#[cfg (feature = "have_gnutls")]
pub struct api_cipher_hd_st { _private: [u8; 0] }
/// C: crypto.h 32: the pointer type itself.
#[cfg (feature = "have_gnutls")]
pub type gnutls_cipher_hd_t = *mut api_cipher_hd_st;
/// C: crypto.h 83: `typedef struct api_aead_cipher_hd_st *gnutls_aead_cipher_hd_t;`
#[cfg (feature = "have_gnutls")]
pub struct api_aead_cipher_hd_st { _private: [u8; 0] }
/// C: crypto.h 83: the pointer type itself.
#[cfg (feature = "have_gnutls")]
pub type gnutls_aead_cipher_hd_t = *mut api_aead_cipher_hd_st;
/// C: crypto.h 127: `typedef struct hash_hd_st *gnutls_hash_hd_t;`
#[cfg (feature = "have_gnutls")]
pub struct hash_hd_st { _private: [u8; 0] }
/// C: crypto.h 127: the pointer type itself.
#[cfg (feature = "have_gnutls")]
pub type gnutls_hash_hd_t = *mut hash_hd_st;
/// C: crypto.h 128: `typedef struct hmac_hd_st *gnutls_hmac_hd_t;`
#[cfg (feature = "have_gnutls")]
pub struct hmac_hd_st { _private: [u8; 0] }
/// C: crypto.h 128: the pointer type itself.
#[cfg (feature = "have_gnutls")]
pub type gnutls_hmac_hd_t = *mut hmac_hd_st;
/// C: <gnutls/gnutls.h> 2300: `typedef void (*gnutls_free_function)(void *);`
#[cfg (feature = "have_gnutls")]
pub type gnutls_free_function = Option<unsafe extern "C" fn (*mut c_void)>;
/// C: <gnutls/gnutls.h> 2323: `typedef void (*gnutls_log_func)(int, const char *);`
#[cfg (feature = "have_gnutls")]
pub type gnutls_log_func = Option<unsafe extern "C" fn (c_int, *const c_char)>;
/// C: <gnutls/gnutls.h> 2324: `typedef void (*gnutls_audit_log_func)(gnutls_session_t, const char *);`
#[cfg (feature = "have_gnutls")]
pub type gnutls_audit_log_func = Option<unsafe extern "C" fn (gnutls_session_t, *const c_char)>;
/// C: <gnutls/gnutls.h> 2388-2389: `typedef ssize_t (*gnutls_pull_func)(gnutls_transport_ptr_t, void *,
/// size_t);` -- `ssize_t` is the port's `c_long` (`lib/include/unistd.in.rs` 34).
#[cfg (feature = "have_gnutls")]
pub type gnutls_pull_func = Option<unsafe extern "C" fn (gnutls_transport_ptr_t, *mut c_void,
                                                          size_t) -> ssize_t>;
/// C: <gnutls/gnutls.h> 2390-2391: `typedef ssize_t (*gnutls_push_func)(gnutls_transport_ptr_t,
/// const void *, size_t);`
#[cfg (feature = "have_gnutls")]
pub type gnutls_push_func = Option<unsafe extern "C" fn (gnutls_transport_ptr_t, *const c_void,
                                                          size_t) -> ssize_t>;
/// C: <gnutls/gnutls.h> 2400: `typedef int (*gnutls_errno_func)(gnutls_transport_ptr_t);`
#[cfg (feature = "have_gnutls")]
pub type gnutls_errno_func = Option<unsafe extern "C" fn (gnutls_transport_ptr_t) -> c_int>;

/* The constants C 30-1130 tests, each a clang reading against gnutls 3.8.13 (the header on this
   machine; `docs/ports/gnutls.md`'s `## Evidence` has the probe and its output).  C's enums are
   `pub type = c_int` (above), and these are the named values of <gnutls/gnutls.h>.  */
/// C: gnutls.h: `#define GNUTLS_E_SUCCESS 0`
#[cfg (feature = "have_gnutls")]
pub const GNUTLS_E_SUCCESS: c_int = 0;
/// C: gnutls.h: `#define GNUTLS_E_AGAIN -28`
#[cfg (feature = "have_gnutls")]
pub const GNUTLS_E_AGAIN: c_int = -28;
/// C: gnutls.h: `#define GNUTLS_E_INTERRUPTED -52`
#[cfg (feature = "have_gnutls")]
pub const GNUTLS_E_INTERRUPTED: c_int = -52;
/// C: gnutls.h: `#define GNUTLS_E_INVALID_SESSION -10`
#[cfg (feature = "have_gnutls")]
pub const GNUTLS_E_INVALID_SESSION: c_int = -10;
/// C: gnutls.h: `#define GNUTLS_E_UNEXPECTED_PACKET_LENGTH -9`
#[cfg (feature = "have_gnutls")]
pub const GNUTLS_E_UNEXPECTED_PACKET_LENGTH: c_int = -9;
/// C: gnutls.h: `#define GNUTLS_E_MEMORY_ERROR -25`
#[cfg (feature = "have_gnutls")]
pub const GNUTLS_E_MEMORY_ERROR: c_int = -25;
/// C: gnutls.h: `#define GNUTLS_E_WARNING_ALERT_RECEIVED -16`
#[cfg (feature = "have_gnutls")]
pub const GNUTLS_E_WARNING_ALERT_RECEIVED: c_int = -16;
/// C: gnutls.h: `#define GNUTLS_E_FATAL_ALERT_RECEIVED -12`
#[cfg (feature = "have_gnutls")]
pub const GNUTLS_E_FATAL_ALERT_RECEIVED: c_int = -12;
/// C: gnutls.h: `#define GNUTLS_E_PREMATURE_TERMINATION -110`
#[cfg (feature = "have_gnutls")]
pub const GNUTLS_E_PREMATURE_TERMINATION: c_int = -110;
/// C: gnutls.h: `#define GNUTLS_E_LARGE_PACKET -7`
#[cfg (feature = "have_gnutls")]
pub const GNUTLS_E_LARGE_PACKET: c_int = -7;
/// C: gnutls.h: `#define GNUTLS_E_PUSH_ERROR -53`
#[cfg (feature = "have_gnutls")]
pub const GNUTLS_E_PUSH_ERROR: c_int = -53;
/// C: gnutls.h: `#define GNUTLS_E_SHORT_MEMORY_BUFFER -51`
#[cfg (feature = "have_gnutls")]
pub const GNUTLS_E_SHORT_MEMORY_BUFFER: c_int = -51;
/// C: <gnutls/gnutls.h> 856-859: the `gnutls_x509_crt_fmt_t` value `GNUTLS_X509_FMT_PEM`
#[cfg (feature = "have_gnutls")]
pub const GNUTLS_X509_FMT_DER: gnutls_x509_crt_fmt_t = 0;
#[cfg (feature = "have_gnutls")]
pub const GNUTLS_X509_FMT_PEM: gnutls_x509_crt_fmt_t = 1;
/// C: gnutls.h: `#define GNUTLS_E_APPLICATION_ERROR_MIN -65500` (`src/gnutls.h` 58-59 builds the
/// two `GNUTLS_EMACS_ERROR_*` codes on it; both are beyond this slice and stay owed).
#[cfg (feature = "have_gnutls")]
pub const GNUTLS_E_APPLICATION_ERROR_MIN: c_int = -65500;

#[cfg(feature = "have_gnutls")]
pub const GNUTLS_DIG_SHA1: gnutls_digest_algorithm_t = 3;
#[cfg(all(feature = "have_gnutls", feature = "have_gnutls_keyid_use_sha256"))]
pub const GNUTLS_KEYID_USE_SHA256: c_uint = 1;

#[cfg(feature = "have_gnutls")]
pub const GNUTLS_TLS1_2: gnutls_protocol_t = 4;
#[cfg(feature = "have_gnutls")]
pub const GNUTLS_CRT_PRINT_FULL: gnutls_certificate_print_formats_t = 0;
#[cfg(feature = "have_gnutls")]
pub const GNUTLS_CERT_INVALID: c_uint = 1 << 1;
#[cfg(feature = "have_gnutls")]
pub const GNUTLS_CERT_REVOKED: c_uint = 1 << 5;
#[cfg(feature = "have_gnutls")]
pub const GNUTLS_CERT_SIGNER_NOT_FOUND: c_uint = 1 << 6;
#[cfg(feature = "have_gnutls")]
pub const GNUTLS_CERT_SIGNER_NOT_CA: c_uint = 1 << 7;
#[cfg(feature = "have_gnutls")]
pub const GNUTLS_CERT_INSECURE_ALGORITHM: c_uint = 1 << 8;
#[cfg(feature = "have_gnutls")]
pub const GNUTLS_CERT_NOT_ACTIVATED: c_uint = 1 << 9;
#[cfg(feature = "have_gnutls")]
pub const GNUTLS_CERT_EXPIRED: c_uint = 1 << 10;
#[cfg(feature = "have_gnutls")]
pub const GNUTLS_CERT_SIGNATURE_FAILURE: c_uint = 1 << 11;
#[cfg(feature = "have_gnutls")]
pub const GNUTLS_CERT_REVOCATION_DATA_SUPERSEDED: c_uint = 1 << 12;
#[cfg(feature = "have_gnutls")]
pub const GNUTLS_CERT_REVOCATION_DATA_ISSUED_IN_FUTURE: c_uint = 1 << 15;
#[cfg(feature = "have_gnutls")]
pub const GNUTLS_CERT_SIGNER_CONSTRAINTS_FAILURE: c_uint = 1 << 16;
#[cfg(feature = "have_gnutls")]
pub const GNUTLS_CERT_PURPOSE_MISMATCH: c_uint = 1 << 18;
#[cfg(feature = "have_gnutls")]
pub const GNUTLS_CERT_MISSING_OCSP_STATUS: c_uint = 1 << 19;
#[cfg(feature = "have_gnutls")]
pub const GNUTLS_CERT_INVALID_OCSP_STATUS: c_uint = 1 << 20;

/* The functions C 30-1130 calls on the non-`WINDOWSNT` arm; C reaches them through
   `<gnutls/gnutls.h>`'s own declarations.  On the `windowsnt` arm C 460-587 `#define`s each of
   these names to its `fn_*` loaded pointer instead (carried as the comment block at C 460-587),
   which is why these declarations are `not (feature = "windowsnt")`.  */
/// C: <gnutls/gnutls.h> 2290: `int gnutls_handshake(gnutls_session_t session);`
#[cfg (all (feature = "have_gnutls", not (feature = "windowsnt")))]
unsafe extern "C" {
  pub fn gnutls_handshake (session: gnutls_session_t) -> c_int;
}
/// C: <gnutls/gnutls.h> 2421-2424: `ssize_t gnutls_record_send(gnutls_session_t, const void *, size_t);`
#[cfg (all (feature = "have_gnutls", not (feature = "windowsnt")))]
unsafe extern "C" {
  pub fn gnutls_record_send (session: gnutls_session_t, data: *const c_void,
                             data_size: size_t) -> ssize_t;
}
/// C: <gnutls/gnutls.h> 2425-2427: `ssize_t gnutls_record_recv(gnutls_session_t, void *, size_t);`
#[cfg (all (feature = "have_gnutls", not (feature = "windowsnt")))]
unsafe extern "C" {
  pub fn gnutls_record_recv (session: gnutls_session_t, data: *mut c_void,
                             data_size: size_t) -> ssize_t;
}
/// C: <gnutls/gnutls.h> 2432: `size_t gnutls_record_check_pending(gnutls_session_t session);`
#[cfg (all (feature = "have_gnutls", not (feature = "windowsnt")))]
unsafe extern "C" {
  pub fn gnutls_record_check_pending (session: gnutls_session_t) -> size_t;
}
/// C: <gnutls/gnutls.h> 2475: `int gnutls_error_is_fatal(int error);`
#[cfg (all (feature = "have_gnutls", not (feature = "windowsnt")))]
unsafe extern "C" {
  pub fn gnutls_error_is_fatal (error: c_int) -> c_int;
}
/// C: <gnutls/gnutls.h> 2472: `const char *gnutls_strerror(int error);`
#[cfg (all (feature = "have_gnutls", not (feature = "windowsnt")))]
unsafe extern "C" {
  pub fn gnutls_strerror (error: c_int) -> *const c_char;
}
/// C: <gnutls/gnutls.h> 2258: `gnutls_alert_description_t gnutls_alert_get(gnutls_session_t session);`
#[cfg (all (feature = "have_gnutls", not (feature = "windowsnt")))]
unsafe extern "C" {
  pub fn gnutls_alert_get (session: gnutls_session_t) -> gnutls_alert_description_t;
}
/// C: <gnutls/gnutls.h> 2259: `const char *gnutls_alert_get_name(gnutls_alert_description_t alert);`
#[cfg (all (feature = "have_gnutls", not (feature = "windowsnt")))]
unsafe extern "C" {
  pub fn gnutls_alert_get_name (alert: gnutls_alert_description_t) -> *const c_char;
}
/// C: <gnutls/gnutls.h> 2417-2419: `void gnutls_transport_set_ptr2(gnutls_session_t,
/// gnutls_transport_ptr_t recv_ptr, gnutls_transport_ptr_t send_ptr);`
#[cfg (all (feature = "have_gnutls", not (feature = "windowsnt")))]
unsafe extern "C" {
  pub fn gnutls_transport_set_ptr2 (session: gnutls_session_t, recv_ptr: gnutls_transport_ptr_t,
                                    send_ptr: gnutls_transport_ptr_t);
}
/// C: <gnutls/gnutls.h> 2435-2437: `void gnutls_transport_set_errno_function(gnutls_session_t session,
/// gnutls_errno_func errno_func);`
#[cfg (all (feature = "have_gnutls", not (feature = "windowsnt")))]
unsafe extern "C" {
  pub fn gnutls_transport_set_errno_function (session: gnutls_session_t,
                                              errno_func: gnutls_errno_func);
}
/// C: <gnutls/gnutls.h> 2428-2430: `void gnutls_transport_set_pull_function(gnutls_session_t session,
/// gnutls_pull_func pull_func);`
#[cfg (all (feature = "have_gnutls", not (feature = "windowsnt")))]
unsafe extern "C" {
  pub fn gnutls_transport_set_pull_function (session: gnutls_session_t, pull_func: gnutls_pull_func);
}
/// C: <gnutls/gnutls.h> 2421-2424: `void gnutls_transport_set_push_function(gnutls_session_t session,
/// gnutls_push_func push_func);`
#[cfg (all (feature = "have_gnutls", not (feature = "windowsnt")))]
unsafe extern "C" {
  pub fn gnutls_transport_set_push_function (session: gnutls_session_t, push_func: gnutls_push_func);
}
/// C: <gnutls/gnutls.h> 2438-2439: `void gnutls_transport_set_errno(gnutls_session_t session, int err);`
#[cfg (all (feature = "have_gnutls", not (feature = "windowsnt")))]
unsafe extern "C" {
  pub fn gnutls_transport_set_errno (session: gnutls_session_t, err: c_int);
}
/// C: x509.h 134: `void gnutls_x509_crt_deinit(gnutls_x509_crt_t cert);`
#[cfg (all (feature = "have_gnutls", not (feature = "windowsnt")))]
unsafe extern "C" {
  pub fn gnutls_x509_crt_deinit (cert: gnutls_x509_crt_t);
}
/// C: x509.h 130-132: `int gnutls_x509_crt_export(gnutls_x509_crt_t cert,
/// gnutls_x509_crt_fmt_t format, void *output_data, size_t *output_data_size);`
#[cfg (all (feature = "have_gnutls", not (feature = "windowsnt")))]
unsafe extern "C" {
  pub fn gnutls_x509_crt_export (cert: gnutls_x509_crt_t, format: gnutls_x509_crt_fmt_t,
                                 output_data: *mut c_void, output_data_size: *mut size_t) -> c_int;
}
/// C: <gnutls/gnutls.h> 2287: `void gnutls_deinit(gnutls_session_t session);`
#[cfg (all (feature = "have_gnutls", not (feature = "windowsnt")))]
unsafe extern "C" {
  pub fn gnutls_deinit (session: gnutls_session_t);
}
/// C: <gnutls/gnutls.h> 2081: `void gnutls_certificate_free_credentials(gnutls_certificate_credentials_t
/// sc);`
#[cfg (all (feature = "have_gnutls", not (feature = "windowsnt")))]
unsafe extern "C" {
  pub fn gnutls_certificate_free_credentials (sc: gnutls_certificate_credentials_t);
}
/// C: <gnutls/gnutls.h> 2074: `void gnutls_anon_free_client_credentials(gnutls_anon_client_credentials_t
/// sc);`
#[cfg (all (feature = "have_gnutls", not (feature = "windowsnt")))]
unsafe extern "C" {
  pub fn gnutls_anon_free_client_credentials (sc: gnutls_anon_client_credentials_t);
}

/* The names this slice takes from headers whose mirrors do not carry them yet (P-72 rule 4):
   `message`, `memory_full` and `nconc2`, declared at this use site the way
   `src/process.rs` 1609, `src/image.rs` 203 and `src/font.rs` 381 declare
   them at theirs.  Their single declaration point is owed back to the mirrors
   (docs/ports/gnutls.md's `## Owed`).  */
#[cfg (feature = "have_gnutls")]
unsafe extern "C-unwind" {
  /// C: lisp.h 4389: `extern void message (const char *, ...) ATTRIBUTE_FORMAT_PRINTF (1, 2);`
  pub fn message (format: *const c_char, ...);
  /// C: lisp.h 4432: `extern AVOID memory_full (size_t);`
  pub fn memory_full (nbytes: size_t) -> !;

}


// Additional directly linked declarations use the existing measured loader
// signatures. These are declarations only; actual SDK/API and enabled product
// validation remain pending until the complete TLS business path is written.
#[cfg(all(feature = "have_gnutls", not(feature = "windowsnt")))]
unsafe extern "C" {
  pub fn gnutls_anon_allocate_client_credentials(arg0: *mut gnutls_anon_client_credentials_t) -> c_int;
  pub fn gnutls_bye(arg0: gnutls_session_t, arg1: gnutls_close_request_t) -> c_int;
  pub fn gnutls_certificate_allocate_credentials(arg0: *mut gnutls_certificate_credentials_t) -> c_int;
  pub fn gnutls_certificate_get_peers(arg0: gnutls_session_t, arg1: *mut c_uint) -> *const gnutls_datum_t;
  pub fn gnutls_certificate_set_verify_flags(arg0: gnutls_certificate_credentials_t, arg1: c_uint);
  pub fn gnutls_certificate_set_x509_crl_file(arg0: gnutls_certificate_credentials_t, arg1: *const c_char, arg2: gnutls_x509_crt_fmt_t) -> c_int;
  pub fn gnutls_certificate_set_x509_key_file(arg0: gnutls_certificate_credentials_t, arg1: *const c_char, arg2: *const c_char, arg3: gnutls_x509_crt_fmt_t) -> c_int;
  pub fn gnutls_certificate_set_x509_key_file2(arg0: gnutls_certificate_credentials_t, arg1: *const c_char, arg2: *const c_char, arg3: gnutls_x509_crt_fmt_t, arg4: *const c_char, arg5: c_uint) -> c_int;
  pub fn gnutls_certificate_set_x509_system_trust(arg0: gnutls_certificate_credentials_t) -> c_int;
  pub fn gnutls_certificate_set_x509_trust_file(arg0: gnutls_certificate_credentials_t, arg1: *const c_char, arg2: gnutls_x509_crt_fmt_t) -> c_int;
  pub fn gnutls_certificate_type_get(arg0: gnutls_session_t) -> gnutls_certificate_type_t;
  pub fn gnutls_certificate_verify_peers2(arg0: gnutls_session_t, arg1: *mut c_uint) -> c_int;
  pub fn gnutls_credentials_set(arg0: gnutls_session_t, arg1: gnutls_credentials_type_t, arg2: *mut c_void) -> c_int;
  pub fn gnutls_dh_set_prime_bits(arg0: gnutls_session_t, arg1: c_uint);
  pub fn gnutls_dh_get_prime_bits(arg0: gnutls_session_t) -> c_int;
  pub fn gnutls_global_init() -> c_int;
  pub fn gnutls_global_set_log_function(arg0: gnutls_log_func);
  pub fn gnutls_global_set_audit_log_function(arg0: gnutls_audit_log_func);
  pub fn gnutls_global_set_log_level(arg0: c_int);
  pub fn gnutls_init(arg0: *mut gnutls_session_t, arg1: c_uint) -> c_int;
  pub fn gnutls_priority_set_direct(arg0: gnutls_session_t, arg1: *const c_char, arg2: *mut *const c_char) -> c_int;
  pub fn gnutls_x509_crt_check_hostname(arg0: gnutls_x509_crt_t, arg1: *const c_char) -> c_uint;
  pub fn gnutls_x509_crt_check_issuer(arg0: gnutls_x509_crt_t, arg1: gnutls_x509_crt_t) -> c_uint;
  pub fn gnutls_x509_crt_import(arg0: gnutls_x509_crt_t, arg1: *const gnutls_datum_t, arg2: gnutls_x509_crt_fmt_t) -> c_int;
  pub fn gnutls_x509_crt_init(arg0: *mut gnutls_x509_crt_t) -> c_int;
  pub fn gnutls_x509_crt_get_fingerprint(arg0: gnutls_x509_crt_t, arg1: gnutls_digest_algorithm_t, arg2: *mut c_void, arg3: *mut size_t) -> c_int;
  pub fn gnutls_x509_crt_get_version(arg0: gnutls_x509_crt_t) -> c_int;
  pub fn gnutls_x509_crt_get_serial(arg0: gnutls_x509_crt_t, arg1: *mut c_void, arg2: *mut size_t) -> c_int;
  pub fn gnutls_x509_crt_get_issuer_dn(arg0: gnutls_x509_crt_t, arg1: *mut c_char, arg2: *mut size_t) -> c_int;
  pub fn gnutls_x509_crt_get_activation_time(arg0: gnutls_x509_crt_t) -> time_t;
  pub fn gnutls_x509_crt_get_expiration_time(arg0: gnutls_x509_crt_t) -> time_t;
  pub fn gnutls_x509_crt_get_dn(arg0: gnutls_x509_crt_t, arg1: *mut c_char, arg2: *mut size_t) -> c_int;
  pub fn gnutls_x509_crt_get_pk_algorithm(arg0: gnutls_x509_crt_t, arg1: *mut c_uint) -> c_int;
  pub fn gnutls_x509_crt_print(arg0: gnutls_x509_crt_t, arg1: gnutls_certificate_print_formats_t, arg2: *mut gnutls_datum_t) -> c_int;
  pub fn gnutls_pk_algorithm_get_name(arg0: gnutls_pk_algorithm_t) -> *const c_char;
  pub fn gnutls_pk_bits_to_sec_param(arg0: gnutls_pk_algorithm_t, arg1: c_uint) -> gnutls_sec_param_t;
  pub fn gnutls_x509_crt_get_issuer_unique_id(arg0: gnutls_x509_crt_t, arg1: *mut c_char, arg2: *mut size_t) -> c_int;
  pub fn gnutls_x509_crt_get_subject_unique_id(arg0: gnutls_x509_crt_t, arg1: *mut c_char, arg2: *mut size_t) -> c_int;
  pub fn gnutls_x509_crt_get_signature_algorithm(arg0: gnutls_x509_crt_t) -> c_int;
  pub fn gnutls_x509_crt_get_key_id(arg0: gnutls_x509_crt_t, arg1: c_uint, arg2: *mut c_uchar, arg3: *mut size_t) -> c_int;
  pub fn gnutls_sec_param_get_name(arg0: gnutls_sec_param_t) -> *const c_char;
  pub fn gnutls_sign_get_name(arg0: gnutls_sign_algorithm_t) -> *const c_char;
  pub fn gnutls_server_name_set(arg0: gnutls_session_t, arg1: gnutls_server_name_type_t, arg2: *const c_void, arg3: size_t) -> c_int;
  pub fn gnutls_kx_get(arg0: gnutls_session_t) -> gnutls_kx_algorithm_t;
  pub fn gnutls_kx_get_name(arg0: gnutls_kx_algorithm_t) -> *const c_char;
  pub fn gnutls_protocol_get_version(arg0: gnutls_session_t) -> gnutls_protocol_t;
  pub fn gnutls_protocol_get_name(arg0: gnutls_protocol_t) -> *const c_char;
  pub fn gnutls_cipher_get(arg0: gnutls_session_t) -> gnutls_cipher_algorithm_t;
  pub fn gnutls_cipher_get_name(arg0: gnutls_cipher_algorithm_t) -> *const c_char;
  pub fn gnutls_mac_get(arg0: gnutls_session_t) -> gnutls_mac_algorithm_t;
  pub fn gnutls_mac_get_name(arg0: gnutls_mac_algorithm_t) -> *const c_char;
  pub fn gnutls_compression_get(arg0: gnutls_session_t) -> gnutls_compression_method_t;
  pub fn gnutls_compression_get_name(arg0: gnutls_compression_method_t) -> *const c_char;
  pub fn gnutls_safe_renegotiation_status(arg0: gnutls_session_t) -> c_uint;
  pub fn gnutls_mac_list() -> *const gnutls_mac_algorithm_t;
  pub fn gnutls_mac_get_nonce_size(arg0: gnutls_mac_algorithm_t) -> size_t;
  pub fn gnutls_mac_get_key_size(arg0: gnutls_mac_algorithm_t) -> size_t;
  #[cfg(feature = "have_gnutls_digest_list")]
  pub fn gnutls_digest_list() -> *const gnutls_digest_algorithm_t;
  #[cfg(feature = "have_gnutls_digest_list")]
  pub fn gnutls_digest_get_name(arg0: gnutls_digest_algorithm_t) -> *const c_char;
  pub fn gnutls_cipher_list() -> *const gnutls_cipher_algorithm_t;
  #[cfg(feature = "have_gnutls_cipher_get_iv_size")]
  pub fn gnutls_cipher_get_iv_size(arg0: gnutls_cipher_algorithm_t) -> c_uint;
  pub fn gnutls_cipher_get_key_size(arg0: gnutls_cipher_algorithm_t) -> size_t;
  pub fn gnutls_cipher_get_block_size(arg0: gnutls_cipher_algorithm_t) -> c_uint;
  #[cfg(feature = "have_gnutls_cipher_get_tag_size")]
  pub fn gnutls_cipher_get_tag_size(arg0: gnutls_cipher_algorithm_t) -> c_uint;
  pub fn gnutls_cipher_init(arg0: *mut gnutls_cipher_hd_t, arg1: gnutls_cipher_algorithm_t, arg2: *const gnutls_datum_t, arg3: *const gnutls_datum_t) -> c_int;
  pub fn gnutls_cipher_set_iv(arg0: gnutls_cipher_hd_t, arg1: *mut c_void, arg2: size_t);
  pub fn gnutls_cipher_encrypt2(arg0: gnutls_cipher_hd_t, arg1: *const c_void, arg2: size_t, arg3: *mut c_void, arg4: size_t) -> c_int;
  pub fn gnutls_cipher_deinit(arg0: gnutls_cipher_hd_t);
  pub fn gnutls_cipher_decrypt2(arg0: gnutls_cipher_hd_t, arg1: *const c_void, arg2: size_t, arg3: *mut c_void, arg4: size_t) -> c_int;
  pub fn gnutls_aead_cipher_init(arg0: *mut gnutls_aead_cipher_hd_t, arg1: gnutls_cipher_algorithm_t, arg2: *const gnutls_datum_t) -> c_int;
  pub fn gnutls_aead_cipher_deinit(arg0: gnutls_aead_cipher_hd_t);
  pub fn gnutls_aead_cipher_encrypt(arg0: gnutls_aead_cipher_hd_t, arg1: *const c_void, arg2: size_t, arg3: *const c_void, arg4: size_t, arg5: size_t, arg6: *const c_void, arg7: size_t, arg8: *mut c_void, arg9: *mut size_t) -> c_int;
  pub fn gnutls_aead_cipher_decrypt(arg0: gnutls_aead_cipher_hd_t, arg1: *const c_void, arg2: size_t, arg3: *const c_void, arg4: size_t, arg5: size_t, arg6: *const c_void, arg7: size_t, arg8: *mut c_void, arg9: *mut size_t) -> c_int;
  pub fn gnutls_session_etm_status(arg0: gnutls_session_t) -> c_uint;
  pub fn gnutls_hmac_init(arg0: *mut gnutls_hmac_hd_t, arg1: gnutls_mac_algorithm_t, arg2: *const c_void, arg3: size_t) -> c_int;
  pub fn gnutls_hmac_get_len(arg0: gnutls_mac_algorithm_t) -> c_uint;
  pub fn gnutls_hmac(arg0: gnutls_hmac_hd_t, arg1: *const c_void, arg2: size_t) -> c_int;
  pub fn gnutls_hmac_deinit(arg0: gnutls_hmac_hd_t, arg1: *mut c_void);
  pub fn gnutls_hmac_output(arg0: gnutls_hmac_hd_t, arg1: *mut c_void);
  pub fn gnutls_hash_init(arg0: *mut gnutls_hash_hd_t, arg1: gnutls_digest_algorithm_t) -> c_int;
  pub fn gnutls_hash_get_len(arg0: gnutls_digest_algorithm_t) -> c_uint;
  pub fn gnutls_hash(arg0: gnutls_hash_hd_t, arg1: *const c_void, arg2: size_t) -> c_int;
  pub fn gnutls_hash_deinit(arg0: gnutls_hash_hd_t, arg1: *mut c_void);
  pub fn gnutls_hash_output(arg0: gnutls_hash_hd_t, arg1: *mut c_void);
  pub fn gnutls_ext_get_name(arg0: c_uint) -> *const c_char;
}


// Actual installed public header values. Version values are generated from the
// linked library header by the owned target configuration, never guessed.
#[cfg(feature = "have_gnutls")]
const GNUTLS_CRT_X509: gnutls_certificate_type_t = 1;
#[cfg(feature = "have_gnutls")]
const GNUTLS_CLIENT: c_uint = 1 << 1;
#[cfg(feature = "have_gnutls")]
const GNUTLS_NONBLOCK: c_uint = 1 << 3;
#[cfg(feature = "have_gnutls")]
const GNUTLS_CRD_CERTIFICATE: gnutls_credentials_type_t = 1;
#[cfg(feature = "have_gnutls")]
const GNUTLS_CRD_ANON: gnutls_credentials_type_t = 2;
#[cfg(feature = "have_gnutls")]
const GNUTLS_SHUT_RDWR: gnutls_close_request_t = 0;
#[cfg(feature = "have_gnutls")]
const GNUTLS_SHUT_WR: gnutls_close_request_t = 1;
#[cfg(feature = "have_gnutls")]
const GNUTLS_VERIFY_ALLOW_X509_V1_CA_CRT: c_uint = 0;
#[cfg(feature = "have_gnutls")]
const GNUTLS_PKCS_PLAIN: c_uint = 1;
#[cfg(feature = "have_gnutls")]
const GNUTLS_PKCS_NULL_PASSWORD: c_uint = 1 << 8;
#[cfg(feature = "have_gnutls3")]
const GNUTLS_CIPHER_NULL: gnutls_cipher_algorithm_t = 1;
#[cfg(feature = "have_gnutls3")]
const GNUTLS_CIPHER_UNKNOWN: gnutls_cipher_algorithm_t = 0;
#[cfg(feature = "have_gnutls3")]
const GNUTLS_MAC_UNKNOWN: gnutls_mac_algorithm_t = 0;
#[cfg(feature = "have_gnutls3")]
const GNUTLS_DIG_UNKNOWN: gnutls_digest_algorithm_t = 0;

#[cfg(all(feature = "have_gnutls", feature = "windowsnt"))]
compile_error!("The enabled Windows TLS loader requires its own complete integration; this target is not advertised.");

#[cfg(feature = "have_gnutls")]
#[unsafe(no_mangle)]
pub static mut global_gnutls_log_level: EMACS_INT = 0;

#[cfg(all(feature = "have_gnutls3", not(feature = "windowsnt")))]
unsafe extern "C-unwind" {
  fn extract_data_from_object(spec: Lisp_Object, start: *mut ptrdiff_t,
                              end: *mut ptrdiff_t) -> *mut c_char;
}
#[cfg(all(feature = "have_gnutls3", not(feature = "windowsnt")))]
unsafe extern "C" {
  fn memset_explicit(data: *mut c_void, value: c_int, bytes: size_t) -> *mut c_void;
}

/* The local logging macros use this unit's private callbacks. Process-facing
   types, stages and public prototypes have their shared header definitions. */
/// C: gnutls.h 64-68: `#define GNUTLS_LOG(level, max, string) do { if ((level) <= (max))
/// gnutls_log_function (level, "(Emacs) " string); } while (false)` -- C's adjacent-literal
/// concatenation is `concat!`, so `$string` is a literal, which is what every call site inside
/// C 30-1130 passes.  A macro body resolves its names where it is defined, so this definition
/// refers to `gnutls_log_function` directly (PORTING.md 2.1).
#[cfg (feature = "have_gnutls")]
macro_rules! GNUTLS_LOG {
  ($level:expr, $max:expr, $string:literal) => {
    {
      if ($level) <= ($max)
	{
	  gnutls_log_function ($level,
			       concat! ("(Emacs) ", $string, "\0").as_ptr () as *const c_char);
	}
    }
  };
}
/// C: gnutls.h 70-74: `#define GNUTLS_LOG2(level, max, string, extra) ... gnutls_log_function2
/// (level, "(Emacs) " string, extra) ...`
#[cfg (feature = "have_gnutls")]
macro_rules! GNUTLS_LOG2 {
  ($level:expr, $max:expr, $string:literal, $extra:expr) => {
    {
      if ($level) <= ($max)
	{
	  gnutls_log_function2 ($level,
				concat! ("(Emacs) ", $string, "\0").as_ptr () as *const c_char,
				$extra);
	}
    }
  };
}

/* ----------------------------------------------------------------------------------------------
   C 30: `#ifdef HAVE_GNUTLS` -- the file gate.  Both arms are carried: the `#ifdef` side is the
   text below (C 30-1130 of this slice; C 1131-2964 still owed), and the `#endif` side is C 2336's
   own live tail -- `Fgnutls_available_p`'s frozen body and `syms_of_gnutls` -- which sits at the
   end of this unit.  */

/* C 32-80: the ten version conditions and the eleven macros they define.  A `#define` has no Rust
   counterpart (P-86 ①), so each is recorded here and every later `#ifdef` carries the feature
   named after the macro it tests; the frozen `GNUTLS_VERSION_NUMBER` is undefined, which the
   evaluator reads as 0 (`tools/check-cond-eval.py src/gnutls.c` -> 104 conditions, 0 unknown, 0
   disagreements).  `docs/ports/gnutls.md`'s `## Decisions` carries the same table.

     C 32-34   `# if GNUTLS_VERSION_NUMBER >= 0x030014`  -> HAVE_GNUTLS_X509_SYSTEM_TRUST
     C 36-39   `# if GNUTLS_VERSION_NUMBER >= 0x030200`  -> HAVE_GNUTLS_CERTIFICATE_SET_X509_KEY_FILE2
                                                          -> HAVE_GNUTLS_CIPHER_GET_IV_SIZE
     C 41-44   `# if GNUTLS_VERSION_NUMBER >= 0x030202`  -> HAVE_GNUTLS_CIPHER_GET_TAG_SIZE
                                                          -> HAVE_GNUTLS_DIGEST_LIST
     C 46-48   `# if GNUTLS_VERSION_NUMBER >= 0x030205`  -> HAVE_GNUTLS_EXT__DUMBFW
     C 50-52   `# if GNUTLS_VERSION_NUMBER >= 0x030400`  -> HAVE_GNUTLS_ETM_STATUS
     C 54-56   `# if GNUTLS_VERSION_NUMBER >= 0x030401`  -> HAVE_GNUTLS_KEYID_USE_SHA256
     C 58-60   `# if GNUTLS_VERSION_NUMBER <  0x030600`  -> HAVE_GNUTLS_COMPRESSION_GET
     C 64-66   `# if GNUTLS_VERSION_NUMBER >= 0x030300`  -> HAVE_GNUTLS_MAC_GET_NONCE_SIZE
     C 68-70   `# if GNUTLS_VERSION_NUMBER >= 0x030501`  -> HAVE_GNUTLS_EXT_GET_NAME
     C 78-80   `# if GNUTLS_VERSION_NUMBER >= 0x030501`  -> HAVE_GNUTLS_AEAD

   (C 62-63 and 71-77 are C's own prose comments, carried at the segment's head.)  */

/* C 82-86: `# ifdef WINDOWSNT` -> `feature = "windowsnt"`.  `<windows.h>` (C 83) has no file in
   the reference tree, so `HMODULE` is the pointer PORTING.md 0.2 maps it to (`*mut c_void`, the
   spelling `src/include/w32common.rs` 189-192 already uses for `hbfont_init_w32_funcs`).  */
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
const _: () = ();
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
const _: () = ();

/// C 88: `static int emacs_gnutls_handle_error (gnutls_session_t, int);` -- C's forward
/// declaration of a definition later in this same file.  Rust has no counterpart, so the line is
/// dropped and the definition sits at its own C position (PORTING.md 0.1-12/P-91).
/// C 90: `static bool gnutls_global_initialized;` -- C's tentative definition, i.e. zero
/// (PORTING.md 0.1-13).
#[cfg (feature = "have_gnutls")]
static mut gnutls_global_initialized: bool = false;
/// C 92-93: `static void gnutls_log_function (int, const char *);` /
/// `static void gnutls_log_function2 (int, const char *, const char *);` -- the same dropped
/// forward declarations (0.1-12); the definitions are at C 615-618 and C 622-626.
/// C 94-96: `# ifdef HAVE_GNUTLS3` -> `feature = "have_gnutls3"`, carrying
/// `static void gnutls_audit_log_function (gnutls_session_t, const char *);` -- that arm's own
/// forward declaration, defined at C 604-611.
/// C 99-102: `enum extra_peer_verification { CERTIFICATE_NOT_MATCHING = 2 };` -- the port's
/// C-enum shape (`pub type = c_int` plus the member; C 100-101 are the enum's two lines and
/// C 98/103 the braces).
#[cfg (feature = "have_gnutls")]
pub type extra_peer_verification = c_int;
/// C 101: `CERTIFICATE_NOT_MATCHING = 2`
#[cfg (feature = "have_gnutls")]
pub const CERTIFICATE_NOT_MATCHING: extra_peer_verification = 2;

/* ----------------------------------------------------------------------------------------------
   C 104-589: `# ifdef WINDOWSNT` -- the lazy loading of GnuTLS's entry points from the DLL.
   `WINDOWSNT` is `undef` in the frozen configuration, so no line below is compiled on this target
   (P-51 names the non-frozen side), and this machine has no Windows cross toolchain (no
   `*-w64-mingw32-gcc`), so this arm's evidence is `text-only` (P-86 ③⑵): there is no compile
   reading of it.  */

/// C: w32common.h 77-79: `#define DEF_DLL_FN(type, func, ...)  typedef type (CDECL *W32_PFN_##func)`
/// `__VA_ARGS__; static W32_PFN_##func fn_##func` -- the macro
/// `src/include/w32common.rs` keeps `deferred` (P-22).  This unit is its third consumer
/// (`src/decompress.rs` and `src/lcms.rs` are the first two) and repeats the long
/// form for the same reason: a unit is its own crate.  C pastes `W32_PFN_##func` and `fn_##func`
/// together, which Rust cannot do, so both generated names are parameters here and C's
/// `__VA_ARGS__` (the parameter list, with any trailing attributes) is the `$args` fragment.
/// C's `CDECL` is `__cdecl`, empty on this target's ABI.
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
macro_rules! DEF_DLL_FN {
  ($type:ty, $func:ident, $pfn:ident, $fnvar:ident, ($($arg:ty),* $(,)?)) => {
    pub type $pfn = Option<unsafe extern "C" fn ($($arg),*) -> $type>;
    static mut $fnvar: $pfn = None;
  };
}
/// C: w32common.h 82-89: `#define LOAD_DLL_FN(lib, func) do { fn_##func = (W32_PFN_##func)`
/// `get_proc_addr (lib, #func); if (!fn_##func) return false; } while (false)`.  `do { … } while
/// (false)` is a block expression (PORTING.md 2.1); C's `#func` is the stringified name, also a
/// parameter here; the assignment's cast between two function-pointer types is C's, and Rust has
/// no `as` cast to an `Option<fn>` type, so it is the `transmute` that the equal widths make the
/// same operation.  `get_proc_addr` is the mirror's own inline definition
/// (`src/include/w32common.rs` 165-170).
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
macro_rules! LOAD_DLL_FN {
  ($lib:expr, $func:ident, $pfn:ident, $fnvar:ident, $name:expr) => {
    {
      $fnvar = std::mem::transmute::<VOIDFNPTR, $pfn> (get_proc_addr ($lib, $name.as_ptr ()));
      if $fnvar.is_none () { return false; }
    }
  };
}
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (gnutls_alert_description_t, gnutls_alert_get, W32_PFN_gnutls_alert_get,
             fn_gnutls_alert_get, (gnutls_session_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (*const c_char, gnutls_alert_get_name, W32_PFN_gnutls_alert_get_name,
             fn_gnutls_alert_get_name, (gnutls_alert_description_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (c_int, gnutls_anon_allocate_client_credentials,
             W32_PFN_gnutls_anon_allocate_client_credentials,
             fn_gnutls_anon_allocate_client_credentials,
             (*mut gnutls_anon_client_credentials_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! ((), gnutls_anon_free_client_credentials,
             W32_PFN_gnutls_anon_free_client_credentials,
             fn_gnutls_anon_free_client_credentials, (gnutls_anon_client_credentials_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (c_int, gnutls_bye, W32_PFN_gnutls_bye, fn_gnutls_bye,
             (gnutls_session_t, gnutls_close_request_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (c_int, gnutls_certificate_allocate_credentials,
             W32_PFN_gnutls_certificate_allocate_credentials,
             fn_gnutls_certificate_allocate_credentials,
             (*mut gnutls_certificate_credentials_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! ((), gnutls_certificate_free_credentials,
             W32_PFN_gnutls_certificate_free_credentials,
             fn_gnutls_certificate_free_credentials, (gnutls_certificate_credentials_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (*const gnutls_datum_t, gnutls_certificate_get_peers,
             W32_PFN_gnutls_certificate_get_peers, fn_gnutls_certificate_get_peers,
             (gnutls_session_t, *mut c_uint));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! ((), gnutls_certificate_set_verify_flags,
             W32_PFN_gnutls_certificate_set_verify_flags,
             fn_gnutls_certificate_set_verify_flags,
             (gnutls_certificate_credentials_t, c_uint));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (c_int, gnutls_certificate_set_x509_crl_file,
             W32_PFN_gnutls_certificate_set_x509_crl_file,
             fn_gnutls_certificate_set_x509_crl_file,
             (gnutls_certificate_credentials_t, *const c_char, gnutls_x509_crt_fmt_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (c_int, gnutls_certificate_set_x509_key_file,
             W32_PFN_gnutls_certificate_set_x509_key_file,
             fn_gnutls_certificate_set_x509_key_file,
             (gnutls_certificate_credentials_t, *const c_char, *const c_char, gnutls_x509_crt_fmt_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt", feature = "have_gnutls_certificate_set_x509_key_file2"))]
DEF_DLL_FN! (c_int, gnutls_certificate_set_x509_key_file2,
             W32_PFN_gnutls_certificate_set_x509_key_file2,
             fn_gnutls_certificate_set_x509_key_file2,
             (gnutls_certificate_credentials_t, *const c_char, *const c_char, gnutls_x509_crt_fmt_t, *const c_char, c_uint));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt", feature = "have_gnutls_x509_system_trust"))]
DEF_DLL_FN! (c_int, gnutls_certificate_set_x509_system_trust,
             W32_PFN_gnutls_certificate_set_x509_system_trust,
             fn_gnutls_certificate_set_x509_system_trust, (gnutls_certificate_credentials_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (c_int, gnutls_certificate_set_x509_trust_file,
             W32_PFN_gnutls_certificate_set_x509_trust_file,
             fn_gnutls_certificate_set_x509_trust_file,
             (gnutls_certificate_credentials_t, *const c_char, gnutls_x509_crt_fmt_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (gnutls_certificate_type_t, gnutls_certificate_type_get,
             W32_PFN_gnutls_certificate_type_get, fn_gnutls_certificate_type_get,
             (gnutls_session_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (c_int, gnutls_certificate_verify_peers2, W32_PFN_gnutls_certificate_verify_peers2,
             fn_gnutls_certificate_verify_peers2, (gnutls_session_t, *mut c_uint));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (c_int, gnutls_credentials_set, W32_PFN_gnutls_credentials_set,
             fn_gnutls_credentials_set,
             (gnutls_session_t, gnutls_credentials_type_t, *mut c_void));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! ((), gnutls_deinit, W32_PFN_gnutls_deinit, fn_gnutls_deinit, (gnutls_session_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! ((), gnutls_dh_set_prime_bits, W32_PFN_gnutls_dh_set_prime_bits,
             fn_gnutls_dh_set_prime_bits, (gnutls_session_t, c_uint));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (c_int, gnutls_dh_get_prime_bits, W32_PFN_gnutls_dh_get_prime_bits,
             fn_gnutls_dh_get_prime_bits, (gnutls_session_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (c_int, gnutls_error_is_fatal, W32_PFN_gnutls_error_is_fatal,
             fn_gnutls_error_is_fatal, (c_int));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (c_int, gnutls_global_init, W32_PFN_gnutls_global_init, fn_gnutls_global_init, ());
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! ((), gnutls_global_set_log_function, W32_PFN_gnutls_global_set_log_function,
             fn_gnutls_global_set_log_function, (gnutls_log_func));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt", feature = "have_gnutls3"))]
DEF_DLL_FN! ((), gnutls_global_set_audit_log_function,
             W32_PFN_gnutls_global_set_audit_log_function,
             fn_gnutls_global_set_audit_log_function, (gnutls_audit_log_func));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! ((), gnutls_global_set_log_level, W32_PFN_gnutls_global_set_log_level,
             fn_gnutls_global_set_log_level, (c_int));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (c_int, gnutls_handshake, W32_PFN_gnutls_handshake, fn_gnutls_handshake,
             (gnutls_session_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (c_int, gnutls_init, W32_PFN_gnutls_init, fn_gnutls_init,
             (*mut gnutls_session_t, c_uint));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (c_int, gnutls_priority_set_direct, W32_PFN_gnutls_priority_set_direct,
             fn_gnutls_priority_set_direct,
             (gnutls_session_t, *const c_char, *mut *const c_char));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (size_t, gnutls_record_check_pending, W32_PFN_gnutls_record_check_pending,
             fn_gnutls_record_check_pending, (gnutls_session_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (ssize_t, gnutls_record_recv, W32_PFN_gnutls_record_recv, fn_gnutls_record_recv,
             (gnutls_session_t, *mut c_void, size_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (ssize_t, gnutls_record_send, W32_PFN_gnutls_record_send, fn_gnutls_record_send,
             (gnutls_session_t, *const c_void, size_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (*const c_char, gnutls_strerror, W32_PFN_gnutls_strerror, fn_gnutls_strerror,
             (c_int));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! ((), gnutls_transport_set_errno, W32_PFN_gnutls_transport_set_errno,
             fn_gnutls_transport_set_errno, (gnutls_session_t, c_int));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! ((), gnutls_transport_set_ptr2, W32_PFN_gnutls_transport_set_ptr2,
             fn_gnutls_transport_set_ptr2,
             (gnutls_session_t, gnutls_transport_ptr_t, gnutls_transport_ptr_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! ((), gnutls_transport_set_pull_function, W32_PFN_gnutls_transport_set_pull_function,
             fn_gnutls_transport_set_pull_function, (gnutls_session_t, gnutls_pull_func));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! ((), gnutls_transport_set_push_function, W32_PFN_gnutls_transport_set_push_function,
             fn_gnutls_transport_set_push_function, (gnutls_session_t, gnutls_push_func));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (c_int, gnutls_x509_crt_check_hostname, W32_PFN_gnutls_x509_crt_check_hostname,
             fn_gnutls_x509_crt_check_hostname, (gnutls_x509_crt_t, *const c_char));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (c_int, gnutls_x509_crt_check_issuer, W32_PFN_gnutls_x509_crt_check_issuer,
             fn_gnutls_x509_crt_check_issuer, (gnutls_x509_crt_t, gnutls_x509_crt_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! ((), gnutls_x509_crt_deinit, W32_PFN_gnutls_x509_crt_deinit,
             fn_gnutls_x509_crt_deinit, (gnutls_x509_crt_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (c_int, gnutls_x509_crt_export, W32_PFN_gnutls_x509_crt_export,
             fn_gnutls_x509_crt_export,
             (gnutls_x509_crt_t, gnutls_x509_crt_fmt_t, *mut c_void, *mut size_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (c_int, gnutls_x509_crt_import, W32_PFN_gnutls_x509_crt_import,
             fn_gnutls_x509_crt_import,
             (gnutls_x509_crt_t, *const gnutls_datum_t, gnutls_x509_crt_fmt_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (c_int, gnutls_x509_crt_init, W32_PFN_gnutls_x509_crt_init, fn_gnutls_x509_crt_init,
             (*mut gnutls_x509_crt_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (c_int, gnutls_x509_crt_get_fingerprint, W32_PFN_gnutls_x509_crt_get_fingerprint,
             fn_gnutls_x509_crt_get_fingerprint,
             (gnutls_x509_crt_t, gnutls_digest_algorithm_t, *mut c_void, *mut size_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (c_int, gnutls_x509_crt_get_version, W32_PFN_gnutls_x509_crt_get_version,
             fn_gnutls_x509_crt_get_version, (gnutls_x509_crt_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (c_int, gnutls_x509_crt_get_serial, W32_PFN_gnutls_x509_crt_get_serial,
             fn_gnutls_x509_crt_get_serial, (gnutls_x509_crt_t, *mut c_void, *mut size_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (c_int, gnutls_x509_crt_get_issuer_dn, W32_PFN_gnutls_x509_crt_get_issuer_dn,
             fn_gnutls_x509_crt_get_issuer_dn, (gnutls_x509_crt_t, *mut c_char, *mut size_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (time_t, gnutls_x509_crt_get_activation_time,
             W32_PFN_gnutls_x509_crt_get_activation_time,
             fn_gnutls_x509_crt_get_activation_time, (gnutls_x509_crt_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (time_t, gnutls_x509_crt_get_expiration_time,
             W32_PFN_gnutls_x509_crt_get_expiration_time,
             fn_gnutls_x509_crt_get_expiration_time, (gnutls_x509_crt_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (c_int, gnutls_x509_crt_get_dn, W32_PFN_gnutls_x509_crt_get_dn,
             fn_gnutls_x509_crt_get_dn, (gnutls_x509_crt_t, *mut c_char, *mut size_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (c_int, gnutls_x509_crt_get_pk_algorithm, W32_PFN_gnutls_x509_crt_get_pk_algorithm,
             fn_gnutls_x509_crt_get_pk_algorithm, (gnutls_x509_crt_t, *mut c_uint));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (c_int, gnutls_x509_crt_print, W32_PFN_gnutls_x509_crt_print,
             fn_gnutls_x509_crt_print,
             (gnutls_x509_crt_t, gnutls_certificate_print_formats_t, *mut gnutls_datum_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (*const c_char, gnutls_pk_algorithm_get_name, W32_PFN_gnutls_pk_algorithm_get_name,
             fn_gnutls_pk_algorithm_get_name, (gnutls_pk_algorithm_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (gnutls_sec_param_t, gnutls_pk_bits_to_sec_param, W32_PFN_gnutls_pk_bits_to_sec_param,
             fn_gnutls_pk_bits_to_sec_param, (gnutls_pk_algorithm_t, c_uint));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (c_int, gnutls_x509_crt_get_issuer_unique_id,
             W32_PFN_gnutls_x509_crt_get_issuer_unique_id,
             fn_gnutls_x509_crt_get_issuer_unique_id,
             (gnutls_x509_crt_t, *mut c_char, *mut size_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (c_int, gnutls_x509_crt_get_subject_unique_id,
             W32_PFN_gnutls_x509_crt_get_subject_unique_id,
             fn_gnutls_x509_crt_get_subject_unique_id,
             (gnutls_x509_crt_t, *mut c_char, *mut size_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (c_int, gnutls_x509_crt_get_signature_algorithm,
             W32_PFN_gnutls_x509_crt_get_signature_algorithm,
             fn_gnutls_x509_crt_get_signature_algorithm, (gnutls_x509_crt_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (c_int, gnutls_x509_crt_get_key_id, W32_PFN_gnutls_x509_crt_get_key_id,
             fn_gnutls_x509_crt_get_key_id,
             (gnutls_x509_crt_t, c_uint, *mut c_uchar, *mut size_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (*const c_char, gnutls_sec_param_get_name, W32_PFN_gnutls_sec_param_get_name,
             fn_gnutls_sec_param_get_name, (gnutls_sec_param_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (*const c_char, gnutls_sign_get_name, W32_PFN_gnutls_sign_get_name,
             fn_gnutls_sign_get_name, (gnutls_sign_algorithm_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (c_int, gnutls_server_name_set, W32_PFN_gnutls_server_name_set,
             fn_gnutls_server_name_set,
             (gnutls_session_t, gnutls_server_name_type_t, *const c_void, size_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (gnutls_kx_algorithm_t, gnutls_kx_get, W32_PFN_gnutls_kx_get, fn_gnutls_kx_get,
             (gnutls_session_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (*const c_char, gnutls_kx_get_name, W32_PFN_gnutls_kx_get_name,
             fn_gnutls_kx_get_name, (gnutls_kx_algorithm_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (gnutls_protocol_t, gnutls_protocol_get_version,
             W32_PFN_gnutls_protocol_get_version, fn_gnutls_protocol_get_version,
             (gnutls_session_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (*const c_char, gnutls_protocol_get_name, W32_PFN_gnutls_protocol_get_name,
             fn_gnutls_protocol_get_name, (gnutls_protocol_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (gnutls_cipher_algorithm_t, gnutls_cipher_get, W32_PFN_gnutls_cipher_get,
             fn_gnutls_cipher_get, (gnutls_session_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (*const c_char, gnutls_cipher_get_name, W32_PFN_gnutls_cipher_get_name,
             fn_gnutls_cipher_get_name, (gnutls_cipher_algorithm_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (gnutls_mac_algorithm_t, gnutls_mac_get, W32_PFN_gnutls_mac_get, fn_gnutls_mac_get,
             (gnutls_session_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (*const c_char, gnutls_mac_get_name, W32_PFN_gnutls_mac_get_name,
             fn_gnutls_mac_get_name, (gnutls_mac_algorithm_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt", feature = "have_gnutls_compression_get"))]
DEF_DLL_FN! (gnutls_compression_method_t, gnutls_compression_get,
             W32_PFN_gnutls_compression_get, fn_gnutls_compression_get, (gnutls_session_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt", feature = "have_gnutls_compression_get"))]
DEF_DLL_FN! (*const c_char, gnutls_compression_get_name, W32_PFN_gnutls_compression_get_name,
             fn_gnutls_compression_get_name, (gnutls_compression_method_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
DEF_DLL_FN! (c_uint, gnutls_safe_renegotiation_status, W32_PFN_gnutls_safe_renegotiation_status,
             fn_gnutls_safe_renegotiation_status, (gnutls_session_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt", feature = "have_gnutls3"))]
DEF_DLL_FN! (*const gnutls_mac_algorithm_t, gnutls_mac_list, W32_PFN_gnutls_mac_list,
             fn_gnutls_mac_list, ());
#[cfg (all (feature = "have_gnutls", feature = "windowsnt", feature = "have_gnutls3", feature = "have_gnutls_mac_get_nonce_size"))]
DEF_DLL_FN! (size_t, gnutls_mac_get_nonce_size, W32_PFN_gnutls_mac_get_nonce_size,
             fn_gnutls_mac_get_nonce_size, (gnutls_mac_algorithm_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt", feature = "have_gnutls3"))]
DEF_DLL_FN! (size_t, gnutls_mac_get_key_size, W32_PFN_gnutls_mac_get_key_size,
             fn_gnutls_mac_get_key_size, (gnutls_mac_algorithm_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt", feature = "have_gnutls3", feature = "have_gnutls_digest_list"))]
DEF_DLL_FN! (*const gnutls_digest_algorithm_t, gnutls_digest_list, W32_PFN_gnutls_digest_list,
             fn_gnutls_digest_list, ());
#[cfg (all (feature = "have_gnutls", feature = "windowsnt", feature = "have_gnutls3", feature = "have_gnutls_digest_list"))]
DEF_DLL_FN! (*const c_char, gnutls_digest_get_name, W32_PFN_gnutls_digest_get_name,
             fn_gnutls_digest_get_name, (gnutls_digest_algorithm_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt", feature = "have_gnutls3"))]
DEF_DLL_FN! (*mut gnutls_cipher_algorithm_t, gnutls_cipher_list, W32_PFN_gnutls_cipher_list,
             fn_gnutls_cipher_list, ());
#[cfg (all (feature = "have_gnutls", feature = "windowsnt", feature = "have_gnutls3", feature = "have_gnutls_cipher_get_iv_size"))]
DEF_DLL_FN! (c_int, gnutls_cipher_get_iv_size, W32_PFN_gnutls_cipher_get_iv_size,
             fn_gnutls_cipher_get_iv_size, (gnutls_cipher_algorithm_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt", feature = "have_gnutls3"))]
DEF_DLL_FN! (size_t, gnutls_cipher_get_key_size, W32_PFN_gnutls_cipher_get_key_size,
             fn_gnutls_cipher_get_key_size, (gnutls_cipher_algorithm_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt", feature = "have_gnutls3"))]
DEF_DLL_FN! (c_int, gnutls_cipher_get_block_size, W32_PFN_gnutls_cipher_get_block_size,
             fn_gnutls_cipher_get_block_size, (gnutls_cipher_algorithm_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt", feature = "have_gnutls3", feature = "have_gnutls_cipher_get_tag_size"))]
DEF_DLL_FN! (c_int, gnutls_cipher_get_tag_size, W32_PFN_gnutls_cipher_get_tag_size,
             fn_gnutls_cipher_get_tag_size, (gnutls_cipher_algorithm_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt", feature = "have_gnutls3"))]
DEF_DLL_FN! (c_int, gnutls_cipher_init, W32_PFN_gnutls_cipher_init, fn_gnutls_cipher_init,
             (*mut gnutls_cipher_hd_t, gnutls_cipher_algorithm_t, *const gnutls_datum_t, *const gnutls_datum_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt", feature = "have_gnutls3"))]
DEF_DLL_FN! ((), gnutls_cipher_set_iv, W32_PFN_gnutls_cipher_set_iv, fn_gnutls_cipher_set_iv,
             (gnutls_cipher_hd_t, *mut c_void, size_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt", feature = "have_gnutls3"))]
DEF_DLL_FN! (c_int, gnutls_cipher_encrypt2, W32_PFN_gnutls_cipher_encrypt2,
             fn_gnutls_cipher_encrypt2,
             (gnutls_cipher_hd_t, *const c_void, size_t, *mut c_void, size_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt", feature = "have_gnutls3"))]
DEF_DLL_FN! ((), gnutls_cipher_deinit, W32_PFN_gnutls_cipher_deinit, fn_gnutls_cipher_deinit,
             (gnutls_cipher_hd_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt", feature = "have_gnutls3"))]
DEF_DLL_FN! (c_int, gnutls_cipher_decrypt2, W32_PFN_gnutls_cipher_decrypt2,
             fn_gnutls_cipher_decrypt2,
             (gnutls_cipher_hd_t, *const c_void, size_t, *mut c_void, size_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt", feature = "have_gnutls3", feature = "have_gnutls_aead"))]
DEF_DLL_FN! (c_int, gnutls_aead_cipher_init, W32_PFN_gnutls_aead_cipher_init,
             fn_gnutls_aead_cipher_init,
             (*mut gnutls_aead_cipher_hd_t, gnutls_cipher_algorithm_t, *const gnutls_datum_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt", feature = "have_gnutls3", feature = "have_gnutls_aead"))]
DEF_DLL_FN! ((), gnutls_aead_cipher_deinit, W32_PFN_gnutls_aead_cipher_deinit,
             fn_gnutls_aead_cipher_deinit, (gnutls_aead_cipher_hd_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt", feature = "have_gnutls3", feature = "have_gnutls_aead"))]
DEF_DLL_FN! (c_int, gnutls_aead_cipher_encrypt, W32_PFN_gnutls_aead_cipher_encrypt,
             fn_gnutls_aead_cipher_encrypt,
             (gnutls_aead_cipher_hd_t, *const c_void, size_t, *const c_void, size_t, size_t, *const c_void, size_t, *mut c_void, *mut size_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt", feature = "have_gnutls3", feature = "have_gnutls_aead"))]
DEF_DLL_FN! (c_int, gnutls_aead_cipher_decrypt, W32_PFN_gnutls_aead_cipher_decrypt,
             fn_gnutls_aead_cipher_decrypt,
             (gnutls_aead_cipher_hd_t, *const c_void, size_t, *const c_void, size_t, size_t, *const c_void, size_t, *mut c_void, *mut size_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt", feature = "have_gnutls3", feature = "have_gnutls_etm_status"))]
DEF_DLL_FN! (c_uint, gnutls_session_etm_status, W32_PFN_gnutls_session_etm_status,
             fn_gnutls_session_etm_status, (gnutls_session_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt", feature = "have_gnutls3"))]
DEF_DLL_FN! (c_int, gnutls_hmac_init, W32_PFN_gnutls_hmac_init, fn_gnutls_hmac_init,
             (*mut gnutls_hmac_hd_t, gnutls_mac_algorithm_t, *const c_void, size_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt", feature = "have_gnutls3"))]
DEF_DLL_FN! (c_int, gnutls_hmac_get_len, W32_PFN_gnutls_hmac_get_len, fn_gnutls_hmac_get_len,
             (gnutls_mac_algorithm_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt", feature = "have_gnutls3"))]
DEF_DLL_FN! (c_int, gnutls_hmac, W32_PFN_gnutls_hmac, fn_gnutls_hmac,
             (gnutls_hmac_hd_t, *const c_void, size_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt", feature = "have_gnutls3"))]
DEF_DLL_FN! ((), gnutls_hmac_deinit, W32_PFN_gnutls_hmac_deinit, fn_gnutls_hmac_deinit,
             (gnutls_hmac_hd_t, *mut c_void));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt", feature = "have_gnutls3"))]
DEF_DLL_FN! ((), gnutls_hmac_output, W32_PFN_gnutls_hmac_output, fn_gnutls_hmac_output,
             (gnutls_hmac_hd_t, *mut c_void));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt", feature = "have_gnutls3"))]
DEF_DLL_FN! (c_int, gnutls_hash_init, W32_PFN_gnutls_hash_init, fn_gnutls_hash_init,
             (*mut gnutls_hash_hd_t, gnutls_digest_algorithm_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt", feature = "have_gnutls3"))]
DEF_DLL_FN! (c_int, gnutls_hash_get_len, W32_PFN_gnutls_hash_get_len, fn_gnutls_hash_get_len,
             (gnutls_digest_algorithm_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt", feature = "have_gnutls3"))]
DEF_DLL_FN! (c_int, gnutls_hash, W32_PFN_gnutls_hash, fn_gnutls_hash,
             (gnutls_hash_hd_t, *const c_void, size_t));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt", feature = "have_gnutls3"))]
DEF_DLL_FN! ((), gnutls_hash_deinit, W32_PFN_gnutls_hash_deinit, fn_gnutls_hash_deinit,
             (gnutls_hash_hd_t, *mut c_void));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt", feature = "have_gnutls3"))]
DEF_DLL_FN! ((), gnutls_hash_output, W32_PFN_gnutls_hash_output, fn_gnutls_hash_output,
             (gnutls_hash_hd_t, *mut c_void));
#[cfg (all (feature = "have_gnutls", feature = "windowsnt", feature = "have_gnutls3", feature = "have_gnutls_ext_get_name"))]
DEF_DLL_FN! (*const c_char, gnutls_ext_get_name, W32_PFN_gnutls_ext_get_name,
             fn_gnutls_ext_get_name, (c_uint));

/// C 302: `static gnutls_free_function *gnutls_free_func;` -- C's tentative definition, i.e. a
/// null pointer (PORTING.md 0.1-13).
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
static mut gnutls_free_func: *mut gnutls_free_function = std::ptr::null_mut ();

/// C 304-458: `init_gnutls_functions`.  `HMODULE` is C's `<windows.h>` handle, spelled
/// `*mut c_void` (the region's note above); `w32_delayed_load` is the declaration
/// `src/include/w32.rs` carries for `w32.h` 197, and `GetProcAddress` the one
/// `src/include/w32common.rs` calls.
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
unsafe fn init_gnutls_functions () -> bool
{
  let mut library: *mut c_void;
  let mut max_log_level: c_int = 1;

  library = w32_delayed_load (Qgnutls ());
  if library.is_null ()
    {
      GNUTLS_LOG! (1, max_log_level, "GnuTLS library not found");
      return false;
    }

  LOAD_DLL_FN! (library, gnutls_alert_get, W32_PFN_gnutls_alert_get, fn_gnutls_alert_get, c"gnutls_alert_get");
  LOAD_DLL_FN! (library, gnutls_alert_get_name, W32_PFN_gnutls_alert_get_name, fn_gnutls_alert_get_name, c"gnutls_alert_get_name");
  LOAD_DLL_FN! (library, gnutls_anon_allocate_client_credentials, W32_PFN_gnutls_anon_allocate_client_credentials, fn_gnutls_anon_allocate_client_credentials, c"gnutls_anon_allocate_client_credentials");
  LOAD_DLL_FN! (library, gnutls_anon_free_client_credentials, W32_PFN_gnutls_anon_free_client_credentials, fn_gnutls_anon_free_client_credentials, c"gnutls_anon_free_client_credentials");
  LOAD_DLL_FN! (library, gnutls_bye, W32_PFN_gnutls_bye, fn_gnutls_bye, c"gnutls_bye");
  LOAD_DLL_FN! (library, gnutls_certificate_allocate_credentials, W32_PFN_gnutls_certificate_allocate_credentials, fn_gnutls_certificate_allocate_credentials, c"gnutls_certificate_allocate_credentials");
  LOAD_DLL_FN! (library, gnutls_certificate_free_credentials, W32_PFN_gnutls_certificate_free_credentials, fn_gnutls_certificate_free_credentials, c"gnutls_certificate_free_credentials");
  LOAD_DLL_FN! (library, gnutls_certificate_get_peers, W32_PFN_gnutls_certificate_get_peers, fn_gnutls_certificate_get_peers, c"gnutls_certificate_get_peers");
  LOAD_DLL_FN! (library, gnutls_certificate_set_verify_flags, W32_PFN_gnutls_certificate_set_verify_flags, fn_gnutls_certificate_set_verify_flags, c"gnutls_certificate_set_verify_flags");
  LOAD_DLL_FN! (library, gnutls_certificate_set_x509_crl_file, W32_PFN_gnutls_certificate_set_x509_crl_file, fn_gnutls_certificate_set_x509_crl_file, c"gnutls_certificate_set_x509_crl_file");
  LOAD_DLL_FN! (library, gnutls_certificate_set_x509_key_file, W32_PFN_gnutls_certificate_set_x509_key_file, fn_gnutls_certificate_set_x509_key_file, c"gnutls_certificate_set_x509_key_file");
  #[cfg (feature = "have_gnutls_certificate_set_x509_key_file2")]
  { LOAD_DLL_FN! (library, gnutls_certificate_set_x509_key_file2, W32_PFN_gnutls_certificate_set_x509_key_file2, fn_gnutls_certificate_set_x509_key_file2, c"gnutls_certificate_set_x509_key_file2"); }
  #[cfg (feature = "have_gnutls_x509_system_trust")]
  { LOAD_DLL_FN! (library, gnutls_certificate_set_x509_system_trust, W32_PFN_gnutls_certificate_set_x509_system_trust, fn_gnutls_certificate_set_x509_system_trust, c"gnutls_certificate_set_x509_system_trust"); }
  LOAD_DLL_FN! (library, gnutls_certificate_set_x509_trust_file, W32_PFN_gnutls_certificate_set_x509_trust_file, fn_gnutls_certificate_set_x509_trust_file, c"gnutls_certificate_set_x509_trust_file");
  LOAD_DLL_FN! (library, gnutls_certificate_type_get, W32_PFN_gnutls_certificate_type_get, fn_gnutls_certificate_type_get, c"gnutls_certificate_type_get");
  LOAD_DLL_FN! (library, gnutls_certificate_verify_peers2, W32_PFN_gnutls_certificate_verify_peers2, fn_gnutls_certificate_verify_peers2, c"gnutls_certificate_verify_peers2");
  LOAD_DLL_FN! (library, gnutls_credentials_set, W32_PFN_gnutls_credentials_set, fn_gnutls_credentials_set, c"gnutls_credentials_set");
  LOAD_DLL_FN! (library, gnutls_deinit, W32_PFN_gnutls_deinit, fn_gnutls_deinit, c"gnutls_deinit");
  LOAD_DLL_FN! (library, gnutls_dh_set_prime_bits, W32_PFN_gnutls_dh_set_prime_bits, fn_gnutls_dh_set_prime_bits, c"gnutls_dh_set_prime_bits");
  LOAD_DLL_FN! (library, gnutls_dh_get_prime_bits, W32_PFN_gnutls_dh_get_prime_bits, fn_gnutls_dh_get_prime_bits, c"gnutls_dh_get_prime_bits");
  LOAD_DLL_FN! (library, gnutls_error_is_fatal, W32_PFN_gnutls_error_is_fatal, fn_gnutls_error_is_fatal, c"gnutls_error_is_fatal");
  LOAD_DLL_FN! (library, gnutls_global_init, W32_PFN_gnutls_global_init, fn_gnutls_global_init, c"gnutls_global_init");
  LOAD_DLL_FN! (library, gnutls_global_set_log_function, W32_PFN_gnutls_global_set_log_function, fn_gnutls_global_set_log_function, c"gnutls_global_set_log_function");
  #[cfg (feature = "have_gnutls3")]
  { LOAD_DLL_FN! (library, gnutls_global_set_audit_log_function, W32_PFN_gnutls_global_set_audit_log_function, fn_gnutls_global_set_audit_log_function, c"gnutls_global_set_audit_log_function"); }
  LOAD_DLL_FN! (library, gnutls_global_set_log_level, W32_PFN_gnutls_global_set_log_level, fn_gnutls_global_set_log_level, c"gnutls_global_set_log_level");
  LOAD_DLL_FN! (library, gnutls_handshake, W32_PFN_gnutls_handshake, fn_gnutls_handshake, c"gnutls_handshake");
  LOAD_DLL_FN! (library, gnutls_init, W32_PFN_gnutls_init, fn_gnutls_init, c"gnutls_init");
  LOAD_DLL_FN! (library, gnutls_priority_set_direct, W32_PFN_gnutls_priority_set_direct, fn_gnutls_priority_set_direct, c"gnutls_priority_set_direct");
  LOAD_DLL_FN! (library, gnutls_record_check_pending, W32_PFN_gnutls_record_check_pending, fn_gnutls_record_check_pending, c"gnutls_record_check_pending");
  LOAD_DLL_FN! (library, gnutls_record_recv, W32_PFN_gnutls_record_recv, fn_gnutls_record_recv, c"gnutls_record_recv");
  LOAD_DLL_FN! (library, gnutls_record_send, W32_PFN_gnutls_record_send, fn_gnutls_record_send, c"gnutls_record_send");
  LOAD_DLL_FN! (library, gnutls_strerror, W32_PFN_gnutls_strerror, fn_gnutls_strerror, c"gnutls_strerror");
  LOAD_DLL_FN! (library, gnutls_transport_set_errno, W32_PFN_gnutls_transport_set_errno, fn_gnutls_transport_set_errno, c"gnutls_transport_set_errno");
  LOAD_DLL_FN! (library, gnutls_transport_set_ptr2, W32_PFN_gnutls_transport_set_ptr2, fn_gnutls_transport_set_ptr2, c"gnutls_transport_set_ptr2");
  LOAD_DLL_FN! (library, gnutls_transport_set_pull_function, W32_PFN_gnutls_transport_set_pull_function, fn_gnutls_transport_set_pull_function, c"gnutls_transport_set_pull_function");
  LOAD_DLL_FN! (library, gnutls_transport_set_push_function, W32_PFN_gnutls_transport_set_push_function, fn_gnutls_transport_set_push_function, c"gnutls_transport_set_push_function");
  LOAD_DLL_FN! (library, gnutls_x509_crt_check_hostname, W32_PFN_gnutls_x509_crt_check_hostname, fn_gnutls_x509_crt_check_hostname, c"gnutls_x509_crt_check_hostname");
  LOAD_DLL_FN! (library, gnutls_x509_crt_check_issuer, W32_PFN_gnutls_x509_crt_check_issuer, fn_gnutls_x509_crt_check_issuer, c"gnutls_x509_crt_check_issuer");
  LOAD_DLL_FN! (library, gnutls_x509_crt_deinit, W32_PFN_gnutls_x509_crt_deinit, fn_gnutls_x509_crt_deinit, c"gnutls_x509_crt_deinit");
  LOAD_DLL_FN! (library, gnutls_x509_crt_export, W32_PFN_gnutls_x509_crt_export, fn_gnutls_x509_crt_export, c"gnutls_x509_crt_export");
  LOAD_DLL_FN! (library, gnutls_x509_crt_import, W32_PFN_gnutls_x509_crt_import, fn_gnutls_x509_crt_import, c"gnutls_x509_crt_import");
  LOAD_DLL_FN! (library, gnutls_x509_crt_init, W32_PFN_gnutls_x509_crt_init, fn_gnutls_x509_crt_init, c"gnutls_x509_crt_init");
  LOAD_DLL_FN! (library, gnutls_x509_crt_get_fingerprint, W32_PFN_gnutls_x509_crt_get_fingerprint, fn_gnutls_x509_crt_get_fingerprint, c"gnutls_x509_crt_get_fingerprint");
  LOAD_DLL_FN! (library, gnutls_x509_crt_get_version, W32_PFN_gnutls_x509_crt_get_version, fn_gnutls_x509_crt_get_version, c"gnutls_x509_crt_get_version");
  LOAD_DLL_FN! (library, gnutls_x509_crt_get_serial, W32_PFN_gnutls_x509_crt_get_serial, fn_gnutls_x509_crt_get_serial, c"gnutls_x509_crt_get_serial");
  LOAD_DLL_FN! (library, gnutls_x509_crt_get_issuer_dn, W32_PFN_gnutls_x509_crt_get_issuer_dn, fn_gnutls_x509_crt_get_issuer_dn, c"gnutls_x509_crt_get_issuer_dn");
  LOAD_DLL_FN! (library, gnutls_x509_crt_get_activation_time, W32_PFN_gnutls_x509_crt_get_activation_time, fn_gnutls_x509_crt_get_activation_time, c"gnutls_x509_crt_get_activation_time");
  LOAD_DLL_FN! (library, gnutls_x509_crt_get_expiration_time, W32_PFN_gnutls_x509_crt_get_expiration_time, fn_gnutls_x509_crt_get_expiration_time, c"gnutls_x509_crt_get_expiration_time");
  LOAD_DLL_FN! (library, gnutls_x509_crt_get_dn, W32_PFN_gnutls_x509_crt_get_dn, fn_gnutls_x509_crt_get_dn, c"gnutls_x509_crt_get_dn");
  LOAD_DLL_FN! (library, gnutls_x509_crt_get_pk_algorithm, W32_PFN_gnutls_x509_crt_get_pk_algorithm, fn_gnutls_x509_crt_get_pk_algorithm, c"gnutls_x509_crt_get_pk_algorithm");
  LOAD_DLL_FN! (library, gnutls_x509_crt_print, W32_PFN_gnutls_x509_crt_print, fn_gnutls_x509_crt_print, c"gnutls_x509_crt_print");
  LOAD_DLL_FN! (library, gnutls_pk_algorithm_get_name, W32_PFN_gnutls_pk_algorithm_get_name, fn_gnutls_pk_algorithm_get_name, c"gnutls_pk_algorithm_get_name");
  LOAD_DLL_FN! (library, gnutls_pk_bits_to_sec_param, W32_PFN_gnutls_pk_bits_to_sec_param, fn_gnutls_pk_bits_to_sec_param, c"gnutls_pk_bits_to_sec_param");
  LOAD_DLL_FN! (library, gnutls_x509_crt_get_issuer_unique_id, W32_PFN_gnutls_x509_crt_get_issuer_unique_id, fn_gnutls_x509_crt_get_issuer_unique_id, c"gnutls_x509_crt_get_issuer_unique_id");
  LOAD_DLL_FN! (library, gnutls_x509_crt_get_subject_unique_id, W32_PFN_gnutls_x509_crt_get_subject_unique_id, fn_gnutls_x509_crt_get_subject_unique_id, c"gnutls_x509_crt_get_subject_unique_id");
  LOAD_DLL_FN! (library, gnutls_x509_crt_get_signature_algorithm, W32_PFN_gnutls_x509_crt_get_signature_algorithm, fn_gnutls_x509_crt_get_signature_algorithm, c"gnutls_x509_crt_get_signature_algorithm");
  LOAD_DLL_FN! (library, gnutls_x509_crt_get_key_id, W32_PFN_gnutls_x509_crt_get_key_id, fn_gnutls_x509_crt_get_key_id, c"gnutls_x509_crt_get_key_id");
  LOAD_DLL_FN! (library, gnutls_sec_param_get_name, W32_PFN_gnutls_sec_param_get_name, fn_gnutls_sec_param_get_name, c"gnutls_sec_param_get_name");
  LOAD_DLL_FN! (library, gnutls_sign_get_name, W32_PFN_gnutls_sign_get_name, fn_gnutls_sign_get_name, c"gnutls_sign_get_name");
  LOAD_DLL_FN! (library, gnutls_server_name_set, W32_PFN_gnutls_server_name_set, fn_gnutls_server_name_set, c"gnutls_server_name_set");
  LOAD_DLL_FN! (library, gnutls_kx_get, W32_PFN_gnutls_kx_get, fn_gnutls_kx_get, c"gnutls_kx_get");
  LOAD_DLL_FN! (library, gnutls_kx_get_name, W32_PFN_gnutls_kx_get_name, fn_gnutls_kx_get_name, c"gnutls_kx_get_name");
  LOAD_DLL_FN! (library, gnutls_protocol_get_version, W32_PFN_gnutls_protocol_get_version, fn_gnutls_protocol_get_version, c"gnutls_protocol_get_version");
  LOAD_DLL_FN! (library, gnutls_protocol_get_name, W32_PFN_gnutls_protocol_get_name, fn_gnutls_protocol_get_name, c"gnutls_protocol_get_name");
  LOAD_DLL_FN! (library, gnutls_cipher_get, W32_PFN_gnutls_cipher_get, fn_gnutls_cipher_get, c"gnutls_cipher_get");
  LOAD_DLL_FN! (library, gnutls_cipher_get_name, W32_PFN_gnutls_cipher_get_name, fn_gnutls_cipher_get_name, c"gnutls_cipher_get_name");
  LOAD_DLL_FN! (library, gnutls_mac_get, W32_PFN_gnutls_mac_get, fn_gnutls_mac_get, c"gnutls_mac_get");
  LOAD_DLL_FN! (library, gnutls_mac_get_name, W32_PFN_gnutls_mac_get_name, fn_gnutls_mac_get_name, c"gnutls_mac_get_name");
  #[cfg (feature = "have_gnutls_compression_get")]
  { LOAD_DLL_FN! (library, gnutls_compression_get, W32_PFN_gnutls_compression_get, fn_gnutls_compression_get, c"gnutls_compression_get"); }
  #[cfg (feature = "have_gnutls_compression_get")]
  { LOAD_DLL_FN! (library, gnutls_compression_get_name, W32_PFN_gnutls_compression_get_name, fn_gnutls_compression_get_name, c"gnutls_compression_get_name"); }
  LOAD_DLL_FN! (library, gnutls_safe_renegotiation_status, W32_PFN_gnutls_safe_renegotiation_status, fn_gnutls_safe_renegotiation_status, c"gnutls_safe_renegotiation_status");
  #[cfg (feature = "have_gnutls3")]
  { LOAD_DLL_FN! (library, gnutls_mac_list, W32_PFN_gnutls_mac_list, fn_gnutls_mac_list, c"gnutls_mac_list"); }
  #[cfg (feature = "have_gnutls3", feature = "have_gnutls_mac_get_nonce_size")]
  { LOAD_DLL_FN! (library, gnutls_mac_get_nonce_size, W32_PFN_gnutls_mac_get_nonce_size, fn_gnutls_mac_get_nonce_size, c"gnutls_mac_get_nonce_size"); }
  #[cfg (feature = "have_gnutls3")]
  { LOAD_DLL_FN! (library, gnutls_mac_get_key_size, W32_PFN_gnutls_mac_get_key_size, fn_gnutls_mac_get_key_size, c"gnutls_mac_get_key_size"); }
  #[cfg (feature = "have_gnutls3", feature = "have_gnutls_digest_list")]
  { LOAD_DLL_FN! (library, gnutls_digest_list, W32_PFN_gnutls_digest_list, fn_gnutls_digest_list, c"gnutls_digest_list"); }
  #[cfg (feature = "have_gnutls3", feature = "have_gnutls_digest_list")]
  { LOAD_DLL_FN! (library, gnutls_digest_get_name, W32_PFN_gnutls_digest_get_name, fn_gnutls_digest_get_name, c"gnutls_digest_get_name"); }
  #[cfg (feature = "have_gnutls3")]
  { LOAD_DLL_FN! (library, gnutls_cipher_list, W32_PFN_gnutls_cipher_list, fn_gnutls_cipher_list, c"gnutls_cipher_list"); }
  #[cfg (feature = "have_gnutls3", feature = "have_gnutls_cipher_get_iv_size")]
  { LOAD_DLL_FN! (library, gnutls_cipher_get_iv_size, W32_PFN_gnutls_cipher_get_iv_size, fn_gnutls_cipher_get_iv_size, c"gnutls_cipher_get_iv_size"); }
  #[cfg (feature = "have_gnutls3")]
  { LOAD_DLL_FN! (library, gnutls_cipher_get_key_size, W32_PFN_gnutls_cipher_get_key_size, fn_gnutls_cipher_get_key_size, c"gnutls_cipher_get_key_size"); }
  #[cfg (feature = "have_gnutls3")]
  { LOAD_DLL_FN! (library, gnutls_cipher_get_block_size, W32_PFN_gnutls_cipher_get_block_size, fn_gnutls_cipher_get_block_size, c"gnutls_cipher_get_block_size"); }
  #[cfg (feature = "have_gnutls3", feature = "have_gnutls_cipher_get_tag_size")]
  { LOAD_DLL_FN! (library, gnutls_cipher_get_tag_size, W32_PFN_gnutls_cipher_get_tag_size, fn_gnutls_cipher_get_tag_size, c"gnutls_cipher_get_tag_size"); }
  #[cfg (feature = "have_gnutls3")]
  { LOAD_DLL_FN! (library, gnutls_cipher_init, W32_PFN_gnutls_cipher_init, fn_gnutls_cipher_init, c"gnutls_cipher_init"); }
  #[cfg (feature = "have_gnutls3")]
  { LOAD_DLL_FN! (library, gnutls_cipher_set_iv, W32_PFN_gnutls_cipher_set_iv, fn_gnutls_cipher_set_iv, c"gnutls_cipher_set_iv"); }
  #[cfg (feature = "have_gnutls3")]
  { LOAD_DLL_FN! (library, gnutls_cipher_encrypt2, W32_PFN_gnutls_cipher_encrypt2, fn_gnutls_cipher_encrypt2, c"gnutls_cipher_encrypt2"); }
  #[cfg (feature = "have_gnutls3")]
  { LOAD_DLL_FN! (library, gnutls_cipher_deinit, W32_PFN_gnutls_cipher_deinit, fn_gnutls_cipher_deinit, c"gnutls_cipher_deinit"); }
  #[cfg (feature = "have_gnutls3")]
  { LOAD_DLL_FN! (library, gnutls_cipher_decrypt2, W32_PFN_gnutls_cipher_decrypt2, fn_gnutls_cipher_decrypt2, c"gnutls_cipher_decrypt2"); }
  #[cfg (feature = "have_gnutls3", feature = "have_gnutls_aead")]
  { LOAD_DLL_FN! (library, gnutls_aead_cipher_init, W32_PFN_gnutls_aead_cipher_init, fn_gnutls_aead_cipher_init, c"gnutls_aead_cipher_init"); }
  #[cfg (feature = "have_gnutls3", feature = "have_gnutls_aead")]
  { LOAD_DLL_FN! (library, gnutls_aead_cipher_deinit, W32_PFN_gnutls_aead_cipher_deinit, fn_gnutls_aead_cipher_deinit, c"gnutls_aead_cipher_deinit"); }
  #[cfg (feature = "have_gnutls3", feature = "have_gnutls_aead")]
  { LOAD_DLL_FN! (library, gnutls_aead_cipher_encrypt, W32_PFN_gnutls_aead_cipher_encrypt, fn_gnutls_aead_cipher_encrypt, c"gnutls_aead_cipher_encrypt"); }
  #[cfg (feature = "have_gnutls3", feature = "have_gnutls_aead")]
  { LOAD_DLL_FN! (library, gnutls_aead_cipher_decrypt, W32_PFN_gnutls_aead_cipher_decrypt, fn_gnutls_aead_cipher_decrypt, c"gnutls_aead_cipher_decrypt"); }
  #[cfg (feature = "have_gnutls3", feature = "have_gnutls_etm_status")]
  { LOAD_DLL_FN! (library, gnutls_session_etm_status, W32_PFN_gnutls_session_etm_status, fn_gnutls_session_etm_status, c"gnutls_session_etm_status"); }
  #[cfg (feature = "have_gnutls3")]
  { LOAD_DLL_FN! (library, gnutls_hmac_init, W32_PFN_gnutls_hmac_init, fn_gnutls_hmac_init, c"gnutls_hmac_init"); }
  #[cfg (feature = "have_gnutls3")]
  { LOAD_DLL_FN! (library, gnutls_hmac_get_len, W32_PFN_gnutls_hmac_get_len, fn_gnutls_hmac_get_len, c"gnutls_hmac_get_len"); }
  #[cfg (feature = "have_gnutls3")]
  { LOAD_DLL_FN! (library, gnutls_hmac, W32_PFN_gnutls_hmac, fn_gnutls_hmac, c"gnutls_hmac"); }
  #[cfg (feature = "have_gnutls3")]
  { LOAD_DLL_FN! (library, gnutls_hmac_deinit, W32_PFN_gnutls_hmac_deinit, fn_gnutls_hmac_deinit, c"gnutls_hmac_deinit"); }
  #[cfg (feature = "have_gnutls3")]
  { LOAD_DLL_FN! (library, gnutls_hmac_output, W32_PFN_gnutls_hmac_output, fn_gnutls_hmac_output, c"gnutls_hmac_output"); }
  #[cfg (feature = "have_gnutls3")]
  { LOAD_DLL_FN! (library, gnutls_hash_init, W32_PFN_gnutls_hash_init, fn_gnutls_hash_init, c"gnutls_hash_init"); }
  #[cfg (feature = "have_gnutls3")]
  { LOAD_DLL_FN! (library, gnutls_hash_get_len, W32_PFN_gnutls_hash_get_len, fn_gnutls_hash_get_len, c"gnutls_hash_get_len"); }
  #[cfg (feature = "have_gnutls3")]
  { LOAD_DLL_FN! (library, gnutls_hash, W32_PFN_gnutls_hash, fn_gnutls_hash, c"gnutls_hash"); }
  #[cfg (feature = "have_gnutls3")]
  { LOAD_DLL_FN! (library, gnutls_hash_deinit, W32_PFN_gnutls_hash_deinit, fn_gnutls_hash_deinit, c"gnutls_hash_deinit"); }
  #[cfg (feature = "have_gnutls3")]
  { LOAD_DLL_FN! (library, gnutls_hash_output, W32_PFN_gnutls_hash_output, fn_gnutls_hash_output, c"gnutls_hash_output"); }
  #[cfg (feature = "have_gnutls3", feature = "have_gnutls_ext_get_name")]
  { LOAD_DLL_FN! (library, gnutls_ext_get_name, W32_PFN_gnutls_ext_get_name, fn_gnutls_ext_get_name, c"gnutls_ext_get_name"); }

  /* gnutls_free is a variable inside GnuTLS, whose value is the
     "free" function.  So it needs special handling.  */
  gnutls_free_func
    = GetProcAddress (library, c"gnutls_free".as_ptr ()) as *mut gnutls_free_function;
  if gnutls_free_func.is_null ()
    { return false; }

  max_log_level = clip_to_bounds (INT_MIN as isize, global_gnutls_log_level as isize,
                                  INT_MAX as isize) as c_int;
  {
    let name: Lisp_Object = CAR_SAFE (Fget (Qgnutls (), QCloaded_from ()));
    GNUTLS_LOG2! (1, max_log_level, "GnuTLS library loaded:",
		  if STRINGP (name) { SDATA (name) as *const c_char } else { c"unknown".as_ptr () });
  }

  return true;
}

/* C 460-587: the `#define`s that rename each entry point to its loaded pointer, plus C 583-587's
   `#undef gnutls_free` / `#define gnutls_free (*gnutls_free_func)`.  An object-like `#define` has
   no Rust counterpart, so -- as `src/decompress.rs` 22-26 and `src/lcms.rs` 27-29
   record for the same two macros' consumers -- the arms spell the call sites the way C's
   preprocessor leaves them: `fn_<func> (...)` under `windowsnt` and the plain name otherwise.
   Every call site in this slice does exactly that (C 646-649, 657, 762, 786, 860-861, 888, …).
   C 471's, 495's, 535's, 537's, 541's, 546's, 551's, 559's, 565's and 578's nested `#ifdef`s are
   the same ten sub-gates as the `DEF_DLL_FN` block's, so the `fn_*` names they rename exist
   exactly when the matching `DEF_DLL_FN` item does.  */

/* ----------------------------------------------------------------------------------------------
   C 591-1130: the rest of the slice, under the file gate alone except where an `#ifdef` names a
   sub-gate; C's conditional lines are carried as comments (P-86 ①).  The `windowsnt` arm's
   `#define`s at C 460-587 rename every entry point to its loaded pointer, so each call site below
   that C's preprocessor rewrites is spelled twice -- `fn_<func> (...)` under `windowsnt` and the
   plain name otherwise, the shape `src/decompress.rs` 373-392 records for the same class.  */

/* Report memory exhaustion if ERR is an out-of-memory indication.  */
/// C 594-599: `check_memory_full`.
#[cfg (feature = "have_gnutls")]
unsafe fn check_memory_full (err: c_int)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  /* When GnuTLS exhausts memory, it doesn't say how much memory it
     asked for, so tell the Emacs allocator that GnuTLS asked for no
     bytes.  This isn't accurate, but it's good enough.  */
  if err == GNUTLS_E_MEMORY_ERROR
    { memory_full (0); }

  }
}

/* C 601: `# ifdef HAVE_GNUTLS3` -> `feature = "have_gnutls3"` (C 612 closes the arm).  */
/* Log a simple audit message.  */
/// C 603-611: `gnutls_audit_log_function`.
#[cfg (all (feature = "have_gnutls", feature = "have_gnutls3"))]
unsafe extern "C" fn gnutls_audit_log_function (session: gnutls_session_t, string: *const c_char)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  if global_gnutls_log_level >= 1
    {
      message (c"gnutls.c: [audit] %s".as_ptr (), string);
    }

  }
}

/* Log a simple message.  */
/// C 615-618: `gnutls_log_function`.
#[cfg (feature = "have_gnutls")]
unsafe extern "C" fn gnutls_log_function (level: c_int, string: *const c_char)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  message (c"gnutls.c: [%d] %s".as_ptr (), level, string);

  }
}

/* Log a message and a string.  */
/// C 622-626: `gnutls_log_function2`.
#[cfg (feature = "have_gnutls")]
unsafe fn gnutls_log_function2 (level: c_int, string: *const c_char, extra: *const c_char)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  message (c"gnutls.c: [%d] %s %s".as_ptr (), level, string, extra);

  }
}

/// C 629-660: `gnutls_try_handshake`.  C's `while ((ret = gnutls_handshake (state)) < 0)` is the
/// loop with the assignment as its condition; `ret` needs a value before the first test
/// (PORTING.md 1.5).
#[cfg (feature = "have_gnutls")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn gnutls_try_handshake (proc_: *mut Lisp_Process) -> c_int
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let state: gnutls_session_t = (*proc_).gnutls_state;
  let mut ret: c_int = 0;
  /* C's `proc->is_non_blocking_client` is a `bool_bf` bit of the process record; the port
     reads it through the mirror's mask instead of a field (PORTING.md 1.3,
     `include/process.rs` 108-118's `PROCESS_IS_NON_BLOCKING_CLIENT_SHIFT`).  */
  let mut non_blocking: bool
    = ((*proc_).bits2 >> PROCESS_IS_NON_BLOCKING_CLIENT_SHIFT) & 1 != 0;
  /* Sleep for ten milliseconds when busy-looping in
     gnutls_handshake.  */
  let mut delay: timespec = timespec { tv_sec: 0, tv_nsec: 1000 * 1000 * 10 };

  if (*proc_).gnutls_complete_negotiation_p()
    { non_blocking = false; }

  if non_blocking
    { (*proc_).set_gnutls_p(true); }

  loop
    {
      #[cfg (not (feature = "windowsnt"))]
      { ret = gnutls_handshake (state); }
      #[cfg (feature = "windowsnt")]
      { ret = (fn_gnutls_handshake.unwrap ()) (state); }
      if !(ret < 0)
	{ break; }
      if emacs_gnutls_handle_error (state, ret) == 0   /* fatal */
	{ break; }
      maybe_quit ();
      if non_blocking && ret != GNUTLS_E_INTERRUPTED
	{ break; }
      nanosleep (&raw const delay, std::ptr::null_mut ());
    }

  (*proc_).gnutls_initstage = GNUTLS_STAGE_HANDSHAKE_TRIED;

  if ret == GNUTLS_E_SUCCESS
    {
      /* Here we're finally done.  */
      (*proc_).gnutls_initstage = GNUTLS_STAGE_READY;
    }
  else
    {
      /* check_memory_full (gnutls_alert_send_appropriate (state, ret));  */
    }
  return ret;

  }
}

/* C 669-692: `# ifndef WINDOWSNT` -> `not (feature = "windowsnt")`.  */
/// C 671-691: `emacs_gnutls_nonblock_errno`.  C's `case 0:` under C 677's `# ifdef _AIX` falls
/// through to the two cases below; `_AIX` is `undef` in the frozen table (macros.tsv 4036), so
/// the arm is the `aix` feature, a name already registered (feature-coverage.tsv 146).
#[cfg (all (feature = "have_gnutls", not (feature = "windowsnt")))]
unsafe extern "C" fn emacs_gnutls_nonblock_errno (ptr: gnutls_transport_ptr_t) -> c_int
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let err: c_int = *errno ();

  match err
    {
      /* This is taken from the GnuTLS system_errno function circa 2016;
	 see <https://savannah.gnu.org/support/?107464>.  */
      #[cfg (feature = "aix")]
      0 =>
	{
	  *errno () = EAGAIN;
	  /* Fall through.  */
	  return EAGAIN;
	}
      EINPROGRESS | ENOTCONN =>
	{ return EAGAIN; }

      _ =>
	{ return err; }
    }

  }
}

/// C 694-726: `emacs_gnutls_handshake`.  C's `(gnutls_transport_ptr_t) proc` and
/// `(void *) (intptr_t) proc->infd` are the port's pointer casts, and C's `&emacs_gnutls_push`
/// (a function's address) is the `Option<fn>` constructor (the DEV_DLL_FN family's pointer
/// type).
#[cfg (feature = "have_gnutls")]
unsafe fn emacs_gnutls_handshake (proc_: *mut Lisp_Process) -> c_int
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let state: gnutls_session_t = (*proc_).gnutls_state;

  if (*proc_).gnutls_initstage < GNUTLS_STAGE_HANDSHAKE_CANDO
    { return -1; }

  if (*proc_).gnutls_initstage < GNUTLS_STAGE_TRANSPORT_POINTERS_SET
    {
      /* C 704-713: `# ifdef WINDOWSNT` / `# else`.  */
      #[cfg (feature = "windowsnt")]
      {
	/* On W32 we cannot transfer socket handles between different runtime
	   libraries, so we tell GnuTLS to use our special push/pull
	   functions.  */
	(fn_gnutls_transport_set_ptr2.unwrap ()) (state,
						  proc_ as gnutls_transport_ptr_t,
						  proc_ as gnutls_transport_ptr_t);
	(fn_gnutls_transport_set_push_function.unwrap ()) (state, Some (emacs_gnutls_push));
	(fn_gnutls_transport_set_pull_function.unwrap ()) (state, Some (emacs_gnutls_pull));
      }
      #[cfg (not (feature = "windowsnt"))]
      {
	/* This is how GnuTLS takes sockets: as file descriptors passed
	   in.  For an Emacs process socket, infd and outfd are the
	   same but we use this two-argument version for clarity.  */
	gnutls_transport_set_ptr2 (state, (*proc_).infd as isize as *mut c_void,
				   (*proc_).outfd as isize as *mut c_void);
	if ((*proc_).bits2 >> PROCESS_IS_NON_BLOCKING_CLIENT_SHIFT) & 1 != 0
	  { gnutls_transport_set_errno_function (state, Some (emacs_gnutls_nonblock_errno)); }
      }

      (*proc_).gnutls_initstage = GNUTLS_STAGE_TRANSPORT_POINTERS_SET;
    }

  return gnutls_try_handshake (proc_);

  }
}

/// C 728-732: `emacs_gnutls_record_check_pending`.
#[cfg (feature = "have_gnutls")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn emacs_gnutls_record_check_pending (state: gnutls_session_t) -> ptrdiff_t
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  #[cfg (not (feature = "windowsnt"))]
  { return gnutls_record_check_pending (state) as ptrdiff_t; }
  #[cfg (feature = "windowsnt")]
  { return (fn_gnutls_record_check_pending.unwrap ()) (state) as ptrdiff_t; }

  }
}

/* C 734-741: `# ifdef WINDOWSNT` -> `feature = "windowsnt"`.  */
/// C 735-740: `emacs_gnutls_transport_set_errno`.
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn emacs_gnutls_transport_set_errno (state: gnutls_session_t, err: c_int)
{
  (fn_gnutls_transport_set_errno.unwrap ()) (state, err);
}

/// C 743-770: `emacs_gnutls_write`.
#[cfg (feature = "have_gnutls")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn emacs_gnutls_write (proc_: *mut Lisp_Process, buf_: *const c_char,
					     nbyte: ptrdiff_t) -> ptrdiff_t
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let state: gnutls_session_t = (*proc_).gnutls_state;

  if (*proc_).gnutls_initstage != GNUTLS_STAGE_READY
    {
      *errno () = EAGAIN;
      return 0;
    }

  let mut bytes_written: ptrdiff_t = 0;
  let mut buf_: *const c_char = buf_;
  let mut nbyte: ptrdiff_t = nbyte;

  while nbyte > 0
    {
      let mut rtnval: ssize_t;
      loop
	{
	  #[cfg (not (feature = "windowsnt"))]
	  { rtnval = gnutls_record_send (state, buf_ as *const c_void, nbyte as size_t); }
	  #[cfg (feature = "windowsnt")]
	  { rtnval = (fn_gnutls_record_send.unwrap ()) (state, buf_ as *const c_void,
							nbyte as size_t); }
	  if rtnval != GNUTLS_E_INTERRUPTED as ssize_t
	    { break; }
	}

      if rtnval < 0
	{
	  emacs_gnutls_handle_error (state, rtnval as c_int);
	  break;
	}

      buf_ = buf_.add (rtnval as usize);
      nbyte -= rtnval as ptrdiff_t;
      bytes_written += rtnval as ptrdiff_t;
    }

  return bytes_written;

  }
}

/// C 772-796: `emacs_gnutls_read`.
#[cfg (feature = "have_gnutls")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn emacs_gnutls_read (proc_: *mut Lisp_Process, buf_: *mut c_char,
					    nbyte: ptrdiff_t) -> ptrdiff_t
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let state: gnutls_session_t = (*proc_).gnutls_state;

  if (*proc_).gnutls_initstage != GNUTLS_STAGE_READY
    {
      *errno () = EAGAIN;
      return -1;
    }

  let mut rtnval: ssize_t;
  loop
    {
      #[cfg (not (feature = "windowsnt"))]
      { rtnval = gnutls_record_recv (state, buf_ as *mut c_void, nbyte as size_t); }
      #[cfg (feature = "windowsnt")]
      { rtnval = (fn_gnutls_record_recv.unwrap ()) (state, buf_ as *mut c_void, nbyte as size_t); }
      if rtnval != GNUTLS_E_INTERRUPTED as ssize_t
	{ break; }
    }

  if rtnval >= 0
    { return rtnval as ptrdiff_t; }
  else if rtnval == GNUTLS_E_UNEXPECTED_PACKET_LENGTH as ssize_t
    /* The peer closed the connection. */
    { return 0; }
  else
    { return emacs_gnutls_handle_error (state, rtnval as c_int) as ptrdiff_t; }

  }
}

/// C 798-803: `emacs_gnutls_strerror`.
#[cfg (feature = "have_gnutls")]
unsafe fn emacs_gnutls_strerror (err: c_int) -> *const c_char
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  #[cfg (not (feature = "windowsnt"))]
  let str_: *const c_char = gnutls_strerror (err);
  #[cfg (feature = "windowsnt")]
  let str_: *const c_char = (fn_gnutls_strerror.unwrap ()) (err);
  return if !str_.is_null () { str_ } else { c"unknown".as_ptr () };

  }
}

/* Report a GnuTLS error to the user.
   SESSION is the GnuTLS session, ERR is the (negative) GnuTLS error code.
   Return 0 if the error was fatal, -1 (setting errno) otherwise so
   that the caller can notice the error and attempt a repair.  */
/// C 806-897: `emacs_gnutls_handle_error`.  C's two `switch`es are the port's `match`es; the
/// first one's `FALLTHROUGH` into `default:` is the second log inside the `GNUTLS_E_AGAIN` arm,
/// and the second one's `# ifdef EMSGSIZE` / `# if defined HAVE_GNUTLS3 && defined ECONNRESET`
/// arms carry `emsgsize` and `have_gnutls3`+`econnreset` (the frozen table *defines* `EMSGSIZE`
/// 40 and `ECONNRESET` 54, so both arms are live once the file gate is on).
#[cfg (feature = "have_gnutls")]
unsafe fn emacs_gnutls_handle_error (session: gnutls_session_t, err: c_int) -> c_int
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let mut ret: c_int = 0;

  /* TODO: use a Lisp_Object generated by gnutls_make_error?  */

  check_memory_full (err);

  let max_log_level: c_int
    = clip_to_bounds (INT_MIN as isize, global_gnutls_log_level as isize,
		      INT_MAX as isize) as c_int;

  /* TODO: use gnutls-error-fatalp and gnutls-error-string.  */

  let mut str_: *const c_char = emacs_gnutls_strerror (err);
  let mut errnum: c_int = EINVAL;

  #[cfg (not (feature = "windowsnt"))]
  let fatal: c_int = gnutls_error_is_fatal (err);
  #[cfg (feature = "windowsnt")]
  let fatal: c_int = (fn_gnutls_error_is_fatal.unwrap ()) (err);

  if fatal != 0
    {
      let mut level: c_int = 1;
      #[cfg (feature = "have_gnutls3")]
      {
	if err == GNUTLS_E_PREMATURE_TERMINATION
	  { level = 3; }
      }

      GNUTLS_LOG2! (level, max_log_level, "fatal error:", str_);
      ret = 0;
    }
  else
    {
      ret = -1;

      match err
	{
	  GNUTLS_E_AGAIN =>
	    {
	      GNUTLS_LOG2! (3, max_log_level, "retry:", str_);
	      /* FALLTHROUGH */
	      GNUTLS_LOG2! (1, max_log_level, "non-fatal error:", str_);
	    }
	  _ =>
	    {
	      GNUTLS_LOG2! (1, max_log_level, "non-fatal error:", str_);
	    }
	}

      match err
	{
	  GNUTLS_E_AGAIN =>
	    {
	      errnum = EAGAIN;
	    }

	  /* C 871-876: `# ifdef EMSGSIZE` -> `feature = "emsgsize"`.  */
	  #[cfg (feature = "emsgsize")]
	  GNUTLS_E_LARGE_PACKET | GNUTLS_E_PUSH_ERROR =>
	    {
	      errnum = EMSGSIZE;
	    }

	  /* C 878-882: `# if defined HAVE_GNUTLS3 && defined ECONNRESET`.  */
	  #[cfg (all (feature = "have_gnutls3", feature = "econnreset"))]
	  GNUTLS_E_PREMATURE_TERMINATION =>
	    {
	      errnum = ECONNRESET;
	    }

	  _ => {}
	}
    }

  if err == GNUTLS_E_WARNING_ALERT_RECEIVED
      || err == GNUTLS_E_FATAL_ALERT_RECEIVED
    {
      #[cfg (not (feature = "windowsnt"))]
      let alert: c_int = gnutls_alert_get (session) as c_int;
      #[cfg (feature = "windowsnt")]
      let alert: c_int = (fn_gnutls_alert_get.unwrap ()) (session);
      let level: c_int = if err == GNUTLS_E_FATAL_ALERT_RECEIVED { 0 } else { 1 };
      #[cfg (not (feature = "windowsnt"))]
      { str_ = gnutls_alert_get_name (alert as gnutls_alert_description_t); }
      #[cfg (feature = "windowsnt")]
      { str_ = (fn_gnutls_alert_get_name.unwrap ()) (alert); }
      if str_.is_null ()
	{ str_ = c"unknown".as_ptr (); }

      GNUTLS_LOG2! (level, max_log_level, "Received alert: ", str_);
    }

  *errno () = errnum;
  return ret;

  }
}

/* convert an integer error to a Lisp_Object; it will be either a
   known symbol like 'gnutls_e_interrupted' and 'gnutls_e_again' or
   simply the integer value of the error.  GNUTLS_E_SUCCESS is mapped
   to Qt.  */
/// C 906-923: `gnutls_make_error`.
#[cfg (feature = "have_gnutls")]
unsafe fn gnutls_make_error (err: c_int) -> Lisp_Object
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  match err
    {
      GNUTLS_E_SUCCESS =>
	{ return Qt (); }
      GNUTLS_E_AGAIN =>
	{ return Qgnutls_e_again (); }
      GNUTLS_E_INTERRUPTED =>
	{ return Qgnutls_e_interrupted (); }
      GNUTLS_E_INVALID_SESSION =>
	{ return Qgnutls_e_invalid_session (); }
      _ => {}
    }

  check_memory_full (err);
  return make_fixnum (err as EMACS_INT);

  }
}

/// C 925-936: `gnutls_deinit_certificates`.
#[cfg (feature = "have_gnutls")]
unsafe fn gnutls_deinit_certificates (p: *mut Lisp_Process)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  if (*p).gnutls_certificates.is_null ()
    { return; }

  let mut i: c_int = 0;
  while i < (*p).gnutls_certificates_length
    {
      /* C's `p->gnutls_certificates[i]` is a pointer index (PORTING.md); the port spells it.  */
      gnutls_x509_crt_deinit (*((*p).gnutls_certificates.add (i as usize)));
      i += 1;
    }

  xfree ((*p).gnutls_certificates as *mut c_void);
  (*p).gnutls_certificates = std::ptr::null_mut ();

  }
}

/// C 938-977: `emacs_gnutls_deinit`.
#[cfg (feature = "have_gnutls")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn emacs_gnutls_deinit (proc_: Lisp_Object) -> Lisp_Object
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let mut log_level: c_int;

  CHECK_PROCESS (proc_);

  if !(*XPROCESS (proc_)).gnutls_p()
    { return Qnil (); }

  log_level = (*XPROCESS (proc_)).gnutls_log_level;

  if !(*XPROCESS (proc_)).gnutls_x509_cred.is_null ()
    {
      GNUTLS_LOG! (2, log_level, "Deallocating x509 credentials");
      #[cfg (not (feature = "windowsnt"))]
      { gnutls_certificate_free_credentials ((*XPROCESS (proc_)).gnutls_x509_cred); }
      #[cfg (feature = "windowsnt")]
      { (fn_gnutls_certificate_free_credentials.unwrap ()) ((*XPROCESS (proc_)).gnutls_x509_cred); }
      (*XPROCESS (proc_)).gnutls_x509_cred = std::ptr::null_mut ();
    }

  if !(*XPROCESS (proc_)).gnutls_anon_cred.is_null ()
    {
      GNUTLS_LOG! (2, log_level, "Deallocating anon credentials");
      #[cfg (not (feature = "windowsnt"))]
      { gnutls_anon_free_client_credentials ((*XPROCESS (proc_)).gnutls_anon_cred); }
      #[cfg (feature = "windowsnt")]
      { (fn_gnutls_anon_free_client_credentials.unwrap ()) ((*XPROCESS (proc_)).gnutls_anon_cred); }
      (*XPROCESS (proc_)).gnutls_anon_cred = std::ptr::null_mut ();
    }

  if !(*XPROCESS (proc_)).gnutls_state.is_null ()
    {
      #[cfg (not (feature = "windowsnt"))]
      { gnutls_deinit ((*XPROCESS (proc_)).gnutls_state); }
      #[cfg (feature = "windowsnt")]
      { (fn_gnutls_deinit.unwrap ()) ((*XPROCESS (proc_)).gnutls_state); }
      (*XPROCESS (proc_)).gnutls_state = std::ptr::null_mut ();
      if GNUTLS_INITSTAGE (proc_) >= GNUTLS_STAGE_INIT
	{ (*XPROCESS (proc_)).gnutls_initstage = GNUTLS_STAGE_INIT - 1; }
    }

  if !(*XPROCESS (proc_)).gnutls_certificates.is_null ()
    { gnutls_deinit_certificates (XPROCESS (proc_)); }

  (*XPROCESS (proc_)).set_gnutls_p(false);
  return Qt ();

  }
}

/// C 979-990: the DEFUN `gnutls-asynchronous-parameters`.
#[cfg (feature = "have_gnutls")]
DEFUN! ("gnutls-asynchronous-parameters", Fgnutls_asynchronous_parameters,
	Sgnutls_asynchronous_parameters, 2, 2, 0,
       doc: "Mark this process as being a pre-init GnuTLS process.\nThe second parameter is the list of parameters to feed to gnutls-boot\nto finish setting up the connection.",
       (proc_: Lisp_Object, params: Lisp_Object),
       {
  CHECK_PROCESS (proc_);

  (*XPROCESS (proc_)).gnutls_boot_parameters = params;
  return Qnil ();
});

/// C 992-1000: the DEFUN `gnutls-get-initstage`.
#[cfg (feature = "have_gnutls")]
DEFUN! ("gnutls-get-initstage", Fgnutls_get_initstage, Sgnutls_get_initstage, 1, 1, 0,
       doc: "Return the GnuTLS init stage of process PROC.\nSee also `gnutls-boot'.",
       (proc_: Lisp_Object),
       {
  CHECK_PROCESS (proc_);

  return make_fixnum (GNUTLS_INITSTAGE (proc_) as EMACS_INT);
});

/// C 1002-1013: the DEFUN `gnutls-errorp`.
#[cfg (feature = "have_gnutls")]
DEFUN! ("gnutls-errorp", Fgnutls_errorp, Sgnutls_errorp, 1, 1, 0,
       doc: "Return t if ERROR indicates a GnuTLS problem.\nERROR is an integer or a symbol with an integer `gnutls-code' property.\nusage: (gnutls-errorp ERROR)",
       (err: Lisp_Object),
       {
  if EQ (err, Qt ())
      || EQ (err, Qgnutls_e_again ())
    { return Qnil (); }

  return Qt ();
});

/// C 1015-1045: the DEFUN `gnutls-error-fatalp`.
#[cfg (feature = "have_gnutls")]
DEFUN! ("gnutls-error-fatalp", Fgnutls_error_fatalp, Sgnutls_error_fatalp, 1, 1, 0,
       doc: "Return non-nil if ERROR is fatal.\nERROR is an integer or a symbol with an integer `gnutls-code' property.\nUsage: (gnutls-error-fatalp ERROR)",
       (err: Lisp_Object),
       {
  let mut err: Lisp_Object = err;
  let code: Lisp_Object;

  if EQ (err, Qt ()) { return Qnil (); }

  if SYMBOLP (err)
    {
      code = Fget (err, Qgnutls_code ());
      if NUMBERP (code)
	{
	  err = code;
	}
      else
	{
	  error (c"Symbol has no numeric gnutls-code property".as_ptr ());
	}
    }

  if !TYPE_RANGED_FIXNUMP (INT_MIN as EMACS_INT, err, INT_MAX as EMACS_INT)
    { error (c"Not an error symbol or code".as_ptr ()); }

  #[cfg (not (feature = "windowsnt"))]
  let is_fatal: c_int = gnutls_error_is_fatal (XFIXNUM (err) as c_int);
  #[cfg (feature = "windowsnt")]
  let is_fatal: c_int = (fn_gnutls_error_is_fatal.unwrap ()) (XFIXNUM (err) as c_int);
  if 0 == is_fatal
    { return Qnil (); }

  return Qt ();
});

/// C 1047-1074: the DEFUN `gnutls-error-string`.
#[cfg (feature = "have_gnutls")]
DEFUN! ("gnutls-error-string", Fgnutls_error_string, Sgnutls_error_string, 1, 1, 0,
       doc: "Return a description of ERROR.\nERROR is an integer or a symbol with an integer `gnutls-code' property.\nusage: (gnutls-error-string ERROR)",
       (err: Lisp_Object),
       {
  let mut err: Lisp_Object = err;
  let code: Lisp_Object;

  if EQ (err, Qt ()) { return build_string (c"Not an error".as_ptr ()); }

  if SYMBOLP (err)
    {
      code = Fget (err, Qgnutls_code ());
      if NUMBERP (code)
	{
	  err = code;
	}
      else
	{
	  return build_string (c"Symbol has no numeric gnutls-code property".as_ptr ());
	}
    }

  if !TYPE_RANGED_FIXNUMP (INT_MIN as EMACS_INT, err, INT_MAX as EMACS_INT)
    { return build_string (c"Not an error symbol or code".as_ptr ()); }

  return build_string (emacs_gnutls_strerror (XFIXNUM (err) as c_int));
});

/// C 1076-1082: the DEFUN `gnutls-deinit`.
#[cfg (feature = "have_gnutls")]
DEFUN! ("gnutls-deinit", Fgnutls_deinit, Sgnutls_deinit, 1, 1, 0,
       doc: "Deallocate GnuTLS resources associated with process PROC.\nSee also `gnutls-boot'.",
       (proc_: Lisp_Object),
       {
  return emacs_gnutls_deinit (proc_);
});

/// C 1084-1102: `gnutls_hex_string`.  C's `ckd_mul (&retlen, buf_size, 3) || ckd_add (&retlen,
/// retlen, prefix_length - (buf_size != 0))` is the `overflowing_*` pair with the write-back the
/// frozen `<stdckdint.h>` arm asks for (P-15, `src/casefiddle.rs` 315-320's shape): both
/// stores happen only when their operation did not overflow, in C's own left-to-right order.
#[cfg (feature = "have_gnutls")]
unsafe fn gnutls_hex_string (buf_: *mut c_uchar, buf_size: ptrdiff_t,
			     prefix: *const c_char) -> Lisp_Object
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let prefix_length: ptrdiff_t = strlen (prefix) as ptrdiff_t;
  let mut retlen: ptrdiff_t = 0;
  let (retlen_mul, mul_overflow) = buf_size.overflowing_mul (3);
  let mut overflowed = mul_overflow;
  retlen = retlen_mul;
  if !overflowed
    {
      let (retlen_add, add_overflow)
	= retlen.overflowing_add (prefix_length - (buf_size != 0) as ptrdiff_t);
      retlen = retlen_add;
      overflowed = add_overflow;
    }
  if overflowed
    { string_overflow (); }
  let ret: Lisp_Object = make_uninit_string (retlen as EMACS_INT);
  let string: *mut c_char = SSDATA (ret);
  strcpy (string, prefix);

  let mut i: ptrdiff_t = 0;
  while i < buf_size
    {
      sprintf (string.add (i as usize * 3 + prefix_length as usize),
	       if i == buf_size - 1 { c"%02x".as_ptr () } else { c"%02x:".as_ptr () },
	       *buf_.add (i as usize) as c_int);
      i += 1;
    }

  return ret;

  }
}

/// C 1104-1130: `emacs_gnutls_certificate_export_pem`.
#[cfg (feature = "have_gnutls")]
unsafe fn emacs_gnutls_certificate_export_pem (cert: gnutls_x509_crt_t) -> Lisp_Object
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let mut size: size_t = 0;
  #[cfg (not (feature = "windowsnt"))]
  let mut err: c_int = gnutls_x509_crt_export (cert, GNUTLS_X509_FMT_PEM,
					       std::ptr::null_mut (), &raw mut size);
  #[cfg (feature = "windowsnt")]
  let mut err: c_int
    = (fn_gnutls_x509_crt_export.unwrap ()) (cert, GNUTLS_X509_FMT_PEM,
					     std::ptr::null_mut (), &raw mut size);
  check_memory_full (err);

  if err == GNUTLS_E_SHORT_MEMORY_BUFFER
    {
      USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
      let buf: *mut c_char = SAFE_ALLOCA! (sa_avail, size as ptrdiff_t) as *mut c_char;
      #[cfg (not (feature = "windowsnt"))]
      { err = gnutls_x509_crt_export (cert, GNUTLS_X509_FMT_PEM, buf as *mut c_void,
				      &raw mut size); }
      #[cfg (feature = "windowsnt")]
      { err = (fn_gnutls_x509_crt_export.unwrap ()) (cert, GNUTLS_X509_FMT_PEM,
						     buf as *mut c_void, &raw mut size); }
      check_memory_full (err);

      if err < GNUTLS_E_SUCCESS
	{
	  error (c"GnuTLS certificate export error: %s".as_ptr (),
		 emacs_gnutls_strerror (err));
	}

      let result: Lisp_Object = build_string (buf);
      SAFE_FREE! (sa_count, sa_mark);
      return result;
    }
  else if err < GNUTLS_E_SUCCESS
    { error (c"GnuTLS certificate export error: %s".as_ptr (), emacs_gnutls_strerror (err)); }

  return Qnil ();

  }
}

// P-401: the certificate detail path preserves the original property order,
// short-buffer protocol, library errors and explicit external-buffer release.
#[cfg(all(feature = "have_gnutls", not(feature = "windowsnt")))]
unsafe fn emacs_gnutls_certificate_details(cert: gnutls_x509_crt_t) -> Lisp_Object {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let mut res = Qnil();
  let version = gnutls_x509_crt_get_version(cert);
  check_memory_full(version);
  if version >= GNUTLS_E_SUCCESS {
    res = nconc2(res, list2(QCversion(), make_fixnum(version as EMACS_INT)));
  }

  let mut size: size_t = 0;
  let mut err = gnutls_x509_crt_get_serial(cert, std::ptr::null_mut(), &raw mut size);
  check_memory_full(err);
  if err == GNUTLS_E_SHORT_MEMORY_BUFFER {
    let serial = xmalloc(size);
    err = gnutls_x509_crt_get_serial(cert, serial, &raw mut size);
    check_memory_full(err);
    if err >= GNUTLS_E_SUCCESS {
      res = nconc2(res, list2(QCserial_number(),
        gnutls_hex_string(serial.cast(), size as ptrdiff_t, c"".as_ptr())));
    }
    xfree(serial);
  }

  size = 0;
  err = gnutls_x509_crt_get_issuer_dn(cert, std::ptr::null_mut(), &raw mut size);
  check_memory_full(err);
  if err == GNUTLS_E_SHORT_MEMORY_BUFFER {
    let dn = xmalloc(size) as *mut c_char;
    err = gnutls_x509_crt_get_issuer_dn(cert, dn, &raw mut size);
    check_memory_full(err);
    if err >= GNUTLS_E_SUCCESS {
      res = nconc2(res, list2(QCissuer(), make_string(dn, size as ptrdiff_t)));
    }
    xfree(dn.cast());
  }

  // Darwin c_int is signed32: INT_STRLEN_BOUND(int) + 1 + sizeof("-12-31").
  let mut date: [c_char; 19] = [0; 19];
  let mut broken_down: tm = std::mem::zeroed();
  let mut instant = gnutls_x509_crt_get_activation_time(cert);
  if !gmtime_r(&raw const instant, &raw mut broken_down).is_null()
      && strftime(date.as_mut_ptr(), date.len(), c"%Y-%m-%d".as_ptr(), &raw const broken_down) != 0 {
    res = nconc2(res, list2(QCvalid_from(), build_string(date.as_ptr())));
  }
  instant = gnutls_x509_crt_get_expiration_time(cert);
  if !gmtime_r(&raw const instant, &raw mut broken_down).is_null()
      && strftime(date.as_mut_ptr(), date.len(), c"%Y-%m-%d".as_ptr(), &raw const broken_down) != 0 {
    res = nconc2(res, list2(QCvalid_to(), build_string(date.as_ptr())));
  }

  size = 0;
  err = gnutls_x509_crt_get_dn(cert, std::ptr::null_mut(), &raw mut size);
  check_memory_full(err);
  if err == GNUTLS_E_SHORT_MEMORY_BUFFER {
    let dn = xmalloc(size) as *mut c_char;
    err = gnutls_x509_crt_get_dn(cert, dn, &raw mut size);
    check_memory_full(err);
    if err >= GNUTLS_E_SUCCESS {
      res = nconc2(res, list2(QCsubject(), make_string(dn, size as ptrdiff_t)));
    }
    xfree(dn.cast());
  }

  let mut bits: c_uint = 0;
  err = gnutls_x509_crt_get_pk_algorithm(cert, &raw mut bits);
  check_memory_full(err);
  if err >= GNUTLS_E_SUCCESS {
    let name = gnutls_pk_algorithm_get_name(err as gnutls_pk_algorithm_t);
    if !name.is_null() {
      res = nconc2(res, list2(QCpublic_key_algorithm(), build_string(name)));
    }
    let level = gnutls_sec_param_get_name(gnutls_pk_bits_to_sec_param(err as gnutls_pk_algorithm_t, bits));
    res = nconc2(res, list2(QCcertificate_security_level(), build_string(level)));
  }

  size = 0;
  err = gnutls_x509_crt_get_issuer_unique_id(cert, std::ptr::null_mut(), &raw mut size);
  check_memory_full(err);
  if err == GNUTLS_E_SHORT_MEMORY_BUFFER {
    let buf = xmalloc(size);
    err = gnutls_x509_crt_get_issuer_unique_id(cert, buf.cast(), &raw mut size);
    check_memory_full(err);
    if err >= GNUTLS_E_SUCCESS {
      res = nconc2(res, list2(QCissuer_unique_id(), make_string(buf.cast(), size as ptrdiff_t)));
    }
    xfree(buf);
  }
  size = 0;
  err = gnutls_x509_crt_get_subject_unique_id(cert, std::ptr::null_mut(), &raw mut size);
  check_memory_full(err);
  if err == GNUTLS_E_SHORT_MEMORY_BUFFER {
    let buf = xmalloc(size);
    err = gnutls_x509_crt_get_subject_unique_id(cert, buf.cast(), &raw mut size);
    check_memory_full(err);
    if err >= GNUTLS_E_SUCCESS {
      res = nconc2(res, list2(QCsubject_unique_id(), make_string(buf.cast(), size as ptrdiff_t)));
    }
    xfree(buf);
  }

  err = gnutls_x509_crt_get_signature_algorithm(cert);
  check_memory_full(err);
  if err >= GNUTLS_E_SUCCESS {
    let name = gnutls_sign_get_name(err as gnutls_sign_algorithm_t);
    if !name.is_null() {
      res = nconc2(res, list2(QCsignature_algorithm(), build_string(name)));
    }
  }

  size = 0;
  err = gnutls_x509_crt_get_key_id(cert, 0, std::ptr::null_mut(), &raw mut size);
  check_memory_full(err);
  if err == GNUTLS_E_SHORT_MEMORY_BUFFER {
    let buf = xmalloc(size);
    err = gnutls_x509_crt_get_key_id(cert, 0, buf.cast(), &raw mut size);
    check_memory_full(err);
    if err >= GNUTLS_E_SUCCESS {
      res = nconc2(res, list2(QCpublic_key_id(),
        gnutls_hex_string(buf.cast(), size as ptrdiff_t, c"sha1:".as_ptr())));
    }
    xfree(buf);
  }
  #[cfg(feature = "have_gnutls_keyid_use_sha256")]
  {
    size = 0;
    err = gnutls_x509_crt_get_key_id(cert, GNUTLS_KEYID_USE_SHA256,
                                   std::ptr::null_mut(), &raw mut size);
    check_memory_full(err);
    if err == GNUTLS_E_SHORT_MEMORY_BUFFER {
      let buf = xmalloc(size);
      err = gnutls_x509_crt_get_key_id(cert, GNUTLS_KEYID_USE_SHA256, buf.cast(), &raw mut size);
      check_memory_full(err);
      if err >= GNUTLS_E_SUCCESS {
        res = nconc2(res, list2(QCpublic_key_id_sha256(),
          gnutls_hex_string(buf.cast(), size as ptrdiff_t, c"sha256:".as_ptr())));
      }
      xfree(buf);
    }
  }
  size = 0;
  err = gnutls_x509_crt_get_fingerprint(cert, GNUTLS_DIG_SHA1, std::ptr::null_mut(), &raw mut size);
  check_memory_full(err);
  if err == GNUTLS_E_SHORT_MEMORY_BUFFER {
    let buf = xmalloc(size);
    err = gnutls_x509_crt_get_fingerprint(cert, GNUTLS_DIG_SHA1, buf, &raw mut size);
    check_memory_full(err);
    if err >= GNUTLS_E_SUCCESS {
      res = nconc2(res, list2(QCcertificate_id(),
        gnutls_hex_string(buf.cast(), size as ptrdiff_t, c"sha1:".as_ptr())));
    }
    xfree(buf);
  }
  nconc2(res, list2(QCpem(), emacs_gnutls_certificate_export_pem(cert)))

  }
}

#[cfg(all(feature = "have_gnutls", not(feature = "windowsnt")))]
DEFUN! ("gnutls-peer-status-warning-describe", Fgnutls_peer_status_warning_describe,
       Sgnutls_peer_status_warning_describe, 1, 1, 0,
       doc: "Describe the warning of a GnuTLS peer status from `gnutls-peer-status'.",
       (status_symbol: Lisp_Object), {
  CHECK_SYMBOL(status_symbol);
  for (keyword, message) in [
    (QCinvalid(), c"certificate could not be verified"),
    (QCrevoked(), c"certificate was revoked (CRL)"),
    (QCself_signed(), c"certificate signer was not found (self-signed)"),
    (QCunknown_ca(), c"the certificate was signed by an unknown and therefore untrusted authority"),
    (QCnot_ca(), c"certificate signer is not a CA"),
    (QCinsecure(), c"certificate was signed with an insecure algorithm"),
    (QCnot_activated(), c"certificate is not yet activated"),
    (QCexpired(), c"certificate has expired"),
    (QCno_host_match(), c"certificate host does not match hostname"),
    (QCsignature_failure(), c"certificate signature could not be verified"),
    (QCrevocation_data_superseded(), c"certificate revocation data are old and have been superseded"),
    (QCrevocation_data_issued_in_future(), c"certificate revocation data have a future issue date"),
    (QCsigner_constraints_failure(), c"certificate signer constraints were violated"),
    (QCpurpose_mismatch(), c"certificate does not match the intended purpose"),
    (QCmissing_ocsp_status(), c"certificate requires the server to send a OCSP certificate status, but no status was received"),
    (QCinvalid_ocsp_status(), c"the received OCSP certificate status is invalid"),
  ] {
    if EQ(status_symbol, keyword) { return build_string(message.as_ptr()); }
  }
  Qnil()
});

#[cfg(all(feature = "have_gnutls", not(feature = "windowsnt")))]
DEFUN! ("gnutls-peer-status", Fgnutls_peer_status, Sgnutls_peer_status, 1, 1, 0,
       doc: "Describe GnuTLS peer certificate of PROC and any warnings about it.\n\nThe return value is a property list with top-level keys :warnings and\n:certificates.\n\nThe :warnings entry is a list of symbols whose descriptions can be\nobtained with `gnutls-peer-status-warning-describe'.\n\nThe :certificates entry is the certificate chain for the connection,\nwith the host certificate first and intermediary certificates (if any)\nfollowing it.  In addition, for backwards compatibility, the host\ncertificate is also returned as the :certificate entry.\n\nThe list in :certificates entry can also include one or more of the\nproperties of the TLS connection, when applicable, using the following\nkeys:\n\n  :cipher              a string naming the cipher algorithm, or \"NULL\"\n\n  :compression         a string naming the compression algorithm, or \"NULL\"\n\n  :diffie-hellman-prime-bits\n\t\t       the number if bits used in Diffie-Hellman key exchange,\n\t\t\t an integer\n\n  :encrypt-then-mac    t if encrypt-then-mac negotiation was successful,\n                         nil otherwise\n\n  :key-exchange        key exchange algorithm, a string\n\n  :mac                 a string naming the MAC algorithm, or \"NULL\"\n\n  :protocol            TLS protocol version as a string\n\n  :safe-renegotiation  t if safe renegotiation is used, otherwise nil\n",
       (proc_: Lisp_Object), {
  CHECK_PROCESS(proc_);
  if GNUTLS_INITSTAGE(proc_) != GNUTLS_STAGE_READY { return Qnil(); }
  let process = XPROCESS(proc_);
  let verification = (*process).gnutls_peer_verification;
  let mut warnings = Qnil();
  for (flag, keyword) in [
    (GNUTLS_CERT_INVALID, QCinvalid()),
    (GNUTLS_CERT_REVOKED, QCrevoked()),
    (GNUTLS_CERT_SIGNER_NOT_FOUND, QCunknown_ca()),
    (GNUTLS_CERT_SIGNER_NOT_CA, QCnot_ca()),
    (GNUTLS_CERT_INSECURE_ALGORITHM, QCinsecure()),
    (GNUTLS_CERT_NOT_ACTIVATED, QCnot_activated()),
    (GNUTLS_CERT_EXPIRED, QCexpired()),
  ] {
    if verification & flag != 0 { warnings = Fcons(keyword, warnings); }
  }
  #[cfg(feature = "gnutls_version_number_ge_0x030100")]
  if verification & GNUTLS_CERT_SIGNATURE_FAILURE != 0 {
    warnings = Fcons(QCsignature_failure(), warnings);
  }
  #[cfg(feature = "gnutls_version_number_ge_0x030114")]
  {
    if verification & GNUTLS_CERT_REVOCATION_DATA_SUPERSEDED != 0 {
      warnings = Fcons(QCrevocation_data_superseded(), warnings);
    }
    if verification & GNUTLS_CERT_REVOCATION_DATA_ISSUED_IN_FUTURE != 0 {
      warnings = Fcons(QCrevocation_data_issued_in_future(), warnings);
    }
    if verification & GNUTLS_CERT_SIGNER_CONSTRAINTS_FAILURE != 0 {
      warnings = Fcons(QCsigner_constraints_failure(), warnings);
    }
  }
  #[cfg(feature = "gnutls_version_number_ge_0x030400")]
  if verification & GNUTLS_CERT_PURPOSE_MISMATCH != 0 {
    warnings = Fcons(QCpurpose_mismatch(), warnings);
  }
  #[cfg(feature = "gnutls_version_number_ge_0x030501")]
  {
    if verification & GNUTLS_CERT_MISSING_OCSP_STATUS != 0 {
      warnings = Fcons(QCmissing_ocsp_status(), warnings);
    }
    if verification & GNUTLS_CERT_INVALID_OCSP_STATUS != 0 {
      warnings = Fcons(QCinvalid_ocsp_status(), warnings);
    }
  }
  if (*process).gnutls_extra_peer_verification & CERTIFICATE_NOT_MATCHING as c_uint != 0 {
    warnings = Fcons(QCno_host_match(), warnings);
  }
  let certificates = (*process).gnutls_certificates;
  if !certificates.is_null() && gnutls_x509_crt_check_issuer(*certificates, *certificates) != 0 {
    warnings = Fcons(QCself_signed(), warnings);
  }
  let mut result = if NILP(warnings) { Qnil() } else { list2(QCwarnings(), warnings) };
  if !certificates.is_null() {
    let mut certs = Qnil();
    for index in 0..(*process).gnutls_certificates_length {
      certs = nconc2(certs, list1(emacs_gnutls_certificate_details(*certificates.add(index as usize))));
    }
    result = nconc2(result, list2(QCcertificates(), certs));
    result = nconc2(result, list2(QCcertificate(), Fcar(certs)));
  }
  let state = (*process).gnutls_state;
  let bits = gnutls_dh_get_prime_bits(state);
  check_memory_full(bits);
  if bits > 0 {
    result = nconc2(result, list2(QCdiffie_hellman_prime_bits(), make_fixnum(bits as EMACS_INT)));
  }
  result = nconc2(result, list2(QCkey_exchange(),
    build_string(gnutls_kx_get_name(gnutls_kx_get(state)))));
  let protocol = gnutls_protocol_get_version(state);
  result = nconc2(result, list2(QCprotocol(), build_string(gnutls_protocol_get_name(protocol))));
  result = nconc2(result, list2(QCcipher(), build_string(gnutls_cipher_get_name(gnutls_cipher_get(state)))));
  result = nconc2(result, list2(QCmac(), build_string(gnutls_mac_get_name(gnutls_mac_get(state)))));
  #[cfg(feature = "have_gnutls_compression_get")]
  {
    result = nconc2(result, list2(QCcompression(),
      build_string(gnutls_compression_get_name(gnutls_compression_get(state)))));
  }
  #[cfg(feature = "have_gnutls_etm_status")]
  {
    result = nconc2(result, list2(QCencrypt_then_mac(),
      if gnutls_session_etm_status(state) != 0 { Qt() } else { Qnil() }));
  }
  if protocol <= GNUTLS_TLS1_2 {
    result = nconc2(result, list2(QCsafe_renegotiation(),
      if gnutls_safe_renegotiation_status(state) != 0 { Qt() } else { Qnil() }));
  }
  result
});

#[cfg(all(feature = "have_gnutls", not(feature = "windowsnt")))]
unsafe fn emacs_gnutls_global_init() -> Lisp_Object {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let mut result = GNUTLS_E_SUCCESS;
  if !gnutls_global_initialized {
    result = gnutls_global_init();
    if result == GNUTLS_E_SUCCESS { gnutls_global_initialized = true; }
  }
  gnutls_make_error(result)

  }
}

#[cfg(all(feature = "have_gnutls", not(feature = "windowsnt")))]
unsafe fn gnutls_ip_address_p(mut string: *const c_char) -> bool {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  loop {
    let c = *string;
    string = string.add(1);
    if c == 0 { return true; }
    if !(c == b'.' as c_char || c == b':' as c_char
         || (c >= b'0' as c_char && c <= b'9' as c_char)) { return false; }
  }

  }
}

#[cfg(all(feature = "have_gnutls", not(feature = "windowsnt")))]
unsafe extern "C-unwind" fn boot_error(process: *mut Lisp_Process, message: *const c_char, mut args: ...) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  if (*process).is_non_blocking_client() {
    pset_status(process, list2(Qfailed(), vformat_string(message, args)));
  } else {
    verror(message, args);
  }

  }
}

#[cfg(all(feature = "have_gnutls", not(feature = "windowsnt")))]
mod gnutls_allocator {
  use super::*;
  // The library exports a data symbol containing its current non-null free
  // callback. It is not a directly exported function with this name.
  unsafe extern "C" {
    pub static mut gnutls_free: unsafe extern "C" fn(*mut c_void);
  }
}

#[cfg(all(feature = "have_gnutls", not(feature = "windowsnt")))]
DEFUN! ("gnutls-format-certificate", Fgnutls_format_certificate,
       Sgnutls_format_certificate, 1, 1, 0,
       doc: "Format a X.509 certificate to a string.\n\nGiven a PEM-encoded X.509 certificate CERT, returns a human-readable\nstring representation.",
       (cert: Lisp_Object), {
  CHECK_STRING(cert);
  let mut crt: gnutls_x509_crt_t = std::ptr::null_mut();
  let mut err = gnutls_x509_crt_init(&raw mut crt);
  check_memory_full(err);
  if err < GNUTLS_E_SUCCESS {
    error(c"gnutls-format-certificate error: %s".as_ptr(), emacs_gnutls_strerror(err));
  }
  let data = gnutls_datum_t { data: SDATA(cert), size: strlen(SSDATA(cert)) as c_uint };
  err = gnutls_x509_crt_import(crt, &raw const data, GNUTLS_X509_FMT_PEM);
  check_memory_full(err);
  if err < GNUTLS_E_SUCCESS {
    gnutls_x509_crt_deinit(crt);
    error(c"gnutls-format-certificate error: %s".as_ptr(), emacs_gnutls_strerror(err));
  }
  let mut output: gnutls_datum_t = std::mem::zeroed();
  err = gnutls_x509_crt_print(crt, GNUTLS_CRT_PRINT_FULL, &raw mut output);
  check_memory_full(err);
  if err < GNUTLS_E_SUCCESS {
    gnutls_x509_crt_deinit(crt);
    error(c"gnutls-format-certificate error: %s".as_ptr(), emacs_gnutls_strerror(err));
  }
  let result = make_unibyte_string(output.data.cast(), output.size as ptrdiff_t);
  (gnutls_allocator::gnutls_free)(output.data.cast());
  gnutls_x509_crt_deinit(crt);
  result
});

// P-401: original trust/hostname verification and boot control flow.
#[cfg(all(feature = "have_gnutls", not(feature = "windowsnt")))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn gnutls_verify_boot(proc_: Lisp_Object, mut proplist: Lisp_Object) -> Lisp_Object {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let p = XPROCESS(proc_);
  let state = (*p).gnutls_state;
  let mut peer_verification: c_uint = 0;
  let max_log_level = (*p).gnutls_log_level;
  if NILP(proplist) { proplist = Fcdr(plist_get((*p).childp, QCtls_parameters())); }
  let verify_error = plist_get(proplist, QCverify_error());
  let hostname = plist_get(proplist, QChostname());
  let verify_error_all = EQ(verify_error, Qt());
  if !verify_error_all && NILP(Flistp(verify_error)) {
    boot_error(p, c"gnutls-boot: invalid :verify_error parameter (not a list)".as_ptr());
    return Qnil();
  }
  if !STRINGP(hostname) {
    boot_error(p, c"gnutls-boot: invalid :hostname parameter (not a string)".as_ptr());
    return Qnil();
  }
  let c_hostname = SSDATA(hostname);
  let mut ret = gnutls_certificate_verify_peers2(state, &raw mut peer_verification);
  if ret < GNUTLS_E_SUCCESS { return gnutls_make_error(ret); }
  (*p).gnutls_peer_verification = peer_verification;
  let warnings = plist_get(Fgnutls_peer_status(proc_), QCwarnings());
  let mut tail = warnings;
  while CONSP(tail) {
    let warning = XCAR(tail);
    let text = Fgnutls_peer_status_warning_describe(warning);
    if !NILP(text) { GNUTLS_LOG2!(1, max_log_level, "verification:", SSDATA(text)); }
    tail = XCDR(tail);
  }
  if peer_verification != 0 {
    if verify_error_all || !NILP(Fmember(QCtrustfiles(), verify_error)) {
      emacs_gnutls_deinit(proc_);
      boot_error(p, c"Certificate validation failed %s, verification code %x".as_ptr(), c_hostname, peer_verification);
      return Qnil();
    }
    GNUTLS_LOG2!(1, max_log_level, "certificate validation failed:", c_hostname);
  }
  if gnutls_certificate_type_get(state) == GNUTLS_CRT_X509 {
    let mut cert_list_length: c_uint = 0;
    let cert_list = gnutls_certificate_get_peers(state, &raw mut cert_list_length);
    let mut failed_import = 0;
    if cert_list.is_null() {
      emacs_gnutls_deinit(proc_);
      boot_error(p, c"No x509 certificate was found\n".as_ptr());
      return Qnil();
    }
    (*p).gnutls_certificates = xmalloc(cert_list_length as size_t * std::mem::size_of::<gnutls_x509_crt_t>()).cast();
    (*p).gnutls_certificates_length = cert_list_length as c_int;
    let mut i = cert_list_length as c_int - 1;
    while i >= 0 {
      let mut cert: gnutls_x509_crt_t = std::ptr::null_mut();
      // Preserve the original distinction: this init return is not assigned to ret.
      gnutls_x509_crt_init(&raw mut cert);
      if ret < GNUTLS_E_SUCCESS { failed_import = ret; }
      else {
        ret = gnutls_x509_crt_import(cert, cert_list.add(i as usize), GNUTLS_X509_FMT_DER);
        if ret < GNUTLS_E_SUCCESS { failed_import = ret; }
      }
      *(*p).gnutls_certificates.add(i as usize) = cert;
      i -= 1;
    }
    if failed_import != 0 {
      gnutls_deinit_certificates(p);
      return gnutls_make_error(failed_import);
    }
    let err = gnutls_x509_crt_check_hostname(*(*p).gnutls_certificates, c_hostname);
    check_memory_full(err as c_int);
    if err == 0 {
      (*p).gnutls_extra_peer_verification |= CERTIFICATE_NOT_MATCHING as c_uint;
      if verify_error_all || !NILP(Fmember(QChostname(), verify_error)) {
        emacs_gnutls_deinit(proc_);
        boot_error(p, c"The x509 certificate does not match \"%s\"".as_ptr(), c_hostname);
        return Qnil();
      }
      GNUTLS_LOG2!(1, max_log_level, "x509 certificate does not match:", c_hostname);
    }
  }
  (*p).set_gnutls_p(true);
  gnutls_make_error(ret)

  }
}

#[cfg(all(feature = "have_gnutls", feature = "have_gnutls_certificate_set_x509_key_file2", not(feature = "windowsnt")))]
unsafe fn key_file2_aux(flags: Lisp_Object) -> c_uint {
  let mut rv = 0;
  let mut tail = flags;
  FOR_EACH_TAIL_SAFE!(tail, {
    let flag = XCAR(tail);
    if EQ(flag, Qgnutls_pkcs_plain()) { rv |= GNUTLS_PKCS_PLAIN; }
    else if EQ(flag, Qgnutls_pkcs_null_password()) { rv |= GNUTLS_PKCS_NULL_PASSWORD; }
    // Other flags are compiled only when they are preprocessor macros, exactly
    // as the source's #ifdef tests; modern enum members alone do not define them.
    macro_rules! optional_flag {
      ($gate:literal, $symbol:ident, $bit:expr) => {
        #[cfg(feature = $gate)]
        if EQ(flag, $symbol()) { rv |= $bit; }
      };
    }
    optional_flag!("gnutls_pkcs_pkcs12_3des", Qgnutls_pkcs_pkcs12_3des, 1 << 1);
    optional_flag!("gnutls_pkcs_pkcs12_arcfour", Qgnutls_pkcs_pkcs12_arcfour, 1 << 2);
    optional_flag!("gnutls_pkcs_pkcs12_rc2_40", Qgnutls_pkcs_pkcs12_rc2_40, 1 << 3);
    optional_flag!("gnutls_pkcs_pbes2_3des", Qgnutls_pkcs_pbes2_3des, 1 << 4);
    optional_flag!("gnutls_pkcs_pbes2_aes_128", Qgnutls_pkcs_pbes2_aes_128, 1 << 5);
    optional_flag!("gnutls_pkcs_pbes2_aes_192", Qgnutls_pkcs_pbes2_aes_192, 1 << 6);
    optional_flag!("gnutls_pkcs_pbes2_aes_256", Qgnutls_pkcs_pbes2_aes_256, 1 << 7);
    optional_flag!("gnutls_pkcs_pbes2_des", Qgnutls_pkcs_pbes2_des, 1 << 9);
    optional_flag!("gnutls_pkcs_pbes1_des_md5", Qgnutls_pkcs_pbes1_des_md5, 1 << 10);
    optional_flag!("gnutls_pkcs_pbes2_gost_tc26z", Qgnutls_pkcs_pbes2_gost_tc26z, 1 << 11);
    optional_flag!("gnutls_pkcs_pbes2_gost_cpa", Qgnutls_pkcs_pbes2_gost_cpa, 1 << 12);
    optional_flag!("gnutls_pkcs_pbes2_gost_cpb", Qgnutls_pkcs_pbes2_gost_cpb, 1 << 13);
    optional_flag!("gnutls_pkcs_pbes2_gost_cpc", Qgnutls_pkcs_pbes2_gost_cpc, 1 << 14);
    optional_flag!("gnutls_pkcs_pbes2_gost_cpd", Qgnutls_pkcs_pbes2_gost_cpd, 1 << 15);
  });
  rv
}

#[cfg(all(feature = "have_gnutls", not(feature = "windowsnt")))]
DEFUN!("gnutls-boot", Fgnutls_boot, Sgnutls_boot, 3, 3, 0,
  doc: " Initialize GnuTLS client for process PROC with TYPE+PROPLIST.\nCurrently only client mode is supported.  Return a success/failure\nvalue you can check with `gnutls-errorp'.\n\nTYPE is a symbol, either `gnutls-anon' or `gnutls-x509pki'.\nPROPLIST is a property list with the following keys:\n\n:hostname is a string naming the remote host.\n\n:priority is a GnuTLS priority string, defaults to \"NORMAL\".\n\n:trustfiles is a list of PEM-encoded trust files for `gnutls-x509pki'.\n\n:crlfiles is a list of PEM-encoded CRL lists for `gnutls-x509pki'.\n\n:keylist is an alist of PEM-encoded key files and PEM-encoded\ncertificates for `gnutls-x509pki'.\n\n:callbacks is an alist of callback functions, see below.\n\n:loglevel is the debug level requested from GnuTLS, try 4.\n\n:verify-flags is a bitset as per GnuTLS'\ngnutls_certificate_set_verify_flags.\n\n:verify-hostname-error is ignored.  Pass :hostname in :verify-error\ninstead.\n\n:verify-error is a list of symbols to express verification checks or\nt to do all checks.  Currently it can contain `:trustfiles' and\n`:hostname' to verify the certificate or the hostname respectively.\n\n:min-prime-bits is the minimum accepted number of bits the client will\naccept in Diffie-Hellman key exchange.\n\n:complete-negotiation, if non-nil, will make negotiation complete\nbefore returning even on non-blocking sockets.\n\n:pass, the password of the private key as per GnuTLS'\ngnutls_certificate_set_x509_key_file2.  Specify as nil to have a NULL\npassword.\n\n:flags, a list of symbols relating to :pass, each specifying a flag:\nGNUTLS_PKCS_PLAIN, GNUTLS_PKCS_PKCS12_3DES,\nGNUTLS_PKCS_PKCS12_ARCFOUR, GNUTLS_PKCS_PKCS12_RC2_40,\nGNUTLS_PKCS_PBES2_3DES, GNUTLS_PKCS_PBES2_AES_128,\nGNUTLS_PKCS_PBES2_AES_192, GNUTLS_PKCS_PBES2_AES_256,\nGNUTLS_PKCS_NULL_PASSWORD, GNUTLS_PKCS_PBES2_DES,\nGNUTLS_PKCS_PBES2_DES_MD5, GNUTLS_PKCS_PBES2_GOST_TC26Z,\nGNUTLS_PKCS_PBES2_GOST_CPA, GNUTLS_PKCS_PBES2_GOST_CPB,\nGNUTLS_PKCS_PBES2_GOST_CPC, GNUTLS_PKCS_PBES2_GOST_CPD.  If not\nspecified, or if nil, the bitflag with value 0 is used.\nNote that some of these are only supported since GnuTLS 3.6.3.\n\nThe debug level will be set for this process AND globally for GnuTLS.\nSo if you set it higher or lower at any point, it affects global\ndebugging.\n\nNote that the priority is set on the client.  The server does not use\nthe protocols's priority except for disabling protocols that were not\nspecified.\n\nProcesses must be initialized with this function before other GnuTLS\nfunctions are used.  This function allocates resources which can only\nbe deallocated by calling `gnutls-deinit' or by calling it again.\n\nThe :pass and :flags keys are ignored with old versions of GnuTLS, and\n:flags is ignored if :pass is not specified.\n\nThe callbacks alist can have a `verify' key, associated with a\nverification function (UNUSED).\n\nEach authentication type may need additional information in order to\nwork.  For X.509 PKI (`gnutls-x509pki'), you probably need at least\none trustfile (usually a CA bundle).  ",
  (proc_: Lisp_Object, type_: Lisp_Object, proplist: Lisp_Object), {
  let mut max_log_level = 0;
  let mut x509_cred: gnutls_certificate_credentials_t = std::ptr::null_mut();
  let mut anon_cred: gnutls_anon_client_credentials_t = std::ptr::null_mut();
  let mut priority_string_ptr = c"NORMAL".as_ptr();
  let p = XPROCESS(proc_);
  CHECK_PROCESS(proc_);
  CHECK_SYMBOL(type_);
  CHECK_LIST(proplist);
  if NILP(Fgnutls_available_p()) {
    boot_error(p, c"GnuTLS not available".as_ptr()); return Qnil();
  }
  if !EQ(type_, Qgnutls_x509pki()) && !EQ(type_, Qgnutls_anon()) {
    boot_error(p, c"Invalid GnuTLS credential type".as_ptr()); return Qnil();
  }
  let hostname = plist_get(proplist, QChostname());
  let priority_string = plist_get(proplist, QCpriority());
  let trustfiles = plist_get(proplist, QCtrustfiles());
  let keylist = plist_get(proplist, QCkeylist());
  let crlfiles = plist_get(proplist, QCcrlfiles());
  let loglevel = plist_get(proplist, QCloglevel());
  let prime_bits = plist_get(proplist, QCmin_prime_bits());
  let pass = plist_get(proplist, QCpass());
  let flags = plist_get(proplist, QCflags());
  let c_pass = if STRINGP(pass) { SSDATA(pass) } else { std::ptr::null_mut() };
  if !STRINGP(hostname) {
    boot_error(p, c"gnutls-boot: invalid :hostname parameter (not a string)".as_ptr()); return Qnil();
  }
  let c_hostname = SSDATA(hostname);
  let mut state = (*p).gnutls_state;
  if INTEGERP(loglevel) {
    gnutls_global_set_log_function(Some(gnutls_log_function));
    #[cfg(feature = "have_gnutls3")]
    gnutls_global_set_audit_log_function(Some(gnutls_audit_log_function));
    let level = if FIXNUMP(loglevel) {
      clip_to_bounds(INT_MIN as EMACS_INT, XFIXNUM(loglevel), INT_MAX as EMACS_INT) as c_int
    } else if NILP(Fnatnump(loglevel)) { INT_MIN } else { INT_MAX };
    gnutls_global_set_log_level(level);
    max_log_level = level;
    (*p).gnutls_log_level = max_log_level;
  }
  GNUTLS_LOG2!(1, max_log_level, "connecting to host:", c_hostname);
  let global_init = emacs_gnutls_global_init();
  if !NILP(Fgnutls_errorp(global_init)) { return global_init; }
  emacs_gnutls_deinit(proc_);
  (*p).gnutls_state = std::ptr::null_mut();
  (*p).gnutls_x509_cred = std::ptr::null_mut();
  (*p).gnutls_anon_cred = std::ptr::null_mut();
  pset_gnutls_cred_type(p, type_);
  (*p).gnutls_initstage = GNUTLS_STAGE_EMPTY;
  GNUTLS_LOG!(1, max_log_level, "allocating credentials");
  if EQ(type_, Qgnutls_x509pki()) {
    let mut verify_flags_native = GNUTLS_VERIFY_ALLOW_X509_V1_CA_CRT;
    GNUTLS_LOG!(2, max_log_level, "allocating x509 credentials");
    check_memory_full(gnutls_certificate_allocate_credentials(&raw mut x509_cred));
    (*p).gnutls_x509_cred = x509_cred;
    let verify_flags = plist_get(proplist, QCverify_flags());
    if TYPE_RANGED_FIXNUMP(0, verify_flags, c_uint::MAX as EMACS_INT) {
      verify_flags_native = XFIXNAT(verify_flags) as c_uint;
      GNUTLS_LOG!(2, max_log_level, "setting verification flags");
    } else if NILP(verify_flags) {
      GNUTLS_LOG!(2, max_log_level, "using default verification flags");
    } else { GNUTLS_LOG!(2, max_log_level, "ignoring invalid verify-flags"); }
    gnutls_certificate_set_verify_flags(x509_cred, verify_flags_native);
  } else {
    GNUTLS_LOG!(2, max_log_level, "allocating anon credentials");
    check_memory_full(gnutls_anon_allocate_client_credentials(&raw mut anon_cred));
    (*p).gnutls_anon_cred = anon_cred;
  }
  (*p).gnutls_initstage = GNUTLS_STAGE_CRED_ALLOC;
  let mut ret;
  if EQ(type_, Qgnutls_x509pki()) {
    #[cfg(feature = "have_gnutls_x509_system_trust")]
    {
      ret = gnutls_certificate_set_x509_system_trust(x509_cred);
      if ret < GNUTLS_E_SUCCESS {
        check_memory_full(ret);
        if 4 <= max_log_level {
          message(c"gnutls.c: [%d] %s %d".as_ptr(), 4 as c_int,
                  c"setting system trust failed with code ".as_ptr(), ret);
        }
      }
    }
    let mut tail = trustfiles;
    while CONSP(tail) {
      let mut trustfile = XCAR(tail);
      if STRINGP(trustfile) {
        GNUTLS_LOG2!(1, max_log_level, "setting the trustfile: ", SSDATA(trustfile));
        trustfile = ENCODE_FILE(trustfile);
        ret = gnutls_certificate_set_x509_trust_file(x509_cred, SSDATA(trustfile), GNUTLS_X509_FMT_PEM);
        if ret < GNUTLS_E_SUCCESS { return gnutls_make_error(ret); }
      } else { emacs_gnutls_deinit(proc_); boot_error(p, c"Invalid trustfile".as_ptr()); return Qnil(); }
      tail = XCDR(tail);
    }
    tail = crlfiles;
    while CONSP(tail) {
      let mut crlfile = XCAR(tail);
      if STRINGP(crlfile) {
        GNUTLS_LOG2!(1, max_log_level, "setting the CRL file: ", SSDATA(crlfile));
        crlfile = ENCODE_FILE(crlfile);
        ret = gnutls_certificate_set_x509_crl_file(x509_cred, SSDATA(crlfile), GNUTLS_X509_FMT_PEM);
        if ret < GNUTLS_E_SUCCESS { return gnutls_make_error(ret); }
      } else { emacs_gnutls_deinit(proc_); boot_error(p, c"Invalid CRL file".as_ptr()); return Qnil(); }
      tail = XCDR(tail);
    }
    tail = keylist;
    while CONSP(tail) {
      let mut keyfile = Fcar(XCAR(tail));
      let mut certfile = Fcar(Fcdr(XCAR(tail)));
      if STRINGP(keyfile) && STRINGP(certfile) {
        GNUTLS_LOG2!(1, max_log_level, "setting the client key file: ", SSDATA(keyfile));
        GNUTLS_LOG2!(1, max_log_level, "setting the client cert file: ", SSDATA(certfile));
        keyfile = ENCODE_FILE(keyfile);
        certfile = ENCODE_FILE(certfile);
        #[cfg(feature = "have_gnutls_certificate_set_x509_key_file2")]
        {
          ret = if !NILP(plist_member(proplist, QCpass())) {
            let aux_key_file = key_file2_aux(flags);
            gnutls_certificate_set_x509_key_file2(x509_cred, SSDATA(certfile), SSDATA(keyfile), GNUTLS_X509_FMT_PEM, c_pass, aux_key_file)
          } else {
            gnutls_certificate_set_x509_key_file(x509_cred, SSDATA(certfile), SSDATA(keyfile), GNUTLS_X509_FMT_PEM)
          };
        }
        #[cfg(not(feature = "have_gnutls_certificate_set_x509_key_file2"))]
        { ret = gnutls_certificate_set_x509_key_file(x509_cred, SSDATA(certfile), SSDATA(keyfile), GNUTLS_X509_FMT_PEM); }
        if ret < GNUTLS_E_SUCCESS { return gnutls_make_error(ret); }
      } else {
        emacs_gnutls_deinit(proc_);
        boot_error(p, if STRINGP(keyfile) { c"Invalid client cert file".as_ptr() } else { c"Invalid client key file".as_ptr() });
        return Qnil();
      }
      tail = XCDR(tail);
    }
  }
  (*p).gnutls_initstage = GNUTLS_STAGE_FILES;
  GNUTLS_LOG!(1, max_log_level, "gnutls callbacks");
  (*p).gnutls_initstage = GNUTLS_STAGE_CALLBACKS;
  GNUTLS_LOG!(1, max_log_level, "gnutls_init");
  let mut gnutls_flags = GNUTLS_CLIENT;
  #[cfg(feature = "gnutls_nonblock")]
  if (*p).is_non_blocking_client() { gnutls_flags |= GNUTLS_NONBLOCK; }
  ret = gnutls_init(&raw mut state, gnutls_flags);
  (*p).gnutls_state = state;
  if ret < GNUTLS_E_SUCCESS { return gnutls_make_error(ret); }
  (*p).gnutls_initstage = GNUTLS_STAGE_INIT;
  if STRINGP(priority_string) {
    priority_string_ptr = SSDATA(priority_string);
    GNUTLS_LOG2!(1, max_log_level, "got non-default priority string:", priority_string_ptr);
  } else { GNUTLS_LOG2!(1, max_log_level, "using default priority string:", priority_string_ptr); }
  GNUTLS_LOG!(1, max_log_level, "setting the priority string");
  ret = gnutls_priority_set_direct(state, priority_string_ptr, std::ptr::null_mut());
  if ret < GNUTLS_E_SUCCESS { return gnutls_make_error(ret); }
  (*p).gnutls_initstage = GNUTLS_STAGE_PRIORITY;
  if FIXNUMP(prime_bits) { gnutls_dh_set_prime_bits(state, XUFIXNUM(prime_bits) as c_uint); }
  ret = if EQ(type_, Qgnutls_x509pki()) {
    gnutls_credentials_set(state, GNUTLS_CRD_CERTIFICATE, x509_cred.cast())
  } else { gnutls_credentials_set(state, GNUTLS_CRD_ANON, anon_cred.cast()) };
  if ret < GNUTLS_E_SUCCESS { return gnutls_make_error(ret); }
  if !gnutls_ip_address_p(c_hostname) {
    ret = gnutls_server_name_set(state, GNUTLS_NAME_DNS, c_hostname.cast(), strlen(c_hostname));
    if ret < GNUTLS_E_SUCCESS { return gnutls_make_error(ret); }
  }
  (*p).set_gnutls_complete_negotiation_p(!NILP(plist_get(proplist, QCcomplete_negotiation())));
  (*p).gnutls_initstage = GNUTLS_STAGE_CRED_SET;
  ret = emacs_gnutls_handshake(p);
  if ret < GNUTLS_E_SUCCESS { return gnutls_make_error(ret); }
  gnutls_verify_boot(proc_, proplist)
});

#[cfg(all(feature = "have_gnutls", not(feature = "windowsnt")))]
DEFUN!("gnutls-bye", Fgnutls_bye, Sgnutls_bye, 2, 2, 0,
  doc: " Terminate current GnuTLS connection for process PROC.\nThe connection should have been initiated using `gnutls-handshake'.\n\nIf CONT is not nil the TLS connection gets terminated and further\nreceives and sends will be disallowed.  If the return value is zero you\nmay continue using the connection.  If CONT is nil, GnuTLS actually\nsends an alert containing a close request and waits for the peer to\nreply with the same message.  In order to reuse the connection you\nshould wait for an EOF from the peer.\n\nThis function may also return `gnutls-e-again', or\n`gnutls-e-interrupted'.  ",
  (proc_: Lisp_Object, cont: Lisp_Object), {
  CHECK_PROCESS(proc_);
  let p = XPROCESS(proc_);
  let state = (*p).gnutls_state;
  if !(*p).gnutls_certificates.is_null() { gnutls_deinit_certificates(p); }
  let ret = gnutls_bye(state, if NILP(cont) { GNUTLS_SHUT_RDWR } else { GNUTLS_SHUT_WR });
  gnutls_make_error(ret)
});

#[cfg(all(feature = "have_gnutls3", not(feature = "windowsnt")))]
DEFUN!("gnutls-ciphers", Fgnutls_ciphers, Sgnutls_ciphers, 0, 0, 0,
  doc: " Return alist of GnuTLS symmetric cipher descriptions as plists.\nThe alist key is the cipher name. ", (), {
  let mut ciphers = Qnil();
  let gciphers = gnutls_cipher_list();
  let mut pos = 0;
  while *gciphers.add(pos) != 0 {
    let gca = *gciphers.add(pos);
    pos += 1;
    if gca == GNUTLS_CIPHER_NULL { continue; }
    let cipher_name = gnutls_cipher_get_name(gca);
    if cipher_name.is_null() { continue; }
    let cipher_symbol = intern(cipher_name);
    let cipher_tag_size = gnutls_cipher_get_tag_size(gca) as ptrdiff_t;
    let cp = listn(15, cipher_symbol,
      QCcipher_id(), make_fixnum(gca as EMACS_INT), QCtype(), Qgnutls_type_cipher(),
      QCcipher_aead_capable(), if cipher_tag_size == 0 { Qnil() } else { Qt() },
      QCcipher_tagsize(), make_fixnum(cipher_tag_size as EMACS_INT),
      QCcipher_blocksize(), make_fixnum(gnutls_cipher_get_block_size(gca) as EMACS_INT),
      QCcipher_keysize(), make_fixnum(gnutls_cipher_get_key_size(gca) as EMACS_INT),
      QCcipher_ivsize(), make_fixnum(gnutls_cipher_get_iv_size(gca) as EMACS_INT));
    ciphers = Fcons(cp, ciphers);
  }
  ciphers
});

#[cfg(all(feature = "have_gnutls3", not(feature = "windowsnt")))]
unsafe fn gnutls_symmetric_aead(encrypting: bool, gca: gnutls_cipher_algorithm_t,
    cipher: Lisp_Object, kdata: *const c_char, ksize: ptrdiff_t,
    vdata: *const c_char, vsize: ptrdiff_t, idata: *const c_char,
    isize: ptrdiff_t, mut aead_auth: Lisp_Object) -> Lisp_Object {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  #[cfg(feature = "have_gnutls_aead")]
  {
    let desc = if encrypting { c"encrypt" } else { c"decrypt" };
    let actual_iv = make_unibyte_string(vdata, vsize);
    let mut acipher: gnutls_aead_cipher_hd_t = std::ptr::null_mut();
    let key_datum = gnutls_datum_t { data: kdata as *mut c_uchar, size: ksize as c_uint };
    let mut ret = gnutls_aead_cipher_init(&raw mut acipher, gca, &raw const key_datum);
    if ret < GNUTLS_E_SUCCESS {
      error(c"GnuTLS AEAD cipher %s/%s initialization failed: %s".as_ptr(),
            gnutls_cipher_get_name(gca), desc.as_ptr(), emacs_gnutls_strerror(ret));
    }
    let cipher_tag_size = gnutls_cipher_get_tag_size(gca) as ptrdiff_t;
    let (tagged_size, overflowed) = isize.overflowing_add(cipher_tag_size);
    if overflowed || size_t::MAX < tagged_size as size_t { memory_full_up(); }
    let mut storage_length = tagged_size as size_t;
    USE_SAFE_ALLOCA!(sa_avail, sa_count, sa_mark);
    let storage = SAFE_ALLOCA!(sa_avail, storage_length) as *mut c_char;
    let mut aead_auth_data: *const c_char = std::ptr::null();
    let mut aead_auth_size: ptrdiff_t = 0;
    if !NILP(aead_auth) {
      if BUFFERP(aead_auth) || STRINGP(aead_auth) { aead_auth = list1(aead_auth); }
      CHECK_CONS(aead_auth);
      let mut astart_byte = 0;
      let mut aend_byte = 0;
      let adata = extract_data_from_object(aead_auth, &raw mut astart_byte, &raw mut aend_byte);
      if adata.is_null() { error(c"GnuTLS AEAD cipher auth extraction failed".as_ptr()); }
      aead_auth_data = adata;
      aead_auth_size = aend_byte - astart_byte;
    }
    ret = if encrypting {
      gnutls_aead_cipher_encrypt(acipher, vdata.cast(), vsize as size_t,
        aead_auth_data.cast(), aead_auth_size as size_t, cipher_tag_size as size_t,
        idata.cast(), isize as size_t, storage.cast(), &raw mut storage_length)
    } else {
      gnutls_aead_cipher_decrypt(acipher, vdata.cast(), vsize as size_t,
        aead_auth_data.cast(), aead_auth_size as size_t, cipher_tag_size as size_t,
        idata.cast(), isize as size_t, storage.cast(), &raw mut storage_length)
    };
    let mut output = Qnil();
    if GNUTLS_E_SUCCESS <= ret { output = make_unibyte_string(storage, storage_length as ptrdiff_t); }
    memset_explicit(storage.cast(), 0, storage_length);
    gnutls_aead_cipher_deinit(acipher);
    if ret < GNUTLS_E_SUCCESS {
      error(if encrypting { c"GnuTLS AEAD cipher %s encryption failed: %s".as_ptr() }
            else { c"GnuTLS AEAD cipher %s decryption failed: %s".as_ptr() },
            gnutls_cipher_get_name(gca), emacs_gnutls_strerror(ret));
    }
    SAFE_FREE!(sa_count, sa_mark);
    return list2(output, actual_iv);
  }
  #[cfg(not(feature = "have_gnutls_aead"))]
  { error(c"GnuTLS AEAD cipher %jd is invalid or not found".as_ptr(), gca as i64); }

  }
}

#[cfg(all(feature = "have_gnutls3", not(feature = "windowsnt")))]
static mut cipher_cache: Lisp_Object = lisp_h_Qnil();

// Original compatibility definitions for library versions predating these APIs.
#[cfg(all(feature = "have_gnutls3", not(feature = "windowsnt"), not(feature = "have_gnutls_cipher_get_iv_size")))]
unsafe fn gnutls_cipher_get_iv_size(cipher: gnutls_cipher_algorithm_t) -> c_uint {
  gnutls_cipher_get_block_size(cipher)
}
#[cfg(all(feature = "have_gnutls3", not(feature = "windowsnt"), not(feature = "have_gnutls_cipher_get_tag_size")))]
unsafe fn gnutls_cipher_get_tag_size(_cipher: gnutls_cipher_algorithm_t) -> c_uint { 0 }
#[cfg(all(feature = "have_gnutls3", not(feature = "windowsnt"), not(feature = "have_gnutls_digest_list")))]
unsafe fn gnutls_digest_list() -> *const gnutls_digest_algorithm_t { gnutls_mac_list().cast() }
#[cfg(all(feature = "have_gnutls3", not(feature = "windowsnt"), not(feature = "have_gnutls_digest_list")))]
unsafe fn gnutls_digest_get_name(id: gnutls_digest_algorithm_t) -> *const c_char { gnutls_mac_get_name(id) }

#[cfg(all(feature = "have_gnutls3", not(feature = "windowsnt")))]
unsafe fn gnutls_symmetric(encrypting: bool, mut cipher: Lisp_Object,
    mut key: Lisp_Object, mut iv: Lisp_Object, mut input: Lisp_Object,
    aead_auth: Lisp_Object) -> Lisp_Object {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  if BUFFERP(key) || STRINGP(key) { key = list1(key); }
  CHECK_CONS(key);
  if BUFFERP(input) || STRINGP(input) { input = list1(input); }
  CHECK_CONS(input);
  if BUFFERP(iv) || STRINGP(iv) { iv = list1(iv); }
  CHECK_CONS(iv);
  let desc = if encrypting { c"encrypt" } else { c"decrypt" };
  let mut gca = GNUTLS_CIPHER_UNKNOWN;
  let mut info = Qnil();
  if STRINGP(cipher) { cipher = intern(SSDATA(cipher)); }
  if SYMBOLP(cipher) {
    if NILP(cipher_cache) { cipher_cache = Fgnutls_ciphers(); }
    info = Fassq(cipher, cipher_cache);
    if !CONSP(info) { xsignal2(Qerror(), build_string(c"GnuTLS cipher is invalid or not found".as_ptr()), cipher); }
    info = XCDR(info);
  } else if TYPE_RANGED_FIXNUMP(c_int::MIN as EMACS_INT, cipher, c_int::MAX as EMACS_INT) {
    gca = XFIXNUM(cipher) as gnutls_cipher_algorithm_t;
  } else { info = cipher; }
  if !NILP(info) && CONSP(info) {
    let v = plist_get(info, QCcipher_id());
    if TYPE_RANGED_FIXNUMP(c_int::MIN as EMACS_INT, v, c_int::MAX as EMACS_INT) { gca = XFIXNUM(v) as gnutls_cipher_algorithm_t; }
  }
  let key_size = gnutls_cipher_get_key_size(gca) as ptrdiff_t;
  if key_size == 0 { xsignal2(Qerror(), build_string(c"GnuTLS cipher is invalid or not found".as_ptr()), cipher); }
  let (mut kstart_byte, mut kend_byte) = (0, 0);
  let kdata = extract_data_from_object(key, &raw mut kstart_byte, &raw mut kend_byte);
  if kdata.is_null() { error(c"GnuTLS cipher key extraction failed".as_ptr()); }
  if kend_byte - kstart_byte != key_size {
    error(c"GnuTLS cipher %s/%s key length %td is not equal to the required %td".as_ptr(),
      gnutls_cipher_get_name(gca), desc.as_ptr(), kend_byte - kstart_byte, key_size);
  }
  let (mut vstart_byte, mut vend_byte) = (0, 0);
  let vdata = extract_data_from_object(iv, &raw mut vstart_byte, &raw mut vend_byte);
  if vdata.is_null() { error(c"GnuTLS cipher IV extraction failed".as_ptr()); }
  let iv_size = gnutls_cipher_get_iv_size(gca) as ptrdiff_t;
  if vend_byte - vstart_byte != iv_size {
    error(c"GnuTLS cipher %s/%s IV length %td is not equal to the required %td".as_ptr(),
      gnutls_cipher_get_name(gca), desc.as_ptr(), vend_byte - vstart_byte, iv_size);
  }
  let actual_iv = make_unibyte_string(vdata, vend_byte - vstart_byte);
  let (mut istart_byte, mut iend_byte) = (0, 0);
  let idata = extract_data_from_object(input, &raw mut istart_byte, &raw mut iend_byte);
  if idata.is_null() { error(c"GnuTLS cipher input extraction failed".as_ptr()); }
  if gnutls_cipher_get_tag_size(gca) > 0 {
    let aead_output = gnutls_symmetric_aead(encrypting, gca, cipher,
      kdata, kend_byte - kstart_byte, vdata, vend_byte - vstart_byte,
      idata, iend_byte - istart_byte, aead_auth);
    if STRINGP(XCAR(key)) { Fclear_string(XCAR(key)); }
    return aead_output;
  }
  let cipher_block_size = gnutls_cipher_get_block_size(gca) as ptrdiff_t;
  if (iend_byte - istart_byte) % cipher_block_size != 0 {
    error(c"GnuTLS cipher %s/%s input block length %td is not a multiple of the required %td".as_ptr(),
      gnutls_cipher_get_name(gca), desc.as_ptr(), iend_byte - istart_byte, cipher_block_size);
  }
  let mut hcipher: gnutls_cipher_hd_t = std::ptr::null_mut();
  let key_datum = gnutls_datum_t { data: kdata as *mut c_uchar, size: (kend_byte - kstart_byte) as c_uint };
  let mut ret = gnutls_cipher_init(&raw mut hcipher, gca, &raw const key_datum, std::ptr::null());
  if ret < GNUTLS_E_SUCCESS {
    error(c"GnuTLS cipher %s/%s initialization failed: %s".as_ptr(),
      gnutls_cipher_get_name(gca), desc.as_ptr(), emacs_gnutls_strerror(ret));
  }
  gnutls_cipher_set_iv(hcipher, vdata.cast(), (vend_byte - vstart_byte) as size_t);
  let storage_length = iend_byte - istart_byte;
  let storage = make_uninit_string(storage_length as EMACS_INT);
  ret = if encrypting {
    gnutls_cipher_encrypt2(hcipher, idata.cast(), (iend_byte - istart_byte) as size_t,
      SSDATA(storage).cast(), storage_length as size_t)
  } else {
    gnutls_cipher_decrypt2(hcipher, idata.cast(), (iend_byte - istart_byte) as size_t,
      SSDATA(storage).cast(), storage_length as size_t)
  };
  if STRINGP(XCAR(key)) { Fclear_string(XCAR(key)); }
  if ret < GNUTLS_E_SUCCESS {
    gnutls_cipher_deinit(hcipher);
    error(if encrypting { c"GnuTLS cipher %s encryption failed: %s".as_ptr() }
          else { c"GnuTLS cipher %s decryption failed: %s".as_ptr() },
      gnutls_cipher_get_name(gca), emacs_gnutls_strerror(ret));
  }
  gnutls_cipher_deinit(hcipher);
  list2(storage, actual_iv)

  }
}

#[cfg(all(feature = "have_gnutls3", not(feature = "windowsnt")))]
DEFUN!("gnutls-symmetric-encrypt", Fgnutls_symmetric_encrypt, Sgnutls_symmetric_encrypt, 4, 5, 0,
  doc: " Encrypt INPUT with symmetric CIPHER, KEY+AEAD_AUTH, and IV to a unibyte string.\n\nReturn nil on error.\n\nThe KEY can be specified as a buffer or string or in other ways (see\nInfo node `(elisp)Format of GnuTLS Cryptography Inputs').  The KEY\nwill be wiped after use if it's a string.\n\nThe IV and INPUT and the optional AEAD_AUTH can also be specified as a\nbuffer or string or in other ways.\n\nThe alist of symmetric ciphers can be obtained with `gnutls-ciphers'.\nThe CIPHER may be a string or symbol matching a key in that alist, or\na plist with the :cipher-id numeric property, or the number itself.\n\nAEAD ciphers: these ciphers will have a `gnutls-ciphers' entry with\n:cipher-aead-capable set to t.  AEAD_AUTH can be supplied for\nthese AEAD ciphers, but it may still be omitted (nil) as well. ",
  (cipher: Lisp_Object, key: Lisp_Object, iv: Lisp_Object, input: Lisp_Object, aead_auth: Lisp_Object), {
  gnutls_symmetric(true, cipher, key, iv, input, aead_auth)
});

#[cfg(all(feature = "have_gnutls3", not(feature = "windowsnt")))]
DEFUN!("gnutls-symmetric-decrypt", Fgnutls_symmetric_decrypt, Sgnutls_symmetric_decrypt, 4, 5, 0,
  doc: " Decrypt INPUT with symmetric CIPHER, KEY+AEAD_AUTH, and IV to a unibyte string.\n\nReturn nil on error.\n\nThe KEY can be specified as a buffer or string or in other ways (see\nInfo node `(elisp)Format of GnuTLS Cryptography Inputs').  The KEY\nwill be wiped after use if it's a string.\n\nThe IV and INPUT and the optional AEAD_AUTH can also be specified as a\nbuffer or string or in other ways.\n\nThe alist of symmetric ciphers can be obtained with `gnutls-ciphers'.\nThe CIPHER may be a string or symbol matching a key in that alist, or\na plist with the `:cipher-id' numeric property, or the number itself.\n\nAEAD ciphers: these ciphers will have a `gnutls-ciphers' entry with\n:cipher-aead-capable set to t.  AEAD_AUTH can be supplied for\nthese AEAD ciphers, but it may still be omitted (nil) as well. ",
  (cipher: Lisp_Object, key: Lisp_Object, iv: Lisp_Object, input: Lisp_Object, aead_auth: Lisp_Object), {
  gnutls_symmetric(false, cipher, key, iv, input, aead_auth)
});

#[cfg(all(feature = "have_gnutls3", not(feature = "windowsnt")))]
DEFUN!("gnutls-macs", Fgnutls_macs, Sgnutls_macs, 0, 0, 0,
  doc: " Return alist of GnuTLS mac-algorithm method descriptions as plists.\n\nUse the value of the alist (extract it with `alist-get' for instance)\nwith `gnutls-hash-mac'.  The alist key is the mac-algorithm method\nname. ", (), {
  let mut mac_algorithms = Qnil();
  let macs = gnutls_mac_list();
  let mut pos = 0;
  while *macs.add(pos) != 0 {
    let gma = *macs.add(pos);
    let gma_symbol = intern(gnutls_mac_get_name(gma));
    let mut nonce_size: size_t = 0;
    #[cfg(feature = "have_gnutls_mac_get_nonce_size")]
    { nonce_size = gnutls_mac_get_nonce_size(gma); }
    let mp = listn(11, gma_symbol, QCmac_algorithm_id(), make_fixnum(gma as EMACS_INT),
      QCtype(), Qgnutls_type_mac_algorithm(), QCmac_algorithm_length(), make_fixnum(gnutls_hmac_get_len(gma) as EMACS_INT),
      QCmac_algorithm_keysize(), make_fixnum(gnutls_mac_get_key_size(gma) as EMACS_INT),
      QCmac_algorithm_noncesize(), make_fixnum(nonce_size as EMACS_INT));
    mac_algorithms = Fcons(mp, mac_algorithms);
    pos += 1;
  }
  mac_algorithms
});

#[cfg(all(feature = "have_gnutls3", not(feature = "windowsnt")))]
DEFUN!("gnutls-digests", Fgnutls_digests, Sgnutls_digests, 0, 0, 0,
  doc: " Return alist of GnuTLS digest-algorithm method descriptions as plists.\n\nUse the value of the alist (extract it with `alist-get' for instance)\nwith `gnutls-hash-digest'.  The alist key is the digest-algorithm\nmethod name. ", (), {
  let mut digest_algorithms = Qnil();
  let digests = gnutls_digest_list();
  let mut pos = 0;
  while *digests.add(pos) != 0 {
    let gda = *digests.add(pos);
    let gda_symbol = intern(gnutls_digest_get_name(gda));
    let mp = listn(7, gda_symbol, QCdigest_algorithm_id(), make_fixnum(gda as EMACS_INT),
      QCtype(), Qgnutls_type_digest_algorithm(), QCdigest_algorithm_length(), make_fixnum(gnutls_hash_get_len(gda) as EMACS_INT));
    digest_algorithms = Fcons(mp, digest_algorithms);
    pos += 1;
  }
  digest_algorithms
});

#[cfg(all(feature = "have_gnutls3", not(feature = "windowsnt")))]
DEFUN!("gnutls-hash-mac", Fgnutls_hash_mac, Sgnutls_hash_mac, 3, 3, 0,
  doc: " Hash INPUT with HASH-METHOD and KEY into a unibyte string.\n\nReturn nil on error.\n\nThe KEY can be specified as a buffer or string or in other ways (see\nInfo node `(elisp)Format of GnuTLS Cryptography Inputs').  The KEY\nwill be wiped after use if it's a string.\n\nThe INPUT can also be specified as a buffer or string or in other\nways.\n\nThe alist of MAC algorithms can be obtained with `gnutls-macs'.  The\nHASH-METHOD may be a string or symbol matching a key in that alist, or\na plist with the `:mac-algorithm-id' numeric property, or the number\nitself. ",
  (hash_method: Lisp_Object, key: Lisp_Object, input: Lisp_Object), {
  let mut hash_method = hash_method;
  let mut key = key;
  let mut input = input;
  if BUFFERP(input) || STRINGP(input) { input = list1(input); }
  CHECK_CONS(input);
  if BUFFERP(key) || STRINGP(key) { key = list1(key); }
  CHECK_CONS(key);
  let mut gma = GNUTLS_MAC_UNKNOWN;
  let mut info = Qnil();
  if STRINGP(hash_method) { hash_method = intern(SSDATA(hash_method)); }
  if SYMBOLP(hash_method) {
    info = Fassq(hash_method, Fgnutls_macs());
    if !CONSP(info) { xsignal2(Qerror(), build_string(c"GnuTLS MAC-method is invalid or not found".as_ptr()), hash_method); }
    info = XCDR(info);
  } else if TYPE_RANGED_FIXNUMP(c_int::MIN as EMACS_INT, hash_method, c_int::MAX as EMACS_INT) {
    gma = XFIXNUM(hash_method) as gnutls_mac_algorithm_t;
  } else { info = hash_method; }
  if !NILP(info) && CONSP(info) {
    let v = plist_get(info, QCmac_algorithm_id());
    if TYPE_RANGED_FIXNUMP(c_int::MIN as EMACS_INT, v, c_int::MAX as EMACS_INT) { gma = XFIXNUM(v) as gnutls_mac_algorithm_t; }
  }
  let digest_length = gnutls_hmac_get_len(gma) as ptrdiff_t;
  if digest_length == 0 { xsignal2(Qerror(), build_string(c"GnuTLS MAC-method is invalid or not found".as_ptr()), hash_method); }
  let (mut kstart_byte, mut kend_byte) = (0, 0);
  let kdata = extract_data_from_object(key, &raw mut kstart_byte, &raw mut kend_byte);
  if kdata.is_null() { error(c"GnuTLS MAC key extraction failed".as_ptr()); }
  let mut hmac: gnutls_hmac_hd_t = std::ptr::null_mut();
  let mut ret = gnutls_hmac_init(&raw mut hmac, gma, kdata.offset(kstart_byte).cast(), (kend_byte - kstart_byte) as size_t);
  if ret < GNUTLS_E_SUCCESS {
    error(c"GnuTLS MAC %s initialization failed: %s".as_ptr(), gnutls_mac_get_name(gma), emacs_gnutls_strerror(ret));
  }
  let (mut istart_byte, mut iend_byte) = (0, 0);
  let idata = extract_data_from_object(input, &raw mut istart_byte, &raw mut iend_byte);
  if idata.is_null() { error(c"GnuTLS MAC input extraction failed".as_ptr()); }
  let digest = make_uninit_string(digest_length as EMACS_INT);
  ret = gnutls_hmac(hmac, idata.offset(istart_byte).cast(), (iend_byte - istart_byte) as size_t);
  if STRINGP(XCAR(key)) { Fclear_string(XCAR(key)); }
  if ret < GNUTLS_E_SUCCESS {
    gnutls_hmac_deinit(hmac, std::ptr::null_mut());
    error(c"GnuTLS MAC %s application failed: %s".as_ptr(), gnutls_mac_get_name(gma), emacs_gnutls_strerror(ret));
  }
  gnutls_hmac_output(hmac, SSDATA(digest).cast());
  gnutls_hmac_deinit(hmac, std::ptr::null_mut());
  digest
});

#[cfg(all(feature = "have_gnutls3", not(feature = "windowsnt")))]
DEFUN!("gnutls-hash-digest", Fgnutls_hash_digest, Sgnutls_hash_digest, 2, 2, 0,
  doc: " Digest INPUT with DIGEST-METHOD into a unibyte string.\n\nReturn nil on error.\n\nThe INPUT can be specified as a buffer or string or in other\nways (see Info node `(elisp)Format of GnuTLS Cryptography Inputs').\n\nThe alist of digest algorithms can be obtained with `gnutls-digests'.\nThe DIGEST-METHOD may be a string or symbol matching a key in that\nalist, or a plist with the `:digest-algorithm-id' numeric property, or\nthe number itself. ",
  (digest_method: Lisp_Object, input: Lisp_Object), {
  let mut digest_method = digest_method;
  let mut input = input;
  if BUFFERP(input) || STRINGP(input) { input = list1(input); }
  CHECK_CONS(input);
  let mut gda = GNUTLS_DIG_UNKNOWN;
  let mut info = Qnil();
  if STRINGP(digest_method) { digest_method = intern(SSDATA(digest_method)); }
  if SYMBOLP(digest_method) {
    info = Fassq(digest_method, Fgnutls_digests());
    if !CONSP(info) { xsignal2(Qerror(), build_string(c"GnuTLS digest-method is invalid or not found".as_ptr()), digest_method); }
    info = XCDR(info);
  } else if TYPE_RANGED_FIXNUMP(c_int::MIN as EMACS_INT, digest_method, c_int::MAX as EMACS_INT) {
    gda = XFIXNUM(digest_method) as gnutls_digest_algorithm_t;
  } else { info = digest_method; }
  if !NILP(info) && CONSP(info) {
    let v = plist_get(info, QCdigest_algorithm_id());
    if TYPE_RANGED_FIXNUMP(c_int::MIN as EMACS_INT, v, c_int::MAX as EMACS_INT) { gda = XFIXNUM(v) as gnutls_digest_algorithm_t; }
  }
  let digest_length = gnutls_hash_get_len(gda) as ptrdiff_t;
  if digest_length == 0 { xsignal2(Qerror(), build_string(c"GnuTLS digest-method is invalid or not found".as_ptr()), digest_method); }
  let mut hash: gnutls_hash_hd_t = std::ptr::null_mut();
  let mut ret = gnutls_hash_init(&raw mut hash, gda);
  if ret < GNUTLS_E_SUCCESS { error(c"GnuTLS digest initialization failed: %s".as_ptr(), emacs_gnutls_strerror(ret)); }
  let digest = make_uninit_string(digest_length as EMACS_INT);
  let (mut istart_byte, mut iend_byte) = (0, 0);
  let idata = extract_data_from_object(input, &raw mut istart_byte, &raw mut iend_byte);
  if idata.is_null() { error(c"GnuTLS digest input extraction failed".as_ptr()); }
  ret = gnutls_hash(hash, idata.offset(istart_byte).cast(), (iend_byte - istart_byte) as size_t);
  if ret < GNUTLS_E_SUCCESS {
    gnutls_hash_deinit(hash, std::ptr::null_mut());
    error(c"GnuTLS digest application failed: %s".as_ptr(), emacs_gnutls_strerror(ret));
  }
  gnutls_hash_output(hash, SSDATA(digest).cast());
  gnutls_hash_deinit(hash, std::ptr::null_mut());
  digest
});


// The `DEFVAR_*` variables this C file owns: C's storage is `globals.f_<vname>`
// (lisp.h 3495-3530) and `struct emacs_globals globals` is defined once, in alloc.c; the port's
// `include/globals.rs` mirrors the names as declarations, so the unit that owns one defines it
// here (status 3.29, the callint.rs shape).
#[unsafe(no_mangle)]
pub static mut Vlibgnutls_version: Lisp_Object = lisp_h_Qnil ();

DEFUN! ("gnutls-available-p", Fgnutls_available_p, Sgnutls_available_p, 0, 0, 0,
       doc: "Return list of capabilities if GnuTLS is available in this instance of Emacs.\n\n...if supported         : then...\nGnuTLS 3 or higher      : the list will contain `gnutls3'.\nGnuTLS MACs             : the list will contain `macs'.\nGnuTLS digests          : the list will contain `digests'.\nGnuTLS symmetric ciphers: the list will contain `ciphers'.\nGnuTLS AEAD ciphers     : the list will contain `AEAD-ciphers'.\n%DUMBFW                 : the list will contain `ClientHello\\ Padding'.\nAny GnuTLS extension with ID up to 100\n                        : the list will contain its name.  ",
       (),
       {
  let mut capabilities: Lisp_Object = Qnil ();

  #[cfg(all(feature = "have_gnutls", not(feature = "windowsnt")))]
  {
    capabilities = Fcons(Qgnutls(), capabilities);
    #[cfg(feature = "have_gnutls_ext__dumbfw")]
    { capabilities = Fcons(QClientHello_Padding(), capabilities); }
    #[cfg(feature = "have_gnutls3")]
    {
      capabilities = Fcons(Qgnutls3(), capabilities);
      capabilities = Fcons(Qdigests(), capabilities);
      capabilities = Fcons(Qciphers(), capabilities);
      #[cfg(feature = "have_gnutls_aead")]
      { capabilities = Fcons(QAEAD_ciphers(), capabilities); }
      capabilities = Fcons(Qmacs(), capabilities);
      #[cfg(feature = "have_gnutls_ext_get_name")]
      for ext in 0..100 {
        let name = gnutls_ext_get_name(ext);
        if !name.is_null() {
          let cap = intern(name);
          if NILP(Fmemq(cap, capabilities)) { capabilities = Fcons(cap, capabilities); }
        }
      }
    }
  }

  return capabilities;
});

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn syms_of_gnutls ()
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  DEFVAR_LISP! (c"libgnutls-version", Vlibgnutls_version,
		"The version of libgnutls that Emacs was compiled with.\nThe version number is encoded as an integer with the major version in\nthe ten thousands place, minor version in the hundreds, and patch\nlevel in the ones.  For builds without libgnutls, the value is -1.");
  #[cfg(feature = "have_gnutls")]
  { Vlibgnutls_version = make_fixnum(GNUTLS_COMPILED_VERSION as EMACS_INT); }
  #[cfg(not(feature = "have_gnutls"))]
  { Vlibgnutls_version = make_fixnum(-1); }
  #[cfg(all(feature = "have_gnutls", not(feature = "windowsnt")))]
  {
    gnutls_global_initialized = false;
    PDUMPER_IGNORE!(gnutls_global_initialized);
  DEFSYM!(Qgnutls_code, "gnutls-code");
  DEFSYM!(Qgnutls_anon, "gnutls-anon");
  DEFSYM!(Qgnutls_x509pki, "gnutls-x509pki");

  DEFSYM!(QChostname, ":hostname");
  DEFSYM!(QCpriority, ":priority");
  DEFSYM!(QCtrustfiles, ":trustfiles");
  DEFSYM!(QCkeylist, ":keylist");
  DEFSYM!(QCcrlfiles, ":crlfiles");
  DEFSYM!(QCmin_prime_bits, ":min-prime-bits");
  DEFSYM!(QCloglevel, ":loglevel");
  DEFSYM!(QCcomplete_negotiation, ":complete-negotiation");
  DEFSYM!(QCpass, ":pass");
  DEFSYM!(QCflags, ":flags");
  DEFSYM!(QCverify_flags, ":verify-flags");
  DEFSYM!(QCverify_error, ":verify-error");
  DEFSYM!(Qgnutls_pkcs_plain, "GNUTLS_PKCS_PLAIN");
  DEFSYM!(Qgnutls_pkcs_pkcs12_3des, "GNUTLS_PKCS_PKCS12_3DES");
  DEFSYM!(Qgnutls_pkcs_pkcs12_arcfour, "GNUTLS_PKCS_PKCS12_ARCFOUR");
  DEFSYM!(Qgnutls_pkcs_pkcs12_rc2_40, "GNUTLS_PKCS_PKCS12_RC2_40");
  DEFSYM!(Qgnutls_pkcs_pbes2_3des, "GNUTLS_PKCS_PBES2_3DES");
  DEFSYM!(Qgnutls_pkcs_pbes2_aes_128, "GNUTLS_PKCS_PBES2_AES_128");
  DEFSYM!(Qgnutls_pkcs_pbes2_aes_192, "GNUTLS_PKCS_PBES2_AES_192");
  DEFSYM!(Qgnutls_pkcs_pbes2_aes_256, "GNUTLS_PKCS_PBES2_AES_256");
  DEFSYM!(Qgnutls_pkcs_null_password, "GNUTLS_PKCS_NULL_PASSWORD");
  DEFSYM!(Qgnutls_pkcs_pbes2_des, "GNUTLS_PKCS_PBES2_DES");
  DEFSYM!(Qgnutls_pkcs_pbes1_des_md5, "GNUTLS_PKCS_PBES1_DES_MD5");
  DEFSYM!(Qgnutls_pkcs_pbes2_gost_tc26z, "GNUTLS_PKCS_PBES2_GOST_TC26Z");
  DEFSYM!(Qgnutls_pkcs_pbes2_gost_cpa, "GNUTLS_PKCS_PBES2_GOST_CPA");
  DEFSYM!(Qgnutls_pkcs_pbes2_gost_cpb, "GNUTLS_PKCS_PBES2_GOST_CPB");
  DEFSYM!(Qgnutls_pkcs_pbes2_gost_cpc, "GNUTLS_PKCS_PBES2_GOST_CPC");
  DEFSYM!(Qgnutls_pkcs_pbes2_gost_cpd, "GNUTLS_PKCS_PBES2_GOST_CPD");

  DEFSYM!(QCcipher_id, ":cipher-id");
  DEFSYM!(QCcipher_aead_capable, ":cipher-aead-capable");
  DEFSYM!(QCcipher_blocksize, ":cipher-blocksize");
  DEFSYM!(QCcipher_keysize, ":cipher-keysize");
  DEFSYM!(QCcipher_tagsize, ":cipher-tagsize");
  DEFSYM!(QCcipher_ivsize, ":cipher-ivsize");

  DEFSYM!(QCmac_algorithm_id, ":mac-algorithm-id");
  DEFSYM!(QCmac_algorithm_noncesize, ":mac-algorithm-noncesize");
  DEFSYM!(QCmac_algorithm_keysize, ":mac-algorithm-keysize");
  DEFSYM!(QCmac_algorithm_length, ":mac-algorithm-length");

  DEFSYM!(QCdigest_algorithm_id, ":digest-algorithm-id");
  DEFSYM!(QCdigest_algorithm_length, ":digest-algorithm-length");

    DEFSYM!(QCtype, ":type");
    DEFSYM!(Qgnutls_type_cipher, "gnutls-symmetric-cipher");
    DEFSYM!(Qgnutls_type_mac_algorithm, "gnutls-mac-algorithm");
    DEFSYM!(Qgnutls_type_digest_algorithm, "gnutls-digest-algorithm");
    DEFSYM!(Qgnutls_e_interrupted, "gnutls-e-interrupted");
    Fput(Qgnutls_e_interrupted(), Qgnutls_code(), make_fixnum(GNUTLS_E_INTERRUPTED as EMACS_INT));
    DEFSYM!(Qgnutls_e_again, "gnutls-e-again");
    Fput(Qgnutls_e_again(), Qgnutls_code(), make_fixnum(GNUTLS_E_AGAIN as EMACS_INT));
    DEFSYM!(Qgnutls_e_invalid_session, "gnutls-e-invalid-session");
    Fput(Qgnutls_e_invalid_session(), Qgnutls_code(), make_fixnum(GNUTLS_E_INVALID_SESSION as EMACS_INT));
    DEFSYM!(Qgnutls_e_not_ready_for_handshake, "gnutls-e-not-ready-for-handshake");
    Fput(Qgnutls_e_not_ready_for_handshake(), Qgnutls_code(), make_fixnum(GNUTLS_E_APPLICATION_ERROR_MIN as EMACS_INT));
    defsubr(&raw mut Sgnutls_get_initstage);
    defsubr(&raw mut Sgnutls_asynchronous_parameters);
    defsubr(&raw mut Sgnutls_errorp);
    defsubr(&raw mut Sgnutls_error_fatalp);
    defsubr(&raw mut Sgnutls_error_string);
    defsubr(&raw mut Sgnutls_boot);
    defsubr(&raw mut Sgnutls_deinit);
    defsubr(&raw mut Sgnutls_bye);
    defsubr(&raw mut Sgnutls_peer_status);
    defsubr(&raw mut Sgnutls_peer_status_warning_describe);
    defsubr(&raw mut Sgnutls_format_certificate);
    #[cfg(feature = "have_gnutls3")]
    {
      defsubr(&raw mut Sgnutls_ciphers);
      defsubr(&raw mut Sgnutls_macs);
      defsubr(&raw mut Sgnutls_digests);
      defsubr(&raw mut Sgnutls_hash_mac);
      defsubr(&raw mut Sgnutls_hash_digest);
      defsubr(&raw mut Sgnutls_symmetric_encrypt);
      defsubr(&raw mut Sgnutls_symmetric_decrypt);
      cipher_cache = Qnil();
      staticpro(&raw mut cipher_cache);
    }
    DEFVAR_INT!(c"gnutls-log-level", global_gnutls_log_level,
      "Logging level used by the GnuTLS functions.\nSet this larger than 0 to get debug output in the *Messages* buffer.\n1 is for important messages, 2 is for debug data, and higher numbers\nare as per the GnuTLS logging conventions.");
    global_gnutls_log_level = 0;
  }

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
}
