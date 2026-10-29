//! Owned TLS target configuration and relocatable library/license delivery.
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;

fn digest(path: &Path) -> Result<String, String> {
    let mut input = fs::File::open(path).map_err(|e| e.to_string())?;
    let mut hash = Sha256::new();
    let mut buffer = [0u8; 65536];
    loop {
        let n = input.read(&mut buffer).map_err(|e| e.to_string())?;
        if n == 0 { break; }
        hash.update(&buffer[..n]);
    }
    Ok(format!("{:x}", hash.finalize()))
}

fn enabled(config: &Value) -> Result<bool, String> {
    let mut found = None;
    for section in ["macros", "features"] {
        if let Some(values) = config[section].as_object() {
            for (name, value) in values {
                if !name.eq_ignore_ascii_case("HAVE_GNUTLS") { continue; }
                let truth = match (value.as_str(), value.as_i64(), value.as_bool()) {
                    (Some("1" | "true"), _, _) | (_, Some(1), _) | (_, _, Some(true)) => true,
                    (Some("0" | "false" | "undefined" | "absent"), _, _) | (_, Some(0), _) | (_, _, Some(false)) => false,
                    _ => return Err("invalid HAVE_GNUTLS setting".into()),
                };
                if found.is_some_and(|prior| prior != truth) {
                    return Err("conflicting HAVE_GNUTLS settings".into());
                }
                found = Some(truth);
            }
        }
    }
    Ok(found.unwrap_or(false))
}

fn prefix() -> Result<PathBuf, String> {
    let path = if let Some(value) = std::env::var_os("FEMACS_GNUTLS_PREFIX") {
        PathBuf::from(value)
    } else {
        let output = Command::new("pkg-config").args(["--variable=prefix", "gnutls"])
            .output().map_err(|e| format!("TLS requires installed GnuTLS headers/libraries and pkg-config: {e}"))?;
        if !output.status.success() || output.stdout.len() > 16384 {
            return Err("pkg-config could not locate installed GnuTLS; install the dependency or set FEMACS_GNUTLS_PREFIX".into());
        }
        let text = std::str::from_utf8(&output.stdout).map_err(|e| e.to_string())?.trim();
        if text.is_empty() || text.lines().count() != 1 { return Err("invalid GnuTLS prefix".into()); }
        PathBuf::from(text)
    };
    fs::canonicalize(path).map_err(|e| format!("GnuTLS prefix: {e}"))
}

fn integer_define(text: &str, name: &str) -> Result<u32, String> {
    let mut values = Vec::new();
    for line in text.lines() {
        let words: Vec<_> = line.split_whitespace().collect();
        if words.len() >= 3 && words[0] == "#define" && words[1] == name {
            let value = words[2];
            let value = if let Some(hex) = value.strip_prefix("0x") {
                u32::from_str_radix(hex, 16)
            } else { value.parse() }.map_err(|_| format!("nonliteral GnuTLS version macro {name}"))?;
            values.push(value);
        }
    }
    if values.len() != 1 { return Err(format!("expected one actual header macro {name}")); }
    Ok(values[0])
}

fn header_identities(prefix: &Path) -> Result<Vec<(String, String)>, String> {
    let root = prefix.join("include/gnutls");
    let mut pending = vec![root.clone()];
    let mut files = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(&directory).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let kind = entry.file_type().map_err(|e| e.to_string())?;
            if kind.is_symlink() { return Err("TLS header tree contains a symlink".into()); }
            if kind.is_dir() { pending.push(entry.path()); }
            else if kind.is_file() && entry.path().extension().is_some_and(|s| s == "h") {
                files.push(entry.path());
            }
        }
    }
    files.sort();
    for name in ["gnutls.h", "x509.h", "crypto.h", "compat.h"] {
        if !files.contains(&root.join(name)) { return Err(format!("missing TLS public header {name}")); }
    }
    files.into_iter().map(|path| {
        Ok((path.strip_prefix(&root).map_err(|e| e.to_string())?
            .to_str().ok_or("TLS header name is not UTF8")?.to_owned(), digest(&path)?))
    }).collect()
}

fn headers_abi(headers: &[(String, String)]) -> Result<String, String> {
    Ok(format!("{:x}", Sha256::digest(serde_json::to_vec(headers).map_err(|e| e.to_string())?)))
}

