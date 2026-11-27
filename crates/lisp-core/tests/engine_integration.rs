#![forbid(unsafe_code)]
//! Real-process integration tests for `lisp_core::Engine`, `Value`, `Error`, cache fallback, and CLI.

use std::env;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;

use lisp_core::rpc::{DEFAULT_MAX_FRAME_BYTES, serve_http_rpc};
use lisp_core::stdlib_cache::{ensure_unpacked, exe_pdmp_path, resolve_cache_root};
use lisp_core::{BootstrapMode, Engine, EngineConfig, Error, Value};

const TEST_ENV_KEY: &str = "LISP_CORE_TEST_CASE";

fn scrub_cmd_env(cmd: &mut Command) {
    for var in ["EMACSLOADPATH", "EMACSDATA", "EMACSDOC", "EMACSPATH"] {
        cmd.env_remove(var);
    }
}

fn spawn_child_case(case: &str, cache_dir: &Path) {
    let exe = env::current_exe().expect("resolve test executable");
    let mut cmd = Command::new(exe);
    cmd.env(TEST_ENV_KEY, case)
        .env("LISP_CORE_CACHE_DIR", cache_dir);
    scrub_cmd_env(&mut cmd);
    let out = cmd.output().expect("spawn child test case");
    if !out.status.success() {
        panic!(
            "integration case '{case}' failed (status={:?})\nstdout:\n{}\nstderr:\n{}",
            out.status,
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
    }
}

fn case_engine_minimal_no_charsets() {
    let mut engine = Engine::minimal().expect("init minimal engine without charsets");
    let val = engine.eval("(+ 20 22)").expect("eval 42");
    assert_eq!(val, Value::Int(42));
}

fn case_engine_new_roundtrips_and_call() {
    let mut engine = Engine::new().expect("init default engine");

    assert_eq!(engine.eval("nil").expect("nil"), Value::Nil);
    assert_eq!(engine.eval("t").expect("t"), Value::Bool(true));
    assert_eq!(engine.eval("42").expect("int"), Value::Int(42));
    assert_eq!(engine.eval("2.5").expect("float"), Value::Float(2.5));
    assert_eq!(
        engine.eval("\"hello\"").expect("string"),
        Value::String("hello".to_string())
    );
    assert_eq!(
        engine.eval("(quote sym)").expect("symbol"),
        Value::Symbol("sym".to_string())
    );
    assert_eq!(
        engine.eval("(list 1 \"two\")").expect("list"),
        Value::List(vec![Value::Int(1), Value::String("two".to_string())])
    );
    assert_eq!(
        engine.eval("[10 20]").expect("vector"),
        Value::Vector(vec![Value::Int(10), Value::Int(20)])
    );
    assert_eq!(
        engine.eval("(unibyte-string 0 255 128 1)").expect("bytes"),
        Value::Bytes(vec![0, 255, 128, 1])
    );

    let sum = engine
        .call("+", &[Value::Int(20), Value::Int(22)])
        .expect("call +");
    assert_eq!(sum, Value::Int(42));

    let dup = Engine::new();
    assert!(matches!(dup, Err(Error::AlreadyInitialized)));
}

fn case_engine_register_fn_and_bytes_roundtrip() {
    let mut engine = Engine::new().expect("init engine for register_fn");

    engine
        .register_fn("rust-add-mul", |args| {
            let a = args.first().and_then(Value::as_int).unwrap_or(0);
            let b = args.get(1).and_then(Value::as_int).unwrap_or(0);
            Ok(Value::Int((a + b) * 2))
        })
        .expect("register rust-add-mul");
    assert_eq!(
        engine.eval("(rust-add-mul 10 11)").expect("call host fn"),
        Value::Int(42)
    );

    engine
        .register_fn("rust-xor-bytes", |args| {
            let bytes = args
                .first()
                .and_then(Value::as_bytes)
                .ok_or_else(|| "expected bytes argument".to_string())?;
            let xored: Vec<u8> = bytes.iter().map(|b| b ^ 0xFF).collect();
            Ok(Value::Bytes(xored))
        })
        .expect("register rust-xor-bytes");
    assert_eq!(
        engine
            .eval("(rust-xor-bytes (unibyte-string #x00 #xff #x80 #x01))")
            .expect("bytes roundtrip"),
        Value::Bytes(vec![0xff, 0x00, 0x7f, 0xfe])
    );

    engine
        .register_fn("rust-always-fail", |_args| Err("host boom".to_string()))
        .expect("register rust-always-fail");
    assert_eq!(
        engine
            .eval("(condition-case err (rust-always-fail) (error (cadr err)))")
            .expect("catch host error"),
        Value::String("host boom".to_string())
    );

    let syms = engine.complete_symbols().expect("complete symbols");
    assert!(syms.iter().any(|s| s == "rust-add-mul"));
}

fn case_engine_error_variants() {
    let mut engine = Engine::minimal().expect("init minimal engine");
    assert!(matches!(engine.eval("("), Err(Error::Syntax { .. })));
    assert!(matches!(
        engine.eval("(error \"boom\")"),
        Err(Error::Runtime { .. })
    ));
    assert!(matches!(
        engine.eval("(kill-emacs 7)"),
        Err(Error::Exit(7))
    ));
    assert!(matches!(
        engine.eval("(load \"/nonexistent/lisp-core-missing-file.el\")"),
        Err(Error::Io { .. })
    ));
}

fn case_engine_corrupt_pdmp_fallback_bad_magic() {
    let cache_root = resolve_cache_root(None);
    let layout = ensure_unpacked(&cache_root).expect("unpack stdlib cache");
    let pdmp = exe_pdmp_path(&layout.version_root);
    fs::write(&pdmp, b"BADMAGIC\x00\x00\x00\x00\x00\x00\x00\x00\xff").expect("write bad magic");

    let cfg = EngineConfig::default()
        .with_bootstrap(BootstrapMode::AutoCache)
        .with_cache_dir(&cache_root);
    let mut engine = Engine::init(cfg).expect("init falls back cleanly on bad pdmp magic");
    assert_eq!(engine.eval("(+ 20 22)").expect("eval 42"), Value::Int(42));
}

fn case_engine_corrupt_pdmp_fallback_corrupt_body() {
    let cache_root = resolve_cache_root(None);
    let layout = ensure_unpacked(&cache_root).expect("unpack stdlib cache");
    let pdmp = exe_pdmp_path(&layout.version_root);
    let mut bytes = b"DUMPEDGNUEMACS\x00\x00".to_vec();
    bytes.extend_from_slice(&[0xFF; 128]);
    fs::write(&pdmp, bytes).expect("write valid-magic corrupt-body pdmp");

    let cfg = EngineConfig::default()
        .with_bootstrap(BootstrapMode::AutoCache)
        .with_cache_dir(&cache_root);
    let mut engine = Engine::init(cfg).expect("init falls back cleanly on corrupt pdmp body");
    assert_eq!(engine.eval("(+ 20 22)").expect("eval 42"), Value::Int(42));
}

fn case_engine_rpc_unix_pair() {
    let cache_root = resolve_cache_root(None);
    let scratch = cache_root.join("rpc-scratch");
    fs::create_dir_all(&scratch).expect("create rpc scratch");
    let handler = scratch.join("handler.el");
    fs::write(
        &handler,
        ";;; handler.el -*- lexical-binding: t -*-\n\
         (defun test-http-entry (req)\n\
           (list (cons 'status 200)\n\
                 (cons 'headers '((\"x-rpc\" . \"ok\")))\n\
                 (cons 'body (format \"%s:%s\" (alist-get 'method req) (alist-get 'path req)))))\n",
    )
    .expect("write rpc handler");

    let mut engine = Engine::new().expect("init engine for rpc");
    let (srv, mut cli) = UnixStream::pair().expect("UnixStream::pair");

    let client_thread = thread::spawn(move || {
        cli.write_all(
            br#"{"jsonrpc":"2.0","id":9,"method":"http.handle","params":{"method":"GET","path":"/ok","query":"","headers":[],"body":"","requestId":"r9"}}"#
                .as_slice(),
        )
        .expect("write request");
        cli.write_all(b"\n").expect("write newline");
        cli.flush().expect("flush");
        let mut line = String::new();
        let mut reader = BufReader::new(&cli);
        reader.read_line(&mut line).expect("read rpc response");
        line
    });

    let exit_code = serve_http_rpc(
        &mut engine,
        srv,
        &handler,
        "test-http-entry",
        DEFAULT_MAX_FRAME_BYTES,
    )
    .expect("serve_http_rpc returns cleanly");
    assert_eq!(exit_code, 0);

    let line = client_thread.join().expect("join client thread");
    let parsed: serde_json::Value = serde_json::from_str(&line).expect("parse rpc json");
    assert_eq!(parsed["id"], 9);
    assert_eq!(parsed["result"]["status"], 200);
    assert_eq!(parsed["result"]["body"], "GET:/ok");
}

fn run_cli_subcommands(bin: &Path, work_root: &Path, cache_dir: &Path) {
    let run_bin = |args: &[&str]| -> std::process::Output {
        let mut cmd = Command::new(bin);
        cmd.args(args)
            .current_dir("/tmp")
            .env("LISP_CORE_CACHE_DIR", cache_dir);
        scrub_cmd_env(&mut cmd);
        cmd.output().expect("execute lisp-core binary")
    };

    let eval_out = run_bin(&["--no-auto-warm", "eval", "(+ 20 22)"]);
    assert!(eval_out.status.success(), "{eval_out:?}");
    assert_eq!(String::from_utf8_lossy(&eval_out.stdout).trim(), "42");
    assert!(
        eval_out.stderr.is_empty(),
        "cold boot stderr must be silent by default, got: {}",
        String::from_utf8_lossy(&eval_out.stderr)
    );

    let verbose_out = run_bin(&["--no-auto-warm", "--verbose", "eval", "(+ 20 22)"]);
    assert!(verbose_out.status.success(), "{verbose_out:?}");
    assert!(
        String::from_utf8_lossy(&verbose_out.stderr).contains("Loading "),
        "--verbose should emit bootstrap load progress: {}",
        String::from_utf8_lossy(&verbose_out.stderr)
    );

    let script = work_root.join("it_script.el");
    fs::write(
        &script,
        "#!lisp-core run\n(princ (format \"%s|%s\\n\" lexical-binding command-line-args-left))\n",
    )
    .expect("write script");
    let run_out = run_bin(&["run", script.to_str().unwrap(), "a", "b"]);
    assert!(run_out.status.success(), "{run_out:?}");
    assert!(
        String::from_utf8_lossy(&run_out.stdout).contains("t|(a b)"),
        "{run_out:?}"
    );

    let clean_el = work_root.join("clean.el");
    fs::write(
        &clean_el,
        ";;; clean.el -*- lexical-binding: t -*-\n(defun it-ok (x) (+ x 1))\n(provide 'clean)\n",
    )
    .expect("write clean.el");
    let comp_ok = run_bin(&["compile", "--strict", clean_el.to_str().unwrap()]);
    assert!(comp_ok.status.success(), "{comp_ok:?}");
    assert!(work_root.join("clean.elc").is_file());

    let warn_el = work_root.join("warn.el");
    fs::write(
        &warn_el,
        ";;; warn.el -*- lexical-binding: t -*-\n(defun it-warn () (missing-fn-xyz 1))\n(provide 'warn)\n",
    )
    .expect("write warn.el");
    let comp_fail = run_bin(&["compile", "--strict", warn_el.to_str().unwrap()]);
    assert!(!comp_fail.status.success());

    let ert_ok = work_root.join("ert_ok.el");
    fs::write(
        &ert_ok,
        ";;; ert_ok.el -*- lexical-binding: t -*-\n(require 'ert)\n(ert-deftest it-pass () (should (= 42 (+ 20 22))))\n",
    )
    .expect("write ert_ok.el");
    let test_ok = run_bin(&["test", ert_ok.to_str().unwrap()]);
    assert!(test_ok.status.success(), "{test_ok:?}");

    let ert_bad = work_root.join("ert_bad.el");
    fs::write(
        &ert_bad,
        ";;; ert_bad.el -*- lexical-binding: t -*-\n(require 'ert)\n(ert-deftest it-fail () (should (= 1 2)))\n",
    )
    .expect("write ert_bad.el");
    let test_bad = run_bin(&["test", ert_bad.to_str().unwrap()]);
    assert!(!test_bad.status.success());

    let digest_out = run_bin(&["exec", "digest", ert_ok.to_str().unwrap()]);
    assert!(digest_out.status.success(), "{digest_out:?}");
    let digest_json: serde_json::Value =
        serde_json::from_slice(&digest_out.stdout).expect("parse digest json");
    assert_eq!(digest_json["schema"], "femacs-test-digest-v1");

    let sup_out = run_bin(&["exec", "supervise", "--", "/bin/echo", "it-sup"]);
    assert!(sup_out.status.success(), "{sup_out:?}");
    let sup_json: serde_json::Value =
        serde_json::from_slice(&sup_out.stdout).expect("parse supervise json");
    assert_eq!(sup_json["schema"], "femacs-test-exec-v1");
    assert_eq!(sup_json["children_reaped"], true);

    let pid = std::process::id().to_string();
    let mem_out = run_bin(&["exec", "mem-tree", &pid]);
    assert!(mem_out.status.success(), "{mem_out:?}");
    let mem_json: serde_json::Value =
        serde_json::from_slice(&mem_out.stdout).expect("parse mem-tree json");
    assert_eq!(mem_json["schema"], "femacs-test-mem-v1");
}

fn main() {
    if let Ok(case) = env::var(TEST_ENV_KEY) {
        match case.as_str() {
            "engine_minimal_no_charsets" => case_engine_minimal_no_charsets(),
            "engine_new_roundtrips_and_call" => case_engine_new_roundtrips_and_call(),
            "engine_register_fn_and_bytes_roundtrip" => {
                case_engine_register_fn_and_bytes_roundtrip()
            }
            "engine_error_variants" => case_engine_error_variants(),
            "engine_corrupt_pdmp_fallback_bad_magic" => {
                case_engine_corrupt_pdmp_fallback_bad_magic()
            }
            "engine_corrupt_pdmp_fallback_corrupt_body" => {
                case_engine_corrupt_pdmp_fallback_corrupt_body()
            }
            "engine_rpc_unix_pair" => case_engine_rpc_unix_pair(),
            other => panic!("unknown integration test case: {other}"),
        }
        return;
    }

    let tmp_root = env::temp_dir().join(format!("lisp-core-it-{}", std::process::id()));
    let _ = fs::remove_dir_all(&tmp_root);
    let cache_dir = tmp_root.join("cache");
    let work_dir = tmp_root.join("work");
    fs::create_dir_all(&cache_dir).expect("create temp cache dir");
    fs::create_dir_all(&work_dir).expect("create temp work dir");

    for case in [
        "engine_minimal_no_charsets",
        "engine_new_roundtrips_and_call",
        "engine_register_fn_and_bytes_roundtrip",
        "engine_error_variants",
        "engine_corrupt_pdmp_fallback_bad_magic",
        "engine_corrupt_pdmp_fallback_corrupt_body",
        "engine_rpc_unix_pair",
    ] {
        println!("running integration subprocess case: {case}");
        spawn_child_case(case, &cache_dir);
    }

    let bin = PathBuf::from(env!("CARGO_BIN_EXE_lisp-core"));
    println!("running cli subcommand integration checks");
    run_cli_subcommands(&bin, &work_dir, &cache_dir);

    let _ = fs::remove_dir_all(&tmp_root);
    println!("engine_integration: all 8 integration cases passed");
}
