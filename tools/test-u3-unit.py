#!/usr/bin/env python3
"""U3 protective unit tests for `crates/femacs-vm/src/nonlocal.rs` and U3 boundary invariants.

Compiles `crates/femacs-vm/src/nonlocal.rs` verbatim in isolation (`rustc --test`) against
a minimal 20-line type stub to test `catch_vm_nonlocal`, genuine Rust `panic!` re-propagation,
LIFO `ScopedLispRoot` marking/unwinding, `HostDropGuard` exact-once `Drop`, and `.eln`
pending-exit handler matching/thread cleanup, plus structural regression checks for the
exact defects fixed during U3 (`flush_stack_call_func` register-spill frame lifetime,
`prstack` reallocation re-indexing, `module_signal_or_throw` `C-unwind` ABI, `run_thread`
`catch_vm_nonlocal`, and `comp.rs` status helper table alignment).
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
import tempfile
import time
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent

RUST_UNIT_HARNESS = r"""
#![allow(non_camel_case_types, non_upper_case_globals)]

pub mod common {
    use core::ffi::{c_int, c_void};
    use std::sync::atomic::{AtomicUsize, Ordering};

    pub type Lisp_Object = *mut c_void;
    pub type nonlocal_exit = c_int;
    pub const NONLOCAL_EXIT_SIGNAL: nonlocal_exit = 0;
    pub const NONLOCAL_EXIT_THROW: nonlocal_exit = 1;

    #[repr(C)]
    pub struct handler {
        pub dummy: usize,
    }

    pub struct ThreadGlobal;
    static THREAD_PTR: AtomicUsize = AtomicUsize::new(0);

    impl ThreadGlobal {
        pub fn get(&self) -> *mut c_void {
            THREAD_PTR.load(Ordering::Relaxed) as *mut c_void
        }
        pub fn set(&self, v: *mut c_void) {
            THREAD_PTR.store(v as usize, Ordering::Relaxed);
        }
    }

    pub static current_thread: ThreadGlobal = ThreadGlobal;

    #[inline]
    pub const fn lisp_h_Qnil() -> Lisp_Object {
        core::ptr::null_mut()
    }
}

pub mod eval {
    use crate::common::Lisp_Object;
    pub unsafe extern "C-unwind" fn Ffuncall(
        nargs: isize,
        args: *mut Lisp_Object,
    ) -> Lisp_Object {
        assert!(nargs >= 1);
        unsafe { *args }
    }
}

pub mod roots {
    use core::cell::Cell;
    use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

    static NEXT_TID: AtomicU64 = AtomicU64::new(1);
    static OWNER_KEY: AtomicUsize = AtomicUsize::new(0);
    static OWNER_TID: AtomicU64 = AtomicU64::new(0);

    std::thread_local! {
        static TID: u64 = NEXT_TID.fetch_add(1, Ordering::Relaxed);
        static TLS_KEY: Cell<usize> = const { Cell::new(0) };
    }

    pub(crate) unsafe fn bind_vm_lock_owner(thread_key: usize) {
        let tid = TID.with(|t| *t);
        TLS_KEY.with(|k| k.set(thread_key));
        OWNER_TID.store(tid, Ordering::Release);
        OWNER_KEY.store(thread_key, Ordering::Release);
    }

    pub(crate) unsafe fn unbind_vm_lock_owner() {
        TLS_KEY.with(|k| k.set(0));
        OWNER_KEY.store(0, Ordering::Release);
        OWNER_TID.store(0, Ordering::Release);
    }

    pub(crate) fn tls_vm_lock_matches(thread_key: usize) -> bool {
        thread_key != 0 && TLS_KEY.with(|k| k.get()) == thread_key
    }

    pub(crate) fn is_current_os_thread_vm_lock_owner() -> bool {
        let tid = TID.with(|t| *t);
        let owner_key = OWNER_KEY.load(Ordering::Acquire);
        owner_key != 0
            && OWNER_TID.load(Ordering::Acquire) == tid
            && TLS_KEY.with(|k| k.get()) == owner_key
    }