fn configure_header_features(config: &mut Value, prefix: &Path) -> Result<(), String> {
    let header = prefix.join("include/gnutls/gnutls.h");
    let text = fs::read_to_string(&header).map_err(|e| e.to_string())?;
    let major = integer_define(&text, "GNUTLS_VERSION_MAJOR")?;
    let minor = integer_define(&text, "GNUTLS_VERSION_MINOR")?;
    let patch = integer_define(&text, "GNUTLS_VERSION_PATCH")?;
    let version = integer_define(&text, "GNUTLS_VERSION_NUMBER")?;
    if major > 255 || minor > 255 || patch > 255 || version != (major << 16 | minor << 8 | patch) {
        return Err("inconsistent installed GnuTLS version macros".into());
    }
    if version < 0x030000 { return Err("owned macOS TLS integration requires GnuTLS 3 or later".into()); }
    let x509 = fs::read_to_string(prefix.join("include/gnutls/x509.h")).map_err(|e| e.to_string())?;
    let flags = [
        ("HAVE_GNUTLS3", true),
        ("HAVE_GNUTLS_X509_SYSTEM_TRUST", version >= 0x030014),
        ("HAVE_GNUTLS_CERTIFICATE_SET_X509_KEY_FILE2", version >= 0x030200),
        ("HAVE_GNUTLS_CIPHER_GET_IV_SIZE", version >= 0x030200),
        ("HAVE_GNUTLS_CIPHER_GET_TAG_SIZE", version >= 0x030202),
        ("HAVE_GNUTLS_DIGEST_LIST", version >= 0x030202),
        ("HAVE_GNUTLS_EXT__DUMBFW", version >= 0x030205),
        ("HAVE_GNUTLS_ETM_STATUS", version >= 0x030400),
        ("HAVE_GNUTLS_KEYID_USE_SHA256", version >= 0x030401),
        ("HAVE_GNUTLS_COMPRESSION_GET", version < 0x030600),
        ("HAVE_GNUTLS_MAC_GET_NONCE_SIZE", version >= 0x030300),
        ("HAVE_GNUTLS_EXT_GET_NAME", version >= 0x030501),
        ("HAVE_GNUTLS_AEAD", version >= 0x030501),
        ("GNUTLS_VERSION_NUMBER_GE_0X030100", version >= 0x030100),
        ("GNUTLS_VERSION_NUMBER_GE_0X030114", version >= 0x030114),
        ("GNUTLS_VERSION_NUMBER_GE_0X030400", version >= 0x030400),
        ("GNUTLS_VERSION_NUMBER_GE_0X030501", version >= 0x030501),
        ("GNUTLS_NONBLOCK", text.lines().any(|line| {
            let mut w = line.split_whitespace(); w.next() == Some("#define") && w.next() == Some("GNUTLS_NONBLOCK")
        })),
    ];
    let mut settings: Vec<(String, Value)> = flags.into_iter()
        .map(|(key, value)| (key.to_owned(), Value::String(if value { "1" } else { "undefined" }.into()))).collect();
    for (key, value) in [("GNUTLS_VERSION_MAJOR", major), ("GNUTLS_VERSION_MINOR", minor),
                         ("GNUTLS_VERSION_PATCH", patch), ("GNUTLS_VERSION_NUMBER", version)] {
        settings.push((key.into(), Value::String(value.to_string())));
    }
    for suffix in ["PKCS12_3DES", "PKCS12_ARCFOUR", "PKCS12_RC2_40", "PBES2_3DES",
                   "PBES2_AES_128", "PBES2_AES_192", "PBES2_AES_256", "PBES2_DES",
                   "PBES1_DES_MD5", "PBES2_GOST_TC26Z", "PBES2_GOST_CPA", "PBES2_GOST_CPB",
                   "PBES2_GOST_CPC", "PBES2_GOST_CPD"] {
        let key = format!("GNUTLS_PKCS_{suffix}");
        let defined = x509.lines().any(|line| {
            let mut w = line.split_whitespace(); w.next() == Some("#define") && w.next() == Some(key.as_str())
        });
        settings.push((key, Value::String(if defined { "1" } else { "undefined" }.into())));
    }
    for (key, value) in settings {
        for section in ["macros", "features"] {
            if let Some(values) = config[section].as_object_mut() {
                values.retain(|name, _| !name.eq_ignore_ascii_case(&key));
            }
        }
        let section = if matches!(key.as_str(), "GNUTLS_VERSION_MAJOR" | "GNUTLS_VERSION_MINOR"
                                  | "GNUTLS_VERSION_PATCH" | "GNUTLS_VERSION_NUMBER") {
            "macros"
        } else { "features" };
        if config.get(section).is_none() { config[section] = json!({}); }
        config[section].as_object_mut().ok_or("invalid TLS configuration section")?.insert(key, value);
    }
    config["macros"]["GNUTLS_COMPILED_VERSION"] = Value::String((major * 10000 + minor * 100 + patch).to_string());
    Ok(())
}

