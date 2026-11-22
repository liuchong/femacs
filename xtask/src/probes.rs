//! Typed compiler/audit probes. Every invocation owns its materialization and evidence.
#![forbid(unsafe_code)]
use crate::tooling::{self, RunDirectory, StepPlan, StepReceipt};
use serde::Serialize;
use serde_json::json;
use std::{
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
};
#[derive(Clone, Copy, Debug, Serialize)]
pub(crate) enum ProbeKind {
    Differential,
    RustRun,
    RustMetadata,
    AbiSyntax,
    Jni,
    Miri,
    Bringup,
    Kotlin,
}
#[derive(Clone, Copy, Debug, Serialize)]
pub(crate) enum ReferenceRequirement {
    None,
    Tree,
    ConfiguredTree,
}
#[derive(Clone, Debug, Serialize)]
pub(crate) struct ProbeSpec {
    pub name: &'static str,
    pub kind: ProbeKind,
    pub rust_source: Option<&'static str>,
    pub c_source: Option<&'static str>,
    pub generator: Option<&'static str>,
    pub reference: ReferenceRequirement,
}
#[derive(Default, Clone, Debug)]
pub(crate) struct ProbeOptions {
    pub out: Option<PathBuf>,
    pub reference_source: Option<PathBuf>,
    pub reference_build: Option<PathBuf>,
    pub ndk: Option<PathBuf>,
    pub extra_sources: Vec<PathBuf>,
    pub write_baseline: Option<PathBuf>,
    pub reference_java: bool,
}
#[derive(Debug, Serialize)]
pub(crate) struct ProbeReport {
    schema: &'static str,
    name: String,
    kind: ProbeKind,
    status: String,
    steps: Vec<StepReceipt>,
    inputs: serde_json::Value,
    comparisons: Vec<serde_json::Value>,
    errors: Vec<String>,
    output: PathBuf,
}
pub(crate) fn inventory() -> Vec<ProbeSpec> {
    use ProbeKind::*;
    use ReferenceRequirement::{ConfiguredTree, Tree};
    vec![
        ProbeSpec {
            name: "inline",
            kind: Differential,
            rust_source: Some("spikes/inline_probe.rs"),
            c_source: Some("spikes/inline_probe.c"),
            generator: None,
            reference: ConfiguredTree,
        },
        ProbeSpec {
            name: "accessor",
            kind: Differential,
            rust_source: Some("spikes/accessor_probe.rs"),
            c_source: Some("spikes/accessor_probe.c"),
            generator: None,
            reference: ConfiguredTree,
        },
        ProbeSpec {
            name: "write",
            kind: Differential,
            rust_source: Some("spikes/write_probe.rs"),
            c_source: Some("spikes/write_probe.c"),
            generator: None,
            reference: ConfiguredTree,
        },
        ProbeSpec {
            name: "va",
            kind: Differential,
            rust_source: Some("spikes/va_probe.rs"),
            c_source: Some("spikes/va_probe.c"),
            generator: None,
            reference: ReferenceRequirement::None,
        },
        ProbeSpec {
            name: "syshdrs",
            kind: Differential,
            rust_source: Some("spikes/syshdrs_probe.rs"),
            c_source: Some("spikes/syshdrs_probe.c"),
            generator: None,
            reference: ReferenceRequirement::None,
        },
        ProbeSpec {
            name: "gmp-abi",
            kind: AbiSyntax,
            rust_source: None,
            c_source: None,
            generator: Some("tools/gen-gmp-abi-probe.py"),
            reference: Tree,
        },
        ProbeSpec {
            name: "lisp-abi",
            kind: AbiSyntax,
            rust_source: None,
            c_source: None,
            generator: Some("tools/gen-lisp-abi-probe.py"),
            reference: ConfiguredTree,
        },
        ProbeSpec {
            name: "fam",
            kind: RustRun,
            rust_source: Some("spikes/fam_probe.rs"),
            c_source: None,
            generator: None,
            reference: ReferenceRequirement::None,
        },
        ProbeSpec {
            name: "shadow",
            kind: RustRun,
            rust_source: Some("spikes/shadow_probe.rs"),
            c_source: None,
            generator: None,
            reference: ReferenceRequirement::None,
        },
        ProbeSpec {
            name: "macro",
            kind: RustMetadata,
            rust_source: Some("spikes/macro_probe.rs"),
            c_source: None,
            generator: None,
            reference: ReferenceRequirement::None,
        },
        ProbeSpec {
            name: "jni",
            kind: Jni,
            rust_source: Some("src/include/jni.rs"),
            c_source: None,
            generator: Some("tools/probe-inputs.py"),
            reference: ReferenceRequirement::None,
        },
        ProbeSpec {
            name: "miri",
            kind: Miri,
            rust_source: Some("spikes/miri/src/main.rs"),
            c_source: None,
            generator: None,
            reference: ReferenceRequirement::None,
        },
        ProbeSpec {
            name: "bringup",
            kind: Bringup,
            rust_source: Some("spikes/layout.rs"),
            c_source: Some("spikes/layout_c.c"),
            generator: Some("tools/gen-layout-fields.py"),
            reference: ConfiguredTree,
        },
        ProbeSpec {
            name: "kotlin",
            kind: Kotlin,
            rust_source: None,
            c_source: None,
            generator: None,
            reference: ReferenceRequirement::None,
        },
    ]
}
fn source_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace")
        .to_owned()
}
fn step(repo: &Path, id: &str, program: impl Into<OsString>, args: Vec<OsString>) -> StepPlan {
    StepPlan {
        id: id.into(),
        program: program.into(),
        args,
        cwd: repo.to_owned(),
        env: Vec::new(),
    }
}
fn words(items: &[&str]) -> Vec<OsString> {
    items.iter().map(OsString::from).collect()
}
fn references(repo: &Path, opts: &ProbeOptions) -> (PathBuf, PathBuf) {
    (
        opts.reference_source
            .clone()
            .unwrap_or_else(|| repo.join("emacs")),
        opts.reference_build.clone().unwrap_or_else(|| {
            std::env::var_os("CFG")
                .map(PathBuf::from)
                .unwrap_or_else(|| repo.join("build/emacs-ref"))
        }),
    )
}
fn include_flags(repo: &Path, opts: &ProbeOptions, lisp: bool) -> Vec<OsString> {
    let (reference, config) = references(repo, opts);
    let mut a = Vec::new();
    let mut include = |flag: &str, path: PathBuf| {
        a.push(flag.into());
        a.push(path.into_os_string());
    };
    if lisp {
        include("-I", config.join("src"));
        include("-I", reference.join("src"));
        include("-I", reference.join("lib"));
        include("-I", config.join("lib"));
    } else {
        include("-I", config.join("lib"));
        include("-idirafter", source_root().join("spikes/shim"));
        include("-I", config.join("src"));
        include("-I", reference.join("src"));
        include("-I", reference.join("lib"));
    }
    a
}
fn rust_step(
    repo: &Path,
    run: &RunDirectory,
    id: &str,
    source: PathBuf,
    output: PathBuf,
    extra: &[&str],
) -> Result<StepPlan, String> {
    let mut args = vec![
        "probe".into(),
        source.into_os_string(),
        output.into_os_string(),
    ];
    args.extend(words(extra));
    let mut planned = step(
        repo,
        id,
        std::env::current_exe()
            .map_err(|e| e.to_string())?
            .into_os_string(),
        args,
    );
    planned
        .env
        .push(("RUSTUP_TOOLCHAIN".into(), pin(repo)?.into()));
    Ok(planned)
}
pub(crate) fn pin(repo: &Path) -> Result<String, String> {
    let content = fs::read_to_string(repo.join("rust-toolchain.toml"))
        .or_else(|_| fs::read_to_string(source_root().join("rust-toolchain.toml")))
        .map_err(|e| e.to_string())?;
    content
        .lines()
        .find_map(|line| {
            line.trim()
                .strip_prefix("channel")
                .and_then(|s| s.split_once('='))
                .map(|(_, v)| v.trim().trim_matches('"').to_string())
        })
        .filter(|s| !s.is_empty())
        .ok_or("pinned compiler channel missing".into())
}
fn ndk_prebuilt(repo: &Path, opts: &ProbeOptions) -> Result<PathBuf, String> {
    let ndk = opts
        .ndk
        .clone()
        .or_else(|| std::env::var_os("ANDROID_NDK_ROOT").map(PathBuf::from));
    let ndk = match ndk {
        Some(p) => p,
        None => {
            let sdk = std::env::var_os("ANDROID_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|| {
                    PathBuf::from(std::env::var_os("HOME").unwrap_or_default())
                        .join("Library/Android/sdk")
                });
            crate::android::toolchain::highest_version_dir(&sdk.join("ndk"))?
        }
    };
    let base = ndk.join("toolchains/llvm/prebuilt");
    let mut entries = fs::read_dir(&base)
        .map_err(|e| format!("NDK prebuilt {}: {e}", base.display()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    entries.sort_by_key(|e| e.file_name());
    entries
        .into_iter()
        .map(|e| e.path())
        .find(|p| p.join("bin/clang").is_file() && p.join("sysroot/usr/include/jni.h").is_file())
        .ok_or("NDK clang/jni.h missing".into())
}
pub(crate) fn plan(
    repo: &Path,
    spec: &ProbeSpec,
    opts: &ProbeOptions,
    run: &RunDirectory,
) -> Result<Vec<StepPlan>, String> {
    let out = &run.path;
    let mut steps = Vec::new();
    match spec.kind {
        ProbeKind::Differential => {
            let mut args = words(&["-std=gnu23", "-O1"]);
            if matches!(spec.reference, ReferenceRequirement::ConfiguredTree) {
                args.extend(include_flags(repo, opts, false));
            }
            args.extend([
                repo.join(spec.c_source.ok_or("C fixture missing")?)
                    .into_os_string(),
                "-o".into(),
                out.join("c").into_os_string(),
            ]);
            steps.push(step(repo, "c-build", "clang", args));
            steps.push(rust_step(
                repo,
                run,
                "rust-build",
                repo.join(spec.rust_source.ok_or("Rust fixture missing")?),
                out.join("rust"),
                &[],
            )?);
            steps.push(step(repo, "c-run", out.join("c").into_os_string(), vec![]));
            steps.push(step(
                repo,
                "rust-run",
                out.join("rust").into_os_string(),
                vec![],
            ));
        }
        ProbeKind::RustRun | ProbeKind::RustMetadata => {
            let metadata = matches!(spec.kind, ProbeKind::RustMetadata);
            let output = out.join(if metadata {
                "macro_probe.rmeta"
            } else {
                "rust"
            });
            steps.push(rust_step(
                repo,
                run,
                "rust-build",
                repo.join(spec.rust_source.ok_or("Rust fixture missing")?),
                output.clone(),
                if metadata {
                    &["--crate-type", "lib", "--emit", "metadata"]
                } else {
                    &[]
                },
            )?);
            if !metadata {
                steps.push(step(repo, "rust-run", output.into_os_string(), vec![]));
            }
        }
        ProbeKind::AbiSyntax => {
            let mut generation = vec![
                repo.join(spec.generator.ok_or("generator missing")?)
                    .into_os_string(),
            ];
            if spec.name == "lisp-abi" {
                generation.extend([
                    "--reference-header".into(),
                    references(repo, opts).0.join("src/lisp.h").into_os_string(),
                ]);
            }
            steps.push(step(repo, "generate", "python3", generation));
            let mut args = words(&[
                "-std=gnu23",
                "-fsyntax-only",
                "-Werror=incompatible-pointer-types",
            ]);
            if spec.name == "gmp-abi" {
                args.extend(words(&["-include", "stdio.h", "-I"]));
                args.push(references(repo, opts).0.join("lib").into_os_string());
            } else {
                args.extend(include_flags(repo, opts, true));
            }
            args.push(out.join("harness.c").into_os_string());
            steps.push(step(repo, "c-check", "clang", args));
        }
        ProbeKind::Jni => {
            let ndk = ndk_prebuilt(repo, opts).or_else(|error| {
                opts.ndk
                    .as_ref()
                    .map(|root| {
                        root.join("toolchains/llvm/prebuilt")
                            .join(if cfg!(target_os = "macos") {
                                "darwin-x86_64"
                            } else if cfg!(target_os = "windows") {
                                "windows-x86_64"
                            } else {
                                "linux-x86_64"
                            })
                    })
                    .ok_or(error)
            })?;
            steps.push(step(
                repo,
                "generate",
                "python3",
                vec![
                    source_root().join("tools/probe-inputs.py").into_os_string(),
                    "jni".into(),
                    ndk.join("sysroot/usr/include/jni.h").into_os_string(),
                    source_root().join("src/include/jni.rs").into_os_string(),
                    out.clone().into_os_string(),
                ],
            ));
            steps.push(step(
                repo,
                "c-check",
                ndk.join("bin/clang").into_os_string(),
                vec![
                    "--target=aarch64-linux-android24".into(),
                    format!("--sysroot={}", ndk.join("sysroot").display()).into(),
                    "-c".into(),
                    out.join("verify.c").into_os_string(),
                    "-o".into(),
                    out.join("verify.o").into_os_string(),
                ],
            ));
            steps.push(rust_step(
                repo,
                run,
                "rust-build",
                out.join("rust-jni.rs"),
                out.join("rust"),
                &[],
            )?);
            steps.push(step(
                repo,
                "rust-run",
                out.join("rust").into_os_string(),
                vec![],
            ));
        }
        ProbeKind::Miri => {
            let mut s = step(
                repo,
                "miri",
                "rustup",
                vec![
                    "run".into(),
                    pin(repo)?.into(),
                    "cargo".into(),
                    "miri".into(),
                    "run".into(),
                    "--manifest-path".into(),
                    out.join("miri-crate/Cargo.toml").into_os_string(),
                ],
            );
            s.env.push((
                "CARGO_TARGET_DIR".into(),
                out.join("miri-target").into_os_string(),
            ));
            steps.push(s);
        }
        ProbeKind::Bringup => {
            let (reference, config) = references(repo, opts);
            steps.push(step(
                repo,
                "fields",
                "python3",
                vec![
                    repo.join("tools/gen-layout-fields.py").into_os_string(),
                    "--reference".into(),
                    reference.into_os_string(),
                    "--configure".into(),
                    config.into_os_string(),
                    "--out".into(),
                    out.join("layout_fields.h").into_os_string(),
                ],
            ));
            let mut args = words(&["-std=gnu23", "-O0"]);
            args.extend(include_flags(repo, opts, false));
            args.extend([
                out.join("layout_c.c").into_os_string(),
                "-o".into(),
                out.join("layout_c").into_os_string(),
            ]);
            steps.push(step(repo, "layout-c-build", "clang", args));
            steps.push(rust_step(
                repo,
                run,
                "layout-rust-build",
                repo.join("spikes/layout.rs"),
                out.join("layout_rust"),
                &[],
            )?);
            for side in ["c", "rust"] {
                steps.push(step(
                    repo,
                    &format!("layout-{side}-run"),
                    out.join(format!("layout_{side}")).into_os_string(),
                    vec![],
                ));
            }
            steps.push(step(
                repo,
                "layout-compare",
                "python3",
                vec![
                    repo.join("spikes/compare_layout.py").into_os_string(),
                    out.join("layout-c-run.stdout.log").into_os_string(),
                    out.join("layout-rust-run.stdout.log").into_os_string(),
                ],
            ));
            steps.push(rust_step(
                repo,
                run,
                "jump-build",
                repo.join("spikes/jump.rs"),
                out.join("jump"),
                &[],
            )?);
            steps.push(step(
                repo,
                "jump-run",
                out.join("jump").into_os_string(),
                vec![],
            ));
            for opt in [0, 2, 3] {
                let id = format!("scan-opt{opt}");
                let level = format!("opt-level={opt}");
                let mut s = rust_step(
                    repo,
                    run,
                    &format!("{id}-build"),
                    repo.join("spikes/scan.rs"),
                    out.join(&id),
                    &["-C", &level, "-C", "debuginfo=0"],
                )?;
                s.env.push(("SPIKE_OPT".into(), format!("-{opt}").into()));
                steps.push(s);
                steps.push(step(
                    repo,
                    &format!("{id}-run"),
                    out.join(&id).into_os_string(),
                    vec![],
                ));
            }
        }
        ProbeKind::Kotlin => {
            let mut args = words(&["android", "kotlin", "--out"]);
            args.push(out.join("kotlin").into_os_string());
            args.extend([
                "--source-root".into(),
                repo.join("platform/android/kotlin").into_os_string(),
            ]);
            for source in &opts.extra_sources {
                args.push("--source".into());
                args.push(source.into());
            }
            if opts.reference_java {
                args.push("--reference-java".into());
            }
            steps.push(step(
                repo,
                "kotlin",
                std::env::current_exe()
                    .map_err(|e| e.to_string())?
                    .into_os_string(),
                args,
            ));
        }
    }
    Ok(steps)
}
fn validate_inputs(
    repo: &Path,
    spec: &ProbeSpec,
    opts: &ProbeOptions,
) -> Result<serde_json::Value, String> {
    let mut hashes = serde_json::Map::new();
    for input in [spec.rust_source, spec.c_source, spec.generator]
        .into_iter()
        .flatten()
    {
        hashes.insert(
            input.into(),
            json!(tooling::sha256_file(&repo.join(input))?),
        );
    }
    if matches!(spec.kind, ProbeKind::Miri) {
        hashes.insert(
            "miri_manifest".into(),
            json!(tooling::sha256_file(&repo.join("spikes/miri/Cargo.toml"))?),
        );
    }
    let (reference, config) = references(repo, opts);
    if !matches!(spec.reference, ReferenceRequirement::None) {
        let header = reference.join(if spec.name == "gmp-abi" {
            "lib/mini-gmp.h"
        } else {
            "src/lisp.h"
        });
        hashes.insert(
            "reference_header".into(),
            json!(tooling::sha256_file(&header)?),
        );
    }
    if matches!(spec.reference, ReferenceRequirement::ConfiguredTree) {
        hashes.insert(
            "reference_configuration".into(),
            json!(tooling::sha256_file(&config.join("src/config.h"))?),
        );
    }
    if matches!(spec.kind, ProbeKind::Jni) {
        let ndk = ndk_prebuilt(repo, opts)?;
        hashes.insert(
            "ndk_jni_header".into(),
            json!(tooling::sha256_file(
                &ndk.join("sysroot/usr/include/jni.h")
            )?),
        );
    }
    Ok(
        json!({"sources":hashes,"rust_toolchain":pin(repo)?,"reference_source":if matches!(spec.reference,ReferenceRequirement::None){None}else{Some(reference)},"reference_build":if matches!(spec.reference,ReferenceRequirement::ConfiguredTree){Some(config)}else{None}}),
    )
}
/// Use the same exact-source graph expansion as ordinary Rust probes, but give
/// Cargo an independent workspace/lockfile under this invocation's owned output.
fn prepare_miri(repo: &Path, run: &RunDirectory) -> Result<(), String> {
    let root = run.path.join("miri-crate");
    fs::create_dir(&root).map_err(|e| e.to_string())?;
    crate::materialize(&repo.join("spikes/miri/src/main.rs"), &root.join("graph"))?;
    let manifest =
        fs::read_to_string(repo.join("spikes/miri/Cargo.toml")).map_err(|e| e.to_string())?;
    if !manifest.contains("path = \"src/main.rs\"") {
        return Err("unexpected Miri fixture entrypoint".into());
    }
    let manifest = manifest.replace("path = \"src/main.rs\"", "path = \"graph/_root.rs\"");
    fs::write(
        root.join("Cargo.toml"),
        format!("{manifest}\n[workspace]\n"),
    )
    .map_err(|e| e.to_string())
}

pub(crate) fn run_probe(
    repo: &Path,
    name: &str,
    opts: &ProbeOptions,
) -> Result<ProbeReport, String> {
    let spec = inventory()
        .into_iter()
        .find(|s| s.name == name)
        .ok_or_else(|| format!("unknown audit probe: {name}"))?;
    let inputs = validate_inputs(repo, &spec, opts)?;
    let baseline = opts
        .write_baseline
        .as_ref()
        .map(|p| tooling::guard_scratch(repo, p))
        .transpose()?;
    if baseline.as_ref().is_some_and(|p| p.exists()) {
        return Err("baseline destination already exists".into());
    }
    if baseline.is_some() && name != "bringup" {
        return Err("baseline output is only supported by bringup".into());
    }
    let run = tooling::owned_new_output(repo, opts.out.as_deref(), name)?;
    if matches!(spec.kind, ProbeKind::Bringup) {
        fs::copy(repo.join("spikes/layout_c.c"), run.path.join("layout_c.c"))
            .map_err(|e| e.to_string())?;
    }
    let mut report = ProbeReport {
        schema: "femacs-audit-probe-v1",
        name: name.into(),
        kind: spec.kind,
        status: "failed".into(),
        steps: vec![],
        inputs,
        comparisons: vec![],
        errors: vec![],
        output: run.path.clone(),
    };
    let result = (|| -> Result<(), String> {
        if matches!(spec.kind, ProbeKind::Miri) {
            prepare_miri(repo, &run)?;
            report.inputs["materialized"]["miri"] =
                json!({"files":tooling::graph_input_hashes(&run.path.join("miri-crate/graph"))?});
        }
        for planned in plan(repo, &spec, opts, &run)? {
            let receipt = tooling::execute_step(&planned, &run)?;
            let result = tooling::require_success(&receipt);
            if planned.args.first().is_some_and(|arg| arg == "probe") {
                let path = PathBuf::from(
                    planned
                        .args
                        .get(2)
                        .ok_or("Rust probe output argument missing")?,
                )
                .with_extension("inputs.json");
                if path.is_file() {
                    let identity: serde_json::Value =
                        serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?)
                            .map_err(|e| e.to_string())?;
                    if identity["schema"] != "femacs-materialized-probe-inputs-v1"
                        || !identity["files"].is_object()
                    {
                        return Err("Rust probe materialized identities missing".into());
                    }
                    report.inputs["materialized"][&planned.id] = identity;
                } else if result.is_ok() {
                    return Err("successful Rust probe omitted materialized identities".into());
                }
            }
            if result.is_ok()
                && planned.id == "generate"
                && matches!(spec.kind, ProbeKind::AbiSyntax)
            {
                fs::copy(&receipt.stdout_log, run.path.join("harness.c"))
                    .map_err(|e| e.to_string())?;
            }
            report.steps.push(receipt);
            if let Err(error) = result {
                report.errors.push(error);
                if !matches!(spec.kind, ProbeKind::Bringup) {
                    break;
                }
            }
        }
        if matches!(spec.kind, ProbeKind::Differential) && report.errors.is_empty() {
            let c = tooling::sha256_file(&run.path.join("c-run.stdout.log"))?;
            let rust = tooling::sha256_file(&run.path.join("rust-run.stdout.log"))?;
            report
                .comparisons
                .push(json!({"c_sha256":c,"rust_sha256":rust,"equal":c==rust}));
            if c != rust {
                report.errors.push("C and Rust stdout differ".into());
            }
        }
        if report.errors.is_empty() {
            if let Some(path) = baseline {
                if let Some(parent) = path.parent() {
                    fs::create_dir_all(parent).map_err(|e| e.to_string())?;
                }
                let p = step(
                    repo,
                    "baseline",
                    "python3",
                    vec![
                        repo.join("spikes/to-baseline.py").into_os_string(),
                        run.path.join("layout-c-run.stdout.log").into_os_string(),
                        path.into_os_string(),
                    ],
                );
                let receipt = tooling::execute_step(&p, &run)?;
                let result = tooling::require_success(&receipt);
                report.steps.push(receipt);
                result?;
            }
        }
        Ok(())
    })();
    if let Err(error) = result {
        report.errors.push(error);
    }
    if report.errors.is_empty() {
        report.status = "passed".into();
    }
    tooling::write_json(&run.path.join("report.json"), &report)?;
    if report.status != "passed" {
        return Err(format!(
            "{name}: {}; report {}",
            report.errors.join("; "),
            run.path.join("report.json").display()
        ));
    }
    Ok(report)
}
pub(crate) fn command(args: &[String]) -> i32 {
    if args.first().is_some_and(|a| a == "--list") {
        for s in inventory() {
            println!("{}\t{:?}\t{:?}", s.name, s.kind, s.reference);
        }
        return 0;
    }
    let result = (|| -> Result<(), String> {
        let name = args.first().ok_or("audit-probe needs a name or --list")?;
        let mut opts = ProbeOptions::default();
        let mut at = 1;
        while at < args.len() {
            if args[at] == "--reference-java" {
                opts.reference_java = true;
                at += 1;
                continue;
            }
            if !args[at].starts_with('-') {
                opts.extra_sources.push(args[at].clone().into());
                at += 1;
                continue;
            }
            let v = PathBuf::from(args.get(at + 1).ok_or("probe option needs value")?);
            match args[at].as_str() {
                "--out" => opts.out = Some(v),
                "--reference-source" => opts.reference_source = Some(v),
                "--reference-build" => opts.reference_build = Some(v),
                "--ndk" => opts.ndk = Some(v),
                "--write-baseline" => opts.write_baseline = Some(v),
                _ => return Err(format!("unknown probe option {}", args[at])),
            }
            at += 2;
        }
        let repo = fs::canonicalize(".").map_err(|e| e.to_string())?;
        let report = run_probe(&repo, name, &opts)?;
        println!("{}: {} ({})", name, report.status, report.output.display());
        Ok(())
    })();
    match result {
        Ok(()) => 0,
        Err(e) => crate::fail(e),
    }
}
#[cfg(test)]
#[path = "probes_tests.rs"]
mod tests;
