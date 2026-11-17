//! Capture the selected PNG/JPEG runtime libraries and licenses for macOS products.
//!
//! The image decoders are ordinary dylibs resolved through pkg-config.  Only the
//! libraries and their license texts are copied into the product; headers stay
//! build inputs.  Every staged byte is recorded in a receipt the linker and the
//! product receipt re-verify, and no library is ever taken from the system.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Read;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

const LIBS: &[&str] = &["libpng", "libjpeg", "sqlite3"];

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

fn flag(config: &Value, key: &str, reported: &mut Option<bool>) -> Result<(), String> {
    for section in ["macros", "features"] {
        if let Some(value) = config[section].get(key) {
            let enabled = match (value.as_str(), value.as_i64(), value.as_bool()) {
                (Some("1" | "true"), _, _) | (_, Some(1), _) | (_, _, Some(true)) => true,
                (Some("0" | "false" | "undefined" | "absent"), _, _) | (_, Some(0), _) | (_, _, Some(false)) => false,
                _ => return Err(format!("invalid {key} setting")),
            };
            if reported.is_some_and(|previous| previous != enabled) {
                return Err(format!("conflicting {key} settings"));
            }
            *reported = Some(enabled);
            break;
        }
    }
    Ok(())
}

fn enabled(config: &Value) -> Result<bool, String> {
    let mut any = false;
    for adaptor in LIBS {
        let key = match *adaptor {
            "libpng" => "HAVE_PNG", "libjpeg" => "HAVE_JPEG", _ => "HAVE_SQLITE3",
        };
        let mut value = None;
        flag(config, key, &mut value)?;
        any |= value.unwrap_or(false);
    }
    Ok(any)
}

fn prefix(adaptor: &str) -> Result<PathBuf, String> {
    let variable = format!("FEMACS_{}_PREFIX", adaptor.to_ascii_uppercase().replace('-', "_"));
    let path = if let Some(value) = std::env::var_os(&variable) {
        PathBuf::from(value)
    } else {
        let output = Command::new("pkg-config").args(["--variable=prefix", adaptor])
            .output().map_err(|e| format!("image support requires pkg-config and {adaptor}: {e}"))?;
        if !output.status.success() || output.stdout.len() > 16384 {
            return Err(format!("pkg-config could not locate {adaptor}; install it or set {variable}"));
        }
        let text = std::str::from_utf8(&output.stdout).map_err(|e| e.to_string())?.trim();
        if text.is_empty() || text.lines().count() != 1 { return Err(format!("invalid {adaptor} prefix")); }
        PathBuf::from(text)
    };
    fs::canonicalize(path).map_err(|e| format!("{adaptor} prefix: {e}"))
}

fn dylibs(prefix: &Path, adaptor: &str) -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    let Ok(entries) = fs::read_dir(prefix.join("lib")) else { return candidates };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or_default();
        if !name.ends_with(".dylib") || path.is_symlink() { continue; }
        let wanted = match adaptor {
            "libpng" => name.starts_with("libpng"),
            "libjpeg" => name.starts_with("libjpeg"),
            _ => name.starts_with("libsqlite3"),
        };
        if wanted { candidates.push(path); }
    }
    candidates.sort_by_key(|path| path.file_name().map(|n| n.len()).unwrap_or(usize::MAX));
    candidates
}

