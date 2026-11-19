//! Stage B cases for runtime configuration ownership. Not executed before D.
//! Included as a child module of runtime.rs; no environment mutation is needed.
use super::*;

#[test]
fn native_object_selection_keeps_the_base_and_adds_one_dynlib_for_both_legs() {
    let objects = vec!["emacs.o".to_owned(), "comp.o".to_owned()];
    for variant in ["native", "native-collector"] {
        for leg in ["normal", "checking"] {
            let selected = select_product_objects(objects.clone(), Some(&json!({
                "variant":variant, "compiler_leg":leg
            }))).unwrap();
            assert_eq!(selected, ["emacs.o", "comp.o", "dynlib.o"]);
            assert_eq!(select_product_objects(selected.clone(), Some(&json!({"variant":variant}))).unwrap(), selected);
        }
    }
}

#[test]
fn baseline_collector_and_unbound_object_selection_preserve_exact_inputs() {
    let objects = vec!["emacs.o".to_owned(), "comp.o".to_owned()];
    assert_eq!(select_product_objects(objects.clone(), None).unwrap(), objects);
    for variant in ["baseline", "collector"] {
        assert_eq!(select_product_objects(objects.clone(), Some(&json!({"variant":variant}))).unwrap(), objects);
    }
}

#[test]
fn bound_object_selection_rejects_unknown_or_missing_variants() {
    for binding in [json!({}), json!({"variant":null}), json!({"variant":"other"})] {
        assert!(select_product_objects(vec!["emacs.o".into()], Some(&binding)).is_err());
    }
}

#[test]
fn compiler_pass_selection_preserves_legacy_and_exact_bound_legs() {
    assert_eq!(compile_legs(None).unwrap(), (true, true));
    assert_eq!(compile_legs(Some(&json!({"compiler_leg":"normal"}))).unwrap(), (true, false));
    assert_eq!(compile_legs(Some(&json!({"compiler_leg":"checking"}))).unwrap(), (false, true));
}

#[test]
fn compiler_pass_selection_rejects_unknown_or_missing_bound_legs() {
    for binding in [json!({}), json!({"compiler_leg":null}),
                    json!({"compiler_leg":"both"}), json!({"compiler_leg":true})] {
        assert!(compile_legs(Some(&binding)).is_err());
    }
}

#[test]
fn helper_inputs_select_checked_tool_and_libraries_from_a_checked_only_root() {
    let root = owned_repo("helper-checked").join("objects");
    fs::create_dir(&root).unwrap();
    for name in ["make-fingerprint.checking.o", "sha256.checking.o", "mini-gmp-gnulib.checking.o"] {
        fs::write(root.join(name), name.as_bytes()).unwrap();
    }
    let members = vec!["sha256.o".to_owned(), "mini-gmp-gnulib.o".to_owned()];
    let (tool, libraries) = crate::helper_link_inputs(&root, "make-fingerprint", &members, true).unwrap();
    assert_eq!(tool, root.join("make-fingerprint.checking.o"));
    assert_eq!(libraries.len(), 2);
    assert!(libraries.iter().all(|p| p.ends_with(".checking.o")));
}

#[test]
fn helper_inputs_preserve_plain_legacy_names_and_gnulib_aliases() {
    let root = owned_repo("helper-plain").join("objects");
    fs::create_dir(&root).unwrap();
    for name in ["make-fingerprint.o", "sha256.o", "mini-gmp.o", "sha256.checking.o"] {
        fs::write(root.join(name), name.as_bytes()).unwrap();
    }
    let members = vec!["sha256.o".to_owned(), "mini-gmp-gnulib.o".to_owned()];
    let (tool, libraries) = crate::helper_link_inputs(&root, "make-fingerprint", &members, false).unwrap();
    assert_eq!(tool, root.join("make-fingerprint.o"));
    assert_eq!(libraries.len(), 2);
    assert!(libraries.iter().all(|p| !p.ends_with(".checking.o")));
    assert!(libraries.iter().any(|p| p.ends_with("mini-gmp.o")));
}

