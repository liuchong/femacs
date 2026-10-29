//! Owned manual generation, verified resource sets and atomic Info publication.
use crate::{bundle, tooling};
use serde_json::{Value, json};
use std::{
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
};

#[derive(Clone, Copy, Debug)]
pub(super) enum ManualFormat {
    Info,
    Html,
}
impl ManualFormat {
    fn name(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Html => "html",
        }
    }
}
#[derive(Debug)]
pub(super) struct ManualReport {
    pub(super) root: PathBuf,
    pub(super) receipt: Value,
}

fn registry(root: &Path) -> Result<Value, String> {
    let text = fs::read_to_string(root.join(".femacs-manuals.json"))
        .map_err(|e| format!("manual set metadata: {e}"))?;
    serde_json::from_str(&text).map_err(|e| e.to_string())
}

fn relative_file(root: &Path, name: &str) -> Result<PathBuf, String> {
    let path = Path::new(name);
    if path.is_absolute()
        || !path
            .components()
            .all(|c| matches!(c, std::path::Component::Normal(_)))
    {
        return Err("manual metadata contains an escaping path".into());
    }
    let base = fs::canonicalize(root).map_err(|e| e.to_string())?;
    let candidate = fs::canonicalize(base.join(path)).map_err(|e| e.to_string())?;
    if !candidate.starts_with(&base) || candidate.starts_with(base.join("emacs")) {
        return Err("manual input escapes the owned root".into());
    }
    Ok(candidate)
}

pub(super) fn verify_set(root: &Path, manifest: &Path) -> Result<(), String> {
    let data = registry(root)?;
    if data["schema"] != "femacs-manual-set-v1" || data["format"] != "info" {
        return Err("not a verified Info set".into());
    }
    if data["manifest_sha256"].as_str() != Some(&tooling::sha256_file(manifest)?) {
        return Err("manual set belongs to a different manifest".into());
    }
    let source: Value = serde_json::from_slice(&fs::read(manifest).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    let manuals = source["manuals"]
        .as_array()
        .ok_or("manual manifest has no roots")?;
    let files = data["files"]
        .as_object()
        .ok_or("manual set has no file identities")?;
    let mut expected = std::collections::BTreeSet::new();
    expected.insert("dir".to_string());
    for manual in manuals {
        let stem = manual["info_name"].as_str().ok_or("manual stem missing")?;
        if stem.is_empty() || stem.contains('/') || stem.contains('\\') {
            return Err("invalid manual output stem".into());
        }
        expected.insert(format!("{stem}.info"));
    }
    if expected != files.keys().cloned().collect() {
        return Err("incomplete or extra Info set membership".into());
    }
    for (name, hash) in files {
        if !fs::symlink_metadata(root.join(name))
            .map_err(|e| e.to_string())?
            .file_type()
            .is_file()
        {
            return Err(format!(
                "manual resource is not an owned regular file: {name}"
            ));
        }
        if tooling::sha256_file(&relative_file(root, name)?)?.as_str()
            != hash.as_str().ok_or("invalid manual hash")?
        {
            return Err(format!("manual identity mismatch: {name}"));
        }
    }
    for entry in fs::read_dir(root).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let name = entry.file_name().to_string_lossy().to_string();
        if !entry.file_type().map_err(|e| e.to_string())?.is_file()
            || (!expected.contains(&name) && name != ".femacs-manuals.json")
        {
            return Err(format!("unexpected Info resource: {name}"));
        }
    }
    Ok(())
}

pub(super) fn build(
    repo: &Path,
    program: &Path,
    out: Option<&Path>,
    format: ManualFormat,
) -> Result<ManualReport, String> {
    let repo = fs::canonicalize(repo).map_err(|e| e.to_string())?;
    let program = fs::canonicalize(program).map_err(|e| format!("owned manual exporter: {e}"))?;
    let fallback = repo.join("build/manuals").join(tooling::nonce()?);
    let output = tooling::guard_scratch(&repo, out.unwrap_or(&fallback))?;
    if fs::symlink_metadata(&output).is_ok() {
        return Err("manual output already exists".into());
    }
    let logs = tooling::owned_new_output(&repo, None, "manual-command")?;
    let plan = tooling::StepPlan {
        id: "generate-manuals".into(),
        program: OsString::from("python3"),
        args: vec![
            repo.join("tools/build-manuals.py").into_os_string(),
            "--program".into(),
            program.clone().into_os_string(),
            "--format".into(),
            format.name().into(),
            "--out".into(),
            output.clone().into_os_string(),
        ],
        cwd: repo.clone(),
        env: Vec::new(),
    };
    let step = tooling::execute_step(&plan, &logs)?;
    tooling::require_success(&step)?;
    let receipt: Value =
        serde_json::from_slice(&fs::read(output.join("receipt.json")).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    if receipt["schema"] != "femacs-manuals-v1"
        || receipt["status"] != "passed"
        || receipt["expected_count"] != receipt["passed_count"]
        || receipt["errors"]
            .as_array()
            .map(|v| !v.is_empty())
            .unwrap_or(true)
        || receipt["program_sha256"].as_str() != Some(&tooling::sha256_file(&program)?)
    {
        return Err("manual builder did not return complete current-product evidence".into());
    }
    if matches!(format, ManualFormat::Info) {
        verify_set(
            &output.join("info"),
            &repo.join("build-support/manuals.json"),
        )?;
    }
    Ok(ManualReport {
        root: output,
        receipt,
    })
}

pub(super) fn copy_set(from: &Path, to: &Path) -> Result<(), String> {
    fs::create_dir_all(to).map_err(|e| e.to_string())?;
    for entry in fs::read_dir(from).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        if !entry.file_type().map_err(|e| e.to_string())?.is_file() {
            return Err("manual set contains a non-file".into());
        }
        crate::owned_copy::copy_file(&entry.path(), &to.join(entry.file_name()))?;
    }
    Ok(())
}

