#![forbid(unsafe_code)]
//! Thin OS supervision, process-tree memory accounting, and streaming SHA-256 primitives
//! consumed by the pure Lisp test runner under `tools/testing/`.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs::{self, File};
use std::io::Read;
use std::os::unix::process::{CommandExt, ExitStatusExt};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::os_ffi;

#[cfg(test)]
#[path = "test_exec_tests.rs"]
mod test_exec_tests;

const DEFAULT_TIMEOUT_MS: u64 = 60_000;
const DEFAULT_POLL_MS: u64 = 50;
const DEFAULT_MAX_OUTPUT_BYTES: usize = 4 * 1024 * 1024;
const ESRCH: i32 = 3;

#[derive(Serialize)]
struct SuperviseReceipt {
    schema: &'static str,
    exit_code: Option<i32>,
    signal: Option<i32>,
    timed_out: bool,
    memory_exceeded: bool,
    elapsed_ms: u64,
    peak_rss_bytes: u64,
    peak_footprint_bytes: u64,
    peak_accounted_bytes: u64,
    max_tree_pids: usize,
    children_reaped: bool,
    stdout: String,
    stderr: String,
    stdout_truncated: bool,
    stderr_truncated: bool,
}

#[derive(Serialize)]
struct MemTreeEntry {
    rss_bytes: u64,
    footprint_bytes: u64,
    accounted_bytes: u64,
    tree_pids: Vec<u32>,
}

#[derive(Serialize)]
struct MemTreeReceipt {
    schema: &'static str,
    total_rss_bytes: u64,
    total_footprint_bytes: u64,
    total_accounted_bytes: u64,
    per_pid: BTreeMap<String, MemTreeEntry>,
}

#[derive(Serialize)]
struct DigestEntry {
    path: String,
    kind: &'static str,
    sha256: String,
    file_count: usize,
    bytes: u64,
}

#[derive(Serialize)]
struct DigestReceipt {
    schema: &'static str,
    digest: String,
    file_count: usize,
    total_bytes: u64,
    entries: Vec<DigestEntry>,
}

#[derive(Clone, Copy)]
struct ProcRow {
    pid: u32,
    ppid: u32,
    pgid: u32,
    rss_bytes: u64,
}

#[derive(Default)]
struct BoundedCapture {
    text: String,
    truncated: bool,
}

pub(crate) fn command(args: &[String]) -> i32 {
    match run_command(args) {
        Ok(code) => code,
        Err(msg) => {
            eprintln!("xtask test-exec: {msg}");
            2
        }
    }
}

fn run_command(args: &[String]) -> Result<i32, String> {
    let Some((sub, rest)) = args.split_first() else {
        return Err("usage: xtask test-exec <supervise|mem-tree|digest> ...".to_string());
    };
    match sub.as_str() {
        "supervise" => cmd_supervise(rest),
        "mem-tree" => cmd_mem_tree(rest),
        "digest" => cmd_digest(rest),
        other => Err(format!(
            "unknown subcommand '{other}'; expected supervise, mem-tree, or digest"
        )),
    }
}

fn next_arg<'a>(args: &'a [String], idx: &mut usize, flag: &str) -> Result<&'a str, String> {
    *idx += 1;
    args.get(*idx)
        .map(String::as_str)
        .ok_or_else(|| format!("option {flag} requires a value"))
}

fn parse_u64_arg(args: &[String], idx: &mut usize, flag: &str) -> Result<u64, String> {
    let raw = next_arg(args, idx, flag)?;
    raw.parse::<u64>()
        .map_err(|_| format!("invalid integer for {flag}: {raw}"))
}

struct SuperviseSpec {
    timeout_ms: u64,
    memory_limit_bytes: u64,
    poll_ms: u64,
    max_output_bytes: usize,
    cwd: Option<PathBuf>,
    env_set: Vec<(String, String)>,
    env_unset: Vec<String>,
    pty: bool,
    program: String,
    program_args: Vec<String>,
}

