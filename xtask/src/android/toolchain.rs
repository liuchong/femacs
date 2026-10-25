//! One numeric SDK/NDK and current-LTS Java discovery policy; no old JBR fallback.
use super::AndroidError;
use serde::Serialize;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};
#[derive(Clone, Debug)]
pub(crate) struct ToolchainOptions {
    pub sdk: Option<PathBuf>,
    pub ndk: Option<PathBuf>,
    pub jdk: Option<PathBuf>,
    pub kotlinc: Option<PathBuf>,
    pub stdlib: Option<PathBuf>,
    pub api: u32,
    pub ndk_api: u32,
}
impl Default for ToolchainOptions {
    fn default() -> Self {
        Self {
            sdk: None,
            ndk: None,
            jdk: None,
            kotlinc: None,
            stdlib: None,
            api: 36,
            ndk_api: 21,
        }
    }
}
#[derive(Clone, Debug, Serialize)]
pub(crate) struct Jdk {
    pub home: PathBuf,
    pub major: u32,
    pub version: String,
}
#[derive(Clone, Debug, Serialize)]
pub(crate) struct AndroidToolchain {
    pub sdk_root: PathBuf,
    pub build_tools: PathBuf,
    pub ndk_root: PathBuf,
    pub clang: PathBuf,
    pub readelf: PathBuf,
    pub nm: PathBuf,
    pub jdk: Jdk,
    pub kotlinc: PathBuf,
    pub kotlin_stdlib: PathBuf,
    pub android_jar: PathBuf,
}
pub(crate) fn parse_numeric_version(text: &str) -> Result<Vec<u64>, String> {
    if text.is_empty() {
        return Err("empty version".into());
    }
    text.split('.')
        .map(|s| {
            if s.is_empty() || !s.bytes().all(|c| c.is_ascii_digit()) {
                Err(format!("not a stable numeric version: {text}"))
            } else {
                s.parse::<u64>().map_err(|e| e.to_string())
            }
        })
        .collect()
}
pub(crate) fn highest_version_dir(root: &Path) -> Result<PathBuf, String> {
    let mut entries = fs::read_dir(root)
        .map_err(|e| format!("{}: {e}", root.display()))?
        .filter_map(Result::ok)
        .filter(|e| e.path().is_dir())
        .filter_map(|e| {
            parse_numeric_version(&e.file_name().to_string_lossy())
                .ok()
                .map(|v| (v, e.path()))
        })
        .collect::<Vec<_>>();
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    entries.pop().map(|(_, p)| p).ok_or_else(|| {
        format!(
            "no installed stable numeric version under {}",
            root.display()
        )
    })
}
fn env_path(name: &str) -> Option<PathBuf> {
    std::env::var_os(name)
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
}
fn path_program(name: &str) -> Option<PathBuf> {
    std::env::split_paths(&std::env::var_os("PATH")?)
        .map(|p| p.join(name))
        .find(|p| p.is_file())
}
fn studio_compiler() -> Option<PathBuf> {
    if let Some(studio) = env_path("STUDIO") {
        let compiler = studio.join("plugins/Kotlin/kotlinc/bin/kotlinc");
        return compiler.is_file().then_some(compiler);
    }
    if !cfg!(target_os = "macos") {
        return None;
    }
    // Ask the installed-app metadata index, without embedding a machine installation path.
    let result = Command::new("mdfind")
        .arg("kMDItemCFBundleIdentifier == \"com.google.android.studio\"")
        .output()
        .ok()?;
    if !result.status.success() || result.stdout.len() > 65536 {
        return None;
    }
    let mut candidates = String::from_utf8(result.stdout)
        .ok()?
        .lines()
        .map(|line| PathBuf::from(line).join("Contents/plugins/Kotlin/kotlinc/bin/kotlinc"))
        .filter(|path| path.is_file())
        .collect::<Vec<_>>();
    candidates.sort();
    candidates.dedup();
    if candidates.len() == 1 {
        candidates.pop()
    } else {
        None
    }
}

