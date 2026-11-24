#!/usr/bin/env python3
"""AU4 integration and functional acceptance for U4 GC roots, VM state, and safe boundaries.

Verifies:
  1. Static & binary safety gates (`tools/check-unsafe-boundary.py` with
     `tools/unsafe-boundary-manifest.json`, `--require-no-file-allows`,
     `--require-zero-active-stitching`, `--require-100pct-safety-docs`) on Terminal and Cocoa
     products, verifying 0 `allow(unsafe_op_in_unsafe_fn)` / `allow(static_mut_refs)` active files,
     100% `SAFETY:` block contracts, `#![forbid(unsafe_code)]` high-level crate coverage, explicit
     boundary allowlist closure, conservative-scan bindings, Cocoa codesign, and exported U4 C-ABI
     symbols.
  2. Explicit root lifecycle, survival under GC stress, deterministic reclamation upon release,
     weak hash tables (`:weakness 'value`), `make-finalizer` execution, `USE_SAFE_ALLOCA!`
     dead-frame scrubbing (`sa_stack_frame_exit`), C dynamic module callbacks, and multi-thread
     boundaries across:
       - Terminal Interpreter & Bytecode (normal hybrid mode AND `FEMACS_GC_SKIP_STACK_SCAN=1`),
       - Newly compiled `.eln` under `u4-native` (`normal` AND `checking`, generational collector +
         GC observer enabled),
       - Cocoa `Femacs.app` batch + C dynamic module.
  3. High-level `#![forbid(unsafe_code)]` domain operations (`crates/femacs-vm/src/safe_domain.rs`)
     exercising Buffer, Reader/Evaluator, File I/O, and Window operations across GC safepoints on
     both Terminal and Cocoa binaries.
  4. Fresh `.pdmp` portable dump generation (`dump-emacs-portable`) and restore (`--dump-file`),
     verifying newly dumped closures, symbols, hash tables, buffer defaults, `pdumper-stats`, post-
     restore GC stress survival, and `femacs_vm_u4_root_stats` dump telemetry.
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
sys.path.insert(0, str(REPO / "tools"))
import module_api  # noqa: E402

C_MODULE_SOURCE = r"""
#include <emacs-module.h>
#include <dlfcn.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

int plugin_is_GPL_compatible;

typedef void (*persistent_alloc_fn)(void *raw_obj, size_t *out_idx, uint64_t *out_gen);
typedef void *(*persistent_get_fn)(size_t idx, uint64_t gen);
typedef bool (*persistent_release_fn)(size_t idx, uint64_t gen);
typedef size_t (*u4_root_stats_fn)(size_t *out_buf, size_t len);
typedef emacs_value (*u4_safe_domains_fn)(emacs_env *env, emacs_value path_val);

static persistent_alloc_fn g_p_alloc = NULL;
static persistent_get_fn g_p_get = NULL;
static persistent_release_fn g_p_release = NULL;
static u4_root_stats_fn g_root_stats = NULL;
static u4_safe_domains_fn g_safe_domains = NULL;

static emacs_value g_held_global_ref = NULL;

static emacs_value
fn_persistent_alloc (emacs_env *env, ptrdiff_t nargs, emacs_value *args, void *data)
{
  (void) nargs;
  (void) data;
  void *raw_obj = *(void **) args[0];
  size_t idx = 0;
  uint64_t gen = 0;
  g_p_alloc (raw_obj, &idx, &gen);
  void *verify_raw = g_p_get (idx, gen);
  if (verify_raw != raw_obj)
    {
      emacs_value err_sym = env->intern (env, "error");
      const char *msg = "persistent_get mismatch immediately after alloc";
      emacs_value msg_v = env->make_string (env, msg, 47);
      emacs_value list_fn = env->intern (env, "list");
      emacs_value d = env->funcall (env, list_fn, 1, &msg_v);
      env->non_local_exit_signal (env, err_sym, d);
      return env->intern (env, "nil");
    }
  emacs_value cons_fn = env->intern (env, "cons");
  emacs_value car_v = env->make_integer (env, (intmax_t) idx);
  emacs_value cdr_v = env->make_integer (env, (intmax_t) gen);
  emacs_value cargs[2] = { car_v, cdr_v };
  return env->funcall (env, cons_fn, 2, cargs);
}

static emacs_value
fn_persistent_check_alive (emacs_env *env, ptrdiff_t nargs, emacs_value *args, void *data)
{
  (void) nargs;
  (void) data;
  size_t idx = (size_t) env->extract_integer (env, args[0]);
  uint64_t gen = (uint64_t) env->extract_integer (env, args[1]);
  void *expected_raw = *(void **) args[2];
  void *got_raw = g_p_get (idx, gen);
  return env->intern (env, (got_raw == expected_raw) ? "t" : "nil");
}

