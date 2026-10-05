// femacs: port of src/gnutls.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  This revision lands **C 30-1130**, the first slice of the 3,196-line target; everything
// from C 1131 to C 2964 is still owed, and `docs/ports/gnutls.md`'s `## C line ranges` table is the
// accounting that covers every reference line.
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
// (`femacs/src/include/gnutls.rs`), but it is still the P-08 stub: its own `#ifdef HAVE_GNUTLS`
// arm is not carried, so the handful of its names this slice needs (`GNUTLS_STAGE_*`, the
// `GNUTLS_LOG*` macros, `GNUTLS_INITSTAGE`) are declared at this use site under the same gate and
// are **owed back to that mirror** (docs/ports/gnutls.md's `## Owed`; the P-72 rule-4 shape
// `femacs/src/font.rs` 380-381 uses for `nconc2`).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

include!("include/config.rs");                  // C 19: `#include <config.h>`
/* C 20's `<errno.h>`: the frozen configuration has no `build/emacs-ref/lib/errno.h` (gnulib's
   `errno-h` module removes it; `femacs/lib/include/errno.in.rs` 3-20 records the measurement), so
   the header C sees is the *system* one, and the port's home for the frozen target's values is
   `errno.in.rs` -- the template mirror that carries both halves (`errno.rs`'s `#include_next` plus
   the frozen fallbacks ENOTCONN 57 / ECONNRESET 54 the gated text below needs).
   `femacs/exec/mipsfpu.rs` 27 includes it for the same C line.  */
include!("../lib/include/errno.in.rs");         // C 20: `#include <errno.h>`
include!("../lib/include/stdio.rs");            // C 21: `#include <stdio.h>`

/* C 23-28's six includes sit *outside* C 30's `#ifdef HAVE_GNUTLS` (P-86 ⑧'s "gate the include
   graph" describes the other shape, where the includes are inside the gate; here C's own text has
   them before it, and the live text below -- `Fgnutls_available_p` and `syms_of_gnutls` -- needs
   `lisp.h` and this file's own header whatever the gate says).  `process.h` is the one the
   *gated* text needs, so its mirror is included under the gate (the `atimer.rs` precedent this
   unit's previous revision recorded); `coding.h`, `buffer.h` and `pdumper.h` have no reader in
   C 30-1130 -- the text that reads them is C 1131-3196, still owed -- so they stay out until that
   text lands (P-88 ⑤'s single-declaration-point rule keeps them in their mirrors).  */
include!("include/lisp.rs");                    // C 23: `#include "lisp.h"`
include!("include/gnutls.rs");                  // C 25: `#include "gnutls.h"`

#[cfg (feature = "have_gnutls")]
include!("include/process.rs");                 // C 24: `#include "process.h"`

/* The system headers the gated body reaches through C's own includes: `<string.h>` (lisp.h 30;
   `strcpy`/`strlen`/`sprintf`), `<limits.h>` (lisp.h 32; `INT_MIN`/`INT_MAX`), `ssize_t`
   (`<sys/types.h>`/`<unistd.h>`), `<time.h>` (`time_t`, `nanosleep`) and C23 `<stdckdint.h>`
   (the `ckd_mul`/`ckd_add` pair at C 1089-1090, P-15).  */
#[cfg (feature = "have_gnutls")]
include!("../lib/include/string.rs");           // lisp.h 30: `<string.h>`
#[cfg (feature = "have_gnutls")]
include!("../lib/include/limits.in.rs");        // lisp.h 32: `<limits.h>`
#[cfg (feature = "have_gnutls")]
include!("../lib/include/unistd.in.rs");        // `ssize_t` (unistd.in.rs 34)
#[cfg (feature = "have_gnutls")]
include!("../lib/include/time.in.rs");          // `time_t`, `struct timespec`, `nanosleep`
#[cfg (feature = "have_gnutls")]
include!("../lib/include/stdckdint.in.rs");     // `ckd_mul`/`ckd_add` (the frozen arm is P-15)

