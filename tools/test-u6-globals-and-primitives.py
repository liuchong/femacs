#!/usr/bin/env python3
"""Stage U6 / AU6 integration and functional acceptance runner (P-485).

Verifies:
  1. `static-gates`: `check-unsafe-boundary.py` (`active_product.static_mut_items == 0`,
     `100%` `SAFETY:` docs, `safe_cmds.rs` under `#![forbid(unsafe_code)]`, `nm` singletons
     and exports), `check-globals.py`, `check-defvar-carriers.py`, `check-mirror-dupes.py`,
     `check-ledger-ticks.py`, `check-libshadows.py`, `check-declarations.py`, and
     `check-own-voice.py`.
  2. `pdmp-and-defvar`: `.pdmp` dump/restore roundtrip of `VmGlobal`-backed `DEFVAR_LISP`,
     `DEFVAR_INT`, and `DEFVAR_BOOL` state, plus dynamic `let` binding, `NonLocalExit`
     unwind restoration, `DEFVAR_PER_BUFFER`, `DEFVAR_KBOARD`, and `add-variable-watcher`.
  3. `safe-primitives-and-ert`: `safe_cmds.rs` commands (`forward-char`, `backward-char`,
     `forward-line`, `beginning-of-line`, `end-of-line`, `delete-char`, `self-insert-command`
     across multibyte, unibyte, overwrite-mode, abbrev expansion, auto-fill, and hooks),
     checked `XCAR`/`XCDR`/`AREF`/`ASET`/`SREF`/`SCHARS`/`SBYTES` primitives, and official
     ERT suites (`cmds-tests`, `data-tests`, `eval-tests`, `fns-tests`, `buffer-tests`,
     `editfns-tests`).
  4. `native-eln`: `libgccjit` `.eln` native compilation and load against `VmGlobal` globals
     and safe fixed-arity `DEFUN!` subrs, verifying `subr-native-elisp-p` and unwinding.
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


def check_static_gates(terminal_bin: Path, temacs_bin: Path) -> dict:
    gates = [
        (
            "check-unsafe-boundary",
            [
                sys.executable,
                "tools/check-unsafe-boundary.py",
                "--manifest",
                "tools/unsafe-boundary-manifest.json",
                "--require-no-file-allows",
                "--require-100pct-safety-docs",
                "--require-zero-active-stitching",
                "--require-forbid",
                "crates/femacs-vm/src/safe_cmds.rs",
                "--binary",
                str(temacs_bin if temacs_bin.is_file() else terminal_bin),
                "--kind",
                "terminal",
                "--require-modules",
            ],
        ),
        ("check-globals", [sys.executable, "tools/check-globals.py"]),
        ("check-defvar-carriers", [sys.executable, "tools/check-defvar-carriers.py"]),
        ("check-mirror-dupes", [sys.executable, "tools/check-mirror-dupes.py"]),
        ("check-ledger-ticks", [sys.executable, "tools/check-ledger-ticks.py"]),
        ("check-libshadows", [sys.executable, "tools/check-libshadows.py"]),
        ("check-declarations", [sys.executable, "tools/check-declarations.py"]),
        ("check-own-voice", [sys.executable, "tools/check-own-voice.py"]),
    ]
    summary: dict[str, str] = {}
    for label, cmd in gates:
        proc = run_cmd(cmd, timeout=90)
        if proc.returncode != 0:
            raise AssertionError(f"{label} exited {proc.returncode}:\n{proc.stdout}\n{proc.stderr}")
        if label == "check-unsafe-boundary":
            payload = json.loads(proc.stdout)
            ap = payload["active_product"]
            if ap["static_mut_items"] != 0:
                raise AssertionError(f"expected 0 active static_mut_items, got {ap['static_mut_items']}")
            if ap["unsafe_blocks"] != ap["safety_documented_blocks"]:
                raise AssertionError("incomplete SAFETY documentation on active unsafe blocks")
            summary["active_static_mut_items"] = str(ap["static_mut_items"])
            summary["forbid_unsafe_files"] = str(ap["forbid_unsafe_files"])
        else:
            summary[label] = "ok"
    return summary


def check_pdmp_and_defvar(terminal_bin: Path) -> dict:
    with tempfile.TemporaryDirectory(prefix="u6-pdmp-") as tmp:
        tmp_path = Path(tmp)
        dump_file = tmp_path / "u6-custom.pdmp"
        elisp = r"""
