#![forbid(unsafe_code)]
//! Line-delimited JSON-RPC 2.0 `http.handle` wire framing and REPL completeness helpers.

use std::io::{self, BufRead, Write};
#[cfg(feature = "vm")]
use std::io::{BufReader, BufWriter};
#[cfg(feature = "vm")]
use std::os::unix::net::UnixStream;
#[cfg(feature = "vm")]
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[cfg(feature = "vm")]
use crate::engine::Engine;

pub const DEFAULT_RPC_ENTRY: &str = "femacs-http-handle";
pub const DEFAULT_MAX_FRAME_BYTES: usize = 4 * 1024 * 1024;

pub const HTTP_RPC_DISPATCH_LISP: &str = r#"
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

#[derive(Debug, Deserialize)]
pub struct RpcRequest {
    pub jsonrpc: String,
    #[serde(default)]
    pub id: JsonValue,
    pub method: String,
    #[serde(default)]
    pub params: JsonValue,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpRequestParams {
    pub method: String,
    pub path: String,
    #[serde(default)]
    pub query: String,
    #[serde(default)]
    pub headers: Vec<[String; 2]>,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub request_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
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
    TooLarge,
    InvalidUtf8,
    Line(String),
}

/// Read one newline-delimited UTF-8 frame up to `max_bytes`.
pub fn read_bounded_line<R: BufRead>(reader: &mut R, max_bytes: usize) -> io::Result<FrameRead> {
    let mut raw = Vec::new();
    let mut oversized = false;
    loop {
        let avail = reader.fill_buf()?;
        if avail.is_empty() {
            if raw.is_empty() && !oversized {
                return Ok(FrameRead::Eof);
            }
            if oversized {
                return Ok(FrameRead::TooLarge);
            }
            return match String::from_utf8(raw) {
                Ok(s) => Ok(FrameRead::Line(s)),
                Err(_) => Ok(FrameRead::InvalidUtf8),
            };
        }
        if let Some(pos) = avail.iter().position(|&b| b == b'\n') {
            let take = pos + 1;
            if !oversized && raw.len().saturating_add(take - 1) <= max_bytes {
                raw.extend_from_slice(&avail[..pos]);
            } else {
                oversized = true;
            }
            reader.consume(take);
            if oversized {
                return Ok(FrameRead::TooLarge);
            }
            if raw.last() == Some(&b'\r') {
                raw.pop();
            }
            return match String::from_utf8(raw) {
                Ok(s) => Ok(FrameRead::Line(s)),
                Err(_) => Ok(FrameRead::InvalidUtf8),
            };
        }
        if !oversized && raw.len().saturating_add(avail.len()) <= max_bytes {
            raw.extend_from_slice(avail);
        } else {
            oversized = true;
        }
        let len = avail.len();
        reader.consume(len);
    }
}

pub fn write_rpc_error<W: Write>(
    writer: &mut W,
    id: JsonValue,
    code: i64,
    message: &str,
    data: Option<JsonValue>,
) -> io::Result<()> {
    let mut err_obj = serde_json::json!({
        "code": code,
        "message": message,
    });
    if let Some(d) = data {
        err_obj["data"] = d;
    }
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "id": id,
        "error": err_obj,
    });
    writeln!(writer, "{payload}")?;
    writer.flush()
}

pub fn write_rpc_result<W: Write, T: Serialize>(
    writer: &mut W,
    id: JsonValue,
    result: &T,
) -> io::Result<()> {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": result,
    });
    writeln!(writer, "{payload}")?;
    writer.flush()
}

/// Return `true` when `src` contains at least one balanced, non-comment S-expression.
pub fn has_complete_lisp_form(src: &str) -> bool {
    let bytes = src.as_bytes();
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
                if i + 1 < bytes.len() {
                    i += 2;
                    continue;
                }
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
                if depth > 0 {
                    depth -= 1;
                }
            }
            _ => {
                saw_token = true;
            }
        }
        i += 1;
    }

    saw_token && depth == 0 && !in_string
}

#[cfg(feature = "vm")]
/// Run the persistent JSON-RPC `http.handle` server over an inherited socketpair.
pub fn serve_http_rpc(
    engine: &mut Engine,
    stream: UnixStream,
    handler: &Path,
    entry: &str,
    max_frame_bytes: usize,
) -> Result<i32, String> {
    engine
        .eval_silent(HTTP_RPC_DISPATCH_LISP)
        .map_err(|e| e.to_string())?;
    let handler_str = handler.to_string_lossy();
    engine
        .load_library(&handler_str)
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
                    JsonValue::Null,
                    -32600,
                    "RPC request frame exceeds maximum allowed size",
                    Some(serde_json::json!({ "maxFrameBytes": max_frame_bytes })),
                )
                .map_err(|e| format!("write rpc error: {e}"))?;
            }
            Ok(FrameRead::InvalidUtf8) => {
                write_rpc_error(
                    &mut writer,
                    JsonValue::Null,
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
                            JsonValue::Null,
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

                let call_res = engine.funcall_two_strings_to_string(
                    "femacs-http--dispatch",
                    entry,
                    &req_json,
                );
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
                                Some(serde_json::json!({ "kind": "invalid-response" })),
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
                            Some(err.to_rpc_data()),
                        )
                        .map_err(|e| format!("write lisp error: {e}"))?;
                    }
                }
            }
            Err(e) => return Err(format!("read rpc socket: {e}")),
        }
    }
}
