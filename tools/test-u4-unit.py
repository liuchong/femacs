#!/usr/bin/env python3
"""U4 protective unit tests for `crates/femacs-vm/src/roots.rs`, `stdcountof.rs`, and U4 gates.

Compiles `crates/femacs-vm/src/roots.rs` and `lib/include/stdcountof.rs` verbatim in isolation
(`rustc --test` under `#![deny(static_mut_refs, unsafe_op_in_unsafe_fn)]`) to test:
  1. `Rooted` / `Handle` / `MutableHandle` / `NoGcToken` / `UnrootedBorrow` LIFO order, unwind
     truncation across `panic!`, and per-thread slab cleanup (`cleanup_thread_u4_slab`).
  2. `ScopedRawSpan` progressive `set_len` marking `[0..len)`, nested LIFO pop, and unwind safety.
  3. `PersistentRoot` + C-ABI `femacs_vm_persistent_root_{alloc,get,release}` free-list slot reuse
     and ABA generation rejection when a freed slot is reallocated to a new object.
  4. `LispVal` fixnum tag roundtrip (`from_fixnum` / `as_fixnum` / `nil`).
  5. `stdcountof.rs` `#![forbid(unsafe_code)]` raw-pointer helpers (`femacs_countof_ptr`,
     `femacs_sizeof_ptr`, `femacs_array_ptr`, `femacs_array_mut_ptr`) on `static mut` arrays
     with zero `static_mut_refs` violations.
Also runs `check-unsafe-boundary.py --selftest`, `#![forbid(unsafe_code)]` crate unit suites
(`femacs-pure`, `femacs` CLI), and structural regression checks for the exact defects fixed in U4.
"""

from __future__ import annotations

import json
import subprocess
import sys
import tempfile
import time
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent

