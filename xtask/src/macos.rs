//! Isolated native Cocoa target. No reference object enters the product link.
use super::*;

fn object_dir(repo: &Path, root: &Path) -> Result<PathBuf, String> {
    if let Some(binding) = crate::runtime::verify_selected_root(repo, root)? {
        if binding["product"] != "cocoa" {
            return Err("Cocoa object root has a different product binding".into());
        }
    }
    let objects = root.join("objects");
    if objects.is_symlink() {
        return Err("Cocoa object root is a symlink".into());
    }
    Ok(objects)
}

fn generate_paths(repo: &Path, config: &Path) -> Result<(), String> {
    let mut code = String::from(
        "// Generated target-local bundle paths.\nmod cocoa_target_paths {\nuse super::*;\nunsafe extern \"C\" { #[link_name=\"ns_relocate\"] fn relocate(path: *const c_char) -> *const c_char; }\n",
    );
    for (name, tree) in [
        ("PATH_LOADSEARCH", "lisp"),
        ("PATH_EXEC", "libexec"),
        ("PATH_DATA", "etc"),
        ("PATH_DOC", "etc"),
        ("PATH_INFO", "info"),
        ("PATH_BITMAPS", "etc/images"),
    ] {
        let owned = if tree == "lisp" { standalone::lisp_paths(repo, false) }
                    else { repo.join(tree).display().to_string() };
        code.push_str(&format!("#[inline(always)] pub fn {name}() -> *const c_char {{ if will_dump_p() {{ c{owned:?}.as_ptr() }} else {{ /* SAFETY: Relocates bundle-relative NUL-terminated path into static path cache. */ unsafe {{ relocate(c\"Contents/Resources/{tree}\".as_ptr()) }} }} }}\n"));
    }
    for (name, path) in [
        ("PATH_DUMPLOADSEARCH", standalone::lisp_paths(repo, false)),
        ("PATH_DUMPLOADSEARCH_SLASH", standalone::lisp_paths(repo, true)),
    ] {
        code.push_str(&format!(
            "#[inline(always)] pub fn {name}() -> *const c_char {{ c{path:?}.as_ptr() }}\n"
        ));
    }
    code.push_str("#[inline(always)] pub fn PATH_SITELOADSEARCH() -> *const c_char { c\"\".as_ptr() }\n#[inline(always)] pub fn PATH_GAME() -> *const c_char { core::ptr::null() }\n}\npub use cocoa_target_paths::*;\n");
    fs::write(config.join("epaths.rs"), code).map_err(|e| e.to_string())
}

