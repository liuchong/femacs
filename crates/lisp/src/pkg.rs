#![forbid(unsafe_code)]
//! Minimal project scaffolding (`init`) and deterministic dependency manager (`pkg`).
//!
//! Reads/writes `lisp.toml` manifests and atomic `lisp.lock` lockfiles, vendors local `--path`
//! and remote `--url` dependencies into `.lisp-packages/`, and injects project + package source
//! directories into `load-path`.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::error::Error;

pub const MANIFEST_FILE: &str = "lisp.toml";
pub const LOCK_FILE: &str = "lisp.lock";
pub const PACKAGES_DIR: &str = ".lisp-packages";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Manifest {
    pub package: PackageSection,
    #[serde(default)]
    pub dependencies: BTreeMap<String, DependencySpec>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageSection {
    pub name: String,
    pub version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entry: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencySpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Lockfile {
    pub version: u32,
    #[serde(default)]
    pub package: Vec<LockedPackage>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LockedPackage {
    pub name: String,
    pub version: String,
    pub source: String,
    pub install_dir: String,
    pub sha256: String,
}

pub fn validate_package_name(name: &str) -> Result<(), Error> {
    if name.is_empty()
        || name == "."
        || name == ".."
        || !name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        return Err(Error::InitFailed(format!(
            "invalid package name '{name}': use ASCII letters, digits, '-' or '_'"
        )));
    }
    Ok(())
}

/// Scaffold a new Lisp project (`lisp.toml`, `src/main.el`, `tests/main-test.el`, `.gitignore`).
pub fn init_project(dir: &Path, name_opt: Option<&str>, force: bool) -> Result<PathBuf, Error> {
    fs::create_dir_all(dir)?;
    let abs_dir = dir.canonicalize().unwrap_or_else(|_| dir.to_path_buf());
    let default_name = abs_dir
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("lisp-app");
    let raw_name = name_opt.unwrap_or(default_name);
    let pkg_name: String = raw_name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect();
    validate_package_name(&pkg_name)?;

    let manifest_path = abs_dir.join(MANIFEST_FILE);
    if manifest_path.exists() && !force {
        return Err(Error::InitFailed(format!(
            "{} already exists (pass --force to overwrite)",
            manifest_path.display()
        )));
    }

    let manifest = Manifest {
        package: PackageSection {
            name: pkg_name.clone(),
            version: "0.1.0".to_string(),
            entry: Some("src/main.el".to_string()),
            description: Some(format!("{pkg_name} standalone Lisp application")),
        },
        dependencies: BTreeMap::new(),
    };
    write_manifest(&abs_dir, &manifest)?;

    let src_dir = abs_dir.join("src");
    let tests_dir = abs_dir.join("tests");
    fs::create_dir_all(&src_dir)?;
    fs::create_dir_all(&tests_dir)?;

    let main_path = src_dir.join("main.el");
    if force || !main_path.exists() {
        let main_el = format!(
            ";;; main.el --- Entry point for {pkg_name} -*- lexical-binding: t; -*-\n\n\
             (defun {pkg_name}-greet (who)\n  \
               \"Return greeting string for WHO.\"\n  \
               (format \"hello, %s!\" who))\n\n\
             (when (and (boundp 'command-line-args-left) (not (bound-and-true-p ert--running-tests)))\n  \
               (let ((who (or (car command-line-args-left) \"world\")))\n    \
                 (princ (concat ({pkg_name}-greet who) \"\\n\"))))\n\n\
             (provide 'main)\n;;; main.el ends here\n"
        );
        fs::write(&main_path, main_el)?;
    }

    let test_path = tests_dir.join("main-test.el");
    if force || !test_path.exists() {
        let test_el = format!(
            ";;; main-test.el --- Tests for {pkg_name} -*- lexical-binding: t; -*-\n\n\
             (require 'ert)\n\
             (defvar ert--running-tests nil)\n\
             (let ((ert--running-tests t))\n  \
               (require 'main))\n\n\
             (ert-deftest {pkg_name}-greet-test ()\n  \
               (should (equal ({pkg_name}-greet \"lisp\") \"hello, lisp!\")))\n\n\
             ;;; main-test.el ends here\n"
        );
        fs::write(&test_path, test_el)?;
    }

    let gitignore_path = abs_dir.join(".gitignore");
    if !gitignore_path.exists() {
        fs::write(&gitignore_path, "/.lisp-packages/\n**/*.elc\n")?;
    }

    Ok(abs_dir)
}

/// Walk ancestor directories from `start` to locate the enclosing project root containing `lisp.toml`.
pub fn find_project_root(start: &Path) -> Option<PathBuf> {
    let mut cur = if start.is_file() {
        start.parent()?.to_path_buf()
    } else {
        start.to_path_buf()
    };
    loop {
        if cur.join(MANIFEST_FILE).is_file() {
            return Some(cur);
        }
        if !cur.pop() {
            return None;
        }
    }
}

/// Resolve `[<root>/src]` + installed `.lisp-packages/*/src` (or root) directories for `load-path`.
pub fn discover_project_load_paths(start: &Path) -> Vec<PathBuf> {
    let Some(root) = find_project_root(start) else {
        return Vec::new();
    };
    let mut paths = Vec::new();
    let src_dir = root.join("src");
    if src_dir.is_dir() {
        paths.push(src_dir);
    } else {
        paths.push(root.clone());
    }

    let pkg_root = root.join(PACKAGES_DIR);
    if let Ok(entries) = fs::read_dir(&pkg_root) {
        let mut dirs: Vec<PathBuf> = entries
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter(|p| p.is_dir())
            .collect();
        dirs.sort();
        for pkg_dir in dirs {
            let pkg_src = pkg_dir.join("src");
            if pkg_src.is_dir() {
                paths.push(pkg_src);
            } else {
                paths.push(pkg_dir);
            }
        }
    }
    paths
}

/// Resolve entry script path from `lisp.toml` (`package.entry` or `src/main.el`).
pub fn discover_project_entry(start: &Path) -> Option<PathBuf> {
    let root = find_project_root(start)?;
    if let Ok(manifest) = read_manifest(&root) {
        if let Some(entry) = manifest.package.entry {
            let candidate = root.join(entry);
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }
    let fallback = root.join("src/main.el");
    if fallback.is_file() {
        Some(fallback)
    } else {
        None
    }
}

/// Discover sorted `.el` test files under `<project_root>/tests/`.
pub fn discover_project_tests(start: &Path) -> Vec<PathBuf> {
    let Some(root) = find_project_root(start) else {
        return Vec::new();
    };
    let tests_dir = root.join("tests");
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(tests_dir) {
        for entry in entries.filter_map(Result::ok) {
            let p = entry.path();
            if p.is_file() && p.extension().is_some_and(|ext| ext == "el") {
                files.push(p);
            }
        }
    }
    files.sort();
    files
}

fn read_manifest(project_root: &Path) -> Result<Manifest, Error> {
    let path = project_root.join(MANIFEST_FILE);
    let raw = fs::read_to_string(&path).map_err(|e| {
        Error::InitFailed(format!("failed to read {}: {e}", path.display()))
    })?;
    toml::from_str(&raw).map_err(|e| {
        Error::InitFailed(format!("invalid TOML in {}: {e}", path.display()))
    })
}

fn write_manifest(project_root: &Path, manifest: &Manifest) -> Result<(), Error> {
    let path = project_root.join(MANIFEST_FILE);
    let text = toml::to_string_pretty(manifest).map_err(|e| {
        Error::InitFailed(format!("failed to serialize {}: {e}", path.display()))
    })?;
    write_atomic(&path, text.as_bytes())
}

fn write_lockfile(project_root: &Path, lock: &Lockfile) -> Result<(), Error> {
    let path = project_root.join(LOCK_FILE);
    let text = toml::to_string_pretty(lock).map_err(|e| {
        Error::InitFailed(format!("failed to serialize {}: {e}", path.display()))
    })?;
    write_atomic(&path, text.as_bytes())
}

fn write_atomic(target: &Path, bytes: &[u8]) -> Result<(), Error> {
    let file_name = target
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("tmp");
    let tmp = target.with_file_name(format!(".{file_name}.tmp.{}", std::process::id()));
    fs::write(&tmp, bytes)?;
    fs::rename(&tmp, target)?;
    Ok(())
}

fn copy_dir_filtered(src: &Path, dst: &Path) -> Result<(), Error> {
    fs::create_dir_all(dst)?;
    let mut entries: Vec<PathBuf> = fs::read_dir(src)?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .collect();
    entries.sort();
    for path in entries {
        let name = path.file_name().unwrap_or_default();
        if name == ".git" || name == PACKAGES_DIR || name == "target" {
            continue;
        }
        let meta = fs::symlink_metadata(&path)?;
        if meta.file_type().is_symlink() {
            continue;
        }
        let target = dst.join(name);
        if meta.is_dir() {
            copy_dir_filtered(&path, &target)?;
        } else if meta.is_file() {
            fs::copy(&path, &target)?;
        }
    }
    Ok(())
}

fn hash_directory_sha256(root: &Path) -> Result<String, Error> {
    let mut files = Vec::new();
    collect_files_sorted(root, root, &mut files)?;
    let mut hasher = Sha256::new();
    for rel in files {
        let rel_str = rel.to_string_lossy().replace('\\', "/");
        let bytes = fs::read(root.join(&rel))?;
        hasher.update((rel_str.len() as u64).to_le_bytes());
        hasher.update(rel_str.as_bytes());
        hasher.update((bytes.len() as u64).to_le_bytes());
        hasher.update(&bytes);
    }
    Ok(hex_digest(&hasher.finalize()))
}

fn collect_files_sorted(root: &Path, dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), Error> {
    let mut entries: Vec<PathBuf> = fs::read_dir(dir)?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            collect_files_sorted(root, &path, out)?;
        } else if path.is_file() {
            let rel = path.strip_prefix(root).unwrap_or(&path).to_path_buf();
            out.push(rel);
        }
    }
    Ok(())
}

fn hex_digest(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        use std::fmt::Write as _;
        let _ = write!(s, "{b:02x}");
    }
    s
}

