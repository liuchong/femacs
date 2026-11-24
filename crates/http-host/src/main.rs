#![forbid(unsafe_code)]
//! CLI entrypoint for `http-host` (`C2`).

use std::env;
use std::process;

use http_host::{CliOutcome, parse_http_host_args, serve};

#[tokio::main]
async fn main() {
    let raw_args: Vec<_> = env::args_os().skip(1).collect();
    let cfg = match parse_http_host_args(&raw_args) {
        Ok(c) => c,
        Err(outcome) => {
            match &outcome {
                CliOutcome::Info(msg) => println!("{msg}"),
                CliOutcome::Usage(msg) => eprintln!("http-host: {msg}"),
            }
            process::exit(outcome.code());
        }
    };

    match serve(cfg).await {
        Ok(code) => process::exit(code),
        Err(msg) => {
            eprintln!("http-host: {msg}");
            process::exit(1);
        }
    }
}