    pub(crate) fn is_vm_lock_owner(thread_key: usize) -> bool {
        let tid = TID.with(|t| *t);
        thread_key != 0
            && crate::common::current_thread.get() as usize == thread_key
            && OWNER_KEY.load(Ordering::Acquire) == thread_key
            && OWNER_TID.load(Ordering::Acquire) == tid
            && TLS_KEY.with(|k| k.get()) == thread_key
    }
}

#[path = "__NONLOCAL_RS_PATH__"]
pub mod nonlocal;

#[cfg(test)]
mod tests {
    use core::ffi::c_void;
    use std::panic::{AssertUnwindSafe, catch_unwind};
    use std::sync::Mutex;

    use crate::common::{
        Lisp_Object, NONLOCAL_EXIT_SIGNAL, NONLOCAL_EXIT_THROW, current_thread, handler,
    };
    use crate::nonlocal::{
        HostDropGuard, NonLocalExit, NonLocalExitKind, ScopedLispRoot, catch_vm_nonlocal,
        check_and_propagate_comp_native_exit, cleanup_thread_state,
        comp_native_exit_pending_for_current_thread, consume_comp_native_exit_for_handler,
        femacs_vm_call0_with_host_drop_guard, femacs_vm_host_drop_counts, host_drop_counts,
        mark_thread_explicit_roots, raise_vm_nonlocal, resume_vm_nonlocal, scoped_root_counts,
        set_comp_native_exit,
    };
    use crate::roots::{bind_vm_lock_owner, unbind_vm_lock_owner};

    static SERIAL: Mutex<()> = Mutex::new(());

    fn fake_obj(tag: usize) -> Lisp_Object {
        tag as Lisp_Object
    }

    fn set_thread(id: usize) -> *mut c_void {
        let ptr = id as *mut c_void;
        current_thread.set(ptr);
        if id != 0 {
            // SAFETY: serialized test owns simulated VM lock.
            unsafe { bind_vm_lock_owner(id) };
        } else {
            // SAFETY: releasing simulated VM lock.
            unsafe { unbind_vm_lock_owner() };
        }
        ptr
    }

    fn collect_marked(thread: *mut c_void) -> Vec<usize> {
        let mut out = Vec::new();
        // SAFETY: called under serialized test VM lock.
        unsafe { mark_thread_explicit_roots(thread, |obj| out.push(obj as usize)) };
        out
    }

    #[test]
    fn catch_vm_nonlocal_catches_signal_and_throw_and_clears_inflight() {
        let _lock = SERIAL.lock().unwrap();
        let thread = set_thread(0x1000);

        let mut h1 = handler { dummy: 1 };
        let mut h2 = handler { dummy: 2 };

        assert_eq!(NonLocalExitKind::from_raw(NONLOCAL_EXIT_SIGNAL), NonLocalExitKind::Signal);
        assert_eq!(NonLocalExitKind::from_raw(NONLOCAL_EXIT_THROW), NonLocalExitKind::Throw);
        assert_eq!(NonLocalExitKind::Signal.to_raw(), NONLOCAL_EXIT_SIGNAL);
        assert_eq!(NonLocalExitKind::Throw.to_raw(), NONLOCAL_EXIT_THROW);

        let res_sig: Result<usize, NonLocalExit> = catch_vm_nonlocal(|| {
            // SAFETY: isolated unit test inside `catch_vm_nonlocal`.
            unsafe {
                raise_vm_nonlocal(NonLocalExit {
                    kind: NonLocalExitKind::Signal,
                    target: &raw mut h1,
                    value: fake_obj(0xAA01),
                })
            }
        });
        let err_sig = res_sig.expect_err("expected Signal NonLocalExit");
        assert_eq!(err_sig.kind, NonLocalExitKind::Signal);
        assert_eq!(err_sig.target, &raw mut h1);
        assert_eq!(err_sig.value as usize, 0xAA01);
        assert!(collect_marked(thread).is_empty(), "inflight root must clear once caught");

        let res_thr: Result<usize, NonLocalExit> = catch_vm_nonlocal(|| {
            // SAFETY: isolated unit test inside `catch_vm_nonlocal`.
            unsafe {
                raise_vm_nonlocal(NonLocalExit {
                    kind: NonLocalExitKind::Throw,
                    target: &raw mut h2,
                    value: fake_obj(0xBB02),
                })
            }
        });
        let err_thr = res_thr.expect_err("expected Throw NonLocalExit");
        assert_eq!(err_thr.kind, NonLocalExitKind::Throw);
        assert_eq!(err_thr.target, &raw mut h2);
        assert_eq!(err_thr.value as usize, 0xBB02);
        assert!(collect_marked(thread).is_empty());

        // SAFETY: test cleanup under VM lock.
        unsafe { cleanup_thread_state(thread) };
        set_thread(0);
    }

