#![forbid(unsafe_code)]
//! Pure-Rust multi-core concurrency runtime (`std/chan`, `std/sync`, `std/task`) for `lisp`.
//!
//! Backed by isolated per-thread `NoGil` VMs (`isolate_vm`), channels, atomics, shared maps,
//! tasks, and a bounded pre-warmed worker pool without `unsafe` or C dependencies.

use std::collections::{BTreeMap, HashMap, VecDeque};
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use lisp_vm::ConcurrencyMode;

use crate::engine::{Engine, EngineConfig};
use crate::error::Error;
use crate::value::Value;

pub type WorkerFactory = Arc<dyn Fn() -> Result<Engine, Error> + Send + Sync + 'static>;

struct ChanInner {
    queue: VecDeque<Value>,
    cap: usize,
    closed: bool,
}

struct Channel {
    inner: Mutex<ChanInner>,
    not_empty: Condvar,
    not_full: Condvar,
}

impl Channel {
    fn new(cap: usize) -> Self {
        Self {
            inner: Mutex::new(ChanInner {
                queue: VecDeque::with_capacity(cap.min(256)),
                cap,
                closed: false,
            }),
            not_empty: Condvar::new(),
            not_full: Condvar::new(),
        }
    }

    fn send(&self, val: Value) -> Result<(), String> {
        let mut g = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        loop {
            if g.closed {
                return Err("send on closed channel".to_string());
            }
            if g.cap == 0 || g.queue.len() < g.cap {
                g.queue.push_back(val);
                self.not_empty.notify_one();
                return Ok(());
            }
            g = self.not_full.wait(g).unwrap_or_else(|e| e.into_inner());
        }
    }

