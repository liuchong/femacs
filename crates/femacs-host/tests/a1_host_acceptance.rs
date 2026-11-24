#![forbid(unsafe_code)]
//! Real-process Stage A1 integration & functional acceptance harness (`femacs-host`).
//!
//! Invoked per scenario via `A1_SCENARIO` so each single-init VM scenario executes in a
//! fresh OS process, verifying headless bootstrap, GC rooting, Lisp conditions/`quit`/`read`/I/O
//! error classification, `unwind-protect` cleanup, script loading, portable dump roundtrips,
//! stale/corrupt dump rejection, single-init enforcement, frame/terminal diagnostics, and
//! `#![forbid(unsafe_code)]` composition with `femacs_vm::safe_domain`.

use std::ffi::OsString;
use std::path::PathBuf;

use femacs_host::{HostConfig, HostError, LispHost};

fn env_path(key: &str) -> PathBuf {
    PathBuf::from(std::env::var_os(key).unwrap_or_else(|| panic!("missing env {key}")))
}

fn scenario_cold_minimal_lifecycle() {
    let mut host = LispHost::init(HostConfig::cold_minimal()).expect("cold minimal init");

    // 1. Buffer creation, UTF-8 insertion, GC across live text, and readback.
    let readback = host
        .read_eval(
            r#"
            (let ((buf (get-buffer-create "*a1-buf*")))
              (save-current-buffer
                (set-buffer buf)
                (setq buffer-undo-list t)
                (erase-buffer)
                (insert "hello 世界")
                (garbage-collect)
                (buffer-string)))
            "#,
        )
        .expect("buffer + GC readback");
    assert_eq!(readback, "\"hello 世界\"");

    // Explicit host GC API also works cleanly.
    host.garbage_collect().expect("explicit garbage_collect");

    // 2. Lisp condition with `unwind-protect` cleanup verification + next call recovery.
    let err_cond = host
        .read_eval(
            r#"
            (progn
              (setq a1-cleanup-count 0)
              (unwind-protect
                  (signal 'error (list (format "a1-boom %d" 7)))
                (setq a1-cleanup-count (1+ a1-cleanup-count))))
            "#,
        )
        .expect_err("error must return HostError::LispCondition");
    match err_cond {
        HostError::LispCondition {
            symbol,
            message,
            data,
        } => {
            assert_eq!(symbol, "error");
            assert!(message.contains("a1-boom 7"), "unexpected message: {message}");
            assert!(data.contains("a1-boom 7"), "unexpected data: {data}");
        }
        other => panic!("expected LispCondition, got {other:?}"),
    }
    assert_eq!(
        host.read_eval("a1-cleanup-count").expect("cleanup count"),
        "1"
    );

    // 3. Void-variable condition classification.
    let err_void = host
        .read_eval("a1-definitely-unbound-symbol")
        .expect_err("void symbol must return HostError::LispCondition");
    assert!(matches!(err_void, HostError::LispCondition { symbol, .. } if symbol == "void-variable"));

    // 4. `quit` signal AND `quit-flag` polling with `unwind-protect` cleanup.
    let err_quit1 = host
        .read_eval(
            r#"
            (unwind-protect
                (signal 'quit nil)
              (setq a1-cleanup-count (+ a1-cleanup-count 10)))
            "#,
        )
        .expect_err("signal quit must return HostError::Quit");
    assert!(matches!(err_quit1, HostError::Quit { .. }), "got {err_quit1:?}");
    assert_eq!(host.read_eval("a1-cleanup-count").unwrap(), "11");

    let err_quit2 = host
        .read_eval(
            r#"
            (unwind-protect
                (progn
                  (setq quit-flag t)
                  (while t nil))
              (setq a1-cleanup-count (+ a1-cleanup-count 100)))
            "#,
        )
        .expect_err("quit-flag loop must return HostError::Quit");
    assert!(matches!(err_quit2, HostError::Quit { .. }), "got {err_quit2:?}");
    assert_eq!(host.read_eval("a1-cleanup-count").unwrap(), "111");

    // 5. Read syntax & empty-input EOF errors.
    let err_eof = host.read_eval("").expect_err("empty input must fail read");
    assert!(matches!(err_eof, HostError::Read { .. }), "got {err_eof:?}");

    let err_unterminated = host
        .read_eval("(+ 1 ")
        .expect_err("unterminated list must fail read");
    assert!(
        matches!(err_unterminated, HostError::Read { .. }),
        "got {err_unterminated:?}"
    );

    // 6. Next normal evaluation succeeds cleanly with balanced specpdl & GC roots.
    assert_eq!(host.read_eval("; leading comment\n (+ 20 22) ; trailing").unwrap(), "42");

    let diag = host.inspect_environment().expect("inspect_environment");
    assert_eq!(diag.specpdl_depth, 0);
    assert_eq!(diag.rooting_counters.persistent_live, 0);
    assert_eq!(
        diag.rooting_counters.rooted_pushed,
        diag.rooting_counters.rooted_popped
    );
}

fn scenario_cold_loadup_and_script() {
    let lisp_dir = env_path("A1_LISP_DIR");
    let tmp_dir = env_path("A1_TMP_DIR");
    let mut host =
        LispHost::init(HostConfig::cold_loadup(lisp_dir)).expect("cold loadup init");

    // Full Elisp stdlib + byte-compile + cl-lib + json-serialize checks.
    let stdlib_out = host
        .read_eval(
            r#"
            (progn
              (require 'cl-lib)
              (defun a1-square (x) (* x x))
              (byte-compile #'a1-square)
              (list
                (compiled-function-p (symbol-function 'a1-square))
                (a1-square 9)
                (cl-reduce #'+ '(1 2 3 4))
                (json-serialize '((ok . t) (n . 7)))))
            "#,
        )
        .expect("cold loadup stdlib evaluation");
    assert!(stdlib_out.starts_with("(t 81 10 "), "got {stdlib_out}");

    // Script execution with shebang, argv/command-line-args-left, buffer + GC.
    let script_ok = tmp_dir.join("a1_ok_script.el");
    std::fs::write(
        &script_ok,
        r#"#!/usr/bin/env femacs-lisp
(setq a1-seen-left (copy-sequence command-line-args-left))
(setq a1-seen-argv (copy-sequence argv))
(with-current-buffer (get-buffer-create "*a1-script*")
  (erase-buffer)
  (insert (mapconcat #'identity argv ":"))
  (garbage-collect))
"#,
    )
    .expect("write ok script");

    host.load_script(
        &script_ok,
        &[OsString::from("alpha"), OsString::from("beta")],
    )
    .expect("load_script success");

    assert_eq!(
        host.read_eval("(list a1-seen-left a1-seen-argv command-line-args-left argv)")
            .unwrap(),
        r#"(("alpha" "beta") ("alpha" "beta") nil nil)"#
    );
    assert_eq!(
        host.read_eval(r#"(with-current-buffer "*a1-script*" (buffer-string))"#)
            .unwrap(),
        "\"alpha:beta\""
    );

    // Missing script & directory path return HostError::Io.
    let missing_err = host
        .load_script(&tmp_dir.join("missing_script.el"), &[])
        .expect_err("missing script must return Io");
    assert!(matches!(missing_err, HostError::Io { .. }), "got {missing_err:?}");

    let dir_err = host
        .load_script(&tmp_dir, &[])
        .expect_err("directory path must return Io");
    assert!(matches!(dir_err, HostError::Io { .. }), "got {dir_err:?}");

    // Failing script raises LispCondition and still cleans up command-line-args-left.
    let script_bad = tmp_dir.join("a1_bad_script.el");
    std::fs::write(&script_bad, "(error \"script-boom %s\" (car argv))\n")
        .expect("write bad script");
    let script_err = host
        .load_script(&script_bad, &[OsString::from("bad-arg")])
        .expect_err("failing script must return LispCondition");
    assert!(
        matches!(script_err, HostError::LispCondition { ref symbol, ref message, .. }
            if symbol == "error" && message.contains("script-boom bad-arg")),
        "got {script_err:?}"
    );
    assert_eq!(
        host.read_eval("(list (+ 100 23) command-line-args-left argv)").unwrap(),
        "(123 nil nil)"
    );
}

fn scenario_self_dump_create() {
    let lisp_dir = env_path("A1_LISP_DIR");
    let dump_out = env_path("A1_DUMP_OUT");
    let mut host = LispHost::init(HostConfig::cold_loadup(lisp_dir)).expect("cold loadup");
    let _ = host
        .read_eval(
            r#"
            (progn
              (defun a1-dumped-fn (s) (concat "dumped-" s))
              (setq purify-flag nil))
            "#,
        )
        .expect("define preloaded helper");
    let expr = format!(
        "(dump-emacs-portable {:?})",
        dump_out.to_string_lossy().as_ref()
    );
    host.read_eval(&expr).expect("dump-emacs-portable");
    assert!(
        std::fs::metadata(&dump_out).map(|m| m.len()).unwrap_or(0) > 100_000,
        "missing or empty generated pdmp at {}",
        dump_out.display()
    );
}

fn scenario_dump_file_roundtrip() {
    let dump_in = env_path("A1_DUMP_IN");
    let lisp_dir = env_path("A1_LISP_DIR");
    let mut host = LispHost::init(HostConfig::with_dump(dump_in).with_load_path(lisp_dir))
        .expect("dump file init");

    let diag = host.inspect_environment().expect("inspect dump env");
    assert!(diag.dumped_with_pdumper, "expected dumped_with_pdumper=true");
    assert!(
        !diag.pdumper_fingerprint.is_empty(),
        "expected non-empty pdumper_fingerprint"
    );

    assert_eq!(
        host.read_eval(r#"(a1-dumped-fn "ok")"#).unwrap(),
        "\"dumped-ok\""
    );
    let err = host.read_eval("(error \"dump-recover\")").unwrap_err();
    assert!(matches!(err, HostError::LispCondition { .. }));
    assert_eq!(host.read_eval("(+ 20 22)").unwrap(), "42");
}

fn scenario_bad_dump_rejected() {
    let bad_path = env_path("A1_BAD_DUMP");
    let err = match LispHost::init(HostConfig::with_dump(&bad_path)) {
        Ok(_) => panic!("corrupt/mismatched pdmp unexpectedly initialized: {}", bad_path.display()),
        Err(e) => e,
    };
    assert!(
        matches!(err, HostError::InitFailed(_)),
        "expected InitFailed, got {err:?}"
    );
}

fn scenario_second_init_rejected() {
    let host = LispHost::init(HostConfig::cold_minimal()).expect("first init");
    match LispHost::init(HostConfig::cold_minimal()) {
        Ok(_) => panic!("second init while active succeeded"),
        Err(err) => assert_eq!(err, HostError::AlreadyInitialized),
    }
    drop(host);
    assert!(
        femacs_vm::roots::GcContext::current().is_none(),
        "GcContext must remain unacquirable after LispHost shutdown"
    );
    match LispHost::init(HostConfig::cold_minimal()) {
        Ok(_) => panic!("init after shutdown succeeded"),
        Err(err) => assert_eq!(err, HostError::AlreadyInitialized),
    }
}

fn scenario_headless_diagnostics() {
    let mut host = LispHost::init(HostConfig::cold_minimal()).expect("headless init");
    let diag = host.inspect_environment().expect("inspect_environment");
    assert!(diag.noninteractive, "expected noninteractive=true");
    assert!(diag.inhibit_window_system, "expected inhibit_window_system=true");
    assert!(!diag.display_graphic_p, "expected display_graphic_p=false");
    assert_eq!(diag.frame_count, 1, "frames={:?}", diag.frame_types);
    assert_eq!(diag.frame_types, vec!["t"]);
    assert_eq!(diag.terminal_count, 1, "terms={:?}", diag.terminal_types);
    assert_eq!(diag.terminal_types, vec!["t"]);
    assert!(!diag.dumped_with_pdumper);
    assert_eq!(diag.specpdl_depth, 0);

    // Confirm Lisp view matches: single initial frame, not graphic, no GUI window system.
    assert_eq!(
        host.read_eval(
            "(list noninteractive (length (frame-list)) (framep (car (frame-list))) (length (terminal-list)) (terminal-live-p (car (terminal-list))))"
        )
        .unwrap(),
        "(t 1 t 1 t)"
    );
}

fn scenario_safe_domain_protected() {
    let lisp_dir = env_path("A1_LISP_DIR");
    let tmp_dir = env_path("A1_TMP_DIR");
    let tmp_file = tmp_dir.join("a1_safe_domain.txt");
    let config = match std::env::var_os("A1_DUMP_IN") {
        Some(dump_in) => HostConfig::with_dump(PathBuf::from(dump_in)).with_load_path(lisp_dir),
        None => HostConfig::cold_loadup(lisp_dir),
    };
    let mut host = LispHost::init(config).expect("safe-domain host init");

    let summary = host
        .exercise_safe_domains(&tmp_file)
        .expect("safe_domain exercise inside protected host boundary");
    assert!(summary.contains("buf="), "unexpected summary: {summary}");
    assert!(
        summary.contains("eval=(4 9 25 49)"),
        "unexpected summary: {summary}"
    );
    assert!(summary.contains("io_ok=true"), "unexpected summary: {summary}");
    assert!(summary.contains("win=true:"), "unexpected summary: {summary}");

    // Error & quit inside `safe_domain::read_eval_and_format_utf8` must unwind as recoverable
    // `HostError`s without fatal aborts, leaving the host healthy for subsequent calls.
    let err_cond = host
        .read_eval_safe_domain("(signal 'error (list \"safe-domain-boom\"))")
        .expect_err("safe_domain error signal must return LispCondition");
    assert!(
        matches!(
            err_cond,
            HostError::LispCondition {
                ref symbol,
                ref message,
                ..
            } if symbol == "error" && message.contains("safe-domain-boom")
        ),
        "got {err_cond:?}"
    );

    let err_quit1 = host
        .read_eval_safe_domain("(signal 'quit nil)")
        .expect_err("safe_domain quit signal must return Quit");
    assert!(matches!(err_quit1, HostError::Quit { .. }), "got {err_quit1:?}");

    let err_quit2 = host
        .read_eval_safe_domain("(progn (setq quit-flag t) (while t nil))")
        .expect_err("safe_domain quit-flag loop must return Quit");
    assert!(matches!(err_quit2, HostError::Quit { .. }), "got {err_quit2:?}");

    assert_eq!(host.read_eval_safe_domain("(+ 19 23)").unwrap(), "42");

    host.garbage_collect().expect("post safe_domain GC");
    let diag = host.inspect_environment().expect("inspect after safe_domain");
    assert_eq!(diag.specpdl_depth, 0);
    assert_eq!(diag.rooting_counters.persistent_live, 0);
    assert_eq!(
        diag.rooting_counters.rooted_pushed,
        diag.rooting_counters.rooted_popped
    );
    assert_eq!(host.read_eval("(+ 1 2)").unwrap(), "3");
}

fn scenario_host_exit_unwind() {
    let mut host = LispHost::init(HostConfig::cold_minimal()).expect("host-exit-unwind init");

    let err1 = host
        .read_eval("(unwind-protect (kill-emacs 7) (setq bad-cleanup t) (kill-emacs 2))")
        .expect_err("unwind-protect must not swallow kill-emacs exit code");
    assert_eq!(err1, HostError::ExitRequest(7));
    assert_eq!(host.read_eval("(boundp 'bad-cleanup)").unwrap(), "nil");

    let err2 = host
        .read_eval("(condition-case _ (kill-emacs 9) (t 'caught))")
        .expect_err("condition-case t must not catch kill-emacs");
    assert_eq!(err2, HostError::ExitRequest(9));

    host.read_eval("(setq kill-emacs-hook '((lambda () (kill-emacs 11))))")
        .expect("install kill-emacs-hook");
    let err3 = host
        .read_eval("(kill-emacs 3)")
        .expect_err("reentrant kill-emacs inside hook must propagate ExitRequest(11)");
    assert_eq!(err3, HostError::ExitRequest(11));
    host.read_eval("(setq kill-emacs-hook nil)")
        .expect("clear kill-emacs-hook after unwind");

    let err4 = host
        .eval_with_args(
            "(condition-case _ (kill-emacs 12) (t 'caught))",
            false,
            &[OsString::from("--with-arg")],
        )
        .expect_err("eval_with_args must unwind UnbindGuard safely over nested handler");
    assert_eq!(err4, HostError::ExitRequest(12));

    assert_eq!(
        host.read_eval("(let ((x 0)) (unwind-protect (setq x 1) (setq x (+ x 10))) x)")
            .unwrap(),
        "11"
    );
}

fn main() {
    let scenario = std::env::var("A1_SCENARIO").unwrap_or_else(|_| "cold-minimal-lifecycle".into());
    match scenario.as_str() {
        "cold-minimal-lifecycle" => scenario_cold_minimal_lifecycle(),
        "cold-loadup-and-script" => scenario_cold_loadup_and_script(),
        "self-dump-create" => scenario_self_dump_create(),
        "dump-file-roundtrip" => scenario_dump_file_roundtrip(),
        "bad-dump-rejected" => scenario_bad_dump_rejected(),
        "second-init-rejected" => scenario_second_init_rejected(),
        "headless-diagnostics" => scenario_headless_diagnostics(),
        "safe-domain-protected" => scenario_safe_domain_protected(),
        "host-exit-unwind" => scenario_host_exit_unwind(),
        other => panic!("unknown A1_SCENARIO={other}"),
    }
    println!("A1_SCENARIO_OK={scenario}");
}
