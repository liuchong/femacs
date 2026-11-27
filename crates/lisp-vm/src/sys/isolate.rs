//! Per-isolate VM global state table (`isolate_vm` feature, Phase 2).
//!
//! When `feature = "isolate_vm"` is enabled, each VM instance owns an isolated
//! heap-backed copy of every touched `VmGlobal<T>` initialized from its static
//! `.data`/`.bss` template, enabling lock-free multi-core execution (`NoGil`)
//! without changing `#[repr(transparent)]` static layouts for editor builds.

#![deny(unsafe_op_in_unsafe_fn, static_mut_refs)]

use core::cell::Cell;
use core::ptr::{self, NonNull};
use std::alloc::{Layout, alloc, dealloc, handle_alloc_error};

use crate::sys::VmGlobal;

#[derive(Clone, Copy)]
struct GlobalSlot {
    key: usize,
    ptr: *mut u8,
    size: usize,
}

impl Default for GlobalSlot {
    #[inline(always)]
    fn default() -> Self {
        Self {
            key: 0,
            ptr: ptr::null_mut(),
            size: 0,
        }
    }
}

#[derive(Clone, Copy)]
struct IntervalEntry {
    static_start: usize,
    static_end: usize,
    isolate_start: usize,
}

#[derive(Clone, Copy)]
struct ReverseIntervalEntry {
    isolate_start: usize,
    isolate_end: usize,
    static_start: usize,
}

struct OwnedAlloc {
    ptr: NonNull<u8>,
    layout: Layout,
}

/// Owned per-isolate heap copies of `VmGlobal<T>` storage and bidirectional
/// static <-> isolate address translation tables.
pub struct IsolateContext {
    slots: Vec<GlobalSlot>,
    slot_count: usize,
    intervals: Vec<IntervalEntry>,
    intervals_sorted: bool,
    reverse_intervals: Vec<ReverseIntervalEntry>,
    reverse_sorted: bool,
    owned_allocs: Vec<OwnedAlloc>,
    pub(crate) roots: crate::roots::IsolateRootsState,
    pub(crate) nonlocal: crate::nonlocal::IsolateNonLocalState,
    pub(crate) vm_state: crate::state::IsolateVmState,
    pub(crate) host_flags: crate::host_runtime::IsolateHostFlags,
}

// SAFETY: Every allocation in `owned_allocs` and every slot pointer belongs exclusively to
// this `IsolateContext` (initialized by copying immutable `.data`/`.bss` static templates).
// An `IsolateContext` is only entered on one OS thread at a time via `IsolateScopeGuard`.
unsafe impl Send for IsolateContext {}

std::thread_local! {
    static CURRENT_ISOLATE: Cell<*mut IsolateContext> = const { Cell::new(ptr::null_mut()) };
}

/// RAII guard binding `CURRENT_ISOLATE` on the current OS thread and restoring the
/// previous isolate pointer on drop. Intentionally `!Send`.
pub struct IsolateScopeGuard {
    prev: *mut IsolateContext,
    current: *mut IsolateContext,
}

impl IsolateScopeGuard {
    /// Enter `ctx` on the current thread.
    ///
    /// # Safety
    /// `ctx` must point to a live, uniquely accessed `IsolateContext` for the full
    /// lifetime of the returned guard (no concurrent mutable access on another thread).
    #[inline]
    pub unsafe fn enter(ctx: *mut IsolateContext) -> Self {
        let prev = CURRENT_ISOLATE.with(|cell| {
            let old = cell.get();
            cell.set(ctx);
            old
        });
        Self { prev, current: ctx }
    }
}

impl Drop for IsolateScopeGuard {
    #[inline]
    fn drop(&mut self) {
        let _ = self.current;
        CURRENT_ISOLATE.with(|cell| cell.set(self.prev));
    }
}

