#!/usr/bin/env python3
"""Independent stability verification for SD-A1, SD-A2, and SD-A4.

Separates pure helper unit tests (`--mode unit` / `--self-test` / `--unit-only`)
from real TTY process integration tests (`--mode tty`), or runs both (`--mode all`).
Writes per-case timing and retained stdout/stderr receipts to an exclusive output
directory (`--out`).
"""

from __future__ import annotations

import argparse
from datetime import datetime, timezone
import errno
import fcntl
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import pty
import signal
import subprocess
import sys
import tempfile
import termios
import time
from typing import Any
import unittest
from unittest import mock


ROOT = Path(__file__).resolve().parent.parent
TERMINAL_BINARY = ROOT / "build/terminal/port/src/emacs"
TTY_FAILURE_FIXTURE = ROOT / "tools/fixtures/tty-failure.el"
MACOS_IME_TOOL = ROOT / "tools/test-macos-ime.py"
SCHEMA = "femacs-goal-stability-v1"
MAX_CALLBACK_BYTES = 1_048_576


def now_iso() -> str:
    return datetime.now(timezone.utc).isoformat()


def default_dev_tty_path() -> str:
    return str(Path(os.sep) / "dev" / "tty")


def new_output(path: Path) -> Path:
    resolved = (path if path.is_absolute() else (ROOT / path)).resolve()
    reference = (ROOT / "emacs").resolve()
    if resolved == reference or reference in resolved.parents:
        raise ValueError("output must not be inside the read-only reference tree")
    resolved.mkdir(parents=True, exist_ok=False)
    return resolved


def artifact_identity(binary: Path) -> list[dict[str, str]]:
    binary = binary.resolve()
    files = [binary, *sorted(binary.parent.glob("*.pdmp"))]
    result = []
    for path in files:
        digest = hashlib.sha256()
        with path.open("rb") as stream:
            for block in iter(lambda: stream.read(1024 * 1024), b""):
                digest.update(block)
        result.append({"path": str(path), "sha256": digest.hexdigest()})
    return result


def load_macos_ime_module() -> Any:
    spec = importlib.util.spec_from_file_location("femacs_test_macos_ime", MACOS_IME_TOOL)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load module spec from {MACOS_IME_TOOL}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    if not hasattr(module, "native_window_ready"):
        raise RuntimeError("tools/test-macos-ime.py is missing required helper native_window_ready(records, title)")
    if not hasattr(module, "read_callbacks"):
        raise RuntimeError("tools/test-macos-ime.py is missing required helper read_callbacks(path)")
    return module


def make_valid_ready_record(
    *,
    title: str = "femacs-ime-test",
    window_number: Any = 101,
    key_window_number: Any = 101,
    window_is_key: Any = True,
    view_is_first_responder: Any = True,
    key_window: Any = True,
    app_active: Any = True,
    event: str = "windowDidBecomeKey",
    details: str = "ready",
    uptime: float = 12.5,
) -> dict[str, Any]:
    return {
        "uptime": uptime,
        "event": event,
        "details": details,
        "windowNumber": window_number,
        "windowTitle": title,
        "windowIsKey": window_is_key,
        "viewIsFirstResponder": view_is_first_responder,
        "keyWindowNumber": key_window_number,
        "keyWindow": key_window,
        "appActive": app_active,
    }


