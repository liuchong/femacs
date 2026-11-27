//! SpiderMonkey-inspired GC rooting, handle, and `NoGcToken` API for `femacs-vm` (U4).
//!
//! Distinguishes three explicit value lifetimes across GC safepoints:
//!   1. Call-scope LIFO roots (`Rooted`, `Handle`, `MutableHandle`, `ScopedRawSpan`),
//!   2. Explicitly released long-lived host roots (`PersistentRoot`),
//!   3. Short unrooted borrows (`UnrootedBorrow<'nogc, T>`) statically invalidated by `&mut GcContext`
//!      before entering Lisp evaluation, allocation, callbacks, or thread switches.

#![deny(unsafe_op_in_unsafe_fn, static_mut_refs)]

use core::ffi::c_void;
use core::marker::PhantomData;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};

use crate::common::{
    EMACS_INT, FIXNUMP, Lisp_Object, XFIXNUM, current_thread, lisp_h_Qnil, make_fixnum,
};

/// Immediate Lisp value (`nil` or tagged fixnum) carrying zero GC heap reference.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Immediate(Lisp_Object);

// SAFETY: `Immediate` contains strictly `lisp_h_Qnil()` or a `FIXNUMP` integer word and never
// references GC-managed heap storage.
unsafe impl Send for Immediate {}
// SAFETY: `Immediate` is an immutable non-heap tagged immediate value.
unsafe impl Sync for Immediate {}

impl Immediate {
    #[inline]
    pub const fn nil() -> Self {
        Self(lisp_h_Qnil())
    }

    #[inline]
    pub fn from_fixnum(n: isize) -> Self {
        Self(make_fixnum(n as EMACS_INT))
    }

    #[inline]
    pub fn is_nil(self) -> bool {
        self.0 == lisp_h_Qnil()
    }

    #[inline]
    pub fn as_fixnum(self) -> Option<isize> {
        if FIXNUMP(self.0) {
            Some(XFIXNUM(self.0) as isize)
        } else {
            None
        }
    }

    #[inline]
    pub(crate) const fn into_lisp_val(self) -> LispVal {
        LispVal(self.0)
    }

    #[inline]
    pub(crate) const fn into_raw(self) -> Lisp_Object {
        self.0
    }
}

impl Default for Immediate {
    #[inline]
    fn default() -> Self {
        Self::nil()
    }
}

/// Crate-internal copyable wrapper around a tagged `Lisp_Object` word (`!Send + !Sync`).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct LispVal(Lisp_Object);

impl LispVal {
    #[inline]
    pub(crate) const fn nil() -> Self {
        Self(lisp_h_Qnil())
    }

    /// Construct a `LispVal` from a raw `Lisp_Object` word.
    ///
    /// # Safety
    /// `raw` must be a valid tagged `Lisp_Object` for the live VM heap (or an immediate fixnum /
    /// constant / symbol) under the active VM global lock.
    #[inline]
    pub(crate) unsafe fn from_raw(raw: Lisp_Object) -> Self {
        Self(raw)
    }

    #[inline]
    pub(crate) const fn into_raw(self) -> Lisp_Object {
        self.0
    }

    #[inline]
    pub(crate) fn is_nil(self) -> bool {
        self.0 == lisp_h_Qnil()
    }

    #[inline]
    pub(crate) fn from_fixnum(n: isize) -> Self {
        Self(make_fixnum(n as EMACS_INT))
    }

    #[inline]
    pub(crate) fn as_fixnum(self) -> Option<isize> {
        if FIXNUMP(self.0) {
            Some(XFIXNUM(self.0) as isize)
        } else {
            None
        }
    }
}

impl Default for LispVal {
    #[inline]
    fn default() -> Self {
        Self::nil()
    }
}

/// Token proving no GC, Lisp evaluation, callback, or thread lock release can occur while held.
pub struct NoGcToken<'a> {
    _marker: PhantomData<(&'a GcContext, *const ())>,
}

/// Short-lived borrow bound to `&'nogc NoGcToken` so Rust's borrow checker statically ends the
/// borrow before any `&mut GcContext` operation (`eval`, `funcall`, `garbage_collect`, `yield`).
#[derive(Clone, Copy, Debug)]
pub struct UnrootedBorrow<'nogc, T> {
    value: T,
    _nogc: PhantomData<(&'nogc NoGcToken<'nogc>, *const ())>,
}

impl<'nogc, T: Copy> UnrootedBorrow<'nogc, T> {
    #[inline]
    pub fn new(_token: &'nogc NoGcToken<'nogc>, value: T) -> Self {
        Self {
            value,
            _nogc: PhantomData,
        }
    }

    #[inline]
    pub fn get(self) -> T {
        self.value
    }
}

/// Exclusive single-lease execution context representing ownership of the VM lock on the current
/// OS thread and Lisp `thread_state` across GC safepoints.
pub struct GcContext {
    thread_key: usize,
    _not_send_sync: PhantomData<*mut ()>,
}

impl Drop for GcContext {
    #[inline]
    fn drop(&mut self) {
        GC_CONTEXT_ACTIVE.store(false, Ordering::Release);
    }
}

