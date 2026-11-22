//! Independent Android target and typed compatibility routes.
#![forbid(unsafe_code)]
use crate::tooling::{self, RunDirectory, StepPlan, StepReceipt};
use serde_json::{Value, json};
use std::{
    ffi::OsString,
    fmt, fs,
    path::{Path, PathBuf},
};
mod audit;
pub(crate) mod config;
mod gate;
mod package;
pub(crate) mod toolchain;
pub(crate) use config::{
    BuildTarget, UnitsManifest, build_target, config_source, expected_objects, load_units,
    support_directory,
};
pub(crate) use package::{PackageReport, package_apk, validate_package_report};
pub(crate) use toolchain::{
    AndroidToolchain, Jdk, ToolchainOptions, discover_toolchain, parse_numeric_version,
};
#[derive(Debug)]
pub(crate) enum AndroidError {
    Input(String),
    Command(String),
    Verification(String),
}
impl fmt::Display for AndroidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Input(s) | Self::Command(s) | Self::Verification(s) => f.write_str(s),
        }
    }
}
impl AndroidError {
    fn code(&self) -> i32 {
        match self {
            Self::Input(_) => 2,
            Self::Command(_) => 3,
            Self::Verification(_) => 4,
        }
    }
}
fn input(e: impl fmt::Display) -> AndroidError {
    AndroidError::Input(e.to_string())
}
#[derive(Clone, Debug)]
pub(crate) struct BuildOptions {
    pub run_root: Option<PathBuf>,
    pub support_dir: PathBuf,
    pub tools: ToolchainOptions,
    pub package: bool,
    pub assets: bool,
    pub debuggable: bool,
}
#[derive(Clone, Debug)]
pub(crate) struct PackageOptions {
    pub run_root: Option<PathBuf>,
    pub native_so: Option<PathBuf>,
    pub assets: bool,
    pub debuggable: bool,
    pub tools: ToolchainOptions,
}
#[derive(Debug)]
pub(crate) struct ConfigOptions {
    pub reference: PathBuf,
    pub build_dir: PathBuf,
    pub out: PathBuf,
    pub tools: ToolchainOptions,
}
pub(crate) type BuildReport = Value;
pub(crate) type ConfigReport = Value;
pub(crate) use audit::measure_reference_config;
fn execute(
    repo: &Path,
    run: &RunDirectory,
    tools: &AndroidToolchain,
    steps: &mut Vec<StepReceipt>,
    id: &str,
    program: impl Into<OsString>,
    args: Vec<OsString>,
) -> Result<StepReceipt, AndroidError> {
    let plan = StepPlan {
        id: id.into(),
        program: program.into(),
        args,
        cwd: repo.to_owned(),
        env: vec![
            ("JAVA_HOME".into(), tools.jdk.home.clone().into_os_string()),
            (
                "ANDROID_HOME".into(),
                tools.sdk_root.clone().into_os_string(),
            ),
            (
                "ANDROID_NDK_ROOT".into(),
                tools.ndk_root.clone().into_os_string(),
            ),
        ],
    };
    let receipt = tooling::execute_step(&plan, run).map_err(AndroidError::Command)?;
    let result = tooling::require_success(&receipt).map_err(AndroidError::Command);
    steps.push(receipt.clone());
    result?;
    Ok(receipt)
}
fn target_env(run: &Path, support: &Path, tools: &AndroidToolchain) -> Vec<(OsString, OsString)> {
    vec![
        ("FEMACS_BUILD_TARGET".into(), "android-arm64".into()),
        ("FEMACS_TARGET".into(), "aarch64-linux-android".into()),
        (
            "FEMACS_ANDROID_CC".into(),
            tools.clang.clone().into_os_string(),
        ),
        (
            "FEMACS_CONFIG_FILE".into(),
            support.join("config.json").into_os_string(),
        ),
        (
            "FEMACS_CONFIG_RS".into(),
            run.join("config/config.rs").into_os_string(),
        ),
        (
            "FEMACS_UNITS_DIR".into(),
            run.join("units").into_os_string(),
        ),
        (
            "FEMACS_TOOL_UNITS".into(),
            run.join("units").into_os_string(),
        ),
        (
            "FEMACS_BUILD_ROOT".into(),
            run.join("native").into_os_string(),
        ),
        ("FEMACS_PGO".into(), "off".into()),
    ]
}
fn native_command(
    repo: &Path,
    run: &RunDirectory,
    env: &[(OsString, OsString)],
    steps: &mut Vec<StepReceipt>,
    id: &str,
    args: Vec<OsString>,
) -> Result<(), AndroidError> {
    let plan = StepPlan {
        id: id.into(),
        program: std::env::current_exe().map_err(input)?.into_os_string(),
        args,
        cwd: repo.to_owned(),
        env: env.to_vec(),
    };
    let receipt = tooling::execute_step(&plan, run).map_err(AndroidError::Command)?;
    let result = tooling::require_success(&receipt).map_err(AndroidError::Command);
    steps.push(receipt);
    result
}
pub(crate) fn build_android(opts: &BuildOptions) -> Result<BuildReport, AndroidError> {
    let repo = config::repo();
    let tools = discover_toolchain(&opts.tools)?;
    let units = load_units(&opts.support_dir).map_err(input)?;
    let cfg = fs::read(opts.support_dir.join("config.json")).map_err(input)?;
    let cfg: Value = serde_json::from_slice(&cfg).map_err(input)?;
    if cfg["abi"] != "arm64-v8a" || cfg["ndk_api"] != 21 || !cfg["macros"].is_object() {
        return Err(AndroidError::Input(
            "owned arm64 target configuration metadata invalid".into(),
        ));
    }
    let rust = std::process::Command::new("rustc")
        .current_dir(&repo)
        .args([
            "--print",
            "target-libdir",
            "--target",
            "aarch64-linux-android",
        ])
        .output()
        .map_err(input)?;
    if !rust.status.success() || rust.stdout.len() > 65536 {
        return Err(AndroidError::Input(
            "pinned Rust Android target unavailable; install its rust-std target".into(),
        ));
    }
    let libdir = PathBuf::from(String::from_utf8_lossy(&rust.stdout).trim());
    if !fs::read_dir(&libdir)
        .map_err(input)?
        .filter_map(Result::ok)
        .any(|e| e.file_name().to_string_lossy().starts_with("libcore-"))
    {
        return Err(AndroidError::Input("pinned Android core archive missing; rustup target add aarch64-linux-android for the pin".into()));
    }
    let run = tooling::owned_new_output(&repo, opts.run_root.as_deref(), "android-build")
        .map_err(input)?;
    fs::create_dir(run.path.join("config")).map_err(input)?;
    let mut steps = vec![];
    let result = (|| -> Result<Value, AndroidError> {
        let list = run.path.join("units.txt");
        let text = units
            .core
            .iter()
            .chain(&units.library)
            .map(String::as_str)
            .collect::<Vec<_>>()
            .join("\n")
            + "\n";
        fs::write(&list, text).map_err(input)?;
        execute(
            &repo,
            &run,
            &tools,
            &mut steps,
            "resources-base",
            "python3",
            vec![
                repo.join("tools/bootstrap-resources.py").into_os_string(),
                "--family".into(),
                "base".into(),
            ],
        )?;
        let env = target_env(&run.path, &opts.support_dir, &tools);
        native_command(
            &repo,
            &run,
            &env,
            &mut steps,
            "config",
            vec!["gen-config".into()],
        )?;
        native_command(
            &repo,
            &run,
            &env,
            &mut steps,
            "units",
            vec!["check-units".into(), list.into_os_string()],
        )?;
        let normal = expected_objects(&units, &run.path.join("units"), false).map_err(input)?;
        let checking = expected_objects(&units, &run.path.join("units"), true).map_err(input)?;
        native_command(
            &repo,
            &run,
            &env,
            &mut steps,
            "link-normal",
            vec![
                "link-shared".into(),
                "aarch64-linux-android".into(),
                "--units-manifest".into(),
                opts.support_dir.join("core-units.txt").into_os_string(),
            ],
        )?;
        let so = run
            .path
            .join("native/port/aarch64-linux-android/libemacs.so");
        let normal_copy = run.path.join("libemacs.so");
        fs::copy(&so, &normal_copy).map_err(input)?;
        let mut checked_env = env.clone();
        checked_env.push(("FEMACS_LINK_CHECKING".into(), "1".into()));
        native_command(
            &repo,
            &run,
            &checked_env,
            &mut steps,
            "link-checking",
            vec![
                "link-shared".into(),
                "aarch64-linux-android".into(),
                "--units-manifest".into(),
                opts.support_dir.join("core-units.txt").into_os_string(),
            ],
        )?;
        let checked = run.path.join("libemacs-checking.so");
        fs::copy(&so, &checked).map_err(input)?;
        let normal_gate =
            tooling::owned_new_output(&repo, Some(&run.path.join("normal-gate")), "normal-gate")
                .map_err(input)?;
        let checking_gate = tooling::owned_new_output(
            &repo,
            Some(&run.path.join("checking-gate")),
            "checking-gate",
        )
        .map_err(input)?;
        let native = gate::native(&repo, &normal_gate, &tools, &normal_copy, &mut steps)?;
        let checking_native = gate::native(&repo, &checking_gate, &tools, &checked, &mut steps)?;
        let checks = gate::static_checks(&repo, &run, &tools, &opts.support_dir, &mut steps)?;
        let package = if opts.package {
            Some(package_apk(&PackageOptions {
                run_root: Some(run.path.join("package")),
                native_so: Some(normal_copy.clone()),
                assets: opts.assets,
                debuggable: opts.debuggable,
                tools: opts.tools.clone(),
            })?)
        } else {
            None
        };
        let asset_root = if opts.assets && !opts.package {
            Some(build_assets(&repo, &run, &tools, &mut steps)?)
        } else {
            None
        };
        Ok(
            json!({"schema":"femacs-android-build-v1","status":"passed","steps":steps,"toolchain":tools,"inputs":{"config":tooling::sha256_file(&opts.support_dir.join("config.json")).map_err(input)?,"core":tooling::sha256_file(&opts.support_dir.join("core-units.txt")).map_err(input)?,"library":tooling::sha256_file(&opts.support_dir.join("lib-units.txt")).map_err(input)?},"objects":{"normal":normal,"checking":checking},"generated_config_sha256":tooling::sha256_file(&run.path.join("config/config.rs")).map_err(input)?,"products":{"native":{"path":normal_copy,"sha256":native["sha256"]},"checking":{"path":checked,"sha256":tooling::sha256_file(&checked).map_err(input)?}},"native_gates":{"normal":native,"checking":checking_native},"static":checks,"package":package,"assets":asset_root,"errors":[]}),
        )
    })();
    match result {
        Ok(report) => {
            tooling::write_json(&run.path.join("report.json"), &report).map_err(input)?;
            Ok(report)
        }
        Err(e) => {
            tooling::write_json(&run.path.join("report.json"),&json!({"schema":"femacs-android-build-v1","status":"failed","steps":steps,"errors":[e.to_string()]})).map_err(input)?;
            Err(e)
        }
    }
}
fn build_assets(
    repo: &Path,
    run: &RunDirectory,
    tools: &AndroidToolchain,
    steps: &mut Vec<StepReceipt>,
) -> Result<PathBuf, AndroidError> {
    let host = run.path.join("host-tool");
    crate::standalone::terminal_in(repo, &host, false).map_err(AndroidError::Command)?;
    let env = vec![
        ("FEMACS_BUILD_TARGET".into(), "macos-terminal".into()),
        (
            "FEMACS_CONFIG_FILE".into(),
            repo.join("build-support/macos/terminal/config.json")
                .into_os_string(),
        ),
        (
            "FEMACS_CONFIG_RS".into(),
            host.join("config/config.rs").into_os_string(),
        ),
        (
            "FEMACS_EPATHS_RS".into(),
            host.join("config/epaths.rs").into_os_string(),
        ),
        (
            "FEMACS_UNITS_DIR".into(),
            host.join("objects").into_os_string(),
        ),
        (
            "FEMACS_TOOL_UNITS".into(),
            host.join("objects").into_os_string(),
        ),
        ("FEMACS_BUILD_ROOT".into(), host.clone().into_os_string()),
        ("FEMACS_PGO".into(), "off".into()),
    ];
    native_command(
        repo,
        run,
        &env,
        steps,
        "asset-tool",
        vec![
            "link-program".into(),
            "lib-src/asset-directory-tool.c".into(),
        ],
    )?;
    let tool = host.join("port/lib-src/asset-directory-tool");
    let assets = run.path.join("pack");
    execute(
        repo,
        run,
        tools,
        steps,
        "assets",
        "python3",
        vec![
            repo.join("tools/gen-android-assets.py").into_os_string(),
            "--asset-directory-tool".into(),
            tool.into_os_string(),
            "--info".into(),
            crate::manuals::for_product(repo, &host)
                .map_err(AndroidError::Command)?
                .into_os_string(),
            "--out".into(),
            assets.clone().into_os_string(),
        ],
    )?;
    Ok(assets)
}
pub(crate) fn command(args: &[String]) -> i32 {
    let result = (|| -> Result<Value, AndroidError> {
        let mode = args
            .first()
            .ok_or_else(|| input("android needs build/package/kotlin/config"))?;
        let mut tools = ToolchainOptions::default();
        let mut out = None;
        let mut support = support_directory(BuildTarget::AndroidArm64);
        let mut native = None;
        let mut assets = false;
        let mut package = false;
        let mut debug = std::env::var("FEMACS_ANDROID_DEBUGGABLE").is_ok_and(|s| s == "1");
        let mut reference = None;
        let mut build_dir = None;
        let mut prereqs = false;
        let mut extra = vec![];
        let mut reference_java = false;
        let mut at = 1;
        while at < args.len() {
            let key = &args[at];
            match key.as_str() {
                "--assets" => {
                    assets = true;
                    at += 1;
                    continue;
                }
                "--package" => {
                    package = true;
                    at += 1;
                    continue;
                }
                "--debuggable" => {
                    debug = true;
                    at += 1;
                    continue;
                }
                "--check-prereqs" => {
                    prereqs = true;
                    at += 1;
                    continue;
                }
                "--reference-java" => {
                    reference_java = true;
                    at += 1;
                    continue;
                }
                _ => {}
            }
            let value = args
                .get(at + 1)
                .ok_or_else(|| input(format!("{key} needs a value")))?;
            let path = PathBuf::from(value);
            match key.as_str() {
                "--out" | "--keep" => out = Some(path),
                "--apk" => {
                    out = Some(path);
                    package = true;
                }
                "--support-dir" => support = path,
                "--native" | "--native-so" => native = Some(path),
                "--sdk" => tools.sdk = Some(path),
                "--ndk" => tools.ndk = Some(path),
                "--jdk" => tools.jdk = Some(path),
                "--kotlinc" => tools.kotlinc = Some(path),
                "--stdlib" => tools.stdlib = Some(path),
                "--api" => tools.api = value.parse().map_err(input)?,
                "--ndk-api" => tools.ndk_api = value.parse().map_err(input)?,
                "--reference" => reference = Some(path),
                "--build-dir" => build_dir = Some(path),
                "--source" => extra.push(path),
                "--source-root" => {
                    if fs::canonicalize(path).map_err(input)?
                        != config::repo().join("platform/android/kotlin")
                    {
                        return Err(input("only owned Kotlin source root is supported"));
                    }
                }
                _ => return Err(input(format!("unknown Android option {key}"))),
            }
            at += 2;
        }
        match mode.as_str() {
            "build" => build_android(&BuildOptions {
                run_root: out,
                support_dir: support,
                tools,
                package,
                assets,
                debuggable: debug,
            }),
            "package" => package_apk(&PackageOptions {
                run_root: out,
                native_so: native,
                assets,
                debuggable: debug,
                tools,
            }),
            "kotlin" if package => package_apk(&PackageOptions {
                run_root: out,
                native_so: native,
                assets,
                debuggable: debug,
                tools,
            }),
            "kotlin" => {
                let repo = config::repo();
                let tc = discover_toolchain(&tools)?;
                let run =
                    tooling::owned_new_output(&repo, out.as_deref(), "kotlin").map_err(input)?;
                let mut steps = vec![];
                let classes = package::kotlin_into(
                    &repo,
                    &run,
                    &tc,
                    native.as_deref(),
                    debug,
                    &extra,
                    reference_java,
                    &mut steps,
                )?;
                let report = json!({"schema":"femacs-android-kotlin-v1","status":"passed","toolchain":tc,"steps":steps,"classes":classes,"reference_java_audit":reference_java,"device_ready":false});
                tooling::write_json(&run.path.join("report.json"), &report).map_err(input)?;
                Ok(report)
            }
            "config" if prereqs => Ok(
                json!({"schema":"femacs-android-prerequisites-v1","toolchain":discover_toolchain(&tools)?,"status":"present; configure not executed"}),
            ),
            "config" => measure_reference_config(&ConfigOptions {
                reference: reference
                    .ok_or_else(|| input("config audit needs explicit --reference"))?,
                build_dir: build_dir
                    .ok_or_else(|| input("config audit needs fresh --build-dir"))?,
                out: out.ok_or_else(|| input("config audit needs fresh --out"))?,
                tools,
            }),
            _ => Err(input("unknown Android command")),
        }
    })();
    match result {
        Ok(report) => {
            println!(
                "{}",
                serde_json::to_string_pretty(&report).expect("JSON report")
            );
            0
        }
        Err(e) => {
            eprintln!("{e}");
            e.code()
        }
    }
}
pub(crate) fn platforms(args: &[String]) -> i32 {
    gate::command(args)
}
#[cfg(test)]
#[path = "tests.rs"]
mod tests;

#[cfg(test)]
mod review_tests;
