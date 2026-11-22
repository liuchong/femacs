//! Independent Apple Silicon macOS bootstrap. No reference source or binary.
#![forbid(unsafe_code)]
use super::*;

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

pub(super) fn terminal_configured(repo: &Path, root: &Path, publish_info: bool, config_file: &Path) -> Result<(), String> {
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
    resources(&repo, "base", None)?;
    stage_module_api(repo, &root.join("port/include"))?;
    // Generate checkout-local paths for the terminal configuration.
    let mut paths = String::new();
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
        paths.push_str(&format!(
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
    paths.push_str("pub fn PATH_SITELOADSEARCH() -> *const c_char { c\"\".as_ptr() }\npub fn PATH_GAME() -> *const c_char { core::ptr::null() }\n");
    if crate::runtime::verify_selected_root(repo, &root)?.is_some() {
        paths = crate::runtime::terminal_paths_source(repo, &root);
    }
    fs::write(config.join("epaths.rs"), paths).map_err(|e| e.to_string())?;
    let executable = std::env::current_exe().map_err(|e| e.to_string())?;
    let invoke = |args: &[&str]| -> Result<(), String> {
        let mut command = Command::new(&executable);
        command.current_dir(&repo)
            .args(args)
            .env_remove("FEMACS_MACOS_GUI")
            .env_remove("FEMACS_TARGET")
            .env_remove("FEMACS_LINK_CHECKING")
            .env("FEMACS_BUILD_TARGET", "macos-terminal")
            .env("FEMACS_CONFIG", "darwin-arm64")
            .env(
                "FEMACS_CONFIG_FILE",
                &prepared_config,
            )
            .env("FEMACS_CONFIG_RS", config.join("config.rs"))
            .env("FEMACS_EPATHS_RS", config.join("epaths.rs"))
            .env("FEMACS_UNITS_DIR", &objects)
            .env("FEMACS_TOOL_UNITS", &objects)
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
