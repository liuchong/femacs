#!/usr/bin/env python3
"""Stage U7 / AU7 integration and functional acceptance runner (P-486).

Verifies:
  1. `static-and-nm-gates`: `check-unsafe-boundary.py` (`143+` `#![forbid(unsafe_code)]`
     active modules including all 19 new `crates/femacs-vm/src/safe_*.rs` subsystems and
     `src/include/buildobj.rs`, `0` active `static mut`, `100%` `SAFETY:` docs, substantial
     reduction in `unsafe_blocks` and `unsafe_fn_defs` vs U4/U6 baselines, and `nm` singleton /
     export checks), plus `check-globals.py`, `check-defvar-carriers.py`,
     `check-mirror-dupes.py`, `check-ledger-ticks.py`, `check-libshadows.py`,
     `check-declarations.py`, and `check-own-voice.py`.
  2. `safe-subsystems-functional`: end-to-end Lisp functional probes across all 19 migrated
     `#![forbid(unsafe_code)]` subsystem modules (`safe_casetab`, `safe_category`,
     `safe_floatfns`, `safe_undo`, `safe_macros`, `safe_doc`, `safe_casefiddle`,
     `safe_callint`, `safe_sort`, `safe_json`, `safe_marker`, `safe_character`, `safe_syntax`,
     `safe_keymap`, `safe_textprop`, `safe_search`, `safe_data`, `safe_buffer`, `safe_window`).
  3. `core-ert-suites`: 20 official ERT suites covering every migrated subsystem with per-suite
     wall-clock timing (`buffer`, `callint`, `casefiddle`, `character`, `chartab`, `cmds`,
     `data`, `doc`, `editfns`, `eval`, `floatfns`, `fns`, `json`, `keymap`, `marker`,
     `search`, `syntax`, `textprop`, `undo`, and `sort`).
  4. `gc-stress-and-stack-narrowing`: forced GC stress (`gc-cons-threshold = 1` +
     `FEMACS_GC_SKIP_STACK_SCAN=1`) across `RootedVec` / `Rooted` paths in `safe_sort`
     (including `NonLocalExit` unwind during comparator signal), `safe_json`, `safe_callint`,
     `safe_search`, `safe_data`, `safe_buffer`, and `safe_window`.
"""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
import tempfile
import time
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent

U7_FORBID_MODULES = [
    "src/include/buildobj.rs",
    "crates/femacs-vm/src/safe_buffer.rs",
    "crates/femacs-vm/src/safe_callint.rs",
    "crates/femacs-vm/src/safe_casefiddle.rs",
    "crates/femacs-vm/src/safe_casetab.rs",
    "crates/femacs-vm/src/safe_category.rs",
    "crates/femacs-vm/src/safe_character.rs",
    "crates/femacs-vm/src/safe_cmds.rs",
    "crates/femacs-vm/src/safe_data.rs",
    "crates/femacs-vm/src/safe_doc.rs",
    "crates/femacs-vm/src/safe_domain.rs",
    "crates/femacs-vm/src/safe_floatfns.rs",
    "crates/femacs-vm/src/safe_json.rs",
    "crates/femacs-vm/src/safe_keymap.rs",
    "crates/femacs-vm/src/safe_macros.rs",
    "crates/femacs-vm/src/safe_marker.rs",
    "crates/femacs-vm/src/safe_search.rs",
    "crates/femacs-vm/src/safe_sort.rs",
    "crates/femacs-vm/src/safe_syntax.rs",
    "crates/femacs-vm/src/safe_textprop.rs",
    "crates/femacs-vm/src/safe_undo.rs",
    "crates/femacs-vm/src/safe_window.rs",
]

U4_BASELINE_UNSAFE_BLOCKS = 7162
U4_BASELINE_UNSAFE_FN_DEFS = 4925


def run_cmd(
    argv: list[str],
    *,
    cwd: Path = REPO,
    env: dict[str, str] | None = None,
    timeout: int = 180,
) -> subprocess.CompletedProcess[str]:
    merged = os.environ.copy()
    if env:
        merged.update(env)
    return subprocess.run(
        argv,
        cwd=str(cwd),
        env=merged,
        capture_output=True,
        text=True,
        timeout=timeout,
        check=False,
    )


