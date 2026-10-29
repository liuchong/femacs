//! Capture the selected LCMS library and licenses before linking macOS products.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Read;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

fn digest(path: &Path) -> Result<String, String> {
    let mut input = fs::File::open(path).map_err(|e| e.to_string())?;
    let mut hash = Sha256::new();
    let mut buffer = [0u8; 65536];
    loop {
        let count = input.read(&mut buffer).map_err(|e| e.to_string())?;
        if count == 0 { break; }
        hash.update(&buffer[..count]);
    }
    Ok(format!("{:x}", hash.finalize()))
}

fn enabled(config: &Value) -> Result<bool, String> {
    let mut result = None;
    for section in ["macros", "features"] {
        if let Some(value) = config[section].get("HAVE_LCMS2") {
            let flag = match (value.as_str(), value.as_i64(), value.as_bool()) {
                (Some("1" | "true"), _, _) | (_, Some(1), _) | (_, _, Some(true)) => true,
                (Some("0" | "false" | "undefined" | "absent"), _, _) | (_, Some(0), _) | (_, _, Some(false)) => false,
                _ => return Err("invalid HAVE_LCMS2 setting".into()),
            };
            if result.is_some_and(|previous| previous != flag) {
                return Err("conflicting HAVE_LCMS2 settings".into());
            }
            result = Some(flag);
        }
    }
    Ok(result.unwrap_or(false))
}

fn prefix() -> Result<PathBuf, String> {
    let path = if let Some(value) = std::env::var_os("FEMACS_LCMS2_PREFIX") {
        PathBuf::from(value)
    } else {
        let output = Command::new("pkg-config").args(["--variable=prefix", "lcms2"])
            .output().map_err(|e| format!("LCMS requires installed lcms2 and pkg-config: {e}"))?;
        if !output.status.success() || output.stdout.len() > 16384 {
            return Err("pkg-config could not locate lcms2; install it or set FEMACS_LCMS2_PREFIX".into());
        }
        let text = std::str::from_utf8(&output.stdout).map_err(|e| e.to_string())?.trim();
        if text.is_empty() || text.lines().count() != 1 { return Err("invalid LCMS prefix".into()); }
        PathBuf::from(text)
    };
    fs::canonicalize(path).map_err(|e| format!("LCMS prefix: {e}"))
}

fn inputs() -> Result<(PathBuf, PathBuf, u32), String> {
    let prefix = prefix()?;
    let header = fs::canonicalize(prefix.join("include/lcms2.h")).map_err(|e| e.to_string())?;
    let library = fs::canonicalize(prefix.join("lib/liblcms2.dylib")).map_err(|e| e.to_string())?;
    let text = fs::read_to_string(&header).map_err(|e| e.to_string())?;
    let versions: Vec<_> = text.lines().filter_map(|line| {
        let words: Vec<_> = line.split_whitespace().collect();
        if words.len() >= 3 && words[0] == "#define" && words[1] == "LCMS_VERSION" {
            Some(words[2].parse::<u32>().map_err(|_| "nonliteral LCMS_VERSION".to_owned()))
        } else { None }
    }).collect::<Result<_, _>>()?;
    if versions.len() != 1 { return Err("LCMS header must have one literal LCMS_VERSION".into()); }
    Ok((header, library, versions[0]))
}

pub(crate) fn bind_configuration(config: &mut Value) -> Result<(), String> {
    if !enabled(config)? { return Ok(()); }
    let (header, library, version) = inputs()?;
    let header_hash = digest(&header)?;
    let identity = crate::runtime::capture_tls_dependency_identity(
        &header, &library, &crate::macos::deployment_target()?)?;
    if digest(&header)? != header_hash { return Err("LCMS header changed during binding".into()); }
    let abi = identity["abi_id"].as_str().ok_or("LCMS capture has no ABI identity")?;
    let macros = config["macros"].as_object_mut().ok_or("invalid LCMS macro configuration")?;
    for (key, value) in [("FEMACS_LCMS2_DEPENDENCY_ABI", abi),
                         ("FEMACS_LCMS2_HEADER_SHA256", header_hash.as_str())] {
        macros.insert(key.into(), Value::String(serde_json::to_string(value).map_err(|e| e.to_string())?));
    }
    macros.insert("FEMACS_LCMS2_VERSION".into(), Value::String(version.to_string()));
    Ok(())
}

