//! Independent B-phase contracts for the owned tooling execution boundary.
//!
//! These tests are compiled only after the C implementation includes this file from
//! `tooling.rs`.  They never infer a product default and never hide a failed child as a
//! successful receipt.

use super::*;

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
            .join("build/tooling-tests")
            .join(format!("{name}-{}", tag()));
        fs::create_dir_all(&path).expect("create tooling fixture");
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

fn log_text(path: &Path, run: &RunDirectory) -> String {
    fs::read_to_string(path)
        .or_else(|_| fs::read_to_string(run.path.join(path)))
        .unwrap_or_default()
}

fn all_log_text(run: &RunDirectory) -> String {
    let mut text = String::new();
    let Ok(entries) = fs::read_dir(&run.path) else {
        return text;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
            if let Ok(s) = fs::read_to_string(&path) {
                text.push_str(&s);
                text.push('\n');
            }
        }
    }
    text
}

fn plan(id: &str, program: impl Into<OsString>, args: &[&str], cwd: &Path) -> StepPlan {
    StepPlan {
        id: id.to_string(),
        program: program.into(),
        args: args.iter().map(OsString::from).collect(),
        cwd: cwd.to_path_buf(),
        env: Vec::new(),
    }
}

#[test]
fn owned_output_is_fresh_and_does_not_reuse_a_previous_run() {
    let _case_time = crate::test_support::CaseTimer::new(
        "tooling_tests.rs::owned_output_is_fresh_and_does_not_reuse_a_previous_run",
    );
    let fx = Fixture::new("owned-fresh");
    fs::create_dir_all(fx.path().join("build")).expect("build dir");

    let first = owned_new_output(fx.path(), None, "unit").expect("first owned output");
    assert!(first.path.is_dir());
    assert!(
        first
            .path
            .starts_with(fx.path().join("build/acceptance/unit"))
    );
    let sentinel = first.path.join("keep.txt");
    fs::write(&sentinel, b"preserve").expect("sentinel");

    let second = owned_new_output(fx.path(), None, "unit").expect("second owned output");
    assert_ne!(
        first.path, second.path,
        "fresh outputs need distinct identities"
    );
    assert_eq!(
        fs::read(&sentinel).expect("previous run remains"),
        b"preserve"
    );
}

#[test]
fn owned_output_accepts_explicit_fresh_owned_directory() {
    let _case_time = crate::test_support::CaseTimer::new(
        "tooling_tests.rs::owned_output_accepts_explicit_fresh_owned_directory",
    );
    let fx = Fixture::new("owned-explicit");
    let requested = fx.path().join("build/owned-run");
    let run = owned_new_output(fx.path(), Some(&requested), "unit").expect("explicit output");
    assert_eq!(run.path, requested);
    assert!(requested.is_dir());
}

#[test]
fn owned_output_rejects_occupied_file_and_tracked_source_paths() {
    let _case_time = crate::test_support::CaseTimer::new(
        "tooling_tests.rs::owned_output_rejects_occupied_file_and_tracked_source_paths",
    );
    let fx = Fixture::new("owned-occupied");
    let occupied = fx.path().join("build/occupied");
    fs::create_dir_all(&occupied).expect("occupied dir");
    let sentinel = occupied.join("sentinel");
    fs::write(&sentinel, b"old").expect("sentinel");
    assert!(owned_new_output(fx.path(), Some(&occupied), "unit").is_err());
    assert_eq!(fs::read(&sentinel).expect("occupied content"), b"old");

    let file = fx.path().join("build/not-a-directory");
    fs::write(&file, b"file").expect("file");
    assert!(owned_new_output(fx.path(), Some(&file), "unit").is_err());

    for relative in ["src/new-output", "lisp/new-output", "test/new-output"] {
        let requested = fx.path().join(relative);
        assert!(
            owned_new_output(fx.path(), Some(&requested), "unit").is_err(),
            "a caller-selected source/data path must not become output: {relative}"
        );
    }
}

#[test]
fn owned_output_rejects_reference_and_symlink_into_reference() {
    let _case_time = crate::test_support::CaseTimer::new(
        "tooling_tests.rs::owned_output_rejects_reference_and_symlink_into_reference",
    );
    let fx = Fixture::new("owned-reference");
    let reference = fx.path().join("emacs");
    fs::create_dir_all(reference.join("src")).expect("reference");
    let under = reference.join("build-output");
    assert!(owned_new_output(fx.path(), Some(&under), "unit").is_err());

    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        let alias = fx.path().join("alias");
        symlink(&reference, &alias).expect("reference symlink");
        let through = alias.join("build-output");
        assert!(owned_new_output(fx.path(), Some(&through), "unit").is_err());
    }
}

#[test]
fn execute_step_records_separate_logs_exit_and_identity() {
    let _case_time = crate::test_support::CaseTimer::new(
        "tooling_tests.rs::execute_step_records_separate_logs_exit_and_identity",
    );
    let fx = Fixture::new("execute-ok");
    let run = owned_new_output(fx.path(), None, "unit").expect("run");
    let step = plan(
        "owned-ok",
        "sh",
        &["-c", "printf out-token; printf err-token >&2"],
        fx.path(),
    );
    let receipt = execute_step(&step, &run).expect("successful child");
    assert_eq!(receipt.id, "owned-ok");
    assert_eq!(receipt.exit_code, Some(0));
    assert!(receipt.completed);
    assert!(receipt.elapsed_ms >= 0.0);
    assert!(log_text(&receipt.stdout_log, &run).contains("out-token"));
    assert!(log_text(&receipt.stderr_log, &run).contains("err-token"));
    assert_ne!(
        receipt.stdout_log, receipt.stderr_log,
        "stdout and stderr stay separate"
    );
}

