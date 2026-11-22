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
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::common::{
    EMACS_INT, FIXNUMP, Lisp_Object, XFIXNUM, current_thread, lisp_h_Qnil, make_fixnum,
};

/// Safe copyable wrapper around a tagged `Lisp_Object` word.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LispVal(Lisp_Object);

// SAFETY: `LispVal` is a tagged word value; dereferencing heap storage requires a VM lock + root
// proof inside `vm_primitives`, never raw pointer access from `LispVal` itself.
unsafe impl Send for LispVal {}
// SAFETY: `LispVal` carries immutable word identity only.
unsafe impl Sync for LispVal {}

impl LispVal {
    #[inline]
    pub const fn nil() -> Self {
        Self(lisp_h_Qnil())
    }

    #[inline]
    pub fn from_raw(raw: Lisp_Object) -> Self {
        Self(raw)
    }

    #[inline]
    pub const fn into_raw(self) -> Lisp_Object {
        self.0
    }

    #[inline]
    pub fn is_nil(self) -> bool {
        self.0 == lisp_h_Qnil()
    }

    #[inline]
    pub fn from_fixnum(n: isize) -> Self {
        Self(make_fixnum(n as EMACS_INT))
    }

    #[inline]
    pub fn as_fixnum(self) -> Option<isize> {
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
    _marker: PhantomData<&'a ()>,
}

/// Short-lived borrow bound to `&'nogc NoGcToken` so Rust's borrow checker statically ends the
/// borrow before any `&mut GcContext` operation (`eval`, `funcall`, `garbage_collect`, `yield`).
#[derive(Clone, Copy, Debug)]
pub struct UnrootedBorrow<'nogc, T> {
    value: T,
    _nogc: PhantomData<&'nogc NoGcToken<'nogc>>,
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

/// Mutable execution context representing ownership of the active Lisp thread across GC safepoints.
pub struct GcContext {
    _private: (),
}

impl GcContext {
    /// Obtain the active thread's `GcContext` when `current_thread` is initialized.
    #[inline]
    pub fn current() -> Option<Self> {
        if current_thread_key() == 0 {
            None
        } else {
            Some(Self { _private: () })
        }
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
}

#[derive(Clone, Copy)]
struct RawRootSpan {
    ptr: usize,
    len: usize,
}

#[derive(Default)]
struct ThreadRootSlab {
    values: Vec<LispVal>,
    raw_spans: Vec<RawRootSpan>,
}

#[derive(Clone, Copy)]
struct PersistentSlot {
    generation: u64,
    occupied: bool,
    value: LispVal,
}

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

static THREAD_ROOT_SLABS: Mutex<Option<HashMap<usize, ThreadRootSlab>>> = Mutex::new(None);
static PERSISTENT_ROOTS: Mutex<Option<PersistentRootTable>> = Mutex::new(None);

static ROOTED_SLOTS_PUSHED: AtomicUsize = AtomicUsize::new(0);
static ROOTED_SLOTS_POPPED: AtomicUsize = AtomicUsize::new(0);
static RAW_SPANS_PUSHED: AtomicUsize = AtomicUsize::new(0);
static RAW_SPANS_POPPED: AtomicUsize = AtomicUsize::new(0);
static PERSISTENT_CREATED: AtomicUsize = AtomicUsize::new(0);
static PERSISTENT_RELEASED: AtomicUsize = AtomicUsize::new(0);

#[inline]
pub(crate) fn current_thread_key() -> usize {
    // SAFETY: `current_thread` is the single-VM active thread pointer `VmGlobal` extern slot.
    unsafe { current_thread.get() as usize }
}

fn with_thread_slab<R>(thread_key: usize, f: impl FnOnce(&mut ThreadRootSlab) -> R) -> R {
    let mut guard = THREAD_ROOT_SLABS
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let map = guard.get_or_insert_with(HashMap::new);
    let slab = map.entry(thread_key).or_insert_with(|| ThreadRootSlab {
        values: Vec::with_capacity(32),
        raw_spans: Vec::with_capacity(16),
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
    fallback: LispVal,
    _lifetime: PhantomData<&'a LispVal>,
}

impl<'a> Handle<'a> {
    #[inline]
    pub fn get(self) -> LispVal {
        if self.is_persistent {
            with_persistent_table(|table| {
                if let Some(slot) = table.slots.get(self.slot_or_index)
                    && slot.occupied
                    && slot.generation == self.generation
                {
                    slot.value
                } else {
                    LispVal::nil()
                }
            })
        } else if self.thread_key == 0 {
            self.fallback
        } else {
            with_thread_slab(self.thread_key, |slab| {
                slab.values
                    .get(self.slot_or_index)
                    .copied()
                    .unwrap_or(self.fallback)
            })
        }
    }
}

/// Mutable GC-safe handle (`SpiderMonkey JS::MutableHandle<Value>`) referencing a `Rooted` slot.
pub struct MutableHandle<'a> {
    rooted: &'a mut Rooted,
}

impl<'a> MutableHandle<'a> {
    #[inline]
    pub fn get(&self) -> LispVal {
        self.rooted.get()
    }

    #[inline]
    pub fn set(&mut self, value: LispVal) {
        self.rooted.set(value);
    }

    #[inline]
    pub fn handle(&self) -> Handle<'_> {
        self.rooted.handle()
    }
}

/// Call-scope LIFO GC root (`SpiderMonkey JS::Rooted<Value>`).
pub struct Rooted {
    thread_key: usize,
    index: usize,
    local_shadow: LispVal,
}

impl Rooted {
    #[inline]
    pub fn new(initial: LispVal) -> Self {
        let thread_key = current_thread_key();
        let index = if thread_key == 0 {
            0
        } else {
            with_thread_slab(thread_key, |slab| {
                let idx = slab.values.len();
                slab.values.push(initial);
                idx
            })
        };
        ROOTED_SLOTS_PUSHED.fetch_add(1, Ordering::Relaxed);
        Self {
            thread_key,
            index,
            local_shadow: initial,
        }
    }