impl IsolateContext {
    /// Allocate a fresh per-isolate context with an 8192-slot open-addressed table.
    pub fn new() -> Box<Self> {
        Box::new(Self {
            slots: vec![GlobalSlot::default(); 8192],
            slot_count: 0,
            intervals: Vec::with_capacity(2048),
            intervals_sorted: true,
            reverse_intervals: Vec::with_capacity(2048),
            reverse_sorted: true,
            owned_allocs: Vec::with_capacity(2048),
            roots: crate::roots::IsolateRootsState::default(),
            nonlocal: crate::nonlocal::IsolateNonLocalState::default(),
            vm_state: crate::state::IsolateVmState::default(),
            host_flags: crate::host_runtime::IsolateHostFlags::default(),
        })
    }

    #[inline(always)]
    fn hash_key(key: usize, mask: usize) -> usize {
        let mut h = (key >> 3).wrapping_mul(0x9e37_79b9_7f4a_7c15);
        h ^= h >> 32;
        h & mask
    }

    fn grow_slots(&mut self) {
        let new_cap = (self.slots.len() * 2).max(8192);
        let mask = new_cap - 1;
        let mut new_slots = vec![GlobalSlot::default(); new_cap];
        for slot in &self.slots {
            if slot.key == 0 {
                continue;
            }
            let mut idx = Self::hash_key(slot.key, mask);
            while new_slots[idx].key != 0 {
                idx = (idx + 1) & mask;
            }
            new_slots[idx] = *slot;
        }
        self.slots = new_slots;
    }

    #[inline]
    fn find_exact(&self, key: usize) -> Option<(*mut u8, usize)> {
        if key == 0 || self.slots.is_empty() {
            return None;
        }
        let mask = self.slots.len() - 1;
        let mut idx = Self::hash_key(key, mask);
        loop {
            let slot = self.slots[idx];
            if slot.key == key {
                return Some((slot.ptr, slot.size));
            }
            if slot.key == 0 {
                return None;
            }
            idx = (idx + 1) & mask;
        }
    }

    fn ensure_intervals_sorted(&mut self) {
        if !self.intervals_sorted {
            self.intervals.sort_unstable_by_key(|e| e.static_start);
            self.intervals_sorted = true;
        }
    }

    fn ensure_reverse_sorted(&mut self) {
        if !self.reverse_sorted {
            self.reverse_intervals.sort_unstable_by_key(|e| e.isolate_start);
            self.reverse_sorted = true;
        }
    }

    fn find_interval_entry(&mut self, addr: usize) -> Option<IntervalEntry> {
        self.ensure_intervals_sorted();
        let idx = self.intervals.partition_point(|e| e.static_start <= addr);
        if idx == 0 {
            return None;
        }
        let entry = self.intervals[idx - 1];
        if addr >= entry.static_start && addr < entry.static_end {
            Some(entry)
        } else {
            None
        }
    }

    fn find_interval(&mut self, addr: usize) -> Option<*mut u8> {
        let entry = self.find_interval_entry(addr)?;
        let offset = addr - entry.static_start;
        Some((entry.isolate_start + offset) as *mut u8)
    }

    fn insert_slot(&mut self, key: usize, ptr: *mut u8, size: usize) {
        if (self.slot_count + 1) * 10 >= self.slots.len() * 7 {
            self.grow_slots();
        }
        let mask = self.slots.len() - 1;
        let mut idx = Self::hash_key(key, mask);
        while self.slots[idx].key != 0 {
            idx = (idx + 1) & mask;
        }
        self.slots[idx] = GlobalSlot { key, ptr, size };
        self.slot_count += 1;
    }

    /// Return this isolate's copy of `[static_ptr, static_ptr + size)`, allocating and
    /// cloning the static template bytes on first access.
    #[inline]
    pub fn get_or_alloc(&mut self, static_ptr: *const u8, size: usize, align: usize) -> *mut u8 {
        let key = static_ptr as usize;
        debug_assert!(key != 0);
        if let Some((ptr, existing_size)) = self.find_exact(key) {
            assert!(
                existing_size >= size,
                "isolate_vm: undersized pre-allocation at {key:#x}: existing={existing_size} < requested={size}"
            );
            return ptr;
        }
        self.alloc_and_insert(static_ptr, size, align)
    }