fn inputs(adaptor: &str) -> Result<(PathBuf, PathBuf), String> {
    let header_name = match adaptor {
        "libpng" => "png.h", "libjpeg" => "jpeglib.h", _ => "sqlite3.h",
    };
    let mut prefix = prefix(adaptor)?;
    let mut candidates = dylibs(&prefix, adaptor);
    if candidates.is_empty() && adaptor == "sqlite3" {
        // The SDK's sqlite3.pc wins whenever PKG_CONFIG_PATH names a prefix that
        // does not carry sqlite3 itself, and the SDK ships only a .tbd, not a
        // library.  The product needs a real library with extension loading, so
        // try the maintained kegs before giving up.
        for candidate in ["/opt/homebrew/opt/sqlite", "/usr/local/opt/sqlite"] {
            let Ok(resolved) = fs::canonicalize(candidate) else { continue };
            let found = dylibs(&resolved, adaptor);
            if !found.is_empty() { prefix = resolved; candidates = found; break; }
        }
    }
    let header = fs::canonicalize(prefix.join("include").join(header_name))
        .map_err(|e| format!("{} header: {e}", adaptor))?;
    let library = candidates.into_iter().next()
        .ok_or_else(|| format!("{adaptor} prefix has no dynamic library"))?;
    Ok((header, fs::canonicalize(library).map_err(|e| e.to_string())?))
}

fn license_bytes(prefix: &Path, adaptor: &str) -> Result<(Vec<u8>, String), String> {
    let entries: Vec<PathBuf> = fs::read_dir(prefix).map_err(|e| e.to_string())?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.is_file())
        .collect();
    let find = |wanted: &[&str]| -> Option<PathBuf> {
        let mut names: Vec<PathBuf> = entries.iter()
            .filter(|path| {
                let name = path.file_name().and_then(|n| n.to_str())
                    .unwrap_or_default().to_ascii_uppercase();
                wanted.iter().any(|prefix| name.starts_with(prefix))
            })
            .cloned()
            .collect();
        names.sort();
        names.into_iter().next()
    };
    let name_of = |path: &Path| -> String {
        path.file_name().and_then(|n| n.to_str()).unwrap_or_default().to_string()
    };
    if let Some(path) = find(&["LICENSE", "LICENCE", "COPYING"]) {
        return Ok((fs::read(&path).map_err(|e| e.to_string())?, name_of(&path)));
    }
    // Some kegs ship the terms only in their README (the IJG's "LEGAL ISSUES"
    // section is the license of the jpeg keg).  The file is copied whole, which
    // keeps the notice and its conditions together.
    if let Some(path) = find(&["README"]) {
        let bytes = fs::read(&path).map_err(|e| e.to_string())?;
        let text = String::from_utf8_lossy(&bytes);
        if text.contains("LEGAL ISSUES")
            || text.to_ascii_lowercase().contains("copyright")
            || text.to_ascii_lowercase().contains("all rights reserved") {
            return Ok((bytes, name_of(&path)));
        }
    }
    // SQLite is public domain and its keg intentionally ships no license file.
    // Record the package's own SPDX conclusion instead of inventing text.
    if adaptor == "sqlite3" {
        let sbom = prefix.join("sbom.spdx.json");
        if let Ok(bytes) = fs::read(&sbom) {
            if let Ok(value) = serde_json::from_slice::<Value>(&bytes) {
                let package = value.get("packages").and_then(|p| p.get(0)).cloned().unwrap_or(Value::Null);
                let name = package.get("name").and_then(|v| v.as_str()).unwrap_or(adaptor);
                let version = package.get("versionInfo").and_then(|v| v.as_str()).unwrap_or("");
                let license = package.get("licenseConcluded").and_then(|v| v.as_str()).unwrap_or("unknown");
                return Ok((format!(
                    "{name} {version}\n\nSPDX licenseConcluded: {license}\n\
                     Source metadata: {}\nhttps://sqlite.org/copyright.html\n",
                    sbom.display()).into_bytes(), "sbom.spdx.json".to_string()));
            }
        }
    }
    Err(format!("image dependency prefix has no license text: {}", prefix.display()))
}

