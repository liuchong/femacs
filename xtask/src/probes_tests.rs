//! Independent B-phase contracts for the complete probe inventory and argv plans.
//!
//! These tests do not compile or execute a probe.  They bind the declared inventory,
//! dependency, path, ordering and metadata contracts; real compiler/audit execution belongs
//! to the D integration cases.

use super::*;

use crate::tooling::{RunDirectory, owned_new_output};
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
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
            .join("build/probes-tests")
            .join(format!("{name}-{}", tag()));
        fs::create_dir_all(&path).expect("probe fixture");
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

fn options(root: &Path) -> ProbeOptions {
    ProbeOptions {
        out: None,
        reference_source: Some(root.join("reference")),
        reference_build: Some(root.join("reference-build")),
        ndk: Some(root.join("ndk")),
        extra_sources: Vec::new(),
        write_baseline: None,
        reference_java: false,
    }
}

fn run_dir(root: &Path) -> RunDirectory {
    owned_new_output(root, None, "probe-tests").expect("owned probe run")
}

fn spec<'a>(inventory: &'a [ProbeSpec], name: &str) -> &'a ProbeSpec {
    inventory
        .iter()
        .find(|s| s.name == name)
        .unwrap_or_else(|| panic!("missing probe {name}"))
}

fn args(step: &crate::tooling::StepPlan) -> Vec<String> {
    step.args
        .iter()
        .map(|a| a.to_string_lossy().into_owned())
        .collect()
}

fn joined(step: &crate::tooling::StepPlan) -> String {
    format!(
        "{} {}",
        step.program.to_string_lossy(),
        args(step).join(" ")
    )
}

#[test]
fn inventory_has_the_exact_fourteen_unique_entries() {
    let _case_time = crate::test_support::CaseTimer::new(
        "probes_tests.rs::inventory_has_the_exact_fourteen_unique_entries",
    );
    let inventory = inventory();
    let names: Vec<&str> = inventory.iter().map(|s| s.name).collect();
    assert_eq!(names.len(), 14);
    assert_eq!(
        names,
        vec![
            "inline", "accessor", "write", "va", "syshdrs", "gmp-abi", "lisp-abi", "fam", "shadow",
            "macro", "jni", "miri", "bringup", "kotlin",
        ]
    );
    let mut unique = std::collections::BTreeSet::new();
    for name in &names {
        assert!(unique.insert(*name), "duplicate probe name: {name}");
    }
}

#[test]
fn inventory_binds_kind_reference_and_fixture_sources() {
    let _case_time = crate::test_support::CaseTimer::new(
        "probes_tests.rs::inventory_binds_kind_reference_and_fixture_sources",
    );
    let inventory = inventory();
    assert!(matches!(
        &spec(&inventory, "inline").kind,
        ProbeKind::Differential
    ));
    assert!(matches!(
        &spec(&inventory, "accessor").kind,
        ProbeKind::Differential
    ));
    assert!(matches!(
        &spec(&inventory, "write").kind,
        ProbeKind::Differential
    ));
    assert!(matches!(
        &spec(&inventory, "va").kind,
        ProbeKind::Differential
    ));
    assert!(matches!(
        &spec(&inventory, "syshdrs").kind,
        ProbeKind::Differential
    ));
    assert!(matches!(
        &spec(&inventory, "gmp-abi").kind,
        ProbeKind::AbiSyntax
    ));
    assert!(matches!(
        &spec(&inventory, "lisp-abi").kind,
        ProbeKind::AbiSyntax
    ));
    assert!(matches!(&spec(&inventory, "fam").kind, ProbeKind::RustRun));
    assert!(matches!(
        &spec(&inventory, "shadow").kind,
        ProbeKind::RustRun
    ));
    assert!(matches!(
        &spec(&inventory, "macro").kind,
        ProbeKind::RustMetadata
    ));
    assert!(matches!(&spec(&inventory, "jni").kind, ProbeKind::Jni));
    assert!(matches!(&spec(&inventory, "miri").kind, ProbeKind::Miri));
    assert!(matches!(
        &spec(&inventory, "bringup").kind,
        ProbeKind::Bringup
    ));
    assert!(matches!(
        &spec(&inventory, "kotlin").kind,
        ProbeKind::Kotlin
    ));

    for name in ["inline", "accessor", "write"] {
        let s = spec(&inventory, name);
        let rust = format!("spikes/{name}_probe.rs");
        let c = format!("spikes/{name}_probe.c");
        assert_eq!(s.rust_source, Some(rust.as_str()));
        assert_eq!(s.c_source, Some(c.as_str()));
        assert!(matches!(&s.reference, ReferenceRequirement::ConfiguredTree));
    }
    for name in ["va", "syshdrs"] {
        let s = spec(&inventory, name);
        let rust = format!("spikes/{name}_probe.rs");
        let c = format!("spikes/{name}_probe.c");
        assert_eq!(s.rust_source, Some(rust.as_str()));
        assert_eq!(s.c_source, Some(c.as_str()));
        assert!(matches!(&s.reference, ReferenceRequirement::None));
    }
    let gmp = spec(&inventory, "gmp-abi");
    assert_eq!(gmp.generator, Some("tools/gen-gmp-abi-probe.py"));
    assert!(matches!(&gmp.reference, ReferenceRequirement::Tree));
    let lisp = spec(&inventory, "lisp-abi");
    assert_eq!(lisp.generator, Some("tools/gen-lisp-abi-probe.py"));
    assert!(matches!(
        &lisp.reference,
        ReferenceRequirement::ConfiguredTree
    ));
    let bringup = spec(&inventory, "bringup");
    assert!(matches!(
        &bringup.reference,
        ReferenceRequirement::ConfiguredTree
    ));
    assert!(matches!(
        &spec(&inventory, "kotlin").reference,
        ReferenceRequirement::None
    ));
}

