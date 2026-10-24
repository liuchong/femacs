//! Isolated native Cocoa target. No reference object enters the product link.
use super::*;

fn object_dir(repo: &Path, root: &Path) -> PathBuf {
    use std::hash::{Hash, Hasher};
    let mut key=std::collections::hash_map::DefaultHasher::new();
    root.hash(&mut key);
    repo.join(".femacs-cache/macos").join(format!("{:016x}",key.finish()))
}

fn generate_paths(repo: &Path, config: &Path) -> Result<(), String> {
    let mut code=String::from("// Generated target-local bundle paths.\nmod cocoa_target_paths {\nuse super::*;\nunsafe extern \"C\" { #[link_name=\"ns_relocate\"] fn relocate(path: *const c_char) -> *const c_char; }\n");
    for (name, tree) in [("PATH_LOADSEARCH","lisp"),("PATH_EXEC","libexec"),("PATH_DATA","etc"),("PATH_DOC","etc"),("PATH_INFO","info"),("PATH_BITMAPS","etc/images")] {
        let owned=repo.join(tree).display().to_string();
        code.push_str(&format!("#[inline(always)] pub fn {name}() -> *const c_char {{ if will_dump_p() {{ c{owned:?}.as_ptr() }} else {{ unsafe {{ relocate(c\"Contents/Resources/{tree}\".as_ptr()) }} }} }}\n"));
    }
    let lisp=repo.join("lisp").display().to_string();
    for (name,path) in [("PATH_DUMPLOADSEARCH",lisp.clone()),("PATH_DUMPLOADSEARCH_SLASH",format!("{lisp}/"))] {
        code.push_str(&format!("#[inline(always)] pub fn {name}() -> *const c_char {{ c{path:?}.as_ptr() }}\n"));
    }
    code.push_str("#[inline(always)] pub fn PATH_SITELOADSEARCH() -> *const c_char { c\"\".as_ptr() }\n#[inline(always)] pub fn PATH_GAME() -> *const c_char { core::ptr::null() }\n}\npub use cocoa_target_paths::*;\n");
    fs::write(config.join("epaths.rs"),code).map_err(|e|e.to_string())
}

fn run(cmd: &mut Command, description: &str) -> Result<(), String> {
    println!("== {description} ==");
    let status = cmd.status().map_err(|e| format!("{description}: {e}"))?;
    if status.success() { Ok(()) } else { Err(format!("{description}: {status}")) }
}

fn deployment_target() -> Result<String, String> {
    if let Ok(value) = std::env::var("FEMACS_MACOS_DEPLOYMENT_TARGET") { return Ok(value); }
    let output = Command::new("sw_vers").arg("-productVersion").output().map_err(|e| e.to_string())?;
    if !output.status.success() { return Err("cannot read host macOS deployment version".into()); }
    let text = String::from_utf8_lossy(&output.stdout);
    let version: Vec<&str> = text.trim().split('.').take(2).collect();
    if version.is_empty() || version.iter().any(|s| s.is_empty() || !s.chars().all(|c| c.is_ascii_digit())) { return Err("invalid macOS version".into()); }
    Ok(version.join("."))
}

