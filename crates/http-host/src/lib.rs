#![forbid(unsafe_code)]
//! Safe standalone HTTP edge host delegating requests to a single supervised `femacs-lisp --rpc http`
//! worker process over an inherited `AF_UNIX` `SOCK_STREAM` socketpair (`C2`).

use std::env;
use std::ffi::OsString;
use std::io::{self, BufRead, BufReader, Write};
use std::net::Shutdown;
use std::os::fd::OwnedFd;
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use axum::Router;
use axum::body::{Body, to_bytes};
use axum::extract::{DefaultBodyLimit, Request, State};
use axum::http::{HeaderMap, HeaderName, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::net::TcpListener;
use tokio::sync::{mpsc, oneshot};

pub const DEFAULT_LISTEN_ADDR: &str = "127.0.0.1:0";
pub const DEFAULT_HANDLER_ENTRY: &str = "femacs-http-handle";
pub const DEFAULT_MAX_BODY_BYTES: usize = 1024 * 1024;
pub const DEFAULT_MAX_QUEUE: usize = 64;
pub const DEFAULT_TIMEOUT_MS: u64 = 15_000;
pub const MIN_RPC_FRAME_BYTES: usize = 2 * 1024 * 1024;
pub const RPC_IO_POLL_SLICE: Duration = Duration::from_millis(50);
pub const HTTP_HOST_VERSION: &str = "http-host 0.1.0 (femacs 32.0.50)";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HttpHostConfig {
    pub listen: String,
    pub lisp_bin: Option<PathBuf>,
    pub handler: PathBuf,
    pub entry: String,
    pub load_paths: Vec<PathBuf>,
    pub lisp_dir: Option<PathBuf>,
    pub data_dir: Option<PathBuf>,
    pub dump_file: Option<PathBuf>,
    pub no_dump: bool,
    pub max_body_bytes: usize,
    pub max_queue: usize,
    pub timeout_ms: u64,
    pub rpc_max_frame_bytes: Option<usize>,
}

impl HttpHostConfig {
    pub fn effective_rpc_max_frame_bytes(&self) -> usize {
        self.rpc_max_frame_bytes.unwrap_or_else(|| {
            self.max_body_bytes
                .saturating_mul(2)
                .saturating_add(65_536)
                .max(MIN_RPC_FRAME_BYTES)
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CliOutcome {
    Info(String),
    Usage(String),
}

impl CliOutcome {
    pub fn code(&self) -> i32 {
        match self {
            Self::Info(_) => 0,
            Self::Usage(_) => 2,
        }
    }
}

pub fn usage_text() -> &'static str {
    "Usage: http-host --handler FILE [OPTIONS]\n\
     Options:\n\
       --listen ADDR             Bind TCP address (default: 127.0.0.1:0)\n\
       --lisp-bin PATH           Path to femacs-lisp executable (default: sibling femacs-lisp)\n\
       --handler, --rpc-handler FILE  Lisp handler source file (required)\n\
       --entry, --rpc-entry FUNC Entry function name (default: femacs-http-handle)\n\
       -L, --directory, --load-path DIR  Prepend DIR to Lisp load-path\n\
       --lisp-dir DIR            Override Lisp library directory forwarded to femacs-lisp\n\
       --data-dir DIR            Override shared data directory forwarded to femacs-lisp\n\
       --dump-file PATH          Forward portable dump path to femacs-lisp\n\
       --no-dump                 Run femacs-lisp without loading a portable dump\n\
       --max-body-bytes N        Maximum HTTP request body size in bytes (default: 1048576)\n\
       --max-queue N             Maximum queued in-flight requests (default: 64)\n\
       --timeout-ms N            Per-request backend timeout in milliseconds (default: 15000)\n\
       --rpc-max-frame-bytes N   Override JSON-RPC socket frame bound\n\
       -h, --help                Show this help\n\
       -V, --version             Show version"
}

fn next_arg_os(args: &[OsString], idx: &mut usize, flag: &str) -> Result<OsString, CliOutcome> {
    *idx += 1;
    args.get(*idx)
        .cloned()
        .ok_or_else(|| CliOutcome::Usage(format!("option {flag} requires an argument\n{}", usage_text())))
}

fn next_arg_str(args: &[OsString], idx: &mut usize, flag: &str) -> Result<String, CliOutcome> {
    next_arg_os(args, idx, flag)?
        .into_string()
        .map_err(|_| CliOutcome::Usage(format!("option {flag} requires valid UTF-8")))
}

fn parse_positive_usize(args: &[OsString], idx: &mut usize, flag: &str) -> Result<usize, CliOutcome> {
    let raw = next_arg_str(args, idx, flag)?;
    let n = raw
        .parse::<usize>()
        .map_err(|_| CliOutcome::Usage(format!("invalid {flag} value: {raw}")))?;
    if n == 0 {
        return Err(CliOutcome::Usage(format!("{flag} must be positive")));
    }
    Ok(n)
}

pub fn parse_http_host_args(args: &[OsString]) -> Result<HttpHostConfig, CliOutcome> {
    let mut listen = DEFAULT_LISTEN_ADDR.to_string();
    let mut lisp_bin: Option<PathBuf> = None;
    let mut handler: Option<PathBuf> = None;
    let mut entry = DEFAULT_HANDLER_ENTRY.to_string();
    let mut load_paths: Vec<PathBuf> = Vec::new();
    let mut lisp_dir: Option<PathBuf> = None;
    let mut data_dir: Option<PathBuf> = None;
    let mut dump_file: Option<PathBuf> = None;
    let mut no_dump = false;
    let mut max_body_bytes = DEFAULT_MAX_BODY_BYTES;
    let mut max_queue = DEFAULT_MAX_QUEUE;
    let mut timeout_ms = DEFAULT_TIMEOUT_MS;
    let mut rpc_max_frame_bytes: Option<usize> = None;

    let mut i = 0usize;
    while i < args.len() {
        let cur = args[i].to_string_lossy();
        match cur.as_ref() {
            "-h" | "--help" => return Err(CliOutcome::Info(usage_text().to_string())),
            "-V" | "--version" => return Err(CliOutcome::Info(HTTP_HOST_VERSION.to_string())),
            "--listen" => listen = next_arg_str(args, &mut i, "--listen")?,
            "--lisp-bin" => {
                lisp_bin = Some(PathBuf::from(next_arg_os(args, &mut i, "--lisp-bin")?));
            }
            "--handler" | "--rpc-handler" => {
                let flag = cur.into_owned();
                handler = Some(PathBuf::from(next_arg_os(args, &mut i, &flag)?));
            }
            "--entry" | "--rpc-entry" => {
                let flag = cur.into_owned();
                entry = next_arg_str(args, &mut i, &flag)?;
            }
            "-L" | "--directory" | "--load-path" => {
                let flag = cur.into_owned();
                load_paths.push(PathBuf::from(next_arg_os(args, &mut i, &flag)?));
            }
            "--lisp-dir" => {
                lisp_dir = Some(PathBuf::from(next_arg_os(args, &mut i, "--lisp-dir")?));
            }
            "--data-dir" => {
                data_dir = Some(PathBuf::from(next_arg_os(args, &mut i, "--data-dir")?));
            }
            "--dump-file" => {
                dump_file = Some(PathBuf::from(next_arg_os(args, &mut i, "--dump-file")?));
            }
            "--no-dump" => no_dump = true,
            "--max-body-bytes" => {
                max_body_bytes = parse_positive_usize(args, &mut i, "--max-body-bytes")?;
            }
            "--max-queue" => {
                max_queue = parse_positive_usize(args, &mut i, "--max-queue")?;
            }
            "--timeout-ms" => {
                let raw = next_arg_str(args, &mut i, "--timeout-ms")?;
                let n = raw
                    .parse::<u64>()
                    .map_err(|_| CliOutcome::Usage(format!("invalid --timeout-ms value: {raw}")))?;
                if n == 0 {
                    return Err(CliOutcome::Usage("--timeout-ms must be positive".to_string()));
                }
                timeout_ms = n;
            }
            "--rpc-max-frame-bytes" => {
                rpc_max_frame_bytes =
                    Some(parse_positive_usize(args, &mut i, "--rpc-max-frame-bytes")?);
            }
            other => {
                return Err(CliOutcome::Usage(format!(
                    "unrecognized option: {other}\n{}",
                    usage_text()
                )));
            }
        }
        i += 1;
    }

    if no_dump && dump_file.is_some() {
        return Err(CliOutcome::Usage(
            "--no-dump cannot be combined with --dump-file".to_string(),
        ));
    }

    let handler = handler.ok_or_else(|| {
        CliOutcome::Usage(format!("missing required --handler <FILE>\n{}", usage_text()))
    })?;

    Ok(HttpHostConfig {
        listen,
        lisp_bin,
        handler,
        entry,
        load_paths,
        lisp_dir,
        data_dir,
        dump_file,
        no_dump,
        max_body_bytes,
        max_queue,
        timeout_ms,
        rpc_max_frame_bytes,
    })
}

pub fn resolve_lisp_bin(exe_path: Option<&Path>, explicit: Option<&Path>) -> PathBuf {
    if let Some(path) = explicit {
        return path.to_path_buf();
    }
    if let Some(sibling) = exe_path
        .and_then(|p| p.parent())
        .map(|dir| dir.join("femacs-lisp"))
        .filter(|candidate| candidate.is_file())
    {
        return sibling;
    }
    PathBuf::from("femacs-lisp")
}

pub fn build_backend_args(cfg: &HttpHostConfig) -> Vec<OsString> {
    let mut out = Vec::new();
    for dir in &cfg.load_paths {
        out.push(OsString::from("-L"));
        out.push(dir.as_os_str().to_os_string());
    }
    if let Some(dir) = &cfg.lisp_dir {
        out.push(OsString::from("--lisp-dir"));
        out.push(dir.as_os_str().to_os_string());
    }
    if let Some(dir) = &cfg.data_dir {
        out.push(OsString::from("--data-dir"));
        out.push(dir.as_os_str().to_os_string());
    }
    if let Some(dump) = &cfg.dump_file {
        out.push(OsString::from("--dump-file"));
        out.push(dump.as_os_str().to_os_string());
    }
    if cfg.no_dump {
        out.push(OsString::from("--no-dump"));
    }
    out.push(OsString::from("--rpc"));
    out.push(OsString::from("http"));
    out.push(OsString::from("--rpc-handler"));
    out.push(cfg.handler.as_os_str().to_os_string());
    out.push(OsString::from("--rpc-entry"));
    out.push(OsString::from(&cfg.entry));
    out.push(OsString::from("--rpc-max-frame-bytes"));
    out.push(OsString::from(cfg.effective_rpc_max_frame_bytes().to_string()));
    out
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct RpcHttpRequest {
    pub method: String,
    pub path: String,
    pub query: String,
    pub headers: Vec<[String; 2]>,
    pub body: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct RpcHttpResponse {
    pub status: u16,
    #[serde(default)]
    pub headers: Vec<[String; 2]>,
    #[serde(default)]
    pub body: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RpcResponseFrame {
    Result {
        id: Value,
        response: RpcHttpResponse,
    },
    Error {
        id: Value,
        code: i64,
        message: String,
        data: Option<Value>,
    },
}

pub fn format_rpc_request_line(id: u64, req: &RpcHttpRequest) -> Result<String, String> {
    let payload = json!({
        "jsonrpc": "2.0",
        "id": id,
        "method": "http.handle",
        "params": req,
    });
    let mut line = serde_json::to_string(&payload).map_err(|e| format!("encode rpc frame: {e}"))?;
    line.push('\n');
    Ok(line)
}

pub fn parse_rpc_response_line(line: &str) -> Result<RpcResponseFrame, String> {
    let val: Value =
        serde_json::from_str(line).map_err(|e| format!("invalid JSON-RPC response: {e}"))?;
    let id = val.get("id").cloned().unwrap_or(Value::Null);
    if let Some(err_val) = val.get("error") {
        let code = err_val
            .get("code")
            .and_then(Value::as_i64)
            .unwrap_or(-32000);
        let message = err_val
            .get("message")
            .and_then(Value::as_str)
            .unwrap_or("backend error")
            .to_string();
        let data = err_val.get("data").cloned();
        return Ok(RpcResponseFrame::Error {
            id,
            code,
            message,
            data,
        });
    }
    let result_val = val
        .get("result")
        .ok_or_else(|| "JSON-RPC frame missing both result and error".to_string())?;
    let response: RpcHttpResponse = serde_json::from_value(result_val.clone())
        .map_err(|e| format!("invalid http.handle result payload: {e}"))?;
    Ok(RpcResponseFrame::Result { id, response })
}

pub fn is_fatal_rpc_error(data: &Option<Value>) -> bool {
    data.as_ref()
        .and_then(|d| d.get("kind"))
        .and_then(Value::as_str)
        == Some("exit-request")
}

#[derive(Debug, PartialEq, Eq)]
pub enum BoundedLineRead {
    Eof,
    Line(String),
    TooLarge,
    TimedOut,
    Shutdown,
}

fn read_bounded_rpc_line_with<R: BufRead>(
    reader: &mut R,
    max_bytes: usize,
    retry_timeouts: bool,
    mut before_fill: impl FnMut(&mut R) -> io::Result<Option<BoundedLineRead>>,
    is_shutdown: impl Fn() -> bool,
) -> io::Result<BoundedLineRead> {
    let mut line = Vec::new();
    let mut too_large = false;
    loop {
        if let Some(early) = before_fill(reader)? {
            return Ok(early);
        }
        let buf = match reader.fill_buf() {
            Ok(b) => b,
            Err(e)
                if retry_timeouts
                    && matches!(
                        e.kind(),
                        io::ErrorKind::WouldBlock
                            | io::ErrorKind::TimedOut
                            | io::ErrorKind::Interrupted
                    ) =>
            {
                continue;
            }
            Err(e) => {
                if is_shutdown() {
                    return Ok(BoundedLineRead::Shutdown);
                }
                return Err(e);
            }
        };
        if buf.is_empty() {
            if is_shutdown() {
                return Ok(BoundedLineRead::Shutdown);
            }
            if too_large {
                return Ok(BoundedLineRead::TooLarge);
            }
            if line.is_empty() {
                return Ok(BoundedLineRead::Eof);
            }
            if line.last() == Some(&b'\r') {
                line.pop();
            }
            let s = String::from_utf8(line)
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "non-UTF-8 RPC frame"))?;
            return Ok(BoundedLineRead::Line(s));
        }

        if let Some(nl) = buf.iter().position(|&b| b == b'\n') {
            let chunk = &buf[..nl];
            if too_large || line.len().saturating_add(chunk.len()).saturating_add(1) > max_bytes {
                reader.consume(nl + 1);
                return Ok(BoundedLineRead::TooLarge);
            }
            line.extend_from_slice(chunk);
            reader.consume(nl + 1);
            if line.last() == Some(&b'\r') {
                line.pop();
            }
            let s = String::from_utf8(line)
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "non-UTF-8 RPC frame"))?;
            return Ok(BoundedLineRead::Line(s));
        }

        let len = buf.len();
        if !too_large && line.len().saturating_add(len) <= max_bytes {
            line.extend_from_slice(buf);
        } else {
            too_large = true;
            line.clear();
        }
        reader.consume(len);
    }
}

pub fn read_bounded_rpc_line<R: BufRead>(
    reader: &mut R,
    max_bytes: usize,
) -> io::Result<BoundedLineRead> {
    read_bounded_rpc_line_with(reader, max_bytes, false, |_| Ok(None), || false)
}

fn read_bounded_rpc_line_with_deadline(
    reader: &mut BufReader<UnixStream>,
    max_bytes: usize,
    deadline: Instant,
    shutdown_flag: &AtomicBool,
) -> io::Result<BoundedLineRead> {
    read_bounded_rpc_line_with(
        reader,
        max_bytes,
        true,
        |r| {
            if shutdown_flag.load(Ordering::Acquire) {
                return Ok(Some(BoundedLineRead::Shutdown));
            }
            let now = Instant::now();
            if now >= deadline {
                return Ok(Some(BoundedLineRead::TimedOut));
            }
            if r.buffer().is_empty() {
                let slice = deadline
                    .saturating_duration_since(now)
                    .min(RPC_IO_POLL_SLICE)
                    .max(Duration::from_millis(1));
                r.get_ref().set_read_timeout(Some(slice))?;
            }
            Ok(None)
        },
        || shutdown_flag.load(Ordering::Acquire),
    )
}

#[derive(Debug, PartialEq, Eq)]
enum WriteOutcome {
    Complete,
    TimedOut,
    Shutdown,
}

fn write_all_with_deadline(
    stream: &mut UnixStream,
    mut data: &[u8],
    deadline: Instant,
    shutdown_flag: &AtomicBool,
) -> io::Result<WriteOutcome> {
    while !data.is_empty() {
        if shutdown_flag.load(Ordering::Acquire) {
            return Ok(WriteOutcome::Shutdown);
        }
        let now = Instant::now();
        if now >= deadline {
            return Ok(WriteOutcome::TimedOut);
        }
        let slice = deadline
            .saturating_duration_since(now)
            .min(RPC_IO_POLL_SLICE)
            .max(Duration::from_millis(1));
        stream.set_write_timeout(Some(slice))?;
        match stream.write(data) {
            Ok(0) => {
                if shutdown_flag.load(Ordering::Acquire) {
                    return Ok(WriteOutcome::Shutdown);
                }
                return Err(io::Error::new(
                    io::ErrorKind::WriteZero,
                    "failed to write RPC request bytes",
                ));
            }
            Ok(n) => {
                data = &data[n..];
            }
            Err(e)
                if matches!(
                    e.kind(),
                    io::ErrorKind::WouldBlock
                        | io::ErrorKind::TimedOut
                        | io::ErrorKind::Interrupted
                ) =>
            {
                continue;
            }
            Err(e) => {
                if shutdown_flag.load(Ordering::Acquire) {
                    return Ok(WriteOutcome::Shutdown);
                }
                return Err(e);
            }
        }
    }
    Ok(WriteOutcome::Complete)
}

#[derive(Debug)]
enum WorkerReply {
    Http(RpcHttpResponse),
    HandlerError(String),
    BackendDown(String),
    Timeout(String),
}

struct WorkerJob {
    id: u64,
    request: RpcHttpRequest,
    deadline: Instant,
    respond: oneshot::Sender<WorkerReply>,
}

#[derive(Clone)]
struct AppState {
    tx: mpsc::Sender<WorkerJob>,
    max_body_bytes: usize,
    timeout: Duration,
    next_id: Arc<AtomicU64>,
    shutdown_flag: Arc<AtomicBool>,
}

fn kill_child_slot_now(child_slot: &Arc<Mutex<Option<Child>>>) {
    let taken = {
        let mut guard = child_slot.lock().unwrap_or_else(|e| e.into_inner());
        guard.take()
    };
    if let Some(mut child) = taken {
        let _ = child.kill();
        let _ = child.wait();
    }
}

fn reap_child_slot(child_slot: &Arc<Mutex<Option<Child>>>, grace: Duration) {
    let taken = {
        let mut guard = child_slot.lock().unwrap_or_else(|e| e.into_inner());
        guard.take()
    };
    if let Some(mut child) = taken {
        wait_child_clean(&mut child, grace);
    }
}

fn drain_queued_jobs(rx: &mut mpsc::Receiver<WorkerJob>, reason: &str) {
    while let Ok(job) = rx.try_recv() {
        if job.respond.is_closed() {
            continue;
        }
        let reply = if Instant::now() >= job.deadline {
            WorkerReply::Timeout("request timed out before dispatch".to_string())
        } else {
            WorkerReply::BackendDown(reason.to_string())
        };
        let _ = job.respond.send(reply);
    }
}

fn spawn_stderr_forwarder<R: io::Read + Send + 'static>(
    stream: R,
    prefix: &'static str,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut reader = BufReader::new(stream);
        let mut buf = Vec::new();
        loop {
            buf.clear();
            match reader.read_until(b'\n', &mut buf) {
                Ok(0) => break,
                Ok(_) => {
                    let text = String::from_utf8_lossy(&buf);
                    let mut err = io::stderr().lock();
                    if text.ends_with('\n') {
                        let _ = err.write_all(text.as_bytes());
                    } else {
                        let _ = writeln!(err, "{text}");
                    }
                    let _ = err.flush();
                }
                Err(e) => {
                    let mut err = io::stderr().lock();
                    let _ = writeln!(err, "http-host {prefix} pipe read error: {e}");
                    let _ = err.flush();
                    break;
                }
            }
        }
    })
}