pub(super) fn publish_info(repo: &Path, verified: &Path) -> Result<(), String> {
    let manifest = repo.join("build-support/manuals.json");
    verify_set(verified, &manifest)?;
    let previous = repo.join(format!(".info.previous-{}", tooling::nonce()?));
    let published = bundle::publish_directory(
        &repo.join("info"),
        Some(&previous),
        |stage| copy_set(verified, stage),
        |stage| verify_set(stage, &manifest),
    )?;
    let logs = tooling::owned_new_output(repo, None, "info-publication")?;
    let record = json!({"schema":"femacs-info-publication-v1","destination":published.destination,
        "previous":published.previous,"new_set":registry(&repo.join("info"))?,
        "previous_retained":published.previous.is_some()});
    tooling::write_json(&logs.path.join("receipt.json"), &record)?;
    Ok(())
}

pub(crate) fn recipes_match(repo: &Path, metadata: &Value) -> Result<bool, String> {
    for (key, file) in [
        ("builder_sha256", "tools/build-manuals.py"),
        ("version_recipe_sha256", "tools/version_resources.py"),
    ] {
        if metadata[key].as_str()
            != Some(tooling::sha256_file(&relative_file(repo, file)?)?.as_str())
        {
            return Ok(false);
        }
    }
    Ok(true)
}

pub(super) fn for_product(repo: &Path, root: &Path) -> Result<PathBuf, String> {
    let program = root.join("port/src/emacs");
    let program_hash = tooling::sha256_file(&program)?;
    let pointer = root.join("config/manual-set.json");
    let manifest = repo.join("build-support/manuals.json");
    if let Ok(text) = fs::read_to_string(&pointer) {
        if let Ok(data) = serde_json::from_str::<Value>(&text) {
            if let Some(path) = data["info"].as_str() {
                let candidate = PathBuf::from(path);
                if tooling::guard_scratch(repo, &candidate).is_ok()
                    && verify_set(&candidate, &manifest).is_ok()
                {
                    let metadata = registry(&candidate)?;
                    let current = recipes_match(repo, &metadata)?
                        && metadata["program_sha256"].as_str() == Some(program_hash.as_str())
                        && metadata["inputs"]
                            .as_object()
                            .map(|inputs| {
                                inputs.iter().all(|(name, hash)| {
                                    relative_file(repo, name)
                                        .and_then(|p| tooling::sha256_file(&p))
                                        .ok()
                                        .as_deref()
                                        == hash.as_str()
                                })
                            })
                            .unwrap_or(false);
                    if current {
                        return Ok(candidate);
                    }
                }
            }
        }
    }
    let report = build(repo, &program, None, ManualFormat::Info)?;
    let info = report.root.join("info");
    fs::create_dir_all(pointer.parent().ok_or("manual pointer has no parent")?)
        .map_err(|e| e.to_string())?;
    let staged = pointer.with_extension(format!("{}.json", tooling::nonce()?));
    tooling::write_json(
        &staged,
        &json!({"schema":"femacs-manual-pointer-v1","info":info,"program_sha256":program_hash}),
    )?;
    fs::rename(staged, &pointer).map_err(|e| e.to_string())?;
    Ok(info)
}

pub(super) fn command(args: &[String]) -> i32 {
    let result = (|| -> Result<(), String> {
        let repo = fs::canonicalize(".").map_err(|e| e.to_string())?;
        let mut format = ManualFormat::Info;
        let mut program = repo.join("build/terminal/port/src/emacs");
        let mut out = None;
        let mut at = 0;
        while at < args.len() {
            let value = args.get(at + 1).ok_or("manual option requires a value")?;
            match args[at].as_str() {
                "--program" => program = PathBuf::from(value),
                "--out" => out = Some(PathBuf::from(value)),
                "--format" => {
                    format = match value.as_str() {
                        "info" => ManualFormat::Info,
                        "html" => ManualFormat::Html,
                        _ => return Err("format must be info or html".into()),
                    }
                }
                _ => return Err("unknown build-manuals option".into()),
            }
            at += 2;
        }
        let report = build(&repo, &program, out.as_deref(), format)?;
        println!(
            "Manuals: {} ({})",
            report.root.display(),
            report.receipt["status"]
        );
        Ok(())
    })();
    match result {
        Ok(()) => 0,
        Err(error) => {
            eprintln!("{error}");
            1
        }
    }
}

#[cfg(test)]
#[path = "manuals_tests.rs"]
mod tests;
