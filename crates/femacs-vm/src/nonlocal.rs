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
    pub(crate) target: *mut handler,
    pub(crate) value: Lisp_Object,
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

struct ScopedRootSlot {
    generation: u64,
    occupied: bool,
    len: usize,
    inline: [Lisp_Object; 4],
    overflow: Vec<Lisp_Object>,
}

struct ThreadNonLocalState {
    scoped_slots: Vec<ScopedRootSlot>,
    free_slot_indices: Vec<usize>,
    next_slot_generation: u64,
    scoped_roots_created: usize,
    scoped_roots_dropped: usize,
    inflight_value: Lisp_Object,
    inflight_active: bool,
    native_pending: NativePendingExit,
}

impl Default for ThreadNonLocalState {
    fn default() -> Self {
        Self {
            scoped_slots: Vec::with_capacity(16),
            free_slot_indices: Vec::with_capacity(8),
            next_slot_generation: 1,
            scoped_roots_created: 0,
            scoped_roots_dropped: 0,
            inflight_value: lisp_h_Qnil(),
            inflight_active: false,
            native_pending: NativePendingExit::default(),
        }
    }
}

// SAFETY: `Lisp_Object` and `*mut handler` inside `ThreadNonLocalState` are accessed only under
// `THREAD_STATES` mutex while the VM global lock serializes Lisp execution and GC marking.
unsafe impl Send for ThreadNonLocalState {}

static THREAD_STATES: Mutex<Option<HashMap<usize, Box<ThreadNonLocalState>>>> = Mutex::new(None);
static STATE_EPOCH: AtomicUsize = AtomicUsize::new(1);

#[derive(Clone, Copy)]
struct TlsNonLocalState {
    vm_lock_key: usize,
    borrowed: bool,
    cache_epoch: usize,
    cache_key: usize,
    cache_ptr: *mut ThreadNonLocalState,
}

impl TlsNonLocalState {
    const EMPTY: Self = Self {
        vm_lock_key: 0,
        borrowed: false,
        cache_epoch: 0,
        cache_key: 0,
        cache_ptr: core::ptr::null_mut(),
    };
}

std::thread_local! {
    static TLS_STATE: core::cell::Cell<TlsNonLocalState> =
        const { core::cell::Cell::new(TlsNonLocalState::EMPTY) };
}

static ACTIVE_COMP_NATIVE_EXITS: AtomicUsize = AtomicUsize::new(0);
static RETIRED_SCOPED_ROOTS_CREATED: AtomicUsize = AtomicUsize::new(0);
static RETIRED_SCOPED_ROOTS_DROPPED: AtomicUsize = AtomicUsize::new(0);
static HOST_PROBES_CREATED: AtomicUsize = AtomicUsize::new(0);
static HOST_PROBES_DROPPED: AtomicUsize = AtomicUsize::new(0);

#[inline]
fn current_thread_key() -> usize {
    // SAFETY: `current_thread` is the single-VM active thread pointer `VmGlobal` extern slot.
    unsafe { current_thread.get() as usize }
}

/// Return whether the current OS thread currently holds a `ThreadNonLocalState` borrow.
#[inline]
pub(crate) fn tls_state_borrowed() -> bool {
    TLS_STATE.with(|cell| cell.get().borrowed)
}

/// Record the calling OS thread's active VM lock key for fast `TLS_STATE` lookup.
#[inline]
pub(crate) fn set_tls_vm_lock_key(thread_key: usize) {
    TLS_STATE.with(|cell| {
        let mut s = cell.get();
        s.vm_lock_key = thread_key;
        cell.set(s);
    });
}

/// Clear the calling OS thread's cached `ThreadNonLocalState` pointer across lock handoffs.
#[inline]
pub(crate) fn clear_tls_state_cache() {
    TLS_STATE.with(|cell| {
        let mut s = cell.get();
        s.vm_lock_key = 0;
        s.cache_epoch = 0;
        s.cache_key = 0;
        s.cache_ptr = core::ptr::null_mut();
        cell.set(s);
    });
}

struct StateBorrowGuard {
    prev: bool,
}

impl Drop for StateBorrowGuard {
    #[inline(always)]
    fn drop(&mut self) {
        TLS_STATE.with(|cell| {
            let mut s = cell.get();
            s.borrowed = self.prev;
            cell.set(s);
        });
    }
}

