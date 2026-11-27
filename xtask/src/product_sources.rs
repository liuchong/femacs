#![forbid(unsafe_code)]
use std::collections::BTreeSet;
use std::path::{Component, Path};

/// The source providers selected by the maintained macOS link manifests.
/// Historical port ledgers are useful for the port checks, but are not product
/// inputs and must not decide which Rust sources the product compiles.
pub(super) struct ProductSources {
    pub(super) rust: BTreeSet<String>,
    pub(super) swift: BTreeSet<String>,
}

impl ProductSources {
    pub(super) fn resolve(
        repo: &Path,
        objects: &[String],
        archive: &[String],
        cocoa: bool,
    ) -> Result<Self, String> {
        let mut sources = Self {
            rust: BTreeSet::new(),
            swift: BTreeSet::new(),
        };

        for name in objects {
            let object = object_path(name)?;
            let ported_rust = Path::new("crates/lisp-vm/src/ported/src")
                .join(object)
                .with_extension("rs");
            let root_rust = Path::new("src").join(object).with_extension("rs");
            let rust_path = if repo.join(&ported_rust).is_file() {
                Some(ported_rust)
            } else if repo.join(&root_rust).is_file() {
                Some(root_rust)
            } else {
                None
            };
            let swift = Path::new("platform/macos").join(object).with_extension("swift");
            match (rust_path, cocoa && repo.join(&swift).is_file()) {
                (Some(rust), false) => {
                    insert_source(&mut sources.rust, &rust)?;
                }
                (None, true) => {
                    insert_source(&mut sources.swift, &swift)?;
                }
                (Some(_), true) => return Err(format!("{name}: both Rust and Swift provide the object")),
                (None, false) => return Err(format!("{name}: no product source provider")),
            }
        }

        for name in archive {
            let object = object_path(name)?;
            let ported_src = Path::new("crates/lisp-vm/src/ported/lib")
                .join(object)
                .with_extension("rs");
            let root_src = Path::new("lib").join(object).with_extension("rs");
            let source = if repo.join(&ported_src).is_file() {
                ported_src
            } else {
                root_src
            };
            insert_existing(repo, &mut sources.rust, &source)?;
            // This archive member and its implementation are separate inputs.
            // In the current manifests mini-gmp-gnulib provides the configuration
            // wrapper and mini-gmp provides the exported mpz functions.
            if let Some(stem) = source
                .file_stem()
                .and_then(|stem| stem.to_str())
                .and_then(|stem| stem.strip_suffix("-gnulib"))
            {
                let implementation = source.with_file_name(format!("{stem}.rs"));
                if repo.join(&implementation).is_file() {
                    insert_source(&mut sources.rust, &implementation)?;
                }
            }
        }
        Ok(sources)
    }
}

fn object_path(name: &str) -> Result<&Path, String> {
    let path = Path::new(name);
    if path.extension().and_then(|extension| extension.to_str()) != Some("o")
        || !path.components().all(|entry| matches!(entry, Component::Normal(_)))
    {
        return Err(format!("{name}: invalid product object name"));
    }
    Ok(path)
}

fn insert_existing(repo: &Path, sources: &mut BTreeSet<String>, path: &Path) -> Result<(), String> {
    if !repo.join(path).is_file() {
        return Err(format!("{}: missing product source", path.display()));
    }
    insert_source(sources, path)
}

fn insert_source(sources: &mut BTreeSet<String>, path: &Path) -> Result<(), String> {
    let source = path
        .to_str()
        .ok_or_else(|| format!("{}: non-UTF-8 product source", path.display()))?;
    sources.insert(source.to_owned());
    Ok(())
}
