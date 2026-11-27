#![forbid(unsafe_code)]
//! Standalone general-purpose CLI parser, subcommand dispatch, REPL, and batch runner.

use std::ffi::OsString;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use lisp_vm::host_runtime::adopt_inherited_rpc_socket;

use crate::brand;
use crate::engine::{BootstrapMode, Engine, EngineConfig};
use crate::error::Error;
use crate::exec;
use crate::repl;
use crate::rpc::{DEFAULT_MAX_FRAME_BYTES, DEFAULT_RPC_ENTRY, serve_http_rpc};
use crate::stdlib_cache::{
    atomic_temp_dump_path, clean_cache, maybe_spawn_background_warm, repl_history_path,
    resolve_cache_root, try_acquire_warm_lock,
};

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
pub enum CacheAction {
    Path,
    Clean,
    Warm {
        dump_out: Option<PathBuf>,
        quiet: bool,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Command {
    FastExit { code: i32, stdout: String },
    Exec(Vec<String>),
    Cache {
        cache_dir: Option<PathBuf>,
        action: CacheAction,
    },
    Eval {
        exprs: Vec<String>,
        options: CommonOptions,
    },
    Run {
        file: PathBuf,
        script_args: Vec<OsString>,
        options: CommonOptions,
    },
    Repl {
        options: CommonOptions,
        actions: Vec<BatchAction>,
    },
    Compile {
        strict: bool,
        files: Vec<PathBuf>,
        options: CommonOptions,
    },
    Test {
        files: Vec<PathBuf>,
        options: CommonOptions,
    },
    RpcHttp {
        handler: PathBuf,
        entry: String,
        fd: Option<i32>,
        max_frame_bytes: usize,
        options: CommonOptions,
        actions: Vec<BatchAction>,
    },
    DumpOut {
        out_path: PathBuf,
        options: CommonOptions,
        actions: Vec<BatchAction>,
    },
    Batch {
        batch: bool,
        options: CommonOptions,
        actions: Vec<BatchAction>,
    },
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CommonOptions {
    pub cache_dir: Option<PathBuf>,
    pub no_dump: bool,
    pub no_auto_warm: bool,
    pub verbose: bool,
    pub lisp_dir: Option<PathBuf>,
    pub data_dir: Option<PathBuf>,
    pub dump_file: Option<PathBuf>,
    pub load_paths: Vec<PathBuf>,
    pub libraries: Vec<String>,
}

const HELP_TEXT: &str = "\
lisp-core 0.0.1 - Standalone general-purpose Lisp runtime

USAGE:
  lisp-core <SUBCOMMAND> [OPTIONS]
  lisp-core <FILE> [ARGS...]
  lisp-core [BATCH_FLAGS...]

SUBCOMMANDS:
  eval <EXPR...>              Evaluate expression(s) and print the final value
  run <FILE> [ARGS...]        Run a Lisp script file (strips leading `#!` shebang)
  repl                        Start an interactive read-eval-print loop on stdin
  compile [--strict] <FILES>  Byte-compile files (`--strict` fails on warnings)
  test <FILES...>             Load ERT test files and run tests in batch mode
  cache path|warm|clean       Manage the unpacked stdlib and `.pdmp` runtime cache
  exec supervise|mem-tree|digest ...  Run process supervisor, memory accounting, or SHA-256

COMMON OPTIONS:
  --cache-dir <DIR>           Override runtime stdlib / `.pdmp` cache directory
  --no-dump                   Cold-boot without reading cached portable dump
  --no-auto-warm              Disable detached background `.pdmp` warm caching
  --verbose                   Emit bootstrap load progress diagnostics to stderr
  -L, --directory <DIR>       Prepend DIR to load-path
  -l, --load <FILE>           Load Lisp library before subcommand
  --rpc http                  Start JSON-RPC 2.0 `http.handle` worker loop
  -h, --help                  Print this help text
  -V, --version               Print version information

BATCH COMPATIBILITY FLAGS:
  --batch, -Q, --quick, -e/--eval <EXPR>, -f/--funcall <FN>, --script <FILE>,
  --repl, --lisp-dir <DIR>, --data-dir <DIR>, --dump-file <PATH>, --dump-out <PATH>";

pub fn parse_args(args: &[OsString]) -> Result<Command, (i32, String)> {
    if args.is_empty() {
        return Ok(Command::Repl {
            options: CommonOptions::default(),
            actions: Vec::new(),
        });
    }

    let first = args[0].to_string_lossy();
    match first.as_ref() {
        "-V" | "--version" => {
            if args.len() > 1 {
                return Err((2, format!("unexpected argument after {first}")));
            }
            return Ok(Command::FastExit {
                code: 0,
                stdout: format!("{} {}", brand::CLI_NAME, brand::VERSION),
            });
        }
        "-h" | "--help" => {
            return Ok(Command::FastExit {
                code: 0,
                stdout: HELP_TEXT.to_string(),
            });
        }
        "exec" => {
            let rest = args[1..]
                .iter()
                .map(|s| s.to_string_lossy().into_owned())
                .collect();
            return Ok(Command::Exec(rest));
        }
        "cache" => return parse_cache_subcommand(&args[1..]),
        "eval" => return parse_eval_subcommand(&args[1..]),
        "run" => return parse_run_subcommand(&args[1..]),
        "repl" => return parse_repl_subcommand(&args[1..]),
        "compile" => return parse_compile_subcommand(&args[1..]),
        "test" => return parse_test_subcommand(&args[1..]),
        _ => {}
    }

    if !first.starts_with('-') {
        let p = PathBuf::from(&args[0]);
        if p.exists()
            || first.ends_with(".el")
            || first.ends_with(".lisp")
            || first.contains('/')
        {
            return Ok(Command::Run {
                file: p,
                script_args: args[1..].to_vec(),
                options: CommonOptions::default(),
            });
        }
        return Err((
            2,
            format!("unknown subcommand or missing script '{first}' (try `--help`)"),
        ));
    }

    if first.starts_with('-') {
        let mut lead_opts = CommonOptions::default();
        let mut idx = 0usize;
        while idx < args.len() {
            let cur = args[idx].to_string_lossy();
            match parse_common_flag(&cur, args, &mut idx, &mut lead_opts) {
                Ok(true) => idx += 1,
                _ => break,
            }
        }
        if let Some(sub_os) = args.get(idx) {
            let sub = sub_os.to_string_lossy();
            let rest = &args[idx + 1..];
            match sub.as_ref() {
                "cache" => {
                    let mut cmd = parse_cache_subcommand(rest)?;
                    if let Command::Cache { ref mut cache_dir, .. } = cmd {
                        if cache_dir.is_none() {
                            *cache_dir = lead_opts.cache_dir;
                        }
                    }
                    return Ok(cmd);
                }
                "eval" => {
                    let mut cmd = parse_eval_subcommand(rest)?;
                    if let Command::Eval { ref mut options, .. } = cmd {
                        merge_common_options(options, &lead_opts);
                    }
                    return Ok(cmd);
                }
                "run" => {
                    let mut cmd = parse_run_subcommand(rest)?;
                    if let Command::Run { ref mut options, .. } = cmd {
                        merge_common_options(options, &lead_opts);
                    }
                    return Ok(cmd);
                }
                "repl" => {
                    let mut cmd = parse_repl_subcommand(rest)?;
                    if let Command::Repl { ref mut options, .. } = cmd {
                        merge_common_options(options, &lead_opts);
                    }
                    return Ok(cmd);
                }
                "compile" => {
                    let mut cmd = parse_compile_subcommand(rest)?;
                    if let Command::Compile { ref mut options, .. } = cmd {
                        merge_common_options(options, &lead_opts);
                    }
                    return Ok(cmd);
                }
                "test" => {
                    let mut cmd = parse_test_subcommand(rest)?;
                    if let Command::Test { ref mut options, .. } = cmd {
                        merge_common_options(options, &lead_opts);
                    }
                    return Ok(cmd);
                }
                _ => {}
            }
        }
        return parse_legacy_flags(args);
    }

    parse_legacy_flags(args)
}

fn merge_common_options(dst: &mut CommonOptions, lead: &CommonOptions) {
    if dst.cache_dir.is_none() {
        dst.cache_dir = lead.cache_dir.clone();
    }
    dst.no_dump |= lead.no_dump;
    dst.no_auto_warm |= lead.no_auto_warm;
    dst.verbose |= lead.verbose;
    if dst.lisp_dir.is_none() {
        dst.lisp_dir = lead.lisp_dir.clone();
    }
    if dst.data_dir.is_none() {
        dst.data_dir = lead.data_dir.clone();
    }
    if dst.dump_file.is_none() {
        dst.dump_file = lead.dump_file.clone();
    }
    let mut combined_lp = lead.load_paths.clone();
    combined_lp.extend(dst.load_paths.iter().cloned());
    dst.load_paths = combined_lp;
    let mut combined_libs = lead.libraries.clone();
    combined_libs.extend(dst.libraries.iter().cloned());
    dst.libraries = combined_libs;
}

fn take_val(args: &[OsString], idx: &mut usize, flag: &str) -> Result<OsString, (i32, String)> {
    *idx += 1;
    args.get(*idx)
        .cloned()
        .ok_or_else(|| (2, format!("option {flag} requires a value")))
}

fn take_str(args: &[OsString], idx: &mut usize, flag: &str) -> Result<String, (i32, String)> {
    Ok(take_val(args, idx, flag)?.to_string_lossy().into_owned())
}

fn parse_common_flag(
    flag: &str,
    args: &[OsString],
    idx: &mut usize,
    opts: &mut CommonOptions,
) -> Result<bool, (i32, String)> {
    match flag {
        "--cache-dir" => {
            opts.cache_dir = Some(PathBuf::from(take_val(args, idx, flag)?));
            Ok(true)
        }
        "--no-dump" => {
            opts.no_dump = true;
            Ok(true)
        }
        "--no-auto-warm" => {
            opts.no_auto_warm = true;
            Ok(true)
        }
        "--verbose" => {
            opts.verbose = true;
            Ok(true)
        }
        "-L" | "--directory" => {
            opts.load_paths.push(PathBuf::from(take_val(args, idx, flag)?));
            Ok(true)
        }
        "-l" | "--load" => {
            opts.libraries.push(take_str(args, idx, flag)?);
            Ok(true)
        }
        "--lisp-dir" => {
            opts.lisp_dir = Some(PathBuf::from(take_val(args, idx, flag)?));
            Ok(true)
        }
        "--data-dir" => {
            opts.data_dir = Some(PathBuf::from(take_val(args, idx, flag)?));
            Ok(true)
        }
        "--dump-file" => {
            opts.dump_file = Some(PathBuf::from(take_val(args, idx, flag)?));
            Ok(true)
        }
        _ => Ok(false),
    }
}

fn parse_cache_subcommand(args: &[OsString]) -> Result<Command, (i32, String)> {
    let mut sub: Option<String> = None;
    let mut cache_dir: Option<PathBuf> = None;
    let mut dump_out: Option<PathBuf> = None;
    let mut quiet = false;
    let mut idx = 0usize;
    while idx < args.len() {
        let s = args[idx].to_string_lossy();
        match s.as_ref() {
            "--cache-dir" => cache_dir = Some(PathBuf::from(take_val(args, &mut idx, "--cache-dir")?)),
            "--dump-out" => dump_out = Some(PathBuf::from(take_val(args, &mut idx, "--dump-out")?)),
            "--quiet" | "-q" => quiet = true,
            "-h" | "--help" => {
                return Ok(Command::FastExit {
                    code: 0,
                    stdout: "usage: lisp-core cache <path|warm|clean> [--cache-dir DIR] [--dump-out PATH] [--quiet]".to_string(),
                });
            }
            arg if arg.starts_with('-') => return Err((2, format!("unknown cache option: {arg}"))),
            word if sub.is_none() => sub = Some(word.to_string()),
            other => return Err((2, format!("unexpected argument to cache: {other}"))),
        }
        idx += 1;
    }
    let action = match sub.as_deref() {
        Some("path") => CacheAction::Path,
        Some("clean") => CacheAction::Clean,
        Some("warm") => CacheAction::Warm { dump_out, quiet },
        Some(other) => return Err((2, format!("unknown cache subcommand '{other}'; expected path, warm, or clean"))),
        None => return Err((2, "usage: lisp-core cache <path|warm|clean>".to_string())),
    };
    Ok(Command::Cache { cache_dir, action })
}

fn parse_eval_subcommand(args: &[OsString]) -> Result<Command, (i32, String)> {
    let mut options = CommonOptions::default();
    let mut exprs = Vec::new();
    let mut idx = 0usize;
    while idx < args.len() {
        let cur = args[idx].to_string_lossy();
        if cur == "--" {
            for rest in &args[idx + 1..] {
                exprs.push(rest.to_string_lossy().into_owned());
            }
            break;
        }
        if parse_common_flag(&cur, args, &mut idx, &mut options)? {
            idx += 1;
            continue;
        }
        if cur == "-h" || cur == "--help" {
            return Ok(Command::FastExit {
                code: 0,
                stdout: "usage: lisp-core eval [--no-dump] [-L DIR] [-l FILE] <EXPR...>".to_string(),
            });
        }
        if cur.starts_with('-') {
            return Err((2, format!("unknown option for eval: {cur}")));
        }
        exprs.push(cur.into_owned());
        idx += 1;
    }
    if exprs.is_empty() {
        return Err((2, "eval requires at least one expression".to_string()));
    }
    Ok(Command::Eval { exprs, options })
}

fn parse_run_subcommand(args: &[OsString]) -> Result<Command, (i32, String)> {
    let mut options = CommonOptions::default();
    let mut idx = 0usize;
    while idx < args.len() {
        let cur = args[idx].to_string_lossy();
        if cur == "--" {
            idx += 1;
            break;
        }
        if parse_common_flag(&cur, args, &mut idx, &mut options)? {
            idx += 1;
            continue;
        }
        if cur == "-h" || cur == "--help" {
            return Ok(Command::FastExit {
                code: 0,
                stdout: "usage: lisp-core run [--no-dump] [-L DIR] [-l FILE] <FILE> [ARGS...]".to_string(),
            });
        }
        if cur.starts_with('-') {
            return Err((2, format!("unknown option for run: {cur}")));
        }
        break;
    }
    let Some(file_os) = args.get(idx) else {
        return Err((2, "run requires a script file path".to_string()));
    };
    Ok(Command::Run {
        file: PathBuf::from(file_os),
        script_args: args[idx + 1..].to_vec(),
        options,
    })
}

fn parse_repl_subcommand(args: &[OsString]) -> Result<Command, (i32, String)> {
    let mut options = CommonOptions::default();
    let mut idx = 0usize;
    while idx < args.len() {
        let cur = args[idx].to_string_lossy();
        if parse_common_flag(&cur, args, &mut idx, &mut options)? {
            idx += 1;
            continue;
        }
        if cur == "-h" || cur == "--help" {
            return Ok(Command::FastExit {
                code: 0,
                stdout: "usage: lisp-core repl [--no-dump] [-L DIR] [-l FILE]".to_string(),
            });
        }
        return Err((2, format!("unknown option for repl: {cur}")));
    }
    Ok(Command::Repl {
        options,
        actions: Vec::new(),
    })
}

fn parse_compile_subcommand(args: &[OsString]) -> Result<Command, (i32, String)> {
    let mut options = CommonOptions::default();
    let mut strict = false;
    let mut files = Vec::new();
    let mut idx = 0usize;
    while idx < args.len() {
        let cur = args[idx].to_string_lossy();
        if cur == "--" {
            for rest in &args[idx + 1..] {
                files.push(PathBuf::from(rest));
            }
            break;
        }
        if cur == "--strict" {
            strict = true;
            idx += 1;
            continue;
        }
        if parse_common_flag(&cur, args, &mut idx, &mut options)? {
            idx += 1;
            continue;
        }
        if cur == "-h" || cur == "--help" {
            return Ok(Command::FastExit {
                code: 0,
                stdout: "usage: lisp-core compile [--strict] [-L DIR] [-l FILE] <FILES...>".to_string(),
            });
        }
        if cur.starts_with('-') {
            return Err((2, format!("unknown option for compile: {cur}")));
        }
        files.push(PathBuf::from(&args[idx]));
        idx += 1;
    }
    if files.is_empty() {
        return Err((2, "compile requires at least one source file".to_string()));
    }
    Ok(Command::Compile {
        strict,
        files,
        options,
    })
}

fn parse_test_subcommand(args: &[OsString]) -> Result<Command, (i32, String)> {
    let mut options = CommonOptions::default();
    let mut files = Vec::new();
    let mut idx = 0usize;
    while idx < args.len() {
        let cur = args[idx].to_string_lossy();
        if cur == "--" {
            for rest in &args[idx + 1..] {
                files.push(PathBuf::from(rest));
            }
            break;
        }
        if parse_common_flag(&cur, args, &mut idx, &mut options)? {
            idx += 1;
            continue;
        }
        if cur == "-h" || cur == "--help" {
            return Ok(Command::FastExit {
                code: 0,
                stdout: "usage: lisp-core test [-L DIR] [-l FILE] <FILES...>".to_string(),
            });
        }
        if cur.starts_with('-') {
            return Err((2, format!("unknown option for test: {cur}")));
        }
        files.push(PathBuf::from(&args[idx]));
        idx += 1;
    }
    if files.is_empty() {
        return Err((2, "test requires at least one ERT test file".to_string()));
    }
    Ok(Command::Test { files, options })
}

fn parse_legacy_flags(args: &[OsString]) -> Result<Command, (i32, String)> {
    let mut options = CommonOptions::default();
    let mut actions = Vec::new();
    let mut batch = false;
    let mut repl = false;
    let mut dump_out: Option<PathBuf> = None;
    let mut rpc_mode: Option<String> = None;
    let mut rpc_handler: Option<PathBuf> = None;
    let mut rpc_entry = DEFAULT_RPC_ENTRY.to_string();
    let mut rpc_fd: Option<i32> = None;
    let mut max_frame_bytes = DEFAULT_MAX_FRAME_BYTES;

    let mut idx = 0usize;
    while idx < args.len() {
        let raw = &args[idx];
        let s = raw.to_string_lossy();
        if let Some(form) = s.strip_prefix("--eval=") {
            actions.push(BatchAction::Eval(form.to_string()));
            idx += 1;
            continue;
        }
        if let Some(file) = s.strip_prefix("--load=") {
            actions.push(BatchAction::Load(file.to_string()));
            idx += 1;
            continue;
        }
        if let Some(dir) = s.strip_prefix("--directory=") {
            actions.push(BatchAction::AddLoadPath(PathBuf::from(dir)));
            idx += 1;
            continue;
        }
        if let Some(func) = s.strip_prefix("--funcall=") {
            actions.push(BatchAction::Funcall {
                func: func.to_string(),
                args: Vec::new(),
            });
            idx += 1;
            continue;
        }

        match s.as_ref() {
            "--batch" | "-batch" | "-Q" | "--quick" | "-q" | "--no-init-file" | "--no-site-file" => {
                if matches!(s.as_ref(), "--batch" | "-batch") {
                    batch = true;
                }
            }
            "--repl" => repl = true,
            "--no-dump" => options.no_dump = true,
            "--no-auto-warm" => options.no_auto_warm = true,
            "--verbose" => options.verbose = true,
            "--cache-dir" => options.cache_dir = Some(PathBuf::from(take_val(args, &mut idx, "--cache-dir")?)),
            "--lisp-dir" => options.lisp_dir = Some(PathBuf::from(take_val(args, &mut idx, "--lisp-dir")?)),
            "--data-dir" => options.data_dir = Some(PathBuf::from(take_val(args, &mut idx, "--data-dir")?)),
            "--dump-file" => options.dump_file = Some(PathBuf::from(take_val(args, &mut idx, "--dump-file")?)),
            "--dump-out" => dump_out = Some(PathBuf::from(take_val(args, &mut idx, "--dump-out")?)),
            "-L" | "--directory" => {
                let d = PathBuf::from(take_val(args, &mut idx, &s)?);
                options.load_paths.push(d.clone());
                actions.push(BatchAction::AddLoadPath(d));
            }
            "-l" | "--load" => {
                actions.push(BatchAction::Load(take_str(args, &mut idx, &s)?));
            }
            "-e" | "--eval" => {
                actions.push(BatchAction::Eval(take_str(args, &mut idx, &s)?));
            }
            "-f" | "--funcall" => {
                actions.push(BatchAction::Funcall {
                    func: take_str(args, &mut idx, &s)?,
                    args: Vec::new(),
                });
            }
            "--script" => {
                let path = PathBuf::from(take_val(args, &mut idx, "--script")?);
                let rest = args[idx + 1..].to_vec();
                actions.push(BatchAction::Script { path, args: rest });
                break;
            }
            "--rpc" => rpc_mode = Some(take_str(args, &mut idx, "--rpc")?),
            "--rpc-handler" => rpc_handler = Some(PathBuf::from(take_val(args, &mut idx, "--rpc-handler")?)),
            "--rpc-entry" => rpc_entry = take_str(args, &mut idx, "--rpc-entry")?,
            "--rpc-fd" => {
                let raw_fd = take_str(args, &mut idx, "--rpc-fd")?;
                let parsed = raw_fd
                    .parse::<i32>()
                    .map_err(|e| (2, format!("invalid --rpc-fd '{raw_fd}': {e}")))?;
                rpc_fd = Some(parsed);
            }
            "--rpc-max-frame-bytes" => {
                let raw_sz = take_str(args, &mut idx, "--rpc-max-frame-bytes")?;
                let parsed = raw_sz
                    .parse::<usize>()
                    .map_err(|e| (2, format!("invalid --rpc-max-frame-bytes '{raw_sz}': {e}")))?;
                if parsed < 256 {
                    return Err((2, "--rpc-max-frame-bytes must be >= 256".to_string()));
                }
                max_frame_bytes = parsed;
            }
            "-h" | "--help" => {
                return Ok(Command::FastExit {
                    code: 0,
                    stdout: HELP_TEXT.to_string(),
                });
            }
            "-V" | "--version" => {
                return Ok(Command::FastExit {
                    code: 0,
                    stdout: format!("lisp-core {}", env!("CARGO_PKG_VERSION")),
                });
            }
            "--" => {
                for extra in &args[idx + 1..] {
                    actions.push(BatchAction::ExtraArg(extra.clone()));
                }
                break;
            }
            _ if s.starts_with('-') => {
                return Err((2, format!("unknown option: {s}")));
            }
            _ => {
                actions.push(BatchAction::ExtraArg(raw.clone()));
            }
        }
        idx += 1;
    }

    if let Some(mode) = rpc_mode {
        if mode != "http" {
            return Err((2, format!("unsupported --rpc mode '{mode}'; expected 'http'")));
        }
        let handler = rpc_handler.ok_or_else(|| (2, "--rpc http requires --rpc-handler <PATH>".to_string()))?;
        return Ok(Command::RpcHttp {
            handler,
            entry: rpc_entry,
            fd: rpc_fd,
            max_frame_bytes,
            options,
            actions,
        });
    }
    if let Some(out_path) = dump_out {
        return Ok(Command::DumpOut {
            out_path,
            options,
            actions,
        });
    }
    if repl {
        return Ok(Command::Repl { options, actions });
    }
    Ok(Command::Batch {
        batch,
        options,
        actions,
    })
}

/// Convert unconsumed tail arguments returned by Lisp `-l`/`-e` execution into [`BatchAction`]s.
pub fn parse_remaining_batch_actions(args: &[OsString]) -> Result<Vec<BatchAction>, (i32, String)> {
    if args.is_empty() {
        return Ok(Vec::new());
    }
    match parse_legacy_flags(args)? {
        Command::Batch { actions, .. } => Ok(actions),
        _ => Err((2, "unexpected mode switch in trailing command-line-args-left".to_string())),
    }
}

/// Convert [`BatchAction`] items back to raw argv tokens.
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
            BatchAction::Funcall { func, .. } => {
                out.push(OsString::from("-f"));
                out.push(OsString::from(func));
            }
            BatchAction::Script { path, args } => {
                out.push(OsString::from("--script"));
                out.push(path.as_os_str().to_os_string());
                out.extend(args.iter().cloned());
            }
            BatchAction::ExtraArg(arg) => out.push(arg.clone()),
        }
    }
    out
}

