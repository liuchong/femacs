#![forbid(unsafe_code)]
//! Versioned on-disk unpack cache for the bundled standard library and cached portable dumps.

use std::fs;
#[cfg(feature = "vm")]
use std::fs::File;
#[cfg(feature = "vm")]
use std::io::{self, Read};
use std::path::{Component, Path, PathBuf};
#[cfg(feature = "vm")]
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(feature = "vm")]
use flate2::read::GzDecoder;

use crate::error::Error;

/// SHA-256 hex digest of the bundled `stdlib/` tree computed at build time.
pub const STDLIB_SHA256: &str = env!("LISP_CORE_STDLIB_SHA256");

#[cfg(feature = "vm")]
static STDLIB_BUNDLE_GZ: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/stdlib.bundle.gz"));

#[cfg(feature = "vm")]
const DUMP_MAGIC: [u8; 16] = [
    b'D', b'U', b'M', b'P', b'E', b'D', b'G', b'N', b'U', b'E', b'M', b'A', b'C', b'S', 0, 0,
];

/// Materialized directory layout for a cached runtime installation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CacheLayout {
    pub version_root: PathBuf,
    pub stdlib_dir: PathBuf,
    pub data_dir: PathBuf,
    pub boot_script: PathBuf,
    pub pdmp_path: PathBuf,
}

/// Resolve the cache root directory, obeying `override_dir`, `LISP_CORE_CACHE_DIR`,
/// `XDG_CACHE_HOME`, and platform defaults without requiring any external installation.
pub fn resolve_cache_root(override_dir: Option<&Path>) -> PathBuf {
    if let Some(dir) = override_dir {
        return dir.to_path_buf();
    }
    if let Some(val) = std::env::var_os("LISP_CORE_CACHE_DIR") {
        if !val.is_empty() {
            return PathBuf::from(val);
        }
    }
    if let Some(xdg) = std::env::var_os("XDG_CACHE_HOME") {
        if !xdg.is_empty() {
            return PathBuf::from(xdg).join("lisp-core");
        }
    }
    if let Some(home) = std::env::var_os("HOME") {
        if !home.is_empty() {
            let home_path = PathBuf::from(home);
            if cfg!(target_os = "macos") {
                return home_path.join("Library/Caches/lisp-core");
            }
            return home_path.join(".cache/lisp-core");
        }
    }
    std::env::temp_dir().join("lisp-core-cache")
}

/// Return the digest-scoped cache directory for this package version and stdlib digest.
pub fn versioned_cache_dir(cache_root: &Path) -> PathBuf {
    let short_sha = &STDLIB_SHA256[..STDLIB_SHA256.len().min(16)];
    cache_root.join(format!("v{}-{short_sha}", env!("CARGO_PKG_VERSION")))
}

/// Return the default portable dump path inside `version_root` keyed by the current executable.
pub fn exe_pdmp_path(version_root: &Path) -> PathBuf {
    let stem = std::env::current_exe()
        .ok()
        .and_then(|p| p.file_stem().map(|s| s.to_string_lossy().into_owned()))
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "lisp-core".to_string());
    version_root.join(format!("{stem}.pdmp"))
}

/// Compute expected [`CacheLayout`] paths beneath `cache_root`.
pub fn layout_for_root(cache_root: &Path) -> CacheLayout {
    let version_root = versioned_cache_dir(cache_root);
    let stdlib_dir = version_root.join("stdlib");
    let data_dir = version_root.join("data");
    let boot_script = stdlib_dir.join("core-boot.el");
    let pdmp_path = exe_pdmp_path(&version_root);
    CacheLayout {
        version_root,
        stdlib_dir,
        data_dir,
        boot_script,
        pdmp_path,
    }
}

/// Remove the cache directory tree cleanly (no-op if absent).
pub fn clean_cache(cache_root: &Path) -> Result<(), Error> {
    match fs::remove_dir_all(cache_root) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(Error::Io {
            symbol: "file-error".to_string(),
            message: format!("failed to remove cache dir {}: {e}", cache_root.display()),
            data: cache_root.display().to_string(),
        }),
    }
}

