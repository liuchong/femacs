#![forbid(unsafe_code)]
//! Standalone general-purpose Elisp-compatible Lisp CLI binary entrypoint.

mod cli;
mod repl;

use std::env;
use std::io::{self, Write};
use std::process;

fn main() {
    let raw_args: Vec<_> = env::args_os().skip(1).collect();
    let code = cli::run_cli(&raw_args);
    let _ = io::stdout().flush();
    let _ = io::stderr().flush();
    process::exit(code);
}
