//! Requested host/static/native legs have independent, explicit results.
use super::*;
use serde_json::json;
use std::{collections::BTreeSet, fs, path::Path};
/// Validate the measured dependency ratchet; an unknown provider cannot be certified.
pub(crate) fn validate_import_report(report: &Value) -> Result<(), AndroidError> {
    let count = report["imports"]
        .as_u64()
        .ok_or_else(|| input("import count missing"))?;
    let mut sets = Vec::new();
    for key in ["unresolved", "new", "stale", "unknown_needed"] {
        let values = report[key]
            .as_array()
            .ok_or_else(|| input(format!("missing {key} array")))?;
        let mut names = BTreeSet::new();
        for value in values {
            let name = value
                .as_str()
                .filter(|s| !s.is_empty())
                .ok_or_else(|| input(format!("invalid {key} entry")))?;
            if !names.insert(name) {
                return Err(input(format!("duplicate {key} entry")));
            }
        }
        sets.push(names);
    }
    if sets[0].len() as u64 > count
        || !sets[1].is_subset(&sets[0])
        || !sets[2].is_disjoint(&sets[0])
    {
        return Err(input("inconsistent import ratchet"));
    }
    if !sets[1].is_empty() || !sets[3].is_empty() {
        return Err(AndroidError::Verification(
            "new unresolved import or unmeasured NEEDED provider".into(),
        ));
    }
    Ok(())
}
pub(super) fn native(
    repo: &Path,
    run: &RunDirectory,
    tools: &AndroidToolchain,
    so: &Path,
    steps: &mut Vec<StepReceipt>,
) -> Result<Value, AndroidError> {
    execute(
        repo,
        run,
        tools,
        steps,
        "native-header",
        tools.readelf.clone(),
        vec!["-h".into(), so.into()],
    )?;
    let header = fs::read_to_string(run.path.join("native-header.stdout.log")).map_err(input)?;
    if !header.contains("AArch64") || !header.contains("DYN") {
        return Err(AndroidError::Verification(
            "native input is not an AArch64 shared ELF".into(),
        ));
    }
    execute(
        repo,
        run,
        tools,
        steps,
        "native-exports",
        tools.nm.clone(),
        vec!["-D".into(), "--defined-only".into(), so.into()],
    )?;
    let exports = fs::read_to_string(run.path.join("native-exports.stdout.log")).map_err(input)?;
    let names = exports
        .lines()
        .filter_map(|l| l.split_whitespace().last())
        .collect::<BTreeSet<_>>();
    let declarations =
        fs::read_to_string(repo.join("platform/android/kotlin/EmacsNative.kt")).map_err(input)?;
    let mut expected = vec![];
    for part in declarations.split("external fun ").skip(1) {
        let name = part
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
            .collect::<String>();
        if name.is_empty() {
            return Err(AndroidError::Verification(
                "unparsed JNI external method".into(),
            ));
        }
        let symbol = format!("Java_org_femacs_EmacsNative_{}", name.replace('_', "_1"));
        if !names.contains(symbol.as_str()) {
            return Err(AndroidError::Verification(format!(
                "missing JNI export {symbol}"
            )));
        }
        expected.push(symbol);
    }
    if expected.is_empty() {
        return Err(AndroidError::Verification(
            "no actual JNI declarations were inspected".into(),
        ));
    }
    execute(
        repo,
        run,
        tools,
        steps,
        "native-imports",
        "python3",
        vec![
            repo.join("tools/check-android-imports.py").into_os_string(),
            "--so".into(),
            so.into(),
            "--api".into(),
            "21".into(),
            "--json".into(),
        ],
    )?;
    let imports: Value = serde_json::from_slice(
        &fs::read(run.path.join("native-imports.stdout.log")).map_err(input)?,
    )
    .map_err(input)?;
    validate_import_report(&imports)?;
    Ok(
        json!({"sha256":tooling::sha256_file(so).map_err(input)?,"abi":"arm64-v8a","jni_exports_verified":true,"jni_exports":expected,"needed_verified":true,"import_scope":"existing API21 NDK ratchet; device loading remains separate","imports":imports}),
    )
}
pub(super) fn static_checks(
    repo: &Path,
    run: &RunDirectory,
    tools: &AndroidToolchain,
    support: &Path,
    steps: &mut Vec<StepReceipt>,
) -> Result<Value, AndroidError> {
    let units = load_units(support).map_err(input)?;
    let list = run.path.join("assertion-units.txt");
    fs::write(
        &list,
        units
            .core
            .iter()
            .chain(&units.library)
            .map(String::as_str)
            .collect::<Vec<_>>()
            .join("\n")
            + "\n",
    )
    .map_err(input)?;
    execute(
        repo,
        run,
        tools,
        steps,
        "assertions",
        "python3",
        vec![
            repo.join("tools/check-android-assertions.py")
                .into_os_string(),
            "--check".into(),
            "--config-file".into(),
            support.join("config.json").into_os_string(),
            "--units-file".into(),
            list.into_os_string(),
        ],
    )?;
    execute(
        repo,
        run,
        tools,
        steps,
        "target-layout",
        "python3",
        vec![
            repo.join("tools/check-android-layout.py").into_os_string(),
            "--ndk-bin".into(),
            tools
                .readelf
                .parent()
                .ok_or_else(|| input("NDK bin missing"))?
                .into(),
            "--out".into(),
            run.path.join("layout-measurements").into_os_string(),
        ],
    )?;
    execute(
        repo,
        run,
        tools,
        steps,
        "target-constants",
        "python3",
        vec![
            repo.join("tools/check-android-constants.py")
                .into_os_string(),
            "--ndk".into(),
            tools.clang.clone().into_os_string(),
        ],
    )?;
    let probe = crate::probes::run_probe(
        repo,
        "jni",
        &crate::probes::ProbeOptions {
            out: Some(run.path.join("jni")),
            ndk: Some(tools.ndk_root.clone()),
            ..Default::default()
        },
    )
    .map_err(AndroidError::Verification)?;
    Ok(
        json!({"assertions":"passed","layout":"passed","constants":"passed","jni":probe,"device_acceptance":false}),
    )
}
pub(crate) fn command(args: &[String]) -> i32 {
    let result = (|| -> Result<Value, AndroidError> {
        let repo = config::repo();
        let mut selected = vec![];
        let mut out = None;
        let mut at = 0;
        while at < args.len() {
            match args[at].as_str() {
                "--macos" | "--android-static" | "--android-native" | "--android-package" => {
                    selected.push(args[at].trim_start_matches("--").to_string())
                }
                "--out" => {
                    at += 1;
                    out = Some(PathBuf::from(
                        args.get(at)
                            .ok_or_else(|| input("--out needs a directory"))?,
                    ));
                }
                other => return Err(input(format!("unknown platform gate option {other}"))),
            }
            at += 1;
        }
        if selected.is_empty() {
            selected = vec![
                "macos".into(),
                "android-static".into(),
                "android-native".into(),
                "android-package".into(),
            ];
        }
        selected.sort();
        selected.dedup();
        let run =
            tooling::owned_new_output(&repo, out.as_deref(), "platform-gates").map_err(input)?;
        let mut legs = vec![];
        let mut steps = vec![];
        let mut failed = false;
        let tools = if selected.iter().any(|s| s.starts_with("android")) {
            Some(discover_toolchain(&ToolchainOptions::default())?)
        } else {
            None
        };
        let needs_native = selected
            .iter()
            .any(|s| s == "android-native" || s == "android-package");
        let build = if needs_native {
            Some(build_android(&BuildOptions {
                run_root: Some(run.path.join("android")),
                support_dir: support_directory(BuildTarget::AndroidArm64),
                tools: ToolchainOptions::default(),
                package: selected.iter().any(|s| s == "android-package"),
                assets: selected.iter().any(|s| s == "android-package"),
                debuggable: false,
            }))
        } else {
            None
        };
        for leg in &selected {
            let outcome = match leg.as_str() {
                "macos" => {
                    let plan = StepPlan {
                        id: "macos".into(),
                        program: "python3".into(),
                        args: vec![
                            repo.join("tools/local-acceptance.py").into_os_string(),
                            "--out".into(),
                            run.path.join("macos").into_os_string(),
                        ],
                        cwd: repo.clone(),
                        env: vec![],
                    };
                    let receipt =
                        tooling::execute_step(&plan, &run).map_err(AndroidError::Command)?;
                    let result =
                        tooling::require_success(&receipt).map_err(AndroidError::Verification);
                    steps.push(receipt);
                    result.map(|_|json!({"scope":"current local noninteractive host checks; GUI explicit acceptance separate"}))
                }
                "android-static" if !needs_native => static_checks(
                    &repo,
                    &run,
                    tools.as_ref().expect("Android preflight"),
                    &support_directory(BuildTarget::AndroidArm64),
                    &mut steps,
                ),
                "android-static" | "android-native" | "android-package" => {
                    match build.as_ref().expect("native build requested") {
                        Ok(report) => Ok(report.clone()),
                        Err(error) => Err(AndroidError::Verification(error.to_string())),
                    }
                }
                _ => return Err(input("unrecognized gate leg")),
            };
            match outcome{Ok(evidence)=>legs.push(json!({"id":leg,"requested":true,"executed":true,"status":"passed","evidence":evidence})),Err(error)=>{failed=true;legs.push(json!({"id":leg,"requested":true,"status":"not-met","error":error.to_string()}));}}
        }
        let report = json!({"schema":"femacs-platform-gates-v1","requested":selected,"legs":legs,"steps":steps,"status":if failed{"failed"}else{"passed"},"device_acceptance":false});
        tooling::write_json(&run.path.join("report.json"), &report).map_err(input)?;
        if failed {
            return Err(AndroidError::Verification(format!(
                "platform gates not met; {}",
                run.path.display()
            )));
        }
        Ok(report)
    })();
    match result {
        Ok(report) => {
            println!("{}", report);
            0
        }
        Err(error) => {
            eprintln!("{error}");
            error.code()
        }
    }
}
