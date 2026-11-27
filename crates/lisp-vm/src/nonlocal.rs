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
struct VmHostExitUnwind(i32);

/// Outcome of executing a top-level host body inside `catch_vm_nonlocal_or_host_exit`.
pub(crate) enum HostUnwindOutcome<T> {
    Ok(T),
    NonLocal(NonLocalExit),
    HostExit(i32),
}

#[derive(Clone, Copy)]
enum NativePendingKind {
    None,
    NonLocal(NonLocalExit),
    HostExit(i32),
}

#[derive(Clone, Copy)]
struct NativePendingExit {
    kind: NativePendingKind,
}

impl NativePendingExit {
    #[inline]
    fn is_pending(self) -> bool {
        !matches!(self.kind, NativePendingKind::None)
    }
}

impl Default for NativePendingExit {
    fn default() -> Self {
        Self {
            kind: NativePendingKind::None,
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
pub(crate) struct TlsNonLocalState {
    vm_lock_key: usize,
    borrowed: bool,
    cache_isolate: usize,
    cache_epoch: usize,
    cache_key: usize,
    cache_ptr: *mut ThreadNonLocalState,
}

impl TlsNonLocalState {
    const EMPTY: Self = Self {
        vm_lock_key: 0,
        borrowed: false,
        cache_isolate: 0,
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

#[cfg(feature = "isolate_vm")]
pub(crate) struct IsolateNonLocalState {
    pub(crate) thread_states: Mutex<Option<HashMap<usize, Box<ThreadNonLocalState>>>>,
    pub(crate) state_epoch: AtomicUsize,
    pub(crate) active_comp_native_exits: AtomicUsize,
    pub(crate) retired_scoped_roots_created: AtomicUsize,
    pub(crate) retired_scoped_roots_dropped: AtomicUsize,
    pub(crate) host_probes_created: AtomicUsize,
    pub(crate) host_probes_dropped: AtomicUsize,
}

#[cfg(feature = "isolate_vm")]
impl Default for IsolateNonLocalState {
    fn default() -> Self {
        Self {
            thread_states: Mutex::new(None),
            state_epoch: AtomicUsize::new(1),
            active_comp_native_exits: AtomicUsize::new(0),
            retired_scoped_roots_created: AtomicUsize::new(0),
            retired_scoped_roots_dropped: AtomicUsize::new(0),
            host_probes_created: AtomicUsize::new(0),
            host_probes_dropped: AtomicUsize::new(0),
        }
    }
}

#[inline(always)]
fn current_isolate_usize() -> usize {
    #[cfg(feature = "isolate_vm")]
    {
        crate::sys::isolate::current_isolate_ptr() as usize
    }
    #[cfg(not(feature = "isolate_vm"))]
    {
        0
    }
}

#[inline(always)]
fn nl_thread_states() -> &'static Mutex<Option<HashMap<usize, Box<ThreadNonLocalState>>>> {
    #[cfg(feature = "isolate_vm")]
    {
        let iso = crate::sys::isolate::current_isolate_ptr();
        if !iso.is_null() {
            // SAFETY: `iso` is uniquely entered via `IsolateScopeGuard` on the current thread.
            return unsafe { &(&*iso).nonlocal.thread_states };
        }
    }
    &THREAD_STATES
}

#[inline(always)]
fn nl_state_epoch() -> &'static AtomicUsize {
    #[cfg(feature = "isolate_vm")]
    {
        let iso = crate::sys::isolate::current_isolate_ptr();
        if !iso.is_null() {
            // SAFETY: `iso` is uniquely entered via `IsolateScopeGuard` on the current thread.
            return unsafe { &(&*iso).nonlocal.state_epoch };
        }
    }
    &STATE_EPOCH
}

#[inline(always)]
fn nl_active_comp_native_exits() -> &'static AtomicUsize {
    #[cfg(feature = "isolate_vm")]
    {
        let iso = crate::sys::isolate::current_isolate_ptr();
        if !iso.is_null() {
            // SAFETY: `iso` is uniquely entered via `IsolateScopeGuard` on the current thread.
            return unsafe { &(&*iso).nonlocal.active_comp_native_exits };
        }
    }
    &ACTIVE_COMP_NATIVE_EXITS
}

#[inline(always)]
fn nl_retired_scoped_roots_created() -> &'static AtomicUsize {
    #[cfg(feature = "isolate_vm")]
    {
        let iso = crate::sys::isolate::current_isolate_ptr();
        if !iso.is_null() {
            // SAFETY: `iso` is uniquely entered via `IsolateScopeGuard` on the current thread.
            return unsafe { &(&*iso).nonlocal.retired_scoped_roots_created };
        }
    }
    &RETIRED_SCOPED_ROOTS_CREATED
}

#[inline(always)]
fn nl_retired_scoped_roots_dropped() -> &'static AtomicUsize {
    #[cfg(feature = "isolate_vm")]
    {
        let iso = crate::sys::isolate::current_isolate_ptr();
        if !iso.is_null() {
            // SAFETY: `iso` is uniquely entered via `IsolateScopeGuard` on the current thread.
            return unsafe { &(&*iso).nonlocal.retired_scoped_roots_dropped };
        }
    }
    &RETIRED_SCOPED_ROOTS_DROPPED
}