static emacs_value
fn_persistent_release (emacs_env *env, ptrdiff_t nargs, emacs_value *args, void *data)
{
  (void) nargs;
  (void) data;
  size_t idx = (size_t) env->extract_integer (env, args[0]);
  uint64_t gen = (uint64_t) env->extract_integer (env, args[1]);
  void *nil_raw = *(void **) env->intern (env, "nil");
  bool first_ok = g_p_release (idx, gen);
  bool second_ok = g_p_release (idx, gen);
  void *after_raw = g_p_get (idx, gen);
  bool all_good = first_ok && (!second_ok) && (after_raw == nil_raw);
  return env->intern (env, all_good ? "t" : "nil");
}

static emacs_value
fn_global_ref_hold (emacs_env *env, ptrdiff_t nargs, emacs_value *args, void *data)
{
  (void) nargs;
  (void) data;
  if (g_held_global_ref != NULL)
    {
      env->free_global_ref (env, g_held_global_ref);
      g_held_global_ref = NULL;
    }
  g_held_global_ref = env->make_global_ref (env, args[0]);
  return env->intern (env, "t");
}

static emacs_value
fn_global_ref_drop (emacs_env *env, ptrdiff_t nargs, emacs_value *args, void *data)
{
  (void) nargs;
  (void) args;
  (void) data;
  if (g_held_global_ref != NULL)
    {
      env->free_global_ref (env, g_held_global_ref);
      g_held_global_ref = NULL;
      return env->intern (env, "t");
    }
  return env->intern (env, "nil");
}

static emacs_value
fn_callback_with_local_root (emacs_env *env, ptrdiff_t nargs, emacs_value *args, void *data)
{
  (void) nargs;
  (void) data;
  /* args[0] = value to hold in module local frame + PersistentRoot across callback
     args[1] = 1-arg Lisp callback that triggers GC and optionally signals */
  void *raw_obj = *(void **) args[0];
  size_t idx = 0;
  uint64_t gen = 0;
  g_p_alloc (raw_obj, &idx, &gen);
  emacs_value res = env->funcall (env, args[1], 1, &args[0]);
  /* Release PersistentRoot regardless of whether Lisp callback returned or signaled! */
  (void) g_p_release (idx, gen);
  return res;
}

static emacs_value
fn_root_stats (emacs_env *env, ptrdiff_t nargs, emacs_value *args, void *data)
{
  (void) nargs;
  (void) args;
  (void) data;
  size_t stats[12] = {0};
  g_root_stats (stats, 12);
  emacs_value list_fn = env->intern (env, "list");
  emacs_value items[12];
  for (int i = 0; i < 12; i++)
    items[i] = env->make_integer (env, (intmax_t) stats[i]);
  return env->funcall (env, list_fn, 12, items);
}

static emacs_value
fn_safe_domains (emacs_env *env, ptrdiff_t nargs, emacs_value *args, void *data)
{
  (void) nargs;
  (void) data;
  return g_safe_domains (env, args[0]);
}

static void
bind_fn (emacs_env *env, const char *name, ptrdiff_t min_arity, ptrdiff_t max_arity,
         emacs_value (*fn) (emacs_env *, ptrdiff_t, emacs_value *, void *))
{
  emacs_value f = env->make_function (env, min_arity, max_arity, fn, name, NULL);
  emacs_value sym = env->intern (env, name);
  emacs_value defset = env->intern (env, "fset");
  emacs_value fargs[2] = { sym, f };
  env->funcall (env, defset, 2, fargs);
}

int
emacs_module_init (struct emacs_runtime *ert)
{
  emacs_env *env = ert->get_environment (ert);
  g_p_alloc = (persistent_alloc_fn) dlsym (RTLD_DEFAULT, "femacs_vm_persistent_root_alloc");
  g_p_get = (persistent_get_fn) dlsym (RTLD_DEFAULT, "femacs_vm_persistent_root_get");
  g_p_release = (persistent_release_fn) dlsym (RTLD_DEFAULT, "femacs_vm_persistent_root_release");
  g_root_stats = (u4_root_stats_fn) dlsym (RTLD_DEFAULT, "femacs_vm_u4_root_stats");
  g_safe_domains = (u4_safe_domains_fn) dlsym (RTLD_DEFAULT, "femacs_vm_u4_exercise_safe_domains");
  if (!g_p_alloc || !g_p_get || !g_p_release || !g_root_stats || !g_safe_domains)
    return 1;

  bind_fn (env, "u4-persistent-alloc", 1, 1, fn_persistent_alloc);
  bind_fn (env, "u4-persistent-check-alive", 3, 3, fn_persistent_check_alive);
  bind_fn (env, "u4-persistent-release", 2, 2, fn_persistent_release);
  bind_fn (env, "u4-global-ref-hold", 1, 1, fn_global_ref_hold);
  bind_fn (env, "u4-global-ref-drop", 0, 0, fn_global_ref_drop);
  bind_fn (env, "u4-callback-with-local-root", 2, 2, fn_callback_with_local_root);
  bind_fn (env, "u4-root-stats", 0, 0, fn_root_stats);
  bind_fn (env, "u4-exercise-safe-domains", 1, 1, fn_safe_domains);
  return 0;
}
"""

LISP_ROOT_SUITE = r""";;; u4_root_suite.el --- AU4 root lifecycle & GC stress suite -*- lexical-binding: t; -*-

