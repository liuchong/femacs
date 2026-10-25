//! Independent B-phase contracts for the generic transactional directory publisher.
//!
//! The existing seven app-publication tests in `bundle.rs` remain authoritative for the
//! compatibility wrapper.  These cases bind the new generic directory interface, including
//! explicit previous-set retention and failure-before-swap preservation.

use super::*;

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

fn repo() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask lives under the repository root")
        .to_path_buf()
}

fn tag() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock before epoch")
        .as_nanos();
    format!("{}-{nanos}", std::process::id())
}

struct Fixture(PathBuf);

impl Fixture {
    fn new(name: &str) -> Self {
        let path = repo()
            .join("build/bundle-tests")
            .join(format!("{name}-{}", tag()));
        fs::create_dir_all(&path).expect("bundle fixture");
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }

    fn destination(&self) -> PathBuf {
        self.path().join("info")
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn write(stage: &Path, name: &str, value: &[u8]) -> Result<(), String> {
    let destination = stage.join(name);
    fs::create_dir_all(destination.parent().ok_or("fixture has no parent")?)
        .map_err(|e| e.to_string())?;
    fs::write(destination, value).map_err(|e| e.to_string())
}

fn entries(path: &Path) -> usize {
    fs::read_dir(path).map(|it| it.count()).unwrap_or(0)
}

#[test]
fn first_publication_creates_generic_directory_and_no_previous() {
    let _case_time = crate::test_support::CaseTimer::new(
        "bundle_tests.rs::first_publication_creates_generic_directory_and_no_previous",
    );
    let fx = Fixture::new("first-directory");
    let published = publish_directory(
        &fx.destination(),
        None,
        |stage| write(stage, "manual.info", b"new"),
        |stage| {
            if stage.join("manual.info").is_file() {
                Ok(())
            } else {
                Err("missing info".into())
            }
        },
    )
    .expect("first publication");
    assert_eq!(published.destination, fx.destination());
    assert!(published.previous.is_none());
    assert_eq!(
        fs::read(fx.destination().join("manual.info")).expect("new file"),
        b"new"
    );
    assert_eq!(
        entries(fx.path()),
        1,
        "only the published directory remains"
    );
}

#[test]
fn replacement_can_retain_the_actual_previous_set() {
    let _case_time = crate::test_support::CaseTimer::new(
        "bundle_tests.rs::replacement_can_retain_the_actual_previous_set",
    );
    let fx = Fixture::new("keep-previous");
    let destination = fx.destination();
    fs::create_dir_all(&destination).expect("old set");
    fs::write(destination.join("old.info"), b"old").expect("old info");
    let previous = fx.path().join("info.previous");

    let published = publish_directory(
        &destination,
        Some(&previous),
        |stage| write(stage, "new.info", b"new"),
        |stage| {
            if stage.join("new.info").is_file() && !stage.join("old.info").exists() {
                Ok(())
            } else {
                Err("replacement set is not clean".into())
            }
        },
    )
    .expect("replacement");

    assert_eq!(published.previous.as_deref(), Some(previous.as_path()));
    assert_eq!(
        fs::read(previous.join("old.info")).expect("old preserved"),
        b"old"
    );
    assert!(!previous.join("new.info").exists());
    assert_eq!(
        fs::read(destination.join("new.info")).expect("new published"),
        b"new"
    );
    assert!(!destination.join("old.info").exists());
    assert_eq!(
        entries(fx.path()),
        2,
        "destination and retained previous only"
    );
}

#[test]
fn replacement_without_previous_removes_the_old_set_after_swap() {
    let _case_time = crate::test_support::CaseTimer::new(
        "bundle_tests.rs::replacement_without_previous_removes_the_old_set_after_swap",
    );
    let fx = Fixture::new("drop-previous");
    let destination = fx.destination();
    fs::create_dir_all(&destination).expect("old set");
    fs::write(destination.join("old.info"), b"old").expect("old");
    let published = publish_directory(
        &destination,
        None,
        |stage| write(stage, "new.info", b"new"),
        |_| Ok(()),
    )
    .expect("replacement");
    assert!(published.previous.is_none());
    assert!(destination.join("new.info").is_file());
    assert!(!destination.join("old.info").exists());
    assert_eq!(entries(fx.path()), 1);
}

#[test]
fn population_failure_leaves_previous_set_and_no_stage() {
    let _case_time = crate::test_support::CaseTimer::new(
        "bundle_tests.rs::population_failure_leaves_previous_set_and_no_stage",
    );
    let fx = Fixture::new("populate-failure");
    let destination = fx.destination();
    fs::create_dir_all(&destination).expect("old set");
    fs::write(destination.join("old.info"), b"old").expect("old");
    let result = publish_directory(
        &destination,
        Some(&fx.path().join("info.previous")),
        |stage| {
            write(stage, "partial.info", b"partial")?;
            Err("population failed".into())
        },
        |_| panic!("verify must not run after population failure"),
    );
    assert!(result.is_err());
    assert_eq!(
        fs::read(destination.join("old.info")).expect("old remains"),
        b"old"
    );
    assert!(!fx.path().join("info.previous").exists());
    assert_eq!(entries(fx.path()), 1, "stage removed, no previous promoted");
}

#[test]
fn verification_failure_leaves_previous_set_and_no_stage() {
    let _case_time = crate::test_support::CaseTimer::new(
        "bundle_tests.rs::verification_failure_leaves_previous_set_and_no_stage",
    );
    let fx = Fixture::new("verify-failure");
    let destination = fx.destination();
    fs::create_dir_all(&destination).expect("old set");
    fs::write(destination.join("old.info"), b"old").expect("old");
    let result = publish_directory(
        &destination,
        Some(&fx.path().join("info.previous")),
        |stage| write(stage, "new.info", b"new"),
        |_| Err("verification failed".into()),
    );
    assert!(result.is_err());
    assert_eq!(
        fs::read(destination.join("old.info")).expect("old remains"),
        b"old"
    );
    assert!(!fx.path().join("info.previous").exists());
    assert_eq!(entries(fx.path()), 1);
}

#[test]
fn failed_first_publication_leaves_no_destination_or_stage() {
    let _case_time = crate::test_support::CaseTimer::new(
        "bundle_tests.rs::failed_first_publication_leaves_no_destination_or_stage",
    );
    let fx = Fixture::new("first-failure");
    let result = publish_directory(
        &fx.destination(),
        None,
        |stage| write(stage, "partial.info", b"partial"),
        |_| Err("verify failed".into()),
    );
    assert!(result.is_err());
    assert!(!fx.destination().exists());
    assert_eq!(entries(fx.path()), 0);
}

#[test]
fn destination_symlink_and_file_are_rejected_before_population() {
    let _case_time = crate::test_support::CaseTimer::new(
        "bundle_tests.rs::destination_symlink_and_file_are_rejected_before_population",
    );
    let fx = Fixture::new("destination-boundary");
    let outside = fx.path().join("outside");
    fs::create_dir_all(&outside).expect("outside");
    fs::write(outside.join("sentinel"), b"untouched").expect("sentinel");

    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        let link = fx.path().join("linked-info");
        symlink(&outside, &link).expect("destination symlink");
        let result =
            publish_directory(&link, None, |_| panic!("populate must not run"), |_| Ok(()));
        assert!(result.is_err());
        assert_eq!(
            fs::read(outside.join("sentinel")).expect("outside"),
            b"untouched"
        );
    }

    let file = fx.path().join("plain-file");
    fs::write(&file, b"untouched").expect("file");
    assert!(
        publish_directory(&file, None, |_| panic!("populate must not run"), |_| Ok(())).is_err()
    );
    assert_eq!(fs::read(&file).expect("file untouched"), b"untouched");
}

#[test]
fn existing_backup_destination_is_rejected_before_population() {
    let _case_time = crate::test_support::CaseTimer::new(
        "bundle_tests.rs::existing_backup_destination_is_rejected_before_population",
    );
    let fx = Fixture::new("backup-boundary");
    let destination = fx.destination();
    fs::create_dir_all(&destination).expect("old set");
    fs::write(destination.join("old.info"), b"old").expect("old");
    let previous = fx.path().join("info.previous");
    fs::create_dir_all(&previous).expect("occupied backup");
    fs::write(previous.join("sentinel"), b"backup").expect("backup sentinel");

    let result = publish_directory(
        &destination,
        Some(&previous),
        |_| panic!("populate must not run when the backup is occupied"),
        |_| Ok(()),
    );
    assert!(result.is_err());
    assert_eq!(
        fs::read(previous.join("sentinel")).expect("backup intact"),
        b"backup"
    );
    assert_eq!(
        fs::read(destination.join("old.info")).expect("destination intact"),
        b"old"
    );
    assert_eq!(entries(fx.path()), 2);
}

#[test]
fn app_compatibility_wrapper_still_publishes() {
    let _case_time = crate::test_support::CaseTimer::new(
        "bundle_tests.rs::app_compatibility_wrapper_still_publishes",
    );
    let fx = Fixture::new("app-wrapper");
    let app = fx.path().join("Femacs.app");
    publish(
        &app,
        |stage| write(stage, "Contents/Resources/new", b"app"),
        |stage| {
            if stage.join("Contents/Resources/new").is_file() {
                Ok(())
            } else {
                Err("missing".into())
            }
        },
    )
    .expect("app wrapper");
    assert_eq!(
        fs::read(app.join("Contents/Resources/new")).expect("app file"),
        b"app"
    );
    assert_eq!(entries(fx.path()), 1);
}
