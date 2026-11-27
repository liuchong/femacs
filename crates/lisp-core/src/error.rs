#![forbid(unsafe_code)]
//! Error types returned by [`crate::Engine`] and runtime initialization.

use std::fmt;

#[cfg(feature = "vm")]
use lisp_vm::host_runtime::VmHostError;

/// Recoverable errors surfaced by `lisp-core` runtime initialization and evaluation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    AlreadyInitialized,
    InitFailed(String),
    Exit(i32),
    Syntax {
        symbol: String,
        message: String,
        data: String,
    },
    Runtime {
        symbol: String,
        message: String,
        data: String,
    },
    Interrupted {
        message: String,
    },
    Io {
        symbol: String,
        message: String,
        data: String,
    },
}

impl Error {
    /// Convert any Lisp/runtime failure into a structured JSON error payload for RPC/tooling.
    pub fn to_rpc_data(&self) -> serde_json::Value {
        match self {
            Self::AlreadyInitialized => serde_json::json!({ "kind": "already-initialized" }),
            Self::InitFailed(message) => {
                serde_json::json!({ "kind": "init-failed", "message": message })
            }
            Self::Exit(code) => serde_json::json!({ "kind": "exit-request", "exitCode": code }),
            Self::Syntax {
                symbol,
                message,
                data,
            } => serde_json::json!({
                "kind": "read-error",
                "symbol": symbol,
                "message": message,
                "data": data,
            }),
            Self::Runtime {
                symbol,
                message,
                data,
            } => serde_json::json!({
                "kind": "lisp-condition",
                "symbol": symbol,
                "message": message,
                "data": data,
            }),
            Self::Interrupted { message } => serde_json::json!({
                "kind": "quit",
                "symbol": "quit",
                "message": message,
            }),
            Self::Io {
                symbol,
                message,
                data,
            } => serde_json::json!({
                "kind": "io-error",
                "symbol": symbol,
                "message": message,
                "data": data,
            }),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyInitialized => f.write_str("Lisp VM runtime already initialized"),
            Self::InitFailed(msg) => write!(f, "Lisp VM init failed: {msg}"),
            Self::Exit(code) => write!(f, "Lisp exit requested with code {code}"),
            Self::Syntax {
                symbol,
                message,
                data,
            } => write!(f, "Lisp read error ({symbol}): {message} [{data}]"),
            Self::Runtime {
                symbol,
                message,
                data,
            } => write!(f, "Lisp condition ({symbol}): {message} [{data}]"),
            Self::Interrupted { message } => write!(f, "Lisp quit: {message}"),
            Self::Io {
                symbol,
                message,
                data,
            } => write!(f, "Lisp I/O error ({symbol}): {message} [{data}]"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io {
            symbol: "file-error".to_string(),
            message: err.to_string(),
            data: String::new(),
        }
    }
}

#[cfg(feature = "vm")]
impl From<VmHostError> for Error {
    fn from(err: VmHostError) -> Self {
        match err {
            VmHostError::AlreadyInitialized => Self::AlreadyInitialized,
            VmHostError::InitFailed(msg) => Self::InitFailed(msg),
            VmHostError::ExitRequest(code) => Self::Exit(code),
            VmHostError::Read {
                symbol,
                message,
                data,
            } => Self::Syntax {
                symbol,
                message,
                data,
            },
            VmHostError::LispCondition {
                symbol,
                message,
                data,
            } => Self::Runtime {
                symbol,
                message,
                data,
            },
            VmHostError::Quit { message } => Self::Interrupted { message },
            VmHostError::Io {
                symbol,
                message,
                data,
            } => Self::Io {
                symbol,
                message,
                data,
            },
        }
    }
}