    #[test]
    fn catch_vm_nonlocal_repropagates_genuine_rust_panic() {
        let _lock = SERIAL.lock().unwrap();
        let thread = set_thread(0x2000);

        let outer = catch_unwind(AssertUnwindSafe(|| {
            let _ = catch_vm_nonlocal(|| -> usize {
                panic!("genuine-rust-bug-not-lisp-signal");
            });
        }));
        let payload = outer.expect_err("genuine panic must not be swallowed by catch_vm_nonlocal");
        let msg = payload
            .downcast_ref::<&'static str>()
            .copied()
            .unwrap_or("");
        assert_eq!(msg, "genuine-rust-bug-not-lisp-signal");

        // SAFETY: test cleanup under VM lock.
        unsafe { cleanup_thread_state(thread) };
        set_thread(0);
    }

    #[test]
    fn scoped_lisp_root_lifo_and_unwind_truncation() {
        let _lock = SERIAL.lock().unwrap();
        let thread = set_thread(0x3000);
        let mut h = handler { dummy: 3 };
        let (rc0, rd0) = scoped_root_counts();

        {
            // SAFETY: fake tagged words for isolated unit test.
            let mut outer = unsafe { ScopedLispRoot::new(&[fake_obj(11), fake_obj(22)]) };
            assert_eq!(collect_marked(thread), vec![11, 22]);
            // SAFETY: fake tagged word.
            unsafe { outer.set(1, fake_obj(220)) };
            assert_eq!(collect_marked(thread), vec![11, 220]);

            let unwound = catch_vm_nonlocal(|| {
                // SAFETY: fake tagged words.
                let _inner = unsafe { ScopedLispRoot::new(&[fake_obj(33), fake_obj(44)]) };
                assert_eq!(collect_marked(thread), vec![11, 220, 33, 44]);
                // SAFETY: inside `catch_vm_nonlocal`.
                unsafe {
                    raise_vm_nonlocal(NonLocalExit {
                        kind: NonLocalExitKind::Throw,
                        target: &raw mut h,
                        value: fake_obj(999),
                    });
                }
            });
            assert!(unwound.is_err());
            // Inner root must have released cleanly during unwind while outer remains intact!
            assert_eq!(collect_marked(thread), vec![11, 220]);
        }
        assert!(collect_marked(thread).is_empty());
        let (rc1, rd1) = scoped_root_counts();
        assert_eq!(rc1 - rc0, 2);
        assert_eq!(rd1 - rd0, 2);

        // SAFETY: test cleanup under VM lock.
        unsafe { cleanup_thread_state(thread) };
        set_thread(0);
    }