fn run_rpc_worker(
    mut rx: mpsc::Receiver<WorkerJob>,
    mut stream: UnixStream,
    max_frame_bytes: usize,
    shutdown_flag: Arc<AtomicBool>,
    child_slot: Arc<Mutex<Option<Child>>>,
) {
    let read_sock = match stream.try_clone() {
        Ok(s) => s,
        Err(e) => {
            while let Some(job) = rx.blocking_recv() {
                if !job.respond.is_closed() {
                    let _ = job
                        .respond
                        .send(WorkerReply::BackendDown(format!("clone rpc socket: {e}")));
                }
            }
            return;
        }
    };
    let mut reader = BufReader::new(read_sock);
    let mut dead_reason: Option<String> = None;

    while let Some(job) = rx.blocking_recv() {
        if job.respond.is_closed() {
            continue;
        }
        if Instant::now() >= job.deadline {
            let _ = job.respond.send(WorkerReply::Timeout(
                "request timed out in queue before dispatch".to_string(),
            ));
            continue;
        }
        if shutdown_flag.load(Ordering::Acquire) {
            let _ = job
                .respond
                .send(WorkerReply::BackendDown("http-host shutting down".to_string()));
            drain_queued_jobs(&mut rx, "http-host shutting down");
            break;
        }
        if let Some(reason) = &dead_reason {
            let _ = job.respond.send(WorkerReply::BackendDown(reason.clone()));
            continue;
        }

        let frame_line = match format_rpc_request_line(job.id, &job.request) {
            Ok(line) => line,
            Err(e) => {
                let _ = job.respond.send(WorkerReply::HandlerError(e));
                continue;
            }
        };
        if frame_line.len() > max_frame_bytes {
            let _ = job.respond.send(WorkerReply::HandlerError(format!(
                "serialized RPC request exceeds --rpc-max-frame-bytes ({max_frame_bytes})"
            )));
            continue;
        }

        match write_all_with_deadline(&mut stream, frame_line.as_bytes(), job.deadline, &shutdown_flag) {
            Ok(WriteOutcome::Complete) => {}
            Ok(WriteOutcome::TimedOut) => {
                let _ = job
                    .respond
                    .send(WorkerReply::Timeout("Lisp handler timed out".to_string()));
                let reason = "backend terminated after request timeout".to_string();
                dead_reason = Some(reason.clone());
                let _ = stream.shutdown(Shutdown::Both);
                kill_child_slot_now(&child_slot);
                drain_queued_jobs(&mut rx, &reason);
                continue;
            }
            Ok(WriteOutcome::Shutdown) => {
                let _ = job
                    .respond
                    .send(WorkerReply::BackendDown("http-host shutting down".to_string()));
                let _ = stream.shutdown(Shutdown::Both);
                drain_queued_jobs(&mut rx, "http-host shutting down");
                return;
            }
            Err(e) => {
                let msg = format!("backend RPC write failed: {e}");
                dead_reason = Some(msg.clone());
                let _ = job.respond.send(WorkerReply::BackendDown(msg));
                continue;
            }
        }

        loop {
            match read_bounded_rpc_line_with_deadline(
                &mut reader,
                max_frame_bytes,
                job.deadline,
                &shutdown_flag,
            ) {
                Ok(BoundedLineRead::Eof) => {
                    let msg = "backend process closed RPC socket".to_string();
                    dead_reason = Some(msg.clone());
                    let _ = job.respond.send(WorkerReply::BackendDown(msg));
                    break;
                }
                Ok(BoundedLineRead::TimedOut) => {
                    let _ = job
                        .respond
                        .send(WorkerReply::Timeout("Lisp handler timed out".to_string()));
                    let reason = "backend terminated after request timeout".to_string();
                    dead_reason = Some(reason.clone());
                    let _ = stream.shutdown(Shutdown::Both);
                    kill_child_slot_now(&child_slot);
                    drain_queued_jobs(&mut rx, &reason);
                    break;
                }
                Ok(BoundedLineRead::Shutdown) => {
                    let _ = job
                        .respond
                        .send(WorkerReply::BackendDown("http-host shutting down".to_string()));
                    let _ = stream.shutdown(Shutdown::Both);
                    drain_queued_jobs(&mut rx, "http-host shutting down");
                    return;
                }
                Ok(BoundedLineRead::TooLarge) => {
                    let _ = job.respond.send(WorkerReply::HandlerError(format!(
                        "backend RPC response exceeded {max_frame_bytes} bytes"
                    )));
                    break;
                }
                Ok(BoundedLineRead::Line(line)) => {
                    if line.trim().is_empty() {
                        continue;
                    }
                    match parse_rpc_response_line(&line) {
                        Ok(RpcResponseFrame::Result { id, response }) => {
                            if id == json!(job.id) {
                                let _ = job.respond.send(WorkerReply::Http(response));
                                break;
                            }
                        }
                        Ok(RpcResponseFrame::Error {
                            id,
                            code,
                            message,
                            data,
                        }) => {
                            let fatal = is_fatal_rpc_error(&data);
                            {
                                let mut err = io::stderr().lock();
                                let _ = writeln!(err, "femacs-lisp rpc error ({code}): {message}");
                                let _ = err.flush();
                            }
                            if fatal {
                                let msg = format!("backend exited via Lisp exit-request: {message}");
                                dead_reason = Some(msg.clone());
                                if id == json!(job.id) || id.is_null() {
                                    let _ = job.respond.send(WorkerReply::BackendDown(msg));
                                    break;
                                }
                            } else if id == json!(job.id) || id.is_null() {
                                let _ = job.respond.send(WorkerReply::HandlerError(message));
                                break;
                            }
                        }
                        Err(e) => {
                            let _ = job.respond.send(WorkerReply::HandlerError(e));
                            break;
                        }
                    }
                }
                Err(e) => {
                    if shutdown_flag.load(Ordering::Acquire) {
                        let _ = job
                            .respond
                            .send(WorkerReply::BackendDown("http-host shutting down".to_string()));
                        let _ = stream.shutdown(Shutdown::Both);
                        drain_queued_jobs(&mut rx, "http-host shutting down");
                        return;
                    }
                    let msg = format!("backend RPC read failed: {e}");
                    dead_reason = Some(msg.clone());
                    let _ = job.respond.send(WorkerReply::BackendDown(msg));
                    break;
                }
            }
        }
    }

    let _ = stream.shutdown(Shutdown::Both);
}

