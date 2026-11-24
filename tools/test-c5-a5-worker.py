#!/usr/bin/env python3
"""Verify Phase I Stage C5/A5 opt-in Eliscript worker integration.

Archives a clean snapshot of the Eliscript repository without modifying the
source checkout, builds the frozen runtime and worker artifacts via the
delivered standalone Lisp package, strips compiler/bootstrap files, copies the
FEmacs opt-in client and acceptance fixture into a detached workspace, and runs
full asynchronous worker acceptance inside both installed Cocoa Fe.app and
rebuilt Terminal PTY sessions.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import pty
import select
import shutil
import signal
import subprocess
import tempfile
import time
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
DEFAULT_CLIENT = REPO / "examples/eliscript-worker/femacs-eliscript-worker.el"
DEFAULT_FIXTURE = REPO / "tools/fixtures/c5-a5-worker-cases.el"

C5_PROBES_ELI = r"""(module c5.probes
  (import "node:fs/promises" appendFile)
  (import "eliscript/platform/worker.mjs" workerProgress)

  (defun chunked-echo (payload _context)
    (let ((caps (and _context (get _context :capabilities))))
      (when caps
        (workerProgress caps (js-object :stage "halfway"
                                        :received (not (nullish? payload)))))
      payload))

  (defasync slow-cancellable (steps delay-ms _context)
    (let ((signal (and _context (get _context :signal)))
          (caps (and _context (get _context :capabilities)))
          (i 0))
      (while (< i steps)
        (when (and signal (get signal :aborted))
          (let ((err (new globalThis.Error "probe cancelled")))
            (put err :code "cancelled")
            (throw err)))
        (setq i (1+ i))
        (when caps
          (workerProgress caps (js-object :step i :steps steps)))
        (await (new globalThis.Promise
                 (lambda (resolve _reject)
                   (globalThis.setTimeout (lambda () (funcall resolve nil))
                                          delay-ms)))))
      (js-object :completed steps)))

  (defun trigger-mapped-error (reason _context)
    (throw (new globalThis.Error (str "c5-mapped-error: " reason))))

  (defun hang-forever (_context)
    (new globalThis.Promise (lambda (_resolve _reject) nil)))

  (defun stderr-log-echo (msg _context)
    (js-call globalThis.console :log (str "worker-stderr-log:" msg))
    (js-object :echoed msg))

  (defasync side-effect-append (file-path token delay-ms _context)
    (await (appendFile file-path (str token "\n") "utf8"))
    (await (new globalThis.Promise
             (lambda (resolve _reject)
               (globalThis.setTimeout
                 (lambda () (funcall resolve (js-object :appended token)))
                 delay-ms)))))

  (defun ping-resolution (_context)
    (js-object :resolved t
               :progressFn (= (value-type workerProgress) "function")))

  (export chunked-echo slow-cancellable trigger-mapped-error hang-forever
          stderr-log-echo side-effect-append ping-resolution))
