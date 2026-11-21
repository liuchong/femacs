#!/usr/bin/env python3
"""AU3 integration and functional acceptance for U3 explicit non-local exits.

Verifies:
  1. Unsafe/ABI gates (`tools/check-unsafe-boundary.py --require-zero-active-stitching`) on
     Terminal and Cocoa binaries, Cocoa bundle signature & frame flags, and exported C-ABI
     destructor probe symbols (`femacs_vm_host_drop_counts`,
     `femacs_vm_module_call0_with_host_guard`).
  2. Formal rejection of pre-U3 / mismatched-ABI `.eln` compilation units via `native-elisp-load`
     (`native-lisp-file-inconsistent`), formal `.el` -> `.eln` compilation via `native-compile`
     under U3 `ABI_VERSION` "14", and `nm` proof that newly generated `.eln` binaries carry zero
     `_setjmp`/`setjmp`/`longjmp` imports.
  3. Identical observable event order, return values, dynamic-binding restoration, recursive
     `prin1` stack growth/unwind safety, GC survival during `unwind-protect`, and clean post-error
     subsequent calls across all four execution tiers:
       - Interpreter,
       - Bytecode (`byte-compile`),
       - Newly compiled `.eln` (`u3-native` normal + `ENABLE_CHECKING`),
       - Real C dynamic module (`.dylib` across nested C <-> VM frames),
     plus Terminal and Cocoa editor binaries.
  4. Exact-once Rust `Drop` execution (`HostDropGuard` and `ScopedLispRoot`) across normal returns,
     `signal`, `throw`, `quit`, `process_input`, and cleanup re-exits, with zero unwinds crossing
     C FFI (`c_frames_entered == c_frames_exited_normally`).
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
#include <stddef.h>
#include <stdint.h>
#include <string.h>

int plugin_is_GPL_compatible;

typedef void (*host_drop_counts_fn)(
    size_t *out_host_created,
    size_t *out_host_dropped,
    size_t *out_root_created,
    size_t *out_root_dropped
);

typedef emacs_value (*module_call0_guard_fn)(
    emacs_env *env,
    emacs_value func
);

static host_drop_counts_fn g_drop_counts = NULL;
static module_call0_guard_fn g_call0_guard = NULL;
static size_t g_c_entered = 0;
static size_t g_c_exited = 0;

static emacs_value make_int_list4(
    emacs_env *env,
    intmax_t a,
    intmax_t b,
    intmax_t c,
    intmax_t d
) {
    emacs_value fn_list = env->intern(env, "list");
    emacs_value vals[4];
    vals[0] = env->make_integer(env, a);
    vals[1] = env->make_integer(env, b);
    vals[2] = env->make_integer(env, c);
    vals[3] = env->make_integer(env, d);
    return env->funcall(env, fn_list, 4, vals);
}

/* (au3-mod-counts) -> (host-created host-dropped root-created root-dropped) */
static emacs_value F_au3_mod_counts(
    emacs_env *env,
    ptrdiff_t nargs,
    emacs_value *args,
    void *data
) {
    (void)nargs;
    (void)args;
    (void)data;
    size_t hc = 0, hd = 0, rc = 0, rd = 0;
    g_drop_counts(&hc, &hd, &rc, &rd);
    return make_int_list4(env, (intmax_t)hc, (intmax_t)hd, (intmax_t)rc, (intmax_t)rd);
}

/* (au3-mod-c-frame-stats) -> (c-entered c-exited diff 0) */
static emacs_value F_au3_mod_c_frame_stats(
    emacs_env *env,
    ptrdiff_t nargs,
    emacs_value *args,
    void *data
) {
    (void)nargs;
    (void)args;
    (void)data;
    return make_int_list4(
        env,
        (intmax_t)g_c_entered,
        (intmax_t)g_c_exited,
        (intmax_t)(g_c_entered - g_c_exited),
        0
    );
}

/* Inner C frame: invokes zero-arg Lisp callable via `femacs_vm_module_call0_with_host_guard`.
   If an exit occurred, it remains pending on `env` and returns normally through this C frame
   without C stack unwinding (`g_c_exited++` always runs). */
static emacs_value F_au3_mod_inner_guard(
    emacs_env *env,
    ptrdiff_t nargs,
    emacs_value *args,
    void *data
) {
    (void)nargs;
    (void)data;
    g_c_entered += 1;
    emacs_value res = g_call0_guard(env, args[0]);
    g_c_exited += 1;
    return res;
}

/* Outer C frame: calls Lisp `funcall` on `au3-mod-inner-guard` so an exit traverses:
     Lisp/Bytecode/.eln -> Rust `call0_with_host_drop_guard_inner` (HostDropGuard #2)
     -> caught at inner C module boundary -> normal C return (`g_c_exited++`)
     -> VM re-raises `NonLocalExit` -> Rust `call0_with_host_drop_guard_inner` (HostDropGuard #1)
     -> caught at outer C module boundary -> normal C return (`g_c_exited++`)
     -> VM propagates to Lisp caller! */
static emacs_value F_au3_mod_outer_transparent(
    emacs_env *env,
    ptrdiff_t nargs,
    emacs_value *args,
    void *data
) {
    (void)nargs;
    (void)data;
    g_c_entered += 1;
    emacs_value thunk = args[0];
    /* Wrap the outer call in a Lisp lambda so `g_call0_guard` also guards the outer frame. */
    emacs_value list_fn = env->intern(env, "list");
    emacs_value quote_sym = env->intern(env, "quote");
    emacs_value lambda_sym = env->intern(env, "lambda");
    emacs_value nil_sym = env->intern(env, "nil");
    emacs_value inner_sym = env->intern(env, "au3-mod-inner-guard");

    emacs_value q_args[2] = { quote_sym, thunk };
    emacs_value quoted_thunk = env->funcall(env, list_fn, 2, q_args);
    emacs_value body_args[2] = { inner_sym, quoted_thunk };
    emacs_value body_form = env->funcall(env, list_fn, 2, body_args);
    emacs_value lam_args[3] = { lambda_sym, nil_sym, body_form };
    emacs_value outer_lambda = env->funcall(env, list_fn, 3, lam_args);

    emacs_value out = g_call0_guard(env, outer_lambda);
    g_c_exited += 1;
    return out;
}

/* (au3-mod-probe-intercept THUNK)
   Calls THUNK through `g_call0_guard`, inspects `non_local_exit_get`, verifies that calling
   `env->funcall` while an exit is pending short-circuits cleanly without executing Lisp or
   unwinding C, clears the pending exit, verifies `HostDropGuard` and `ScopedLispRoot` counts
   already balanced BEFORE returning from C, and returns a structured status list:
     (exit-kind sym-or-tag data-or-val hc-delta balance-ok short-circuit-ok) */
static emacs_value F_au3_mod_probe_intercept(
    emacs_env *env,
    ptrdiff_t nargs,
    emacs_value *args,
    void *data
) {
    (void)nargs;
    (void)data;
    g_c_entered += 1;
    size_t hc0 = 0, hd0 = 0, rc0 = 0, rd0 = 0;
    g_drop_counts(&hc0, &hd0, &rc0, &rd0);

    emacs_value val = g_call0_guard(env, args[0]);

    size_t hc1 = 0, hd1 = 0, rc1 = 0, rd1 = 0;
    g_drop_counts(&hc1, &hd1, &rc1, &rd1);

    emacs_value sym = NULL;
    emacs_value payload = NULL;
    enum emacs_funcall_exit code = env->non_local_exit_get(env, &sym, &payload);

    int short_circuit_ok = 1;
    if (code != emacs_funcall_exit_return) {
        /* While exit is pending, `funcall` must not run or overwrite the pending exit. */
        emacs_value plus_sym = env->intern(env, "+");
        emacs_value dummy = env->funcall(env, plus_sym, 0, NULL);
        (void)dummy;
        enum emacs_funcall_exit after = env->non_local_exit_check(env);
        if (after != code) {
            short_circuit_ok = 0;
        }
        env->non_local_exit_clear(env);
    }

    int balance_ok = (hc1 == hd1)
        && ((hc1 - hc0) == 1)
        && ((hd1 - hd0) == 1)
        && ((rc1 - rc0) == (rd1 - rd0))
        && ((rc1 - rc0) >= 1);
    emacs_value kind_sym;
    emacs_value out_first;
    emacs_value out_second;
    if (code == emacs_funcall_exit_return) {
        kind_sym = env->intern(env, "return");
        out_first = val;
        out_second = env->intern(env, "nil");
    } else if (code == emacs_funcall_exit_signal) {
        kind_sym = env->intern(env, "signal");
        out_first = sym;
        out_second = payload;
    } else {
        kind_sym = env->intern(env, "throw");
        out_first = sym;
        out_second = payload;
    }

    emacs_value list_fn = env->intern(env, "list");
    emacs_value items[6];
    items[0] = kind_sym;
    items[1] = out_first;
    items[2] = out_second;
    items[3] = env->make_integer(env, (intmax_t)(hc1 - hc0));
    items[4] = env->intern(env, balance_ok ? "t" : "nil");
    items[5] = env->intern(env, short_circuit_ok ? "t" : "nil");
    emacs_value ret = env->funcall(env, list_fn, 6, items);
    g_c_exited += 1;
    return ret;
}

/* (au3-mod-check-process-input) -> calls `env->process_input(env)` and returns `'continue`
   or leaves the pending `quit` exit on `env` for the VM to propagate on return. */
static emacs_value F_au3_mod_check_process_input(
    emacs_env *env,
    ptrdiff_t nargs,
    emacs_value *args,
    void *data
) {
    (void)nargs;
    (void)args;
    (void)data;
    g_c_entered += 1;
    enum emacs_process_input_result r = env->process_input(env);
    g_c_exited += 1;
    if (r == emacs_process_input_quit) {
        return NULL;
    }
    return env->intern(env, "continue");
}

/* (au3-mod-originate MODE SYM-OR-TAG DATA-OR-VAL) */
static emacs_value F_au3_mod_originate(
    emacs_env *env,
    ptrdiff_t nargs,
    emacs_value *args,
    void *data
) {
    (void)nargs;
    (void)data;
    g_c_entered += 1;
    emacs_value sig_sym = env->intern(env, "signal");
    if (env->eq(env, args[0], sig_sym)) {
        env->non_local_exit_signal(env, args[1], args[2]);
    } else {
        env->non_local_exit_throw(env, args[1], args[2]);
    }
    g_c_exited += 1;
    return NULL;
}

static void bind_mod_fn(
    emacs_env *env,
    const char *name,
    ptrdiff_t min_arity,
    ptrdiff_t max_arity,
    emacs_function fn
) {
    emacs_value fval = env->make_function(env, min_arity, max_arity, fn, name, NULL);
    emacs_value defalias = env->intern(env, "defalias");
    emacs_value sym = env->intern(env, name);
    emacs_value args[2] = { sym, fval };
    env->funcall(env, defalias, 2, args);
}

int emacs_module_init(struct emacs_runtime *ert) {
    if ((size_t)ert->size < sizeof(*ert)) {
        return 1;
    }
    emacs_env *env = ert->get_environment(ert);
    if ((size_t)env->size < sizeof(*env)) {
        return 2;
    }
    g_drop_counts = (host_drop_counts_fn)dlsym(RTLD_DEFAULT, "femacs_vm_host_drop_counts");
    g_call0_guard = (module_call0_guard_fn)dlsym(
        RTLD_DEFAULT,
        "femacs_vm_module_call0_with_host_guard"
    );
    if (!g_drop_counts || !g_call0_guard) {
        return 3;
    }
    bind_mod_fn(env, "au3-mod-counts", 0, 0, F_au3_mod_counts);
    bind_mod_fn(env, "au3-mod-c-frame-stats", 0, 0, F_au3_mod_c_frame_stats);
    bind_mod_fn(env, "au3-mod-inner-guard", 1, 1, F_au3_mod_inner_guard);
    bind_mod_fn(env, "au3-mod-outer-transparent", 1, 1, F_au3_mod_outer_transparent);
    bind_mod_fn(env, "au3-mod-probe-intercept", 1, 1, F_au3_mod_probe_intercept);
    bind_mod_fn(env, "au3-mod-check-process-input", 0, 0, F_au3_mod_check_process_input);
    bind_mod_fn(env, "au3-mod-originate", 3, 3, F_au3_mod_originate);
    return 0;
}
"""