    #[cold]
    #[inline(never)]
    fn alloc_and_insert(&mut self, static_ptr: *const u8, size: usize, align: usize) -> *mut u8 {
        let key = static_ptr as usize;
        if let Some((ptr, existing_size)) = self.find_exact(key) {
            assert!(
                existing_size >= size,
                "isolate_vm: undersized pre-allocation at {key:#x}: existing={existing_size} < requested={size}"
            );
            return ptr;
        }
        if size > 0 {
            let req_end = key
                .checked_add(size)
                .expect("isolate_vm: static address overflow");
            if let Some(entry) = self.find_interval_entry(key) {
                assert!(
                    req_end <= entry.static_end,
                    "isolate_vm: partial interval overlap at {key:#x}..{req_end:#x} against {:#x}..{:#x}",
                    entry.static_start,
                    entry.static_end
                );
                let sub_ptr = (entry.isolate_start + (key - entry.static_start)) as *mut u8;
                self.insert_slot(key, sub_ptr, size);
                return sub_ptr;
            }
            self.ensure_intervals_sorted();
            let next_idx = self.intervals.partition_point(|e| e.static_start <= key);
            if next_idx < self.intervals.len() && self.intervals[next_idx].static_start < req_end {
                let next = self.intervals[next_idx];
                panic!(
                    "isolate_vm: interval overlap allocating {key:#x}..{req_end:#x} against {:#x}..{:#x}",
                    next.static_start, next.static_end
                );
            }
        }
        let effective_ptr: *mut u8 = if size == 0 {
            NonNull::<u8>::dangling().as_ptr()
        } else {
            let layout = Layout::from_size_align(size, align.max(1))
                .expect("invalid VmGlobal layout for isolate allocation");
            // SAFETY: `layout.size() > 0` and `layout.align()` is valid and power-of-two.
            let raw = unsafe { alloc(layout) };
            let Some(non_null) = NonNull::new(raw) else {
                handle_alloc_error(layout);
            };
            // SAFETY: `static_ptr` is a `'static` `VmGlobal<T>` template of `size` readable bytes
            // and `raw` is a freshly allocated, non-overlapping buffer of `size` writable bytes.
            unsafe {
                ptr::copy_nonoverlapping(static_ptr, non_null.as_ptr(), size);
            }
            self.owned_allocs.push(OwnedAlloc {
                ptr: non_null,
                layout,
            });
            self.intervals.push(IntervalEntry {
                static_start: key,
                static_end: key.saturating_add(size),
                isolate_start: non_null.as_ptr() as usize,
            });
            self.intervals_sorted = false;
            self.reverse_intervals.push(ReverseIntervalEntry {
                isolate_start: non_null.as_ptr() as usize,
                isolate_end: (non_null.as_ptr() as usize).saturating_add(size),
                static_start: key,
            });
            self.reverse_sorted = false;
            non_null.as_ptr()
        };

        self.insert_slot(key, effective_ptr, size);
        effective_ptr
    }

    /// Pre-register a static `VmGlobal<T>` in this isolate.
    #[inline]
    pub fn pre_register<T>(&mut self, global: &'static VmGlobal<T>) -> *mut T {
        self.get_or_alloc(
            global.as_static_ptr() as *const u8,
            core::mem::size_of::<T>(),
            core::mem::align_of::<T>(),
        ) as *mut T
    }