fn install_single_dep(
    project_root: &Path,
    name: &str,
    spec: &DependencySpec,
) -> Result<LockedPackage, Error> {
    validate_package_name(name)?;
    let pkg_root = project_root.join(PACKAGES_DIR);
    fs::create_dir_all(&pkg_root)?;
    let dest = pkg_root.join(name);
    if dest.exists() {
        fs::remove_dir_all(&dest)?;
    }
    fs::create_dir_all(&dest)?;

    let (source_desc, digest) = match (&spec.path, &spec.url) {
        (Some(rel_or_abs), None) => {
            let src_path = {
                let raw = Path::new(rel_or_abs);
                if raw.is_absolute() {
                    raw.to_path_buf()
                } else {
                    project_root.join(raw)
                }
            };
            if !src_path.is_dir() {
                return Err(Error::InitFailed(format!(
                    "dependency '{name}' path does not exist: {}",
                    src_path.display()
                )));
            }
            copy_dir_filtered(&src_path, &dest)?;
            let digest = hash_directory_sha256(&dest)?;
            (format!("path:{rel_or_abs}"), digest)
        }
        (None, Some(url)) => {
            install_url_dep(name, url, spec.sha256.as_deref(), &dest)?
        }
        _ => {
            return Err(Error::InitFailed(format!(
                "dependency '{name}' must specify exactly one of `path` or `url`"
            )));
        }
    };

    if let Some(ref pinned) = spec.sha256 {
        if !pinned.eq_ignore_ascii_case(&digest) {
            let _ = fs::remove_dir_all(&dest);
            return Err(Error::InitFailed(format!(
                "sha256 mismatch for dependency '{name}': expected {pinned}, got {digest}"
            )));
        }
    }

    let version = spec
        .version
        .clone()
        .or_else(|| {
            read_manifest(&dest)
                .ok()
                .map(|m| m.package.version)
        })
        .unwrap_or_else(|| "0.1.0".to_string());

    Ok(LockedPackage {
        name: name.to_string(),
        version,
        source: source_desc,
        install_dir: format!("{PACKAGES_DIR}/{name}"),
        sha256: digest,
    })
}

