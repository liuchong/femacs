#![forbid(unsafe_code)]
//! End-to-end integration tests for the standalone `elisp` binary CLI, REPL, pkg, and RPC.

use std::env;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
#[cfg(feature = "std")]
use std::thread;

fn scrub_cmd_env(cmd: &mut Command) {
    for var in ["EMACSLOADPATH", "EMACSDATA", "EMACSDOC", "EMACSPATH"] {
        cmd.env_remove(var);
    }
}

fn run_bin(bin: &Path, cache_dir: &Path, args: &[&str]) -> std::process::Output {
    let mut cmd = Command::new(bin);
    cmd.args(args)
        .current_dir("/tmp")
        .env("LISP_CACHE_DIR", cache_dir);
    scrub_cmd_env(&mut cmd);
    cmd.output().expect("execute elisp binary")
}

fn test_basic_cli_subcommands(bin: &Path, work_root: &Path, cache_dir: &Path) {
    let ver = run_bin(bin, cache_dir, &["--version"]);
    assert!(ver.status.success(), "{ver:?}");
    assert_eq!(String::from_utf8_lossy(&ver.stdout).trim(), "elisp 0.0.1");

    let help = run_bin(bin, cache_dir, &["--help"]);
    assert!(help.status.success(), "{help:?}");
    let help_text = String::from_utf8_lossy(&help.stdout);
    assert!(help_text.contains("elisp <SUBCOMMAND>"), "{help_text}");
    assert!(help_text.contains("--rpc-addr"), "{help_text}");

    let bad = run_bin(bin, cache_dir, &["--unknown-flag-xyz"]);
    assert_eq!(bad.status.code(), Some(2));

    let eval_out = run_bin(bin, cache_dir, &["--no-auto-warm", "eval", "(+ 20 22)"]);
    assert!(eval_out.status.success(), "{eval_out:?}");
    assert_eq!(String::from_utf8_lossy(&eval_out.stdout).trim(), "42");
    assert!(
        eval_out.stderr.is_empty(),
        "cold boot stderr must be silent by default, got: {}",
        String::from_utf8_lossy(&eval_out.stderr)
    );

    let verbose_out = run_bin(
        bin,
        cache_dir,
        &["--no-auto-warm", "--verbose", "eval", "(+ 20 22)"],
    );
    assert!(verbose_out.status.success(), "{verbose_out:?}");
    assert!(
        String::from_utf8_lossy(&verbose_out.stderr).contains("Loading "),
        "--verbose should emit bootstrap load progress: {}",
        String::from_utf8_lossy(&verbose_out.stderr)
    );

    let cache_path = run_bin(bin, cache_dir, &["cache", "path"]);
    assert!(cache_path.status.success(), "{cache_path:?}");
    assert_eq!(
        PathBuf::from(String::from_utf8_lossy(&cache_path.stdout).trim()),
        cache_dir
    );

    let warm_out = run_bin(bin, cache_dir, &["cache", "warm", "--quiet"]);
    assert!(warm_out.status.success(), "{warm_out:?}");

    let warm_gc_out = run_bin(
        bin,
        cache_dir,
        &[
            "--no-auto-warm",
            "eval",
            "(progn (setq gc-cons-threshold 200000) (require 'bytecomp) (garbage-collect) 'warm-gc-ok)",
        ],
    );
    assert!(warm_gc_out.status.success(), "{warm_gc_out:?}");
    assert!(
        String::from_utf8_lossy(&warm_gc_out.stdout).contains("warm-gc-ok"),
        "{warm_gc_out:?}"
    );

    let script = work_root.join("it_script.el");
    fs::write(
        &script,
        "#!elisp run\n(princ (format \"%s|%s\\n\" lexical-binding command-line-args-left))\n",
    )
    .expect("write script");
    let run_out = run_bin(bin, cache_dir, &["run", script.to_str().unwrap(), "a", "b"]);
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
    let comp_ok = run_bin(bin, cache_dir, &["compile", "--strict", clean_el.to_str().unwrap()]);
    assert!(comp_ok.status.success(), "{comp_ok:?}");
    assert!(work_root.join("clean.elc").is_file());

    let warn_el = work_root.join("warn.el");
    fs::write(
        &warn_el,
        ";;; warn.el -*- lexical-binding: t -*-\n(defun it-warn () (missing-fn-xyz 1))\n(provide 'warn)\n",
    )
    .expect("write warn.el");
    let comp_fail = run_bin(bin, cache_dir, &["compile", "--strict", warn_el.to_str().unwrap()]);
    assert!(!comp_fail.status.success());

    let ert_ok = work_root.join("ert_ok.el");
    fs::write(
        &ert_ok,
        ";;; ert_ok.el -*- lexical-binding: t -*-\n(require 'ert)\n(ert-deftest it-pass () (garbage-collect) (should (= 42 (+ 20 22))))\n",
    )
    .expect("write ert_ok.el");
    let test_ok = run_bin(bin, cache_dir, &["test", ert_ok.to_str().unwrap()]);
    assert!(test_ok.status.success(), "{test_ok:?}");

    let ert_bad = work_root.join("ert_bad.el");
    fs::write(
        &ert_bad,
        ";;; ert_bad.el -*- lexical-binding: t -*-\n(require 'ert)\n(ert-deftest it-fail () (should (= 1 2)))\n",
    )
    .expect("write ert_bad.el");
    let test_bad = run_bin(bin, cache_dir, &["test", ert_bad.to_str().unwrap()]);
    assert!(!test_bad.status.success());

    let digest_out = run_bin(bin, cache_dir, &["exec", "digest", ert_ok.to_str().unwrap()]);
    assert!(digest_out.status.success(), "{digest_out:?}");
    let digest_json: serde_json::Value =
        serde_json::from_slice(&digest_out.stdout).expect("parse digest json");
    assert_eq!(digest_json["schema"], "femacs-test-digest-v1");

    #[cfg(unix)]
    {
        let sup_out = run_bin(bin, cache_dir, &["exec", "supervise", "--", "/bin/echo", "it-sup"]);
        assert!(sup_out.status.success(), "{sup_out:?}");
        let sup_json: serde_json::Value =
            serde_json::from_slice(&sup_out.stdout).expect("parse supervise json");
        assert_eq!(sup_json["schema"], "femacs-test-exec-v1");
        assert_eq!(sup_json["children_reaped"], true);
    }

    let pid = std::process::id().to_string();
    let mem_out = run_bin(bin, cache_dir, &["exec", "mem-tree", &pid]);
    assert!(mem_out.status.success(), "{mem_out:?}");
    let mem_json: serde_json::Value =
        serde_json::from_slice(&mem_out.stdout).expect("parse mem-tree json");
    assert_eq!(mem_json["schema"], "femacs-test-mem-v1");

    let mut pipe_repl = Command::new(bin);
    pipe_repl
        .args(["--no-auto-warm", "repl"])
        .env("LISP_CACHE_DIR", cache_dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    scrub_cmd_env(&mut pipe_repl);
    let mut child = pipe_repl.spawn().expect("spawn piped repl");
    {
        let stdin = child.stdin.as_mut().expect("stdin pipe");
        stdin.write_all(b"(+ 19 23)\n").expect("write repl line");
    }
    let repl_out = child.wait_with_output().expect("wait repl");
    assert!(repl_out.status.success(), "{repl_out:?}");
    assert_eq!(String::from_utf8_lossy(&repl_out.stdout).trim(), "42");
}

fn test_cli_rpc_tcp_loopback(bin: &Path, work_root: &Path, cache_dir: &Path) {
    let handler = work_root.join("cli_rpc_handler.el");
    fs::write(
        &handler,
        ";;; cli_rpc_handler.el -*- lexical-binding: t -*-\n\
         (defun femacs-http-handle (req)\n\
           (list (cons 'status 200)\n\
                 (cons 'headers '((\"x-cli\" . \"elisp\")))\n\
                 (cons 'body (format \"%s:%s\" (alist-get 'method req) (alist-get 'path req)))))\n",
    )
    .expect("write cli rpc handler");

    let probe = TcpListener::bind("127.0.0.1:0").expect("probe ephemeral port");
    let addr = probe.local_addr().expect("probe addr");
    drop(probe);

    let mut cmd = Command::new(bin);
    cmd.args([
        "--no-auto-warm",
        "--rpc",
        "http",
        "--rpc-addr",
        &addr.to_string(),
        "--rpc-handler",
        handler.to_str().expect("utf8 handler path"),
    ])
    .env("LISP_CACHE_DIR", cache_dir)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped());
    scrub_cmd_env(&mut cmd);
    let child = cmd.spawn().expect("spawn rpc server");

    let mut stream = None;
    for _ in 0..80 {
        if let Ok(s) = TcpStream::connect(addr) {
            stream = Some(s);
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    let mut conn = stream.expect("connect to elisp --rpc-addr listener");
    writeln!(
        conn,
        "{{\"jsonrpc\":\"2.0\",\"id\":7,\"method\":\"http.handle\",\"params\":{{\"method\":\"GET\",\"path\":\"/cli\",\"query\":\"\",\"headers\":[],\"body\":\"\",\"requestId\":\"r7\"}}}}"
    )
    .expect("write rpc request");
    conn.flush().expect("flush rpc request");
    let _ = conn.shutdown(std::net::Shutdown::Write);
    let mut line = String::new();
    BufReader::new(&conn)
        .read_line(&mut line)
        .expect("read rpc response");
    drop(conn);

    let parsed: serde_json::Value = serde_json::from_str(&line).expect("parse rpc json");
    assert_eq!(parsed["id"], 7);
    assert_eq!(parsed["result"]["status"], 200);
    assert_eq!(parsed["result"]["body"], "GET:/cli");

    let out = child.wait_with_output().expect("wait rpc exit");
    assert!(out.status.success(), "{out:?}");
}

#[cfg(feature = "std")]
fn test_init_and_pkg_cli_workflow(bin: &Path, work_dir: &Path, cache_dir: &Path) {
    use lisp::prelude::{Engine, EngineConfig};

    let wf = work_dir.join("pkg-workflow");
    let _ = fs::remove_dir_all(&wf);
    fs::create_dir_all(&wf).unwrap();

    // 1. Local `--path` dependency `local-math`.
    let dep_local = wf.join("local-math");
    let init_local = run_bin(
        bin,
        cache_dir,
        &["init", dep_local.to_str().unwrap(), "--name", "local-math"],
    );
    assert!(init_local.status.success(), "{init_local:?}");
    fs::write(
        dep_local.join("src/local-math.el"),
        ";;; local-math.el -*- lexical-binding: t; -*-\n(defun local-math-double (x) (* 2 x))\n(provide 'local-math)\n",
    )
    .unwrap();

    // 2. Remote `--url` tarball served over localhost HTTP.
    let remote_src = wf.join("remote-fmt-src");
    fs::create_dir_all(remote_src.join("src")).unwrap();
    fs::write(
        remote_src.join("lisp.toml"),
        "[package]\nname = \"remote-fmt\"\nversion = \"0.3.0\"\n",
    )
    .unwrap();
    fs::write(
        remote_src.join("src/remote-fmt.el"),
        ";;; remote-fmt.el -*- lexical-binding: t; -*-\n(defun remote-fmt-tag (n) (format \"val=%d\" n))\n(provide 'remote-fmt)\n",
    )
    .unwrap();

    let mut srv_eng =
        Engine::init(EngineConfig::default().with_cache_dir(cache_dir)).expect("init srv engine");
    let tgz_path = wf.join("remote-fmt.tar.gz");
    let port = srv_eng
        .eval(&format!(
            "(progn
               (require 'std/archive)
               (require 'std/fs)
               (require 'std/http)
               (std-tar-pack {:?} {:?} t)
               (let ((srv (std-http-server-start \"127.0.0.1:0\")))
                 (setq it-pkg-srv-id (gethash \"id\" srv))
                 (gethash \"port\" srv)))",
            remote_src.to_string_lossy(),
            tgz_path.to_string_lossy()
        ))
        .expect("pack and start pkg http server")
        .as_int()
        .expect("pkg server port");

    let tgz_path_str = tgz_path.to_string_lossy().into_owned();
    let srv_thread = thread::spawn(move || {
        for _ in 0..2 {
            srv_eng
                .eval(&format!(
                    "(let ((req (std-http-server-accept it-pkg-srv-id 30000)))
                       (when req
                         (std-http-server-respond
                          it-pkg-srv-id
                          (gethash \"id\" req)
                          200
                          '((\"content-type\" . \"application/gzip\"))
                          (std-fs-read-bytes {:?}))))",
                    tgz_path_str
                ))
                .expect("serve tar.gz");
        }
        srv_eng.eval("(std-http-server-stop it-pkg-srv-id)").unwrap();
    });

    // 3. `elisp init`, `pkg add`, `pkg list`, `pkg update`, `run`, `test`, `pkg remove`.
    let app = wf.join("my-app");
    let run_cli = |args: &[&str], cwd: &Path| {
        let mut c = Command::new(bin);
        scrub_cmd_env(&mut c);
        c.current_dir(cwd)
            .env("LISP_CACHE_DIR", cache_dir)
            .env("LISP_NO_AUTO_WARM", "1")
            .args(args);
        let o = c.output().expect("run cli");
        assert!(
            o.status.success(),
            "cli {:?} failed:\nstdout:\n{}\nstderr:\n{}",
            args,
            String::from_utf8_lossy(&o.stdout),
            String::from_utf8_lossy(&o.stderr),
        );
        String::from_utf8_lossy(&o.stdout).into_owned()
    };

    run_cli(&["init", app.to_str().unwrap(), "--name", "my-app"], &wf);
    run_cli(
        &["pkg", "add", "local-math", "--path", "../local-math"],
        &app,
    );
    let url = format!("http://127.0.0.1:{port}/remote-fmt.tar.gz");
    run_cli(&["pkg", "add", "remote-fmt", "--url", &url], &app);

    let list_out = run_cli(&["pkg", "list"], &app);
    assert!(list_out.contains("local-math"), "{list_out}");
    assert!(list_out.contains("remote-fmt v0.3.0"), "{list_out}");

    run_cli(&["pkg", "update"], &app);
    srv_thread.join().expect("join pkg server");

    fs::write(
        app.join("src/main.el"),
        ";;; main.el -*- lexical-binding: t; -*-\n\
         (require 'local-math)\n\
         (require 'remote-fmt)\n\
         (defun my-app-run (n)\n  \
           (remote-fmt-tag (local-math-double n)))\n\
         (when (not (bound-and-true-p ert--running-tests))\n  \
           (princ (concat (my-app-run 21) \"\\n\")))\n\
         (provide 'main)\n",
    )
    .unwrap();
    fs::write(
        app.join("tests/main-test.el"),
        ";;; main-test.el -*- lexical-binding: t; -*-\n\
         (require 'ert)\n\
         (defvar ert--running-tests t)\n\
         (require 'main)\n\
         (ert-deftest my-app-integration-test ()\n  \
           (should (equal (my-app-run 21) \"val=42\")))\n",
    )
    .unwrap();

    let run_out = run_cli(&["run"], &app);
    assert_eq!(run_out.trim(), "val=42");

    run_cli(&["test"], &app);

    run_cli(&["pkg", "remove", "local-math"], &app);
    assert!(!app.join(".lisp-packages/local-math").exists());
}

fn main() {
    let bin = PathBuf::from(env!("CARGO_BIN_EXE_elisp"));
    let tmp_root = env::temp_dir().join(format!("elisp-it-{}", std::process::id()));
    let _ = fs::remove_dir_all(&tmp_root);
    let cache_dir = tmp_root.join("cache");
    let work_dir = tmp_root.join("work");
    fs::create_dir_all(&cache_dir).expect("create cache dir");
    fs::create_dir_all(&work_dir).expect("create work dir");

    println!("running elisp basic cli subcommand checks");
    test_basic_cli_subcommands(&bin, &work_dir, &cache_dir);

    println!("running elisp --rpc-addr tcp loopback check");
    test_cli_rpc_tcp_loopback(&bin, &work_dir, &cache_dir);

    #[cfg(feature = "std")]
    {
        println!("running elisp init & pkg workflow check");
        test_init_and_pkg_cli_workflow(&bin, &work_dir, &cache_dir);
    }

    let _ = fs::remove_dir_all(&tmp_root);
    println!("cli_integration: all elisp CLI integration cases passed");
}