fn check_content_length(headers: &HeaderMap, max_bytes: usize) -> Result<(), Response> {
    if let Some(raw) = headers.get(axum::http::header::CONTENT_LENGTH)
        && let Ok(text) = raw.to_str()
        && let Ok(len) = text.trim().parse::<usize>()
        && len > max_bytes
    {
        return Err((
            StatusCode::PAYLOAD_TOO_LARGE,
            format!("request body exceeds --max-body-bytes ({max_bytes})"),
        )
            .into_response());
    }
    Ok(())
}

fn build_http_response(rpc_resp: RpcHttpResponse) -> Response {
    let status = StatusCode::from_u16(rpc_resp.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    let mut builder = Response::builder().status(status);
    for [k, v] in rpc_resp.headers {
        let lower = k.to_ascii_lowercase();
        if lower == "content-length" || lower == "transfer-encoding" {
            continue;
        }
        if let (Ok(name), Ok(val)) = (
            HeaderName::from_bytes(k.as_bytes()),
            HeaderValue::from_str(&v),
        ) {
            builder = builder.header(name, val);
        }
    }
    builder
        .body(Body::from(rpc_resp.body))
        .unwrap_or_else(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response())
}

async fn handle_http(State(state): State<AppState>, req: Request) -> Response {
    if state.shutdown_flag.load(Ordering::Acquire) {
        return (StatusCode::BAD_GATEWAY, "http-host shutting down").into_response();
    }
    if let Err(resp) = check_content_length(req.headers(), state.max_body_bytes) {
        return resp;
    }
    let (parts, body) = req.into_parts();
    let bytes = match to_bytes(body, state.max_body_bytes).await {
        Ok(b) => b,
        Err(_) => {
            return (
                StatusCode::PAYLOAD_TOO_LARGE,
                format!(
                    "request body exceeds --max-body-bytes ({})",
                    state.max_body_bytes
                ),
            )
                .into_response();
        }
    };
    let body_text = match String::from_utf8(bytes.to_vec()) {
        Ok(s) => s,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                "request body must be valid UTF-8",
            )
                .into_response();
        }
    };

    let headers: Vec<[String; 2]> = parts
        .headers
        .iter()
        .filter_map(|(k, v)| {
            v.to_str()
                .ok()
                .map(|vs| [k.as_str().to_string(), vs.to_string()])
        })
        .collect();

    let rpc_req = RpcHttpRequest {
        method: parts.method.as_str().to_string(),
        path: parts.uri.path().to_string(),
        query: parts.uri.query().unwrap_or("").to_string(),
        headers,
        body: body_text,
    };

    let req_id = state.next_id.fetch_add(1, Ordering::Relaxed);
    let deadline = Instant::now() + state.timeout;
    let (resp_tx, resp_rx) = oneshot::channel();
    let job = WorkerJob {
        id: req_id,
        request: rpc_req,
        deadline,
        respond: resp_tx,
    };

    if let Err(err) = state.tx.try_send(job) {
        return match err {
            mpsc::error::TrySendError::Full(_) => {
                (StatusCode::SERVICE_UNAVAILABLE, "backend request queue full").into_response()
            }
            mpsc::error::TrySendError::Closed(_) => {
                (StatusCode::BAD_GATEWAY, "backend worker shut down").into_response()
            }
        };
    }

    match tokio::time::timeout_at(tokio::time::Instant::from_std(deadline), resp_rx).await {
        Ok(Ok(WorkerReply::Http(resp))) => build_http_response(resp),
        Ok(Ok(WorkerReply::HandlerError(msg))) => {
            (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
        }
        Ok(Ok(WorkerReply::BackendDown(msg))) => (StatusCode::BAD_GATEWAY, msg).into_response(),
        Ok(Ok(WorkerReply::Timeout(msg))) => (StatusCode::GATEWAY_TIMEOUT, msg).into_response(),
        Ok(Err(_)) => (StatusCode::BAD_GATEWAY, "backend closed reply channel").into_response(),
        Err(_) => (StatusCode::GATEWAY_TIMEOUT, "Lisp handler timed out").into_response(),
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        let _ = tokio::signal::ctrl_c().await;
    };
    let terminate = async {
        if let Ok(mut sig) =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        {
            sig.recv().await;
        } else {
            std::future::pending::<()>().await;
        }
    };
    tokio::select! {
        () = ctrl_c => {}
        () = terminate => {}
    }
}