#[test]
fn helper_inputs_reject_the_opposite_leg_instead_of_falling_back() {
    let root = owned_repo("helper-opposite").join("objects");
    fs::create_dir(&root).unwrap();
    for name in ["make-fingerprint.o", "sha256.o"] {
        fs::write(root.join(name), name.as_bytes()).unwrap();
    }
    let members = vec!["sha256.o".to_owned()];
    assert!(crate::helper_link_inputs(&root, "make-fingerprint", &members, true).is_err());
    fs::write(root.join("make-fingerprint.checking.o"), b"checked tool").unwrap();
    assert!(crate::helper_link_inputs(&root, "make-fingerprint", &members, true).is_err());
}
use serde_json::json;

fn args(values: &[&str]) -> Vec<String> {
    values.iter().map(|s| (*s).to_owned()).collect()
}

fn options(product: &str, variant: &str, observer: bool) -> RuntimeBuildOptions {
    let mut values = args(&["--product", product, "--variant", variant,
        "--out", "build/runtime/contract-case"]);
    if variant == "native" || variant == "native-collector" {
        values.extend(args(&["--backend-prefix", "build/runtime/backend-input"]));
    }
    if observer {
        values.push("--observer".into());
    }
    parse_options(&values).expect("valid explicit fixture options")
}

fn base() -> serde_json::Value {
    json!({"macros": {
        "HAVE_NATIVE_COMP": "undefined", "HAVE_LIBGCCJIT_H": "undefined",
        "HAVE_LIBGCCJIT": "undefined", "FEMACS_GC_OBSERVER": "undefined",
        "FEMACS_GC_GENERATIONAL": "undefined", "HAVE_NS": "1",
        "EMACS_CONFIG_OPTIONS": "\"fixture\""
    }, "features": {}, "feature_strings": {}})
}

#[test]
fn parse_both_products_and_all_variants() {
    for product in ["terminal", "cocoa"] {
        for variant in ["baseline", "native", "collector", "native-collector"] {
            let parsed = options(product, variant, false);
            assert_eq!(parsed.out, PathBuf::from("build/runtime/contract-case"));
            assert!(!parsed.observer);
            assert_eq!(parsed.backend_prefix.is_some(), variant.contains("native"));
        }
    }
    assert!(options("cocoa", "native-collector", true).observer);
}

#[test]
fn reject_missing_required_options_and_values() {
    for values in [
        vec![],
        args(&["--product", "terminal"]),
        args(&["--product", "terminal", "--variant", "baseline"]),
        args(&["--product", "terminal", "--variant", "baseline", "--out"]),
    ] {
        assert!(parse_options(&values).is_err(), "accepted incomplete options: {values:?}");
    }
}

#[test]
fn reject_unknown_products_variants_options_and_positionals() {
    for (key, value) in [("--product", "unknown"), ("--variant", "unknown")] {
        let mut values = args(&["--product", "terminal", "--variant", "baseline", "--out", "build/runtime/case"]);
        let index = values.iter().position(|v| v == key).unwrap();
        values[index + 1] = value.into();
        assert!(parse_options(&values).is_err());
    }
    for extra in ["--unknown", "positional"] {
        let mut values = args(&["--product", "terminal", "--variant", "baseline", "--out", "build/runtime/case"]);
        values.push(extra.into());
        assert!(parse_options(&values).is_err());
    }
}

#[test]
fn reject_duplicate_options_even_if_values_are_equal() {
    for extra in [args(&["--product", "terminal"]), args(&["--variant", "baseline"]),
                  args(&["--out", "build/runtime/case"]), args(&["--observer", "--observer"])] {
        let mut values = args(&["--product", "terminal", "--variant", "baseline", "--out", "build/runtime/case"]);
        values.extend(extra);
        assert!(parse_options(&values).is_err());
    }
}

#[test]
fn backend_prefix_is_required_for_native_and_rejected_otherwise() {
    let native = args(&["--product", "terminal", "--variant", "native", "--out", "build/runtime/case"]);
    assert!(parse_options(&native).is_err());
    let nonnative = args(&["--product", "terminal", "--variant", "collector", "--out", "build/runtime/case",
                          "--backend-prefix", "build/runtime/backend"]);
    assert!(parse_options(&nonnative).is_err());
}