/* ----------------------------------------------------------------------------------------------
   C 25: `#include "gnutls.h"` -> gnutls.h 21: `#include <gnutls/gnutls.h>`, gnutls.h 26:
   `#include <gnutls/x509.h>`, gnutls.h 29: `#include <gnutls/crypto.h>` -- the third-party
   surface the reference tree does not carry (P-88 ①: the declarations stand in the unit, not in
   `femacs/src/include/`, which mirrors the *reference* headers; `check-target-names.py` reads that
   directory by the reference header's name).  Types are `#[repr(C)]` aliases with the header's own
   line in `/// C:` (P-55), the enums are the `pub type = c_int` shape the port uses for every C
   enum (`femacs/src/coding.rs` 413), and the values are clang readings.  Every name the header
   declares only for a *later* slice (the rest of the file's API) is not declared here -- this
   segment carries exactly the surface C 30-1130 names.  */

/// C: <gnutls/gnutls.h> 1276: `typedef void *gnutls_transport_ptr_t;`
#[cfg (feature = "have_gnutls")]
pub type gnutls_transport_ptr_t = *mut c_void;
/// C: <gnutls/gnutls.h> 1278-1279: `struct gnutls_session_int;` /
/// `typedef struct gnutls_session_int *gnutls_session_t;` -- an incomplete type named only through
/// a pointer (PORTING.md 0.1-2's zero-sized stand-in).
#[cfg (feature = "have_gnutls")]
pub struct gnutls_session_int { _private: [u8; 0] }
/// C: <gnutls/gnutls.h> 1279: the pointer type itself.
#[cfg (feature = "have_gnutls")]
pub type gnutls_session_t = *mut gnutls_session_int;
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
pub type gnutls_server_name_type_t = c_int;
/// C: <gnutls/gnutls.h> 2413-2415 ... the `GNUTLS_X509_FMT_*` enum, <gnutls/gnutls.h> 856-859:
/// `typedef enum { GNUTLS_X509_FMT_DER = 0, GNUTLS_X509_FMT_PEM = 1 } gnutls_x509_crt_fmt_t;`
/// (values measured: DER 0, PEM 1).
#[cfg (feature = "have_gnutls")]
pub type gnutls_x509_crt_fmt_t = c_int;
/// C: <gnutls/gnutls.h> 791-794: `typedef enum { GNUTLS_SHUT_WR = 1, GNUTLS_SHUT_RDWR = 2 }
/// gnutls_close_request_t;` (sizeof 4).
#[cfg (feature = "have_gnutls")]
pub type gnutls_close_request_t = c_int;
/// C: <gnutls/gnutls.h> 614-648: `typedef enum { ... } gnutls_alert_description_t;` (sizeof 4).
#[cfg (feature = "have_gnutls")]
pub type gnutls_alert_description_t = c_int;
/// C: <gnutls/gnutls.h> 287-293: `typedef enum { ... } gnutls_credentials_type_t;` (sizeof 4).
#[cfg (feature = "have_gnutls")]
pub type gnutls_credentials_type_t = c_int;
/// C: <gnutls/gnutls.h> 159-219: `typedef enum gnutls_cipher_algorithm { ... } gnutls_cipher_algorithm_t;`
/// (sizeof 4).
#[cfg (feature = "have_gnutls")]
pub type gnutls_cipher_algorithm_t = c_int;
/// C: <gnutls/gnutls.h> 335-371: `typedef enum { ... } gnutls_mac_algorithm_t;` (sizeof 4).
#[cfg (feature = "have_gnutls")]
pub type gnutls_mac_algorithm_t = c_int;
/// C: <gnutls/gnutls.h> 398-421: `typedef enum { ... } gnutls_digest_algorithm_t;` (sizeof 4).
#[cfg (feature = "have_gnutls")]
pub type gnutls_digest_algorithm_t = c_int;
/// C: <gnutls/gnutls.h> 243-261: `typedef enum { ... } gnutls_kx_algorithm_t;` (sizeof 4).
#[cfg (feature = "have_gnutls")]
pub type gnutls_kx_algorithm_t = c_int;
/// C: <gnutls/gnutls.h> 813-828: `typedef enum { ... } gnutls_protocol_t;` (sizeof 4).
#[cfg (feature = "have_gnutls")]
pub type gnutls_protocol_t = c_int;
/// C: <gnutls/gnutls.h> 440-447: `typedef enum { ... } gnutls_compression_method_t;` (sizeof 4).
#[cfg (feature = "have_gnutls")]
pub type gnutls_compression_method_t = c_int;
/// C: <gnutls/gnutls.h> 841-847: `typedef enum { ... } gnutls_certificate_type_t;` (sizeof 4).
#[cfg (feature = "have_gnutls")]
pub type gnutls_certificate_type_t = c_int;
/// C: <gnutls/gnutls.h> 1002-1061: `typedef enum { ... } gnutls_sign_algorithm_t;` (sizeof 4).
#[cfg (feature = "have_gnutls")]
pub type gnutls_sign_algorithm_t = c_int;
/// C: <gnutls/gnutls.h> 1203-1216: `typedef enum { ... } gnutls_sec_param_t;` (sizeof 4).
#[cfg (feature = "have_gnutls")]
pub type gnutls_sec_param_t = c_int;
/// C: <gnutls/gnutls.h> 908-933: `typedef enum { ... } gnutls_pk_algorithm_t;` (sizeof 4).
#[cfg (feature = "have_gnutls")]
pub type gnutls_pk_algorithm_t = c_int;
/// C: <gnutls/gnutls.h> 871-877: `typedef enum gnutls_certificate_print_formats { ... }
/// gnutls_certificate_print_formats_t;` (sizeof 4).
#[cfg (feature = "have_gnutls")]
pub type gnutls_certificate_print_formats_t = c_int;
/// C: <gnutls/gnutls.h> 2040: `typedef struct gnutls_x509_crt_int *gnutls_x509_crt_t;`
#[cfg (feature = "have_gnutls")]
pub struct gnutls_x509_crt_int { _private: [u8; 0] }
/// C: <gnutls/gnutls.h> 2040: the pointer type itself.
#[cfg (feature = "have_gnutls")]
pub type gnutls_x509_crt_t = *mut gnutls_x509_crt_int;
/// C: <gnutls/gnutls.h> 2050-2052: `struct gnutls_certificate_credentials_st;` /
/// `typedef struct gnutls_certificate_credentials_st *gnutls_certificate_credentials_t;`
#[cfg (feature = "have_gnutls")]
pub struct gnutls_certificate_credentials_st { _private: [u8; 0] }
/// C: <gnutls/gnutls.h> 2050-2052: the pointer type itself.
#[cfg (feature = "have_gnutls")]
pub type gnutls_certificate_credentials_t = *mut gnutls_certificate_credentials_st;
/// C: <gnutls/gnutls.h> 2054: `typedef gnutls_certificate_credentials_t
/// gnutls_certificate_client_credentials;` -- the alias `src/process.h` 203 names its
/// `gnutls_x509_cred` member with.
#[cfg (feature = "have_gnutls")]
pub type gnutls_certificate_client_credentials = gnutls_certificate_credentials_t;
/// C: <gnutls/gnutls.h> 2057-2059: `typedef struct gnutls_anon_client_credentials_st
/// *gnutls_anon_client_credentials_t;`
#[cfg (feature = "have_gnutls")]
pub struct gnutls_anon_client_credentials_st { _private: [u8; 0] }
/// C: <gnutls/gnutls.h> 2057-2059: the pointer type itself.
#[cfg (feature = "have_gnutls")]
pub type gnutls_anon_client_credentials_t = *mut gnutls_anon_client_credentials_st;
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
/// size_t);` -- `ssize_t` is the port's `c_long` (`femacs/lib/include/unistd.in.rs` 34).
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
pub const GNUTLS_X509_FMT_PEM: gnutls_x509_crt_fmt_t = 1;
/// C: gnutls.h: `#define GNUTLS_E_APPLICATION_ERROR_MIN -65500` (`src/gnutls.h` 58-59 builds the
/// two `GNUTLS_EMACS_ERROR_*` codes on it; both are beyond this slice and stay owed).
#[cfg (feature = "have_gnutls")]
pub const GNUTLS_E_APPLICATION_ERROR_MIN: c_int = -65500;

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
   `femacs/src/process.rs` 1609, `femacs/src/image.rs` 203 and `femacs/src/font.rs` 381 declare
   them at theirs.  Their single declaration point is owed back to the mirrors
   (docs/ports/gnutls.md's `## Owed`).  */
#[cfg (feature = "have_gnutls")]
unsafe extern "C" {
  /// C: lisp.h 4389: `extern void message (const char *, ...) ATTRIBUTE_FORMAT_PRINTF (1, 2);`
  pub fn message (format: *const c_char, ...);
  /// C: lisp.h 4432: `extern AVOID memory_full (size_t);`
  pub fn memory_full (nbytes: size_t) -> !;
  /// C: lisp.h 4271: `extern Lisp_Object nconc2 (Lisp_Object, Lisp_Object);`
  pub fn nconc2 (a: Lisp_Object, b: Lisp_Object) -> Lisp_Object;
}

/* src/gnutls.h's own names this slice needs, declared at the use site because that header's
   mirror is still the P-08 stub (`femacs/src/include/gnutls.rs`; its `#ifdef HAVE_GNUTLS` arm is
   owed).  `/// C: gnutls.h <line>` cites the *reference* header, so `check-citations.py` checks
   them against `emacs/src/gnutls.h`.  */
/// C: gnutls.h 37-54: `typedef enum { GNUTLS_STAGE_EMPTY = 0, ... GNUTLS_STAGE_READY }
/// gnutls_initstage_t;` -- the port's C-enum shape (`pub type = c_int` plus the members).
#[cfg (feature = "have_gnutls")]
pub type gnutls_initstage_t = c_int;
/// C: gnutls.h 40: `GNUTLS_STAGE_EMPTY = 0,`
#[cfg (feature = "have_gnutls")]
pub const GNUTLS_STAGE_EMPTY: gnutls_initstage_t = 0;
/// C: gnutls.h 41: `GNUTLS_STAGE_CRED_ALLOC,`
#[cfg (feature = "have_gnutls")]
pub const GNUTLS_STAGE_CRED_ALLOC: gnutls_initstage_t = 1;
/// C: gnutls.h 42: `GNUTLS_STAGE_FILES,`
#[cfg (feature = "have_gnutls")]
pub const GNUTLS_STAGE_FILES: gnutls_initstage_t = 2;
/// C: gnutls.h 43: `GNUTLS_STAGE_CALLBACKS,`
#[cfg (feature = "have_gnutls")]
pub const GNUTLS_STAGE_CALLBACKS: gnutls_initstage_t = 3;
/// C: gnutls.h 44: `GNUTLS_STAGE_INIT,`
#[cfg (feature = "have_gnutls")]
pub const GNUTLS_STAGE_INIT: gnutls_initstage_t = 4;
/// C: gnutls.h 45: `GNUTLS_STAGE_PRIORITY,`
#[cfg (feature = "have_gnutls")]
pub const GNUTLS_STAGE_PRIORITY: gnutls_initstage_t = 5;
/// C: gnutls.h 46: `GNUTLS_STAGE_CRED_SET,`
#[cfg (feature = "have_gnutls")]
pub const GNUTLS_STAGE_CRED_SET: gnutls_initstage_t = 6;
/// C: gnutls.h 49: `GNUTLS_STAGE_HANDSHAKE_CANDO = GNUTLS_STAGE_CRED_SET,`
#[cfg (feature = "have_gnutls")]
pub const GNUTLS_STAGE_HANDSHAKE_CANDO: gnutls_initstage_t = GNUTLS_STAGE_CRED_SET;
/// C: gnutls.h 50: `GNUTLS_STAGE_TRANSPORT_POINTERS_SET,`
#[cfg (feature = "have_gnutls")]
pub const GNUTLS_STAGE_TRANSPORT_POINTERS_SET: gnutls_initstage_t = 7;
/// C: gnutls.h 51: `GNUTLS_STAGE_HANDSHAKE_TRIED,`
#[cfg (feature = "have_gnutls")]
pub const GNUTLS_STAGE_HANDSHAKE_TRIED: gnutls_initstage_t = 8;
/// C: gnutls.h 53: `GNUTLS_STAGE_READY`
#[cfg (feature = "have_gnutls")]
pub const GNUTLS_STAGE_READY: gnutls_initstage_t = 9;
/// C: gnutls.h 35: `#define GNUTLS_EMACS_HANDSHAKES_LIMIT 6000` (beyond this slice; carried here
/// with the rest of the header's own arm, owed back to the mirror).
#[cfg (feature = "have_gnutls")]
pub const GNUTLS_EMACS_HANDSHAKES_LIMIT: c_int = 6000;
/// C: gnutls.h 56: `#define GNUTLS_EMACS_ERROR_NOT_LOADED (GNUTLS_E_APPLICATION_ERROR_MIN + 1)`
#[cfg (feature = "have_gnutls")]
pub const GNUTLS_EMACS_ERROR_NOT_LOADED: c_int = GNUTLS_E_APPLICATION_ERROR_MIN + 1;
/// C: gnutls.h 57: `#define GNUTLS_EMACS_ERROR_INVALID_TYPE GNUTLS_E_APPLICATION_ERROR_MIN`
#[cfg (feature = "have_gnutls")]
pub const GNUTLS_EMACS_ERROR_INVALID_TYPE: c_int = GNUTLS_E_APPLICATION_ERROR_MIN;
/// C: gnutls.h 59: `#define GNUTLS_INITSTAGE(proc) (XPROCESS (proc)->gnutls_initstage)` -- C's
/// function-like macro, carried as the port's inline function (`include/process.rs` 125's
/// `CHECK_PROCESS` shape).  It reads `Lisp_Process.gnutls_initstage`, the field
/// `include/process.rs` does not carry yet (Owed).
#[cfg (feature = "have_gnutls")]
#[inline(always)]
pub unsafe fn GNUTLS_INITSTAGE (proc_: Lisp_Object) -> gnutls_initstage_t
{
  (*XPROCESS (proc_)).gnutls_initstage
}
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
   spelling `femacs/src/include/w32common.rs` 189-192 already uses for `hbfont_init_w32_funcs`).  */
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
include!("include/w32common.rs");               // C 84: `#  include "w32common.h"`
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
include!("include/w32.rs");                     // C 85: `#  include "w32.h"`

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
/// `femacs/src/include/w32common.rs` keeps `deferred` (P-22).  This unit is its third consumer
/// (`femacs/src/decompress.rs` and `femacs/src/lcms.rs` are the first two) and repeats the long
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
/// (`femacs/src/include/w32common.rs` 165-170).
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
DEF_DLL_FN! (c_int, gnutls_pk_bits_to_sec_param, W32_PFN_gnutls_pk_bits_to_sec_param,
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
/// `femacs/src/include/w32.rs` carries for `w32.h` 197, and `GetProcAddress` the one
/// `femacs/src/include/w32common.rs` calls.
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
   no Rust counterpart, so -- as `femacs/src/decompress.rs` 22-26 and `femacs/src/lcms.rs` 27-29
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
   plain name otherwise, the shape `femacs/src/decompress.rs` 373-392 records for the same class.  */

/* Report memory exhaustion if ERR is an out-of-memory indication.  */
/// C 594-599: `check_memory_full`.
#[cfg (feature = "have_gnutls")]
unsafe fn check_memory_full (err: c_int)
{
  /* When GnuTLS exhausts memory, it doesn't say how much memory it
     asked for, so tell the Emacs allocator that GnuTLS asked for no
     bytes.  This isn't accurate, but it's good enough.  */
  if err == GNUTLS_E_MEMORY_ERROR
    { memory_full (0); }
}

/* C 601: `# ifdef HAVE_GNUTLS3` -> `feature = "have_gnutls3"` (C 612 closes the arm).  */
/* Log a simple audit message.  */
/// C 603-611: `gnutls_audit_log_function`.
#[cfg (all (feature = "have_gnutls", feature = "have_gnutls3"))]
unsafe fn gnutls_audit_log_function (session: gnutls_session_t, string: *const c_char)
{
  if global_gnutls_log_level >= 1
    {
      message (c"gnutls.c: [audit] %s".as_ptr (), string);
    }
}

/* Log a simple message.  */
/// C 615-618: `gnutls_log_function`.
#[cfg (feature = "have_gnutls")]
unsafe fn gnutls_log_function (level: c_int, string: *const c_char)
{
  message (c"gnutls.c: [%d] %s".as_ptr (), level, string);
}

/* Log a message and a string.  */
/// C 622-626: `gnutls_log_function2`.
#[cfg (feature = "have_gnutls")]
unsafe fn gnutls_log_function2 (level: c_int, string: *const c_char, extra: *const c_char)
{
  message (c"gnutls.c: [%d] %s %s".as_ptr (), level, string, extra);
}

/// C 629-660: `gnutls_try_handshake`.  C's `while ((ret = gnutls_handshake (state)) < 0)` is the
/// loop with the assignment as its condition; `ret` needs a value before the first test
/// (PORTING.md 1.5).
#[cfg (feature = "have_gnutls")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gnutls_try_handshake (proc_: *mut Lisp_Process) -> c_int
{
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

  if (*proc_).gnutls_complete_negotiation_p
    { non_blocking = false; }

  if non_blocking
    { (*proc_).gnutls_p = true; }

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

/* C 669-692: `# ifndef WINDOWSNT` -> `not (feature = "windowsnt")`.  */
/// C 671-691: `emacs_gnutls_nonblock_errno`.  C's `case 0:` under C 677's `# ifdef _AIX` falls
/// through to the two cases below; `_AIX` is `undef` in the frozen table (macros.tsv 4036), so
/// the arm is the `aix` feature, a name already registered (feature-coverage.tsv 146).
#[cfg (all (feature = "have_gnutls", not (feature = "windowsnt")))]
unsafe extern "C" fn emacs_gnutls_nonblock_errno (ptr: gnutls_transport_ptr_t) -> c_int
{
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

/// C 694-726: `emacs_gnutls_handshake`.  C's `(gnutls_transport_ptr_t) proc` and
/// `(void *) (intptr_t) proc->infd` are the port's pointer casts, and C's `&emacs_gnutls_push`
/// (a function's address) is the `Option<fn>` constructor (the DEV_DLL_FN family's pointer
/// type).
#[cfg (feature = "have_gnutls")]
unsafe fn emacs_gnutls_handshake (proc_: *mut Lisp_Process) -> c_int
{
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

/// C 728-732: `emacs_gnutls_record_check_pending`.
#[cfg (feature = "have_gnutls")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_gnutls_record_check_pending (state: gnutls_session_t) -> ptrdiff_t
{
  #[cfg (not (feature = "windowsnt"))]
  { return gnutls_record_check_pending (state) as ptrdiff_t; }
  #[cfg (feature = "windowsnt")]
  { return (fn_gnutls_record_check_pending.unwrap ()) (state) as ptrdiff_t; }
}

/* C 734-741: `# ifdef WINDOWSNT` -> `feature = "windowsnt"`.  */
/// C 735-740: `emacs_gnutls_transport_set_errno`.
#[cfg (all (feature = "have_gnutls", feature = "windowsnt"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_gnutls_transport_set_errno (state: gnutls_session_t, err: c_int)
{
  (fn_gnutls_transport_set_errno.unwrap ()) (state, err);
}

/// C 743-770: `emacs_gnutls_write`.
#[cfg (feature = "have_gnutls")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_gnutls_write (proc_: *mut Lisp_Process, buf_: *const c_char,
					     nbyte: ptrdiff_t) -> ptrdiff_t
{
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

/// C 772-796: `emacs_gnutls_read`.
#[cfg (feature = "have_gnutls")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_gnutls_read (proc_: *mut Lisp_Process, buf_: *mut c_char,
					    nbyte: ptrdiff_t) -> ptrdiff_t
{
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

/// C 798-803: `emacs_gnutls_strerror`.
#[cfg (feature = "have_gnutls")]
unsafe fn emacs_gnutls_strerror (err: c_int) -> *const c_char
{
  #[cfg (not (feature = "windowsnt"))]
  let str_: *const c_char = gnutls_strerror (err);
  #[cfg (feature = "windowsnt")]
  let str_: *const c_char = (fn_gnutls_strerror.unwrap ()) (err);
  return if !str_.is_null () { str_ } else { c"unknown".as_ptr () };
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
      let alert: c_int = gnutls_alert_get (session);
      #[cfg (feature = "windowsnt")]
      let alert: c_int = (fn_gnutls_alert_get.unwrap ()) (session);
      let level: c_int = if err == GNUTLS_E_FATAL_ALERT_RECEIVED { 0 } else { 1 };
      #[cfg (not (feature = "windowsnt"))]
      { str_ = gnutls_alert_get_name (alert); }
      #[cfg (feature = "windowsnt")]
      { str_ = (fn_gnutls_alert_get_name.unwrap ()) (alert); }
      if str_.is_null ()
	{ str_ = c"unknown".as_ptr (); }

      GNUTLS_LOG2! (level, max_log_level, "Received alert: ", str_);
    }

  *errno () = errnum;
  return ret;
}

/* convert an integer error to a Lisp_Object; it will be either a
   known symbol like 'gnutls_e_interrupted' and 'gnutls_e_again' or
   simply the integer value of the error.  GNUTLS_E_SUCCESS is mapped
   to Qt.  */
/// C 906-923: `gnutls_make_error`.
#[cfg (feature = "have_gnutls")]
unsafe fn gnutls_make_error (err: c_int) -> Lisp_Object
{
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

/// C 925-936: `gnutls_deinit_certificates`.
#[cfg (feature = "have_gnutls")]
unsafe fn gnutls_deinit_certificates (p: *mut Lisp_Process)
{
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

/// C 938-977: `emacs_gnutls_deinit`.
#[cfg (feature = "have_gnutls")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_gnutls_deinit (proc_: Lisp_Object) -> Lisp_Object
{
  let mut log_level: c_int;

  CHECK_PROCESS (proc_);

  if !(*XPROCESS (proc_)).gnutls_p
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

  (*XPROCESS (proc_)).gnutls_p = false;
  return Qt ();
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
/// frozen `<stdckdint.h>` arm asks for (P-15, `femacs/src/casefiddle.rs` 315-320's shape): both
/// stores happen only when their operation did not overflow, in C's own left-to-right order.
#[cfg (feature = "have_gnutls")]
unsafe fn gnutls_hex_string (buf_: *mut c_uchar, buf_size: ptrdiff_t,
			     prefix: *const c_char) -> Lisp_Object
{
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

/// C 1104-1130: `emacs_gnutls_certificate_export_pem`.
#[cfg (feature = "have_gnutls")]
unsafe fn emacs_gnutls_certificate_export_pem (cert: gnutls_x509_crt_t) -> Lisp_Object
{
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
      USE_SAFE_ALLOCA! (sa_avail, sa_count);
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
      SAFE_FREE! (sa_count);
      return result;
    }
  else if err < GNUTLS_E_SUCCESS
    { error (c"GnuTLS certificate export error: %s".as_ptr (), emacs_gnutls_strerror (err)); }

  return Qnil ();
}

/* ----------------------------------------------------------------------------------------------
   C 1131-2964: **not ported**.  docs/ports/gnutls.md's range table records the region; the next
   definition in C order is C 1132's `emacs_gnutls_certificate_details`, and the next *live* item
   of the frozen configuration is C 2949's `Fgnutls_available_p` -- the text below.  C 2947's
   `#endif /* HAVE_GNUTLS3 */`, C 2965's `#ifdef HAVE_GNUTLS` and the arms inside the two live
   functions are the dead side this unit records in place.  */

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

  // C 2965-3013's `#ifdef HAVE_GNUTLS` region is dead text (docs/ports/gnutls.md).

  return capabilities;
});

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_gnutls ()
{
  DEFVAR_LISP! (c"libgnutls-version", Vlibgnutls_version,
		"The version of libgnutls that Emacs was compiled with.\nThe version number is encoded as an integer with the major version in\nthe ten thousands place, minor version in the hundreds, and patch\nlevel in the ones.  For builds without libgnutls, the value is -1.");
  // C 3027-3031's `#ifdef HAVE_GNUTLS` arm is dead; the live arm is `(-1)` (C 3032).
  Vlibgnutls_version = make_fixnum (-1);

  // C 3036-3142's `#ifdef HAVE_GNUTLS` region is dead text (docs/ports/gnutls.md records it; P-86 ①
  // says it is transcribed when this function's arms are taken up).

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