impl GcContext {
    /// Obtain the sole active `GcContext` iff the current OS thread holds the VM global lock for
    /// `current_thread` and no outer `GcContext` lease is currently active.
    #[inline]
    pub fn current() -> Option<Self> {
        let key = current_thread_key();
        if !is_vm_lock_owner(key) {
            return None;
        }
        if GC_CONTEXT_ACTIVE
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            return None;
        }
        Some(Self {
            thread_key: key,
            _not_send_sync: PhantomData,
        })
    }

    /// Push a scoped immediate GC root bound to the current VM-lock-owning thread.
    #[inline]
    pub fn root_immediate(&self, initial: Immediate) -> Rooted {
        assert!(
            is_vm_lock_owner(self.thread_key),
            "GcContext::root_immediate requires active VM lock ownership"
        );
        // SAFETY: `Immediate` is restricted to nil or fixnum (never a GC heap pointer).
        unsafe { Rooted::new_raw(initial.into_lisp_val()) }
    }

    /// Push a fresh `Rooted` slot initialized from an already-rooted `Handle`.
    #[inline]
    pub fn root_handle(&self, handle: Handle<'_>) -> Rooted {
        assert!(
            is_vm_lock_owner(self.thread_key),
            "GcContext::root_handle requires active VM lock ownership"
        );
        let raw = handle.get_raw();
        // SAFETY: `handle` proves `raw` is live in an occupied root slot under the VM lock.
        unsafe { Rooted::new_raw(raw) }
    }

    /// Borrow a `NoGcToken` that statically blocks `&mut self` safepoint calls until dropped.
    #[inline]
    pub fn nogc(&self) -> NoGcToken<'_> {
        NoGcToken {
            _marker: PhantomData,
        }
    }

    /// Enter a safepoint operation, statically invalidating all outstanding `NoGcToken` borrows.
    #[inline]
    pub fn safepoint<R>(&mut self, f: impl FnOnce(&mut Self) -> R) -> R {
        f(self)
    }

    /// Snapshot active and retired GC rooting counters while proving exclusive VM lock ownership.
    #[inline]
    pub fn rooting_counters(&self) -> RootingCounters {
        debug_assert!(is_vm_lock_owner(self.thread_key));
        rooting_counters()
    }

    /// Execute `body` with `(*current_thread).m_stack_bottom` narrowed to this entry boundary
    /// (`U7` / `P-486`), excluding outer host/Rust frames that hold only explicit
    /// `Rooted`/`PersistentRoot`/`RootedVec` handles from conservative `mark_c_stack` scanning.
    #[inline(never)]
    pub fn with_narrowed_stack_bottom<R>(&mut self, body: impl FnOnce(&mut Self) -> R) -> R {
        assert!(
            is_vm_lock_owner(self.thread_key),
            "GcContext::with_narrowed_stack_bottom requires active VM lock ownership on current OS thread"
        );
        let anchor: usize = 0;
        let anchor_ptr = (&raw const anchor) as *const core::ffi::c_char;
        // SAFETY: `self.thread_key` is verified as the active VM lock owner; reads/updates
        // `m_stack_bottom` on the owning Lisp thread and restores it cleanly via `Drop`.
        let guard = unsafe {
            let ct = current_thread.get();
            if ct.is_null() || (*ct).m_stack_bottom.is_null() {
                NarrowedStackScanGuard {
                    thread_ptr: core::ptr::null_mut(),
                    saved_bottom: core::ptr::null(),
                    active: false,
                }
            } else {
                let saved = (*ct).m_stack_bottom;
                let should_narrow = (anchor_ptr as usize) < (saved as usize);
                if should_narrow {
                    (*ct).m_stack_bottom = anchor_ptr;
                    NARROWED_STACK_ENTRIES.fetch_add(1, Ordering::Relaxed);
                }
                NarrowedStackScanGuard {
                    thread_ptr: ct,
                    saved_bottom: saved,
                    active: should_narrow,
                }
            }
        };
        let result = body(self);
        drop(guard);
        core::hint::black_box(anchor);
        result
    }

    /// Temporarily release `GC_CONTEXT_ACTIVE` while `&mut self` is exclusively borrowed across a
    /// VM callback/evaluation safepoint (preventing any overlapping `NoGcToken`), allowing
    /// re-entrant dynamic-module `GcContext::current()` on the same lock-owning OS thread.
    #[inline]
    pub(crate) fn with_reentrant_safepoint<R>(&mut self, f: impl FnOnce() -> R) -> R {
        assert!(
            is_vm_lock_owner(self.thread_key),
            "GcContext safepoint requires active VM lock ownership on current OS thread"
        );
        struct RestoreContextActive(bool);
        impl Drop for RestoreContextActive {
            #[inline]
            fn drop(&mut self) {
                if self.0 {
                    let was_clear = GC_CONTEXT_ACTIVE
                        .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                        .is_ok();
                    assert!(
                        was_clear,
                        "re-entrant GcContext handle escaped inner safepoint"
                    );
                }
            }
        }
        let suspended = GC_CONTEXT_ACTIVE
            .compare_exchange(true, false, Ordering::AcqRel, Ordering::Acquire)
            .is_ok();
        let _restore = RestoreContextActive(suspended);
        f()
    }
}

#[derive(Clone, Copy)]
struct ThreadRootSlot {
    generation: u64,
    occupied: bool,
    value: LispVal,
}

#[derive(Clone, Copy)]
struct RawSpanSlot {
    generation: u64,
    occupied: bool,
    ptr: usize,
    len: usize,
}

struct ThreadRootSlab {
    slots: Vec<ThreadRootSlot>,
    free_slot_indices: Vec<usize>,
    next_slot_generation: u64,
    raw_spans: Vec<RawSpanSlot>,
    free_span_indices: Vec<usize>,
    next_span_generation: u64,
    rooted_pushed: usize,
    rooted_popped: usize,
    raw_spans_pushed: usize,
    raw_spans_popped: usize,
}

// SAFETY: `ThreadRootSlab` is housed inside `THREAD_ROOT_SLABS` (protected by `Mutex`) and
// accessed otherwise only by the OS thread holding the VM global lock.
unsafe impl Send for ThreadRootSlab {}

impl Default for ThreadRootSlab {
    fn default() -> Self {
        Self {
            slots: Vec::with_capacity(32),
            free_slot_indices: Vec::with_capacity(16),
            next_slot_generation: 1,
            raw_spans: Vec::with_capacity(16),
            free_span_indices: Vec::with_capacity(8),
            next_span_generation: 1,
            rooted_pushed: 0,
            rooted_popped: 0,
            raw_spans_pushed: 0,
            raw_spans_popped: 0,
        }
    }
}

#[derive(Clone, Copy)]
struct PersistentSlot {
    generation: u64,
    occupied: bool,
    value: LispVal,
}

// SAFETY: `PersistentSlot` lives solely inside `PERSISTENT_ROOTS` (protected by `Mutex`).
unsafe impl Send for PersistentSlot {}

struct PersistentRootTable {
    slots: Vec<PersistentSlot>,
    free_indices: Vec<usize>,
    next_generation: u64,
    live_count: usize,
}

impl Default for PersistentRootTable {
    fn default() -> Self {
        Self {
            slots: Vec::with_capacity(32),
            free_indices: Vec::with_capacity(16),
            next_generation: 1,
            live_count: 0,
        }
    }
}

static THREAD_ROOT_SLABS: Mutex<Option<HashMap<usize, Box<ThreadRootSlab>>>> = Mutex::new(None);
static PERSISTENT_ROOTS: Mutex<Option<PersistentRootTable>> = Mutex::new(None);
static SLAB_EPOCH: AtomicUsize = AtomicUsize::new(1);
static NEXT_OS_TID: AtomicU64 = AtomicU64::new(1);
static VM_LOCK_OWNER_THREAD_KEY: AtomicUsize = AtomicUsize::new(0);
static VM_LOCK_OWNER_OS_TID: AtomicU64 = AtomicU64::new(0);
static GC_CONTEXT_ACTIVE: AtomicBool = AtomicBool::new(false);

