//! Independent Apple Silicon macOS bootstrap. No reference source or binary.
#![forbid(unsafe_code)]
use super::*;
use sha2::{Digest, Sha256};

pub(super) fn check_host() -> Result<(), String> {
    if !cfg!(all(target_os = "macos", target_arch = "aarch64")) {
        return Err("independent targets currently support Apple Silicon macOS only".into());
    }
    Ok(())
}

pub(super) fn resources(repo: &Path, family: &str, program: Option<&Path>) -> Result<(), String> {
    let mut command = Command::new("python3");
    command
        .current_dir(repo)
        .args(["tools/bootstrap-resources.py", "--family", family]);
    if let Some(program) = program {
        command.arg("--program").arg(program);
    }
    let status = command.status().map_err(|e| e.to_string())?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("resource generation {family}: {status}"))
    }
}

/// The first dump search entry also identifies the source/data tree in the VM.
/// Keep that canonical root; non-preloaded corrections belong to the product.
pub(super) fn lisp_paths(repo: &Path, trailing: bool) -> String {
    format!("{}{}", repo.join("lisp").display(), if trailing { "/" } else { "" })
}

pub(super) fn install_library_corrections(repo: &Path, lisp: &Path) -> Result<(), String> {
    let status = Command::new("python3").current_dir(repo)
        .args(["tools/library_corrections.py", "--action", "install", "--lisp-root"])
        .arg(lisp).status().map_err(|e| e.to_string())?;
    if status.success() { Ok(()) } else { Err(format!("install generated library corrections: {status}")) }
}

pub(super) fn stage_module_api(repo: &Path, destination: &Path) -> Result<(), String> {
    fs::create_dir_all(destination).map_err(|e| e.to_string())?;
    crate::owned_copy::copy_file(
        &repo.join("build/generation/module-api/emacs-module.h"),
        &destination.join("emacs-module.h"),
    )
}

pub(super) fn build_terminal() -> i32 {
    match terminal() {
        Ok(()) => 0,
        Err(e) => fail(e),
    }
}

fn terminal() -> Result<(), String> {
    check_host()?;
    let repo = canonical(Path::new(REPO));
    terminal_in(&repo, &repo.join("build/terminal"), true)
}

pub(super) fn terminal_in(repo: &Path, root: &Path, publish_info: bool) -> Result<(), String> {
    terminal_configured(repo, root, publish_info, &repo.join("build-support/macos/terminal/config.json"))
}

pub(super) struct PreparedTerminalEnv {
    pub(super) prepared_config: PathBuf,
    pub(super) config_dir: PathBuf,
    pub(super) objects_dir: PathBuf,
}

pub(super) fn prepare_terminal_env(
    repo: &Path,
    root: &Path,
    config_file: &Path,
) -> Result<PreparedTerminalEnv, String> {
    prepare_terminal_env_with_paths(repo, root, config_file, None)
}

fn prepare_terminal_env_with_paths(
    repo: &Path,
    root: &Path,
    config_file: &Path,
    relocatable_exec: Option<&str>,
) -> Result<PreparedTerminalEnv, String> {
    check_host()?;
    let root = crate::tooling::guard_scratch(repo, root)?;
    let config = root.join("config");
    let objects = root.join("objects");
    fs::create_dir_all(&config).map_err(|e| e.to_string())?;
    let prepared_config = config.join("darwin-terminal.json");
    let tls_config = config.join("darwin-terminal-tls.json");
    crate::tls::prepare(repo, &root, config_file, &tls_config)?;
    let lcms_config = config.join("darwin-terminal-lcms.json");
    crate::lcms::prepare(repo, &root, &tls_config, &lcms_config)?;
    let image_config = config.join("darwin-terminal-image.json");
    crate::image::prepare(repo, &root, &lcms_config, &image_config)?;
    crate::treesit::prepare(repo, &root, &image_config, &prepared_config)?;
    resources(repo, "base", None)?;
    stage_module_api(repo, &root.join("port/include"))?;
    let paths = if let Some(exec_rel) = relocatable_exec {
        crate::runtime::terminal_paths_with_exec(repo, &root, exec_rel)
    } else if crate::runtime::verify_selected_root(repo, &root)?.is_some() {
        crate::runtime::terminal_paths_source(repo, &root)
    } else {
        // Generate checkout-local paths for the terminal configuration.
        let mut s = String::new();
        for (name, rel) in [
            ("PATH_LOADSEARCH", "lisp"),
            ("PATH_DUMPLOADSEARCH", "lisp"),
            ("PATH_DUMPLOADSEARCH_SLASH", "lisp/"),
            ("PATH_EXEC", "build/terminal/port/lib-src"),
            ("PATH_DATA", "etc"),
            ("PATH_DOC", "etc"),
            ("PATH_INFO", "info"),
            ("PATH_BITMAPS", "etc/images"),
        ] {
            s.push_str(&format!(
                "#[inline(always)] pub fn {name}() -> *const c_char {{ c{:?}.as_ptr() }}\n",
                if name.starts_with("PATH_LOADSEARCH") || name.starts_with("PATH_DUMPLOADSEARCH") {
                    lisp_paths(repo, name == "PATH_DUMPLOADSEARCH_SLASH")
                } else if name == "PATH_EXEC" {
                    root.join("port/lib-src").display().to_string()
                } else {
                    repo.join(rel).display().to_string()
                }
            ));
        }
        s.push_str("pub fn PATH_SITELOADSEARCH() -> *const c_char { c\"\".as_ptr() }\npub fn PATH_GAME() -> *const c_char { core::ptr::null() }\n");
        s
    };
    fs::write(config.join("epaths.rs"), paths).map_err(|e| e.to_string())?;
    Ok(PreparedTerminalEnv {
        prepared_config,
        config_dir: config,
        objects_dir: objects,
    })
}

