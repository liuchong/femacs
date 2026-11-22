#![forbid(unsafe_code)]
//! Exclusive staging, verified atomic directory publication and owned backups.
use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

struct Stage {
    path: PathBuf,
    cleanup: bool,
}
impl Drop for Stage {
    fn drop(&mut self) {
        if self.cleanup {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}

#[derive(Debug)]
pub(crate) struct Published {
    pub(crate) destination: PathBuf,
    pub(crate) previous: Option<PathBuf>,
}

fn publication_path(path: &Path) -> Result<PathBuf, String> {
    let repo = fs::canonicalize(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .ok_or("missing repository")?,
    )
    .map_err(|e| e.to_string())?;
    let actual = crate::tooling::resolved_output(path)?;
    let info = repo.join("info");
    let info_adjacent = actual.parent() == Some(repo.as_path())
        && actual
            .file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.starts_with(".info."))
            .unwrap_or(false);
    if actual != info
        && !info_adjacent
        && ![repo.join("build"), repo.join("dist")]
            .iter()
            .any(|root| actual.starts_with(root) && actual != *root)
    {
        return Err("publication must stay in owned build/dist or the generated Info set".into());
    }
    Ok(actual)
}

/// App callers retain the existing contract; generic data callers can keep an old set.
pub(super) fn publish(
    destination: &Path,
    populate: impl FnOnce(&Path) -> Result<(), String>,
    verify: impl FnOnce(&Path) -> Result<(), String>,
) -> Result<(), String> {
    publish_directory(destination, None, populate, verify).map(|_| ())
}

pub(crate) fn publish_directory(
    destination: &Path,
    keep_previous: Option<&Path>,
    populate: impl FnOnce(&Path) -> Result<(), String>,
    verify: impl FnOnce(&Path) -> Result<(), String>,
) -> Result<Published, String> {
    // Reject a symlink itself before canonicalization can hide its identity.
    match fs::symlink_metadata(destination) {
        Ok(m) if m.file_type().is_symlink() || !m.is_dir() => {
            return Err(
                "publication destination must be an owned directory, not a symlink/file".into(),
            );
        }
        Ok(_) => (),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
        Err(e) => return Err(e.to_string()),
    }
    let destination = publication_path(destination)?;
    let parent = destination.parent().ok_or("publication has no parent")?;
    let previous = if let Some(path) = keep_previous {
        if fs::symlink_metadata(path).is_ok() {
            return Err("previous-set destination already exists".into());
        }
        let path = publication_path(path)?;
        if path == destination || path.parent() != Some(parent) {
            return Err("previous set must be a fresh adjacent owned path".into());
        }
        Some(path)
    } else {
        None
    };
    fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    let name = destination
        .file_name()
        .ok_or("publication has no name")?
        .to_string_lossy();
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_nanos();
    let suffix = if destination.extension().and_then(|n| n.to_str()) == Some("app") {
        ".app"
    } else {
        ""
    };
    let stage_path = parent.join(format!(
        ".{name}.stage-{}-{nonce}{suffix}",
        std::process::id()
    ));
    fs::create_dir(&stage_path).map_err(|e| e.to_string())?;
    let mut stage = Stage {
        path: stage_path,
        cleanup: true,
    };
    populate(&stage.path)?;
    verify(&stage.path)?;
    let mut retained = None;
    if destination.exists() {
        exchange(&stage.path, &destination)?;
        if let Some(path) = previous {
            if let Err(error) = fs::rename(&stage.path, &path) {
                stage.cleanup = false;
                return Err(format!(
                    "new directory published; previous set retained at {} because backup failed: {error}",
                    stage.path.display()
                ));
            }
            retained = Some(path);
        } else if let Err(error) = fs::remove_dir_all(&stage.path) {
            stage.cleanup = false;
            return Err(format!(
                "new directory published; old set remains at {} because cleanup failed: {error}",
                stage.path.display()
            ));
        }
    } else {
        fs::rename(&stage.path, &destination).map_err(|e| e.to_string())?;
    }
    Ok(Published {
        destination,
        previous: retained,
    })
}

fn exchange(left: &Path, right: &Path) -> Result<(), String> {
    crate::os_ffi::exchange_dirs(left, right)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path::PathBuf, time::Instant};
    static SERIAL: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    struct Fixture(PathBuf);
    impl Fixture {
        fn new() -> Self {
            let p = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("build/bundle-tests")
                .join(format!(
                    "{}-{}",
                    std::process::id(),
                    SERIAL.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
                ));
            fs::create_dir_all(&p).unwrap();
            Self(p)
        }
        fn app(&self) -> PathBuf {
            self.0.join("Femacs.app")
        }
        fn stage(&self, p: &std::path::Path) -> Result<(), String> {
            fs::create_dir_all(p.join("Contents/Resources")).map_err(|e| e.to_string())?;
            fs::write(p.join("Contents/Resources/new"), b"new").map_err(|e| e.to_string())
        }
        fn leftovers(&self) -> usize {
            fs::read_dir(&self.0).unwrap().count()
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    fn timed(name: &str, run: impl FnOnce()) {
        let t = Instant::now();
        run();
        println!("CASE {name} seconds={:.6}", t.elapsed().as_secs_f64());
    }
    #[test]
    fn first_publication() {
        timed("first_publication", || {
            let f = Fixture::new();
            publish(
                &f.app(),
                |p| f.stage(p),
                |p| {
                    if p.join("Contents/Resources/new").is_file() {
                        Ok(())
                    } else {
                        Err("missing".into())
                    }
                },
            )
            .unwrap();
            assert_eq!(
                fs::read(f.app().join("Contents/Resources/new")).unwrap(),
                b"new"
            );
            assert_eq!(f.leftovers(), 1);
        });
    }
    #[test]
    fn replacement_removes_obsolete_resource() {
        timed("replacement_removes_obsolete_resource", || {
            let f = Fixture::new();
            fs::create_dir_all(f.app()).unwrap();
            fs::write(f.app().join("obsolete"), b"old").unwrap();
            publish(&f.app(), |p| f.stage(p), |_| Ok(())).unwrap();
            assert!(!f.app().join("obsolete").exists());
            assert!(f.app().join("Contents/Resources/new").is_file());
            assert_eq!(f.leftovers(), 1);
        });
    }
    #[test]
    fn population_failure_preserves_old() {
        timed("population_failure_preserves_old", || {
            let f = Fixture::new();
            fs::create_dir_all(f.app()).unwrap();
            fs::write(f.app().join("old"), b"old").unwrap();
            assert!(
                publish(
                    &f.app(),
                    |p| {
                        f.stage(p)?;
                        Err("populate failed".into())
                    },
                    |_| panic!("verify must not run")
                )
                .is_err()
            );
            assert_eq!(fs::read(f.app().join("old")).unwrap(), b"old");
            assert_eq!(f.leftovers(), 1);
        });
    }
    #[test]
    fn verification_failure_preserves_old() {
        timed("verification_failure_preserves_old", || {
            let f = Fixture::new();
            fs::create_dir_all(f.app()).unwrap();
            fs::write(f.app().join("old"), b"old").unwrap();
            assert!(
                publish(
                    &f.app(),
                    |p| f.stage(p),
                    |_| Err("sign/verify failed".into())
                )
                .is_err()
            );
            assert_eq!(fs::read(f.app().join("old")).unwrap(), b"old");
            assert_eq!(f.leftovers(), 1);
        });
    }
    #[test]
    fn failed_first_publication_leaves_no_app() {
        timed("failed_first_publication_leaves_no_app", || {
            let f = Fixture::new();
            assert!(publish(&f.app(), |p| f.stage(p), |_| Err("verify failed".into())).is_err());
            assert!(!f.app().exists());
            assert_eq!(f.leftovers(), 0);
        });
    }
    #[cfg(unix)]
    #[test]
    fn symlink_target_is_never_followed() {
        timed("symlink_target_is_never_followed", || {
            let f = Fixture::new();
            let outside = f.0.join("outside");
            fs::create_dir(&outside).unwrap();
            fs::write(outside.join("old"), b"untouched").unwrap();
            std::os::unix::fs::symlink(&outside, f.app()).unwrap();
            assert!(publish(&f.app(), |_| panic!("populate must not run"), |_| Ok(())).is_err());
            assert_eq!(fs::read(outside.join("old")).unwrap(), b"untouched");
        });
    }
    #[test]
    fn file_target_rejected() {
        timed("file_target_rejected", || {
            let f = Fixture::new();
            fs::write(f.app(), b"untouched").unwrap();
            assert!(publish(&f.app(), |_| panic!("populate must not run"), |_| Ok(())).is_err());
            assert_eq!(fs::read(f.app()).unwrap(), b"untouched");
        });
    }
}

#[cfg(test)]
#[path = "bundle_tests.rs"]
mod generic_tests;