#[test]
fn export_fault_hooks_require_an_explicit_observer_profile() {
    let mut good = args(&["--product", "terminal", "--variant", "baseline", "--out", "build/runtime/case",
                          "--observer", "--test-hooks"]);
    let selected = parse_options(&good).unwrap();
    assert!(selected.observer && selected.test_hooks);
    let configured = canonical_config(&base(), &selected).unwrap();
    assert_eq!(configured["macros"]["FEMACS_GC_OBSERVER_TEST_HOOKS"], "1");
    good.retain(|arg| arg != "--observer");
    assert!(parse_options(&good).is_err());
    assert!(!options("terminal", "baseline", false).test_hooks);
}

#[test]
fn configuration_does_not_mutate_the_committed_base() {
    let input = base();
    let saved = input.clone();
    let selected = canonical_config(&input, &options("cocoa", "native-collector", true)).unwrap();
    assert_eq!(input, saved);
    for key in ["HAVE_NATIVE_COMP", "HAVE_LIBGCCJIT_H", "HAVE_LIBGCCJIT",
                "FEMACS_GC_OBSERVER", "FEMACS_GC_GENERATIONAL"] {
        assert_eq!(selected["macros"][key], "1", "missing selected macro {key}");
    }
    assert_eq!(selected["macros"]["HAVE_NS"], saved["macros"]["HAVE_NS"]);
}

#[test]
fn collector_and_observer_are_independent_of_native() {
    let selected = canonical_config(&base(), &options("terminal", "collector", true)).unwrap();
    assert_eq!(selected["macros"]["HAVE_NATIVE_COMP"], "undefined");
    assert_eq!(selected["macros"]["FEMACS_GC_GENERATIONAL"], "1");
    assert_eq!(selected["macros"]["FEMACS_GC_OBSERVER"], "1");
    let baseline = canonical_config(&base(), &options("terminal", "baseline", false)).unwrap();
    assert_eq!(baseline["macros"]["FEMACS_GC_GENERATIONAL"], "undefined");
    assert_eq!(baseline["macros"]["FEMACS_GC_OBSERVER"], "undefined");
}

#[test]
fn case_normalized_macro_feature_conflict_is_not_silently_overridden() {
    let mut input = base();
    input["macros"]["HAVE_NATIVE_COMP"] = json!("1");
    input["features"]["have_native_comp"] = json!("false");
    assert!(canonical_config(&input, &options("cocoa", "native", false)).is_err());
}

#[test]
fn same_normalized_setting_is_not_a_false_conflict() {
    let mut input = base();
    input["features"]["have_native_comp"] = json!("undefined");
    let selected = canonical_config(&input, &options("cocoa", "native", false)).unwrap();
    assert_eq!(selected["macros"]["HAVE_NATIVE_COMP"], "1");
    assert_eq!(selected["features"]["have_native_comp"], "1");
}

#[test]
fn canonical_digest_ignores_key_order_but_not_feature_changes() {
    let first = canonical_config(&base(), &options("terminal", "baseline", false)).unwrap();
    let text = serde_json::to_string_pretty(&first).unwrap();
    let reparsed: serde_json::Value = serde_json::from_str(&text).unwrap();
    let a = config_digest(&first).unwrap();
    assert_eq!(a, config_digest(&reparsed).unwrap());
    assert_eq!(a.len(), 64);
    assert!(a.bytes().all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c)));
    let other = canonical_config(&base(), &options("terminal", "collector", false)).unwrap();
    assert_ne!(a, config_digest(&other).unwrap());
}

#[test]
fn abi_key_does_not_hash_itself() {
    let mut input = base();
    let first = abi_input_digest(&input).unwrap();
    let config_before = config_digest(&input).unwrap();
    input["macros"]["FEMACS_RUNTIME_ABI_ID"] = json!("\"previous-run-id\"");
    assert_eq!(first, abi_input_digest(&input).unwrap());
    assert_ne!(config_before, config_digest(&input).unwrap());
}

