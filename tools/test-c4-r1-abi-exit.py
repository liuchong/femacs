#!/usr/bin/env python3
"""Targeted C4-R1 ABI-safe host-exit closure suite for `dist/lisp`.

Compiles a pure public-ABI C dynamic module (`emacs-module.h` only, zero internal VM symbols)
and runs real `femacs-lisp --batch` subprocesses covering every host-exit crossing scenario
raised in `[FEMACS-C4-A4-REVIEW]`:
  1. C dynamic module callback invoking `(kill-emacs 7)` (skipped Lisp `unwind-protect` /
     `condition-case`, C cleanup runs cleanly, exit code 7).
  2. C module calling `env->non_local_exit_clear(env)` cannot swallow `host_exit_pending`.
  3. Nested Lisp -> C module outer -> Lisp -> C module inner -> `(kill-emacs 17)`: inner then
     outer C cleanup both run cleanly, exit code 17.
  4. C module `emacs_module_init` invoking `(kill-emacs 11)`: init cleanup runs cleanly, exit 11.
  5. Lisp subthread `(make-thread ...)` invoking `(kill-emacs 19)` joined by main thread while
     skipping both subthread and main-thread Lisp `unwind-protect`/`condition-case`, exit 19.
  6. Lisp subthread holding a mutex calling `(kill-emacs 29)` wakes main wait without deadlock.
  7. Lisp subthread calling `(kill-emacs 27)` stops busy main-thread eval loop promptly.
  8. Combined: Lisp subthread inside C dynamic module calling `(kill-emacs 23)`.
  9. Combined: Main thread inside C dynamic module joining subthread calling `(kill-emacs 31)`.
"""

from __future__ import annotations

import argparse
import hashlib
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
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int plugin_is_GPL_compatible;

static char *copy_lisp_string(emacs_env *env, emacs_value val) {
    ptrdiff_t size = 0;
    if (!env->copy_string_contents(env, val, NULL, &size) || size <= 0) {
        return NULL;
    }
    char *buf = (char *)malloc((size_t)size);
    if (!buf) {
        return NULL;
    }
    if (!env->copy_string_contents(env, val, buf, &size)) {
        free(buf);
        return NULL;
    }
    return buf;
}

static void append_marker(const char *path, const char *tag, int before_exit, int clear_attempted, int after_exit, int post_intern_null) {
    if (!path || !*path) {
        return;
    }
    FILE *fp = fopen(path, "a");
    if (!fp) {
        return;
    }
    fprintf(
        fp,
        "%s before=%d clear=%d after=%d post_intern_null=%d\n",
        tag ? tag : "unknown",
        before_exit,
        clear_attempted,
        after_exit,
        post_intern_null
    );
    fflush(fp);
    fclose(fp);
}

/* (c4r1-call-with-cleanup FN TAG MARKER-FILE CLEAR-FLAG) */
static emacs_value F_c4r1_call_with_cleanup(
    emacs_env *env,
    ptrdiff_t nargs,
    emacs_value *args,
    void *data
) {
    (void)nargs;
    (void)data;
    emacs_value fn = args[0];
    char *tag = copy_lisp_string(env, args[1]);
    char *marker_path = copy_lisp_string(env, args[2]);
    bool clear_flag = env->is_not_nil(env, args[3]);
    char *cleanup_heap_buf = (char *)malloc(64);
    if (cleanup_heap_buf) {
        strcpy(cleanup_heap_buf, "c4r1-cleanup-live");
    }

    emacs_value res = env->funcall(env, fn, 0, NULL);
    enum emacs_funcall_exit exit_before = env->non_local_exit_check(env);
    enum emacs_funcall_exit exit_after = exit_before;
    int clear_attempted = 0;
    int post_intern_null = 0;

    if (exit_before != emacs_funcall_exit_return) {
        emacs_value esym = NULL;
        emacs_value edata = NULL;
        (void)env->non_local_exit_get(env, &esym, &edata);
        if (clear_flag) {
            clear_attempted = 1;
            env->non_local_exit_clear(env);
            exit_after = env->non_local_exit_check(env);
        }
        /* Must safely no-op if pending host exit remained active. */
        emacs_value probed = env->intern(env, "c4r1-after-exit-probe");
        post_intern_null = (probed == NULL) ? 1 : 0;
    }

    append_marker(
        marker_path,
        tag,
        (int)exit_before,
        clear_attempted,
        (int)exit_after,
        post_intern_null
    );

    free(cleanup_heap_buf);
    free(marker_path);
    free(tag);
    return res;
}