pub(crate) fn prepare(repo: &Path, root: &Path, selected: &Path, destination: &Path) -> Result<(), String> {
    let mut config: Value = serde_json::from_slice(&fs::read(selected).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    if !enabled(&config)? { return crate::tooling::write_json(destination, &config); }
    crate::standalone::check_host()?;
    let owned = crate::tooling::guard_scratch(repo, &root.join("lcms"))?;
    if owned.exists() { return Err("LCMS staging exists; use the original bound build".into()); }
    let mut resolved = config.clone();
    bind_configuration(&mut resolved)?;
    if root.join("runtime-binding.json").is_file() && resolved != config {
        return Err("LCMS dependency changed after precompile binding".into());
    }
    config = resolved;
    let (header, library, version) = inputs()?;
    let header_hash = digest(&header)?;
    let expected_header: String = serde_json::from_str(config["macros"]["FEMACS_LCMS2_HEADER_SHA256"]
        .as_str().ok_or("missing LCMS header binding")?).map_err(|e| e.to_string())?;
    if header_hash != expected_header || config["macros"]["FEMACS_LCMS2_VERSION"] != version.to_string() {
        return Err("LCMS header changed before capture".into());
    }
    let expected: String = serde_json::from_str(config["macros"]["FEMACS_LCMS2_DEPENDENCY_ABI"]
        .as_str().ok_or("missing LCMS dependency binding")?).map_err(|e| e.to_string())?;
    // Reuse the library closure/signing machinery; LCMS has its own owned root.
    let identity = crate::runtime::stage_tls_dependencies(repo, &owned, &header, &library,
        &crate::macos::deployment_target()?, Some(&expected))?;
    if digest(&header)? != header_hash { return Err("LCMS header changed during capture".into()); }
    let primary = identity["dependencies"].as_array().ok_or("missing LCMS dependencies")?.iter()
        .find(|row| row["source_sha256"] == identity["library_sha256"])
        .ok_or("LCMS primary library absent from dependency closure")?;
    let name = Path::new(primary["destination"].as_str().ok_or("missing LCMS destination")?)
        .file_name().and_then(|name| name.to_str()).ok_or("invalid LCMS basename")?;
    crate::tooling::write_json(&owned.join("build-receipt.json"), &json!({
        "schema":1, "version":version, "header_sha256":header_hash,
        "primary":name, "identity":identity,
        "scope":"captured LCMS library/license closure; functional acceptance separate"
    }))?;
    copy_product_runtime(root, &root.join("port/runtime-lcms"), &root.join("port/licenses/lcms"))?;
    crate::tooling::write_json(destination, &config)
}

pub(crate) fn copy_product_runtime(root: &Path, libraries: &Path, licenses: &Path) -> Result<(), String> {
    let source = root.join("lcms/runtime-native");
    if !source.is_dir() { return Ok(()); }
    crate::macos::copy_tree(&source, libraries)?;
    crate::macos::copy_tree(&root.join("lcms/licenses"), licenses)
}

pub(crate) fn link_flags(repo: &Path) -> Result<Vec<String>, String> {
    let Some(selected) = crate::android::config_source(None)? else { return Ok(Vec::new()); };
    let config: Value = serde_json::from_slice(&fs::read(selected).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    if !enabled(&config)? { return Ok(Vec::new()); }
    let root = std::env::var_os("FEMACS_BUILD_ROOT").map(PathBuf::from).ok_or("LCMS requires selected build root")?;
    crate::tooling::guard_scratch(repo, &root)?;
    let receipt: Value = serde_json::from_slice(&fs::read(root.join("lcms/build-receipt.json"))
        .map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    let name = receipt["primary"].as_str().ok_or("missing LCMS primary basename")?;
    if !Path::new(name).components().all(|part| matches!(part, Component::Normal(_)))
        || Path::new(name).components().count() != 1 { return Err("unsafe LCMS basename".into()); }
    for item in receipt["identity"]["dependencies"].as_array().ok_or("missing LCMS closure")? {
        let relative = Path::new(item["destination"].as_str().ok_or("missing LCMS dependency")?);
        if !relative.components().all(|part| matches!(part, Component::Normal(_))) {
            return Err("unsafe LCMS dependency destination".into());
        }
        let basename = relative.file_name().ok_or("missing LCMS dependency basename")?;
        if digest(&root.join("port/runtime-lcms").join(basename))?
            != item["staged_sha256"].as_str().ok_or("missing signed LCMS library identity")? {
            return Err("staged LCMS library changed before link".into());
        }
    }
    Ok(vec![root.join("port/runtime-lcms").join(name).display().to_string(),
        "-Wl,-rpath,@executable_path/../runtime-lcms".into(),
        "-Wl,-rpath,@executable_path/../Frameworks/runtime-lcms".into()])
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn disabled_target_needs_no_lcms_dependency_binding() {
        let started = Instant::now();
        let mut config = json!({"macros":{"HAVE_LCMS2":"undefined"},
                                "features":{"HAVE_LCMS2":"0"}});
        let original = config.clone();
        bind_configuration(&mut config).unwrap();
        assert_eq!(config, original);
        eprintln!("LCMS_PROTECTION_CASE disabled_target_needs_no_lcms_dependency_binding {:.9}",
                  started.elapsed().as_secs_f64());
    }

    #[test]
    fn contradictory_targets_fail_before_dependency_capture() {
        let started = Instant::now();
        let mut config = json!({"macros":{"HAVE_LCMS2":"1"},
                                "features":{"HAVE_LCMS2":"undefined"}});
        let original = config.clone();
        assert_eq!(bind_configuration(&mut config).unwrap_err(),
                   "conflicting HAVE_LCMS2 settings");
        assert_eq!(config, original);
        eprintln!("LCMS_PROTECTION_CASE contradictory_targets_fail_before_dependency_capture {:.9}",
                  started.elapsed().as_secs_f64());
    }
}
