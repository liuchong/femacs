//! Owned run outputs and streamed command evidence shared by local tooling.
#![forbid(unsafe_code)]
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    ffi::OsString,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Component, Path, PathBuf},
    process::{Command, Stdio},
    sync::atomic::{AtomicU64, Ordering},
    time::{Instant, SystemTime, UNIX_EPOCH},
};

static NONCE: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
pub(crate) struct StepPlan {
    pub(crate) id: String,
    pub(crate) program: OsString,
    pub(crate) args: Vec<OsString>,
    pub(crate) cwd: PathBuf,
    pub(crate) env: Vec<(OsString, OsString)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct StepReceipt {
    pub(crate) id: String,
    pub(crate) exit_code: Option<i32>,
    pub(crate) elapsed_ms: f64,
    pub(crate) stdout_log: PathBuf,
    pub(crate) stderr_log: PathBuf,
    pub(crate) completed: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct RunDirectory {
    pub(crate) path: PathBuf,
}

pub(crate) fn nonce() -> Result<String, String> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_nanos();
    Ok(format!(
        "{}-{now}-{}",
        std::process::id(),
        NONCE.fetch_add(1, Ordering::Relaxed)
    ))
}

/// Resolve an output through its existing ancestors, including a symlink parent.
/// No parent traversal or mutation occurs while determining its actual owner.
pub(crate) fn resolved_output(path: &Path) -> Result<PathBuf, String> {
    if path.components().any(|c| matches!(c, Component::ParentDir)) {
        return Err("output path must not contain parent traversal".into());
    }
    let absolute = if path.is_absolute() {
        path.to_owned()
    } else {
        std::env::current_dir()
            .map_err(|e| e.to_string())?
            .join(path)
    };
    let mut ancestor = absolute.as_path();
    let mut tail = Vec::new();
    loop {
        match fs::symlink_metadata(ancestor) {
            Ok(metadata) => {
                if metadata.file_type().is_symlink() && !ancestor.exists() {
                    return Err("output has a broken symlink ancestor".into());
                }
                let mut actual = fs::canonicalize(ancestor).map_err(|e| e.to_string())?;
                for part in tail.iter().rev() {
                    actual.push(part);
                }
                return Ok(actual);
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                tail.push(
                    ancestor
                        .file_name()
                        .ok_or("output has no existing ancestor")?
                        .to_owned(),
                );
                ancestor = ancestor.parent().ok_or("output has no parent")?;
            }
            Err(e) => return Err(e.to_string()),
        }
    }
}

pub(crate) fn guard_scratch(repo: &Path, requested: &Path) -> Result<PathBuf, String> {
    let repo = fs::canonicalize(repo).map_err(|e| format!("repository: {e}"))?;
    let raw = if requested.is_absolute() {
        requested.to_owned()
    } else {
        repo.join(requested)
    };
    if fs::symlink_metadata(&raw)
        .map(|m| m.file_type().is_symlink())
        .unwrap_or(false)
    {
        return Err("output itself must not be a symlink".into());
    }
    let path = resolved_output(&raw)?;
    if path == repo || path.starts_with(repo.join("emacs")) {
        return Err("output must not replace source or the optional reference".into());
    }
    let allowed = [repo.join("build"), repo.join(".femacs-cache")];
    if !allowed
        .iter()
        .any(|root| path.starts_with(root) && path != *root)
    {
        return Err("output must be an owned build/cache child, not source/data".into());
    }
    Ok(path)
}

pub(crate) fn owned_new_output(
    repo: &Path,
    requested: Option<&Path>,
    family: &str,
) -> Result<RunDirectory, String> {
    if family.is_empty()
        || !family
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"-_".contains(&c))
    {
        return Err("run family must be a simple name".into());
    }
    let repo = fs::canonicalize(repo).map_err(|e| e.to_string())?;
    let default = repo.join("build/acceptance").join(family).join(nonce()?);
    let path = guard_scratch(&repo, requested.unwrap_or(&default))?;
    if fs::symlink_metadata(&path).is_ok() {
        return Err("run output already exists; preserve its evidence".into());
    }
    fs::create_dir_all(path.parent().ok_or("run output has no parent")?)
        .map_err(|e| e.to_string())?;
    fs::create_dir(&path).map_err(|e| format!("exclusive run output: {e}"))?;
    Ok(RunDirectory { path })
}

pub(crate) fn write_json<T: Serialize + ?Sized>(path: &Path, value: &T) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| format!("new receipt {}: {e}", path.display()))?;
    serde_json::to_writer_pretty(&mut file, value).map_err(|e| e.to_string())?;
    file.write_all(b"\n").map_err(|e| e.to_string())
}