pub(super) fn terminal_configured(repo: &Path, root: &Path, publish_info: bool, config_file: &Path) -> Result<(), String> {
    let prep = prepare_terminal_env(repo, root, config_file)?;
    let root = crate::tooling::guard_scratch(repo, root)?;
    let executable = std::env::current_exe().map_err(|e| e.to_string())?;
    let invoke = |args: &[&str]| -> Result<(), String> {
        let mut command = Command::new(&executable);
        command.current_dir(repo)
            .args(args)
            .env_remove("FEMACS_MACOS_GUI")
            .env_remove("FEMACS_TARGET")
            .env_remove("FEMACS_LINK_CHECKING")
            .env("FEMACS_BUILD_TARGET", "macos-terminal")
            .env("FEMACS_CONFIG", "darwin-arm64")
            .env("FEMACS_CONFIG_FILE", &prep.prepared_config)
            .env("FEMACS_CONFIG_RS", prep.config_dir.join("config.rs"))
            .env("FEMACS_EPATHS_RS", prep.config_dir.join("epaths.rs"))
            .env("FEMACS_UNITS_DIR", &prep.objects_dir)
            .env("FEMACS_TOOL_UNITS", &prep.objects_dir)
            .env("FEMACS_BUILD_ROOT", &root)
            .env("FEMACS_PGO", "off")
            .env("FEMACS_RELEASE_BUILD", "1");
        let status = crate::runtime::configured_child(repo, &root, &mut command)?
            .status()
            .map_err(|e| e.to_string())?;
        if status.success() {
            Ok(())
        } else {
            Err(format!("{}: {status}", args.join(" ")))
        }
    };
    invoke(&["gen-config"])?;
    invoke(&["link-owned"])?;
    invoke(&["link-program", "lib-src/emacsclient.c"])?;
    invoke(&["link-program", "lib-src/hexl.c"])?;
    resources(&repo, "doc", None)?;
    invoke(&["dump-bootstrap"])?;
    resources(&repo, "all", Some(&root.join("port/src/emacs")))?;
    invoke(&["dump"])?;
    let info = crate::manuals::for_product(&repo, &root)?;
    if crate::runtime::verify_selected_root(repo, &root)?.is_some() {
        crate::runtime::package_terminal(repo, &root, &info)?;
    }
    if publish_info {
        crate::manuals::publish_info(&repo, &info)?;
    }
    println!("Terminal: {}", root.join("port/src/emacs").display());
    Ok(())
}

pub(super) fn build_lisp(args: &[String]) -> i32 {
    let mut out: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                i += 1;
                let Some(val) = args.get(i) else {
                    eprintln!("xtask build-lisp: --out requires a path");
                    return 2;
                };
                out = Some(PathBuf::from(val));
            }
            other => {
                eprintln!("xtask build-lisp: unknown argument `{other}`");
                return 2;
            }
        }
        i += 1;
    }
    let repo = fs::canonicalize(REPO).unwrap_or_else(|_| PathBuf::from(REPO));
    let dest = out
        .map(|p| if p.is_absolute() { p } else { repo.join(p) })
        .unwrap_or_else(|| repo.join("dist/lisp"));
    match lisp_package_in(&repo, &repo.join("build/lisp"), &dest) {
        Ok(()) => 0,
        Err(e) => fail(e),
    }
}

