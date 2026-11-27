#![forbid(unsafe_code)]
//! Real-process integration tests for `lisp::Engine`, `Value`, `Error`, `NoGil`, and cache fallback.

use std::env;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
#[cfg(unix)]
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::process::Command;
use std::thread;

use lisp::prelude::{BootstrapMode, ConcurrencyMode, Engine, EngineConfig, Error, Value};
use lisp::rpc::{DEFAULT_MAX_FRAME_BYTES, serve_http_rpc};
use lisp::stdlib_cache::{ensure_unpacked, exe_pdmp_path, resolve_cache_root};

const TEST_ENV_KEY: &str = "LISP_TEST_CASE";

fn scrub_cmd_env(cmd: &mut Command) {
    for var in ["EMACSLOADPATH", "EMACSDATA", "EMACSDOC", "EMACSPATH"] {
        cmd.env_remove(var);
    }
}

fn spawn_child_case(case: &str, cache_dir: &Path) {
    let exe = env::current_exe().expect("resolve test executable");
    let mut cmd = Command::new(exe);
    cmd.env(TEST_ENV_KEY, case)
        .env("LISP_CACHE_DIR", cache_dir);
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

    assert_eq!(engine.concurrency(), ConcurrencyMode::NoGil);
    engine.eval("(setq it-iso-var 111)").expect("set var on first isolate");

    let mut second = Engine::new().expect("second concurrent NoGil engine succeeds");
    assert_eq!(
        second.eval("(boundp 'it-iso-var)").expect("boundp check"),
        Value::Nil
    );
    second.eval("(setq it-iso-var 222)").expect("set var on second isolate");
    assert_eq!(engine.eval("it-iso-var").expect("read first"), Value::Int(111));
    assert_eq!(second.eval("it-iso-var").expect("read second"), Value::Int(222));
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
    let _ = fs::remove_file(&pdmp);
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
    let _ = fs::remove_file(&pdmp);
}

#[cfg(unix)]
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

fn case_engine_rpc_tcp_loopback() {
    let cache_root = resolve_cache_root(None);
    let scratch = cache_root.join("rpc-tcp-scratch");
    fs::create_dir_all(&scratch).expect("create rpc tcp scratch");
    let handler = scratch.join("handler.el");
    fs::write(
        &handler,
        ";;; handler.el -*- lexical-binding: t -*-\n\
         (defun test-tcp-http-entry (req)\n\
           (list (cons 'status 200)\n\
                 (cons 'headers '((\"x-rpc-transport\" . \"tcp\")))\n\
                 (cons 'body (format \"%s:%s\" (alist-get 'method req) (alist-get 'path req)))))\n",
    )
    .expect("write rpc tcp handler");

    let listener = TcpListener::bind("127.0.0.1:0").expect("bind tcp loopback");
    let addr = listener.local_addr().expect("listener addr");

    let client_thread = thread::spawn(move || {
        let mut cli = TcpStream::connect(addr).expect("connect rpc tcp");
        cli.write_all(
            br#"{"jsonrpc":"2.0","id":11,"method":"http.handle","params":{"method":"POST","path":"/tcp","query":"","headers":[],"body":"","requestId":"r11"}}"#
                .as_slice(),
        )
        .expect("write tcp rpc request");
        cli.write_all(b"\n").expect("write newline");
        cli.flush().expect("flush");
        let _ = cli.shutdown(std::net::Shutdown::Write);
        let mut line = String::new();
        let mut reader = BufReader::new(&cli);
        reader.read_line(&mut line).expect("read tcp rpc response");
        line
    });

    let (srv, _) = listener.accept().expect("accept tcp rpc");
    let mut engine = Engine::new().expect("init engine for tcp rpc");
    let exit_code = serve_http_rpc(
        &mut engine,
        srv,
        &handler,
        "test-tcp-http-entry",
        DEFAULT_MAX_FRAME_BYTES,
    )
    .expect("serve_http_rpc tcp returns cleanly");
    assert_eq!(exit_code, 0);

    let line = client_thread.join().expect("join client thread");
    let parsed: serde_json::Value = serde_json::from_str(&line).expect("parse tcp rpc json");
    assert_eq!(parsed["id"], 11);
    assert_eq!(parsed["result"]["status"], 200);
    assert_eq!(parsed["result"]["body"], "POST:/tcp");
}

