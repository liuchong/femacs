//! Unified Cargo build and link pipeline for `femacs-host` and `femacs-tools`.
//!
//! Replaces per-unit `rustc --emit=obj` compilation, source-graph materialization,
//! `LINK_DRIVER`, and `libgnu-port.a` archiving with workspace Cargo builds while
//! preserving target-local generated configuration, Swift object inputs, and
//! runtime artifact receipts.
#![forbid(unsafe_code)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const DECLARED_HOST_FEATURES: &[&str] = &[
    "have_ns",
    "have_window_system",
    "have_modules",
    "have_native_comp",
    "femacs_gc_observer",
    "femacs_gc_generational",
    "femacs_gc_observer_test_hooks",
];

const TOOL_BINS: &[&str] = &[
    "make-fingerprint",
    "hexl",
    "asset-directory-tool",
    "emacsclient",
];

pub(crate) fn host_target_triple() -> &'static str {
    if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
        "aarch64-apple-darwin"
    } else if cfg!(all(target_os = "macos", target_arch = "x86_64")) {
        "x86_64-apple-darwin"
    } else if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
        "x86_64-unknown-linux-gnu"
    } else {
        "aarch64-unknown-linux-gnu"
    }
}

pub(crate) fn cargo_target_dir() -> PathBuf {
    let root = crate::build_root();
    if root == Path::new("build") {
        PathBuf::from("build/cargo")
    } else {
        root.join("cargo")
    }
}

pub(crate) fn tools_target_dir() -> PathBuf {
    let root = crate::build_root();
    if std::env::var_os("FEMACS_RUNTIME_BINDING").is_some() {
        root.join("cargo-tools")
    } else {
        PathBuf::from("build/cargo-tools")
    }
}

pub(crate) fn extract_cargo_features(cfgs: &[String]) -> Vec<String> {
    let mut features = Vec::new();
    for &decl in DECLARED_HOST_FEATURES {
        let needle = format!("feature=\"{decl}\"");
        if cfgs.iter().any(|c| c == &needle) {
            features.push(decl.to_string());
        }
    }
    features
}

pub(crate) fn build_encoded_rustflags(
    cfgs: &[String],
    checking: bool,
    include_pgo: bool,
) -> String {
    let mut flags: Vec<String> = vec!["-C".into(), "linker=clang".into()];
    if let Ok(v) = std::env::var("FEMACS_OPT_LEVEL") {
        if !v.is_empty() {
            flags.push("-C".into());
            flags.push(format!("opt-level={v}"));
        }
    }
    if let Ok(v) = std::env::var("FEMACS_OVERFLOW_CHECKS") {
        if !v.is_empty() {
            flags.push("-C".into());
            flags.push(format!("overflow-checks={v}"));
        }
    }
    if let Ok(v) = std::env::var("FEMACS_TARGET_CPU") {
        if !v.is_empty() {
            flags.push("-C".into());
            flags.push(format!("target-cpu={v}"));
        }
    }
    if std::env::var("FEMACS_DEBUGINFO")
        .map(|v| !v.is_empty() && v != "0")
        .unwrap_or(false)
    {
        flags.push("-C".into());
        flags.push("debuginfo=1".into());
    }
    if include_pgo {
        flags.extend(crate::pgo_codegen_flags());
    }
    flags.extend(crate::sanitizer_flags());
    for cfg in cfgs {
        flags.push("--cfg".into());
        flags.push(cfg.clone());
    }
    if checking {
        flags.push("--cfg".into());
        flags.push("enable_checking".into());
    }
    flags.join("\x1f")
}

fn walk_rlibs(dir: &Path, depth: usize, prefix: &str, out: &mut Vec<(std::time::SystemTime, PathBuf)>) -> Result<(), String> {
    if depth == 0 || !dir.is_dir() {
        return Ok(());
    }
    for entry in fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let ft = entry.file_type().map_err(|e| e.to_string())?;
        if ft.is_dir() {
            walk_rlibs(&path, depth - 1, prefix, out)?;
            continue;
        }
        let Some(name) = path.file_name().and_then(|s| s.to_str()) else {
            continue;
        };
        if name.starts_with(prefix) && name.ends_with(".rlib") {
            let mtime = entry
                .metadata()
                .and_then(|m| m.modified())
                .unwrap_or(std::time::UNIX_EPOCH);
            out.push((mtime, path));
        }
    }
    Ok(())
}

fn find_newest_rlib(release_dir: &Path, prefix: &str) -> Result<PathBuf, String> {
    let mut matches: Vec<(std::time::SystemTime, PathBuf)> = Vec::new();
    walk_rlibs(release_dir, 5, prefix, &mut matches)?;
    matches.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
    matches
        .pop()
        .map(|(_, p)| p)
        .ok_or_else(|| format!("missing built {prefix}*.rlib under {}", release_dir.display()))
}