fn verify_no_symlinks(dir: &Path) -> Result<(), String> {
    for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let meta = fs::symlink_metadata(&path).map_err(|e| e.to_string())?;
        if meta.file_type().is_symlink() {
            return Err(format!("standalone Lisp package contains unexpected symlink: {}", path.display()));
        }
        if meta.is_dir() {
            verify_no_symlinks(&path)?;
        }
    }
    Ok(())
}

fn codesign_verify_strict(path: &Path) -> Result<(), String> {
    if !cfg!(target_os = "macos") {
        return Ok(());
    }
    let verify = Command::new("codesign")
        .args(["--verify", "--strict"])
        .arg(path)
        .status()
        .map_err(|e| format!("codesign --verify {}: {e}", path.display()))?;
    if !verify.success() {
        return Err(format!("codesign --verify --strict failed for {}", path.display()));
    }
    Ok(())
}

fn codesign_adhoc_and_verify(path: &Path) -> Result<(), String> {
    if !cfg!(target_os = "macos") {
        return Ok(());
    }
    let sign = Command::new("codesign")
        .args(["-s", "-", "-f"])
        .arg(path)
        .status()
        .map_err(|e| format!("codesign {}: {e}", path.display()))?;
    if !sign.success() {
        return Err(format!("codesign -s - -f failed for {}", path.display()));
    }
    codesign_verify_strict(path)
}

fn build_http_host_binary(repo: &Path, root: &Path) -> Result<PathBuf, String> {
    let target = crate::cargo_build::host_target_triple();
    let target_dir = root.join("cargo-http");
    let status = Command::new("cargo")
        .current_dir(repo)
        .args([
            "build",
            "--release",
            "--target",
            target,
            "-p",
            "http-host",
            "--bin",
            "http-host",
            "--target-dir",
        ])
        .arg(&target_dir)
        .status()
        .map_err(|e| format!("cargo build http-host: {e}"))?;
    if !status.success() {
        return Err(format!("cargo build -p http-host failed ({status})"));
    }
    let built = target_dir.join(target).join("release").join("http-host");
    if !built.is_file() {
        return Err(format!("missing built http-host binary at {}", built.display()));
    }
    Ok(built)
}

fn run_lisp_self_dump(lisp_bin: &Path, repo: &Path, pdmp_out: &Path) -> Result<(), String> {
    let _ = fs::remove_file(pdmp_out);
    let status = Command::new(lisp_bin)
        .current_dir(repo)
        .arg("--no-dump")
        .arg("--lisp-dir")
        .arg(repo.join("lisp"))
        .arg("--data-dir")
        .arg(repo.join("etc"))
        .arg("--dump-out")
        .arg(pdmp_out)
        .env_remove("EMACSLOADPATH")
        .env_remove("EMACSDATA")
        .env_remove("EMACSDOC")
        .status()
        .map_err(|e| format!("run {} --dump-out: {e}", lisp_bin.display()))?;
    if !status.success() {
        return Err(format!("{} --dump-out failed ({status})", lisp_bin.display()));
    }
    let header = fs::read(pdmp_out).map_err(|e| format!("read {}: {e}", pdmp_out.display()))?;
    if !header.starts_with(b"DUMP") {
        return Err(format!("invalid portable dump header at {}", pdmp_out.display()));
    }
    Ok(())
}

pub(super) fn resolve_source_commit(repo: &Path) -> String {
    if let Ok(explicit) = std::env::var("FEMACS_SOURCE_COMMIT") {
        let trimmed = explicit.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }
    if repo.join(".git").exists()
        && let Ok(out) = Command::new("git")
            .args(["-C", &repo.to_string_lossy(), "rev-parse", "HEAD"])
            .output()
        && out.status.success()
    {
        let head = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if !head.is_empty() {
            return head;
        }
    }
    "exported".to_string()
}