fn stage(repo: &Path, root: &Path, adaptor: &str, out: &Path) -> Result<Value, String> {
    let (header, library) = inputs(adaptor)?;
    // The license sits at the resolved library prefix, which may differ from
    // what pkg-config reported (the SDK answers for sqlite3 but carries no
    // library and no license).
    let prefix = library.parent().and_then(|lib| lib.parent())
        .ok_or_else(|| format!("{adaptor} library has no prefix"))?;
    let (license_bytes, license_source) = license_bytes(prefix, adaptor)?;
    if !out.is_dir() {
        crate::tooling::guard_scratch(repo, out)?;
        fs::create_dir(out).map_err(|e| e.to_string())?;
    }
    let base = library.file_name().and_then(|n| n.to_str()).ok_or("invalid image library name")?;
    if !Path::new(base).components().all(|part| matches!(part, Component::Normal(_))) {
        return Err("unsafe image library name".into());
    }
    let destination = out.join(base);
    fs::copy(&library, &destination).map_err(|e| e.to_string())?;
    let status = Command::new("install_name_tool")
        .args(["-id".to_owned(), format!("@rpath/{base}"), destination.display().to_string()])
        .status().map_err(|e| e.to_string())?;
    if !status.success() { return Err("install_name_tool could not rewrite image library".into()); }
    // Rewriting a load command invalidates the keg's ad-hoc signature, and the
    // arm64 loader kills a process that maps an invalidly signed library
    // (measured: the dump program exited with SIGKILL).  Re-sign the staged copy
    // exactly as the other captured runtimes do, then verify before recording it.
    let signed = Command::new("codesign")
        .args(["--force", "--sign", "-"]).arg(&destination)
        .status().map_err(|e| e.to_string())?;
    if !signed.success() { return Err(format!("codesign could not sign staged {adaptor} library")); }
    let verified = Command::new("codesign").arg("--verify").arg(&destination)
        .status().map_err(|e| e.to_string())?;
    if !verified.success() { return Err(format!("staged {adaptor} library signature does not verify")); }
    let license_name = format!("{}.txt", adaptor);
    let staged_license = out.join(&license_name);
    fs::write(&staged_license, &license_bytes).map_err(|e| e.to_string())?;
    let _ = root;
    Ok(json!({
        "adaptor": adaptor,
        "header_sha256": digest(&header)?,
        "source_sha256": digest(&library)?,
        "destination": base,
        "staged_sha256": digest(&destination)?,
        "license": {"source": license_source,
                    "source_sha256": format!("{:x}", Sha256::digest(&license_bytes)),
                    "destination": license_name,
                    "staged_sha256": digest(&staged_license)?}
    }))
}

pub(crate) fn bind_configuration(config: &mut Value) -> Result<(), String> {
    if !enabled(config)? { return Ok(()); }
    let macros = config["macros"].as_object_mut().ok_or("invalid image macro configuration")?;
    for adaptor in LIBS {
        let (header, library) = inputs(adaptor)?;
        let key = match *adaptor {
            "libpng" => "PNG", "libjpeg" => "JPEG", _ => "SQLITE3",
        };
        macros.insert(format!("FEMACS_{key}_HEADER_SHA256"),
                      Value::String(serde_json::to_string(&digest(&header)?).map_err(|e| e.to_string())?));
        macros.insert(format!("FEMACS_{key}_LIBRARY_SHA256"),
                      Value::String(serde_json::to_string(&digest(&library)?).map_err(|e| e.to_string())?));
    }
    Ok(())
}