class GoalStabilityUnitTests(unittest.TestCase):
    """Pure unit tests for SD-A4 (`native_window_ready` and `read_callbacks`)."""

    @classmethod
    def setUpClass(cls) -> None:
        cls.ime = load_macos_ime_module()

    def test_output_guards_preserve_reference_and_symlink_targets(self) -> None:
        spec = importlib.util.spec_from_file_location("femacs_release_guard", ROOT / "tools/test-release-baseline.py")
        release = importlib.util.module_from_spec(spec)
        sys.modules[spec.name] = release
        spec.loader.exec_module(release)
        base = ROOT / "build/goal-output-guard-unit"
        base.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(dir=base) as directory:
            root = Path(directory)
            reference = root / "emacs"
            reference.mkdir()
            marker = reference / "marker"
            marker.write_bytes(b"unchanged reference")
            alias = root / "alias"
            alias.symlink_to(reference, target_is_directory=True)
            for module in (sys.modules[__name__], release):
                root_name = "ROOT" if module is sys.modules[__name__] else "REPO"
                with mock.patch.object(module, root_name, root):
                    for forbidden in (reference, reference / "new", alias / "new"):
                        with self.subTest(tool=module.__name__, path=forbidden):
                            with self.assertRaises(ValueError):
                                module.new_output(forbidden)
                            if module is release:
                                with self.assertRaises(ValueError):
                                    release.checked_output_path(forbidden / "extra.json")
                    owned = root / ("owned-" + module.__name__)
                    self.assertEqual(module.new_output(owned), owned)
                    with self.assertRaises(FileExistsError):
                        module.new_output(owned)
                    if module is release:
                        cli_output = root / "cli-output"
                        with self.assertRaises(ValueError):
                            release.main(["--self-test", "--out", str(cli_output),
                                          "--json-out", str(reference / "extra.json")])
                        self.assertFalse(cli_output.exists())
            self.assertEqual(marker.read_bytes(), b"unchanged reference")
            self.assertFalse((reference / "new").exists())

    def test_artifact_identity_detects_binary_and_dump_mutation(self) -> None:
        base = ROOT / "build/goal-identity-unit"
        base.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(dir=base) as directory:
            binary = Path(directory) / "editor"
            dump = binary.with_suffix(".pdmp")
            binary.write_bytes(b"executable fixture")
            dump.write_bytes(b"dump fixture")
            before = artifact_identity(binary)
            self.assertEqual(before, artifact_identity(binary))
            dump.write_bytes(b"changed dump")
            self.assertNotEqual(before, artifact_identity(binary))
            before = artifact_identity(binary)
            binary.write_bytes(b"changed executable")
            self.assertNotEqual(before, artifact_identity(binary))

    def test_sd_a4_exact_current_window_accepted(self) -> None:
        ready = self.ime.native_window_ready
        rec = make_valid_ready_record(title="femacs-ime-frame-a", window_number=42, key_window_number=42)
        self.assertTrue(ready([rec], "femacs-ime-frame-a"))

        # Earlier focus-loss or other-window rows followed by a valid latest row for the target title succeed.
        seq = [
            make_valid_ready_record(
                title="femacs-ime-frame-a",
                window_number=42,
                key_window_number=None,
                window_is_key=False,
                key_window=False,
                event="windowDidResignKey",
            ),
            make_valid_ready_record(
                title="femacs-ime-frame-b",
                window_number=99,
                key_window_number=99,
                event="windowDidBecomeKey",
            ),
        ]
        self.assertTrue(ready(seq, "femacs-ime-frame-b"))
        self.assertFalse(ready(seq, "femacs-ime-frame-a"))

    def test_sd_a4_absent_or_wrong_title_rejected(self) -> None:
        ready = self.ime.native_window_ready
        self.assertFalse(ready([], "femacs-ime-test"))
        self.assertFalse(ready([make_valid_ready_record(title="other-window")], "femacs-ime-test"))
        self.assertFalse(ready([make_valid_ready_record(title=None)], "femacs-ime-test"))  # type: ignore[arg-type]
        self.assertFalse(ready([make_valid_ready_record(title="")], ""))

        missing_title = make_valid_ready_record()
        missing_title.pop("windowTitle", None)
        self.assertFalse(ready([missing_title], "femacs-ime-test"))

    def test_sd_a4_boolean_null_or_nonpositive_ids_rejected(self) -> None:
        ready = self.ime.native_window_ready
        # In Python isinstance(True, int) is True; contract requires strict non-bool positive integers.
        for bad_id in (True, False, None, 0, -1, -99, "101", 101.0):
            with self.subTest(bad_window_number=bad_id):
                self.assertFalse(
                    ready(
                        [make_valid_ready_record(window_number=bad_id, key_window_number=101)],
                        "femacs-ime-test",
                    )
                )
            with self.subTest(bad_key_window_number=bad_id):
                self.assertFalse(
                    ready(
                        [make_valid_ready_record(window_number=101, key_window_number=bad_id)],
                        "femacs-ime-test",
                    )
                )
            with self.subTest(both_bad_id=bad_id):
                self.assertFalse(
                    ready(
                        [make_valid_ready_record(window_number=bad_id, key_window_number=bad_id)],
                        "femacs-ime-test",
                    )
                )

    def test_sd_a4_wrong_key_number_false_responder_or_inactive_app_rejected(self) -> None:
        ready = self.ime.native_window_ready
        self.assertFalse(
            ready(
                [make_valid_ready_record(window_number=101, key_window_number=102)],
                "femacs-ime-test",
            )
        )
        for field in ("windowIsKey", "viewIsFirstResponder", "keyWindow", "appActive"):
            for bad_bool in (False, None, 1, 0, "true"):
                with self.subTest(field=field, bad_bool=bad_bool):
                    rec = make_valid_ready_record()
                    rec[field] = bad_bool
                    self.assertFalse(ready([rec], "femacs-ime-test"))
            with self.subTest(field=field, missing=True):
                rec = make_valid_ready_record()
                rec.pop(field, None)
                self.assertFalse(ready([rec], "femacs-ime-test"))

    def test_sd_a4_latest_focus_loss_or_stale_other_window_rejected(self) -> None:
        ready = self.ime.native_window_ready
        first_ready = make_valid_ready_record(
            title="femacs-ime-frame-1",
            window_number=201,
            key_window_number=201,
            event="becomeFirstResponder",
        )
        resign_key = make_valid_ready_record(
            title="femacs-ime-frame-1",
            window_number=201,
            key_window_number=None,
            window_is_key=False,
            key_window=False,
            event="windowDidResignKey",
        )
        resign_responder = make_valid_ready_record(
            title="femacs-ime-frame-1",
            window_number=201,
            key_window_number=201,
            view_is_first_responder=False,
            event="resignFirstResponder",
        )
        inactive_app = make_valid_ready_record(
            title="femacs-ime-frame-1",
            window_number=201,
            key_window_number=201,
            app_active=False,
            event="keyDown",
        )
        second_window_event = make_valid_ready_record(
            title="femacs-ime-frame-2",
            window_number=202,
            key_window_number=202,
            event="windowDidBecomeKey",
        )
        global_app_loss = make_valid_ready_record(
            title=None,  # type: ignore[arg-type]
            window_number=None,
            key_window_number=None,
            window_is_key=False,
            view_is_first_responder=False,
            key_window=False,
            app_active=False,
            event="applicationDidResignActive",
        )
        self.assertFalse(ready([first_ready, resign_key], "femacs-ime-frame-1"))
        self.assertFalse(ready([first_ready, resign_responder], "femacs-ime-frame-1"))
        self.assertFalse(ready([first_ready, inactive_app], "femacs-ime-frame-1"))
        self.assertFalse(ready([first_ready, second_window_event], "femacs-ime-frame-1"))
        self.assertFalse(ready([first_ready, global_app_loss], "femacs-ime-frame-1"))

    def test_sd_a4_malformed_rows_never_ready(self) -> None:
        ready = self.ime.native_window_ready
        self.assertFalse(ready(["not-a-dict"], "femacs-ime-test"))  # type: ignore[list-item]
        self.assertFalse(ready([None], "femacs-ime-test"))  # type: ignore[list-item]
        self.assertFalse(
            ready(["not-a-dict", make_valid_ready_record(title="femacs-ime-test")], "femacs-ime-test")  # type: ignore[list-item]
        )
        self.assertFalse(
            ready([make_valid_ready_record(title="femacs-ime-test"), "not-a-dict"], "femacs-ime-test")  # type: ignore[list-item]
        )
        self.assertFalse(ready([make_valid_ready_record(), {}], "femacs-ime-test"))

    def test_sd_a4_read_callbacks_incomplete_final_append_tolerated(self) -> None:
        read_callbacks = self.ime.read_callbacks
        with tempfile.TemporaryDirectory(prefix="femacs-sd-a4-callbacks-") as tmp:
            missing_cb = Path(tmp) / "nonexistent-callbacks.jsonl"
            self.assertEqual(read_callbacks(missing_cb), [])

            cb_path = Path(tmp) / "callbacks.jsonl"
            rec1 = make_valid_ready_record(title="femacs-ime-1", window_number=11, key_window_number=11)
            rec2 = make_valid_ready_record(title="femacs-ime-2", window_number=22, key_window_number=22)
            partial = '{"uptime":30.0,"event":"keyDown","windowTitle":"femacs-ime-2"'
            cb_path.write_text(
                json.dumps(rec1) + "\n" + json.dumps(rec2) + "\n" + partial,
                encoding="utf-8",
            )
            rows = read_callbacks(cb_path)
            self.assertEqual(len(rows), 2)
            self.assertTrue(self.ime.native_window_ready(rows, "femacs-ime-2"))
            self.assertFalse(self.ime.native_window_ready(rows, "femacs-ime-1"))

            # Once the trailing line is completed with a newline, all 3 rows are parsed.
            rec3 = make_valid_ready_record(title="femacs-ime-3", window_number=33, key_window_number=33)
            cb_path.write_text(
                json.dumps(rec1) + "\n" + json.dumps(rec2) + "\n" + json.dumps(rec3) + "\n",
                encoding="utf-8",
            )
            rows_complete = read_callbacks(cb_path)
            self.assertEqual(len(rows_complete), 3)
            self.assertTrue(self.ime.native_window_ready(rows_complete, "femacs-ime-3"))

    def test_sd_a4_read_callbacks_completed_malformed_json_and_size_cap_fail(self) -> None:
        read_callbacks = self.ime.read_callbacks
        with tempfile.TemporaryDirectory(prefix="femacs-sd-a4-bad-cb-") as tmp:
            bad_line = Path(tmp) / "malformed.jsonl"
            bad_line.write_text(
                json.dumps(make_valid_ready_record()) + "\n{broken-json-line\n",
                encoding="utf-8",
            )
            with self.assertRaises(ValueError):
                read_callbacks(bad_line)

            non_object = Path(tmp) / "non-object.jsonl"
            non_object.write_text("[1, 2, 3]\n", encoding="utf-8")
            with self.assertRaises(ValueError):
                read_callbacks(non_object)

            oversized = Path(tmp) / "oversized.jsonl"
            line = json.dumps(make_valid_ready_record(details="x" * 512)) + "\n"
            repeats = (MAX_CALLBACK_BYTES // len(line.encode("utf-8"))) + 4
            oversized.write_text(line * repeats, encoding="utf-8")
            self.assertGreater(oversized.stat().st_size, MAX_CALLBACK_BYTES)
            with self.assertRaises(ValueError):
                read_callbacks(oversized)


def run_unit_suite(out_dir: Path) -> tuple[bool, list[dict[str, Any]]]:
    loader = unittest.TestLoader()
    loaded = loader.loadTestsFromTestCase(GoalStabilityUnitTests)
    cases: list[dict[str, Any]] = []
    all_passed = True

    for test in loaded:
        case_id = test.id().split(".")[-1]
        single_suite = unittest.TestSuite([test])
        result = unittest.TestResult()
        t0 = time.monotonic()
        single_suite.run(result)
        elapsed_ms = round((time.monotonic() - t0) * 1000.0, 3)
        failures = [msg for _tc, msg in result.failures + result.errors]
        passed = result.wasSuccessful()
        if not passed:
            all_passed = False
        log_path = out_dir / f"{case_id}.log"
        log_path.write_text("\n".join(failures) if failures else "PASS\n", encoding="utf-8")
        cases.append(
            {
                "id": case_id,
                "category": "unit",
                "requirement": "SD-A4",
                "passed": passed,
                "elapsed_ms": elapsed_ms,
                "log": str(log_path.relative_to(ROOT)) if log_path.is_relative_to(ROOT) else str(log_path),
                "error": failures[0] if failures else None,
            }
        )
    return all_passed, cases


def terminate_process_group(proc: subprocess.Popen[bytes]) -> None:
    if proc.poll() is not None:
        return
    try:
        os.killpg(proc.pid, signal.SIGTERM)
    except OSError:
        try:
            proc.terminate()
        except OSError:
            pass
    try:
        proc.wait(timeout=2.0)
        return
    except subprocess.TimeoutExpired:
        pass
    try:
        os.killpg(proc.pid, signal.SIGKILL)
    except OSError:
        try:
            proc.kill()
        except OSError:
            pass
    try:
        proc.wait(timeout=2.0)
    except subprocess.TimeoutExpired:
        pass


def run_bounded_process(
    argv: list[str],
    *,
    env: dict[str, str],
    cwd: Path,
    stdin_fd: int | None = None,
    preexec_fn: Any = None,
    timeout_s: float = 15.0,
) -> tuple[int, str, str, float, bool]:
    t0 = time.monotonic()
    proc = subprocess.Popen(
        argv,
        cwd=str(cwd),
        env=env,
        stdin=stdin_fd if stdin_fd is not None else subprocess.DEVNULL,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        start_new_session=(preexec_fn is None),
        preexec_fn=preexec_fn,
    )
    timed_out = False
    try:
        stdout_b, stderr_b = proc.communicate(timeout=timeout_s)
    except subprocess.TimeoutExpired:
        timed_out = True
        terminate_process_group(proc)
        stdout_b, stderr_b = proc.communicate()
    elapsed_ms = round((time.monotonic() - t0) * 1000.0, 3)
    rc = proc.returncode if proc.returncode is not None else -1
    stdout_text = stdout_b.decode("utf-8", errors="replace")
    stderr_text = stderr_b.decode("utf-8", errors="replace")
    return rc, stdout_text, stderr_text, elapsed_ms, timed_out


def write_case_logs(out_dir: Path, case_id: str, stdout_text: str, stderr_text: str) -> tuple[str, str]:
    stdout_path = out_dir / f"{case_id}.stdout.log"
    stderr_path = out_dir / f"{case_id}.stderr.log"
    stdout_path.write_text(stdout_text, encoding="utf-8")
    stderr_path.write_text(stderr_text, encoding="utf-8")
    out_rel = str(stdout_path.relative_to(ROOT)) if stdout_path.is_relative_to(ROOT) else str(stdout_path)
    err_rel = str(stderr_path.relative_to(ROOT)) if stderr_path.is_relative_to(ROOT) else str(stderr_path)
    return out_rel, err_rel


def run_tty_integration(out_dir: Path, binary: Path, timeout_s: float) -> tuple[bool, list[dict[str, Any]]]:
    if not binary.is_file() or not os.access(binary, os.X_OK):
        raise RuntimeError(f"terminal binary is missing or not executable: {binary}")
    if not TTY_FAILURE_FIXTURE.is_file():
        raise RuntimeError(f"TTY failure Lisp fixture is missing: {TTY_FAILURE_FIXTURE}")

    work_dir = out_dir / "tty-work"
    work_dir.mkdir(parents=True, exist_ok=False)
    isolated_home = work_dir / "home"
    isolated_home.mkdir(parents=True, exist_ok=False)

    base_env = os.environ.copy()
    base_env["HOME"] = str(isolated_home)
    base_env["LC_ALL"] = "C"
    base_env["TERM"] = "vt100"

    missing_tty = work_dir / "owned-cli-missing-tty"
    regular_tty = work_dir / "owned-cli-regular-file.txt"
    regular_tty.write_text("regular file, not a tty\n", encoding="utf-8")
    directory_tty = work_dir / "owned-cli-directory"
    directory_tty.mkdir(parents=True, exist_ok=False)

    cases: list[dict[str, Any]] = []
    all_passed = True

    def record_case(
        case_id: str,
        requirement: str,
        passed: bool,
        elapsed_ms: float,
        exit_code: int,
        timed_out: bool,
        stdout_text: str,
        stderr_text: str,
        detail: str,
        extra: dict[str, Any] | None = None,
    ) -> None:
        nonlocal all_passed
        if not passed:
            all_passed = False
        stdout_rel, stderr_rel = write_case_logs(out_dir, case_id, stdout_text, stderr_text)
        row: dict[str, Any] = {
            "id": case_id,
            "category": "tty_integration",
            "requirement": requirement,
            "passed": passed,
            "elapsed_ms": elapsed_ms,
            "exit_code": exit_code,
            "timed_out": timed_out,
            "stdout_log": stdout_rel,
            "stderr_log": stderr_rel,
            "detail": detail,
        }
        if extra:
            row.update(extra)
        cases.append(row)

    # 1. SD-A1: `-Q -nw -t <missing>` exits 1 with formatted path + strerror + trailing newline.
    expected_enoent = os.strerror(errno.ENOENT)
    rc, out_s, err_s, ms, to = run_bounded_process(
        [str(binary), "-Q", "-nw", "-t", str(missing_tty)],
        env=base_env,
        cwd=isolated_home,
        timeout_s=timeout_s,
    )
    ok_missing = (
        not to
        and rc == 1
        and not out_s
        and err_s == f"{binary}: {missing_tty}: {expected_enoent}\n"
    )
    record_case(
        "sd_a1_fatal_minus_t_nonexistent",
        "SD-A1",
        ok_missing,
        ms,
        rc,
        to,
        out_s,
        err_s,
        f"expected rc=1 and {missing_tty}: {expected_enoent}",
    )

    # 2. SD-A1: `-Q -nw -t <regular-file>` exits 1 with formatted path + `not a tty\n`.
    rc, out_s, err_s, ms, to = run_bounded_process(
        [str(binary), "-Q", "-nw", "-t", str(regular_tty)],
        env=base_env,
        cwd=isolated_home,
        timeout_s=timeout_s,
    )
    ok_regular = (
        not to
        and rc == 1
        and not out_s
        and err_s == f"{binary}: {regular_tty}: not a tty\n"
    )
    record_case(
        "sd_a1_fatal_minus_t_regular_file",
        "SD-A1",
        ok_regular,
        ms,
        rc,
        to,
        out_s,
        err_s,
        f"expected rc=1 and {regular_tty}: not a tty",
    )

    # 3. SD-A1: `-Q -nw -t <directory>` exits 1 with formatted path + strerror(EISDIR) + trailing newline.
    expected_eisdir = os.strerror(errno.EISDIR)
    rc, out_s, err_s, ms, to = run_bounded_process(
        [str(binary), "-Q", "-nw", "-t", str(directory_tty)],
        env=base_env,
        cwd=isolated_home,
        timeout_s=timeout_s,
    )
    ok_dir = (
        not to
        and rc == 1
        and not out_s
        and err_s == f"{binary}: {directory_tty}: {expected_eisdir}\n"
    )
    record_case(
        "sd_a1_fatal_minus_t_directory",
        "SD-A1",
        ok_dir,
        ms,
        rc,
        to,
        out_s,
        err_s,
        f"expected rc=1 and {directory_tty}: {expected_eisdir}",
    )

    # 4. SD-A1: Detached `-Q -nw` with PTY stdin (`isatty(0)==1`) but no controlling TTY (`setsid` without
    # `TIOCSCTTY`) hits early `init_tty` `(*tty).input.is_null()` -> `maybe_fatal` -> `vfatal(str_, args)`
    # and must format `%s` (`Could not open file: <dev_tty>\n`) with status 1 and a single newline.
    master_fd, slave_fd = pty.openpty()
    try:
        rc, out_s, err_s, ms, to = run_bounded_process(
            [str(binary), "-Q", "-nw"],
            env=base_env,
            cwd=isolated_home,
            stdin_fd=slave_fd,
            timeout_s=timeout_s,
        )
    finally:
        os.close(slave_fd)
        os.close(master_fd)

    dev_tty_str = default_dev_tty_path()
    expected_detached_line = f"emacs: Could not open file: {dev_tty_str}\n"
    ok_detached = (
        not to
        and rc == 1
        and err_s == expected_detached_line
    )
    record_case(
        "sd_a1_fatal_detached_nw_no_controlling_tty",
        "SD-A1",
        ok_detached,
        ms,
        rc,
        to,
        out_s,
        err_s,
        f"expected exact vfatal output {expected_detached_line!r}",
    )

    # 5. SD-A1 + SD2: `-Q -nw` on an owned controlling PTY (`setsid` + `TIOCSCTTY`) with an undefined
    # `TERM` value exercises the post-`name` fatal path (`maybe_fatal` -> `delete_tty` using `unlink_tty`
    # -> `vfatal` formatting `%s` for the terminal type) and exits 1 without crash or hang.
    undef_term = "femacs-undefined-term-sd-a1"
    env_undef = dict(base_env)
    env_undef["TERM"] = undef_term
    c_master_fd, c_slave_fd = pty.openpty()

    def attach_controlling_tty() -> None:
        os.setsid()
        fcntl.ioctl(c_slave_fd, termios.TIOCSCTTY, 0)

    try:
        rc, out_s, err_s, ms, to = run_bounded_process(
            [str(binary), "-Q", "-nw"],
            env=env_undef,
            cwd=isolated_home,
            stdin_fd=c_slave_fd,
            preexec_fn=attach_controlling_tty,
            timeout_s=timeout_s,
        )
    finally:
        os.close(c_slave_fd)
        os.close(c_master_fd)

    expected_term_prefix = (
        f"emacs: Terminal type {undef_term} is not defined.\n"
        "If that is not the actual type of terminal you have,\n"
        "use the Bourne shell command 'TERM=...; export TERM' (C-shell:\n"
        "'setenv TERM ...') to specify the correct type.  It may be necessary\n"
        "to do 'unset TERMINFO' (C-shell: 'unsetenv TERMINFO') as well.\n"
    )
    ok_undef_term = (
        not to
        and rc == 1
        and not out_s
        and err_s == expected_term_prefix
    )
    record_case(
        "sd_a1_fatal_named_tty_undefined_term_type",
        "SD-A1",
        ok_undef_term,
        ms,
        rc,
        to,
        out_s,
        err_s,
        f"expected rc=1 and vfatal prefix {expected_term_prefix!r}",
    )

    # 6. SD-A2: Recoverable `make-terminal-frame` failures in a live editor process via
    # `tools/fixtures/tty-failure.el` (including both early open/isatty failure branches and the
    # post-open undefined-`tty-type` `delete_tty` branch on an owned PTY slave).
    fixture_work = work_dir / "recoverable-fixture"
    fixture_report = out_dir / "tty-failure-report.json"
    env_fixture = dict(base_env)
    env_fixture["FEMACS_TTY_FAIL_DIR"] = str(fixture_work)
    env_fixture["FEMACS_TTY_FAIL_REPORT"] = str(fixture_report)

    f_master_fd, f_slave_fd = pty.openpty()
    try:
        slave_name = os.ttyname(f_slave_fd)
        env_fixture["FEMACS_TTY_FAIL_PTY"] = slave_name
        rc, out_s, err_s, ms, to = run_bounded_process(
            [str(binary), "-Q", "--batch", "-l", str(TTY_FAILURE_FIXTURE)],
            env=env_fixture,
            cwd=isolated_home,
            timeout_s=timeout_s,
        )
    finally:
        os.close(f_slave_fd)
        os.close(f_master_fd)

    report_payload: dict[str, Any] | None = None
    report_ok = False
    report_detail = "missing tty-failure-report.json"
    if not to and rc == 0 and fixture_report.is_file():
        try:
            report_payload = json.loads(fixture_report.read_text(encoding="utf-8"))
        except json.JSONDecodeError as exc:
            report_detail = f"invalid JSON in tty-failure-report.json: {exc}"
        else:
            saved_file = Path(str(report_payload.get("saved_file", "")))
            expected_survivor = "femacs-tty-failure-survivor-你好\nsecond-line-ok\n"
            survivor_ok = (
                saved_file.is_file()
                and saved_file.read_text(encoding="utf-8") == expected_survivor
            )
            records = report_payload.get("records")
            kinds = [r.get("kind") for r in records] if isinstance(records, list) else []
            report_ok = (
                report_payload.get("schema") == "femacs-tty-failure-v1"
                and report_payload.get("passed") is True
                and report_payload.get("failure_count") == 10
                and kinds.count("missing") == 3
                and kinds.count("regular") == 3
                and kinds.count("directory") == 3
                and kinds.count("undefined-term-type") == 1
                and report_payload.get("terminal_count_before") == report_payload.get("terminal_count_after")
                and report_payload.get("terminal_names_before") == report_payload.get("terminal_names_after")
                and report_payload.get("frame_count_before") == report_payload.get("frame_count_after")
                and report_payload.get("all_terminals_live") is True
                and report_payload.get("gc_completed") is True
                and survivor_ok
            )
            report_detail = (
                "verified 10 recoverable make-terminal-frame failures, invariant terminal-list/frame-list, GC, and UTF-8 buffer save"
                if report_ok
                else f"report validation failed: {report_payload}"
            )

    record_case(
        "sd_a2_recoverable_make_terminal_frame_failures",
        "SD-A2",
        report_ok,
        ms,
        rc,
        to,
        out_s,
        err_s,
        report_detail,
        extra={"fixture_report": report_payload},
    )

    return all_passed, cases


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Run SD-A1, SD-A2, and SD-A4 stability verification."
    )
    parser.add_argument(
        "--out",
        type=Path,
        required=True,
        help="Exclusive output directory for test logs and results.json.",
    )
    parser.add_argument(
        "--mode",
        choices=("all", "unit", "tty"),
        default="all",
        help="Select pure unit suite (`unit`), real TTY integration (`tty`), or `all`.",
    )
    parser.add_argument(
        "--self-test",
        "--unit-only",
        dest="unit_only",
        action="store_true",
        help="Alias for `--mode unit` (run pure SD-A4 unit tests only).",
    )
    parser.add_argument(
        "--binary",
        type=Path,
        default=TERMINAL_BINARY,
        help="Path to the terminal editor executable for `--mode tty` / `--mode all`.",
    )
    parser.add_argument(
        "--timeout",
        type=float,
        default=15.0,
        help="Per-subprocess timeout in seconds for TTY integration cases.",
    )
    return parser.parse_args(argv)


