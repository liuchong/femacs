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
        GcContext, LispVal, PersistentRoot, Rooted, RootedVec, ScopedRawSpan, UnrootedBorrow,
        bind_vm_lock_owner, cleanup_thread_u4_slab, femacs_vm_persistent_root_alloc,
        femacs_vm_persistent_root_get, femacs_vm_persistent_root_release, mark_persistent_roots,
        mark_thread_u4_slab_roots, rooting_counters, unbind_vm_lock_owner,
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
            bind_vm_lock_owner(id);
        } else {
            unbind_vm_lock_owner();
        }
        ptr
    }

    fn marked_slab(th: *mut c_void) -> Vec<usize> {
        let mut out = Vec::new();
        mark_thread_u4_slab_roots(th, |o| out.push(o as usize));
        out
    }

    fn marked_persistent() -> Vec<usize> {
        let mut out = Vec::new();
        mark_persistent_roots(|o| out.push(o as usize));
        out
    }

    #[test]
    fn rooted_handles_nogc_borrow_lifo_and_unwind_truncation() {
        let _lk = SERIAL.lock().unwrap();
        let th = set_thread(0x1000);
        cleanup_thread_u4_slab(th);

        let mut ctx = GcContext::current().expect("active thread context");
        let borrowed_sum = {
            let nogc = ctx.nogc();
            let b1 = UnrootedBorrow::new(&nogc, 19usize);
            let b2 = UnrootedBorrow::new(&nogc, 23usize);
            b1.get() + b2.get()
        };
        let after_sp = ctx.safepoint(|_| borrowed_sum);
        assert_eq!(after_sp, 42);

        let r1 = Rooted::new(LispVal::from_raw(fake_obj(11)));
        let mut r2 = Rooted::new(LispVal::from_raw(fake_obj(22)));
        assert_eq!(marked_slab(th), vec![11 << 3, 22 << 3]);

        r2.handle_mut().set(LispVal::from_raw(fake_obj(33)));
        assert_eq!(r2.handle().get(), LispVal::from_raw(fake_obj(33)));
        assert_eq!(marked_slab(th), vec![11 << 3, 33 << 3]);

        let panicked = catch_unwind(AssertUnwindSafe(|| {
            let _r3 = Rooted::new(LispVal::from_raw(fake_obj(44)));
            assert_eq!(marked_slab(th), vec![11 << 3, 33 << 3, 44 << 3]);
            panic!("unwind inner Rooted");
        }));
        assert!(panicked.is_err());
        // `_r3` must have been truncated cleanly while `r1` and `r2` survive!
        assert_eq!(marked_slab(th), vec![11 << 3, 33 << 3]);
        assert_eq!(r1.get(), LispVal::from_raw(fake_obj(11)));

        drop(r2);
        drop(r1);
        assert!(marked_slab(th).is_empty());
        cleanup_thread_u4_slab(th);
        set_thread(0);
    }

    #[test]
    fn scoped_raw_span_progressive_set_len_and_unwind() {
        let _lk = SERIAL.lock().unwrap();
        let th = set_thread(0x2000);
        cleanup_thread_u4_slab(th);

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
        cleanup_thread_u4_slab(th);
        set_thread(0);
    }

    #[test]
    fn persistent_root_slot_reuse_and_aba_generation_rejection() {
        let _lk = SERIAL.lock().unwrap();
        let before = rooting_counters();

        let mut p1 = PersistentRoot::new(LispVal::from_raw(fake_obj(501)));
        let (slot_idx_1, gen_1) = p1.raw_token();
        assert_eq!(p1.handle().get(), LispVal::from_raw(fake_obj(501)));
        assert!(marked_persistent().contains(&(501 << 3)));

        assert!(p1.release());
        assert!(!p1.release());
        let stale_get_after_release = catch_unwind(AssertUnwindSafe(|| p1.handle().get()));
        assert!(
            stale_get_after_release.is_err(),
            "stale Handle::get() after PersistentRoot release must panic"
        );
        assert!(!marked_persistent().contains(&(501 << 3)));

        // Reallocate via C-ABI entrypoint: free-list reuses `slot_idx_1` with bumped generation!
        let mut idx2: usize = usize::MAX;
        let mut gen2: u64 = 0;
        femacs_vm_persistent_root_alloc(fake_obj(502), &raw mut idx2, &raw mut gen2);
        assert_eq!(idx2, slot_idx_1);
        assert!(gen2 > gen_1);
        assert_eq!(femacs_vm_persistent_root_get(idx2, gen2), fake_obj(502));

        // Stale generation on the SAME slot index MUST still be rejected (ABA safety)!
        assert_eq!(femacs_vm_persistent_root_get(slot_idx_1, gen_1), lisp_h_Qnil());
        assert!(!femacs_vm_persistent_root_release(slot_idx_1, gen_1));
        let stale_get_after_reuse = catch_unwind(AssertUnwindSafe(|| p1.handle().get()));
        assert!(
            stale_get_after_reuse.is_err(),
            "stale Handle::get() after slot reuse must panic on generation mismatch"
        );
        assert!(marked_persistent().contains(&(502 << 3)));

        assert!(femacs_vm_persistent_root_release(idx2, gen2));
        assert!(!femacs_vm_persistent_root_release(idx2, gen2));
        assert_eq!(femacs_vm_persistent_root_get(idx2, gen2), lisp_h_Qnil());

        let after = rooting_counters();
        assert_eq!(after.persistent_live, before.persistent_live);
    }

    #[test]
    fn lisp_val_fixnum_and_nil_tag_roundtrip() {
        assert!(LispVal::nil().is_nil());
        assert_eq!(LispVal::nil().as_fixnum(), None);
        for n in [-1024isize, -1, 0, 1, 42, 1_000_000] {
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
        cleanup_thread_u4_slab(th);

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
        cleanup_thread_u4_slab(th);
        set_thread(0);
    }

    #[test]
    fn rooted_out_of_order_drop_preserves_inner_live_root_and_handle() {
        let _lk = SERIAL.lock().unwrap();
        let th = set_thread(0x4000);
        cleanup_thread_u4_slab(th);

        let outer = Rooted::new(LispVal::from_raw(fake_obj(71)));
        let mut inner = Rooted::new(LispVal::from_raw(fake_obj(72)));
        assert_eq!(marked_slab(th), vec![71 << 3, 72 << 3]);

        // Drop `outer` BEFORE `inner`! Previously `truncate(self.index)` wiped `inner` in release.
        drop(outer);
        assert_eq!(inner.get(), LispVal::from_raw(fake_obj(72)));
        assert_eq!(inner.handle().get(), LispVal::from_raw(fake_obj(72)));
        assert_eq!(marked_slab(th), vec![72 << 3]);

        inner.set(LispVal::from_raw(fake_obj(73)));
        // Free-list slot reuse must not disturb live `inner` at slot 1.
        let reused = Rooted::new(LispVal::from_raw(fake_obj(74)));
        assert_eq!(reused.get(), LispVal::from_raw(fake_obj(74)));
        assert_eq!(inner.handle().get(), LispVal::from_raw(fake_obj(73)));
        assert_eq!(marked_slab(th), vec![74 << 3, 73 << 3]);

        drop(reused);
        drop(inner);
        assert!(marked_slab(th).is_empty());
        cleanup_thread_u4_slab(th);
        set_thread(0);
    }

    #[test]
    fn rooted_vec_out_of_order_drop_preserves_other_live_spans() {
        let _lk = SERIAL.lock().unwrap();
        let th = set_thread(0x5000);
        cleanup_thread_u4_slab(th);

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
        cleanup_thread_u4_slab(th);
        set_thread(0);
    }

    #[test]
    fn rooted_new_panics_on_non_owner_os_thread() {
        let _lk = SERIAL.lock().unwrap();
        let th = set_thread(0x6000);
        cleanup_thread_u4_slab(th);

        let res = std::thread::spawn(|| {
            catch_unwind(AssertUnwindSafe(|| {
                let _bad = Rooted::new(LispVal::from_raw(fake_obj(91)));
            }))
        })
        .join()
        .expect("join thread");
        assert!(res.is_err(), "non-owner OS thread Rooted::new must panic");
        assert!(marked_slab(th).is_empty());
        cleanup_thread_u4_slab(th);
        set_thread(0);
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
            assert "9 passed; 0 failed" in proc.stdout, proc.stdout

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
            "crates/femacs-host/src/lib.rs",
            "crates/femacs-host/src/bin/temacs.rs",
            "xtask/src/main.rs",
        ):
            txt = (REPO / rel).read_text(encoding="utf-8")
            assert "deny(static_mut_refs, unsafe_op_in_unsafe_fn)" in txt, f"missing deny gate in {rel}"

        for rel in (
            "crates/femacs-pure/src/lib.rs",
            "crates/femacs-vm/src/safe_domain.rs",
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