#[inline(always)]
fn with_thread_state<R>(thread_key: usize, f: impl FnOnce(&mut ThreadNonLocalState) -> R) -> R {
    let state = TLS_STATE.with(|cell| {
        let s = cell.get();
        assert!(!s.borrowed, "re-entrant ThreadNonLocalState borrow forbidden");
        let mut next = s;
        next.borrowed = true;
        cell.set(next);
        s
    });
    let _borrow_guard = StateBorrowGuard { prev: state.borrowed };
    if thread_key != 0 && state.vm_lock_key == thread_key {
        let epoch = STATE_EPOCH.load(Ordering::Acquire);
        if state.cache_epoch == epoch && state.cache_key == thread_key && !state.cache_ptr.is_null() {
            // SAFETY: `state.cache_ptr` points to a heap-pinned `Box<ThreadNonLocalState>` stored in
            // `THREAD_STATES`; `set_tls_vm_lock_key` is only invoked from `pub(crate) unsafe fn
            // bind_vm_lock_owner`, proving single-OS-thread VM lock ownership, `StateBorrowGuard`
            // prevents re-entrant `&mut`, and `cleanup_thread_state` increments `STATE_EPOCH` with
            // `SeqCst` while holding the VM lock before removing any entry.
            return f(unsafe { &mut *state.cache_ptr });
        }
    }
    with_thread_state_slow(thread_key, f)
}

#[cold]
#[inline(never)]
fn with_thread_state_slow<R>(thread_key: usize, f: impl FnOnce(&mut ThreadNonLocalState) -> R) -> R {
    assert!(
        crate::roots::is_vm_lock_owner(thread_key),
        "ThreadNonLocalState access requires active VM lock ownership on current OS thread"
    );
    let mut guard = THREAD_STATES
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let epoch = STATE_EPOCH.load(Ordering::Acquire);
    let map = guard.get_or_insert_with(HashMap::new);
    let entry = map
        .entry(thread_key)
        .or_insert_with(|| Box::new(ThreadNonLocalState::default()));
    let ptr: *mut ThreadNonLocalState = &mut **entry;
    TLS_STATE.with(|cell| {
        let mut s = cell.get();
        s.cache_epoch = epoch;
        s.cache_key = thread_key;
        s.cache_ptr = ptr;
        cell.set(s);
    });
    f(entry)
}

/// Slot-based scoped GC root guard for `Lisp_Object` values held across Lisp hooks, `unbind_to`
/// cleanups, or unwinding frames. Releases its slot deterministically on `Drop`, preserving other
/// live roots even under out-of-order drops.
pub(crate) struct ScopedLispRoot {
    thread_key: usize,
    slot_index: usize,
    generation: u64,
    len: usize,
    _not_send_sync: core::marker::PhantomData<*mut ()>,
}