fn owned_repo(name: &str) -> PathBuf {
    use std::sync::atomic::{AtomicU64, Ordering};
    static SEQUENCE: AtomicU64 = AtomicU64::new(0);
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap()
        .join("build/acceptance/runtime-build-contracts")
        .join(format!("{name}-{}-{}", std::process::id(), SEQUENCE.fetch_add(1, Ordering::Relaxed)));
    std::fs::create_dir_all(root.join("build/runtime")).unwrap();
    std::fs::create_dir_all(root.join("dist/macos/Femacs.app")).unwrap();
    std::fs::create_dir_all(root.join("emacs")).unwrap();
    root
}

#[test]
fn output_is_fresh_under_owned_runtime_tree() {
    let repo = owned_repo("fresh");
    let out = repo.join("build/runtime/candidate");
    assert_eq!(validate_output(&repo, &out, false).unwrap(), out);
    assert!(!out.exists(), "validation must not start the build or publish anything");
    for bad in [&repo, &repo.join("emacs/output"), &repo.join("dist/macos/Femacs.app"),
                &repo.join("build"), &repo.join("other-output")] {
        assert!(validate_output(&repo, bad, false).is_err(), "unsafe output accepted: {bad:?}");
    }
}

#[test]
fn existing_output_is_preserved_and_only_explicit_continuation_can_open_it() {
    let repo = owned_repo("existing");
    let out = repo.join("build/runtime/candidate");
    std::fs::create_dir(&out).unwrap();
    std::fs::write(out.join("keep"), b"unchanged candidate").unwrap();
    assert!(validate_output(&repo, &out, false).is_err());
    assert_eq!(std::fs::read(out.join("keep")).unwrap(), b"unchanged candidate");
    assert_eq!(validate_output(&repo, &out, true).unwrap(), out);
    assert!(validate_output(&repo, &repo.join("dist/macos/Femacs.app"), true).is_err());
}

#[cfg(unix)]
#[test]
fn symlink_components_and_parent_escape_are_rejected() {
    let repo = owned_repo("symlink");
    let alias = repo.join("build/runtime/alias");
    std::os::unix::fs::symlink(repo.join("emacs"), &alias).unwrap();
    assert!(validate_output(&repo, &alias.join("candidate"), false).is_err());
    assert!(validate_output(&repo, &repo.join("build/runtime/../../emacs/candidate"), false).is_err());
}

fn continuation() -> serde_json::Value {
    json!({"schema":1,"source_digest":"a".repeat(64),"config_digest":"b".repeat(64),
        "product":"cocoa","variant":"native","target":"aarch64-apple-darwin",
        "optimization":"2","native_abi":"c".repeat(64),
        "backend_identity":{"library_sha256":"d".repeat(64),"header_sha256":"e".repeat(64)}})
}

#[test]
fn continuation_is_bound_to_every_selected_identity() {
    let expected = continuation();
    assert!(validate_continuation(&expected, &expected).is_ok());
    for key in ["source_digest", "config_digest", "product", "variant", "target", "optimization", "native_abi", "backend_identity"] {
        let mut changed = expected.clone();
        changed[key] = json!("other");
        assert!(validate_continuation(&expected, &changed).is_err(), "ignored continuation field {key}");
        let mut missing = expected.clone();
        missing.as_object_mut().unwrap().remove(key);
        assert!(validate_continuation(&expected, &missing).is_err());
    }
}

fn dependency(name: &str, edges: &[&str]) -> BackendDependency {
    BackendDependency {
        logical_id: name.into(), source: PathBuf::from("fixture-libraries").join(name),
        source_sha256: "a".repeat(64), staged_sha256: None,
        destination: PathBuf::from("runtime-native").join(name),
        install_id: format!("@loader_path/{name}"),
        edges: edges.iter().map(|e| (*e).into()).collect(),
        license: PathBuf::from("licenses").join(format!("{name}.COPYING")),
    }
}