RUST_UNIT_HARNESS = r"""
#![deny(static_mut_refs, unsafe_op_in_unsafe_fn)]
#![allow(non_camel_case_types, non_upper_case_globals)]

pub mod common {
    use core::ffi::{c_char, c_void};
    use std::sync::atomic::{AtomicUsize, Ordering};

    pub type Lisp_Object = *mut c_void;
    pub type EMACS_INT = isize;

    #[repr(C)]
    pub struct thread_state {
        pub m_stack_bottom: *const c_char,
    }

    pub struct ThreadGlobal;
    static THREAD_PTR: AtomicUsize = AtomicUsize::new(0);

    impl ThreadGlobal {
        pub fn get(&self) -> *mut thread_state {
            THREAD_PTR.load(Ordering::Relaxed) as *mut thread_state
        }
        pub fn set(&self, v: *mut thread_state) {
            THREAD_PTR.store(v as usize, Ordering::Relaxed);
        }
    }

    pub static current_thread: ThreadGlobal = ThreadGlobal;

    #[inline]
    pub const fn lisp_h_Qnil() -> Lisp_Object {
        core::ptr::null_mut()
    }

    #[inline]
    pub fn make_fixnum(n: EMACS_INT) -> Lisp_Object {
        (((n as usize) << 2) | 2) as Lisp_Object
    }

    #[inline]
    pub fn FIXNUMP(x: Lisp_Object) -> bool {
        ((x as usize) & 3) == 2
    }

    #[inline]
    pub fn XFIXNUM(x: Lisp_Object) -> EMACS_INT {
        ((x as isize) >> 2) as EMACS_INT
    }
}

#[path = "__ROOTS_RS_PATH__"]
pub mod roots;

pub mod nonlocal {
    pub(crate) fn clear_tls_state_cache() {}
    pub(crate) fn set_tls_vm_lock_key(_: usize) {}
    pub(crate) fn tls_state_borrowed() -> bool { false }
}

#[macro_use]
#[path = "__STDCOUNTOF_RS_PATH__"]
pub mod stdcountof;

#[cfg(test)]
mod tests {
    use core::ffi::c_void;
    use std::panic::{AssertUnwindSafe, catch_unwind};
    use std::sync::Mutex;

    use crate::common::{Lisp_Object, current_thread, lisp_h_Qnil};
    use crate::roots::{
        GcContext, Immediate, LispVal, PersistentRoot, Rooted, RootedVec, ScopedRawSpan,
        UnrootedBorrow, bind_vm_lock_owner, cleanup_thread_u4_slab,
        femacs_vm_persistent_root_alloc, femacs_vm_persistent_root_get,
        femacs_vm_persistent_root_release, mark_persistent_roots, mark_thread_u4_slab_roots,
        rooting_counters, unbind_vm_lock_owner,
    };
    use crate::stdcountof::{
        femacs_array_mut_ptr, femacs_array_ptr, femacs_countof_ptr, femacs_sizeof_ptr,
    };

    static SERIAL: Mutex<()> = Mutex::new(());

    fn fake_obj(tag: usize) -> Lisp_Object {
        (tag << 3) as Lisp_Object
    }

    fn set_thread(id: usize) -> *mut c_void {
        let ptr = id as *mut c_void;
        current_thread.set(ptr as *mut crate::common::thread_state);
        if id != 0 {
            // SAFETY: serialized test acquires `SERIAL` lock simulating VM global lock ownership.
            unsafe { bind_vm_lock_owner(id) };
        } else {
            // SAFETY: clears test lock ownership immediately before releasing `SERIAL`.
            unsafe { unbind_vm_lock_owner() };
        }
        ptr
    }

    fn marked_slab(th: *mut c_void) -> Vec<usize> {
        let mut out = Vec::new();
        // SAFETY: called under serialized test VM lock on owning thread.
        unsafe { mark_thread_u4_slab_roots(th, |o| out.push(o as usize)) };
        out
    }

    fn marked_persistent() -> Vec<usize> {
        let mut out = Vec::new();
        // SAFETY: called under serialized test VM lock.
        unsafe { mark_persistent_roots(|o| out.push(o as usize)) };
        out
    }

    #[test]
    fn rooted_handles_nogc_borrow_lifo_and_unwind_truncation() {
        let _lk = SERIAL.lock().unwrap();
        let th = set_thread(0x1000);
        // SAFETY: test owns VM lock and resets slab for `th`.
        unsafe { cleanup_thread_u4_slab(th) };

        let mut ctx = GcContext::current().expect("active thread context");
        let borrowed_sum = {
            let nogc = ctx.nogc();
            let b1 = UnrootedBorrow::new(&nogc, 19usize);
            let b2 = UnrootedBorrow::new(&nogc, 23usize);
            b1.get() + b2.get()
        };
        let after_sp = ctx.safepoint(|_| borrowed_sum);
        assert_eq!(after_sp, 42);

        // SAFETY: fake tagged words constructed solely for isolated root-slab tracking tests.
        let r1 = unsafe { Rooted::new_raw(LispVal::from_raw(fake_obj(11))) };
        // SAFETY: same as above.
        let mut r2 = unsafe { Rooted::new_raw(LispVal::from_raw(fake_obj(22))) };
        assert_eq!(marked_slab(th), vec![11 << 3, 22 << 3]);

        // SAFETY: same as above.
        unsafe { r2.handle_mut().set_raw(LispVal::from_raw(fake_obj(33))) };
        assert_eq!(r2.handle().get_raw(), unsafe { LispVal::from_raw(fake_obj(33)) });
        assert_eq!(marked_slab(th), vec![11 << 3, 33 << 3]);

        let panicked = catch_unwind(AssertUnwindSafe(|| {
            // SAFETY: fake tagged word for isolated unwind tracking test.
            let _r3 = unsafe { Rooted::new_raw(LispVal::from_raw(fake_obj(44))) };
            assert_eq!(marked_slab(th), vec![11 << 3, 33 << 3, 44 << 3]);
            panic!("unwind inner Rooted");
        }));
        assert!(panicked.is_err());
        // `_r3` must have been released cleanly while `r1` and `r2` survive!
        assert_eq!(marked_slab(th), vec![11 << 3, 33 << 3]);
        assert_eq!(r1.get_raw(), unsafe { LispVal::from_raw(fake_obj(11)) });

        drop(r2);
        drop(r1);
        drop(ctx);
        assert!(marked_slab(th).is_empty());
        // SAFETY: all `Rooted` handles on `th` are dropped.
        unsafe { cleanup_thread_u4_slab(th) };
        set_thread(0);
    }

    #[test]
    fn scoped_raw_span_progressive_set_len_and_unwind() {
        let _lk = SERIAL.lock().unwrap();
        let th = set_thread(0x2000);
        // SAFETY: test owns VM lock and resets slab for `th`.
        unsafe { cleanup_thread_u4_slab(th) };

        let mut slots: [Lisp_Object; 6] = [
            fake_obj(101),
            fake_obj(102),
            fake_obj(103),
            fake_obj(999),
            fake_obj(998),
            fake_obj(997),
        ];
        // SAFETY: `slots` lives on this stack frame for the entire lifetime of `span`.
        let mut span = unsafe { ScopedRawSpan::new(slots.as_ptr(), 0) };
        assert!(marked_slab(th).is_empty());

        // Progressively expose initialized slots `[0..2)` then `[0..3)`; uninitialized `[3..6)`
        // must NOT be marked!
        // SAFETY: first 2 elements of `slots` are initialized.
        unsafe { span.set_len(2) };
        assert_eq!(marked_slab(th), vec![101 << 3, 102 << 3]);

        // SAFETY: first 3 elements of `slots` are initialized.
        unsafe { span.set_len(3) };
        assert_eq!(marked_slab(th), vec![101 << 3, 102 << 3, 103 << 3]);

        let panicked = catch_unwind(AssertUnwindSafe(|| {
            let inner_arr: [Lisp_Object; 2] = [fake_obj(201), fake_obj(202)];
            // SAFETY: `inner_arr` outlives `_inner_span`.
            let _inner_span = unsafe { ScopedRawSpan::new(inner_arr.as_ptr(), 2) };
            assert_eq!(
                marked_slab(th),
                vec![101 << 3, 102 << 3, 103 << 3, 201 << 3, 202 << 3]
            );
            panic!("unwind inner span");
        }));
        assert!(panicked.is_err());
        assert_eq!(marked_slab(th), vec![101 << 3, 102 << 3, 103 << 3]);

        core::hint::black_box(&mut slots)[0] = fake_obj(109);
        assert_eq!(marked_slab(th), vec![109 << 3, 102 << 3, 103 << 3]);

        drop(span);
        assert!(marked_slab(th).is_empty());
        // SAFETY: all spans on `th` are dropped.
        unsafe { cleanup_thread_u4_slab(th) };
        set_thread(0);
    }

    #[test]
    fn persistent_root_slot_reuse_and_aba_generation_rejection() {
        let _lk = SERIAL.lock().unwrap();
        let th = set_thread(0x7000);
        // SAFETY: test owns VM lock.
        unsafe { cleanup_thread_u4_slab(th) };
        let before = rooting_counters();

        // SAFETY: fake tagged word for isolated persistent root test.
        let mut p1 = unsafe { PersistentRoot::new_raw(LispVal::from_raw(fake_obj(501))) };
        let (slot_idx_1, gen_1) = p1.raw_token();
        assert_eq!(p1.handle().get_raw(), unsafe { LispVal::from_raw(fake_obj(501)) });
        assert!(marked_persistent().contains(&(501 << 3)));

        assert!(p1.release());
        assert!(!p1.release());
        let stale_get_after_release = catch_unwind(AssertUnwindSafe(|| p1.handle().is_nil()));
        assert!(
            stale_get_after_release.is_err(),
            "stale Handle inspection after PersistentRoot release must panic"
        );
        assert!(!marked_persistent().contains(&(501 << 3)));

        // Reallocate via C-ABI entrypoint: free-list reuses `slot_idx_1` with bumped generation!
        let mut idx2: usize = usize::MAX;
        let mut gen2: u64 = 0;
        // SAFETY: `idx2` and `gen2` are valid aligned writable locals.
        unsafe { femacs_vm_persistent_root_alloc(fake_obj(502), &raw mut idx2, &raw mut gen2) };
        assert_eq!(idx2, slot_idx_1);
        assert!(gen2 > gen_1);
        // SAFETY: called under serialized test VM lock on owning thread.
        assert_eq!(unsafe { femacs_vm_persistent_root_get(idx2, gen2) }, fake_obj(502));

        // Stale generation on the SAME slot index MUST still be rejected (ABA safety)!
        // SAFETY: called under serialized test VM lock on owning thread.
        unsafe {
            assert_eq!(femacs_vm_persistent_root_get(slot_idx_1, gen_1), lisp_h_Qnil());
            assert!(!femacs_vm_persistent_root_release(slot_idx_1, gen_1));
        }
        let stale_get_after_reuse = catch_unwind(AssertUnwindSafe(|| p1.handle().is_nil()));
        assert!(
            stale_get_after_reuse.is_err(),
            "stale Handle inspection after slot reuse must panic on generation mismatch"
        );
        assert!(marked_persistent().contains(&(502 << 3)));

        // SAFETY: called under serialized test VM lock on owning thread.
        unsafe {
            assert!(femacs_vm_persistent_root_release(idx2, gen2));
            assert!(!femacs_vm_persistent_root_release(idx2, gen2));
            assert_eq!(femacs_vm_persistent_root_get(idx2, gen2), lisp_h_Qnil());
        }

        let after = rooting_counters();
        assert_eq!(after.persistent_live, before.persistent_live);
        // SAFETY: test cleanup under VM lock.
        unsafe { cleanup_thread_u4_slab(th) };
        set_thread(0);
    }

    #[test]
    fn immediate_and_lisp_val_fixnum_and_nil_tag_roundtrip() {
        assert!(Immediate::nil().is_nil());
        assert_eq!(Immediate::nil().as_fixnum(), None);
        assert!(LispVal::nil().is_nil());
        assert_eq!(LispVal::nil().as_fixnum(), None);
        for n in [-1024isize, -1, 0, 1, 42, 1_000_000] {
            let imm = Immediate::from_fixnum(n);
            assert!(!imm.is_nil());
            assert_eq!(imm.as_fixnum(), Some(n));
            let v = LispVal::from_fixnum(n);
            assert!(!v.is_nil());
            assert_eq!(v.as_fixnum(), Some(n));
        }
    }

    #[test]
    fn stdcountof_pure_pointer_helpers_zero_static_mut_ref() {
        static mut TABLE: [u64; 7] = [1, 2, 3, 4, 5, 6, 7];
        assert_eq!(countof!(TABLE), 7);
        assert_eq!(femacs_countof_ptr(&raw const TABLE), 7);
        assert_eq!(femacs_sizeof_ptr(&raw const TABLE), 56);
        let rp = femacs_array_ptr(&raw const TABLE);
        let mp = femacs_array_mut_ptr(&raw mut TABLE);
        // SAFETY: serialized test writes and reads valid slot 3 of `TABLE`.
        unsafe {
            *mp.add(3) = 99;
            assert_eq!(*rp.add(3), 99);
        }
    }

    #[test]
    fn gc_context_is_single_lease_and_rejects_other_os_thread() {
        let _lk = SERIAL.lock().unwrap();
        let th = set_thread(0x3000);
        // SAFETY: test owns VM lock.
        unsafe { cleanup_thread_u4_slab(th) };

        let mut ctx1 = GcContext::current().expect("lock owner obtains first GcContext");
        assert!(
            GcContext::current().is_none(),
            "second simultaneous GcContext::current() must return None"
        );
        ctx1.safepoint(|_| {
            assert!(
                GcContext::current().is_none(),
                "safepoint closure still holds GcContext lease"
            );
        });
        let spawned = std::thread::spawn(|| GcContext::current().is_some())
            .join()
            .expect("join thread");
        assert!(
            !spawned,
            "another OS thread must NEVER obtain GcContext even when current_thread != null"
        );
        drop(ctx1);

        let ctx2 = GcContext::current().expect("re-acquire GcContext after drop");
        drop(ctx2);
        // SAFETY: test cleanup under VM lock.
        unsafe { cleanup_thread_u4_slab(th) };
        set_thread(0);
    }

    #[test]
    fn rooted_out_of_order_drop_preserves_inner_live_root_and_handle() {
        let _lk = SERIAL.lock().unwrap();
        let th = set_thread(0x4000);
        // SAFETY: test owns VM lock.
        unsafe { cleanup_thread_u4_slab(th) };

        // SAFETY: fake tagged words for isolated root-slab tests.
        let outer = unsafe { Rooted::new_raw(LispVal::from_raw(fake_obj(71))) };
        let mut inner = unsafe { Rooted::new_raw(LispVal::from_raw(fake_obj(72))) };
        assert_eq!(marked_slab(th), vec![71 << 3, 72 << 3]);

        // Drop `outer` BEFORE `inner`! Previously `truncate(self.index)` wiped `inner` in release.
        drop(outer);
        assert_eq!(inner.get_raw(), unsafe { LispVal::from_raw(fake_obj(72)) });
        assert_eq!(inner.handle().get_raw(), unsafe { LispVal::from_raw(fake_obj(72)) });
        assert_eq!(marked_slab(th), vec![72 << 3]);

        // SAFETY: fake tagged words for isolated root-slab tests.
        unsafe { inner.set_raw(LispVal::from_raw(fake_obj(73))) };
        // Free-list slot reuse must not disturb live `inner` at slot 1.
        let reused = unsafe { Rooted::new_raw(LispVal::from_raw(fake_obj(74))) };
        assert_eq!(reused.get_raw(), unsafe { LispVal::from_raw(fake_obj(74)) });
        assert_eq!(inner.handle().get_raw(), unsafe { LispVal::from_raw(fake_obj(73)) });
        assert_eq!(marked_slab(th), vec![74 << 3, 73 << 3]);

        drop(reused);
        drop(inner);
        assert!(marked_slab(th).is_empty());
        // SAFETY: test cleanup under VM lock.
        unsafe { cleanup_thread_u4_slab(th) };
        set_thread(0);
    }

    #[test]
    fn rooted_vec_out_of_order_drop_preserves_other_live_spans() {
        let _lk = SERIAL.lock().unwrap();
        let th = set_thread(0x5000);
        // SAFETY: test owns VM lock.
        unsafe { cleanup_thread_u4_slab(th) };

        let mut v1 = RootedVec::new();
        v1.push(fake_obj(81));
        let mut v2 = RootedVec::new();
        v2.push(fake_obj(82));
        v2.push(fake_obj(83));
        assert_eq!(marked_slab(th), vec![81 << 3, 82 << 3, 83 << 3]);

        drop(v1);
        assert_eq!(marked_slab(th), vec![82 << 3, 83 << 3]);

        drop(v2);
        assert!(marked_slab(th).is_empty());
        // SAFETY: test cleanup under VM lock.
        unsafe { cleanup_thread_u4_slab(th) };
        set_thread(0);
    }

    #[test]
    fn rooted_new_panics_on_non_owner_os_thread() {
        let _lk = SERIAL.lock().unwrap();
        let th = set_thread(0x6000);
        // SAFETY: test owns VM lock.
        unsafe { cleanup_thread_u4_slab(th) };

        let res = std::thread::spawn(|| {
            catch_unwind(AssertUnwindSafe(|| {
                // SAFETY: fake tagged word for isolated test.
                let _bad = unsafe { Rooted::new_raw(LispVal::from_raw(fake_obj(91))) };
            }))
        })
        .join()
        .expect("join thread");
        assert!(res.is_err(), "non-owner OS thread Rooted::new_raw must panic");
        assert!(marked_slab(th).is_empty());
        // SAFETY: test cleanup under VM lock.
        unsafe { cleanup_thread_u4_slab(th) };
        set_thread(0);
    }

    #[test]
    fn cleanup_and_mark_reject_non_owner_os_thread() {
        let _lk = SERIAL.lock().unwrap();
        let th = set_thread(0x6100);
        // SAFETY: test owns VM lock.
        unsafe { cleanup_thread_u4_slab(th) };
        // SAFETY: immediate fixnum word.
        let live_root = unsafe { Rooted::new_raw(LispVal::from_fixnum(123)) };
        let live_word = LispVal::from_fixnum(123).into_raw() as usize;
        assert_eq!(marked_slab(th), vec![live_word]);

        let th_addr = th as usize;
        let (clean_err, mark_slab_err, mark_pers_err, cnt_err) = std::thread::spawn(move || {
            let foreign = th_addr as *mut c_void;
            let c1 = catch_unwind(AssertUnwindSafe(|| unsafe { cleanup_thread_u4_slab(foreign) }));
            let c2 = catch_unwind(AssertUnwindSafe(|| unsafe {
                mark_thread_u4_slab_roots(foreign, |_| {})
            }));
            let c3 = catch_unwind(AssertUnwindSafe(|| unsafe { mark_persistent_roots(|_| {}) }));
            let c4 = catch_unwind(AssertUnwindSafe(|| {
                let _ = rooting_counters();
            }));
            (c1.is_err(), c2.is_err(), c3.is_err(), c4.is_err())
        })
        .join()
        .expect("join thread");

        assert!(clean_err, "non-owner cleanup_thread_u4_slab must panic");
        assert!(mark_slab_err, "non-owner mark_thread_u4_slab_roots must panic");
        assert!(mark_pers_err, "non-owner mark_persistent_roots must panic");
        assert!(cnt_err, "non-owner rooting_counters must panic");
        assert_eq!(live_root.as_fixnum(), Some(123));
        assert_eq!(marked_slab(th), vec![live_word]);

        drop(live_root);
        // SAFETY: test cleanup under VM lock.
        unsafe { cleanup_thread_u4_slab(th) };
        set_thread(0);
    }

    #[test]
    fn bind_rejects_second_os_thread_while_locked_and_unbind_rejects_active_context() {
        let _lk = SERIAL.lock().unwrap();
        let th = set_thread(0x6200);
        // SAFETY: test owns VM lock.
        unsafe { cleanup_thread_u4_slab(th) };

        let th_addr = th as usize;
        let (bind_err, unbind_err) = std::thread::spawn(move || {
            let b = catch_unwind(AssertUnwindSafe(|| unsafe { bind_vm_lock_owner(th_addr) }));
            let u = catch_unwind(AssertUnwindSafe(|| unsafe { unbind_vm_lock_owner() }));
            (b.is_err(), u.is_err())
        })
        .join()
        .expect("join thread");
        assert!(bind_err, "non-owner bind_vm_lock_owner must panic while locked");
        assert!(unbind_err, "non-owner unbind_vm_lock_owner must panic while locked");

        let ctx = GcContext::current().expect("owner holds lock and gets GcContext");
        let unbind_while_ctx = catch_unwind(AssertUnwindSafe(|| unsafe { unbind_vm_lock_owner() }));
        assert!(
            unbind_while_ctx.is_err(),
            "unbind_vm_lock_owner must panic while GcContext lease is still active"
        );
        drop(ctx);

        // SAFETY: test cleanup under VM lock.
        unsafe { cleanup_thread_u4_slab(th) };
        set_thread(0);
    }

    #[test]
    fn main_and_worker_lock_handoff_preserves_out_of_order_roots() {
        let _lk = SERIAL.lock().unwrap();
        static VM_HANDOFF_MUTEX: Mutex<()> = Mutex::new(());
        let main_th = 0x6301usize as *mut c_void;
        let worker_th_addr = 0x6302usize;

        // 1. Main thread acquires lock, creates two Rooted slots, and hands off lock.
        let (outer_main, inner_main) = {
            let _g = VM_HANDOFF_MUTEX.lock().unwrap();
            current_thread.set(main_th as *mut crate::common::thread_state);
            // SAFETY: owns `VM_HANDOFF_MUTEX`.
            unsafe {
                bind_vm_lock_owner(main_th as usize);
                cleanup_thread_u4_slab(main_th);
            }
            let ctx = GcContext::current().expect("main acquires GcContext");
            let outer = ctx.root_immediate(Immediate::from_fixnum(1001));
            let inner = ctx.root_immediate(Immediate::from_fixnum(1002));
            drop(ctx);
            // SAFETY: ending lock turn while no GcContext/slab borrow is live.
            unsafe { unbind_vm_lock_owner() };
            current_thread.set(core::ptr::null_mut());
            (outer, inner)
        };

        // 2. Worker OS thread acquires physical lock, proves no GcContext before binding, runs
        //    out-of-order Rooted drops, cleans up worker slab, and hands lock back.
        std::thread::spawn(move || {
            let worker_th = worker_th_addr as *mut c_void;
            let _g = VM_HANDOFF_MUTEX.lock().unwrap();
            assert!(
                GcContext::current().is_none(),
                "worker cannot acquire GcContext before bind_vm_lock_owner"
            );
            current_thread.set(worker_th as *mut crate::common::thread_state);
            // SAFETY: worker holds `VM_HANDOFF_MUTEX`.
            unsafe {
                bind_vm_lock_owner(worker_th_addr);
                cleanup_thread_u4_slab(worker_th);
            }
            let ctx_w = GcContext::current().expect("worker acquires GcContext after bind");
            let w_outer = ctx_w.root_immediate(Immediate::from_fixnum(2001));
            let w_inner = ctx_w.root_immediate(Immediate::from_fixnum(2002));
            drop(w_outer);
            assert_eq!(w_inner.as_fixnum(), Some(2002));
            drop(w_inner);
            drop(ctx_w);
            // SAFETY: worker drops all roots before cleanup and unbind.
            unsafe {
                cleanup_thread_u4_slab(worker_th);
                unbind_vm_lock_owner();
            }
            current_thread.set(core::ptr::null_mut());
        })
        .join()
        .expect("worker handoff join");

        // 3. Main thread re-acquires physical lock, confirms Main's live roots survived untouched,
        //    and performs out-of-order drop on Main!
        {
            let _g = VM_HANDOFF_MUTEX.lock().unwrap();
            assert!(GcContext::current().is_none());
            current_thread.set(main_th as *mut crate::common::thread_state);
            // SAFETY: main re-acquired `VM_HANDOFF_MUTEX`.
            unsafe { bind_vm_lock_owner(main_th as usize) };
            let _ctx2 = GcContext::current().expect("main re-acquires GcContext");
            assert_eq!(outer_main.as_fixnum(), Some(1001));
            assert_eq!(inner_main.as_fixnum(), Some(1002));
            drop(outer_main);
            assert_eq!(inner_main.handle().as_fixnum(), Some(1002));
            drop(inner_main);
            assert!(marked_slab(main_th).is_empty());
            drop(_ctx2);
            // SAFETY: test cleanup under `VM_HANDOFF_MUTEX`.
            unsafe {
                cleanup_thread_u4_slab(main_th);
                unbind_vm_lock_owner();
            }
            current_thread.set(core::ptr::null_mut());
        }
    }
}
"""