#[inline(always)]
fn nl_host_probes_created() -> &'static AtomicUsize {
    #[cfg(feature = "isolate_vm")]
    {
        let iso = crate::sys::isolate::current_isolate_ptr();
        if !iso.is_null() {
            // SAFETY: `iso` is uniquely entered via `IsolateScopeGuard` on the current thread.
            return unsafe { &(&*iso).nonlocal.host_probes_created };
        }
    }
    &HOST_PROBES_CREATED
}

#[inline(always)]
fn nl_host_probes_dropped() -> &'static AtomicUsize {
    #[cfg(feature = "isolate_vm")]
    {
        let iso = crate::sys::isolate::current_isolate_ptr();
        if !iso.is_null() {
            // SAFETY: `iso` is uniquely entered via `IsolateScopeGuard` on the current thread.
            return unsafe { &(&*iso).nonlocal.host_probes_dropped };
        }
    }
    &HOST_PROBES_DROPPED
}

#[cfg(feature = "isolate_vm")]
pub(crate) fn swap_tls_state_for_isolate(owner_key: usize, iso_usize: usize) -> TlsNonLocalState {
    TLS_STATE.with(|cell| {
        let prev = cell.get();
        cell.set(TlsNonLocalState {
            vm_lock_key: owner_key,
            borrowed: false,
            cache_isolate: iso_usize,
            cache_epoch: 0,
            cache_key: 0,
            cache_ptr: core::ptr::null_mut(),
        });
        prev
    })
}

