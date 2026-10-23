//! Independent Apple Silicon macOS bootstrap. No reference source or binary.
use super::*;

pub(super) fn check_host() -> Result<(), String> {
    if !cfg!(all(target_os="macos",target_arch="aarch64")) {
        return Err("independent targets currently support Apple Silicon macOS only".into());
    }
    Ok(())
}

pub(super) fn resources(repo: &Path, family: &str, program: Option<&Path>) -> Result<(), String> {
    let mut command=Command::new("python3");
    command.current_dir(repo).args(["tools/bootstrap-resources.py","--family",family]);
    if let Some(program)=program { command.arg("--program").arg(program); }
    let status=command.status().map_err(|e|e.to_string())?;
    if status.success() { Ok(()) } else { Err(format!("resource generation {family}: {status}")) }
}

pub(super) fn build_terminal() -> i32 {
    match terminal() { Ok(())=>0, Err(e)=>fail(e) }
}

fn terminal() -> Result<(), String> {
    check_host()?;
    let repo=canonical(Path::new(REPO));
    let root=repo.join("build/terminal");
    let config=root.join("config");
    let objects=root.join("objects");
    fs::create_dir_all(&config).map_err(|e|e.to_string())?;
    resources(&repo,"base",None)?;
    // Generate checkout-local paths for the terminal configuration.
    let mut paths=String::new();
    for (name,rel) in [("PATH_LOADSEARCH","lisp"),("PATH_DUMPLOADSEARCH","lisp"),
        ("PATH_DUMPLOADSEARCH_SLASH","lisp/"),("PATH_EXEC","build/terminal/port/lib-src"),
        ("PATH_DATA","etc"),("PATH_DOC","etc"),("PATH_INFO","info"),("PATH_BITMAPS","etc/images")] {
        paths.push_str(&format!("#[inline(always)] pub fn {name}() -> *const c_char {{ c{:?}.as_ptr() }}\n",repo.join(rel).display().to_string()));
    }
    paths.push_str("pub fn PATH_SITELOADSEARCH() -> *const c_char { c\"\".as_ptr() }\npub fn PATH_GAME() -> *const c_char { core::ptr::null() }\n");
    fs::write(config.join("epaths.rs"),paths).map_err(|e|e.to_string())?;
    let executable=std::env::current_exe().map_err(|e|e.to_string())?;
    let invoke=|args: &[&str]| -> Result<(),String> {
        let status=Command::new(&executable).current_dir(&repo).args(args)
            .env("FEMACS_CONFIG","darwin-arm64")
            .env("FEMACS_CONFIG_FILE",repo.join("build-support/macos/terminal/config.json"))
            .env("FEMACS_CONFIG_RS",config.join("config.rs"))
            .env("FEMACS_EPATHS_RS",config.join("epaths.rs"))
            .env("FEMACS_UNITS_DIR",&objects).env("FEMACS_TOOL_UNITS",&objects)
            .env("FEMACS_BUILD_ROOT",&root).env("FEMACS_PGO","off").env("FEMACS_RELEASE_BUILD","1")
            .status().map_err(|e|e.to_string())?;
        if status.success() { Ok(()) } else { Err(format!("{}: {status}",args.join(" "))) }
    };
    invoke(&["gen-config"])?;
    invoke(&["build-units"])?;
    invoke(&["link-owned"])?;
    resources(&repo,"doc",None)?;
    invoke(&["dump-bootstrap"])?;
    resources(&repo,"all",Some(&root.join("port/src/emacs")))?;
    invoke(&["dump"])?;
    println!("Terminal: {}",root.join("port/src/emacs").display());
    Ok(())
}