#[test]
fn compiler_leg_has_a_distinct_configuration_and_bound_continuation() {
    let mut normal = continuation();
    normal["compiler_leg"] = json!("normal");
    let mut checking = normal.clone();
    checking["compiler_leg"] = json!("checking");
    assert!(validate_continuation(&normal, &checking).is_err());
    let mut config = base();
    let before = abi_input_digest(&config).unwrap();
    config["runtime_compiler_leg"] = json!("checking");
    assert_ne!(before, abi_input_digest(&config).unwrap());
}

#[test]
fn checking_link_uses_checked_rust_and_existing_swift_providers() {
    for stem in ["alloc", "comp", "mini-gmp", "abi_shim"] {
        assert_eq!(selected_object_name(stem, true, false), format!("{stem}.checking.o"));
        assert_eq!(selected_object_name(stem, false, false), format!("{stem}.o"));
    }
    assert_eq!(selected_object_name("nsterm", true, true), "nsterm.o");
    assert_eq!(selected_object_name("macfont", false, true), "macfont.o");
}

#[test]
fn runtime_terminal_paths_use_executable_relative_live_resources() {
    let source = terminal_paths_source(Path::new("owned-source"), Path::new("owned-build"));
    for name in ["PATH_LOADSEARCH", "PATH_DUMPLOADSEARCH", "PATH_DUMPLOADSEARCH_SLASH",
                 "PATH_EXEC", "PATH_DATA", "PATH_DOC", "PATH_INFO", "PATH_BITMAPS"] {
        assert!(source.contains(&format!("pub fn {name}()")));
    }
    assert!(source.contains("_NSGetExecutablePath"));
    assert!(source.contains("AtomicU8"));
    assert!(source.contains("will_dump_p()"));
    assert!(source.contains("../lisp"));
    assert!(source.contains("../info"));
    assert!(!source.contains("std::env::current_exe"));
}

#[test]
fn exported_source_binds_an_explicit_nonempty_inventory_and_current_bytes() {
    let repo = owned_repo("export-inventory");
    std::fs::create_dir_all(repo.join("src")).unwrap();
    std::fs::write(repo.join("src/alloc.rs"), b"original source\n").unwrap();
    std::fs::write(repo.join(".femacs-runtime-sources.json"),
        serde_json::to_vec(&json!({"schema":1,"files":["src/alloc.rs"]})).unwrap()).unwrap();
    let original = source_digest(&repo).unwrap();
    std::fs::write(repo.join("src/alloc.rs"), b"changed source\n").unwrap();
    assert_ne!(original, source_digest(&repo).unwrap());
}

#[test]
fn absent_export_inventory_cannot_be_an_empty_parent_git_hash() {
    let repo = owned_repo("missing-export-inventory");
    assert!(source_digest(&repo).is_err());
    std::fs::write(repo.join(".femacs-runtime-sources.json"),
        serde_json::to_vec(&json!({"schema":1,"files":[]})).unwrap()).unwrap();
    assert!(source_digest(&repo).is_err());
}

#[test]
fn frozen_source_digest_accepts_a_current_directory_based_repo_locator() {
    let repo = owned_repo("relative-export-owner");
    std::fs::create_dir_all(repo.join("src")).unwrap();
    std::fs::write(repo.join("src/alloc.rs"), b"bound current bytes\n").unwrap();
    std::fs::write(repo.join(".femacs-runtime-sources.json"),
        serde_json::to_vec(&json!({"schema":1,"files":["src/alloc.rs"]})).unwrap()).unwrap();
    let cwd = std::env::current_dir().unwrap();
    let relative = if let Ok(sub) = repo.strip_prefix(&cwd) {
        Path::new(".").join(sub)
    } else {
        Path::new("./..").join(repo.strip_prefix(cwd.parent().unwrap()).unwrap())
    };
    assert_eq!(source_digest(&repo).unwrap(), source_digest(&relative).unwrap());
}

