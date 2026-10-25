//! B-phase package validation/ordering cases. Do not execute before full G4 C.
use super::*;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

struct Fixture(PathBuf);

#[test]
fn sdk_badging_accepts_legacy_and_current_names_without_ambiguous_or_false_matches() {
    let _timer = crate::test_support::CaseTimer::new("sdk_badging_exact_fields");
    let names = &["sdkVersion", "minSdkVersion"];
    for name in names {
        assert_eq!(
            sdk_version(&format!("{name}:'21'\ntargetSdkVersion:'37'\n"), names).unwrap(),
            21
        );
    }
    assert_eq!(sdk_version("minSdkVersion:'121'\n", names).unwrap(), 121);
    for text in [
        "",
        "description sdkVersion:'21'",
        "minSdkVersion:'21x'",
        "minSdkVersion:'21' trailing",
        "sdkVersion:'21'\nminSdkVersion:'21'",
        "minSdkVersion:'21'\nminSdkVersion:'22'",
    ] {
        assert!(sdk_version(text, names).is_err(), "{text}");
    }
    assert_eq!(
        sdk_version("targetSdkVersion:'37'\n", &["targetSdkVersion"]).unwrap(),
        37
    );
}

#[test]
fn archive_measurement_preserves_the_real_archive_step_receipt() {
    let _timer = crate::test_support::CaseTimer::new(
        "archive_measurement_preserves_the_real_archive_step_receipt",
    );
    let fixture = Fixture::new();
    fs::create_dir_all(&fixture.0).unwrap();
    let repo = config::repo();
    let run = RunDirectory {
        path: fixture.0.clone(),
    };
    let candidate = run.path.join("candidate.apk");
    let plan = StepPlan {
        id: "archive".into(), program: "python3".into(), cwd: repo.clone(), env: vec![],
        args: vec!["-c".into(),
            "import sys,zipfile; z=zipfile.ZipFile(sys.argv[1],'x'); z.writestr('assets/directory-tree',b'owned'); z.close()".into(),
            candidate.clone().into_os_string()],
    };
    let archived = tooling::execute_step(&plan, &run).unwrap();
    assert_eq!(archived.exit_code, Some(0));
    let original_receipt = fs::read(run.path.join("archive.json")).unwrap();
    let placeholder = fixture.0.clone();
    let tools = AndroidToolchain {
        sdk_root: placeholder.clone(),
        build_tools: placeholder.clone(),
        ndk_root: placeholder.clone(),
        clang: placeholder.clone(),
        readelf: placeholder.clone(),
        nm: placeholder.clone(),
        jdk: Jdk {
            home: placeholder.clone(),
            major: 25,
            version: "fixture".into(),
        },
        kotlinc: placeholder.clone(),
        kotlin_stdlib: placeholder.clone(),
        android_jar: placeholder,
    };
    let mut steps = vec![archived];
    let measured = measure_archive(&repo, &run, &tools, &mut steps, &candidate).unwrap();
    assert_eq!(measured["assets_count"], 1);
    assert_eq!(measured["directory_tree"], true);
    assert_eq!(
        measured["sha256"],
        tooling::sha256_file(&candidate).unwrap()
    );
    assert_eq!(
        fs::read(run.path.join("archive.json")).unwrap(),
        original_receipt
    );
    assert!(run.path.join("measure.json").is_file());
    assert!(run.path.join("archive-measurement.json").is_file());
    assert_eq!(steps.len(), 2);
}

