//! Resource/Kotlin/dex/APK pipeline. Verification precedes candidate publication.
use super::*;
use serde_json::{Value, json};
use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};
pub(crate) type PackageReport = Value;
const ACTIVITIES: [&str; 5] = [
    "org.femacs.EmacsActivity",
    "org.femacs.EmacsOpenActivity",
    "org.femacs.EmacsMultitaskActivity",
    "org.femacs.EmacsPreferencesActivity",
    "org.femacs.EmacsLauncherPreferencesActivity",
];
fn strings(value: &Value) -> BTreeSet<&str> {
    value
        .as_array()
        .map(|a| a.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default()
}

fn sdk_version(badging: &str, names: &[&str]) -> Result<u32, AndroidError> {
    let values = badging
        .lines()
        .filter_map(|line| {
            let (name, value) = line.split_once(':')?;
            names.contains(&name).then_some(value)
        })
        .collect::<Vec<_>>();
    if values.len() != 1 {
        return Err(AndroidError::Verification(
            "missing or ambiguous manifest SDK field".into(),
        ));
    }
    values[0]
        .strip_prefix('\'')
        .and_then(|v| v.strip_suffix('\''))
        .and_then(|v| v.parse().ok())
        .ok_or_else(|| AndroidError::Verification("invalid numeric manifest SDK field".into()))
}
fn hash(value: &Value) -> bool {
    value
        .as_str()
        .is_some_and(|s| s.len() == 64 && s.bytes().all(|b| b.is_ascii_hexdigit()))
}
pub(crate) fn validate_package_report(
    opts: &PackageOptions,
    report: &PackageReport,
) -> Result<(), AndroidError> {
    let members = strings(&report["apk"]["members"]);
    let classes = strings(&report["classes"]);
    let activities = strings(&report["activities"]);
    let required = ["AndroidManifest.xml", "resources.arsc", "classes.dex"];
    let base = report["schema"] == "femacs-android-package-v1"
        && report["status"] == "passed"
        && report["package"] == "org.femacs"
        && report["debuggable"] == opts.debuggable
        && report["apk"]["min_sdk"] == 21
        && report["apk"]["target_sdk"] == 37
        && report["apk"]["zipaligned"] == true
        && hash(&report["apk"]["sha256"])
        && report["signer"]["verified"] == true
        && hash(&report["signer"]["certificate_sha256"])
        && report["dex"]["kotlin_runtime"] == true
        && hash(&report["dex"]["sha256"])
        && classes.contains("kotlin.jvm.internal.Intrinsics")
        && required.iter().all(|m| members.contains(m))
        && ACTIVITIES
            .iter()
            .all(|a| activities.contains(a) && classes.contains(a))
        && report["errors"].as_array().is_some_and(|a| a.is_empty());
    if !base {
        return Err(AndroidError::Verification(
            "incomplete or inconsistent package/signature/dex/manifest evidence".into(),
        ));
    }
    if let Some(so) = &opts.native_so {
        if !members.contains("lib/arm64-v8a/libemacs.so")
            || report["native"]["sha256"].as_str()
                != Some(
                    tooling::sha256_file(so)
                        .map_err(AndroidError::Input)?
                        .as_str(),
                )
            || report["native"]["abi"] != "arm64-v8a"
            || report["native"]["jni_exports_verified"] != true
            || report["native"]["needed_verified"] != true
        {
            return Err(AndroidError::Verification(
                "native identity/JNI/NEEDED evidence mismatch".into(),
            ));
        }
    } else if !report["native"].is_null() || members.iter().any(|m| m.starts_with("lib/")) {
        return Err(AndroidError::Verification(
            "structural package unexpectedly carries native code".into(),
        ));
    }
    if opts.assets {
        if !members.contains("assets/directory-tree")
            || !members.contains("assets/lisp/loadup.el")
            || !members.contains("assets/etc/DOC")
            || !members.iter().any(|m| m.starts_with("assets/bitmaps/"))
            || report["assets"]["directory_tree"] != true
            || report["assets"]["stored"] != true
            || report["assets"]["count"].as_u64().unwrap_or(0) < 4
            || !hash(&report["assets"]["sha256"])
        {
            return Err(AndroidError::Verification(
                "asset tree missing, incomplete or compressed".into(),
            ));
        }
    } else if !report["assets"].is_null() || members.iter().any(|m| m.starts_with("assets/")) {
        return Err(AndroidError::Verification("unexpected asset tree".into()));
    }
    if report["device_ready"] != (opts.native_so.is_some() && opts.assets) {
        return Err(AndroidError::Verification(
            "structural/native package readiness mismatch".into(),
        ));
    }
    Ok(())
}
pub(crate) fn collect_files(root: &Path, extension: &str) -> Result<Vec<PathBuf>, AndroidError> {
    let mut files = vec![];
    for entry in fs::read_dir(root).map_err(input)? {
        let entry = entry.map_err(input)?;
        let kind = entry.file_type().map_err(input)?;
        if kind.is_symlink() {
            return Err(AndroidError::Input("symlink in compiler input tree".into()));
        }
        if kind.is_dir() {
            files.extend(collect_files(&entry.path(), extension)?);
        } else if entry.path().extension().is_some_and(|v| v == extension) {
            files.push(entry.path());
        }
    }
    files.sort();
    Ok(files)
}
pub(crate) fn kotlin_into(
    repo: &Path,
    run: &RunDirectory,
    tools: &AndroidToolchain,
    native: Option<&Path>,
    debuggable: bool,
    extra: &[PathBuf],
    reference_java: bool,
    steps: &mut Vec<StepReceipt>,
) -> Result<PathBuf, AndroidError> {
    let generated = run.path.join("generated");
    fs::create_dir(&generated).map_err(input)?;
    let rclasses = run.path.join("rclasses");
    fs::create_dir(&rclasses).map_err(input)?;
    let classes = run.path.join("classes");
    fs::create_dir(&classes).map_err(input)?;
    let mut manifest =
        fs::read_to_string(repo.join("platform/android/AndroidManifest.xml")).map_err(input)?;
    if debuggable {
        if !manifest.contains("android:debuggable=\"false\"") {
            return Err(AndroidError::Input(
                "manifest debug attribute missing".into(),
            ));
        }
        manifest = manifest.replace(
            "android:debuggable=\"false\"",
            "android:debuggable=\"true\"",
        );
    }
    let staged = run.path.join("AndroidManifest.xml");
    fs::write(&staged, manifest).map_err(input)?;
    execute(
        repo,
        run,
        tools,
        steps,
        "resources-compile",
        tools.build_tools.join("aapt2"),
        vec![
            "compile".into(),
            "--dir".into(),
            repo.join("platform/android/res").into_os_string(),
            "-o".into(),
            run.path.join("res.zip").into_os_string(),
        ],
    )?;
    execute(
        repo,
        run,
        tools,
        steps,
        "resources-link",
        tools.build_tools.join("aapt2"),
        vec![
            "link".into(),
            "-o".into(),
            run.path.join("resources.ap_").into_os_string(),
            "-I".into(),
            tools.android_jar.clone().into_os_string(),
            "--manifest".into(),
            staged.into_os_string(),
            "--java".into(),
            generated.clone().into_os_string(),
            run.path.join("res.zip").into_os_string(),
        ],
    )?;
    let mut java = vec![
        "--release".into(),
        "17".into(),
        "-classpath".into(),
        tools.android_jar.clone().into_os_string(),
        "-d".into(),
        rclasses.clone().into_os_string(),
    ];
    let sources = collect_files(&generated, "java")?;
    if sources.is_empty() {
        return Err(AndroidError::Verification(
            "aapt2 produced no Java resource classes".into(),
        ));
    }
    java.extend(sources.into_iter().map(PathBuf::into_os_string));
    execute(
        repo,
        run,
        tools,
        steps,
        "resources-javac",
        tools.jdk.home.join("bin/javac"),
        java,
    )?;
    execute(
        repo,
        run,
        tools,
        steps,
        "resources-jar",
        tools.jdk.home.join("bin/jar"),
        vec![
            "cf".into(),
            run.path.join("R.jar").into_os_string(),
            "-C".into(),
            rclasses.into_os_string(),
            ".".into(),
        ],
    )?;
    let mut generate = vec![
        repo.join("tools/gen-emacs-config.py").into_os_string(),
        "--out".into(),
        generated.clone().into_os_string(),
        "--readelf".into(),
        tools.readelf.clone().into_os_string(),
    ];
    if let Some(library) = native {
        generate.extend(["--library".into(), library.into()]);
    }
    execute(repo, run, tools, steps, "emacs-config", "python3", generate)?;
    // Compile the owned closure even for a named leaf; no upstream stand-ins or stubs.
    let mut kotlin = if reference_java {
        if extra.is_empty() {
            return Err(AndroidError::Input(
                "reference Java source audit requires explicit owned --source leaves".into(),
            ));
        }
        extra.to_vec()
    } else {
        collect_files(&repo.join("platform/android/kotlin"), "kt")?
    };
    for leaf in extra {
        let actual = fs::canonicalize(leaf).map_err(input)?;
        if !actual.starts_with(repo.join("platform/android/kotlin"))
            || actual.extension().is_none_or(|e| e != "kt")
        {
            return Err(AndroidError::Input(
                "Kotlin audit source must be owned".into(),
            ));
        }
    }
    if reference_java {
        let audit = run.path.join("reference-java-audit");
        fs::create_dir(&audit).map_err(input)?;
        let selected = extra
            .iter()
            .filter_map(|p| p.file_stem())
            .map(|s| s.to_string_lossy().into_owned())
            .collect::<BTreeSet<_>>();
        let reference = repo.join("emacs/java/org/gnu/emacs");
        for source in collect_files(&reference, "java")? {
            if selected.contains(
                source
                    .file_stem()
                    .ok_or_else(|| input("Java source stem missing"))?
                    .to_string_lossy()
                    .as_ref(),
            ) {
                continue;
            }
            let text = fs::read_to_string(&source).map_err(input)?;
            let staged = audit.join(
                source
                    .file_name()
                    .ok_or_else(|| input("Java source name missing"))?,
            );
            fs::write(
                &staged,
                text.replace("package org.gnu.emacs;", "package org.femacs;"),
            )
            .map_err(input)?;
            kotlin.push(staged);
        }
    }
    kotlin.extend(collect_files(&generated, "kt")?);
    let cp = std::env::join_paths([
        tools.android_jar.clone(),
        run.path.join("R.jar"),
        tools.kotlin_stdlib.clone(),
    ])
    .map_err(|e| AndroidError::Input(e.to_string()))?;
    let mut args = vec![
        "-no-stdlib".into(),
        "-classpath".into(),
        cp,
        "-d".into(),
        classes.clone().into_os_string(),
    ];
    args.extend(kotlin.into_iter().map(PathBuf::into_os_string));
    execute(
        repo,
        run,
        tools,
        steps,
        "kotlin",
        tools.kotlinc.clone(),
        args,
    )?;
    Ok(classes)
}
fn measure_archive(
    repo: &Path,
    run: &RunDirectory,
    tools: &AndroidToolchain,
    steps: &mut Vec<StepReceipt>,
    candidate: &Path,
) -> Result<Value, AndroidError> {
    // archive.json belongs to the archive step receipt, not the child output.
    let observation = run.path.join("archive-measurement.json");
    execute(
        repo,
        run,
        tools,
        steps,
        "measure",
        "python3",
        vec![
            repo.join("tools/android-archive.py").into_os_string(),
            "inspect".into(),
            "--apk".into(),
            candidate.into(),
            "--out".into(),
            observation.clone().into_os_string(),
        ],
    )?;
    serde_json::from_slice(&fs::read(observation).map_err(input)?).map_err(input)
}

pub(crate) fn package_apk(opts: &PackageOptions) -> Result<PackageReport, AndroidError> {
    let repo = config::repo();
    let tools = discover_toolchain(&opts.tools)?;
    if opts.assets && opts.native_so.is_none() {
        return Err(AndroidError::Input(
            "assets package requires its actual native product".into(),
        ));
    }
    if let Some(so) = &opts.native_so {
        tooling::sha256_file(so).map_err(AndroidError::Input)?;
    }
    let run = tooling::owned_new_output(&repo, opts.run_root.as_deref(), "android-package")
        .map_err(AndroidError::Input)?;
    let mut steps = vec![];
    let result = (|| -> Result<Value, AndroidError> {
        let native = if let Some(so) = &opts.native_so {
            Some(gate::native(&repo, &run, &tools, so, &mut steps)?)
        } else {
            None
        };
        let classes = kotlin_into(
            &repo,
            &run,
            &tools,
            opts.native_so.as_deref(),
            opts.debuggable,
            &[],
            false,
            &mut steps,
        )?;
        let dex = run.path.join("dex");
        fs::create_dir(&dex).map_err(input)?;
        let mut args = vec![
            "--lib".into(),
            tools.android_jar.clone().into_os_string(),
            "--min-api".into(),
            "21".into(),
            "--output".into(),
            dex.clone().into_os_string(),
        ];
        args.extend(
            collect_files(&classes, "class")?
                .into_iter()
                .map(PathBuf::into_os_string),
        );
        args.extend(
            collect_files(&run.path.join("rclasses"), "class")?
                .into_iter()
                .map(PathBuf::into_os_string),
        );
        args.push(tools.kotlin_stdlib.clone().into_os_string());
        execute(
            &repo,
            &run,
            &tools,
            &mut steps,
            "dex",
            tools.build_tools.join("d8"),
            args,
        )?;
        let assets = if opts.assets {
            Some(build_assets(&repo, &run, &tools, &mut steps)?)
        } else {
            None
        };
        let unsigned = run.path.join("unsigned.apk");
        let mut args = vec![
            repo.join("tools/android-archive.py").into_os_string(),
            "pack".into(),
            "--resources".into(),
            run.path.join("resources.ap_").into_os_string(),
            "--dex".into(),
            dex.into_os_string(),
            "--out".into(),
            unsigned.clone().into_os_string(),
        ];
        if let Some(so) = &opts.native_so {
            args.extend(["--native".into(), so.into()]);
        }
        if let Some(assets) = &assets {
            args.extend(["--assets".into(), assets.into()]);
        }
        if let Some(wrap) = std::env::var_os("FEMACS_ANDROID_WRAP").filter(|v| !v.is_empty()) {
            if !opts.debuggable || opts.native_so.is_none() {
                return Err(AndroidError::Input(
                    "explicit wrap helper requires debuggable native package".into(),
                ));
            }
            args.extend(["--wrap".into(), wrap]);
        }
        execute(&repo, &run, &tools, &mut steps, "archive", "python3", args)?;
        let aligned = run.path.join("aligned.apk");
        execute(
            &repo,
            &run,
            &tools,
            &mut steps,
            "align",
            tools.build_tools.join("zipalign"),
            vec![
                "-f".into(),
                "4".into(),
                unsigned.into_os_string(),
                aligned.clone().into_os_string(),
            ],
        )?;
        if std::env::var_os("FEMACS_ANDROID_WRAP").is_some_and(|v| !v.is_empty()) {
            execute(
                &repo,
                &run,
                &tools,
                &mut steps,
                "wrap-mode",
                "python3",
                vec![
                    repo.join("tools/android-archive.py").into_os_string(),
                    "wrap-mode".into(),
                    "--apk".into(),
                    aligned.clone().into_os_string(),
                ],
            )?;
        }
        execute(
            &repo,
            &run,
            &tools,
            &mut steps,
            "key",
            tools.jdk.home.join("bin/keytool"),
            vec![
                "-genkeypair".into(),
                "-keystore".into(),
                run.path.join("debug.keystore").into_os_string(),
                "-storepass".into(),
                "android".into(),
                "-keypass".into(),
                "android".into(),
                "-alias".into(),
                "androiddebugkey".into(),
                "-dname".into(),
                "CN=femacs debug".into(),
                "-keyalg".into(),
                "RSA".into(),
                "-validity".into(),
                "10000".into(),
            ],
        )?;
        let candidate = run.path.join("candidate.apk");
        execute(
            &repo,
            &run,
            &tools,
            &mut steps,
            "sign",
            tools.build_tools.join("apksigner"),
            vec![
                "sign".into(),
                "--ks".into(),
                run.path.join("debug.keystore").into_os_string(),
                "--ks-pass".into(),
                "pass:android".into(),
                "--out".into(),
                candidate.clone().into_os_string(),
                aligned.into_os_string(),
            ],
        )?;
        execute(
            &repo,
            &run,
            &tools,
            &mut steps,
            "signature",
            tools.build_tools.join("apksigner"),
            vec![
                "verify".into(),
                "--print-certs".into(),
                candidate.clone().into_os_string(),
            ],
        )?;
        execute(
            &repo,
            &run,
            &tools,
            &mut steps,
            "alignment-verify",
            tools.build_tools.join("zipalign"),
            vec!["-c".into(), "4".into(), candidate.clone().into_os_string()],
        )?;
        execute(
            &repo,
            &run,
            &tools,
            &mut steps,
            "badging",
            tools.build_tools.join("aapt2"),
            vec![
                "dump".into(),
                "badging".into(),
                candidate.clone().into_os_string(),
            ],
        )?;
        execute(
            &repo,
            &run,
            &tools,
            &mut steps,
            "manifest-tree",
            tools.build_tools.join("aapt2"),
            vec![
                "dump".into(),
                "xmltree".into(),
                "--file".into(),
                "AndroidManifest.xml".into(),
                candidate.clone().into_os_string(),
            ],
        )?;
        let measured = measure_archive(&repo, &run, &tools, &mut steps, &candidate)?;
        if std::env::var_os("FEMACS_ANDROID_WRAP").is_some_and(|v| !v.is_empty())
            && measured["wrap_mode"] != "0o100755"
        {
            return Err(AndroidError::Verification(
                "explicit packaged wrap helper lost executable mode".into(),
            ));
        }
        let badging = fs::read_to_string(run.path.join("badging.stdout.log")).map_err(input)?;
        let tree = fs::read_to_string(run.path.join("manifest-tree.stdout.log")).map_err(input)?;
        let cert = fs::read_to_string(run.path.join("signature.stdout.log")).map_err(input)?;
        let cert = cert
            .lines()
            .find_map(|l| {
                l.split_once("certificate SHA-256 digest:")
                    .map(|(_, v)| v.trim().to_owned())
            })
            .ok_or_else(|| {
                AndroidError::Verification("verified certificate digest missing".into())
            })?;
        if !badging
            .lines()
            .any(|l| l.starts_with("package: name='org.femacs'"))
            || sdk_version(&badging, &["sdkVersion", "minSdkVersion"])? != 21
            || sdk_version(&badging, &["targetSdkVersion"])? != 37
            || !ACTIVITIES
                .iter()
                .all(|a| tree.contains(a) || tree.contains(&a.replace("org.femacs.", ".")))
        {
            return Err(AndroidError::Verification(
                "actual compiled manifest package, activities or SDK differ".into(),
            ));
        }
        let debug = badging
            .lines()
            .any(|l| l.starts_with("application-debuggable"));
        let asset_report = if opts.assets {
            json!({"count":measured["assets_count"],"sha256":measured["assets_sha256"],"stored":measured["assets_stored"],"directory_tree":measured["directory_tree"]})
        } else {
            Value::Null
        };
        let native_report = if let Some(mut data) = native {
            if data["sha256"] != measured["native_sha256"] {
                return Err(AndroidError::Verification(
                    "packaged native library identity differs".into(),
                ));
            }
            data["abi"] = json!("arm64-v8a");
            data
        } else {
            Value::Null
        };
        let report = json!({"schema":"femacs-android-package-v1","status":"passed","package":"org.femacs","activities":ACTIVITIES,"classes":measured["classes"],"debuggable":debug,"apk":{"path":run.path.join("emacs.apk"),"members":measured["members"],"sha256":measured["sha256"],"min_sdk":21,"target_sdk":37,"zipaligned":true},"dex":{"sha256":measured["dex_sha256"],"kotlin_runtime":strings(&measured["classes"]).contains("kotlin.jvm.internal.Intrinsics")},"native":native_report,"assets":asset_report,"signer":{"verified":true,"certificate_sha256":cert},"device_ready":opts.native_so.is_some() && opts.assets,"device_acceptance":false,"toolchain":tools,"steps":steps,"inputs":{"manifest":tooling::sha256_file(&repo.join("platform/android/AndroidManifest.xml")).map_err(AndroidError::Input)?},"errors":[]});
        validate_package_report(opts, &report)?;
        fs::rename(candidate, run.path.join("emacs.apk")).map_err(input)?;
        Ok(report)
    })();
    match result {
        Ok(report) => {
            tooling::write_json(&run.path.join("report.json"), &report)
                .map_err(AndroidError::Input)?;
            Ok(report)
        }
        Err(e) => {
            let failed = json!({"schema":"femacs-android-package-v1","status":"failed","steps":steps,"errors":[e.to_string()],"device_ready":false});
            tooling::write_json(&run.path.join("report.json"), &failed)
                .map_err(AndroidError::Input)?;
            Err(e)
        }
    }
}
#[cfg(test)]
#[path = "package_tests.rs"]
mod tests;