def run_checked(cmd: list[str], *, timeout: int = 120) -> subprocess.CompletedProcess[str]:
    proc = subprocess.run(
        cmd,
        cwd=REPO,
        capture_output=True,
        text=True,
        timeout=timeout,
        check=False,
    )
    if proc.returncode != 0:
        raise RuntimeError(
            f"command failed (exit {proc.returncode}): {' '.join(cmd)}\n"
            f"stdout:\n{proc.stdout[-2000:]}\nstderr:\n{proc.stderr[-2000:]}"
        )
    return proc


def main() -> int:
    t0_all = time.monotonic()
    groups: list[dict] = []

    def run_group(name: str, fn) -> None:
        t0 = time.monotonic()
        fn()
        dt = time.monotonic() - t0
        groups.append({"group": name, "seconds": round(dt, 4), "status": "passed"})
        print(f"GROUP {name} seconds={dt:.4f} status=passed", flush=True)

    def group1_isolated_roots_and_stdcountof_rust_tests() -> None:
        with tempfile.TemporaryDirectory(prefix="femacs-u4-unit-") as tmp:
            work = Path(tmp)
            harness = work / "u4_unit_harness.rs"
            bin_out = work / "u4_unit_harness"
            src = (
                RUST_UNIT_HARNESS.replace(
                    "__ROOTS_RS_PATH__", str((REPO / "crates/femacs-vm/src/roots.rs").resolve())
                )
                .replace(
                    "__STDCOUNTOF_RS_PATH__", str((REPO / "lib/include/stdcountof.rs").resolve())
                )
                .lstrip()
            )
            harness.write_text(src, encoding="utf-8")
            run_checked(
                [
                    "rustc",
                    "--edition=2024",
                    "--test",
                    "-O",
                    str(harness),
                    "-o",
                    str(bin_out),
                ],
                timeout=60,
            )
            proc = run_checked([str(bin_out), "--test-threads=1"], timeout=30)
            assert "12 passed; 0 failed" in proc.stdout, proc.stdout

    def group4_external_safe_caller_boundary_gate() -> None:
        # Part 4a: Real `femacs-vm` crate `#![forbid(unsafe_code)]` compile-fail, lifetime & compile-pass gate.
        with tempfile.TemporaryDirectory(prefix="femacs-real-vm-gate-") as real_tmp:
            real_work = Path(real_tmp)
            target_dir = real_work / "cargo-target"
            run_checked(
                [
                    "cargo",
                    "check",
                    "-p",
                    "femacs-vm",
                    "--target-dir",
                    str(target_dir),
                ],
                timeout=180,
            )
            rmetas = sorted((target_dir / "debug").rglob("libfemacs_vm-*.rmeta"))
            assert rmetas, f"missing libfemacs_vm-*.rmeta under {target_dir / 'debug'}"
            real_rmeta = rmetas[-1]
            dep_dirs = sorted({
                str(p.parent)
                for pat in ("*.rmeta", "*.rlib")
                for p in (target_dir / "debug").rglob(pat)
            })
            dep_flags = []
            for d in dep_dirs:
                dep_flags.extend(["-L", f"dependency={d}"])

            real_compile_pass_src = """
#![forbid(unsafe_code)]
extern crate femacs_vm;

use femacs_vm::nonlocal::VmResult;
use femacs_vm::roots::{GcContext, Immediate, PersistentRoot, Rooted};
use femacs_vm::{safe_domain, state, vm_primitives};

pub fn inspect_readonly_vm_state() -> (bool, usize) {
    (state::has_active_thread(), state::current_thread_addr())
}

pub fn safe_real_vm_flow(ctx: &mut GcContext, text: &str) -> VmResult<Option<String>> {
    let mut r = ctx.root_immediate(Immediate::from_fixnum(7));
    assert_eq!(r.as_fixnum(), Some(7));
    r.set_immediate(Immediate::nil());
    assert!(r.is_nil());

    let s: Rooted = vm_primitives::make_utf8_string(ctx, text)?;
    let clone_r = Rooted::from_handle(ctx, s.handle());
    assert!(s.same_object(clone_r.handle()));
    let mut p = PersistentRoot::from_handle(ctx, s.handle());
    drop(s);
    drop(clone_r);
    vm_primitives::trigger_garbage_collect(ctx)?;
    let _buf = vm_primitives::get_buffer_create(ctx, p.handle())?;
    let evaled = vm_primitives::eval_lexical(ctx, p.handle())?;
    let _ = vm_primitives::lisp_string_to_owned(ctx, evaled.handle());
    let out = vm_primitives::lisp_string_to_owned(ctx, p.handle());
    p.release();
    let _dom: Rooted = safe_domain::exercise_all_safe_domains(ctx, "ignored")?;
    Ok(out)
}
"""
            pass_rs = real_work / "real_pass.rs"
            pass_out = real_work / "libreal_pass.rmeta"
            pass_rs.write_text(real_compile_pass_src.lstrip(), encoding="utf-8")
            run_checked(
                [
                    "rustc",
                    "--edition=2024",
                    "--crate-type=lib",
                    "--emit=metadata",
                    "--extern",
                    f"femacs_vm={real_rmeta}",
                    *dep_flags,
                    str(pass_rs),
                    "-o",
                    str(pass_out),
                ],
                timeout=60,
            )

            real_compile_fail_cases = [
                ("writable_current_thread_set", "pub fn p() { femacs_vm::thread::current_thread.set(core::ptr::null_mut()); }"),
                ("private_current_thread_get", "pub fn p() { let _ = femacs_vm::thread::current_thread.get(); }"),
                ("writable_all_threads_set", "pub fn p() { femacs_vm::thread::all_threads.set(core::ptr::null_mut()); }"),
                ("private_common_current_thread", "pub fn p() { femacs_vm::common::current_thread.set(core::ptr::null_mut()); }"),
                ("private_femacs_sys_reexport", "pub fn p() { let _ = femacs_vm::femacs_sys::VmGlobal::new(0usize); }"),
                ("private_vm_global_reexport", "pub fn p() { let _ = femacs_vm::VmGlobal::new(0usize); }"),
                ("private_emacs_editor_main", "pub fn p() { let _ = femacs_vm::emacs::femacs_editor_main; }"),
                ("unsafe_editor_main", "pub fn p() { let _ = femacs_vm::editor_main(0, core::ptr::null_mut()); }"),
                ("private_safe_cmds", "pub fn p() { let _ = femacs_vm::safe_cmds::move_point; }"),
                ("private_alloc_mod", "pub fn p() { let _ = femacs_vm::alloc::Fgarbage_collect; }"),
                ("bind_vm_lock_owner", "pub fn p() { femacs_vm::roots::bind_vm_lock_owner(1); }"),
                ("unbind_vm_lock_owner", "pub fn p() { femacs_vm::roots::unbind_vm_lock_owner(); }"),
                ("cleanup_thread_u4_slab", "pub fn p() { femacs_vm::roots::cleanup_thread_u4_slab(core::ptr::null_mut()); }"),
                ("mark_thread_u4_slab_roots", "pub fn p() { femacs_vm::roots::mark_thread_u4_slab_roots(core::ptr::null_mut(), |_| {}); }"),
                ("mark_persistent_roots", "pub fn p() { femacs_vm::roots::mark_persistent_roots(|_| {}); }"),
                ("rooting_counters_free", "pub fn p() { let _ = femacs_vm::roots::rooting_counters(); }"),
                ("cleanup_thread_state", "pub fn p() { femacs_vm::nonlocal::cleanup_thread_state(core::ptr::null_mut()); }"),
                ("set_comp_native_exit", "pub fn p() { let _ = femacs_vm::nonlocal::set_comp_native_exit; }"),
                ("private_lisp_val", "pub fn p(_: femacs_vm::roots::LispVal) {}"),
                ("removed_rooted_new", "pub fn p() { let _ = femacs_vm::roots::Rooted::new; }"),
                ("removed_persistent_root_new", "pub fn p() { let _ = femacs_vm::roots::PersistentRoot::new; }"),
                ("private_handle_get_raw", "pub fn p(h: femacs_vm::roots::Handle<'_>) { let _ = h.get_raw(); }"),
                ("removed_rooted_get", "pub fn p(r: &femacs_vm::roots::Rooted) { let _ = r.get(); }"),
                ("removed_persistent_get", "pub fn p(p: &femacs_vm::roots::PersistentRoot) { let _ = p.get(); }"),
                ("unsafe_persistent_alloc_cabi", "pub fn p() { femacs_vm::roots::femacs_vm_persistent_root_alloc(core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut()); }"),
                ("unsafe_persistent_get_cabi", "pub fn p() { let _ = femacs_vm::roots::femacs_vm_persistent_root_get(0, 0); }"),
                ("unsafe_persistent_release_cabi", "pub fn p() { let _ = femacs_vm::roots::femacs_vm_persistent_root_release(0, 0); }"),
                ("unsafe_host_drop_counts_cabi", "pub fn p() { femacs_vm::nonlocal::femacs_vm_host_drop_counts(core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut()); }"),
                ("unsafe_u4_root_stats_cabi", "pub fn p() { let _ = femacs_vm::state::femacs_vm_u4_root_stats(core::ptr::null_mut(), 0); }"),
                ("gc_context_not_send", "fn assert_send<T: Send>() {} pub fn p() { assert_send::<femacs_vm::roots::GcContext>(); }"),
                ("rooted_not_send", "fn assert_send<T: Send>() {} pub fn p() { assert_send::<femacs_vm::roots::Rooted>(); }"),
                ("handle_not_send", "fn assert_send<T: Send>() {} pub fn p() { assert_send::<femacs_vm::roots::Handle<'static>>(); }"),
                ("vm_state_not_send", "fn assert_send<T: Send>() {} pub fn p() { assert_send::<femacs_vm::state::VmState>(); }"),
                ("thread_state_not_send", "fn assert_send<T: Send>() {} pub fn p() { assert_send::<femacs_vm::state::ThreadState>(); }"),
                ("rooted_non_copy_move", "pub fn p(ctx: &mut femacs_vm::roots::GcContext) { if let Ok(r) = femacs_vm::vm_primitives::make_utf8_string(ctx, \"hi\") { let _m = r; let _dup = r; } }"),
                ("handle_outlives_drop_rooted", "pub fn p(ctx: &mut femacs_vm::roots::GcContext) { if let Ok(r) = femacs_vm::vm_primitives::make_utf8_string(ctx, \"hi\") { let h = r.handle(); drop(r); let _ = h.is_nil(); } }"),
                ("handle_outlives_persistent_release", "pub fn p() { let mut p = femacs_vm::roots::PersistentRoot::from_immediate(femacs_vm::roots::Immediate::nil()); let h = p.handle(); p.release(); let _ = h.is_nil(); }"),
                ("unrooted_borrow_crosses_gc", "pub fn p(ctx: &mut femacs_vm::roots::GcContext) { let nogc = ctx.nogc(); let b = femacs_vm::roots::UnrootedBorrow::new(&nogc, 1usize); let _ = femacs_vm::vm_primitives::trigger_garbage_collect(ctx); let _ = b.get(); }"),
            ]
            for name, body in real_compile_fail_cases:
                bad_rs = real_work / f"real_bad_{name}.rs"
                bad_rmeta = real_work / f"libreal_bad_{name}.rmeta"
                bad_rs.write_text(
                    f"#![forbid(unsafe_code)]\nextern crate femacs_vm;\n{body}\n",
                    encoding="utf-8",
                )
                comp = subprocess.run(
                    [
                        "rustc",
                        "--edition=2024",
                        "--crate-type=lib",
                        "--emit=metadata",
                        "--extern",
                        f"femacs_vm={real_rmeta}",
                        *dep_flags,
                        str(bad_rs),
                        "-o",
                        str(bad_rmeta),
                    ],
                    cwd=REPO,
                    capture_output=True,
                    text=True,
                    timeout=30,
                    check=False,
                )
                assert comp.returncode != 0, f"real femacs-vm external forbid(unsafe_code) caller unexpectedly compiled `{name}`!"
                assert "error[" in comp.stderr or "error:" in comp.stderr, (
                    f"expected real compiler diagnostic for `{name}`, got:\n{comp.stderr}"
                )

        # Part 4b: Runtime multi-thread lock handoff & out-of-order `Rooted` drop check.
        boundary_lib_src = r"""
#![deny(static_mut_refs, unsafe_op_in_unsafe_fn)]
#![allow(non_camel_case_types, non_upper_case_globals)]

pub mod common {
    use core::ffi::{c_char, c_int, c_void};
    use std::sync::atomic::{AtomicUsize, Ordering};

    pub type Lisp_Object = *mut c_void;
    pub type EMACS_INT = isize;
    pub type nonlocal_exit = c_int;
    pub const NONLOCAL_EXIT_SIGNAL: nonlocal_exit = 0;
    pub const NONLOCAL_EXIT_THROW: nonlocal_exit = 1;

    #[repr(C)]
    pub struct handler {
        pub dummy: usize,
    }

    #[repr(C)]
    pub struct thread_state {
        pub m_stack_bottom: *const c_char,
    }

    pub struct ThreadGlobal;
    static THREAD_PTR: AtomicUsize = AtomicUsize::new(0);

    impl ThreadGlobal {
        pub fn get(&self) -> *mut thread_state {
            THREAD_PTR.load(Ordering::Acquire) as *mut thread_state
        }
        pub(crate) fn set(&self, v: *mut thread_state) {
            THREAD_PTR.store(v as usize, Ordering::Release);
        }
    }

    // Intentionally safe-readable across crates just like `src/thread.rs:183`!
    pub static current_thread: ThreadGlobal = ThreadGlobal;

    #[inline]
    pub const fn lisp_h_Qnil() -> Lisp_Object {
        core::ptr::null_mut()
    }

    #[inline]
    pub fn make_fixnum(n: EMACS_INT) -> Lisp_Object {
        (((n as usize) << 2) | 2) as Lisp_Object
    }

    #[inline]
    pub fn FIXNUMP(x: Lisp_Object) -> bool {
        ((x as usize) & 3) == 2
    }

    #[inline]
    pub fn XFIXNUM(x: Lisp_Object) -> EMACS_INT {
        ((x as isize) >> 2) as EMACS_INT
    }
}

pub mod eval {
    use crate::common::Lisp_Object;
    pub unsafe extern "C-unwind" fn Ffuncall(_nargs: isize, args: *mut Lisp_Object) -> Lisp_Object {
        unsafe { *args }
    }
}

#[path = "__ROOTS_RS_PATH__"]
pub mod roots;

#[path = "__NONLOCAL_RS_PATH__"]
pub mod nonlocal;

static VM_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// Physical-lock-backed VM kernel entry: the ONLY way external code can enter a VM lock epoch.
pub fn with_vm_lock<R>(thread: &mut common::thread_state, f: impl FnOnce() -> R) -> R {
    struct UnbindOnExit;
    impl Drop for UnbindOnExit {
        fn drop(&mut self) {
            // SAFETY: holds `VM_LOCK` mutex on this OS thread while unbinding lock ownership.
            unsafe { roots::unbind_vm_lock_owner() };
            common::current_thread.set(core::ptr::null_mut());
        }
    }

    let _guard = VM_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let key = (&raw mut *thread) as usize;
    common::current_thread.set(&raw mut *thread);
    // SAFETY: holds `VM_LOCK` mutex on this OS thread with live `thread_state`.
    unsafe { roots::bind_vm_lock_owner(key) };
    let _unbind = UnbindOnExit;
    f()
}

pub fn cleanup_thread_after_use(thread: &mut common::thread_state) {
    let key = (&raw mut *thread) as usize;
    with_vm_lock(thread, || {
        let raw = key as *mut core::ffi::c_void;
        // SAFETY: `VM_LOCK` is held and caller ensures all thread roots were dropped.
        unsafe {
            nonlocal::cleanup_thread_state(raw);
            roots::cleanup_thread_u4_slab(raw);
        }
    });
}
"""
        compile_fail_cases = [
            ("bind_vm_lock_owner", "femacs_vm_boundary::roots::bind_vm_lock_owner(1);"),
            ("unbind_vm_lock_owner", "femacs_vm_boundary::roots::unbind_vm_lock_owner();"),
            ("cleanup_thread_u4_slab", "femacs_vm_boundary::roots::cleanup_thread_u4_slab(core::ptr::null_mut());"),
            ("mark_thread_u4_slab_roots", "femacs_vm_boundary::roots::mark_thread_u4_slab_roots(core::ptr::null_mut(), |_| {});"),
            ("mark_persistent_roots", "femacs_vm_boundary::roots::mark_persistent_roots(|_| {});"),
            ("rooting_counters_free", "let _ = femacs_vm_boundary::roots::rooting_counters();"),
            ("with_narrowed_stack_bottom_free", "femacs_vm_boundary::roots::with_narrowed_stack_bottom(|| {});"),
            ("lispval_from_raw", "let _ = femacs_vm_boundary::roots::LispVal::from_raw(core::ptr::null_mut());"),
            ("persistent_root_alloc_cabi", "femacs_vm_boundary::roots::femacs_vm_persistent_root_alloc(core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());"),
            ("persistent_root_get_cabi", "let _ = femacs_vm_boundary::roots::femacs_vm_persistent_root_get(0, 0);"),
            ("persistent_root_release_cabi", "let _ = femacs_vm_boundary::roots::femacs_vm_persistent_root_release(0, 0);"),
            ("cleanup_thread_state", "femacs_vm_boundary::nonlocal::cleanup_thread_state(core::ptr::null_mut());"),
            ("set_comp_native_exit", "let _ = femacs_vm_boundary::nonlocal::set_comp_native_exit;"),
            ("host_drop_counts_cabi", "let _ = femacs_vm_boundary::nonlocal::femacs_vm_host_drop_counts(core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());"),
        ]
        external_runtime_src = r"""
#![forbid(unsafe_code)]

use femacs_vm_boundary::{cleanup_thread_after_use, with_vm_lock};
use femacs_vm_boundary::common::{current_thread, thread_state};
use femacs_vm_boundary::roots::{GcContext, Immediate, Rooted};

fn main() {
    assert!(
        GcContext::current().is_none(),
        "external safe caller outside VM lock cannot obtain GcContext"
    );

    let stack_anchor: usize = 0;
    let fake_bottom = (&raw const stack_anchor) as usize + 4096;
    let mut main_ts = thread_state { m_stack_bottom: fake_bottom as *const _ };

    let outer_main = with_vm_lock(&mut main_ts, || {
        // Safe code CAN read global `current_thread`, just like `src/thread.rs:183`.
        assert!(!current_thread.get().is_null());

        // Spawn another OS thread while Main holds physical lock and `current_thread` is non-null:
        // purely safe external caller MUST fail to get `GcContext`.
        let other_ctx = std::thread::spawn(|| {
            assert!(!current_thread.get().is_null());
            GcContext::current().is_some()
        })
        .join()
        .expect("join external non-owner thread");
        assert!(!other_ctx, "other OS thread cannot acquire GcContext");

        let mut ctx = GcContext::current().expect("lock holder obtains GcContext");
        let _cnt = ctx.rooting_counters();
        ctx.with_narrowed_stack_bottom(|ctx| {
            let outer = Rooted::from_immediate(ctx, Immediate::from_fixnum(11));
            let inner = Rooted::from_handle(ctx, outer.handle());
            assert!(outer.same_object(inner.handle()));
            // Out-of-order drop: dropping outer first leaves inner alive!
            drop(outer);
            let mut inner = inner;
            inner.set_immediate(Immediate::from_fixnum(22));
            assert_eq!(inner.as_fixnum(), Some(22));
            assert_eq!(inner.handle().as_fixnum(), Some(22));
            drop(inner);
        });
        ctx.root_immediate(Immediate::from_fixnum(33))
    });

    assert!(GcContext::current().is_none(), "released lock clears GcContext");

    // Lock handoff to worker thread and back to main thread.
    std::thread::spawn(|| {
        let mut worker_ts = thread_state { m_stack_bottom: core::ptr::null() };
        with_vm_lock(&mut worker_ts, || {
            let ctx = GcContext::current().expect("worker lock holder obtains GcContext");
            let w_outer = ctx.root_immediate(Immediate::from_fixnum(44));
            let w_inner = ctx.root_immediate(Immediate::from_fixnum(55));
            drop(w_outer);
            assert_eq!(w_inner.as_fixnum(), Some(55));
            drop(w_inner);
        });
        cleanup_thread_after_use(&mut worker_ts);
    })
    .join()
    .expect("join worker handoff");

    with_vm_lock(&mut main_ts, || {
        let _ctx = GcContext::current().expect("main reacquires lock");
        assert_eq!(outer_main.as_fixnum(), Some(33));
        drop(outer_main);
    });
    cleanup_thread_after_use(&mut main_ts);
}
"""
        with tempfile.TemporaryDirectory(prefix="femacs-u4-ext-boundary-") as tmp:
            work = Path(tmp)
            lib_rs = work / "femacs_vm_boundary.rs"
            rlib = work / "libfemacs_vm_boundary.rlib"
            lib_rs.write_text(
                boundary_lib_src.replace(
                    "__ROOTS_RS_PATH__", str((REPO / "crates/femacs-vm/src/roots.rs").resolve())
                )
                .replace(
                    "__NONLOCAL_RS_PATH__", str((REPO / "crates/femacs-vm/src/nonlocal.rs").resolve())
                )
                .lstrip(),
                encoding="utf-8",
            )
            run_checked(
                ["rustc", "--edition=2024", "--crate-type=lib", "--crate-name=femacs_vm_boundary", "-O", str(lib_rs), "-o", str(rlib)],
                timeout=60,
            )

            for name, stmt in compile_fail_cases:
                bad_rs = work / f"bad_{name}.rs"
                bad_bin = work / f"bad_{name}"
                bad_rs.write_text(
                    f"#![forbid(unsafe_code)]\nextern crate femacs_vm_boundary;\nfn main() {{\n    {stmt}\n}}\n",
                    encoding="utf-8",
                )
                comp = subprocess.run(
                    ["rustc", "--edition=2024", "--extern", f"femacs_vm_boundary={rlib}", str(bad_rs), "-o", str(bad_bin)],
                    cwd=REPO,
                    capture_output=True,
                    text=True,
                    timeout=30,
                    check=False,
                )
                assert comp.returncode != 0, f"external forbid(unsafe_code) caller unexpectedly compiled `{name}`!"

            ext_rs = work / "ext_runtime_ok.rs"
            ext_bin = work / "ext_runtime_ok"
            ext_rs.write_text(external_runtime_src.lstrip(), encoding="utf-8")
            run_checked(
                ["rustc", "--edition=2024", "--extern", f"femacs_vm_boundary={rlib}", "-O", str(ext_rs), "-o", str(ext_bin)],
                timeout=60,
            )
            run_checked([str(ext_bin)], timeout=30)

    def group2_forbid_crates_and_boundary_tool_selftests() -> None:
        run_checked([sys.executable, str(REPO / "tools/check-unsafe-boundary.py"), "--selftest"], timeout=30)
        run_checked([sys.executable, str(REPO / "tools/test-u3-unit.py")], timeout=90)
        run_checked(
            ["cargo", "test", "--target-dir", "build/cargo", "-p", "femacs-pure", "-p", "femacs", "--lib"],
            timeout=120,
        )

    def group3_u4_structural_defect_regressions() -> None:
        # 1. Crate-level compiler lints (`deny(static_mut_refs, unsafe_op_in_unsafe_fn)` & `forbid(unsafe_code)`)
        for rel in (
            "crates/femacs-sys/src/lib.rs",
            "crates/femacs-vm/src/lib.rs",
            "crates/femacs-vm/src/host_runtime.rs",
            "crates/femacs-host/src/bin/temacs.rs",
            "xtask/src/main.rs",
        ):
            txt = (REPO / rel).read_text(encoding="utf-8")
            assert "deny(static_mut_refs, unsafe_op_in_unsafe_fn)" in txt, f"missing deny gate in {rel}"

        for rel in (
            "crates/femacs-pure/src/lib.rs",
            "crates/femacs-vm/src/safe_domain.rs",
            "crates/femacs-host/src/lib.rs",
            "cli/src/lib.rs",
            "cli/src/bin/fe.rs",
            "cli/src/bin/femacs.rs",
            "packaging/src/lib.rs",
            "packaging/src/main.rs",
            "lib/include/stdcountof.rs",
            "src/include/macuvs.rs",
        ):
            txt = (REPO / rel).read_text(encoding="utf-8")
            assert "#![forbid(unsafe_code)]" in txt, f"missing #![forbid(unsafe_code)] in {rel}"

        # 2. `SaStackUnwindGuard::drop -> sa_stack_frame_exit` dead-frame scrub (`P-172`-preserving)
        lisp_rs = (REPO / "src/include/lisp.rs").read_text(encoding="utf-8")
        alloc_rs = (REPO / "src/alloc.rs").read_text(encoding="utf-8")
        assert "sa_stack_frame_exit(self.mark, self.depth)" in lisp_rs
        assert "pub unsafe extern \"C-unwind\" fn sa_stack_frame_exit" in alloc_rs

        # 3. `C-unwind` specpdl callbacks in `runtime_gc_observation.rs` and `runtime_gc_collector.rs`
        obs_rs = (REPO / "src/runtime_gc_observation.rs").read_text(encoding="utf-8")
        col_rs = (REPO / "src/runtime_gc_collector.rs").read_text(encoding="utf-8")
        assert 'unsafe extern "C-unwind" fn abort_cycle' in obs_rs
        assert 'unsafe extern "C-unwind" fn mark_snapshot' in obs_rs
        assert 'unsafe extern "C-unwind" fn release_snapshot' in obs_rs
        assert 'unsafe extern "C-unwind" fn abort_cycle' in col_rs
        assert 'unsafe extern "C-unwind" fn mark_status' in col_rs
        assert 'unsafe extern "C-unwind" fn release_status' in col_rs

        # 4. `have_native_comp` symbol guard in `crates/femacs-vm/src/state.rs`
        state_rs = (REPO / "crates/femacs-vm/src/state.rs").read_text(encoding="utf-8")
        assert '#[cfg(feature = "have_native_comp")]' in state_rs

    run_group("1_isolated_roots_and_stdcountof_rust_tests", group1_isolated_roots_and_stdcountof_rust_tests)
    run_group("2_forbid_crates_and_boundary_tool_selftests", group2_forbid_crates_and_boundary_tool_selftests)
    run_group("3_u4_structural_defect_regressions", group3_u4_structural_defect_regressions)
    run_group("4_external_safe_caller_boundary_gate", group4_external_safe_caller_boundary_gate)

    print(
        json.dumps(
            {
                "suite": "U4_protective_unit_tests",
                "groups_passed": len(groups),
                "groups": groups,
                "wall_seconds": round(time.monotonic() - t0_all, 4),
            },
            indent=2,
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