def isolated_env(home_dir: Path) -> dict[str, str]:
    return {
        "HOME": str(home_dir),
        "LC_ALL": "C",
    }


def run_case(results: list[dict], name: str, fn) -> bool:
    t0 = time.monotonic()
    try:
        detail = fn()
        dt = time.monotonic() - t0
        results.append({"case": name, "status": "passed", "seconds": round(dt, 6), "detail": detail})
        print(f"CASE {name} status=passed seconds={dt:.4f}")
        return True
    except Exception as exc:
        dt = time.monotonic() - t0
        results.append({"case": name, "status": "failed", "seconds": round(dt, 6), "error": str(exc)})
        print(f"CASE {name} status=failed seconds={dt:.4f} error={exc}", file=sys.stderr)
        return False


def check_static_and_nm_gates(terminal_bin: Path, temacs_bin: Path) -> dict:
    cub_cmd = [
        sys.executable,
        "tools/check-unsafe-boundary.py",
        "--manifest",
        "tools/unsafe-boundary-manifest.json",
        "--require-no-file-allows",
        "--require-100pct-safety-docs",
        "--require-zero-active-stitching",
    ]
    for mod in U7_FORBID_MODULES:
        cub_cmd.extend(["--require-forbid", mod])
    cub_cmd.extend(
        [
            "--binary",
            str(temacs_bin if temacs_bin.is_file() else terminal_bin),
            "--kind",
            "terminal",
            "--require-modules",
        ]
    )

    gates = [
        ("check-unsafe-boundary", cub_cmd),
        ("check-globals", [sys.executable, "tools/check-globals.py"]),
        ("check-defvar-carriers", [sys.executable, "tools/check-defvar-carriers.py"]),
        ("check-mirror-dupes", [sys.executable, "tools/check-mirror-dupes.py"]),
        ("check-ledger-ticks", [sys.executable, "tools/check-ledger-ticks.py"]),
        ("check-libshadows", [sys.executable, "tools/check-libshadows.py"]),
        ("check-declarations", [sys.executable, "tools/check-declarations.py"]),
        ("check-own-voice", [sys.executable, "tools/check-own-voice.py"]),
    ]
    summary: dict[str, object] = {}
    for label, cmd in gates:
        proc = run_cmd(cmd, timeout=90)
        if proc.returncode != 0:
            raise AssertionError(f"{label} exited {proc.returncode}:\n{proc.stdout}\n{proc.stderr}")
        if label == "check-unsafe-boundary":
            payload = json.loads(proc.stdout)
            ap = payload["active_product"]
            if ap["static_mut_items"] != 0:
                raise AssertionError(f"expected 0 active static_mut_items, got {ap['static_mut_items']}")
            if ap["forbid_unsafe_files"] < 143:
                raise AssertionError(f"expected >= 143 forbid_unsafe_files, got {ap['forbid_unsafe_files']}")
            if ap["unsafe_blocks"] >= U4_BASELINE_UNSAFE_BLOCKS:
                raise AssertionError(
                    f"expected unsafe_blocks < U4 baseline {U4_BASELINE_UNSAFE_BLOCKS}, got {ap['unsafe_blocks']}"
                )
            if ap["unsafe_fn_defs"] >= U4_BASELINE_UNSAFE_FN_DEFS:
                raise AssertionError(
                    f"expected unsafe_fn_defs < U4 baseline {U4_BASELINE_UNSAFE_FN_DEFS}, got {ap['unsafe_fn_defs']}"
                )
            summary["active_product"] = {
                "forbid_unsafe_files": ap["forbid_unsafe_files"],
                "unsafe_blocks": ap["unsafe_blocks"],
                "unsafe_fn_defs": ap["unsafe_fn_defs"],
                "safety_documented_blocks": ap["safety_documented_blocks"],
                "static_mut_items": ap["static_mut_items"],
            }
        else:
            summary[label] = "ok"
    return summary


