// Shared opaque library handles and public process initialization states.
// Pointer targets are library-owned; Rust never constructs or frees them.
// [U1-MODULE-MIRROR]
use super::*;
use core::ffi::{c_int, c_uint};

#[repr(C)]
pub struct gnutls_session_int { _private: [u8; 0] }
pub type gnutls_session_t = *mut gnutls_session_int;
#[repr(C)]
pub struct gnutls_x509_crt_int { _private: [u8; 0] }
pub type gnutls_x509_crt_t = *mut gnutls_x509_crt_int;
#[repr(C)]
pub struct gnutls_certificate_credentials_st { _private: [u8; 0] }
pub type gnutls_certificate_credentials_t = *mut gnutls_certificate_credentials_st;
pub type gnutls_certificate_client_credentials = gnutls_certificate_credentials_t;
#[repr(C)]
pub struct gnutls_anon_client_credentials_st { _private: [u8; 0] }
pub type gnutls_anon_client_credentials_t = *mut gnutls_anon_client_credentials_st;

pub type gnutls_initstage_t = c_uint;
pub const GNUTLS_STAGE_EMPTY: gnutls_initstage_t = 0;
pub const GNUTLS_STAGE_CRED_ALLOC: gnutls_initstage_t = 1;
pub const GNUTLS_STAGE_FILES: gnutls_initstage_t = 2;
pub const GNUTLS_STAGE_CALLBACKS: gnutls_initstage_t = 3;
pub const GNUTLS_STAGE_INIT: gnutls_initstage_t = 4;
pub const GNUTLS_STAGE_PRIORITY: gnutls_initstage_t = 5;
pub const GNUTLS_STAGE_CRED_SET: gnutls_initstage_t = 6;
pub const GNUTLS_STAGE_HANDSHAKE_CANDO: gnutls_initstage_t = GNUTLS_STAGE_CRED_SET;
pub const GNUTLS_STAGE_TRANSPORT_POINTERS_SET: gnutls_initstage_t = 7;
pub const GNUTLS_STAGE_HANDSHAKE_TRIED: gnutls_initstage_t = 8;
pub const GNUTLS_STAGE_READY: gnutls_initstage_t = 9;
pub const GNUTLS_EMACS_HANDSHAKES_LIMIT: c_int = 6000;
pub const GNUTLS_EMACS_ERROR_NOT_LOADED: c_int = -65500 + 1;
pub const GNUTLS_EMACS_ERROR_INVALID_TYPE: c_int = -65500;
