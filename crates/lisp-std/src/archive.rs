#![forbid(unsafe_code)]
//! Pure-Rust gzip/zlib, tar/tar.gz, and zip archive codecs (`std/archive`).

use std::fs::{self, File};
use std::io::{BufReader, Cursor, Read, Write};
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;

use flate2::Compression;
use flate2::read::{GzDecoder, ZlibDecoder};
use flate2::write::{GzEncoder, ZlibEncoder};
use walkdir::WalkDir;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

use crate::StdHandler;
use crate::value::{StdValue, expect_bool, expect_bytes, expect_str};

pub(crate) fn sanitize_archive_rel_path(raw: &Path) -> Result<PathBuf, String> {
    let mut clean = PathBuf::new();
    for comp in raw.components() {
        match comp {
            Component::Normal(seg) => clean.push(seg),
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return Err(format!(
                    "unsafe archive entry path rejected: {}",
                    raw.display()
                ));
            }
        }
    }
    if clean.as_os_str().is_empty() {
        return Err(format!(
            "empty archive entry path rejected: {}",
            raw.display()
        ));
    }
    Ok(clean)
}

pub fn tar_unpack_reader<R: Read>(reader: R, dest_dir: &Path, gzip: bool) -> Result<usize, String> {
    fs::create_dir_all(dest_dir)
        .map_err(|e| format!("failed to create dest dir {}: {e}", dest_dir.display()))?;
    let mut archive: tar::Archive<Box<dyn Read>> = if gzip {
        tar::Archive::new(Box::new(GzDecoder::new(reader)))
    } else {
        tar::Archive::new(Box::new(reader))
    };

    let entries = archive
        .entries()
        .map_err(|e| format!("failed to read tar entries: {e}"))?;
    let mut count = 0usize;
    for entry_res in entries {
        let mut entry = entry_res.map_err(|e| format!("corrupt tar entry: {e}"))?;
        let entry_type = entry.header().entry_type();
        if entry_type.is_symlink() || entry_type.is_hard_link() {
            return Err("refusing symlink or hardlink inside tar archive".to_string());
        }
        let raw_path = entry
            .path()
            .map_err(|e| format!("invalid tar entry path: {e}"))?
            .into_owned();
        // Skip bare "." root directory entries in tar archives.
        if raw_path == Path::new(".") || raw_path == Path::new("./") {
            continue;
        }
        let rel = sanitize_archive_rel_path(&raw_path)?;
        let out_path = dest_dir.join(&rel);
        if entry_type.is_dir() {
            fs::create_dir_all(&out_path)
                .map_err(|e| format!("failed to create dir {}: {e}", out_path.display()))?;
            count += 1;
            continue;
        }
        if entry_type.is_file() {
            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("failed to create dir {}: {e}", parent.display()))?;
            }
            let mut out_file = File::create(&out_path)
                .map_err(|e| format!("failed to create file {}: {e}", out_path.display()))?;
            std::io::copy(&mut entry, &mut out_file)
                .map_err(|e| format!("failed to unpack {}: {e}", out_path.display()))?;
            count += 1;
        }
    }
    Ok(count)
}

pub fn zip_unpack_reader<R: Read + std::io::Seek>(reader: R, dest_dir: &Path) -> Result<usize, String> {
    fs::create_dir_all(dest_dir)
        .map_err(|e| format!("failed to create dest dir {}: {e}", dest_dir.display()))?;
    let mut archive = ZipArchive::new(reader).map_err(|e| format!("invalid zip archive: {e}"))?;
    let mut count = 0usize;
    for idx in 0..archive.len() {
        let mut entry = archive
            .by_index(idx)
            .map_err(|e| format!("corrupt zip entry #{idx}: {e}"))?;
        if entry.is_symlink() {
            return Err(format!("refusing symlink entry in zip archive: {}", entry.name()));
        }
        let Some(safe_rel) = entry.enclosed_name() else {
            return Err(format!(
                "unsafe archive entry path rejected: {}",
                entry.name()
            ));
        };
        let clean_rel = sanitize_archive_rel_path(&safe_rel)?;
        let out_path = dest_dir.join(&clean_rel);
        if entry.is_dir() {
            fs::create_dir_all(&out_path)
                .map_err(|e| format!("failed to create dir {}: {e}", out_path.display()))?;
        } else {
            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("failed to create dir {}: {e}", parent.display()))?;
            }
            let mut out_file = File::create(&out_path)
                .map_err(|e| format!("failed to create file {}: {e}", out_path.display()))?;
            std::io::copy(&mut entry, &mut out_file)
                .map_err(|e| format!("failed to unpack {}: {e}", out_path.display()))?;
        }
        count += 1;
    }
    Ok(count)
}

