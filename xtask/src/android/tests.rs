//! Independent B-phase contracts for the Android build/package/config owner.
//!
//! These are source/config/plan-level cases.  They do not execute a compiler, SDK tool or
//! device; the real SDK/native/package integration belongs to the D acceptance recipes.

use super::*;

use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

fn repo() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask lives under the repository root")
        .to_path_buf()
}

fn tag() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock before epoch")
        .as_nanos();
    format!("{}-{nanos}", std::process::id())
}

struct Fixture(PathBuf);

impl Fixture {
    fn new(name: &str) -> Self {
        let path = repo()
            .join("build/android-tests")
            .join(format!("{name}-{}", tag()));
        fs::create_dir_all(&path).expect("android fixture");
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn env_lock() -> std::sync::MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .expect("env lock")
}

struct EnvGuard {
    key: &'static str,
    old: Option<OsString>,
}

impl EnvGuard {
    fn set(key: &'static str, value: &str) -> Self {
        let old = std::env::var_os(key);
        unsafe {
            std::env::set_var(key, value);
        }
        Self { key, old }
    }

    fn remove(key: &'static str) -> Self {
        let old = std::env::var_os(key);
        unsafe {
            std::env::remove_var(key);
        }
        Self { key, old }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        match &self.old {
            Some(value) => unsafe {
                std::env::set_var(self.key, value);
            },
            None => unsafe {
                std::env::remove_var(self.key);
            },
        }
    }
}

fn toolchain_options(root: &Path) -> ToolchainOptions {
    ToolchainOptions {
        sdk: Some(root.join("sdk")),
        ndk: Some(root.join("ndk")),
        jdk: Some(root.join("jdk")),
        kotlinc: Some(root.join("kotlinc")),
        stdlib: Some(root.join("stdlib.jar")),
        api: 36,
        ndk_api: 21,
    }
}

#[test]
fn support_directories_are_target_specific() {
    let _case_time = crate::test_support::CaseTimer::new(
        "android::tests.rs::support_directories_are_target_specific",
    );
    assert!(support_directory(BuildTarget::AndroidArm64).ends_with("build-support/android/arm64"));
    assert!(support_directory(BuildTarget::MacTerminal).ends_with("build-support/macos/terminal"));
    assert!(support_directory(BuildTarget::MacCocoa).ends_with("build-support/macos/cocoa"));
}

#[test]
fn explicit_build_target_has_priority_and_unknown_values_fail() {
    let _case_time = crate::test_support::CaseTimer::new(
        "android::tests.rs::explicit_build_target_has_priority_and_unknown_values_fail",
    );
    let _lock = env_lock();
    let _old = EnvGuard::remove("FEMACS_BUILD_TARGET");
    let _gui = EnvGuard::remove("FEMACS_MACOS_GUI");
    assert!(matches!(
        build_target().expect("host target"),
        BuildTarget::MacTerminal
    ));

    let _target = EnvGuard::set("FEMACS_BUILD_TARGET", "android-arm64");
    assert!(matches!(
        build_target().expect("android target"),
        BuildTarget::AndroidArm64
    ));
    let _unknown = EnvGuard::set("FEMACS_BUILD_TARGET", "aarch64-unknown");
    assert!(build_target().is_err());
}

#[test]
fn config_source_prefers_file_then_stem_and_never_invents_a_path() {
    let _case_time = crate::test_support::CaseTimer::new(
        "android::tests.rs::config_source_prefers_file_then_stem_and_never_invents_a_path",
    );
    let _lock = env_lock();
    let fx = Fixture::new("config-source");
    let file = fx.path().join("android.json");
    fs::write(&file, "{\"macros\":{},\"features\":{}}").expect("config file");
    let file_text = file.display().to_string();

    let _file = EnvGuard::set("FEMACS_CONFIG_FILE", &file_text);
    let _stem = EnvGuard::set("FEMACS_CONFIG", "android-arm64");
    assert_eq!(
        config_source(None).expect("file source"),
        Some(file.clone())
    );
    assert_eq!(
        config_source(Some("unrelated")).expect("file wins"),
        Some(file.clone())
    );
    drop(_file);

    let _stem = EnvGuard::set("FEMACS_CONFIG", "android-arm64");
    let from_stem = config_source(None).expect("stem source");
    assert!(from_stem.is_some());
    assert!(from_stem.unwrap().ends_with("android-arm64.json"));

    let _missing = EnvGuard::set("FEMACS_CONFIG_FILE", "/nonexistent/config.json");
    assert!(
        config_source(None).is_err(),
        "an explicit missing/unreadable configuration must be an error, not empty cfg"
    );
    let _missing = EnvGuard::set("FEMACS_CONFIG_FILE", "/nonexistent/config.json");
    assert!(config_source(Some("android-arm64")).is_err());
}

#[test]
fn config_source_without_any_override_is_none() {
    let _case_time = crate::test_support::CaseTimer::new(
        "android::tests.rs::config_source_without_any_override_is_none",
    );
    let _lock = env_lock();
    let _file = EnvGuard::remove("FEMACS_CONFIG_FILE");
    let _stem = EnvGuard::remove("FEMACS_CONFIG");
    assert_eq!(config_source(None).expect("no override"), None);
}

#[test]
fn load_units_reads_comments_and_rejects_duplicates_escape_and_auxiliary_paths() {
    let _case_time = crate::test_support::CaseTimer::new(
        "android::tests.rs::load_units_reads_comments_and_rejects_duplicates_escape_and_auxiliary_paths",
    );
    let fx = Fixture::new("units");
    let support = fx.path();
    fs::write(support.join("core-units.txt"), "# core\nsrc/doprnt.c\n").expect("core units");
    fs::write(support.join("lib-units.txt"), "# library\nlib/mini-gmp.c\n").expect("lib units");
    let units = load_units(support).expect("valid units");
    assert!(
        units
            .core
            .iter()
            .any(|u| u.ends_with("doprnt.c") || u.ends_with("doprnt.rs"))
    );
    assert!(units.library.iter().any(|u| u.ends_with("mini-gmp.c")));

    for (name, text) in [
        ("duplicate", "src/doprnt.c\nsrc/doprnt.c\n"),
        ("escape", "../src/doprnt.c\n"),
        ("absolute", "/tmp/doprnt.c\n"),
        ("auxiliary", "lib-src/emacsclient.c\n"),
        ("unknown", "src/no-such-owned-unit.c\n"),
    ] {
        let bad = fx.path().join(format!("{name}-core.txt"));
        fs::write(&bad, text).expect("bad units");
        let bad_support = fx.path().join(format!("{name}-support"));
        fs::create_dir_all(&bad_support).expect("bad support");
        fs::copy(&bad, bad_support.join("core-units.txt")).expect("copy core");
        fs::write(bad_support.join("lib-units.txt"), "").expect("empty lib");
        assert!(load_units(&bad_support).is_err(), "{name} must fail");
    }
}

#[test]
fn expected_objects_distinguishes_checking_twins_from_normal_objects_and_rejects_extras() {
    let _case_time = crate::test_support::CaseTimer::new(
        "android::tests.rs::expected_objects_distinguishes_checking_twins_from_normal_objects_and_rejects_extras",
    );
    let _lock = env_lock();
    let _target = EnvGuard::set("FEMACS_TARGET", "aarch64-linux-android");
    let fx = Fixture::new("objects");
    let root = fx.path();
    let units = UnitsManifest {
        core: vec!["src/doprnt.rs".into()],
        library: vec!["lib/mini-gmp.rs".into()],
    };
    for name in [
        "doprnt.aarch64-linux-android.o",
        "doprnt.checking.aarch64-linux-android.o",
        "mini-gmp.aarch64-linux-android.o",
        "mini-gmp.checking.aarch64-linux-android.o",
    ] {
        fs::write(root.join(name), b"").expect("object fixture");
    }

    let normal = expected_objects(&units, root, false).expect("normal objects");
    let normal_text = normal
        .iter()
        .map(|p| p.display().to_string())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(normal_text.contains("doprnt"));
    assert!(normal_text.contains("mini-gmp"));
    assert!(!normal_text.contains(".checking."));
    assert!(!normal_text.contains("extra."));

    let checking = expected_objects(&units, root, true).expect("checking objects");
    let checking_text = checking
        .iter()
        .map(|p| p.display().to_string())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(checking_text.contains(".checking."));
    assert!(!checking_text.contains("extra."));
    fs::write(root.join("extra.aarch64-linux-android.o"), b"").expect("unexpected object");
    assert!(
        expected_objects(&units, root, false).is_err(),
        "unexpected target objects must fail"
    );
    assert!(
        expected_objects(&units, root, true).is_err(),
        "unexpected checking membership must fail"
    );
}

#[test]
fn missing_toolchain_is_reported_before_any_android_output_exists() {
    let _case_time = crate::test_support::CaseTimer::new(
        "android::tests.rs::missing_toolchain_is_reported_before_any_android_output_exists",
    );
    let fx = Fixture::new("missing-toolchain");
    let root = fx.path();
    let opts = toolchain_options(root);
    let result = discover_toolchain(&opts);
    assert!(
        result.is_err(),
        "empty explicit tool paths cannot be accepted"
    );
    assert!(!root.join("output").exists());
}

#[test]
fn build_preflight_failure_does_not_modify_tracked_config_or_create_a_product() {
    let _case_time = crate::test_support::CaseTimer::new(
        "android::tests.rs::build_preflight_failure_does_not_modify_tracked_config_or_create_a_product",
    );
    let fx = Fixture::new("build-failure");
    let root = fx.path();
    let config_before = fs::read(repo().join("src/include/config.rs")).expect("tracked config");
    let support = root.join("support");
    fs::create_dir_all(&support).expect("support");
    fs::write(
        support.join("config.json"),
        "{\"macros\":{},\"features\":{}}",
    )
    .expect("config");
    fs::write(support.join("core-units.txt"), "src/doprnt.c\n").expect("core");
    fs::write(support.join("lib-units.txt"), "lib/mini-gmp.c\n").expect("lib");
    let opts = BuildOptions {
        run_root: Some(root.join("run")),
        support_dir: support,
        tools: toolchain_options(root),
        package: false,
        assets: false,
        debuggable: false,
    };
    assert!(build_android(&opts).is_err());
    assert_eq!(
        fs::read(repo().join("src/include/config.rs")).expect("tracked config remains"),
        config_before,
        "Android build must not mutate tracked configuration"
    );
    assert!(
        !root.join("run/port").exists(),
        "no failed product may be published"
    );
}

#[test]
fn package_without_a_real_toolchain_or_native_product_fails_and_leaves_no_candidate() {
    let _case_time = crate::test_support::CaseTimer::new(
        "android::tests.rs::package_without_a_real_toolchain_or_native_product_fails_and_leaves_no_candidate",
    );
    let fx = Fixture::new("package-failure");
    let root = fx.path();
    let opts = PackageOptions {
        run_root: Some(root.join("run")),
        native_so: None,
        assets: false,
        debuggable: false,
        tools: toolchain_options(root),
    };
    assert!(package_apk(&opts).is_err());
    assert!(!root.join("run/emacs.apk").exists());
}

#[test]
fn reference_config_audit_requires_explicit_fresh_owned_paths() {
    let _case_time = crate::test_support::CaseTimer::new(
        "android::tests.rs::reference_config_audit_requires_explicit_fresh_owned_paths",
    );
    let fx = Fixture::new("config-audit");
    let root = fx.path();
    let opts = ConfigOptions {
        reference: root.join("missing-reference"),
        build_dir: root.join("audit-build"),
        out: root.join("audit-out"),
        tools: toolchain_options(root),
    };
    assert!(measure_reference_config(&opts).is_err());
    assert!(!root.join("audit-out").exists());

    let existing = root.join("existing-out");
    fs::create_dir_all(&existing).expect("existing output");
    let sentinel = existing.join("keep");
    fs::write(&sentinel, b"keep").expect("sentinel");
    let opts = ConfigOptions {
        reference: root.join("missing-reference"),
        build_dir: root.join("audit-build-2"),
        out: existing.clone(),
        tools: toolchain_options(root),
    };
    assert!(measure_reference_config(&opts).is_err());
    assert_eq!(fs::read(&sentinel).expect("existing evidence"), b"keep");
}
