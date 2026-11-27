#![forbid(unsafe_code)]
//! 100% pure-Rust standard library batteries for standalone `lisp-core`.
//!
//! Covers HTTP/HTTPS client & server (`std/http`), embedded ACID KV & relational SQL
//! (`std/kv`, `std/db`), JSON/TOML/CSV/YAML/URL/Base64 (`std/*`), tar/zip/gzip (`std/archive`),
//! crypto/HMAC/UUID (`std/crypto`), time/timezones (`std/time`), filesystem/glob/path
//! (`std/fs`, `std/path`), and regex/subprocess/env (`std/regex`, `std/proc`, `std/env`).

pub mod archive;
pub mod crypto;
pub mod db;
pub mod formats;
pub mod fs;
pub mod http;
pub mod system;
pub mod time;
pub mod value;

use std::sync::Arc;
use std::time::Instant;

pub use archive::extract_archive_bytes;
pub use http::{download_url_bytes, install_pure_rust_tls_provider};
pub use value::StdValue;

/// Type-erased callback signature used to register `std/*` primitives on an embedding VM.
pub type StdHandler = Arc<dyn Fn(&[StdValue]) -> Result<StdValue, String> + Send + Sync + 'static>;

/// Shared stateful runtime for `lisp-std` (embedded HTTP servers, open `redb` handles, monotonic clock).
pub struct StdRuntime {
    http_servers: Arc<http::HttpServerRegistry>,
    dbs: Arc<db::DbRegistry>,
    epoch: Instant,
}

impl StdRuntime {
    /// Create a new pure-Rust standard library runtime state container.
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            http_servers: Arc::new(http::HttpServerRegistry::new()),
            dbs: Arc::new(db::DbRegistry::new()),
            epoch: Instant::now(),
        })
    }

    /// Stop all embedded HTTP servers owned by this runtime.
    pub fn shutdown(&self) {
        self.http_servers.shutdown_all();
    }
}

impl Drop for StdRuntime {
    fn drop(&mut self) {
        self.http_servers.shutdown_all();
    }
}

/// Register every `std/*` pure-Rust host primitive into `register`.
pub fn register_all<F>(runtime: &Arc<StdRuntime>, mut register: F)
where
    F: FnMut(&str, StdHandler),
{
    http::register(&runtime.http_servers, &mut register);
    db::register(&runtime.dbs, &mut register);
    formats::register(&mut register);
    archive::register(&mut register);
    crypto::register(&mut register);
    time::register(runtime.epoch, &mut register);
    fs::register(&mut register);
    system::register(&mut register);
}
