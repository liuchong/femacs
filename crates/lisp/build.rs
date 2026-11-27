#![forbid(unsafe_code)]
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use flate2::Compression;
use flate2::write::GzEncoder;
use sha2::{Digest, Sha256};

fn collect_files(root: &Path, dir: &Path, out: &mut Vec<PathBuf>) {
    let mut entries: Vec<PathBuf> = fs::read_dir(dir)
        .expect("read stdlib dir")
        .filter_map(Result::ok)
        .map(|e| e.path())
        .collect();
    entries.sort();
    for p in entries {
        if p.is_dir() {
            collect_files(root, &p, out);
        } else if p.is_file() {
            if p.extension().is_some_and(|e| e == "elc") {
                continue;
            }
            out.push(p);
        }
    }
}

fn hex_digest(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        use std::fmt::Write as _;
        let _ = write!(s, "{b:02x}");
    }
    s
}

fn main() {
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_VM");
    if env::var_os("CARGO_FEATURE_VM").is_none() {
        println!("cargo:rustc-env=LISP_STDLIB_SHA256=disabled");
        return;
    }

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let stdlib_root = manifest_dir.join("stdlib");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rerun-if-changed={}", stdlib_root.display());

    let mut files = Vec::new();
    collect_files(&stdlib_root, &stdlib_root, &mut files);

    let bundle_path = out_dir.join("stdlib.bundle.gz");
    let out_file = File::create(&bundle_path).expect("create stdlib.bundle.gz");
    let mut encoder = GzEncoder::new(out_file, Compression::best());
    let mut hasher = Sha256::new();

    for path in &files {
        println!("cargo:rerun-if-changed={}", path.display());
        let rel = path
            .strip_prefix(&stdlib_root)
            .unwrap()
            .to_string_lossy()
            .replace('\\', "/");
        let data = fs::read(path).expect("read stdlib file");
        let rel_bytes = rel.as_bytes();
        let rel_len = u32::try_from(rel_bytes.len()).expect("path length fits u32");
        let data_len = u32::try_from(data.len()).expect("file length fits u32");

        for chunk in [
            &rel_len.to_le_bytes()[..],
            rel_bytes,
            &data_len.to_le_bytes()[..],
            &data,
        ] {
            encoder.write_all(chunk).expect("write stdlib bundle chunk");
            hasher.update(chunk);
        }
    }

    encoder.finish().expect("finish gzip stdlib bundle");
    let sha_hex = hex_digest(&hasher.finalize());
    println!("cargo:rustc-env=LISP_STDLIB_SHA256={sha_hex}");
}