#[derive(Clone, Copy)]
struct TlsSlabState {
    os_tid: u64,
    vm_lock_key: usize,
    borrowed: bool,
    cache_epoch: usize,
    cache_key: usize,
    cache_ptr: *mut ThreadRootSlab,
}

impl TlsSlabState {
    const EMPTY: Self = Self {
        os_tid: 0,
        vm_lock_key: 0,
        borrowed: false,
        cache_epoch: 0,
        cache_key: 0,
        cache_ptr: core::ptr::null_mut(),
    };
}

std::thread_local! {
    static TLS_SLAB: core::cell::Cell<TlsSlabState> = const { core::cell::Cell::new(TlsSlabState::EMPTY) };
}

static RETIRED_ROOTED_PUSHED: AtomicUsize = AtomicUsize::new(0);
static RETIRED_ROOTED_POPPED: AtomicUsize = AtomicUsize::new(0);
static RETIRED_RAW_SPANS_PUSHED: AtomicUsize = AtomicUsize::new(0);
static RETIRED_RAW_SPANS_POPPED: AtomicUsize = AtomicUsize::new(0);
static PERSISTENT_CREATED: AtomicUsize = AtomicUsize::new(0);
static PERSISTENT_RELEASED: AtomicUsize = AtomicUsize::new(0);
static NARROWED_STACK_ENTRIES: AtomicUsize = AtomicUsize::new(0);
static NARROWED_STACK_RESTORES: AtomicUsize = AtomicUsize::new(0);

#[inline]
fn current_os_tid() -> u64 {
    TLS_SLAB.with(|cell| {
        let mut s = cell.get();
        if s.os_tid == 0 {
            s.os_tid = NEXT_OS_TID.fetch_add(1, Ordering::Relaxed);
            cell.set(s);
        }
        s.os_tid
    })
}

#[inline]
pub(crate) fn current_thread_key() -> usize {
    // SAFETY: `current_thread` is the single-VM active thread pointer `VmGlobal` extern slot.
    unsafe { current_thread.get() as usize }
}

/// Bind VM global lock ownership to `(current_os_tid(), thread_key)`.
///
/// # Safety
/// Caller must physically hold the VM's `global_lock` on the current OS thread (or run
/// single-threaded during early initialization before secondary threads exist), and `thread_key`
/// must be a valid `thread_state` pointer.
#[inline]
pub(crate) unsafe fn bind_vm_lock_owner(thread_key: usize) {
    assert_ne!(thread_key, 0, "bind_vm_lock_owner requires non-null thread");
    let tid = current_os_tid();
    let prev_tid = VM_LOCK_OWNER_OS_TID.load(Ordering::Acquire);
    assert!(
        prev_tid == 0 || prev_tid == tid,
        "bind_vm_lock_owner called by OS thread {tid} while OS thread {prev_tid} still owns VM lock"
    );
    TLS_SLAB.with(|cell| {
        let mut s = cell.get();
        s.vm_lock_key = thread_key;
        cell.set(s);
    });
    crate::nonlocal::set_tls_vm_lock_key(thread_key);
    VM_LOCK_OWNER_OS_TID.store(tid, Ordering::Release);
    VM_LOCK_OWNER_THREAD_KEY.store(thread_key, Ordering::Release);
}

/// Clear VM global lock ownership immediately before releasing `global_lock`.
///
/// # Safety
/// Caller must currently own the VM's physical `global_lock` on this OS thread and have ended all
/// active `GcContext`, `ThreadRootSlab`, and `ThreadNonLocalState` borrows on this thread.
#[inline]
pub(crate) unsafe fn unbind_vm_lock_owner() {
    let tid = current_os_tid();
    let prev_tid = VM_LOCK_OWNER_OS_TID.load(Ordering::Acquire);
    assert!(
        prev_tid == 0 || prev_tid == tid,
        "unbind_vm_lock_owner called by OS thread {tid} while OS thread {prev_tid} owns VM lock"
    );
    assert!(
        !GC_CONTEXT_ACTIVE.load(Ordering::Acquire),
        "cannot release VM lock while an active GcContext lease exists"
    );
    assert!(
        !TLS_SLAB.with(|cell| cell.get().borrowed),
        "cannot release VM lock while ThreadRootSlab is borrowed"
    );
    assert!(
        !crate::nonlocal::tls_state_borrowed(),
        "cannot release VM lock while ThreadNonLocalState is borrowed"
    );
    TLS_SLAB.with(|cell| {
        let mut s = cell.get();
        s.vm_lock_key = 0;
        s.cache_epoch = 0;
        s.cache_key = 0;
        s.cache_ptr = core::ptr::null_mut();
        cell.set(s);
    });
    crate::nonlocal::clear_tls_state_cache();
    VM_LOCK_OWNER_THREAD_KEY.store(0, Ordering::Release);
    VM_LOCK_OWNER_OS_TID.store(0, Ordering::Release);
}

#[inline]
pub(crate) fn tls_vm_lock_matches(thread_key: usize) -> bool {
    thread_key != 0 && TLS_SLAB.with(|cell| cell.get().vm_lock_key) == thread_key
}

#[inline]
pub(crate) fn is_current_os_thread_vm_lock_owner() -> bool {
    let owner_key = VM_LOCK_OWNER_THREAD_KEY.load(Ordering::Acquire);
    owner_key != 0
        && VM_LOCK_OWNER_OS_TID.load(Ordering::Acquire) == current_os_tid()
        && TLS_SLAB.with(|cell| cell.get().vm_lock_key) == owner_key
}

#[inline]
pub(crate) fn is_vm_lock_owner(thread_key: usize) -> bool {
    thread_key != 0
        && current_thread_key() == thread_key
        && VM_LOCK_OWNER_THREAD_KEY.load(Ordering::Acquire) == thread_key
        && VM_LOCK_OWNER_OS_TID.load(Ordering::Acquire) == current_os_tid()
        && TLS_SLAB.with(|cell| cell.get().vm_lock_key) == thread_key
}

struct SlabBorrowGuard {
    prev: bool,
}

impl Drop for SlabBorrowGuard {
    #[inline(always)]
    fn drop(&mut self) {
        TLS_SLAB.with(|cell| {
            let mut s = cell.get();
            s.borrowed = self.prev;
            cell.set(s);
        });
    }
}