pub(super) fn link_flags() -> Vec<String> {
    let mut flags = vec!["-Wl,-dead_strip".to_string()];
    for framework in ["AppKit", "Foundation", "Carbon", "CoreText", "CoreGraphics", "QuartzCore", "IOKit", "WebKit", "UniformTypeIdentifiers"] {
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
    match build() { Ok(()) => 0, Err(e) => fail(e) }
}

fn build() -> Result<(), String> {
    if !cfg!(target_os = "macos") { return Err("build-macos requires macOS".into()); }
    let repo = canonical(Path::new(REPO));
    let root = std::env::var_os("FEMACS_MACOS_ROOT").map(PathBuf::from)
        .unwrap_or_else(|| repo.join("build/macos"));
    fs::create_dir_all(&root).map_err(|e| e.to_string())?;
    let root = canonical(&root);
    if root.starts_with(repo.join("emacs")) || root == repo { return Err("unsafe Cocoa output root".into()); }
    standalone::check_host()?;
    let reference = root.join("reference"); // legacy environment slot, never read by standalone build
    let config = root.join("config");
    let objects = object_dir(&repo,&root);
    for dir in [&config, &objects] { fs::create_dir_all(dir).map_err(|e| e.to_string())?; }
    standalone::resources(&repo, "base", None)?;
    let frozen = config.join("darwin-cocoa.json");
    fs::copy(repo.join("build-support/macos/cocoa/config.json"), &frozen).map_err(|e| e.to_string())?;
    run(Command::new("python3").current_dir(&repo).arg("tools/gen-macos-runtime.py")
        .arg("--out").arg(&config), "generate target-local Cocoa globals")?;
    let exports = config.join("swift-exports.rs");
    generate_paths(&repo,&config)?;
    run(Command::new("python3").current_dir(&repo).arg("tools/gen-macos-exports.py")
        .arg("--out").arg(&exports), "materialize actual Swift ABI providers")?;
    // Run another xtask process with a target-local environment. Never mutate shared env/config.
    let executable = std::env::current_exe().map_err(|e| e.to_string())?;
    let deployment = deployment_target()?;
    let config_rs = config.join("config.rs");
    let invoke = |name: &str, arguments: &[&str]| -> Result<(), String> {
        run(Command::new(&executable).args(arguments).env("MACOSX_DEPLOYMENT_TARGET", &deployment)
            .env("FEMACS_CONFIG", "darwin-cocoa")
            .env("FEMACS_CONFIG_FILE", &frozen).env("FEMACS_CONFIG_RS", &config_rs)
            .env("FEMACS_GUI_EXPORTS", &exports).env("FEMACS_MACOS_GUI", "1")
            .env("FEMACS_GLOBALS_RS", config.join("globals.rs"))
            .env("FEMACS_SYMBOL_NAMES_RS", config.join("symbol-names.rs"))
            .env("FEMACS_GUI_RUNTIME", config.join("runtime.rs"))
            .env("FEMACS_EPATHS_RS", config.join("epaths.rs")).env("FEMACS_UNITS_DIR",&objects)
            .env("FEMACS_REFERENCE_BUILD", &reference).env("FEMACS_BUILD_ROOT", &root)
            .env("FEMACS_PGO", "off").env("FEMACS_RELEASE_BUILD","1"), name)
    };
    invoke("generate target-local config", &["gen-config"])?;
    invoke("build Cocoa Rust core", &["build-units"])?;
    invoke("build Cocoa ABI providers", &["build-tu", "src/abi_shim.c"])?;
    finish(&repo, &root)
}

/// Continue platform integration using the already-built target core and ABI objects.
pub(super) fn link_macos() -> i32 {
    let repo = canonical(Path::new(REPO));
    let root = std::env::var_os("FEMACS_MACOS_ROOT").map(PathBuf::from).unwrap_or_else(|| repo.join("build/macos"));
    match finish(&repo, &canonical(&root)) { Ok(()) => 0, Err(e) => fail(e) }
}

/// Package an already linked and dumped target; useful after independent runtime gates.
pub(super) fn package_macos() -> i32 {
    let repo = canonical(Path::new(REPO));
    let root = std::env::var_os("FEMACS_MACOS_ROOT").map(PathBuf::from).unwrap_or_else(|| repo.join("build/macos"));
    let root=canonical(&root);
    if root == repo || root.starts_with(repo.join("emacs")) { return fail("unsafe Cocoa output root"); }
    for name in ["emacs", "emacs.pdmp", "bootstrap-emacs.pdmp", "pair.sha256"] {
        if !root.join("port/src").join(name).is_file() { return fail(format!("missing dumped Cocoa input {name}")); }
    }
    match package(&repo, &root) { Ok(()) => 0, Err(e) => fail(e) }
}

fn finish(repo: &Path, root: &Path) -> Result<(), String> {
    if root == repo || root.starts_with(repo.join("emacs")) { return Err("unsafe Cocoa output root".into()); }
    let objects = object_dir(repo,root);
    if !objects.join("abi_shim.o").is_file() { return Err("missing Cocoa ABI object; run build-macos".into()); }
    for required in ["config/config.rs", "config/darwin-cocoa.json"] {
        if !root.join(required).is_file() { return Err(format!("missing Cocoa build input {required}; run build-macos first")); }
    }
    let executable = std::env::current_exe().map_err(|e| e.to_string())?;
    let deployment = deployment_target()?;
    let invoke = |name: &str, arguments: &[&str]| -> Result<(), String> {
        run(Command::new(&executable).args(arguments).env("MACOSX_DEPLOYMENT_TARGET", &deployment)
            .env("FEMACS_CONFIG", "darwin-cocoa")
            .env("FEMACS_CONFIG_FILE", root.join("config/darwin-cocoa.json"))
            .env("FEMACS_CONFIG_RS", root.join("config/config.rs"))
            .env("FEMACS_GUI_EXPORTS", root.join("config/swift-exports.rs"))
            .env("FEMACS_GLOBALS_RS", root.join("config/globals.rs"))
            .env("FEMACS_SYMBOL_NAMES_RS", root.join("config/symbol-names.rs"))
            .env("FEMACS_GUI_RUNTIME", root.join("config/runtime.rs"))
            .env("FEMACS_EPATHS_RS", root.join("config/epaths.rs")).env("FEMACS_UNITS_DIR",&objects)
            .env("FEMACS_MACOS_GUI", "1")
            .env("FEMACS_REFERENCE_BUILD", root.join("reference"))
            .env("FEMACS_BUILD_ROOT", root).env("FEMACS_PGO", "off").env("FEMACS_RELEASE_BUILD","1"), name)
    };
    for name in ["macfont", "nsfns", "nsimage", "nsmenu", "nsselect", "nsterm"] {
        run(Command::new("xcrun").env("MACOSX_DEPLOYMENT_TARGET", &deployment).args(["swiftc", "-parse-as-library", "-O", "-g", "-module-name"])
            .arg(format!("femacs_{name}")).arg("-c")
            .arg(repo.join(format!("platform/macos/{name}.swift")))
            .arg("-o").arg(objects.join(format!("{name}.o"))), &format!("Swift {name}"))?;
    }
    // make -n must be able to enumerate its generated inputs; only the archive member table
    // comes from the existing fixed host build, never its C object code in our link.
    invoke("link complete Cocoa program", &["link-owned"])?;
    standalone::resources(repo, "doc", None)?;
    invoke("bootstrap Cocoa Lisp image", &["dump-bootstrap"])?;
    standalone::resources(repo, "all", Some(&root.join("port/src/emacs")))?;
    invoke("dump final Cocoa Lisp image", &["dump"])?;
    package(&repo, &root)?;
    println!("GUI app: {}", repo.join("dist/macos/Femacs.app").display());
    Ok(())
}

fn copy_tree(from: &Path, to: &Path) -> Result<(), String> {
    fs::create_dir_all(to).map_err(|e| e.to_string())?;
    for entry in fs::read_dir(from).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let dest = to.join(entry.file_name());
        if entry.file_type().map_err(|e| e.to_string())?.is_dir() { copy_tree(&entry.path(), &dest)?; }
        else { fs::copy(entry.path(), dest).map_err(|e| e.to_string())?; }
    }
    Ok(())
}