ELISP_SUITE = r"""
;;; au3-suite.el --- U3 non-local exit equivalence suite  -*- lexical-binding: t; -*-

(defvar au3-dyn-state 'base)
(defvar au3-events nil)

(define-error 'au3-custom-err "AU3 custom error")

(defun au3-rec (ev)
  (setq au3-events (cons ev au3-events)))

(defun au3-deep-nested-list (depth)
  (let ((acc '(leaf)))
    (dotimes (i depth)
      (setq acc (list i acc (cons 'pair i))))
    acc))

(defun au3-recursive-print-unwind-probe ()
  (let ((deep (au3-deep-nested-list 28))
        (count 0))
    (catch 'au3-print-abort
      (prin1 deep
             (lambda (_ch)
               (setq count (1+ count))
               (when (= count 12)
                 (let ((s (prin1-to-string (au3-deep-nested-list 18))))
                   (when (> (length s) 10)
                     (throw 'au3-print-abort 'print-unwound))))))
      'not-reached)))

(defun au3-s1-normal ()
  (setq au3-events nil)
  (let ((val
         (let ((au3-dyn-state 'outer))
           (au3-rec (list 'enter au3-dyn-state))
           (unwind-protect
               (catch 'au3-outer
                 (condition-case res
                     (let ((au3-dyn-state 'inner))
                       (unwind-protect
                           (progn
                             (au3-rec (list 'body au3-dyn-state))
                             42)
                         (au3-rec (list 'inner-cleanup au3-dyn-state))))
                   (error (list 'unexpected res))
                   (:success
                    (au3-rec (list 'on-success res au3-dyn-state))
                    (+ res 8))))
             (au3-rec (list 'outer-cleanup au3-dyn-state))))))
    (list val au3-dyn-state (nreverse au3-events))))

(defun au3-s2-nested-catch ()
  (setq au3-events nil)
  (let ((val
         (catch 'au3-outer-tag
           (let ((au3-dyn-state 'outer-let))
             (unwind-protect
                 (catch 'au3-inner-tag
                   (let ((au3-dyn-state 'inner-let))
                     (unwind-protect
                         (progn
                           (au3-rec (list 'before-throw au3-dyn-state))
                           (throw 'au3-outer-tag (list 'escaped au3-dyn-state)))
                       (au3-rec (list 'inner-uwp au3-dyn-state)))))
               (au3-rec (list 'outer-uwp au3-dyn-state)))))))
    (list val au3-dyn-state (nreverse au3-events))))

(defun au3-s3-condition-propagate ()
  (setq au3-events nil)
  (let ((val
         (condition-case outer-err
             (handler-bind
                 ((au3-custom-err
                   (lambda (err)
                     (au3-rec (list 'handler-bind-seen (car err) au3-dyn-state)))))
               (let ((au3-dyn-state 'outer-cond))
                 (unwind-protect
                     (condition-case inner-err
                         (let ((au3-dyn-state 'inner-cond))
                           (unwind-protect
                               (progn
                                 (au3-rec (list 'signaling au3-dyn-state))
                                 (signal 'au3-custom-err '(payload-7)))
                             (au3-rec (list 'inner-cond-uwp au3-dyn-state))))
                       (arith-error (list 'wrong-inner inner-err)))
                   (au3-rec (list 'outer-cond-uwp au3-dyn-state)))))
           (au3-custom-err
            (au3-rec (list 'caught-custom (cdr outer-err) au3-dyn-state))
            (list 'handled (car outer-err) (cdr outer-err))))))
    (list val au3-dyn-state (nreverse au3-events))))

(defun au3-s4-quit ()
  (setq au3-events nil)
  (let ((val
         (condition-case q
             (unwind-protect
                 (progn
                   (let ((inhibit-quit t))
                     (setq quit-flag t)
                     (au3-rec (list 'while-inhibited quit-flag)))
                   ;; Leaving `inhibit-quit` scope or evaluating further triggers `quit`.
                   (eval '(+ 1 2))
                   'never-after-quit)
               (au3-rec (list 'quit-uwp quit-flag)))
           ((debug quit)
            (au3-rec (list 'caught-quit (car q) quit-flag))
            'quit-recovered))))
    (list val quit-flag (nreverse au3-events))))

(defun au3-s5-reexit-throw-gc-print ()
  (setq au3-events nil)
  (let ((val
         (catch 'au3-cleanup-escape
           (unwind-protect
               (unwind-protect
                   (progn
                     (au3-rec 'body-signal-first)
                     (signal 'au3-custom-err '(initial-error)))
                 (let ((live (list (copy-sequence "gc-rooted-during-cleanup") 99))
                       (pr (au3-recursive-print-unwind-probe)))
                   (garbage-collect)
                   (au3-rec (list 'inner-cleanup-gc (car live) (cadr live) pr))
                   (throw 'au3-cleanup-escape (list 'cleanup-won (car live) pr))))
             (au3-rec 'outermost-uwp-after-override)))))
    (list val au3-dyn-state (nreverse au3-events))))

(defun au3-s6-reexit-signal-in-cleanup ()
  (setq au3-events nil)
  (let ((val
         (condition-case err
             (catch 'au3-superseded-tag
               (unwind-protect
                   (unwind-protect
                       (progn
                         (au3-rec 'body-throw-first)
                         (throw 'au3-superseded-tag 'stale-throw))
                     (au3-rec 'cleanup-signals-arith)
                     (/ 1 0))
                 (au3-rec 'outer-uwp-on-arith)))
           (arith-error
            (au3-rec (list 'caught-arith (car err)))
            'arith-overrode-throw))))
    (list val au3-dyn-state (nreverse au3-events))))

(defun au3-s7-post-error-next-call ()
  (setq au3-events nil)
  (let ((d0 (internal--labeled-narrow-to-region-check)))
    (ignore d0)
    (let ((r1 (condition-case e
                  (let ((au3-dyn-state 'boom1))
                    (signal 'au3-custom-err '(one)))
                (au3-custom-err (list 'ok1 (cdr e) au3-dyn-state))))
          (r2 (catch 't2
                (let ((au3-dyn-state 'boom2))
                  (throw 't2 (list 'ok2 au3-dyn-state)))))
          (r3 (condition-case _q
                  (progn
                    (setq quit-flag t)
                    (eval '(+ 1 1)))
                ((debug quit) (list 'ok3 quit-flag au3-dyn-state))))
          (r4 (let ((au3-dyn-state 'clean-next))
                (list 'normal-next (+ 20 22) au3-dyn-state quit-flag))))
      (list r1 r2 r3 r4 au3-dyn-state))))

(defun au3-run-all-direct ()
  (list
   (cons 's1 (au3-s1-normal))
   (cons 's2 (au3-s2-nested-catch))
   (cons 's3 (au3-s3-condition-propagate))
   (cons 's4 (au3-s4-quit))
   (cons 's5 (au3-s5-reexit-throw-gc-print))
   (cons 's6 (au3-s6-reexit-signal-in-cleanup))
   (cons 's7 (au3-s7-post-error-next-call))))

(defun au3-run-all-via-module ()
  (list
   (cons 's1 (au3-mod-outer-transparent #'au3-s1-normal))
   (cons 's2 (au3-mod-outer-transparent #'au3-s2-nested-catch))
   (cons 's3 (au3-mod-outer-transparent #'au3-s3-condition-propagate))
   (cons 's4 (au3-mod-outer-transparent #'au3-s4-quit))
   (cons 's5 (au3-mod-outer-transparent #'au3-s5-reexit-throw-gc-print))
   (cons 's6 (au3-mod-outer-transparent #'au3-s6-reexit-signal-in-cleanup))
   (cons 's7 (au3-mod-outer-transparent #'au3-s7-post-error-next-call))))

(provide 'au3-suite)
"""