#[inline(always)]
fn with_thread_slab<R>(thread_key: usize, f: impl FnOnce(&mut ThreadRootSlab) -> R) -> R {
    let state = TLS_SLAB.with(|cell| {
        let s = cell.get();
        assert!(!s.borrowed, "re-entrant ThreadRootSlab borrow forbidden");
        let mut next = s;
        next.borrowed = true;
        cell.set(next);
        s
    });
    let _borrow_guard = SlabBorrowGuard { prev: state.borrowed };
    if thread_key != 0 && state.vm_lock_key == thread_key {
        let epoch = SLAB_EPOCH.load(Ordering::Acquire);
        if state.cache_epoch == epoch && state.cache_key == thread_key && !state.cache_ptr.is_null() {
            // SAFETY: `state.cache_ptr` points to a heap-pinned `Box<ThreadRootSlab>` in
            // `THREAD_ROOT_SLABS`; `pub(crate) unsafe fn bind_vm_lock_owner` restricts `vm_lock_key`
            // to the sole VM lock holder, `SlabBorrowGuard` guarantees unique `&mut`, and
            // `cleanup_thread_u4_slab` bumps `SLAB_EPOCH` (`SeqCst`) under VM lock before removal.
            return f(unsafe { &mut *state.cache_ptr });
        }
    }
    with_thread_slab_slow(thread_key, f)
}

#[cold]
#[inline(never)]
fn with_thread_slab_slow<R>(thread_key: usize, f: impl FnOnce(&mut ThreadRootSlab) -> R) -> R {
    assert!(
        is_vm_lock_owner(thread_key),
        "ThreadRootSlab access requires active VM lock ownership on current OS thread"
    );
    let mut guard = THREAD_ROOT_SLABS
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let epoch = SLAB_EPOCH.load(Ordering::Acquire);
    let map = guard.get_or_insert_with(HashMap::new);
    let slab = map
        .entry(thread_key)
        .or_insert_with(|| Box::new(ThreadRootSlab::default()));
    let ptr: *mut ThreadRootSlab = &mut **slab;
    TLS_SLAB.with(|cell| {
        let mut s = cell.get();
        s.cache_epoch = epoch;
        s.cache_key = thread_key;
        s.cache_ptr = ptr;
        cell.set(s);
    });
    f(slab)
}

fn with_persistent_table<R>(f: impl FnOnce(&mut PersistentRootTable) -> R) -> R {
    let mut guard = PERSISTENT_ROOTS
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let table = guard.get_or_insert_with(PersistentRootTable::default);
    f(table)
}

/// Shared GC-safe handle (`SpiderMonkey JS::Handle<Value>`) borrowing an active root slot.
#[derive(Clone, Copy, Debug)]
pub struct Handle<'a> {
    thread_key: usize,
    slot_or_index: usize,
    is_persistent: bool,
    generation: u64,
    _lifetime: PhantomData<(*const (), &'a ())>,
}

impl<'a> Handle<'a> {
    #[inline]
    pub(crate) fn get_raw(self) -> LispVal {
        if self.is_persistent {
            with_persistent_table(|table| {
                let slot = table
                    .slots
                    .get(self.slot_or_index)
                    .copied()
                    .expect("PersistentRoot Handle slot index out of bounds");
                assert!(
                    slot.occupied && slot.generation == self.generation,
                    "stale or released persistent root handle"
                );
                slot.value
            })
        } else {
            let idx = self.slot_or_index;
            let expected_gen = self.generation;
            with_thread_slab(self.thread_key, |slab| {
                let slot = slab
                    .slots
                    .get(idx)
                    .copied()
                    .expect("Handle slot index out of bounds");
                assert!(
                    slot.occupied && slot.generation == expected_gen,
                    "stale or released thread root handle"
                );
                slot.value
            })
        }
    }

    #[inline]
    pub fn is_nil(self) -> bool {
        self.get_raw().is_nil()
    }

    #[inline]
    pub fn as_fixnum(self) -> Option<isize> {
        self.get_raw().as_fixnum()
    }

    #[inline]
    pub fn as_immediate(self) -> Option<Immediate> {
        let raw = self.get_raw().into_raw();
        if raw == lisp_h_Qnil() || FIXNUMP(raw) {
            Some(Immediate(raw))
        } else {
            None
        }
    }

    #[inline]
    pub fn same_object(self, other: Handle<'_>) -> bool {
        self.get_raw().into_raw() == other.get_raw().into_raw()
    }
}

/// Mutable GC-safe handle (`SpiderMonkey JS::MutableHandle<Value>`) referencing a `Rooted` slot.
pub struct MutableHandle<'a> {
    rooted: &'a mut Rooted,
}

impl<'a> MutableHandle<'a> {
    #[inline]
    pub(crate) fn get_raw(&self) -> LispVal {
        self.rooted.get_raw()
    }

    /// Replace the slot word with a caller-validated raw `LispVal`.
    ///
    /// # Safety
    /// `value` must be a valid tagged `Lisp_Object` for the live VM heap under the active VM lock.
    #[inline]
    pub(crate) unsafe fn set_raw(&mut self, value: LispVal) {
        // SAFETY: caller guarantees `value` is a live tagged `Lisp_Object` under the VM lock.
        unsafe { self.rooted.set_raw(value) };
    }

    #[inline]
    pub fn is_nil(&self) -> bool {
        self.rooted.is_nil()
    }

    #[inline]
    pub fn as_fixnum(&self) -> Option<isize> {
        self.rooted.as_fixnum()
    }

    #[inline]
    pub fn as_immediate(&self) -> Option<Immediate> {
        self.rooted.as_immediate()
    }

    #[inline]
    pub fn set_immediate(&mut self, value: Immediate) {
        self.rooted.set_immediate(value);
    }

    #[inline]
    pub fn set_from_handle(&mut self, handle: Handle<'_>) {
        self.rooted.set_from_handle(handle);
    }

    #[inline]
    pub fn handle(&self) -> Handle<'_> {
        self.rooted.handle()
    }
}

/// Scoped thread-local GC root (`SpiderMonkey JS::Rooted<Value>`) with generation-checked
/// independent slot release (sound even under out-of-order drops).
pub struct Rooted {
    thread_key: usize,
    index: usize,
    generation: u64,
    _not_send_sync: PhantomData<*mut ()>,
}