fn parse_supervise_args(args: &[String]) -> Result<SuperviseSpec, String> {
    let mut timeout_ms = DEFAULT_TIMEOUT_MS;
    let mut memory_limit_bytes = 0u64;
    let mut poll_ms = DEFAULT_POLL_MS;
    let mut max_output_bytes = DEFAULT_MAX_OUTPUT_BYTES;
    let mut cwd: Option<PathBuf> = None;
    let mut env_set: Vec<(String, String)> = Vec::new();
    let mut env_unset: Vec<String> = Vec::new();
    let mut pty = false;

    let mut i = 0usize;
    let mut prog_start: Option<usize> = None;
    while i < args.len() {
        let cur = args[i].as_str();
        match cur {
            "--" => {
                prog_start = Some(i + 1);
                break;
            }
            "--timeout-ms" => timeout_ms = parse_u64_arg(args, &mut i, "--timeout-ms")?,
            "--memory-limit-bytes" => {
                memory_limit_bytes = parse_u64_arg(args, &mut i, "--memory-limit-bytes")?;
            }
            "--poll-ms" => poll_ms = parse_u64_arg(args, &mut i, "--poll-ms")?.clamp(20, 250),
            "--max-output-bytes" => {
                let val = parse_u64_arg(args, &mut i, "--max-output-bytes")?;
                max_output_bytes = usize::try_from(val).unwrap_or(DEFAULT_MAX_OUTPUT_BYTES);
            }
            "--cwd" => cwd = Some(PathBuf::from(next_arg(args, &mut i, "--cwd")?)),
            "--env" => {
                let raw = next_arg(args, &mut i, "--env")?;
                let Some((k, v)) = raw.split_once('=') else {
                    return Err(format!("--env expects K=V, got: {raw}"));
                };
                env_set.push((k.to_string(), v.to_string()));
            }
            "--unset-env" => {
                env_unset.push(next_arg(args, &mut i, "--unset-env")?.to_string());
            }
            "--pty" => pty = true,
            other if other.starts_with('-') => {
                return Err(format!("unknown supervise option: {other}"));
            }
            _ => {
                prog_start = Some(i);
                break;
            }
        }
        i += 1;
    }

    let start = prog_start.ok_or_else(|| {
        "supervise requires -- <PROGRAM> [ARGS...]".to_string()
    })?;
    let Some(program) = args.get(start) else {
        return Err("supervise requires non-empty <PROGRAM>".to_string());
    };
    Ok(SuperviseSpec {
        timeout_ms,
        memory_limit_bytes,
        poll_ms,
        max_output_bytes,
        cwd,
        env_set,
        env_unset,
        pty,
        program: program.clone(),
        program_args: args[start + 1..].to_vec(),
    })
}

fn spawn_pipe_reader<R: Read + Send + 'static>(
    mut reader: R,
    max_bytes: usize,
) -> thread::JoinHandle<BoundedCapture> {
    thread::spawn(move || {
        let mut kept: Vec<u8> = Vec::new();
        let mut buf = [0u8; 8192];
        let mut truncated = false;
        loop {
            match reader.read(&mut buf) {
                Ok(0) | Err(_) => break,
                Ok(n) => {
                    if kept.len() < max_bytes {
                        let room = max_bytes - kept.len();
                        let take = n.min(room);
                        kept.extend_from_slice(&buf[..take]);
                        if take < n {
                            truncated = true;
                        }
                    } else {
                        truncated = true;
                    }
                }
            }
        }
        BoundedCapture {
            text: String::from_utf8_lossy(&kept).into_owned(),
            truncated,
        }
    })
}

fn snapshot_process_table() -> Vec<ProcRow> {
    let Ok(out) = Command::new("ps")
        .args(["-o", "pid=,ppid=,pgid=,rss=", "-ax"])
        .output()
    else {
        return Vec::new();
    };
    if !out.status.success() {
        return Vec::new();
    }
    let text = String::from_utf8_lossy(&out.stdout);
    let mut rows = Vec::new();
    for line in text.lines() {
        let mut parts = line.split_whitespace();
        let (Some(p), Some(pp), Some(pg), Some(rss_kb)) =
            (parts.next(), parts.next(), parts.next(), parts.next())
        else {
            continue;
        };
        let (Ok(pid), Ok(ppid), Ok(pgid), Ok(rss)) = (
            p.parse::<u32>(),
            pp.parse::<u32>(),
            pg.parse::<u32>(),
            rss_kb.parse::<u64>(),
        ) else {
            continue;
        };
        rows.push(ProcRow {
            pid,
            ppid,
            pgid,
            rss_bytes: rss.saturating_mul(1024),
        });
    }
    rows
}

