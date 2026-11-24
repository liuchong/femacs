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
import re
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

REQUIRED_CLOSURE_FILES = (
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
)
HEX40_RE = re.compile(r"^[0-9a-f]{40}$")
HEX64_RE = re.compile(r"^[0-9a-f]{64}$")

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

  (defun report-cwd (_context)
    (js-object :cwd (js-call globalThis.process :cwd)))

  (export chunked-echo slow-cancellable trigger-mapped-error hang-forever
          stderr-log-echo side-effect-append ping-resolution report-cwd))
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
    if dst.exists() or dst.is_symlink():
        raise FileExistsError(f"refusing to overwrite existing runtime tree: {dst}")
    shutil.copytree(
        src,
        dst,
        symlinks=False,
        ignore=shutil.ignore_patterns(".DS_Store", "*.eli", "node_modules"),
    )


def compute_closure_inventory(pkg_dir: Path) -> tuple[dict[str, dict[str, object]], str]:
    """Walk every entry under PKG_DIR (excluding frozen-manifest.json) and digest the closure."""
    if not pkg_dir.is_dir():
        raise FileNotFoundError(f"frozen package directory does not exist: {pkg_dir}")
    files: dict[str, dict[str, object]] = {}
    for entry in sorted(pkg_dir.rglob("*")):
        rel_path = entry.relative_to(pkg_dir)
        rel = rel_path.as_posix()
        if entry.is_symlink():
            raise ValueError(f"unexpected symlink in frozen package: {rel}")
        if "compiler" in rel_path.parts or "node_modules" in rel_path.parts:
            raise ValueError(f"forbidden directory leaked into frozen package: {rel}")
        if entry.is_dir():
            continue
        if not entry.is_file():
            raise ValueError(f"unsupported non-regular entry in frozen package: {rel}")
        if rel == "frozen-manifest.json":
            continue
        if rel_path.suffix == ".eli" or rel == "tools/bootstrap-runtime.el":
            raise ValueError(f"forbidden source/bootstrap file in frozen package: {rel}")
        files[rel] = {
            "bytes": entry.stat().st_size,
            "sha256": sha256_file(entry),
        }
    lines = [
        f"{rel}\0{files[rel]['bytes']}\0{files[rel]['sha256']}"
        for rel in sorted(files)
    ]
    closure_sha256 = hashlib.sha256(("\n".join(lines) + "\n").encode("utf-8")).hexdigest()
    return files, closure_sha256


def verify_frozen_package(pkg_dir: Path) -> dict[str, object]:
    """Validate frozen-manifest.json and verify full runtime-closure integrity on disk."""
    if not pkg_dir.is_dir():
        raise FileNotFoundError(f"frozen package directory does not exist: {pkg_dir}")
    manifest_path = pkg_dir / "frozen-manifest.json"
    if not manifest_path.is_file():
        raise FileNotFoundError(f"missing frozen-manifest.json in {pkg_dir}")
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    if manifest.get("schema") != "femacs-c5-frozen-eliscript-v1":
        raise ValueError(f"unexpected frozen manifest schema: {manifest.get('schema')!r}")
    commit_sha = manifest.get("eliscript_commit")
    if not isinstance(commit_sha, str) or not HEX40_RE.match(commit_sha):
        raise ValueError(f"invalid eliscript_commit in frozen manifest: {commit_sha!r}")
    expected_closure = manifest.get("closure_sha256")
    if not isinstance(expected_closure, str) or not HEX64_RE.match(expected_closure):
        raise ValueError(f"invalid closure_sha256 in frozen manifest: {expected_closure!r}")
    builder_sha = manifest.get("builder_femacs_lisp_sha256")
    if not isinstance(builder_sha, str) or not HEX64_RE.match(builder_sha):
        raise ValueError(f"invalid builder_femacs_lisp_sha256 in frozen manifest: {builder_sha!r}")
    manifest_files = manifest.get("files")
    if not isinstance(manifest_files, dict):
        raise ValueError("frozen manifest files must be a JSON object")
    if manifest.get("file_count") != len(manifest_files):
        raise ValueError(
            f"frozen manifest file_count={manifest.get('file_count')!r} != {len(manifest_files)}"
        )
    for req in REQUIRED_CLOSURE_FILES:
        if req not in manifest_files:
            raise ValueError(f"frozen manifest missing required closure file: {req}")

    actual_files, actual_closure = compute_closure_inventory(pkg_dir)
    missing = sorted(set(manifest_files) - set(actual_files))
    extra = sorted(set(actual_files) - set(manifest_files))
    if missing or extra:
        raise ValueError(
            f"frozen package file set mismatch in {pkg_dir}: missing={missing[:5]} extra={extra[:5]}"
        )
    for rel, expected in sorted(manifest_files.items()):
        if not isinstance(expected, dict) or actual_files[rel] != expected:
            raise ValueError(
                f"frozen package file integrity mismatch for {rel}: "
                f"expected={expected!r} actual={actual_files[rel]!r}"
            )
    if actual_closure != expected_closure:
        raise ValueError(
            f"frozen package closure_sha256 mismatch: expected={expected_closure} actual={actual_closure}"
        )
    return manifest


