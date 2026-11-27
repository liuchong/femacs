#![forbid(unsafe_code)]
//! Centralized naming and environment branding for the standalone Lisp runtime.
//!
//! Keep crate/binary/CLI/environment identifiers here so pre-publish renaming touches one module.

pub const CLI_NAME: &str = "lisp-core";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const CACHE_APP_DIR: &str = "lisp-core";
pub const CACHE_FALLBACK_TEMP_DIR: &str = "lisp-core-cache";
pub const REPL_HISTORY_FILE: &str = "history.txt";

pub const REPL_PROMPT: &str = "lisp> ";
pub const REPL_CONTINUATION_PROMPT: &str = "... > ";

pub const ENV_CACHE_DIR: &str = "LISP_CORE_CACHE_DIR";
pub const ENV_VERBOSE: &str = "LISP_CORE_VERBOSE";
pub const ENV_NO_AUTO_WARM: &str = "LISP_CORE_NO_AUTO_WARM";