/// Build one tool binary from `femacs-tools` (`make-fingerprint`, `hexl`,
/// `asset-directory-tool`, `emacsclient`) into `port/lib-src/<stem>`.
pub(crate) fn build_tool_program(rel: Option<&String>) -> i32 {
    let Some(rel) = rel else {
        return crate::fail("link-program needs a tool path, e.g. lib-src/emacsclient.c");
    };
    let stem = Path::new(rel)
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    if !TOOL_BINS.contains(&stem.as_str()) {
        return crate::fail(format!(
            "unsupported tool binary '{stem}'; expected one of {}",
            TOOL_BINS.join(", ")
        ));
    }
    let cfgs = match crate::config_features() {
        Ok(v) => v,
        Err(e) => return crate::fail(e),
    };
    let (normal, checked) = match crate::runtime::unit_compile_legs(Path::new(crate::REPO)) {
        Ok(selection) => selection,
        Err(error) => return crate::fail(error),
    };
    let checking = !normal && checked;
    let target = host_target_triple();
    let target_dir = tools_target_dir();
    let encoded = build_encoded_rustflags(&cfgs, checking, false);

    let mut cmd = Command::new("cargo");
    cmd.current_dir(crate::REPO)
        .args([
            "build",
            "--release",
            "--target",
            target,
            "-p",
            "femacs-tools",
            "--bin",
            &stem,
        ])
        .arg("--target-dir")
        .arg(&target_dir)
        .env("CARGO_ENCODED_RUSTFLAGS", encoded);
    if !crate::sanitizer_flags().is_empty() {
        cmd.env("RUSTC_BOOTSTRAP", "1");
    }

    println!("== cargo build (femacs-tools): {stem} ==");
    match cmd.status() {
        Ok(s) if s.success() => {}
        Ok(s) => return crate::fail(format!("cargo build of tool {stem} failed ({s})")),
        Err(e) => return crate::fail(format!("cargo build of tool {stem}: {e}")),
    }

    let built = target_dir.join(target).join("release").join(&stem);
    if !built.is_file() {
        return crate::fail(format!("missing built tool binary {}", built.display()));
    }
    let out_path = crate::port_dir().join("lib-src").join(&stem);
    if let Some(parent) = out_path.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            return crate::fail(format!("creating {}: {e}", parent.display()));
        }
    }
    if let Err(e) = crate::owned_copy::copy_file(&built, &out_path) {
        return crate::fail(e);
    }
    if cfg!(target_os = "macos") {
        let signed = Command::new("codesign")
            .args(["-s", "-", "-f"])
            .arg(&out_path)
            .status();
        match signed {
            Ok(s) if s.success() => {}
            Ok(s) => return crate::fail(format!("codesign failed for {} ({s})", out_path.display())),
            Err(e) => return crate::fail(format!("codesign {}: {e}", out_path.display())),
        }
    }
    if std::env::var_os("FEMACS_RUNTIME_BINDING").is_some() {
        let _ = fs::remove_dir_all(&target_dir);
    }
    println!("linked {}", out_path.display());
    0
}

