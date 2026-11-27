#![forbid(unsafe_code)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::{compute_digest, compute_mem_tree, run_supervise};

struct TempTestDir {
    path: PathBuf,
}

impl TempTestDir {
    fn new(prefix: &str) -> Self {
        let base = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("build/test-exec-unit-tmp");
        let path = base.join(format!("{prefix}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("create unit temp dir");
        Self { path }
    }
}

impl Drop for TempTestDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

#[test]
fn supervise_captures_stdout_stderr_and_nonzero_exit() {
    let args: Vec<String> = vec![
        "--".into(),
        "/bin/sh".into(),
        "-c".into(),
        "printf 'out-ok\\n'; printf 'err-ok\\n' >&2; exit 7".into(),
    ];
    let receipt = run_supervise(&args).expect("supervise");
    assert_eq!(receipt.schema, "femacs-test-exec-v1");
    assert_eq!(receipt.exit_code, Some(7));
    assert!(!receipt.timed_out);
    assert!(!receipt.memory_exceeded);
    assert!(receipt.children_reaped);
    assert_eq!(receipt.stdout, "out-ok\n");
    assert_eq!(receipt.stderr, "err-ok\n");
}

#[test]
fn supervise_timeout_kills_process_group_and_grandchild() {
    let args: Vec<String> = vec![
        "--timeout-ms".into(),
        "250".into(),
        "--poll-ms".into(),
        "25".into(),
        "--".into(),
        "/bin/sh".into(),
        "-c".into(),
        "sleep 30 & echo GRAND=$!; wait".into(),
    ];
    let receipt = run_supervise(&args).expect("supervise timeout");
    assert!(receipt.timed_out, "expected timed_out=true");
    assert!(receipt.children_reaped, "expected children_reaped=true");
    if let Some(pid_str) = receipt
        .stdout
        .lines()
        .find_map(|ln| ln.strip_prefix("GRAND="))
    {
        let out = Command::new("ps")
            .args(["-p", pid_str.trim(), "-o", "pid="])
            .output()
            .expect("ps check");
        assert!(
            String::from_utf8_lossy(&out.stdout).trim().is_empty(),
            "grandchild {pid_str} still alive"
        );
    }
}

#[test]
fn supervise_truncates_large_output_cleanly() {
    let args: Vec<String> = vec![
        "--max-output-bytes".into(),
        "128".into(),
        "--".into(),
        "/bin/sh".into(),
        "-c".into(),
        "awk 'BEGIN { for (i = 0; i < 4096; i++) printf \"x\" }'".into(),
    ];
    let receipt = run_supervise(&args).expect("supervise truncated");
    assert_eq!(receipt.exit_code, Some(0));
    assert!(receipt.stdout_truncated);
    assert_eq!(receipt.stdout.len(), 128);
}

#[test]
fn supervise_pty_captures_output() {
    let args: Vec<String> = vec![
        "--pty".into(),
        "--".into(),
        "/bin/echo".into(),
        "pty-probe-ok".into(),
    ];
    let receipt = run_supervise(&args).expect("supervise pty");
    assert_eq!(receipt.exit_code, Some(0));
    assert!(receipt.stdout.contains("pty-probe-ok"));
}

#[test]
fn mem_tree_reports_nonzero_memory_for_current_pid() {
    let pid = std::process::id().to_string();
    let receipt = compute_mem_tree(&[pid.clone()]).expect("mem-tree");
    assert_eq!(receipt.schema, "femacs-test-mem-v1");
    assert!(receipt.total_rss_bytes > 0);
    let entry = receipt.per_pid.get(&pid).expect("current pid entry");
    assert!(entry.tree_pids.contains(&std::process::id()));
}

#[test]
fn digest_is_deterministic_and_sensitive_to_tree_changes() {
    let tmp = TempTestDir::new("digest");
    let sub = tmp.path.join("sub");
    fs::create_dir_all(&sub).expect("mkdir");
    let f1 = sub.join("a.txt");
    let f2 = sub.join("b.txt");
    fs::write(&f1, b"alpha\n").expect("write a");
    fs::write(&f2, b"beta\n").expect("write b");

    let args = vec![tmp.path.display().to_string()];
    let r1 = compute_digest(&args).expect("digest 1");
    let r2 = compute_digest(&args).expect("digest 2");
    assert_eq!(r1.schema, "femacs-test-digest-v1");
    assert_eq!(r1.digest, r2.digest);
    assert_eq!(r1.file_count, 2);

    fs::write(&f2, b"beta-modified\n").expect("mutate b");
    let r3 = compute_digest(&args).expect("digest 3");
    assert_ne!(r1.digest, r3.digest);
}