pub(super) fn hash_resource_tree(root: &Path) -> Result<serde_json::Value, String> {
    let root_meta = fs::symlink_metadata(root).map_err(|e| format!("stat {}: {e}", root.display()))?;
    if !root_meta.file_type().is_dir() {
        return Err(format!("resource root is not a directory: {}", root.display()));
    }
    let mut pending = vec![root.to_path_buf()];
    let mut records: Vec<(String, u64, String)> = Vec::new();
    while let Some(dir) = pending.pop() {
        for entry in fs::read_dir(&dir).map_err(|e| format!("readdir {}: {e}", dir.display()))? {
            let entry = entry.map_err(|e| e.to_string())?;
            let p = entry.path();
            let meta = fs::symlink_metadata(&p).map_err(|e| format!("stat {}: {e}", p.display()))?;
            let ft = meta.file_type();
            if ft.is_dir() {
                pending.push(p);
            } else if ft.is_file() {
                let rel_path = p.strip_prefix(root).map_err(|e| e.to_string())?;
                let mut parts = Vec::new();
                for comp in rel_path.components() {
                    let std::path::Component::Normal(seg) = comp else {
                        return Err(format!("unexpected path component in {}", p.display()));
                    };
                    parts.push(
                        seg.to_str()
                            .ok_or_else(|| format!("non-UTF-8 path in {}", p.display()))?,
                    );
                }
                let rel = parts.join("/");
                let digest = crate::tooling::sha256_file(&p)?;
                records.push((rel, meta.len(), digest));
            } else {
                return Err(format!("symlink or special file in resource tree: {}", p.display()));
            }
        }
    }
    records.sort_by(|a, b| a.0.cmp(&b.0));

    let mut tree_hash = Sha256::new();
    let mut total_bytes: u64 = 0;
    for (rel, bytes, digest) in &records {
        total_bytes = total_bytes.saturating_add(*bytes);
        let line = format!("{rel}\0{bytes}\0{digest}\n");
        tree_hash.update(line.as_bytes());
    }
    let tree_sha256 = format!("{:x}", tree_hash.finalize());
    Ok(serde_json::json!({
        "file_count": records.len(),
        "sha256": tree_sha256,
        "total_bytes": total_bytes,
    }))
}

pub(super) fn lisp_package_manifest_value(
    repo: &Path,
    stage: &Path,
) -> Result<serde_json::Value, String> {
    let mut binaries = serde_json::Map::new();
    for name in ["femacs-lisp", "femacs-lisp.pdmp", "http-host"] {
        let p = stage.join("bin").join(name);
        let meta = fs::metadata(&p).map_err(|e| format!("stat {}: {e}", p.display()))?;
        let digest = crate::tooling::sha256_file(&p)?;
        binaries.insert(
            name.to_string(),
            serde_json::json!({
                "bytes": meta.len(),
                "relative_path": format!("bin/{name}"),
                "sha256": digest,
            }),
        );
    }
    let mut resource_trees = serde_json::Map::new();
    for dir_name in [
        "lisp",
        "etc",
        "runtime-tls",
        "runtime-lcms",
        "runtime-image",
        "runtime-treesit",
        "licenses",
    ] {
        let tree_path = stage.join(dir_name);
        if tree_path.is_dir() {
            resource_trees.insert(dir_name.to_string(), hash_resource_tree(&tree_path)?);
        }
    }
    let target = crate::cargo_build::host_target_triple();
    let build_config = "macos-terminal/darwin-arm64/release";
    let source_commit = resolve_source_commit(repo);
    let canonical = serde_json::json!({
        "binaries": binaries,
        "build_config": build_config,
        "resource_trees": resource_trees,
        "source_commit": source_commit,
        "target": target,
    });
    let canonical_bytes = serde_json::to_vec(&canonical).map_err(|e| e.to_string())?;
    let package_sha256 = format!("{:x}", Sha256::digest(&canonical_bytes));
    Ok(serde_json::json!({
        "schema": 1,
        "product": "femacs-lisp",
        "source_commit": source_commit,
        "target": target,
        "build_config": build_config,
        "package_sha256": package_sha256,
        "directories": [
            "bin",
            "lisp",
            "etc",
            "runtime-tls",
            "runtime-lcms",
            "runtime-image",
            "runtime-treesit",
            "licenses"
        ],
        "binaries": binaries,
        "resource_trees": resource_trees,
    }))
}

