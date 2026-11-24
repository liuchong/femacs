#!/usr/bin/env python3
"""Phase II Stage C4/A4 dual-host modern local `.eli` extension acceptance.

Verifies without any JavaScript runtime (`PATH=/usr/bin:/bin`,
`ELISCRIPT_JS_RUNTIME=/nonexistent/no-js`) on isolated read-only
`git archive HEAD` snapshots of the downstream compiler repository:
1. Batch ERT and headless `kill-emacs` exit-code / `unwind-protect` /
   `condition-case` bypass contract across `femacs-lisp` and the reference host.
2. Selected existing pure-Lisp ERT suites (84 passing tests across 10 files)
   plus explicit parity failure of the 2 JS-AOT/worker `t16-*` tests.
3. Same-source modern `.eli` local interpretation (`eval-string` and `eval-file`)
   covering Lisp-2, unique `nil` falsiness, lexical + dynamic bindings, conditions
   and side-effect ordering, checked/untyped macros, persistent values, and
   `defmodule` lexical encapsulation / live `import-values` reload.
4. Explicit observable rejection parity for out-of-scope remote `engine-call`,
   JS host object/import calls, modern AOT compilation without JS, and static
   contract/module/async diagnostics.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
CACHE_TOOL = REPO / "tools" / "femacs-lisp-cache.py"

PURE_ERT_SUITES: list[tuple[str, int]] = [
    ("tests/eliscript-evaluation-tests.el", 4),
    ("tests/eliscript-analyzer-depth-tests.el", 10),
    ("tests/eliscript-lower-depth-tests.el", 6),
    ("tests/eliscript-emitter-depth-tests.el", 6),
    ("tests/eliscript-ir-await-cache-tests.el", 4),
    ("tests/eliscript-reader-preprocess-tests.el", 4),
    ("tests/eliscript-transient-depth-tests.el", 10),
    ("tests/eliscript-portable-depth-tests.el", 7),
    ("tests/eliscript-version-comparison-tests.el", 4),
]

SYNTAX_LOCAL_SELECTOR = "'(not \"eliscript-0-1-0-t16-\")"
SYNTAX_LOCAL_EXPECTED = 29
SYNTAX_JS_SELECTOR = "\"eliscript-0-1-0-t16-\""
SYNTAX_JS_EXPECTED = 2

ERT_SUMMARY_RE = re.compile(
    r"^Ran (\d+) tests?, (\d+) results? as expected, (\d+) unexpected",
    re.MULTILINE,
)


def run_cmd(
    argv: list[str],
    *,
    cwd: Path | None = None,
    env: dict[str, str] | None = None,
    timeout: int = 300,
) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        argv,
        cwd=str(cwd) if cwd else None,
        env=env,
        text=True,
        capture_output=True,
        timeout=timeout,
    )


def verify_codesign_if_darwin(binary: Path) -> None:
    if sys.platform != "darwin":
        return
    res = run_cmd(["codesign", "--verify", "--deep", "--strict", str(binary)], timeout=30)
    if res.returncode != 0:
        raise AssertionError(f"strict codesign check failed for {binary}: {res.stderr}")


def isolated_env(home_dir: Path) -> dict[str, str]:
    home_dir.mkdir(parents=True, exist_ok=True)
    tmp_dir = home_dir / "tmp"
    tmp_dir.mkdir(parents=True, exist_ok=True)
    return {
        "HOME": str(home_dir),
        "XDG_CONFIG_HOME": str(home_dir / ".config"),
        "XDG_CACHE_HOME": str(home_dir / ".cache"),
        "XDG_DATA_HOME": str(home_dir / ".local" / "share"),
        "TMPDIR": str(tmp_dir),
        "PATH": "/usr/bin:/bin",
        "TERM": "dumb",
        "LC_ALL": "C.UTF-8",
        "LANG": "C.UTF-8",
        "ELISCRIPT_JS_RUNTIME": "/nonexistent/no-js",
    }


def reference_env(home_dir: Path) -> dict[str, str]:
    env = isolated_env(home_dir)
    env["PATH"] = "/usr/bin:/bin"
    return env


def claim_fresh_output_dir(out_arg: Path) -> Path:
    if out_arg.is_symlink() or out_arg.exists():
        raise AssertionError(
            f"--out destination already exists or is a symlink; pass a new directory: {out_arg}"
        )
    out_dir = out_arg.resolve()
    if out_dir.is_symlink() or out_dir.exists():
        raise AssertionError(f"--out destination already exists; pass a new directory: {out_dir}")
    out_dir.parent.mkdir(parents=True, exist_ok=True)
    out_dir.mkdir(exist_ok=False)
    return out_dir


def extract_clean_git_snapshot(src_repo: Path, dst_dir: Path) -> str:
    if not (src_repo / ".git").exists():
        raise AssertionError(f"expected git repository at {src_repo}")
    rev = run_cmd(["git", "-C", str(src_repo), "rev-parse", "HEAD"], timeout=30)
    if rev.returncode != 0:
        raise AssertionError(f"git rev-parse failed for {src_repo}: {rev.stderr}")
    commit = rev.stdout.strip()

    dst = dst_dir.resolve()
    if dst.exists():
        raise AssertionError(f"snapshot destination already exists: {dst}")
    dst.mkdir(parents=True, exist_ok=False)
    tar_path = dst.parent / f"{dst.name}.tar"
    arch = run_cmd(["git", "-C", str(src_repo), "archive", "--output", str(tar_path), "HEAD"], timeout=120)
    if arch.returncode != 0:
        raise AssertionError(f"git archive HEAD failed: {arch.stderr}")
    unt = run_cmd(["tar", "-xf", str(tar_path), "-C", str(dst)], timeout=120)
    if unt.returncode != 0:
        raise AssertionError(f"tar extract failed: {unt.stderr}")
    tar_path.unlink(missing_ok=True)
    return commit


def host_load_flags(snap_dir: Path) -> list[str]:
    return [
        "-L",
        str(snap_dir / "compiler"),
        "-L",
        str(snap_dir / "editor"),
        "-L",
        str(snap_dir / "tools"),
        "-L",
        str(snap_dir / "tools" / "org"),
        "-L",
        str(snap_dir / "tools" / "worker"),
        "-L",
        str(snap_dir / "tests"),
    ]


def assert_equal_json_values(left: object, right: object, *, label: str) -> None:
    l_txt = json.dumps(left, sort_keys=True, ensure_ascii=False)
    r_txt = json.dumps(right, sort_keys=True, ensure_ascii=False)
    if l_txt != r_txt:
        raise AssertionError(f"{label} mismatch:\n  left ={l_txt}\n  right={r_txt}")


def check_ert_and_kill_contract(
    lisp_bin: Path,
    ref_bin: Path,
    clean_env: dict[str, str],
    ref_env: dict[str, str],
) -> dict[str, object]:
    results: dict[str, object] = {}

    for label, host_bin, host_env in (("femacs", lisp_bin, clean_env), ("reference", ref_bin, ref_env)):
        r_empty = run_cmd(
            [str(host_bin), "--batch", "-Q", "-l", "ert", "-f", "ert-run-tests-batch-and-exit"],
            env=host_env,
            timeout=60,
        )
        combined = r_empty.stdout + r_empty.stderr
        assert r_empty.returncode == 0, (
            f"{label} empty ERT expected exit 0, got {r_empty.returncode}:\n{combined}"
        )
        assert "Error running tests" not in combined and "backtrace()" not in combined, (
            f"{label} ERT printed unexpected unwind failure:\n{combined}"
        )

        r_fail = run_cmd(
            [
                str(host_bin),
                "--batch",
                "-Q",
                "-l",
                "ert",
                "--eval",
                "(ert-deftest c4-expected-fail () (should nil))",
                "-f",
                "ert-run-tests-batch-and-exit",
            ],
            env=host_env,
            timeout=60,
        )
        assert r_fail.returncode == 1, f"{label} failing ERT expected exit 1, got {r_fail.returncode}"

        r_uwp = run_cmd(
            [
                str(host_bin),
                "--batch",
                "-Q",
                "--eval",
                '(unwind-protect (kill-emacs 7) (princ "BAD-CLEANUP") (kill-emacs 2))',
            ],
            env=host_env,
            timeout=60,
        )
        assert r_uwp.returncode == 7, f"{label} unwind-protect kill-emacs expected 7, got {r_uwp.returncode}"
        assert "BAD-CLEANUP" not in (r_uwp.stdout + r_uwp.stderr), (
            f"{label} ran unwind-protect cleanup during kill-emacs"
        )

        r_cc = run_cmd(
            [
                str(host_bin),
                "--batch",
                "-Q",
                "--eval",
                '(condition-case _ (kill-emacs 9) (t (princ "BAD-CATCH") 99))',
            ],
            env=host_env,
            timeout=60,
        )
        assert r_cc.returncode == 9, f"{label} condition-case kill-emacs expected 9, got {r_cc.returncode}"
        assert "BAD-CATCH" not in (r_cc.stdout + r_cc.stderr), (
            f"{label} condition-case caught kill-emacs"
        )

        results[label] = {
            "empty_ert_exit": r_empty.returncode,
            "fail_ert_exit": r_fail.returncode,
            "unwind_protect_kill_exit": r_uwp.returncode,
            "condition_case_kill_exit": r_cc.returncode,
        }

    assert_equal_json_values(results["femacs"], results["reference"], label="ert-kill-contract")
    return results


def _run_ert_command(
    host_bin: Path,
    snap_dir: Path,
    test_file: str,
    host_env: dict[str, str],
    selector_expr: str | None = None,
) -> tuple[int, int, int, int, str]:
    argv = [str(host_bin), "--batch", "-Q", *host_load_flags(snap_dir), "-l", str(snap_dir / test_file)]
    if selector_expr is None:
        argv.extend(["-f", "ert-run-tests-batch-and-exit"])
    else:
        argv.extend(["--eval", f"(ert-run-tests-batch-and-exit {selector_expr})"])
    res = run_cmd(argv, cwd=snap_dir, env=host_env, timeout=180)
    combined = res.stdout + "\n" + res.stderr
    matches = ERT_SUMMARY_RE.findall(combined)
    if not matches:
        raise AssertionError(f"missing ERT summary for {test_file} on {host_bin} (rc={res.returncode}):\n{combined}")
    ran, expected, unexpected = (int(x) for x in matches[-1])
    return res.returncode, ran, expected, unexpected, combined


def check_ert_suites_parity(
    lisp_bin: Path,
    ref_bin: Path,
    snap_f: Path,
    snap_r: Path,
    clean_env: dict[str, str],
    ref_env: dict[str, str],
) -> dict[str, object]:
    suite_records: list[dict[str, object]] = []
    total_passed = 0

    for rel_path, expected_count in PURE_ERT_SUITES:
        rc_f, ran_f, exp_f, unexp_f, out_f = _run_ert_command(lisp_bin, snap_f, rel_path, clean_env)
        rc_r, ran_r, exp_r, unexp_r, out_r = _run_ert_command(ref_bin, snap_r, rel_path, ref_env)
        assert (rc_f, ran_f, exp_f, unexp_f) == (0, expected_count, expected_count, 0), (
            f"femacs failed {rel_path}: rc={rc_f} ran={ran_f} exp={exp_f} unexp={unexp_f}\n{out_f}"
        )
        assert (rc_r, ran_r, exp_r, unexp_r) == (0, expected_count, expected_count, 0), (
            f"reference failed {rel_path}: rc={rc_r} ran={ran_r} exp={exp_r} unexp={unexp_r}\n{out_r}"
        )
        total_passed += ran_f
        suite_records.append({"file": rel_path, "selector": "all", "count": ran_f, "unexpected": 0})

    # Local modern `.eli` syntax tests (29 passing tests without JS)
    rc_f, ran_f, exp_f, unexp_f, out_f = _run_ert_command(
        lisp_bin, snap_f, "tests/eliscript-syntax-tests.el", clean_env, SYNTAX_LOCAL_SELECTOR
    )
    rc_r, ran_r, exp_r, unexp_r, out_r = _run_ert_command(
        ref_bin, snap_r, "tests/eliscript-syntax-tests.el", ref_env, SYNTAX_LOCAL_SELECTOR
    )
    assert (rc_f, ran_f, exp_f, unexp_f) == (0, SYNTAX_LOCAL_EXPECTED, SYNTAX_LOCAL_EXPECTED, 0), (
        f"femacs failed syntax local subset: rc={rc_f} ran={ran_f} exp={exp_f} unexp={unexp_f}\n{out_f}"
    )
    assert (rc_r, ran_r, exp_r, unexp_r) == (0, SYNTAX_LOCAL_EXPECTED, SYNTAX_LOCAL_EXPECTED, 0), (
        f"reference failed syntax local subset: rc={rc_r} ran={ran_r} exp={exp_r} unexp={unexp_r}\n{out_r}"
    )
    total_passed += ran_f
    suite_records.append({
        "file": "tests/eliscript-syntax-tests.el",
        "selector": SYNTAX_LOCAL_SELECTOR,
        "count": ran_f,
        "unexpected": 0,
    })

    # Verify the 2 out-of-scope JS AOT/worker tests fail with exit 1 on both zero-JS hosts
    rc_jf, ran_jf, exp_jf, unexp_jf, out_jf = _run_ert_command(
        lisp_bin, snap_f, "tests/eliscript-syntax-tests.el", clean_env, SYNTAX_JS_SELECTOR
    )
    rc_jr, ran_jr, exp_jr, unexp_jr, out_jr = _run_ert_command(
        ref_bin, snap_r, "tests/eliscript-syntax-tests.el", ref_env, SYNTAX_JS_SELECTOR
    )
    assert (rc_jf, ran_jf, exp_jf, unexp_jf) == (1, SYNTAX_JS_EXPECTED, 0, SYNTAX_JS_EXPECTED), (
        f"femacs expected exit 1 / 2 unexpected on out-of-scope t16: rc={rc_jf} ran={ran_jf}\n{out_jf}"
    )
    assert (rc_jr, ran_jr, exp_jr, unexp_jr) == (1, SYNTAX_JS_EXPECTED, 0, SYNTAX_JS_EXPECTED), (
        f"reference expected exit 1 / 2 unexpected on out-of-scope t16: rc={rc_jr} ran={ran_jr}\n{out_jr}"
    )

    return {
        "passing_files": len(suite_records),
        "total_passing_tests": total_passed,
        "excluded_out_of_scope_t16": SYNTAX_JS_EXPECTED,
        "suites": suite_records,
    }


MODERN_ELI_FILE_SOURCE = r""";;; c4-modern-sample.eli --- Same-source modern local Eliscript sample -*- lexical-binding: t; -*-

