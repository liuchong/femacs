#![deny(static_mut_refs, unsafe_op_in_unsafe_fn)]
//! Standalone general-purpose Lisp programming language runtime, embedding engine, and CLI.

pub mod brand;
#[cfg(feature = "vm")]
pub mod cli;
#[cfg(feature = "vm")]
pub mod engine;
pub mod error;
pub mod exec;
mod os_ffi;
#[cfg(feature = "vm")]
pub mod repl;
pub mod rpc;
pub mod stdlib_cache;
pub mod value;

#[cfg(feature = "vm")]
pub use engine::{BootstrapMode, Engine, EngineConfig};
pub use error::Error;
pub use value::Value;