/// Build and link `temacs` through Cargo (`femacs-host --bin temacs`), record runtime
/// link receipts when bound, rewrite the binary fingerprint, and run optional differential gates.
pub(crate) fn link_temacs(arg: Option<&str>) -> i32 {
    let checking = std::env::var("FEMACS_LINK_CHECKING").is_ok_and(|value| value == "1");
    let cocoa = std::env::var_os("FEMACS_MACOS_GUI").is_some();
    let repo = Path::new(crate::REPO);

    let cfgs = match crate::config_features() {
        Ok(v) => v,
        Err(e) => return crate::fail(e),
    };
    let cargo_features = extract_cargo_features(&cfgs);

    let mut link_tokens: Vec<String> = Vec::new();
    let mut swift_objs: Vec<String> = Vec::new();
    if cocoa {
        let objects_manifest = match crate::target_manifest("objects.txt") {
            Ok(v) => v,
            Err(e) => return crate::fail(e),
        };
        for tok in &objects_manifest {
            let Some(stem) = tok.strip_suffix(".o") else {
                continue;
            };
            if ["macfont", "nsfns", "nsimage", "nsmenu", "nsselect", "nsterm"].contains(&stem) {
                let p = crate::units_dir().join(format!("{stem}.o"));
                if !p.is_file() {
                    return crate::fail(format!(
                        "missing compiled Swift object {}; run Swift step first",
                        p.display()
                    ));
                }
                let abs = fs::canonicalize(&p).unwrap_or(p).display().to_string();
                swift_objs.push(abs.clone());
                link_tokens.push(abs);
            }
        }
        link_tokens.extend(crate::macos::link_flags());
    }

    match crate::target_manifest("libraries.txt") {
        Ok(libs) => link_tokens.extend(libs),
        Err(e) => return crate::fail(e),
    }
    for extra in [
        crate::tls::link_flags(repo),
        crate::lcms::link_flags(repo),
        crate::treesit::link_flags(repo),
        crate::image::link_flags(repo),
        crate::runtime::link_flags(repo),
    ] {
        match extra {
            Ok(flags) => link_tokens.extend(flags),
            Err(e) => return crate::fail(e),
        }
    }
    link_tokens.push("-lSystem".into());

    let _ = fs::create_dir_all(crate::port_dir());
    let port_root = fs::canonicalize(crate::port_dir()).unwrap_or_else(|_| crate::port_dir());
    if let Err(e) = crate::port_tree(&port_root) {
        return crate::fail(format!("building the port's tree layout: {e}"));
    }

    let target = host_target_triple();
    let target_dir = cargo_target_dir();
    let encoded = build_encoded_rustflags(&cfgs, checking, true);

    let mut cmd = Command::new("cargo");
    cmd.current_dir(repo)
        .args([
            "rustc",
            "--release",
            "--target",
            target,
            "-p",
            "femacs-host",
            "--bin",
            "temacs",
        ])
        .arg("--target-dir")
        .arg(&target_dir)
        .env("CARGO_ENCODED_RUSTFLAGS", encoded);
    if !cargo_features.is_empty() {
        cmd.arg("--features").arg(cargo_features.join(","));
    }
    if !crate::sanitizer_flags().is_empty() {
        cmd.env("RUSTC_BOOTSTRAP", "1");
    }
    cmd.arg("--");
    for tok in &link_tokens {
        cmd.arg("-C").arg(format!("link-arg={tok}"));
    }

    println!(
        "== cargo rustc (femacs-host --bin temacs): features=[{}], checking={checking} ==",
        cargo_features.join(",")
    );
    match cmd.status() {
        Ok(s) if s.success() => {}
        Ok(s) => return crate::fail(format!("cargo rustc for temacs failed ({s})")),
        Err(e) => return crate::fail(format!("cargo rustc for temacs: {e}")),
    }

    let built_temacs = target_dir.join(target).join("release").join("temacs");
    if !built_temacs.is_file() {
        return crate::fail(format!(
            "missing Cargo-built temacs at {}",
            built_temacs.display()
        ));
    }

    let out_path = port_root.join("src").join("temacs");
    if let Err(e) = crate::owned_copy::copy_file(&built_temacs, &out_path) {
        return crate::fail(e);
    }

    if std::env::var_os("FEMACS_RUNTIME_BINDING").is_some() {
        let release_dir = target_dir.join(target).join("release");
        let objects_out = crate::units_dir();
        let load_out = crate::load_dir();
        if let Err(e) = fs::create_dir_all(&objects_out)
            .and_then(|()| fs::create_dir_all(&load_out))
        {
            return crate::fail(format!("creating runtime link receipt dirs: {e}"));
        }
        let suffix = if checking { ".checking.rlib" } else { ".rlib" };
        let mut recorded_objs = swift_objs;
        for (prefix, stem) in [
            ("libfemacs_pure-", "libfemacs_pure"),
            ("libfemacs_vm-", "libfemacs_vm"),
        ] {
            let src_rlib = match find_newest_rlib(&release_dir, prefix) {
                Ok(p) => p,
                Err(e) => return crate::fail(e),
            };
            let dst = objects_out.join(format!("{stem}{suffix}"));
            if let Err(e) = crate::owned_copy::copy_file(&src_rlib, &dst) {
                return crate::fail(e);
            }
            let abs = fs::canonicalize(&dst).unwrap_or(dst).display().to_string();
            recorded_objs.push(abs);
        }
        let sys_rlib = match find_newest_rlib(&release_dir, "libfemacs_sys-") {
            Ok(p) => p,
            Err(e) => return crate::fail(e),
        };
        let archive_dst = load_out.join(format!("libfemacs_sys{suffix}"));
        if let Err(e) = crate::owned_copy::copy_file(&sys_rlib, &archive_dst) {
            return crate::fail(e);
        }
        let empty_members: Vec<String> = Vec::new();
        if let Err(e) =
            crate::runtime::record_link_inputs(repo, &recorded_objs, &empty_members, &archive_dst)
        {
            return crate::fail(e);
        }
        let _ = fs::remove_dir_all(&target_dir);
    }

    if let Err(e) = crate::finalize_binary(&out_path) {
        return crate::fail(e);
    }
    if arg == Some("--owned") {
        println!("standalone Cargo link complete");
        return 0;
    }

    println!("== coding eol gate ==");
    let mut coder = Command::new("python3");
    coder
        .current_dir(repo)
        .args(["tools/check-coding-eol.py", "--port"])
        .arg(&out_path)
        .arg("--reference")
        .arg("build/emacs-ref/src/emacs");
    crate::set_profile_env(&mut coder, "gate");
    match coder.status() {
        Ok(s) if s.success() => {
            println!("coding eol: every case converts as the reference does");
            0
        }
        Ok(_) => crate::fail("a decoded text differs from the reference's for the same bytes"),
        Err(e) => crate::fail(e),
    }
}

