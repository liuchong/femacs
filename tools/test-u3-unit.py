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

    pub type Lisp_Object = *mut c_void;
    pub type nonlocal_exit = c_int;
    pub const NONLOCAL_EXIT_SIGNAL: nonlocal_exit = 0;
    pub const NONLOCAL_EXIT_THROW: nonlocal_exit = 1;

    #[repr(C)]
    pub struct handler {
        pub dummy: usize,
    }

    pub static mut current_thread: *mut c_void = core::ptr::null_mut();

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

    static SERIAL: Mutex<()> = Mutex::new(());

    fn fake_obj(tag: usize) -> Lisp_Object {
        tag as Lisp_Object
    }

    fn collect_marked(thread: *mut c_void) -> Vec<usize> {
        let mut out = Vec::new();
        mark_thread_explicit_roots(thread, |obj| out.push(obj as usize));
        out
    }

    #[test]
    fn catch_vm_nonlocal_catches_signal_and_throw_and_clears_inflight() {
        let _lock = SERIAL.lock().unwrap();
        let thread = 0x1000usize as *mut c_void;
        unsafe { current_thread = thread };

        let mut h1 = handler { dummy: 1 };
        let mut h2 = handler { dummy: 2 };

        assert_eq!(NonLocalExitKind::from_raw(NONLOCAL_EXIT_SIGNAL), NonLocalExitKind::Signal);
        assert_eq!(NonLocalExitKind::from_raw(NONLOCAL_EXIT_THROW), NonLocalExitKind::Throw);
        assert_eq!(NonLocalExitKind::Signal.to_raw(), NONLOCAL_EXIT_SIGNAL);
        assert_eq!(NonLocalExitKind::Throw.to_raw(), NONLOCAL_EXIT_THROW);

        let res_sig: Result<usize, NonLocalExit> = catch_vm_nonlocal(|| {
            raise_vm_nonlocal(NonLocalExit {
                kind: NonLocalExitKind::Signal,
                target: &raw mut h1,
                value: fake_obj(0xAA01),
            })
        });
        let err_sig = res_sig.expect_err("expected Signal NonLocalExit");
        assert_eq!(err_sig.kind, NonLocalExitKind::Signal);
        assert_eq!(err_sig.target, &raw mut h1);
        assert_eq!(err_sig.value as usize, 0xAA01);
        assert!(collect_marked(thread).is_empty(), "inflight root must clear once caught");

        let res_thr: Result<usize, NonLocalExit> = catch_vm_nonlocal(|| {
            raise_vm_nonlocal(NonLocalExit {
                kind: NonLocalExitKind::Throw,
                target: &raw mut h2,
                value: fake_obj(0xBB02),
            })
        });
        let err_thr = res_thr.expect_err("expected Throw NonLocalExit");
        assert_eq!(err_thr.kind, NonLocalExitKind::Throw);
        assert_eq!(err_thr.target, &raw mut h2);
        assert_eq!(err_thr.value as usize, 0xBB02);
        assert!(collect_marked(thread).is_empty());

        cleanup_thread_state(thread);
        unsafe { current_thread = core::ptr::null_mut() };
    }

    #[test]
    fn catch_vm_nonlocal_repropagates_genuine_rust_panic() {
        let _lock = SERIAL.lock().unwrap();
        let thread = 0x2000usize as *mut c_void;
        unsafe { current_thread = thread };

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

        cleanup_thread_state(thread);
        unsafe { current_thread = core::ptr::null_mut() };
    }

    #[test]
    fn scoped_lisp_root_lifo_and_unwind_truncation() {
        let _lock = SERIAL.lock().unwrap();
        let thread = 0x3000usize as *mut c_void;
        unsafe { current_thread = thread };
        let mut h = handler { dummy: 3 };
        let (rc0, rd0) = scoped_root_counts();

        {
            let mut outer = ScopedLispRoot::new(&[fake_obj(11), fake_obj(22)]);
            assert_eq!(collect_marked(thread), vec![11, 22]);
            outer.set(1, fake_obj(220));
            assert_eq!(collect_marked(thread), vec![11, 220]);

            let unwound = catch_vm_nonlocal(|| {
                let _inner = ScopedLispRoot::new(&[fake_obj(33), fake_obj(44)]);
                assert_eq!(collect_marked(thread), vec![11, 220, 33, 44]);
                raise_vm_nonlocal(NonLocalExit {
                    kind: NonLocalExitKind::Throw,
                    target: &raw mut h,
                    value: fake_obj(999),
                });
            });
            assert!(unwound.is_err());
            // Inner root must have truncated cleanly during unwind while outer remains intact!
            assert_eq!(collect_marked(thread), vec![11, 220]);
        }
        assert!(collect_marked(thread).is_empty());
        let (rc1, rd1) = scoped_root_counts();
        assert_eq!(rc1 - rc0, 2);
        assert_eq!(rd1 - rd0, 2);

        cleanup_thread_state(thread);
        unsafe { current_thread = core::ptr::null_mut() };
    }

    #[test]
    fn host_drop_guard_exact_once_across_reexit_and_c_probe() {
        let _lock = SERIAL.lock().unwrap();
        let thread = 0x4000usize as *mut c_void;
        unsafe { current_thread = thread };
        let mut h_inner = handler { dummy: 4 };
        let mut h_outer = handler { dummy: 5 };
        let (hc0, hd0) = host_drop_counts();

        let outer = catch_vm_nonlocal(|| {
            let _g_outer = HostDropGuard::new();
            let inner = catch_vm_nonlocal(|| {
                let _g_inner = HostDropGuard::new();
                raise_vm_nonlocal(NonLocalExit {
                    kind: NonLocalExitKind::Signal,
                    target: &raw mut h_inner,
                    value: fake_obj(1),
                });
            });
            assert!(inner.is_err());
            resume_vm_nonlocal(NonLocalExit {
                kind: NonLocalExitKind::Throw,
                target: &raw mut h_outer,
                value: fake_obj(2),
            });
        });
        assert_eq!(outer.unwrap_err().value as usize, 2);

        let ret = unsafe { femacs_vm_call0_with_host_drop_guard(fake_obj(77)) };
        assert_eq!(ret as usize, 77);

        let (mut hc1, mut hd1, mut rc1, mut rd1) = (0, 0, 0, 0);
        femacs_vm_host_drop_counts(&raw mut hc1, &raw mut hd1, &raw mut rc1, &raw mut rd1);
        assert_eq!(hc1 - hc0, 3);
        assert_eq!(hd1 - hd0, 3);
        assert_eq!(hc1, hd1);
        assert_eq!(rc1, rd1);

        cleanup_thread_state(thread);
        unsafe { current_thread = core::ptr::null_mut() };
    }

    #[test]
    fn comp_native_pending_exit_matching_propagation_and_thread_cleanup() {
        let _lock = SERIAL.lock().unwrap();
        let thread = 0x5000usize as *mut c_void;
        unsafe { current_thread = thread };
        let mut h_target = handler { dummy: 10 };
        let mut h_other = handler { dummy: 20 };

        assert_eq!(comp_native_exit_pending_for_current_thread(), 0);
        set_comp_native_exit(NonLocalExit {
            kind: NonLocalExitKind::Throw,
            target: &raw mut h_target,
            value: fake_obj(0xCAFE),
        });
        assert_eq!(comp_native_exit_pending_for_current_thread(), 1);
        assert_eq!(collect_marked(thread), vec![0xCAFE]);

        // Non-matching handler must NOT consume the pending exit.
        assert_eq!(consume_comp_native_exit_for_handler(&raw mut h_other), 0);
        assert_eq!(comp_native_exit_pending_for_current_thread(), 1);

        // Matching handler consumes and clears the pending exit.
        assert_eq!(consume_comp_native_exit_for_handler(&raw mut h_target), 1);
        assert_eq!(comp_native_exit_pending_for_current_thread(), 0);
        assert!(collect_marked(thread).is_empty());

        // Unconsumed pending exit converts back to a VM NonLocalExit unwind on boundary check.
        set_comp_native_exit(NonLocalExit {
            kind: NonLocalExitKind::Signal,
            target: &raw mut h_target,
            value: fake_obj(0xBEEF),
        });
        let propagated = catch_vm_nonlocal(|| {
            check_and_propagate_comp_native_exit();
        });
        let exit = propagated.expect_err("expected pending .eln exit to propagate");
        assert_eq!(exit.kind, NonLocalExitKind::Signal);
        assert_eq!(exit.value as usize, 0xBEEF);
        assert_eq!(comp_native_exit_pending_for_current_thread(), 0);

        // Finalizing a thread with an active pending exit decrements global counter cleanly.
        let dying_thread = 0x6000usize as *mut c_void;
        unsafe { current_thread = dying_thread };
        set_comp_native_exit(NonLocalExit {
            kind: NonLocalExitKind::Signal,
            target: &raw mut h_target,
            value: fake_obj(0xDEAD),
        });
        cleanup_thread_state(dying_thread);
        unsafe { current_thread = thread };
        assert_eq!(comp_native_exit_pending_for_current_thread(), 0);

        cleanup_thread_state(thread);
        unsafe { current_thread = core::ptr::null_mut() };
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
        if "prstack.stack.offset (entry_idx)" not in print_rs:
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
        run_thread_slice = thread_rs[run_thread_idx : run_thread_idx + 1800]
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
        return {"abi_version": "14", "helper_link_table_entries": actual_base}

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