    #[inline]
    pub fn get(&self) -> LispVal {
        if self.thread_key == 0 {
            self.local_shadow
        } else {
            with_thread_slab(self.thread_key, |slab| slab.values[self.index])
        }
    }

    #[inline]
    pub fn set(&mut self, value: LispVal) {
        self.local_shadow = value;
        if self.thread_key != 0 {
            let idx = self.index;
            with_thread_slab(self.thread_key, |slab| {
                slab.values[idx] = value;
            });
        }
    }

    #[inline]
    pub fn handle(&self) -> Handle<'_> {
        Handle {
            thread_key: self.thread_key,
            slot_or_index: self.index,
            is_persistent: false,
            generation: 0,
            fallback: self.local_shadow,
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
        if self.thread_key != 0 {
            with_thread_slab(self.thread_key, |slab| {
                debug_assert_eq!(slab.values.len(), self.index + 1);
                slab.values.truncate(self.index);
            });
        }
        ROOTED_SLOTS_POPPED.fetch_add(1, Ordering::Relaxed);
    }
}

/// LIFO registration for an active stack/alloca array of `Lisp_Object` slots during evaluation
/// (`eval_sub`, `apply_lambda`, `Fprog1`, `Flet`), allowing progressive updates of active length
/// as arguments are evaluated left-to-right.
pub struct ScopedRawSpan {
    thread_key: usize,
    span_index: usize,
}

impl ScopedRawSpan {
    /// Register `[ptr .. ptr + initial_len)` as an active per-thread GC root span.
    ///
    /// # Safety
    /// `ptr` must remain valid for reading at least `active_len` initialized `Lisp_Object` words
    /// on the owning Lisp thread until this `ScopedRawSpan` is dropped.
    #[inline]
    pub unsafe fn new(ptr: *const Lisp_Object, initial_len: usize) -> Self {
        let thread_key = current_thread_key();
        let span_index = if thread_key == 0 || ptr.is_null() {
            usize::MAX
        } else {
            with_thread_slab(thread_key, |slab| {
                let idx = slab.raw_spans.len();
                slab.raw_spans.push(RawRootSpan {
                    ptr: ptr as usize,
                    len: initial_len,
                });
                idx
            })
        };
        RAW_SPANS_PUSHED.fetch_add(1, Ordering::Relaxed);
        Self {
            thread_key,
            span_index,
        }
    }

