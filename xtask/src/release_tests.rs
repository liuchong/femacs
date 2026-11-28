//! Unit tests for the unified 5-crate `xtask release` workflow.
#![forbid(unsafe_code)]

use super::*;
use std::collections::HashSet;

#[test]
fn publish_crates_cover_all_five_in_topological_order() {
    let names: Vec<&str> = PUBLISH_CRATES.iter().map(|c| c.name).collect();
    assert_eq!(
        names,
        vec!["lisp-vm", "lisp-std", "femacs", "lisp", "elisp"]
    );
    let mut seen = HashSet::new();
    for c in PUBLISH_CRATES {
        for &dep in c.internal_deps {
            assert!(
                seen.contains(dep),
                "{} depends on {} which must appear earlier",
                c.name,
                dep
            );
        }
        seen.insert(c.name);
    }
}

#[test]
fn semver_and_sparse_index_urls_match_crates_io_layout() {
    assert!(is_valid_semver("0.0.1"));
    assert!(is_valid_semver("0.1.0-alpha.1"));
    assert!(!is_valid_semver("v0.0.1"));
    assert!(!is_valid_semver("0.1"));
    assert!(!is_valid_semver("0.0.1-"));

    assert_eq!(sparse_index_url("a"), "https://index.crates.io/1/a");
    assert_eq!(sparse_index_url("ab"), "https://index.crates.io/2/ab");
    assert_eq!(sparse_index_url("abc"), "https://index.crates.io/3/a/abc");
    assert_eq!(sparse_index_url("lisp"), "https://index.crates.io/li/sp/lisp");
    assert_eq!(
        sparse_index_url("elisp"),
        "https://index.crates.io/el/is/elisp"
    );
    assert_eq!(
        sparse_index_url("femacs"),
        "https://index.crates.io/fe/ma/femacs"
    );
    assert_eq!(
        sparse_index_url("lisp-vm"),
        "https://index.crates.io/li/sp/lisp-vm"
    );
}

#[test]
fn bump_manifest_updates_package_and_sibling_pins_only() {
    let sample = r#"[package]
name = "lisp"
version = "0.0.1"
edition.workspace = true

[dependencies]
lisp-vm = { version = "0.0.1", path = "../lisp-vm", optional = true }
lisp-std = { version = "0.0.1", path = "../lisp-std", optional = true }
serde = { version = "1", features = ["derive"] }
toml = { version = "0.8", optional = true }
"#;
    let bumped = bump_manifest_text(sample, "0.0.2", &["lisp-vm", "lisp-std"]).expect("bump");
    assert_eq!(
        read_package_field(&bumped, "version").as_deref(),
        Some("0.0.2")
    );
    assert_eq!(
        read_dep_version(&bumped, "lisp-vm").as_deref(),
        Some("0.0.2")
    );
    assert_eq!(
        read_dep_version(&bumped, "lisp-std").as_deref(),
        Some("0.0.2")
    );
    assert!(bumped.contains(r#"serde = { version = "1", features = ["derive"] }"#));
    assert!(bumped.contains(r#"toml = { version = "0.8", optional = true }"#));
}

#[test]
fn release_notes_include_all_five_crates_and_windows_disclosure() {
    let notes = render_release_notes("0.0.1", &[String::from("femacs")]);
    for name in ["elisp", "lisp", "lisp-std", "lisp-vm", "femacs"] {
        assert!(notes.contains(name), "missing crate {name} in release notes");
    }
    assert!(notes.contains("published earlier; unified into `v0.0.1`"));
    assert!(notes.contains("Cross-compile architecture verified ONLY"));
    assert!(notes.contains("no physical Windows host runtime verification"));
    assert!(notes.contains("https://femacs.com/pages/lisp.html"));
}
