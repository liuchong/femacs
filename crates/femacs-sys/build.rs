#![forbid(unsafe_code)]
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-env-changed=FEMACS_CONFIG_RS");
    let repo = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("../..");
    let cfg = env::var("FEMACS_CONFIG_RS")
        .map(PathBuf::from)
        .map(|p| if p.is_relative() { repo.join(p) } else { p })
        .unwrap_or_else(|_| repo.join("src/include/config.rs"));
    let cfg = cfg.canonicalize().unwrap_or(cfg);
    println!("cargo:rerun-if-changed={}", cfg.display());
    println!("cargo:rustc-env=FEMACS_CONFIG_RS={}", cfg.display());
}
