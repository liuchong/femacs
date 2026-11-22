//! Independent regressions for manual-cache provenance.
#![forbid(unsafe_code)]
use super::recipes_match;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

static SERIAL: AtomicU64 = AtomicU64::new(0);

struct Scratch(PathBuf);
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
impl Scratch {
    fn path(&self) -> &Path {
        &self.0
    }
}

fn scratch() -> Scratch {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../build/manuals-review")
        .join(format!(
            "{}-{nanos}-{}",
            std::process::id(),
            SERIAL.fetch_add(1, Ordering::Relaxed)
        ));
    fs::create_dir_all(&path).unwrap();
    Scratch(path)
}

fn sha(path: &Path) -> String {
    let mut digest = Sha256::new();
    digest.update(fs::read(path).unwrap());
    format!("{:x}", digest.finalize())
}

fn fixture() -> (Scratch, Value) {
    let scratch = scratch();
    let tools = scratch.path().join("tools");
    fs::create_dir_all(&tools).unwrap();
    let builder = tools.join("build-manuals.py");
    let recipe = tools.join("version_resources.py");
    fs::write(&builder, b"owned builder").unwrap();
    fs::write(&recipe, b"owned recipe").unwrap();
    let metadata = json!({
        "builder_sha256": sha(&builder),
        "version_recipe_sha256": sha(&recipe)
    });
    (scratch, metadata)
}

#[test]
fn current_builder_and_version_recipe_are_accepted() {
    let _timer =
        crate::test_support::CaseTimer::new("current_builder_and_version_recipe_are_accepted");
    let (scratch, metadata) = fixture();
    assert!(recipes_match(scratch.path(), &metadata).unwrap());
}

#[test]
fn mutated_builder_or_version_recipe_is_rejected() {
    let _timer =
        crate::test_support::CaseTimer::new("mutated_builder_or_version_recipe_is_rejected");
    let (builder_scratch, builder_metadata) = fixture();
    fs::write(
        builder_scratch.path().join("tools/build-manuals.py"),
        b"changed builder",
    )
    .unwrap();
    assert!(!recipes_match(builder_scratch.path(), &builder_metadata).unwrap());

    let (recipe_scratch, recipe_metadata) = fixture();
    fs::write(
        recipe_scratch.path().join("tools/version_resources.py"),
        b"changed recipe",
    )
    .unwrap();
    assert!(!recipes_match(recipe_scratch.path(), &recipe_metadata).unwrap());
}

#[test]
fn missing_provenance_hash_is_not_accepted() {
    let _timer = crate::test_support::CaseTimer::new("missing_provenance_hash_is_not_accepted");
    let (scratch, metadata) = fixture();
    let mut missing_builder = metadata.clone();
    missing_builder
        .as_object_mut()
        .unwrap()
        .remove("builder_sha256");
    assert!(!recipes_match(scratch.path(), &missing_builder).unwrap());

    let mut missing_recipe = metadata;
    missing_recipe
        .as_object_mut()
        .unwrap()
        .remove("version_recipe_sha256");
    assert!(!recipes_match(scratch.path(), &missing_recipe).unwrap());
}

#[test]
fn missing_source_file_cannot_match() {
    let _timer = crate::test_support::CaseTimer::new("missing_source_file_cannot_match");
    let (scratch, metadata) = fixture();
    fs::remove_file(scratch.path().join("tools/version_resources.py")).unwrap();
    let result = recipes_match(scratch.path(), &metadata);
    assert!(matches!(result, Ok(false) | Err(_)));
}