(require 'eliscript)
(require 'eliscript-js)

(defmacro-checked c4-with-header (val)
  (declare (eliscript :inputs ("macro-input.txt")))
  (let ((prefix (macro-read-file "macro-input.txt"))
        (tmp (macro-gensym "c4")))
    `(let ((,tmp ,val))
       (format "%s:%S" ,prefix ,tmp))))

(defvar *c4-dyn-scale* 3)

(defmodule c4-counter-mod
  (defconst-lexical mod-magic 100)
  (defvar-lexical mod-count 10)
  (defun-lexical scale-it (n)
    (* n *c4-dyn-scale*))
  (defun bump (delta)
    (setq mod-count (+ mod-count delta))
    (+ mod-magic (scale-it mod-count)))
  (defun read-count ()
    mod-count)
  (export-functions bump read-count)
  (export-values mod-count))

(defun c4-run-modern-suite ()
  (eliscript-runtime-eval-string
   "(require 'eliscript)
    (import-functions c4-counter-mod :as c4-)
    (import-values c4-counter-mod :as c4-v-)
    (let* ((lisp2-sym 21)
           (lisp2-fn (progn
                       (defun c4-dual (x) (* x 2))
                       (defvar c4-dual 21)
                       (list (c4-dual c4-dual) (symbol-value 'c4-dual))))
           (lexical-err
            (condition-case err
                (let ((f (lambda (x) (+ x 1))))
                  (f 5))
              (error (symbol-name (car err)))))
           (truthiness
            (list
             (if nil 1 0)
             (if '() 1 0)
             (if (list) 1 0)
             (if 0 1 0)
             (if \"\" 1 0)
             (if [] 1 0)
             (if (persistent-vector) 1 0)
             (if option-none 1 0)
             (if (result-error \"e\") 1 0)
             (if js-false 1 0)
             (if js-null 1 0)
             (if js-undefined 1 0)))
           (order-log nil)
           (unwind-res
            (catch 'c4-tag
              (unwind-protect
                  (condition-case err
                      (progn
                        (push 'try-enter order-log)
                        (signal 'error (list \"inner\")))
                    (error
                     (push (cadr err) order-log)
                     (throw 'c4-tag 'thrown-ok)))
                (push 'cleanup-ran order-log))))
           (dyn-closure
            (progn
              (defun read-dyn () *c4-dyn-scale*)
              (let ((c (let ((k 5)) (lambda (v) (+ v k (read-dyn))))))
                (list (funcall c 2)
                      (let ((*c4-dyn-scale* 10)) (funcall c 2))
                      *c4-dyn-scale*))))
           (checked-macro (c4-with-header (+ 40 2)))
           (pvec (persistent-vector 10 20 30))
           (pmap (persistent-map :a 1 :b 2))
           (pmap2 (value-assoc pmap :b 9))
           (pset (persistent-set :x :y))
           (pvec2 (value-assoc pvec 1 200))
           (trans-vec (with-transient (tv pvec)
                        (conj! tv 40)))
           (trans-map (with-transient (tm pmap)
                        (assoc! tm :b 99)))
           (cell (state-cell 5))
           (cell-next (cell-update! cell (lambda (n) (+ n 7))))
           (sum-res
            (result-bind (result-ok 12) (lambda (n) (result-ok (+ n 30)))))
           (case-vals
            (progn
              (defun-case describe-v
                (() \"zero\")
                ((n) (format \"one:%s\" n))
                ((a b) (format \"two:%s,%s\" a b)))
              (list (describe-v) (describe-v 7) (describe-v 3 4))))
           (bound-vals
            (let-bind (([a [b &rest rest]] [1 [2 3 4]]))
              (list a b rest)))
           (mod-before c4-v-mod-count)
           (mod-step1 (let ((*c4-dyn-scale* 4)) (c4-bump 5)))
           (mod-after c4-v-mod-count)
           (mod-read (c4-read-count)))
      (list
       (cons 'lisp2 (vconcat lisp2-fn))
       (cons 'lexicalFnCallErr lexical-err)
       (cons 'truthiness (vconcat truthiness))
       (cons 'unwindRes (symbol-name unwind-res))
       (cons 'orderLog (vconcat (mapcar (lambda (x) (format \"%s\" x)) (nreverse order-log))))
       (cons 'dynClosure (vconcat dyn-closure))
       (cons 'checkedMacro checked-macro)
       (cons 'persistentOriginal (vconcat (list (value-get pmap :b) (value-nth pvec2 1) (value-nth trans-vec 3) (value-get pmap2 :b) (value-get trans-map :b) (if (value-get pset :y) t :false))))
       (cons 'stateCell (vconcat (list cell-next (cell-value cell) (result-value sum-res))))
       (cons 'casesAndDestruct (vconcat (append case-vals (list (vconcat bound-vals)))))
       (cons 'moduleState (vconcat (list mod-before mod-step1 mod-after mod-read)))))"))
"""


def run_modern_eval_on_host(
    host_bin: Path,
    snap_dir: Path,
    work_dir: Path,
    host_env: dict[str, str],
) -> dict[str, object]:
    work_dir.mkdir(parents=True, exist_ok=True)
    (work_dir / "macro-input.txt").write_text("HDR", encoding="utf-8")
    eli_path = work_dir / "c4-modern-sample.eli"
    eli_path.write_text(MODERN_ELI_FILE_SOURCE, encoding="utf-8")

    driver = f"""(progn
  (require 'json)
  (require 'eliscript-runtime)
  (eliscript-runtime-reset-state!)
  (let ((default-directory {json.dumps(str(work_dir) + "/")}))
    (eliscript-runtime-eval-file {json.dumps(str(eli_path))})
    (let* ((res (c4-run-modern-suite))
           (tail-val (eliscript-runtime-eval-string
                      "(require 'eliscript)
                       (defun-tail fact-acc (n acc)
                         (if (= n 0) acc (recur (1- n) (* acc n))))
                       (fact-acc 6 1)"))
           (payload (list (cons 'invocation invocation-name)
                          (cons 'noninteractive (if noninteractive t :false))
                          (cons 'factorial tail-val)
                          (cons 'suite res))))
      (princ (json-serialize payload)))))"""

    res = run_cmd(
        [str(host_bin), "--batch", "-Q", *host_load_flags(snap_dir), "--eval", driver],
        cwd=work_dir,
        env=host_env,
        timeout=120,
    )
    if res.returncode != 0:
        raise AssertionError(
            f"modern .eli eval failed on {host_bin} (rc={res.returncode}):\n{res.stdout}\n{res.stderr}"
        )
    return json.loads(res.stdout)


def check_modern_script_parity(
    lisp_bin: Path,
    ref_bin: Path,
    snap_f: Path,
    snap_r: Path,
    out_dir: Path,
    clean_env: dict[str, str],
    ref_env: dict[str, str],
) -> dict[str, object]:
    res_f = run_modern_eval_on_host(lisp_bin, snap_f, out_dir / "modern-femacs", clean_env)
    res_r = run_modern_eval_on_host(ref_bin, snap_r, out_dir / "modern-ref", ref_env)

    assert res_f["invocation"] == "femacs-lisp", f"unexpected invocation: {res_f}"
    assert res_r["invocation"].lower() == "emacs", f"unexpected ref invocation: {res_r}"
    assert res_f["noninteractive"] is True and res_r["noninteractive"] is True

    assert_equal_json_values(
        {"factorial": res_f["factorial"], "suite": res_f["suite"]},
        {"factorial": res_r["factorial"], "suite": res_r["suite"]},
        label="modern-local-script-parity",
    )

    suite = res_f["suite"]
    assert res_f["factorial"] == 720
    assert suite["lisp2"] == [42, 21]
    assert suite["lexicalFnCallErr"] == "void-function"
    assert suite["truthiness"] == [0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1]
    assert suite["orderLog"] == ["try-enter", "inner", "cleanup-ran"]
    assert suite["persistentOriginal"] == [2, 200, 40, 9, 99, True]
    assert suite["casesAndDestruct"] == ["zero", "one:7", "two:3,4", [1, 2, [3, 4]]]
    assert suite["moduleState"] == [10, 160, 15, 15]
    return {"femacs": res_f, "reference": res_r}


def check_rejections_parity(
    lisp_bin: Path,
    ref_bin: Path,
    snap_f: Path,
    snap_r: Path,
    out_dir: Path,
    clean_env: dict[str, str],
    ref_env: dict[str, str],
) -> list[dict[str, object]]:
    cases: list[tuple[str, str, str, str]] = [
        (
            "remote-engine-js",
            "(progn (require 'eliscript) (eliscript-async-wait (engine-call 'js 'add 1 2)))",
            "eliscript-engine-handle-error",
            "Engine unavailable or not connected",
        ),
        (
            "remote-engine-worker",
            "(progn (require 'eliscript) (eliscript-async-wait (engine-call 'worker 'add 1 2)))",
            "eliscript-engine-handle-error",
            "Engine unavailable or not connected",
        ),
        (
            "revoked-local-handle",
            "(progn (require 'eliscript) (let ((h (engine-bind #'1+))) (engine-revoke! h) (eliscript-async-wait (engine-call 'local h 1))))",
            "eliscript-engine-handle-error",
            "Invalid, mismatched-owner, or revoked",
        ),
        (
            "js-import-call",
            '(progn (require \'eliscript-js) (js-import "fs" :as c4-fs) (js-call c4-fs :readFileSync "x"))',
            "error",
            "is not callable",
        ),
        (
            "js-object-call",
            "(progn (require 'eliscript-js) (js-call (js-object :x 1) :toString))",
            "error",
            "is not callable",
        ),
        (
            "aot-compile-without-js",
            '(let ((el (expand-file-name "c4-aot.el" default-directory))) (with-temp-file el (insert "(defun f (x) x)\\n")) (eliscript-runtime-compile-elisp-module (list el) (expand-file-name "c4-aot.mjs" default-directory)))',
            "error",
            "Failed to build",
        ),
        (
            "static-const-reassign",
            "(eliscript-runtime-eval-string \"(require 'eliscript)\\n(defmodule c4-bad-c (defconst-lexical k 1) (defun f () (setq k 2)) (export-functions f))\")",
            "eliscript-compile-error",
            "ELI-M0002",
        ),
        (
            "static-unknown-module",
            "(eliscript-runtime-eval-string \"(require 'eliscript)\\n(import-functions c4-no-such-mod)\")",
            "eliscript-compile-error",
            "ELI-M0003",
        ),
        (
            "static-unexported-fn",
            "(eliscript-runtime-eval-string \"(require 'eliscript)\\n(defmodule c4-exp1 (defun pub () 1) (export-functions pub))\\n(import-functions c4-exp1 :only (missing-fn))\")",
            "eliscript-compile-error",
            "ELI-M0004",
        ),
        (
            "static-unexported-val",
            "(eliscript-runtime-eval-string \"(require 'eliscript)\\n(defmodule c4-exp2 (defvar-lexical x 1) (export-values x))\\n(import-values c4-exp2 :only (missing-val))\")",
            "eliscript-compile-error",
            "ELI-M0005",
        ),
        (
            "static-undeclared-macro-read-file",
            "(eliscript-runtime-eval-string \"(require 'eliscript)\\n(defmacro-checked c4-bad-m () (macro-read-file \\\"secret.txt\\\"))\\n(c4-bad-m)\")",
            "eliscript-compile-error",
            "ELI-M0011",
        ),
        (
            "static-sync-await",
            "(eliscript-runtime-eval-string \"(require 'eliscript)\\n(defun c4-bad-await (f) (await f))\")",
            "eliscript-compile-error",
            "ELI-A0001",
        ),
        (
            "static-impure-contract",
            "(eliscript-runtime-eval-string \"(require 'eliscript)\\n(defvar c4-bad-glob 0)\\n(defun c4-pure-bad () (declare (eliscript :pure t)) (setq c4-bad-glob 1))\")",
            "eliscript-contract-error",
            "ELI-C0001",
        ),
        (
            "static-extension-conflict",
            "(eliscript-runtime-eval-string \"(defun value-get () 1)\\n(require 'eliscript-data)\")",
            "eliscript-compile-error",
            "ELI-M0001",
        ),
    ]

    driver_body = "\n".join(
        f"""(push (list (cons 'id {json.dumps(cid)})
                    (cons 'result
                          (condition-case err
                              (progn
                                (eliscript-runtime-reset-state!)
                                {form}
                                (list (cons 'status "unexpected-ok")))
                            ((error quit)
                             (list (cons 'status "err")
                                   (cons 'symbol (symbol-name (car err)))
                                   (cons 'message (error-message-string err)))))))
              out)"""
        for cid, form, _sym, _sub in cases
    )
    driver = f"""(progn
  (require 'json)
  (require 'eliscript-runtime)
  (let ((out nil))
    {driver_body}
    (princ (json-serialize (vconcat (nreverse out))))))"""

    def run_host(host_bin: Path, snap_dir: Path, work_dir: Path, env: dict[str, str]) -> dict[str, dict[str, str]]:
        work_dir.mkdir(parents=True, exist_ok=True)
        res = run_cmd(
            [str(host_bin), "--batch", "-Q", *host_load_flags(snap_dir), "--eval", driver],
            cwd=work_dir,
            env=env,
            timeout=120,
        )
        if res.returncode != 0:
            raise AssertionError(
                f"rejection harness failed on {host_bin} (rc={res.returncode}):\n{res.stdout}\n{res.stderr}"
            )
        entries = json.loads(res.stdout)
        return {item["id"]: item["result"] for item in entries}

    map_f = run_host(lisp_bin, snap_f, out_dir / "reject-femacs", clean_env)
    map_r = run_host(ref_bin, snap_r, out_dir / "reject-ref", ref_env)

    records: list[dict[str, object]] = []
    for cid, _form, exp_sym, exp_sub in cases:
        rf = map_f[cid]
        rr = map_r[cid]
        assert rf["status"] == "err", f"femacs did not reject {cid}: {rf}"
        assert rr["status"] == "err", f"reference did not reject {cid}: {rr}"
        assert rf["symbol"] == exp_sym, f"femacs {cid} symbol={rf['symbol']} expected={exp_sym}"
        assert rr["symbol"] == exp_sym, f"reference {cid} symbol={rr['symbol']} expected={exp_sym}"
        assert exp_sub in rf["message"], f"femacs {cid} msg={rf['message']!r} missing {exp_sub!r}"
        assert exp_sub in rr["message"], f"reference {cid} msg={rr['message']!r} missing {exp_sub!r}"
        records.append({"id": cid, "symbol": rf["symbol"], "femacs_msg": rf["message"]})

    return records


def run_selftest() -> int:
    with tempfile.TemporaryDirectory(prefix="femacs-c4-selftest-") as tmp:
        root = Path(tmp)
        existing = root / "existing-out"
        existing.mkdir()
        sentinel = existing / "keep.txt"
        sentinel.write_text("keep\n", encoding="utf-8")

        try:
            claim_fresh_output_dir(existing)
            raise AssertionError("expected claim_fresh_output_dir to reject existing dir")
        except AssertionError:
            pass
        assert sentinel.read_text(encoding="utf-8") == "keep\n"

        try:
            main([
                "--package",
                str(root / "missing-pkg"),
                "--eliscript-repo",
                str(existing),
                "--reference-host",
                str(sentinel),
                "--out",
                str(existing),
            ])
            raise AssertionError("expected main to reject invalid package without modifying existing --out")
        except AssertionError:
            pass
        assert sentinel.read_text(encoding="utf-8") == "keep\n"

        assert_equal_json_values({"a": [1, True], "b": "x"}, {"b": "x", "a": [1, True]}, label="selftest")
        fresh = claim_fresh_output_dir(root / "nested" / "fresh")
        assert fresh.is_dir()

    print("test-c4-a4-modern-local selftest PASS")
    return 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="C4/A4 dual-host modern local .eli acceptance")
    parser.add_argument("--selftest", action="store_true")
    parser.add_argument("--package", type=Path, default=REPO / "dist" / "lisp")
    parser.add_argument("--eliscript-repo", type=Path, required=False)
    parser.add_argument("--reference-host", type=Path, default=Path("/opt/homebrew/bin/emacs"))
    parser.add_argument("--out", type=Path, default=REPO / "build" / "acceptance" / "c4-a4-run")
    args = parser.parse_args(argv)

    if args.selftest:
        return run_selftest()
    if not args.eliscript_repo:
        parser.error("--eliscript-repo is required unless --selftest is specified")

    pkg_dir = args.package.resolve()
    ref_bin = args.reference_host
    if not ref_bin.is_file():
        raise AssertionError(f"reference host not found: {ref_bin}")
    if not args.eliscript_repo.is_dir():
        raise AssertionError(f"eliscript repository directory not found: {args.eliscript_repo}")

    # Validate package and strict codesign BEFORE creating output directory
    ver = run_cmd([sys.executable, str(CACHE_TOOL), "verify", "--package", str(pkg_dir), "--json"], timeout=60)
    if ver.returncode != 0:
        raise AssertionError(f"package verify failed: {ver.stdout} {ver.stderr}")
    manifest = json.loads((pkg_dir / "manifest.json").read_text(encoding="utf-8"))
    lisp_bin = pkg_dir / "bin" / "femacs-lisp"
    http_bin = pkg_dir / "bin" / "http-host"
    verify_codesign_if_darwin(lisp_bin)
    verify_codesign_if_darwin(http_bin)

    out_dir = claim_fresh_output_dir(args.out)
    clean_env = isolated_env(out_dir / "home-femacs")
    ref_env = reference_env(out_dir / "home-ref")

    # 1. Clean read-only snapshots of downstream repo HEAD
    snap_f = out_dir / "host-femacs" / "src"
    snap_r = out_dir / "host-ref" / "src"
    commit_f = extract_clean_git_snapshot(args.eliscript_repo, snap_f)
    commit_r = extract_clean_git_snapshot(args.eliscript_repo, snap_r)
    assert commit_f == commit_r

    # 2. ERT & kill-emacs unwind contract parity
    contract_info = check_ert_and_kill_contract(lisp_bin, ref_bin, clean_env, ref_env)

    # 3. Existing pure-Lisp ERT suites parity (84 passing + 2 expected t16 failures)
    ert_info = check_ert_suites_parity(lisp_bin, ref_bin, snap_f, snap_r, clean_env, ref_env)

    # 4. Same-source modern `.eli` execution parity (`eval-file` + `eval-string`)
    modern_info = check_modern_script_parity(lisp_bin, ref_bin, snap_f, snap_r, out_dir, clean_env, ref_env)

    # 5. Observable rejection & static diagnostic parity
    rejection_info = check_rejections_parity(lisp_bin, ref_bin, snap_f, snap_r, out_dir, clean_env, ref_env)

    report = {
        "status": "PASS",
        "downstream_commit": commit_f,
        "package_manifest": {
            "source_commit": manifest["source_commit"],
            "target": manifest["target"],
            "build_config": manifest["build_config"],
            "package_sha256": manifest["package_sha256"],
        },
        "ert_kill_contract": contract_info,
        "ert_suites": ert_info,
        "modern_script": {
            "femacs_invocation": modern_info["femacs"]["invocation"],
            "reference_invocation": modern_info["reference"]["invocation"],
            "factorial": modern_info["femacs"]["factorial"],
        },
        "rejections": rejection_info,
    }
    (out_dir / "report.json").write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print(
        f"test-c4-a4-modern-local PASS: downstream={commit_f[:12]} "
        f"ert={ert_info['total_passing_tests']}passed/{ert_info['passing_files']}files "
        f"rejections={len(rejection_info)} pkg_sha={manifest['package_sha256'][:16]}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