#[cfg(feature = "vm")]
/// Verify the leading 16-byte portable dump magic header before attempting VM restore.
pub fn is_valid_dump_header(path: &Path) -> bool {
    let Ok(mut f) = File::open(path) else {
        return false;
    };
    let mut header = [0u8; 16];
    let Ok(()) = f.read_exact(&mut header) else {
        return false;
    };
    header == DUMP_MAGIC
}

#[cfg(feature = "vm")]
fn io_err(context: &str, path: &Path, err: io::Error) -> Error {
    Error::Io {
        symbol: "file-error".to_string(),
        message: format!("{context} ({}): {err}", path.display()),
        data: path.display().to_string(),
    }
}

#[cfg(feature = "vm")]
fn is_layout_ready(layout: &CacheLayout) -> bool {
    let marker_path = layout.version_root.join(".ready");
    let Ok(marker) = fs::read_to_string(marker_path) else {
        return false;
    };
    marker.trim() == STDLIB_SHA256
        && layout.boot_script.is_file()
        && layout.data_dir.join("charsets").is_dir()
}

#[cfg(feature = "vm")]
fn sanitize_relative_path(rel: &str) -> Result<PathBuf, Error> {
    let p = Path::new(rel);
    let mut clean = PathBuf::new();
    for comp in p.components() {
        match comp {
            Component::Normal(seg) => clean.push(seg),
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return Err(Error::InitFailed(format!(
                    "unsafe path in bundled stdlib archive: {rel}"
                )));
            }
        }
    }
    if clean.as_os_str().is_empty() {
        return Err(Error::InitFailed(
            "empty path entry in bundled stdlib archive".to_string(),
        ));
    }
    Ok(clean)
}

#[cfg(feature = "vm")]
fn read_exact_u32(reader: &mut impl Read) -> Result<Option<u32>, Error> {
    let mut buf = [0u8; 4];
    let mut read = 0usize;
    while read < 4 {
        match reader.read(&mut buf[read..]) {
            Ok(0) => {
                if read == 0 {
                    return Ok(None);
                }
                return Err(Error::InitFailed(
                    "truncated bundled stdlib archive header".to_string(),
                ));
            }
            Ok(n) => read += n,
            Err(e) => {
                return Err(Error::InitFailed(format!(
                    "failed to decompress bundled stdlib archive: {e}"
                )));
            }
        }
    }
    Ok(Some(u32::from_le_bytes(buf)))
}

#[cfg(feature = "vm")]
fn unpack_bundle_to(staging_root: &Path) -> Result<(), Error> {
    let stdlib_root = staging_root.join("stdlib");
    let charsets_dir = staging_root.join("data/charsets");
    fs::create_dir_all(&stdlib_root).map_err(|e| io_err("create staging stdlib dir", &stdlib_root, e))?;
    fs::create_dir_all(&charsets_dir).map_err(|e| io_err("create staging data dir", &charsets_dir, e))?;

    let mut decoder = GzDecoder::new(STDLIB_BUNDLE_GZ);
    while let Some(path_len) = read_exact_u32(&mut decoder)? {
        let mut path_bytes = vec![0u8; path_len as usize];
        decoder
            .read_exact(&mut path_bytes)
            .map_err(|e| Error::InitFailed(format!("truncated bundled path: {e}")))?;
        let rel = String::from_utf8(path_bytes)
            .map_err(|e| Error::InitFailed(format!("non-UTF8 path in stdlib bundle: {e}")))?;
        let rel_path = sanitize_relative_path(&rel)?;

        let data_len = read_exact_u32(&mut decoder)?
            .ok_or_else(|| Error::InitFailed("missing file size in stdlib bundle".to_string()))?;
        let mut data = vec![0u8; data_len as usize];
        decoder
            .read_exact(&mut data)
            .map_err(|e| Error::InitFailed(format!("truncated bundled file {rel}: {e}")))?;

        let dst = stdlib_root.join(rel_path);
        if let Some(parent) = dst.parent() {
            fs::create_dir_all(parent).map_err(|e| io_err("create parent dir", parent, e))?;
        }
        fs::write(&dst, &data).map_err(|e| io_err("write bundled stdlib file", &dst, e))?;
    }

    let marker_path = staging_root.join(".ready");
    fs::write(&marker_path, STDLIB_SHA256)
        .map_err(|e| io_err("write stdlib cache ready marker", &marker_path, e))?;
    Ok(())
}