fn install_url_dep(
    name: &str,
    url: &str,
    expected_sha256: Option<&str>,
    dest: &Path,
) -> Result<(String, String), Error> {
    #[cfg(feature = "std")]
    {
        let bytes = lisp_std::download_url_bytes(url, 30_000).map_err(|e| {
            Error::InitFailed(format!("failed to download dependency '{name}' from {url}: {e}"))
        })?;
        let dl_digest = hex_digest(&Sha256::digest(&bytes));
        if let Some(pinned) = expected_sha256 {
            if !pinned.eq_ignore_ascii_case(&dl_digest) {
                return Err(Error::InitFailed(format!(
                    "download sha256 mismatch for '{name}': expected {pinned}, got {dl_digest}"
                )));
            }
        }
        let lower = url.split('?').next().unwrap_or(url).to_ascii_lowercase();
        if lower.ends_with(".el") {
            let src_dir = dest.join("src");
            fs::create_dir_all(&src_dir)?;
            fs::write(src_dir.join(format!("{name}.el")), &bytes)?;
        } else {
            lisp_std::extract_archive_bytes(url.split('?').next().unwrap_or(url), &bytes, dest)
                .map_err(|e| {
                    Error::InitFailed(format!(
                        "failed to extract dependency archive '{name}' ({url}): {e}"
                    ))
                })?;
            normalize_single_top_level_dir(dest)?;
        }
        Ok((format!("url:{url}"), dl_digest))
    }
    #[cfg(not(feature = "std"))]
    {
        let _ = (expected_sha256, dest);
        Err(Error::InitFailed(format!(
            "cannot install URL dependency '{name}' ({url}): `std` feature is disabled"
        )))
    }
}

