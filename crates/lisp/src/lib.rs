#![deny(static_mut_refs, unsafe_op_in_unsafe_fn)]
//! Standalone pure-Rust, No-GIL, Elisp-compatible embeddable Lisp runtime.

pub mod brand;
#[cfg(feature = "vm")]
pub mod concurrency;
#[cfg(feature = "vm")]
pub mod engine;
pub mod error;
pub mod exec;
mod os_ffi;
#[cfg(feature = "pkg")]
pub mod pkg;
pub mod rpc;
pub mod stdlib_cache;
pub mod value;

#[cfg(feature = "vm")]
pub use engine::{BootstrapMode, ConcurrencyMode, Engine, EngineConfig};
pub use error::Error;
pub use value::Value;

#[cfg(feature = "vm")]
pub use lisp_vm as vm;
#[cfg(feature = "std")]
pub use lisp_std as stdlib;

/// Convenience imports for Rust embedders.
pub mod prelude {
    #[cfg(feature = "vm")]
    pub use crate::engine::{BootstrapMode, ConcurrencyMode, Engine, EngineConfig};
    pub use crate::error::Error;
    pub use crate::value::Value;
}
