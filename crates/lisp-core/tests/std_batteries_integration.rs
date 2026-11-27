#![forbid(unsafe_code)]
//! End-to-end integration tests for `lisp-std` batteries (`std/*`) and `init` / `pkg` workflow.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;

use lisp_core::{Engine, EngineConfig, Value};

const TEST_ENV_KEY: &str = "LISP_STD_TEST_CASE";

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
            "std_batteries case '{case}' failed (status={:?})\nstdout:\n{}\nstderr:\n{}",
            out.status,
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
    }
}

fn new_engine(cache_dir: &Path) -> Engine {
    Engine::init(EngineConfig::default().with_cache_dir(cache_dir)).expect("init engine")
}

fn case_http_client_and_server() {
    let cache = env::var_os("LISP_CORE_CACHE_DIR").unwrap();
    let mut srv_eng = new_engine(Path::new(&cache));
    let mut cli_eng = new_engine(Path::new(&cache));
    let port_val = srv_eng
        .eval(
            "(progn
               (require 'std/http)
               (let ((srv (std-http-server-start \"127.0.0.1:0\")))
                 (setq it-srv-id (gethash \"id\" srv))
                 (gethash \"port\" srv)))",
        )
        .expect("start localhost http server");
    let port = port_val.as_int().expect("server port int");

    let srv_thread = thread::spawn(move || {
        // Handle 3 HTTP requests: GET /hello?who=lisp, POST /echo, GET /missing -> 404
        for _ in 0..3 {
            let ok = srv_eng
                .eval(
                    "(let ((req (std-http-server-accept it-srv-id 15000)))
                       (when req
                         (let ((method (gethash \"method\" req))
                               (path (gethash \"path\" req))
                               (query (gethash \"query\" req))
                               (hdrs (gethash \"headers\" req))
                               (req-id (gethash \"id\" req)))
                           (cond
                            ((and (equal method \"GET\") (equal path \"/hello\"))
                             (std-http-server-respond
                              it-srv-id req-id 200
                              '((\"x-server\" . \"lisp-std\"))
                              (format \"hi %s\" (gethash \"who\" query))))
                            ((and (equal method \"POST\") (equal path \"/echo\"))
                             (require 'std/json)
                             (let* ((in (std-json-parse (gethash \"body\" req)))
                                    (out (make-hash-table :test 'equal)))
                               (puthash \"echo\" (gethash \"msg\" in) out)
                               (puthash \"token\" (gethash \"x-token\" hdrs) out)
                               (std-http-server-respond
                                it-srv-id req-id 201
                                '((\"content-type\" . \"application/json\"))
                                (std-json-stringify out))))
                            (t
                             (std-http-server-respond it-srv-id req-id 404 nil \"nope\")))
                           t)))",
                )
                .expect("serve request");
            assert_eq!(ok, Value::Bool(true));
        }
        srv_eng.eval("(std-http-server-stop it-srv-id)").unwrap();
    });

    let res = cli_eng
        .eval(&format!(
            "(progn
               (require 'std/http)
               (let* ((r1 (std-http-get \"http://127.0.0.1:{port}/hello?who=lisp\"))
                      (p (make-hash-table :test 'equal))
                      (_ (puthash \"msg\" \"ping\" p))
                      (r2 (std-http-post-json \"http://127.0.0.1:{port}/echo\" p '((\"x-token\" . \"abc\"))))
                      (r2json (std-json-parse (gethash \"body\" r2)))
                      (r3 (std-http-get \"http://127.0.0.1:{port}/missing\")))
                 (list (gethash \"status\" r1)
                       (gethash \"ok\" r1)
                       (gethash \"body\" r1)
                       (gethash \"x-server\" (gethash \"headers\" r1))
                       (gethash \"status\" r2)
                       (gethash \"echo\" r2json)
                       (gethash \"token\" r2json)
                       (gethash \"status\" r3)
                       (gethash \"ok\" r3))))"
        ))
        .expect("run http client requests");
    srv_thread.join().expect("join server thread");

    assert_eq!(
        res,
        Value::List(vec![
            Value::Int(200),
            Value::Bool(true),
            Value::String("hi lisp".to_string()),
            Value::String("lisp-std".to_string()),
            Value::Int(201),
            Value::String("ping".to_string()),
            Value::String("abc".to_string()),
            Value::Int(404),
            Value::Nil,
        ])
    );
}

fn case_kv_and_sql_persistence() {
    let cache = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap(); // unused; use LISP_CORE_CACHE_DIR
    let _ = cache;
    let cache_dir = env::var_os("LISP_CORE_CACHE_DIR").unwrap();
    let work = PathBuf::from(&cache_dir).parent().unwrap().join("db-work");
    let _ = fs::remove_dir_all(&work);
    fs::create_dir_all(&work).unwrap();
    let kv_path = work.join("store.redb");
    let sql_path = work.join("app.redb");

    let mut eng = new_engine(Path::new(&cache_dir));
    let kv_res = eng
        .eval(&format!(
            "(progn
               (require 'std/kv)
               (std-kv-with (kv {:?})
                 (std-kv-set kv \"app:a\" \"alpha\")
                 (std-kv-set kv \"app:b\" (unibyte-string 0 255 42))
                 (std-kv-set kv \"other\" \"z\")
                 (std-kv-del kv \"other\"))
               (std-kv-with (kv {:?})
                 (list (std-kv-get kv \"app:a\")
                       (std-kv-get-bytes kv \"app:b\")
                       (std-kv-list kv \"app:\")
                       (std-kv-get kv \"other\"))))",
            kv_path.to_string_lossy(),
            kv_path.to_string_lossy()
        ))
        .expect("kv persistence");
    assert_eq!(
        kv_res,
        Value::List(vec![
            Value::String("alpha".to_string()),
            Value::Bytes(vec![0, 255, 42]),
            Value::List(vec![
                Value::String("app:a".to_string()),
                Value::String("app:b".to_string())
            ]),
            Value::Nil,
        ])
    );

    let sql_res = eng
        .eval(&format!(
            "(progn
               (require 'std/db)
               (std-db-with (db {:?})
                 (std-db-exec db \"CREATE TABLE items (id INTEGER PRIMARY KEY, name TEXT NOT NULL, qty INTEGER)\")
                 (std-db-with-tx db
                   (std-db-exec db \"INSERT INTO items (id, name, qty) VALUES (?, ?, ?)\" '(1 \"pen\" 5))
                   (std-db-exec db \"INSERT INTO items (id, name, qty) VALUES (?, ?, ?)\" '(2 \"book\" 12)))
                 (ignore-errors
                   (std-db-with-tx db
                     (std-db-exec db \"UPDATE items SET qty = 99 WHERE id = 1\")
                     (error \"abort tx\")))
                 (std-db-exec db \"UPDATE items SET qty = 7 WHERE id = 1\"))
               (std-db-with (db {:?})
                 (let ((rows (std-db-query db \"SELECT name, qty FROM items WHERE qty >= ? ORDER BY qty DESC\" '(7)))
                       (cnt (car (std-db-query db \"SELECT COUNT(*) AS total FROM items\")))
                       (tbls (std-db-tables db)))
                   (list (length rows)
                         (gethash \"name\" (nth 0 rows))
                         (gethash \"qty\" (nth 1 rows))
                         (gethash \"total\" cnt)
                         tbls))))",
            sql_path.to_string_lossy(),
            sql_path.to_string_lossy()
        ))
        .expect("sql persistence");
    assert_eq!(
        sql_res,
        Value::List(vec![
            Value::Int(2),
            Value::String("book".to_string()),
            Value::Int(7),
            Value::Int(2),
            Value::List(vec![Value::String("items".to_string())]),
        ])
    );
}

fn case_formats_archive_crypto_time_fs_regex_proc_cli_log() {
    let cache_dir = env::var_os("LISP_CORE_CACHE_DIR").unwrap();
    let work = PathBuf::from(&cache_dir).parent().unwrap().join("all-work");
    let _ = fs::remove_dir_all(&work);
    fs::create_dir_all(&work).unwrap();

    let mut eng = new_engine(Path::new(&cache_dir));
    let res = eng
        .eval(&format!(
            "(progn
               (require 'std/json)
               (require 'std/toml)
               (require 'std/csv)
               (require 'std/yaml)
               (require 'std/url)
               (require 'std/base64)
               (require 'std/archive)
               (require 'std/crypto)
               (require 'std/time)
               (require 'std/fs)
               (require 'std/path)
               (require 'std/regex)
               (require 'std/proc)
               (require 'std/env)
               (require 'std/cli)
               (require 'std/log)
               (let* ((j (std-json-parse (std-json-stringify #s(hash-table test equal data (\"a\" 1 \"b\" \"hi\")))))
                      (tmap (std-toml-parse (std-toml-stringify #s(hash-table test equal data (\"title\" \"demo\" \"n\" 7)))))
                      (ymap (std-yaml-parse \"k: hello\\narr:\\n  - 10\\n  - 20\\n\"))
                      (crows (std-csv-parse \"id,name\\n1,alice\\n2,bob\\n\" t))
                      (csv-out (std-csv-stringify crows '(\"id\" \"name\")))
                      (u (std-url-parse \"https://example.com:8443/v1/items?q=hello+world#top\"))
                      (b64 (std-base64-decode-string (std-base64-encode \"rust-lisp\" t) t))
                      (mac (std-crypto-hmac-sha256 \"Jefe\" \"what do ya want for nothing?\"))
                      (rnd (std-crypto-random-bytes 16))
                      (u4 (std-crypto-uuid-v4))
                      (ts (std-time-add-seconds \"2026-01-01T00:00:00Z\" 90))
                      (parsed-ts (std-time-parse-rfc3339 ts))
                      (root {:?})
                      (src (std-path-join (list root \"src\")))
                      (tgz (std-path-join (list root \"bundle.tar.gz\")))
                      (zipf (std-path-join (list root \"bundle.zip\")))
                      (out1 (std-path-join (list root \"out-tar\")))
                      (out2 (std-path-join (list root \"out-zip\")))
                      (_ (std-fs-mkdir-p (std-path-join (list src \"nested\"))))
                      (_ (std-fs-write-string (std-path-join (list src \"nested\" \"a.el\")) \"(+ 1 2)\" t))
                      (_ (std-tar-pack src tgz t))
                      (_ (std-tar-unpack tgz out1 t))
                      (_ (std-zip-pack src zipf))
                      (_ (std-zip-unpack zipf out2))
                      (globbed (std-fs-glob out1 \"**/*.el\"))
                      (caps (std-regex-captures \"([a-z]+)-([0-9]+)\" \"item-42\"))
                      (dotenv (std-env-parse-dotenv \"export FOO=bar\\nNUM=9\\n\"))
                      (cli (std-cli-parse
                            '((:name port :short \"-p\" :long \"--port\" :parse int :default 8080)
                              (:name verbose :short \"-v\" :flag t))
                            '(\"--port\" \"9000\" \"-v\" \"run\")))
                      (proc-ok (std-proc-run \"sh\" '(\"-c\" \"printf hello\")))
                      (proc-to (std-proc-run \"sh\" '(\"-c\" \"sleep 2\") '((\"timeout-ms\" . 50))))
                      (log-json (let ((std-log-format 'json))
                                  (std-json-parse (std-log-format-line 'info \"ready\" '((\"k\" . 3)))))))
                 (list (gethash \"a\" j)
                       (gethash \"n\" tmap)
                       (gethash \"k\" ymap)
                       (length crows)
                       (std-regex-match-p \"1,alice\" csv-out)
                       (gethash \"q\" (gethash \"query\" u))
                       b64
                       (equal mac \"5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843\")
                       (length rnd)
                       (std-regex-match-p \"^[0-9a-f-]+$\" u4)
                       (gethash \"minute\" parsed-ts)
                       (std-fs-read-string (std-path-join (list out2 \"nested\" \"a.el\")))
                       (length globbed)
                       (nth 2 caps)
                       (gethash \"FOO\" dotenv)
                       (gethash \"port\" cli)
                       (gethash \"verbose\" cli)
                       (gethash \"stdout\" proc-ok)
                       (gethash \"timed-out\" proc-to)
                       (gethash \"msg\" log-json))))",
            work.to_string_lossy()
        ))
        .expect("eval batteries suite");

    assert_eq!(
        res,
        Value::List(vec![
            Value::Int(1),
            Value::Int(7),
            Value::String("hello".to_string()),
            Value::Int(2),
            Value::Bool(true),
            Value::String("hello world".to_string()),
            Value::String("rust-lisp".to_string()),
            Value::Bool(true),
            Value::Int(16),
            Value::Bool(true),
            Value::Int(1),
            Value::String("(+ 1 2)".to_string()),
            Value::Int(1),
            Value::String("42".to_string()),
            Value::String("bar".to_string()),
            Value::Int(9000),
            Value::Bool(true),
            Value::String("hello".to_string()),
            Value::Bool(true),
            Value::String("ready".to_string()),
        ])
    );
}

fn case_nogil_parallel_batteries() {
    let cache_dir = env::var_os("LISP_CORE_CACHE_DIR").unwrap();
    let mut eng = new_engine(Path::new(&cache_dir));
    let hashes = eng
        .eval(
            "(progn
               (require 'std/task)
               (require 'std/crypto)
               (std-task-parallel-map
                (lambda (s)
                  (require 'std/crypto)
                  (require 'std/json)
                  (let ((m (make-hash-table :test 'equal)))
                    (puthash \"h\" (std-crypto-sha256 s) m)
                    (gethash \"h\" (std-json-parse (std-json-stringify m)))))
                '(\"one\" \"two\" \"three\" \"four\")
                4))",
        )
        .expect("parallel std/task + std/crypto + std/json");

    let list = hashes.as_list().expect("list of 4 digests");
    assert_eq!(list.len(), 4);
    assert_eq!(
        list[0],
        Value::String("7692c3ad3540bb803c020b3aee66cd8887123234ea0c6e7143c0add73ff431ed".to_string())
    );
}

fn run_init_and_pkg_cli_workflow(bin: &Path, work_dir: &Path, cache_dir: &Path) {
    let wf = work_dir.join("pkg-workflow");
    let _ = fs::remove_dir_all(&wf);
    fs::create_dir_all(&wf).unwrap();

    // 1. Create a local `--path` dependency `local-math`.
    let dep_local = wf.join("local-math");
    let mut cmd = Command::new(bin);
    scrub_cmd_env(&mut cmd);
    let out = cmd
        .args(["init", dep_local.to_str().unwrap(), "--name", "local-math"])
        .output()
        .expect("init local-math");
    assert!(out.status.success(), "init local-math failed: {:?}", out);
    fs::write(
        dep_local.join("src/local-math.el"),
        ";;; local-math.el -*- lexical-binding: t; -*-\n(defun local-math-double (x) (* 2 x))\n(provide 'local-math)\n",
    )
    .unwrap();

    // 2. Create a remote `--url` dependency archive (`remote-fmt.tar.gz`) and serve over localhost HTTP!
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

    let mut srv_eng = new_engine(cache_dir);
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
        // Serve 2 GET fetches (pkg add --url + pkg update)
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

    // 3. `lisp-core init` main app, add path dep + url dep, `pkg list`, `run`, `test`, `pkg update`, `pkg remove`.
    let app = wf.join("my-app");
    let run_cli = |args: &[&str], cwd: &Path| {
        let mut c = Command::new(bin);
        scrub_cmd_env(&mut c);
        c.current_dir(cwd)
            .env("LISP_CORE_CACHE_DIR", cache_dir)
            .env("LISP_CORE_NO_AUTO_WARM", "1")
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
    if let Ok(case) = env::var(TEST_ENV_KEY) {
        match case.as_str() {
            "http_client_and_server" => case_http_client_and_server(),
            "kv_and_sql_persistence" => case_kv_and_sql_persistence(),
            "formats_archive_crypto_time_fs_regex_proc_cli_log" => {
                case_formats_archive_crypto_time_fs_regex_proc_cli_log()
            }
            "nogil_parallel_batteries" => case_nogil_parallel_batteries(),
            other => panic!("unknown std_batteries test case: {other}"),
        }
        return;
    }

    let tmp_root = env::temp_dir().join(format!("lisp-std-it-{}", std::process::id()));
    let _ = fs::remove_dir_all(&tmp_root);
    let cache_dir = tmp_root.join("cache");
    let work_dir = tmp_root.join("work");
    fs::create_dir_all(&cache_dir).expect("create temp cache dir");
    fs::create_dir_all(&work_dir).expect("create temp work dir");

    for case in [
        "http_client_and_server",
        "kv_and_sql_persistence",
        "formats_archive_crypto_time_fs_regex_proc_cli_log",
        "nogil_parallel_batteries",
    ] {
        println!("running std_batteries subprocess case: {case}");
        spawn_child_case(case, &cache_dir);
    }

    let bin = PathBuf::from(env!("CARGO_BIN_EXE_lisp-core"));
    println!("running init & pkg CLI workflow check");
    run_init_and_pkg_cli_workflow(&bin, &work_dir, &cache_dir);

    let _ = fs::remove_dir_all(&tmp_root);
    println!("std_batteries_integration: all 5 suites passed");
}
