#![forbid(unsafe_code)]
//! Standalone `femacs-lisp` CLI parsing, relocatable resource discovery, REPL form boundary
//! detection, and bounded JSON-RPC 2.0 `http.handle` wire framing (`C2`).

use std::env;
use std::ffi::OsString;
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};

use femacs_vm::host_runtime::{HostBootstrap, HostInitConfig, VmHostError};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

pub const DEFAULT_RPC_MAX_FRAME_BYTES: usize = 2 * 1024 * 1024;
pub const DEFAULT_RPC_ENTRY: &str = "femacs-http-handle";
pub const FEMACS_LISP_VERSION: &str = "femacs-lisp 0.1.0 (femacs 32.0.50)";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BatchAction {
    AddLoadPath(PathBuf),
    Load(String),
    Eval(String),
    Funcall { func: String, args: Vec<OsString> },
    Script { path: PathBuf, args: Vec<OsString> },
    ExtraArg(OsString),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RpcMode {
    Http {
        handler: PathBuf,
        entry: String,
        max_frame_bytes: usize,
        fd: Option<i32>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RunMode {
    Actions,
    Repl,
    DumpOut(PathBuf),
    Rpc(RpcMode),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParsedCli {
    pub batch: bool,
    pub quick: bool,
    pub no_dump: bool,
    pub dump_file: Option<PathBuf>,
    pub lisp_dir: Option<PathBuf>,
    pub data_dir: Option<PathBuf>,
    pub actions: Vec<BatchAction>,
    pub mode: RunMode,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CliExit {
    Info(String),
    Usage(String),
}

impl CliExit {
    pub fn code(&self) -> i32 {
        match self {
            Self::Info(_) => 0,
            Self::Usage(_) => 2,
        }
    }
}

pub fn usage_text() -> &'static str {
    "Usage: femacs-lisp [OPTIONS] [--eval FORM | --script FILE [-- ARGS...] | --repl | --rpc http --handler FILE]\n\
     Options:\n\
       -e, --eval FORM          Evaluate FORM and print result (or silent under --batch)\n\
       -l, --load FILE          Load Lisp file or library\n\
       -L, --directory DIR      Prepend DIR to load-path\n\
       -f, --funcall FUNC ...   Call FUNC with trailing arguments in command-line-args-left\n\
           --script FILE [...]  Execute FILE as a Lisp script with trailing ARGS\n\
           --repl               Run interactive Lisp REPL on stdin/stdout\n\
           --batch              Suppress implicit --eval value printing\n\
       -Q, --quick              Accept standard batch isolation flags\n\
           --no-site-file       Accept standard batch isolation flags\n\
           --no-site-lisp       Accept standard batch isolation flags\n\
           --lisp-dir DIR       Override standard library Lisp root directory\n\
           --data-dir DIR       Override shared etc/charsets data directory\n\
           --dump-file PATH     Restore portable dump from PATH\n\
           --no-dump            Cold-boot from Lisp sources without loading .pdmp\n\
           --dump-out PATH      Cold-boot loadup and write relocatable .pdmp to PATH\n\
           --rpc http           Serve JSON-RPC 2.0 `http.handle` requests over socketpair\n\
           --rpc-handler FILE   Lisp handler file loaded for --rpc http\n\
           --rpc-entry FUNC     Entry function symbol (default: femacs-http-handle)\n\
           --rpc-max-frame-bytes N  Max JSON-RPC frame size in bytes (default: 2097152)\n\
           --rpc-fd N           Inherited socketpair descriptor (default: stdin fd 0)\n\
       -h, --help               Show this help\n\
       -V, --version            Show version"
}

fn next_arg_os(args: &[OsString], idx: &mut usize, flag: &str) -> Result<OsString, CliExit> {
    *idx += 1;
    args.get(*idx)
        .cloned()
        .ok_or_else(|| CliExit::Usage(format!("option {flag} requires an argument\n{}", usage_text())))
}

fn next_arg_str(args: &[OsString], idx: &mut usize, flag: &str) -> Result<String, CliExit> {
    next_arg_os(args, idx, flag)?
        .into_string()
        .map_err(|_| CliExit::Usage(format!("option {flag} requires valid UTF-8")))
}

/// Parse standalone `femacs-lisp` CLI arguments (excluding argv[0]).
pub fn parse_cli_args(args: &[OsString]) -> Result<ParsedCli, CliExit> {
    let mut batch = false;
    let mut quick = false;
    let mut no_dump = false;
    let mut dump_file: Option<PathBuf> = None;
    let mut dump_out: Option<PathBuf> = None;
    let mut lisp_dir: Option<PathBuf> = None;
    let mut data_dir: Option<PathBuf> = None;
    let mut actions: Vec<BatchAction> = Vec::new();
    let mut saw_batch_consumer = false;
    let mut repl = false;
    let mut rpc_kind: Option<String> = None;
    let mut rpc_handler: Option<PathBuf> = None;
    let mut rpc_entry = DEFAULT_RPC_ENTRY.to_string();
    let mut rpc_max_frame_bytes = DEFAULT_RPC_MAX_FRAME_BYTES;
    let mut rpc_fd: Option<i32> = None;

    let mut i = 0usize;
    while i < args.len() {
        let cur = args[i].to_string_lossy();
        match cur.as_ref() {
            "-h" | "--help" => return Err(CliExit::Info(usage_text().to_string())),
            "-V" | "--version" => return Err(CliExit::Info(FEMACS_LISP_VERSION.to_string())),
            "-batch" | "--batch" => batch = true,
            "-Q" | "--quick" => quick = true,
            "--no-site-file" | "--no-site-lisp" | "-nsl" => {}
            "--no-dump" => no_dump = true,
            "--dump-file" => {
                dump_file = Some(PathBuf::from(next_arg_os(args, &mut i, "--dump-file")?));
            }
            "--dump-out" => {
                dump_out = Some(PathBuf::from(next_arg_os(args, &mut i, "--dump-out")?));
            }
            "--lisp-dir" => {
                lisp_dir = Some(PathBuf::from(next_arg_os(args, &mut i, "--lisp-dir")?));
            }
            "--data-dir" => {
                data_dir = Some(PathBuf::from(next_arg_os(args, &mut i, "--data-dir")?));
            }
            "-L" | "--directory" => {
                let flag = cur.into_owned();
                actions.push(BatchAction::AddLoadPath(PathBuf::from(next_arg_os(
                    args, &mut i, &flag,
                )?)));
            }
            "-l" | "--load" => {
                let flag = cur.into_owned();
                actions.push(BatchAction::Load(next_arg_str(args, &mut i, &flag)?));
                saw_batch_consumer = true;
            }
            "-e" | "--eval" => {
                let flag = cur.into_owned();
                actions.push(BatchAction::Eval(next_arg_str(args, &mut i, &flag)?));
                saw_batch_consumer = true;
            }
            "-f" | "--funcall" => {
                let flag = cur.into_owned();
                let func = next_arg_str(args, &mut i, &flag)?;
                let tail = args.get(i + 1..).unwrap_or(&[]).to_vec();
                actions.push(BatchAction::Funcall { func, args: tail });
                break;
            }
            "--script" => {
                let script = PathBuf::from(next_arg_os(args, &mut i, "--script")?);
                let mut rest = args.get(i + 1..).unwrap_or(&[]);
                if rest.first().is_some_and(|a| a == "--") {
                    rest = &rest[1..];
                }
                actions.push(BatchAction::Script {
                    path: script,
                    args: rest.to_vec(),
                });
                break;
            }
            "--repl" => repl = true,
            "--rpc" => {
                rpc_kind = Some(next_arg_str(args, &mut i, "--rpc")?);
            }
            "--rpc-handler" | "--handler" => {
                let flag = cur.into_owned();
                rpc_handler = Some(PathBuf::from(next_arg_os(args, &mut i, &flag)?));
            }
            "--rpc-entry" => {
                rpc_entry = next_arg_str(args, &mut i, "--rpc-entry")?;
            }
            "--rpc-max-frame-bytes" => {
                let raw = next_arg_str(args, &mut i, "--rpc-max-frame-bytes")?;
                rpc_max_frame_bytes = raw.parse::<usize>().map_err(|_| {
                    CliExit::Usage(format!("invalid --rpc-max-frame-bytes value: {raw}"))
                })?;
                if rpc_max_frame_bytes == 0 {
                    return Err(CliExit::Usage(
                        "--rpc-max-frame-bytes must be positive".to_string(),
                    ));
                }
            }
            "--rpc-fd" => {
                let raw = next_arg_str(args, &mut i, "--rpc-fd")?;
                let fd = raw
                    .parse::<i32>()
                    .map_err(|_| CliExit::Usage(format!("invalid --rpc-fd value: {raw}")))?;
                if fd < 0 {
                    return Err(CliExit::Usage("--rpc-fd must be >= 0".to_string()));
                }
                rpc_fd = Some(fd);
            }
            "--" => {
                if batch
                    && saw_batch_consumer
                    && !repl
                    && rpc_kind.is_none()
                    && dump_out.is_none()
                {
                    for item in &args[i..] {
                        actions.push(BatchAction::ExtraArg(item.clone()));
                    }
                    break;
                }
                return Err(CliExit::Usage(format!(
                    "unrecognized option or argument: --\n{}",
                    usage_text()
                )));
            }
            other => {
                if batch
                    && saw_batch_consumer
                    && !repl
                    && rpc_kind.is_none()
                    && dump_out.is_none()
                {
                    actions.push(BatchAction::ExtraArg(args[i].clone()));
                } else {
                    return Err(CliExit::Usage(format!(
                        "unrecognized option or argument: {other}\n{}",
                        usage_text()
                    )));
                }
            }
        }
        i += 1;
    }

    if no_dump && dump_file.is_some() {
        return Err(CliExit::Usage(
            "--no-dump cannot be combined with --dump-file".to_string(),
        ));
    }

    let mode = if let Some(out) = dump_out {
        if repl || rpc_kind.is_some() || dump_file.is_some() {
            return Err(CliExit::Usage(
                "--dump-out cannot be combined with --repl, --rpc, or --dump-file".to_string(),
            ));
        }
        if actions.iter().any(|a| {
            matches!(
                a,
                BatchAction::Script { .. }
                    | BatchAction::Eval(_)
                    | BatchAction::Funcall { .. }
                    | BatchAction::ExtraArg(_)
            )
        }) {
            return Err(CliExit::Usage(
                "--dump-out cannot be combined with --script, --eval, --funcall, or trailing batch arguments".to_string(),
            ));
        }
        RunMode::DumpOut(out)
    } else if let Some(kind) = rpc_kind {
        if kind != "http" {
            return Err(CliExit::Usage(format!(
                "unsupported --rpc mode '{kind}'; expected 'http'"
            )));
        }
        if repl {
            return Err(CliExit::Usage(
                "--rpc cannot be combined with --repl".to_string(),
            ));
        }
        if actions.iter().any(|a| {
            matches!(
                a,
                BatchAction::Script { .. }
                    | BatchAction::Funcall { .. }
                    | BatchAction::ExtraArg(_)
            )
        }) {
            return Err(CliExit::Usage(
                "--rpc http cannot be combined with --script, --funcall, or trailing batch arguments".to_string(),
            ));
        }
        let handler = rpc_handler.ok_or_else(|| {
            CliExit::Usage("--rpc http requires --rpc-handler <FILE>".to_string())
        })?;
        RunMode::Rpc(RpcMode::Http {
            handler,
            entry: rpc_entry,
            max_frame_bytes: rpc_max_frame_bytes,
            fd: rpc_fd,
        })
    } else if repl {
        if actions.iter().any(|a| {
            matches!(
                a,
                BatchAction::Script { .. }
                    | BatchAction::Eval(_)
                    | BatchAction::Funcall { .. }
                    | BatchAction::ExtraArg(_)
            )
        }) {
            return Err(CliExit::Usage(
                "--repl cannot be combined with --script, --eval, --funcall, or trailing batch arguments".to_string(),
            ));
        }
        RunMode::Repl
    } else {
        let has_work = actions.iter().any(|a| {
            matches!(
                a,
                BatchAction::Eval(_)
                    | BatchAction::Load(_)
                    | BatchAction::Script { .. }
                    | BatchAction::Funcall { .. }
            )
        });
        if !has_work {
            return Err(CliExit::Usage(format!(
                "missing execution mode (--eval, --script, --repl, --load, --funcall, or --rpc)\n{}",
                usage_text()
            )));
        }
        RunMode::Actions
    };

    Ok(ParsedCli {
        batch,
        quick,
        no_dump,
        dump_file,
        lisp_dir,
        data_dir,
        actions,
        mode,
    })
}

/// Convert a pending suffix of `BatchAction` items back into CLI tokens suitable for binding
/// `command-line-args-left` while executing a `-l` or `-e` step.
pub fn actions_to_argv(actions: &[BatchAction]) -> Vec<OsString> {
    let mut out = Vec::new();
    for action in actions {
        match action {
            BatchAction::AddLoadPath(dir) => {
                out.push(OsString::from("-L"));
                out.push(dir.as_os_str().to_os_string());
            }
            BatchAction::Load(file) => {
                out.push(OsString::from("-l"));
                out.push(OsString::from(file));
            }
            BatchAction::Eval(form) => {
                out.push(OsString::from("-e"));
                out.push(OsString::from(form));
            }
            BatchAction::Funcall { func, args } => {
                out.push(OsString::from("-f"));
                out.push(OsString::from(func));
                out.extend(args.iter().cloned());
            }
            BatchAction::Script { path, args } => {
                out.push(OsString::from("--script"));
                out.push(path.as_os_str().to_os_string());
                if !args.is_empty() {
                    out.push(OsString::from("--"));
                    out.extend(args.iter().cloned());
                }
            }
            BatchAction::ExtraArg(arg) => {
                out.push(arg.clone());
            }
        }
    }
    out
}

/// Re-parse remaining `command-line-args-left` returned by a `-l` or `-e` step back into
/// pending `BatchAction` items, failing with exit code 2 if an unconsumed head option remains.
pub fn parse_remaining_batch_actions(args: &[OsString]) -> Result<Vec<BatchAction>, CliExit> {
    let mut actions = Vec::new();
    let mut saw_consumer = false;
    let mut i = 0usize;
    while i < args.len() {
        let cur = args[i].to_string_lossy();
        match cur.as_ref() {
            "-batch" | "--batch" | "-Q" | "--quick" | "--no-site-file" | "--no-site-lisp"
            | "-nsl" => {}
            "-L" | "--directory" => {
                let flag = cur.into_owned();
                actions.push(BatchAction::AddLoadPath(PathBuf::from(next_arg_os(
                    args, &mut i, &flag,
                )?)));
            }
            "-l" | "--load" => {
                let flag = cur.into_owned();
                actions.push(BatchAction::Load(next_arg_str(args, &mut i, &flag)?));
                saw_consumer = true;
            }
            "-e" | "--eval" => {
                let flag = cur.into_owned();
                actions.push(BatchAction::Eval(next_arg_str(args, &mut i, &flag)?));
                saw_consumer = true;
            }
            "-f" | "--funcall" => {
                let flag = cur.into_owned();
                let func = next_arg_str(args, &mut i, &flag)?;
                let tail = args.get(i + 1..).unwrap_or(&[]).to_vec();
                actions.push(BatchAction::Funcall { func, args: tail });
                break;
            }
            "--script" => {
                let script = PathBuf::from(next_arg_os(args, &mut i, "--script")?);
                let mut rest = args.get(i + 1..).unwrap_or(&[]);
                if rest.first().is_some_and(|a| a == "--") {
                    rest = &rest[1..];
                }
                actions.push(BatchAction::Script {
                    path: script,
                    args: rest.to_vec(),
                });
                break;
            }
            "--" => {
                if saw_consumer {
                    for item in &args[i..] {
                        actions.push(BatchAction::ExtraArg(item.clone()));
                    }
                    break;
                }
                return Err(CliExit::Usage(format!(
                    "unrecognized option or argument: --\n{}",
                    usage_text()
                )));
            }
            other => {
                if saw_consumer {
                    actions.push(BatchAction::ExtraArg(args[i].clone()));
                } else {
                    return Err(CliExit::Usage(format!(
                        "unrecognized option or argument: {other}\n{}",
                        usage_text()
                    )));
                }
            }
        }
        i += 1;
    }
    Ok(actions)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PackagePaths {
    pub bootstrap: HostBootstrap,
    pub lisp_dir: Option<PathBuf>,
    pub data_dir: Option<PathBuf>,
}

fn non_empty_env_dir(val: Option<OsString>) -> Option<PathBuf> {
    val.and_then(|v| {
        let s = v.to_string_lossy();
        let trimmed = s.split(':').find(|part| !part.is_empty())?;
        Some(PathBuf::from(trimmed))
    })
}

/// Resolve relocatable Lisp/data/pdump paths relative to `exe_path` with explicit overrides first.
pub fn resolve_package_paths(
    exe_path: Option<&Path>,
    cli: &ParsedCli,
    env_lisp_dir: Option<OsString>,
    env_emacs_loadpath: Option<OsString>,
    env_data_dir: Option<OsString>,
    env_emacs_data: Option<OsString>,
) -> Result<PackagePaths, String> {
    let exe_dir = exe_path.and_then(|p| p.parent());
    let pkg_root = exe_dir.and_then(|d| d.parent());

    let sibling_lisp = pkg_root
        .map(|r| r.join("lisp"))
        .filter(|p| p.is_dir())
        .map(|p| std::fs::canonicalize(&p).unwrap_or(p));
    let sibling_etc = pkg_root
        .map(|r| r.join("etc"))
        .filter(|p| p.is_dir())
        .map(|p| std::fs::canonicalize(&p).unwrap_or(p));

    let lisp_dir = cli
        .lisp_dir
        .clone()
        .or_else(|| non_empty_env_dir(env_lisp_dir))
        .or_else(|| non_empty_env_dir(env_emacs_loadpath))
        .or(sibling_lisp)
        .map(|p| std::fs::canonicalize(&p).unwrap_or(p));

    let data_dir = cli
        .data_dir
        .clone()
        .or_else(|| non_empty_env_dir(env_data_dir))
        .or_else(|| non_empty_env_dir(env_emacs_data))
        .or(sibling_etc)
        .map(|p| std::fs::canonicalize(&p).unwrap_or(p));

    let sibling_pdmp = exe_dir.map(|d| d.join("femacs-lisp.pdmp"));

    let bootstrap = match &cli.mode {
        RunMode::DumpOut(_) => {
            if lisp_dir.is_none() {
                return Err("cannot locate lisp directory for --dump-out".to_string());
            }
            HostBootstrap::ColdLoadup
        }
        _ if cli.no_dump => {
            if lisp_dir
                .as_ref()
                .is_some_and(|d| d.join("loadup.el").is_file())
            {
                HostBootstrap::ColdLoadup
            } else {
                HostBootstrap::ColdMinimal
            }
        }
        _ if cli.dump_file.is_some() => HostBootstrap::DumpFile(cli.dump_file.clone().unwrap()),
        _ => {
            if let Some(pdmp) = sibling_pdmp.filter(|p| p.is_file()) {
                HostBootstrap::DumpFile(pdmp)
            } else if lisp_dir
                .as_ref()
                .is_some_and(|d| d.join("loadup.el").is_file())
            {
                HostBootstrap::ColdLoadup
            } else {
                return Err(
                    "neither sibling femacs-lisp.pdmp nor loadup.el found near executable"
                        .to_string(),
                );
            }
        }
    };

    Ok(PackagePaths {
        bootstrap,
        lisp_dir,
        data_dir,
    })
}

/// Build `HostConfig` from parsed CLI flags and resolved package directories.
pub fn build_host_config(cli: &ParsedCli, resolved: &PackagePaths) -> HostInitConfig {
    let mut cfg = match &resolved.bootstrap {
        HostBootstrap::ColdMinimal => HostInitConfig::cold_minimal(),
        HostBootstrap::ColdLoadup => HostInitConfig::cold_loadup(
            resolved
                .lisp_dir
                .clone()
                .unwrap_or_else(|| PathBuf::from("lisp")),
        ),
        HostBootstrap::DumpFile(path) => {
            let mut c = HostInitConfig::with_dump(path.clone());
            if let Some(lisp) = &resolved.lisp_dir {
                c = c.with_load_path(lisp.clone()).with_replace_load_path(true);
            }
            c
        }
    };
    cfg = cfg.with_program_name("femacs-lisp");
    if matches!(cli.mode, RunMode::DumpOut(_)) {
        cfg = cfg.with_defer_delayed_custom_init(true);
    }
    if let Some(data_dir) = &resolved.data_dir {
        cfg = cfg.with_data_dir(data_dir.clone());
    }
    cfg
}

/// Resolve paths using the current OS process's `current_exe` and environment.
pub fn resolve_process_paths(cli: &ParsedCli) -> Result<PackagePaths, String> {
    let exe = env::current_exe()
        .ok()
        .map(|p| std::fs::canonicalize(&p).unwrap_or(p));
    resolve_package_paths(
        exe.as_deref(),
        cli,
        env::var_os("FEMACS_LISP_DIR"),
        env::var_os("EMACSLOADPATH"),
        env::var_os("FEMACS_DATA_DIR"),
        env::var_os("EMACSDATA"),
    )
}

/// Detect whether `buf` contains at least one complete (or ready-to-fail-read) Lisp form for the
/// interactive REPL reader loop.
pub fn has_complete_lisp_form(buf: &str) -> bool {
    let bytes = buf.as_bytes();
    let mut i = 0usize;
    let mut depth: isize = 0;
    let mut in_string = false;
    let mut in_comment = false;
    let mut saw_token = false;

    while i < bytes.len() {
        let b = bytes[i];
        if in_comment {
            if b == b'\n' {
                in_comment = false;
            }
            i += 1;
            continue;
        }
        if in_string {
            if b == b'\\' {
                i += 2;
                continue;
            }
            if b == b'"' {
                in_string = false;
            }
            i += 1;
            continue;
        }
        match b {
            b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c => {}
            b';' => {
                in_comment = true;
            }
            b'?' => {
                saw_token = true;
                if i + 1 < bytes.len() && bytes[i + 1] == b'\\' {
                    i += 3;
                    continue;
                }
                i += 2;
                continue;
            }
            b'"' => {
                saw_token = true;
                in_string = true;
            }
            b'(' | b'[' => {
                saw_token = true;
                depth += 1;
            }
            b')' | b']' => {
                saw_token = true;
                depth -= 1;
                if depth <= 0 {
                    return true;
                }
            }
            b'\'' | b'`' | b',' | b'@' | b'#' => {
                saw_token = true;
            }
            _ => {
                saw_token = true;
            }
        }
        i += 1;
    }

    saw_token && depth <= 0 && !in_string
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct RpcRequest {
    pub jsonrpc: String,
    pub id: Value,
    pub method: String,
    #[serde(default)]
    pub params: Value,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct HttpRequestParams {
    pub method: String,
    pub path: String,
    #[serde(default)]
    pub query: String,
    #[serde(default)]
    pub headers: Vec<[String; 2]>,
    #[serde(default)]
    pub body: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct HttpResponseResult {
    pub status: u16,
    #[serde(default)]
    pub headers: Vec<[String; 2]>,
    #[serde(default)]
    pub body: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FrameRead {
    Eof,
    Line(String),
    TooLarge,
    InvalidUtf8,
}

/// Read one `\n`-terminated UTF-8 line bounded strictly by `max_bytes`. Oversized frames drain to
/// `\n` cleanly so framing stays intact on subsequent requests.
pub fn read_bounded_line<R: BufRead>(reader: &mut R, max_bytes: usize) -> std::io::Result<FrameRead> {
    let mut line = Vec::new();
    let mut too_large = false;
    loop {
        let buf = reader.fill_buf()?;
        if buf.is_empty() {
            if too_large {
                return Ok(FrameRead::TooLarge);
            }
            if line.is_empty() {
                return Ok(FrameRead::Eof);
            }
            if line.last() == Some(&b'\r') {
                line.pop();
            }
            return Ok(match String::from_utf8(line) {
                Ok(s) => FrameRead::Line(s),
                Err(_) => FrameRead::InvalidUtf8,
            });
        }

        if let Some(nl_pos) = buf.iter().position(|&b| b == b'\n') {
            let chunk = &buf[..nl_pos];
            if too_large || line.len().saturating_add(chunk.len()).saturating_add(1) > max_bytes {
                reader.consume(nl_pos + 1);
                return Ok(FrameRead::TooLarge);
            }
            line.extend_from_slice(chunk);
            reader.consume(nl_pos + 1);
            if line.last() == Some(&b'\r') {
                line.pop();
            }
            return Ok(match String::from_utf8(line) {
                Ok(s) => FrameRead::Line(s),
                Err(_) => FrameRead::InvalidUtf8,
            });
        }

        let chunk_len = buf.len();
        if !too_large && line.len().saturating_add(chunk_len) <= max_bytes {
            line.extend_from_slice(buf);
        } else {
            too_large = true;
            line.clear();
        }
        reader.consume(chunk_len);
    }
}

pub fn write_rpc_error<W: Write>(
    writer: &mut W,
    id: Value,
    code: i64,
    message: &str,
    data: Option<Value>,
) -> std::io::Result<()> {
    let err_obj = if let Some(d) = data {
        json!({ "code": code, "message": message, "data": d })
    } else {
        json!({ "code": code, "message": message })
    };
    let payload = json!({
        "jsonrpc": "2.0",
        "id": id,
        "error": err_obj,
    });
    serde_json::to_writer(&mut *writer, &payload)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    writer.write_all(b"\n")?;
    writer.flush()
}

pub fn write_rpc_result<W: Write>(
    writer: &mut W,
    id: Value,
    result: &HttpResponseResult,
) -> std::io::Result<()> {
    let payload = json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": result,
    });
    serde_json::to_writer(&mut *writer, &payload)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    writer.write_all(b"\n")?;
    writer.flush()
}

/// Convert `VmHostError` into structured JSON-RPC error payload (`code = -32000`).
pub fn host_error_to_rpc_data(err: &VmHostError) -> Value {
    match err {
        VmHostError::Read {
            symbol,
            message,
            data,
        } => json!({ "kind": "read", "symbol": symbol, "message": message, "data": data }),
        VmHostError::LispCondition {
            symbol,
            message,
            data,
        } => json!({ "kind": "lisp-condition", "symbol": symbol, "message": message, "data": data }),
        VmHostError::Quit { message } => {
            json!({ "kind": "quit", "symbol": "quit", "message": message })
        }
        VmHostError::Io {
            symbol,
            message,
            data,
        } => json!({ "kind": "io", "symbol": symbol, "message": message, "data": data }),
        VmHostError::ExitRequest(code) => {
            json!({ "kind": "exit-request", "code": code, "message": err.to_string() })
        }
        VmHostError::AlreadyInitialized | VmHostError::InitFailed(_) => {
            json!({ "kind": "init", "message": err.to_string() })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use std::os::unix::net::UnixStream;
    use std::sync::atomic::{AtomicU64, Ordering};

    static SEQ: AtomicU64 = AtomicU64::new(0);

    struct ScratchDir(PathBuf);

    impl ScratchDir {
        fn new(label: &str) -> Self {
            let n = SEQ.fetch_add(1, Ordering::Relaxed);
            let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../build")
                .join(format!("host-cli-unit-{label}-{}-{n}", std::process::id()));
            std::fs::create_dir_all(&root).unwrap();
            Self(std::fs::canonicalize(&root).unwrap_or(root))
        }
    }

    impl Drop for ScratchDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn os_args(items: &[&str]) -> Vec<OsString> {
        items.iter().map(OsString::from).collect()
    }

    #[test]
    fn parses_actions_funcall_and_script_tail_args() {
        let cli = parse_cli_args(&os_args(&[
            "--batch",
            "-Q",
            "-L",
            "dir-a",
            "-l",
            "lib-b",
            "-e",
            "(+ 1 2)",
            "-f",
            "custom-batch-fn",
            "-dir",
            "out",
            "--no-reduction",
        ]))
        .expect("funcall batch args parse");
        assert!(cli.batch);
        assert!(cli.quick);
        assert_eq!(cli.mode, RunMode::Actions);
        assert_eq!(
            cli.actions,
            vec![
                BatchAction::AddLoadPath(PathBuf::from("dir-a")),
                BatchAction::Load("lib-b".to_string()),
                BatchAction::Eval("(+ 1 2)".to_string()),
                BatchAction::Funcall {
                    func: "custom-batch-fn".to_string(),
                    args: os_args(&["-dir", "out", "--no-reduction"]),
                },
            ]
        );

        let script = parse_cli_args(&os_args(&[
            "--script",
            "examples/lisp-web/build-site.el",
            "--",
            "--out",
            "site",
        ]))
        .expect("script args parse");
        assert_eq!(
            script.actions,
            vec![BatchAction::Script {
                path: PathBuf::from("examples/lisp-web/build-site.el"),
                args: os_args(&["--out", "site"]),
            }]
        );
    }

    #[test]
    fn rejects_conflicting_or_incomplete_cli_modes() {
        assert_eq!(
            parse_cli_args(&os_args(&["--no-dump", "--dump-file", "a.pdmp"]))
                .unwrap_err()
                .code(),
            2
        );
        assert_eq!(
            parse_cli_args(&os_args(&["--dump-out", "a.pdmp", "--eval", "1"]))
                .unwrap_err()
                .code(),
            2
        );
        assert_eq!(
            parse_cli_args(&os_args(&["--rpc", "ws", "--handler", "h.el"]))
                .unwrap_err()
                .code(),
            2
        );
        assert_eq!(
            parse_cli_args(&os_args(&["--version"])).unwrap_err().code(),
            0
        );
    }

    #[test]
    fn resolves_package_paths_with_override_precedence_and_canonical_symlinks() {
        let scratch = ScratchDir::new("pkg-paths");
        let real_lisp = scratch.0.join("real-lisp");
        let real_etc = scratch.0.join("real-etc");
        let env_lisp = scratch.0.join("env-lisp");
        let pkg = scratch.0.join("pkg");
        let bin = pkg.join("bin");
        std::fs::create_dir_all(&real_lisp).unwrap();
        std::fs::create_dir_all(&real_etc).unwrap();
        std::fs::create_dir_all(&env_lisp).unwrap();
        std::fs::create_dir_all(&bin).unwrap();
        std::fs::write(real_lisp.join("loadup.el"), "()").unwrap();
        std::os::unix::fs::symlink(&real_lisp, pkg.join("lisp")).unwrap();
        std::os::unix::fs::symlink(&real_etc, pkg.join("etc")).unwrap();
        let exe = bin.join("femacs-lisp");
        std::fs::write(&exe, b"bin").unwrap();
        let pdmp = bin.join("femacs-lisp.pdmp");
        std::fs::write(&pdmp, b"DUMP").unwrap();

        let default_cli = parse_cli_args(&os_args(&["--eval", "1"])).unwrap();
        // Sibling resolves symlink to canonical `real-lisp` when no override env is set.
        let resolved_reloc =
            resolve_package_paths(Some(&exe), &default_cli, None, None, None, None).unwrap();
        assert_eq!(resolved_reloc.bootstrap, HostBootstrap::DumpFile(pdmp.clone()));
        assert_eq!(resolved_reloc.lisp_dir, Some(real_lisp.clone()));
        assert_eq!(resolved_reloc.data_dir, Some(real_etc.clone()));

        // Explicit EMACSLOADPATH overrides sibling symlink (critical for bootstrap generators).
        let resolved_env = resolve_package_paths(
            Some(&exe),
            &default_cli,
            None,
            Some(env_lisp.as_os_str().to_os_string()),
            None,
            None,
        )
        .unwrap();
        assert_eq!(resolved_env.lisp_dir, Some(env_lisp.clone()));

        let dump_out_cli = parse_cli_args(&os_args(&["--dump-out", "out.pdmp"])).unwrap();
        let host_cfg = build_host_config(&dump_out_cli, &resolved_reloc);
        assert!(host_cfg.defer_delayed_custom_init);
    }

    #[test]
    fn repl_form_detection_handles_strings_comments_and_nested_exprs() {
        assert!(!has_complete_lisp_form("   \n; (unclosed comment)\n"));
        assert!(!has_complete_lisp_form("(let ((s \"unterminated )"));
        assert!(!has_complete_lisp_form("(defun f (x)\n  (+ x 1)"));
        assert!(has_complete_lisp_form("(defun f (x)\n  (+ x 1))\n"));
        assert!(has_complete_lisp_form("(vector ?\\( ?\\\" \"a)b\")"));
        assert!(has_complete_lisp_form("; heading comment\n(+ 1 2)\n"));
    }

    #[test]
    fn bounded_line_reader_drains_oversized_frame_without_corrupting_next() {
        let payload = format!("{}\nvalid-next\r\n", "x".repeat(40));
        let mut reader = Cursor::new(payload.into_bytes());
        assert_eq!(read_bounded_line(&mut reader, 16).unwrap(), FrameRead::TooLarge);
        assert_eq!(
            read_bounded_line(&mut reader, 32).unwrap(),
            FrameRead::Line("valid-next".to_string())
        );
        assert_eq!(read_bounded_line(&mut reader, 32).unwrap(), FrameRead::Eof);

        let mut bad_utf8 = Cursor::new(vec![0xff, 0xfe, b'\n']);
        assert_eq!(
            read_bounded_line(&mut bad_utf8, 32).unwrap(),
            FrameRead::InvalidUtf8
        );
    }

    #[test]
    fn rpc_error_mapping_and_socket_fd_checks_preserve_safety_invariants() {
        let exit_json = host_error_to_rpc_data(&VmHostError::ExitRequest(7));
        assert_eq!(exit_json["kind"], "exit-request");
        assert_eq!(exit_json["code"], 7);

        let err = crate::adopt_inherited_rpc_socket(Some(-1)).unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);

        let (sock_a, _sock_b) = UnixStream::pair().unwrap();
        assert!(crate::stream_has_cloexec(&sock_a).unwrap());
    }

    #[test]
    fn parses_batch_load_with_trailing_extra_args_and_rejects_unbacked_unknown_flags() {
        let cli = parse_cli_args(&os_args(&[
            "--batch",
            "-Q",
            "-l",
            "tools/backends/bootstrap.el",
            "--stage0",
            "--output",
            "dist/bootstrap/stage0",
        ]))
        .expect("batch load with trailing args should defer arg consumption");
        assert!(cli.batch);
        assert!(cli.quick);
        assert_eq!(cli.mode, RunMode::Actions);
        assert_eq!(
            cli.actions,
            vec![
                BatchAction::Load("tools/backends/bootstrap.el".to_string()),
                BatchAction::ExtraArg(OsString::from("--stage0")),
                BatchAction::ExtraArg(OsString::from("--output")),
                BatchAction::ExtraArg(OsString::from("dist/bootstrap/stage0")),
            ]
        );

        let dashdash = parse_cli_args(&os_args(&[
            "--batch", "-Q", "-l", "a.el", "--", "arg1", "-f",
        ]))
        .expect("batch -- after consumer passes rest verbatim");
        assert_eq!(
            dashdash.actions,
            vec![
                BatchAction::Load("a.el".to_string()),
                BatchAction::ExtraArg(OsString::from("--")),
                BatchAction::ExtraArg(OsString::from("arg1")),
                BatchAction::ExtraArg(OsString::from("-f")),
            ]
        );

        // Unknown flags before any -l/-e consumer, or with --repl/--dump-out, stay exit 2.
        assert_eq!(
            parse_cli_args(&os_args(&["--batch", "--no-such-flag-c3"]))
                .unwrap_err()
                .code(),
            2
        );
        assert_eq!(
            parse_cli_args(&os_args(&["--batch", "--", "--bad"]))
                .unwrap_err()
                .code(),
            2
        );
        assert_eq!(
            parse_cli_args(&os_args(&["--batch", "--repl", "-l", "a.el", "--extra"]))
                .unwrap_err()
                .code(),
            2
        );
        assert_eq!(
            parse_cli_args(&os_args(&["--batch", "--dump-out", "a.pdmp", "-l", "a.el", "--extra"]))
                .unwrap_err()
                .code(),
            2
        );

        // actions_to_argv and parse_remaining_batch_actions roundtrip and reject unconsumed head flags.
        let tail_actions = parse_remaining_batch_actions(&os_args(&[
            "-L", "compiler", "-l", "b.el", "--stage0",
        ]))
        .expect("remaining consumer with trailing arg reparses cleanly");
        assert_eq!(
            tail_actions,
            vec![
                BatchAction::AddLoadPath(PathBuf::from("compiler")),
                BatchAction::Load("b.el".to_string()),
                BatchAction::ExtraArg(OsString::from("--stage0")),
            ]
        );
        assert_eq!(
            actions_to_argv(&tail_actions),
            os_args(&["-L", "compiler", "-l", "b.el", "--stage0"])
        );
        assert_eq!(
            parse_remaining_batch_actions(&os_args(&["--unconsumed-head"]))
                .unwrap_err()
                .code(),
            2
        );
        assert_eq!(
            parse_remaining_batch_actions(&os_args(&["--"]))
                .unwrap_err()
                .code(),
            2
        );
    }
}