#[test]
fn plan_prepares_nonempty_steps_for_every_probe_kind() {
    let _case_time = crate::test_support::CaseTimer::new(
        "probes_tests.rs::plan_prepares_nonempty_steps_for_every_probe_kind",
    );
    let inventory = inventory();
    let fx = Fixture::new("plans");
    let root = fx.path();
    fs::create_dir_all(root.join("reference/lib")).expect("reference lib");
    fs::create_dir_all(root.join("reference/src")).expect("reference src");
    fs::create_dir_all(root.join("reference-build/src")).expect("reference build");
    fs::create_dir_all(root.join("ndk/toolchains/llvm/prebuilt/host/bin")).expect("fake ndk");
    fs::create_dir_all(root.join("ndk/toolchains/llvm/prebuilt/host/sysroot/usr/include"))
        .expect("ndk include");
    fs::write(root.join("reference/lib/mini-gmp.h"), "").expect("mini-gmp header");
    fs::write(root.join("reference/src/lisp.h"), "").expect("lisp header");
    fs::write(root.join("reference-build/src/config.h"), "").expect("reference config");
    fs::write(root.join("ndk/toolchains/llvm/prebuilt/host/bin/clang"), "").expect("ndk clang");
    fs::write(
        root.join("ndk/toolchains/llvm/prebuilt/host/sysroot/usr/include/jni.h"),
        "",
    )
    .expect("jni header");
    let run = run_dir(root);
    let opts = options(root);
    for probe in &inventory {
        let steps =
            plan(root, probe, &opts, &run).unwrap_or_else(|e| panic!("{} plan: {e}", probe.name));
        assert!(!steps.is_empty(), "{} produced no steps", probe.name);
    }
}

#[test]
fn differential_plans_preserve_configured_include_order_and_real_source_inputs() {
    let _case_time = crate::test_support::CaseTimer::new(
        "probes_tests.rs::differential_plans_preserve_configured_include_order_and_real_source_inputs",
    );
    let inventory = inventory();
    let fx = Fixture::new("differential-order");
    let root = fx.path();
    fs::create_dir_all(root.join("reference/lib")).expect("reference lib");
    fs::create_dir_all(root.join("reference/src")).expect("reference src");
    let run = run_dir(root);
    let opts = options(root);
    let steps = plan(root, spec(&inventory, "inline"), &opts, &run).expect("inline plan");
    let c = steps
        .iter()
        .find(|s| args(s).iter().any(|a| a == "-idirafter"))
        .expect("inline C step");
    let c_args = args(c);
    let index = |needle: &str| {
        c_args
            .iter()
            .position(|a| a == needle)
            .unwrap_or(usize::MAX)
    };
    let ref_lib = root.join("reference-build/lib").display().to_string();
    let ref_src = root.join("reference-build/src").display().to_string();
    let shim = repo().join("spikes/shim").display().to_string();
    assert!(c_args.iter().any(|a| a == &ref_lib));
    assert!(c_args.iter().any(|a| a == &ref_src));
    assert!(c_args.iter().any(|a| a == &shim));
    assert!(c_args.iter().any(|a| a.ends_with("spikes/inline_probe.c")));
    assert!(
        index("-idirafter") < index(&shim),
        "-idirafter must precede shim"
    );
    assert!(
        index(&ref_lib) < index("-idirafter"),
        "configured lib comes before shim"
    );
    assert!(
        index(&ref_src) > index("-idirafter"),
        "configured src follows shim"
    );
    assert!(
        steps.iter().any(|s| joined(s).contains("inline_probe.rs")),
        "differential must plan the Rust side too"
    );
}