fn collect_tree_closure(rows: &[ProcRow], root_pid: u32) -> BTreeSet<u32> {
    let mut closure = BTreeSet::new();
    closure.insert(root_pid);
    for row in rows {
        if row.pgid == root_pid {
            closure.insert(row.pid);
        }
    }
    let mut queue: VecDeque<u32> = closure.iter().copied().collect();
    while let Some(cur) = queue.pop_front() {
        for row in rows {
            if row.ppid == cur && closure.insert(row.pid) {
                queue.push_back(row.pid);
            }
        }
    }
    closure
}

fn sample_pid_memories(
    rows: &[ProcRow],
    roots: &[u32],
) -> (u64, u64, u64, BTreeMap<String, MemTreeEntry>) {
    let ps_rss_by_pid: BTreeMap<u32, u64> = rows.iter().map(|r| (r.pid, r.rss_bytes)).collect();
    let mut all_pids = BTreeSet::new();
    let mut per_pid_closures: Vec<(u32, BTreeSet<u32>)> = Vec::with_capacity(roots.len());
    for &root in roots {
        let closure = collect_tree_closure(rows, root);
        all_pids.extend(closure.iter().copied());
        per_pid_closures.push((root, closure));
    }

    let mut sampled: BTreeMap<u32, (u64, u64, u64)> = BTreeMap::new();
    for pid in all_pids {
        let fallback_rss = ps_rss_by_pid.get(&pid).copied().unwrap_or(0);
        let (rss, fp) = pid_memory_or_fallback(pid, fallback_rss);
        sampled.insert(pid, (rss, fp, rss.max(fp)));
    }

    let mut total_rss = 0u64;
    let mut total_fp = 0u64;
    let mut total_acc = 0u64;
    for (rss, fp, acc) in sampled.values() {
        total_rss = total_rss.saturating_add(*rss);
        total_fp = total_fp.saturating_add(*fp);
        total_acc = total_acc.saturating_add(*acc);
    }

    let mut per_pid = BTreeMap::new();
    for (root, closure) in per_pid_closures {
        let mut r_sum = 0u64;
        let mut f_sum = 0u64;
        let mut a_sum = 0u64;
        for pid in &closure {
            if let Some(&(r, f, a)) = sampled.get(pid) {
                r_sum = r_sum.saturating_add(r);
                f_sum = f_sum.saturating_add(f);
                a_sum = a_sum.saturating_add(a);
            }
        }
        per_pid.insert(
            root.to_string(),
            MemTreeEntry {
                rss_bytes: r_sum,
                footprint_bytes: f_sum,
                accounted_bytes: a_sum,
                tree_pids: closure.into_iter().collect(),
            },
        );
    }

    (total_rss, total_fp, total_acc, per_pid)
}

fn pid_memory_or_fallback(pid: u32, ps_rss_bytes: u64) -> (u64, u64) {
    let pid_i32 = i32::try_from(pid).unwrap_or(0);
    if let Some((rss, fp)) = os_ffi::pid_rusage_memory(pid_i32) {
        (rss.max(ps_rss_bytes), fp)
    } else {
        (ps_rss_bytes, ps_rss_bytes)
    }
}

fn terminate_process_group(child: &mut Child, pgid: i32) {
    let _ = os_ffi::signal_process_group(pgid, os_ffi::SIGTERM);
    let grace_deadline = Instant::now() + Duration::from_millis(200);
    while Instant::now() < grace_deadline {
        match child.try_wait() {
            Ok(Some(_)) => return,
            Ok(None) => thread::sleep(Duration::from_millis(20)),
            Err(_) => break,
        }
    }
    let _ = os_ffi::signal_process_group(pgid, os_ffi::SIGKILL);
    let _ = child.kill();
}