def prepare_frozen_package(
    eliscript_repo: Path,
    eliscript_ref: str,
    femacs_lisp: Path,
    package_out: Path,
) -> dict[str, object]:
    if package_out.exists() or package_out.is_symlink():
        raise FileExistsError(f"refusing to overwrite existing package output: {package_out}")
    commit_sha = run_checked(
        ["git", "-C", str(eliscript_repo), "rev-parse", eliscript_ref],
        timeout=15.0,
    ).stdout.strip()
    package_out.parent.mkdir(parents=True, exist_ok=True)

    with tempfile.TemporaryDirectory(prefix="c5-eliscript-build-") as build_tmp:
        build_root = Path(build_tmp)
        archive_src = build_root / "src"
        archive_src.mkdir(parents=True, exist_ok=True)
        stage_pkg = build_root / "package_stage"
        stage_pkg.mkdir(parents=True, exist_ok=True)
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

        # Stage compiled analysis and C5 probe modules inside build_root, strip
        # per-module node_modules there, and copy only the runtime closure.
        build_env = env.copy()
        build_env["ELISCRIPT_JS_RUNTIME"] = shutil.which("bun") or "bun"
        analysis_stage = build_root / "analysis_stage"
        run_checked(
            [
                str(archive_src / "bin/eliscript-build"),
                "--root",
                str(archive_src / "examples/emacs-analysis"),
                "--out-dir",
                str(analysis_stage),
                str(archive_src / "examples/emacs-analysis/analysis.eli"),
            ],
            cwd=archive_src,
            env=build_env,
            timeout=120.0,
        )
        copy_runtime_tree(analysis_stage, stage_pkg / "analysis")

        probe_src_dir = build_root / "probes_src"
        probe_src_dir.mkdir(parents=True, exist_ok=True)
        probe_entry = probe_src_dir / "c5-probes.eli"
        probe_entry.write_text(C5_PROBES_ELI, encoding="utf-8")
        probes_stage = build_root / "probes_stage"
        run_checked(
            [
                str(archive_src / "bin/eliscript-build"),
                "--root",
                str(probe_src_dir),
                "--out-dir",
                str(probes_stage),
                str(probe_entry),
            ],
            cwd=archive_src,
            env=build_env,
            timeout=120.0,
        )
        copy_runtime_tree(probes_stage, stage_pkg / "probes")

        # Copy frozen runtime, platform, stdlib, dist/runtime, package.json, and worker Lisp closure.
        shutil.copy2(archive_src / "package.json", stage_pkg / "package.json")
        copy_runtime_tree(archive_src / "runtime", stage_pkg / "runtime")
        copy_runtime_tree(archive_src / "platform", stage_pkg / "platform")
        copy_runtime_tree(archive_src / "stdlib", stage_pkg / "stdlib")
        copy_runtime_tree(archive_src / "dist/runtime", stage_pkg / "dist/runtime")

        client_dst = stage_pkg / "tools/worker"
        client_dst.mkdir(parents=True, exist_ok=True)
        for name in (
            "eliscript-value-codec.el",
            "eliscript-value-stream.el",
            "eliscript-worker.el",
            "eliscript-service.el",
        ):
            shutil.copy2(archive_src / "tools/worker" / name, client_dst / name)

        closure_files, closure_sha256 = compute_closure_inventory(stage_pkg)
        for req in REQUIRED_CLOSURE_FILES:
            assert req in closure_files, f"missing required closure file: {req}"
        manifest = {
            "schema": "femacs-c5-frozen-eliscript-v1",
            "eliscript_commit": commit_sha,
            "builder_femacs_lisp_sha256": sha256_file(femacs_lisp),
            "file_count": len(closure_files),
            "closure_sha256": closure_sha256,
            "files": closure_files,
        }
        (stage_pkg / "frozen-manifest.json").write_text(
            json.dumps(manifest, indent=2) + "\n", encoding="utf-8"
        )
        verify_frozen_package(stage_pkg)
        shutil.move(str(stage_pkg), str(package_out))
    return verify_frozen_package(package_out)


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
    *,
    extra_env: dict[str, str] | None = None,
    unset_env: tuple[str, ...] | list[str] | None = None,
) -> dict[str, object]:
    app_bin = app_bundle / "Contents/MacOS/Femacs"
    assert app_bin.is_file(), f"missing installed GUI binary: {app_bin}"
    report_path = work_dir / "cocoa-report.json"
    env = os.environ.copy()
    if unset_env:
        for key in unset_env:
            env.pop(key, None)
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
    if extra_env:
        env.update(extra_env)
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
    *,
    extra_env: dict[str, str] | None = None,
    unset_env: tuple[str, ...] | list[str] | None = None,
) -> dict[str, object]:
    assert term_bin.is_file(), f"missing terminal binary: {term_bin}"
    report_path = work_dir / "terminal-report.json"
    env = os.environ.copy()
    if unset_env:
        for key in unset_env:
            env.pop(key, None)
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
    if extra_env:
        env.update(extra_env)
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