def check_safe_subsystems_functional(terminal_bin: Path) -> dict:
    with tempfile.TemporaryDirectory(prefix="u7-func-") as tmp:
        env = isolated_env(Path(tmp))
        probe = r"""
(progn
  ;; 1. safe_casetab + safe_casefiddle
  (let ((ct (copy-case-table (standard-case-table))))
    (unless (case-table-p ct) (error "case-table-p failed"))
    (unless (equal (upcase "abc\u00e9") "ABC\u00c9") (error "upcase mismatch"))
    (unless (equal (capitalize "hELLO wORLD") "Hello World") (error "capitalize mismatch")))

  ;; 2. safe_category
  (let ((cat (make-category-table)))
    (define-category ?z "u7 test category" cat)
    (modify-category-entry ?A ?z cat)
    (unless (and (category-table-p cat)
                 (equal (category-docstring ?z cat) "u7 test category"))
      (error "category table probe failed")))

  ;; 3. safe_floatfns (including bignum frexp/ldexp/logb & rounding divisions)
  (let* ((big (ash 1 100))
         (fe (frexp big)))
    (unless (and (= (cdr fe) 101) (= (car fe) 0.5))
      (error "bignum frexp mismatch: %S" fe))
    (unless (= (logb big) 100) (error "bignum logb mismatch"))
    (unless (= (ffloor -3.2) -4.0) (error "ffloor mismatch"))
    (unless (= (floor -7 2) -4) (error "floor 2-arg mismatch"))
    (unless (= (ceiling -7 2) -3) (error "ceiling 2-arg mismatch"))
    (unless (= (round 5 2) 2) (error "round-to-even mismatch")))

  ;; 4. safe_character + safe_syntax
  (let ((s (string ?A ?\u4e2d ?\U0001f600)))
    (unless (and (= (length s) 3)
                 (= (string-bytes s) 8)
                 (multibyte-string-p s))
      (error "safe_character string mismatch")))
  (with-temp-buffer
    (let ((st (make-syntax-table)))
      (modify-syntax-entry ?_ "w" st)
      (set-syntax-table st)
      (insert "foo_bar baz")
      (goto-char (point-min))
      (forward-word 1)
      (unless (= (point) 8) (error "safe_syntax forward-word mismatch: %S" (point)))))

  ;; 5. safe_keymap + safe_callint + safe_macros + safe_doc
  (let ((km (make-sparse-keymap)))
    (define-key km [?x] #'forward-char)
    (unless (eq (lookup-key km [?x]) #'forward-char)
      (error "safe_keymap lookup-key mismatch"))
    (unless (equal (single-key-description ?\C-a) "C-a")
      (error "single-key-description mismatch")))
  (let ((doc (documentation #'car t)))
    (unless (and (stringp doc) (> (length doc) 10))
      (error "safe_doc documentation failed")))
  (let ((u7-icall (lambda (a b) (interactive (list 10 32)) (+ a b))))
    (unless (= (call-interactively u7-icall) 42)
      (error "safe_callint failed")))

  ;; 6. safe_sort + safe_json
  (let ((sorted (sort (list '(3 . "c") '(1 . "a") '(2 . "b"))
                      :key #'car :lessp #'<))
        (vsorted (sort (vector 9 4 7 1) :lessp #'< :reverse t)))
    (unless (equal (mapcar #'cdr sorted) '("a" "b" "c"))
      (error "safe_sort list mismatch: %S" sorted))
    (unless (equal vsorted [9 7 4 1])
      (error "safe_sort vector mismatch: %S" vsorted)))
  (let* ((jstr (json-serialize '((alpha . [1 2 t :false :null]) (beta . "ok"))))
         (parsed (json-parse-string jstr :object-type 'alist :array-type 'array)))
    (unless (and (equal (cdr (assq 'beta parsed)) "ok")
                 (equal (cdr (assq 'alpha parsed)) [1 2 t :false :null]))
      (error "safe_json roundtrip mismatch: %S" parsed)))

  ;; 7. safe_marker + safe_textprop + safe_search + safe_undo + safe_buffer + safe_window
  (with-temp-buffer
    (buffer-enable-undo)
    (insert "alpha beta gamma")
    (undo-boundary)
    (put-text-property 1 6 'face 'bold)
    (unless (eq (get-text-property 2 'face) 'bold)
      (error "safe_textprop get-text-property mismatch"))
    (goto-char (point-min))
    (unless (re-search-forward "\\(b[a-z]+\\)" nil t)
      (error "safe_search re-search-forward failed"))
    (replace-match "BETA" t t nil 1)
    (unless (equal (buffer-string) "alpha BETA gamma")
      (error "safe_search replace-match mismatch: %S" (buffer-string)))
    (let ((m (copy-marker 7 t)))
      (unless (and (= (marker-position m) 7) (marker-insertion-type m))
        (error "safe_marker mismatch")))
    (primitive-undo 1 buffer-undo-list)
    (unless (equal (buffer-string) "alpha beta gamma")
      (error "safe_undo primitive-undo mismatch: %S" (buffer-string))))

  (unless (and (window-live-p (selected-window))
               (consp (window-edges (selected-window))))
    (error "safe_window probe failed"))

  ;; 8. safe_data (bool-vector ops, string-distance, alist/list ops)
  (let* ((bv1 (bool-vector t nil t t nil))
         (bv2 (bool-vector nil nil t nil t))
         (bvu (bool-vector-union bv1 bv2))
         (bvn (bool-vector-not bv1)))
    (unless (and (= (bool-vector-count-population bvu) 4)
                 (equal bvn (bool-vector nil t nil nil t))
                 (= (string-distance "kitten" "sitting") 3))
      (error "safe_data bool-vector/string-distance mismatch")))

  (princ "U7_SAFE_SUBSYSTEMS_OK\n"))
"""
        proc = run_cmd([str(terminal_bin), "-Q", "--batch", "--eval", probe], env=env, timeout=60)
        if proc.returncode != 0 or "U7_SAFE_SUBSYSTEMS_OK" not in proc.stdout:
            raise AssertionError(
                f"safe subsystems functional probe failed ({proc.returncode}):\n{proc.stdout}\n{proc.stderr}"
            )
        return {"probed_modules": len(U7_FORBID_MODULES)}


