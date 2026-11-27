#![forbid(unsafe_code)]
//! Thin OS supervision, process-tree memory accounting, and streaming SHA-256 primitives
//! consumed by the pure Lisp test runner under `tools/testing/`.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs::{self, File};
use std::io::Read;
use std::os::unix::process::{CommandExt, ExitStatusExt};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::{Duration, Instant};

use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::os_ffi;

#[cfg(test)]
#[path = "exec_tests.rs"]
mod exec_tests;

const DEFAULT_TIMEOUT_MS: u64 = 60_000;
const DEFAULT_POLL_MS: u64 = 50;
const DEFAULT_MAX_OUTPUT_BYTES: usize = 4 * 1024 * 1024;
const PIPE_DRAIN_TIMEOUT_MS: u64 = 500;
const SIGNAL_DRAIN_TIMEOUT_MS: u64 = 300;

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
    output_drained: bool,
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

#[derive(Clone)]
struct ProcRow {
    pid: u32,
    ppid: u32,
    pgid: u32,
    rss_bytes: u64,
    lstart: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ProcIdentity {
    start_abstime: Option<u64>,
    lstart: String,
}

#[derive(Default)]
struct BoundedCapture {
    text: String,
    truncated: bool,
}

struct PipeDrain {
    done_rx: mpsc::Receiver<()>,
    snapshot: Arc<Mutex<BoundedCapture>>,
}

impl PipeDrain {
    fn drain_with_timeout(self, timeout: Duration) -> (BoundedCapture, bool) {
        let drained = match self.done_rx.recv_timeout(timeout) {
            Ok(()) | Err(mpsc::RecvTimeoutError::Disconnected) => true,
            Err(mpsc::RecvTimeoutError::Timeout) => false,
        };
        let guard = self.snapshot.lock().unwrap_or_else(|e| e.into_inner());
        let cap = BoundedCapture {
            text: guard.text.clone(),
            truncated: guard.truncated,
        };
        (cap, drained)
    }
}

fn proc_identity_for_row(row: &ProcRow) -> ProcIdentity {
    let pid_i32 = i32::try_from(row.pid).unwrap_or(0);
    let start_abstime = os_ffi::pid_rusage_info(pid_i32)
        .map(|(_, _, abstime)| abstime)
        .filter(|&t| t > 0);
    ProcIdentity {
        start_abstime,
        lstart: row.lstart.clone(),
    }
}

fn same_identity(recorded: &ProcIdentity, live: &ProcIdentity) -> bool {
    match (recorded.start_abstime, live.start_abstime) {
        (Some(a), Some(b)) => a == b,
        _ => !recorded.lstart.is_empty() && recorded.lstart == live.lstart,
    }
}

fn finalize_children_reaped(reaped: bool, stdout_drained: bool, stderr_drained: bool) -> bool {
    reaped && stdout_drained && stderr_drained
}

/// Dispatch `exec` / `test-exec` CLI subcommands (`supervise`, `mem-tree`, `digest`).
pub fn run_cli(args: &[String]) -> i32 {
    match run_command(args) {
        Ok(code) => code,
        Err(msg) => {
            eprintln!("lisp-core exec: {msg}");
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

fn spawn_pipe_drain<R: Read + Send + 'static>(
    mut reader: R,
    max_bytes: usize,
) -> PipeDrain {
    let snapshot = Arc::new(Mutex::new(BoundedCapture::default()));
    let snapshot_ref = Arc::clone(&snapshot);
    let (done_tx, done_rx) = mpsc::channel();
    thread::spawn(move || {
        let mut raw_kept: Vec<u8> = Vec::new();
        let mut buf = [0u8; 8192];
        loop {
            match reader.read(&mut buf) {
                Ok(0) | Err(_) => break,
                Ok(n) => {
                    let mut guard = snapshot_ref.lock().unwrap_or_else(|e| e.into_inner());
                    if raw_kept.len() < max_bytes {
                        let room = max_bytes - raw_kept.len();
                        let take = n.min(room);
                        raw_kept.extend_from_slice(&buf[..take]);
                        guard.text = String::from_utf8_lossy(&raw_kept).into_owned();
                        if take < n {
                            guard.truncated = true;
                        }
                    } else {
                        guard.truncated = true;
                    }
                }
            }
        }
        let _ = done_tx.send(());
    });
    PipeDrain { done_rx, snapshot }
}

fn snapshot_process_table() -> Result<Vec<ProcRow>, String> {
    let out = Command::new("ps")
        .args(["-o", "pid=,ppid=,pgid=,rss=,lstart=", "-ax"])
        .output()
        .map_err(|e| format!("failed to run ps: {e}"))?;
    if !out.status.success() {
        return Err(format!("ps exited with status {}", out.status));
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
        let lstart = parts.collect::<Vec<_>>().join(" ");
        rows.push(ProcRow {
            pid,
            ppid,
            pgid,
            rss_bytes: rss.saturating_mul(1024),
            lstart,
        });
    }
    Ok(rows)
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

struct TrackedTree {
    root_pid: u32,
    root_retired: bool,
    self_pid: u32,
    self_pgid: u32,
    launch_start_abstime: Option<u64>,
    procs: BTreeMap<u32, ProcIdentity>,
}

impl TrackedTree {
    fn new_with_rows(
        root_pid: u32,
        self_pid: u32,
        self_pgid: u32,
        rows: Option<&[ProcRow]>,
    ) -> Self {
        let mut tree = Self {
            root_pid,
            root_retired: false,
            self_pid,
            self_pgid,
            launch_start_abstime: None,
            procs: BTreeMap::new(),
        };
        if let Some(initial_rows) = rows {
            tree.launch_start_abstime = initial_rows
                .iter()
                .find(|r| r.pid == root_pid)
                .and_then(|r| proc_identity_for_row(r).start_abstime);
            let _ = tree.refresh(initial_rows);
        }
        tree
    }

    fn new(root_pid: u32) -> Self {
        let rows = snapshot_process_table().ok();
        Self::new_with_rows(
            root_pid,
            std::process::id(),
            os_ffi::current_pgid(),
            rows.as_deref(),
        )
    }

    fn is_safe_pgid(&self, pgid: u32) -> bool {
        pgid > 1 && pgid != self.self_pgid && pgid != self.self_pid
    }

    fn drop_root_after_wait(&mut self) {
        self.root_retired = true;
        self.procs.remove(&self.root_pid);
    }

    fn refresh(&mut self, rows: &[ProcRow]) -> Vec<ProcRow> {
        self.refresh_with_ident(rows, proc_identity_for_row)
    }

    fn refresh_with_ident<F>(&mut self, rows: &[ProcRow], mut ident_fn: F) -> Vec<ProcRow>
    where
        F: FnMut(&ProcRow) -> ProcIdentity,
    {
        let mut by_pid: BTreeMap<u32, &ProcRow> = BTreeMap::new();
        for row in rows {
            by_pid.insert(row.pid, row);
        }

        let mut alive_pids = BTreeSet::new();
        let mut recycled_pids = Vec::new();
        let mut active_pgids = BTreeSet::new();

        for (&pid, recorded) in &self.procs {
            if let Some(&row) = by_pid.get(&pid) {
                let live_ident = ident_fn(row);
                if same_identity(recorded, &live_ident) {
                    alive_pids.insert(pid);
                    if self.is_safe_pgid(row.pgid) {
                        active_pgids.insert(row.pgid);
                    }
                } else {
                    recycled_pids.push(pid);
                }
            }
        }
        for pid in recycled_pids {
            self.procs.remove(&pid);
        }

        if !self.root_retired {
            if let Some(&root_row) = by_pid.get(&self.root_pid) {
                if !self.procs.contains_key(&self.root_pid) {
                    let ident = ident_fn(root_row);
                    if self.launch_start_abstime.is_none() {
                        self.launch_start_abstime = ident.start_abstime;
                    }
                    let ok_time = match (self.launch_start_abstime, ident.start_abstime) {
                        (Some(base), Some(cur)) => cur >= base,
                        _ => true,
                    };
                    if ok_time {
                        self.procs.insert(self.root_pid, ident);
                        alive_pids.insert(self.root_pid);
                        if self.is_safe_pgid(root_row.pgid) {
                            active_pgids.insert(root_row.pgid);
                        }
                    }
                }
            }
        }

        for row in rows {
            if row.pid <= 1 || row.pid == self.self_pid || (self.root_retired && row.pid == self.root_pid) {
                continue;
            }
            if active_pgids.contains(&row.pgid) && !self.procs.contains_key(&row.pid) {
                let ident = ident_fn(row);
                let ok_time = match (self.launch_start_abstime, ident.start_abstime) {
                    (Some(base), Some(cur)) => cur >= base,
                    _ => true,
                };
                if ok_time {
                    self.procs.insert(row.pid, ident);
                    alive_pids.insert(row.pid);
                }
            }
        }

        let mut queue: VecDeque<u32> = alive_pids.iter().copied().collect();
        while let Some(cur) = queue.pop_front() {
            for row in rows {
                if row.pid <= 1
                    || row.pid == self.self_pid
                    || (self.root_retired && row.pid == self.root_pid)
                    || row.ppid != cur
                {
                    continue;
                }
                if alive_pids.insert(row.pid) {
                    let ident = ident_fn(row);
                    self.procs.entry(row.pid).or_insert(ident);
                    if self.is_safe_pgid(row.pgid) {
                        active_pgids.insert(row.pgid);
                    }
                    queue.push_back(row.pid);
                }
            }
        }

        alive_pids
            .into_iter()
            .filter_map(|pid| by_pid.get(&pid).map(|&r| r.clone()))
            .collect()
    }
}

fn sample_tracked_memory(alive: &[ProcRow]) -> (u64, u64, u64, usize) {
    let mut total_rss = 0u64;
    let mut total_fp = 0u64;
    let mut total_acc = 0u64;
    for row in alive {
        let (rss, fp) = pid_memory_or_fallback(row.pid, row.rss_bytes);
        total_rss = total_rss.saturating_add(rss);
        total_fp = total_fp.saturating_add(fp);
        total_acc = total_acc.saturating_add(rss.max(fp));
    }
    (total_rss, total_fp, total_acc, alive.len())
}

fn sample_pid_memories(
    rows: &[ProcRow],
    roots: &[u32],
) -> (u64, u64, u64, BTreeMap<String, MemTreeEntry>) {
    let ps_rss_by_pid: BTreeMap<u32, u64> = rows.iter().map(|r| (r.pid, r.rss_bytes)).collect();
    let mut all_pids = BTreeSet::new();
    let mut per_pid_closures: Vec<(u32, BTreeSet<u32>)> = Vec::with_capacity(roots.len());
    for &root in roots {
        if !ps_rss_by_pid.contains_key(&root) {
            continue;
        }
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

fn verified_signal_targets(tree: &TrackedTree, survivors: &[ProcRow]) -> (BTreeSet<i32>, Vec<i32>) {
    let mut pgids = BTreeSet::new();
    let mut pids = Vec::with_capacity(survivors.len());
    for row in survivors {
        if tree.is_safe_pgid(row.pgid) {
            if let Ok(pgid) = i32::try_from(row.pgid) {
                pgids.insert(pgid);
            }
        }
        if row.pid > 1 && row.pid != tree.self_pid {
            if let Ok(pid) = i32::try_from(row.pid) {
                pids.push(pid);
            }
        }
    }
    (pgids, pids)
}

fn signal_tree_snapshot(tree: &TrackedTree, survivors: &[ProcRow], sig: i32) {
    let (pgids, pids) = verified_signal_targets(tree, survivors);
    for pgid in pgids {
        let _ = os_ffi::signal_process_group(pgid, sig);
    }
    for pid in pids {
        let _ = os_ffi::signal_process(pid, sig);
    }
}

fn terminate_tracked_tree(child: &mut Child, tree: &mut TrackedTree) {
    if let Ok(rows) = snapshot_process_table() {
        let initial = tree.refresh(&rows);
        signal_tree_snapshot(tree, &initial, os_ffi::SIGTERM);
    }
    if !tree.root_retired {
        if let Ok(root_pid) = i32::try_from(tree.root_pid) {
            let _ = os_ffi::signal_process(root_pid, os_ffi::SIGTERM);
        }
    }
    let grace_deadline = Instant::now() + Duration::from_millis(200);
    let mut child_done = false;
    while Instant::now() < grace_deadline {
        if !child_done {
            match child.try_wait() {
                Ok(Some(_)) => {
                    child_done = true;
                    tree.drop_root_after_wait();
                }
                Err(_) => child_done = true,
                Ok(None) => {}
            }
        }
        if let Ok(rows) = snapshot_process_table() {
            let rem = tree.refresh(&rows);
            let non_root_alive = rem.iter().any(|r| r.pid != tree.root_pid);
            if child_done && !non_root_alive {
                return;
            }
        }
        thread::sleep(Duration::from_millis(20));
    }
    if let Ok(rows) = snapshot_process_table() {
        let survivors = tree.refresh(&rows);
        signal_tree_snapshot(tree, &survivors, os_ffi::SIGKILL);
    }
    if !child_done {
        let _ = child.kill();
    }
}

fn reap_tracked_tree_with_ident<S, I, F>(
    tree: &mut TrackedTree,
    mut snap_fn: S,
    mut ident_fn: I,
    mut signal_fn: F,
) -> bool
where
    S: FnMut() -> Result<Vec<ProcRow>, String>,
    I: FnMut(&ProcRow) -> ProcIdentity,
    F: FnMut(&TrackedTree, &[ProcRow], i32),
{
    let mut signaled_term = false;

    let term_deadline = Instant::now() + Duration::from_millis(200);
    loop {
        if let Ok(rows) = snap_fn() {
            let survivors = tree.refresh_with_ident(&rows, &mut ident_fn);
            if survivors.is_empty() {
                return true;
            }
            if !signaled_term {
                signal_fn(tree, &survivors, os_ffi::SIGTERM);
                signaled_term = true;
            }
        }
        if Instant::now() >= term_deadline {
            break;
        }
        thread::sleep(Duration::from_millis(20));
    }

    let mut signaled_kill = false;
    let kill_deadline = Instant::now() + Duration::from_millis(300);
    loop {
        if let Ok(rows) = snap_fn() {
            let survivors = tree.refresh_with_ident(&rows, &mut ident_fn);
            if survivors.is_empty() {
                return true;
            }
            if !signaled_kill {
                signal_fn(tree, &survivors, os_ffi::SIGKILL);
                signaled_kill = true;
            }
        }
        if Instant::now() >= kill_deadline {
            break;
        }
        thread::sleep(Duration::from_millis(20));
    }
    match snap_fn() {
        Ok(rows) => tree.refresh_with_ident(&rows, &mut ident_fn).is_empty(),
        Err(_) => false,
    }
}

fn reap_tracked_tree(tree: &mut TrackedTree) -> bool {
    reap_tracked_tree_with_ident(
        tree,
        snapshot_process_table,
        proc_identity_for_row,
        signal_tree_snapshot,
    )
}

fn shell_quote_posix(arg: &str) -> String {
    let mut out = String::with_capacity(arg.len() + 2);
    out.push('\'');
    for ch in arg.chars() {
        if ch == '\'' {
            out.push_str("'\\''");
        } else {
            out.push(ch);
        }
    }
    out.push('\'');
    out
}

fn build_child_command(spec: &SuperviseSpec) -> Command {
    let mut cmd = if spec.pty && cfg!(target_os = "macos") {
        let mut c = Command::new("/usr/bin/script");
        c.args(["-q", "/dev/null", &spec.program]);
        c.args(&spec.program_args);
        c
    } else if spec.pty && cfg!(target_os = "linux") {
        let mut cmd_parts = Vec::with_capacity(1 + spec.program_args.len());
        cmd_parts.push(shell_quote_posix(&spec.program));
        for arg in &spec.program_args {
            cmd_parts.push(shell_quote_posix(arg));
        }
        let mut c = Command::new("script");
        c.args(["-qefc", &cmd_parts.join(" "), "/dev/null"]);
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
    let mut tree = TrackedTree::new(root_pid);

    let stdout_drain = spawn_pipe_drain(
        child.stdout.take().ok_or("missing stdout pipe")?,
        spec.max_output_bytes,
    );
    let stderr_drain = spawn_pipe_drain(
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
            terminate_tracked_tree(&mut child, &mut tree);
            let _ = child.wait();
            tree.drop_root_after_wait();
            let reaped = reap_tracked_tree(&mut tree);
            let (_, out_ok) = stdout_drain.drain_with_timeout(Duration::from_millis(SIGNAL_DRAIN_TIMEOUT_MS));
            let (_, err_ok) = stderr_drain.drain_with_timeout(Duration::from_millis(SIGNAL_DRAIN_TIMEOUT_MS));
            return Err(format!(
                "supervise terminated by signal; reaped={reaped} drained={}",
                out_ok && err_ok
            ));
        }

        if let Ok(rows) = snapshot_process_table() {
            let alive = tree.refresh(&rows);
            if !alive.is_empty() {
                let (rss, fp, acc, count) = sample_tracked_memory(&alive);
                peak_rss_bytes = peak_rss_bytes.max(rss);
                peak_footprint_bytes = peak_footprint_bytes.max(fp);
                peak_accounted_bytes = peak_accounted_bytes.max(acc);
                max_tree_pids = max_tree_pids.max(count);
                if spec.memory_limit_bytes > 0 && acc > spec.memory_limit_bytes {
                    memory_exceeded = true;
                    terminate_tracked_tree(&mut child, &mut tree);
                    break child.wait().map_err(|e| e.to_string())?;
                }
            }
        }

        if let Some(done) = child.try_wait().map_err(|e| e.to_string())? {
            break done;
        }

        let elapsed_now = started.elapsed().as_millis() as u64;
        if spec.timeout_ms > 0 && elapsed_now >= spec.timeout_ms {
            timed_out = true;
            terminate_tracked_tree(&mut child, &mut tree);
            break child.wait().map_err(|e| e.to_string())?;
        }

        thread::sleep(Duration::from_millis(spec.poll_ms));
    };

    tree.drop_root_after_wait();
    let elapsed_ms = started.elapsed().as_millis() as u64;
    let reaped = reap_tracked_tree(&mut tree);

    let (stdout_cap, stdout_drained) =
        stdout_drain.drain_with_timeout(Duration::from_millis(PIPE_DRAIN_TIMEOUT_MS));
    let (stderr_cap, stderr_drained) =
        stderr_drain.drain_with_timeout(Duration::from_millis(PIPE_DRAIN_TIMEOUT_MS));
    let output_drained = stdout_drained && stderr_drained;
    let children_reaped = finalize_children_reaped(reaped, stdout_drained, stderr_drained);

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
        output_drained,
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
    let rows = snapshot_process_table()?;
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