def run_cmd(
    argv: list[str],
    *,
    cwd: Path = REPO,
    env: dict[str, str] | None = None,
    timeout: float = 60.0,
) -> subprocess.CompletedProcess[bytes]:
    return subprocess.run(
        argv,
        cwd=str(cwd),
        env=env,
        capture_output=True,
        timeout=timeout,
        check=False,
    )


def check_cmd_ok(
    argv: list[str],
    *,
    cwd: Path = REPO,
    env: dict[str, str] | None = None,
    timeout: float = 60.0,
    label: str = "command",
) -> str:
    res = run_cmd(argv, cwd=cwd, env=env, timeout=timeout)
    out = res.stdout.decode("utf-8", errors="replace")
    err = res.stderr.decode("utf-8", errors="replace")
    if res.returncode != 0:
        raise RuntimeError(
            f"{label} failed (exit={res.returncode}):\nSTDOUT:\n{out[-2000:]}\nSTDERR:\n{err[-2000:]}"
        )
    return out


def build_c_module(work_dir: Path) -> Path:
    header_path = module_api.render_module_api(REPO)
    c_file = work_dir / "au3_mod.c"
    dylib_file = work_dir / "au3_mod.dylib"
    c_file.write_text(C_MODULE_SOURCE, encoding="utf-8")
    check_cmd_ok(
        [
            "clang",
            "-O2",
            "-Wall",
            "-Wextra",
            "-Werror",
            "-dynamiclib",
            "-Wl,-undefined,dynamic_lookup",
            f"-I{header_path.parent}",
            str(c_file),
            "-o",
            str(dylib_file),
        ],
        timeout=30.0,
        label="clang build au3_mod.dylib",
    )
    return dylib_file


