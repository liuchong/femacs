//! Target configuration and exact independent translation-unit membership.
#![forbid(unsafe_code)]
use std::{
    collections::BTreeSet,
    fs,
    path::{Component, Path, PathBuf},
};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BuildTarget {
    MacTerminal,
    MacCocoa,
    AndroidArm64,
}
#[derive(Clone, Debug)]
pub(crate) struct UnitsManifest {
    pub core: Vec<String>,
    pub library: Vec<String>,
}
pub(crate) fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace")
        .to_owned()
}
pub(crate) fn config_source(stem: Option<&str>) -> Result<Option<PathBuf>, String> {
    let explicit = std::env::var_os("FEMACS_CONFIG_FILE")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from);
    let selected = explicit.or_else(|| {
        stem.map(str::to_owned)
            .or_else(|| std::env::var("FEMACS_CONFIG").ok())
            .filter(|s| !s.is_empty())
            .map(|s| {
                repo()
                    .join("docs/baseline/config")
                    .join(format!("{s}.json"))
            })
    });
    if let Some(path) = &selected {
        let text = fs::read_to_string(path)
            .map_err(|e| format!("explicit configuration {}: {e}", path.display()))?;
        let data: serde_json::Value =
            serde_json::from_str(&text).map_err(|e| format!("configuration JSON: {e}"))?;
        if !data["macros"].is_object() {
            return Err("configuration has no macros object".into());
        }
    }
    Ok(selected)
}
pub(crate) fn build_target() -> Result<BuildTarget, String> {
    match std::env::var("FEMACS_BUILD_TARGET")
        .ok()
        .filter(|s| !s.is_empty())
        .as_deref()
    {
        Some("macos-terminal") => Ok(BuildTarget::MacTerminal),
        Some("macos-cocoa") => Ok(BuildTarget::MacCocoa),
        Some("android-arm64") => Ok(BuildTarget::AndroidArm64),
        Some(other) => Err(format!("unknown build target {other}")),
        None => Ok(if std::env::var_os("FEMACS_MACOS_GUI").is_some() {
            BuildTarget::MacCocoa
        } else {
            BuildTarget::MacTerminal
        }),
    }
}
pub(crate) fn support_directory(target: BuildTarget) -> PathBuf {
    repo().join(match target {
        BuildTarget::MacTerminal => "build-support/macos/terminal",
        BuildTarget::MacCocoa => "build-support/macos/cocoa",
        BuildTarget::AndroidArm64 => "build-support/android/arm64",
    })
}
fn parse_units(
    path: &Path,
    prefix: &str,
    seen: &mut BTreeSet<String>,
) -> Result<Vec<String>, String> {
    let mut out = vec![];
    let source = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    for line in source
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty() && !s.starts_with('#'))
    {
        let path = Path::new(line);
        if !line.starts_with(prefix)
            || !path.components().all(|c| matches!(c, Component::Normal(_)))
            || !matches!(path.extension().and_then(|s| s.to_str()), Some("c" | "rs"))
        {
            return Err(format!("not an owned target unit: {line}"));
        }
        let mapped = path.with_extension("rs");
        let stem = path
            .file_stem()
            .ok_or("unit stem absent")?
            .to_string_lossy()
            .to_string();
        if matches!(stem.as_str(), "cm" | "termcap" | "terminfo")
            || !repo().join(&mapped).is_file()
            || !seen.insert(stem)
        {
            return Err(format!(
                "missing, duplicate or excluded target unit: {line}"
            ));
        }
        let actual = fs::canonicalize(repo().join(&mapped)).map_err(|e| e.to_string())?;
        if !actual.starts_with(repo().join(prefix.trim_end_matches('/'))) {
            return Err("unit escapes owned source root".into());
        }
        out.push(line.to_owned());
    }
    Ok(out)
}
pub(crate) fn load_units(support: &Path) -> Result<UnitsManifest, String> {
    let mut seen = BTreeSet::new();
    let core = parse_units(&support.join("core-units.txt"), "src/", &mut seen)?;
    let library = parse_units(&support.join("lib-units.txt"), "lib/", &mut seen)?;
    if core.is_empty() || library.is_empty() {
        return Err("target core/library manifest is empty".into());
    }
    Ok(UnitsManifest { core, library })
}
pub(crate) fn expected_objects(
    units: &UnitsManifest,
    root: &Path,
    checking: bool,
) -> Result<Vec<PathBuf>, String> {
    let target = std::env::var("FEMACS_TARGET")
        .ok()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "aarch64-linux-android".into());
    let mut allowed = BTreeSet::new();
    let mut result = vec![];
    for name in units.core.iter().chain(&units.library) {
        let stem = Path::new(name)
            .file_stem()
            .ok_or("unit stem missing")?
            .to_string_lossy();
        for check in [false, true] {
            allowed.insert(crate::check_units::object_path(
                root,
                &stem,
                check,
                Some(&target),
            ));
        }
        let path = crate::check_units::object_path(root, &stem, checking, Some(&target));
        if !path.is_file() {
            return Err(format!(
                "required {} object missing: {}",
                if checking { "checking" } else { "normal" },
                path.display()
            ));
        }
        result.push(path);
    }
    for entry in fs::read_dir(root).map_err(|e| e.to_string())? {
        let path = entry.map_err(|e| e.to_string())?.path();
        if path
            .file_name()
            .is_some_and(|n| n.to_string_lossy().ends_with(&format!(".{target}.o")))
            && !allowed.contains(&path)
        {
            return Err(format!("unexpected target object: {}", path.display()));
        }
    }
    Ok(result)
}