def run_guard_self_tests(valid_pkg: Path) -> dict[str, object]:
    """Verify read-only input guards, non-overwrite output guards, and closure integrity rejection."""
    verified = verify_frozen_package(valid_pkg)
    script = str(Path(__file__).resolve())
    checks: list[dict[str, object]] = []

    with tempfile.TemporaryDirectory(prefix="femacs-c5-guards-") as tmp_dir:
        tmp = Path(tmp_dir)

        # 1. Existing --package-out is refused and preserved untouched.
        existing_pkg_out = tmp / "existing-pkg-out"
        existing_pkg_out.mkdir()
        marker1 = existing_pkg_out / "keep.txt"
        marker1.write_text("preserve-me", encoding="utf-8")
        p1 = subprocess.run(
            ["python3", script, "--prepare-only", "--package-out", str(existing_pkg_out)],
            capture_output=True,
            text=True,
            timeout=15.0,
        )
        assert p1.returncode != 0 and marker1.read_text(encoding="utf-8") == "preserve-me"
        checks.append({"id": "refuse_existing_package_out", "passed": True, "rc": p1.returncode})

        # 2. Existing --out is refused and preserved untouched.
        existing_out = tmp / "existing-out"
        existing_out.mkdir()
        marker2 = existing_out / "keep.txt"
        marker2.write_text("preserve-out", encoding="utf-8")
        p2 = subprocess.run(
            [
                "python3",
                script,
                "--frozen-package",
                str(valid_pkg),
                "--out",
                str(existing_out),
            ],
            capture_output=True,
            text=True,
            timeout=15.0,
        )
        assert p2.returncode != 0 and marker2.read_text(encoding="utf-8") == "preserve-out"
        checks.append({"id": "refuse_existing_out_dir", "passed": True, "rc": p2.returncode})

        # 3. --frozen-package missing frozen-manifest.json is refused with no rebuild fallback.
        no_manifest_pkg = tmp / "no-manifest-pkg"
        shutil.copytree(valid_pkg, no_manifest_pkg, symlinks=False)
        (no_manifest_pkg / "frozen-manifest.json").unlink()
        marker3 = no_manifest_pkg / "sentinel.txt"
        marker3.write_text("intact", encoding="utf-8")
        p3 = subprocess.run(
            [
                "python3",
                script,
                "--frozen-package",
                str(no_manifest_pkg),
                "--out",
                str(tmp / "out-3"),
            ],
            capture_output=True,
            text=True,
            timeout=15.0,
        )
        assert (
            p3.returncode != 0
            and marker3.read_text(encoding="utf-8") == "intact"
            and not (no_manifest_pkg / "frozen-manifest.json").exists()
            and not (tmp / "out-3").exists()
        )
        checks.append({"id": "refuse_missing_manifest_no_rebuild", "passed": True, "rc": p3.returncode})

        # 4. --frozen-package combined with --prepare-only or --package-out is refused.
        p4a = subprocess.run(
            ["python3", script, "--frozen-package", str(valid_pkg), "--prepare-only"],
            capture_output=True,
            text=True,
            timeout=15.0,
        )
        p4b = subprocess.run(
            [
                "python3",
                script,
                "--frozen-package",
                str(valid_pkg),
                "--package-out",
                str(tmp / "out-4b"),
            ],
            capture_output=True,
            text=True,
            timeout=15.0,
        )
        assert p4a.returncode != 0 and p4b.returncode != 0
        checks.append(
            {
                "id": "refuse_conflicting_frozen_input_and_prepare",
                "passed": True,
                "rc_prepare": p4a.returncode,
                "rc_package_out": p4b.returncode,
            }
        )

        # 5. Tampered runtime closure file is rejected before running acceptance.
        tampered_file_pkg = tmp / "tampered-file-pkg"
        shutil.copytree(valid_pkg, tampered_file_pkg, symlinks=False)
        with (tampered_file_pkg / "stdlib/bit.mjs").open("a", encoding="utf-8") as f:
            f.write("\n// tampered\n")
        p5 = subprocess.run(
            [
                "python3",
                script,
                "--frozen-package",
                str(tampered_file_pkg),
                "--out",
                str(tmp / "out-5"),
            ],
            capture_output=True,
            text=True,
            timeout=15.0,
        )
        assert p5.returncode != 0 and not (tmp / "out-5").exists()
        checks.append({"id": "reject_tampered_closure_file", "passed": True, "rc": p5.returncode})

        # 6. Extra injected file in frozen package is rejected.
        extra_file_pkg = tmp / "extra-file-pkg"
        shutil.copytree(valid_pkg, extra_file_pkg, symlinks=False)
        (extra_file_pkg / "rogue.mjs").write_text("export const rogue = 1;\n", encoding="utf-8")
        p6 = subprocess.run(
            [
                "python3",
                script,
                "--frozen-package",
                str(extra_file_pkg),
                "--out",
                str(tmp / "out-6"),
            ],
            capture_output=True,
            text=True,
            timeout=15.0,
        )
        assert p6.returncode != 0 and not (tmp / "out-6").exists()
        checks.append({"id": "reject_extra_injected_file", "passed": True, "rc": p6.returncode})

        # 7. Tampered closure_sha256 in frozen-manifest.json is rejected.
        tampered_digest_pkg = tmp / "tampered-digest-pkg"
        shutil.copytree(valid_pkg, tampered_digest_pkg, symlinks=False)
        bad_manifest = json.loads((tampered_digest_pkg / "frozen-manifest.json").read_text(encoding="utf-8"))
        bad_manifest["closure_sha256"] = "0" * 64
        (tampered_digest_pkg / "frozen-manifest.json").write_text(
            json.dumps(bad_manifest, indent=2) + "\n", encoding="utf-8"
        )
        p7 = subprocess.run(
            [
                "python3",
                script,
                "--frozen-package",
                str(tampered_digest_pkg),
                "--out",
                str(tmp / "out-7"),
            ],
            capture_output=True,
            text=True,
            timeout=15.0,
        )
        assert p7.returncode != 0 and not (tmp / "out-7").exists()
        checks.append({"id": "reject_tampered_closure_digest", "passed": True, "rc": p7.returncode})

        # 8. Placing --out inside read-only --frozen-package is refused without mutating closure.
        nested_pkg = tmp / "nested-out-pkg"
        shutil.copytree(valid_pkg, nested_pkg, symlinks=False)
        before_closure = verify_frozen_package(nested_pkg)["closure_sha256"]
        nested_out = nested_pkg / "nested-report"
        p8 = subprocess.run(
            [
                "python3",
                script,
                "--frozen-package",
                str(nested_pkg),
                "--out",
                str(nested_out),
            ],
            capture_output=True,
            text=True,
            timeout=15.0,
        )
        after_closure = verify_frozen_package(nested_pkg)["closure_sha256"]
        assert (
            p8.returncode != 0
            and not nested_out.exists()
            and after_closure == before_closure
        )
        checks.append(
            {
                "id": "refuse_out_inside_frozen_package",
                "passed": True,
                "rc": p8.returncode,
                "closure_sha256": after_closure,
            }
        )

    checks.append(
        {
            "id": "verify_valid_frozen_package",
            "passed": True,
            "closure_sha256": verified["closure_sha256"],
            "file_count": verified["file_count"],
        }
    )
    return {
        "schema": "femacs-c5-guards-v1",
        "passed": True,
        "closure_sha256": verified["closure_sha256"],
        "file_count": verified["file_count"],
        "checks": checks,
    }


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--app", type=Path, default=Path("/Applications/Fe.app"))
    parser.add_argument("--terminal", type=Path, default=REPO / "build/terminal/port/src/emacs")
    parser.add_argument("--eliscript-repo", type=Path, default=REPO.parent / "eliscript")
    parser.add_argument("--eliscript-ref", default="HEAD")
    parser.add_argument("--femacs-lisp", type=Path, default=REPO / "dist/lisp/bin/femacs-lisp")
    parser.add_argument("--runner", default="bun")
    parser.add_argument("--frozen-package", type=Path, default=None)
    parser.add_argument("--package-out", type=Path, default=None)
    parser.add_argument("--prepare-only", action="store_true")
    parser.add_argument("--self-test-guards", action="store_true")
    parser.add_argument("--out", type=Path, default=REPO / "build/acceptance/c5-a5-worker")
    args = parser.parse_args(argv)

    if args.self_test_guards:
        if args.frozen_package is None or args.prepare_only or args.package_out is not None:
            parser.error("--self-test-guards requires --frozen-package without --prepare-only/--package-out")
        guard_report = run_guard_self_tests(args.frozen_package.resolve())
        print(json.dumps(guard_report, indent=2))
        return 0

    app_bundle = args.app.resolve()
    terminal_bin = args.terminal.resolve()
    femacs_lisp = args.femacs_lisp.resolve()
    out_dir = args.out.resolve()

    if args.frozen_package is not None:
        if args.prepare_only or args.package_out is not None:
            parser.error("--frozen-package is a read-only input and cannot be combined with --prepare-only or --package-out")
        pkg_dir = args.frozen_package.resolve()
        manifest = verify_frozen_package(pkg_dir)
        if out_dir == pkg_dir or out_dir.is_relative_to(pkg_dir):
            raise ValueError(
                f"refusing to place --out inside read-only --frozen-package: out={out_dir} pkg={pkg_dir}"
            )
        if out_dir.exists():
            raise FileExistsError(f"refusing to overwrite existing output directory: {out_dir}")
    else:
        pkg_target = (args.package_out or (out_dir / "frozen-eliscript")).resolve()
        if pkg_target.exists() or pkg_target.is_symlink():
            raise FileExistsError(f"refusing to overwrite existing package output: {pkg_target}")
        if not args.prepare_only and out_dir.exists():
            raise FileExistsError(f"refusing to overwrite existing output directory: {out_dir}")
        manifest = prepare_frozen_package(
            args.eliscript_repo.resolve(), args.eliscript_ref, femacs_lisp, pkg_target
        )
        pkg_dir = pkg_target
        if args.prepare_only:
            print(
                f"Prepared frozen Eliscript package at {pkg_dir} "
                f"(commit={manifest['eliscript_commit']} closure_sha256={manifest['closure_sha256']} files={manifest['file_count']})"
            )
            return 0

    if out_dir.exists():
        if args.frozen_package is not None:
            raise FileExistsError(f"refusing to overwrite existing output directory: {out_dir}")
        # Check for existing report/artifacts if out_dir was created only as parent of pkg_dir.
        if (out_dir / "c5-a5-worker-report.json").exists():
            raise FileExistsError(f"refusing to overwrite existing report in {out_dir}")
    out_dir.mkdir(parents=True, exist_ok=True)

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
        "frozen_package_closure_sha256": manifest["closure_sha256"],
        "frozen_package_file_count": manifest["file_count"],
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