#[cfg(feature = "isolate_vm")]
pub(crate) fn restore_tls_state_after_isolate(prev: TlsNonLocalState) {
    TLS_STATE.with(|cell| cell.set(prev));
}

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
        s.cache_isolate = 0;
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
    let cur_iso = current_isolate_usize();
    if thread_key != 0 && state.vm_lock_key == thread_key && state.cache_isolate == cur_iso {
        let epoch = nl_state_epoch().load(Ordering::Acquire);
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
    let mut guard = nl_thread_states()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let epoch = nl_state_epoch().load(Ordering::Acquire);
    let cur_iso = current_isolate_usize();
    let map = guard.get_or_insert_with(HashMap::new);
    let entry = map
        .entry(thread_key)
        .or_insert_with(|| Box::new(ThreadNonLocalState::default()));
    let ptr: *mut ThreadNonLocalState = &mut **entry;
    TLS_STATE.with(|cell| {
        let mut s = cell.get();
        s.cache_isolate = cur_iso;
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
            nl_retired_scoped_roots_created().fetch_add(1, Ordering::Relaxed);
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
            nl_retired_scoped_roots_dropped().fetch_add(1, Ordering::Relaxed);
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
        nl_host_probes_created().fetch_add(1, Ordering::Relaxed);
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
        nl_host_probes_dropped().fetch_add(1, Ordering::Relaxed);
    }
}

#[inline]
pub(crate) fn host_drop_counts() -> (usize, usize) {
    (
        nl_host_probes_created().load(Ordering::Relaxed),
        nl_host_probes_dropped().load(Ordering::Relaxed),
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
    let mut created = nl_retired_scoped_roots_created().load(Ordering::Relaxed);
    let mut dropped = nl_retired_scoped_roots_dropped().load(Ordering::Relaxed);
    let guard = nl_thread_states()
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
        let guard = nl_thread_states()
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
        if let NativePendingKind::NonLocal(exit) = state.native_pending.kind {
            objs.push(exit.value);
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
    let mut guard = nl_thread_states()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    nl_state_epoch().fetch_add(1, Ordering::SeqCst);
    TLS_STATE.with(|cell| {
        let mut s = cell.get();
        s.cache_isolate = 0;
        s.cache_epoch = 0;
        s.cache_key = 0;
        s.cache_ptr = core::ptr::null_mut();
        cell.set(s);
    });
    if let Some(map) = guard.as_mut()
        && let Some(removed) = map.remove(&key)
    {
        nl_retired_scoped_roots_created().fetch_add(removed.scoped_roots_created, Ordering::Relaxed);
        nl_retired_scoped_roots_dropped().fetch_add(removed.scoped_roots_dropped, Ordering::Relaxed);
        if removed.native_pending.is_pending() {
            nl_active_comp_native_exits().fetch_sub(1, Ordering::Relaxed);
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
/// host-exit unwind (`VmHostExitUnwind`) or genuine Rust `panic!` untouched via `resume_unwind`.
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

/// Raise a headless `(kill-emacs CODE)` host-exit unwind bypassed by every Lisp handler/catch
/// frame and caught strictly at the outermost host boundary (`catch_vm_nonlocal_or_host_exit`).
#[inline(never)]
pub(crate) fn raise_host_exit(code: i32) -> ! {
    resume_unwind(Box::new(VmHostExitUnwind(code)))
}

/// Outermost host catch intercepting both `VmNonLocalUnwind` and `VmHostExitUnwind`, while
/// re-propagating any genuine Rust `panic!` untouched.
#[inline]
pub(crate) fn catch_vm_nonlocal_or_host_exit<R, F: FnOnce() -> R>(
    body: F,
) -> HostUnwindOutcome<R> {
    match catch_unwind(AssertUnwindSafe(body)) {
        Ok(val) => HostUnwindOutcome::Ok(val),
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
                HostUnwindOutcome::NonLocal(exit)
            }
            Err(payload) => match payload.downcast::<VmHostExitUnwind>() {
                Ok(boxed) => HostUnwindOutcome::HostExit(boxed.0),
                Err(other_panic) => resume_unwind(other_panic),
            },
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
        if matches!(state.native_pending.kind, NativePendingKind::HostExit(_)) {
            return;
        }
        if !state.native_pending.is_pending() {
            nl_active_comp_native_exits().fetch_add(1, Ordering::Relaxed);
        }
        state.native_pending = NativePendingExit {
            kind: NativePendingKind::NonLocal(exit),
        };
    });
}

/// Record a pending host-exit code at a C-ABI `.eln` helper boundary so no `VmHostExitUnwind`
/// unwinds across `.eln` C frames.
///
/// # Safety
/// Must be called on the active VM-lock-owning thread at an `.eln` helper boundary.
pub(crate) unsafe fn set_comp_native_host_exit(code: i32) {
    let key = current_thread_key();
    if key == 0 {
        return;
    }
    with_thread_state(key, |state| {
        if matches!(state.native_pending.kind, NativePendingKind::HostExit(_)) {
            return;
        }
        if !state.native_pending.is_pending() {
            nl_active_comp_native_exits().fetch_add(1, Ordering::Relaxed);
        }
        state.native_pending = NativePendingExit {
            kind: NativePendingKind::HostExit(code),
        };
    });
}

#[inline(always)]
pub(crate) fn comp_native_exit_pending_fast() -> bool {
    nl_active_comp_native_exits().load(Ordering::Relaxed) != 0
}

pub(crate) fn comp_native_exit_pending_for_current_thread() -> c_int {
    if !comp_native_exit_pending_fast() {
        return 0;
    }
    let key = current_thread_key();
    if key == 0 {
        return 0;
    }
    with_thread_state(key, |state| {
        c_int::from(state.native_pending.is_pending())
    })
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
    with_thread_state(key, |state| match state.native_pending.kind {
        NativePendingKind::NonLocal(exit) if exit.target == candidate => {
            state.native_pending = NativePendingExit::default();
            nl_active_comp_native_exits().fetch_sub(1, Ordering::Relaxed);
            1
        }
        NativePendingKind::None | NativePendingKind::NonLocal(_) | NativePendingKind::HostExit(_) => {
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
    let kind = with_thread_state(key, |state| {
        let taken = state.native_pending.kind;
        if state.native_pending.is_pending() {
            state.native_pending = NativePendingExit::default();
            nl_active_comp_native_exits().fetch_sub(1, Ordering::Relaxed);
        }
        taken
    });
    match kind {
        NativePendingKind::None => {}
        NativePendingKind::NonLocal(exit) => {
            // SAFETY: converts pending `.eln` exit back into a `NonLocalExit` on the active VM thread.
            unsafe { raise_vm_nonlocal(exit) };
        }
        NativePendingKind::HostExit(code) => {
            crate::host_runtime::resume_host_exit_unwind(code);
        }
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