fn need(path: PathBuf, what: &str) -> Result<PathBuf, AndroidError> {
    if !path.is_file() {
        Err(AndroidError::Input(format!(
            "missing {what}: {}; install the required toolchain or pass its explicit path",
            path.display()
        )))
    } else {
        // Multicall tools (notably llvm-readelf -> llvm-readobj) select their
        // interface from argv[0]. Normalize the parent without replacing the leaf.
        let parent = path
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or(Path::new("."));
        let name = path
            .file_name()
            .ok_or_else(|| AndroidError::Input("tool path has no name".into()))?;
        fs::canonicalize(parent)
            .map(|base| base.join(name))
            .map_err(|e| AndroidError::Input(e.to_string()))
    }
}
fn short_version(program: &Path, args: &[&str]) -> Result<String, AndroidError> {
    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(|e| AndroidError::Input(e.to_string()))?;
    if !output.status.success() {
        return Err(AndroidError::Input(format!(
            "version probe failed for {}",
            program.display()
        )));
    }
    if output.stdout.len() + output.stderr.len() > 65536 {
        return Err(AndroidError::Input(
            "unexpected oversized tool version response".into(),
        ));
    }
    Ok(format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
    .trim()
    .into())
}
pub(crate) fn discover_toolchain(
    opts: &ToolchainOptions,
) -> Result<AndroidToolchain, AndroidError> {
    if opts.api < 21 || opts.ndk_api < 21 {
        return Err(AndroidError::Input(
            "API below the retained21 floor is unsupported".into(),
        ));
    }
    let sdk = opts
        .sdk
        .clone()
        .or_else(|| env_path("ANDROID_HOME"))
        .or_else(|| env_path("ANDROID_SDK_ROOT"))
        .unwrap_or_else(|| {
            PathBuf::from(std::env::var_os("HOME").unwrap_or_default()).join("Library/Android/sdk")
        });
    let sdk_root = fs::canonicalize(&sdk)
        .map_err(|e| AndroidError::Input(format!("Android SDK {}: {e}", sdk.display())))?;
    let build_tools =
        highest_version_dir(&sdk_root.join("build-tools")).map_err(AndroidError::Input)?;
    for name in ["aapt2", "d8", "zipalign", "apksigner"] {
        need(build_tools.join(name), name)?;
    }
    let ndk = opts
        .ndk
        .clone()
        .or_else(|| env_path("ANDROID_NDK_ROOT"))
        .map(Ok)
        .unwrap_or_else(|| highest_version_dir(&sdk_root.join("ndk")))
        .map_err(AndroidError::Input)?;
    let ndk_root = fs::canonicalize(&ndk).map_err(|e| AndroidError::Input(e.to_string()))?;
    let mut prebuilt = fs::read_dir(ndk_root.join("toolchains/llvm/prebuilt"))
        .map_err(|e| AndroidError::Input(e.to_string()))?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.join("bin/clang").is_file())
        .collect::<Vec<_>>();
    prebuilt.sort();
    let prebuilt = prebuilt
        .into_iter()
        .next()
        .ok_or_else(|| AndroidError::Input("NDK host compiler missing".into()))?;
    let clang = need(
        prebuilt.join(format!("bin/aarch64-linux-android{}-clang", opts.ndk_api)),
        "Android target clang",
    )?;
    let readelf = need(prebuilt.join("bin/llvm-readelf"), "NDK readelf")?;
    let nm = need(prebuilt.join("bin/llvm-nm"), "NDK nm")?;
    let java_home = opts
        .jdk
        .clone()
        .or_else(|| env_path("FEMACS_JAVA_HOME"))
        .or_else(|| env_path("JAVA_HOME"))
        .or_else(|| {
            let output = Command::new("brew")
                .args(["--prefix", "openjdk@25"])
                .output()
                .ok()?;
            if !output.status.success() || output.stdout.len() > 65536 {
                return None;
            }
            let path = PathBuf::from(String::from_utf8_lossy(&output.stdout).trim());
            path.join("bin/javac").is_file().then_some(path)
        })
        .ok_or_else(|| AndroidError::Input("latest LTS JDK25 is required; pass --jdk".into()))?;
    let home = fs::canonicalize(java_home).map_err(|e| AndroidError::Input(e.to_string()))?;
    for program in ["java", "javac", "keytool", "jar"] {
        need(home.join("bin").join(program), program)?;
    }
    let version = short_version(&home.join("bin/java"), &["-version"])?;
    let major = version
        .split('"')
        .nth(1)
        .and_then(|s| s.split('.').next())
        .and_then(|s| s.parse::<u32>().ok())
        .ok_or_else(|| AndroidError::Input(format!("cannot parse Java version: {version}")))?;
    if major != 25 {
        return Err(AndroidError::Input(format!(
            "latest LTS JDK25 required, found {major}; no old-JDK fallback"
        )));
    }
    let compiler = opts
        .kotlinc
        .clone()
        .or_else(|| env_path("KOTLINC"))
        .or_else(|| path_program("kotlinc"))
        .or_else(studio_compiler)
        .ok_or_else(|| AndroidError::Input("Kotlin compiler missing; pass --kotlinc".into()))?;
    let kotlinc = need(compiler, "kotlinc")?;
    let stdlib = opts
        .stdlib
        .clone()
        .or_else(|| env_path("KOTLIN_STDLIB"))
        .unwrap_or_else(|| {
            kotlinc
                .parent()
                .expect("compiler bin")
                .join("../lib/kotlin-stdlib.jar")
        });
    let kotlin_stdlib = need(stdlib, "Kotlin stdlib")?;
    let android_jar = need(
        sdk_root.join(format!("platforms/android-{}/android.jar", opts.api)),
        "android.jar",
    )?;
    Ok(AndroidToolchain {
        sdk_root,
        build_tools,
        ndk_root,
        clang,
        readelf,
        nm,
        jdk: Jdk {
            home,
            major,
            version,
        },
        kotlinc,
        kotlin_stdlib,
        android_jar,
    })
}

#[cfg(all(test, unix))]
mod invocation_tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;
    #[test]
    fn executable_alias_keeps_argv_zero_mode_through_symlink() {
        let _timer = crate::test_support::CaseTimer::new("executable_alias_keeps_argv_zero_mode");
        let repo = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let run = crate::tooling::owned_new_output(repo, None, "tool-alias-test").unwrap();
        struct Scratch(PathBuf);
        impl Drop for Scratch {
            fn drop(&mut self) {
                let _ = std::fs::remove_dir_all(&self.0);
            }
        }
        let _cleanup = Scratch(run.path.clone());
        let bin = run.path.join("bin with spaces");
        fs::create_dir(&bin).unwrap();
        let target = bin.join("llvm-readobj");
        fs::write(&target,"#!/usr/bin/env python3\nimport pathlib,sys\nprint('readelf mode' if pathlib.Path(sys.argv[0]).name=='llvm-readelf' else 'wrong multicall mode')\n").unwrap();
        fs::set_permissions(&target, fs::Permissions::from_mode(0o755)).unwrap();
        let alias = bin.join("llvm-readelf");
        std::os::unix::fs::symlink(&target, &alias).unwrap();
        let selected = need(alias, "test readelf").unwrap();
        let output = Command::new(selected).output().unwrap();
        assert!(output.status.success());
        assert_eq!(
            String::from_utf8(output.stdout).unwrap().trim(),
            "readelf mode"
        );
    }
}