impl Fixture {
    fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../build/package-contracts")
            .join(format!("{}-{nonce}", std::process::id()));
        fs::create_dir_all(&path).unwrap();
        Self(path)
    }
    fn opts(&self, native: bool, assets: bool, debuggable: bool) -> PackageOptions {
        let so = self.0.join("libemacs.so");
        fs::write(&so, b"owned-native-input").unwrap();
        PackageOptions {
            run_root: Some(self.0.join("new-output")),
            native_so: if native { Some(so) } else { None },
            assets,
            debuggable,
            tools: ToolchainOptions {
                sdk: None,
                ndk: None,
                jdk: None,
                kotlinc: None,
                stdlib: None,
                api: 36,
                ndk_api: 21,
            },
        }
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn valid(opts: &PackageOptions) -> Value {
    let mut members = vec!["AndroidManifest.xml", "resources.arsc", "classes.dex"];
    if opts.native_so.is_some() {
        members.push("lib/arm64-v8a/libemacs.so");
    }
    if opts.assets {
        members.extend([
            "assets/directory-tree",
            "assets/lisp/loadup.el",
            "assets/etc/DOC",
            "assets/bitmaps/gray.xbm",
        ]);
    }
    let native = opts.native_so.as_ref().map(|p| {
        json!({
            "sha256":format!("{:x}",Sha256::digest(fs::read(p).unwrap())),
            "abi":"arm64-v8a","jni_exports_verified":true,"needed_verified":true
        })
    });
    let assets = if opts.assets {
        json!({"count":4,"directory_tree":true,"stored":true,
                                     "sha256":"a".repeat(64)})
    } else {
        Value::Null
    };
    json!({
        "schema":"femacs-android-package-v1", "status":"passed",
        "package":"org.femacs", "debuggable":opts.debuggable,
        "activities":["org.femacs.EmacsActivity","org.femacs.EmacsOpenActivity",
                      "org.femacs.EmacsMultitaskActivity","org.femacs.EmacsPreferencesActivity",
                      "org.femacs.EmacsLauncherPreferencesActivity"],
        "classes":["org.femacs.EmacsActivity","org.femacs.EmacsOpenActivity",
                   "org.femacs.EmacsMultitaskActivity","org.femacs.EmacsPreferencesActivity",
                   "org.femacs.EmacsLauncherPreferencesActivity","kotlin.jvm.internal.Intrinsics"],
        "apk":{"members":members,"min_sdk":21,"target_sdk":37,
               "sha256":"b".repeat(64),"zipaligned":true},
        "dex":{"sha256":"c".repeat(64),"kotlin_runtime":true},
        "native":native,"assets":assets,
        "signer":{"verified":true,"certificate_sha256":"d".repeat(64)},
        "device_ready":opts.native_so.is_some() && opts.assets,
        "errors":[]
    })
}

#[test]
fn package_modes_keep_structural_native_assets_and_debug_state_separate() {
    let _case_time = crate::test_support::CaseTimer::new(
        "android::package_tests.rs::package_modes_keep_structural_native_assets_and_debug_state_separate",
    );
    let fixture = Fixture::new();
    for (native, assets, debuggable) in [
        (false, false, false),
        (true, false, false),
        (true, true, false),
        (true, true, true),
    ] {
        let opts = fixture.opts(native, assets, debuggable);
        validate_package_report(&opts, &valid(&opts)).expect("complete declared package mode");
    }
}

#[test]
fn every_required_archive_member_and_runtime_identity_is_enforced() {
    let _case_time = crate::test_support::CaseTimer::new(
        "android::package_tests.rs::every_required_archive_member_and_runtime_identity_is_enforced",
    );
    let fixture = Fixture::new();
    let opts = fixture.opts(true, true, true);
    let good = valid(&opts);
    for missing in [
        "AndroidManifest.xml",
        "resources.arsc",
        "classes.dex",
        "lib/arm64-v8a/libemacs.so",
        "assets/directory-tree",
    ] {
        let mut bad = good.clone();
        bad["apk"]["members"]
            .as_array_mut()
            .unwrap()
            .retain(|v| v.as_str() != Some(missing));
        assert!(
            validate_package_report(&opts, &bad).is_err(),
            "missing {missing}"
        );
    }
    for key in ["signer", "dex", "native", "assets"] {
        let mut bad = good.clone();
        bad.as_object_mut().unwrap().remove(key);
        assert!(
            validate_package_report(&opts, &bad).is_err(),
            "missing {key}"
        );
    }
    let mut bad = good.clone();
    bad["classes"]
        .as_array_mut()
        .unwrap()
        .retain(|v| v.as_str() != Some("kotlin.jvm.internal.Intrinsics"));
    assert!(validate_package_report(&opts, &bad).is_err());
    let mut bad = good.clone();
    bad["classes"]
        .as_array_mut()
        .unwrap()
        .retain(|v| v.as_str() != Some("org.femacs.EmacsActivity"));
    assert!(validate_package_report(&opts, &bad).is_err());
}

#[test]
fn signature_target_native_and_asset_mismatches_cannot_publish() {
    let _case_time = crate::test_support::CaseTimer::new(
        "android::package_tests.rs::signature_target_native_and_asset_mismatches_cannot_publish",
    );
    let fixture = Fixture::new();
    let opts = fixture.opts(true, true, true);
    for (pointer, replacement) in [
        ("/signer/verified", json!(false)),
        ("/signer/certificate_sha256", json!("")),
        ("/apk/zipaligned", json!(false)),
        ("/apk/min_sdk", json!(19)),
        ("/apk/target_sdk", json!(36)),
        ("/native/sha256", json!("0".repeat(64))),
        ("/native/abi", json!("x86_64")),
        ("/native/jni_exports_verified", json!(false)),
        ("/native/needed_verified", json!(false)),
        ("/assets/directory_tree", json!(false)),
        ("/assets/stored", json!(false)),
        ("/dex/kotlin_runtime", json!(false)),
        ("/debuggable", json!(false)),
        ("/package", json!("invalid.package")),
        ("/status", json!("failed")),
        ("/errors", json!(["failed signature"])),
    ] {
        let mut bad = valid(&opts);
        *bad.pointer_mut(pointer).unwrap() = replacement;
        assert!(
            validate_package_report(&opts, &bad).is_err(),
            "accepted {pointer}"
        );
    }
}

#[test]
fn structural_packages_cannot_claim_native_device_readiness() {
    let _case_time = crate::test_support::CaseTimer::new(
        "android::package_tests.rs::structural_packages_cannot_claim_native_device_readiness",
    );
    let fixture = Fixture::new();
    let opts = fixture.opts(false, false, false);
    let mut bad = valid(&opts);
    bad["device_ready"] = json!(true);
    assert!(validate_package_report(&opts, &bad).is_err());
    let mut bad = valid(&opts);
    bad["native"] = json!({"sha256":"f".repeat(64)});
    assert!(validate_package_report(&opts, &bad).is_err());
}

#[test]
fn numeric_versions_do_not_pick_9_after_35_or_silently_accept_preview_names() {
    let _case_time = crate::test_support::CaseTimer::new(
        "android::package_tests.rs::numeric_versions_do_not_pick_9_after_35_or_silently_accept_preview_names",
    );
    assert!(parse_numeric_version("35.0.1").unwrap() > parse_numeric_version("9.0.0").unwrap());
    assert!(
        parse_numeric_version("29.0.14206865").unwrap()
            > parse_numeric_version("28.2.13676358").unwrap()
    );
    for invalid in ["", "stable", "35..1", "35.0.0-rc2", "../35.0.0", "35/0"] {
        assert!(
            parse_numeric_version(invalid).is_err(),
            "invalid version {invalid}"
        );
    }
}
