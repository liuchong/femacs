//! Explicit VM non-local exit representation, scoped GC roots, and FFI boundary guards (U3).
//!
//! All Lisp-level non-local exits (`signal`, `throw`, `quit`) propagate inside `femacs-vm` as
//! typed `Result<T, NonLocalExit>` boundaries backed by deterministic Rust stack unwinding
//! (`extern "C-unwind"`), replacing `setjmp`/`longjmp` across the interpreter, bytecode engine,
//! native `.eln` runtime helpers, and external C dynamic module vtable boundaries.

#![deny(unsafe_op_in_unsafe_fn)]

use core::ffi::{c_int, c_void};
use std::collections::HashMap;
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use crate::common::{
    Lisp_Object, NONLOCAL_EXIT_SIGNAL, NONLOCAL_EXIT_THROW, current_thread, handler, lisp_h_Qnil,
    nonlocal_exit,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NonLocalExitKind {
    Signal,
    Throw,
}

impl NonLocalExitKind {
    #[inline]
    pub fn from_raw(raw: nonlocal_exit) -> Self {
        if raw == NONLOCAL_EXIT_SIGNAL {
            Self::Signal
        } else {
            debug_assert_eq!(raw, NONLOCAL_EXIT_THROW);
            Self::Throw
        }
    }

    #[inline]
    pub fn to_raw(self) -> nonlocal_exit {
        match self {
            Self::Signal => NONLOCAL_EXIT_SIGNAL,
            Self::Throw => NONLOCAL_EXIT_THROW,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NonLocalExit {
    pub kind: NonLocalExitKind,
    pub target: *mut handler,
    pub value: Lisp_Object,
}

// SAFETY: `NonLocalExit` payloads are created, unwound, and caught strictly on the owning Lisp
// thread while holding the VM global lock; `std::panic::resume_unwind` requires `Send + 'static`.
unsafe impl Send for NonLocalExit {}

pub type VmResult<T = Lisp_Object> = Result<T, NonLocalExit>;

struct VmNonLocalUnwind(NonLocalExit);

#[derive(Clone, Copy)]
struct NativePendingExit {
    pending: bool,
    exit: NonLocalExit,
}

impl Default for NativePendingExit {
    fn default() -> Self {
        Self {
            pending: false,
            exit: NonLocalExit {
                kind: NonLocalExitKind::Signal,
                target: core::ptr::null_mut(),
                value: lisp_h_Qnil(),
            },
        }
    }
}

#[derive(Default)]
struct ThreadNonLocalState {
    scoped_roots: Vec<Lisp_Object>,
    inflight_value: Lisp_Object,
    inflight_active: bool,
    native_pending: NativePendingExit,
}

// SAFETY: `Lisp_Object` and `*mut handler` inside `ThreadNonLocalState` are accessed only under
// `THREAD_STATES` mutex while the VM global lock serializes Lisp execution and GC marking.
unsafe impl Send for ThreadNonLocalState {}

static THREAD_STATES: Mutex<Option<HashMap<usize, Box<ThreadNonLocalState>>>> = Mutex::new(None);
static STATE_EPOCH: AtomicUsize = AtomicUsize::new(1);

std::thread_local! {
    static TLS_STATE_CACHE: core::cell::Cell<(usize, usize, *mut ThreadNonLocalState)> =
        const { core::cell::Cell::new((0, 0, core::ptr::null_mut())) };
}

static ACTIVE_COMP_NATIVE_EXITS: AtomicUsize = AtomicUsize::new(0);
static SCOPED_ROOTS_CREATED: AtomicUsize = AtomicUsize::new(0);
static SCOPED_ROOTS_DROPPED: AtomicUsize = AtomicUsize::new(0);
static HOST_PROBES_CREATED: AtomicUsize = AtomicUsize::new(0);
static HOST_PROBES_DROPPED: AtomicUsize = AtomicUsize::new(0);

#[inline]
fn current_thread_key() -> usize {
    // SAFETY: `current_thread` is the single-VM active thread pointer `VmGlobal` extern slot.
    unsafe { current_thread.get() as usize }
}

#[inline(always)]
fn with_thread_state<R>(thread_key: usize, f: impl FnOnce(&mut ThreadNonLocalState) -> R) -> R {
    let epoch = STATE_EPOCH.load(Ordering::Acquire);
    let (c_epoch, c_key, c_ptr) = TLS_STATE_CACHE.with(|c| c.get());
    if c_epoch == epoch && c_key == thread_key && !c_ptr.is_null() {
        // SAFETY: `c_ptr` points to a heap-pinned `Box<ThreadNonLocalState>` stored in
        // `THREAD_STATES`; `cleanup_thread_state` increments `STATE_EPOCH` before removing any
        // entry, and single-VM thread execution serializes mutable access to that thread's state
        // across non-reentrant leaf closures `f`.
        return f(unsafe { &mut *c_ptr });
    }
    with_thread_state_slow(thread_key, f)
}

#[cold]
#[inline(never)]
fn with_thread_state_slow<R>(thread_key: usize, f: impl FnOnce(&mut ThreadNonLocalState) -> R) -> R {
    let mut guard = THREAD_STATES
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let epoch = STATE_EPOCH.load(Ordering::Acquire);
    let map = guard.get_or_insert_with(HashMap::new);
    let entry = map.entry(thread_key).or_insert_with(|| {
        Box::new(ThreadNonLocalState {
            scoped_roots: Vec::with_capacity(16),
            inflight_value: lisp_h_Qnil(),
            inflight_active: false,
            native_pending: NativePendingExit::default(),
        })
    });
    let ptr: *mut ThreadNonLocalState = &mut **entry;
    TLS_STATE_CACHE.with(|c| c.set((epoch, thread_key, ptr)));
    f(entry)
}

/// LIFO scoped GC root guard for `Lisp_Object` values held across Lisp hooks, `unbind_to`
/// cleanups, or unwinding frames. Deterministically pops its root slots on `Drop`.
pub struct ScopedLispRoot {
    thread_key: usize,
    base: usize,
    len: usize,
}

impl ScopedLispRoot {
    #[inline]
    pub fn new(values: &[Lisp_Object]) -> Self {
        let thread_key = current_thread_key();
        let len = values.len();
        let base = if thread_key == 0 || len == 0 {
            0
        } else {
            with_thread_state(thread_key, |state| {
                let base = state.scoped_roots.len();
                state.scoped_roots.extend_from_slice(values);
                base
            })
        };
        SCOPED_ROOTS_CREATED.fetch_add(1, Ordering::Relaxed);
        Self {
            thread_key,
            base,
            len,
        }
    }

    #[inline]
    pub fn set(&mut self, index: usize, value: Lisp_Object) {
        assert!(index < self.len);
        if self.thread_key != 0 {
            let slot = self.base + index;
            with_thread_state(self.thread_key, |state| {
                state.scoped_roots[slot] = value;
            });
        }
    }
}

impl Drop for ScopedLispRoot {
    fn drop(&mut self) {
        if self.thread_key != 0 && self.len != 0 {
            with_thread_state(self.thread_key, |state| {
                debug_assert_eq!(state.scoped_roots.len(), self.base + self.len);
                state.scoped_roots.truncate(self.base);
            });
        }
        SCOPED_ROOTS_DROPPED.fetch_add(1, Ordering::Relaxed);
    }
}

/// Deterministic Rust host resource guard used to verify exact-once `Drop` execution across
/// multi-frame VM unwinds without leaking or double-dropping.
pub struct HostDropGuard {
    dropped: AtomicBool,
}

impl HostDropGuard {
    #[inline]
    pub fn new() -> Self {
        HOST_PROBES_CREATED.fetch_add(1, Ordering::Relaxed);
        Self {
            dropped: AtomicBool::new(false),
        }
    }
}

impl Default for HostDropGuard {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for HostDropGuard {
    fn drop(&mut self) {
        let already = self.dropped.swap(true, Ordering::SeqCst);
        assert!(!already, "HostDropGuard dropped more than once");
        HOST_PROBES_DROPPED.fetch_add(1, Ordering::Relaxed);
    }
}

#[inline]
pub fn host_drop_counts() -> (usize, usize) {
    (
        HOST_PROBES_CREATED.load(Ordering::Relaxed),
        HOST_PROBES_DROPPED.load(Ordering::Relaxed),
    )
}

#[inline]
pub fn scoped_root_counts() -> (usize, usize) {
    (
        SCOPED_ROOTS_CREATED.load(Ordering::Relaxed),
        SCOPED_ROOTS_DROPPED.load(Ordering::Relaxed),
    )
}

/// Mark all explicit `ScopedLispRoot`, in-flight `NonLocalExit`, and pending `.eln` exit roots
/// belonging to `thread` during GC (`mark_one_thread`).
pub fn mark_thread_explicit_roots(
    thread: *mut c_void,
    mut mark_fn: impl FnMut(Lisp_Object),
) {
    let key = thread as usize;
    if key == 0 {
        return;
    }
    let snapshot: Vec<Lisp_Object> = {
        let guard = THREAD_STATES
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let Some(map) = guard.as_ref() else {
            return;
        };
        let Some(state) = map.get(&key) else {
            return;
        };
        let mut objs = Vec::with_capacity(state.scoped_roots.len() + 2);
        objs.extend_from_slice(&state.scoped_roots);
        if state.inflight_active {
            objs.push(state.inflight_value);
        }
        if state.native_pending.pending {
            objs.push(state.native_pending.exit.value);
        }
        objs
    };
    for obj in snapshot {
        mark_fn(obj);
    }
}

/// Remove per-thread explicit root tracking when a Lisp thread is finalized.
pub fn cleanup_thread_state(thread: *mut c_void) {
    let key = thread as usize;
    if key == 0 {
        return;
    }
    let mut guard = THREAD_STATES
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    STATE_EPOCH.fetch_add(1, Ordering::SeqCst);
    TLS_STATE_CACHE.with(|c| c.set((0, 0, core::ptr::null_mut())));
    if let Some(map) = guard.as_mut()
        && let Some(removed) = map.remove(&key)
        && removed.native_pending.pending
    {
        ACTIVE_COMP_NATIVE_EXITS.fetch_sub(1, Ordering::Relaxed);
    }
}

/// Raise a new Lisp `NonLocalExit`, rooting its payload on the current thread and unwinding
/// Rust frames until a matching VM catcher or FFI boundary guard intercepts it.
#[inline(never)]
pub fn raise_vm_nonlocal(exit: NonLocalExit) -> ! {
    let key = current_thread_key();
    if key != 0 {
        with_thread_state(key, |state| {
            state.inflight_value = exit.value;
            state.inflight_active = true;
        });
    }
    resume_unwind(Box::new(VmNonLocalUnwind(exit)))
}

/// Continue propagating an already-raised `NonLocalExit` to an enclosing handler.
#[inline(never)]
pub fn resume_vm_nonlocal(exit: NonLocalExit) -> ! {
    raise_vm_nonlocal(exit)
}

/// Run `body` and catch only Lisp `NonLocalExit` unwinds (`VmResult<R>`), re-propagating any
/// genuine Rust `panic!` untouched via `resume_unwind`.
#[inline]
pub fn catch_vm_nonlocal<R, F: FnOnce() -> R>(body: F) -> VmResult<R> {
    match catch_unwind(AssertUnwindSafe(body)) {
        Ok(val) => Ok(val),
        Err(payload) => match payload.downcast::<VmNonLocalUnwind>() {
            Ok(boxed) => {
                let exit = boxed.0;
                let key = current_thread_key();
                if key != 0 {
                    with_thread_state(key, |state| {
                        state.inflight_value = lisp_h_Qnil();
                        state.inflight_active = false;
                    });
                }
                Err(exit)
            }
            Err(other_panic) => resume_unwind(other_panic),
        },
    }
}

/// Record a pending `NonLocalExit` at a C-ABI `.eln` helper boundary so no unwind crosses `.eln`
/// C frames.
pub fn set_comp_native_exit(exit: NonLocalExit) {
    let key = current_thread_key();
    if key == 0 {
        return;
    }
    with_thread_state(key, |state| {
        if !state.native_pending.pending {
            ACTIVE_COMP_NATIVE_EXITS.fetch_add(1, Ordering::Relaxed);
        }
        state.native_pending = NativePendingExit {
            pending: true,
            exit,
        };
    });
}

#[inline(always)]
pub fn comp_native_exit_pending_fast() -> bool {
    ACTIVE_COMP_NATIVE_EXITS.load(Ordering::Relaxed) != 0
}

pub fn comp_native_exit_pending_for_current_thread() -> c_int {
    if !comp_native_exit_pending_fast() {
        return 0;
    }
    let key = current_thread_key();
    if key == 0 {
        return 0;
    }
    with_thread_state(key, |state| c_int::from(state.native_pending.pending))
}

/// If the current thread has a pending `.eln` exit targeting `candidate`, clear the pending
/// state and return `1`; otherwise leave it pending and return `0`.
pub fn consume_comp_native_exit_for_handler(candidate: *mut handler) -> c_int {
    if !comp_native_exit_pending_fast() {
        return 0;
    }
    let key = current_thread_key();
    if key == 0 {
        return 0;
    }
    with_thread_state(key, |state| {
        if state.native_pending.pending && state.native_pending.exit.target == candidate {
            state.native_pending = NativePendingExit::default();
            ACTIVE_COMP_NATIVE_EXITS.fetch_sub(1, Ordering::Relaxed);
            1
        } else {
            0
        }
    })
}

#[inline(never)]
fn propagate_comp_native_exit_slow() {
    let key = current_thread_key();
    if key == 0 {
        return;
    }
    let maybe_exit = with_thread_state(key, |state| {
        if state.native_pending.pending {
            let exit = state.native_pending.exit;
            state.native_pending = NativePendingExit::default();
            ACTIVE_COMP_NATIVE_EXITS.fetch_sub(1, Ordering::Relaxed);
            Some(exit)
        } else {
            None
        }
    });
    if let Some(exit) = maybe_exit {
        raise_vm_nonlocal(exit);
    }
}

/// Fast inline check executed after invoking a potentially native-compiled `Lisp_Subr` or
/// `top_level_run`: if the `.eln` frame returned an unhandled pending exit, convert it back into
/// a VM `NonLocalExit` unwind.
#[inline(always)]
pub fn check_and_propagate_comp_native_exit() {
    if comp_native_exit_pending_fast() {
        propagate_comp_native_exit_slow();
    }
}

/// Host C-ABI probe entry points used by `AU3` dynamic-module and integration tests to verify
/// exact-once Rust `Drop` execution across multi-frame Lisp unwinds without polluting `obarray`.
#[unsafe(no_mangle)]
pub extern "C" fn femacs_vm_host_drop_counts(
    out_host_created: *mut usize,
    out_host_dropped: *mut usize,
    out_root_created: *mut usize,
    out_root_dropped: *mut usize,
) {
    let (hc, hd) = host_drop_counts();
    let (rc, rd) = scoped_root_counts();
    if !out_host_created.is_null() {
        // SAFETY: caller provides a valid writable `usize` pointer.
        unsafe { *out_host_created = hc };
    }
    if !out_host_dropped.is_null() {
        // SAFETY: caller provides a valid writable `usize` pointer.
        unsafe { *out_host_dropped = hd };
    }
    if !out_root_created.is_null() {
        // SAFETY: caller provides a valid writable `usize` pointer.
        unsafe { *out_root_created = rc };
    }
    if !out_root_dropped.is_null() {
        // SAFETY: caller provides a valid writable `usize` pointer.
        unsafe { *out_root_dropped = rd };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn femacs_vm_call0_with_host_drop_guard(
    fun: Lisp_Object,
) -> Lisp_Object {
    let _host_guard = HostDropGuard::new();
    let _root_guard = ScopedLispRoot::new(&[fun]);
    // SAFETY: `Ffuncall` expects `(nargs, args)` with `nargs >= 1` on the active Lisp thread.
    unsafe {
        let mut args = [fun];
        crate::eval::Ffuncall(1, args.as_mut_ptr())
    }
}