    #[test]
    fn host_drop_guard_exact_once_across_reexit_and_c_probe() {
        let _lock = SERIAL.lock().unwrap();
        let thread = set_thread(0x4000);
        let mut h_inner = handler { dummy: 4 };
        let mut h_outer = handler { dummy: 5 };
        let (hc0, hd0) = host_drop_counts();

        let outer = catch_vm_nonlocal(|| {
            let _g_outer = HostDropGuard::new();
            let inner = catch_vm_nonlocal(|| {
                let _g_inner = HostDropGuard::new();
                // SAFETY: inside `catch_vm_nonlocal`.
                unsafe {
                    raise_vm_nonlocal(NonLocalExit {
                        kind: NonLocalExitKind::Signal,
                        target: &raw mut h_inner,
                        value: fake_obj(1),
                    });
                }
            });
            assert!(inner.is_err());
            // SAFETY: inside `catch_vm_nonlocal`.
            unsafe {
                resume_vm_nonlocal(NonLocalExit {
                    kind: NonLocalExitKind::Throw,
                    target: &raw mut h_outer,
                    value: fake_obj(2),
                });
            }
        });
        assert_eq!(outer.unwrap_err().value as usize, 2);

        let ret = unsafe { femacs_vm_call0_with_host_drop_guard(fake_obj(77)) };
        assert_eq!(ret as usize, 77);

        let (mut hc1, mut hd1, mut rc1, mut rd1) = (0, 0, 0, 0);
        // SAFETY: out pointers are valid writable locals under active lock.
        unsafe {
            femacs_vm_host_drop_counts(&raw mut hc1, &raw mut hd1, &raw mut rc1, &raw mut rd1);
        }
        assert_eq!(hc1 - hc0, 3);
        assert_eq!(hd1 - hd0, 3);
        assert_eq!(hc1, hd1);
        assert_eq!(rc1, rd1);

        // SAFETY: test cleanup under VM lock.
        unsafe { cleanup_thread_state(thread) };
        set_thread(0);
    }

    #[test]
    fn comp_native_pending_exit_matching_propagation_and_thread_cleanup() {
        let _lock = SERIAL.lock().unwrap();
        let thread = set_thread(0x5000);
        let mut h_target = handler { dummy: 10 };
        let mut h_other = handler { dummy: 20 };

        assert_eq!(comp_native_exit_pending_for_current_thread(), 0);
        // SAFETY: isolated unit test on active owner thread.
        unsafe {
            set_comp_native_exit(NonLocalExit {
                kind: NonLocalExitKind::Throw,
                target: &raw mut h_target,
                value: fake_obj(0xCAFE),
            });
        }
        assert_eq!(comp_native_exit_pending_for_current_thread(), 1);
        assert_eq!(collect_marked(thread), vec![0xCAFE]);

        // Non-matching handler must NOT consume the pending exit.
        assert_eq!(unsafe { consume_comp_native_exit_for_handler(&raw mut h_other) }, 0);
        assert_eq!(comp_native_exit_pending_for_current_thread(), 1);

        // Matching handler consumes and clears the pending exit.
        assert_eq!(unsafe { consume_comp_native_exit_for_handler(&raw mut h_target) }, 1);
        assert_eq!(comp_native_exit_pending_for_current_thread(), 0);
        assert!(collect_marked(thread).is_empty());

        // Unconsumed pending exit converts back to a VM NonLocalExit unwind on boundary check.
        // SAFETY: isolated unit test on active owner thread.
        unsafe {
            set_comp_native_exit(NonLocalExit {
                kind: NonLocalExitKind::Signal,
                target: &raw mut h_target,
                value: fake_obj(0xBEEF),
            });
        }
        let propagated = catch_vm_nonlocal(|| {
            // SAFETY: inside `catch_vm_nonlocal`.
            unsafe { check_and_propagate_comp_native_exit() };
        });
        let exit = propagated.expect_err("expected pending .eln exit to propagate");
        assert_eq!(exit.kind, NonLocalExitKind::Signal);
        assert_eq!(exit.value as usize, 0xBEEF);
        assert_eq!(comp_native_exit_pending_for_current_thread(), 0);

        // Finalizing a thread with an active pending exit decrements global counter cleanly.
        let dying_thread = set_thread(0x6000);
        // SAFETY: active thread `0x6000`.
        unsafe {
            set_comp_native_exit(NonLocalExit {
                kind: NonLocalExitKind::Signal,
                target: &raw mut h_target,
                value: fake_obj(0xDEAD),
            });
            cleanup_thread_state(dying_thread);
        }
        set_thread(0x5000);
        assert_eq!(comp_native_exit_pending_for_current_thread(), 0);

        // SAFETY: test cleanup under VM lock.
        unsafe { cleanup_thread_state(thread) };
        set_thread(0);
    }