pub(crate) fn bind_configuration(config: &mut Value) -> Result<(), String> {
    if !enabled(config)? { return Ok(()); }
    let prefix = prefix()?;
    let headers = header_identities(&prefix)?;
    configure_header_features(config, &prefix)?;
    let header = prefix.join("include/gnutls/gnutls.h");
    let library = prefix.join("lib/libgnutls.dylib");
    let deployment = crate::macos::deployment_target()?;
    let identity = crate::runtime::capture_tls_dependency_identity(&header, &library, &deployment)?;
    if header_identities(&prefix)? != headers { return Err("TLS headers changed during precompile binding".into()); }
    let abi = identity["abi_id"].as_str().ok_or("TLS capture has no ABI identity")?;
    config["macros"]["FEMACS_TLS_DEPENDENCY_ABI"] = Value::String(serde_json::to_string(abi).map_err(|e| e.to_string())?);
    config["macros"]["FEMACS_TLS_HEADERS_ABI"] = Value::String(serde_json::to_string(&headers_abi(&headers)?).map_err(|e| e.to_string())?);
    config["macros"].as_object_mut().ok_or("invalid TLS macro configuration")?
        .retain(|name, _| !name.strip_prefix("FEMACS_TLS_HEADER_")
            .and_then(|tail| tail.strip_suffix("_SHA256"))
            .is_some_and(|index| !index.is_empty() && index.bytes().all(|byte| byte.is_ascii_digit())));
    for (index, (_, hash)) in headers.iter().enumerate() {
        config["macros"][format!("FEMACS_TLS_HEADER_{index}_SHA256")] = Value::String(serde_json::to_string(hash).map_err(|e| e.to_string())?);
    }
    Ok(())
}

