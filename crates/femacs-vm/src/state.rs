//! Single-owner `VmState`, `ThreadState`, and 9-category GC root inventory for `femacs-vm` (U4).
//!
//! Tracks VM lifecycle ownership, pdump load/hook state, thread registration, and per-cycle GC
//! root accounting across static/pdump roots, thread records, interpreter argument spans,
//! bytecode frames, `specpdl`/handlers, scoped/non-local roots, persistent host handles,
//! native/module tables, and bounded conservative stack/alloca ranges.

#![deny(unsafe_op_in_unsafe_fn, static_mut_refs)]

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use crate::common::{CONSP, Lisp_Object, XCDR, all_threads, current_buffer_alist, current_thread};
use crate::nonlocal::{host_drop_counts, scoped_root_counts};
use crate::roots::{RootingCounters, rooting_counters};

static GC_CYCLES_COMPLETED: AtomicUsize = AtomicUsize::new(0);
static LAST_GC_PERSISTENT_MARKED: AtomicUsize = AtomicUsize::new(0);
static LAST_GC_THREAD_SLAB_MARKED: AtomicUsize = AtomicUsize::new(0);
static LAST_GC_CONSERVATIVE_STACK_BYTES: AtomicUsize = AtomicUsize::new(0);
static LAST_GC_SA_ACTIVE_BLOCKS: AtomicUsize = AtomicUsize::new(0);
static LAST_GC_SA_FREE_SCRUBBED_BLOCKS: AtomicUsize = AtomicUsize::new(0);
static PDUMPER_LOADED_FLAG: AtomicBool = AtomicBool::new(false);
static PDUMPER_HOOK_INVOCATIONS: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ThreadStateSnapshot {
    pub active_thread_count: usize,
    pub specpdl_depth: usize,
    pub handler_depth: usize,
    pub bytecode_active: bool,
    pub stack_span_bytes: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VmRootInventory {
    pub gc_cycles_completed: usize,
    pub dump_loaded: bool,
    pub dump_early_hooks: usize,
    pub dump_late_hooks: usize,
    pub dump_hook_invocations: usize,
    pub live_buffers: usize,
    pub module_global_refs_table_live: bool,
    pub native_comp_units_table_live: bool,
    pub thread: ThreadStateSnapshot,
    pub rooting: RootingCounters,
    pub nonlocal_scoped_created: usize,
    pub nonlocal_scoped_dropped: usize,
    pub host_guards_created: usize,
    pub host_guards_dropped: usize,
    pub last_gc_persistent_marked: usize,
    pub last_gc_thread_slab_marked: usize,
    pub last_gc_conservative_stack_bytes: usize,
    pub last_gc_sa_active_blocks: usize,
    pub last_gc_sa_free_scrubbed_blocks: usize,
}

/// Zero-sized handle representing single-VM ownership on the active Lisp thread.
pub struct VmState {
    _private: (),
}

/// Per-thread view over the owning Lisp `thread_state`.
pub struct ThreadState {
    _private: (),
}

impl VmState {
    #[inline]
    pub fn current() -> Option<Self> {
        // SAFETY: `current_thread` is the single-VM active thread pointer `VmGlobal` extern slot.
        if unsafe { current_thread.get() }.is_null() {
            None
        } else {
            Some(Self { _private: () })
        }
    }

    #[inline]
    pub fn current_thread_state(&self) -> ThreadState {
        ThreadState { _private: () }
    }

    pub fn snapshot(&self) -> VmRootInventory {
        snapshot_vm_root_inventory()
    }
}

impl ThreadState {
    pub fn snapshot(&self) -> ThreadStateSnapshot {
        snapshot_current_thread()
    }
}

#[inline]
pub fn note_pdumper_loaded() {
    PDUMPER_LOADED_FLAG.store(true, Ordering::Relaxed);
}

#[inline]
pub fn note_pdumper_hook_run() {
    PDUMPER_HOOK_INVOCATIONS.fetch_add(1, Ordering::Relaxed);
}

#[inline]
pub fn begin_gc_root_cycle() {
    LAST_GC_PERSISTENT_MARKED.store(0, Ordering::Relaxed);
    LAST_GC_THREAD_SLAB_MARKED.store(0, Ordering::Relaxed);
    LAST_GC_CONSERVATIVE_STACK_BYTES.store(0, Ordering::Relaxed);
    LAST_GC_SA_ACTIVE_BLOCKS.store(0, Ordering::Relaxed);
}

#[inline]
pub fn note_gc_persistent_marked(count: usize) {
    LAST_GC_PERSISTENT_MARKED.fetch_add(count, Ordering::Relaxed);
}

#[inline]
pub fn note_gc_thread_slab_marked(count: usize) {
    LAST_GC_THREAD_SLAB_MARKED.fetch_add(count, Ordering::Relaxed);
}

#[inline]
pub fn note_gc_conservative_stack_span(bytes: usize) {
    LAST_GC_CONSERVATIVE_STACK_BYTES.fetch_add(bytes, Ordering::Relaxed);
}

#[inline]
pub fn note_gc_sa_active_blocks(blocks: usize) {
    LAST_GC_SA_ACTIVE_BLOCKS.store(blocks, Ordering::Relaxed);
}

#[inline]
pub fn note_sa_free_scrubbed(blocks: usize) {
    LAST_GC_SA_FREE_SCRUBBED_BLOCKS.fetch_add(blocks, Ordering::Relaxed);
}

#[inline]
pub fn finish_gc_root_cycle() {
    GC_CYCLES_COMPLETED.fetch_add(1, Ordering::Relaxed);
}

fn snapshot_current_thread() -> ThreadStateSnapshot {
    // SAFETY: reads active `all_threads` list and `current_thread` fields while holding the VM
    // global lock on the Lisp thread.
    unsafe {
        let mut active_thread_count = 0usize;
        let mut iter = all_threads.get();
        while !iter.is_null() {
            active_thread_count += 1;
            iter = (*iter).next_thread;
        }
        let ct = current_thread.get();
        if ct.is_null() {
            return ThreadStateSnapshot {
                active_thread_count,
                specpdl_depth: 0,
                handler_depth: 0,
                bytecode_active: false,
                stack_span_bytes: 0,
            };
        }
        let specpdl_depth = if !(*ct).m_specpdl.is_null() && !(*ct).m_specpdl_ptr.is_null() {
            ((*ct).m_specpdl_ptr.offset_from((*ct).m_specpdl)).max(0) as usize
        } else {
            0
        };
        let mut handler_depth = 0usize;
        let mut h = (*ct).m_handlerlist;
        while !h.is_null() {
            handler_depth += 1;
            h = (*h).next;
        }
        let bytecode_active =
            !(*ct).bc.fp.is_null() && ((*ct).bc.fp as *const u8) != ((*ct).bc.stack as *const u8);
        let bottom = (*ct).m_stack_bottom as usize;
        let top = (*ct).stack_top as usize;
        let stack_span_bytes = bottom.abs_diff(top);
        ThreadStateSnapshot {
            active_thread_count,
            specpdl_depth,
            handler_depth,
            bytecode_active,
            stack_span_bytes,
        }
    }
}

pub fn snapshot_vm_root_inventory() -> VmRootInventory {
    let rooting = rooting_counters();
    let (nl_c, nl_d) = scoped_root_counts();
    let (hd_c, hd_d) = host_drop_counts();
    let thread = snapshot_current_thread();

    let (pd_loaded, early_hooks, late_hooks) = crate::pdumper::u4_pdumper_counts();
    let mod_live = crate::emacs_module::u4_module_refs_table_live();
    let (live_buffers, comp_live) = {
        let mut nbufs = 0usize;
        let mut tail: Lisp_Object = current_buffer_alist();
        while CONSP(tail) {
            nbufs += 1;
            tail = XCDR(tail);
        }
        #[cfg(feature = "have_native_comp")]
        let comp_ok = crate::common::HASH_TABLE_P(crate::common::Vcomp_loaded_comp_units_h.get());
        #[cfg(not(feature = "have_native_comp"))]
        let comp_ok = false;
        (nbufs, comp_ok)
    };
    let dump_loaded = pd_loaded || PDUMPER_LOADED_FLAG.load(Ordering::Relaxed);

    VmRootInventory {
        gc_cycles_completed: GC_CYCLES_COMPLETED.load(Ordering::Relaxed),
        dump_loaded,
        dump_early_hooks: early_hooks,
        dump_late_hooks: late_hooks,
        dump_hook_invocations: PDUMPER_HOOK_INVOCATIONS.load(Ordering::Relaxed),
        live_buffers,
        module_global_refs_table_live: mod_live,
        native_comp_units_table_live: comp_live,
        thread,
        rooting,
        nonlocal_scoped_created: nl_c,
        nonlocal_scoped_dropped: nl_d,
        host_guards_created: hd_c,
        host_guards_dropped: hd_d,
        last_gc_persistent_marked: LAST_GC_PERSISTENT_MARKED.load(Ordering::Relaxed),
        last_gc_thread_slab_marked: LAST_GC_THREAD_SLAB_MARKED.load(Ordering::Relaxed),
        last_gc_conservative_stack_bytes: LAST_GC_CONSERVATIVE_STACK_BYTES.load(Ordering::Relaxed),
        last_gc_sa_active_blocks: LAST_GC_SA_ACTIVE_BLOCKS.load(Ordering::Relaxed),
        last_gc_sa_free_scrubbed_blocks: LAST_GC_SA_FREE_SCRUBBED_BLOCKS.load(Ordering::Relaxed),
    }
}

/// Exported C-ABI telemetry probe for `AU4` dynamic-module & integration verification.
/// Writes 12 `usize` metrics into `out_buf` (`len >= 12`).
#[unsafe(no_mangle)]
pub extern "C" fn femacs_vm_u4_root_stats(out_buf: *mut usize, len: usize) -> usize {
    let snap = snapshot_vm_root_inventory();
    let fields: [usize; 12] = [
        snap.gc_cycles_completed,
        usize::from(snap.dump_loaded),
        snap.dump_early_hooks + snap.dump_late_hooks,
        snap.thread.active_thread_count,
        snap.thread.specpdl_depth,
        snap.rooting.rooted_pushed,
        snap.rooting.rooted_popped,
        snap.rooting.raw_spans_pushed,
        snap.rooting.raw_spans_popped,
        snap.rooting.persistent_live,
        snap.last_gc_persistent_marked,
        snap.last_gc_sa_free_scrubbed_blocks,
    ];
    if !out_buf.is_null() && len >= fields.len() {
        for (i, val) in fields.iter().copied().enumerate() {
            // SAFETY: caller guarantees `out_buf` is writable for `len >= 12` `usize` slots.
            unsafe { *out_buf.add(i) = val };
        }
    }
    fields.len()
}