pub fn extract_archive_bytes(url_or_name: &str, bytes: &[u8], dest: &Path) -> Result<(), String> {
    let lower = url_or_name.to_ascii_lowercase();
    if lower.ends_with(".zip") {
        zip_unpack_reader(Cursor::new(bytes), dest)?;
        Ok(())
    } else if lower.ends_with(".tar.gz") || lower.ends_with(".tgz") {
        tar_unpack_reader(Cursor::new(bytes), dest, true)?;
        Ok(())
    } else if lower.ends_with(".tar") {
        tar_unpack_reader(Cursor::new(bytes), dest, false)?;
        Ok(())
    } else {
        Err(format!(
            "unsupported archive format for '{url_or_name}'; expected .tar.gz, .tgz, .tar, or .zip"
        ))
    }
}

pub(crate) fn register(mut reg: impl FnMut(&str, StdHandler)) {
    reg(
        "%std-gzip-compress",
        Arc::new(|args| {
            let data = expect_bytes(args.first(), "data")?;
            let mut enc = GzEncoder::new(Vec::new(), Compression::default());
            enc.write_all(data)
                .map_err(|e| format!("gzip compress error: {e}"))?;
            let out = enc
                .finish()
                .map_err(|e| format!("gzip finish error: {e}"))?;
            Ok(StdValue::Bytes(out))
        }),
    );

    reg(
        "%std-gzip-decompress",
        Arc::new(|args| {
            let data = expect_bytes(args.first(), "data")?;
            let mut dec = GzDecoder::new(data);
            let mut out = Vec::new();
            dec.read_to_end(&mut out)
                .map_err(|e| format!("gzip decompress error: {e}"))?;
            Ok(StdValue::Bytes(out))
        }),
    );

    reg(
        "%std-zlib-compress",
        Arc::new(|args| {
            let data = expect_bytes(args.first(), "data")?;
            let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
            enc.write_all(data)
                .map_err(|e| format!("zlib compress error: {e}"))?;
            let out = enc
                .finish()
                .map_err(|e| format!("zlib finish error: {e}"))?;
            Ok(StdValue::Bytes(out))
        }),
    );

    reg(
        "%std-zlib-decompress",
        Arc::new(|args| {
            let data = expect_bytes(args.first(), "data")?;
            let mut dec = ZlibDecoder::new(data);
            let mut out = Vec::new();
            dec.read_to_end(&mut out)
                .map_err(|e| format!("zlib decompress error: {e}"))?;
            Ok(StdValue::Bytes(out))
        }),
    );

    reg(
        "%std-tar-pack",
        Arc::new(|args| {
            let src_dir = Path::new(expect_str(args.first(), "src-dir")?);
            let out_path = Path::new(expect_str(args.get(1), "out-path")?);
            let gzip = expect_bool(args.get(2));
            if let Some(parent) = out_path.parent() {
                if !parent.as_os_str().is_empty() {
                    fs::create_dir_all(parent).map_err(|e| {
                        format!("failed to create parent dir {}: {e}", parent.display())
                    })?;
                }
            }
            let out_file = File::create(out_path)
                .map_err(|e| format!("failed to create tar {}: {e}", out_path.display()))?;
            if gzip {
                let gz = GzEncoder::new(out_file, Compression::default());
                let mut builder = tar::Builder::new(gz);
                builder
                    .append_dir_all(".", src_dir)
                    .map_err(|e| format!("tar append error: {e}"))?;
                let gz = builder
                    .into_inner()
                    .map_err(|e| format!("tar finish error: {e}"))?;
                gz.finish()
                    .map_err(|e| format!("tar.gz finish error: {e}"))?;
            } else {
                let mut builder = tar::Builder::new(out_file);
                builder
                    .append_dir_all(".", src_dir)
                    .map_err(|e| format!("tar append error: {e}"))?;
                builder
                    .finish()
                    .map_err(|e| format!("tar finish error: {e}"))?;
            }
            Ok(StdValue::Bool(true))
        }),
    );

    reg(
        "%std-tar-unpack",
        Arc::new(|args| {
            let archive_path = Path::new(expect_str(args.first(), "archive-path")?);
            let dest_dir = Path::new(expect_str(args.get(1), "dest-dir")?);
            let gzip = expect_bool(args.get(2));
            let f = File::open(archive_path)
                .map_err(|e| format!("failed to open {}: {e}", archive_path.display()))?;
            let count = tar_unpack_reader(BufReader::new(f), dest_dir, gzip)?;
            Ok(StdValue::Int(count as i64))
        }),
    );

    reg(
        "%std-tar-list",
        Arc::new(|args| {
            let archive_path = Path::new(expect_str(args.first(), "archive-path")?);
            let gzip = expect_bool(args.get(1));
            let f = File::open(archive_path)
                .map_err(|e| format!("failed to open {}: {e}", archive_path.display()))?;
            let reader = BufReader::new(f);
            let mut archive: tar::Archive<Box<dyn Read>> = if gzip {
                tar::Archive::new(Box::new(GzDecoder::new(reader)))
            } else {
                tar::Archive::new(Box::new(reader))
            };
            let mut names = Vec::new();
            for entry_res in archive
                .entries()
                .map_err(|e| format!("tar entries error: {e}"))?
            {
                let entry = entry_res.map_err(|e| format!("tar entry error: {e}"))?;
                let p = entry
                    .path()
                    .map_err(|e| format!("tar path error: {e}"))?;
                names.push(StdValue::String(p.to_string_lossy().replace('\\', "/")));
            }
            Ok(StdValue::List(names))
        }),
    );

    reg(
        "%std-zip-pack",
        Arc::new(|args| {
            let src_dir = Path::new(expect_str(args.first(), "src-dir")?);
            let out_path = Path::new(expect_str(args.get(1), "out-path")?);
            if let Some(parent) = out_path.parent() {
                if !parent.as_os_str().is_empty() {
                    fs::create_dir_all(parent).map_err(|e| {
                        format!("failed to create parent dir {}: {e}", parent.display())
                    })?;
                }
            }
            let out_file = File::create(out_path)
                .map_err(|e| format!("failed to create zip {}: {e}", out_path.display()))?;
            let mut writer = ZipWriter::new(out_file);
            let opts = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);

            for entry in WalkDir::new(src_dir).sort_by_file_name() {
                let entry = entry.map_err(|e| format!("zip walk error: {e}"))?;
                if entry.depth() == 0 {
                    continue;
                }
                let rel = entry
                    .path()
                    .strip_prefix(src_dir)
                    .map_err(|e| format!("zip rel path error: {e}"))?;
                let name = rel.to_string_lossy().replace('\\', "/");
                if entry.file_type().is_dir() {
                    let dir_name = if name.ends_with('/') {
                        name
                    } else {
                        format!("{name}/")
                    };
                    writer
                        .add_directory(&dir_name, opts)
                        .map_err(|e| format!("zip add_directory error: {e}"))?;
                } else if entry.file_type().is_file() {
                    writer
                        .start_file(&name, opts)
                        .map_err(|e| format!("zip start_file error: {e}"))?;
                    let bytes = fs::read(entry.path())
                        .map_err(|e| format!("zip read {} error: {e}", entry.path().display()))?;
                    writer
                        .write_all(&bytes)
                        .map_err(|e| format!("zip write error: {e}"))?;
                }
            }
            writer
                .finish()
                .map_err(|e| format!("zip finish error: {e}"))?;
            Ok(StdValue::Bool(true))
        }),
    );

    reg(
        "%std-zip-unpack",
        Arc::new(|args| {
            let archive_path = Path::new(expect_str(args.first(), "archive-path")?);
            let dest_dir = Path::new(expect_str(args.get(1), "dest-dir")?);
            let f = File::open(archive_path)
                .map_err(|e| format!("failed to open {}: {e}", archive_path.display()))?;
            let count = zip_unpack_reader(BufReader::new(f), dest_dir)?;
            Ok(StdValue::Int(count as i64))
        }),
    );

    reg(
        "%std-zip-list",
        Arc::new(|args| {
            let archive_path = Path::new(expect_str(args.first(), "archive-path")?);
            let f = File::open(archive_path)
                .map_err(|e| format!("failed to open {}: {e}", archive_path.display()))?;
            let mut archive = ZipArchive::new(BufReader::new(f))
                .map_err(|e| format!("invalid zip archive: {e}"))?;
            let mut names = Vec::with_capacity(archive.len());
            for i in 0..archive.len() {
                let entry = archive
                    .by_index(i)
                    .map_err(|e| format!("zip entry #{i} error: {e}"))?;
                names.push(StdValue::String(entry.name().to_string()));
            }
            Ok(StdValue::List(names))
        }),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_archive_rel_path_blocks_traversal_and_roots() {
        assert!(sanitize_archive_rel_path(Path::new("../escape.txt")).is_err());
        assert!(sanitize_archive_rel_path(Path::new("a/../../b")).is_err());
        assert!(sanitize_archive_rel_path(Path::new("/etc/passwd")).is_err());
        assert!(sanitize_archive_rel_path(Path::new("")).is_err());
        assert_eq!(
            sanitize_archive_rel_path(Path::new("./src/main.el")).unwrap(),
            PathBuf::from("src/main.el")
        );
    }

    #[test]
    fn tar_unpack_rejects_parent_dir_traversal_archive() {
        let mut tar_buf = Vec::new();
        {
            let mut hdr = tar::Header::new_gnu();
            let data = b"evil";
            hdr.set_size(data.len() as u64);
            hdr.set_mode(0o644);
            hdr.set_cksum();
            let mut builder = tar::Builder::new(&mut tar_buf);
            // Write raw header bypassing builder's path check to simulate malicious input.
            let name_bytes = b"../../evil.txt";
            let raw = hdr.as_mut_bytes();
            raw[..name_bytes.len()].copy_from_slice(name_bytes);
            raw[name_bytes.len()] = 0;
            hdr.set_cksum();
            builder.append(&hdr, &data[..]).unwrap();
            builder.finish().unwrap();
        }
        let tmp = std::env::temp_dir().join(format!("lisp-std-tar-test-{}", uuid::Uuid::new_v4()));
        let err = tar_unpack_reader(Cursor::new(tar_buf), &tmp, false).unwrap_err();
        assert!(err.contains("unsafe archive entry path rejected"), "{err}");
        let _ = fs::remove_dir_all(&tmp);
    }
}