def build_wrong_abi_eln(work_dir: Path, abi_literal_bytes: bytes) -> Path:
    abi_file = work_dir / "abi-literal.bin"
    abi_file.write_bytes(abi_literal_bytes)
    wrong_eln = work_dir / "legacy_pre_u3_wrong_abi.eln"
    env = os.environ.copy()
    env["FEMACS_RUNTIME_ABI_LITERAL_INPUT"] = str(abi_file)
    check_cmd_ok(
        [
            "rustc",
            "--edition=2024",
            "--crate-type=cdylib",
            "--cfg",
            'negative="wrong-abi"',
            str(REPO / "tools/fixtures/runtime-native-unit.rs"),
            "-o",
            str(wrong_eln),
        ],
        env=env,
        timeout=30.0,
        label="rustc build wrong-abi .eln",
    )
    return wrong_eln


def run_batch_json(
    binary: Path,
    lisp_expr: str,
    *,
    extra_env: dict[str, str] | None = None,
    timeout: float = 90.0,
) -> object:
    env = os.environ.copy()
    if extra_env:
        env.update(extra_env)
    wrapped = (
        "(progn "
        "(require 'json) "
        f"(let ((au3--out {lisp_expr})) "
        "(princ \"AU3_JSON_BEGIN\\n\") "
        "(princ (json-serialize au3--out :false-object :json-false :null-object nil)) "
        "(princ \"\\nAU3_JSON_END\\n\")))"
    )
    res = run_cmd(
        [str(binary), "--batch", "-Q", "--eval", wrapped],
        env=env,
        timeout=timeout,
    )
    combined = (
        res.stdout.decode("utf-8", errors="replace")
        + "\n"
        + res.stderr.decode("utf-8", errors="replace")
    )
    if res.returncode != 0:
        raise RuntimeError(
            f"{binary} exited {res.returncode}:\n{combined[-3000:]}"
        )
    begin_marker = "AU3_JSON_BEGIN\n"
    end_marker = "\nAU3_JSON_END"
    b_idx = combined.find(begin_marker)
    e_idx = combined.find(end_marker)
    if b_idx < 0 or e_idx < 0 or e_idx <= b_idx:
        raise RuntimeError(f"missing JSON markers from {binary}:\n{combined[-3000:]}")
    payload = combined[b_idx + len(begin_marker) : e_idx].strip()
    return json.loads(payload)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument(
        "--terminal-bin",
        type=Path,
        default=REPO / "build/terminal/port/src/emacs",
    )
    ap.add_argument(
        "--cocoa-app",
        type=Path,
        default=REPO / "dist/macos/Femacs.app",
    )
    ap.add_argument(
        "--native-bin",
        type=Path,
        default=REPO / "build/runtime/u3-native/dist/terminal/src/emacs",
    )
    ap.add_argument(
        "--native-checking-bin",
        type=Path,
        default=REPO / "build/runtime/u3-native/checking/port/src/emacs",
    )
    ap.add_argument("--libgccjit-prefix", type=Path, default=Path("/opt/homebrew/opt/libgccjit"))
    ap.add_argument("--json-out", type=Path, help="Optional JSON report path")
    args = ap.parse_args()

    cocoa_bin = args.cocoa_app / "Contents/MacOS/Femacs"
    for label, path in (
        ("terminal-bin", args.terminal_bin),
        ("cocoa-bin", cocoa_bin),
        ("native-bin", args.native_bin),
        ("native-checking-bin", args.native_checking_bin),
    ):
        if not path.exists():
            raise FileNotFoundError(f"missing required {label}: {path}")

    native_env = {}
    libgccjit_lib = args.libgccjit_prefix / "lib/gcc/current"
    if libgccjit_lib.is_dir():
        native_env["LIBRARY_PATH"] = str(libgccjit_lib)
        native_env["DYLD_FALLBACK_LIBRARY_PATH"] = str(libgccjit_lib)

    results: list[dict] = []
    failures = 0

    def run_case(name: str, fn) -> None:
        nonlocal failures
        t0 = time.monotonic()
        try:
            detail = fn()
            dt = time.monotonic() - t0
            results.append({"id": name, "status": "passed", "seconds": round(dt, 6), "detail": detail})
            print(f"CASE {name} status=passed seconds={dt:.4f}")
        except Exception as exc:
            dt = time.monotonic() - t0
            failures += 1
            results.append({"id": name, "status": "failed", "seconds": round(dt, 6), "error": str(exc)})
            print(f"CASE {name} status=failed seconds={dt:.4f} error={exc}", file=sys.stderr)

    with tempfile.TemporaryDirectory(prefix="au3-nonlocal-") as tmp_str:
        work = Path(tmp_str)
        dylib_path = build_c_module(work)
        suite_el = work / "au3-suite.el"
        # Replace `internal--labeled-narrow-to-region-check` with portable `recursion-depth`.
        suite_text = ELISP_SUITE.lstrip().replace(
            "(internal--labeled-narrow-to-region-check)",
            "(recursion-depth)",
        )
        suite_el.write_text(suite_text, encoding="utf-8")

        def case_unsafe_and_product_gates() -> dict:
            gate_json = work / "u3-unsafe.json"
            check_cmd_ok(
                [
                    sys.executable,
                    str(REPO / "tools/check-unsafe-boundary.py"),
                    "--require-zero-active-stitching",
                    "--out",
                    str(gate_json),
                    "--binary",
                    str(args.terminal_bin),
                    "--kind",
                    "terminal",
                    "--binary",
                    str(cocoa_bin),
                    "--kind",
                    "cocoa",
                ],
                timeout=60.0,
                label="check-unsafe-boundary",
            )
            check_cmd_ok(
                ["codesign", "--verify", "--deep", "--strict", str(args.cocoa_app)],
                timeout=30.0,
                label="codesign Femacs.app",
            )
            frame_flags_out = REPO / "build/acceptance/au3-frame-flags"
            if frame_flags_out.exists():
                import shutil
                shutil.rmtree(frame_flags_out)
            try:
                check_cmd_ok(
                    [
                        sys.executable,
                        str(REPO / "tools/test-cocoa-frame-flags.py"),
                        "--out",
                        str(frame_flags_out),
                    ],
                    timeout=30.0,
                    label="test-cocoa-frame-flags",
                )
            finally:
                if frame_flags_out.exists():
                    import shutil
                    shutil.rmtree(frame_flags_out)
            exported_verified = {}
            for tag, bin_path in (
                ("terminal", args.terminal_bin),
                ("cocoa", cocoa_bin),
                ("native", args.native_bin),
                ("native_checking", args.native_checking_bin),
            ):
                nm_out = check_cmd_ok(
                    ["nm", "-gU", str(bin_path)],
                    timeout=30.0,
                    label=f"nm -gU {tag}",
                )
                for sym in (
                    "_femacs_vm_host_drop_counts",
                    "_femacs_vm_module_call0_with_host_guard",
                ):
                    if sym not in nm_out:
                        raise AssertionError(f"{tag} binary missing exported symbol {sym}")
                exported_verified[tag] = True
            gate_data = json.loads(gate_json.read_text(encoding="utf-8"))
            return {
                "active_stitching_count": gate_data.get("active_stitching_count", 0),
                "exported_probe_symbols": exported_verified,
            }

        compiled_eln_holder: dict[str, Path] = {}

        def case_legacy_eln_rejected_and_u3_recompiled() -> dict:
            abi_info = run_batch_json(
                args.native_bin,
                "(vector comp-abi-hash (prin1-to-string comp-abi-hash))",
                extra_env=native_env,
            )
            abi_hash, abi_prin1 = abi_info[0], abi_info[1]
            if not isinstance(abi_hash, str) or len(abi_hash) < 6:
                raise AssertionError(f"unexpected comp-abi-hash: {abi_info!r}")

            wrong_eln = build_wrong_abi_eln(work, abi_prin1.encode("utf-8"))

            rejections = {}
            for leg, bin_path in (
                ("normal", args.native_bin),
                ("checking", args.native_checking_bin),
            ):
                rej = run_batch_json(
                    bin_path,
                    f"(condition-case err "
                    f"    (progn (native-elisp-load {json.dumps(str(wrong_eln))}) "
                    f"           (vector \"unexpected-load\" \"none\")) "
                    f"  (error (vector \"rejected\" (symbol-name (car err)))))",
                    extra_env=native_env,
                )
                if rej != ["rejected", "native-lisp-file-inconsistent"]:
                    raise AssertionError(f"{leg} did not reject wrong-abi .eln properly: {rej!r}")
                rejections[leg] = rej[1]

            eln_cache = work / "eln-cache"
            eln_cache.mkdir(parents=True, exist_ok=True)
            eln_out_info = run_batch_json(
                args.native_bin,
                f"(progn "
                f"  (startup-redirect-eln-cache {json.dumps(str(eln_cache))}) "
                f"  (let ((eln (native-compile {json.dumps(str(suite_el))}))) "
                f"    (load eln nil t t) "
                f"    (vector eln "
                f"            (and (subr-native-elisp-p (symbol-function 'au3-s1-normal)) t) "
                f"            (and (subr-native-elisp-p (symbol-function 'au3-s5-reexit-throw-gc-print)) t))))",
                extra_env=native_env,
                timeout=120.0,
            )
            eln_path = Path(eln_out_info[0])
            if not eln_path.is_file() or eln_out_info[1] is not True or eln_out_info[2] is not True:
                raise AssertionError(f"formal native-compile failed: {eln_out_info!r}")
            compiled_eln_holder["path"] = eln_path

            nm_undef = check_cmd_ok(["nm", "-u", str(eln_path)], timeout=20.0, label="nm -u u3 .eln")
            for forbidden in ("setjmp", "longjmp"):
                if forbidden in nm_undef:
                    raise AssertionError(
                        f"newly compiled U3 .eln still imports `{forbidden}`:\n{nm_undef}"
                    )
            nm_defined = check_cmd_ok(["nm", "-gU", str(eln_path)], timeout=20.0, label="nm -gU u3 .eln")
            for required in ("_freloc_link_table", "_freloc_hash", "_top_level_run"):
                if required not in nm_defined:
                    raise AssertionError(f"U3 .eln missing `{required}`:\n{nm_defined}")

            eln_cache_chk = work / "eln-cache-checking"
            eln_cache_chk.mkdir(parents=True, exist_ok=True)
            eln_chk_str = run_batch_json(
                args.native_checking_bin,
                f"(progn "
                f"  (startup-redirect-eln-cache {json.dumps(str(eln_cache_chk))}) "
                f"  (native-compile {json.dumps(str(suite_el))}))",
                extra_env=native_env,
                timeout=120.0,
            )
            eln_chk_path = Path(eln_chk_str)
            if not eln_chk_path.is_file():
                raise AssertionError(f"checking native-compile failed: {eln_chk_str!r}")
            compiled_eln_holder["checking_path"] = eln_chk_path

            return {
                "comp_abi_hash": abi_hash,
                "legacy_rejections": rejections,
                "compiled_eln": eln_path.name,
                "compiled_checking_eln": eln_chk_path.name,
                "zero_setjmp_imports": True,
            }

        def case_four_tier_equivalence() -> dict:
            eln_path = compiled_eln_holder.get("path")
            eln_chk_path = compiled_eln_holder.get("checking_path")
            if eln_path is None or eln_chk_path is None:
                raise RuntimeError("compiled .eln missing from previous step")

            suite_elc = work / "au3-suite.elc"
            run_batch_json(
                args.terminal_bin,
                f"(and (byte-compile-file {json.dumps(str(suite_el))}) t)",
            )
            if not suite_elc.is_file():
                raise AssertionError("byte-compile-file did not emit au3-suite.elc")

            def eval_tier(
                binary: Path,
                load_target: Path,
                expect_pred: str,
                extra_env: dict[str, str] | None = None,
            ) -> dict:
                expr = (
                    f"(progn "
                    f"  (module-load {json.dumps(str(dylib_path))}) "
                    f"  (load {json.dumps(str(load_target))} nil t t) "
                    f"  (unless ({expect_pred} (symbol-function 'au3-s1-normal)) "
                    f"    (error \"unexpected function representation for %s\" '{expect_pred})) "
                    f"  (let ((direct (prin1-to-string (au3-run-all-direct))) "
                    f"        (via-mod (prin1-to-string (au3-run-all-via-module))) "
                    f"        (cstats (au3-mod-c-frame-stats)) "
                    f"        (hcounts (au3-mod-counts))) "
                    f"    (vector direct via-mod (prin1-to-string cstats) (prin1-to-string hcounts))))"
                )
                vec = run_batch_json(binary, expr, extra_env=extra_env, timeout=90.0)
                return {
                    "direct": vec[0],
                    "via_module": vec[1],
                    "c_frame_stats": vec[2],
                    "host_counts": vec[3],
                }

            tiers = {
                "terminal_interp": eval_tier(args.terminal_bin, suite_el, "interpreted-function-p"),
                "terminal_bytecode": eval_tier(args.terminal_bin, suite_elc, "byte-code-function-p"),
                "cocoa_interp": eval_tier(cocoa_bin, suite_el, "interpreted-function-p"),
                "cocoa_bytecode": eval_tier(cocoa_bin, suite_elc, "byte-code-function-p"),
                "native_normal_eln": eval_tier(
                    args.native_bin, eln_path, "subr-native-elisp-p", extra_env=native_env
                ),
                "native_checking_eln": eval_tier(
                    args.native_checking_bin, eln_chk_path, "subr-native-elisp-p", extra_env=native_env
                ),
            }

            canonical = tiers["terminal_interp"]["direct"]
            for tier_name, outcome in tiers.items():
                if outcome["direct"] != canonical:
                    raise AssertionError(
                        f"direct trace mismatch on {tier_name}:\n"
                        f"expected: {canonical}\nactual:   {outcome['direct']}"
                    )
                if outcome["via_module"] != canonical:
                    raise AssertionError(
                        f"C dynamic-module trace mismatch on {tier_name}:\n"
                        f"expected: {canonical}\nactual:   {outcome['via_module']}"
                    )
                # Verify c_entered == c_exited (third element of `(entered exited diff 0)` is 0).
                if not outcome["c_frame_stats"].endswith(" 0 0)"):
                    raise AssertionError(
                        f"unbalanced C FFI frames on {tier_name}: {outcome['c_frame_stats']}"
                    )

            return {
                "tiers_verified": sorted(tiers.keys()),
                "canonical_trace_length": len(canonical),
                "sample_trace_prefix": canonical[:240],
            }

        def case_host_drop_exact_once_and_ffi_isolation() -> dict:
            eln_path = compiled_eln_holder.get("path")
            if eln_path is None:
                raise RuntimeError("compiled .eln missing")

            probe_el = work / "au3-drop-probe.el"
            probe_el.write_text(
                r"""
;;; au3-drop-probe.el --- HostDropGuard + FFI boundary probe  -*- lexical-binding: t; -*-

(define-error 'au3-probe-err "AU3 probe error")

(defun au3-probe-ok ()
  (+ 19 23))

(defun au3-probe-sig ()
  (unwind-protect
      (signal 'au3-probe-err '(boom-data))
    (garbage-collect)))

(defun au3-probe-thr ()
  (unwind-protect
      (throw 'au3-probe-tag 'thrown-payload)
    (garbage-collect)))

(defun au3-probe-reexit ()
  (unwind-protect
      (signal 'au3-probe-err '(first))
    (garbage-collect)
    (throw 'au3-probe-tag 'second-from-cleanup)))

(defun au3-probe-mod-originate-sig ()
  (au3-mod-originate 'signal 'au3-probe-err '(from-c-module)))

(defun au3-probe-mod-originate-thr ()
  (au3-mod-originate 'throw 'au3-probe-tag 'from-c-throw))

(defun au3-probe-quit-via-process-input ()
  (setq quit-flag t)
  (au3-mod-check-process-input))

(defun au3-run-drop-matrix ()
  (let* ((c0 (au3-mod-counts))
         (r-ok (au3-mod-probe-intercept #'au3-probe-ok))
         (r-sig (au3-mod-probe-intercept #'au3-probe-sig))
         (r-thr (au3-mod-probe-intercept #'au3-probe-thr))
         (r-reexit (au3-mod-probe-intercept #'au3-probe-reexit))
         (r-csig (au3-mod-probe-intercept #'au3-probe-mod-originate-sig))
         (r-cthr (au3-mod-probe-intercept #'au3-probe-mod-originate-thr))
         (r-quit (au3-mod-probe-intercept #'au3-probe-quit-via-process-input))
         ;; Also pass an ephemeral freshly constructed closure with zero external Lisp bindings
         ;; so only `ScopedLispRoot` + stack hold it across GC during unwind!
         (r-ephem (au3-mod-probe-intercept
                   (let ((secret (copy-sequence "ephemeral-closure-alive")))
                     (lambda ()
                       (unwind-protect
                           (progn
                             (garbage-collect)
                             (throw 'au3-probe-tag secret))
                         (garbage-collect))))))
         ;; Background Lisp thread signaling an uncaught error + `unwind-protect` cleanup:
         ;; proves `run_thread` (`extern "C"`) catches/records thread error cleanly without
         ;; unwinding across `pthread_create`!
         (bg-uwp-ran nil)
         (bg-th (make-thread
                 (lambda ()
                   (unwind-protect
                       (au3-mod-originate 'signal 'au3-probe-err '(bg-thread-signal))
                     (setq bg-uwp-ran 'bg-cleaned)))))
         (r-bg (list 'bg-done (thread-join bg-th) (thread-last-error t) bg-uwp-ran))
         ;; Immediately verify a clean subsequent normal call after all non-local exits:
         (r-after (au3-mod-probe-intercept #'au3-probe-ok))
         (c1 (au3-mod-counts))
         (fstats (au3-mod-c-frame-stats)))
    (list
     (cons 'c0 c0)
     (cons 'c1 c1)
     (cons 'fstats fstats)
     (cons 'ok r-ok)
     (cons 'sig r-sig)
     (cons 'thr r-thr)
     (cons 'reexit r-reexit)
     (cons 'csig r-csig)
     (cons 'cthr r-cthr)
     (cons 'quit r-quit)
     (cons 'ephem r-ephem)
     (cons 'bg r-bg)
     (cons 'after r-after))))
""",
                encoding="utf-8",
            )
            probe_el.write_text(probe_el.read_text(encoding="utf-8").lstrip(), encoding="utf-8")
            probe_elc = work / "au3-drop-probe.elc"
            run_batch_json(
                args.terminal_bin,
                f"(and (byte-compile-file {json.dumps(str(probe_el))}) t)",
            )
            eln_cache2 = work / "eln-cache-probe"
            eln_cache2.mkdir(parents=True, exist_ok=True)
            probe_eln_str = run_batch_json(
                args.native_bin,
                f"(progn "
                f"  (startup-redirect-eln-cache {json.dumps(str(eln_cache2))}) "
                f"  (native-compile {json.dumps(str(probe_el))}))",
                extra_env=native_env,
                timeout=120.0,
            )
            probe_eln = Path(probe_eln_str)

            eln_cache2_chk = work / "eln-cache-probe-checking"
            eln_cache2_chk.mkdir(parents=True, exist_ok=True)
            probe_eln_chk_str = run_batch_json(
                args.native_checking_bin,
                f"(progn "
                f"  (startup-redirect-eln-cache {json.dumps(str(eln_cache2_chk))}) "
                f"  (native-compile {json.dumps(str(probe_el))}))",
                extra_env=native_env,
                timeout=120.0,
            )
            probe_eln_chk = Path(probe_eln_chk_str)

            verified_modes = {}
            for mode_name, binary, target_file, env_map in (
                ("interp", args.terminal_bin, probe_el, None),
                ("bytecode", args.terminal_bin, probe_elc, None),
                ("native_normal", args.native_bin, probe_eln, native_env),
                ("native_checking", args.native_checking_bin, probe_eln_chk, native_env),
            ):
                out_str = run_batch_json(
                    binary,
                    f"(progn "
                    f"  (module-load {json.dumps(str(dylib_path))}) "
                    f"  (load {json.dumps(str(target_file))} nil t t) "
                    f"  (prin1-to-string (au3-run-drop-matrix)))",
                    extra_env=env_map,
                    timeout=90.0,
                )
                expected_fragments = (
                    "(ok return 42 nil 1 t t)",
                    "(sig signal au3-probe-err (boom-data) 1 t t)",
                    "(thr throw au3-probe-tag thrown-payload 1 t t)",
                    "(reexit throw au3-probe-tag second-from-cleanup 1 t t)",
                    "(csig signal au3-probe-err (from-c-module) 1 t t)",
                    "(cthr throw au3-probe-tag from-c-throw 1 t t)",
                    "(quit signal quit nil 1 t t)",
                    "(ephem throw au3-probe-tag \"ephemeral-closure-alive\" 1 t t)",
                    "(bg bg-done nil (au3-probe-err bg-thread-signal) bg-cleaned)",
                    "(after return 42 nil 1 t t)",
                )
                for frag in expected_fragments:
                    if frag not in out_str:
                        raise AssertionError(
                            f"mode {mode_name} missing expected probe record `{frag}`:\n{out_str}"
                        )
                # Parse c0, c1, fstats via Lisp to verify exact total delta == 9 and c_entered == c_exited!
                nums = run_batch_json(
                    binary,
                    f"(progn "
                    f"  (module-load {json.dumps(str(dylib_path))}) "
                    f"  (load {json.dumps(str(target_file))} nil t t) "
                    f"  (let* ((res (au3-run-drop-matrix)) "
                    f"         (c0 (cdr (assq 'c0 res))) "
                    f"         (c1 (cdr (assq 'c1 res))) "
                    f"         (fs (cdr (assq 'fstats res)))) "
                    f"    (vector (- (nth 0 c1) (nth 0 c0)) "
                    f"            (- (nth 1 c1) (nth 1 c0)) "
                    f"            (= (nth 0 c1) (nth 1 c1)) "
                    f"            (= (- (nth 2 c1) (nth 2 c0)) (- (nth 3 c1) (nth 3 c0))) "
                    f"            (nth 0 fs) "
                    f"            (nth 1 fs) "
                    f"            (nth 2 fs))))",
                    extra_env=env_map,
                    timeout=90.0,
                )
                # 9 `au3-mod-probe-intercept` calls => exact 9 HostDropGuards created & 9 dropped!
                if nums[:4] != [9, 9, True, True] or nums[6] != 0 or nums[4] < 9:
                    raise AssertionError(f"mode {mode_name} drop/FFI count mismatch: {nums!r}")
                verified_modes[mode_name] = {
                    "host_created_delta": nums[0],
                    "host_dropped_delta": nums[1],
                    "c_frames_entered": nums[4],
                    "c_frames_exited": nums[5],
                }

            return verified_modes

        run_case("u3_unsafe_and_product_gates", case_unsafe_and_product_gates)
        run_case(
            "u3_legacy_eln_rejected_and_u3_recompiled",
            case_legacy_eln_rejected_and_u3_recompiled,
        )
        run_case("u3_four_tier_nonlocal_equivalence", case_four_tier_equivalence)
        run_case(
            "u3_host_drop_exact_once_and_ffi_isolation",
            case_host_drop_exact_once_and_ffi_isolation,
        )

    summary = {
        "suite": "AU3-nonlocal-exit",
        "passed": len(results) - failures,
        "failed": failures,
        "cases": results,
    }
    if args.json_out:
        args.json_out.parent.mkdir(parents=True, exist_ok=True)
        args.json_out.write_text(json.dumps(summary, indent=2) + "\n", encoding="utf-8")

    print(
        f"SUMMARY suite=AU3 passed={summary['passed']} failed={summary['failed']}"
    )
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