fn run(cmd: &mut Command, description: &str) -> Result<(), String> {
    println!("== {description} ==");
    let status = cmd.status().map_err(|e| format!("{description}: {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("{description}: {status}"))
    }
}

pub(super) fn deployment_target() -> Result<String, String> {
    if let Ok(value) = std::env::var("FEMACS_MACOS_DEPLOYMENT_TARGET") {
        return Ok(value);
    }
    let output = Command::new("sw_vers")
        .arg("-productVersion")
        .output()
        .map_err(|e| e.to_string())?;
    if !output.status.success() {
        return Err("cannot read host macOS deployment version".into());
    }
    let text = String::from_utf8_lossy(&output.stdout);
    let version: Vec<&str> = text.trim().split('.').take(2).collect();
    if version.is_empty()
        || version
            .iter()
            .any(|s| s.is_empty() || !s.chars().all(|c| c.is_ascii_digit()))
    {
        return Err("invalid macOS version".into());
    }
    Ok(version.join("."))
}

pub(super) fn link_flags() -> Vec<String> {
    let mut flags = vec!["-Wl,-dead_strip".to_string()];
    for framework in [
        "AppKit",
        "Foundation",
        "Carbon",
        "CoreText",
        "CoreGraphics",
        "QuartzCore",
        "IOKit",
        "WebKit",
        "UniformTypeIdentifiers",
    ] {
        flags.extend(["-framework".to_string(), framework.to_string()]);
    }
    if let Ok(o) = Command::new("xcrun").args(["--find", "swiftc"]).output() {
        let path = PathBuf::from(String::from_utf8_lossy(&o.stdout).trim());
        if let Some(bin) = path.parent() {
            let lib = bin.join("../lib/swift/macosx");
            flags.push(format!("-L{}", lib.display()));
            flags.push(format!("-Wl,-rpath,{}", lib.display()));
        }
    }
    flags
}

pub(super) fn build_macos() -> i32 {
    match build() {
        Ok(()) => 0,
        Err(e) => fail(e),
    }
}

fn build() -> Result<(), String> {
    if !cfg!(target_os = "macos") {
        return Err("build-macos requires macOS".into());
    }
    let repo = canonical(Path::new(REPO));
    let root = std::env::var_os("FEMACS_MACOS_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo.join("build/macos"));
    build_configured(&repo, &root, &repo.join("build-support/macos/cocoa/config.json"),
        &repo.join("dist/macos/Femacs.app"))
}

pub(super) fn build_configured(repo: &Path, root: &Path, selected_config: &Path,
                                destination: &Path) -> Result<(), String> {
    standalone::check_host()?;
    let repo = canonical(repo);
    fs::create_dir_all(&root).map_err(|e| e.to_string())?;
    let root = canonical(&root);
    if root.starts_with(repo.join("emacs")) || root == repo {
        return Err("unsafe Cocoa output root".into());
    }
    standalone::check_host()?;
    let reference = root.join("reference"); // legacy environment slot, never read by standalone build
    let config = root.join("config");
    fs::create_dir_all(&config).map_err(|e| e.to_string())?;
    standalone::resources(&repo, "base", None)?;
    let frozen = config.join("darwin-cocoa.json");
    let tls_config = config.join("darwin-cocoa-tls.json");
    crate::tls::prepare(&repo, &root, selected_config, &tls_config)?;
    let lcms_config = config.join("darwin-cocoa-lcms.json");
    crate::lcms::prepare(&repo, &root, &tls_config, &lcms_config)?;
    let image_config = config.join("darwin-cocoa-image.json");
    crate::image::prepare(&repo, &root, &lcms_config, &image_config)?;
    crate::treesit::prepare(&repo, &root, &image_config, &frozen)?;
    // A selected object root requires the prepared target copy to be present.
    let objects = object_dir(&repo, &root)?;
    fs::create_dir_all(&objects).map_err(|e| e.to_string())?;
    run(
        Command::new("python3")
            .current_dir(&repo)
            .arg("tools/gen-macos-runtime.py")
            .arg("--out")
            .arg(&config),
        "generate target-local Cocoa globals",
    )?;
    let exports = config.join("swift-exports.rs");
    generate_paths(&repo, &config)?;
    run(
        Command::new("python3")
            .current_dir(&repo)
            .arg("tools/gen-macos-exports.py")
            .arg("--out")
            .arg(&exports),
        "materialize actual Swift ABI providers",
    )?;
    // Run another xtask process with a target-local environment. Never mutate shared env/config.
    let executable = std::env::current_exe().map_err(|e| e.to_string())?;
    let deployment = deployment_target()?;
    let config_rs = config.join("config.rs");
    let invoke = |name: &str, arguments: &[&str]| -> Result<(), String> {
        run(
            crate::runtime::configured_child(&repo, &root, Command::new(&executable)
                .args(arguments)
                .env("MACOSX_DEPLOYMENT_TARGET", &deployment)
                .env("FEMACS_CONFIG", "darwin-cocoa")
                .env("FEMACS_CONFIG_FILE", &frozen)
                .env("FEMACS_CONFIG_RS", &config_rs)
                .env("FEMACS_GUI_EXPORTS", &exports)
                .env("FEMACS_MACOS_GUI", "1")
                .env("FEMACS_GLOBALS_RS", config.join("globals.rs"))
                .env("FEMACS_SYMBOL_NAMES_RS", config.join("symbol-names.rs"))
                .env("FEMACS_GUI_RUNTIME", config.join("runtime.rs"))
                .env("FEMACS_EPATHS_RS", config.join("epaths.rs"))
                .env("FEMACS_UNITS_DIR", &objects)
                .env("FEMACS_REFERENCE_BUILD", &reference)
                .env("FEMACS_BUILD_ROOT", &root)
                .env("FEMACS_PGO", "off")
                .env("FEMACS_RELEASE_BUILD", "1"))?,
            name,
        )
    };
    invoke("generate target-local config", &["gen-config"])?;
    finish_configured(&repo, &root, destination)
}

/// Continue platform integration using the already-built target core and ABI objects.
pub(super) fn link_macos() -> i32 {
    let repo = canonical(Path::new(REPO));
    let root = std::env::var_os("FEMACS_MACOS_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo.join("build/macos"));
    match finish(&repo, &canonical(&root)) {
        Ok(()) => 0,
        Err(e) => fail(e),
    }
}

/// Package an already linked and dumped target; useful after independent runtime gates.
pub(super) fn package_macos() -> i32 {
    let repo = canonical(Path::new(REPO));
    let root = std::env::var_os("FEMACS_MACOS_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo.join("build/macos"));
    let root = canonical(&root);
    if root == repo || root.starts_with(repo.join("emacs")) {
        return fail("unsafe Cocoa output root");
    }
    for name in ["emacs", "emacs.pdmp", "bootstrap-emacs.pdmp", "pair.sha256"] {
        if !root.join("port/src").join(name).is_file() {
            return fail(format!("missing dumped Cocoa input {name}"));
        }
    }
    match package(&repo, &root) {
        Ok(()) => 0,
        Err(e) => fail(e),
    }
}

fn finish(repo: &Path, root: &Path) -> Result<(), String> {
    let destination = crate::runtime::continuation_destination(repo, root)?
        .unwrap_or_else(|| repo.join("dist/macos/Femacs.app"));
    finish_configured(repo, root, &destination)
}

fn finish_configured(repo: &Path, root: &Path, destination: &Path) -> Result<(), String> {
    if root == repo || root.starts_with(repo.join("emacs")) {
        return Err("unsafe Cocoa output root".into());
    }
    let objects = object_dir(repo, root)?;
    fs::create_dir_all(&objects).map_err(|e| e.to_string())?;
    for required in ["config/config.rs", "config/darwin-cocoa.json"] {
        if !root.join(required).is_file() {
            return Err(format!(
                "missing Cocoa build input {required}; run build-macos first"
            ));
        }
    }
    let executable = std::env::current_exe().map_err(|e| e.to_string())?;
    let deployment = deployment_target()?;
    let invoke = |name: &str, arguments: &[&str]| -> Result<(), String> {
        run(
            crate::runtime::configured_child(repo, root, Command::new(&executable)
                .args(arguments)
                .env("MACOSX_DEPLOYMENT_TARGET", &deployment)
                .env("FEMACS_CONFIG", "darwin-cocoa")
                .env("FEMACS_CONFIG_FILE", root.join("config/darwin-cocoa.json"))
                .env("FEMACS_CONFIG_RS", root.join("config/config.rs"))
                .env("FEMACS_GUI_EXPORTS", root.join("config/swift-exports.rs"))
                .env("FEMACS_GLOBALS_RS", root.join("config/globals.rs"))
                .env(
                    "FEMACS_SYMBOL_NAMES_RS",
                    root.join("config/symbol-names.rs"),
                )
                .env("FEMACS_GUI_RUNTIME", root.join("config/runtime.rs"))
                .env("FEMACS_EPATHS_RS", root.join("config/epaths.rs"))
                .env("FEMACS_UNITS_DIR", &objects)
                .env("FEMACS_TOOL_UNITS", &objects)
                .env("FEMACS_MACOS_GUI", "1")
                .env("FEMACS_REFERENCE_BUILD", root.join("reference"))
                .env("FEMACS_BUILD_TARGET", "macos-cocoa")
                .env("FEMACS_BUILD_ROOT", root)
                .env("FEMACS_PGO", "off")
                .env("FEMACS_RELEASE_BUILD", "1"))?,
            name,
        )
    };
    // Swift builtin records must select the same native ABI as the Rust core.
    let target_config: serde_json::Value = serde_json::from_slice(
        &fs::read(root.join("config/darwin-cocoa.json")).map_err(|e| e.to_string())?,
    ).map_err(|e| e.to_string())?;
    let native_comp = match target_config["features"].get("have_native_comp") {
        Some(value) => match (value.as_str(), value.as_i64()) {
            (Some("1"), _) | (_, Some(1)) => true,
            (Some("0" | "undefined"), _) | (_, Some(0)) => false,
            _ => return Err("invalid Cocoa native feature value".into()),
        },
        None => false,
    };
    let support = repo.join("build-support/macos/cocoa");
    let sources = crate::product_sources::ProductSources::resolve(
        repo,
        &read_manifest(&support.join("objects.txt"))?,
        &read_manifest(&support.join("lib-objects.txt"))?,
        true,
    )?;
    for source in sources.swift {
        let name = Path::new(&source)
            .file_stem()
            .and_then(|stem| stem.to_str())
            .ok_or_else(|| format!("{source}: invalid Swift module name"))?;
        run(
            Command::new("xcrun")
                .env("MACOSX_DEPLOYMENT_TARGET", &deployment)
                .args(["swiftc", "-parse-as-library", "-O", "-g"])
                .args(if native_comp { &["-D", "HAVE_NATIVE_COMP"][..] } else { &[][..] })
                .arg("-module-name")
                .arg(format!("femacs_{name}"))
                .arg("-c")
                .arg(repo.join(&source))
                .arg("-o")
                .arg(objects.join(format!("{name}.o"))),
            &format!("Swift {name}"),
        )?;
    }
    // make -n must be able to enumerate its generated inputs; only the archive member table
    // comes from the existing fixed host build, never its C object code in our link.
    invoke("link complete Cocoa program", &["link-owned"])?;
    invoke("link Cocoa companion client", &["link-program", "lib-src/emacsclient.c"])?;
    invoke("link Cocoa companion hexl", &["link-program", "lib-src/hexl.c"])?;
    standalone::resources(repo, "doc", None)?;
    invoke("bootstrap Cocoa Lisp image", &["dump-bootstrap"])?;
    standalone::resources(repo, "all", Some(&root.join("port/src/emacs")))?;
    invoke("dump final Cocoa Lisp image", &["dump"])?;
    package_configured(&repo, &root, destination)?;
    println!("GUI app: {}", destination.display());
    Ok(())
}

pub(super) fn copy_tree(from: &Path, to: &Path) -> Result<(), String> {
    fs::create_dir_all(to).map_err(|e| e.to_string())?;
    for entry in fs::read_dir(from).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let dest = to.join(entry.file_name());
        if entry.file_type().map_err(|e| e.to_string())?.is_dir() {
            copy_tree(&entry.path(), &dest)?;
        } else {
            crate::owned_copy::copy_file(&entry.path(), &dest)?;
            // Lisp compares source/bytecode modification times when loading.
            // Packaging order must not make unchanged sources look newer.
            let modified = entry.metadata().map_err(|e| e.to_string())?
                .modified().map_err(|e| e.to_string())?;
            fs::File::open(&dest).map_err(|e| e.to_string())?
                .set_times(fs::FileTimes::new().set_modified(modified))
                .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

fn package(repo: &Path, root: &Path) -> Result<(), String> {
    let destination = crate::runtime::continuation_destination(repo, root)?
        .unwrap_or_else(|| repo.join("dist/macos/Femacs.app"));
    package_configured(repo, root, &destination)
}

fn package_configured(repo: &Path, root: &Path, destination: &Path) -> Result<(), String> {
    crate::runtime::verify_selected_root(repo, root)?;
    let info = crate::manuals::for_product(repo, root)?;
    crate::bundle::publish(
        destination,
        |stage| populate_app(repo, root, &info, stage),
        |stage| {
            for required in [
                "Contents/MacOS/Femacs",
                "Contents/MacOS/bin/emacsclient",
                "Contents/MacOS/bin/hexl",
                "Contents/Resources/libexec/emacsclient",
                "Contents/Resources/libexec/hexl",
                "Contents/Resources/libexec/Femacs.pdmp",
                "Contents/Resources/etc/DOC",
                "Contents/Resources/lisp/loaddefs.el",
                "Contents/Info.plist",
                "Contents/Resources/info/dir",
                "Contents/Resources/info/emacs.info",
                "Contents/Resources/info/elisp.info",
                "Contents/Resources/info/org.info",
                "Contents/Resources/info/modus-themes.info",
            ] {
                if !stage.join(required).is_file() {
                    return Err(format!("missing staged app resource: {required}"));
                }
            }
            crate::manuals::verify_set(
                &stage.join("Contents/Resources/info"),
                &repo.join("build-support/manuals.json"),
            )?;
            run(
                Command::new("codesign")
                    .args(["--verify", "--deep", "--strict"])
                    .arg(stage),
                "verify staged Cocoa app",
            )
        },
    )
}

fn populate_app(repo: &Path, root: &Path, info: &Path, app: &Path) -> Result<(), String> {
    // Keep the runnable delivery separate from disposable compiler output.
    let contents = app.join("Contents");
    let macos = contents.join("MacOS");
    let resources = contents.join("Resources");
    fs::create_dir_all(&macos).map_err(|e| e.to_string())?;
    fs::create_dir_all(&resources).map_err(|e| e.to_string())?;
    crate::owned_copy::copy_file(&root.join("port/src/emacs"), &macos.join("Femacs"))?;
    // MacOS contains code only. Data belong in Resources so bundle signing is valid.
    let old_dump = macos.join("Femacs.pdmp");
    if old_dump.exists() {
        fs::remove_file(old_dump).map_err(|e| e.to_string())?;
    }
    let libexec = resources.join("libexec");
    fs::create_dir_all(&libexec).map_err(|e| e.to_string())?;
    crate::owned_copy::copy_file(&root.join("port/src/emacs.pdmp"), &libexec.join("Femacs.pdmp"))?;
    let client = macos.join("bin/emacsclient");
    fs::create_dir_all(client.parent().ok_or("missing client parent")?)
        .map_err(|e| e.to_string())?;
    crate::owned_copy::copy_file(&root.join("port/lib-src/emacsclient"), &client)?;
    #[cfg(unix)]
    std::os::unix::fs::symlink("../../MacOS/bin/emacsclient", libexec.join("emacsclient"))
        .map_err(|e| e.to_string())?;
    #[cfg(not(unix))]
    return Err("Cocoa client packaging requires Unix symbolic links".into());
    run(
        Command::new("codesign")
            .args(["--force", "--sign", "-"])
            .arg(&client),
        "sign Cocoa companion client",
    )?;
    let hexl_bin = macos.join("bin/hexl");
    crate::owned_copy::copy_file(&root.join("port/lib-src/hexl"), &hexl_bin)?;
    #[cfg(unix)]
    std::os::unix::fs::symlink("../../MacOS/bin/hexl", libexec.join("hexl"))
        .map_err(|e| e.to_string())?;
    run(
        Command::new("codesign")
            .args(["--force", "--sign", "-"])
            .arg(&hexl_bin),
        "sign Cocoa companion hexl",
    )?;
    for tree in ["lisp", "etc"] {
        copy_tree(&repo.join(tree), &resources.join(tree))?;
    }
    standalone::install_library_corrections(repo, &resources.join("lisp"))?;
    crate::manuals::copy_set(info, &resources.join("info"))?;
    standalone::stage_module_api(repo, &resources.join("include"))?;
    if root.join("runtime-native").is_dir() {
        crate::runtime::verify_selected_root(repo, root)?;
        copy_tree(&root.join("runtime-native"), &contents.join("Frameworks/runtime-native"))?;
        copy_tree(&root.join("licenses"), &resources.join("licenses"))?;
    }
    crate::tls::copy_product_runtime(root, &contents.join("Frameworks/runtime-tls"),
                                     &resources.join("licenses/tls"))?;
    crate::lcms::copy_product_runtime(root, &contents.join("Frameworks/runtime-lcms"),
                                     &resources.join("licenses/lcms"))?;
    crate::image::copy_product_runtime(root, &contents.join("Frameworks/runtime-image"),
                                      &resources.join("licenses/image"))?;
    crate::treesit::copy_product_runtime(root, &contents.join("Frameworks/runtime-treesit"),
                                        &resources.join("licenses/treesit"))?;
    // Data are genuine owned inputs; the GUI binary remains the Rust/Swift Cocoa link.
    fs::write(contents.join("Info.plist"), r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
<key>CFBundleName</key><string>Femacs</string>
<key>CFBundleDisplayName</key><string>Femacs</string>
<key>CFBundleIdentifier</key><string>org.femacs.editor</string>
<key>CFBundleExecutable</key><string>Femacs</string>
<key>CFBundlePackageType</key><string>APPL</string>
<key>CFBundleVersion</key><string>2</string>
<key>NSPrincipalClass</key><string>NSApplication</string>
<key>NSHighResolutionCapable</key><true/>
</dict></plist>
"#).map_err(|e| e.to_string())?;
    fe_packaging::install_macos_icon(app)?;
    run(
        Command::new("codesign")
            .args(["--force", "--sign", "-"])
            .arg(app),
        "sign staged Cocoa app",
    )
}