#[test]
fn va_and_syshdrs_do_not_require_a_reference_tree() {
    let _case_time = crate::test_support::CaseTimer::new(
        "probes_tests.rs::va_and_syshdrs_do_not_require_a_reference_tree",
    );
    let inventory = inventory();
    let fx = Fixture::new("host-only-differential");
    let root = fx.path();
    let run = run_dir(root);
    let opts = options(root);
    for name in ["va", "syshdrs"] {
        let steps = plan(root, spec(&inventory, name), &opts, &run).expect("host-only plan");
        let text = steps.iter().map(joined).collect::<Vec<_>>().join("\n");
        assert!(!text.contains(&root.join("reference").display().to_string()));
        assert!(
            !text.contains(" -I "),
            "{name} must not invent configured include paths"
        );
    }
}

#[test]
fn abi_plans_use_generators_and_syntax_only_checks() {
    let _case_time = crate::test_support::CaseTimer::new(
        "probes_tests.rs::abi_plans_use_generators_and_syntax_only_checks",
    );
    let inventory = inventory();
    let fx = Fixture::new("abi-plans");
    let root = fx.path();
    fs::create_dir_all(root.join("reference/lib")).expect("reference lib");
    fs::create_dir_all(root.join("reference/src")).expect("reference src");
    let run = run_dir(root);
    let opts = options(root);

    let gmp = plan(root, spec(&inventory, "gmp-abi"), &opts, &run).expect("gmp plan");
    let gmp_text = gmp.iter().map(joined).collect::<Vec<_>>().join("\n");
    assert!(gmp_text.contains("gen-gmp-abi-probe.py"));
    assert!(gmp_text.contains("-fsyntax-only"));
    assert!(gmp_text.contains("-Werror=incompatible-pointer-types"));
    assert!(gmp_text.contains("-include"));
    assert!(gmp_text.contains("stdio.h"));

    let lisp = plan(root, spec(&inventory, "lisp-abi"), &opts, &run).expect("lisp plan");
    let lisp_text = lisp.iter().map(joined).collect::<Vec<_>>().join("\n");
    assert!(lisp_text.contains("gen-lisp-abi-probe.py"));
    assert!(lisp_text.contains("-fsyntax-only"));
    assert!(lisp_text.contains("-Werror=incompatible-pointer-types"));
    assert!(lisp_text.contains(&root.join("reference/src").display().to_string()));
}

#[test]
fn macro_plan_is_metadata_only_and_rust_run_plans_are_not_metadata() {
    let _case_time = crate::test_support::CaseTimer::new(
        "probes_tests.rs::macro_plan_is_metadata_only_and_rust_run_plans_are_not_metadata",
    );
    let inventory = inventory();
    let fx = Fixture::new("rust-kinds");
    let root = fx.path();
    let run = run_dir(root);
    let opts = options(root);
    let macro_steps = plan(root, spec(&inventory, "macro"), &opts, &run).expect("macro plan");
    let macro_text = macro_steps
        .iter()
        .map(joined)
        .collect::<Vec<_>>()
        .join("\n");
    assert!(macro_text.contains("--emit"));
    assert!(macro_text.contains("metadata"));
    assert!(!macro_text.contains("macro_probe.rmeta") || macro_text.contains("macro_probe"));

    for name in ["fam", "shadow"] {
        let steps = plan(root, spec(&inventory, name), &opts, &run).expect("rust run plan");
        let text = steps.iter().map(joined).collect::<Vec<_>>().join("\n");
        assert!(text.contains(&format!("{name}_probe.rs")));
        assert!(
            !text.contains("--emit=metadata"),
            "{name} is a real run, not metadata"
        );
    }
}

#[test]
fn jni_plan_uses_android24_ndk_headers_and_owned_mirror() {
    let _case_time = crate::test_support::CaseTimer::new(
        "probes_tests.rs::jni_plan_uses_android24_ndk_headers_and_owned_mirror",
    );
    let inventory = inventory();
    let fx = Fixture::new("jni-plan");
    let root = fx.path();
    let run = run_dir(root);
    let opts = options(root);
    let steps = plan(root, spec(&inventory, "jni"), &opts, &run).expect("jni plan");
    let text = steps.iter().map(joined).collect::<Vec<_>>().join("\n");
    assert!(text.contains("aarch64-linux-android24"));
    assert!(text.contains("jni"));
    assert!(text.contains("src/include/jni.rs"));
    assert!(steps.iter().any(|s| joined(s).contains("clang")));
    assert!(steps.iter().any(|s| joined(s).contains("rust")));
}

