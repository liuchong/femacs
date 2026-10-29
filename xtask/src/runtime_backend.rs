//! Mechanical installed-backend capture and owned staging.
//!
//! This child module records actual installed inputs and stages copies only.
//! It does not compile native code, load an ELN, prove relocation, or claim
//! runtime acceptance.  All external commands run only when the build owner
//! invokes capture/stage in D.

use super::*;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs as std_fs;
use std::io::{Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

const RUNTIME_NATIVE: &str = "runtime-native";
const LICENSES: &str = "licenses";
const TARGET: &str = "aarch64-apple-darwin";
const MAX_COMMAND_BYTES: usize = 8 * 1024 * 1024;
const COPY_CHUNK: usize = 64 * 1024;

struct CaptureState {
    visited: BTreeMap<PathBuf, String>,
    destinations: BTreeSet<PathBuf>,
    dependencies: Vec<BackendDependency>,
    resources: Vec<BackendResource>,
    license_choice: BTreeMap<PathBuf, PathBuf>,
    license_destinations: BTreeSet<PathBuf>,
}

fn path_text(path: &Path) -> Result<String, String> {
    path.to_str()
        .map(ToOwned::to_owned)
        .ok_or_else(|| format!("path is not valid UTF-8: {path:?}"))
}

fn canonical_file(path: &Path) -> Result<PathBuf, String> {
    let canonical = std_fs::canonicalize(path)
        .map_err(|error| format!("cannot canonicalize {path:?}: {error}"))?;
    let metadata = std_fs::metadata(&canonical)
        .map_err(|error| format!("cannot inspect {canonical:?}: {error}"))?;
    if !metadata.is_file() {
        return Err(format!("not a regular file: {canonical:?}"));
    }
    Ok(canonical)
}

fn canonical_dir(path: &Path) -> Result<PathBuf, String> {
    let canonical = std_fs::canonicalize(path)
        .map_err(|error| format!("cannot canonicalize {path:?}: {error}"))?;
    let metadata = std_fs::metadata(&canonical)
        .map_err(|error| format!("cannot inspect {canonical:?}: {error}"))?;
    if !metadata.is_dir() {
        return Err(format!("not a directory: {canonical:?}"));
    }
    Ok(canonical)
}

fn sha256_file(path: &Path) -> Result<String, String> {
    let mut file = std_fs::File::open(path)
        .map_err(|error| format!("cannot open {path:?}: {error}"))?;
    let metadata = file
        .metadata()
        .map_err(|error| format!("cannot inspect {path:?}: {error}"))?;
    if !metadata.is_file() {
        return Err(format!("not a regular file: {path:?}"));
    }
    let mut digest = Sha256::new();
    let mut buffer = [0u8; COPY_CHUNK];
    loop {
        let count = file
            .read(&mut buffer)
            .map_err(|error| format!("cannot read {path:?}: {error}"))?;
        if count == 0 {
            break;
        }
        digest.update(&buffer[..count]);
    }
    Ok(format!("{:x}", digest.finalize()))
}

fn run_command(program: &str, args: &[String]) -> Result<String, String> {
    let output = Command::new(program)
        .args(args.iter().map(String::as_str))
        .output()
        .map_err(|error| format!("{program} failed to start: {error}"))?;
    if output.stdout.len() > MAX_COMMAND_BYTES || output.stderr.len() > MAX_COMMAND_BYTES {
        return Err(format!("{program} output exceeded the bounded limit"));
    }
    if !output.status.success() {
        return Err(format!(
            "{program} exited {:?}: {}",
            output.status.code(),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    String::from_utf8(output.stdout)
        .map_err(|error| format!("{program} produced non-UTF-8 output: {error}"))
}

fn parse_otool(text: &str) -> Result<(String, Vec<String>), String> {
    let mut lines = text.lines().filter(|line| !line.trim().is_empty());
    let _header = lines.next().ok_or("otool -L produced no header")?;
    let mut names = Vec::new();
    for line in lines {
        let name = line
            .trim()
            .split_whitespace()
            .next()
            .ok_or("otool -L produced an empty dependency line")?;
        names.push(name.to_owned());
    }
    if names.is_empty() {
        return Err("otool -L produced no install ID".into());
    }
    let install_id = names.remove(0);
    Ok((install_id, names))
}

fn otool_l(path: &Path) -> Result<(String, Vec<String>), String> {
    let output = run_command(
        "otool",
        &["-L".to_owned(), path_text(path)?],
    )?;
    parse_otool(&output)
}

fn otool_rpaths(path: &Path) -> Result<Vec<String>, String> {
    let output = run_command(
        "otool",
        &["-l".to_owned(), path_text(path)?],
    )?;
    let mut values = Vec::new();
    let mut waiting = false;
    for line in output.lines() {
        let text = line.trim();
        if text == "cmd LC_RPATH" {
            waiting = true;
            continue;
        }
        if !waiting {
            continue;
        }
        if text.starts_with("cmd ") {
            waiting = false;
            continue;
        }
        if let Some(rest) = text.strip_prefix("path ") {
            let value = rest
                .split(" (offset")
                .next()
                .unwrap_or(rest)
                .trim();
            if !value.is_empty() {
                values.push(value.to_owned());
            }
            waiting = false;
        }
    }
    Ok(values)
}

fn system_install(name: &str) -> bool {
    name.starts_with("/usr/lib/")
        || name.starts_with("/System/Library/")
        || name.starts_with("/Library/Apple/")
}

fn resolve_dependency(raw: &str, current: &Path) -> Result<PathBuf, String> {
    if let Some(rest) = raw.strip_prefix("@loader_path/") {
        let base = current
            .parent()
            .ok_or_else(|| format!("library has no parent: {current:?}"))?;
        return canonical_file(&base.join(rest));
    }
    if let Some(rest) = raw.strip_prefix("@rpath/") {
        let parent = current
            .parent()
            .ok_or_else(|| format!("library has no parent: {current:?}"))?;
        let mut found = BTreeSet::new();
        for rpath in otool_rpaths(current)? {
            let base = if let Some(inner) = rpath.strip_prefix("@loader_path/") {
                parent.join(inner)
            } else if rpath.starts_with("@executable_path") {
                return Err(format!("unsupported @executable_path rpath: {rpath}"));
            } else if Path::new(&rpath).is_absolute() {
                PathBuf::from(&rpath)
            } else {
                parent.join(&rpath)
            };
            let candidate = base.join(rest);
            if candidate.exists() {
                found.insert(canonical_file(&candidate)?);
            }
        }
        if found.len() == 1 {
            return found
                .into_iter()
                .next()
                .ok_or_else(|| "unreachable empty rpath result".to_owned());
        }
        return Err(format!(
            "@rpath dependency {raw} resolved to {} candidates",
            found.len()
        ));
    }
    if raw.starts_with("@executable_path") {
        return Err(format!("unsupported @executable_path dependency: {raw}"));
    }
    if raw.starts_with('@') {
        return Err(format!("unsupported install name: {raw}"));
    }
    if Path::new(raw).is_absolute() {
        return canonical_file(Path::new(raw));
    }
    Err(format!("non-absolute dependency install name: {raw}"))
}

fn package_name(root: &Path) -> String {
    let components: Vec<_> = root.components().collect();
    if let Some(index) = components
        .iter()
        .position(|component| component.as_os_str().to_str() == Some("Cellar"))
    {
        if let Some(name) = components
            .get(index + 1)
            .and_then(|component| component.as_os_str().to_str())
        {
            return name.to_owned();
        }
    }
    root.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("package")
        .to_owned()
}

fn safe_package(value: &str) -> String {
    let mut result = String::new();
    for character in value.chars() {
        if character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | '-') {
            result.push(character);
        } else if !result.ends_with('-') {
            result.push('-');
        }
    }
    let result = result.trim_matches('-');
    if result.is_empty() { "package".into() } else { result.to_owned() }
}

fn license_like(name: &str) -> bool {
    let upper = name.to_ascii_uppercase();
    ["COPYING", "LICENSE", "LICENCE", "NOTICE", "COPYRIGHT"]
        .iter()
        .any(|prefix| upper.starts_with(prefix))
}

fn discover_licenses(root: &Path) -> Vec<PathBuf> {
    let mut found = BTreeSet::new();
    let package = package_name(root);
    let mut directories = vec![root.to_path_buf()];
    directories.push(root.join("share/doc").join(&package));
    directories.push(root.join("share/licenses").join(&package));
    for directory in directories {
        let entries = match std_fs::read_dir(&directory) {
            Ok(entries) => entries,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let file_type = match entry.file_type() {
                Ok(file_type) => file_type,
                Err(_) => continue,
            };
            if !file_type.is_file() {
                continue;
            }
            let name = entry.file_name();
            if license_like(&name.to_string_lossy()) {
                found.insert(entry.path());
            }
        }
    }
    found.into_iter().collect()
}

fn find_package_root(path: &Path) -> Option<PathBuf> {
    let components: Vec<_> = path.components().collect();
    if let Some(index) = components
        .iter()
        .position(|component| component.as_os_str().to_str() == Some("Cellar"))
    {
        if components.len() > index + 2 {
            let mut root = PathBuf::new();
            for component in &components[..index + 3] {
                root.push(component.as_os_str());
            }
            if root.is_dir() {
                return Some(root);
            }
        }
    }
    let mut current = path.parent();
    for _ in 0..5 {
        let directory = current?;
        if !discover_licenses(directory).is_empty() {
            return Some(directory.to_path_buf());
        }
        current = directory.parent();
    }
    None
}

fn package_license(source: &Path, state: &mut CaptureState) -> Result<PathBuf, String> {
    let root = find_package_root(source)
        .ok_or_else(|| format!("cannot locate installed license package for {source:?}"))?;
    if let Some(chosen) = state.license_choice.get(&root) {
        return Ok(chosen.clone());
    }
    let package = package_name(&root);
    let safe = safe_package(&package);
    let files = discover_licenses(&root);
    if files.is_empty() {
        return Err(format!(
            "no real license text found for installed package {package} at {root:?}"
        ));
    }
    let mut chosen = None;
    for file in files {
        // Installed packages can retain COPYING both at their root and in
        // share/doc/<package>. Preserve that namespace, including distinct
        // contents, instead of collapsing all licenses onto their basename.
        let relative = file.strip_prefix(&root).map_err(|_| "license escaped its package root")?;
        if !package_relative(relative) { return Err("unsafe relative license resource".into()); }
        let destination = PathBuf::from(LICENSES).join(&safe).join(relative);
        if !state.license_destinations.insert(destination.clone()) {
            return Err(format!("duplicate staged license destination: {destination:?}"));
        }
        let resource = BackendResource {
            source: file.clone(),
            destination: destination.clone(),
            role: "license".into(),
            sha256: sha256_file(&file)?,
            license: destination.clone(),
        };
        state.resources.push(resource);
        if chosen.is_none() {
            chosen = Some(destination);
        }
    }
    let chosen = chosen.ok_or_else(|| format!("missing license text for {package}"))?;
    state.license_choice.insert(root, chosen.clone());
    Ok(chosen)
}

fn visit_library(path: &Path, state: &mut CaptureState) -> Result<String, String> {
    let canonical = canonical_file(path)?;
    if let Some(logical) = state.visited.get(&canonical) {
        return Ok(logical.clone());
    }
    let (install_id, raw_edges) = otool_l(&canonical)?;
    let basename = canonical
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| format!("library basename is not valid UTF-8: {canonical:?}"))?;
    let logical = format!("{RUNTIME_NATIVE}/{basename}");
    let destination = PathBuf::from(&logical);
    if !state.destinations.insert(destination.clone()) {
        return Err(format!("duplicate backend library destination: {destination:?}"));
    }
    state.visited.insert(canonical.clone(), logical.clone());
    let source_sha256 = sha256_file(&canonical)?;
    let license = package_license(&canonical, state)?;
    let mut edges = Vec::new();
    for raw in raw_edges {
        if raw == install_id {
            continue;
        }
        if system_install(&raw) {
            edges.push(format!("system:{raw}"));
            continue;
        }
        let target = resolve_dependency(&raw, &canonical)?;
        if target == canonical {
            continue;
        }
        let target_logical = visit_library(&target, state)?;
        edges.push(target_logical);
    }
    edges.sort();
    edges.dedup();
    state.dependencies.push(BackendDependency {
        logical_id: logical.clone(),
        source: canonical,
        source_sha256,
        staged_sha256: None,
        destination,
        install_id,
        edges,
        license,
    });
    Ok(logical)
}

fn capture_tools() -> Result<Vec<BackendTool>, String> {
    let xcrun = |args: &[&str]| -> Result<String, String> {
        let owned: Vec<String> = args.iter().map(|arg| (*arg).to_owned()).collect();
        Ok(run_command("xcrun", &owned)?.trim().to_owned())
    };
    let assembler = canonical_file(Path::new(&xcrun(&[
        "--sdk", "macosx", "--find", "as",
    ])?))?;
    let linker = canonical_file(Path::new(&xcrun(&[
        "--sdk", "macosx", "--find", "ld",
    ])?))?;
    let sdk = canonical_dir(Path::new(&xcrun(&[
        "--sdk", "macosx", "--show-sdk-path",
    ])?))?;
    let settings = sdk.join("SDKSettings.json");
    let settings_sha = sha256_file(&settings)?;
    let mut tools = vec![
        BackendTool {
            role: "assembler".into(),
            source: assembler.clone(),
            logical_id: "tool/as".into(),
            sha256: Some(sha256_file(&assembler)?),
            version_receipt: None,
        },
        BackendTool {
            role: "linker".into(),
            source: linker.clone(),
            logical_id: "tool/ld".into(),
            sha256: Some(sha256_file(&linker)?),
            version_receipt: None,
        },
        BackendTool {
            role: "sdk".into(),
            source: sdk,
            logical_id: "sdk/macos".into(),
            sha256: Some(settings_sha),
            version_receipt: None,
        },
    ];
    tools.sort_by(|left, right| left.logical_id.cmp(&right.logical_id));
    Ok(tools)
}

fn feed(digest: &mut Sha256, label: &str, value: &str) {
    digest.update((label.len() as u64).to_le_bytes());
    digest.update(label.as_bytes());
    digest.update((value.len() as u64).to_le_bytes());
    digest.update(value.as_bytes());
}

fn backend_digest(identity: &BackendIdentity, label: &str) -> Result<String, String> {
    let mut digest = Sha256::new();
    feed(&mut digest, "label", label);
    feed(&mut digest, "schema", &identity.schema.to_string());
    match identity.version {
        Some(version) => feed(
            &mut digest,
            "version",
            &format!("{}.{}.{}", version[0], version[1], version[2]),
        ),
        None => feed(&mut digest, "version", "none"),
    }
    feed(&mut digest, "header", &identity.header_sha256);
    feed(&mut digest, "library", &identity.library_sha256);
    feed(&mut digest, "target", &identity.target);
    feed(&mut digest, "deployment", &identity.deployment_target);
    feed(&mut digest, "driver", "embedded");
    for tool in &identity.tools {
        feed(&mut digest, "tool-role", &tool.role);
        feed(&mut digest, "tool-id", &tool.logical_id);
        feed(
            &mut digest,
            "tool-sha",
            tool.sha256.as_deref().unwrap_or("none"),
        );
        feed(
            &mut digest,
            "tool-version",
            tool.version_receipt.as_deref().unwrap_or("none"),
        );
    }
    for resource in &identity.resources {
        feed(&mut digest, "resource-destination", &path_text(&resource.destination)?);
        feed(&mut digest, "resource-role", &resource.role);
        feed(&mut digest, "resource-sha", &resource.sha256);
        feed(&mut digest, "resource-license", &path_text(&resource.license)?);
    }
    for dependency in &identity.dependencies {
        feed(&mut digest, "dependency-id", &dependency.logical_id);
        feed(&mut digest, "dependency-source-sha", &dependency.source_sha256);
        feed(&mut digest, "dependency-install-id", &dependency.install_id);
        feed(&mut digest, "dependency-destination", &path_text(&dependency.destination)?);
        feed(&mut digest, "dependency-license", &path_text(&dependency.license)?);
        let mut edges = dependency.edges.clone();
        edges.sort();
        for edge in edges {
            feed(&mut digest, "dependency-edge", &edge);
        }
        feed(
            &mut digest,
            "dependency-staged",
            dependency.staged_sha256.as_deref().unwrap_or("none"),
        );
    }
    Ok(format!("{:x}", digest.finalize()))
}

fn validate_resources(
    resources: &[BackendResource],
    dependencies: &[BackendDependency],
) -> Result<(), String> {
    if resources.is_empty() {
        return Err("backend has no retained license resources".into());
    }
    let mut destinations = BTreeSet::new();
    let mut licenses = BTreeSet::new();
    for resource in resources {
        if resource.role != "license"
            || !package_relative(&resource.destination)
            || resource.destination == PathBuf::from(LICENSES)
            || !resource.destination.starts_with(Path::new(LICENSES))
            || !package_relative(&resource.license)
            || resource.license != resource.destination
            || !sha256_text(&resource.sha256)
            || !destinations.insert(resource.destination.clone())
        {
            return Err("invalid or duplicate backend license resource".into());
        }
        licenses.insert(resource.destination.clone());
    }
    for dependency in dependencies {
        if !licenses.contains(&dependency.license) {
            return Err(format!(
                "dependency {} has no retained license destination",
                dependency.logical_id
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    fn package_fixture() -> (PathBuf, PathBuf) {
        static SEQUENCE: AtomicU64 = AtomicU64::new(0);
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap()
            .join("build/acceptance/runtime-license-contracts")
            .join(format!("{}-{}", std::process::id(), SEQUENCE.fetch_add(1, Ordering::Relaxed)))
            .join("Cellar/mpfr/fixture");
        std_fs::create_dir_all(root.join("lib")).unwrap();
        std_fs::create_dir_all(root.join("share/doc/mpfr")).unwrap();
        for (relative, bytes) in [
            ("COPYING", "root GPL\n"),
            ("COPYING.LESSER", "root LGPL\n"),
            ("share/doc/mpfr/COPYING", "document GPL\n"),
            ("share/doc/mpfr/COPYING.LESSER", "document LGPL\n"),
        ] { std_fs::write(root.join(relative), bytes).unwrap(); }
        let library = root.join("lib/fixture.dylib");
        std_fs::write(&library, b"license-discovery input only; never loaded").unwrap();
        (root, library)
    }

    fn capture_state() -> CaptureState {
        CaptureState { visited: BTreeMap::new(), destinations: BTreeSet::new(),
            dependencies: Vec::new(), resources: Vec::new(),
            license_choice: BTreeMap::new(), license_destinations: BTreeSet::new() }
    }

    #[test]
    fn license_destinations_preserve_distinct_package_relative_paths() {
        let (root, library) = package_fixture();
        let mut state = capture_state();
        let chosen = package_license(&library, &mut state).unwrap();
        assert_eq!(chosen, PathBuf::from("licenses/mpfr/COPYING"));
        let destinations: BTreeSet<_> = state.resources.iter().map(|row| row.destination.clone()).collect();
        assert_eq!(destinations.len(), 4);
        for relative in ["COPYING", "COPYING.LESSER", "share/doc/mpfr/COPYING", "share/doc/mpfr/COPYING.LESSER"] {
            let destination = Path::new("licenses/mpfr").join(relative);
            assert!(destinations.contains(&destination));
            let resource = state.resources.iter().find(|row| row.destination == destination).unwrap();
            assert_eq!(resource.source, root.join(relative));
            assert_eq!(resource.sha256, sha256_file(&resource.source).unwrap());
        }
    }

    #[test]
    fn libraries_in_one_package_reuse_the_captured_license_set() {
        let (root, library) = package_fixture();
        let mut state = capture_state();
        let first = package_license(&library, &mut state).unwrap();
        let second = root.join("lib/second-fixture.dylib");
        std_fs::write(&second, b"second discovery input only").unwrap();
        assert_eq!(package_license(&second, &mut state).unwrap(), first);
        assert_eq!(state.resources.len(), 4);
        assert_eq!(state.license_destinations.len(), 4);
        assert_eq!(state.license_choice.len(), 1);
    }
}

pub(super) fn capture_backend(
    prefix: &Path,
    target: &str,
    deployment_target: &str,
) -> Result<BackendIdentity, String> {
    if target != TARGET {
        return Err(format!("unsupported backend target: {target}"));
    }
    if deployment_target.trim().is_empty() {
        return Err("backend deployment target is empty".into());
    }
    let prefix = canonical_dir(prefix)?;
    let header = canonical_file(&prefix.join("include").join("libgccjit.h"))?;
    let primary = prefix.join("lib/gcc/current/libgccjit.0.dylib");
    let fallback = prefix.join("lib/libgccjit.0.dylib");
    let primary_exists = primary.exists();
    let fallback_exists = fallback.exists();
    let library = match (primary_exists, fallback_exists) {
        (true, false) => canonical_file(&primary)?,
        (false, true) => canonical_file(&fallback)?,
        (true, true) => {
            return Err("both primary and fallback libgccjit candidates exist".into())
        }
        (false, false) => {
            return Err("no installed libgccjit.0.dylib candidate found".into())
        }
    };
    let header_sha256 = sha256_file(&header)?;
    let library_sha256 = sha256_file(&library)?;
    let tools = capture_tools()?;
    let mut state = CaptureState {
        visited: BTreeMap::new(),
        destinations: BTreeSet::new(),
        dependencies: Vec::new(),
        resources: Vec::new(),
        license_choice: BTreeMap::new(),
        license_destinations: BTreeSet::new(),
    };
    let _root = visit_library(&library, &mut state)?;
    let mut dependencies = state.dependencies;
    dependencies.sort_by(|left, right| left.logical_id.cmp(&right.logical_id));
    let mut resources = state.resources;
    resources.sort_by(|left, right| left.destination.cmp(&right.destination));
    validate_dependencies(&dependencies)?;
    validate_resources(&resources, &dependencies)?;
    let mut identity = BackendIdentity {
        schema: 1,
        version: None,
        header_sha256,
        library_sha256,
        target: target.to_owned(),
        deployment_target: deployment_target.to_owned(),
        driver: BackendDriver::Embedded,
        tools,
        resources,
        dependencies,
        abi_id: String::new(),
        cache_key: String::new(),
    };
    identity.abi_id = backend_digest(&identity, "abi")?;
    identity.cache_key = backend_digest(&identity, "cache")?;
    Ok(identity)
}

// Reuse the same actual dependency/license capture for the TLS library. The
// native compiler capture above keeps its original inputs and policy.
pub(super) fn capture_library_runtime(
    header: &Path, library: &Path, deployment_target: &str,
) -> Result<BackendIdentity, String> {
    if deployment_target.trim().is_empty() { return Err("empty deployment target".into()); }
    let header = canonical_file(header)?;
    let library = canonical_file(library)?;
    let mut state = CaptureState {
        visited: BTreeMap::new(), destinations: BTreeSet::new(),
        dependencies: Vec::new(), resources: Vec::new(),
        license_choice: BTreeMap::new(), license_destinations: BTreeSet::new(),
    };
    visit_library(&library, &mut state)?;
    state.dependencies.sort_by(|a, b| a.logical_id.cmp(&b.logical_id));
    state.resources.sort_by(|a, b| a.destination.cmp(&b.destination));
    validate_dependencies(&state.dependencies)?;
    validate_resources(&state.resources, &state.dependencies)?;
    let mut identity = BackendIdentity {
        schema: 1, version: None, header_sha256: sha256_file(&header)?,
        library_sha256: sha256_file(&library)?, target: TARGET.into(),
        deployment_target: deployment_target.into(), driver: BackendDriver::Embedded,
        tools: capture_tools()?, resources: state.resources, dependencies: state.dependencies,
        abi_id: String::new(), cache_key: String::new(),
    };
    identity.abi_id = backend_digest(&identity, "abi")?;
    identity.cache_key = backend_digest(&identity, "cache")?;
    Ok(identity)
}

fn verify_file(path: &Path, expected: &str) -> Result<(), String> {
    let actual = sha256_file(path)?;
    if actual != expected {
        return Err(format!("source changed since capture: {path:?}"));
    }
    Ok(())
}

fn copy_verified(source: &Path, target: &Path, expected: &str) -> Result<(), String> {
    verify_file(source, expected)?;
    let mut source_handle = std_fs::File::open(source)
        .map_err(|error| format!("cannot open source {source:?}: {error}"))?;
    let metadata = source_handle
        .metadata()
        .map_err(|error| format!("cannot inspect source {source:?}: {error}"))?;
    if !metadata.is_file() {
        return Err(format!("source is not a regular file: {source:?}"));
    }
    let mode = metadata.permissions().mode() & 0o777;
    if let Some(parent) = target.parent() {
        std_fs::create_dir_all(parent)
            .map_err(|error| format!("cannot create {parent:?}: {error}"))?;
    }
    let mut target_handle = std_fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(target)
        .map_err(|error| format!("cannot create staged file {target:?}: {error}"))?;
    let mut digest = Sha256::new();
    let mut buffer = [0u8; COPY_CHUNK];
    loop {
        let count = source_handle
            .read(&mut buffer)
            .map_err(|error| format!("cannot read source {source:?}: {error}"))?;
        if count == 0 {
            break;
        }
        target_handle
            .write_all(&buffer[..count])
            .map_err(|error| format!("cannot write staged file {target:?}: {error}"))?;
        digest.update(&buffer[..count]);
    }
    drop(target_handle);
    let actual = format!("{:x}", digest.finalize());
    if actual != expected {
        return Err(format!("source changed while copying: {source:?}"));
    }
    std_fs::set_permissions(target, std_fs::Permissions::from_mode(mode))
        .map_err(|error| format!("cannot preserve mode on {target:?}: {error}"))?;
    Ok(())
}

fn append_sign_order(
    name: &str,
    graph: &BTreeMap<String, Vec<String>>,
    states: &mut BTreeMap<String, u8>,
    order: &mut Vec<String>,
) -> Result<(), String> {
    match states.get(name).copied().unwrap_or(0) {
        2 => return Ok(()),
        1 => return Err(format!("backend dependency cycle at {name}")),
        _ => {}
    }
    states.insert(name.to_owned(), 1);
    if let Some(edges) = graph.get(name) {
        for edge in edges {
            append_sign_order(edge, graph, states, order)?;
        }
    }
    states.insert(name.to_owned(), 2);
    order.push(name.to_owned());
    Ok(())
}

pub(super) fn stage_backend(
    repo: &Path,
    out: &Path,
    identity: &mut BackendIdentity,
) -> Result<(), String> {
    stage_library_bundle(repo, out, identity, true)
}

pub(super) fn stage_tls_backend(
    repo: &Path, out: &Path, identity: &mut BackendIdentity,
) -> Result<(), String> {
    stage_library_bundle(repo, out, identity, false)
}

fn stage_library_bundle(
    repo: &Path, out: &Path, identity: &mut BackendIdentity, runtime_only: bool,
) -> Result<(), String> {
    if identity.schema != 1
        || identity.target != TARGET
        || identity.deployment_target.trim().is_empty()
        || !sha256_text(&identity.header_sha256)
        || !sha256_text(&identity.library_sha256)
        || !sha256_text(&identity.abi_id)
        || !sha256_text(&identity.cache_key)
    {
        return Err("invalid backend identity for staging".into());
    }
    if identity.dependencies.is_empty() {
        return Err("backend identity has no libraries".into());
    }
    validate_dependencies(&identity.dependencies)?;
    validate_resources(&identity.resources, &identity.dependencies)?;
    for dependency in &identity.dependencies {
        if dependency.staged_sha256.is_some() {
            return Err(format!(
                "dependency already staged: {}",
                dependency.logical_id
            ));
        }
        if dependency.destination.parent() != Some(Path::new(RUNTIME_NATIVE)) {
            return Err(format!(
                "dependency destination is not direct child of {RUNTIME_NATIVE}: {:?}",
                dependency.destination
            ));
        }
    }
    let out = if runtime_only { validate_output(repo, out, true)? }
        else { crate::tooling::guard_scratch(repo, out)? };
    std_fs::create_dir_all(&out)
        .map_err(|error| format!("cannot create runtime output {out:?}: {error}"))?;
    let runtime = out.join(RUNTIME_NATIVE);
    let licenses = out.join(LICENSES);
    if std_fs::symlink_metadata(&runtime).is_ok() {
        return Err(format!("staged runtime destination already exists: {runtime:?}"));
    }
    if std_fs::symlink_metadata(&licenses).is_ok() {
        return Err(format!("staged license destination already exists: {licenses:?}"));
    }
    std_fs::create_dir(&runtime)
        .map_err(|error| format!("cannot create {runtime:?}: {error}"))?;
    std_fs::create_dir(&licenses)
        .map_err(|error| format!("cannot create {licenses:?}: {error}"))?;

    for resource in &identity.resources {
        let target = out.join(&resource.destination);
        copy_verified(&resource.source, &target, &resource.sha256)?;
    }
    for dependency in &identity.dependencies {
        let target = out.join(&dependency.destination);
        copy_verified(&dependency.source, &target, &dependency.source_sha256)?;
    }

    let mut indices = BTreeMap::new();
    for (index, dependency) in identity.dependencies.iter().enumerate() {
        indices.insert(dependency.logical_id.clone(), index);
    }
    for dependency in &identity.dependencies {
        let target = out.join(&dependency.destination);
        let id_name = dependency
            .logical_id
            .rsplit('/')
            .next()
            .ok_or_else(|| format!("invalid dependency logical ID: {}", dependency.logical_id))?;
        run_command(
            "install_name_tool",
            &[
                "-id".to_owned(),
                format!("@rpath/{id_name}"),
                path_text(&target)?,
            ],
        )?;
        let (_new_install_id, raw_edges) = otool_l(&target)?;
        for edge in &dependency.edges {
            if edge.starts_with("system:") {
                continue;
            }
            let index = indices
                .get(edge)
                .ok_or_else(|| format!("missing staged dependency edge target: {edge}"))?;
            let target_dependency = &identity.dependencies[*index];
            let old_name = &target_dependency.install_id;
            let old_base = Path::new(old_name)
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| format!("invalid original install name: {old_name}"))?;
            let target_base = target_dependency
                .logical_id
                .rsplit('/')
                .next()
                .ok_or_else(|| {
                    format!("invalid target logical ID: {}", target_dependency.logical_id)
                })?;
            let matches: Vec<&String> = raw_edges
                .iter()
                .filter(|raw| {
                    !system_install(raw.as_str())
                        && (raw.as_str() == old_name
                            || Path::new(raw)
                                .file_name()
                                .and_then(|name| name.to_str())
                                == Some(old_base))
                })
                .collect();
            if matches.len() != 1 {
                return Err(format!(
                    "edge {edge} matched {} copied install names in {target:?}",
                    matches.len()
                ));
            }
            let new_edge = format!("@loader_path/{target_base}");
            run_command(
                "install_name_tool",
                &[
                    "-change".to_owned(),
                    (*matches[0]).clone(),
                    new_edge,
                    path_text(&target)?,
                ],
            )?;
        }
    }

    let mut graph = BTreeMap::new();
    for dependency in &identity.dependencies {
        let edges: Vec<String> = dependency
            .edges
            .iter()
            .filter(|edge| !edge.starts_with("system:"))
            .cloned()
            .collect();
        graph.insert(dependency.logical_id.clone(), edges);
    }
    let mut states = BTreeMap::new();
    let mut order = Vec::new();
    for dependency in &identity.dependencies {
        append_sign_order(&dependency.logical_id, &graph, &mut states, &mut order)?;
    }
    for name in order {
        let index = indices
            .get(&name)
            .ok_or_else(|| format!("missing sign-order dependency: {name}"))?;
        let target = out.join(&identity.dependencies[*index].destination);
        run_command(
            "codesign",
            &[
                "--force".to_owned(),
                "--sign".to_owned(),
                "-".to_owned(),
                path_text(&target)?,
            ],
        )?;
        let staged = sha256_file(&target)?;
        identity.dependencies[*index].staged_sha256 = Some(staged);
    }

    for dependency in &identity.dependencies {
        verify_file(&dependency.source, &dependency.source_sha256)?;
    }
    for resource in &identity.resources {
        verify_file(&resource.source, &resource.sha256)?;
    }
    Ok(())
}
