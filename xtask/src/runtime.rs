//! P-346 selected runtime-product configuration and identity ownership.
//! This module's pure contracts are separate from actual build receipts.
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::Command;
use std::io::{Read, Write};

#[path = "runtime_backend.rs"]
mod backend;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RuntimeProduct { Terminal, Cocoa }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RuntimeVariant { Baseline, Native, Collector, NativeCollector }
impl RuntimeVariant {
    fn native(self) -> bool { matches!(self, Self::Native | Self::NativeCollector) }
    fn collector(self) -> bool { matches!(self, Self::Collector | Self::NativeCollector) }
}
#[derive(Clone, Debug)]
pub(crate) struct RuntimeBuildOptions {
    product: RuntimeProduct,
    variant: RuntimeVariant,
    observer: bool,
    test_hooks: bool,
    out: PathBuf,
    backend_prefix: Option<PathBuf>,
}
pub(crate) fn parse_options(args: &[String]) -> Result<RuntimeBuildOptions, String> {
    let mut values = BTreeMap::new();
    let mut index = 0;
    while index < args.len() {
        let key = args[index].as_str();
        let flag = matches!(key, "--observer" | "--test-hooks");
        if !flag && !matches!(key, "--product" | "--variant" | "--out" | "--backend-prefix") {
            return Err(format!("unknown runtime-build option: {key}"));
        }
        if values.contains_key(key) { return Err(format!("duplicate runtime-build option: {key}")); }
        index += 1;
        let value = if flag { "1".to_owned() } else {
            let value = args.get(index).ok_or_else(|| format!("missing value for {key}"))?;
            if value.is_empty() || value.starts_with("--") { return Err(format!("missing value for {key}")); }
            index += 1;
            value.clone()
        };
        values.insert(key.to_owned(), value);
    }
    let required = |key: &str| values.get(key).map(String::as_str)
        .ok_or_else(|| format!("required runtime-build option: {key}"));
    let product = match required("--product")? {
        "terminal" => RuntimeProduct::Terminal, "cocoa" => RuntimeProduct::Cocoa,
        _ => return Err("unknown runtime product".into()),
    };
    let variant = match required("--variant")? {
        "baseline" => RuntimeVariant::Baseline, "native" => RuntimeVariant::Native,
        "collector" => RuntimeVariant::Collector, "native-collector" => RuntimeVariant::NativeCollector,
        _ => return Err("unknown runtime variant".into()),
    };
    let out = PathBuf::from(required("--out")?);
    let backend_prefix = values.get("--backend-prefix").map(PathBuf::from);
    if backend_prefix.is_some() != variant.native() {
        return Err("--backend-prefix is required exactly for a native variant".into());
    }
    let observer = values.contains_key("--observer");
    let test_hooks = values.contains_key("--test-hooks");
    if test_hooks && !observer { return Err("--test-hooks requires --observer".into()); }
    Ok(RuntimeBuildOptions { product, variant, observer, test_hooks, out, backend_prefix })
}

fn truth(value: &Value) -> Result<String, String> {
    let text = match value {
        Value::String(text) => text.clone(), Value::Bool(value) => value.to_string(),
        Value::Number(value) => value.to_string(),
        _ => return Err("configuration scalar must be a string, boolean or number".into()),
    };
    Ok(match text.as_str() {
        "1" | "true" => "1".into(), "0" | "false" | "undefined" | "absent" => "0".into(),
        _ => text,
    })
}
pub(crate) fn canonical_config(base: &Value, opts: &RuntimeBuildOptions) -> Result<Value, String> {
    let source = base.as_object().ok_or("runtime configuration must be an object")?;
    let mut settings: BTreeMap<String, String> = BTreeMap::new();
    for section in ["macros", "features"] {
        if let Some(object) = source.get(section) {
            let object = object.as_object().ok_or_else(|| format!("{section} must be an object"))?;
            for (key, value) in object {
                let normalized = key.to_ascii_uppercase();
                let value = truth(value)?;
                if let Some(previous) = settings.insert(normalized.clone(), value.clone()) {
                    if previous != value { return Err(format!("conflicting configuration setting: {normalized}")); }
                }
            }
        }
    }
    if opts.test_hooks && !opts.observer { return Err("test hooks require an observer".into()); }
    let mut selected = base.clone();
    for section in ["macros", "features"] {
        if selected.get(section).is_none() { selected[section] = Value::Object(Map::new()); }
    }
    let gates = [
        ("HAVE_NATIVE_COMP", opts.variant.native()),
        ("HAVE_LIBGCCJIT_H", opts.variant.native()),
        ("HAVE_LIBGCCJIT", opts.variant.native()),
        ("FEMACS_GC_OBSERVER", opts.observer),
        ("FEMACS_GC_GENERATIONAL", opts.variant.collector()),
        ("FEMACS_GC_OBSERVER_TEST_HOOKS", opts.test_hooks),
    ];
    for (key, enabled) in gates {
        let value = Value::String(if enabled { "1" } else { "undefined" }.into());
        // Remove differently cased aliases before inserting the canonical
        // entry, so the actual config reader cannot override it later.
        for section in ["macros", "features"] {
            let object = selected[section].as_object_mut().ok_or("invalid configuration section")?;
            object.retain(|name, _| !name.eq_ignore_ascii_case(key));
            object.insert(if section == "features" { key.to_ascii_lowercase() } else { key.to_owned() }, value.clone());
        }
    }
    Ok(selected)
}
fn canonical_value(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut keys: Vec<_> = map.keys().collect();
            keys.sort();
            Value::Object(keys.into_iter().map(|key| (key.clone(), canonical_value(&map[key]))).collect())
        }
        Value::Array(values) => Value::Array(values.iter().map(canonical_value).collect()),
        _ => value.clone(),
    }
}
pub(crate) fn config_bytes(config: &Value) -> Result<Vec<u8>, String> {
    // The existing config readers are line-oriented. Canonical pretty JSON
    // is therefore the exact hashed/stored input, not compact JSON on disk.
    let mut bytes = serde_json::to_vec_pretty(&canonical_value(config)).map_err(|e| e.to_string())?;
    bytes.push(b'\n');
    Ok(bytes)
}
pub(crate) fn config_digest(config: &Value) -> Result<String, String> {
    Ok(format!("{:x}", Sha256::digest(config_bytes(config)?)))
}
pub(crate) fn abi_input_digest(config: &Value) -> Result<String, String> {
    let mut input = config.clone();
    for section in ["macros", "features", "feature_strings"] {
        if let Some(object) = input.get_mut(section).and_then(Value::as_object_mut) {
            object.retain(|key, _| !key.eq_ignore_ascii_case("FEMACS_RUNTIME_ABI_ID"));
        }
    }
    config_digest(&input)
}
fn no_symlink_components(path: &Path) -> Result<(), String> {
    let mut prefix = PathBuf::new();
    for component in path.components() {
        if matches!(component, Component::ParentDir | Component::CurDir) {
            return Err("runtime paths must not traverse parent/current components".into());
        }
        prefix.push(component.as_os_str());
        match fs::symlink_metadata(&prefix) {
            Ok(meta) if meta.file_type().is_symlink() => return Err("runtime path contains a symlink".into()),
            Ok(meta) if !meta.is_dir() && prefix != path => return Err("runtime parent is not a directory".into()),
            Ok(_) => {},
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {},
            Err(error) => return Err(error.to_string()),
        }
    }
    Ok(())
}
pub(crate) fn validate_output(repo: &Path, output: &Path, allow_existing: bool) -> Result<PathBuf, String> {
    let repo = if repo == Path::new(".") {
        std::env::current_dir().map_err(|e| e.to_string())?
    } else { repo.to_owned() };
    no_symlink_components(&repo)?;
    let repo = fs::canonicalize(&repo).map_err(|e| e.to_string())?;
    let output = if output.is_absolute() { output.to_owned() } else { repo.join(output) };
    no_symlink_components(&output)?;
    let owned = repo.join("build/runtime");
    if !output.starts_with(&owned) || output == owned {
        return Err("runtime output must be a distinct child of build/runtime".into());
    }
    if output.exists() && (!allow_existing || !output.is_dir()) {
        return Err("runtime output exists; only an explicit bound continuation can open it".into());
    }
    Ok(output)
}
pub(crate) fn validate_continuation(expected: &Value, actual: &Value) -> Result<(), String> {
    if expected["schema"] != 1 || actual["schema"] != 1 { return Err("unknown runtime manifest schema".into()); }
    for key in ["source_digest", "config_digest", "product", "variant", "target", "optimization", "native_abi", "backend_identity"] {
        let left = expected.get(key).ok_or_else(|| format!("missing expected continuation field {key}"))?;
        let right = actual.get(key).ok_or_else(|| format!("missing actual continuation field {key}"))?;
        if left != right || (left.is_null() && key != "backend_identity") {
            return Err(format!("runtime continuation identity mismatch: {key}"));
        }
    }
    if let Some(leg) = expected.get("compiler_leg") {
        if actual.get("compiler_leg") != Some(leg) { return Err("runtime compiler leg changed".into()); }
    }
    Ok(())
}

