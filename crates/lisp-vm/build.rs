#![forbid(unsafe_code)]
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn generate_default_config_rs() -> String {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_else(|_| "macos".to_string());
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_else(|_| "aarch64".to_string());
    let system_type = match target_os.as_str() {
        "macos" => "darwin",
        "linux" => "gnu/linux",
        other => other,
    };
    let target_triple = env::var("TARGET").unwrap_or_else(|_| format!("{target_arch}-unknown-{target_os}"));
    let has_sigsetjmp = target_os != "windows";
    let is_windows = target_os == "windows";
    format!(
        r#"// Auto-generated standalone VM configuration (neutral defaults).
pub const ALIGNOF_EMACS_INT: i64 = 8;
pub const EMACS_CONFIGURATION: &core::ffi::CStr = c"{target_triple}";
pub const EMACS_CONFIG_FEATURES: &core::ffi::CStr = c"PDUMPER";
pub const EMACS_CONFIG_OPTIONS: &core::ffi::CStr = c"--standalone-lisp-core";
pub const GC_SETJMP_WORKS: bool = true;
pub const HAIKU: bool = false;
pub const HAVE_C_ALIGNASOF: bool = true;
pub const HAVE_LOG2: bool = true;
pub const HAVE_MODULES: bool = false;
pub const HAVE_NATIVE_COMP: bool = false;
pub const HAVE_NS: bool = false;
pub const HAVE_PDUMPER: bool = true;
pub const HAVE_RINT: bool = true;
pub const HAVE_SIGNBIT: bool = true;
pub const HAVE_SIGSETJMP: bool = {has_sigsetjmp};
pub const HAVE_STATEMENT_EXPRESSIONS: bool = true;
pub const HAVE_STDBIT_H: bool = false;
pub const HAVE_STDCKDINT_H: bool = true;
pub const HAVE_STDCOUNTOF_H: bool = true;
pub const HAVE_STRUCT_ATTRIBUTE_ALIGNED: bool = true;
pub const HAVE_TREE_SITTER: bool = false;
pub const HAVE_TRUNC: bool = true;
pub const HAVE_TYPEOF: bool = true;
pub const HAVE_X_WINDOWS: bool = false;
pub const HAVE__SETJMP: bool = {not_win};
pub const HAVE___BUILTIN_UNWIND_INIT: bool = true;
pub const NATIVE_ELISP_SUFFIX: &core::ffi::CStr = c".eln";
pub const SYSTEM_TYPE: &core::ffi::CStr = c"{system_type}";
pub const USE_X_TOOLKIT: bool = false;
pub const _GL_CONFIG_H_INCLUDED: bool = true;
"#,
        not_win = !is_windows,
    )
}

fn generate_default_epaths_rs() -> String {
    r#"// Auto-generated neutral standalone paths (overridden at runtime via HostInitConfig).
#[inline(always)] pub fn PATH_LOADSEARCH() -> *const c_char { c"".as_ptr() }
#[inline(always)] pub fn PATH_DUMPLOADSEARCH() -> *const c_char { c"".as_ptr() }
#[inline(always)] pub fn PATH_DUMPLOADSEARCH_SLASH() -> *const c_char { c"".as_ptr() }
#[inline(always)] pub fn PATH_SITELOADSEARCH() -> *const c_char { c"".as_ptr() }
#[inline(always)] pub fn PATH_EXEC() -> *const c_char { c".".as_ptr() }
#[inline(always)] pub fn PATH_DATA() -> *const c_char { c".".as_ptr() }
#[inline(always)] pub fn PATH_DOC() -> *const c_char { c".".as_ptr() }
#[inline(always)] pub fn PATH_INFO() -> *const c_char { c".".as_ptr() }
#[inline(always)] pub fn PATH_GAME() -> *const c_char { core::ptr::null() }
#[inline(always)] pub fn PATH_BITMAPS() -> *const c_char { c".".as_ptr() }
"#
    .to_string()
}

fn resolve_override(manifest_dir: &Path, val: &str) -> PathBuf {
    let p = PathBuf::from(val);
    let abs = if p.is_relative() {
        let repo = manifest_dir.parent().and_then(Path::parent).unwrap_or(manifest_dir);
        repo.join(p)
    } else {
        p
    };
    abs.canonicalize().unwrap_or(abs)
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("cargo:rerun-if-env-changed=FEMACS_CONFIG_RS");
    let config_rs = if let Ok(val) = env::var("FEMACS_CONFIG_RS") {
        let resolved = resolve_override(&manifest_dir, &val);
        println!("cargo:rerun-if-changed={}", resolved.display());
        resolved
    } else {
        let p = out_dir.join("default_config.rs");
        fs::write(&p, generate_default_config_rs()).expect("write default_config.rs");
        p
    };
    println!("cargo:rustc-env=LISP_VM_CONFIG_RS={}", config_rs.display());

    println!("cargo:rerun-if-env-changed=FEMACS_EPATHS_RS");
    let epaths_rs = if let Ok(val) = env::var("FEMACS_EPATHS_RS") {
        let resolved = resolve_override(&manifest_dir, &val);
        println!("cargo:rerun-if-changed={}", resolved.display());
        resolved
    } else {
        let p = out_dir.join("default_epaths.rs");
        fs::write(&p, generate_default_epaths_rs()).expect("write default_epaths.rs");
        p
    };
    println!("cargo:rustc-env=LISP_VM_EPATHS_RS={}", epaths_rs.display());

    println!("cargo:rerun-if-env-changed=FEMACS_GLOBALS_RS");
    let globals_rs = if let Ok(val) = env::var("FEMACS_GLOBALS_RS") {
        let resolved = resolve_override(&manifest_dir, &val);
        println!("cargo:rerun-if-changed={}", resolved.display());
        resolved
    } else {
        let fallback = manifest_dir.join("src/ported/src/include/globals.rs");
        println!("cargo:rerun-if-changed={}", fallback.display());
        fallback
    };
    println!("cargo:rustc-env=FEMACS_GLOBALS_RS={}", globals_rs.display());

    for opt_key in ["FEMACS_SYMBOL_NAMES_RS", "FEMACS_GUI_RUNTIME", "FEMACS_GUI_EXPORTS"] {
        println!("cargo:rerun-if-env-changed={opt_key}");
        if let Ok(val) = env::var(opt_key) {
            let resolved = resolve_override(&manifest_dir, &val);
            println!("cargo:rerun-if-changed={}", resolved.display());
            println!("cargo:rustc-env={opt_key}={}", resolved.display());
        }
    }
}
