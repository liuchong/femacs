#![forbid(unsafe_code)]
//! Pure-Rust HTTP/HTTPS client (`ureq` + `rustls` + `rustls-rustcrypto` + `webpki-roots`)
//! and embedded multi-threaded HTTP/1.1 localhost/TCP server (`std/http`).

use std::collections::{BTreeMap, HashMap, VecDeque};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex, OnceLock, mpsc};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use ureq::Agent;
use url::Url;

use crate::StdHandler;
use crate::value::{
    StdValue, expect_int, expect_opt_map, expect_opt_u64, expect_str, expect_u64,
};

const MAX_HTTP_BODY_BYTES: usize = 64 * 1024 * 1024;
const DEFAULT_HTTP_TIMEOUT_MS: u64 = 30_000;

static TLS_PROVIDER_INIT: OnceLock<()> = OnceLock::new();

/// Install the pure-Rust `rustls-rustcrypto` provider as the process default once.
pub fn install_pure_rust_tls_provider() {
    TLS_PROVIDER_INIT.get_or_init(|| {
        let _ = rustls::crypto::CryptoProvider::install_default(rustls_rustcrypto::provider());
    });
}

fn build_agent(timeout_ms: u64) -> Agent {
    install_pure_rust_tls_provider();
    Agent::config_builder()
        .http_status_as_error(false)
        .timeout_global(Some(Duration::from_millis(timeout_ms)))
        .build()
        .into()
}

fn body_bytes_arg(arg: Option<&StdValue>) -> Result<Option<Vec<u8>>, String> {
    match arg {
        None | Some(StdValue::Nil) => Ok(None),
        Some(other) => other
            .as_bytes()
            .map(|b| Some(b.to_vec()))
            .ok_or_else(|| "expected string, bytes, or nil for HTTP body".to_string()),
    }
}

fn header_value_str(v: &StdValue) -> Result<String, String> {
    match v {
        StdValue::String(s) | StdValue::Symbol(s) => Ok(s.clone()),
        StdValue::Int(n) => Ok(n.to_string()),
        StdValue::Float(f) => Ok(f.to_string()),
        StdValue::Bool(b) => Ok(if *b { "true" } else { "false" }.to_string()),
        _ => Err("HTTP header value must be string/number/bool".to_string()),
    }
}

pub fn http_request(
    method: &str,
    url: &str,
    headers: Option<&BTreeMap<String, StdValue>>,
    body: Option<&[u8]>,
    timeout_ms: u64,
) -> Result<(u16, BTreeMap<String, StdValue>, Vec<u8>, String), String> {
    let agent = build_agent(timeout_ms);
    let upper = method.trim().to_ascii_uppercase();

    let mut response = match upper.as_str() {
        "GET" => {
            let mut req = agent.get(url);
            if let Some(hs) = headers {
                for (k, v) in hs {
                    req = req.header(k.as_str(), header_value_str(v)?);
                }
            }
            req.call()
        }
        "HEAD" => {
            let mut req = agent.head(url);
            if let Some(hs) = headers {
                for (k, v) in hs {
                    req = req.header(k.as_str(), header_value_str(v)?);
                }
            }
            req.call()
        }
        "DELETE" => {
            let mut req = agent.delete(url);
            if let Some(hs) = headers {
                for (k, v) in hs {
                    req = req.header(k.as_str(), header_value_str(v)?);
                }
            }
            req.call()
        }
        "POST" => {
            let mut req = agent.post(url);
            if let Some(hs) = headers {
                for (k, v) in hs {
                    req = req.header(k.as_str(), header_value_str(v)?);
                }
            }
            req.send(body.unwrap_or(&[]))
        }
        "PUT" => {
            let mut req = agent.put(url);
            if let Some(hs) = headers {
                for (k, v) in hs {
                    req = req.header(k.as_str(), header_value_str(v)?);
                }
            }
            req.send(body.unwrap_or(&[]))
        }
        "PATCH" => {
            let mut req = agent.patch(url);
            if let Some(hs) = headers {
                for (k, v) in hs {
                    req = req.header(k.as_str(), header_value_str(v)?);
                }
            }
            req.send(body.unwrap_or(&[]))
        }
        other => return Err(format!("unsupported HTTP method '{other}'")),
    }
    .map_err(|e| format!("HTTP {upper} {url} failed: {e}"))?;

    let status = response.status().as_u16();
    let mut resp_headers = BTreeMap::new();
    for (name, val) in response.headers() {
        if let Ok(s) = val.to_str() {
            resp_headers.insert(
                name.as_str().to_ascii_lowercase(),
                StdValue::String(s.to_string()),
            );
        }
    }

    let bytes = response
        .body_mut()
        .with_config()
        .limit(MAX_HTTP_BODY_BYTES as u64)
        .read_to_vec()
        .map_err(|e| format!("HTTP response body read failed for {url}: {e}"))?;

    Ok((status, resp_headers, bytes, url.to_string()))
}