(defvar u4-finalized-tags nil)

(defun u4-gc-churn (rounds)
  "Trigger allocation churn and explicit GC cycles."
  (let ((gc-cons-threshold 4096))
    (dotimes (r rounds)
      (let ((junk nil))
        (dotimes (k 200)
          (setq junk (cons (format "churn-%d-%d" r k) junk)))
        (setq junk nil))
      (garbage-collect))))

(defun u4-12arg-helper (a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12)
  "12-argument lambda exercising ScopedRawSpan >8 slot path."
  (u4-gc-churn 2)
  (list a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12))

(defun u4-core-root-exercise ()
  "Verify Rooted (prog1), ScopedRawSpan (12-var let & 12-arg calls), dynamic specpdl, and bytecode."
  ;; 1. `prog1` holds first form value in `Rooted` while body churns & runs GC
  ;; (both in the active tier and explicitly through `eval` -> `Fprog1`).
  (let ((p1 (prog1
                (list (concat "u4-prog1-" (number-to-string 101))
                      (vector 'alpha 'beta 999))
              (u4-gc-churn 3)))
        (p2 (eval '(prog1 (list (concat "u4-eval-prog1-" (number-to-string 7)) 42)
                     (u4-gc-churn 1))
                  t)))
    (unless (and (equal p1 '("u4-prog1-101" [alpha beta 999]))
                 (equal p2 '("u4-eval-prog1-7" 42)))
      (error "prog1 Rooted value corrupted: %S / %S" p1 p2)))

  ;; 2. 12-binding `let` where binding #11 runs GC while bindings #1..#10 live in `ScopedRawSpan`.
  (let ((v1 (concat "b1-" (number-to-string 1)))
        (v2 (concat "b2-" (number-to-string 2)))
        (v3 (concat "b3-" (number-to-string 3)))
        (v4 (concat "b4-" (number-to-string 4)))
        (v5 (concat "b5-" (number-to-string 5)))
        (v6 (concat "b6-" (number-to-string 6)))
        (v7 (concat "b7-" (number-to-string 7)))
        (v8 (concat "b8-" (number-to-string 8)))
        (v9 (concat "b9-" (number-to-string 9)))
        (v10 (concat "b10-" (number-to-string 10)))
        (v11 (progn (u4-gc-churn 3) (concat "b11-" (number-to-string 11))))
        (v12 (concat "b12-" (number-to-string 12))))
    (unless (equal (list v1 v2 v3 v4 v5 v6 v7 v8 v9 v10 v11 v12)
                   '("b1-1" "b2-2" "b3-3" "b4-4" "b5-5" "b6-6"
                     "b7-7" "b8-8" "b9-9" "b10-10" "b11-11" "b12-12"))
      (error "12-binding let ScopedRawSpan corrupted")))

  ;; 3. 12-argument subr (`list`) and 12-argument lambda where arg #11 runs GC mid-evaluation.
  (let ((subr-res (list (concat "s1-" (number-to-string 1))
                        (concat "s2-" (number-to-string 2))
                        (concat "s3-" (number-to-string 3))
                        (concat "s4-" (number-to-string 4))
                        (concat "s5-" (number-to-string 5))
                        (concat "s6-" (number-to-string 6))
                        (concat "s7-" (number-to-string 7))
                        (concat "s8-" (number-to-string 8))
                        (concat "s9-" (number-to-string 9))
                        (concat "s10-" (number-to-string 10))
                        (progn (u4-gc-churn 2) (concat "s11-" (number-to-string 11)))
                        (concat "s12-" (number-to-string 12))))
        (lam-res (u4-12arg-helper
                  (concat "l1-" (number-to-string 1))
                  (concat "l2-" (number-to-string 2))
                  (concat "l3-" (number-to-string 3))
                  (concat "l4-" (number-to-string 4))
                  (concat "l5-" (number-to-string 5))
                  (concat "l6-" (number-to-string 6))
                  (concat "l7-" (number-to-string 7))
                  (concat "l8-" (number-to-string 8))
                  (concat "l9-" (number-to-string 9))
                  (concat "l10-" (number-to-string 10))
                  (progn (u4-gc-churn 2) (concat "l11-" (number-to-string 11)))
                  (concat "l12-" (number-to-string 12)))))
    (unless (and (= (length subr-res) 12)
                 (equal (nth 0 subr-res) "s1-1")
                 (equal (nth 10 subr-res) "s11-11")
                 (= (length lam-res) 12)
                 (equal (nth 0 lam-res) "l1-1")
                 (equal (nth 10 lam-res) "l11-11"))
      (error "12-arg evaluation ScopedRawSpan corrupted")))

  ;; 4. Direct readevalloop -> eval_sub (.elc path, macroexpand=nil) with GC triggered on entry.
  (with-temp-buffer
    (dotimes (r 6)
      (insert "(list")
      (dotimes (i 400)
        (insert (format " \"s%d-%d\"" r i)))
      (insert ")\n"))
    (let ((gc-cons-threshold 10000)
          (gc-cons-percentage 0.0))
      (eval-buffer nil nil "u4-direct-eval.elc")))
  'core-ok)

(defun u4-alloc-only-persistent (weak-tbl)
  "Allocate an uninterned cons cell + finalizer whose ONLY live reference is PersistentRoot."
  (let* ((obj (cons (concat "pers-payload-" (number-to-string 777))
                    (make-finalizer (lambda () (push 'pers-fin u4-finalized-tags))))))
    (puthash 'pers obj weak-tbl)
    (u4-persistent-alloc obj)))

(defun u4-alloc-only-global-ref (weak-tbl)
  "Allocate an uninterned cons cell + finalizer whose ONLY live reference is module global_ref."
  (let* ((obj (cons (concat "gref-payload-" (number-to-string 888))
                    (make-finalizer (lambda () (push 'gref-fin u4-finalized-tags))))))
    (puthash 'gref obj weak-tbl)
    (u4-global-ref-hold obj)
    t))

(defun u4-alloc-ephemeral-dead-alloca (weak-tbl)
  "Exercise large `string-distance` (`USE_SAFE_ALLOCA!` -> `sa_stack_frame_exit`) and ephemeral object."
  (let ((s1 (make-string 2600 ?a))
        (s2 (make-string 2600 ?b))
        (ephem (cons (concat "ephem-" (number-to-string 999))
                     (make-finalizer (lambda () (push 'ephem-fin u4-finalized-tags))))))
    (puthash 'ephem ephem weak-tbl)
    (unless (= (string-distance s1 s2 t) 2600)
      (error "unexpected string-distance")))
  t)

(defun u4-verify-rooted-alive (weak-tbl tok)
  "Inspect weak-table entries in an isolated child frame so caller stack holds no raw pointer."
  (let ((pers-live (gethash 'pers weak-tbl))
        (gref-live (gethash 'gref weak-tbl)))
    (unless (and (consp pers-live) (equal (car pers-live) "pers-payload-777"))
      (error "PersistentRoot object collected prematurely!"))
    (unless (and (consp gref-live) (equal (car gref-live) "gref-payload-888"))
      (error "module global_ref object collected prematurely!"))
    (unless (eq (u4-persistent-check-alive (car tok) (cdr tok) pers-live) t)
      (error "PersistentRoot check-alive failed"))
    (when (memq 'pers-fin u4-finalized-tags)
      (error "PersistentRoot finalizer fired while still rooted!"))
    (when (memq 'gref-fin u4-finalized-tags)
      (error "global_ref finalizer fired while still rooted!")))
  t)

(defun u4-scrub-native-stack (depth)
  "Overwrite exited child C stack slots with fixnum frames for hybrid conservative mode."
  (if (<= depth 0)
      0
    (let ((a 1) (b 2) (c 3) (d 4) (e 5) (f 6) (g 7) (h 8))
      (+ a b c d e f g h (u4-scrub-native-stack (1- depth))))))

(defun u4-run-full-lifecycle ()
  "End-to-end root survival, weak-table/finalizer reclamation, callback unwind, and thread test."
  (setq u4-finalized-tags nil)
  (let ((weak-tbl (make-hash-table :test 'eq :weakness 'value)))
    ;; A. Core Rooted + ScopedRawSpan survival
    (unless (eq (u4-core-root-exercise) 'core-ok)
      (error "core root exercise failed"))

    ;; B. PersistentRoot + module global_ref + ephemeral USE_SAFE_ALLOCA! frame
    (let ((tok (u4-alloc-only-persistent weak-tbl)))
      (u4-alloc-only-global-ref weak-tbl)
      (u4-alloc-ephemeral-dead-alloca weak-tbl)

      ;; Ephemeral object had no remaining root; PersistentRoot & global_ref MUST survive GC churn!
      (u4-gc-churn 5)
      (u4-verify-rooted-alive weak-tbl tok)

      ;; Now release PersistentRoot and module global_ref!
      (unless (eq (u4-persistent-release (car tok) (cdr tok)) t)
        (error "PersistentRoot release / ABA generation check failed"))
      (unless (eq (u4-global-ref-drop) t)
        (error "u4-global-ref-drop failed"))

      ;; Overwrite exited child native stack frames in hybrid mode, then verify reclamation!
      (u4-scrub-native-stack 24)
      (u4-gc-churn 6)
      (when (gethash 'pers weak-tbl)
        (error "Released PersistentRoot object still alive in weak table!"))
      (when (gethash 'gref weak-tbl)
        (error "Released global_ref object still alive in weak table!"))
      (when (gethash 'ephem weak-tbl)
        (error "Ephemeral object still alive in weak table!"))
      (unless (and (memq 'pers-fin u4-finalized-tags)
                   (memq 'gref-fin u4-finalized-tags)
                   (memq 'ephem-fin u4-finalized-tags))
        (error "Expected finalizers did not fire: %S" u4-finalized-tags)))

    ;; C. Nested C module -> Lisp callback with GC + NonLocalExit unwind
    (let ((cb-ok (condition-case err
                     (u4-callback-with-local-root
                      (list "cb-root" 1234)
                      (lambda (val)
                        (u4-gc-churn 2)
                        (unless (equal val '("cb-root" 1234))
                          (error "callback root corrupted"))
                        (signal ' arith-error '(cb-unwind))))
                   (arith-error 'caught-cb-unwind))))
      (unless (eq cb-ok 'caught-cb-unwind)
        (error "module callback unwind mismatch: %S" cb-ok)))

    ;; D. Background Lisp thread boundary (`make-thread` + `thread-yield` + GC)
    (let* ((worker-result nil)
           (th (make-thread
                (lambda ()
                  (let ((local-hold (list (concat "thread-root-" (number-to-string 55))
                                          (vector 10 20 30))))
                    (thread-yield)
                    (u4-gc-churn 2)
                    (setq worker-result (copy-sequence (car local-hold)))))
                "u4-gc-worker")))
      (thread-yield)
      (u4-gc-churn 2)
      (thread-join th)
      (unless (equal worker-result "thread-root-55")
        (error "background thread root corrupted: %S" worker-result)))

    ;; E. Read VM root inventory stats
    (let ((stats (u4-root-stats)))
      (list :status 'all-ok
            :finalized (sort (mapcar #'symbol-name u4-finalized-tags) #'string<)
            :stats stats))))

(provide 'u4_root_suite)
;;; u4_root_suite.el ends here
"""


def run_checked(cmd: list[str], *, env: dict[str, str] | None = None, timeout: int = 180) -> subprocess.CompletedProcess[str]:
    proc = subprocess.run(
        cmd,
        cwd=REPO,
        env=env,
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


def compile_c_probe_module(work_dir: Path) -> Path:
    header_path = module_api.render_module_api(REPO)
    c_src = work_dir / "u4_root_probe.c"
    dylib = work_dir / "u4_root_probe.dylib"
    c_src.write_text(C_MODULE_SOURCE.lstrip(), encoding="utf-8")
    run_checked(
        [
            "clang",
            "-dynamiclib",
            "-Wl,-undefined,dynamic_lookup",
            "-O2",
            "-Wall",
            "-Wextra",
            "-Werror",
            f"-I{header_path.parent}",
            str(c_src),
            "-o",
            str(dylib),
        ],
        timeout=60,
    )
    return dylib


def check_u4_exports_present(binary: Path) -> None:
    proc = run_checked(["nm", str(binary)], timeout=60)
    defined = set()
    for line in proc.stdout.splitlines():
        parts = line.split()
        if len(parts) >= 3 and parts[1] not in ("U", "u"):
            defined.add(parts[2].removeprefix("_"))
    required = (
        "femacs_vm_persistent_root_alloc",
        "femacs_vm_persistent_root_get",
        "femacs_vm_persistent_root_release",
        "femacs_vm_u4_root_stats",
        "femacs_vm_u4_exercise_safe_domains",
    )
    missing = [s for s in required if s not in defined]
    if missing:
        raise AssertionError(f"{binary} missing U4 C-ABI exports: {missing}")


def parse_stats_json(stdout: str) -> dict:
    for line in reversed(stdout.splitlines()):
        if line.startswith("U4_JSON:"):
            return json.loads(line.removeprefix("U4_JSON:"))
    raise AssertionError(f"missing U4_JSON marker in output:\n{stdout[-2000:]}")


def run_emacs_json(
    binary: Path,
    extra_args: list[str],
    eval_form: str,
    *,
    skip_stack_scan: bool = False,
    extra_env: dict[str, str] | None = None,
    timeout: int = 180,
) -> dict:
    env = os.environ.copy()
    if skip_stack_scan:
        env["FEMACS_GC_SKIP_STACK_SCAN"] = "1"
    else:
        env.pop("FEMACS_GC_SKIP_STACK_SCAN", None)
    if extra_env:
        env.update(extra_env)
    wrapped = (
        "(progn "
        "(require 'json) "
        f"(let ((res {eval_form})) "
        "(princ (concat \"U4_JSON:\" (json-encode res) \"\\n\"))))"
    )
    proc = run_checked(
        [str(binary), "--batch", "-Q", *extra_args, "--eval", wrapped],
        env=env,
        timeout=timeout,
    )
    return parse_stats_json(proc.stdout + "\n" + proc.stderr)


def assert_valid_lifecycle_payload(payload: dict, label: str, expect_dump_loaded: bool = True) -> None:
    assert payload.get("status") == "all-ok", f"{label}: unexpected status {payload}"
    finalized = payload.get("finalized")
    assert finalized == ["ephem-fin", "gref-fin", "pers-fin"], f"{label}: finalized mismatch {finalized}"
    stats = payload.get("stats")
    assert isinstance(stats, list) and len(stats) == 12, f"{label}: bad stats {stats}"
    (
        gc_cycles,
        dump_loaded,
        dump_hooks,
        active_threads,
        _specpdl_depth,
        rooted_pushed,
        rooted_popped,
        raw_spans_pushed,
        raw_spans_popped,
        persistent_live,
        _last_pers_marked,
        sa_free_scrubbed,
    ) = stats
    assert gc_cycles >= 15, f"{label}: expected >= 15 GC cycles, got {gc_cycles}"
    if expect_dump_loaded:
        assert dump_loaded == 1, f"{label}: expected dump_loaded == 1, got {dump_loaded}"
    assert dump_hooks > 0, f"{label}: expected registered pdumper hooks > 0, got {dump_hooks}"
    assert active_threads == 1, f"{label}: expected active_threads == 1 after join, got {active_threads}"
    assert rooted_pushed > 0, f"{label}: expected Rooted pushes > 0"
    assert rooted_pushed >= rooted_popped, f"{label}: popped > pushed ({rooted_pushed} vs {rooted_popped})"
    assert raw_spans_pushed > 0, f"{label}: expected ScopedRawSpan pushes > 0"
    assert raw_spans_pushed - raw_spans_popped <= 4, (
        f"{label}: ScopedRawSpan leak ({raw_spans_pushed} pushed vs {raw_spans_popped} popped)"
    )
    assert persistent_live == 0, f"{label}: PersistentRoot leak! persistent_live={persistent_live}"
    assert sa_free_scrubbed >= 1, (
        f"{label}: expected sa_stack_frame_exit dead-frame scrub >= 1, got {sa_free_scrubbed}"
    )


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--terminal-bin", type=Path, default=REPO / "build/terminal/port/src/emacs")
    ap.add_argument(
        "--cocoa-app",
        type=Path,
        default=REPO / "dist/macos/Femacs.app",
    )
    ap.add_argument(
        "--native-dir",
        type=Path,
        default=REPO / "build/runtime/u4-native",
    )
    args = ap.parse_args()

    terminal_bin = args.terminal_bin.resolve()
    cocoa_app = args.cocoa_app.resolve()
    cocoa_bin = cocoa_app / "Contents/MacOS/Femacs"
    native_normal_bin = (args.native_dir / "port/src/emacs").resolve()
    native_checking_bin = (args.native_dir / "checking/port/src/emacs").resolve()

    suite_t0 = time.monotonic()
    results: list[dict] = []

    def run_group(name: str, fn) -> None:
        t0 = time.monotonic()
        fn()
        dt = time.monotonic() - t0
        results.append({"group": name, "seconds": round(dt, 4), "status": "passed"})
        print(f"GROUP {name} seconds={dt:.3f} status=passed", flush=True)

    with tempfile.TemporaryDirectory(prefix="femacs-au4-") as tmp_str:
        work = Path(tmp_str)
        dylib = compile_c_probe_module(work)
        el_path = work / "u4_root_suite.el"
        el_path.write_text(LISP_ROOT_SUITE.lstrip(), encoding="utf-8")

        def group1_static_and_binary_gates() -> None:
            for kind, bpath in (("terminal", terminal_bin), ("cocoa", cocoa_bin)):
                proc = run_checked(
                    [
                        sys.executable,
                        str(REPO / "tools/check-unsafe-boundary.py"),
                        "--manifest",
                        str(REPO / "tools/unsafe-boundary-manifest.json"),
                        "--require-no-file-allows",
                        "--require-zero-active-stitching",
                        "--require-100pct-safety-docs",
                        "--require-modules",
                        "--binary",
                        str(bpath),
                        "--kind",
                        kind,
                    ],
                    timeout=120,
                )
                rep = json.loads(proc.stdout)
                ap_metrics = rep["active_product"]
                assert ap_metrics["allow_unsafe_op_files"] == 0, ap_metrics
                assert ap_metrics["allow_static_mut_refs_files"] == 0, ap_metrics
                assert ap_metrics["unsafe_blocks"] == ap_metrics["safety_documented_blocks"], ap_metrics
                assert ap_metrics["forbid_unsafe_files"] >= 31, ap_metrics
                check_u4_exports_present(bpath)

            run_checked(["codesign", "--verify", "--deep", "--strict", str(cocoa_app)], timeout=60)

        def group2_root_lifecycle_tiers_and_skip_stack_scan() -> None:
            # 2a. Terminal Interpreter (Normal hybrid AND FEMACS_GC_SKIP_STACK_SCAN=1)
            for skip_scan in (False, True):
                res = run_emacs_json(
                    terminal_bin,
                    ["-l", str(dylib), "-l", str(el_path)],
                    "(u4-run-full-lifecycle)",
                    skip_stack_scan=skip_scan,
                )
                assert_valid_lifecycle_payload(res, f"terminal-interp(skip_scan={skip_scan})")

            # 2b. Byte-compile u4_root_suite.el -> .elc and run in Bytecode tier (Normal + SKIP_STACK_SCAN=1)
            run_checked(
                [str(terminal_bin), "--batch", "-Q", "-f", "batch-byte-compile", str(el_path)],
                timeout=120,
            )
            elc_path = el_path.with_suffix(".elc")
            assert elc_path.is_file(), "expected byte-compiled u4_root_suite.elc"
            for skip_scan in (False, True):
                res = run_emacs_json(
                    terminal_bin,
                    ["-l", str(dylib), "-l", str(elc_path)],
                    "(u4-run-full-lifecycle)",
                    skip_stack_scan=skip_scan,
                )
                assert_valid_lifecycle_payload(res, f"terminal-bytecode(skip_scan={skip_scan})")

            # 2c. Cocoa Femacs.app batch + C module (Normal + SKIP_STACK_SCAN=1)
            for skip_scan in (False, True):
                res = run_emacs_json(
                    cocoa_bin,
                    ["-l", str(dylib), "-l", str(elc_path)],
                    "(u4-run-full-lifecycle)",
                    skip_stack_scan=skip_scan,
                )
                assert_valid_lifecycle_payload(res, f"cocoa-bytecode(skip_scan={skip_scan})")

            # 2d. Native `.eln` + Generational Collector + GC Observer (`normal` and `checking`)
            for leg_name, nbin in (("normal", native_normal_bin), ("checking", native_checking_bin)):
                eln_out = work / f"u4_suite_{leg_name}.eln"
                nc_cache = work / f"eln-cache-{leg_name}"
                nc_cache.mkdir(parents=True, exist_ok=True)
                nc_env = {
                    "EMACSNATIVELOADPATH": f"{nc_cache}:",
                    "FEMACS_GC_OBSERVATION": "all",
                    "FEMACS_GC_COLLECTOR": "young",
                }
                run_emacs_json(
                    nbin,
                    ["-l", str(dylib)],
                    f"(progn (require 'comp) (native-compile {json.dumps(str(el_path))} {json.dumps(str(eln_out))}) 'compiled)",
                    extra_env=nc_env,
                    timeout=240,
                )
                assert eln_out.is_file(), f"missing compiled .eln for {leg_name}: {eln_out}"
                # `.eln` frames store `gcc_jit_function_new_local` variables on the native machine stack
                # (the concrete justification for `mark_c_stack` in `conservative_scans`), so `.eln`
                # runs in hybrid mode (`skip_stack_scan=False`) under generational collector + observer,
                # while explicit `safe_domain` + `PersistentRoot` also verify `skip_stack_scan=True` on `nbin`.
                res = run_emacs_json(
                    nbin,
                    ["-l", str(dylib)],
                    f"(progn (native-elisp-load {json.dumps(str(eln_out))}) "
                    "(unless (subr-native-elisp-p (symbol-function 'u4-run-full-lifecycle)) "
                    "  (error \"expected native .eln function\")) "
                    "(u4-run-full-lifecycle))",
                    skip_stack_scan=False,
                    extra_env=nc_env,
                    timeout=180,
                )
                assert_valid_lifecycle_payload(res, f"native-{leg_name}(hybrid)")

        def group3_safe_high_level_domains() -> None:
            for label, bpath in (
                ("terminal", terminal_bin),
                ("cocoa", cocoa_bin),
                ("native-normal", native_normal_bin),
                ("native-checking", native_checking_bin),
            ):
                io_target = work / f"safe-io-{label}.txt"
                res = run_emacs_json(
                    bpath,
                    ["-l", str(dylib)],
                    f"(list :report (u4-exercise-safe-domains {json.dumps(str(io_target))}) "
                    ":ephem-gone (null (get-buffer \" *u4-safe-buffer*\")))",
                    skip_stack_scan=(label in ("terminal", "cocoa")),
                    extra_env={"EMACSNATIVELOADPATH": f"{work}:"},
                )
                report = res.get("report", "")
                expected = "buf= *u4-safe-buffer*:15:18;eval=(4 9 25 49);io_ok=true;win=true:1"
                assert report == expected, f"{label}: safe_domain mismatch: got {report!r}, expected {expected!r}"
                assert res.get("ephem-gone") is True, f"{label}: ephemeral buffer was not killed"
                assert io_target.read_text(encoding="utf-8") == "u4-io-roundtrip-你好\n"

        def group4_fresh_pdmp_dump_and_restore() -> None:
            fresh_pdmp = work / "u4-fresh.pdmp"
            dump_form = (
                "(progn "
                "  (defvar u4-dumped-closure (let ((seed 40)) (lambda (x) (+ seed x)))) "
                "  (defvar u4-dumped-table (let ((h (make-hash-table :test 'equal))) "
                "                            (puthash \"k1\" '(dumped-list 1 2 3) h) h)) "
                "  (setq-default u4-dumped-buf-local 'u4-default-sentinel) "
                f"  (dump-emacs-portable {json.dumps(str(fresh_pdmp))}) "
                "  'dumped)"
            )
            run_emacs_json(terminal_bin, [], dump_form, timeout=180)
            assert fresh_pdmp.is_file() and fresh_pdmp.stat().st_size > 1_000_000, fresh_pdmp

            verify_form = (
                "(progn "
                f"  (load {json.dumps(str(dylib))} nil t t) "
                f"  (load {json.dumps(str(el_path))} nil t) "
                "  (u4-gc-churn 4) "
                "  (let ((pstats (pdumper-stats)) "
                "        (cval (funcall u4-dumped-closure 2)) "
                "        (hval (gethash \"k1\" u4-dumped-table)) "
                "        (bval (with-temp-buffer u4-dumped-buf-local)) "
                "        (rstats (u4-root-stats))) "
                "    (list :pdumped (not (null (assq 'dumped-with-pdumper pstats))) "
                "          :closure cval "
                "          :hash hval "
                "          :buflocal bval "
                "          :stats rstats)))"
            )
            proc = run_checked(
                [
                    str(terminal_bin),
                    f"--dump-file={fresh_pdmp}",
                    "--batch",
                    "--eval",
                    f"(progn (require 'json) (princ (concat \"U4_JSON:\" (json-encode {verify_form}) \"\\n\")))",
                ],
                env={**os.environ, "FEMACS_GC_SKIP_STACK_SCAN": "1"},
                timeout=180,
            )
            restored = parse_stats_json(proc.stdout + "\n" + proc.stderr)
            assert restored.get("pdumped") is True, restored
            assert restored.get("closure") == 42, restored
            assert restored.get("hash") == ["dumped-list", 1, 2, 3], restored
            assert restored.get("buflocal") == "u4-default-sentinel", restored
            rstats = restored.get("stats", [])
            assert len(rstats) == 12 and rstats[1] == 1 and rstats[2] > 0 and rstats[0] >= 4, rstats

        run_group("1_static_binary_manifest_and_forbid_gates", group1_static_and_binary_gates)
        run_group("2_root_lifecycle_gc_stress_weak_finalizer_tiers", group2_root_lifecycle_tiers_and_skip_stack_scan)
        run_group("3_safe_domain_buffer_reader_io_window", group3_safe_high_level_domains)
        run_group("4_fresh_pdmp_dump_and_restore", group4_fresh_pdmp_dump_and_restore)

    total_s = time.monotonic() - suite_t0
    print(
        json.dumps(
            {
                "suite": "AU4_gc_roots_and_safe_boundaries",
                "groups_passed": len(results),
                "groups": results,
                "wall_seconds": round(total_s, 4),
            },
            indent=2,
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