pub(crate) fn prepare(repo: &Path, root: &Path, selected: &Path, destination: &Path) -> Result<(), String> {
    let mut config: Value = serde_json::from_slice(&fs::read(selected).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    if !enabled(&config)? { return crate::tooling::write_json(destination, &config); }
    crate::standalone::check_host()?;
    if root.join("image").exists() {
        return Err("image staging exists; use the original bound build".into());
    }
    let runtime = root.join("image/runtime-native");
    let licenses = root.join("image/licenses");
    fs::create_dir_all(&runtime).map_err(|e| e.to_string())?;
    fs::create_dir_all(&licenses).map_err(|e| e.to_string())?;
    let mut rows = Vec::new();
    for adaptor in LIBS {
        let key = match *adaptor {
            "libpng" => "HAVE_PNG", "libjpeg" => "HAVE_JPEG", _ => "HAVE_SQLITE3",
        };
        let mut wanted = None;
        flag(&config, key, &mut wanted)?;
        if !wanted.unwrap_or(false) { continue; }
        let staged = stage(repo, root, adaptor, &root.join("image-staging"))?;
        let destination_name = staged["destination"].as_str().ok_or("missing image destination")?;
        fs::copy(root.join("image-staging").join(destination_name), runtime.join(destination_name))
            .map_err(|e| e.to_string())?;
        let license_name = staged["license"]["destination"].as_str().ok_or("missing image license")?;
        fs::copy(root.join("image-staging").join(license_name), licenses.join(license_name))
            .map_err(|e| e.to_string())?;
        rows.push(json!({
            "adaptor": adaptor,
            "source_sha256": staged["source_sha256"],
            "destination": destination_name,
            "staged_sha256": digest(&runtime.join(destination_name))?,
            "license": {"destination": license_name,
                        "staged_sha256": digest(&licenses.join(license_name))?}
        }));
    }
    crate::tooling::write_json(&root.join("image/build-receipt.json"), &json!({
        "schema": 1,
        "libraries": rows,
        "scope": "captured PNG/JPEG runtime libraries and licenses; functional acceptance separate"
    }))?;
    // The linked binary loads these through `@executable_path/../runtime-image`, the same
    // build-tree placement the other captured runtimes use, so the dump and gate runs find
    // the exact bytes the receipt recorded.
    copy_product_runtime(root, &root.join("port/runtime-image"),
                         &root.join("port/licenses/image"))?;
    crate::tooling::write_json(destination, &config)
}

pub(crate) fn copy_product_runtime(root: &Path, libraries: &Path, licenses: &Path) -> Result<(), String> {
    let source = root.join("image/runtime-native");
    if !source.is_dir() { return Ok(()); }
    crate::macos::copy_tree(&source, libraries)?;
    crate::macos::copy_tree(&root.join("image/licenses"), licenses)
}

pub(crate) fn link_flags(repo: &Path) -> Result<Vec<String>, String> {
    let Some(selected) = crate::android::config_source(None)? else { return Ok(Vec::new()); };
    let config: Value = serde_json::from_slice(&fs::read(selected).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    if !enabled(&config)? { return Ok(Vec::new()); }
    let root = std::env::var_os("FEMACS_BUILD_ROOT").map(PathBuf::from)
        .ok_or("image support requires the selected build root")?;
    crate::tooling::guard_scratch(repo, &root)?;
    let receipt: Value = serde_json::from_slice(&fs::read(root.join("image/build-receipt.json"))
        .map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    let mut args = Vec::new();
    for row in receipt["libraries"].as_array().ok_or("missing image receipt libraries")? {
        let name = row["destination"].as_str().ok_or("missing image destination")?;
        if !Path::new(name).components().all(|part| matches!(part, Component::Normal(_)))
            || Path::new(name).components().count() != 1 {
            return Err("unsafe image destination".into());
        }
        let path = root.join("image/runtime-native").join(name);
        if digest(&path)? != row["staged_sha256"].as_str().ok_or("missing image staged identity")? {
            return Err("staged image library changed before link".into());
        }
        args.push(path.display().to_string());
    }
    args.push("-Wl,-rpath,@executable_path/../runtime-image".into());
    args.push("-Wl,-rpath,@executable_path/../Frameworks/runtime-image".into());
    Ok(args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sqlite_alone_enables_the_bundle() {
        let config = json!({"macros": {"HAVE_PNG": "undefined", "HAVE_JPEG": "undefined"},
                            "features": {"HAVE_SQLITE3": "1"}});
        assert_eq!(enabled(&config).unwrap(), true);
    }
}