#[cfg(test)]
pub(crate) fn selected_object_name(stem: &str, checking: bool, swift: bool) -> String {
    if checking && !swift { format!("{stem}.checking.o") } else { format!("{stem}.o") }
}

#[derive(Clone, Debug)]
pub(crate) enum BackendDriver { Embedded }
#[derive(Clone, Debug)]
pub(crate) struct BackendTool {
    role: String, source: PathBuf, logical_id: String,
    sha256: Option<String>, version_receipt: Option<String>,
}
#[derive(Clone, Debug)]
pub(crate) struct BackendResource {
    source: PathBuf, destination: PathBuf, role: String, sha256: String, license: PathBuf,
}
#[derive(Clone, Debug)]
pub(crate) struct BackendDependency {
    logical_id: String, source: PathBuf, source_sha256: String, staged_sha256: Option<String>,
    destination: PathBuf, install_id: String, edges: Vec<String>, license: PathBuf,
}
#[derive(Clone, Debug)]
pub(crate) struct BackendIdentity {
    schema: u32, version: Option<[u32; 3]>, header_sha256: String, library_sha256: String,
    target: String, deployment_target: String, driver: BackendDriver,
    tools: Vec<BackendTool>, resources: Vec<BackendResource>, dependencies: Vec<BackendDependency>,
    abi_id: String, cache_key: String,
}
#[derive(Clone, Debug)]
pub(crate) struct RuntimeBuildConfig {
    source_digest: String, config_digest: String, config_file: PathBuf,
    backend_identity: Option<BackendIdentity>,
}
fn package_relative(path: &Path) -> bool {
    !path.as_os_str().is_empty() && path.components().all(|component| matches!(component, Component::Normal(_)))
}
fn sha256_text(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

pub(crate) fn source_digest(repo: &Path) -> Result<String, String> {
    // Link-owned passes REPO="." while preparation uses an absolute root.
    // Bind both to the actual root before constructing the frozen manifest
    // locator; traversal rejection applies to input names/output paths, not
    // the spelling of this already-selected repository argument.
    let repo = fs::canonicalize(repo).map_err(|e| e.to_string())?;
    let repo = repo.as_path();
    let top = Command::new("git").current_dir(repo)
        .args(["rev-parse", "--show-toplevel"]).output().map_err(|e| e.to_string())?;
    let own_git = top.status.success() &&
        fs::canonicalize(String::from_utf8_lossy(&top.stdout).trim()).ok()
            == Some(fs::canonicalize(repo).map_err(|e| e.to_string())?);
    let mut names: Vec<String> = if own_git {
        let output = Command::new("git").current_dir(repo)
            .args(["ls-files", "--cached", "--others", "--exclude-standard", "-z"])
            .output().map_err(|e| e.to_string())?;
        if !output.status.success() { return Err("cannot bind runtime source inventory".into()); }
        output.stdout.split(|byte| *byte == 0).filter(|name| !name.is_empty())
            .map(|name| std::str::from_utf8(name).map(str::to_owned)
                .map_err(|_| "non-UTF8 runtime source path".to_owned()))
            .collect::<Result<Vec<_>, _>>()?
    } else {
        let path = repo.join(".femacs-runtime-sources.json");
        no_symlink_components(&path)?;
        let meta = fs::metadata(&path).map_err(|_| "frozen runtime export needs an explicit source inventory")?;
        if meta.len() > 16 * 1024 * 1024 { return Err("oversized frozen source inventory".into()); }
        let manifest: Value = serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        if manifest["schema"] != 1 { return Err("unknown frozen source inventory schema".into()); }
        manifest["files"].as_array().ok_or("frozen source inventory has no files")?
            .iter().map(|name| name.as_str().map(str::to_owned)
                .ok_or_else(|| "invalid frozen source input name".to_owned()))
            .collect::<Result<Vec<_>, _>>()?
    };
    if names.is_empty() { return Err("runtime source inventory is empty".into()); }
    names.sort();
    if names.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err("duplicate runtime source input".into());
    }
    let mut digest = Sha256::new();
    let mut source_count = 0_usize;
    for name in names {
        let relative = Path::new(&name);
        if !package_relative(relative) { return Err("unsafe runtime source inventory path".into()); }
        // Optional reference and all owned outputs are never source inputs.
        if ["emacs", "build", "dist", ".femacs-cache", "target"].iter()
            .any(|root| relative.starts_with(root)) { continue; }
        source_count += 1;
        let path = repo.join(relative);
        let meta = fs::symlink_metadata(&path).map_err(|e| format!("source {name}: {e}"))?;
        if !meta.is_file() && !meta.file_type().is_symlink() {
            return Err(format!("unsupported runtime source input: {name}"));
        }
        digest.update(name.as_bytes()); digest.update([0]);
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            digest.update((meta.permissions().mode() & 0o777).to_le_bytes());
        }
        if meta.file_type().is_symlink() {
            digest.update(b"symlink\0");
            digest.update(fs::read_link(&path).map_err(|e| e.to_string())?.to_string_lossy().as_bytes());
        } else {
            digest.update(meta.len().to_le_bytes());
            let mut file = fs::File::open(&path).map_err(|e| e.to_string())?;
            let mut buffer = [0_u8; 65536];
            loop {
                let length = file.read(&mut buffer).map_err(|e| e.to_string())?;
                if length == 0 { break; }
                digest.update(&buffer[..length]);
            }
        }
        digest.update([0]);
    }
    if source_count == 0 { return Err("runtime inventory contains no source inputs".into()); }
    Ok(format!("{:x}", digest.finalize()))
}