#[cfg(feature = "vm")]
/// Atomically materialize the bundled standard library under `cache_root` if missing or stale.
pub fn ensure_unpacked(cache_root: &Path) -> Result<CacheLayout, Error> {
    let layout = layout_for_root(cache_root);
    if is_layout_ready(&layout) {
        return Ok(layout);
    }

    fs::create_dir_all(cache_root).map_err(|e| io_err("create cache root", cache_root, e))?;
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let staging_root = cache_root.join(format!(".staging-{}-{nonce}", std::process::id()));
    let _ = fs::remove_dir_all(&staging_root);

    if let Err(err) = unpack_bundle_to(&staging_root) {
        let _ = fs::remove_dir_all(&staging_root);
        return Err(err);
    }

    if layout.version_root.exists() {
        if is_layout_ready(&layout) {
            let _ = fs::remove_dir_all(&staging_root);
            return Ok(layout);
        }
        let backup = cache_root.join(format!(".stale-{}-{nonce}", std::process::id()));
        let _ = fs::rename(&layout.version_root, &backup);
        let _ = fs::remove_dir_all(&backup);
    }

    match fs::rename(&staging_root, &layout.version_root) {
        Ok(()) => Ok(layout),
        Err(err) => {
            if is_layout_ready(&layout) {
                let _ = fs::remove_dir_all(&staging_root);
                Ok(layout)
            } else {
                let _ = fs::remove_dir_all(&staging_root);
                Err(io_err(
                    "finalize unpacked stdlib cache",
                    &layout.version_root,
                    err,
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_cache_root_prefers_explicit_then_env() {
        let explicit = PathBuf::from("build/custom-cache-dir");
        assert_eq!(resolve_cache_root(Some(&explicit)), explicit);

        let default_root = resolve_cache_root(None);
        assert!(default_root.ends_with("lisp-core") || default_root.ends_with("lisp-core-cache"));
    }

    #[test]
    fn clean_cache_removes_existing_tree_and_ignores_missing() {
        let base = std::env::temp_dir().join(format!("lisp-core-cache-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(base.join("nested")).expect("create test cache dir");
        fs::write(base.join("nested/file.txt"), b"ok").expect("write test cache file");
        clean_cache(&base).expect("clean existing cache");
        assert!(!base.exists());
        clean_cache(&base).expect("clean missing cache is no-op");
    }

    #[test]
    fn versioned_cache_dir_scopes_version_and_stdlib_digest() {
        let root = Path::new("/tmp/lisp-core-root");
        let dir = versioned_cache_dir(root);
        let name = dir
            .file_name()
            .and_then(|s| s.to_str())
            .expect("cache dir name");
        let prefix = format!("v{}-", env!("CARGO_PKG_VERSION"));
        assert!(name.starts_with(&prefix), "unexpected cache dir {name}");
        assert_eq!(name.len(), prefix.len() + STDLIB_SHA256.len().min(16));
    }

    #[cfg(feature = "vm")]
    #[test]
    fn is_valid_dump_header_checks_exact_16_byte_magic() {
        let base = std::env::temp_dir().join(format!("lisp-core-pdmp-hdr-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(&base).expect("create temp dir");

        let valid = base.join("valid.pdmp");
        let mut bytes = Vec::from(DUMP_MAGIC);
        bytes.extend_from_slice(&[1, 2, 3, 4]);
        fs::write(&valid, bytes).expect("write valid magic");
        assert!(is_valid_dump_header(&valid));

        let short = base.join("short.pdmp");
        fs::write(&short, &DUMP_MAGIC[..8]).expect("write short magic");
        assert!(!is_valid_dump_header(&short));

        let wrong = base.join("wrong.pdmp");
        fs::write(&wrong, b"BADMAGIC12345678").expect("write wrong magic");
        assert!(!is_valid_dump_header(&wrong));

        assert!(!is_valid_dump_header(&base.join("missing.pdmp")));
        let _ = fs::remove_dir_all(&base);
    }
}