    /// Translate any exact or interior static `VmGlobal` pointer into this isolate's copy;
    /// returns `ptr` unchanged if it does not refer to a known static global.
    #[inline]
    pub fn resolve_ptr(&mut self, ptr: *const u8) -> *mut u8 {
        if ptr.is_null() {
            return ptr::null_mut();
        }
        let addr = ptr as usize;
        if let Some((exact, _)) = self.find_exact(addr) {
            return exact;
        }
        if let Some(interior) = self.find_interval(addr) {
            return interior;
        }
        ptr as *mut u8
    }

    /// Like `resolve_ptr`, also allocating a slot of `len.max(1)` if `.pdmp` relocation
    /// targets a standalone static `VmGlobal` not yet accessed in this isolate.
    #[inline]
    pub fn resolve_pdump_dst(&mut self, ptr: *mut u8, len: usize) -> *mut u8 {
        if ptr.is_null() {
            return ptr::null_mut();
        }
        let addr = ptr as usize;
        if let Some((exact, existing_size)) = self.find_exact(addr) {
            assert!(
                existing_size >= len,
                "isolate_vm: undersized pdump slot at {addr:#x}: existing={existing_size} < requested={len}"
            );
            return exact;
        }
        if let Some(entry) = self.find_interval_entry(addr) {
            let req_end = addr
                .checked_add(len)
                .expect("isolate_vm: static address overflow in resolve_pdump_dst");
            assert!(
                req_end <= entry.static_end,
                "isolate_vm: pdump partial overlap at {addr:#x}..{req_end:#x} against {:#x}..{:#x}",
                entry.static_start,
                entry.static_end
            );
            return (entry.isolate_start + (addr - entry.static_start)) as *mut u8;
        }
        if len > 0 {
            self.alloc_and_insert(ptr as *const u8, len, len.min(16).next_power_of_two())
        } else {
            ptr
        }
    }

    fn find_reverse_entry(&mut self, addr: usize) -> Option<ReverseIntervalEntry> {
        self.ensure_reverse_sorted();
        let idx = self
            .reverse_intervals
            .partition_point(|e| e.isolate_start <= addr);
        if idx == 0 {
            return None;
        }
        let entry = self.reverse_intervals[idx - 1];
        if addr >= entry.isolate_start && addr < entry.isolate_end {
            Some(entry)
        } else {
            None
        }
    }

    /// Translate an isolate-backed `VmGlobal` pointer back to its static template address.
    #[inline]
    pub fn isolate_to_static(&mut self, ptr: *const u8) -> *const u8 {
        if ptr.is_null() {
            return ptr::null();
        }
        let addr = ptr as usize;
        if let Some(entry) = self.find_reverse_entry(addr) {
            let offset = addr - entry.isolate_start;
            (entry.static_start + offset) as *const u8
        } else {
            ptr
        }
    }

    fn is_isolate_ptr(&mut self, addr: usize) -> bool {
        self.find_reverse_entry(addr).is_some()
    }

    /// Idempotently resolve a `DEFVAR_*` forwarding or `defsubr` pointer into this isolate:
    /// returns already-isolated pointers unchanged, resolves exact/interior static slots,
    /// and allocates an isolate copy for untouched standalone `VmGlobal<T>` pointers.
    #[inline]
    pub fn resolve_fwd_ptr(&mut self, ptr: *mut u8, size: usize, align: usize) -> *mut u8 {
        if ptr.is_null() {
            return ptr;
        }
        let addr = ptr as usize;
        if let Some((exact, existing_size)) = self.find_exact(addr) {
            assert!(
                existing_size >= size,
                "isolate_vm: undersized forwarding slot at {addr:#x}: existing={existing_size} < requested={size}"
            );
            return exact;
        }
        self.resolve_fwd_ptr_cold(ptr, size, align)
    }

    #[cold]
    #[inline(never)]
    fn resolve_fwd_ptr_cold(&mut self, ptr: *mut u8, size: usize, align: usize) -> *mut u8 {
        if self.is_isolate_ptr(ptr as usize) {
            return ptr;
        }
        self.alloc_and_insert(ptr as *const u8, size, align)
    }
}