/// Download raw bytes from `url`, failing on non-2xx HTTP status.
pub fn download_url_bytes(url: &str, timeout_ms: u64) -> Result<Vec<u8>, String> {
    let (status, _, bytes, _) = http_request("GET", url, None, None, timeout_ms)?;
    if !(200..300).contains(&status) {
        return Err(format!("HTTP GET {url} returned status {status}"));
    }
    Ok(bytes)
}

// --- Embedded Pure-Rust HTTP/1.1 Server ---

struct HttpResponsePayload {
    status: u16,
    headers: BTreeMap<String, String>,
    body: Vec<u8>,
}

struct PendingHttpRequest {
    id: u64,
    method: String,
    path: String,
    raw_path: String,
    query: BTreeMap<String, StdValue>,
    headers: BTreeMap<String, StdValue>,
    body: Vec<u8>,
}

struct ServerQueue {
    items: Mutex<VecDeque<(PendingHttpRequest, mpsc::Sender<HttpResponsePayload>)>>,
    cv: Condvar,
    closed: AtomicBool,
}

impl ServerQueue {
    fn new() -> Self {
        Self {
            items: Mutex::new(VecDeque::new()),
            cv: Condvar::new(),
            closed: AtomicBool::new(false),
        }
    }

    fn push(&self, req: PendingHttpRequest, tx: mpsc::Sender<HttpResponsePayload>) {
        if self.closed.load(Ordering::Acquire) {
            return;
        }
        let mut g = self.items.lock().unwrap_or_else(|e| e.into_inner());
        g.push_back((req, tx));
        self.cv.notify_one();
    }

    fn pop_timeout(
        &self,
        timeout_ms: u64,
    ) -> Option<(PendingHttpRequest, mpsc::Sender<HttpResponsePayload>)> {
        let deadline = Instant::now() + Duration::from_millis(timeout_ms);
        let mut g = self.items.lock().unwrap_or_else(|e| e.into_inner());
        loop {
            if let Some(pair) = g.pop_front() {
                return Some(pair);
            }
            if self.closed.load(Ordering::Acquire) || timeout_ms == 0 {
                return None;
            }
            let now = Instant::now();
            if now >= deadline {
                return None;
            }
            let (guard, _) = self
                .cv
                .wait_timeout(g, deadline.saturating_duration_since(now))
                .unwrap_or_else(|e| e.into_inner());
            g = guard;
        }
    }

    fn close(&self) {
        self.closed.store(true, Ordering::Release);
        let mut g = self.items.lock().unwrap_or_else(|e| e.into_inner());
        g.clear();
        self.cv.notify_all();
    }
}

struct EmbeddedHttpServer {
    addr: String,
    port: u16,
    stop: Arc<AtomicBool>,
    queue: Arc<ServerQueue>,
    responders: Mutex<HashMap<u64, mpsc::Sender<HttpResponsePayload>>>,
    next_req_id: AtomicU64,
    accept_thread: Mutex<Option<JoinHandle<()>>>,
}