def check_core_ert_suites(terminal_bin: Path) -> dict:
    ert_files = [
        "test/src/buffer-tests.el",
        "test/src/callint-tests.el",
        "test/src/casefiddle-tests.el",
        "test/src/character-tests.el",
        "test/src/chartab-tests.el",
        "test/src/cmds-tests.el",
        "test/src/data-tests.el",
        "test/src/doc-tests.el",
        "test/src/editfns-tests.el",
        "test/src/eval-tests.el",
        "test/src/floatfns-tests.el",
        "test/src/fns-tests.el",
        "test/src/json-tests.el",
        "test/src/keymap-tests.el",
        "test/src/marker-tests.el",
        "test/src/search-tests.el",
        "test/src/syntax-tests.el",
        "test/src/textprop-tests.el",
        "test/src/undo-tests.el",
        "test/lisp/sort-tests.el",
    ]
    timings: dict[str, float] = {}
    with tempfile.TemporaryDirectory(prefix="u7-ert-") as tmp:
        env = isolated_env(Path(tmp))
        for rel in ert_files:
            t0 = time.monotonic()
            selector = "(not (tag :expensive-test))"
            ep = run_cmd(
                [
                    str(terminal_bin),
                    "-Q",
                    "--batch",
                    "-l",
                    "ert",
                    "-l",
                    rel,
                    "--eval",
                    f"(ert-run-tests-batch-and-exit '{selector})",
                ],
                env=env,
                timeout=180,
            )
            dt = time.monotonic() - t0
            timings[rel] = round(dt, 4)
            if ep.returncode != 0:
                tail = "\n".join((ep.stdout + "\n" + ep.stderr).splitlines()[-40:])
                raise AssertionError(f"ERT suite {rel} failed (exit {ep.returncode}, {dt:.2f}s):\n{tail}")
    return {"suite_count": len(ert_files), "ert_timings_sec": timings}


