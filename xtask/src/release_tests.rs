//! Unit tests for the unified multi-channel `xtask release` workflow.
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
fn bump_npm_and_install_sh_preserve_structure() {
    let pkg = "{\n  \"name\": \"femacs\",\n  \"version\": \"0.0.1\"\n}\n";
    let bumped_pkg = bump_npm_package_json(pkg, "0.0.2").expect("npm bump");
    let parsed: Value = serde_json::from_str(&bumped_pkg).expect("valid json");
    assert_eq!(parsed.get("version").and_then(Value::as_str), Some("0.0.2"));

    let sh = "#!/bin/sh\nFEMACS_VERSION=\"${FEMACS_VERSION:-0.0.1}\"\necho ok\n";
    let bumped_sh = bump_install_sh(sh, "0.0.2").expect("sh bump");
    assert!(bumped_sh.contains("FEMACS_VERSION=\"${FEMACS_VERSION:-0.0.2}\""));
}

#[test]
fn parse_checksums_and_sync_dist_roundtrip() {
    let sums_raw = "\
2678a3a76bb4dfe96e4be9a86094d1223e6f09003b7dc54e95ba8bfe97311bca  femacs-v0.0.2-x86_64-unknown-linux-gnu.tar.gz\n\
b2d6965e261d13bc0030d5c8c923b8361888caf231b154c4cb897d141aa579f6  femacs-v0.0.2-aarch64-unknown-linux-gnu.tar.gz\n\
d341fdca2c411fc1583243ba965d3f360fb74ebdbf25860a5fd7fa843da0c6b0  femacs-v0.0.2-x86_64-apple-darwin.tar.gz\n\
f42a8fddfe5fbd81b90c5146c7824cdd27c7b4af41f8f389ce4103d214d37700  femacs-v0.0.2-aarch64-apple-darwin.tar.gz\n";

    let checksums = parse_checksums(sums_raw).expect("parse checksums");
    assert_eq!(checksums.len(), 4);

    let man_text = render_npm_manifest_json("0.0.2", &checksums).expect("manifest");
    let man: Value = serde_json::from_str(&man_text).expect("valid manifest json");
    assert_eq!(man.get("version").and_then(Value::as_str), Some("0.0.2"));
    assert_eq!(man.get("tag").and_then(Value::as_str), Some("v0.0.2"));
    let targets = man.get("targets").and_then(Value::as_object).expect("targets");
    assert_eq!(targets.len(), 4);
    assert_eq!(
        targets["darwin-x64"]["sha256"].as_str(),
        Some("d341fdca2c411fc1583243ba965d3f360fb74ebdbf25860a5fd7fa843da0c6b0")
    );

    let elisp_formula = render_homebrew_elisp_formula("0.0.2", &checksums).expect("elisp formula");
    assert!(elisp_formula.contains("class Elisp < Formula"));
    assert!(elisp_formula.contains("bin.install \"elisp\""));
    assert!(elisp_formula.contains("version \"0.0.2\""));
    assert!(elisp_formula.contains("femacs-v0.0.2-aarch64-apple-darwin.tar.gz"));

    let formula = render_homebrew_formula("0.0.2", &checksums).expect("formula");
    assert!(formula.contains("class Femacs < Formula"));
    assert!(formula.contains("bin.install \"femacs\", \"fe\""));
    assert!(formula.contains("version \"0.0.2\""));
    assert!(formula.contains("femacs-v0.0.2-aarch64-apple-darwin.tar.gz"));
    assert!(formula.contains("femacs-v0.0.2-x86_64-apple-darwin.tar.gz"));
    assert!(formula.contains("femacs-v0.0.2-aarch64-unknown-linux-gnu.tar.gz"));
    assert!(formula.contains("femacs-v0.0.2-x86_64-unknown-linux-gnu.tar.gz"));
}

#[test]
fn release_notes_include_all_channels_and_windows_disclosure() {
    let notes = render_release_notes("0.0.1", &[String::from("femacs")]);
    for name in [
        "elisp",
        "lisp",
        "lisp-std",
        "lisp-vm",
        "femacs",
        "brew install liuchong/tap/elisp",
        "brew install liuchong/tap/femacs",
        "npm install -g femacs",
        "https://femacs.com/install.sh",
    ] {
        assert!(notes.contains(name), "missing `{name}` in release notes");
    }
    assert!(notes.contains("published earlier; unified into `v0.0.1`"));
    assert!(notes.contains("Cross-compile architecture verified ONLY"));
    assert!(notes.contains("no physical Windows host runtime verification"));
    assert!(notes.contains("https://femacs.com/pages/lisp.html"));
}