fn package(repo: &Path, root: &Path) -> Result<(), String> {
    let app=repo.join("dist/macos/Femacs.app");
    crate::bundle::publish(&app, |stage| populate_app(repo,root,stage), |stage| {
        for required in ["Contents/MacOS/Femacs", "Contents/Resources/libexec/Femacs.pdmp", "Contents/Resources/etc/DOC", "Contents/Resources/lisp/loaddefs.el", "Contents/Info.plist"] {
            if !stage.join(required).is_file() {return Err(format!("missing staged app resource: {required}"));}
        }
        run(Command::new("codesign").args(["--verify","--deep","--strict"]).arg(stage), "verify staged Cocoa app")
    })
}

fn populate_app(repo: &Path, root: &Path, app: &Path) -> Result<(), String> {
    // Keep the runnable delivery separate from disposable compiler output.
    let contents = app.join("Contents");
    let macos = contents.join("MacOS");
    let resources = contents.join("Resources");
    fs::create_dir_all(&macos).map_err(|e| e.to_string())?;
    fs::create_dir_all(&resources).map_err(|e| e.to_string())?;
    fs::copy(root.join("port/src/emacs"), macos.join("Femacs")).map_err(|e| e.to_string())?;
    // MacOS contains code only. Data belong in Resources so bundle signing is valid.
    let old_dump=macos.join("Femacs.pdmp");
    if old_dump.exists() { fs::remove_file(old_dump).map_err(|e|e.to_string())?; }
    let libexec=resources.join("libexec");
    fs::create_dir_all(&libexec).map_err(|e|e.to_string())?;
    fs::copy(root.join("port/src/emacs.pdmp"), libexec.join("Femacs.pdmp")).map_err(|e| e.to_string())?;
    for tree in ["lisp", "etc"] { copy_tree(&repo.join(tree), &resources.join(tree))?; }
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
    run(Command::new("codesign").args(["--force", "--sign", "-"]).arg(app), "sign staged Cocoa app")
}

/// Ask make for variable expansion only; no reference compiler/linker is executed.
pub(super) fn active_objects() -> Result<BTreeSet<String>, String> {
    Ok(target_manifest("objects.txt")?.into_iter().collect())
}

/// Optional differential tooling can still query an explicitly configured makefile.
pub(super) fn make_objects(directory: &Path, variable: &str) -> Result<BTreeSet<String>, String> {
    let output = Command::new("make").args(["-n", "-f", "Makefile", "-f", "-", "femacs-object-list"])
        .current_dir(directory).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn()
        .and_then(|mut child| { use std::io::Write; child.stdin.take().unwrap().write_all(format!("femacs-object-list:; @echo FEMACS_OBJECTS=$({variable})\n").as_bytes())?; child.wait_with_output() }).map_err(|e| e.to_string())?;
    if !output.status.success() { return Err(String::from_utf8_lossy(&output.stderr).into_owned()); }
    let text=String::from_utf8_lossy(&output.stdout);
    let line=text.lines().find_map(|s| s.split_once("FEMACS_OBJECTS=").map(|(_,v)| v)).ok_or("no object list")?;
    Ok(line.split_whitespace().map(str::to_string).collect())
}