pub(crate) fn prepare(repo: &Path, root: &Path, selected: &Path, destination: &Path) -> Result<(), String> {
    let mut config: Value = serde_json::from_slice(&fs::read(selected).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    if !enabled(&config)? { return crate::tooling::write_json(destination, &config); }
    crate::standalone::check_host()?;
    let owned = crate::tooling::guard_scratch(repo, &root.join("tls"))?;
    if owned.exists() { return Err("TLS staging already exists; use its original bound build instead of overwriting it".into()); }
    let prefix = prefix()?;
    let header_inputs = header_identities(&prefix)?;
    let mut resolved = config.clone();
    bind_configuration(&mut resolved)?;
    if root.join("runtime-binding.json").is_file() && resolved != config {
        return Err("TLS target configuration changed after precompile binding".into());
    }
    config = resolved;
    if let Some(expected) = config["macros"].get("FEMACS_TLS_HEADERS_ABI") {
        if expected != &Value::String(serde_json::to_string(&headers_abi(&header_inputs)?).map_err(|e| e.to_string())?) {
            return Err("TLS header inventory changed after precompile binding".into());
        }
    }
    for (index, (_, hash)) in header_inputs.iter().enumerate() {
        let key = format!("FEMACS_TLS_HEADER_{index}_SHA256");
        if let Some(expected) = config["macros"].get(&key) {
            if expected != &Value::String(serde_json::to_string(hash).map_err(|e| e.to_string())?) {
                return Err("TLS header changed after precompile binding".into());
            }
        }
    }
    let header = prefix.join("include/gnutls/gnutls.h");
    let header_hash = digest(&header)?;
    let text = fs::read_to_string(&header).map_err(|e| e.to_string())?;
    let major = integer_define(&text, "GNUTLS_VERSION_MAJOR")?;
    let minor = integer_define(&text, "GNUTLS_VERSION_MINOR")?;
    let patch = integer_define(&text, "GNUTLS_VERSION_PATCH")?;
    let library = fs::canonicalize(prefix.join("lib/libgnutls.dylib")).map_err(|e| e.to_string())?;
    let deployment = crate::macos::deployment_target()?;
    let expected = config["macros"].get("FEMACS_TLS_DEPENDENCY_ABI").and_then(Value::as_str)
        .map(|text| serde_json::from_str::<String>(text).map_err(|e| e.to_string())).transpose()?;
    let identity = crate::runtime::stage_tls_dependencies(repo, &owned, &header, &library, &deployment, expected.as_deref())?;
    if digest(&header)? != header_hash || header_identities(&prefix)? != header_inputs {
        return Err("GnuTLS header changed during capture".into());
    }
    let primary = identity["dependencies"].as_array().ok_or("missing TLS dependencies")?.iter()
        .find(|row| row["source_sha256"] == identity["library_sha256"])
        .ok_or("TLS primary library absent from dependency closure")?;
    let name = Path::new(primary["destination"].as_str().ok_or("missing TLS library destination")?)
        .file_name().ok_or("invalid TLS basename")?.to_str().ok_or("TLS basename not UTF8")?.to_owned();
    crate::tooling::write_json(&owned.join("build-receipt.json"), &json!({
        "schema":1, "version":[major, minor, patch], "header_sha256":header_hash,
        "headers":header_inputs,
        "header_scope":"complete installed include/gnutls header tree; SDK headers belong to the compiler/SDK inputs",
        "primary":name, "identity":identity,
        "scope":"actual captured library dependency/license closure; runtime acceptance pending"
    }))?;
    // This layout is used by the standalone program before its final package is
    // published. The final terminal/App copy uses the same signed library bytes.
    copy_product_runtime(root, &root.join("port/runtime-tls"), &root.join("port/licenses/tls"))?;
    crate::tooling::write_json(destination, &config)
}

pub(crate) fn copy_product_runtime(root: &Path, libraries: &Path, licenses: &Path) -> Result<(), String> {
    let source = root.join("tls/runtime-native");
    if !source.is_dir() { return Ok(()); }
    crate::macos::copy_tree(&source, libraries)?;
    crate::macos::copy_tree(&root.join("tls/licenses"), licenses)
}

pub(crate) fn link_flags(repo: &Path) -> Result<Vec<String>, String> {
    let Some(config_path) = crate::android::config_source(None)? else { return Ok(Vec::new()); };
    let config: Value = serde_json::from_slice(&fs::read(config_path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    if !enabled(&config)? { return Ok(Vec::new()); }
    let root = std::env::var_os("FEMACS_BUILD_ROOT").map(PathBuf::from).ok_or("TLS requires selected build root")?;
    crate::tooling::guard_scratch(repo, &root)?;
    let receipt: Value = serde_json::from_slice(&fs::read(root.join("tls/build-receipt.json")).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    let name = receipt["primary"].as_str().ok_or("missing TLS primary basename")?;
    if Path::new(name).components().count() != 1 { return Err("unsafe TLS primary basename".into()); }
    for item in receipt["identity"]["dependencies"].as_array().ok_or("missing TLS closure")? {
        let relative = Path::new(item["destination"].as_str().ok_or("missing TLS dependency")?);
        if relative.components().any(|c| !matches!(c, std::path::Component::Normal(_))) {
            return Err("unsafe TLS dependency destination".into());
        }
        let basename = relative.file_name().ok_or("missing TLS dependency basename")?;
        let path = root.join("port/runtime-tls").join(basename);
        if digest(&path)? != item["staged_sha256"].as_str().ok_or("missing signed TLS library identity")? {
            return Err("staged TLS library changed before link".into());
        }
    }
    Ok(vec![root.join("port/runtime-tls").join(name).display().to_string(),
        "-Wl,-rpath,@executable_path/../runtime-tls".into(),
        "-Wl,-rpath,@executable_path/../Frameworks/runtime-tls".into()])
}