fn reap_remaining_group(root_pid: u32) -> bool {
    let self_pid = std::process::id();
    if root_pid <= 1 || root_pid == self_pid {
        return true;
    }
    let pgid = match i32::try_from(root_pid) {
        Ok(p) if p > 1 => p,
        _ => return false,
    };

    match os_ffi::signal_process_group(pgid, os_ffi::SIGTERM) {
        Err(ESRCH) => return true,
        Ok(()) | Err(_) => {}
    }

    let deadline = Instant::now() + Duration::from_millis(200);
    while Instant::now() < deadline {
        let rows = snapshot_process_table();
        let survivors = collect_tree_closure(&rows, root_pid)
            .into_iter()
            .any(|pid| pid != self_pid && pid != root_pid);
        if !survivors {
            return true;
        }
        thread::sleep(Duration::from_millis(20));
    }

    match os_ffi::signal_process_group(pgid, os_ffi::SIGKILL) {
        Err(ESRCH) => return true,
        Ok(()) | Err(_) => {}
    }
    let kill_deadline = Instant::now() + Duration::from_millis(250);
    while Instant::now() < kill_deadline {
        let rows = snapshot_process_table();
        let survivors = collect_tree_closure(&rows, root_pid)
            .into_iter()
            .any(|pid| pid != self_pid && pid != root_pid);
        if !survivors {
            return true;
        }
        thread::sleep(Duration::from_millis(20));
    }
    false
}

fn build_child_command(spec: &SuperviseSpec) -> Command {
    let mut cmd = if spec.pty && cfg!(target_os = "macos") {
        let mut c = Command::new("/usr/bin/script");
        c.args(["-q", "/dev/null", &spec.program]);
        c.args(&spec.program_args);
        c
    } else {
        let mut c = Command::new(&spec.program);
        c.args(&spec.program_args);
        c
    };
    if let Some(dir) = &spec.cwd {
        cmd.current_dir(dir);
    }
    for k in &spec.env_unset {
        cmd.env_remove(k);
    }
    for (k, v) in &spec.env_set {
        cmd.env(k, v);
    }
    cmd.stdin(Stdio::null());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    cmd.process_group(0);
    cmd
}

fn run_supervise(args: &[String]) -> Result<SuperviseReceipt, String> {
    let spec = parse_supervise_args(args)?;
    os_ffi::install_termination_handlers();
    let mut cmd = build_child_command(&spec);
    let started = Instant::now();
    let mut child = cmd
        .spawn()
        .map_err(|e| format!("failed to spawn {}: {e}", spec.program))?;
    let root_pid = child.id();
    let pgid = i32::try_from(root_pid).map_err(|_| "pid overflow".to_string())?;

    let stdout_handle = spawn_pipe_reader(
        child.stdout.take().ok_or("missing stdout pipe")?,
        spec.max_output_bytes,
    );
    let stderr_handle = spawn_pipe_reader(
        child.stderr.take().ok_or("missing stderr pipe")?,
        spec.max_output_bytes,
    );

    let mut timed_out = false;
    let mut memory_exceeded = false;
    let mut peak_rss_bytes = 0u64;
    let mut peak_footprint_bytes = 0u64;
    let mut peak_accounted_bytes = 0u64;
    let mut max_tree_pids = 1usize;

    let status = loop {
        if os_ffi::termination_requested() {
            terminate_process_group(&mut child, pgid);
            let _ = child.wait();
            let _ = reap_remaining_group(root_pid);
            let _ = stdout_handle.join();
            let _ = stderr_handle.join();
            return Err("supervise terminated by signal; child process group reaped".to_string());
        }
        let rows = snapshot_process_table();
        let (_, _, _, per_pid) = sample_pid_memories(&rows, &[root_pid]);
        if let Some(entry) = per_pid.get(&root_pid.to_string()) {
            peak_rss_bytes = peak_rss_bytes.max(entry.rss_bytes);
            peak_footprint_bytes = peak_footprint_bytes.max(entry.footprint_bytes);
            peak_accounted_bytes = peak_accounted_bytes.max(entry.accounted_bytes);
            max_tree_pids = max_tree_pids.max(entry.tree_pids.len());
            if spec.memory_limit_bytes > 0 && entry.accounted_bytes > spec.memory_limit_bytes {
                memory_exceeded = true;
                terminate_process_group(&mut child, pgid);
                break child.wait().map_err(|e| e.to_string())?;
            }
        }

        if let Some(done) = child.try_wait().map_err(|e| e.to_string())? {
            break done;
        }

        let elapsed_now = started.elapsed().as_millis() as u64;
        if spec.timeout_ms > 0 && elapsed_now >= spec.timeout_ms {
            timed_out = true;
            terminate_process_group(&mut child, pgid);
            break child.wait().map_err(|e| e.to_string())?;
        }

        thread::sleep(Duration::from_millis(spec.poll_ms));
    };

    let elapsed_ms = started.elapsed().as_millis() as u64;
    let children_reaped = reap_remaining_group(root_pid);

    let stdout_cap = stdout_handle.join().unwrap_or_default();
    let stderr_cap = stderr_handle.join().unwrap_or_default();

    Ok(SuperviseReceipt {
        schema: "femacs-test-exec-v1",
        exit_code: status.code(),
        signal: status.signal(),
        timed_out,
        memory_exceeded,
        elapsed_ms,
        peak_rss_bytes,
        peak_footprint_bytes,
        peak_accounted_bytes,
        max_tree_pids,
        children_reaped,
        stdout: stdout_cap.text,
        stderr: stderr_cap.text,
        stdout_truncated: stdout_cap.truncated,
        stderr_truncated: stderr_cap.truncated,
    })
}

