#![forbid(unsafe_code)]
//! Pure-Rust regex engine, bounded subprocess runner, and `.env` / environment helpers
//! (`std/regex`, `std/proc`, `std/env`).

use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use regex::Regex;

use crate::StdHandler;
use crate::value::{StdValue, expect_list, expect_opt_map, expect_opt_str, expect_str};

pub fn parse_dotenv(text: &str) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for raw_line in text.lines() {
        let trimmed = raw_line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let line = trimmed
            .strip_prefix("export ")
            .map(str::trim_start)
            .unwrap_or(trimmed);
        let Some((raw_key, raw_val)) = line.split_once('=') else {
            continue;
        };
        let key = raw_key.trim();
        if key.is_empty() {
            continue;
        }
        let val_trimmed = raw_val.trim();
        let val = if let Some(inner) = val_trimmed
            .strip_prefix('"')
            .and_then(|s| s.strip_suffix('"'))
        {
            inner
                .replace("\\n", "\n")
                .replace("\\r", "\r")
                .replace("\\t", "\t")
                .replace("\\\"", "\"")
                .replace("\\\\", "\\")
        } else if let Some(inner) = val_trimmed
            .strip_prefix('\'')
            .and_then(|s| s.strip_suffix('\''))
        {
            inner.to_string()
        } else {
            val_trimmed
                .split_once(" #")
                .map(|(v, _)| v.trim_end())
                .unwrap_or(val_trimmed)
                .to_string()
        };
        out.insert(key.to_string(), val);
    }
    out
}