fn build_engine_config(options: &CommonOptions) -> EngineConfig {
    let bootstrap = if options.lisp_dir.is_some() || options.dump_file.is_some() || options.data_dir.is_some() {
        BootstrapMode::Custom {
            lisp_dir: options.lisp_dir.clone(),
            boot_script: None,
            data_dir: options.data_dir.clone(),
            dump_file: options.dump_file.clone(),
        }
    } else if options.no_dump {
        BootstrapMode::ColdStdlib
    } else {
        BootstrapMode::AutoCache
    };
    let mut cfg = EngineConfig::default().with_bootstrap(bootstrap);
    if let Some(dir) = &options.cache_dir {
        cfg = cfg.with_cache_dir(dir);
    }
    if options.verbose
        || std::env::var_os(brand::ENV_VERBOSE)
            .is_some_and(|v| !v.is_empty() && v != "0" && v != "false")
    {
        cfg = cfg.with_verbose(true);
    }
    for dir in &options.load_paths {
        cfg = cfg.with_load_path(dir);
    }
    cfg
}

fn maybe_auto_warm(options: &CommonOptions, engine: &mut Engine) {
    if options.no_auto_warm
        || options.no_dump
        || options.lisp_dir.is_some()
        || options.dump_file.is_some()
        || options.data_dir.is_some()
    {
        return;
    }
    if engine.booted_from_dump().unwrap_or(true) {
        return;
    }
    if let Some(layout) = engine.cache_layout() {
        maybe_spawn_background_warm(options.cache_dir.as_deref(), &layout.pdmp_path);
    }
}