impl ScopedLispRoot {
    /// Register a slice of initialized `Lisp_Object` values as a scoped root.
    ///
    /// # Safety
    /// Every element in `values` must be a valid tagged `Lisp_Object` for the live VM heap under
    /// the active VM global lock.
    #[inline(always)]
    pub(crate) unsafe fn new(values: &[Lisp_Object]) -> Self {
        let len = values.len();
        if len == 0 {
            RETIRED_SCOPED_ROOTS_CREATED.fetch_add(1, Ordering::Relaxed);
            return Self {
                thread_key: 0,
                slot_index: usize::MAX,
                generation: 0,
                len: 0,
                _not_send_sync: core::marker::PhantomData,
            };
        }
        let thread_key = current_thread_key();
        let (slot_index, generation) = with_thread_state(thread_key, |state| {
            state.scoped_roots_created += 1;
            let r#gen = state.next_slot_generation;
            state.next_slot_generation += 1;
            let idx = if let Some(free_idx) = state.free_slot_indices.pop() {
                let slot = &mut state.scoped_slots[free_idx];
                slot.generation = r#gen;
                slot.occupied = true;
                slot.len = len;
                slot.overflow.clear();
                if len <= 4 {
                    slot.inline[..len].copy_from_slice(values);
                } else {
                    slot.inline = [lisp_h_Qnil(); 4];
                    slot.overflow.extend_from_slice(values);
                }
                free_idx
            } else {
                let mut inline = [lisp_h_Qnil(); 4];
                let overflow = if len <= 4 {
                    inline[..len].copy_from_slice(values);
                    Vec::new()
                } else {
                    values.to_vec()
                };
                let idx = state.scoped_slots.len();
                state.scoped_slots.push(ScopedRootSlot {
                    generation: r#gen,
                    occupied: true,
                    len,
                    inline,
                    overflow,
                });
                idx
            };
            (idx, r#gen)
        });
        Self {
            thread_key,
            slot_index,
            generation,
            len,
            _not_send_sync: core::marker::PhantomData,
        }
    }

    /// Replace the `Lisp_Object` at slot `index`.
    ///
    /// # Safety
    /// `value` must be a valid tagged `Lisp_Object` under the active VM global lock.
    #[inline(always)]
    pub(crate) unsafe fn set(&mut self, index: usize, value: Lisp_Object) {
        assert!(index < self.len);
        let slot_index = self.slot_index;
        let expected_gen = self.generation;
        with_thread_state(self.thread_key, |state| {
            let slot = &mut state.scoped_slots[slot_index];
            assert!(
                slot.occupied && slot.generation == expected_gen,
                "stale ScopedLispRoot slot"
            );
            if slot.len <= 4 {
                slot.inline[index] = value;
            } else {
                slot.overflow[index] = value;
            }
        });
    }
}

impl Drop for ScopedLispRoot {
    #[inline(always)]
    fn drop(&mut self) {
        if self.len == 0 {
            RETIRED_SCOPED_ROOTS_DROPPED.fetch_add(1, Ordering::Relaxed);
            return;
        }
        let slot_index = self.slot_index;
        let expected_gen = self.generation;
        with_thread_state(self.thread_key, |state| {
            state.scoped_roots_dropped += 1;
            let slot = &mut state.scoped_slots[slot_index];
            assert!(
                slot.occupied && slot.generation == expected_gen,
                "ScopedLispRoot double-release or slot corruption"
            );
            slot.occupied = false;
            if slot.len <= 4 {
                slot.inline = [lisp_h_Qnil(); 4];
            } else {
                slot.overflow.clear();
            }
            slot.len = 0;
            state.free_slot_indices.push(slot_index);
        });
    }
}

/// Deterministic Rust host resource guard used to verify exact-once `Drop` execution across
/// multi-frame VM unwinds without leaking or double-dropping.
pub(crate) struct HostDropGuard {
    dropped: AtomicBool,
}

impl HostDropGuard {
    #[inline]
    pub(crate) fn new() -> Self {
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
pub(crate) fn host_drop_counts() -> (usize, usize) {
    (
        HOST_PROBES_CREATED.load(Ordering::Relaxed),
        HOST_PROBES_DROPPED.load(Ordering::Relaxed),
    )
}

#[inline]
pub(crate) fn scoped_root_counts() -> (usize, usize) {
    assert!(
        crate::roots::is_current_os_thread_vm_lock_owner(),
        "scoped_root_counts requires active VM lock ownership on current OS thread"
    );
    assert!(
        !TLS_STATE.with(|cell| cell.get().borrowed),
        "cannot read scoped_root_counts while ThreadNonLocalState is borrowed"
    );
    let mut created = RETIRED_SCOPED_ROOTS_CREATED.load(Ordering::Relaxed);
    let mut dropped = RETIRED_SCOPED_ROOTS_DROPPED.load(Ordering::Relaxed);
    let guard = THREAD_STATES
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(map) = guard.as_ref() {
        for state in map.values() {
            created += state.scoped_roots_created;
            dropped += state.scoped_roots_dropped;
        }
    }
    (created, dropped)
}

/// Mark all explicit `ScopedLispRoot`, in-flight `NonLocalExit`, and pending `.eln` exit roots
/// belonging to `thread` during GC (`mark_one_thread`).
///
/// # Safety
/// Must be called only while the current OS thread holds the VM global lock during GC marking.
pub(crate) unsafe fn mark_thread_explicit_roots(
    thread: *mut c_void,
    mut mark_fn: impl FnMut(Lisp_Object),
) {
    assert!(
        crate::roots::is_current_os_thread_vm_lock_owner(),
        "mark_thread_explicit_roots requires active VM lock ownership on current OS thread"
    );
    let key = thread as usize;
    if key == 0 {
        return;
    }
    assert!(
        !TLS_STATE.with(|cell| cell.get().borrowed),
        "cannot mark ThreadNonLocalState while state is borrowed"
    );
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
        let mut objs = Vec::with_capacity(state.scoped_slots.len() * 4 + 2);
        for slot in &state.scoped_slots {
            if slot.occupied {
                if slot.len <= 4 {
                    objs.extend_from_slice(&slot.inline[..slot.len]);
                } else {
                    objs.extend_from_slice(&slot.overflow);
                }
            }
        }
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
///
/// # Safety
/// Must be called only while the current OS thread holds the VM global lock and no `ScopedLispRoot`
/// handles or non-local exit frames referencing `thread` remain live.
pub(crate) unsafe fn cleanup_thread_state(thread: *mut c_void) {
    assert!(
        crate::roots::is_current_os_thread_vm_lock_owner(),
        "cleanup_thread_state requires active VM lock ownership on current OS thread"
    );
    let key = thread as usize;
    if key == 0 {
        return;
    }
    assert!(
        !TLS_STATE.with(|cell| cell.get().borrowed),
        "cannot cleanup ThreadNonLocalState while state is borrowed"
    );
    let mut guard = THREAD_STATES
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    STATE_EPOCH.fetch_add(1, Ordering::SeqCst);
    TLS_STATE.with(|cell| {
        let mut s = cell.get();
        s.cache_epoch = 0;
        s.cache_key = 0;
        s.cache_ptr = core::ptr::null_mut();
        cell.set(s);
    });
    if let Some(map) = guard.as_mut()
        && let Some(removed) = map.remove(&key)
    {
        RETIRED_SCOPED_ROOTS_CREATED.fetch_add(removed.scoped_roots_created, Ordering::Relaxed);
        RETIRED_SCOPED_ROOTS_DROPPED.fetch_add(removed.scoped_roots_dropped, Ordering::Relaxed);
        if removed.native_pending.pending {
            ACTIVE_COMP_NATIVE_EXITS.fetch_sub(1, Ordering::Relaxed);
        }
    }
}

/// Raise a new Lisp `NonLocalExit`, rooting its payload on the current thread and unwinding
/// Rust frames until a matching VM catcher or FFI boundary guard intercepts it.
///
/// # Safety
/// Must be called on the active VM-lock-owning thread with a valid `exit.target` handler and
/// tagged `exit.value` inside a VM unwind scope (`catch_vm_nonlocal`).
#[inline(never)]
pub(crate) unsafe fn raise_vm_nonlocal(exit: NonLocalExit) -> ! {
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
///
/// # Safety
/// `exit` must originate from an active enclosing `catch_vm_nonlocal` boundary on this thread.
#[inline(never)]
pub(crate) unsafe fn resume_vm_nonlocal(exit: NonLocalExit) -> ! {
    // SAFETY: caller forwards a live `NonLocalExit` from `catch_vm_nonlocal`.
    unsafe { raise_vm_nonlocal(exit) }
}

/// Run `body` and catch only Lisp `NonLocalExit` unwinds (`VmResult<R>`), re-propagating any
/// genuine Rust `panic!` untouched via `resume_unwind`.
#[inline]
pub(crate) fn catch_vm_nonlocal<R, F: FnOnce() -> R>(body: F) -> VmResult<R> {
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
///
/// # Safety
/// Must be called on the active VM-lock-owning thread with a valid `.eln` exit payload.
pub(crate) unsafe fn set_comp_native_exit(exit: NonLocalExit) {
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
pub(crate) fn comp_native_exit_pending_fast() -> bool {
    ACTIVE_COMP_NATIVE_EXITS.load(Ordering::Relaxed) != 0
}

pub(crate) fn comp_native_exit_pending_for_current_thread() -> c_int {
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
///
/// # Safety
/// Must be called on the active VM-lock-owning thread with `candidate` pointing to an active or
/// null handler slot only compared for identity.
pub(crate) unsafe fn consume_comp_native_exit_for_handler(candidate: *mut handler) -> c_int {
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
unsafe fn propagate_comp_native_exit_slow() {
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
        // SAFETY: converts pending `.eln` exit back into a `NonLocalExit` on the active VM thread.
        unsafe { raise_vm_nonlocal(exit) };
    }
}

/// Fast inline check executed after invoking a potentially native-compiled `Lisp_Subr` or
/// `top_level_run`: if the `.eln` frame returned an unhandled pending exit, convert it back into
/// a VM `NonLocalExit` unwind.
///
/// # Safety
/// Must be called immediately after a native-compiled return inside an active VM unwind scope on
/// the lock-owning thread.
#[inline(always)]
pub(crate) unsafe fn check_and_propagate_comp_native_exit() {
    if comp_native_exit_pending_fast() {
        // SAFETY: pending exit belongs to the active VM thread's native frame.
        unsafe { propagate_comp_native_exit_slow() };
    }
}

/// Host C-ABI probe entry points used by `AU3` dynamic-module and integration tests to verify
/// exact-once Rust `Drop` execution across multi-frame Lisp unwinds without polluting `obarray`.
///
/// # Safety
/// Must be called under the VM global lock; every non-null out pointer must be valid, aligned,
/// writable `usize` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn femacs_vm_host_drop_counts(
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
    // SAFETY: `fun` is a valid `Lisp_Object` passed on the active VM thread and `Ffuncall` reads
    // `nargs == 1` argument on that thread.
    let _root_guard = unsafe { ScopedLispRoot::new(&[fun]) };
    // SAFETY: `Ffuncall` expects `(nargs, args)` with `nargs >= 1` on the active Lisp thread.
    unsafe {
        let mut args = [fun];
        crate::eval::Ffuncall(1, args.as_mut_ptr())
    }
}