fn cmd_supervise(args: &[String]) -> Result<i32, String> {
    let receipt = run_supervise(args)?;
    let encoded = serde_json::to_string(&receipt).map_err(|e| e.to_string())?;
    println!("{encoded}");
    Ok(0)
}

fn compute_mem_tree(args: &[String]) -> Result<MemTreeReceipt, String> {
    if args.is_empty() {
        return Err("mem-tree requires at least one <PID>".to_string());
    }
    let mut roots = Vec::with_capacity(args.len());
    for raw in args {
        let pid = raw
            .parse::<u32>()
            .map_err(|_| format!("invalid PID: {raw}"))?;
        roots.push(pid);
    }
    let rows = snapshot_process_table();
    let (total_rss, total_fp, total_acc, per_pid) = sample_pid_memories(&rows, &roots);
    Ok(MemTreeReceipt {
        schema: "femacs-test-mem-v1",
        total_rss_bytes: total_rss,
        total_footprint_bytes: total_fp,
        total_accounted_bytes: total_acc,
        per_pid,
    })
}

fn cmd_mem_tree(args: &[String]) -> Result<i32, String> {
    let receipt = compute_mem_tree(args)?;
    let encoded = serde_json::to_string(&receipt).map_err(|e| e.to_string())?;
    println!("{encoded}");
    Ok(0)
}

fn hex_string(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        use std::fmt::Write as _;
        let _ = write!(s, "{b:02x}");
    }
    s
}

fn hash_file_streaming(path: &Path, into: &mut Sha256) -> Result<(String, u64), String> {
    let mut f = File::open(path).map_err(|e| format!("open {}: {e}", path.display()))?;
    let mut file_hasher = Sha256::new();
    let mut buf = [0u8; 64 * 1024];
    let mut total = 0u64;
    loop {
        let n = f
            .read(&mut buf)
            .map_err(|e| format!("read {}: {e}", path.display()))?;
        if n == 0 {
            break;
        }
        file_hasher.update(&buf[..n]);
        into.update(&buf[..n]);
        total = total.saturating_add(n as u64);
    }
    Ok((hex_string(&file_hasher.finalize()), total))
}

fn walk_dir_sorted(root: &Path, cur: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    let mut children = Vec::new();
    for entry in fs::read_dir(cur).map_err(|e| format!("readdir {}: {e}", cur.display()))? {
        let entry = entry.map_err(|e| e.to_string())?;
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        if name_str == ".git" || name_str == "__pycache__" {
            continue;
        }
        children.push(entry.path());
    }
    children.sort();
    for p in children {
        let meta = fs::symlink_metadata(&p).map_err(|e| format!("stat {}: {e}", p.display()))?;
        if meta.file_type().is_dir() {
            walk_dir_sorted(root, &p, out)?;
        } else {
            out.push(p);
        }
    }
    Ok(())
}

