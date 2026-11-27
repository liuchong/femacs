#![forbid(unsafe_code)]
//! Standalone general-purpose CLI parser, subcommand dispatch, REPL, and batch runner.

use std::ffi::OsString;
use std::fs;
use std::io::{self, Write};
use std::net::TcpListener;
use std::path::PathBuf;

pub const CLI_NAME: &str = "elisp";
#[cfg(feature = "repl-tty")]
pub const REPL_PROMPT: &str = "elisp> ";

#[cfg(unix)]
use lisp::vm::host_runtime::adopt_inherited_rpc_socket;

use lisp::brand;
use lisp::exec;
use lisp::pkg::{
    self, DependencySpec, discover_project_entry, discover_project_load_paths,
    discover_project_tests, find_project_root,
};
use lisp::rpc::{DEFAULT_MAX_FRAME_BYTES, DEFAULT_RPC_ENTRY, RpcTransport, serve_http_rpc};
use lisp::stdlib_cache::{
    atomic_temp_dump_path, clean_cache, maybe_spawn_background_warm, repl_history_path,
    resolve_cache_root, try_acquire_warm_lock,
};
use lisp::{BootstrapMode, ConcurrencyMode, Engine, EngineConfig, Error};

use crate::repl;

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
pub enum PkgAction {
    Add {
        name: String,
        path: Option<String>,
        url: Option<String>,
        version: Option<String>,
        sha256: Option<String>,
    },
    Install,
    List,
    Update,
    Remove { name: String },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Command {
    FastExit { code: i32, stdout: String },
    Exec(Vec<String>),
    Init {
        dir: PathBuf,
        name: Option<String>,
        force: bool,
    },
    Pkg {
        manifest_dir: Option<PathBuf>,
        action: PkgAction,
    },
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
        addr: Option<String>,
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
    pub concurrency: Option<ConcurrencyMode>,
    pub lisp_dir: Option<PathBuf>,
    pub data_dir: Option<PathBuf>,
    pub dump_file: Option<PathBuf>,
    pub load_paths: Vec<PathBuf>,
    pub libraries: Vec<String>,
}

const HELP_TEXT: &str = "\
elisp 0.0.1 - Standalone general-purpose Elisp-compatible Lisp CLI & REPL

USAGE:
  elisp <SUBCOMMAND> [OPTIONS]
  elisp <FILE> [ARGS...]
  elisp [BATCH_FLAGS...]

SUBCOMMANDS:
  init [DIR] [--name NAME]            Scaffold a project (lisp.toml, src/, tests/)
  pkg <add|install|list|update|remove>  Manage dependencies (.lisp-packages/, lisp.lock)
  eval <EXPR...>                      Evaluate expression(s) and print the final value
  run [FILE] [ARGS...]                Run a script (or project entry inside lisp.toml)
  repl                                Start an interactive TTY/pipe read-eval-print loop
  compile [--strict] <FILES...>       Byte-compile files (`--strict` fails on warnings)
  test [FILES...]                     Run ERT test files (or tests/* inside lisp.toml)
  cache path|warm|clean               Manage unpacked stdlib and `.pdmp` warm cache
  exec supervise|mem-tree|digest ...  Run process supervisor, mem-tree, or SHA-256 tools

COMMON OPTIONS:
  --cache-dir <DIR>                   Override runtime cache dir (env: LISP_CACHE_DIR)
  --concurrency <MODE>                VM mode: `no-gil` (default) or `gil` (LISP_CONCURRENCY)
  --no-dump                           Cold-boot without reading cached `.pdmp`
  --no-auto-warm                      Disable detached background `.pdmp` caching (LISP_NO_AUTO_WARM)
  --verbose                           Print bootstrap load diagnostics to stderr (LISP_VERBOSE)
  -L, --directory <DIR>               Prepend DIR to load-path
  -l, --load <FILE>                   Load Lisp library before subcommand
  --rpc http [--rpc-fd FD|--rpc-addr HOST:PORT]  Start JSON-RPC 2.0 `http.handle` worker
  -h, --help                          Print this help text
  -V, --version                       Print version information

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
                stdout: format!("{CLI_NAME} {}", brand::VERSION),
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
        "init" => return parse_init_subcommand(&args[1..]),
        "pkg" => return parse_pkg_subcommand(&args[1..]),
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
            || first.contains('\\')
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

fn parse_concurrency_value(raw: &str) -> Result<ConcurrencyMode, (i32, String)> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "no-gil" | "nogil" => Ok(ConcurrencyMode::NoGil),
        "gil" => Ok(ConcurrencyMode::Gil),
        other => Err((
            2,
            format!("invalid concurrency mode '{other}'; expected 'no-gil' or 'gil'"),
        )),
    }
}

fn merge_common_options(dst: &mut CommonOptions, lead: &CommonOptions) {
    if dst.cache_dir.is_none() {
        dst.cache_dir = lead.cache_dir.clone();
    }
    dst.no_dump |= lead.no_dump;
    dst.no_auto_warm |= lead.no_auto_warm;
    dst.verbose |= lead.verbose;
    if dst.concurrency.is_none() {
        dst.concurrency = lead.concurrency;
    }
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
    if let Some(raw) = flag.strip_prefix("--concurrency=") {
        opts.concurrency = Some(parse_concurrency_value(raw)?);
        return Ok(true);
    }
    match flag {
        "--concurrency" => {
            let raw = take_str(args, idx, flag)?;
            opts.concurrency = Some(parse_concurrency_value(&raw)?);
            Ok(true)
        }
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

fn parse_init_subcommand(args: &[OsString]) -> Result<Command, (i32, String)> {
    let mut dir: Option<PathBuf> = None;
    let mut name: Option<String> = None;
    let mut force = false;
    let mut idx = 0usize;
    while idx < args.len() {
        let s = args[idx].to_string_lossy();
        match s.as_ref() {
            "--name" => name = Some(take_str(args, &mut idx, "--name")?),
            "--force" => force = true,
            "-h" | "--help" => {
                return Ok(Command::FastExit {
                    code: 0,
                    stdout: "usage: elisp init [DIR] [--name NAME] [--force]".to_string(),
                });
            }
            arg if arg.starts_with('-') => {
                return Err((2, format!("unknown init option: {arg}")));
            }
            path_arg if dir.is_none() => dir = Some(PathBuf::from(path_arg)),
            other => return Err((2, format!("unexpected argument to init: {other}"))),
        }
        idx += 1;
    }
    let resolved_dir = match dir {
        Some(d) => d,
        None => std::env::current_dir().map_err(|e| (1, format!("cwd error: {e}")))?,
    };
    Ok(Command::Init {
        dir: resolved_dir,
        name,
        force,
    })
}

fn parse_pkg_subcommand(args: &[OsString]) -> Result<Command, (i32, String)> {
    let mut manifest_dir: Option<PathBuf> = None;
    let mut sub: Option<String> = None;
    let mut name: Option<String> = None;
    let mut path: Option<String> = None;
    let mut url: Option<String> = None;
    let mut version: Option<String> = None;
    let mut sha256: Option<String> = None;
    let mut idx = 0usize;
    while idx < args.len() {
        let s = args[idx].to_string_lossy();
        match s.as_ref() {
            "--manifest-dir" | "--project" => {
                manifest_dir = Some(PathBuf::from(take_val(args, &mut idx, &s)?));
            }
            "--path" => path = Some(take_str(args, &mut idx, "--path")?),
            "--url" => url = Some(take_str(args, &mut idx, "--url")?),
            "--version" => version = Some(take_str(args, &mut idx, "--version")?),
            "--sha256" => sha256 = Some(take_str(args, &mut idx, "--sha256")?),
            "-h" | "--help" => {
                return Ok(Command::FastExit {
                    code: 0,
                    stdout: "usage: elisp pkg [--project DIR] <add <NAME> (--path DIR | --url URL) [--version VER] [--sha256 HEX] | install | list | update | remove <NAME>>".to_string(),
                });
            }
            arg if arg.starts_with('-') => {
                return Err((2, format!("unknown pkg option: {arg}")));
            }
            word if sub.is_none() => sub = Some(word.to_string()),
            word if name.is_none() => name = Some(word.to_string()),
            other => return Err((2, format!("unexpected argument to pkg: {other}"))),
        }
        idx += 1;
    }

    let action = match sub.as_deref() {
        Some("add") => {
            let pkg_name = name.ok_or_else(|| (2, "pkg add requires <NAME>".to_string()))?;
            if path.is_none() && url.is_none() {
                return Err((2, "pkg add requires --path <DIR> or --url <URL>".to_string()));
            }
            PkgAction::Add {
                name: pkg_name,
                path,
                url,
                version,
                sha256,
            }
        }
        Some("install") => {
            if name.is_some() {
                return Err((2, "pkg install takes no positional arguments".to_string()));
            }
            PkgAction::Install
        }
        Some("list") => {
            if name.is_some() {
                return Err((2, "pkg list takes no positional arguments".to_string()));
            }
            PkgAction::List
        }
        Some("update") => {
            if name.is_some() {
                return Err((2, "pkg update takes no positional arguments".to_string()));
            }
            PkgAction::Update
        }
        Some("remove") => {
            let pkg_name = name.ok_or_else(|| (2, "pkg remove requires <NAME>".to_string()))?;
            PkgAction::Remove { name: pkg_name }
        }
        Some(other) => {
            return Err((
                2,
                format!("unknown pkg subcommand '{other}'; expected add, install, list, update, or remove"),
            ));
        }
        None => {
            return Err((
                2,
                "usage: elisp pkg <add|install|list|update|remove>".to_string(),
            ));
        }
    };

    Ok(Command::Pkg {
        manifest_dir,
        action,
    })
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
                    stdout: "usage: elisp cache <path|warm|clean> [--cache-dir DIR] [--dump-out PATH] [--quiet]".to_string(),
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
        None => return Err((2, "usage: elisp cache <path|warm|clean>".to_string())),
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
                stdout: "usage: elisp eval [--no-dump] [-L DIR] [-l FILE] <EXPR...>".to_string(),
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
                stdout: "usage: elisp run [--no-dump] [-L DIR] [-l FILE] <FILE> [ARGS...]".to_string(),
            });
        }
        if cur.starts_with('-') {
            return Err((2, format!("unknown option for run: {cur}")));
        }
        break;
    }
    let (file, script_args) = match args.get(idx) {
        Some(file_os) => (PathBuf::from(file_os), args[idx + 1..].to_vec()),
        None => {
            let entry = std::env::current_dir()
                .ok()
                .and_then(|cwd| discover_project_entry(&cwd))
                .ok_or_else(|| (2, "run requires a script file path".to_string()))?;
            (entry, Vec::new())
        }
    };
    Ok(Command::Run {
        file,
        script_args,
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
                stdout: "usage: elisp repl [--no-dump] [-L DIR] [-l FILE]".to_string(),
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
                stdout: "usage: elisp compile [--strict] [-L DIR] [-l FILE] <FILES...>".to_string(),
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
                stdout: "usage: elisp test [-L DIR] [-l FILE] <FILES...>".to_string(),
            });
        }
        if cur.starts_with('-') {
            return Err((2, format!("unknown option for test: {cur}")));
        }
        files.push(PathBuf::from(&args[idx]));
        idx += 1;
    }
    if files.is_empty() {
        if let Ok(cwd) = std::env::current_dir() {
            files = discover_project_tests(&cwd);
        }
        if files.is_empty() {
            return Err((2, "test requires at least one ERT test file".to_string()));
        }
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
    let mut rpc_addr: Option<String> = None;
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
        if let Some(raw) = s.strip_prefix("--concurrency=") {
            options.concurrency = Some(parse_concurrency_value(raw)?);
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
            "--concurrency" => {
                let raw = take_str(args, &mut idx, "--concurrency")?;
                options.concurrency = Some(parse_concurrency_value(&raw)?);
            }
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
            "--rpc-addr" => {
                rpc_addr = Some(take_str(args, &mut idx, "--rpc-addr")?);
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
                    stdout: format!("elisp {}", env!("CARGO_PKG_VERSION")),
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
            addr: rpc_addr,
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

fn resolve_concurrency(options: &CommonOptions) -> Result<ConcurrencyMode, (i32, String)> {
    if let Some(mode) = options.concurrency {
        return Ok(mode);
    }
    if let Some(raw) = std::env::var_os(brand::ENV_CONCURRENCY) {
        let s = raw.to_string_lossy();
        if !s.trim().is_empty() {
            return parse_concurrency_value(&s);
        }
    }
    Ok(ConcurrencyMode::NoGil)
}

fn build_engine_config(options: &CommonOptions) -> Result<EngineConfig, (i32, String)> {
    let concurrency = resolve_concurrency(options)?;
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
    let mut cfg = EngineConfig::default()
        .with_program_name(CLI_NAME)
        .with_bootstrap(bootstrap)
        .with_concurrency(concurrency);
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
    if options.lisp_dir.is_none() {
        if let Ok(cwd) = std::env::current_dir() {
            for dir in discover_project_load_paths(&cwd) {
                cfg = cfg.with_load_path(dir);
            }
        }
    }
    Ok(cfg)
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
    eprintln!("elisp: {err}");
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
                            eprintln!("elisp: {msg}");
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
                            eprintln!("elisp: {msg}");
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
                    "elisp: unrecognized option or argument: {}",
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
            eprintln!("elisp: {msg}");
            return code;
        }
    };

    match cmd {
        Command::FastExit { code, stdout } => {
            println!("{stdout}");
            code
        }
        Command::Exec(args) => exec::run_cli(&args),
        Command::Init { dir, name, force } => {
            match pkg::init_project(&dir, name.as_deref(), force) {
                Ok(created) => {
                    println!("initialized {}", created.display());
                    0
                }
                Err(err) => exit_for_error(&err),
            }
        }
        Command::Pkg {
            manifest_dir,
            action,
        } => {
            let start = match manifest_dir {
                Some(d) => d,
                None => match std::env::current_dir() {
                    Ok(cwd) => cwd,
                    Err(e) => {
                        eprintln!("elisp: cwd error: {e}");
                        return 1;
                    }
                },
            };
            let Some(root) = find_project_root(&start) else {
                eprintln!(
                    "elisp: no {} found in {} or ancestors",
                    pkg::MANIFEST_FILE,
                    start.display()
                );
                return 2;
            };
            match action {
                PkgAction::Add {
                    name,
                    path,
                    url,
                    version,
                    sha256,
                } => {
                    let spec = DependencySpec {
                        path,
                        url,
                        version,
                        sha256,
                    };
                    match pkg::pkg_add(&root, &name, spec) {
                        Ok(locked) => {
                            println!("added {} v{} ({})", locked.name, locked.version, locked.source);
                            0
                        }
                        Err(err) => exit_for_error(&err),
                    }
                }
                PkgAction::Install => match pkg::pkg_install(&root) {
                    Ok(lock) => {
                        println!("installed {} package(s)", lock.package.len());
                        0
                    }
                    Err(err) => exit_for_error(&err),
                },
                PkgAction::List => match pkg::pkg_list(&root) {
                    Ok(lines) => {
                        for line in lines {
                            println!("{line}");
                        }
                        0
                    }
                    Err(err) => exit_for_error(&err),
                },
                PkgAction::Update => match pkg::pkg_update(&root) {
                    Ok(lock) => {
                        println!("updated {} package(s)", lock.package.len());
                        0
                    }
                    Err(err) => exit_for_error(&err),
                },
                PkgAction::Remove { name } => match pkg::pkg_remove(&root, &name) {
                    Ok(true) => {
                        println!("removed {name}");
                        0
                    }
                    Ok(false) => {
                        eprintln!("elisp: dependency '{name}' was not in {}", pkg::MANIFEST_FILE);
                        1
                    }
                    Err(err) => exit_for_error(&err),
                },
            }
        }
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
            let mut cfg = EngineConfig::cold_stdlib()
                .with_program_name(CLI_NAME)
                .with_defer_delayed_custom_init(true);
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
                        eprintln!("elisp: missing cache layout for cache warm");
                        return 1;
                    }
                },
            };
            if let Some(parent) = target.parent() {
                if let Err(err) = fs::create_dir_all(parent) {
                    eprintln!("elisp: failed to create dump parent dir {}: {err}", parent.display());
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
                        eprintln!("elisp: failed to finalize dump {}: {err}", target.display());
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
            let cfg = match build_engine_config(&options) {
                Ok(c) => c,
                Err((code, msg)) => {
                    eprintln!("elisp: {msg}");
                    return code;
                }
            };
            let mut engine = match Engine::init(cfg) {
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
            let cfg = match build_engine_config(&options) {
                Ok(c) => c,
                Err((code, msg)) => {
                    eprintln!("elisp: {msg}");
                    return code;
                }
            };
            let mut engine = match Engine::init(cfg) {
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
            let cfg = match build_engine_config(&options) {
                Ok(c) => c,
                Err((code, msg)) => {
                    eprintln!("elisp: {msg}");
                    return code;
                }
            };
            let mut engine = match Engine::init(cfg) {
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
            let cfg = match build_engine_config(&options) {
                Ok(c) => c,
                Err((code, msg)) => {
                    eprintln!("elisp: {msg}");
                    return code;
                }
            };
            let mut engine = match Engine::init(cfg) {
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
            let cfg = match build_engine_config(&options) {
                Ok(c) => c,
                Err((code, msg)) => {
                    eprintln!("elisp: {msg}");
                    return code;
                }
            };
            let mut engine = match Engine::init(cfg) {
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
            addr,
            max_frame_bytes,
            options,
            actions,
        } => {
            let transport = if let Some(bind_addr) = addr {
                if fd.is_some() {
                    eprintln!("elisp: --rpc-fd and --rpc-addr cannot be combined");
                    return 2;
                }
                let listener = match TcpListener::bind(&bind_addr) {
                    Ok(l) => l,
                    Err(e) => {
                        eprintln!("elisp: failed to bind RPC listener {bind_addr}: {e}");
                        return 2;
                    }
                };
                match listener.accept() {
                    Ok((tcp_stream, _)) => RpcTransport::Tcp(tcp_stream),
                    Err(e) => {
                        eprintln!("elisp: failed to accept RPC client on {bind_addr}: {e}");
                        return 2;
                    }
                }
            } else {
                #[cfg(unix)]
                {
                    match adopt_inherited_rpc_socket(fd) {
                        Ok(s) => RpcTransport::Unix(s),
                        Err(e) => {
                            eprintln!("elisp: failed to adopt inherited RPC socketpair: {e}");
                            return 2;
                        }
                    }
                }
                #[cfg(not(unix))]
                {
                    let _ = fd;
                    eprintln!(
                        "elisp: inherited Unix RPC socketpair (--rpc-fd) is unsupported on this OS; pass --rpc-addr 127.0.0.1:<PORT>"
                    );
                    return 2;
                }
            };
            let cfg = match build_engine_config(&options) {
                Ok(c) => c,
                Err((code, msg)) => {
                    eprintln!("elisp: {msg}");
                    return code;
                }
            };
            let mut engine = match Engine::init(cfg) {
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
            match serve_http_rpc(&mut engine, transport, &handler, &entry, max_frame_bytes) {
                Ok(code) => code,
                Err(msg) => {
                    eprintln!("elisp rpc error: {msg}");
                    1
                }
            }
        }
        Command::DumpOut {
            out_path,
            options,
            actions,
        } => {
            let cfg = match build_engine_config(&options) {
                Ok(c) => c.with_defer_delayed_custom_init(true),
                Err((code, msg)) => {
                    eprintln!("elisp: {msg}");
                    return code;
                }
            };
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
            let cfg = match build_engine_config(&options) {
                Ok(c) => c,
                Err((code, msg)) => {
                    eprintln!("elisp: {msg}");
                    return code;
                }
            };
            let mut engine = match Engine::init(cfg) {
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
            Command::FastExit { code: 0, stdout } if stdout == format!("elisp {}", env!("CARGO_PKG_VERSION"))
        ));

        let help = parse_args(&os(&["--help"])).expect("parse --help");
        assert!(matches!(help, Command::FastExit { code: 0, .. }));

        let err = parse_args(&os(&["--unknown-flag"])).expect_err("unknown flag");
        assert_eq!(err.0, 2);

        let gil_eval = parse_args(&os(&["--concurrency=gil", "eval", "(+ 1 2)"]))
            .expect("parse --concurrency=gil");
        assert!(matches!(
            gil_eval,
            Command::Eval {
                options: CommonOptions {
                    concurrency: Some(ConcurrencyMode::Gil),
                    ..
                },
                ..
            }
        ));

        let nogil_eval = parse_args(&os(&["eval", "--concurrency", "no-gil", "42"]))
            .expect("parse --concurrency no-gil");
        assert!(matches!(
            nogil_eval,
            Command::Eval {
                options: CommonOptions {
                    concurrency: Some(ConcurrencyMode::NoGil),
                    ..
                },
                ..
            }
        ));

        let bad_conc = parse_args(&os(&["--concurrency=invalid", "eval", "1"]))
            .expect_err("invalid concurrency mode");
        assert_eq!(bad_conc.0, 2);

        let init = parse_args(&os(&["init", "my-proj", "--name", "demo", "--force"]))
            .expect("parse init");
        assert!(matches!(
            init,
            Command::Init { dir, name: Some(n), force: true }
            if dir == PathBuf::from("my-proj") && n == "demo"
        ));

        let pkg_add = parse_args(&os(&[
            "pkg",
            "--project",
            "/tmp/app",
            "add",
            "util",
            "--path",
            "../util",
            "--version",
            "0.2.0",
        ]))
        .expect("parse pkg add");
        assert!(matches!(
            pkg_add,
            Command::Pkg {
                manifest_dir: Some(d),
                action: PkgAction::Add { name, path: Some(p), version: Some(v), .. },
            } if d == PathBuf::from("/tmp/app") && name == "util" && p == "../util" && v == "0.2.0"
        ));

        let pkg_list = parse_args(&os(&["pkg", "list"])).expect("parse pkg list");
        assert!(matches!(pkg_list, Command::Pkg { action: PkgAction::List, .. }));
    }
}