fn wait_child_clean(child: &mut Child, timeout: Duration) {
    let deadline = Instant::now() + timeout;
    loop {
        match child.try_wait() {
            Ok(Some(_)) => return,
            Ok(None) if Instant::now() < deadline => thread::sleep(Duration::from_millis(20)),
            _ => {
                let _ = child.kill();
                let _ = child.wait();
                return;
            }
        }
    }
}

pub async fn serve(cfg: HttpHostConfig) -> Result<i32, String> {
    let current_exe = env::current_exe().ok();
    let lisp_bin = resolve_lisp_bin(current_exe.as_deref(), cfg.lisp_bin.as_deref());
    let listener = TcpListener::bind(&cfg.listen)
        .await
        .map_err(|e| format!("failed to bind {}: {e}", cfg.listen))?;
    let local_addr = listener
        .local_addr()
        .map_err(|e| format!("read bound socket address: {e}"))?;

    let (parent_sock, child_sock) =
        UnixStream::pair().map_err(|e| format!("create RPC Unix socketpair: {e}"))?;
    let control_sock = parent_sock
        .try_clone()
        .map_err(|e| format!("clone control RPC socket: {e}"))?;
    let control_sock_sig = parent_sock
        .try_clone()
        .map_err(|e| format!("clone shutdown RPC socket: {e}"))?;
    let child_fd = OwnedFd::from(child_sock);
    let backend_args = build_backend_args(&cfg);

    let child = Command::new(&lisp_bin)
        .args(&backend_args)
        .stdin(Stdio::from(child_fd))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("spawn {}: {e}", lisp_bin.display()))?;

    let backend_pid = child.id();
    let mut child = child;
    let out_pipe = child
        .stdout
        .take()
        .ok_or_else(|| "missing backend stdout pipe".to_string())?;
    let err_pipe = child
        .stderr
        .take()
        .ok_or_else(|| "missing backend stderr pipe".to_string())?;
    let out_fwd = spawn_stderr_forwarder(out_pipe, "stdout");
    let err_fwd = spawn_stderr_forwarder(err_pipe, "stderr");

    let (tx, rx) = mpsc::channel(cfg.max_queue);
    let max_frame = cfg.effective_rpc_max_frame_bytes();
    let shutdown_flag = Arc::new(AtomicBool::new(false));
    let child_slot = Arc::new(Mutex::new(Some(child)));
    let worker_shutdown = Arc::clone(&shutdown_flag);
    let worker_child = Arc::clone(&child_slot);
    let worker_handle = thread::spawn(move || {
        run_rpc_worker(rx, parent_sock, max_frame, worker_shutdown, worker_child);
    });

    let immediate_exit = {
        let mut guard = child_slot.lock().unwrap_or_else(|e| e.into_inner());
        guard.as_mut().and_then(|c| c.try_wait().ok().flatten())
    };
    if let Some(status) = immediate_exit {
        shutdown_flag.store(true, Ordering::Release);
        let _ = control_sock.shutdown(Shutdown::Both);
        drop(tx);
        reap_child_slot(&child_slot, Duration::from_millis(200));
        let _ = worker_handle.join();
        let _ = out_fwd.join();
        let _ = err_fwd.join();
        return Err(format!("backend {} exited immediately with {status}", lisp_bin.display()));
    }

    println!("LISTENING_ADDR={local_addr}");
    println!("BACKEND_PID={backend_pid}");
    let _ = io::stdout().flush();

    let state = AppState {
        tx: tx.clone(),
        max_body_bytes: cfg.max_body_bytes,
        timeout: Duration::from_millis(cfg.timeout_ms),
        next_id: Arc::new(AtomicU64::new(1)),
        shutdown_flag: Arc::clone(&shutdown_flag),
    };
    drop(tx);

    let app = Router::new()
        .fallback(handle_http)
        .with_state(state)
        .layer(DefaultBodyLimit::disable());

    let shutdown_flag_sig = Arc::clone(&shutdown_flag);
    let shutdown = async move {
        shutdown_signal().await;
        shutdown_flag_sig.store(true, Ordering::Release);
        let _ = control_sock_sig.shutdown(Shutdown::Both);
    };

    let serve_res = axum::serve(listener, app)
        .with_graceful_shutdown(shutdown)
        .await;

    shutdown_flag.store(true, Ordering::Release);
    let _ = control_sock.shutdown(Shutdown::Both);
    reap_child_slot(&child_slot, Duration::from_millis(500));
    let _ = worker_handle.join();
    let _ = out_fwd.join();
    let _ = err_fwd.join();

    serve_res.map_err(|e| format!("http server error: {e}"))?;
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use std::sync::atomic::{AtomicU64, Ordering};

    static SEQ: AtomicU64 = AtomicU64::new(0);

    fn os_args(items: &[&str]) -> Vec<OsString> {
        items.iter().map(OsString::from).collect()
    }

    #[test]
    fn parses_cli_and_validates_required_handler_and_bounds() {
        assert_eq!(parse_http_host_args(&[]).unwrap_err().code(), 2);
        assert_eq!(
            parse_http_host_args(&os_args(&["--handler", "a.el", "--no-dump", "--dump-file", "b.pdmp"]))
                .unwrap_err()
                .code(),
            2
        );
        assert_eq!(
            parse_http_host_args(&os_args(&["--handler", "a.el", "--max-body-bytes", "0"]))
                .unwrap_err()
                .code(),
            2
        );

        let cfg = parse_http_host_args(&os_args(&[
            "--listen",
            "127.0.0.1:9000",
            "--handler",
            "examples/lisp-web/http-app.el",
            "-L",
            "examples/lisp-web",
            "--entry",
            "my-entry",
            "--max-body-bytes",
            "2048",
            "--max-queue",
            "16",
            "--timeout-ms",
            "4000",
        ]))
        .expect("valid http-host CLI");
        assert_eq!(cfg.listen, "127.0.0.1:9000");
        assert_eq!(cfg.entry, "my-entry");
        assert_eq!(cfg.max_body_bytes, 2048);
        assert_eq!(cfg.max_queue, 16);
        assert_eq!(cfg.timeout_ms, 4000);
        assert_eq!(cfg.effective_rpc_max_frame_bytes(), MIN_RPC_FRAME_BYTES);

        let args = build_backend_args(&cfg);
        assert!(args.windows(2).any(|w| w == [OsString::from("--rpc"), OsString::from("http")]));
        assert!(args.windows(2).any(|w| w == [OsString::from("--rpc-entry"), OsString::from("my-entry")]));
    }

    #[test]
    fn resolves_sibling_femacs_lisp_binary() {
        let n = SEQ.fetch_add(1, Ordering::Relaxed);
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../build")
            .join(format!("http-host-unit-{}-{n}", std::process::id()));
        let bin_dir = root.join("bin");
        std::fs::create_dir_all(&bin_dir).unwrap();
        let exe = bin_dir.join("http-host");
        let lisp = bin_dir.join("femacs-lisp");
        std::fs::write(&exe, b"exe").unwrap();
        std::fs::write(&lisp, b"lisp").unwrap();
        assert_eq!(resolve_lisp_bin(Some(&exe), None), lisp);
        assert_eq!(
            resolve_lisp_bin(Some(&exe), Some(Path::new("/custom/femacs-lisp"))),
            PathBuf::from("/custom/femacs-lisp")
        );
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn rpc_framing_and_error_classification_handle_valid_and_fatal_frames() {
        let req = RpcHttpRequest {
            method: "POST".to_string(),
            path: "/api/echo".to_string(),
            query: "k=v".to_string(),
            headers: vec![["x-custom".to_string(), "1".to_string()]],
            body: "{\"a\":1}".to_string(),
        };
        let line = format_rpc_request_line(42, &req).unwrap();
        assert!(line.ends_with('\n'));
        let parsed: Value = serde_json::from_str(line.trim()).unwrap();
        assert_eq!(parsed["id"], 42);
        assert_eq!(parsed["method"], "http.handle");

        let ok_frame = parse_rpc_response_line(
            r#"{"jsonrpc":"2.0","id":42,"result":{"status":201,"headers":[["content-type","text/plain"]],"body":"ok"}}"#,
        )
        .unwrap();
        assert!(matches!(ok_frame, RpcResponseFrame::Result { .. }));

        let app_err = parse_rpc_response_line(
            r#"{"jsonrpc":"2.0","id":42,"error":{"code":-32000,"message":"boom","data":{"kind":"lisp-condition"}}}"#,
        )
        .unwrap();
        match app_err {
            RpcResponseFrame::Error { data, .. } => assert!(!is_fatal_rpc_error(&data)),
            _ => panic!("expected RpcResponseFrame::Error"),
        }

        let fatal_err = parse_rpc_response_line(
            r#"{"jsonrpc":"2.0","id":42,"error":{"code":-32000,"message":"exit","data":{"kind":"exit-request","code":0}}}"#,
        )
        .unwrap();
        match fatal_err {
            RpcResponseFrame::Error { data, .. } => assert!(is_fatal_rpc_error(&data)),
            _ => panic!("expected RpcResponseFrame::Error"),
        }

        let framed = format!("{}\n{{\"ok\":true}}\n", "z".repeat(48));
        let mut cursor = Cursor::new(framed.into_bytes());
        assert_eq!(read_bounded_rpc_line(&mut cursor, 16).unwrap(), BoundedLineRead::TooLarge);
        assert_eq!(
            read_bounded_rpc_line(&mut cursor, 32).unwrap(),
            BoundedLineRead::Line("{\"ok\":true}".to_string())
        );
    }

    #[test]
    fn http_response_builder_strips_conflicting_length_headers() {
        let resp = build_http_response(RpcHttpResponse {
            status: 200,
            headers: vec![
                ["Content-Length".to_string(), "9999".to_string()],
                ["Transfer-Encoding".to_string(), "chunked".to_string()],
                ["Content-Type".to_string(), "application/json".to_string()],
                ["X-Custom-Tag".to_string(), "c2".to_string()],
            ],
            body: "{\"ok\":true}".to_string(),
        });
        assert_eq!(resp.status(), StatusCode::OK);
        assert!(resp.headers().get("content-length").is_none());
        assert!(resp.headers().get("transfer-encoding").is_none());
        assert_eq!(resp.headers().get("content-type").unwrap(), "application/json");
        assert_eq!(resp.headers().get("x-custom-tag").unwrap(), "c2");

        let fallback_status = build_http_response(RpcHttpResponse {
            status: 99,
            headers: vec![],
            body: "bad".to_string(),
        });
        assert_eq!(fallback_status.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    fn make_test_worker_job(
        id: u64,
        path: &str,
        deadline: Instant,
    ) -> (WorkerJob, oneshot::Receiver<WorkerReply>) {
        let (tx, rx) = oneshot::channel();
        let job = WorkerJob {
            id,
            request: RpcHttpRequest {
                method: "GET".to_string(),
                path: path.to_string(),
                query: String::new(),
                headers: Vec::new(),
                body: String::new(),
            },
            deadline,
            respond: tx,
        };
        (job, rx)
    }

    #[test]
    fn rpc_worker_evicts_expired_and_disconnected_jobs_without_writing_to_socket() {
        use std::io::Read;

        let (worker_sock, mut peer) = UnixStream::pair().unwrap();
        peer.set_nonblocking(true).unwrap();
        let (tx, rx) = mpsc::channel(8);

        let (expired_job, exp_rx) =
            make_test_worker_job(1, "/expired", Instant::now() - Duration::from_secs(1));
        let (disc_job, disc_rx) =
            make_test_worker_job(2, "/disconnected", Instant::now() + Duration::from_secs(10));
        drop(disc_rx);

        tx.blocking_send(expired_job).unwrap();
        tx.blocking_send(disc_job).unwrap();
        drop(tx);

        run_rpc_worker(
            rx,
            worker_sock,
            MIN_RPC_FRAME_BYTES,
            Arc::new(AtomicBool::new(false)),
            Arc::new(Mutex::new(None)),
        );

        assert!(matches!(exp_rx.blocking_recv(), Ok(WorkerReply::Timeout(_))));
        let mut buf = [0u8; 256];
        match peer.read(&mut buf) {
            Ok(0) => {}
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {}
            other => panic!("expected zero bytes written to RPC peer, got {other:?}"),
        }
    }

    #[test]
    fn rpc_worker_enforces_absolute_deadline_on_hung_peer() {
        let (worker_sock, _peer) = UnixStream::pair().unwrap();
        let (tx, rx) = mpsc::channel(8);
        let shutdown_flag = Arc::new(AtomicBool::new(false));
        let child_slot = Arc::new(Mutex::new(None));

        let handle = thread::spawn(move || {
            run_rpc_worker(rx, worker_sock, MIN_RPC_FRAME_BYTES, shutdown_flag, child_slot);
        });

        let start = Instant::now();
        let (hung_job, hung_rx) =
            make_test_worker_job(1, "/hang", Instant::now() + Duration::from_millis(120));
        let (next_job, next_rx) =
            make_test_worker_job(2, "/after-hang", Instant::now() + Duration::from_secs(10));
        tx.blocking_send(hung_job).unwrap();
        tx.blocking_send(next_job).unwrap();

        assert!(matches!(hung_rx.blocking_recv(), Ok(WorkerReply::Timeout(_))));
        assert!(matches!(next_rx.blocking_recv(), Ok(WorkerReply::BackendDown(_))));
        assert!(start.elapsed() < Duration::from_secs(1));

        drop(tx);
        handle.join().unwrap();
    }

    #[test]
    fn rpc_worker_wakes_promptly_on_shutdown_flag_and_drains_queue() {
        use std::io::Read;

        let (worker_sock, mut peer) = UnixStream::pair().unwrap();
        let control_sock = worker_sock.try_clone().unwrap();
        let (tx, rx) = mpsc::channel(8);
        let shutdown_flag = Arc::new(AtomicBool::new(false));
        let worker_shutdown = Arc::clone(&shutdown_flag);
        let child_slot = Arc::new(Mutex::new(None));

        let handle = thread::spawn(move || {
            run_rpc_worker(rx, worker_sock, MIN_RPC_FRAME_BYTES, worker_shutdown, child_slot);
        });

        let (job1, rx1) =
            make_test_worker_job(1, "/job1", Instant::now() + Duration::from_secs(10));
        let (job2, rx2) =
            make_test_worker_job(2, "/job2", Instant::now() + Duration::from_secs(10));
        tx.blocking_send(job1).unwrap();
        tx.blocking_send(job2).unwrap();

        thread::sleep(Duration::from_millis(80));
        let t0 = Instant::now();
        shutdown_flag.store(true, Ordering::Release);
        let _ = control_sock.shutdown(Shutdown::Both);

        drop(tx);
        handle.join().unwrap();
        assert!(t0.elapsed() < Duration::from_secs(1));

        assert!(matches!(rx1.blocking_recv(), Ok(WorkerReply::BackendDown(_))));
        assert!(matches!(rx2.blocking_recv(), Ok(WorkerReply::BackendDown(_))));

        drop(control_sock);
        let mut written = String::new();
        peer.read_to_string(&mut written).unwrap();
        assert!(written.contains("/job1"), "expected job1 sent to peer: {written:?}");
        assert!(!written.contains("/job2"), "job2 must not be dispatched after shutdown: {written:?}");
    }
}