impl Rooted {
    /// Allocate a `Rooted` slot on the current VM-lock-owning thread from a raw `LispVal`.
    ///
    /// # Safety
    /// `initial` must be a valid tagged `Lisp_Object` for the live VM heap (or immediate) under
    /// active VM global lock ownership before any intervening safepoint.
    #[inline]
    pub(crate) unsafe fn new_raw(initial: LispVal) -> Self {
        let thread_key = current_thread_key();
        let (index, generation) = with_thread_slab(thread_key, |slab| {
            slab.rooted_pushed += 1;
            let r#gen = slab.next_slot_generation;
            slab.next_slot_generation += 1;
            let idx = if let Some(free_idx) = slab.free_slot_indices.pop() {
                slab.slots[free_idx] = ThreadRootSlot {
                    generation: r#gen,
                    occupied: true,
                    value: initial,
                };
                free_idx
            } else {
                let idx = slab.slots.len();
                slab.slots.push(ThreadRootSlot {
                    generation: r#gen,
                    occupied: true,
                    value: initial,
                });
                idx
            };
            (idx, r#gen)
        });
        Self {
            thread_key,
            index,
            generation,
            _not_send_sync: PhantomData,
        }
    }

    #[inline]
    pub fn from_immediate(ctx: &GcContext, initial: Immediate) -> Self {
        ctx.root_immediate(initial)
    }

    #[inline]
    pub fn from_handle(ctx: &GcContext, handle: Handle<'_>) -> Self {
        ctx.root_handle(handle)
    }

    #[inline]
    pub(crate) fn get_raw(&self) -> LispVal {
        let idx = self.index;
        let expected_gen = self.generation;
        with_thread_slab(self.thread_key, |slab| {
            let slot = slab.slots[idx];
            assert!(
                slot.occupied && slot.generation == expected_gen,
                "stale Rooted slot"
            );
            slot.value
        })
    }

    /// Replace the slot word with a caller-validated raw `LispVal`.
    ///
    /// # Safety
    /// `value` must be a valid tagged `Lisp_Object` for the live VM heap under the active VM lock.
    #[inline]
    pub(crate) unsafe fn set_raw(&mut self, value: LispVal) {
        let idx = self.index;
        let expected_gen = self.generation;
        with_thread_slab(self.thread_key, |slab| {
            let slot = &mut slab.slots[idx];
            assert!(
                slot.occupied && slot.generation == expected_gen,
                "stale Rooted slot"
            );
            slot.value = value;
        });
    }

    #[inline]
    pub fn is_nil(&self) -> bool {
        self.get_raw().is_nil()
    }

    #[inline]
    pub fn as_fixnum(&self) -> Option<isize> {
        self.get_raw().as_fixnum()
    }

    #[inline]
    pub fn as_immediate(&self) -> Option<Immediate> {
        let raw = self.get_raw().into_raw();
        if raw == lisp_h_Qnil() || FIXNUMP(raw) {
            Some(Immediate(raw))
        } else {
            None
        }
    }

    #[inline]
    pub fn same_object(&self, other: Handle<'_>) -> bool {
        self.get_raw().into_raw() == other.get_raw().into_raw()
    }

    #[inline]
    pub fn set_immediate(&mut self, value: Immediate) {
        // SAFETY: `Immediate` contains nil or fixnum only (never a GC heap pointer).
        unsafe { self.set_raw(value.into_lisp_val()) };
    }

    #[inline]
    pub fn set_from_handle(&mut self, handle: Handle<'_>) {
        let raw = handle.get_raw();
        // SAFETY: `handle` proves `raw` is live in an occupied root slot under the VM lock.
        unsafe { self.set_raw(raw) };
    }

    #[inline]
    pub fn handle(&self) -> Handle<'_> {
        Handle {
            thread_key: self.thread_key,
            slot_or_index: self.index,
            is_persistent: false,
            generation: self.generation,
            _lifetime: PhantomData,
        }
    }

    #[inline]
    pub fn handle_mut(&mut self) -> MutableHandle<'_> {
        MutableHandle { rooted: self }
    }
}

impl Drop for Rooted {
    #[inline]
    fn drop(&mut self) {
        let idx = self.index;
        let expected_gen = self.generation;
        with_thread_slab(self.thread_key, |slab| {
            slab.rooted_popped += 1;
            let slot = &mut slab.slots[idx];
            assert!(
                slot.occupied && slot.generation == expected_gen,
                "Rooted double-release or slot corruption"
            );
            slot.occupied = false;
            slot.value = LispVal::nil();
            slab.free_slot_indices.push(idx);
        });
    }
}

/// Slot-based registration for an active stack/alloca/Vec array of `Lisp_Object` slots during
/// evaluation (`eval_sub`, `apply_lambda`, `Fprog1`, `Flet`, `RootedVec`), supporting independent
/// release under arbitrary drop order.
pub(crate) struct ScopedRawSpan {
    thread_key: usize,
    span_index: usize,
    generation: u64,
    _not_send_sync: PhantomData<*mut ()>,
}