fn preload_libraries(engine: &mut Engine, libraries: &[String]) -> Result<(), Error> {
    for lib in libraries {
        engine.load_library(lib)?;
    }
    Ok(())
}

fn exit_for_error(err: &Error) -> i32 {
    if let Error::Exit(code) = err {
        return *code;
    }
    eprintln!("lisp-core: {err}");
    1
}

fn run_batch_actions(
    engine: &mut Engine,
    batch: bool,
    initial_actions: &[BatchAction],
) -> Result<i32, Error> {
    let mut queue = initial_actions.to_vec();
    let mut idx = 0usize;

    while idx < queue.len() {
        let action = queue[idx].clone();
        match action {
            BatchAction::AddLoadPath(_) => {
                let mut run_end = idx + 1;
                while matches!(queue.get(run_end), Some(BatchAction::AddLoadPath(_))) {
                    run_end += 1;
                }
                for pos in (idx..run_end).rev() {
                    if let BatchAction::AddLoadPath(dir) = &queue[pos] {
                        engine.prepend_load_path(dir)?;
                    }
                }
                idx = run_end;
            }
            BatchAction::Load(file) => {
                let tail_argv = actions_to_argv(&queue[idx + 1..]);
                let next_argv = engine.load_library_with_args(&file, &tail_argv)?;
                if next_argv == tail_argv {
                    idx += 1;
                } else {
                    match parse_remaining_batch_actions(&next_argv) {
                        Ok(next_actions) => {
                            queue = next_actions;
                            idx = 0;
                        }
                        Err((code, msg)) => {
                            eprintln!("lisp-core: {msg}");
                            return Ok(code);
                        }
                    }
                }
            }
            BatchAction::Eval(form) => {
                let should_print = !batch
                    && !queue[idx + 1..]
                        .iter()
                        .any(|a| matches!(a, BatchAction::Eval(_)));
                let tail_argv = actions_to_argv(&queue[idx + 1..]);
                let (printed, next_argv) = engine.eval_with_args(&form, should_print, &tail_argv)?;
                if let Some(result) = printed {
                    println!("{result}");
                }
                if next_argv == tail_argv {
                    idx += 1;
                } else {
                    match parse_remaining_batch_actions(&next_argv) {
                        Ok(next_actions) => {
                            queue = next_actions;
                            idx = 0;
                        }
                        Err((code, msg)) => {
                            eprintln!("lisp-core: {msg}");
                            return Ok(code);
                        }
                    }
                }
            }
            BatchAction::Funcall { func, args } => {
                engine.funcall_with_args_left(&func, &args)?;
                idx += 1;
            }
            BatchAction::Script { path, args } => {
                engine.run_script(&path, &args)?;
                break;
            }
            BatchAction::ExtraArg(arg) => {
                eprintln!(
                    "lisp-core: unrecognized option or argument: {}",
                    arg.to_string_lossy()
                );
                return Ok(2);
            }
        }
    }
    let _ = io::stdout().flush();
    Ok(0)
}