    /// Update the number of initialized `Lisp_Object` elements in this span.
    ///
    /// # Safety
    /// At least `new_len` elements starting at the registered `ptr` must be initialized.
    #[inline]
    pub unsafe fn set_len(&mut self, new_len: usize) {
        if self.span_index != usize::MAX && self.thread_key != 0 {
            let idx = self.span_index;
            with_thread_slab(self.thread_key, |slab| {
                if let Some(span) = slab.raw_spans.get_mut(idx) {
                    span.len = new_len;
                }
            });
        }
    }
}

impl Drop for ScopedRawSpan {
    #[inline]
    fn drop(&mut self) {
        if self.span_index != usize::MAX && self.thread_key != 0 {
            with_thread_slab(self.thread_key, |slab| {
                debug_assert_eq!(slab.raw_spans.len(), self.span_index + 1);
                slab.raw_spans.truncate(self.span_index);
            });
        }
        RAW_SPANS_POPPED.fetch_add(1, Ordering::Relaxed);
    }
}

/// Long-lived explicitly-released root (`SpiderMonkey JS::PersistentRooted<Value>`).
pub struct PersistentRoot {
    index: usize,
    generation: u64,
    released: bool,
}

impl PersistentRoot {
    pub fn new(initial: LispVal) -> Self {
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
    pub fn get(&self) -> LispVal {
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

    #[inline]
    pub fn set(&mut self, value: LispVal) {
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
    pub fn handle(&self) -> Handle<'_> {
        Handle {
            thread_key: 0,
            slot_or_index: self.index,
            is_persistent: true,
            generation: self.generation,
            fallback: LispVal::nil(),
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
    pub fn raw_token(&self) -> (usize, u64) {
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
pub fn mark_thread_u4_slab_roots(thread: *mut c_void, mut mark_fn: impl FnMut(Lisp_Object)) {
    let key = thread as usize;
    if key == 0 {
        return;
    }
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
        let extra: usize = slab.raw_spans.iter().map(|s| s.len).sum();
        let mut objs = Vec::with_capacity(slab.values.len() + extra);
        for v in &slab.values {
            objs.push(v.into_raw());
        }
        for span in &slab.raw_spans {
            if span.ptr != 0 && span.len != 0 {
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
pub fn cleanup_thread_u4_slab(thread: *mut c_void) {
    let key = thread as usize;
    if key == 0 {
        return;
    }
    let mut guard = THREAD_ROOT_SLABS
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(map) = guard.as_mut() {
        map.remove(&key);
    }
}

/// Mark all occupied `PersistentRoot` slots during GC (`garbage_collect_impl`).
pub fn mark_persistent_roots(mut mark_fn: impl FnMut(Lisp_Object)) {
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

pub fn rooting_counters() -> RootingCounters {
    let persistent_live = with_persistent_table(|t| t.live_count);
    RootingCounters {
        rooted_pushed: ROOTED_SLOTS_PUSHED.load(Ordering::Relaxed),
        rooted_popped: ROOTED_SLOTS_POPPED.load(Ordering::Relaxed),
        raw_spans_pushed: RAW_SPANS_PUSHED.load(Ordering::Relaxed),
        raw_spans_popped: RAW_SPANS_POPPED.load(Ordering::Relaxed),
        persistent_created: PERSISTENT_CREATED.load(Ordering::Relaxed),
        persistent_released: PERSISTENT_RELEASED.load(Ordering::Relaxed),
        persistent_live,
    }
}

/// C-ABI entrypoints used by `AU4` dynamic-module tests to verify cross-FFI `PersistentRoot`
/// registration, GC survival under `FEMACS_GC_SKIP_STACK_SCAN`, and deterministic reclamation
/// upon explicit release without polluting `obarray`.
#[unsafe(no_mangle)]
pub extern "C" fn femacs_vm_persistent_root_alloc(
    raw_obj: Lisp_Object,
    out_index: *mut usize,
    out_gen: *mut u64,
) {
    let mut pr = PersistentRoot::new(LispVal::from_raw(raw_obj));
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

#[unsafe(no_mangle)]
pub extern "C" fn femacs_vm_persistent_root_get(index: usize, generation: u64) -> Lisp_Object {
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

#[unsafe(no_mangle)]
pub extern "C" fn femacs_vm_persistent_root_release(index: usize, generation: u64) -> bool {
    release_persistent_raw(index, generation)
}