/// Check the Cargo workspace crates in both normal and `--cfg enable_checking` modes.
pub(crate) fn check_workspace_crates() -> Result<(), String> {
    let cfgs = crate::config_features()?;
    let cargo_features = extract_cargo_features(&cfgs);
    let target = host_target_triple();
    let target_dir = cargo_target_dir();

    for checking in [false, true] {
        let encoded = build_encoded_rustflags(&cfgs, checking, false);
        let mut cmd = Command::new("cargo");
        cmd.current_dir(crate::REPO)
            .args([
                "check",
                "--target",
                target,
                "-p",
                "femacs-pure",
                "-p",
                "femacs-sys",
                "-p",
                "femacs-vm",
                "-p",
                "femacs-host",
                "-p",
                "femacs-tools",
            ])
            .arg("--target-dir")
            .arg(&target_dir)
            .env("CARGO_ENCODED_RUSTFLAGS", encoded);
        if !cargo_features.is_empty() {
            cmd.arg("--features")
                .arg(format!("femacs-host/{}", cargo_features.join(",femacs-host/")));
        }
        let status = cmd.status().map_err(|e| e.to_string())?;
        if !status.success() {
            return Err(format!(
                "cargo check failed (checking={checking}): {status}"
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_declared_host_features_deterministically() {
        let _timer = crate::test_support::CaseTimer::new("extracts_declared_host_features_deterministically");
        let cfgs = vec![
            "feature=\"have_ns\"".to_string(),
            "feature=\"not_have_android\"".to_string(),
            "feature=\"have_window_system\"".to_string(),
            "feature=\"have_modules\"".to_string(),
            "feature=\"femacs_gc_observer\"".to_string(),
        ];
        assert_eq!(
            extract_cargo_features(&cfgs),
            vec!["have_ns", "have_window_system", "have_modules", "femacs_gc_observer"]
        );
    }

    #[test]
    fn encodes_rustflags_with_unit_separator_and_checking_gate() {
        let _timer = crate::test_support::CaseTimer::new("encodes_rustflags_with_unit_separator_and_checking_gate");
        let cfgs = vec!["feature=\"have_modules\"".to_string()];
        let normal = build_encoded_rustflags(&cfgs, false, false);
        assert!(normal.contains("-C\x1flinker=clang"));
        assert!(normal.contains("--cfg\x1ffeature=\"have_modules\""));
        assert!(!normal.contains("enable_checking"));

        let checked = build_encoded_rustflags(&cfgs, true, false);
        assert!(checked.ends_with("--cfg\x1fenable_checking"));
    }

    #[test]
    fn finds_rlibs_across_deps_and_nested_build_out_layouts() {
        let _timer = crate::test_support::CaseTimer::new("finds_rlibs_across_deps_and_nested_build_out_layouts");
        let base = std::env::temp_dir();
        let pid = std::process::id();
        let mut dir = None;
        for attempt in 0..64u32 {
            let cand = base.join(format!("femacs-rlib-test-{pid}-{attempt}"));
            match fs::create_dir(&cand) {
                Ok(()) => {
                    dir = Some(cand);
                    break;
                }
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(e) => panic!("create_dir failed: {e}"),
            }
        }
        let dir = dir.expect("claimed unique test directory");
        let nested = dir.join("build/femacs-vm/abc123/out");
        let deps = dir.join("deps");
        fs::create_dir_all(&nested).unwrap();
        fs::create_dir_all(&deps).unwrap();
        let nested_rlib = nested.join("libfemacs_vm-abc123.rlib");
        let deps_rlib = deps.join("libfemacs_sys-def456.rlib");
        fs::write(&nested_rlib, b"vm").unwrap();
        fs::write(&deps_rlib, b"sys").unwrap();

        assert_eq!(find_newest_rlib(&dir, "libfemacs_vm-").unwrap(), nested_rlib);
        assert_eq!(find_newest_rlib(&dir, "libfemacs_sys-").unwrap(), deps_rlib);
        assert!(find_newest_rlib(&dir, "libfemacs_missing-").is_err());
        let _ = fs::remove_dir_all(&dir);
    }
}