impl ScopedRawSpan {
    /// Register `[ptr .. ptr + initial_len)` as an active per-thread GC root span.
    ///
    /// # Safety
    /// `ptr` must remain valid for reading at least `active_len` initialized `Lisp_Object` words
    /// on the owning Lisp thread until this `ScopedRawSpan` is dropped.
    #[inline(always)]
    pub(crate) unsafe fn new(ptr: *const Lisp_Object, initial_len: usize) -> Self {
        let thread_key = current_thread_key();
        let (span_index, generation) = with_thread_slab(thread_key, |slab| {
            slab.raw_spans_pushed += 1;
            let r#gen = slab.next_span_generation;
            slab.next_span_generation += 1;
            let raw_ptr = if ptr.is_null() { 0 } else { ptr as usize };
            let idx = if let Some(free_idx) = slab.free_span_indices.pop() {
                slab.raw_spans[free_idx] = RawSpanSlot {
                    generation: r#gen,
                    occupied: true,
                    ptr: raw_ptr,
                    len: initial_len,
                };
                free_idx
            } else {
                let idx = slab.raw_spans.len();
                slab.raw_spans.push(RawSpanSlot {
                    generation: r#gen,
                    occupied: true,
                    ptr: raw_ptr,
                    len: initial_len,
                });
                idx
            };
            (idx, r#gen)
        });
        Self {
            thread_key,
            span_index,
            generation,
            _not_send_sync: PhantomData,
        }
    }

    /// Update the number of initialized `Lisp_Object` elements in this span.
    ///
    /// # Safety
    /// At least `new_len` elements starting at the registered `ptr` must be initialized.
    #[inline]
    pub(crate) unsafe fn set_len(&mut self, new_len: usize) {
        let idx = self.span_index;
        let expected_gen = self.generation;
        with_thread_slab(self.thread_key, |slab| {
            let span = &mut slab.raw_spans[idx];
            assert!(
                span.occupied && span.generation == expected_gen,
                "stale ScopedRawSpan slot"
            );
            span.len = new_len;
        });
    }

    #[inline]
    fn update_ptr_and_len(&mut self, new_ptr: *const Lisp_Object, new_len: usize) {
        let idx = self.span_index;
        let expected_gen = self.generation;
        let raw_ptr = if new_ptr.is_null() {
            0
        } else {
            new_ptr as usize
        };
        with_thread_slab(self.thread_key, |slab| {
            let span = &mut slab.raw_spans[idx];
            assert!(
                span.occupied && span.generation == expected_gen,
                "stale ScopedRawSpan slot"
            );
            span.ptr = raw_ptr;
            span.len = new_len;
        });
    }
}

/// Safe heap-backed `Vec<Lisp_Object>` whose active slice is registered as a LIFO GC root span on
/// the owning Lisp thread (`U7` / `P-486`), replacing `SAFE_NALLOCA!` + manual `mark_objects` in
/// `#![forbid(unsafe_code)]` subsystems (`safe_callint`, `safe_sort`, `safe_json`).
pub(crate) struct RootedVec {
    items: Vec<Lisp_Object>,
    span: ScopedRawSpan,
}

impl RootedVec {
    #[inline]
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    #[inline]
    pub fn with_capacity(cap: usize) -> Self {
        let items = Vec::with_capacity(cap);
        let ptr = items.as_ptr();
        // SAFETY: `items` has length 0; `span` tracks `(items.as_ptr(), items.len())` across all
        // mutations and truncates the LIFO slot before `items` drops.
        let span = unsafe { ScopedRawSpan::new(ptr, 0) };
        Self { items, span }
    }

    #[inline]
    pub fn from_slice(slice: &[Lisp_Object]) -> Self {
        let items = slice.to_vec();
        let ptr = items.as_ptr();
        // SAFETY: `items` holds `slice.len()` initialized `Lisp_Object` elements until `Self` drops.
        let span = unsafe { ScopedRawSpan::new(ptr, items.len()) };
        Self { items, span }
    }