"""


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        while chunk := f.read(65536):
            h.update(chunk)
    return h.hexdigest()


def run_checked(
    cmd: list[str],
    *,
    cwd: Path | None = None,
    env: dict[str, str] | None = None,
    timeout: float = 300.0,
) -> subprocess.CompletedProcess[str]:
    proc = subprocess.run(
        cmd,
        cwd=cwd,
        env=env,
        text=True,
        capture_output=True,
        timeout=timeout,
    )
    if proc.returncode != 0:
        raise RuntimeError(
            f"command failed (rc={proc.returncode}): {' '.join(cmd)}\n"
            f"STDOUT:\n{proc.stdout}\nSTDERR:\n{proc.stderr}"
        )
    return proc


def copy_runtime_tree(src: Path, dst: Path) -> None:
    if dst.exists():
        shutil.rmtree(dst)
    shutil.copytree(
        src,
        dst,
        symlinks=False,
        ignore=shutil.ignore_patterns(".DS_Store", "*.eli"),
    )


def prepare_frozen_package(
    eliscript_repo: Path,
    eliscript_ref: str,
    femacs_lisp: Path,
    package_out: Path,
) -> dict[str, object]:
    commit_sha = run_checked(
        ["git", "-C", str(eliscript_repo), "rev-parse", eliscript_ref],
        timeout=15.0,
    ).stdout.strip()
    if package_out.exists():
        shutil.rmtree(package_out)
    package_out.mkdir(parents=True, exist_ok=True)

    with tempfile.TemporaryDirectory(prefix="c5-eliscript-build-") as build_tmp:
        build_root = Path(build_tmp)
        archive_src = build_root / "src"
        archive_src.mkdir(parents=True, exist_ok=True)
        tar_proc = subprocess.run(
            ["git", "-C", str(eliscript_repo), "archive", "--format=tar", commit_sha],
            capture_output=True,
            check=True,
            timeout=30.0,
        )
        subprocess.run(
            ["tar", "-xf", "-", "-C", str(archive_src)],
            input=tar_proc.stdout,
            check=True,
            timeout=30.0,
        )

        env = os.environ.copy()
        env["EMACS"] = str(femacs_lisp.resolve())
        env.setdefault("ELISCRIPT_NODE_HEAP_MIB", "4096")
        env["NODE_OPTIONS"] = f"--max-old-space-size={env['ELISCRIPT_NODE_HEAP_MIB']}"
        env["ELISCRIPT_JS_RUNTIME"] = "node"

        run_checked(["make", "bootstrap-release"], cwd=archive_src, env=env, timeout=600.0)
        run_checked(["make", "prepare-release-package"], cwd=archive_src, env=env, timeout=600.0)

        # Stage compiled analysis and C5 probe modules while the temporary build tree exists.
        analysis_out = package_out / "analysis"
        build_env = env.copy()
        build_env["ELISCRIPT_JS_RUNTIME"] = shutil.which("bun") or "bun"
        run_checked(
            [
                str(archive_src / "bin/eliscript-build"),
                "--root",
                str(archive_src / "examples/emacs-analysis"),
                "--out-dir",
                str(analysis_out),
                str(archive_src / "examples/emacs-analysis/analysis.eli"),
            ],
            cwd=archive_src,
            env=build_env,
            timeout=120.0,
        )
        if (analysis_out / "node_modules").exists():
            shutil.rmtree(analysis_out / "node_modules")

        probe_src_dir = build_root / "probes_src"
        probe_src_dir.mkdir(parents=True, exist_ok=True)
        probe_entry = probe_src_dir / "c5-probes.eli"
        probe_entry.write_text(C5_PROBES_ELI, encoding="utf-8")
        probes_out = package_out / "probes"
        run_checked(
            [
                str(archive_src / "bin/eliscript-build"),
                "--root",
                str(probe_src_dir),
                "--out-dir",
                str(probes_out),
                str(probe_entry),
            ],
            cwd=archive_src,
            env=build_env,
            timeout=120.0,
        )
        if (probes_out / "node_modules").exists():
            shutil.rmtree(probes_out / "node_modules")

        # Copy frozen runtime, platform, stdlib, dist/runtime, package.json, and worker Lisp closure.
        shutil.copy2(archive_src / "package.json", package_out / "package.json")
        copy_runtime_tree(archive_src / "runtime", package_out / "runtime")
        copy_runtime_tree(archive_src / "platform", package_out / "platform")
        copy_runtime_tree(archive_src / "stdlib", package_out / "stdlib")
        copy_runtime_tree(archive_src / "dist/runtime", package_out / "dist/runtime")

        client_dst = package_out / "tools/worker"
        client_dst.mkdir(parents=True, exist_ok=True)
        for name in (
            "eliscript-value-codec.el",
            "eliscript-value-stream.el",
            "eliscript-worker.el",
            "eliscript-service.el",
        ):
            shutil.copy2(archive_src / "tools/worker" / name, client_dst / name)

    # Confirm compiler/bootstrap files are absent from the frozen runtime package.
    assert not (package_out / "compiler").exists(), "compiler tree leaked into frozen package"
    assert not (
        package_out / "tools/bootstrap-runtime.el"
    ).exists(), "bootstrap-runtime.el leaked into frozen package"
    assert (package_out / "runtime/worker.mjs").is_file(), "missing frozen runtime/worker.mjs"
    assert (package_out / "stdlib/bit.mjs").is_file(), "missing frozen stdlib/bit.mjs"
    assert (package_out / "dist/runtime/lisp.mjs").is_file(), "missing frozen dist/runtime/lisp.mjs"

    key_files = [
        "package.json",
        "runtime/worker.mjs",
        "runtime/worker.mjs.map",
        "platform/worker.mjs",
        "stdlib/bit.mjs",
        "dist/runtime/lisp.mjs",
        "tools/worker/eliscript-worker.el",
        "tools/worker/eliscript-service.el",
        "tools/worker/eliscript-value-codec.el",
        "tools/worker/eliscript-value-stream.el",
        "analysis/analysis.mjs",
        "analysis/eliscript-project.json",
        "probes/c5-probes.mjs",
        "probes/c5-probes.mjs.map",
        "probes/eliscript-project.json",
    ]
    manifest = {
        "schema": "femacs-c5-frozen-eliscript-v1",
        "eliscript_commit": commit_sha,
        "builder_femacs_lisp": str(femacs_lisp),
        "builder_femacs_lisp_sha256": sha256_file(femacs_lisp),
        "files": {
            rel: {
                "bytes": (package_out / rel).stat().st_size,
                "sha256": sha256_file(package_out / rel),
            }
            for rel in key_files
        },
    }
    (package_out / "frozen-manifest.json").write_text(
        json.dumps(manifest, indent=2) + "\n", encoding="utf-8"
    )
    return manifest


def check_opt_in_clean_batch(binary: Path, label: str) -> dict[str, object]:
    proc = run_checked(
        [
            str(binary),
            "-Q",
            "--batch",
            "--eval",
            '(princ (list (featurep (quote femacs-eliscript-worker)) '
            '(featurep (quote eliscript-worker)) '
            '(length (process-list))))',
        ],
        timeout=30.0,
    )
    if "(nil nil 0)" not in proc.stdout:
        raise AssertionError(f"{label} opt-in clean check unexpected output: {proc.stdout!r}")
    return {"label": label, "passed": True, "stdout": proc.stdout.strip()}


def run_cocoa_gui_cases(
    app_bundle: Path,
    worker_el: Path,
    fixture_el: Path,
    pkg_dir: Path,
    runner: str,
    work_dir: Path,
) -> dict[str, object]:
    app_bin = app_bundle / "Contents/MacOS/Femacs"
    assert app_bin.is_file(), f"missing installed GUI binary: {app_bin}"
    report_path = work_dir / "cocoa-report.json"
    env = os.environ.copy()
    env.update(
        {
            "FEMACS_A5_WORKER_EL": str(worker_el),
            "FEMACS_A5_PKG_DIR": str(pkg_dir),
            "FEMACS_A5_RUNNER": runner,
            "FEMACS_A5_EXPECT_GUI": "ns",
            "FEMACS_A5_WORK_DIR": str(work_dir),
            "FEMACS_A5_REPORT": str(report_path),
        }
    )
    started = time.monotonic()
    log_path = work_dir / "cocoa-stdio.log"
    with log_path.open("wb") as logf:
        proc = subprocess.Popen(
            [str(app_bin), "-Q", "-l", str(fixture_el)],
            cwd=work_dir,
            env=env,
            stdout=logf,
            stderr=subprocess.STDOUT,
        )
        try:
            rc = proc.wait(timeout=75.0)
        except subprocess.TimeoutExpired:
            proc.kill()
            proc.wait(timeout=5.0)
            raise RuntimeError("Cocoa GUI A5 harness timed out")
    elapsed = round(time.monotonic() - started, 4)
    if rc != 0 or not report_path.is_file():
        log_text = log_path.read_text(encoding="utf-8", errors="replace")
        rep_text = report_path.read_text(encoding="utf-8", errors="replace") if report_path.is_file() else ""
        raise RuntimeError(f"Cocoa A5 failed (rc={rc}):\nREPORT:\n{rep_text}\nLOG:\n{log_text}")
    data = json.loads(report_path.read_text(encoding="utf-8"))
    if not data.get("passed") or data.get("frame_type") != "ns" or data.get("window_system") != "ns":
        raise RuntimeError(f"Cocoa A5 report indicated failure: {data}")
    data["elapsed_s"] = elapsed
    data["binary_sha256"] = sha256_file(app_bin)
    return data


def run_terminal_pty_cases(
    term_bin: Path,
    worker_el: Path,
    fixture_el: Path,
    pkg_dir: Path,
    runner: str,
    work_dir: Path,
) -> dict[str, object]:
    assert term_bin.is_file(), f"missing terminal binary: {term_bin}"
    report_path = work_dir / "terminal-report.json"
    env = os.environ.copy()
    env.update(
        {
            "TERM": "xterm-256color",
            "LC_ALL": "en_US.UTF-8",
            "FEMACS_A5_WORKER_EL": str(worker_el),
            "FEMACS_A5_PKG_DIR": str(pkg_dir),
            "FEMACS_A5_RUNNER": runner,
            "FEMACS_A5_EXPECT_GUI": "tty",
            "FEMACS_A5_WORK_DIR": str(work_dir),
            "FEMACS_A5_REPORT": str(report_path),
        }
    )
    started = time.monotonic()
    pid, master = pty.fork()
    if pid == 0:
        os.chdir(str(work_dir))
        os.execve(str(term_bin), [str(term_bin), "-nw", "-Q", "-l", str(fixture_el)], env)
    os.set_blocking(master, False)
    screen = bytearray()
    exit_code: int | None = None
    try:
        deadline = time.monotonic() + 75.0
        while time.monotonic() < deadline:
            if select.select([master], [], [], 0.05)[0]:
                try:
                    chunk = os.read(master, 16384)
                except OSError:
                    chunk = b""
                if chunk:
                    screen.extend(chunk)
                    for q, ans in (
                        (b"\x1b[6n", b"\x1b[1;1R"),
                        (b"\x1b[>c", b"\x1b[>0;95;0c"),
                        (b"\x1b[c", b"\x1b[?1;2c"),
                    ):
                        if q in chunk:
                            os.write(master, ans)
            got, status = os.waitpid(pid, os.WNOHANG)
            if got:
                exit_code = os.waitstatus_to_exitcode(status)
                break
        if exit_code is None:
            os.kill(pid, signal.SIGKILL)
            os.waitpid(pid, 0)
            raise RuntimeError("Terminal PTY A5 harness timed out")
    finally:
        try:
            os.close(master)
        except OSError:
            pass
    (work_dir / "terminal-pty.ansi").write_bytes(screen)
    elapsed = round(time.monotonic() - started, 4)
    if exit_code != 0 or not report_path.is_file():
        rep_text = report_path.read_text(encoding="utf-8", errors="replace") if report_path.is_file() else ""
        raise RuntimeError(f"Terminal A5 failed (rc={exit_code}):\nREPORT:\n{rep_text}\nSCREEN:\n{screen[-800:]!r}")
    data = json.loads(report_path.read_text(encoding="utf-8"))
    if not data.get("passed") or data.get("frame_type") != "t":
        raise RuntimeError(f"Terminal A5 report indicated failure: {data}")
    data["elapsed_s"] = elapsed
    data["binary_sha256"] = sha256_file(term_bin)
    return data


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--app", type=Path, default=Path("/Applications/Fe.app"))
    parser.add_argument("--terminal", type=Path, default=REPO / "build/terminal/port/src/emacs")
    parser.add_argument("--eliscript-repo", type=Path, default=REPO.parent / "eliscript")
    parser.add_argument("--eliscript-ref", default="HEAD")
    parser.add_argument("--femacs-lisp", type=Path, default=REPO / "dist/lisp/bin/femacs-lisp")
    parser.add_argument("--runner", default="bun")
    parser.add_argument("--frozen-package", type=Path, default=None)
    parser.add_argument("--prepare-only", action="store_true")
    parser.add_argument("--out", type=Path, default=REPO / "build/acceptance/c5-a5-worker")
    args = parser.parse_args(argv)

    out_dir = args.out.resolve()
    out_dir.mkdir(parents=True, exist_ok=True)
    app_bundle = args.app.resolve()
    terminal_bin = args.terminal.resolve()
    femacs_lisp = args.femacs_lisp.resolve()

    if args.frozen_package is not None and (args.frozen_package / "frozen-manifest.json").is_file() and not args.prepare_only:
        pkg_dir = args.frozen_package.resolve()
        manifest = json.loads((pkg_dir / "frozen-manifest.json").read_text(encoding="utf-8"))
    else:
        pkg_dir = (args.frozen_package or (out_dir / "frozen-eliscript")).resolve()
        manifest = prepare_frozen_package(
            args.eliscript_repo.resolve(), args.eliscript_ref, femacs_lisp, pkg_dir
        )

    if args.prepare_only:
        print(f"Prepared frozen Eliscript package at {pkg_dir} (commit={manifest['eliscript_commit']})")
        return 0

    with tempfile.TemporaryDirectory(prefix="femacs-a5-detached-") as detached_tmp:
        detached = Path(detached_tmp)
        worker_copy = detached / "femacs-eliscript-worker.el"
        fixture_copy = detached / "c5-a5-worker-cases.el"
        shutil.copy2(DEFAULT_CLIENT, worker_copy)
        shutil.copy2(DEFAULT_FIXTURE, fixture_copy)

        opt_in_app = check_opt_in_clean_batch(app_bundle / "Contents/MacOS/Femacs", "Fe.app")
        opt_in_tty = check_opt_in_clean_batch(terminal_bin, "terminal")

        cocoa_work = detached / "cocoa-work"
        cocoa_work.mkdir(parents=True, exist_ok=True)
        cocoa_res = run_cocoa_gui_cases(
            app_bundle, worker_copy, fixture_copy, pkg_dir, args.runner, cocoa_work
        )

        term_work = detached / "terminal-work"
        term_work.mkdir(parents=True, exist_ok=True)
        term_res = run_terminal_pty_cases(
            terminal_bin, worker_copy, fixture_copy, pkg_dir, args.runner, term_work
        )

    summary = {
        "schema": "femacs-c5-a5-worker-v1",
        "runner": args.runner,
        "frozen_package": manifest,
        "opt_in_clean": [opt_in_app, opt_in_tty],
        "cocoa": cocoa_res,
        "terminal": term_res,
    }
    report_file = out_dir / "c5-a5-worker-report.json"
    report_file.write_text(json.dumps(summary, indent=2) + "\n", encoding="utf-8")
    print(f"C5/A5 acceptance PASSED ({args.runner}) -> {report_file}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