impl Default for IsolateContext {
    fn default() -> Self {
        *Self::new()
    }
}

impl Drop for IsolateContext {
    fn drop(&mut self) {
        let self_ptr = self as *mut IsolateContext;
        CURRENT_ISOLATE.with(|cell| {
            if cell.get() == self_ptr {
                cell.set(ptr::null_mut());
            }
        });
        for alloc_rec in self.owned_allocs.drain(..) {
            // SAFETY: Each `OwnedAlloc` was allocated via `std::alloc::alloc(layout)` with
            // `layout.size() > 0` and has not been freed yet.
            unsafe {
                dealloc(alloc_rec.ptr.as_ptr(), alloc_rec.layout);
            }
        }
    }
}

#[inline(always)]
pub(crate) fn current_isolate_ptr() -> *mut IsolateContext {
    CURRENT_ISOLATE.with(|cell| cell.get())
}

#[inline(always)]
pub fn is_isolate_active() -> bool {
    !current_isolate_ptr().is_null()
}

#[inline]
pub fn with_current_isolate<R>(f: impl FnOnce(&mut IsolateContext) -> R) -> Option<R> {
    let ctx = current_isolate_ptr();
    if ctx.is_null() {
        None
    } else {
        // SAFETY: `ctx` was bound by a live `IsolateScopeGuard` on this thread.
        Some(unsafe { f(&mut *ctx) })
    }
}

#[inline]
pub(crate) fn get_or_alloc_for_current_isolate(
    static_ptr: *const u8,
    size: usize,
    align: usize,
) -> *mut u8 {
    let ctx = current_isolate_ptr();
    if ctx.is_null() {
        return static_ptr as *mut u8;
    }
    // SAFETY: `ctx` points to this thread's active `IsolateContext`.
    unsafe { (*ctx).get_or_alloc(static_ptr, size, align) }
}

/// Pre-register a static `VmGlobal<T>` on the current isolate (no-op when no isolate is active).
#[inline(always)]
pub fn pre_register_global<T>(global: &'static VmGlobal<T>) {
    let ctx = current_isolate_ptr();
    if !ctx.is_null() {
        // SAFETY: `ctx` points to this thread's active `IsolateContext`.
        unsafe {
            (*ctx).pre_register(global);
        }
    }
}

#[inline(always)]
pub fn resolve_global_ptr(ptr: *const u8) -> *mut u8 {
    let ctx = current_isolate_ptr();
    if ctx.is_null() {
        ptr as *mut u8
    } else {
        // SAFETY: `ctx` points to this thread's active `IsolateContext`.
        unsafe { (*ctx).resolve_ptr(ptr) }
    }
}

#[inline(always)]
pub fn resolve_fwd_ptr<T>(ptr: *mut T) -> *mut T {
    if ptr.is_null() {
        return ptr;
    }
    let ctx = current_isolate_ptr();
    if ctx.is_null() {
        ptr
    } else {
        // SAFETY: `ctx` points to this thread's active `IsolateContext`.
        unsafe {
            (*ctx).resolve_fwd_ptr(
                ptr as *mut u8,
                core::mem::size_of::<T>(),
                core::mem::align_of::<T>(),
            ) as *mut T
        }
    }
}

#[inline(always)]
pub fn resolve_pdump_dst(ptr: *mut u8, len: usize) -> *mut u8 {
    let ctx = current_isolate_ptr();
    if ctx.is_null() {
        ptr
    } else {
        // SAFETY: `ctx` points to this thread's active `IsolateContext`.
        unsafe { (*ctx).resolve_pdump_dst(ptr, len) }
    }
}

#[inline(always)]
pub fn resolve_if_known_global(ptr: *const u8) -> *mut u8 {
    resolve_global_ptr(ptr)
}