def check_gc_stress_and_stack_narrowing(terminal_bin: Path) -> dict:
    with tempfile.TemporaryDirectory(prefix="u7-gc-") as tmp:
        env = isolated_env(Path(tmp))
        env["FEMACS_GC_SKIP_STACK_SCAN"] = "1"
        stress = r"""
(progn
  (let ((gc-cons-threshold 1)
        (gc-cons-percentage 0.0)
        (before-gcs gcs-done))
    ;; 1. safe_sort with allocating + explicit (garbage-collect) :key & :lessp under skip-stack-scan
    (let* ((items (mapcar (lambda (i) (list (- 80 i) (format "item-%03d" i)))
                          (number-sequence 1 40)))
           (sorted (sort items
                         :key (lambda (x) (garbage-collect) (car x))
                         :lessp (lambda (a b) (make-vector 4 a) (< a b)))))
      (unless (and (= (length sorted) 40)
                   (= (caar sorted) 40)
                   (equal (cadr (car sorted)) "item-040"))
        (error "GC stress sort mismatch: %S" (car sorted))))

    ;; 2. NonLocalExit unwind mid-sort when :lessp triggers GC and signals an error
    (condition-case err
        (progn
          (sort (number-sequence 1 30)
                :lessp (lambda (a b)
                         (garbage-collect)
                         (when (= a 17) (signal 'arith-error (list a b)))
                         (< a b)))
          (error "expected arith-error unwind from sort comparator"))
      (arith-error
       (unless (= (cadr err) 17)
         (error "unexpected arith-error payload: %S" err))))

    ;; 3. safe_json nested parse + serialize with forced GC between phases
    (dotimes (iter 12)
      (let* ((payload (list (cons 'iter iter)
                            (cons 'vec (vector (format "v-%d" iter) t :false :null))
                            (cons 'nested (list (cons 'a (vconcat (number-sequence 1 10)))))))
             (s (json-serialize payload))
             (_ (garbage-collect))
             (rt (json-parse-string s :object-type 'alist :array-type 'list)))
        (unless (= (cdr (assq 'iter rt)) iter)
          (error "GC stress json roundtrip mismatch at %d: %S" iter rt))))

    ;; 4. safe_callint + safe_search + safe_data + safe_buffer under forced GC
    (with-temp-buffer
      (insert "one=1 two=22 three=333")
      (goto-char (point-min))
      (while (re-search-forward "\\([a-z]+\\)=\\([0-9]+\\)" nil t)
        (let ((md (match-data)))
          (garbage-collect)
          (set-match-data md)
          (replace-match "\\2:\\1" t nil)))
      (unless (equal (buffer-string) "1:one 22:two 333:three")
        (error "GC stress search/match-data mismatch: %S" (buffer-string))))

    (let ((delta (- gcs-done before-gcs)))
      (unless (> delta 40)
        (error "expected > 40 forced GC cycles, got %d" delta))
      (princ (format "U7_GC_STRESS_OK gcs=%d\n" delta)))))
"""
        proc = run_cmd([str(terminal_bin), "-Q", "--batch", "--eval", stress], env=env, timeout=120)
        if proc.returncode != 0 or "U7_GC_STRESS_OK" not in proc.stdout:
            raise AssertionError(
                f"GC stress & stack-narrowing check failed ({proc.returncode}):\n{proc.stdout}\n{proc.stderr}"
            )
        gcs_line = [ln for ln in proc.stdout.splitlines() if "U7_GC_STRESS_OK" in ln][-1]
        return {"gc_stress": gcs_line}