impl EmbeddedHttpServer {
    fn start(bind_addr: &str) -> Result<Arc<Self>, String> {
        let listener = TcpListener::bind(bind_addr)
            .map_err(|e| format!("failed to bind HTTP server on '{bind_addr}': {e}"))?;
        listener
            .set_nonblocking(true)
            .map_err(|e| format!("failed to configure nonblocking listener: {e}"))?;
        let local = listener
            .local_addr()
            .map_err(|e| format!("failed to resolve local server addr: {e}"))?;

        let stop = Arc::new(AtomicBool::new(false));
        let queue = Arc::new(ServerQueue::new());
        let srv = Arc::new(Self {
            addr: local.to_string(),
            port: local.port(),
            stop: Arc::clone(&stop),
            queue: Arc::clone(&queue),
            responders: Mutex::new(HashMap::new()),
            next_req_id: AtomicU64::new(1),
            accept_thread: Mutex::new(None),
        });

        let srv_bg = Arc::clone(&srv);
        let handle = thread::spawn(move || {
            while !srv_bg.stop.load(Ordering::Acquire) {
                match listener.accept() {
                    Ok((stream, _peer)) => {
                        let srv_conn = Arc::clone(&srv_bg);
                        thread::spawn(move || {
                            handle_http_connection(srv_conn, stream);
                        });
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(15));
                    }
                    Err(_) => break,
                }
            }
        });

        *srv.accept_thread.lock().unwrap_or_else(|e| e.into_inner()) = Some(handle);
        Ok(srv)
    }

    fn stop_and_join(&self) {
        self.stop.store(true, Ordering::Release);
        self.queue.close();
        self.responders
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clear();
        let h = self
            .accept_thread
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .take();
        if let Some(handle) = h {
            let _ = handle.join();
        }
    }
}

fn reason_phrase(status: u16) -> &'static str {
    match status {
        200 => "OK",
        201 => "Created",
        204 => "No Content",
        301 => "Moved Permanently",
        302 => "Found",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        500 => "Internal Server Error",
        503 => "Service Unavailable",
        _ => "OK",
    }
}

fn parse_raw_target(raw_target: &str) -> (String, BTreeMap<String, StdValue>) {
    let synthetic = format!("http://localhost{raw_target}");
    if let Ok(parsed) = Url::parse(&synthetic) {
        let mut q = BTreeMap::new();
        for (k, v) in parsed.query_pairs() {
            q.insert(k.into_owned(), StdValue::String(v.into_owned()));
        }
        return (parsed.path().to_string(), q);
    }
    let path_only = raw_target
        .split_once('?')
        .map(|(p, _)| p)
        .unwrap_or(raw_target);
    (path_only.to_string(), BTreeMap::new())
}

