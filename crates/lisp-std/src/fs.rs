#![forbid(unsafe_code)]
//! Pure-Rust filesystem, globbing, walkdir, and cross-platform path operations (`std/fs`, `std/path`).

use std::collections::BTreeMap;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;
use std::time::UNIX_EPOCH;

use globset::GlobBuilder;
use uuid::Uuid;
use walkdir::WalkDir;

use crate::StdHandler;
use crate::value::{
    StdValue, expect_bool, expect_bytes, expect_list, expect_opt_str, expect_opt_u64, expect_str,
};

fn path_to_slash_string(p: &Path) -> String {
    p.to_string_lossy().replace('\\', "/")
}

fn normalize_path(raw: &str) -> String {
    let p = Path::new(raw);
    let mut parts: Vec<String> = Vec::new();
    let mut is_abs = false;
    for comp in p.components() {
        match comp {
            Component::Prefix(pre) => {
                parts.push(pre.as_os_str().to_string_lossy().into_owned());
            }
            Component::RootDir => {
                is_abs = true;
            }
            Component::CurDir => {}
            Component::ParentDir => {
                if let Some(last) = parts.last() {
                    if last != ".." {
                        parts.pop();
                        continue;
                    }
                }
                if !is_abs {
                    parts.push("..".to_string());
                }
            }
            Component::Normal(s) => {
                parts.push(s.to_string_lossy().into_owned());
            }
        }
    }
    if is_abs {
        if parts.is_empty() {
            "/".to_string()
        } else if parts
            .first()
            .is_some_and(|first| first.ends_with(':') || first.starts_with("\\\\"))
        {
            format!("{}/{}", parts[0], parts[1..].join("/"))
        } else {
            format!("/{}", parts.join("/"))
        }
    } else if parts.is_empty() {
        ".".to_string()
    } else {
        parts.join("/")
    }
}