    #[test]
    fn scoped_lisp_root_out_of_order_drop_preserves_inner_and_overflow_roots() {
        let _lock = SERIAL.lock().unwrap();
        let thread = set_thread(0x7000);
        // SAFETY: test owns VM lock.
        unsafe { cleanup_thread_state(thread) };

        // SAFETY: fake tagged words for isolated unit test.
        let outer = unsafe { ScopedLispRoot::new(&[fake_obj(1), fake_obj(2)]) };
        let mut inner_big = unsafe {
            ScopedLispRoot::new(&[
                fake_obj(10),
                fake_obj(11),
                fake_obj(12),
                fake_obj(13),
                fake_obj(14),
                fake_obj(15),
            ])
        };
        assert_eq!(collect_marked(thread), vec![1, 2, 10, 11, 12, 13, 14, 15]);

        // Drop `outer` BEFORE `inner_big`! Must keep all 6 `inner_big` elements rooted.
        drop(outer);
        assert_eq!(collect_marked(thread), vec![10, 11, 12, 13, 14, 15]);

        // SAFETY: fake tagged word.
        unsafe { inner_big.set(4, fake_obj(140)) };
        // SAFETY: fake tagged word.
        let reused = unsafe { ScopedLispRoot::new(&[fake_obj(20)]) };
        assert_eq!(collect_marked(thread), vec![20, 10, 11, 12, 13, 140, 15]);

        drop(reused);
        drop(inner_big);
        assert!(collect_marked(thread).is_empty());

        // SAFETY: test cleanup under VM lock.
        unsafe { cleanup_thread_state(thread) };
        set_thread(0);
    }

    #[test]
    fn scoped_lisp_root_rejects_non_owner_os_thread() {
        let _lock = SERIAL.lock().unwrap();
        let thread = set_thread(0x8000);
        // SAFETY: test owns VM lock.
        unsafe { cleanup_thread_state(thread) };

        let res = std::thread::spawn(|| {
            catch_unwind(AssertUnwindSafe(|| {
                // SAFETY: testing rejection of non-owner thread.
                let _bad = unsafe { ScopedLispRoot::new(&[fake_obj(99)]) };
            }))
        })
        .join()
        .expect("join thread");
        assert!(res.is_err(), "non-owner OS thread ScopedLispRoot::new must panic");
        assert!(collect_marked(thread).is_empty());

        // SAFETY: test cleanup under VM lock.
        unsafe { cleanup_thread_state(thread) };
        set_thread(0);
    }