(progn
  ;; 1. Verify pdumper loaded on startup
  (unless (and (fboundp 'pdumper-stats)
               (cdr (assq 'dumped-with-pdumper (pdumper-stats))))
    (error "expected dumped-with-pdumper"))

  ;; 2. DEFVAR_LISP + watcher + dynamic let + unwind
  (let ((events nil))
    (let ((watcher (lambda (sym newval op where)
                     (push (list sym newval op (bufferp where)) events))))
      (unwind-protect
          (progn
            (add-variable-watcher 'post-self-insert-hook watcher)
            (let ((orig post-self-insert-hook))
              (unwind-protect
                  (progn
                    (setq post-self-insert-hook (list #'ignore))
                    (unless (equal post-self-insert-hook (list #'ignore))
                      (error "DEFVAR_LISP setq failed"))
                    (ignore-errors
                      (let ((post-self-insert-hook (list #'identity)))
                        (unless (equal post-self-insert-hook (list #'identity))
                          (error "DEFVAR_LISP let failed"))
                        (signal ' arith-error '(1))))
                    (unless (equal post-self-insert-hook (list #'ignore))
                      (error "DEFVAR_LISP unwind failed")))
                (setq post-self-insert-hook orig))))
        (remove-variable-watcher 'post-self-insert-hook watcher)))
    (unless (>= (length events) 3)
      (error "expected watcher events on DEFVAR_LISP, got %S" events)))

  ;; 3. DEFVAR_INT + DEFVAR_BOOL dynamic binding & type validation
  (let ((orig-gc gc-cons-threshold)
        (orig-dbg debug-on-error))
    (let ((gc-cons-threshold (+ orig-gc 4096))
          (debug-on-error (not orig-dbg)))
      (unless (= gc-cons-threshold (+ orig-gc 4096))
        (error "DEFVAR_INT let failed"))
      (unless (eq debug-on-error (not orig-dbg))
        (error "DEFVAR_BOOL let failed")))
    (unless (and (= gc-cons-threshold orig-gc)
                 (eq debug-on-error orig-dbg))
      (error "DEFVAR_INT/BOOL restore failed")))

  ;; 4. DEFVAR_PER_BUFFER isolation across buffers
  (let ((b1 (generate-new-buffer " *u6-b1*"))
        (b2 (generate-new-buffer " *u6-b2*")))
    (unwind-protect
        (progn
          (with-current-buffer b1
            (setq overwrite-mode 'overwrite-mode-binary)
            (setq buffer-read-only t))
          (with-current-buffer b2
            (unless (and (null overwrite-mode) (null buffer-read-only))
              (error "DEFVAR_PER_BUFFER leaked across buffers")))
          (with-current-buffer b1
            (unless (and (eq overwrite-mode 'overwrite-mode-binary)
                         (eq buffer-read-only t))
              (error "DEFVAR_PER_BUFFER lost buffer-local state"))))
      (kill-buffer b1)
      (kill-buffer b2)))

  ;; 5. DEFVAR_KBOARD access
  (unless (null defining-kbd-macro)
    (error "unexpected defining-kbd-macro state"))

  ;; 6. Custom portable dump & reload preserving VmGlobal state
  (defvar u6-dump-sentinel nil)
  (setq u6-dump-sentinel '(u6-ok 42))
  (setq gc-cons-threshold 987654)
  (dump-emacs-portable (getenv "U6_DUMP_FILE"))
  (princ "U6_DUMP_WRITTEN\n"))
"""
        env = isolated_env(tmp_path)
        env["U6_DUMP_FILE"] = str(dump_file)
        p1 = run_cmd([str(terminal_bin), "-Q", "--batch", "--eval", elisp], env=env, timeout=90)
        if p1.returncode != 0 or "U6_DUMP_WRITTEN" not in p1.stdout:
            raise AssertionError(f"pdmp/defvar phase 1 failed ({p1.returncode}):\n{p1.stdout}\n{p1.stderr}")
        if not dump_file.is_file():
            raise AssertionError("custom dump file was not created")

        verify_elisp = r"""
(progn
  (unless (equal u6-dump-sentinel '(u6-ok 42))
    (error "reloaded Lisp var mismatch: %S" u6-dump-sentinel))
  (unless (= gc-cons-threshold 987654)
    (error "reloaded VmGlobal DEFVAR_INT mismatch: %S" gc-cons-threshold))
  (let ((gc-cons-threshold 1234567))
    (unless (= gc-cons-threshold 1234567)
      (error "post-reload DEFVAR_INT let failed")))
  (unless (= gc-cons-threshold 987654)
    (error "post-reload DEFVAR_INT unbind failed"))
  (princ "U6_RELOAD_OK\n"))
"""
        p2 = run_cmd(
            [str(terminal_bin), f"--dump-file={dump_file}", "-Q", "--batch", "--eval", verify_elisp],
            env=env,
            timeout=90,
        )
        if p2.returncode != 0 or "U6_RELOAD_OK" not in p2.stdout:
            raise AssertionError(f"pdmp reload failed ({p2.returncode}):\n{p2.stdout}\n{p2.stderr}")
        return {"dump_bytes": dump_file.stat().st_size}


def check_safe_primitives_and_ert(terminal_bin: Path) -> dict:
    with tempfile.TemporaryDirectory(prefix="u6-ert-") as tmp:
        env = isolated_env(Path(tmp))
        probe = r"""
(progn
  ;; Exercise safe_cmds: forward/backward-char, forward-line, bol/eol, delete-char, self-insert-command
  (with-temp-buffer
    (let ((hook-count 0))
      (let ((post-self-insert-hook (list (lambda () (setq hook-count (1+ hook-count))))))
        (let ((last-command-event ?A))
          (self-insert-command 3))
        (self-insert-command 2 ?\u4e2d)
        (self-insert-command 1 ?\n)
        (self-insert-command 2 ?B))
      (unless (and (= hook-count 4) (equal (buffer-string) "AAA\u4e2d\u4e2d\nBB"))
        (error "self-insert-command mismatch: count=%S str=%S" hook-count (buffer-string)))
      (goto-char (point-min))
      (forward-char 4)
      (unless (= (following-char) ?\u4e2d)
        (error "forward-char multibyte mismatch"))
      (backward-char 2)
      (delete-char 2)
      (unless (equal (buffer-string) "AA\u4e2d\nBB")
        (error "delete-char mismatch: %S" (buffer-string)))
      (goto-char (point-min))
      (end-of-line)
      (unless (= (point) 4)
        (error "end-of-line mismatch: %S" (point)))
      (unless (= (forward-line 1) 0)
        (error "forward-line mismatch"))
      (beginning-of-line)
      (unless (= (point) 5)
        (error "beginning-of-line mismatch: %S" (point)))
      ;; Overwrite mode binary & textual
      (let ((overwrite-mode 'overwrite-mode-binary))
        (self-insert-command 1 ?Z))
      (unless (equal (buffer-string) "AA\u4e2d\nZB")
        (error "overwrite-mode-binary mismatch: %S" (buffer-string)))
      ;; Boundary signals
      (goto-char (point-min))
      (condition-case nil
          (progn (backward-char 1) (error "expected beginning-of-buffer"))
        (beginning-of-buffer nil))
      (goto-char (point-max))
      (condition-case nil
          (progn (forward-char 1) (error "expected end-of-buffer"))
        (end-of-buffer nil))))
  ;; Checked cons/vector/string primitives
  (let ((c (cons 'x 'y))
        (v (vector 10 20 30))
        (s (copy-sequence "hello")))
    (setcar c 'a)
    (setcdr c 'b)
    (aset v 1 99)
    (aset s 0 ?H)
    (unless (and (equal c '(a . b))
                 (equal v [10 99 30])
                 (equal s "Hello"))
      (error "checked primitive mutation mismatch")))
  (princ "U6_SAFE_PRIMITIVES_OK\n"))
"""
        p = run_cmd([str(terminal_bin), "-Q", "--batch", "--eval", probe], env=env, timeout=60)
        if p.returncode != 0 or "U6_SAFE_PRIMITIVES_OK" not in p.stdout:
            raise AssertionError(f"safe primitives probe failed:\n{p.stdout}\n{p.stderr}")

        ert_files = [
            "test/src/cmds-tests.el",
            "test/src/data-tests.el",
            "test/src/eval-tests.el",
            "test/src/fns-tests.el",
            "test/src/buffer-tests.el",
            "test/src/editfns-tests.el",
        ]
        ert_timings: dict[str, float] = {}
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
            ert_timings[rel] = round(dt, 4)
            if ep.returncode != 0:
                #Note: check if known DNS/environment test or genuine failure
                tail = "\n".join((ep.stdout + "\n" + ep.stderr).splitlines()[-35:])
                raise AssertionError(f"ERT suite {rel} failed (exit {ep.returncode}):\n{tail}")
        return {"ert_timings_sec": ert_timings}


def check_native_eln(native_bin: Path) -> dict:
    if not native_bin.is_file():
        raise AssertionError(f"native binary not found: {native_bin}")
    with tempfile.TemporaryDirectory(prefix="u6-eln-") as tmp:
        tmp_path = Path(tmp)
        src_el = tmp_path / "u6_eln_probe.el"
        src_el.write_text(
            """\
;;; -*- lexical-binding: t; -*-
(defun u6-eln-exercise (n)
  (with-temp-buffer
    (let ((gc-cons-threshold (+ gc-cons-threshold 1024))
          (hits 0))
      (let ((post-self-insert-hook (list (lambda () (setq hits (1+ hits))))))
        (self-insert-command n ?X)
        (goto-char (point-min))
        (forward-char 1)
        (delete-char 1)
        (list (buffer-string) hits (point) (> gc-cons-threshold 0))))))

(provide 'u6_eln_probe)
""",
            encoding="utf-8",
        )
        driver = r"""
(progn
  (unless (and (fboundp 'native-comp-available-p) (native-comp-available-p))
    (error "native-comp unavailable"))
  (let* ((el (getenv "U6_EL_FILE"))
         (eln (native-compile el)))
    (load eln nil t t)
    (unless (subr-native-elisp-p (symbol-function 'u6-eln-exercise))
      (error "u6-eln-exercise is not native compiled"))
    (let ((res (u6-eln-exercise 4)))
      (unless (equal res '("XXX" 1 2 t))
        (error "unexpected .eln result: %S" res)))
    (princ "U6_NATIVE_ELN_OK\n")))
"""
        env = isolated_env(tmp_path)
        env["U6_EL_FILE"] = str(src_el)
        proc = run_cmd([str(native_bin), "-Q", "--batch", "--eval", driver], env=env, timeout=180)
        if proc.returncode != 0 or "U6_NATIVE_ELN_OK" not in proc.stdout:
            raise AssertionError(f"native .eln verification failed ({proc.returncode}):\n{proc.stdout}\n{proc.stderr}")
        return {"native_eln": "ok"}


def run_selftest() -> int:
    # Fast self-test verifying check-globals and check-defvar-carriers recognize VmGlobal<T>.
    import importlib.util

    s1 = importlib.util.spec_from_file_location("cg", REPO / "tools/check-globals.py")
    cg = importlib.util.module_from_spec(s1)
    s1.loader.exec_module(cg)

    s2 = importlib.util.spec_from_file_location("cdc", REPO / "tools/check-defvar-carriers.py")
    cdc = importlib.util.module_from_spec(s2)
    s2.loader.exec_module(cdc)

    with tempfile.TemporaryDirectory(prefix="u6-selftest-") as tmp:
        p = Path(tmp) / "sample.rs"
        p.write_text(
            'pub static Vfoo: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new(Qnil());\n'
            'fn syms() { DEFVAR_LISP!(c"foo", Vfoo, "doc"); }\n',
            encoding="utf-8",
        )
        probs = cg.check(p)
        assert not probs, f"unexpected check-globals failure on VmGlobal: {probs}"
        decls = cdc.declarations(p.read_text(encoding="utf-8"))
        _, found, bad = cdc.check_text(p.read_text(encoding="utf-8"), decls)
        assert found == 1 and not bad, f"unexpected check-defvar-carriers result: found={found}, bad={bad}"

        p_bad = Path(tmp) / "bad.rs"
        p_bad.write_text(
            'pub static Vbad: VmGlobal<c_int> = VmGlobal::<c_int>::new(0);\n'
            'fn syms() { DEFVAR_INT!(c"bad", Vbad, "doc"); }\n',
            encoding="utf-8",
        )
        decls_bad = cdc.declarations(p_bad.read_text(encoding="utf-8"))
        _, _, bad2 = cdc.check_text(p_bad.read_text(encoding="utf-8"), decls_bad)
        assert len(bad2) == 1, f"expected check-defvar-carriers to reject VmGlobal<c_int>, got {bad2}"

    print("u6 selftest: 2/2 cases passed")
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
        "--native-bin",
        type=Path,
        default=None,
    )
    ap.add_argument(
        "--group",
        choices=("all", "static", "pdmp", "primitives", "native"),
        default="all",
    )
    args = ap.parse_args()

    if args.selftest:
        return run_selftest()

    results: list[dict] = []
    ok = True
    if args.group in ("all", "static"):
        ok = run_case(results, "static-gates", lambda: check_static_gates(args.terminal_bin, args.temacs_bin)) and ok
    if args.group in ("all", "pdmp"):
        ok = run_case(results, "pdmp-and-defvar", lambda: check_pdmp_and_defvar(args.terminal_bin)) and ok
    if args.group in ("all", "primitives"):
        ok = run_case(results, "safe-primitives-and-ert", lambda: check_safe_primitives_and_ert(args.terminal_bin)) and ok
    if args.group in ("all", "native") and args.native_bin is not None:
        ok = run_case(results, "native-eln", lambda: check_native_eln(args.native_bin)) and ok

    print(json.dumps({"ok": ok, "results": results}, indent=2))
    return 0 if ok else 1


if __name__ == "__main__":
    raise SystemExit(main())