fn run_repl_loop(
    engine: &mut Engine,
    actions: &[BatchAction],
    options: &CommonOptions,
) -> Result<i32, Error> {
    for action in actions {
        match action {
            BatchAction::AddLoadPath(dir) => engine.prepend_load_path(dir)?,
            BatchAction::Load(file) => engine.load_library(file)?,
            BatchAction::Eval(_)
            | BatchAction::Funcall { .. }
            | BatchAction::Script { .. }
            | BatchAction::ExtraArg(_) => {}
        }
    }

    let history_path = repl_history_path(&resolve_cache_root(options.cache_dir.as_deref()));
    repl::run_repl(engine, Some(&history_path))
}

/// CLI entrypoint executing parsed arguments and returning a process exit code.
pub fn run_cli(raw_args: &[OsString]) -> i32 {
    let cmd = match parse_args(raw_args) {
        Ok(c) => c,
        Err((code, msg)) => {
            eprintln!("lisp-core: {msg}");
            return code;
        }
    };

    match cmd {
        Command::FastExit { code, stdout } => {
            println!("{stdout}");
            code
        }
        Command::Exec(args) => exec::run_cli(&args),
        Command::Cache {
            cache_dir,
            action: CacheAction::Path,
        } => {
            let root = resolve_cache_root(cache_dir.as_deref());
            println!("{}", root.display());
            0
        }
        Command::Cache {
            cache_dir,
            action: CacheAction::Clean,
        } => {
            let root = resolve_cache_root(cache_dir.as_deref());
            match clean_cache(&root) {
                Ok(()) => 0,
                Err(e) => exit_for_error(&e),
            }
        }
        Command::Cache {
            cache_dir,
            action: CacheAction::Warm { dump_out, quiet },
        } => {
            let mut cfg = EngineConfig::cold_stdlib().with_defer_delayed_custom_init(true);
            if let Some(dir) = cache_dir {
                cfg = cfg.with_cache_dir(dir);
            }
            let mut engine = match Engine::init(cfg) {
                Ok(e) => e,
                Err(err) => return exit_for_error(&err),
            };
            let _lock_guard = match (&dump_out, engine.cache_layout()) {
                (None, Some(layout)) => match try_acquire_warm_lock(&layout.version_root) {
                    Some(g) => Some(g),
                    None => {
                        if !quiet {
                            println!("{}", layout.pdmp_path.display());
                        }
                        return 0;
                    }
                },
                _ => None,
            };
            let target = match dump_out {
                Some(p) => p,
                None => match engine.cache_layout() {
                    Some(layout) => layout.pdmp_path.clone(),
                    None => {
                        eprintln!("lisp-core: missing cache layout for cache warm");
                        return 1;
                    }
                },
            };
            if let Some(parent) = target.parent() {
                if let Err(err) = fs::create_dir_all(parent) {
                    eprintln!("lisp-core: failed to create dump parent dir {}: {err}", parent.display());
                    return 1;
                }
            }
            let tmp_target = atomic_temp_dump_path(&target);
            match engine.dump_image(&tmp_target) {
                Ok(()) => match fs::rename(&tmp_target, &target) {
                    Ok(()) => {
                        if !quiet {
                            println!("{}", target.display());
                        }
                        0
                    }
                    Err(err) => {
                        let _ = fs::remove_file(&tmp_target);
                        eprintln!("lisp-core: failed to finalize dump {}: {err}", target.display());
                        1
                    }
                },
                Err(err) => {
                    let _ = fs::remove_file(&tmp_target);
                    exit_for_error(&err)
                }
            }
        }
        Command::Eval { exprs, options } => {
            let mut engine = match Engine::init(build_engine_config(&options)) {
                Ok(e) => e,
                Err(err) => return exit_for_error(&err),
            };
            maybe_auto_warm(&options, &mut engine);
            if let Err(err) = preload_libraries(&mut engine, &options.libraries) {
                return exit_for_error(&err);
            }
            let mut last = String::new();
            for expr in &exprs {
                match engine.eval_printed(expr) {
                    Ok(s) => last = s,
                    Err(err) => return exit_for_error(&err),
                }
            }
            println!("{last}");
            0
        }
        Command::Run {
            file,
            script_args,
            options,
        } => {
            let mut engine = match Engine::init(build_engine_config(&options)) {
                Ok(e) => e,
                Err(err) => return exit_for_error(&err),
            };
            maybe_auto_warm(&options, &mut engine);
            if let Err(err) = preload_libraries(&mut engine, &options.libraries) {
                return exit_for_error(&err);
            }
            match engine.run_script(&file, &script_args) {
                Ok(()) => 0,
                Err(err) => exit_for_error(&err),
            }
        }
        Command::Repl { options, actions } => {
            let mut engine = match Engine::init(build_engine_config(&options)) {
                Ok(e) => e,
                Err(err) => return exit_for_error(&err),
            };
            maybe_auto_warm(&options, &mut engine);
            if let Err(err) = preload_libraries(&mut engine, &options.libraries) {
                return exit_for_error(&err);
            }
            match run_repl_loop(&mut engine, &actions, &options) {
                Ok(code) => code,
                Err(err) => exit_for_error(&err),
            }
        }
        Command::Compile {
            strict,
            files,
            options,
        } => {
            let mut engine = match Engine::init(build_engine_config(&options)) {
                Ok(e) => e,
                Err(err) => return exit_for_error(&err),
            };
            maybe_auto_warm(&options, &mut engine);
            if let Err(err) = preload_libraries(&mut engine, &options.libraries) {
                return exit_for_error(&err);
            }
            match engine.compile_files(strict, &files) {
                Ok(()) => 0,
                Err(err) => exit_for_error(&err),
            }
        }
        Command::Test { files, options } => {
            let mut engine = match Engine::init(build_engine_config(&options)) {
                Ok(e) => e,
                Err(err) => return exit_for_error(&err),
            };
            maybe_auto_warm(&options, &mut engine);
            if let Err(err) = preload_libraries(&mut engine, &options.libraries) {
                return exit_for_error(&err);
            }
            match engine.run_ert_files(&files) {
                Ok(()) => 0,
                Err(err) => exit_for_error(&err),
            }
        }
        Command::RpcHttp {
            handler,
            entry,
            fd,
            max_frame_bytes,
            options,
            actions,
        } => {
            let stream = match adopt_inherited_rpc_socket(fd) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("lisp-core: failed to adopt inherited RPC socketpair: {e}");
                    return 2;
                }
            };
            let mut engine = match Engine::init(build_engine_config(&options)) {
                Ok(e) => e,
                Err(err) => return exit_for_error(&err),
            };
            for action in &actions {
                match action {
                    BatchAction::AddLoadPath(dir) => {
                        if let Err(e) = engine.prepend_load_path(dir) {
                            return exit_for_error(&e);
                        }
                    }
                    BatchAction::Load(file) => {
                        if let Err(e) = engine.load_library(file) {
                            return exit_for_error(&e);
                        }
                    }
                    BatchAction::Eval(form) => {
                        if let Err(e) = engine.eval_silent(form) {
                            return exit_for_error(&e);
                        }
                    }
                    BatchAction::Funcall { .. } | BatchAction::Script { .. } | BatchAction::ExtraArg(_) => {}
                }
            }
            match serve_http_rpc(&mut engine, stream, &handler, &entry, max_frame_bytes) {
                Ok(code) => code,
                Err(msg) => {
                    eprintln!("lisp-core rpc error: {msg}");
                    1
                }
            }
        }
        Command::DumpOut {
            out_path,
            options,
            actions,
        } => {
            let cfg = build_engine_config(&options).with_defer_delayed_custom_init(true);
            let mut engine = match Engine::init(cfg) {
                Ok(e) => e,
                Err(err) => return exit_for_error(&err),
            };
            if let Err(err) = run_batch_actions(&mut engine, true, &actions) {
                return exit_for_error(&err);
            }
            match engine.dump_image(&out_path) {
                Ok(()) => 0,
                Err(err) => exit_for_error(&err),
            }
        }
        Command::Batch {
            batch,
            options,
            actions,
        } => {
            let mut engine = match Engine::init(build_engine_config(&options)) {
                Ok(e) => e,
                Err(err) => return exit_for_error(&err),
            };
            match run_batch_actions(&mut engine, batch, &actions) {
                Ok(code) => code,
                Err(err) => exit_for_error(&err),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn os(items: &[&str]) -> Vec<OsString> {
        items.iter().map(OsString::from).collect()
    }

    #[test]
    fn parse_subcommands_and_merges_leading_and_trailing_common_flags() {
        let cmd = parse_args(&os(&[
            "--cache-dir",
            "/tmp/lc-cache",
            "-L",
            "/tmp/lib-a",
            "eval",
            "--no-dump",
            "-L",
            "/tmp/lib-b",
            "(+ 1 2)",
        ]))
        .expect("parse eval");
        match cmd {
            Command::Eval { exprs, options } => {
                assert_eq!(exprs, vec!["(+ 1 2)"]);
                assert_eq!(options.cache_dir, Some(PathBuf::from("/tmp/lc-cache")));
                assert!(options.no_dump);
                assert_eq!(
                    options.load_paths,
                    vec![PathBuf::from("/tmp/lib-a"), PathBuf::from("/tmp/lib-b")]
                );
            }
            other => panic!("expected Eval, got {other:?}"),
        }

        let compile = parse_args(&os(&["compile", "--strict", "a.el", "b.el"]))
            .expect("parse compile");
        assert!(matches!(
            compile,
            Command::Compile { strict: true, files, .. }
            if files == vec![PathBuf::from("a.el"), PathBuf::from("b.el")]
        ));

        let run = parse_args(&os(&["run", "script.el", "arg1", "arg2"]))
            .expect("parse run");
        assert!(matches!(
            run,
            Command::Run { file, script_args, .. }
            if file == PathBuf::from("script.el") && script_args == os(&["arg1", "arg2"])
        ));

        let test_cmd = parse_args(&os(&["test", "t.el"])).expect("parse test");
        assert!(matches!(test_cmd, Command::Test { files, .. } if files == vec![PathBuf::from("t.el")]));

        let cache = parse_args(&os(&["cache", "warm", "--quiet"])).expect("parse cache warm quiet");
        assert!(matches!(
            cache,
            Command::Cache {
                action: CacheAction::Warm { quiet: true, .. },
                ..
            }
        ));

        let verbose_eval = parse_args(&os(&["--verbose", "--no-auto-warm", "eval", "1"]))
            .expect("parse verbose eval");
        assert!(matches!(
            verbose_eval,
            Command::Eval {
                options: CommonOptions {
                    verbose: true,
                    no_auto_warm: true,
                    ..
                },
                ..
            }
        ));

        let exec = parse_args(&os(&["exec", "digest", "Cargo.toml"]))
            .expect("parse exec");
        assert_eq!(exec, Command::Exec(vec!["digest".into(), "Cargo.toml".into()]));

        let repl = parse_args(&os(&["repl"])).expect("parse repl");
        assert!(matches!(repl, Command::Repl { .. }));
    }

    #[test]
    fn parse_version_help_and_unknown_flag_errors() {
        let ver = parse_args(&os(&["--version"])).expect("parse --version");
        assert!(matches!(
            ver,
            Command::FastExit { code: 0, stdout } if stdout == format!("lisp-core {}", env!("CARGO_PKG_VERSION"))
        ));

        let help = parse_args(&os(&["--help"])).expect("parse --help");
        assert!(matches!(help, Command::FastExit { code: 0, .. }));

        let err = parse_args(&os(&["--unknown-flag"])).expect_err("unknown flag");
        assert_eq!(err.0, 2);
    }
}
