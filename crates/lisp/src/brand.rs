#![forbid(unsafe_code)]
//! Centralized naming and environment branding for the standalone `lisp` runtime.
//!
//! Keep runtime/cache/environment identifiers here so downstream adjustments stay localized.

pub const DEFAULT_PROGRAM_NAME: &str = "lisp";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const CACHE_APP_DIR: &str = "lisp";
pub const CACHE_FALLBACK_TEMP_DIR: &str = "lisp-cache";
pub const REPL_HISTORY_FILE: &str = "history.txt";

pub const ENV_CACHE_DIR: &str = "LISP_CACHE_DIR";
pub const ENV_VERBOSE: &str = "LISP_VERBOSE";
pub const ENV_NO_AUTO_WARM: &str = "LISP_NO_AUTO_WARM";
pub const ENV_CONCURRENCY: &str = "LISP_CONCURRENCY";
