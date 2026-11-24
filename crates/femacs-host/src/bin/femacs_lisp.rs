#![forbid(unsafe_code)]
//! Standalone relocatable Lisp CLI, REPL, and JSON-RPC 2.0 `--rpc http` host (`C2`).

use std::env;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::process;

use femacs_host::lisp_cli::{
    BatchAction, FrameRead, HttpRequestParams, HttpResponseResult, ParsedCli, RpcMode, RpcRequest,
    RunMode, build_host_config, has_complete_lisp_form, host_error_to_rpc_data, parse_cli_args,
    read_bounded_line, resolve_process_paths, write_rpc_error, write_rpc_result,
};
use femacs_host::{HostError, LispHost, adopt_inherited_rpc_socket};
use serde_json::{Value, json};

const HTTP_RPC_DISPATCH_LISP: &str = r#"
(progn
  (defun femacs-http--normalize-header (entry)
    (cond
     ((and (vectorp entry) (>= (length entry) 2))
      (vector (format "%s" (aref entry 0)) (format "%s" (aref entry 1))))
     ((consp entry)
      (let* ((k (car entry))
             (k-str (if (symbolp k) (symbol-name k) (format "%s" k)))
             (raw-v (cdr entry))
             (v-str (if (and (consp raw-v) (null (cdr raw-v)))
                        (format "%s" (car raw-v))
                      (format "%s" (if raw-v raw-v "")))))
        (vector k-str v-str)))))

  (defun femacs-http--dispatch (entry-name req-json)
    (let* ((req (json-parse-string req-json
                                   :object-type 'alist
                                   :array-type 'list
                                   :null-object nil
                                   :false-object :json-false))
           (fn (intern entry-name))
           (_check (unless (fboundp fn)
                     (signal 'void-function (list fn))))
           (resp (funcall fn req))
           (status (or (alist-get 'status resp) 200))
           (raw-headers (alist-get 'headers resp))
           (headers (if raw-headers
                        (apply #'vector
                               (delq nil (mapcar #'femacs-http--normalize-header raw-headers)))
                      (vector)))
           (raw-body (alist-get 'body resp))
           (body-str (cond
                      ((null raw-body) "")
                      ((stringp raw-body) raw-body)
                      (t (format "%s" raw-body)))))
      (json-serialize (list (cons 'status status)
                            (cons 'headers headers)
                            (cons 'body body-str))))))
"#;

fn exit_for_host_error(err: &HostError) -> i32 {
    if let HostError::ExitRequest(code) = err {
        return *code;
    }
    eprintln!("femacs-lisp: {err}");
    1
}

fn run_actions(host: &mut LispHost, batch: bool, actions: &[BatchAction]) -> Result<i32, HostError> {
    let last_eval_idx = actions
        .iter()
        .rposition(|a| matches!(a, BatchAction::Eval(_)));

    for (idx, action) in actions.iter().enumerate() {
        match action {
            BatchAction::AddLoadPath(dir) => host.prepend_load_path(dir)?,
            BatchAction::Load(file) => host.load_library(file)?,
            BatchAction::Eval(form) => {
                if !batch && Some(idx) == last_eval_idx {
                    let result = host.read_eval(form)?;
                    println!("{result}");
                } else {
                    host.eval_silent(form)?;
                }
            }
            BatchAction::Funcall { func, args } => host.funcall_with_args(func, args)?,
            BatchAction::Script { path, args } => host.load_script(path, args)?,
        }
    }
    let _ = io::stdout().flush();
    Ok(0)
}

fn run_repl(host: &mut LispHost, actions: &[BatchAction]) -> Result<i32, HostError> {
    for action in actions {
        match action {
            BatchAction::AddLoadPath(dir) => host.prepend_load_path(dir)?,
            BatchAction::Load(file) => host.load_library(file)?,
            BatchAction::Eval(_)
            | BatchAction::Funcall { .. }
            | BatchAction::Script { .. } => {}
        }
    }

    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut buffer = String::new();
    let mut line = String::new();

    loop {
        line.clear();
        let bytes = match reader.read_line(&mut line) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("femacs-lisp repl read error: {e}");
                return Ok(1);
            }
        };
        if bytes == 0 {
            if !buffer.trim().is_empty() {
                match host.read_eval(&buffer) {
                    Ok(out) => {
                        println!("{out}");
                        let _ = io::stdout().flush();
                    }
                    Err(HostError::ExitRequest(code)) => return Ok(code),
                    Err(err) => {
                        eprintln!("error: {err}");
                        let _ = io::stderr().flush();
                    }
                }
            }
            return Ok(0);
        }

        buffer.push_str(&line);
        if buffer.trim().is_empty() {
            buffer.clear();
            continue;
        }
        if !has_complete_lisp_form(&buffer) {
            continue;
        }

        match host.read_eval(&buffer) {
            Ok(out) => {
                println!("{out}");
                let _ = io::stdout().flush();
            }
            Err(HostError::ExitRequest(code)) => return Ok(code),
            Err(err) => {
                eprintln!("error: {err}");
                let _ = io::stderr().flush();
            }
        }
        buffer.clear();
    }
}

fn run_http_rpc(
    host: &mut LispHost,
    actions: &[BatchAction],
    stream: UnixStream,
    handler: &Path,
    entry: &str,
    max_frame_bytes: usize,
) -> Result<i32, String> {
    for action in actions {
        match action {
            BatchAction::AddLoadPath(dir) => {
                host.prepend_load_path(dir).map_err(|e| e.to_string())?;
            }
            BatchAction::Load(file) => {
                host.load_library(file).map_err(|e| e.to_string())?;
            }
            BatchAction::Eval(form) => {
                host.eval_silent(form).map_err(|e| e.to_string())?;
            }
            BatchAction::Script { .. } | BatchAction::Funcall { .. } => {}
        }
    }

    host.eval_silent(HTTP_RPC_DISPATCH_LISP)
        .map_err(|e| e.to_string())?;
    let handler_str = handler.to_string_lossy();
    host.load_library(&handler_str)
        .map_err(|e| format!("failed to load --rpc-handler {}: {e}", handler.display()))?;

    let read_sock = stream
        .try_clone()
        .map_err(|e| format!("clone rpc socket: {e}"))?;
    let mut reader = BufReader::new(read_sock);
    let mut writer = BufWriter::new(stream);

    loop {
        match read_bounded_line(&mut reader, max_frame_bytes) {
            Ok(FrameRead::Eof) => return Ok(0),
            Ok(FrameRead::TooLarge) => {
                write_rpc_error(
                    &mut writer,
                    Value::Null,
                    -32600,
                    "RPC request frame exceeds maximum allowed size",
                    Some(json!({ "maxFrameBytes": max_frame_bytes })),
                )
                .map_err(|e| format!("write rpc error: {e}"))?;
            }
            Ok(FrameRead::InvalidUtf8) => {
                write_rpc_error(
                    &mut writer,
                    Value::Null,
                    -32700,
                    "RPC request frame is not valid UTF-8",
                    None,
                )
                .map_err(|e| format!("write rpc error: {e}"))?;
            }
            Ok(FrameRead::Line(line)) => {
                if line.trim().is_empty() {
                    continue;
                }
                let req: RpcRequest = match serde_json::from_str(&line) {
                    Ok(r) => r,
                    Err(e) => {
                        write_rpc_error(
                            &mut writer,
                            Value::Null,
                            -32700,
                            &format!("Invalid JSON-RPC request: {e}"),
                            None,
                        )
                        .map_err(|io_e| format!("write parse error: {io_e}"))?;
                        continue;
                    }
                };
                let req_id = req.id.clone();
                if req.jsonrpc != "2.0" {
                    write_rpc_error(
                        &mut writer,
                        req_id,
                        -32600,
                        "Unsupported JSON-RPC version; expected \"2.0\"",
                        None,
                    )
                    .map_err(|e| format!("write version error: {e}"))?;
                    continue;
                }
                if req.method != "http.handle" {
                    write_rpc_error(
                        &mut writer,
                        req_id,
                        -32601,
                        &format!("Method not found: {}", req.method),
                        None,
                    )
                    .map_err(|e| format!("write method error: {e}"))?;
                    continue;
                }
                let params: HttpRequestParams = match serde_json::from_value(req.params) {
                    Ok(p) => p,
                    Err(e) => {
                        write_rpc_error(
                            &mut writer,
                            req_id,
                            -32602,
                            &format!("Invalid http.handle params: {e}"),
                            None,
                        )
                        .map_err(|io_e| format!("write param error: {io_e}"))?;
                        continue;
                    }
                };
                let req_json = serde_json::to_string(&params)
                    .map_err(|e| format!("serialize http params: {e}"))?;

                let call_res =
                    host.funcall_two_strings_to_string("femacs-http--dispatch", entry, &req_json);
                let _ = io::stdout().flush();
                let _ = io::stderr().flush();

                match call_res {
                    Ok(resp_json) => match serde_json::from_str::<HttpResponseResult>(&resp_json) {
                        Ok(parsed) => {
                            write_rpc_result(&mut writer, req_id, &parsed)
                                .map_err(|e| format!("write rpc result: {e}"))?;
                        }
                        Err(e) => {
                            write_rpc_error(
                                &mut writer,
                                req_id,
                                -32000,
                                &format!("Invalid Lisp handler response JSON: {e}"),
                                Some(json!({ "kind": "invalid-response" })),
                            )
                            .map_err(|io_e| format!("write response error: {io_e}"))?;
                        }
                    },
                    Err(err) => {
                        write_rpc_error(
                            &mut writer,
                            req_id,
                            -32000,
                            &err.to_string(),
                            Some(host_error_to_rpc_data(&err)),
                        )
                        .map_err(|e| format!("write lisp error: {e}"))?;
                    }
                }
            }
            Err(e) => return Err(format!("read rpc socket: {e}")),
        }
    }
}

fn run_main() -> i32 {
    let raw_args: Vec<_> = env::args_os().skip(1).collect();
    let cli: ParsedCli = match parse_cli_args(&raw_args) {
        Ok(c) => c,
        Err(exit) => {
            match &exit {
                femacs_host::lisp_cli::CliExit::Info(msg) => println!("{msg}"),
                femacs_host::lisp_cli::CliExit::Usage(msg) => eprintln!("femacs-lisp: {msg}"),
            }
            return exit.code();
        }
    };

    let rpc_stream = match &cli.mode {
        RunMode::Rpc(RpcMode::Http { fd, .. }) => match adopt_inherited_rpc_socket(*fd) {
            Ok(s) => Some(s),
            Err(e) => {
                eprintln!("femacs-lisp: failed to adopt inherited RPC socketpair: {e}");
                return 2;
            }
        },
        _ => None,
    };

    let resolved = match resolve_process_paths(&cli) {
        Ok(r) => r,
        Err(msg) => {
            eprintln!("femacs-lisp: {msg}");
            return 1;
        }
    };
    let config = build_host_config(&cli, &resolved);
    let mut host = match LispHost::init(config) {
        Ok(h) => h,
        Err(err) => return exit_for_host_error(&err),
    };

    match cli.mode {
        RunMode::DumpOut(out_path) => match host.dump_portable(&out_path) {
            Ok(()) => 0,
            Err(err) => exit_for_host_error(&err),
        },
        RunMode::Actions => match run_actions(&mut host, cli.batch, &cli.actions) {
            Ok(code) => code,
            Err(err) => exit_for_host_error(&err),
        },
        RunMode::Repl => match run_repl(&mut host, &cli.actions) {
            Ok(code) => code,
            Err(err) => exit_for_host_error(&err),
        },
        RunMode::Rpc(RpcMode::Http {
            handler,
            entry,
            max_frame_bytes,
            ..
        }) => {
            let stream = rpc_stream.expect("validated RPC socketpair");
            match run_http_rpc(
                &mut host,
                &cli.actions,
                stream,
                &handler,
                &entry,
                max_frame_bytes,
            ) {
                Ok(code) => code,
                Err(msg) => {
                    eprintln!("femacs-lisp rpc error: {msg}");
                    1
                }
            }
        }
    }
}

fn main() {
    let code = run_main();
    let _ = io::stdout().flush();
    let _ = io::stderr().flush();
    process::exit(code);
}