pub(crate) fn sha256_file(path: &Path) -> Result<String, String> {
    let mut file = File::open(path).map_err(|e| format!("input {}: {e}", path.display()))?;
    let mut digest = Sha256::new();
    let mut block = [0u8; 65536];
    loop {
        let count = file.read(&mut block).map_err(|e| e.to_string())?;
        if count == 0 {
            break;
        }
        digest.update(&block[..count]);
    }
    Ok(format!("{:x}", digest.finalize()))
}

pub(crate) fn execute_step(plan: &StepPlan, run: &RunDirectory) -> Result<StepReceipt, String> {
    if plan.id.is_empty()
        || plan.id == "."
        || plan.id == ".."
        || !plan
            .id
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"-_.".contains(&c))
    {
        return Err("step id must be a simple evidence name".into());
    }
    let stdout_log = run.path.join(format!("{}.stdout.log", plan.id));
    let stderr_log = run.path.join(format!("{}.stderr.log", plan.id));
    let stdout = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&stdout_log)
        .map_err(|e| e.to_string())?;
    let mut stderr = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&stderr_log)
        .map_err(|e| e.to_string())?;
    let start = Instant::now();
    let mut command = Command::new(&plan.program);
    if plan
        .env
        .iter()
        .any(|(name, _)| name == "FEMACS_BUILD_TARGET")
    {
        for key in [
            "FEMACS_MACOS_GUI",
            "FEMACS_TARGET",
            "FEMACS_CONFIG",
            "FEMACS_CONFIG_FILE",
            "FEMACS_CONFIG_RS",
            "FEMACS_EPATHS_RS",
            "FEMACS_LINK_CHECKING",
            "FEMACS_RELEASE_BUILD",
            "FEMACS_OPT_LEVEL",
            "FEMACS_TARGET_CPU",
            "FEMACS_PGO",
            "FEMACS_PGO_DIR",
            "FEMACS_SANITIZER",
        ] {
            command.env_remove(key);
        }
    }
    let result = command
        .args(&plan.args)
        .current_dir(&plan.cwd)
        .envs(plan.env.iter().map(|(name, value)| (name, value)))
        .stdin(Stdio::null())
        .stdout(Stdio::from(stdout))
        .stderr(Stdio::from(stderr.try_clone().map_err(|e| e.to_string())?))
        .status();
    let status = match result {
        Ok(status) => status,
        Err(e) => {
            let _ = writeln!(stderr, "failed to execute {}: {e}", plan.id);
            let failed = StepReceipt {
                id: plan.id.clone(),
                exit_code: None,
                elapsed_ms: start.elapsed().as_secs_f64() * 1000.0,
                stdout_log: stdout_log.clone(),
                stderr_log: stderr_log.clone(),
                completed: false,
            };
            write_json(&run.path.join(format!("{}.json", plan.id)), &failed)?;
            return Err(format!(
                "{} could not execute; log {}: {e}",
                plan.id,
                stderr_log.display()
            ));
        }
    };
    let receipt = StepReceipt {
        id: plan.id.clone(),
        exit_code: status.code(),
        elapsed_ms: start.elapsed().as_secs_f64() * 1000.0,
        stdout_log,
        stderr_log,
        completed: true,
    };
    write_json(&run.path.join(format!("{}.json", plan.id)), &receipt)?;
    Ok(receipt)
}

pub(crate) fn require_success(receipt: &StepReceipt) -> Result<(), String> {
    if receipt.completed && receipt.exit_code == Some(0) {
        Ok(())
    } else {
        Err(format!(
            "{} failed ({:?}); see {}",
            receipt.id,
            receipt.exit_code,
            receipt.stderr_log.display()
        ))
    }
}

#[cfg(test)]
#[path = "tooling_tests.rs"]
mod tests;

/// Hash every actual materialized input with stable relative keys. Symlinks and
/// special files cannot silently introduce a borrowed or unbounded graph input.
pub(crate) fn graph_input_hashes(
    root: &Path,
) -> Result<std::collections::BTreeMap<String, String>, String> {
    if !fs::symlink_metadata(root)
        .map_err(|e| e.to_string())?
        .file_type()
        .is_dir()
    {
        return Err("materialized graph root is not a regular directory".into());
    }
    let mut pending = vec![root.to_owned()];
    let mut files = std::collections::BTreeMap::new();
    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(directory).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            let kind = fs::symlink_metadata(&path)
                .map_err(|e| e.to_string())?
                .file_type();
            if kind.is_dir() {
                pending.push(path);
            } else if kind.is_file() {
                let key = path
                    .strip_prefix(root)
                    .map_err(|e| e.to_string())?
                    .to_str()
                    .ok_or("non-UTF8 graph input")?
                    .to_owned();
                files.insert(key, sha256_file(&path)?);
            } else {
                return Err("materialized graph contains a symlink or special file".into());
            }
        }
    }
    if files.is_empty() {
        return Err("materialized graph has no inputs".into());
    }
    Ok(files)
}