pub(super) fn make_dir_tree_writable(path: &Path) -> Result<(), String> {
    use std::os::unix::fs::PermissionsExt;
    if !path.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let p = entry.path();
        let meta = fs::symlink_metadata(&p).map_err(|e| e.to_string())?;
        if meta.file_type().is_symlink() {
            continue;
        }
        let mode = meta.permissions().mode();
        if meta.is_dir() {
            if (mode & 0o700) != 0o700 {
                fs::set_permissions(&p, fs::Permissions::from_mode(mode | 0o700))
                    .map_err(|e| e.to_string())?;
            }
            make_dir_tree_writable(&p)?;
        } else if (mode & 0o200) == 0 {
            fs::set_permissions(&p, fs::Permissions::from_mode(mode | 0o200))
                .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

pub(super) fn lisp_package_in(repo: &Path, root: &Path, destination: &Path) -> Result<(), String> {
    let config_file = repo.join("build-support/macos/terminal/config.json");
    let prep = prepare_terminal_env_with_paths(repo, root, &config_file, Some("../bin"))?;
    let root = crate::tooling::guard_scratch(repo, root)?;
    let executable = std::env::current_exe().map_err(|e| e.to_string())?;
    let invoke = |args: &[&str]| -> Result<(), String> {
        let mut command = Command::new(&executable);
        command
            .current_dir(repo)
            .args(args)
            .env_remove("FEMACS_MACOS_GUI")
            .env_remove("FEMACS_TARGET")
            .env_remove("FEMACS_LINK_CHECKING")
            .env("FEMACS_BUILD_TARGET", "macos-terminal")
            .env("FEMACS_CONFIG", "darwin-arm64")
            .env("FEMACS_CONFIG_FILE", &prep.prepared_config)
            .env("FEMACS_CONFIG_RS", prep.config_dir.join("config.rs"))
            .env("FEMACS_EPATHS_RS", prep.config_dir.join("epaths.rs"))
            .env("FEMACS_UNITS_DIR", &prep.objects_dir)
            .env("FEMACS_TOOL_UNITS", &prep.objects_dir)
            .env("FEMACS_BUILD_ROOT", &root)
            .env("FEMACS_PGO", "off")
            .env("FEMACS_RELEASE_BUILD", "1");
        let status = crate::runtime::configured_child(repo, &root, &mut command)?
            .status()
            .map_err(|e| e.to_string())?;
        if status.success() {
            Ok(())
        } else {
            Err(format!("{}: {status}", args.join(" ")))
        }
    };

    invoke(&["gen-config"])?;
    invoke(&["link-lisp-inner"])?;
    resources(repo, "doc", None)?;

    let lisp_bin = root.join("port/bin/femacs-lisp");
    let pdmp_out = root.join("port/bin/femacs-lisp.pdmp");
    run_lisp_self_dump(&lisp_bin, repo, &pdmp_out)?;
    resources(repo, "all", Some(&lisp_bin))?;
    run_lisp_self_dump(&lisp_bin, repo, &pdmp_out)?;

    let http_bin = build_http_host_binary(repo, &root)?;

    // Ensure any stale read-only runtime files in a previous output can be cleanly removed by publish().
    make_dir_tree_writable(destination)?;

    crate::bundle::publish(
        destination,
        |stage| {
            let bin_dir = stage.join("bin");
            fs::create_dir_all(&bin_dir).map_err(|e| e.to_string())?;
            crate::owned_copy::copy_file(&lisp_bin, &bin_dir.join("femacs-lisp"))?;
            crate::owned_copy::copy_file(&pdmp_out, &bin_dir.join("femacs-lisp.pdmp"))?;
            crate::owned_copy::copy_file(&http_bin, &bin_dir.join("http-host"))?;
            codesign_adhoc_and_verify(&bin_dir.join("femacs-lisp"))?;
            codesign_adhoc_and_verify(&bin_dir.join("http-host"))?;

            crate::macos::copy_tree(&repo.join("lisp"), &stage.join("lisp"))?;
            install_library_corrections(repo, &stage.join("lisp"))?;
            crate::macos::copy_tree(&repo.join("etc"), &stage.join("etc"))?;

            crate::tls::copy_product_runtime(
                &root,
                &stage.join("runtime-tls"),
                &stage.join("licenses/tls"),
            )?;
            crate::lcms::copy_product_runtime(
                &root,
                &stage.join("runtime-lcms"),
                &stage.join("licenses/lcms"),
            )?;
            crate::image::copy_product_runtime(
                &root,
                &stage.join("runtime-image"),
                &stage.join("licenses/image"),
            )?;
            crate::treesit::copy_product_runtime(
                &root,
                &stage.join("runtime-treesit"),
                &stage.join("licenses/treesit"),
            )?;
            make_dir_tree_writable(stage)?;
            let manifest = lisp_package_manifest_value(repo, stage)?;
            crate::tooling::write_json(&stage.join("manifest.json"), &manifest)
        },
        |stage| {
            for rel in [
                "bin/femacs-lisp",
                "bin/femacs-lisp.pdmp",
                "bin/http-host",
                "lisp/loadup.el",
                "etc/charsets",
                "manifest.json",
            ] {
                if !stage.join(rel).exists() {
                    return Err(format!("missing staged artifact: {rel}"));
                }
            }
            verify_no_symlinks(stage)?;
            codesign_verify_strict(&stage.join("bin/femacs-lisp"))?;
            codesign_verify_strict(&stage.join("bin/http-host"))?;
            Ok(())
        },
    )?;
    println!("Standalone Lisp package: {}", destination.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static SEQ: AtomicU64 = AtomicU64::new(0);

    struct Scratch(PathBuf);

    impl Scratch {
        fn new() -> Self {
            let n = SEQ.fetch_add(1, Ordering::Relaxed);
            let p = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("build")
                .join(format!("xtask-standalone-unit-{}-{n}", std::process::id()));
            fs::create_dir_all(&p).unwrap();
            Self(p)
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = make_dir_tree_writable(&self.0);
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn verify_no_symlinks_rejects_symlinks_and_accepts_plain_tree() {
        let scratch = Scratch::new();
        let pkg = scratch.0.join("pkg");
        let bin = pkg.join("bin");
        fs::create_dir_all(&bin).unwrap();
        let real = bin.join("femacs-lisp");
        fs::write(&real, b"elf").unwrap();
        assert!(verify_no_symlinks(&pkg).is_ok());

        std::os::unix::fs::symlink(&real, pkg.join("link-out")).unwrap();
        assert!(verify_no_symlinks(&pkg).is_err());
    }

    #[test]
    fn lisp_package_manifest_records_binary_digests_without_host_paths() {
        let scratch = Scratch::new();
        let bin = scratch.0.join("bin");
        let lisp = scratch.0.join("lisp");
        let etc_sub = scratch.0.join("etc/charsets");
        fs::create_dir_all(&bin).unwrap();
        fs::create_dir_all(&lisp).unwrap();
        fs::create_dir_all(&etc_sub).unwrap();
        fs::write(bin.join("femacs-lisp"), b"lisp-bin").unwrap();
        fs::write(bin.join("femacs-lisp.pdmp"), b"DUMPdata").unwrap();
        fs::write(bin.join("http-host"), b"http-bin").unwrap();
        fs::write(lisp.join("loadup.el"), b"(provide 'loadup)\n").unwrap();
        fs::write(etc_sub.join("ascii.map"), b"0x41 0x41\n").unwrap();

        let manifest = lisp_package_manifest_value(&scratch.0, &scratch.0).unwrap();
        assert_eq!(manifest["schema"], 1);
        assert_eq!(manifest["product"], "femacs-lisp");
        assert_eq!(manifest["build_config"], "macos-terminal/darwin-arm64/release");
        assert_eq!(manifest["source_commit"], "exported");
        assert_eq!(manifest["package_sha256"].as_str().unwrap().len(), 64);
        assert_eq!(manifest["binaries"]["femacs-lisp"]["bytes"], 8);
        assert_eq!(
            manifest["binaries"]["femacs-lisp.pdmp"]["relative_path"],
            "bin/femacs-lisp.pdmp"
        );
        assert_eq!(manifest["resource_trees"]["lisp"]["file_count"], 1);
        assert_eq!(manifest["resource_trees"]["etc"]["file_count"], 1);
        let text = serde_json::to_string(&manifest).unwrap();
        assert!(!text.contains("/Users/") && !text.contains("/home/"));

        let orig_lisp_sha = manifest["resource_trees"]["lisp"]["sha256"].as_str().unwrap().to_string();
        let orig_etc_sha = manifest["resource_trees"]["etc"]["sha256"].as_str().unwrap().to_string();
        let orig_pkg_sha = manifest["package_sha256"].as_str().unwrap().to_string();

        fs::write(lisp.join("loadup.el"), b"(provide 'loadup) ; mutated\n").unwrap();
        let mutated = lisp_package_manifest_value(&scratch.0, &scratch.0).unwrap();
        assert_ne!(
            mutated["resource_trees"]["lisp"]["sha256"].as_str().unwrap(),
            orig_lisp_sha
        );
        assert_ne!(mutated["package_sha256"].as_str().unwrap(), orig_pkg_sha);
        assert_eq!(
            mutated["resource_trees"]["etc"]["sha256"].as_str().unwrap(),
            orig_etc_sha
        );
    }
}