    fn try_send(&self, val: Value) -> Result<bool, String> {
        let mut g = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        if g.closed {
            return Err("send on closed channel".to_string());
        }
        if g.cap == 0 || g.queue.len() < g.cap {
            g.queue.push_back(val);
            self.not_empty.notify_one();
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn recv(&self) -> (bool, Value) {
        let mut g = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        loop {
            if let Some(v) = g.queue.pop_front() {
                self.not_full.notify_one();
                return (true, v);
            }
            if g.closed {
                return (false, Value::Nil);
            }
            g = self.not_empty.wait(g).unwrap_or_else(|e| e.into_inner());
        }
    }

    fn try_recv(&self) -> (Value, Value) {
        let mut g = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(v) = g.queue.pop_front() {
            self.not_full.notify_one();
            (Value::Symbol("ok".to_string()), v)
        } else if g.closed {
            (Value::Symbol("closed".to_string()), Value::Nil)
        } else {
            (Value::Symbol("empty".to_string()), Value::Nil)
        }
    }

    fn recv_timeout(&self, timeout_ms: u64) -> (Value, Value) {
        let deadline = Instant::now() + Duration::from_millis(timeout_ms);
        let mut g = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        loop {
            if let Some(v) = g.queue.pop_front() {
                self.not_full.notify_one();
                return (Value::Symbol("ok".to_string()), v);
            }
            if g.closed {
                return (Value::Symbol("closed".to_string()), Value::Nil);
            }
            let now = Instant::now();
            if now >= deadline {
                return (Value::Symbol("timeout".to_string()), Value::Nil);
            }
            let (guard, _) = self
                .not_empty
                .wait_timeout(g, deadline.saturating_duration_since(now))
                .unwrap_or_else(|e| e.into_inner());
            g = guard;
        }
    }

    fn close(&self) {
        let mut g = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        g.closed = true;
        self.not_empty.notify_all();
        self.not_full.notify_all();
    }

    fn is_closed(&self) -> bool {
        self.inner.lock().unwrap_or_else(|e| e.into_inner()).closed
    }

    fn len(&self) -> usize {
        self.inner.lock().unwrap_or_else(|e| e.into_inner()).queue.len()
    }
}

struct SyncMutexInner {
    locked: bool,
    val: Value,
}

struct LispMutex {
    state: Mutex<SyncMutexInner>,
    cv: Condvar,
}

impl LispMutex {
    fn new(val: Value) -> Self {
        Self {
            state: Mutex::new(SyncMutexInner { locked: false, val }),
            cv: Condvar::new(),
        }
    }

    fn lock(&self) -> Value {
        let mut g = self.state.lock().unwrap_or_else(|e| e.into_inner());
        while g.locked {
            g = self.cv.wait(g).unwrap_or_else(|e| e.into_inner());
        }
        g.locked = true;
        g.val.clone()
    }

    fn unlock(&self, new_val: Option<Value>) {
        let mut g = self.state.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(v) = new_val {
            g.val = v;
        }
        g.locked = false;
        self.cv.notify_one();
    }

    fn get(&self) -> Value {
        self.state.lock().unwrap_or_else(|e| e.into_inner()).val.clone()
    }

    fn set(&self, val: Value) -> Value {
        let mut g = self.state.lock().unwrap_or_else(|e| e.into_inner());
        let old = std::mem::replace(&mut g.val, val);
        old
    }
}

struct TaskResultSlot {
    outcome: Mutex<Option<Result<Value, String>>>,
    cv: Condvar,
}

impl TaskResultSlot {
    fn new() -> Self {
        Self {
            outcome: Mutex::new(None),
            cv: Condvar::new(),
        }
    }

    fn resolve(&self, res: Result<Value, String>) {
        let mut g = self.outcome.lock().unwrap_or_else(|e| e.into_inner());
        if g.is_none() {
            *g = Some(res);
            self.cv.notify_all();
        }
    }

    fn is_done(&self) -> bool {
        self.outcome.lock().unwrap_or_else(|e| e.into_inner()).is_some()
    }

    fn wait(&self) -> Result<Value, String> {
        let mut g = self.outcome.lock().unwrap_or_else(|e| e.into_inner());
        while g.is_none() {
            g = self.cv.wait(g).unwrap_or_else(|e| e.into_inner());
        }
        g.clone().unwrap()
    }

    fn wait_timeout(&self, timeout_ms: u64) -> Option<Result<Value, String>> {
        let deadline = Instant::now() + Duration::from_millis(timeout_ms);
        let mut g = self.outcome.lock().unwrap_or_else(|e| e.into_inner());
        loop {
            if let Some(ref res) = *g {
                return Some(res.clone());
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
}

struct TaskEntry {
    slot: Arc<TaskResultSlot>,
    join: Mutex<Option<JoinHandle<()>>>,
}

impl TaskEntry {
    fn join_thread(&self) {
        let h = self.join.lock().unwrap_or_else(|e| e.into_inner()).take();
        if let Some(handle) = h {
            let _ = handle.join();
        }
    }
}

enum PoolJob {
    Eval {
        code: String,
        slot: Arc<TaskResultSlot>,
    },
    Shutdown,
}

struct PoolQueue {
    jobs: Mutex<VecDeque<PoolJob>>,
    cv: Condvar,
    closed: AtomicBool,
}

impl PoolQueue {
    fn new() -> Self {
        Self {
            jobs: Mutex::new(VecDeque::new()),
            cv: Condvar::new(),
            closed: AtomicBool::new(false),
        }
    }

    fn push(&self, job: PoolJob) -> Result<(), String> {
        if self.closed.load(Ordering::Acquire) {
            return Err("worker pool is shut down".to_string());
        }
        let mut g = self.jobs.lock().unwrap_or_else(|e| e.into_inner());
        g.push_back(job);
        self.cv.notify_one();
        Ok(())
    }

    fn pop(&self) -> Option<PoolJob> {
        let mut g = self.jobs.lock().unwrap_or_else(|e| e.into_inner());
        loop {
            if let Some(job) = g.pop_front() {
                match job {
                    PoolJob::Shutdown => return None,
                    other => return Some(other),
                }
            }
            if self.closed.load(Ordering::Acquire) {
                return None;
            }
            g = self.cv.wait(g).unwrap_or_else(|e| e.into_inner());
        }
    }
}

struct WorkerPool {
    queue: Arc<PoolQueue>,
    workers: Mutex<Vec<JoinHandle<()>>>,
    size: usize,
}

impl WorkerPool {
    fn start(size: usize, factory: WorkerFactory) -> Result<Arc<Self>, String> {
        let n = size.clamp(1, 64);
        let queue = Arc::new(PoolQueue::new());
        let (init_tx, init_rx) = std::sync::mpsc::channel::<Result<(), String>>();
        let mut handles = Vec::with_capacity(n);

        for _ in 0..n {
            let q = Arc::clone(&queue);
            let f = Arc::clone(&factory);
            let tx = init_tx.clone();
            let handle = thread::spawn(move || {
                let mut engine = match f() {
                    Ok(e) => {
                        let _ = tx.send(Ok(()));
                        e
                    }
                    Err(err) => {
                        let _ = tx.send(Err(err.to_string()));
                        return;
                    }
                };
                while let Some(job) = q.pop() {
                    match job {
                        PoolJob::Eval { code, slot } => {
                            let res = engine.eval(&code).map_err(|e| e.to_string());
                            slot.resolve(res);
                        }
                        PoolJob::Shutdown => break,
                    }
                }
            });
            handles.push(handle);
        }
        drop(init_tx);

        for _ in 0..n {
            match init_rx.recv() {
                Ok(Ok(())) => {}
                Ok(Err(msg)) => {
                    let pool = Arc::new(Self {
                        queue,
                        workers: Mutex::new(handles),
                        size: n,
                    });
                    pool.shutdown();
                    return Err(msg);
                }
                Err(_) => break,
            }
        }

        Ok(Arc::new(Self {
            queue,
            workers: Mutex::new(handles),
            size: n,
        }))
    }

    fn submit(&self, code: String) -> Result<Arc<TaskResultSlot>, String> {
        let slot = Arc::new(TaskResultSlot::new());
        self.queue.push(PoolJob::Eval {
            code,
            slot: Arc::clone(&slot),
        })?;
        Ok(slot)
    }

    fn shutdown(&self) {
        self.queue.closed.store(true, Ordering::Release);
        {
            let mut g = self.queue.jobs.lock().unwrap_or_else(|e| e.into_inner());
            for _ in 0..self.size {
                g.push_back(PoolJob::Shutdown);
            }
            self.queue.cv.notify_all();
        }
        let handles: Vec<JoinHandle<()>> = {
            let mut w = self.workers.lock().unwrap_or_else(|e| e.into_inner());
            std::mem::take(&mut *w)
        };
        for h in handles {
            let _ = h.join();
        }
    }
}

/// Shared synchronization & worker registry attached to a [`ConcurrencyHub`] instance.
pub struct ConcurrencyHub {
    mode: ConcurrencyMode,
    next_id: AtomicU64,
    factory: RwLock<Option<WorkerFactory>>,
    channels: Mutex<HashMap<u64, Arc<Channel>>>,
    atomics: Mutex<HashMap<u64, Arc<AtomicI64>>>,
    mutexes: Mutex<HashMap<u64, Arc<LispMutex>>>,
    shared_maps: Mutex<HashMap<u64, Arc<RwLock<BTreeMap<String, Value>>>>>,
    tasks: Mutex<HashMap<u64, Arc<TaskEntry>>>,
    pools: Mutex<HashMap<u64, Arc<WorkerPool>>>,
}

/// Owned by the root [`Engine`] so active worker pools and spawned task threads join cleanly
/// when the root [`Engine`] is dropped (avoiding any `Arc<ConcurrencyHub>` reference cycles).
pub struct ConcurrencyShutdownGuard {
    hub: Arc<ConcurrencyHub>,
}

impl Drop for ConcurrencyShutdownGuard {
    fn drop(&mut self) {
        self.hub.shutdown_all();
    }
}

impl ConcurrencyHub {
    pub fn new(mode: ConcurrencyMode) -> (Arc<Self>, ConcurrencyShutdownGuard) {
        let hub = Arc::new(Self {
            mode,
            next_id: AtomicU64::new(1),
            factory: RwLock::new(None),
            channels: Mutex::new(HashMap::new()),
            atomics: Mutex::new(HashMap::new()),
            mutexes: Mutex::new(HashMap::new()),
            shared_maps: Mutex::new(HashMap::new()),
            tasks: Mutex::new(HashMap::new()),
            pools: Mutex::new(HashMap::new()),
        });
        let guard = ConcurrencyShutdownGuard {
            hub: Arc::clone(&hub),
        };
        (hub, guard)
    }

    pub fn set_worker_factory(&self, factory: WorkerFactory) {
        *self.factory.write().unwrap_or_else(|e| e.into_inner()) = Some(factory);
    }

    fn alloc_id(&self) -> u64 {
        self.next_id.fetch_add(1, Ordering::Relaxed)
    }

    fn ensure_no_gil(&self) -> Result<(), String> {
        if self.mode == ConcurrencyMode::Gil {
            Err("concurrency disabled in gil mode (use --concurrency=no-gil)".to_string())
        } else {
            Ok(())
        }
    }

    fn worker_factory(&self) -> Result<WorkerFactory, String> {
        self.ensure_no_gil()?;
        self.factory
            .read()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
            .ok_or_else(|| "worker factory not configured".to_string())
    }

    fn shutdown_all(&self) {
        *self.factory.write().unwrap_or_else(|e| e.into_inner()) = None;
        let chs: Vec<Arc<Channel>> = {
            self.channels
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .values()
                .cloned()
                .collect()
        };
        for ch in chs {
            ch.close();
        }
        let pools: Vec<Arc<WorkerPool>> = {
            let mut g = self.pools.lock().unwrap_or_else(|e| e.into_inner());
            g.drain().map(|(_, p)| p).collect()
        };
        for pool in pools {
            pool.shutdown();
        }
        let tasks: Vec<Arc<TaskEntry>> = {
            let mut g = self.tasks.lock().unwrap_or_else(|e| e.into_inner());
            g.drain().map(|(_, t)| t).collect()
        };
        for task in tasks {
            task.join_thread();
        }
    }

    fn get_channel(&self, id: u64) -> Result<Arc<Channel>, String> {
        self.channels
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .get(&id)
            .cloned()
            .ok_or_else(|| format!("invalid channel handle {id}"))
    }

    fn get_atomic(&self, id: u64) -> Result<Arc<AtomicI64>, String> {
        self.atomics
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .get(&id)
            .cloned()
            .ok_or_else(|| format!("invalid atomic handle {id}"))
    }

    fn get_mutex(&self, id: u64) -> Result<Arc<LispMutex>, String> {
        self.mutexes
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .get(&id)
            .cloned()
            .ok_or_else(|| format!("invalid mutex handle {id}"))
    }

    fn get_shared(&self, id: u64) -> Result<Arc<RwLock<BTreeMap<String, Value>>>, String> {
        self.shared_maps
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .get(&id)
            .cloned()
            .ok_or_else(|| format!("invalid shared map handle {id}"))
    }

    fn get_task(&self, id: u64) -> Result<Arc<TaskEntry>, String> {
        self.tasks
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .get(&id)
            .cloned()
            .ok_or_else(|| format!("invalid task handle {id}"))
    }

    fn get_pool(&self, id: u64) -> Result<Arc<WorkerPool>, String> {
        self.pools
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .get(&id)
            .cloned()
            .ok_or_else(|| format!("invalid pool handle {id}"))
    }
}

fn expect_u64(val: Option<&Value>, label: &str) -> Result<u64, String> {
    let n = val
        .and_then(Value::as_int)
        .ok_or_else(|| format!("expected integer for {label}"))?;
    u64::try_from(n).map_err(|_| format!("{label} must be non-negative (got {n})"))
}

fn expect_i64(val: Option<&Value>, label: &str) -> Result<i64, String> {
    val.and_then(Value::as_int)
        .ok_or_else(|| format!("expected integer for {label}"))
}

fn expect_str(val: Option<&Value>, label: &str) -> Result<String, String> {
    val.and_then(Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| format!("expected string/symbol for {label}"))
}

/// Build worker [`EngineConfig`] inheriting cache & load paths from parent `config`.
pub fn worker_config_from(config: &EngineConfig) -> EngineConfig {
    let mut cfg = config.clone();
    cfg.concurrency = ConcurrencyMode::NoGil;
    cfg.quiet_bootstrap = true;
    cfg.verbose = false;
    cfg.defer_delayed_custom_init = false;
    cfg
}

/// Register `%std-chan-*`, `%std-sync-*`, and `%std-task-*` host functions on `engine`.
pub fn register_concurrency_builtins(engine: &mut Engine, hub: &Arc<ConcurrencyHub>) -> Result<(), Error> {
    // --- Channels (`std/chan`) ---
    let h = Arc::clone(hub);
    engine.register_fn("%std-chan-make", move |args| {
        let cap = expect_u64(args.first(), "capacity")? as usize;
        let id = h.alloc_id();
        h.channels
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .insert(id, Arc::new(Channel::new(cap)));
        Ok(Value::Int(id as i64))
    })?;

    let h = Arc::clone(hub);
    engine.register_fn("%std-chan-send", move |args| {
        let id = expect_u64(args.first(), "channel")?;
        let val = args.get(1).cloned().unwrap_or(Value::Nil);
        h.get_channel(id)?.send(val)?;
        Ok(Value::Bool(true))
    })?;

    let h = Arc::clone(hub);
    engine.register_fn("%std-chan-try-send", move |args| {
        let id = expect_u64(args.first(), "channel")?;
        let val = args.get(1).cloned().unwrap_or(Value::Nil);
        let ok = h.get_channel(id)?.try_send(val)?;
        Ok(Value::Bool(ok))
    })?;

    let h = Arc::clone(hub);
    engine.register_fn("%std-chan-recv", move |args| {
        let id = expect_u64(args.first(), "channel")?;
        let (ok, val) = h.get_channel(id)?.recv();
        Ok(Value::List(vec![Value::Bool(ok), val]))
    })?;

    let h = Arc::clone(hub);
    engine.register_fn("%std-chan-try-recv", move |args| {
        let id = expect_u64(args.first(), "channel")?;
        let (status, val) = h.get_channel(id)?.try_recv();
        Ok(Value::List(vec![status, val]))
    })?;

    let h = Arc::clone(hub);
    engine.register_fn("%std-chan-recv-timeout", move |args| {
        let id = expect_u64(args.first(), "channel")?;
        let ms = expect_u64(args.get(1), "timeout-ms")?;
        let (status, val) = h.get_channel(id)?.recv_timeout(ms);
        Ok(Value::List(vec![status, val]))
    })?;

    let h = Arc::clone(hub);
    engine.register_fn("%std-chan-close", move |args| {
        let id = expect_u64(args.first(), "channel")?;
        h.get_channel(id)?.close();
        Ok(Value::Bool(true))
    })?;

    let h = Arc::clone(hub);
    engine.register_fn("%std-chan-closed-p", move |args| {
        let id = expect_u64(args.first(), "channel")?;
        Ok(Value::Bool(h.get_channel(id)?.is_closed()))
    })?;

    let h = Arc::clone(hub);
    engine.register_fn("%std-chan-len", move |args| {
        let id = expect_u64(args.first(), "channel")?;
        Ok(Value::Int(h.get_channel(id)?.len() as i64))
    })?;

    // --- Atomics / Mutex / Shared Map (`std/sync`) ---
    let h = Arc::clone(hub);
    engine.register_fn("%std-atomic-make", move |args| {
        let init = args.first().and_then(Value::as_int).unwrap_or(0);
        let id = h.alloc_id();
        h.atomics
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .insert(id, Arc::new(AtomicI64::new(init)));
        Ok(Value::Int(id as i64))
    })?;

    let h = Arc::clone(hub);
    engine.register_fn("%std-atomic-load", move |args| {
        let id = expect_u64(args.first(), "atomic")?;
        Ok(Value::Int(h.get_atomic(id)?.load(Ordering::SeqCst)))
    })?;

    let h = Arc::clone(hub);
    engine.register_fn("%std-atomic-store", move |args| {
        let id = expect_u64(args.first(), "atomic")?;
        let val = expect_i64(args.get(1), "value")?;
        h.get_atomic(id)?.store(val, Ordering::SeqCst);
        Ok(Value::Int(val))
    })?;

    let h = Arc::clone(hub);
    engine.register_fn("%std-atomic-add", move |args| {
        let id = expect_u64(args.first(), "atomic")?;
        let delta = expect_i64(args.get(1), "delta")?;
        let prev = h.get_atomic(id)?.fetch_add(delta, Ordering::SeqCst);
        Ok(Value::Int(prev.wrapping_add(delta)))
    })?;

    let h = Arc::clone(hub);
    engine.register_fn("%std-atomic-cas", move |args| {
        let id = expect_u64(args.first(), "atomic")?;
        let expected = expect_i64(args.get(1), "expected")?;
        let desired = expect_i64(args.get(2), "desired")?;
        let res = h.get_atomic(id)?.compare_exchange(
            expected,
            desired,
            Ordering::SeqCst,
            Ordering::SeqCst,
        );
        Ok(Value::Bool(res.is_ok()))
    })?;

    let h = Arc::clone(hub);
    engine.register_fn("%std-mutex-make", move |args| {
        let init = args.first().cloned().unwrap_or(Value::Nil);
        let id = h.alloc_id();
        h.mutexes
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .insert(id, Arc::new(LispMutex::new(init)));
        Ok(Value::Int(id as i64))
    })?;

    let h = Arc::clone(hub);
    engine.register_fn("%std-mutex-lock", move |args| {
        let id = expect_u64(args.first(), "mutex")?;
        Ok(h.get_mutex(id)?.lock())
    })?;

    let h = Arc::clone(hub);
    engine.register_fn("%std-mutex-unlock", move |args| {
        let id = expect_u64(args.first(), "mutex")?;
        let new_val = args.get(1).cloned();
        h.get_mutex(id)?.unlock(new_val);
        Ok(Value::Bool(true))
    })?;

    let h = Arc::clone(hub);
    engine.register_fn("%std-mutex-get", move |args| {
        let id = expect_u64(args.first(), "mutex")?;
        Ok(h.get_mutex(id)?.get())
    })?;

    let h = Arc::clone(hub);
    engine.register_fn("%std-mutex-set", move |args| {
        let id = expect_u64(args.first(), "mutex")?;
        let val = args.get(1).cloned().unwrap_or(Value::Nil);
        Ok(h.get_mutex(id)?.set(val))
    })?;

    let h = Arc::clone(hub);
    engine.register_fn("%std-shared-make", move |_| {
        let id = h.alloc_id();
        h.shared_maps
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .insert(id, Arc::new(RwLock::new(BTreeMap::new())));
        Ok(Value::Int(id as i64))
    })?;

    let h = Arc::clone(hub);
    engine.register_fn("%std-shared-get", move |args| {
        let id = expect_u64(args.first(), "shared-map")?;
        let key = expect_str(args.get(1), "key")?;
        let default = args.get(2).cloned().unwrap_or(Value::Nil);
        let m = h.get_shared(id)?;
        let g = m.read().unwrap_or_else(|e| e.into_inner());
        Ok(g.get(&key).cloned().unwrap_or(default))
    })?;

    let h = Arc::clone(hub);
    engine.register_fn("%std-shared-set", move |args| {
        let id = expect_u64(args.first(), "shared-map")?;
        let key = expect_str(args.get(1), "key")?;
        let val = args.get(2).cloned().unwrap_or(Value::Nil);
        let m = h.get_shared(id)?;
        m.write().unwrap_or_else(|e| e.into_inner()).insert(key, val.clone());
        Ok(val)
    })?;

    let h = Arc::clone(hub);
    engine.register_fn("%std-shared-del", move |args| {
        let id = expect_u64(args.first(), "shared-map")?;
        let key = expect_str(args.get(1), "key")?;
        let m = h.get_shared(id)?;
        let removed = m.write().unwrap_or_else(|e| e.into_inner()).remove(&key).is_some();
        Ok(Value::Bool(removed))
    })?;

    let h = Arc::clone(hub);
    engine.register_fn("%std-shared-keys", move |args| {
        let id = expect_u64(args.first(), "shared-map")?;
        let m = h.get_shared(id)?;
        let g = m.read().unwrap_or_else(|e| e.into_inner());
        Ok(Value::List(g.keys().cloned().map(Value::String).collect()))
    })?;

    // --- Tasks & Pools (`std/task`) ---
    let h = Arc::clone(hub);
    engine.register_fn("%std-task-mode", move |_| {
        Ok(Value::Symbol(match h.mode {
            ConcurrencyMode::NoGil => "no-gil".to_string(),
            ConcurrencyMode::Gil => "gil".to_string(),
        }))
    })?;

    engine.register_fn("%std-task-cpu-count", move |_| {
        let n = thread::available_parallelism().map(|p| p.get()).unwrap_or(1);
        Ok(Value::Int(n as i64))
    })?;

    let h = Arc::clone(hub);
    engine.register_fn("%std-task-spawn", move |args| {
        let factory = h.worker_factory()?;
        let code = expect_str(args.first(), "code")?;
        let id = h.alloc_id();
        let slot = Arc::new(TaskResultSlot::new());
        let slot_bg = Arc::clone(&slot);
        let join = thread::spawn(move || {
            let res = match factory() {
                Ok(mut eng) => eng.eval(&code).map_err(|e| e.to_string()),
                Err(e) => Err(e.to_string()),
            };
            slot_bg.resolve(res);
        });
        let entry = Arc::new(TaskEntry {
            slot,
            join: Mutex::new(Some(join)),
        });
        h.tasks
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .insert(id, entry);
        Ok(Value::Int(id as i64))
    })?;

    let h = Arc::clone(hub);
    engine.register_fn("%std-task-done-p", move |args| {
        let id = expect_u64(args.first(), "task")?;
        Ok(Value::Bool(h.get_task(id)?.slot.is_done()))
    })?;

    let h = Arc::clone(hub);
    engine.register_fn("%std-task-await", move |args| {
        let id = expect_u64(args.first(), "task")?;
        let entry = h.get_task(id)?;
        let res = entry.slot.wait();
        entry.join_thread();
        h.tasks
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .remove(&id);
        res
    })?;

    let h = Arc::clone(hub);
    engine.register_fn("%std-task-await-timeout", move |args| {
        let id = expect_u64(args.first(), "task")?;
        let ms = expect_u64(args.get(1), "timeout-ms")?;
        let entry = h.get_task(id)?;
        match entry.slot.wait_timeout(ms) {
            Some(Ok(v)) => {
                entry.join_thread();
                h.tasks
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .remove(&id);
                Ok(Value::List(vec![Value::Symbol("ok".to_string()), v]))
            }
            Some(Err(err)) => {
                entry.join_thread();
                h.tasks
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .remove(&id);
                Err(err)
            }
            None => Ok(Value::List(vec![
                Value::Symbol("timeout".to_string()),
                Value::Nil,
            ])),
        }
    })?;

    let h = Arc::clone(hub);
    engine.register_fn("%std-task-pool-create", move |args| {
        let factory = h.worker_factory()?;
        let size = args
            .first()
            .and_then(Value::as_int)
            .unwrap_or_else(|| {
                thread::available_parallelism()
                    .map(|p| p.get() as i64)
                    .unwrap_or(2)
            })
            .max(1) as usize;
        let pool = WorkerPool::start(size, factory)?;
        let id = h.alloc_id();
        h.pools
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .insert(id, pool);
        Ok(Value::Int(id as i64))
    })?;

    let h = Arc::clone(hub);
    engine.register_fn("%std-task-pool-submit", move |args| {
        let pool_id = expect_u64(args.first(), "pool")?;
        let code = expect_str(args.get(1), "code")?;
        let pool = h.get_pool(pool_id)?;
        let slot = pool.submit(code)?;
        let task_id = h.alloc_id();
        let entry = Arc::new(TaskEntry {
            slot,
            join: Mutex::new(None),
        });
        h.tasks
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .insert(task_id, entry);
        Ok(Value::Int(task_id as i64))
    })?;

    let h = Arc::clone(hub);
    engine.register_fn("%std-task-pool-shutdown", move |args| {
        let pool_id = expect_u64(args.first(), "pool")?;
        let pool = h
            .pools
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .remove(&pool_id)
            .ok_or_else(|| format!("invalid pool handle {pool_id}"))?;
        pool.shutdown();
        Ok(Value::Bool(true))
    })?;

    let h = Arc::clone(hub);
    engine.register_fn("%std-task-parallel-map", move |args| {
        let factory = h.worker_factory()?;
        let setup_code = expect_str(args.first(), "setup-code")?;
        let fn_name = expect_str(args.get(1), "fn-name")?;
        let items = match args.get(2) {
            Some(Value::Nil) | None => Vec::new(),
            Some(Value::List(v)) | Some(Value::Vector(v)) => v.clone(),
            Some(_) => return Err("std-task-parallel-map expected list or vector".to_string()),
        };
        if items.is_empty() {
            return Ok(Value::Nil);
        }
        let requested_workers = args
            .get(3)
            .and_then(Value::as_int)
            .filter(|&n| n > 0)
            .map(|n| n as usize)
            .unwrap_or_else(|| {
                thread::available_parallelism()
                    .map(|p| p.get())
                    .unwrap_or(2)
            });
        let n_workers = requested_workers.min(items.len()).clamp(1, 64);

        let items_arc = Arc::new(items);
        let next_idx = Arc::new(AtomicUsize::new(0));
        let results: Arc<Vec<Mutex<Option<Result<Value, String>>>>> = Arc::new(
            (0..items_arc.len())
                .map(|_| Mutex::new(None))
                .collect(),
        );

        let mut handles = Vec::with_capacity(n_workers);
        for _ in 0..n_workers {
            let f = Arc::clone(&factory);
            let setup = setup_code.clone();
            let call_sym = fn_name.clone();
            let items_ref = Arc::clone(&items_arc);
            let idx_ref = Arc::clone(&next_idx);
            let out_ref = Arc::clone(&results);
            handles.push(thread::spawn(move || {
                let mut eng = match f() {
                    Ok(e) => e,
                    Err(err) => {
                        let msg = err.to_string();
                        loop {
                            let i = idx_ref.fetch_add(1, Ordering::Relaxed);
                            if i >= items_ref.len() {
                                break;
                            }
                            *out_ref[i].lock().unwrap_or_else(|e| e.into_inner()) =
                                Some(Err(msg.clone()));
                        }
                        return;
                    }
                };
                if !setup.is_empty() {
                    if let Err(err) = eng.eval_silent(&setup) {
                        let msg = err.to_string();
                        loop {
                            let i = idx_ref.fetch_add(1, Ordering::Relaxed);
                            if i >= items_ref.len() {
                                break;
                            }
                            *out_ref[i].lock().unwrap_or_else(|e| e.into_inner()) =
                                Some(Err(msg.clone()));
                        }
                        return;
                    }
                }
                loop {
                    let i = idx_ref.fetch_add(1, Ordering::Relaxed);
                    if i >= items_ref.len() {
                        break;
                    }
                    let res = eng
                        .call(&call_sym, std::slice::from_ref(&items_ref[i]))
                        .map_err(|e| e.to_string());
                    *out_ref[i].lock().unwrap_or_else(|e| e.into_inner()) = Some(res);
                }
            }));
        }

        for handle in handles {
            let _ = handle.join();
        }

        let mut collected = Vec::with_capacity(results.len());
        for slot in results.iter() {
            let item_res = slot
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .take()
                .unwrap_or_else(|| Err("worker did not produce result".to_string()))?;
            collected.push(item_res);
        }
        Ok(Value::List(collected))
    })?;

    Ok(())
}