#[inline(always)]
pub fn isolate_to_static(ptr: *const u8) -> *const u8 {
    let ctx = current_isolate_ptr();
    if ctx.is_null() {
        ptr
    } else {
        // SAFETY: `ctx` points to this thread's active `IsolateContext`.
        unsafe { (*ctx).isolate_to_static(ptr) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct CompositeGlobal {
        head: i64,
        tail: u64,
    }

    static TEST_SCALAR: VmGlobal<i64> = VmGlobal::new(11);
    static TEST_COMPOSITE: VmGlobal<CompositeGlobal> =
        VmGlobal::new(CompositeGlobal { head: 7, tail: 99 });

    #[test]
    fn isolates_keep_independent_global_copies_and_interior_pointers() {
        let mut iso_a = IsolateContext::new();
        let mut iso_b = IsolateContext::new();

        {
            // SAFETY: `iso_a` is uniquely borrowed in this test scope.
            let _guard = unsafe { IsolateScopeGuard::enter(&raw mut *iso_a) };
            assert!(is_isolate_active());
            assert_eq!(TEST_SCALAR.get(), 11);
            TEST_SCALAR.set(101);
            assert_eq!(TEST_SCALAR.get(), 101);

            pre_register_global(&TEST_COMPOSITE);
            // SAFETY: Computing interior static field address without dereferencing.
            let static_tail = unsafe { ptr::addr_of!((*TEST_COMPOSITE.as_static_ptr()).tail) as *const u8 };
            let iso_tail = resolve_global_ptr(static_tail) as *mut u64;
            assert_ne!(iso_tail as usize, static_tail as usize);
            // SAFETY: `iso_tail` points into `iso_a`'s cloned `CompositeGlobal`.
            unsafe {
                assert_eq!(*iso_tail, 99);
                *iso_tail = 777;
            }
            assert_eq!(isolate_to_static(iso_tail as *const u8), static_tail);
        }

        assert!(!is_isolate_active());
        assert_eq!(TEST_SCALAR.get(), 11);

        {
            // SAFETY: `iso_b` is uniquely borrowed in this test scope.
            let _guard = unsafe { IsolateScopeGuard::enter(&raw mut *iso_b) };
            assert_eq!(TEST_SCALAR.get(), 11);
            TEST_SCALAR.set(202);
            // SAFETY: `TEST_COMPOSITE.as_ptr()` points to `iso_b`'s cloned `CompositeGlobal`.
            unsafe {
                assert_eq!((*TEST_COMPOSITE.as_ptr()).tail, 99);
            }
        }

        // SAFETY: `iso_a` still holds its own mutated values until dropped.
        unsafe {
            let _guard = IsolateScopeGuard::enter(&raw mut *iso_a);
            assert_eq!(TEST_SCALAR.get(), 101);
            assert_eq!((*TEST_COMPOSITE.as_ptr()).tail, 777);
        }
    }

    #[test]
    fn parallel_threads_with_distinct_isolates_do_not_collide() {
        let h1 = std::thread::spawn(|| {
            let mut iso = IsolateContext::new();
            // SAFETY: `iso` belongs exclusively to this thread.
            let _guard = unsafe { IsolateScopeGuard::enter(&raw mut *iso) };
            for i in 0..5000_i64 {
                TEST_SCALAR.set(i * 2 + 1);
                assert_eq!(TEST_SCALAR.get(), i * 2 + 1);
            }
            TEST_SCALAR.get()
        });
        let h2 = std::thread::spawn(|| {
            let mut iso = IsolateContext::new();
            // SAFETY: `iso` belongs exclusively to this thread.
            let _guard = unsafe { IsolateScopeGuard::enter(&raw mut *iso) };
            for i in 0..5000_i64 {
                TEST_SCALAR.set(i * 3 + 2);
                assert_eq!(TEST_SCALAR.get(), i * 3 + 2);
            }
            TEST_SCALAR.get()
        });
        assert_eq!(h1.join().unwrap(), 4999 * 2 + 1);
        assert_eq!(h2.join().unwrap(), 4999 * 3 + 2);
        assert_eq!(TEST_SCALAR.get(), 11);
    }

    #[test]
    fn subrange_returns_interior_and_undersized_slot_panics() {
        static SUB_TEST: VmGlobal<CompositeGlobal> = VmGlobal::new(CompositeGlobal {
            head: 10,
            tail: 20,
        });
        static UNDER_TEST: VmGlobal<CompositeGlobal> = VmGlobal::new(CompositeGlobal {
            head: 30,
            tail: 40,
        });

        let mut iso = IsolateContext::new();
        // Pre-registering full CompositeGlobal allows subsequent sub-range requests cleanly.
        let full = iso.pre_register(&SUB_TEST);
        // SAFETY: Computing interior static field address without dereferencing.
        let tail_static = unsafe { ptr::addr_of!((*SUB_TEST.as_static_ptr()).tail) as *const u8 };
        let tail_iso = iso.get_or_alloc(tail_static, core::mem::size_of::<u64>(), 8);
        assert_eq!(
            tail_iso as usize,
            // SAFETY: `full` is a live pre-registered isolate `CompositeGlobal` allocation.
            unsafe { ptr::addr_of!((*full).tail) as usize }
        );

        // Simulating an undersized pdump slot at offset 0 of UNDER_TEST must panic loudly.
        let under_static = UNDER_TEST.as_static_ptr() as *mut u8;
        let _ = iso.resolve_pdump_dst(under_static, core::mem::size_of::<u64>());
        let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = iso.pre_register(&UNDER_TEST);
        }));
        assert!(caught.is_err(), "expected undersized slot panic");
    }

    #[test]
    fn resolve_fwd_ptr_allocates_untouched_scalar_and_resolves_composite_field() {
        static UNTOUCHED_FWD: VmGlobal<i64> = VmGlobal::new(42);
        static COMPOSITE_FWD: VmGlobal<CompositeGlobal> = VmGlobal::new(CompositeGlobal {
            head: 11,
            tail: 22,
        });

        let mut iso = IsolateContext::new();
        // SAFETY: `iso` is uniquely borrowed in this test scope.
        let _guard = unsafe { IsolateScopeGuard::enter(&raw mut *iso) };

        pre_register_global(&COMPOSITE_FWD);

        let scalar_iso = resolve_fwd_ptr(UNTOUCHED_FWD.as_static_mut_ptr());
        assert_ne!(
            scalar_iso as usize,
            UNTOUCHED_FWD.as_static_mut_ptr() as usize
        );
        // SAFETY: `scalar_iso` points into this isolate's cloned `i64` slot.
        unsafe {
            assert_eq!(*scalar_iso, 42);
            *scalar_iso = 99;
        }
        assert_eq!(UNTOUCHED_FWD.get(), 99);
        let scalar_iso_idempotent = resolve_fwd_ptr(UNTOUCHED_FWD.as_mut_ptr());
        assert_eq!(scalar_iso_idempotent, scalar_iso);
        assert_eq!(
            isolate_to_static(scalar_iso_idempotent as *const u8),
            UNTOUCHED_FWD.as_static_ptr() as *const u8
        );

        // SAFETY: Computing interior static field address without dereferencing.
        let tail_static = unsafe { ptr::addr_of_mut!((*COMPOSITE_FWD.as_static_mut_ptr()).tail) };
        let tail_iso = resolve_fwd_ptr(tail_static);
        assert_ne!(tail_iso as usize, tail_static as usize);
        // SAFETY: `tail_iso` points to the interior field inside `COMPOSITE_FWD`'s isolate copy.
        unsafe {
            assert_eq!(*tail_iso, 22);
            *tail_iso = 88;
            assert_eq!((*COMPOSITE_FWD.as_ptr()).tail, 88);
        }
    }
}