#[cfg(feature = "std")]
fn normalize_single_top_level_dir(dest: &Path) -> Result<(), Error> {
    use std::path::Component;
    if dest.join("lisp.toml").is_file() || dest.join("src").is_dir() {
        return Ok(());
    }
    let entries: Vec<PathBuf> = fs::read_dir(dest)?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .collect();
    if entries.len() == 1 && entries[0].is_dir() {
        let inner = &entries[0];
        if inner.join("lisp.toml").is_file() || inner.join("src").is_dir() {
            for child in fs::read_dir(inner)?.filter_map(Result::ok) {
                let name = child.file_name();
                if let Some(seg) = name.to_str() {
                    if Path::new(seg)
                        .components()
                        .all(|c| matches!(c, Component::Normal(_)))
                    {
                        fs::rename(child.path(), dest.join(name))?;
                    }
                }
            }
            let _ = fs::remove_dir(inner);
        }
    }
    Ok(())
}

/// Add or update dependency `name` in `lisp.toml`, install into `.lisp-packages/`, and update `lisp.lock`.
pub fn pkg_add(
    project_root: &Path,
    name: &str,
    spec: DependencySpec,
) -> Result<LockedPackage, Error> {
    validate_package_name(name)?;
    let mut manifest = read_manifest(project_root)?;
    manifest.dependencies.insert(name.to_string(), spec);
    write_manifest(project_root, &manifest)?;
    let lock = pkg_install(project_root)?;
    lock.package
        .into_iter()
        .find(|p| p.name == name)
        .ok_or_else(|| Error::InitFailed(format!("failed to record '{name}' in {LOCK_FILE}")))
}

/// Install all dependencies declared in `lisp.toml` into `.lisp-packages/` and rewrite `lisp.lock`.
pub fn pkg_install(project_root: &Path) -> Result<Lockfile, Error> {
    let manifest = read_manifest(project_root)?;
    let mut locked = Vec::with_capacity(manifest.dependencies.len());
    for (name, spec) in &manifest.dependencies {
        locked.push(install_single_dep(project_root, name, spec)?);
    }
    let lock = Lockfile {
        version: 1,
        package: locked,
    };
    write_lockfile(project_root, &lock)?;
    Ok(lock)
}