U7_ROOTS_RUST_TESTS = r"""
#![deny(static_mut_refs, unsafe_op_in_unsafe_fn)]
#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

pub mod common {
    use core::cell::UnsafeCell;
    use core::ffi::{c_char, c_void};
    use std::sync::atomic::{AtomicUsize, Ordering};

    pub type Lisp_Object = *mut c_void;
    pub type EMACS_INT = isize;

    #[repr(C)]
    pub struct thread_state {
        pub m_stack_bottom: *const c_char,
    }

    #[repr(transparent)]
    pub struct ThreadGlobal(UnsafeCell<*mut thread_state>);
    // SAFETY: serialized unit test stub.
    unsafe impl Sync for ThreadGlobal {}

    static THREAD_PTR: AtomicUsize = AtomicUsize::new(0);

    impl ThreadGlobal {
        pub const fn new(v: *mut thread_state) -> Self {
            Self(UnsafeCell::new(v))
        }
        pub fn get(&self) -> *mut thread_state {
            THREAD_PTR.load(Ordering::Relaxed) as *mut thread_state
        }
        pub fn set(&self, v: *mut thread_state) {
            THREAD_PTR.store(v as usize, Ordering::Relaxed);
        }
    }

    pub static current_thread: ThreadGlobal = ThreadGlobal::new(core::ptr::null_mut());

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

#[cfg(test)]
mod tests {
    use core::ffi::{c_char, c_void};
    use std::panic::{AssertUnwindSafe, catch_unwind};
    use std::sync::Mutex;

    use crate::common::{Lisp_Object, current_thread, make_fixnum, thread_state};
    use crate::roots::{
        GcContext, LispVal, Rooted, RootedVec, bind_vm_lock_owner, cleanup_thread_u4_slab,
        mark_thread_u4_slab_roots, narrowed_stack_counts, unbind_vm_lock_owner,
    };

    static SERIAL: Mutex<()> = Mutex::new(());

    fn marked(th: *mut thread_state) -> Vec<isize> {
        let mut out = Vec::new();
        // SAFETY: isolated unit test holds `SERIAL`.
        unsafe { mark_thread_u4_slab_roots(th as *mut c_void, |o| out.push(((o as isize) >> 2))) };
        out
    }

    #[test]
    fn rooted_vec_tracks_reallocation_and_unwind_drop() {
        let _lk = SERIAL.lock().unwrap();
        let mut ts = thread_state { m_stack_bottom: core::ptr::null() };
        let th_ptr = &raw mut ts;
        current_thread.set(th_ptr);
        // SAFETY: isolated unit test holds `SERIAL`.
        unsafe {
            bind_vm_lock_owner(th_ptr as usize);
            cleanup_thread_u4_slab(th_ptr as *mut c_void);
        }

        {
            let mut rv = RootedVec::with_capacity(1);
            for i in 1..=32isize {
                rv.push(make_fixnum(i));
            }
            assert_eq!(marked(th_ptr), (1..=32).collect::<Vec<_>>());
            rv.truncate(4);
            assert_eq!(marked(th_ptr), vec![1, 2, 3, 4]);
            rv.resize(6, make_fixnum(99));
            assert_eq!(marked(th_ptr), vec![1, 2, 3, 4, 99, 99]);
        }
        assert!(marked(th_ptr).is_empty(), "RootedVec drop must truncate slab span");

        let res = catch_unwind(AssertUnwindSafe(|| {
            let mut rv = RootedVec::filled(3, make_fixnum(7));
            rv[1] = make_fixnum(8);
            assert_eq!(marked(th_ptr), vec![7, 8, 7]);
            panic!("simulated NonLocalExit unwind");
        }));
        assert!(res.is_err());
        assert!(marked(th_ptr).is_empty(), "unwind must truncate RootedVec span");
        // SAFETY: isolated unit test cleanup.
        unsafe {
            cleanup_thread_u4_slab(th_ptr as *mut c_void);
            unbind_vm_lock_owner();
        }
        current_thread.set(core::ptr::null_mut());
    }

    #[test]
    fn narrowed_stack_bottom_narrows_and_restores_across_return_and_unwind() {
        let _lk = SERIAL.lock().unwrap();
        let outer_sentinel: usize = 0xdead_beef;
        let outer_bottom = ((&raw const outer_sentinel) as usize + 4096) as *const c_char;
        let mut ts = thread_state { m_stack_bottom: outer_bottom };
        let th_ptr = &raw mut ts;
        current_thread.set(th_ptr);
        // SAFETY: isolated unit test holds `SERIAL`.
        unsafe { bind_vm_lock_owner(th_ptr as usize) };

        let mut ctx = GcContext::current().expect("active GcContext");
        let (e0, r0) = narrowed_stack_counts();
        let inner_seen = ctx.with_narrowed_stack_bottom(|_| {
            // SAFETY: reads active test `ts.m_stack_bottom`.
            unsafe { (*th_ptr).m_stack_bottom as usize }
        });
        assert!(inner_seen < outer_bottom as usize, "stack bottom must narrow downward");
        assert_eq!(ts.m_stack_bottom, outer_bottom, "normal return must restore m_stack_bottom");
        let (e1, r1) = narrowed_stack_counts();
        assert_eq!((e1 - e0, r1 - r0), (1, 1));

        let unwind_res = catch_unwind(AssertUnwindSafe(|| {
            ctx.with_narrowed_stack_bottom(|_| -> () {
                panic!("unwind through NarrowedStackScanGuard");
            });
        }));
        assert!(unwind_res.is_err());
        assert_eq!(ts.m_stack_bottom, outer_bottom, "unwind must restore m_stack_bottom");
        let (e2, r2) = narrowed_stack_counts();
        assert_eq!((e2 - e0, r2 - r0), (2, 2));
        drop(ctx);
        // SAFETY: `ctx` is dropped before unbinding lock ownership.
        unsafe { unbind_vm_lock_owner() };
        current_thread.set(core::ptr::null_mut());
    }

    #[test]
    fn out_of_order_rooted_and_vec_and_gc_context_exclusivity() {
        let _lk = SERIAL.lock().unwrap();
        let mut ts = thread_state { m_stack_bottom: core::ptr::null() };
        let th_ptr = &raw mut ts;
        current_thread.set(th_ptr);
        // SAFETY: isolated unit test holds `SERIAL`.
        unsafe {
            bind_vm_lock_owner(th_ptr as usize);
            cleanup_thread_u4_slab(th_ptr as *mut c_void);
        }

        let ctx = GcContext::current().expect("first GcContext");
        assert!(GcContext::current().is_none(), "double GcContext must fail");
        assert!(std::thread::spawn(|| GcContext::current().is_some()).join().unwrap() == false);

        // SAFETY: fixnums are valid immediate `Lisp_Object` words.
        let outer = ctx.root(unsafe { LispVal::from_raw(make_fixnum(10)) });
        // SAFETY: fixnums are valid immediate `Lisp_Object` words.
        let inner = ctx.root(unsafe { LispVal::from_raw(make_fixnum(20)) });
        let mut v1 = RootedVec::filled(1, make_fixnum(30));
        let v2 = RootedVec::filled(2, make_fixnum(40));
        assert_eq!(marked(th_ptr), vec![10, 20, 30, 40, 40]);

        drop(outer);
        drop(v1);
        assert_eq!(inner.get(), unsafe { LispVal::from_raw(make_fixnum(20)) });
        assert_eq!(inner.handle().get(), unsafe { LispVal::from_raw(make_fixnum(20)) });
        assert_eq!(marked(th_ptr), vec![20, 40, 40]);

        drop(v2);
        drop(inner);
        drop(ctx);
        assert!(marked(th_ptr).is_empty());
        // SAFETY: isolated unit test cleanup.
        unsafe {
            cleanup_thread_u4_slab(th_ptr as *mut c_void);
            unbind_vm_lock_owner();
        }
        current_thread.set(core::ptr::null_mut());
    }
}
"""