#[test]
fn execute_step_honours_cwd_and_explicit_environment() {
    let _case_time = crate::test_support::CaseTimer::new(
        "tooling_tests.rs::execute_step_honours_cwd_and_explicit_environment",
    );
    let fx = Fixture::new("execute-env");
    let cwd = fx.path().join("cwd");
    fs::create_dir_all(&cwd).expect("cwd");
    let run = owned_new_output(fx.path(), None, "unit").expect("run");
    let mut step = plan(
        "owned-env",
        "sh",
        &["-c", "printf '%s\\n%s\\n' \"$FEMACS_TEST_VALUE\" \"$PWD\""],
        &cwd,
    );
    step.env.push((
        OsString::from("FEMACS_TEST_VALUE"),
        OsString::from("owned-value"),
    ));
    let receipt = execute_step(&step, &run).expect("child");
    let log = log_text(&receipt.stdout_log, &run);
    assert!(log.contains("owned-value"));
    assert!(log.contains(&cwd.display().to_string()));
}

#[test]
fn execute_step_propagates_nonzero_child_and_retains_failure_log() {
    let _case_time = crate::test_support::CaseTimer::new(
        "tooling_tests.rs::execute_step_propagates_nonzero_child_and_retains_failure_log",
    );
    let fx = Fixture::new("execute-fail");
    let run = owned_new_output(fx.path(), None, "unit").expect("run");
    let step = plan(
        "owned-fail",
        "sh",
        &["-c", "printf failure-token >&2; exit 7"],
        fx.path(),
    );
    let result = execute_step(&step, &run);
    match result {
        Ok(receipt) => {
            assert_eq!(receipt.exit_code, Some(7));
            assert!(receipt.completed);
            assert!(log_text(&receipt.stderr_log, &run).contains("failure-token"));
        }
        Err(_) => {
            assert!(
                all_log_text(&run).contains("failure-token"),
                "failed child log must remain available after error propagation"
            );
        }
    }
}

#[test]
fn execute_step_rejects_missing_executable_without_a_receipt_lie() {
    let _case_time = crate::test_support::CaseTimer::new(
        "tooling_tests.rs::execute_step_rejects_missing_executable_without_a_receipt_lie",
    );
    let fx = Fixture::new("execute-missing");
    let run = owned_new_output(fx.path(), None, "unit").expect("run");
    let missing = fx.path().join("no-such-owned-executable");
    let step = plan("owned-missing", OsString::from(missing), &[], fx.path());
    assert!(execute_step(&step, &run).is_err());
}

#[test]
fn execute_step_reports_a_signaled_child_instead_of_hanging() {
    let _case_time = crate::test_support::CaseTimer::new(
        "tooling_tests.rs::execute_step_reports_a_signaled_child_instead_of_hanging",
    );
    let fx = Fixture::new("execute-signal");
    let run = owned_new_output(fx.path(), None, "unit").expect("run");
    let step = plan("owned-signal", "sh", &["-c", "kill -TERM $$"], fx.path());
    match execute_step(&step, &run) {
        Ok(receipt) => {
            assert!(
                receipt.exit_code != Some(0),
                "a signaled child cannot be a successful receipt"
            );
        }
        Err(_) => {}
    }
}

#[test]
fn a_second_run_does_not_mutate_or_remove_the_first_tree() {
    let _case_time = crate::test_support::CaseTimer::new(
        "tooling_tests.rs::a_second_run_does_not_mutate_or_remove_the_first_tree",
    );
    let fx = Fixture::new("execute-isolation");
    let first = owned_new_output(fx.path(), None, "unit").expect("first run");
    let sentinel = first.path.join("sentinel");
    fs::write(&sentinel, b"first").expect("sentinel");
    let second = owned_new_output(fx.path(), None, "unit").expect("second run");
    let step = plan("owned-isolated", "sh", &["-c", "printf second"], fx.path());
    let _ = execute_step(&step, &second).expect("second child");
    assert_eq!(fs::read(&sentinel).expect("first sentinel"), b"first");
}

#[test]
fn output_path_rejects_a_symlink_destination_itself() {
    let _case_time = crate::test_support::CaseTimer::new(
        "tooling_tests.rs::output_path_rejects_a_symlink_destination_itself",
    );
    let fx = Fixture::new("owned-destination-symlink");
    let target = fx.path().join("target");
    fs::create_dir_all(&target).expect("target");
    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        let output = fx.path().join("build/symlink-output");
        fs::create_dir_all(output.parent().unwrap()).expect("parent");
        symlink(&target, &output).expect("output symlink");
        assert!(owned_new_output(fx.path(), Some(&output), "unit").is_err());
        assert!(target.is_dir(), "the symlink target must not be touched");
    }
}

#[test]
fn materialized_graph_identity_changes_with_included_input_and_rejects_symlinks() {
    let _timer = crate::test_support::CaseTimer::new("materialized_graph_identity");
    let fixture = Fixture::new("graph-identities");
    let root = fixture.path().join("graph");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("_root.rs"), "include!(\"included.rs\");").unwrap();
    fs::write(root.join("included.rs"), "const VALUE:u8=1;").unwrap();
    let before = graph_input_hashes(&root).unwrap();
    assert_eq!(before.len(), 2);
    assert!(before.contains_key("included.rs"));
    fs::write(root.join("included.rs"), "const VALUE:u8=2;").unwrap();
    let after = graph_input_hashes(&root).unwrap();
    assert_eq!(before["_root.rs"], after["_root.rs"]);
    assert_ne!(before["included.rs"], after["included.rs"]);
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(root.join("included.rs"), root.join("borrowed.rs")).unwrap();
        assert!(graph_input_hashes(&root).is_err());
    }
}