pub(crate) fn register(mut reg: impl FnMut(&str, StdHandler)) {
    reg(
        "%std-fs-read-bytes",
        Arc::new(|args| {
            let path = expect_str(args.first(), "path")?;
            let data = fs::read(path).map_err(|e| format!("failed to read {path}: {e}"))?;
            Ok(StdValue::Bytes(data))
        }),
    );

    reg(
        "%std-fs-read-string",
        Arc::new(|args| {
            let path = expect_str(args.first(), "path")?;
            let text =
                fs::read_to_string(path).map_err(|e| format!("failed to read UTF-8 {path}: {e}"))?;
            Ok(StdValue::String(text))
        }),
    );

    reg(
        "%std-fs-write-bytes",
        Arc::new(|args| {
            let path = Path::new(expect_str(args.first(), "path")?);
            let data = expect_bytes(args.get(1), "data")?;
            let create_parents = expect_bool(args.get(2));
            if create_parents {
                if let Some(parent) = path.parent() {
                    if !parent.as_os_str().is_empty() {
                        fs::create_dir_all(parent).map_err(|e| {
                            format!("failed to create parent dir {}: {e}", parent.display())
                        })?;
                    }
                }
            }
            fs::write(path, data).map_err(|e| format!("failed to write {}: {e}", path.display()))?;
            Ok(StdValue::Int(data.len() as i64))
        }),
    );

    reg(
        "%std-fs-append",
        Arc::new(|args| {
            let path = Path::new(expect_str(args.first(), "path")?);
            let data = expect_bytes(args.get(1), "data")?;
            if let Some(parent) = path.parent() {
                if !parent.as_os_str().is_empty() {
                    fs::create_dir_all(parent).map_err(|e| {
                        format!("failed to create parent dir {}: {e}", parent.display())
                    })?;
                }
            }
            let mut f = OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .map_err(|e| format!("failed to append {}: {e}", path.display()))?;
            f.write_all(data)
                .map_err(|e| format!("failed to append write {}: {e}", path.display()))?;
            Ok(StdValue::Int(data.len() as i64))
        }),
    );

    reg(
        "%std-fs-mkdir-p",
        Arc::new(|args| {
            let path = expect_str(args.first(), "path")?;
            fs::create_dir_all(path).map_err(|e| format!("mkdir -p {path} failed: {e}"))?;
            Ok(StdValue::Bool(true))
        }),
    );

    reg(
        "%std-fs-rm-rf",
        Arc::new(|args| {
            let path = Path::new(expect_str(args.first(), "path")?);
            match fs::symlink_metadata(path) {
                Ok(meta) => {
                    if meta.is_dir() {
                        fs::remove_dir_all(path)
                            .map_err(|e| format!("rm -rf {} failed: {e}", path.display()))?;
                    } else {
                        fs::remove_file(path)
                            .map_err(|e| format!("rm {} failed: {e}", path.display()))?;
                    }
                    Ok(StdValue::Bool(true))
                }
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(StdValue::Nil),
                Err(e) => Err(format!("stat {} failed: {e}", path.display())),
            }
        }),
    );

    reg(
        "%std-fs-copy",
        Arc::new(|args| {
            let src = expect_str(args.first(), "src")?;
            let dst = Path::new(expect_str(args.get(1), "dst")?);
            if let Some(parent) = dst.parent() {
                if !parent.as_os_str().is_empty() {
                    fs::create_dir_all(parent).map_err(|e| {
                        format!("failed to create parent dir {}: {e}", parent.display())
                    })?;
                }
            }
            let n = fs::copy(src, dst)
                .map_err(|e| format!("copy {src} -> {} failed: {e}", dst.display()))?;
            Ok(StdValue::Int(n as i64))
        }),
    );

    reg(
        "%std-fs-rename",
        Arc::new(|args| {
            let src = expect_str(args.first(), "src")?;
            let dst = expect_str(args.get(1), "dst")?;
            fs::rename(src, dst).map_err(|e| format!("rename {src} -> {dst} failed: {e}"))?;
            Ok(StdValue::Bool(true))
        }),
    );

    reg(
        "%std-fs-stat",
        Arc::new(|args| {
            let path = Path::new(expect_str(args.first(), "path")?);
            let sym_meta = match fs::symlink_metadata(path) {
                Ok(m) => m,
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(StdValue::Nil),
                Err(e) => return Err(format!("stat {} failed: {e}", path.display())),
            };
            let target_meta = fs::metadata(path).ok();
            let effective = target_meta.as_ref().unwrap_or(&sym_meta);
            let mtime_ms = effective
                .modified()
                .ok()
                .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                .and_then(|d| i64::try_from(d.as_millis()).ok())
                .unwrap_or(0);
            let mut out = BTreeMap::new();
            out.insert("exists".to_string(), StdValue::Bool(true));
            out.insert("is-file".to_string(), StdValue::Bool(effective.is_file()));
            out.insert("is-dir".to_string(), StdValue::Bool(effective.is_dir()));
            out.insert(
                "is-symlink".to_string(),
                StdValue::Bool(sym_meta.file_type().is_symlink()),
            );
            out.insert("size".to_string(), StdValue::Int(effective.len() as i64));
            out.insert(
                "readonly".to_string(),
                StdValue::Bool(effective.permissions().readonly()),
            );
            out.insert("modified-unix-ms".to_string(), StdValue::Int(mtime_ms));
            Ok(StdValue::Map(out))
        }),
    );

    reg(
        "%std-fs-walk",
        Arc::new(|args| {
            let dir = expect_str(args.first(), "dir")?;
            let max_depth = expect_opt_u64(args.get(1), "max-depth")?;
            let mut walker = WalkDir::new(dir).sort_by_file_name();
            if let Some(d) = max_depth {
                walker = walker.max_depth(d as usize);
            }
            let mut paths = Vec::new();
            for entry in walker {
                let entry = entry.map_err(|e| format!("walk {dir} failed: {e}"))?;
                if entry.depth() == 0 {
                    continue;
                }
                paths.push(StdValue::String(path_to_slash_string(entry.path())));
            }
            Ok(StdValue::List(paths))
        }),
    );

    reg(
        "%std-fs-glob",
        Arc::new(|args| {
            let dir = Path::new(expect_str(args.first(), "dir")?);
            let pattern = expect_str(args.get(1), "pattern")?;
            let matcher = GlobBuilder::new(pattern)
                .literal_separator(true)
                .build()
                .map_err(|e| format!("invalid glob pattern '{pattern}': {e}"))?
                .compile_matcher();
            let mut matches = Vec::new();
            for entry in WalkDir::new(dir).sort_by_file_name() {
                let entry = entry.map_err(|e| format!("glob walk {} failed: {e}", dir.display()))?;
                if entry.depth() == 0 {
                    continue;
                }
                let rel = entry
                    .path()
                    .strip_prefix(dir)
                    .unwrap_or_else(|_| entry.path());
                let rel_slash = path_to_slash_string(rel);
                if matcher.is_match(rel_slash.as_str()) {
                    matches.push(StdValue::String(path_to_slash_string(entry.path())));
                }
            }
            Ok(StdValue::List(matches))
        }),
    );

    reg(
        "%std-fs-glob-match",
        Arc::new(|args| {
            let pattern = expect_str(args.first(), "pattern")?;
            let text = expect_str(args.get(1), "text")?;
            let matcher = GlobBuilder::new(pattern)
                .literal_separator(true)
                .build()
                .map_err(|e| format!("invalid glob pattern '{pattern}': {e}"))?
                .compile_matcher();
            Ok(StdValue::Bool(matcher.is_match(text)))
        }),
    );

    reg(
        "%std-fs-tempdir",
        Arc::new(|args| {
            let prefix = expect_opt_str(args.first(), "prefix")?.unwrap_or("lisp-tmp");
            let clean_prefix: String = prefix
                .chars()
                .filter(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_'))
                .collect();
            let stem = if clean_prefix.is_empty() {
                "lisp-tmp"
            } else {
                clean_prefix.as_str()
            };
            let dir = std::env::temp_dir().join(format!("{stem}-{}", Uuid::new_v4().simple()));
            fs::create_dir_all(&dir)
                .map_err(|e| format!("failed to create tempdir {}: {e}", dir.display()))?;
            Ok(StdValue::String(path_to_slash_string(&dir)))
        }),
    );

    reg(
        "%std-path-join",
        Arc::new(|args| {
            let items = expect_list(args.first(), "parts")?;
            let mut acc = PathBuf::new();
            for item in items {
                let s = item
                    .as_str()
                    .ok_or_else(|| "path-join parts must be strings".to_string())?;
                acc.push(s);
            }
            Ok(StdValue::String(path_to_slash_string(&acc)))
        }),
    );

    reg(
        "%std-path-normalize",
        Arc::new(|args| {
            let p = expect_str(args.first(), "path")?;
            Ok(StdValue::String(normalize_path(p)))
        }),
    );

    reg(
        "%std-path-dirname",
        Arc::new(|args| {
            let p = Path::new(expect_str(args.first(), "path")?);
            match p.parent() {
                Some(parent) if !parent.as_os_str().is_empty() => {
                    Ok(StdValue::String(path_to_slash_string(parent)))
                }
                _ => Ok(StdValue::Nil),
            }
        }),
    );

    reg(
        "%std-path-basename",
        Arc::new(|args| {
            let p = Path::new(expect_str(args.first(), "path")?);
            match p.file_name() {
                Some(name) => Ok(StdValue::String(name.to_string_lossy().into_owned())),
                None => Ok(StdValue::Nil),
            }
        }),
    );

    reg(
        "%std-path-stem",
        Arc::new(|args| {
            let p = Path::new(expect_str(args.first(), "path")?);
            match p.file_stem() {
                Some(stem) => Ok(StdValue::String(stem.to_string_lossy().into_owned())),
                None => Ok(StdValue::Nil),
            }
        }),
    );

    reg(
        "%std-path-ext",
        Arc::new(|args| {
            let p = Path::new(expect_str(args.first(), "path")?);
            match p.extension() {
                Some(ext) => Ok(StdValue::String(ext.to_string_lossy().into_owned())),
                None => Ok(StdValue::Nil),
            }
        }),
    );

    reg(
        "%std-path-is-abs",
        Arc::new(|args| {
            let raw = expect_str(args.first(), "path")?;
            let is_abs = Path::new(raw).is_absolute()
                || raw.starts_with('/')
                || raw.starts_with('\\');
            Ok(StdValue::Bool(is_abs))
        }),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_path_handles_dots_and_slashes() {
        assert_eq!(normalize_path("/a/b/../c/./d"), "/a/c/d");
        assert_eq!(normalize_path("a/b/../c"), "a/c");
        assert_eq!(normalize_path("."), ".");
        assert_eq!(normalize_path("/"), "/");
    }
}
