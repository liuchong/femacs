// femacs: port of lib-src/asset-directory-tool.c @ fd7afced403.
#![forbid(unsafe_code)]
//! Safe Rust implementation of `lib-src/asset-directory-tool`.

use std::env;
use std::ffi::OsString;
use std::fs::{self, File};
use std::io::{self, Write};
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::process::ExitCode;

struct DirectoryTree {
    name_bytes: Vec<u8>,
    st_size: u64,
    offset: usize,
    children: Vec<DirectoryTree>,
}

fn os_bytes(s: &std::ffi::OsStr) -> Vec<u8> {
    #[cfg(unix)]
    {
        s.as_bytes().to_vec()
    }
    #[cfg(not(unix))]
    {
        s.to_string_lossy().as_bytes().to_vec()
    }
}

fn build_tree(dir: &Path, parent_offset: &mut usize, need_file_size: bool) -> Result<Vec<DirectoryTree>, String> {
    let entries = fs::read_dir(dir).map_err(|e| format!("opendir: {e}"))?;
    let mut nodes = Vec::new();
    let extra = if need_file_size { 4 } else { 0 };

    for entry_res in entries {
        let entry = entry_res.map_err(|e| format!("readdir: {e}"))?;
        let file_name = entry.file_name();
        let raw_name = os_bytes(&file_name);
        if raw_name == b"." || raw_name == b".." {
            continue;
        }
        let path = entry.path();
        let meta = fs::symlink_metadata(&path).map_err(|e| format!("fstatat: {e}"))?;
        let file_type = meta.file_type();

        if file_type.is_dir() {
            let length = raw_name.len();
            let mut name_with_slash = Vec::with_capacity(length + 1);
            name_with_slash.extend_from_slice(&raw_name);
            name_with_slash.push(b'/');

            let mut subtree_end = *parent_offset + length + 6 + extra;
            let children = build_tree(&path, &mut subtree_end, need_file_size)?;
            *parent_offset = subtree_end;

            nodes.push(DirectoryTree {
                name_bytes: name_with_slash,
                st_size: 0,
                offset: subtree_end,
                children,
            });
        } else if file_type.is_file() {
            let st_size = meta.len();
            if st_size >= 0x1ffffff {
                return Err(format!(
                    "asset-directory-tool: file size exceeds maximum representable in a directory-tree: {}",
                    file_name.to_string_lossy()
                ));
            }
            let length = raw_name.len();
            let next_offset = *parent_offset + length + 5 + extra;
            *parent_offset = next_offset;

            nodes.push(DirectoryTree {
                name_bytes: raw_name,
                st_size,
                offset: next_offset,
                children: Vec::new(),
            });
        }
    }
    Ok(nodes)
}

fn write_node<W: Write>(
    out: &mut W,
    node: &DirectoryTree,
    offset: &mut usize,
    need_file_size: bool,
) -> Result<(), String> {
    out.write_all(&node.name_bytes).map_err(|e| format!("write: {e}"))?;
    out.write_all(&[0u8]).map_err(|e| format!("write: {e}"))?;
    let mut written = node.name_bytes.len() + 1;

    let size_le = (node.st_size as u32).to_le_bytes();
    let offset_le = (node.offset as u32).to_le_bytes();
    if need_file_size {
        out.write_all(&size_le).map_err(|e| format!("write: {e}"))?;
        out.write_all(&offset_le).map_err(|e| format!("write: {e}"))?;
        written += 8;
    } else {
        out.write_all(&offset_le).map_err(|e| format!("write: {e}"))?;
        written += 4;
    }

    *offset += written;

    for child in &node.children {
        write_node(out, child, offset, need_file_size)?;
    }

    if node.offset != *offset {
        return Err(format!(
            "asset-directory-tool: invalid offset: expected {}, got {}.",
            node.offset, *offset
        ));
    }
    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<OsString> = env::args_os().collect();
    let prog = args
        .first()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "asset-directory-tool".to_string());

    if args.len() != 3 && args.len() != 4 {
        let _ = writeln!(io::stderr(), "usage: {prog} directory output-file [--api-8]");
        return ExitCode::FAILURE;
    }

    let need_file_size = args.len() == 4 && args[3].to_string_lossy() == "--api-8";
    let in_dir = Path::new(&args[1]);
    let out_path = Path::new(&args[2]);

    let mut out_file = match File::create(out_path) {
        Ok(f) => f,
        Err(err) => {
            let _ = writeln!(io::stderr(), "open: {err}");
            return ExitCode::FAILURE;
        }
    };

    let header_len = 5 + if need_file_size { 4 } else { 0 };
    let mut build_offset = header_len;
    let nodes = match build_tree(in_dir, &mut build_offset, need_file_size) {
        Ok(n) => n,
        Err(msg) => {
            let _ = writeln!(io::stderr(), "{msg}");
            return ExitCode::FAILURE;
        }
    };

    if let Err(err) = out_file.write_all(b"EMACS") {
        let _ = writeln!(io::stderr(), "write: {err}");
        return ExitCode::FAILURE;
    }
    if need_file_size {
        if let Err(err) = out_file.write_all(b"____") {
            let _ = writeln!(io::stderr(), "write: {err}");
            return ExitCode::FAILURE;
        }
    }

    let mut write_offset = header_len;
    for node in &nodes {
        if let Err(msg) = write_node(&mut out_file, node, &mut write_offset, need_file_size) {
            let _ = writeln!(io::stderr(), "{msg}");
            return ExitCode::FAILURE;
        }
    }

    if let Err(err) = out_file.flush() {
        let _ = writeln!(io::stderr(), "write: {err}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
