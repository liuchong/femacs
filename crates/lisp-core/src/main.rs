#![forbid(unsafe_code)]
//! Standalone general-purpose Lisp CLI binary entrypoint.

use std::env;
use std::io::{self, Write};
use std::process;

fn main() {
    let raw_args: Vec<_> = env::args_os().skip(1).collect();
    let code = lisp_core::cli::run_cli(&raw_args);
    let _ = io::stdout().flush();
    let _ = io::stderr().flush();
    process::exit(code);
}