static void register_subr(
    emacs_env *env,
    const char *name,
    ptrdiff_t min_arity,
    ptrdiff_t max_arity,
    emacs_function fn
) {
    emacs_value fval = env->make_function(env, min_arity, max_arity, fn, name, NULL);
    emacs_value defalias = env->intern(env, "defalias");
    emacs_value sym = env->intern(env, name);
    emacs_value call_args[2] = { sym, fval };
    env->funcall(env, defalias, 2, call_args);
}

int emacs_module_init(struct emacs_runtime *ert) {
    if ((size_t)ert->size < sizeof(*ert)) {
        return 1;
    }
    emacs_env *env = ert->get_environment(ert);
    if ((size_t)env->size < sizeof(*env)) {
        return 2;
    }

    const char *init_code_str = getenv("FEMACS_C4R1_INIT_KILL_CODE");
    const char *init_marker = getenv("FEMACS_C4R1_INIT_MARKER");
    if (init_code_str && *init_code_str) {
        long code = strtol(init_code_str, NULL, 10);
        emacs_value fn_kill = env->intern(env, "kill-emacs");
        emacs_value arg = env->make_integer(env, (intmax_t)code);
        (void)env->funcall(env, fn_kill, 1, &arg);
        enum emacs_funcall_exit before = env->non_local_exit_check(env);
        env->non_local_exit_clear(env);
        enum emacs_funcall_exit after = env->non_local_exit_check(env);
        emacs_value probed = env->intern(env, "c4r1-init-after-exit");
        append_marker(init_marker, "INIT", (int)before, 1, (int)after, (probed == NULL) ? 1 : 0);
        return 0;
    }

    register_subr(env, "c4r1-call-with-cleanup", 4, 4, F_c4r1_call_with_cleanup);
    return 0;
}
"""


def resolve_module_header() -> Path:
    generated = REPO / "build/generation/module-api/emacs-module.h"
    if generated.is_file():
        return generated
    return module_api.render_module_api(REPO)


def compile_test_module(work_dir: Path, header_path: Path) -> Path:
    src_c = work_dir / "c4r1_mod.c"
    dylib = work_dir / "c4r1_mod.dylib"
    src_c.write_text(C_MODULE_SOURCE, encoding="utf-8")
    inc_dir = work_dir / "include"
    inc_dir.mkdir(parents=True, exist_ok=True)
    (inc_dir / "emacs-module.h").write_bytes(header_path.read_bytes())

    cmd = [
        "clang",
        "-shared",
        "-fPIC",
        "-O2",
        "-Wall",
        "-Wextra",
        "-Werror",
        "-I",
        str(inc_dir),
        str(src_c),
        "-o",
        str(dylib),
    ]
    proc = subprocess.run(cmd, capture_output=True, text=True, timeout=60, check=False)
    if proc.returncode != 0:
        raise RuntimeError(
            f"failed to compile test module ({proc.returncode}): {' '.join(cmd)}\n"
            f"STDOUT:\n{proc.stdout}\nSTDERR:\n{proc.stderr}"
        )
    return dylib


def run_batch(
    femacs_lisp: Path,
    expr: str,
    *,
    expected_code: int,
    env_extra: dict[str, str] | None = None,
    timeout: float = 20.0,
) -> dict[str, object]:
    env = os.environ.copy()
    if env_extra:
        env.update(env_extra)
    started = time.monotonic()
    proc = subprocess.run(
        [str(femacs_lisp), "--batch", "--eval", expr],
        cwd=REPO,
        env=env,
        stdin=subprocess.DEVNULL,
        capture_output=True,
        text=True,
        timeout=timeout,
        check=False,
    )
    elapsed = round(time.monotonic() - started, 4)
    if proc.returncode != expected_code:
        raise AssertionError(
            f"unexpected exit code {proc.returncode} (expected {expected_code})\n"
            f"EXPR: {expr}\nSTDOUT:\n{proc.stdout}\nSTDERR:\n{proc.stderr}"
        )
    return {
        "returncode": proc.returncode,
        "elapsed_s": elapsed,
        "stdout": proc.stdout,
        "stderr": proc.stderr,
    }


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="Verify C4-R1 ABI-safe host-exit closure.")
    parser.add_argument("--package", type=Path, default=Path("dist/lisp"))
    parser.add_argument("--out", type=Path, default=Path("build/acceptance/c4-r1-abi-exit"))
    args = parser.parse_args(argv)

    package_dir = (REPO / args.package) if not args.package.is_absolute() else args.package
    out_dir = (REPO / args.out) if not args.out.is_absolute() else args.out
    out_dir.mkdir(parents=True, exist_ok=True)
    femacs_lisp = package_dir / "bin/femacs-lisp"
    if not femacs_lisp.is_file():
        raise FileNotFoundError(f"missing femacs-lisp binary at {femacs_lisp}")

    header = resolve_module_header()
    cases: dict[str, object] = {}

    with tempfile.TemporaryDirectory(prefix="femacs-c4r1-", dir=out_dir) as tmp:
        work = Path(tmp)
        dylib = compile_test_module(work, header)
        dylib_q = json.dumps(str(dylib))

        # 1. C dynamic module callback -> (kill-emacs 7) skips both inner and outer Lisp cleanups
        m1 = work / "marker1.txt"
        b1 = work / "bad1.txt"
        cases["c_module_callback_exit_and_unwind_skip"] = run_batch(
            femacs_lisp,
            f"(progn (module-load {dylib_q}) "
            f"(unwind-protect "
            f"(condition-case _ "
            f"(c4r1-call-with-cleanup "
            f"(lambda () (unwind-protect (kill-emacs 7) (write-region \"BAD\" nil {json.dumps(str(b1))}) (kill-emacs 2))) "
            f"\"cb7\" {json.dumps(str(m1))} nil) "
            f"(t 'swallowed)) "
            f"(write-region \"BAD\" nil {json.dumps(str(b1))}) (kill-emacs 3)))",
            expected_code=7,
        )
        assert not b1.exists(), "Lisp unwind-protect wrote forbidden marker on C module exit"
        assert m1.read_text(encoding="utf-8").splitlines() == [
            "cb7 before=1 clear=0 after=1 post_intern_null=1"
        ], m1.read_text(encoding="utf-8")

        # 2. C dynamic module calling non_local_exit_clear cannot swallow host exit
        m2 = work / "marker2.txt"
        b2 = work / "bad2.txt"
        cases["c_module_non_local_exit_clear_cannot_swallow"] = run_batch(
            femacs_lisp,
            f"(progn (module-load {dylib_q}) "
            f"(c4r1-call-with-cleanup (lambda () (kill-emacs 13)) \"clear13\" {json.dumps(str(m2))} t) "
            f"(write-region \"SWALLOWED\" nil {json.dumps(str(b2))}) "
            f"(kill-emacs 99))",
            expected_code=13,
        )
        assert not b2.exists(), "C module non_local_exit_clear swallowed host exit"
        assert m2.read_text(encoding="utf-8").splitlines() == [
            "clear13 before=1 clear=1 after=1 post_intern_null=1"
        ], m2.read_text(encoding="utf-8")

        # 3. Nested Lisp -> C outer -> Lisp -> C inner -> (kill-emacs 17)
        m3 = work / "marker3.txt"
        b3 = work / "bad3.txt"
        cases["c_module_nested_calls_cleanup_in_order"] = run_batch(
            femacs_lisp,
            f"(progn (module-load {dylib_q}) "
            f"(c4r1-call-with-cleanup "
            f"(lambda () "
            f"(unwind-protect "
            f"(c4r1-call-with-cleanup (lambda () (kill-emacs 17)) \"inner17\" {json.dumps(str(m3))} t) "
            f"(write-region \"BAD\" nil {json.dumps(str(b3))}))) "
            f"\"outer17\" {json.dumps(str(m3))} nil))",
            expected_code=17,
        )
        assert not b3.exists(), "Intermediate Lisp unwind-protect ran across nested C module exit"
        assert m3.read_text(encoding="utf-8").splitlines() == [
            "inner17 before=1 clear=1 after=1 post_intern_null=1",
            "outer17 before=1 clear=0 after=1 post_intern_null=1",
        ], m3.read_text(encoding="utf-8")

        # 4. C module emacs_module_init calling (kill-emacs 11)
        m4 = work / "marker4.txt"
        b4 = work / "bad4.txt"
        cases["c_module_init_exit"] = run_batch(
            femacs_lisp,
            f"(unwind-protect "
            f"(condition-case _ (module-load {dylib_q}) (t 'swallowed)) "
            f"(write-region \"BAD\" nil {json.dumps(str(b4))}) (kill-emacs 2))",
            expected_code=11,
            env_extra={
                "FEMACS_C4R1_INIT_KILL_CODE": "11",
                "FEMACS_C4R1_INIT_MARKER": str(m4),
            },
        )
        assert not b4.exists(), "Lisp unwind-protect ran on module_init exit"
        assert m4.read_text(encoding="utf-8").splitlines() == [
            "INIT before=1 clear=1 after=1 post_intern_null=1"
        ], m4.read_text(encoding="utf-8")

        # 5. Lisp subthread (kill-emacs 19) with thread-join
        b5 = work / "bad5.txt"
        cases["lisp_subthread_exit_and_unwind_skip"] = run_batch(
            femacs_lisp,
            f"(unwind-protect "
            f"(let ((th (make-thread (lambda () "
            f"(unwind-protect (kill-emacs 19) "
            f"(write-region \"SUBBAD\" nil {json.dumps(str(b5))}) (kill-emacs 2)))))) "
            f"(condition-case _ (thread-join th) (t 'swallowed))) "
            f"(write-region \"MAINBAD\" nil {json.dumps(str(b5))}) (kill-emacs 3))",
            expected_code=19,
        )
        assert not b5.exists(), "Lisp unwind-protect ran during subthread or main-thread host exit"

        # 6. Lisp subthread holding mutex wakes blocked main condition-wait without deadlock
        cases["lisp_subthread_holding_mutex_wakes_main"] = run_batch(
            femacs_lisp,
            "(let* ((m (make-mutex \"c4r1\")) "
            "(cv (make-condition-variable m))) "
            "(mutex-lock m) "
            "(let ((th (make-thread (lambda () (with-mutex m (condition-notify cv) (kill-emacs 29)))))) "
            "(condition-wait cv) "
            "(thread-join th)))",
            expected_code=29,
            timeout=10.0,
        )

        # 7. Lisp subthread exiting while main runs busy Lisp eval loop
        cases["lisp_subthread_with_busy_main_loop_quit_poll"] = run_batch(
            femacs_lisp,
            "(let ((th (make-thread (lambda () (thread-yield) (kill-emacs 27))))) "
            "(while t (thread-yield) (ignore th)))",
            expected_code=27,
            timeout=10.0,
        )

        # 8. Combined: Lisp subthread inside C dynamic module calling (kill-emacs 23)
        m8 = work / "marker8.txt"
        cases["combined_subthread_inside_c_module_exit"] = run_batch(
            femacs_lisp,
            f"(progn (module-load {dylib_q}) "
            f"(let ((th (make-thread (lambda () "
            f"(c4r1-call-with-cleanup (lambda () (kill-emacs 23)) \"submod23\" {json.dumps(str(m8))} t))))) "
            f"(condition-case _ (thread-join th) (t 'swallowed)) "
            f"(kill-emacs 99)))",
            expected_code=23,
        )
        assert m8.read_text(encoding="utf-8").splitlines() == [
            "submod23 before=1 clear=1 after=1 post_intern_null=1"
        ], m8.read_text(encoding="utf-8")

        # 9. Combined: Main thread inside C dynamic module joining subthread calling (kill-emacs 31)
        m9 = work / "marker9.txt"
        cases["combined_c_module_joining_exiting_subthread"] = run_batch(
            femacs_lisp,
            f"(progn (module-load {dylib_q}) "
            f"(c4r1-call-with-cleanup "
            f"(lambda () (let ((th (make-thread (lambda () (kill-emacs 31))))) (thread-join th))) "
            f"\"modjoin31\" {json.dumps(str(m9))} t))",
            expected_code=31,
        )
        assert m9.read_text(encoding="utf-8").splitlines() == [
            "modjoin31 before=1 clear=1 after=1 post_intern_null=1"
        ], m9.read_text(encoding="utf-8")

        # 10. Subthread calling (kill-emacs 47) after 0.15s wakes main thread blocked in (sleep-for 2.5)
        cases["lisp_subthread_wakes_main_sleep_for"] = run_batch(
            femacs_lisp,
            "(progn (make-thread (lambda () (sleep-for 0.15) (kill-emacs 47))) (sleep-for 2.5))",
            expected_code=47,
        )
        assert cases["lisp_subthread_wakes_main_sleep_for"]["elapsed_s"] < 1.2, cases[
            "lisp_subthread_wakes_main_sleep_for"
        ]

        # 11. Subthread calling (kill-emacs 53) after 0.15s wakes main thread in indefinite I/O wait
        cases["lisp_subthread_wakes_main_indefinite_io_wait"] = run_batch(
            femacs_lisp,
            "(let ((p (make-pipe-process :name \"c4-r2-idle-wait\" :buffer nil :noquery t))) "
            "(make-thread (lambda () (sleep-for 0.15) (kill-emacs 53))) "
            "(unwind-protect (accept-process-output p) (delete-process p)))",
            expected_code=53,
        )
        assert cases["lisp_subthread_wakes_main_indefinite_io_wait"]["elapsed_s"] < 1.2, cases[
            "lisp_subthread_wakes_main_indefinite_io_wait"
        ]

    summary = {
        "schema": "femacs-c4-r1-abi-exit-v1",
        "package": str(package_dir),
        "femacs_lisp_sha256": hashlib.sha256(femacs_lisp.read_bytes()).hexdigest(),
        "case_count": len(cases),
        "cases": cases,
    }
    report_path = out_dir / "c4-r1-abi-exit-report.json"
    report_path.write_text(json.dumps(summary, indent=2) + "\n", encoding="utf-8")
    print(f"C4-R1 ABI-exit suite PASSED ({len(cases)}/{len(cases)} cases) -> {report_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