pub(crate) fn register(mut reg: impl FnMut(&str, StdHandler)) {
    // --- Regex (`std/regex`) ---
    reg(
        "%std-regex-match-p",
        Arc::new(|args| {
            let pat = expect_str(args.first(), "pattern")?;
            let text = expect_str(args.get(1), "text")?;
            let re = Regex::new(pat).map_err(|e| format!("invalid regex '{pat}': {e}"))?;
            Ok(StdValue::Bool(re.is_match(text)))
        }),
    );

    reg(
        "%std-regex-find",
        Arc::new(|args| {
            let pat = expect_str(args.first(), "pattern")?;
            let text = expect_str(args.get(1), "text")?;
            let re = Regex::new(pat).map_err(|e| format!("invalid regex '{pat}': {e}"))?;
            match re.find(text) {
                Some(m) => Ok(StdValue::String(m.as_str().to_string())),
                None => Ok(StdValue::Nil),
            }
        }),
    );

    reg(
        "%std-regex-find-all",
        Arc::new(|args| {
            let pat = expect_str(args.first(), "pattern")?;
            let text = expect_str(args.get(1), "text")?;
            let re = Regex::new(pat).map_err(|e| format!("invalid regex '{pat}': {e}"))?;
            let items: Vec<StdValue> = re
                .find_iter(text)
                .map(|m| StdValue::String(m.as_str().to_string()))
                .collect();
            Ok(StdValue::List(items))
        }),
    );

    reg(
        "%std-regex-captures",
        Arc::new(|args| {
            let pat = expect_str(args.first(), "pattern")?;
            let text = expect_str(args.get(1), "text")?;
            let re = Regex::new(pat).map_err(|e| format!("invalid regex '{pat}': {e}"))?;
            match re.captures(text) {
                Some(caps) => {
                    let items: Vec<StdValue> = caps
                        .iter()
                        .map(|m| match m {
                            Some(g) => StdValue::String(g.as_str().to_string()),
                            None => StdValue::Nil,
                        })
                        .collect();
                    Ok(StdValue::List(items))
                }
                None => Ok(StdValue::Nil),
            }
        }),
    );

    reg(
        "%std-regex-replace",
        Arc::new(|args| {
            let pat = expect_str(args.first(), "pattern")?;
            let text = expect_str(args.get(1), "text")?;
            let replacement = expect_str(args.get(2), "replacement")?;
            let all = args.get(3).is_some_and(StdValue::is_truthy);
            let re = Regex::new(pat).map_err(|e| format!("invalid regex '{pat}': {e}"))?;
            let res = if all {
                re.replace_all(text, replacement)
            } else {
                re.replace(text, replacement)
            };
            Ok(StdValue::String(res.into_owned()))
        }),
    );

    reg(
        "%std-regex-split",
        Arc::new(|args| {
            let pat = expect_str(args.first(), "pattern")?;
            let text = expect_str(args.get(1), "text")?;
            let re = Regex::new(pat).map_err(|e| format!("invalid regex '{pat}': {e}"))?;
            let items: Vec<StdValue> = re
                .split(text)
                .map(|part| StdValue::String(part.to_string()))
                .collect();
            Ok(StdValue::List(items))
        }),
    );

    // --- Subprocess (`std/proc`) ---
    reg(
        "%std-proc-run",
        Arc::new(|args| {
            let program = expect_str(args.first(), "program")?;
            let argv = expect_list(args.get(1), "args")?;
            let opts = expect_opt_map(args.get(2), "options")?.unwrap_or_default();

            let mut cmd = Command::new(program);
            for arg in argv {
                let s = arg
                    .as_str()
                    .ok_or_else(|| "std-proc-run args must be strings/symbols".to_string())?;
                cmd.arg(s);
            }

            if let Some(cwd) = opts.get("cwd").and_then(StdValue::as_str) {
                cmd.current_dir(cwd);
            }
            if let Some(StdValue::Map(env_map)) = opts.get("env") {
                for (k, v) in env_map {
                    if let Some(vs) = v.as_str() {
                        cmd.env(k, vs);
                    }
                }
            }

            let stdin_data = opts
                .get("stdin")
                .and_then(StdValue::as_bytes)
                .map(|b| b.to_vec());
            let timeout_ms = opts
                .get("timeout-ms")
                .and_then(StdValue::as_int)
                .filter(|&ms| ms > 0)
                .map(|ms| Duration::from_millis(ms as u64));

            cmd.stdin(if stdin_data.is_some() {
                Stdio::piped()
            } else {
                Stdio::null()
            })
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

            let mut child = cmd
                .spawn()
                .map_err(|e| format!("failed to spawn '{program}': {e}"))?;

            if let Some(bytes) = stdin_data {
                if let Some(mut stdin) = child.stdin.take() {
                    let _ = stdin.write_all(&bytes);
                }
            }

            let mut stdout_pipe = child.stdout.take();
            let mut stderr_pipe = child.stderr.take();
            let out_handle = thread::spawn(move || {
                let mut buf = Vec::new();
                if let Some(ref mut r) = stdout_pipe {
                    let _ = r.read_to_end(&mut buf);
                }
                buf
            });
            let err_handle = thread::spawn(move || {
                let mut buf = Vec::new();
                if let Some(ref mut r) = stderr_pipe {
                    let _ = r.read_to_end(&mut buf);
                }
                buf
            });

            let mut timed_out = false;
            let status = if let Some(limit) = timeout_ms {
                let deadline = Instant::now() + limit;
                loop {
                    match child.try_wait() {
                        Ok(Some(st)) => break st,
                        Ok(None) => {
                            if Instant::now() >= deadline {
                                timed_out = true;
                                let _ = child.kill();
                                break child
                                    .wait()
                                    .map_err(|e| format!("wait after timeout kill failed: {e}"))?;
                            }
                            thread::sleep(Duration::from_millis(10));
                        }
                        Err(e) => return Err(format!("process wait error: {e}")),
                    }
                }
            } else {
                child
                    .wait()
                    .map_err(|e| format!("process wait error: {e}"))?
            };

            let stdout_bytes = out_handle.join().unwrap_or_default();
            let stderr_bytes = err_handle.join().unwrap_or_default();
            let code = status.code().unwrap_or(-1);

            let mut out = BTreeMap::new();
            out.insert("code".to_string(), StdValue::Int(i64::from(code)));
            out.insert(
                "ok".to_string(),
                StdValue::Bool(!timed_out && status.success()),
            );
            out.insert("timed-out".to_string(), StdValue::Bool(timed_out));
            out.insert(
                "stdout".to_string(),
                StdValue::String(String::from_utf8_lossy(&stdout_bytes).into_owned()),
            );
            out.insert(
                "stderr".to_string(),
                StdValue::String(String::from_utf8_lossy(&stderr_bytes).into_owned()),
            );
            Ok(StdValue::Map(out))
        }),
    );

    // --- Environment (`std/env`) ---
    reg(
        "%std-env-get",
        Arc::new(|args| {
            let key = expect_str(args.first(), "key")?;
            let default = expect_opt_str(args.get(1), "default")?;
            match std::env::var(key) {
                Ok(v) => Ok(StdValue::String(v)),
                Err(_) => match default {
                    Some(d) => Ok(StdValue::String(d.to_string())),
                    None => Ok(StdValue::Nil),
                },
            }
        }),
    );

    reg(
        "%std-env-all",
        Arc::new(|_| {
            let mut out = BTreeMap::new();
            for (k, v) in std::env::vars() {
                out.insert(k, StdValue::String(v));
            }
            Ok(StdValue::Map(out))
        }),
    );

    reg(
        "%std-env-parse-dotenv",
        Arc::new(|args| {
            let text = expect_str(args.first(), "text")?;
            let parsed = parse_dotenv(text);
            let mut out = BTreeMap::new();
            for (k, v) in parsed {
                out.insert(k, StdValue::String(v));
            }
            Ok(StdValue::Map(out))
        }),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_dotenv_supports_quotes_exports_comments_and_escapes() {
        let src = r#"
# comment
PORT=8080 # inline comment
export APP_NAME="hello\nworld"
SINGLE='raw # not comment'
EMPTY=
"#;
        let map = parse_dotenv(src);
        assert_eq!(map.get("PORT").map(String::as_str), Some("8080"));
        assert_eq!(
            map.get("APP_NAME").map(String::as_str),
            Some("hello\nworld")
        );
        assert_eq!(
            map.get("SINGLE").map(String::as_str),
            Some("raw # not comment")
        );
        assert_eq!(map.get("EMPTY").map(String::as_str), Some(""));
    }
}