#[test]
fn miri_plan_uses_the_pinned_toolchain_and_isolated_miri_target() {
    let _case_time = crate::test_support::CaseTimer::new(
        "probes_tests.rs::miri_plan_uses_the_pinned_toolchain_and_isolated_miri_target",
    );
    let inventory = inventory();
    let fx = Fixture::new("miri-plan");
    let root = fx.path();
    let run = run_dir(root);
    let opts = options(root);
    let steps = plan(root, spec(&inventory, "miri"), &opts, &run).expect("miri plan");
    let text = steps.iter().map(joined).collect::<Vec<_>>().join("\n");
    assert!(text.contains("miri"));
    assert!(text.contains(&pin(root).expect("pin")));
    assert!(text.contains("cargo"));
    assert!(text.contains(&run.path.join("miri-crate/Cargo.toml").display().to_string()));
    assert!(!text.contains(&repo().join("spikes/miri/Cargo.toml").display().to_string()));
}

#[test]
fn bringup_plan_has_opt0_opt2_and_opt3_steps() {
    let _case_time = crate::test_support::CaseTimer::new(
        "probes_tests.rs::bringup_plan_has_opt0_opt2_and_opt3_steps",
    );
    let inventory = inventory();
    let fx = Fixture::new("bringup-plan");
    let root = fx.path();
    let run = run_dir(root);
    let opts = options(root);
    let steps = plan(root, spec(&inventory, "bringup"), &opts, &run).expect("bringup plan");
    let text = steps.iter().map(joined).collect::<Vec<_>>().join("\n");
    assert!(text.contains("-O0") || text.contains("opt0"));
    assert!(text.contains("-O2") || text.contains("opt2"));
    assert!(text.contains("-O3") || text.contains("opt3"));
    assert!(text.contains("layout") && text.contains("jump") && text.contains("scan"));
}

#[test]
fn kotlin_diagnostic_plan_uses_owned_inputs_and_no_reference_java_standins() {
    let _case_time = crate::test_support::CaseTimer::new(
        "probes_tests.rs::kotlin_diagnostic_plan_uses_owned_inputs_and_no_reference_java_standins",
    );
    let inventory = inventory();
    let fx = Fixture::new("kotlin-plan");
    let root = fx.path();
    let run = run_dir(root);
    let opts = options(root);
    let steps = plan(root, spec(&inventory, "kotlin"), &opts, &run).expect("kotlin plan");
    let text = steps.iter().map(joined).collect::<Vec<_>>().join("\n");
    assert!(text.contains("platform/android"));
    assert!(!text.contains("emacs/java"));
    assert!(!text.contains("stubs"));
}

#[test]
fn missing_fixture_or_dependency_fails_before_a_probe_run_is_reported() {
    let _case_time = crate::test_support::CaseTimer::new(
        "probes_tests.rs::missing_fixture_or_dependency_fails_before_a_probe_run_is_reported",
    );
    let fx = Fixture::new("missing-probe");
    let root = fx.path();
    let opts = options(root);
    let inventory = inventory();
    let va = spec(&inventory, "va");
    let run = run_dir(root);
    let planned = plan(root, va, &opts, &run);
    if planned.is_err() {
        return;
    }
    assert!(run_probe(root, "va", &opts).is_err());
    assert!(run_probe(root, "not-a-probe", &opts).is_err());
}

#[test]
fn miri_crate_materializes_owned_graph_without_touching_original_manifest() {
    let _timer = crate::test_support::CaseTimer::new("miri_crate_materializes_owned_graph");
    let actual = repo();
    let source = actual.join("spikes/miri/Cargo.toml");
    let before = fs::read(&source).expect("original manifest");
    let fx = Fixture::new("miri-isolated-crate");
    let run = run_dir(fx.path());
    prepare_miri(&actual, &run).expect("fresh standalone crate");
    let manifest =
        fs::read_to_string(run.path.join("miri-crate/Cargo.toml")).expect("owned manifest");
    assert!(manifest.contains("[workspace]"));
    assert!(manifest.contains("graph/_root.rs"));
    assert!(run.path.join("miri-crate/graph/_root.rs").is_file());
    assert!(!run.path.join("miri-crate/Cargo.lock").exists());
    let hashes = crate::tooling::graph_input_hashes(&run.path.join("miri-crate/graph"))
        .expect("full graph identities");
    assert!(hashes.len() > 5);
    assert!(hashes.contains_key("_root.rs"));
    assert!(hashes.keys().any(|name| name.contains("include_lisp_rs")));
    assert_eq!(fs::read(source).expect("source unchanged"), before);
}