fn handle_http_connection(srv: Arc<EmbeddedHttpServer>, stream: TcpStream) {
    let _ = stream.set_nonblocking(false);
    let _ = stream.set_read_timeout(Some(Duration::from_secs(10)));
    let _ = stream.set_write_timeout(Some(Duration::from_secs(10)));

    let Ok(write_clone) = stream.try_clone() else {
        return;
    };
    let mut reader = BufReader::new(stream);
    let mut request_line = String::new();
    if reader.read_line(&mut request_line).unwrap_or(0) == 0 {
        return;
    }
    let parts: Vec<&str> = request_line.trim_end().split_whitespace().collect();
    if parts.len() < 2 {
        return;
    }
    let method = parts[0].to_ascii_uppercase();
    let raw_path = parts[1].to_string();
    let (path, query) = parse_raw_target(&raw_path);

    let mut headers = BTreeMap::new();
    let mut content_len: usize = 0;
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => {
                let trimmed = line.trim_end_matches(&['\r', '\n'][..]);
                if trimmed.is_empty() {
                    break;
                }
                if let Some((k, v)) = trimmed.split_once(':') {
                    let lower_k = k.trim().to_ascii_lowercase();
                    let val = v.trim().to_string();
                    if lower_k == "content-length" {
                        if let Ok(n) = val.parse::<usize>() {
                            content_len = n.min(MAX_HTTP_BODY_BYTES);
                        }
                    }
                    headers.insert(lower_k, StdValue::String(val));
                }
            }
        }
    }

    let mut body = vec![0u8; content_len];
    if content_len > 0 && reader.read_exact(&mut body).is_err() {
        return;
    }

    let req_id = srv.next_req_id.fetch_add(1, Ordering::Relaxed);
    let (tx, rx) = mpsc::channel::<HttpResponsePayload>();
    let req = PendingHttpRequest {
        id: req_id,
        method,
        path,
        raw_path,
        query,
        headers,
        body,
    };
    srv.queue.push(req, tx);

    let payload = match rx.recv_timeout(Duration::from_secs(30)) {
        Ok(p) => p,
        Err(_) => HttpResponsePayload {
            status: 503,
            headers: BTreeMap::new(),
            body: b"request timed out or server stopped".to_vec(),
        },
    };
    write_http_response(write_clone, &payload);
}

fn write_http_response(mut stream: TcpStream, payload: &HttpResponsePayload) {
    let mut head = format!(
        "HTTP/1.1 {} {}\r\nContent-Length: {}\r\nConnection: close\r\n",
        payload.status,
        reason_phrase(payload.status),
        payload.body.len()
    );
    for (k, v) in &payload.headers {
        let lower = k.to_ascii_lowercase();
        if lower == "content-length" || lower == "connection" || lower == "transfer-encoding" {
            continue;
        }
        let clean_k = k.replace(['\r', '\n'], "");
        let clean_v = v.replace(['\r', '\n'], "");
        head.push_str(&format!("{clean_k}: {clean_v}\r\n"));
    }
    head.push_str("\r\n");
    let _ = stream.write_all(head.as_bytes());
    let _ = stream.write_all(&payload.body);
    let _ = stream.flush();
}

pub struct HttpServerRegistry {
    next_id: AtomicU64,
    servers: Mutex<HashMap<u64, Arc<EmbeddedHttpServer>>>,
}

impl HttpServerRegistry {
    pub fn new() -> Self {
        Self {
            next_id: AtomicU64::new(1),
            servers: Mutex::new(HashMap::new()),
        }
    }

    fn get(&self, id: u64) -> Result<Arc<EmbeddedHttpServer>, String> {
        self.servers
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .get(&id)
            .cloned()
            .ok_or_else(|| format!("invalid HTTP server handle {id}"))
    }

    pub fn shutdown_all(&self) {
        let all: Vec<Arc<EmbeddedHttpServer>> = {
            let mut g = self.servers.lock().unwrap_or_else(|e| e.into_inner());
            g.drain().map(|(_, s)| s).collect()
        };
        for srv in all {
            srv.stop_and_join();
        }
    }
}