fn read_binding(repo: &Path, root: &Path) -> Result<Option<Value>, String> {
    let path = root.join("runtime-binding.json");
    if !path.exists() {
        if root.starts_with(repo.join("build/runtime")) {
            return Err("selected runtime tree has no configuration binding".into());
        }
        return Ok(None);
    }
    validate_output(repo, root, true)?;
    no_symlink_components(&path)?;
    let metadata = fs::metadata(&path).map_err(|e| e.to_string())?;
    if metadata.len() > 4 * 1024 * 1024 { return Err("oversized runtime binding".into()); }
    let binding: Value = serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    if binding["schema"] != 1 { return Err("unknown runtime binding schema".into()); }
    Ok(Some(binding))
}
fn binding_path(root: &Path, binding: &Value, field: &str) -> Result<PathBuf, String> {
    let name = binding[field].as_str().ok_or_else(|| format!("missing runtime binding {field}"))?;
    let relative = Path::new(name);
    if !package_relative(relative) { return Err(format!("unsafe runtime binding {field}")); }
    let path = root.join(relative);
    no_symlink_components(&path)?;
    Ok(path)
}
pub(crate) fn verify_selected_root(repo: &Path, root: &Path) -> Result<Option<Value>, String> {
    let Some(binding) = read_binding(repo, root)? else { return Ok(None); };
    let configuration = binding_path(root, &binding, "config_relative")?;
    let config: Value = serde_json::from_slice(&fs::read(&configuration).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    let mut observed = binding.clone();
    observed["source_digest"] = Value::String(source_digest(repo)?);
    observed["config_digest"] = Value::String(config_digest(&config)?);
    let abi = config["macros"]["FEMACS_RUNTIME_ABI_ID"].as_str()
        .and_then(|text| text.strip_prefix('"')).and_then(|text| text.strip_suffix('"'))
        .ok_or("missing selected native ABI configuration identity")?;
    observed["native_abi"] = Value::String(abi.into());
    validate_continuation(&binding, &observed)?;
    let copied = match binding["product"].as_str() {
        Some("cocoa") => root.join("config/darwin-cocoa.json"),
        Some("terminal") => root.join("config/darwin-terminal.json"),
        _ => return Err("unknown selected runtime product".into()),
    };
    no_symlink_components(&copied)?;
    if !fs::symlink_metadata(&copied).map_err(|e| format!("missing selected target configuration: {e}"))?.is_file() {
        return Err("selected target configuration is not a regular file".into());
    }
    let copied: Value = serde_json::from_slice(&fs::read(copied).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    if config_digest(&copied)? != config_digest(&config)? { return Err("selected target continuation configuration changed".into()); }
    if let Some(dependencies) = binding["backend_identity"]["dependencies"].as_array() {
        for dependency in dependencies {
            let relative = dependency["destination"].as_str().ok_or("missing staged backend destination")?;
            if !package_relative(Path::new(relative)) { return Err("unsafe staged backend destination".into()); }
            let staged = root.join(relative);
            no_symlink_components(&staged)?;
            let expected = dependency["staged_sha256"].as_str().ok_or("missing staged backend identity")?;
            if crate::tooling::sha256_file(&staged)? != expected { return Err("staged native backend changed".into()); }
        }
        for resource in binding["backend_identity"]["resources"].as_array().ok_or("missing native resource closure")? {
            let name = resource["destination"].as_str().ok_or("missing native resource destination")?;
            if !package_relative(Path::new(name)) { return Err("unsafe native resource destination".into()); }
            let path = root.join(name);
            no_symlink_components(&path)?;
            if crate::tooling::sha256_file(&path)? != resource["sha256"].as_str().ok_or("missing native resource digest")? {
                return Err("staged native resource changed".into());
            }
        }
        let private: Value = serde_json::from_slice(&fs::read(root.join("backend-sources.json")).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        for tool in private["tools"].as_array().ok_or("missing native developer tool input identities")? {
            let source = Path::new(tool["source"].as_str().ok_or("missing developer tool source")?);
            let input = if tool["role"] == "sdk" { source.join("SDKSettings.json") } else { source.to_owned() };
            if crate::tooling::sha256_file(&input)? != tool["sha256"].as_str().ok_or("missing developer tool digest")? {
                return Err("native developer tool input changed".into());
            }
        }
    }
    Ok(Some(binding))
}
pub(crate) fn continuation_destination(repo: &Path, root: &Path) -> Result<Option<PathBuf>, String> {
    let Some(binding) = verify_selected_root(repo, root)? else { return Ok(None); };
    if binding["product"] != "cocoa" { return Err("not a Cocoa runtime continuation".into()); }
    Ok(Some(binding_path(root, &binding, "app_relative")?))
}
pub(crate) fn configured_child<'a>(repo: &Path, root: &Path, command: &'a mut Command) -> Result<&'a mut Command, String> {
    if verify_selected_root(repo, root)?.is_some() {
        let binding = read_binding(repo, root)?.ok_or("missing selected runtime binding")?;
        let leg = binding["compiler_leg"].as_str().ok_or("missing runtime compiler leg")?;
        if !matches!(leg, "normal" | "checking") { return Err("unknown runtime compiler leg".into()); }
        command.env("FEMACS_RUNTIME_BINDING", root.join("runtime-binding.json"))
            .env("FEMACS_LINK_CHECKING", if leg == "checking" { "1" } else { "0" })
            .env_remove("FEMACS_TARGET")
            .env("FEMACS_PGO", "off")
            .env_remove("FEMACS_TARGET_CPU").env_remove("FEMACS_DEBUGINFO")
            .env_remove("FEMACS_OVERFLOW_CHECKS")
            .env_remove("GCC_EXEC_PREFIX").env_remove("COMPILER_PATH")
            .env_remove("LIBRARY_PATH");
    } else { command.env_remove("FEMACS_RUNTIME_BINDING"); }
    Ok(command)
}
fn copy_new_or_same(source: &Path, target: &Path) -> Result<(), String> {
    no_symlink_components(target)?;
    if target.exists() {
        if crate::tooling::sha256_file(source)? == crate::tooling::sha256_file(target)? { return Ok(()); }
        return Err("runtime backend continuation file changed".into());
    }
    if let Some(parent) = target.parent() { fs::create_dir_all(parent).map_err(|e| e.to_string())?; }
    let mut input = fs::File::open(source).map_err(|e| e.to_string())?;
    let mut output = fs::OpenOptions::new().write(true).create_new(true).open(target).map_err(|e| e.to_string())?;
    std::io::copy(&mut input, &mut output).map_err(|e| e.to_string())?;
    output.flush().map_err(|e| e.to_string())?;
    fs::set_permissions(target, fs::metadata(source).map_err(|e| e.to_string())?.permissions()).map_err(|e| e.to_string())
}
fn compile_legs(binding: Option<&Value>) -> Result<(bool, bool), String> {
    match binding {
        None => Ok((true, true)),
        Some(value) => match value["compiler_leg"].as_str() {
            Some("normal") => Ok((true, false)),
            Some("checking") => Ok((false, true)),
            _ => Err("unknown or missing runtime compiler leg".into()),
        },
    }
}
pub(crate) fn unit_compile_legs(repo: &Path) -> Result<(bool, bool), String> {
    let Some(path) = std::env::var_os("FEMACS_RUNTIME_BINDING").map(PathBuf::from) else {
        return compile_legs(None);
    };
    if path.file_name().is_none_or(|name| name != "runtime-binding.json") {
        return Err("unknown runtime binding locator".into());
    }
    let root = path.parent().ok_or("runtime binding has no owner")?;
    let binding = verify_selected_root(repo, root)?.ok_or("missing runtime compile binding")?;
    compile_legs(Some(&binding))
}
fn select_product_objects(mut objects: Vec<String>, binding: Option<&Value>) -> Result<Vec<String>, String> {
    match binding {
        None => {},
        Some(value) => match value["variant"].as_str() {
            Some("native" | "native-collector") => {
                if !objects.iter().any(|name| name == "dynlib.o") {
                    objects.push("dynlib.o".into());
                }
            },
            Some("baseline" | "collector") => {},
            _ => return Err("unknown or missing runtime product variant".into()),
        },
    }
    Ok(objects)
}
pub(crate) fn product_objects(repo: &Path, objects: Vec<String>) -> Result<Vec<String>, String> {
    let Some(path) = std::env::var_os("FEMACS_RUNTIME_BINDING").map(PathBuf::from) else {
        return select_product_objects(objects, None);
    };
    if path.file_name().is_none_or(|name| name != "runtime-binding.json") {
        return Err("unknown runtime binding locator".into());
    }
    let root = path.parent().ok_or("runtime binding has no owner")?;
    let binding = verify_selected_root(repo, root)?.ok_or("missing runtime product binding")?;
    select_product_objects(objects, Some(&binding))
}
pub(crate) fn link_flags(repo: &Path) -> Result<Vec<String>, String> {
    let Some(path) = std::env::var_os("FEMACS_RUNTIME_BINDING").map(PathBuf::from) else { return Ok(Vec::new()); };
    let root = path.parent().ok_or("runtime binding has no owner")?;
    if path.file_name().is_none_or(|name| name != "runtime-binding.json") { return Err("unknown runtime binding locator".into()); }
    let binding = verify_selected_root(repo, root)?.ok_or("missing runtime link binding")?;
    if binding["backend_identity"].is_null() { return Ok(Vec::new()); }
    let dependencies = binding["backend_identity"]["dependencies"].as_array().ok_or("missing native dependency closure")?;
    backend_link_flags(root, dependencies)
}
fn backend_link_flags(root: &Path, dependencies: &[Value]) -> Result<Vec<String>, String> {
    let mut library = None;
    for dependency in dependencies {
        let destination = Path::new(dependency["destination"].as_str().ok_or("missing native dependency path")?);
        if !package_relative(destination) { return Err("unsafe native dependency path".into()); }
        let source = root.join(destination);
        let name = source.file_name().ok_or("native dependency has no name")?;
        copy_new_or_same(&source, &root.join("port/src/runtime-native").join(name))?;
        // Capture uses package-relative logical IDs, including runtime-native/.
        if dependency["logical_id"].as_str() == Some("runtime-native/libgccjit.0.dylib") { library = Some(source); }
    }
    let library = library.ok_or("native closure has no libgccjit.0.dylib")?;
    Ok(vec![library.display().to_string(),
        "-Wl,-rpath,@executable_path/runtime-native".into(),
        "-Wl,-rpath,@executable_path/../Frameworks/runtime-native".into()])
}

fn portable_backend(identity: &BackendIdentity) -> Value {
    serde_json::json!({
        "schema": identity.schema, "version": identity.version,
        "header_sha256": identity.header_sha256, "library_sha256": identity.library_sha256,
        "target": identity.target, "deployment_target": identity.deployment_target,
        "driver": "embedded", "abi_id": identity.abi_id, "cache_key": identity.cache_key,
        "tools": identity.tools.iter().map(|tool| serde_json::json!({
            "role": tool.role, "logical_id": tool.logical_id, "sha256": tool.sha256,
            "version_receipt": tool.version_receipt })).collect::<Vec<_>>(),
        "resources": identity.resources.iter().map(|resource| serde_json::json!({
            "destination": resource.destination, "role": resource.role,
            "sha256": resource.sha256, "license": resource.license })).collect::<Vec<_>>(),
        "dependencies": identity.dependencies.iter().map(|dependency| serde_json::json!({
            "logical_id": dependency.logical_id, "source_sha256": dependency.source_sha256,
            "staged_sha256": dependency.staged_sha256, "destination": dependency.destination,
            "install_id": dependency.install_id, "edges": dependency.edges,
            "license": dependency.license })).collect::<Vec<_>>()
    })
}

pub(crate) fn stage_tls_dependencies(
    repo: &Path, root: &Path, header: &Path, library: &Path, deployment: &str,
    expected_abi: Option<&str>,
) -> Result<Value, String> {
    let mut identity = backend::capture_library_runtime(header, library, deployment)?;
    if expected_abi.is_some_and(|expected| expected != identity.abi_id) {
        return Err("TLS dependency closure changed after precompile binding".into());
    }
    backend::stage_tls_backend(repo, root, &mut identity)?;
    let sources = serde_json::json!({
        "libraries": identity.dependencies.iter().map(|item| serde_json::json!({
            "logical_id":item.logical_id, "source":item.source,
            "sha256":item.source_sha256 })).collect::<Vec<_>>(),
        "resources": identity.resources.iter().map(|item| serde_json::json!({
            "destination":item.destination, "source":item.source,
            "sha256":item.sha256 })).collect::<Vec<_>>()
    });
    crate::tooling::write_json(&root.join("sources.json"), &sources)?;
    let portable = portable_backend(&identity);
    crate::tooling::write_json(&root.join("identity.json"), &portable)?;
    Ok(portable)
}

pub(crate) fn capture_tls_dependency_identity(
    header: &Path, library: &Path, deployment: &str,
) -> Result<Value, String> {
    Ok(portable_backend(&backend::capture_library_runtime(header, library, deployment)?))
}
fn write_new(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let mut file = fs::OpenOptions::new().write(true).create_new(true).open(path).map_err(|e| e.to_string())?;
    file.write_all(bytes).map_err(|e| e.to_string())
}

pub(crate) fn terminal_paths_source(repo: &Path, root: &Path) -> String {
    let mut code = String::from(r#"// Generated candidate-only terminal resource paths.
mod runtime_terminal_paths {
use super::*;
use core::sync::atomic::{AtomicU8, Ordering};
static STATES: [AtomicU8; 8] = [const { AtomicU8::new(0) }; 8];
static mut BUFFERS: [[c_char; 4096]; 8] = [[0; 4096]; 8];
unsafe extern "C" {
    #[link_name="_NSGetExecutablePath"] fn executable_path(buffer: *mut c_char, size: *mut u32) -> c_int;
    #[link_name="realpath"] fn canonical_path(path: *const c_char, output: *mut c_char) -> *mut c_char;
    #[link_name="abort"] fn resource_path_failure() -> !;
}
unsafe fn live_path(slot: usize, suffix: &[u8]) -> *const c_char {
    // SAFETY: Initializes bounded 4096-byte static path slot `slot < 8` under atomic CAS state machine.
    unsafe {
        if STATES[slot].load(Ordering::Acquire) != 2 {
            if STATES[slot].compare_exchange(0, 1, Ordering::Acquire, Ordering::Acquire).is_ok() {
                let mut executable = [0 as c_char; 4096];
                let mut resolved = [0 as c_char; 4096];
                let mut size = 4096_u32;
                if executable_path(executable.as_mut_ptr(), &raw mut size) != 0
                    || canonical_path(executable.as_ptr(), resolved.as_mut_ptr()).is_null() { resource_path_failure(); }
                let length = resolved.iter().position(|value| *value == 0).unwrap_or(4096);
                let Some(slash) = resolved[..length].iter().rposition(|value| *value == b'/' as c_char) else { resource_path_failure(); };
                if slash + 1 + suffix.len() >= 4096 { resource_path_failure(); }
                let output = (&raw mut BUFFERS).cast::<[c_char; 4096]>().add(slot).cast::<c_char>();
                core::ptr::copy_nonoverlapping(resolved.as_ptr(), output, slash + 1);
                for (index, byte) in suffix.iter().enumerate() { *output.add(slash + 1 + index) = *byte as c_char; }
                *output.add(slash + 1 + suffix.len()) = 0;
                STATES[slot].store(2, Ordering::Release);
            } else {
                while STATES[slot].load(Ordering::Acquire) != 2 { core::hint::spin_loop(); }
            }
        }
        (&raw const BUFFERS).cast::<[c_char; 4096]>().add(slot).cast::<c_char>()
    }
}
"#);
    for (slot, name, relative, original) in [
        (0, "PATH_LOADSEARCH", "../lisp", repo.join("lisp")),
        (1, "PATH_DUMPLOADSEARCH", "../lisp", repo.join("lisp")),
        (2, "PATH_DUMPLOADSEARCH_SLASH", "../lisp/", repo.join("lisp")),
        (3, "PATH_EXEC", "../lib-src", root.join("port/lib-src")),
        (4, "PATH_DATA", "../etc", repo.join("etc")),
        (5, "PATH_DOC", "../etc", repo.join("etc")),
        (6, "PATH_INFO", "../info", root.join("runtime-info")),
        (7, "PATH_BITMAPS", "../etc/images", repo.join("etc/images")),
    ] {
        let owned = if name.starts_with("PATH_LOADSEARCH") || name.starts_with("PATH_DUMPLOADSEARCH") {
            crate::standalone::lisp_paths(repo, name == "PATH_DUMPLOADSEARCH_SLASH")
        } else { original.display().to_string() };
        code.push_str(&format!("#[inline(always)] pub fn {name}() -> *const c_char {{ if will_dump_p() {{ c{owned:?}.as_ptr() }} else {{ /* SAFETY: Relocates runtime-relative resource slice into static path slot. */ unsafe {{ live_path({slot}, b{relative:?}) }} }} }}\n"));
    }
    code.push_str("pub fn PATH_SITELOADSEARCH() -> *const c_char { c\"\".as_ptr() }\npub fn PATH_GAME() -> *const c_char { core::ptr::null() }\n}\npub use runtime_terminal_paths::*;\n");
    code
}

pub(crate) fn package_terminal(repo: &Path, root: &Path, info: &Path) -> Result<(), String> {
    if !root.join("port/lib-src/emacsclient").is_file() {
        return Err("missing terminal companion client".into());
    }
    let destination = root.join("dist/terminal");
    if destination.exists() { return Err("terminal candidate package already exists".into()); }
    fs::create_dir_all(destination.join("src")).map_err(|e| e.to_string())?;
    for name in ["emacs", "emacs.pdmp", "temacs", "bootstrap-emacs.pdmp"] {
        copy_new_or_same(&root.join("port/src").join(name), &destination.join("src").join(name))?;
    }
    for name in ["lisp", "etc"] {
        crate::macos::copy_tree(&repo.join(name), &destination.join(name))?;
    }
    crate::standalone::install_library_corrections(repo, &destination.join("lisp"))?;
    crate::macos::copy_tree(&root.join("port/lib-src"), &destination.join("lib-src"))?;
    crate::macos::copy_tree(&root.join("port/include"), &destination.join("include"))?;
    crate::manuals::copy_set(info, &destination.join("info"))?;
    if root.join("runtime-native").is_dir() {
        crate::macos::copy_tree(&root.join("runtime-native"), &destination.join("src/runtime-native"))?;
        crate::macos::copy_tree(&root.join("licenses"), &destination.join("licenses"))?;
    }
    crate::tls::copy_product_runtime(root, &destination.join("runtime-tls"),
                                     &destination.join("licenses/tls"))?;
    crate::lcms::copy_product_runtime(root, &destination.join("runtime-lcms"),
                                     &destination.join("licenses/lcms"))?;
    crate::image::copy_product_runtime(root, &destination.join("runtime-image"),
                                      &destination.join("licenses/image"))?;
    crate::treesit::copy_product_runtime(root, &destination.join("runtime-treesit"),
                                       &destination.join("licenses/treesit"))?;
    Ok(())
}

// Receipts traverse concrete files, never follow a resource symlink into a
// different checkout. Link text and file permissions are identity inputs too.
fn tree_identity(tree: &Path) -> Result<Value, String> {
    fn visit(tree: &Path, directory: &Path, files: &mut Map<String, Value>) -> Result<(), String> {
        let mut entries = fs::read_dir(directory).map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?;
        entries.sort_by_key(|entry| entry.file_name());
        for entry in entries {
            let path = entry.path();
            let relative = path.strip_prefix(tree).map_err(|e| e.to_string())?
                .to_str().ok_or("non-UTF8 resource name")?.to_owned();
            let meta = fs::symlink_metadata(&path).map_err(|e| e.to_string())?;
            if meta.is_dir() { visit(tree, &path, files)?; continue; }
            let mut row = if meta.file_type().is_symlink() {
                let target = fs::read_link(&path).map_err(|e| e.to_string())?;
                serde_json::json!({"kind":"symlink", "target":target.to_str().ok_or("non-UTF8 link")?})
            } else if meta.is_file() {
                serde_json::json!({"kind":"file", "bytes":meta.len(),
                    "sha256":crate::tooling::sha256_file(&path)?})
            } else { return Err(format!("unsupported resource input: {relative}")); };
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                row["mode"] = Value::from(meta.permissions().mode() & 0o777);
            }
            files.insert(relative, row);
        }
        Ok(())
    }
    no_symlink_components(tree)?;
    if !tree.is_dir() { return Err("missing product resource tree".into()); }
    let mut files = Map::new();
    visit(tree, tree, &mut files)?;
    if files.is_empty() { return Err("empty product resource tree".into()); }
    Ok(Value::Object(files))
}

fn regular_identity(path: &Path) -> Result<Value, String> {
    no_symlink_components(path)?;
    let meta = fs::symlink_metadata(path).map_err(|e| e.to_string())?;
    if !meta.is_file() { return Err("selected runtime identity requires a regular file".into()); }
    let mut value = serde_json::json!({"kind":"file", "bytes":meta.len(),
        "sha256":crate::tooling::sha256_file(path)?});
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        value["mode"] = Value::from(meta.permissions().mode() & 0o777);
    }
    #[cfg(not(unix))]
    return Err("runtime permission identity requires a supported Unix host".into());
    Ok(value)
}

pub(crate) fn record_link_inputs(repo: &Path, objects: &[String], members: &[String], archive: &Path) -> Result<(), String> {
    let Some(path) = std::env::var_os("FEMACS_RUNTIME_BINDING").map(PathBuf::from) else { return Ok(()); };
    let root = path.parent().ok_or("runtime link has no owned root")?;
    let binding = verify_selected_root(repo, root)?.ok_or("runtime link missing binding")?;
    let mut inputs = Map::new();
    if objects.is_empty() {
        return Err("runtime product-object list is empty".into());
    }
    for (role, names) in [("product-object", objects), ("archive-member", members)] {
        for name in names {
            let input = Path::new(name);
            no_symlink_components(input)?;
            let relative = input.strip_prefix(root).map_err(|_| "link input escaped selected output")?;
            let key = relative.to_str().ok_or("non-UTF8 link input")?;
            let filename = input.file_name().and_then(|v| v.to_str()).ok_or("missing object filename")?;
            let is_swift = ["macfont.o", "nsfns.o", "nsimage.o", "nsmenu.o", "nsselect.o", "nsterm.o"].contains(&filename);
            if binding["compiler_leg"] == "checking"
                && !is_swift
                && !filename.ends_with(".checking.o")
                && !filename.ends_with(".checking.rlib")
            {
                return Err(format!("unchecked runtime object on checking leg: {key}"));
            }
            let mut identity = regular_identity(input)?;
            identity["role"] = Value::String(role.into());
            inputs.insert(key.into(), identity);
        }
    }
    let relative = archive.strip_prefix(root).map_err(|_| "archive escaped selected output")?;
    let mut archive_identity = regular_identity(archive)?;
    archive_identity["path"] = serde_json::json!(relative);
    crate::tooling::write_json(&root.join("runtime-link-inputs.json"), &serde_json::json!({
        "schema":1, "compiler_leg":binding["compiler_leg"], "objects":inputs,
        "archive":archive_identity,
        "link_complete":false
    }))
}
fn prepare_leg(repo: &Path, opts: &RuntimeBuildOptions, root: &Path, leg: &str) -> Result<RuntimeBuildConfig, String> {
    let source = source_digest(repo)?;
    let base_path = match opts.product {
        RuntimeProduct::Terminal => "build-support/macos/terminal/config.json",
        RuntimeProduct::Cocoa => "build-support/macos/cocoa/config.json",
    };
    let base: Value = serde_json::from_slice(&fs::read(repo.join(base_path)).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    let mut selected = canonical_config(&base, opts)?;
    selected["runtime_compiler_leg"] = Value::String(leg.into());
    crate::tls::bind_configuration(&mut selected)?;
    crate::lcms::bind_configuration(&mut selected)?;
    crate::treesit::bind_configuration(&mut selected)?;
    let target = "aarch64-apple-darwin";
    let deployment = crate::macos::deployment_target()?;
    let mut backend_identity = None;
    if let Some(prefix) = &opts.backend_prefix {
        let mut identity = backend::capture_backend(prefix, target, &deployment)?;
        let header = fs::read(prefix.join("include/libgccjit.h")).map_err(|e| e.to_string())?;
        if format!("{:x}", Sha256::digest(&header)) != identity.header_sha256 {
            return Err("native header changed during preparation".into());
        }
        let header = std::str::from_utf8(&header).map_err(|_| "native header is not UTF8")?;
        for line in header.lines() {
            let mut words = line.split_whitespace();
            if words.next() != Some("#define") { continue; }
            let Some(name) = words.next().filter(|name| name.starts_with("LIBGCCJIT_HAVE_")) else { continue; };
            let key = name.to_ascii_uppercase();
            selected["macros"][&key] = Value::String("1".into());
            selected["features"][&key.to_ascii_lowercase()] = Value::String("1".into());
        }
        for (role, key) in [("sdk", "FEMACS_NATIVE_SDKROOT"), ("assembler", "FEMACS_NATIVE_ASSEMBLER_DIR")] {
            let tool = identity.tools.iter().find(|tool| tool.role == role)
                .ok_or_else(|| format!("missing native developer tool: {role}"))?;
            let source = if role == "sdk" { tool.source.as_path() }
                else { tool.source.parent().ok_or("assembler has no directory")? };
            let text = source.to_str().ok_or("developer tool path is not UTF8")?;
            if text.contains(',') || text.contains('\n') { return Err("developer tool path cannot be represented in driver options".into()); }
            selected["macros"][key] = Value::String(serde_json::to_string(text).map_err(|e| e.to_string())?);
        }
        backend::stage_backend(repo, root, &mut identity)?;
        // Exact local input locators are private build evidence, not portable
        // package identity or repository documentation.
        crate::tooling::write_json(&root.join("backend-sources.json"), &serde_json::json!({
            "libraries": identity.dependencies.iter().map(|item| serde_json::json!({
                "logical_id": item.logical_id, "source": item.source, "sha256": item.source_sha256 })).collect::<Vec<_>>(),
            "tools": identity.tools.iter().map(|item| serde_json::json!({
                "role":item.role, "logical_id":item.logical_id, "source":item.source, "sha256":item.sha256 })).collect::<Vec<_>>(),
            "resources": identity.resources.iter().map(|item| serde_json::json!({
                "destination": item.destination, "source": item.source, "sha256": item.sha256 })).collect::<Vec<_>>()
        }))?;
        backend_identity = Some(identity);
    }
    let backend_value = backend_identity.as_ref().map(portable_backend).unwrap_or(Value::Null);
    let mut abi = Sha256::new();
    abi.update(source.as_bytes()); abi.update([0]);
    abi.update(abi_input_digest(&selected)?.as_bytes()); abi.update([0]);
    abi.update(target.as_bytes()); abi.update([0]);
    abi.update(serde_json::to_vec(&backend_value).map_err(|e| e.to_string())?);
    let native_abi = format!("{:x}", abi.finalize());
    selected["macros"]["FEMACS_RUNTIME_ABI_ID"] = Value::String(format!("\"{native_abi}\""));
    let config_file = root.join("runtime-config.json");
    write_new(&config_file, &config_bytes(&selected)?)?;
    let digest = config_digest(&selected)?;
    let product = if opts.product == RuntimeProduct::Terminal { "terminal" } else { "cocoa" };
    let variant = match opts.variant {
        RuntimeVariant::Baseline => "baseline", RuntimeVariant::Native => "native",
        RuntimeVariant::Collector => "collector", RuntimeVariant::NativeCollector => "native-collector",
    };
    let binding = serde_json::json!({
        "schema":1, "source_digest":source, "config_digest":digest,
        "product":product, "variant":variant, "compiler_leg":leg,
        "observer":opts.observer, "test_hooks":opts.test_hooks,
        "target":target, "deployment_target":deployment, "optimization":"2",
        "native_abi":native_abi, "backend_identity":backend_value,
        "config_relative":"runtime-config.json", "app_relative":"dist/Femacs.app"
    });
    crate::tooling::write_json(&root.join("runtime-inputs.json"), &serde_json::json!({
        "schema":1, "source_root":repo,
        "native_cache":root.join("build-native-cache"),
        "backend_prefix":opts.backend_prefix.as_ref().map(|prefix| fs::canonicalize(prefix))
            .transpose().map_err(|e| e.to_string())?
    }))?;
    crate::tooling::write_json(&root.join("runtime-binding.json"), &binding)?;
    Ok(RuntimeBuildConfig { source_digest: source, config_digest: digest, config_file, backend_identity })
}
fn record_product(repo: &Path, root: &Path, product: RuntimeProduct) -> Result<(), String> {
    let binding = verify_selected_root(repo, root)?.ok_or("missing completed product binding")?;
    let mut paths = vec!["port/src/temacs", "port/src/emacs", "port/src/emacs.pdmp", "port/src/bootstrap-emacs.pdmp", "port/lib-src/emacsclient"];
    if product == RuntimeProduct::Cocoa {
        paths.extend(["dist/Femacs.app/Contents/MacOS/Femacs", "dist/Femacs.app/Contents/Resources/libexec/Femacs.pdmp", "dist/Femacs.app/Contents/MacOS/bin/emacsclient"]);
    } else {
        paths.extend(["dist/terminal/src/emacs", "dist/terminal/src/emacs.pdmp", "dist/terminal/lib-src/emacsclient"]);
    }
    let mut files = Map::new();
    let mut file_identities = Map::new();
    for path in paths {
        let identity = regular_identity(&root.join(path))?;
        files.insert(path.into(), identity["sha256"].clone());
        file_identities.insert(path.into(), identity);
    }
    let mut link: Value = serde_json::from_slice(&fs::read(root.join("runtime-link-inputs.json")).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    for (relative, expected) in link["objects"].as_object().ok_or("link receipt has no objects")? {
        if !package_relative(Path::new(relative)) || crate::tooling::sha256_file(&root.join(relative))? != expected["sha256"].as_str().ok_or("missing link object digest")? {
            return Err("runtime link input changed before product receipt".into());
        }
    }
    let archive = link["archive"]["path"].as_str().ok_or("link receipt has no archive path")?;
    if !package_relative(Path::new(archive)) || crate::tooling::sha256_file(&root.join(archive))?
        != link["archive"]["sha256"].as_str().ok_or("link receipt has no archive hash")? {
        return Err("runtime archive changed before product receipt".into());
    }
    link["link_complete"] = Value::Bool(true);
    let mut resources = Map::new();
    resources.insert("generated-config".into(), tree_identity(&root.join("config"))?);
    if product == RuntimeProduct::Cocoa {
        resources.insert("bundle".into(), tree_identity(&root.join("dist/Femacs.app"))?);
    } else {
        resources.insert("terminal-package".into(), tree_identity(&root.join("dist/terminal"))?);
        let pointer: Value = serde_json::from_slice(&fs::read(root.join("config/manual-set.json")).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        let info = Path::new(pointer["info"].as_str().ok_or("missing generated manual root")?);
        crate::tooling::guard_scratch(repo, info)?;
        resources.insert("info".into(), tree_identity(info)?);
    }
    for name in ["runtime-native", "licenses"] {
        if root.join(name).is_dir() { resources.insert(name.into(), tree_identity(&root.join(name))?); }
    }
    if root.join("tls").is_dir() {
        resources.insert("tls".into(), tree_identity(&root.join("tls"))?);
    }
    if root.join("lcms").is_dir() {
        resources.insert("lcms".into(), tree_identity(&root.join("lcms"))?);
    }
    if root.join("image").is_dir() {
        resources.insert("image".into(), tree_identity(&root.join("image"))?);
    }
    if root.join("treesit").is_dir() {
        resources.insert("treesit".into(), tree_identity(&root.join("treesit"))?);
    }
    crate::tooling::write_json(&root.join("runtime-product.json"), &serde_json::json!({
        "schema":1, "identity_version":3, "binding":binding, "files":files,
        "file_identities":file_identities,
        "link_inputs":link, "resources":resources,
        "build_complete":true, "semantic_tests_executed":false,
        "compatibility_accepted":false, "performance_accepted":false
    }))
}
pub(crate) fn command(args: &[String]) -> i32 {
    let result = (|| -> Result<(), String> {
        crate::standalone::check_host()?;
        let repo = fs::canonicalize(".").map_err(|e| e.to_string())?;
        let opts = parse_options(args)?;
        let out = validate_output(&repo, &opts.out, false)?;
        fs::create_dir_all(out.parent().ok_or("runtime output has no parent")?).map_err(|e| e.to_string())?;
        no_symlink_components(&out)?;
        fs::create_dir(&out).map_err(|e| e.to_string())?;
        let mut failures = Vec::new();
        let original_home = std::env::var_os("HOME").map(PathBuf::from).ok_or("missing developer home input")?;
        let rustup_home = std::env::var_os("RUSTUP_HOME").map(PathBuf::from).unwrap_or_else(|| original_home.join(".rustup"));
        let cargo_home = std::env::var_os("CARGO_HOME").map(PathBuf::from).unwrap_or_else(|| original_home.join(".cargo"));
        for leg in ["normal", "checking"] {
            let root = if leg == "normal" { out.clone() } else { out.join("checking") };
            if leg != "normal" { fs::create_dir(&root).map_err(|e| e.to_string())?; }
            // This CLI owns its environment and starts no concurrent worker
            // threads here. Isolate editor/resource writes while preserving
            // the explicitly selected developer toolchain installation.
            for (key, name) in [("HOME", "build-home"), ("TMPDIR", "build-tmp"),
                                ("XDG_CACHE_HOME", "build-cache"),
                                ("EMACSNATIVELOADPATH", "build-native-cache"),
                                ("FEMACS_RUNTIME_NATIVE_CACHE", "build-native-cache")] {
                let path = root.join(name);
                fs::create_dir_all(&path).map_err(|e| e.to_string())?;
                /* SAFETY: Serialized test environment guard mutates process env vars without concurrent readers. */ unsafe { std::env::set_var(key, path); }
            }
            /* SAFETY: Serialized test environment guard mutates process env vars without concurrent readers. */ unsafe {
                std::env::set_var("RUSTUP_HOME", &rustup_home);
                std::env::set_var("CARGO_HOME", &cargo_home);
            }
            let preservation_required = std::env::var_os("FEMACS_RUNTIME_ARTIFACT_JOURNAL").is_some();
            let mut preservation_complete = false;
            let attempt = (|| -> Result<(), String> {
                let config = prepare_leg(&repo, &opts, &root, leg)?;
                match opts.product {
                    RuntimeProduct::Terminal => crate::standalone::terminal_configured(&repo, &root, false, &config.config_file)?,
                    RuntimeProduct::Cocoa => crate::macos::build_configured(&repo, &root, &config.config_file, &root.join("dist/Femacs.app"))?,
                }
                record_product(&repo, &root, opts.product)?;
                if let Some(journal) = std::env::var_os("FEMACS_RUNTIME_ARTIFACT_JOURNAL") {
                    let python = std::env::var_os("FEMACS_RUNTIME_HELPER_PYTHON")
                        .ok_or("selected artifact owner requires its Python interpreter")?;
                    let status = Command::new(python).arg(repo.join("tools/runtime-owner.py"))
                        .arg("--journal").arg(journal)
                        .arg("--product-receipt").arg(root.join("runtime-product.json"))
                        .arg("--source").arg(&repo)
                        .status().map_err(|error| error.to_string())?;
                    if !status.success() { return Err("completed compiler leg preservation failed".into()); }
                    preservation_complete = true;
                }
                Ok(())
            })();
            crate::tooling::write_json(&root.join("runtime-build-outcome.json"), &serde_json::json!({
                "schema":1, "compiler_leg":leg, "build_complete":attempt.is_ok(),
                "error":attempt.as_ref().err(), "tests_executed":false,
                "artifact_preservation_required":preservation_required,
                "artifact_preservation_complete":preservation_complete
            }))?;
            if let Err(error) = attempt {
                if preservation_required && !preservation_complete && root.join("runtime-product.json").is_file() {
                    return Err(format!("{leg}: {error}"));
                }
                failures.push(format!("{leg}: {error}"));
            }
        }
        if !failures.is_empty() { return Err(failures.join("; ")); }
        println!("runtime candidate built under {}; acceptance has not run", out.display());
        Ok(())
    })();
    match result { Ok(()) => 0, Err(error) => { eprintln!("runtime-build: {error}"); 1 } }
}
pub(crate) fn validate_dependencies(dependencies: &[BackendDependency]) -> Result<(), String> {
    let mut names = BTreeSet::new();
    let mut destinations = BTreeSet::new();
    for dependency in dependencies {
        if !package_relative(Path::new(&dependency.logical_id))
            || !names.insert(dependency.logical_id.clone())
            || !package_relative(&dependency.destination)
            || !destinations.insert(dependency.destination.clone())
            || !package_relative(&dependency.license)
            || !sha256_text(&dependency.source_sha256)
            || dependency.staged_sha256.as_ref().is_some_and(|hash| !sha256_text(hash)) {
            return Err("invalid or duplicate backend dependency identity/destination/license".into());
        }
    }
    for dependency in dependencies {
        for edge in &dependency.edges {
            if let Some(system) = edge.strip_prefix("system:") {
                if system.is_empty() { return Err("empty backend system dependency".into()); }
            } else if !names.contains(edge) { return Err(format!("unresolved backend dependency: {edge}")); }
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "runtime_tests.rs"]
mod tests;