    #[test]
    fn cleanup_mark_and_counters_reject_non_owner_os_thread() {
        let _lock = SERIAL.lock().unwrap();
        let thread = set_thread(0x9000);
        // SAFETY: test owns VM lock.
        unsafe { cleanup_thread_state(thread) };
        // SAFETY: fake tagged word.
        let live_root = unsafe { ScopedLispRoot::new(&[fake_obj(303)]) };
        assert_eq!(collect_marked(thread), vec![303]);

        let th_addr = thread as usize;
        let (clean_err, mark_err, cnt_err) = std::thread::spawn(move || {
            let foreign = th_addr as *mut c_void;
            let c1 = catch_unwind(AssertUnwindSafe(|| unsafe { cleanup_thread_state(foreign) }));
            let c2 = catch_unwind(AssertUnwindSafe(|| unsafe {
                mark_thread_explicit_roots(foreign, |_| {})
            }));
            let c3 = catch_unwind(AssertUnwindSafe(|| {
                let _ = scoped_root_counts();
            }));
            (c1.is_err(), c2.is_err(), c3.is_err())
        })
        .join()
        .expect("join thread");

        assert!(clean_err, "non-owner cleanup_thread_state must panic");
        assert!(mark_err, "non-owner mark_thread_explicit_roots must panic");
        assert!(cnt_err, "non-owner scoped_root_counts must panic");
        assert_eq!(collect_marked(thread), vec![303]);

        drop(live_root);
        // SAFETY: test cleanup under VM lock.
        unsafe { cleanup_thread_state(thread) };
        set_thread(0);
    }
}
"""


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--json-out", type=Path, help="Optional JSON output path")
    args = ap.parse_args()

    cases: list[dict] = []
    failures = 0

    def record_case(name: str, fn) -> None:
        nonlocal failures
        t0 = time.monotonic()
        try:
            detail = fn()
            dt = time.monotonic() - t0
            cases.append({"id": name, "status": "passed", "seconds": round(dt, 6), "detail": detail})
            print(f"UNIT {name} status=passed seconds={dt:.6f}")
        except Exception as exc:
            dt = time.monotonic() - t0
            failures += 1
            cases.append({"id": name, "status": "failed", "seconds": round(dt, 6), "error": str(exc)})
            print(f"UNIT {name} status=failed seconds={dt:.6f} error={exc}", file=sys.stderr)

    def test_nonlocal_rs_component() -> dict:
        with tempfile.TemporaryDirectory(prefix="u3-unit-") as tmp:
            work = Path(tmp)
            harness_rs = work / "harness.rs"
            bin_path = work / "nonlocal_unit_test"
            nonlocal_path = (REPO / "crates/femacs-vm/src/nonlocal.rs").resolve()
            harness_rs.write_text(
                RUST_UNIT_HARNESS.replace("__NONLOCAL_RS_PATH__", str(nonlocal_path)),
                encoding="utf-8",
            )
            comp = subprocess.run(
                [
                    "rustc",
                    "--edition=2024",
                    "--test",
                    "-O",
                    str(harness_rs),
                    "-o",
                    str(bin_path),
                ],
                capture_output=True,
                text=True,
                timeout=60.0,
                check=False,
            )
            if comp.returncode != 0:
                raise RuntimeError(f"rustc --test failed:\n{comp.stderr}")

            # Discover and time each Rust unit test individually for per-case timing discipline.
            listing = subprocess.run(
                [str(bin_path), "--list", "--format=terse"],
                capture_output=True,
                text=True,
                timeout=15.0,
                check=True,
            )
            test_names = [
                line.removesuffix(": test").strip()
                for line in listing.stdout.splitlines()
                if line.endswith(": test")
            ]
            if len(test_names) < 5:
                raise AssertionError(f"expected >= 5 unit tests, found {test_names!r}")

            per_test: dict[str, float] = {}
            for tname in test_names:
                t_start = time.monotonic()
                run = subprocess.run(
                    [str(bin_path), "--exact", tname, "--nocapture"],
                    capture_output=True,
                    text=True,
                    timeout=15.0,
                    check=False,
                )
                elapsed = time.monotonic() - t_start
                if run.returncode != 0:
                    raise AssertionError(
                        f"unit test `{tname}` failed:\n{run.stdout}\n{run.stderr}"
                    )
                per_test[tname] = round(elapsed, 6)
                print(f"  RUST_UNIT {tname} status=passed seconds={elapsed:.6f}")
            return {"rust_unit_tests": per_test}

    def test_gc_register_spill_and_print_guards() -> dict:
        lisp_rs = (REPO / "src/include/lisp.rs").read_text(encoding="utf-8")
        alloc_rs = (REPO / "src/alloc.rs").read_text(encoding="utf-8")
        print_rs = (REPO / "src/print.rs").read_text(encoding="utf-8")

        if "#[inline(never)]\npub fn flush_stack_call_func" not in lisp_rs:
            raise AssertionError("flush_stack_call_func must remain #[inline(never)]")
        idx = lisp_rs.index("pub fn flush_stack_call_func")
        body = lisp_rs[idx : idx + 900]
        if "setjmp_for_barrier" not in body or "flush_stack_call_func1" not in body:
            raise AssertionError(
                "flush_stack_call_func must spill registers via setjmp_for_barrier in its own live frame"
            )
        if body.index("setjmp_for_barrier (") > body.index("flush_stack_call_func1 ("):
            raise AssertionError("setjmp_for_barrier call must precede flush_stack_call_func1 call")

        if '#[inline(never)]\n#[unsafe(no_mangle)]\npub unsafe extern "C-unwind" fn flush_stack_call_func1' not in alloc_rs:
            raise AssertionError("flush_stack_call_func1 must remain #[inline(never)]")

        for guard in ("NestedPrintDepthGuard", "PpStackGuard", "PrintStackGuard"):
            if f"impl Drop for {guard}" not in print_rs:
                raise AssertionError(f"src/print.rs missing `impl Drop for {guard}`")
        if "prstack.stack.offset (entry_idx)" not in print_rs and "(*prstack.as_mut_ptr ()).stack.offset (entry_idx)" not in print_rs:
            raise AssertionError("src/print.rs must re-derive `prstack.stack.offset(entry_idx)`")
        return {"guards_verified": 5}

    def test_ffi_and_comp_abi_invariants() -> dict:
        mod_rs = (REPO / "src/emacs-module.rs").read_text(encoding="utf-8")
        thread_rs = (REPO / "src/thread.rs").read_text(encoding="utf-8")
        comp_rs = (REPO / "src/comp.rs").read_text(encoding="utf-8")

        if 'unsafe extern "C-unwind" fn module_signal_or_throw' not in mod_rs:
            raise AssertionError(
                "module_signal_or_throw must be `extern \"C-unwind\"` so Fsignal/Fthrow can unwind"
            )
        if 'unsafe extern "C" fn module_signal_or_throw' in mod_rs:
            raise AssertionError("module_signal_or_throw regressed to `extern \"C\"`")

        run_thread_idx = thread_rs.index('unsafe extern "C" fn run_thread')
        run_thread_slice = thread_rs[run_thread_idx : run_thread_idx + 2400]
        if "catch_vm_nonlocal" not in run_thread_slice:
            raise AssertionError("run_thread (`extern \"C\"`) must guard Lisp body with catch_vm_nonlocal")

        if 'ABI_VERSION: *const c_char = c"14".as_ptr ()' not in comp_rs:
            raise AssertionError("src/comp.rs ABI_VERSION must be \"14\" for U3 status-branch protocol")
        if "fn emit_setjmp" in comp_rs:
            raise AssertionError("src/comp.rs still contains `fn emit_setjmp`")

        declared = re.findall(r'ADD_IMPORTED!\s*\(\s*field_list,\s*(?:c"([^"]+)"\.as_ptr\s*\(\)|invoke_names)', comp_rs)
        # Expand `invoke_names` (0..=8 => 9 entries)
        expanded_count = sum(9 if item == "" else 1 for item in declared)
        m_len = re.search(r"COMP_HELPER_BASE_COUNT:\s*usize\s*=\s*(\d+)\s*;", comp_rs)
        if not m_len:
            raise AssertionError("could not parse COMP_HELPER_BASE_COUNT")
        # Under `femacs_gc_generational` an extra slot exists; base count without that feature is `expanded_count - 1`.
        expected_base = expanded_count - 1
        actual_base = int(m_len.group(1))
        if actual_base != expected_base:
            raise AssertionError(
                f"helper_link_table length ({actual_base}) != declare_runtime_imported_funcs count ({expected_base})"
            )

        # P-488 Option A boundary invariants:
        # 1) All `&mut GcContext` public primitives return `VmResult<...>` via `with_reentrant_safepoint` + `catch_vm_nonlocal`.
        vp_rs = (REPO / "crates/femacs-vm/src/vm_primitives.rs").read_text(encoding="utf-8")
        mut_prim_fns = list(
            re.finditer(
                r"pub\s+fn\s+(\w+)\s*(?:<[^>]+>)?\s*\(([^)]*)\)(?:\s*->\s*([^{]+))?\s*\{",
                vp_rs,
                re.S,
            )
        )
        checked_mut_prims = 0
        for idx, m in enumerate(mut_prim_fns):
            name, params, ret = m.group(1), m.group(2), (m.group(3) or "").strip()
            if "&mut GcContext" not in params:
                continue
            checked_mut_prims += 1
            if not ret.startswith("VmResult<"):
                raise AssertionError(
                    f"vm_primitives::{name} takes `&mut GcContext` and must return `VmResult<...>`, got `{ret}`"
                )
            next_start = mut_prim_fns[idx + 1].start() if idx + 1 < len(mut_prim_fns) else len(vp_rs)
            fn_body = vp_rs[m.end() : next_start]
            if "with_reentrant_safepoint" not in fn_body:
                raise AssertionError(f"vm_primitives::{name} must wrap VM calls in `with_reentrant_safepoint`")
            if "catch_vm_nonlocal" not in fn_body:
                raise AssertionError(f"vm_primitives::{name} must convert unwinds via `catch_vm_nonlocal`")
        if checked_mut_prims < 15:
            raise AssertionError(f"expected >= 15 `&mut GcContext` vm_primitives, checked {checked_mut_prims}")

        # 2) All `.eln` C-ABI helper functions in `src/comp.rs` remain `extern "C"` (zero unwind across `.eln`).
        expected_comp_c_helpers = [
            "comp_wrong_type_argument",
            "comp_push_handler",
            "comp_record_unwind_protect_excursion",
            "comp_helper_unwind_protect",
            "comp_helper_unbind_n",
            "comp_helper_save_restriction",
            "comp_helper_GET_SYMBOL_WITH_POSITION",
            "comp_helper_sanitizer_assert",
            "comp_record_unwind_current_buffer",
            "comp_set_internal",
            "comp_specbind",
            "comp_maybe_gc",
            "comp_maybe_quit",
            *[f"comp_invoke_subr{i}" for i in range(9)],
            "comp_invoke_subr_many",
            "comp_invoke_subr_unevalled",
            "comp_exit_pending",
            "comp_consume_exit_for_handler",
        ]
        for hname in expected_comp_c_helpers:
            if not re.search(rf'(?:unsafe\s+)?extern\s+"C"\s+fn\s+{re.escape(hname)}\b', comp_rs):
                raise AssertionError(f"src/comp.rs `.eln` helper `{hname}` must be `extern \"C\"`")
            if re.search(rf'extern\s+"C-unwind"\s+fn\s+{re.escape(hname)}\b', comp_rs):
                raise AssertionError(f"src/comp.rs `.eln` helper `{hname}` must NOT be `extern \"C-unwind\"`")

        # 3) All `emacs_env` vtable entrypoints in `src/emacs-module.rs` remain `extern "C"`.
        if re.search(r'extern\s+"C-unwind"\s+fn\s+module_(?!finalize_function\b|function_interactive_form\b|function_command_modes\b|function_arity\b|function_documentation\b|function_address\b|function_data\b|signal_or_throw\b)', mod_rs):
            raise AssertionError("emacs_env vtable `module_*` entrypoints must not be `extern \"C-unwind\"`")

        # 4) Raw-pointer C-ABI probes & lock/raw-value VM-kernel primitives require `unsafe` or `pub(crate)`.
        roots_rs = (REPO / "crates/femacs-vm/src/roots.rs").read_text(encoding="utf-8")
        nonlocal_rs = (REPO / "crates/femacs-vm/src/nonlocal.rs").read_text(encoding="utf-8")
        state_rs = (REPO / "crates/femacs-vm/src/state.rs").read_text(encoding="utf-8")
        assert 'pub unsafe extern "C" fn femacs_vm_persistent_root_alloc(' in roots_rs
        assert 'pub unsafe extern "C" fn femacs_vm_host_drop_counts(' in nonlocal_rs
        assert 'pub unsafe extern "C" fn femacs_vm_u4_root_stats(' in state_rs
        assert 'pub(crate) unsafe fn bind_vm_lock_owner(' in roots_rs
        assert 'pub(crate) unsafe fn unbind_vm_lock_owner(' in roots_rs
        assert 'pub unsafe fn from_raw(' in roots_rs

        return {
            "abi_version": "14",
            "helper_link_table_entries": actual_base,
            "vm_primitives_checked": checked_mut_prims,
            "comp_c_helpers_checked": len(expected_comp_c_helpers),
        }

    record_case("nonlocal_rs_isolated_component_tests", test_nonlocal_rs_component)
    record_case("gc_register_spill_and_print_guards", test_gc_register_spill_and_print_guards)
    record_case("ffi_and_comp_abi_invariants", test_ffi_and_comp_abi_invariants)

    summary = {
        "suite": "U3-protective-unit-tests",
        "passed": len(cases) - failures,
        "failed": failures,
        "cases": cases,
    }
    if args.json_out:
        args.json_out.parent.mkdir(parents=True, exist_ok=True)
        args.json_out.write_text(json.dumps(summary, indent=2) + "\n", encoding="utf-8")
    print(f"SUMMARY suite=U3-unit passed={summary['passed']} failed={summary['failed']}")
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