fn case_engine_nogil_multi_thread_parallel_and_gc_stress() {
    let mut handles = Vec::new();
    for tid in 0..4_i64 {
        handles.push(thread::spawn(move || {
            let mut eng = Engine::new().expect("init NoGil engine in worker thread");
            eng.eval(&format!("(setq worker-id {tid})")).expect("set worker-id");
            for round in 0..8_i64 {
                let form = format!(
                    "(let ((acc 0) (i 0))
                       (while (< i 250)
                         (let ((s (format \"w%d-r%d-%d\" worker-id {round} i)))
                           (setq acc (+ acc (length s))))
                         (setq i (1+ i)))
                       (garbage-collect)
                       acc)"
                );
                let out = eng.eval(&form).expect("gc stress loop");
                assert!(out.as_int().unwrap_or(0) > 1000);
            }
            assert_eq!(eng.eval("worker-id").expect("read worker-id"), Value::Int(tid));
            tid
        }));
    }
    let mut results: Vec<i64> = handles.into_iter().map(|h| h.join().expect("join")).collect();
    results.sort_unstable();
    assert_eq!(results, vec![0, 1, 2, 3]);
}

fn case_engine_nogil_send_across_threads() {
    let mut eng = Engine::new().expect("init engine on creator thread");
    eng.eval("(defun send-add (x) (+ x 100))").expect("defun send-add");

    let moved_back = thread::spawn(move || {
        assert_eq!(eng.eval("(send-add 23)").expect("eval moved"), Value::Int(123));
        eng.gc().expect("gc on worker thread");
        eng.eval("(setq moved-state \"ok\")").expect("mutate moved");
        eng
    })
    .join()
    .expect("join moved engine thread");

    let mut eng = moved_back;
    assert_eq!(
        eng.eval("(list (send-add 7) moved-state)").expect("eval returned engine"),
        Value::List(vec![Value::Int(107), Value::String("ok".to_string())])
    );
}

fn case_engine_nogil_std_task_chan_sync() {
    let mut eng = Engine::new().expect("init engine for std concurrency");
    let chan_val = eng
        .eval(
            "(progn
               (require 'std/chan)
               (let ((ch (std-chan-make 2)))
                 (list
                   (std-chan-try-send ch 'a)
                   (std-chan-try-send ch 'b)
                   (std-chan-try-send ch 'c)
                   (std-chan-len ch)
                   (cdr (std-chan-recv ch))
                   (car (std-chan-try-recv ch))
                   (car (std-chan-try-recv ch))
                   (car (std-chan-recv-timeout ch 5))
                   (std-chan-close ch)
                   (std-chan-closed-p ch)
                   (cdr (std-chan-recv ch)))))",
        )
        .expect("std/chan test");
    assert_eq!(
        chan_val,
        Value::List(vec![
            Value::Bool(true),
            Value::Bool(true),
            Value::Nil,
            Value::Int(2),
            Value::Symbol("a".to_string()),
            Value::Symbol("ok".to_string()),
            Value::Symbol("empty".to_string()),
            Value::Symbol("timeout".to_string()),
            Value::Bool(true),
            Value::Bool(true),
            Value::Nil,
        ])
    );

    let sync_val = eng
        .eval(
            "(progn
               (require 'std/sync)
               (let ((a (std-atomic-make 10))
                     (m (std-mutex-make 5))
                     (s (std-shared-make)))
                 (std-atomic-add a 7)
                 (let ((cas-ok (std-atomic-cas a 17 42))
                       (cas-bad (std-atomic-cas a 99 0)))
                   (std-with-mutex m
                     (std-mutex-set m 99))
                   (std-shared-set s \"k1\" '(1 2))
                   (std-shared-set s \"k2\" \"v2\")
                   (std-shared-del s \"k2\")
                   (list (std-atomic-load a) cas-ok cas-bad
                         (std-mutex-get m)
                         (std-shared-get s \"k1\")
                         (std-shared-keys s)))))",
        )
        .expect("std/sync test");
    assert_eq!(
        sync_val,
        Value::List(vec![
            Value::Int(42),
            Value::Bool(true),
            Value::Nil,
            Value::Int(99),
            Value::List(vec![Value::Int(1), Value::Int(2)]),
            Value::List(vec![Value::String("k1".to_string())]),
        ])
    );

    let task_val = eng
        .eval(
            "(progn
               (require 'std/task)
               (require 'std/chan)
               (require 'std/sync)
               (let* ((ch (std-chan-make 8))
                      (atom (std-atomic-make 0))
                      (t1 (std-task-spawn
                            (format \"(progn (std-chan-send %d 'from-t1) (std-atomic-add %d 10) 111)\"
                                    ch atom)))
                      (t2 (std-task-spawn
                            (format \"(progn (std-chan-send %d 'from-t2) (std-atomic-add %d 20) 222)\"
                                    ch atom)))
                      (r1 (std-task-await t1))
                      (r2 (std-task-await t2))
                      (pool (std-task-pool-create 2))
                      (pt (std-task-pool-submit pool (lambda (x) (* x 3)) 14))
                      (pt-lex (std-task-pool-submit pool (eval '(lambda (x) (* x 4)) t) 5)))
                 (defun it-rec-sq (n)
                   (if (<= n 0) 0 (* n n)))
                 (let ((mapped (std-task-parallel-map #'it-rec-sq '(1 2 3 4 5 6) 2))
                       (pr (std-task-await pt))
                       (pr-lex (std-task-await pt-lex)))
                   (std-task-pool-shutdown pool)
                   (list r1 r2 (std-atomic-load atom) (std-chan-len ch) mapped pr pr-lex))))",
        )
        .expect("std/task test");
    assert_eq!(
        task_val,
        Value::List(vec![
            Value::Int(111),
            Value::Int(222),
            Value::Int(30),
            Value::Int(2),
            Value::List(vec![
                Value::Int(1),
                Value::Int(4),
                Value::Int(9),
                Value::Int(16),
                Value::Int(25),
                Value::Int(36),
            ]),
            Value::Int(42),
            Value::Int(20),
        ])
    );
}

fn count_open_fds() -> usize {
    #[cfg(any(target_os = "macos", target_os = "freebsd"))]
    {
        fs::read_dir("/dev/fd").map(|rd| rd.count()).unwrap_or(0)
    }
    #[cfg(target_os = "linux")]
    {
        fs::read_dir("/proc/self/fd").map(|rd| rd.count()).unwrap_or(0)
    }
    #[cfg(not(any(target_os = "macos", target_os = "freebsd", target_os = "linux")))]
    {
        0
    }
}

fn case_engine_nogil_lifecycle_no_fd_leak() {
    {
        let mut warm = Engine::new().expect("warmup engine");
        assert_eq!(warm.eval("(+ 1 1)").unwrap(), Value::Int(2));
    }
    let baseline = count_open_fds();
    for i in 0..12 {
        let mut eng = Engine::new().expect("lifecycle engine");
        assert_eq!(
            eng.eval(&format!("(+ {i} 10)")).unwrap(),
            Value::Int(i + 10)
        );
    }
    let after = count_open_fds();
    assert!(
        after <= baseline + 2,
        "FD leak detected across NoGil engine lifecycles: baseline={baseline} after={after}"
    );
}

fn case_engine_gil_mode_preserves_single_init() {
    let cfg = EngineConfig::default().with_concurrency(ConcurrencyMode::Gil);
    let mut eng = Engine::init(cfg.clone()).expect("first Gil engine init");
    assert_eq!(eng.concurrency(), ConcurrencyMode::Gil);
    assert_eq!(eng.eval("(+ 20 22)").expect("eval in Gil"), Value::Int(42));

    match Engine::init(cfg) {
        Err(Error::AlreadyInitialized) => {}
        Ok(_) => panic!("second Gil engine init unexpectedly succeeded"),
        Err(other) => {
            panic!("second Gil engine init must fail with AlreadyInitialized, got {other:?}")
        }
    }

    let spawn_err = eng.eval("(progn (require 'std/task) (std-task-spawn \"(+ 1 2)\"))");
    assert!(
        matches!(spawn_err, Err(Error::Runtime { .. })),
        "std-task-spawn in Gil mode must fail explicitly, got {spawn_err:?}"
    );
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
            #[cfg(unix)]
            "engine_rpc_unix_pair" => case_engine_rpc_unix_pair(),
            "engine_rpc_tcp_loopback" => case_engine_rpc_tcp_loopback(),
            "engine_nogil_multi_thread_parallel_and_gc_stress" => {
                case_engine_nogil_multi_thread_parallel_and_gc_stress()
            }
            "engine_nogil_send_across_threads" => case_engine_nogil_send_across_threads(),
            "engine_nogil_std_task_chan_sync" => case_engine_nogil_std_task_chan_sync(),
            "engine_nogil_lifecycle_no_fd_leak" => case_engine_nogil_lifecycle_no_fd_leak(),
            "engine_gil_mode_preserves_single_init" => case_engine_gil_mode_preserves_single_init(),
            other => panic!("unknown integration test case: {other}"),
        }
        return;
    }

    let tmp_root = env::temp_dir().join(format!("lisp-it-{}", std::process::id()));
    let _ = fs::remove_dir_all(&tmp_root);
    let cache_dir = tmp_root.join("cache");
    fs::create_dir_all(&cache_dir).expect("create temp cache dir");

    for case in [
        "engine_minimal_no_charsets",
        "engine_new_roundtrips_and_call",
        "engine_register_fn_and_bytes_roundtrip",
        "engine_error_variants",
        "engine_corrupt_pdmp_fallback_bad_magic",
        "engine_corrupt_pdmp_fallback_corrupt_body",
        #[cfg(unix)]
        "engine_rpc_unix_pair",
        "engine_rpc_tcp_loopback",
        "engine_nogil_multi_thread_parallel_and_gc_stress",
        "engine_nogil_send_across_threads",
        "engine_nogil_std_task_chan_sync",
        "engine_nogil_lifecycle_no_fd_leak",
        "engine_gil_mode_preserves_single_init",
    ] {
        println!("running integration subprocess case: {case}");
        spawn_child_case(case, &cache_dir);
    }

    let _ = fs::remove_dir_all(&tmp_root);
    println!("engine_integration: all integration cases passed");
}