def main(argv: list[str]) -> int:
    args = parse_args(argv)
    mode = "unit" if args.unit_only else args.mode
    out_dir = new_output(args.out)
    started_at = now_iso()
    t0 = time.monotonic()

    all_cases: list[dict[str, Any]] = []
    overall_passed = True
    products_before = artifact_identity(args.binary) if mode in ("all", "tty") else []

    if mode in ("all", "unit"):
        unit_ok, unit_cases = run_unit_suite(out_dir)
        overall_passed = overall_passed and unit_ok
        all_cases.extend(unit_cases)

    if mode in ("all", "tty"):
        tty_ok, tty_cases = run_tty_integration(out_dir, args.binary.resolve(), args.timeout)
        overall_passed = overall_passed and tty_ok
        all_cases.extend(tty_cases)

    total_elapsed_ms = round((time.monotonic() - t0) * 1000.0, 3)
    products_after = artifact_identity(args.binary) if products_before else []
    overall_passed = overall_passed and products_before == products_after
    summary = {
        "schema": SCHEMA,
        "mode": mode,
        "started_at": started_at,
        "completed_at": now_iso(),
        "elapsed_ms": total_elapsed_ms,
        "passed": overall_passed,
        "products_before": products_before,
        "products_after": products_after,
        "products_unchanged": products_before == products_after,
        "total_cases": len(all_cases),
        "passed_cases": sum(1 for c in all_cases if c.get("passed") is True),
        "failed_cases": sum(1 for c in all_cases if c.get("passed") is not True),
        "ownership_note": (
            "SD-A2 process-level terminal-list invariance and post-GC buffer survival verify recoverable error handling "
            "and absence of dangling terminal-list entries; C heap deallocation of tty_display_info/Wcm is verified by "
            "source review of src/term.rs."
        ),
        "cases": all_cases,
    }
    results_path = out_dir / "results.json"
    results_path.write_text(json.dumps(summary, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    return 0 if overall_passed else 1


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