pub(crate) fn register(registry: &Arc<HttpServerRegistry>, mut reg: impl FnMut(&str, StdHandler)) {
    reg(
        "%std-http-request",
        Arc::new(|args| {
            let method = expect_str(args.first(), "method")?;
            let url = expect_str(args.get(1), "url")?;
            let headers = expect_opt_map(args.get(2), "headers")?;
            let body = body_bytes_arg(args.get(3))?;
            let timeout_ms = expect_opt_u64(args.get(4), "timeout-ms")?.unwrap_or(DEFAULT_HTTP_TIMEOUT_MS);

            let (status, resp_headers, resp_bytes, final_url) =
                http_request(method, url, headers.as_ref(), body.as_deref(), timeout_ms)?;

            let mut out = BTreeMap::new();
            out.insert("status".to_string(), StdValue::Int(i64::from(status)));
            out.insert("ok".to_string(), StdValue::Bool((200..300).contains(&status)));
            out.insert("headers".to_string(), StdValue::Map(resp_headers));
            out.insert(
                "body".to_string(),
                StdValue::String(String::from_utf8_lossy(&resp_bytes).into_owned()),
            );
            out.insert("body-bytes".to_string(), StdValue::Bytes(resp_bytes));
            out.insert("url".to_string(), StdValue::String(final_url));
            Ok(StdValue::Map(out))
        }),
    );

    let reg_start = Arc::clone(registry);
    reg(
        "%std-http-server-start",
        Arc::new(move |args| {
            let bind_addr = expect_str(args.first(), "bind-addr")?;
            let srv = EmbeddedHttpServer::start(bind_addr)?;
            let id = reg_start.next_id.fetch_add(1, Ordering::Relaxed);
            let mut out = BTreeMap::new();
            out.insert("id".to_string(), StdValue::Int(id as i64));
            out.insert("addr".to_string(), StdValue::String(srv.addr.clone()));
            out.insert("port".to_string(), StdValue::Int(i64::from(srv.port)));
            reg_start
                .servers
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .insert(id, srv);
            Ok(StdValue::Map(out))
        }),
    );

    let reg_accept = Arc::clone(registry);
    reg(
        "%std-http-server-accept",
        Arc::new(move |args| {
            let srv_id = expect_u64(args.first(), "server-id")?;
            let timeout_ms = expect_opt_u64(args.get(1), "timeout-ms")?.unwrap_or(5_000);
            let srv = reg_accept.get(srv_id)?;
            let Some((req, tx)) = srv.queue.pop_timeout(timeout_ms) else {
                return Ok(StdValue::Nil);
            };
            srv.responders
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .insert(req.id, tx);

            let mut out = BTreeMap::new();
            out.insert("id".to_string(), StdValue::Int(req.id as i64));
            out.insert("method".to_string(), StdValue::String(req.method));
            out.insert("path".to_string(), StdValue::String(req.path));
            out.insert("raw-path".to_string(), StdValue::String(req.raw_path));
            out.insert("query".to_string(), StdValue::Map(req.query));
            out.insert("headers".to_string(), StdValue::Map(req.headers));
            out.insert(
                "body".to_string(),
                StdValue::String(String::from_utf8_lossy(&req.body).into_owned()),
            );
            out.insert("body-bytes".to_string(), StdValue::Bytes(req.body));
            Ok(StdValue::Map(out))
        }),
    );

    let reg_respond = Arc::clone(registry);
    reg(
        "%std-http-server-respond",
        Arc::new(move |args| {
            let srv_id = expect_u64(args.first(), "server-id")?;
            let req_id = expect_u64(args.get(1), "req-id")?;
            let status_raw = expect_int(args.get(2), "status")?;
            let status = u16::try_from(status_raw)
                .map_err(|_| format!("invalid HTTP status code {status_raw}"))?;
            let headers_opt = expect_opt_map(args.get(3), "headers")?;
            let body = body_bytes_arg(args.get(4))?.unwrap_or_default();

            let mut headers = BTreeMap::new();
            if let Some(map) = headers_opt {
                for (k, v) in map {
                    headers.insert(k, header_value_str(&v)?);
                }
            }

            let srv = reg_respond.get(srv_id)?;
            let tx = srv
                .responders
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .remove(&req_id)
                .ok_or_else(|| format!("unknown or already-responded request {req_id}"))?;
            let _ = tx.send(HttpResponsePayload {
                status,
                headers,
                body,
            });
            Ok(StdValue::Bool(true))
        }),
    );

    let reg_stop = Arc::clone(registry);
    reg(
        "%std-http-server-stop",
        Arc::new(move |args| {
            let srv_id = expect_u64(args.first(), "server-id")?;
            let removed = reg_stop
                .servers
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .remove(&srv_id);
            if let Some(srv) = removed {
                srv.stop_and_join();
                Ok(StdValue::Bool(true))
            } else {
                Ok(StdValue::Nil)
            }
        }),
    );
}
