#![forbid(unsafe_code)]
use std::env;
use std::path::PathBuf;

fn main() {
    let repo = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("../..");
    for (key, fallback) in [
        ("FEMACS_CONFIG_RS", "src/include/config.rs"),
        ("FEMACS_EPATHS_RS", "src/include/epaths.rs"),
        ("FEMACS_GLOBALS_RS", "src/include/globals.rs"),
    ] {
        println!("cargo:rerun-if-env-changed={key}");
        let p = env::var(key)
            .map(PathBuf::from)
            .map(|p| if p.is_relative() { repo.join(p) } else { p })
            .unwrap_or_else(|_| repo.join(fallback));
        let p = p.canonicalize().unwrap_or(p);
        println!("cargo:rerun-if-changed={}", p.display());
        println!("cargo:rustc-env={key}={}", p.display());
    }
    for opt_key in ["FEMACS_SYMBOL_NAMES_RS", "FEMACS_GUI_RUNTIME", "FEMACS_GUI_EXPORTS"] {
        println!("cargo:rerun-if-env-changed={opt_key}");
        if let Ok(val) = env::var(opt_key) {
            let p = PathBuf::from(&val);
            let p = if p.is_relative() { repo.join(p) } else { p };
            let p = p.canonicalize().unwrap_or(p);
            println!("cargo:rerun-if-changed={}", p.display());
            println!("cargo:rustc-env={opt_key}={}", p.display());
        }
    }
}