    #[inline]
    pub fn filled(len: usize, init: Lisp_Object) -> Self {
        let items = vec![init; len];
        let ptr = items.as_ptr();
        // SAFETY: `items` holds `len` initialized `Lisp_Object` elements until `Self` drops, and
        // field drop order drops `items` only after `span` or keeps `items` valid while `span` is
        // registered (explicit `Drop` below truncates `span` first).
        let span = unsafe { ScopedRawSpan::new(ptr, len) };
        Self { items, span }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    #[inline]
    pub fn push(&mut self, value: Lisp_Object) {
        self.items.push(value);
        self.span
            .update_ptr_and_len(self.items.as_ptr(), self.items.len());
    }

    #[inline]
    pub fn truncate(&mut self, len: usize) {
        self.items.truncate(len);
        self.span
            .update_ptr_and_len(self.items.as_ptr(), self.items.len());
    }

    #[inline]
    pub fn clear(&mut self) {
        self.truncate(0);
    }

    #[inline]
    pub fn resize(&mut self, new_len: usize, value: Lisp_Object) {
        self.items.resize(new_len, value);
        self.span
            .update_ptr_and_len(self.items.as_ptr(), self.items.len());
    }

    #[inline]
    pub fn as_slice(&self) -> &[Lisp_Object] {
        &self.items
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [Lisp_Object] {
        &mut self.items
    }
}

impl core::ops::Deref for RootedVec {
    type Target = [Lisp_Object];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl core::ops::DerefMut for RootedVec {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

impl Default for RootedVec {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for RootedVec {
    #[inline]
    fn drop(&mut self) {
        self.span.update_ptr_and_len(self.items.as_ptr(), 0);
    }
}

impl Drop for ScopedRawSpan {
    #[inline(always)]
    fn drop(&mut self) {
        let idx = self.span_index;
        let expected_gen = self.generation;
        with_thread_slab(self.thread_key, |slab| {
            slab.raw_spans_popped += 1;
            let span = &mut slab.raw_spans[idx];
            assert!(
                span.occupied && span.generation == expected_gen,
                "ScopedRawSpan double-release or slot corruption"
            );
            span.occupied = false;
            span.ptr = 0;
            span.len = 0;
            slab.free_span_indices.push(idx);
        });
    }
}

/// Long-lived explicitly-released root (`SpiderMonkey JS::PersistentRooted<Value>`).
pub struct PersistentRoot {
    index: usize,
    generation: u64,
    released: bool,
}

impl PersistentRoot {
    /// Allocate a persistent root slot holding `initial`.
    ///
    /// # Safety
    /// `initial` must be a valid tagged `Lisp_Object` for the live VM heap (or immediate) before
    /// any intervening GC safepoint.
    pub(crate) unsafe fn new_raw(initial: LispVal) -> Self {
        let (index, generation) = with_persistent_table(|table| {
            let r#gen = table.next_generation;
            table.next_generation += 1;
            table.live_count += 1;
            if let Some(free_idx) = table.free_indices.pop() {
                table.slots[free_idx] = PersistentSlot {
                    generation: r#gen,
                    occupied: true,
                    value: initial,
                };
                (free_idx, r#gen)
            } else {
                let idx = table.slots.len();
                table.slots.push(PersistentSlot {
                    generation: r#gen,
                    occupied: true,
                    value: initial,
                });
                (idx, r#gen)
            }
        });
        PERSISTENT_CREATED.fetch_add(1, Ordering::Relaxed);
        Self {
            index,
            generation,
            released: false,
        }
    }

    #[inline]
    pub fn from_immediate(initial: Immediate) -> Self {
        // SAFETY: `Immediate` holds nil or fixnum only (never a GC heap pointer).
        unsafe { Self::new_raw(initial.into_lisp_val()) }
    }

    #[inline]
    pub fn from_handle(ctx: &GcContext, handle: Handle<'_>) -> Self {
        assert!(
            is_vm_lock_owner(ctx.thread_key),
            "PersistentRoot::from_handle requires active VM lock ownership"
        );
        let raw = handle.get_raw();
        // SAFETY: `handle` proves `raw` is live in an occupied root slot under the VM lock.
        unsafe { Self::new_raw(raw) }
    }

    #[inline]
    pub(crate) fn get_raw(&self) -> LispVal {
        if self.released {
            return LispVal::nil();
        }
        with_persistent_table(|table| {
            if let Some(slot) = table.slots.get(self.index)
                && slot.occupied
                && slot.generation == self.generation
            {
                slot.value
            } else {
                LispVal::nil()
            }
        })
    }

    /// Replace the slot word with a caller-validated raw `LispVal`.
    ///
    /// # Safety
    /// `value` must be a valid tagged `Lisp_Object` for the live VM heap under the active VM lock.
    #[inline]
    pub(crate) unsafe fn set_raw(&mut self, value: LispVal) {
        if self.released {
            return;
        }
        let idx = self.index;
        let r#gen = self.generation;
        with_persistent_table(|table| {
            if let Some(slot) = table.slots.get_mut(idx)
                && slot.occupied
                && slot.generation == r#gen
            {
                slot.value = value;
            }
        });
    }

    #[inline]
    pub fn is_nil(&self) -> bool {
        self.get_raw().is_nil()
    }

    #[inline]
    pub fn as_fixnum(&self) -> Option<isize> {
        self.get_raw().as_fixnum()
    }

    #[inline]
    pub fn as_immediate(&self) -> Option<Immediate> {
        let raw = self.get_raw().into_raw();
        if raw == lisp_h_Qnil() || FIXNUMP(raw) {
            Some(Immediate(raw))
        } else {
            None
        }
    }

    #[inline]
    pub fn set_immediate(&mut self, value: Immediate) {
        // SAFETY: `Immediate` contains nil or fixnum only (never a GC heap pointer).
        unsafe { self.set_raw(value.into_lisp_val()) };
    }

    #[inline]
    pub fn set_from_handle(&mut self, ctx: &GcContext, handle: Handle<'_>) {
        assert!(
            is_vm_lock_owner(ctx.thread_key),
            "PersistentRoot::set_from_handle requires active VM lock ownership"
        );
        let raw = handle.get_raw();
        // SAFETY: `handle` proves `raw` is live in an occupied root slot under the VM lock.
        unsafe { self.set_raw(raw) };
    }

    #[inline]
    pub fn handle(&self) -> Handle<'_> {
        Handle {
            thread_key: 0,
            slot_or_index: self.index,
            is_persistent: true,
            generation: self.generation,
            _lifetime: PhantomData,
        }
    }

    /// Deterministically release the persistent root slot so the next GC cycle can collect its
    /// referent.
    pub fn release(&mut self) -> bool {
        if self.released {
            return false;
        }
        self.released = true;
        release_persistent_raw(self.index, self.generation)
    }

    #[inline]
    pub(crate) fn raw_token(&self) -> (usize, u64) {
        (self.index, self.generation)
    }
}

impl Drop for PersistentRoot {
    fn drop(&mut self) {
        let _ = self.release();
    }
}

pub(crate) fn release_persistent_raw(index: usize, generation: u64) -> bool {
    let did_release = with_persistent_table(|table| {
        if let Some(slot) = table.slots.get_mut(index)
            && slot.occupied
            && slot.generation == generation
        {
            slot.occupied = false;
            slot.value = LispVal::nil();
            table.free_indices.push(index);
            table.live_count -= 1;
            true
        } else {
            false
        }
    });
    if did_release {
        PERSISTENT_RELEASED.fetch_add(1, Ordering::Relaxed);
    }
    did_release
}

/// Mark all U4 thread-scoped `Rooted` values and `ScopedRawSpan` arrays on `thread`.
///
/// # Safety
/// Must be called only while the current OS thread holds the VM global lock and all registered
/// `ScopedRawSpan` backing ranges for `thread` remain valid for reading.
pub(crate) unsafe fn mark_thread_u4_slab_roots(
    thread: *mut c_void,
    mut mark_fn: impl FnMut(Lisp_Object),
) {
    assert!(
        is_current_os_thread_vm_lock_owner(),
        "mark_thread_u4_slab_roots requires active VM lock ownership on current OS thread"
    );
    let key = thread as usize;
    if key == 0 {
        return;
    }
    assert!(
        !TLS_SLAB.with(|cell| cell.get().borrowed),
        "cannot mark ThreadRootSlab while slab is borrowed"
    );
    let snapshot: Vec<Lisp_Object> = {
        let guard = THREAD_ROOT_SLABS
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let Some(map) = guard.as_ref() else {
            return;
        };
        let Some(slab) = map.get(&key) else {
            return;
        };
        let extra: usize = slab
            .raw_spans
            .iter()
            .filter(|s| s.occupied)
            .map(|s| s.len)
            .sum();
        let mut objs = Vec::with_capacity(slab.slots.len() + extra);
        for slot in &slab.slots {
            if slot.occupied {
                objs.push(slot.value.into_raw());
            }
        }
        for span in &slab.raw_spans {
            if span.occupied && span.ptr != 0 && span.len != 0 {
                let base = span.ptr as *const Lisp_Object;
                for i in 0..span.len {
                    // SAFETY: `ScopedRawSpan::new` / `set_len` guarantees `[base .. base + len)`
                    // holds initialized `Lisp_Object` slots on the paused/current Lisp thread.
                    objs.push(unsafe { *base.add(i) });
                }
            }
        }
        objs
    };
    for obj in snapshot {
        mark_fn(obj);
    }
}

/// Remove per-thread U4 root slab when a Lisp thread is finalized.
///
/// # Safety
/// Must be called only while the current OS thread holds the VM global lock and no `Rooted` or
/// `ScopedRawSpan` handles referencing `thread` remain active.
pub(crate) unsafe fn cleanup_thread_u4_slab(thread: *mut c_void) {
    assert!(
        is_current_os_thread_vm_lock_owner(),
        "cleanup_thread_u4_slab requires active VM lock ownership on current OS thread"
    );
    let key = thread as usize;
    if key == 0 {
        return;
    }
    assert!(
        !TLS_SLAB.with(|cell| cell.get().borrowed),
        "cannot cleanup ThreadRootSlab while slab is borrowed"
    );
    let mut guard = THREAD_ROOT_SLABS
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    SLAB_EPOCH.fetch_add(1, Ordering::SeqCst);
    TLS_SLAB.with(|cell| {
        let mut s = cell.get();
        s.cache_epoch = 0;
        s.cache_key = 0;
        s.cache_ptr = core::ptr::null_mut();
        cell.set(s);
    });
    if let Some(map) = guard.as_mut()
        && let Some(removed) = map.remove(&key)
    {
        RETIRED_ROOTED_PUSHED.fetch_add(removed.rooted_pushed, Ordering::Relaxed);
        RETIRED_ROOTED_POPPED.fetch_add(removed.rooted_popped, Ordering::Relaxed);
        RETIRED_RAW_SPANS_PUSHED.fetch_add(removed.raw_spans_pushed, Ordering::Relaxed);
        RETIRED_RAW_SPANS_POPPED.fetch_add(removed.raw_spans_popped, Ordering::Relaxed);
    }
}

/// Mark all occupied `PersistentRoot` slots during GC (`garbage_collect_impl`).
///
/// # Safety
/// Must be called only while the current OS thread holds the VM global lock during GC marking.
pub(crate) unsafe fn mark_persistent_roots(mut mark_fn: impl FnMut(Lisp_Object)) {
    assert!(
        is_current_os_thread_vm_lock_owner(),
        "mark_persistent_roots requires active VM lock ownership on current OS thread"
    );
    let snapshot: Vec<Lisp_Object> = {
        let guard = PERSISTENT_ROOTS
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let Some(table) = guard.as_ref() else {
            return;
        };
        let mut objs = Vec::with_capacity(table.live_count);
        for slot in &table.slots {
            if slot.occupied {
                objs.push(slot.value.into_raw());
            }
        }
        objs
    };
    for obj in snapshot {
        mark_fn(obj);
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RootingCounters {
    pub rooted_pushed: usize,
    pub rooted_popped: usize,
    pub raw_spans_pushed: usize,
    pub raw_spans_popped: usize,
    pub persistent_created: usize,
    pub persistent_released: usize,
    pub persistent_live: usize,
}

pub(crate) fn rooting_counters() -> RootingCounters {
    assert!(
        is_current_os_thread_vm_lock_owner(),
        "rooting_counters requires active VM lock ownership on current OS thread"
    );
    assert!(
        !TLS_SLAB.with(|cell| cell.get().borrowed),
        "cannot read rooting_counters while ThreadRootSlab is borrowed"
    );
    let mut rooted_pushed = RETIRED_ROOTED_PUSHED.load(Ordering::Relaxed);
    let mut rooted_popped = RETIRED_ROOTED_POPPED.load(Ordering::Relaxed);
    let mut raw_spans_pushed = RETIRED_RAW_SPANS_PUSHED.load(Ordering::Relaxed);
    let mut raw_spans_popped = RETIRED_RAW_SPANS_POPPED.load(Ordering::Relaxed);
    {
        let guard = THREAD_ROOT_SLABS
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Some(map) = guard.as_ref() {
            for slab in map.values() {
                rooted_pushed += slab.rooted_pushed;
                rooted_popped += slab.rooted_popped;
                raw_spans_pushed += slab.raw_spans_pushed;
                raw_spans_popped += slab.raw_spans_popped;
            }
        }
    }
    let persistent_live = with_persistent_table(|t| t.live_count);
    RootingCounters {
        rooted_pushed,
        rooted_popped,
        raw_spans_pushed,
        raw_spans_popped,
        persistent_created: PERSISTENT_CREATED.load(Ordering::Relaxed),
        persistent_released: PERSISTENT_RELEASED.load(Ordering::Relaxed),
        persistent_live,
    }
}

/// C-ABI entrypoints used by `AU4` dynamic-module tests to verify cross-FFI `PersistentRoot`
/// registration, GC survival under `FEMACS_GC_SKIP_STACK_SCAN`, and deterministic reclamation
/// upon explicit release without polluting `obarray`.
///
/// # Safety
/// `raw_obj` must be a valid tagged `Lisp_Object`, and any non-null `out_index` / `out_gen` must
/// be valid, aligned, writable storage for `usize` / `u64` respectively.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn femacs_vm_persistent_root_alloc(
    raw_obj: Lisp_Object,
    out_index: *mut usize,
    out_gen: *mut u64,
) {
    // SAFETY: caller guarantees `raw_obj` is a valid tagged `Lisp_Object` and allocates immediately
    // before any GC safepoint.
    let val = unsafe { LispVal::from_raw(raw_obj) };
    // SAFETY: `val` was just validated as a live `Lisp_Object` for this C-ABI call.
    let mut pr = unsafe { PersistentRoot::new_raw(val) };
    let (idx, r#gen) = pr.raw_token();
    // Disarm Drop so ownership transfers to `(index, generation)` until explicit release.
    pr.released = true;
    if !out_index.is_null() {
        // SAFETY: caller provides a valid writable `usize` pointer.
        unsafe { *out_index = idx };
    }
    if !out_gen.is_null() {
        // SAFETY: caller provides a valid writable `u64` pointer.
        unsafe { *out_gen = r#gen };
    }
}

/// C-ABI probe reading the raw `Lisp_Object` stored in a live persistent root slot.
///
/// # Safety
/// Caller must run under the VM global lock and only consume the returned raw word before any
/// subsequent GC safepoint or slot release.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn femacs_vm_persistent_root_get(index: usize, generation: u64) -> Lisp_Object {
    with_persistent_table(|table| {
        if let Some(slot) = table.slots.get(index)
            && slot.occupied
            && slot.generation == generation
        {
            slot.value.into_raw()
        } else {
            lisp_h_Qnil()
        }
    })
}

/// C-ABI probe releasing the persistent root slot identified by `(index, generation)`.
///
/// # Safety
/// Caller must own the `(index, generation)` token returned by `femacs_vm_persistent_root_alloc`
/// under active VM global lock discipline.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn femacs_vm_persistent_root_release(index: usize, generation: u64) -> bool {
    release_persistent_raw(index, generation)
}

struct NarrowedStackScanGuard {
    thread_ptr: *mut crate::common::thread_state,
    saved_bottom: *const core::ffi::c_char,
    active: bool,
}

impl Drop for NarrowedStackScanGuard {
    #[inline]
    fn drop(&mut self) {
        if self.active && !self.thread_ptr.is_null() {
            // SAFETY: restores `m_stack_bottom` on the owning Lisp thread on normal return or
            // `NonLocalExit` unwind.
            unsafe {
                (*self.thread_ptr).m_stack_bottom = self.saved_bottom;
            }
            NARROWED_STACK_RESTORES.fetch_add(1, Ordering::Relaxed);
        }
    }
}

pub fn narrowed_stack_counts() -> (usize, usize) {
    (
        NARROWED_STACK_ENTRIES.load(Ordering::Relaxed),
        NARROWED_STACK_RESTORES.load(Ordering::Relaxed),
    )
}