/// List declared/locked dependencies for `project_root`.
pub fn pkg_list(project_root: &Path) -> Result<Vec<String>, Error> {
    let manifest = read_manifest(project_root)?;
    let lock_map: BTreeMap<String, LockedPackage> = fs::read_to_string(project_root.join(LOCK_FILE))
        .ok()
        .and_then(|s| toml::from_str::<Lockfile>(&s).ok())
        .map(|lf| lf.package.into_iter().map(|p| (p.name.clone(), p)).collect())
        .unwrap_or_default();

    let mut lines = Vec::with_capacity(manifest.dependencies.len());
    for (name, spec) in &manifest.dependencies {
        let src = spec
            .path
            .as_deref()
            .map(|p| format!("path:{p}"))
            .or_else(|| spec.url.as_deref().map(|u| format!("url:{u}")))
            .unwrap_or_else(|| "unknown".to_string());
        if let Some(lp) = lock_map.get(name) {
            let short_sha = &lp.sha256[..lp.sha256.len().min(12)];
            lines.push(format!("{name} v{} ({src}, sha256:{short_sha})", lp.version));
        } else {
            lines.push(format!("{name} (not installed, {src})"));
        }
    }
    Ok(lines)
}

/// Re-fetch/re-install all dependencies and refresh `lisp.lock`.
pub fn pkg_update(project_root: &Path) -> Result<Lockfile, Error> {
    pkg_install(project_root)
}

/// Remove dependency `name` from `lisp.toml`, delete `.lisp-packages/<name>`, and prune `lisp.lock`.
pub fn pkg_remove(project_root: &Path, name: &str) -> Result<bool, Error> {
    validate_package_name(name)?;
    let mut manifest = read_manifest(project_root)?;
    let existed = manifest.dependencies.remove(name).is_some();
    write_manifest(project_root, &manifest)?;
    let installed_dir = project_root.join(PACKAGES_DIR).join(name);
    if installed_dir.exists() {
        fs::remove_dir_all(&installed_dir)?;
    }
    let lock_path = project_root.join(LOCK_FILE);
    let mut lock = fs::read_to_string(&lock_path)
        .ok()
        .and_then(|s| toml::from_str::<Lockfile>(&s).ok())
        .unwrap_or(Lockfile {
            version: 1,
            package: Vec::new(),
        });
    lock.package
        .retain(|pkg| manifest.dependencies.contains_key(&pkg.name));
    write_lockfile(project_root, &lock)?;
    Ok(existed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_and_path_pkg_lifecycle_works() {
        let base = std::env::temp_dir().join(format!("lisp-pkg-unit-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        let app = base.join("my-app");
        let dep = base.join("dep-helper");

        init_project(&dep, Some("dep-helper"), false).expect("init dep");
        fs::write(
            dep.join("src/dep-helper.el"),
            ";;; dep-helper.el -*- lexical-binding: t; -*-\n(defun dep-helper-val () 42)\n(provide 'dep-helper)\n",
        )
        .unwrap();

        init_project(&app, Some("my-app"), false).expect("init app");
        assert!(app.join("lisp.toml").is_file());
        assert!(app.join("src/main.el").is_file());
        assert!(app.join("tests/main-test.el").is_file());

        let locked = pkg_add(
            &app,
            "dep-helper",
            DependencySpec {
                path: Some("../dep-helper".to_string()),
                ..DependencySpec::default()
            },
        )
        .expect("pkg add");
        assert_eq!(locked.name, "dep-helper");
        assert!(app.join(".lisp-packages/dep-helper/src/dep-helper.el").is_file());
        assert!(app.join("lisp.lock").is_file());

        let lp = discover_project_load_paths(&app.join("src"));
        assert!(lp.iter().any(|p| p.ends_with("src")));
        assert!(lp.iter().any(|p| p.ends_with("dep-helper/src")));

        let listed = pkg_list(&app).expect("pkg list");
        assert_eq!(listed.len(), 1);
        assert!(listed[0].contains("dep-helper"));

        assert!(pkg_remove(&app, "dep-helper").expect("pkg remove"));
        assert!(!app.join(".lisp-packages/dep-helper").exists());

        let _ = fs::remove_dir_all(&base);
    }
}
