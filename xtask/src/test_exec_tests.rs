#![forbid(unsafe_code)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use super::{compute_digest, compute_mem_tree, finalize_children_reaped, run_supervise, spawn_pipe_drain};

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

fn assert_pid_dead(pid_str: &str) {
    let trimmed = pid_str.trim();
    assert!(!trimmed.is_empty(), "expected non-empty PID");
    let out = Command::new("ps")
        .args(["-p", trimmed, "-o", "pid="])
        .output()
        .expect("ps check");
    assert!(
        String::from_utf8_lossy(&out.stdout).trim().is_empty(),
        "descendant {trimmed} still alive"
    );
}

#[test]
fn supervise_pty_timeout_reaps_independent_pgid_descendant() {
    let args: Vec<String> = vec![
        "--pty".into(),
        "--timeout-ms".into(),
        "400".into(),
        "--poll-ms".into(),
        "25".into(),
        "--".into(),
        "/bin/sh".into(),
        "-c".into(),
        "echo CHILD_PID=$$; sleep 30 & echo GRAND_PID=$!; wait".into(),
    ];
    let receipt = run_supervise(&args).expect("supervise pty timeout");
    assert!(receipt.timed_out, "expected timed_out=true");
    assert!(receipt.output_drained, "expected output_drained=true");
    assert!(receipt.children_reaped, "expected children_reaped=true");
    let clean_out = receipt.stdout.replace('\r', "");
    let child_pid = clean_out
        .lines()
        .find_map(|ln| ln.split("CHILD_PID=").nth(1))
        .map(str::trim)
        .expect("CHILD_PID line");
    let grand_pid = clean_out
        .lines()
        .find_map(|ln| ln.split("GRAND_PID=").nth(1))
        .map(str::trim)
        .expect("GRAND_PID line");
    assert_pid_dead(child_pid);
    assert_pid_dead(grand_pid);
}

#[test]
fn supervise_reaps_independent_pgid_descendant_holding_stdout_fd() {
    let script = concat!(
        "perl -MPOSIX -e '$|=1; my $p = fork(); die unless defined $p; ",
        "if ($p == 0) { POSIX::setsid() or die; exec \"sleep\", \"30\"; } ",
        "print \"DETACHED_PID=$p\\n\"; select(undef,undef,undef,0.2);'"
    );
    let args: Vec<String> = vec![
        "--timeout-ms".into(),
        "5000".into(),
        "--poll-ms".into(),
        "25".into(),
        "--".into(),
        "/bin/sh".into(),
        "-c".into(),
        script.into(),
    ];
    let started = Instant::now();
    let receipt = run_supervise(&args).expect("supervise detached descendant");
    let elapsed = started.elapsed();
    assert!(
        elapsed < Duration::from_secs(3),
        "supervise blocked too long ({elapsed:?}) on detached stdout holder"
    );
    assert!(!receipt.timed_out, "must finish cleanly without timeout");
    assert!(receipt.output_drained, "expected output_drained=true");
    assert!(receipt.children_reaped, "expected children_reaped=true");
    let detached_pid = receipt
        .stdout
        .lines()
        .find_map(|ln| ln.trim().strip_prefix("DETACHED_PID="))
        .expect("DETACHED_PID line");
    assert_pid_dead(detached_pid);
}

#[test]
fn pipe_drain_times_out_when_writer_stays_open() {
    let mut child = Command::new("/bin/sh")
        .args(["-c", "printf 'partial\\n'; exec sleep 30"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn open-pipe writer");
    let drain = spawn_pipe_drain(child.stdout.take().expect("stdout"), 4096);
    let wait_start = Instant::now();
    while wait_start.elapsed() < Duration::from_secs(1) {
        let ready = drain
            .snapshot
            .lock()
            .map(|g| g.text.contains("partial"))
            .unwrap_or(false);
        if ready {
            break;
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    let drain_start = Instant::now();
    let (cap, drained) = drain.drain_with_timeout(Duration::from_millis(120));
    let drain_elapsed = drain_start.elapsed();
    let _ = child.kill();
    let _ = child.wait();
    assert!(!drained, "writer still holds stdout open");
    assert!(
        drain_elapsed < Duration::from_secs(1),
        "drain_with_timeout blocked too long: {drain_elapsed:?}"
    );
    assert!(cap.text.contains("partial"), "captured={:?}", cap.text);
}

#[test]
fn finalize_children_reaped_requires_reap_and_drained_output() {
    assert!(finalize_children_reaped(true, true, true));
    assert!(!finalize_children_reaped(false, true, true));
    assert!(!finalize_children_reaped(true, false, true));
    assert!(!finalize_children_reaped(true, true, false));
}