#[test]
fn captured_backend_names_select_the_real_primary_and_preserve_staged_copies() {
    let repo = owned_repo("backend-link-inputs");
    let root = repo.join("build/runtime/candidate");
    std::fs::create_dir_all(root.join("runtime-native")).unwrap();
    let mut dependencies = Vec::new();
    for name in ["libgccjit.0.dylib", "libgcc_s.1.1.dylib"] {
        let relative = format!("runtime-native/{name}");
        std::fs::write(root.join(&relative), name.as_bytes()).unwrap();
        dependencies.push(json!({"logical_id":relative,"destination":relative}));
    }
    let flags = backend_link_flags(&root, &dependencies).unwrap();
    assert_eq!(flags[0], root.join("runtime-native/libgccjit.0.dylib").display().to_string());
    assert_eq!(backend_link_flags(&root, &dependencies).unwrap(), flags);
    for name in ["libgccjit.0.dylib", "libgcc_s.1.1.dylib"] {
        assert_eq!(std::fs::read(root.join("port/src/runtime-native").join(name)).unwrap(), name.as_bytes());
    }
    let copied = root.join("port/src/runtime-native/libgccjit.0.dylib");
    std::fs::write(&copied, b"changed staged copy").unwrap();
    assert!(backend_link_flags(&root, &dependencies).is_err());
    assert_eq!(std::fs::read(copied).unwrap(), b"changed staged copy");
}

#[test]
fn backend_link_inputs_reject_missing_primary_and_unsafe_destinations() {
    let repo = owned_repo("backend-link-invalid");
    let root = repo.join("build/runtime/candidate");
    std::fs::create_dir_all(&root).unwrap();
    assert!(backend_link_flags(&root, &[]).is_err());
    let dependencies = [json!({"logical_id":"runtime-native/libgccjit.0.dylib",
                               "destination":"../outside.dylib"})];
    assert!(backend_link_flags(&root, &dependencies).is_err());
    assert!(!root.join("port").exists());
}

#[test]
fn tree_receipt_binds_actual_bytes_modes_and_relative_links() {
    let repo = owned_repo("tree-identity");
    std::fs::create_dir_all(repo.join("resources/lisp")).unwrap();
    std::fs::write(repo.join("resources/lisp/one.el"), b"(list 1)\n").unwrap();
    let before = tree_identity(&repo.join("resources")).unwrap();
    assert_eq!(before["lisp/one.el"]["kind"], "file");
    assert_eq!(before["lisp/one.el"]["bytes"], 9);
    std::fs::write(repo.join("resources/lisp/one.el"), b"(list 2)\n").unwrap();
    assert_ne!(before, tree_identity(&repo.join("resources")).unwrap());
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink("lisp/one.el", repo.join("resources/alias")).unwrap();
        assert_eq!(tree_identity(&repo.join("resources")).unwrap()["alias"]["kind"], "symlink");
        assert_eq!(tree_identity(&repo.join("resources")).unwrap()["alias"]["target"], "lisp/one.el");
    }
}

#[test]
fn tree_receipt_refuses_an_empty_or_missing_resource_tree() {
    let repo = owned_repo("empty-resource-tree");
    assert!(tree_identity(&repo.join("missing")).is_err());
    std::fs::create_dir(repo.join("empty")).unwrap();
    assert!(tree_identity(&repo.join("empty")).is_err());
}

#[test]
fn dependency_edges_must_resolve_without_duplicate_destinations() {
    let one = dependency("one.dylib", &["two.dylib", "system:libSystem"]);
    let two = dependency("two.dylib", &["system:libSystem"]);
    assert!(validate_dependencies(&[one, two]).is_ok());
    assert!(validate_dependencies(&[dependency("one.dylib", &["missing.dylib"])]).is_err());
    assert!(validate_dependencies(&[dependency("same.dylib", &[]), dependency("same.dylib", &[])]).is_err());
}

#[test]
fn dependency_copy_and_license_destinations_cannot_escape_package() {
    let mut bad = dependency("one.dylib", &[]);
    bad.destination = PathBuf::from("../outside");
    assert!(validate_dependencies(&[bad]).is_err());
    let mut bad = dependency("one.dylib", &[]);
    bad.license = PathBuf::from("../outside-license");
    assert!(validate_dependencies(&[bad]).is_err());
}