fn compute_digest(args: &[String]) -> Result<DigestReceipt, String> {
    if args.is_empty() {
        return Err("digest requires at least one <PATH>".to_string());
    }
    let mut combined = Sha256::new();
    let mut entries = Vec::with_capacity(args.len());
    let mut total_files = 0usize;
    let mut total_bytes = 0u64;

    for raw in args {
        let p = Path::new(raw);
        let meta = fs::symlink_metadata(p).map_err(|e| format!("stat {raw}: {e}"))?;
        if meta.file_type().is_file() {
            let mut item_hasher = Sha256::new();
            let (file_hex, bytes) = hash_file_streaming(p, &mut item_hasher)?;
            combined.update(b"F\0");
            combined.update(raw.as_bytes());
            combined.update(b"\0");
            combined.update(bytes.to_le_bytes());
            combined.update(file_hex.as_bytes());
            total_files += 1;
            total_bytes = total_bytes.saturating_add(bytes);
            entries.push(DigestEntry {
                path: raw.clone(),
                kind: "file",
                sha256: file_hex,
                file_count: 1,
                bytes,
            });
        } else if meta.file_type().is_dir() {
            let mut files = Vec::new();
            walk_dir_sorted(p, p, &mut files)?;
            let mut dir_hasher = Sha256::new();
            let mut dir_bytes = 0u64;
            let mut dir_files = 0usize;
            for fp in files {
                let rel = fp
                    .strip_prefix(p)
                    .unwrap_or(&fp)
                    .to_string_lossy()
                    .replace('\\', "/");
                let fmeta = fs::symlink_metadata(&fp)
                    .map_err(|e| format!("stat {}: {e}", fp.display()))?;
                if fmeta.file_type().is_symlink() {
                    let target = fs::read_link(&fp)
                        .map_err(|e| format!("readlink {}: {e}", fp.display()))?;
                    dir_hasher.update(b"L\0");
                    dir_hasher.update(rel.as_bytes());
                    dir_hasher.update(b"\0");
                    dir_hasher.update(target.to_string_lossy().as_bytes());
                    dir_hasher.update(b"\0");
                    dir_files += 1;
                } else if fmeta.file_type().is_file() {
                    let mut scratch = Sha256::new();
                    let (fhex, bytes) = hash_file_streaming(&fp, &mut scratch)?;
                    dir_hasher.update(b"F\0");
                    dir_hasher.update(rel.as_bytes());
                    dir_hasher.update(b"\0");
                    dir_hasher.update(bytes.to_le_bytes());
                    dir_hasher.update(fhex.as_bytes());
                    dir_files += 1;
                    dir_bytes = dir_bytes.saturating_add(bytes);
                }
            }
            let dir_hex = hex_string(&dir_hasher.finalize());
            combined.update(b"D\0");
            combined.update(raw.as_bytes());
            combined.update(b"\0");
            combined.update((dir_files as u64).to_le_bytes());
            combined.update(dir_bytes.to_le_bytes());
            combined.update(dir_hex.as_bytes());
            total_files += dir_files;
            total_bytes = total_bytes.saturating_add(dir_bytes);
            entries.push(DigestEntry {
                path: raw.clone(),
                kind: "dir",
                sha256: dir_hex,
                file_count: dir_files,
                bytes: dir_bytes,
            });
        } else {
            return Err(format!("unsupported path type for digest: {raw}"));
        }
    }

    Ok(DigestReceipt {
        schema: "femacs-test-digest-v1",
        digest: hex_string(&combined.finalize()),
        file_count: total_files,
        total_bytes,
        entries,
    })
}

fn cmd_digest(args: &[String]) -> Result<i32, String> {
    let receipt = compute_digest(args)?;
    let encoded = serde_json::to_string(&receipt).map_err(|e| e.to_string())?;
    println!("{encoded}");
    Ok(0)
}