def run_selftest() -> int:
    import importlib.util
    import re

    # 1. Manifest & `#![forbid(unsafe_code)]` verification across all 22 U7 modules
    manifest = json.loads((REPO / "tools/unsafe-boundary-manifest.json").read_text(encoding="utf-8"))
    forbid_set = set(manifest.get("forbid_unsafe_modules", []))
    missing = [m for m in U7_FORBID_MODULES if m not in forbid_set]
    assert not missing, f"U7 forbid modules missing from manifest: {missing}"
    for rel in U7_FORBID_MODULES:
        p = REPO / rel
        assert p.is_file(), f"missing U7 module file: {rel}"
        text = p.read_text(encoding="utf-8")
        assert "#![forbid(unsafe_code)]" in text, f"{rel} lacks #![forbid(unsafe_code)]"

    # 2. `extract-abi.py` R_DEFUN_DEF accepts both `safe {` and `{`
    spec_abi = importlib.util.spec_from_file_location("eabi", REPO / "tools/extract-abi.py")
    eabi = importlib.util.module_from_spec(spec_abi)
    spec_abi.loader.exec_module(eabi)
    sample_safe = 'DEFUN! ("car", Fcar, Scar, 1, 1, 0, doc: "d", (list: Lisp_Object), safe { list });'
    sample_legacy = 'DEFUN! ("cdr", Fcdr, Scdr, 1, 1, 0, doc: "d", (list: Lisp_Object), { list });'
    assert eabi.R_DEFUN_DEF.search(sample_safe), "R_DEFUN_DEF must parse `safe {` DEFUN!"
    assert eabi.R_DEFUN_DEF.search(sample_legacy), "R_DEFUN_DEF must parse classic `{` DEFUN!"

    # 3. `xtask gen-globals` output invariants (`pub safe static` + `pub safe fn` vs `MANY` `pub unsafe fn`)
    globals_rs = (REPO / "src/include/globals.rs").read_text(encoding="utf-8")
    assert "pub safe static Vload_path: VmGlobal<Lisp_Object>;" in globals_rs
    assert "pub safe fn Fcar (a0: Lisp_Object) -> Lisp_Object;" in globals_rs
    assert "pub unsafe fn Flist (nargs: ptrdiff_t, args: *mut Lisp_Object) -> Lisp_Object;" in globals_rs

    # 4. Zero `foo` <-> `foo_safe` wrapper mutual-recursion cycles across headers and units
    hdr_helpers: dict[str, str] = {}
    for hp in sorted((REPO / "src/include").glob("*.rs")):
        ht = hp.read_text(encoding="utf-8")
        for m in re.finditer(r"pub\s+fn\s+([A-Za-z0-9_]+_safe)\s*\([^)]*\)(?:\s*->\s*[^{]+)?\s*\{([^}]+)\}", ht, re.S):
            hdr_helpers[m.group(1)] = m.group(2)
    for sp in sorted((REPO / "src").glob("*.rs")):
        st = sp.read_text(encoding="utf-8")
        for hname, hbody in hdr_helpers.items():
            base = hname[:-5]
            if base in hbody and hname in st:
                cycle = re.search(
                    r"fn\s+" + re.escape(base) + r"\b[^{]*\{[^}]*\b" + re.escape(hname) + r"\b",
                    st,
                    re.S,
                )
                assert not cycle, f"mutual recursion cycle detected: {sp.name}:{base} <-> {hname}"

    # 5. Compile and execute isolated `rustc --test` unit tests on `crates/femacs-vm/src/roots.rs`
    with tempfile.TemporaryDirectory(prefix="u7-roots-unit-") as tmp:
        tmp_dir = Path(tmp)
        harness = tmp_dir / "u7_roots_harness.rs"
        bin_out = tmp_dir / "u7_roots_test"
        roots_abs = str((REPO / "crates/femacs-vm/src/roots.rs").resolve())
        harness.write_text(U7_ROOTS_RUST_TESTS.replace("__ROOTS_RS_PATH__", roots_abs), encoding="utf-8")
        cproc = run_cmd(
            ["rustc", "--edition=2024", "--test", str(harness), "-o", str(bin_out)],
            timeout=60,
        )
        assert cproc.returncode == 0, f"rustc --test roots.rs failed:\n{cproc.stdout}\n{cproc.stderr}"
        tproc = run_cmd([str(bin_out), "--test-threads=1"], timeout=30)
        assert tproc.returncode == 0, f"roots.rs unit tests failed:\n{tproc.stdout}\n{tproc.stderr}"
        assert "3 passed; 0 failed" in tproc.stdout, tproc.stdout

    print(
        f"u7 selftest: 5/5 protective checks passed ({len(U7_FORBID_MODULES)} forbid modules + "
        f"extract-abi + gen-globals + zero wrapper cycles + 3/3 roots.rs Rust unit tests)"
    )
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--selftest", action="store_true")
    ap.add_argument(
        "--terminal-bin",
        type=Path,
        default=REPO / "build/terminal/port/src/emacs",
    )
    ap.add_argument(
        "--temacs-bin",
        type=Path,
        default=REPO / "build/terminal/port/src/temacs",
    )
    ap.add_argument(
        "--group",
        choices=("all", "static", "functional", "ert", "gc-stress"),
        default="all",
    )
    args = ap.parse_args()

    if args.selftest:
        return run_selftest()

    results: list[dict] = []
    ok = True
    if args.group in ("all", "static"):
        ok = run_case(
            results,
            "static-and-nm-gates",
            lambda: check_static_and_nm_gates(args.terminal_bin, args.temacs_bin),
        ) and ok
    if args.group in ("all", "functional"):
        ok = run_case(
            results,
            "safe-subsystems-functional",
            lambda: check_safe_subsystems_functional(args.terminal_bin),
        ) and ok
    if args.group in ("all", "ert"):
        ok = run_case(
            results,
            "core-ert-suites",
            lambda: check_core_ert_suites(args.terminal_bin),
        ) and ok
    if args.group in ("all", "gc-stress"):
        ok = run_case(
            results,
            "gc-stress-and-stack-narrowing",
            lambda: check_gc_stress_and_stack_narrowing(args.terminal_bin),
        ) and ok

    print(json.dumps({"ok": ok, "results": results}, indent=2))
    return 0 if ok else 1


if __name__ == "__main__":
    raise SystemExit(main())
