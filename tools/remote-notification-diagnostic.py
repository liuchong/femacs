#!/usr/bin/env python3
"""Bounded error-only Base64 and Tramp state diagnostic driver for remote09.

Materializes only ``test/lisp/filenotify-tests.el`` from the pinned stable 31.1
manifest, executes ``file-notify-test09-watched-file-in-watched-dir-remote``
under ``tools/fixtures/file-notify-remote-diagnostic.el`` in isolated per-run
case environments and preserves payload/process/backtrace records on failure.
Case state stays available until a separate identity-checked cleanup consumes
its final receipts; this driver never performs an unrecorded tree deletion.

Run the whole command under controlled-run (--profile runtime); do not nest
another controller inside this script.
"""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path, PurePosixPath
import math
import sys
import time

REPO = Path(__file__).resolve().parent.parent
MANIFEST_REL = "compatibility/stable-31.1/sources.json"
LOGICAL_FILE = "test/lisp/filenotify-tests.el"
DEFAULT_TEST_ID = "file-notify-test09-watched-file-in-watched-dir-remote"
DIAG_FIXTURE_REL = "tools/fixtures/file-notify-remote-diagnostic.el"
BASE_RUNNER_REL = "tools/fixtures/stable-suite-runner.el"
RESULT_SCHEMA = "femacs-remote09-diagnostic-run-v1"


def _load_stable_suite():
    source = REPO / "tools/stable-suite.py"
    spec = importlib.util.spec_from_file_location("femacs_stable_suite", source)
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def _write_json(path: Path, payload: dict) -> None:
    temp = path.with_name(path.name + ".writing")
    with temp.open("x", encoding="utf-8") as stream:
        json.dump(payload, stream, indent=2, ensure_ascii=False, allow_nan=False)
        stream.write("\n")
    os.replace(temp, path)


def _materialize_single_file(suite, manifest: dict, out_root: Path) -> dict:
    matches = [
        entry for entry in manifest["entries"] if entry.get("path") == LOGICAL_FILE
    ]
    if len(matches) != 1:
        raise suite.SuiteError(
            "expected exactly one manifest entry for {0}".format(LOGICAL_FILE)
        )
    entry = matches[0]
    source = suite._source_path(REPO, entry["source_path"])
    suite._verify_source(source, entry)
    target = out_root.joinpath(*PurePosixPath(entry["path"]).parts)
    suite._copy_source(source, target, entry)
    return {
        "logical_path": LOGICAL_FILE,
        "source_path": entry["source_path"],
        "sha256": entry["sha256"],
        "blob": entry["blob"],
        "bytes": entry["size"],
    }


def _load_runtime_execution():
    source = REPO / "tools/runtime-execution.py"
    spec = importlib.util.spec_from_file_location("femacs_remote09_execution", source)
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def _run_iteration(
    suite,
    out_root: Path,
    program: Path,
    dump: Path | None,
    lisp_root: Path,
    fixture: Path,
    test_root: Path,
    test_id: str,
    iteration: int,
    timeout: float,
    label: str,
    *,
    logical_file: str = LOGICAL_FILE,
    file_path: Path | None = None,
    extra_env: dict[str, str] | None = None,
    execution=None,
) -> dict:
    iter_name = "iter-{0:03d}".format(iteration)
    env_parent = out_root / "environments" / label / iter_name
    case_label = "{0}|{1}|{2}|{3}".format(
        logical_file, test_id, iteration, time.time_ns()
    )
    environment = suite.case_environment(case_label, env_parent)
    case_root = Path(environment["FEMACS_RUNTIME_CASE_ROOT"])
    cwd = case_root / "cwd"
    cwd.mkdir(parents=True, exist_ok=True)

    result_dir = out_root / "results" / label
    result_dir.mkdir(parents=True, exist_ok=True)

    result_path = result_dir / (iter_name + ".result.json")
    diag_path = result_dir / (iter_name + ".base64-diag.json")

    if file_path is None:
        file_path = out_root.joinpath(*PurePosixPath(logical_file).parts)
    dump_args = ["--dump-file=" + str(dump)] if dump is not None else []
    argv = (
        [str(program)]
        + dump_args
        + [
            "--batch",
            "-Q",
            "-L",
            str(test_root),
            "-l",
            str(fixture),
            "-f",
            "stable-suite-main",
        ]
    )
    child_env = dict(environment)
    child_env.update(
        {
            "PATH": os.environ.get("PATH", os.defpath),
            "LC_ALL": "C",
            "LANG": "C",
            "TERM": "dumb",
            "PWD": str(cwd),
            "EMACS_TEST_DIRECTORY": str(test_root),
            "FEMACS_RUNTIME_TEST_ROOT": str(case_root),
            "FEMACS_REMOTE09_DIAG_OUT": str(diag_path),
            "STABLE_SUITE_MODE": "run",
            "STABLE_SUITE_FILE": str(file_path),
            "STABLE_SUITE_FILE_ID": logical_file,
            "STABLE_SUITE_TEST": test_id,
            "STABLE_SUITE_RESULT": str(result_path),
        }
    )
    if extra_env:
        child_env.update(extra_env)

    if execution is None:
        execution = _load_runtime_execution()
    process_root = out_root / "processes" / label / iter_name
    process_record = execution.run_process(
        argv, process_root, child_env, timeout, cwd=cwd
    )
    elapsed = process_record["seconds"]
    timed_out = process_record["timed_out"]
    exit_code = process_record["exit_code"]
    spawn_error = process_record.get("error")
    remaining_group_terminated = process_record.get("remaining_group_terminated", False)
    stdout_path = process_root / "stdout.txt"
    stderr_path = process_root / "stderr.txt"

    payload = None
    json_error = None
    if result_path.is_file():
        try:
            payload = suite._read_machine_result(result_path)
        except suite.SuiteError as exc:
            json_error = str(exc)

    diag_payload = None
    if diag_path.is_file():
        try:
            diag_payload = suite._read_machine_result(diag_path)
        except suite.SuiteError as exc:
            json_error = json_error or str(exc)

    record = payload.get("record") if isinstance(payload, dict) else None
    if not isinstance(record, dict):
        record = None

    if spawn_error is not None:
        status = "environment-error"
    elif timed_out:
        status = "timeout"
    elif payload is None:
        status = "result-error"
    else:
        status = payload.get("status")
        if not isinstance(status, str) or not status:
            status = "result-error"
        if exit_code != 0:
            status = "process-exit"

    complete = (
        not timed_out
        and exit_code == 0
        and not remaining_group_terminated
        and isinstance(payload, dict)
        and status
        not in ("timeout", "process-exit", "result-error", "environment-error")
    )
    observed_for_assess = {
        "test_id": record.get("test_id") if record is not None else test_id,
        "status": record.get("status") if record is not None else status,
        "complete": complete,
        "exit_code": exit_code,
        "seconds": record.get("duration") if record is not None else elapsed,
        "diagnostic": (
            (record.get("diagnostic") or record.get("condition") or "")
            if record is not None
            else (json_error or spawn_error or "")
        ),
    }
    assessment = suite.assess_case(test_id, observed_for_assess)
    captures = (
        record.get("base64_error_captures", []) if isinstance(record, dict) else []
    )
    passed = (
        assessment["passed"] is True
        and not captures
        and diag_payload is None
        and not remaining_group_terminated
    )

    retirement = {"status": "retained", "reason": "await final receipt and identity-checked cleanup"}

    return {
        "iteration": iteration,
        "product": label,
        "test_id": test_id,
        "passed": passed,
        "status": observed_for_assess["status"],
        "complete": complete,
        "exit_code": exit_code,
        "timed_out": timed_out,
        "remaining_group_terminated": remaining_group_terminated,
        "seconds": elapsed,
        "assessment": assessment,
        "base64_error_capture_count": len(captures) if isinstance(captures, list) else 0,
        "base64_error_captures": captures,
        "diagnostic_file": (
            suite._relative_to(out_root, diag_path) if diag_path.is_file() else None
        ),
        "diagnostic_payload": diag_payload,
        "result_file": (
            suite._relative_to(out_root, result_path) if result_path.is_file() else None
        ),
        "stdout_log": suite._relative_to(out_root, stdout_path),
        "stderr_log": suite._relative_to(out_root, stderr_path),
        "environment_retirement": retirement,
        "environment_root": str(env_parent),
        "process": process_record,
        "error": json_error or spawn_error,
    }


def _check_post_exit_retention(out_root: Path, row: dict, expect_diag_file: bool) -> dict:
    env_root = Path(row["environment_root"])
    proc_dir = Path(row["process"]["evidence_directory"])
    res_file = out_root / row["result_file"] if row.get("result_file") else None
    diag_file = out_root / row["diagnostic_file"] if row.get("diagnostic_file") else None
    stdout_file = out_root / row["stdout_log"]
    stderr_file = out_root / row["stderr_log"]
    proc_json = proc_dir / "process.json"
    case_dirs = sorted(p for p in env_root.glob("case-*") if p.is_dir()) if env_root.is_dir() else []
    diag_ok = (diag_file is not None and diag_file.is_file()) if expect_diag_file else (diag_file is None)
    passed = (
        env_root.is_dir()
        and len(case_dirs) == 1
        and (case_dirs[0] / "cwd").is_dir()
        and proc_dir.is_dir()
        and proc_json.is_file()
        and stdout_file.is_file()
        and stderr_file.is_file()
        and res_file is not None
        and res_file.is_file()
        and diag_ok
        and row.get("environment_retirement", {}).get("status") == "retained"
    )
    return {
        "passed": passed,
        "environment_dir_exists": env_root.is_dir(),
        "case_dir_count": len(case_dirs),
        "process_json_exists": proc_json.is_file(),
        "stdout_exists": stdout_file.is_file(),
        "stderr_exists": stderr_file.is_file(),
        "result_file_exists": bool(res_file and res_file.is_file()),
        "diagnostic_file_matches_expectation": diag_ok,
        "retirement_status": row.get("environment_retirement", {}).get("status"),
    }


def _run_verify_capture(
    suite,
    execution,
    out_root: Path,
    paths: dict,
    fixture: Path,
    timeout: float,
    label: str,
) -> dict:
    test_root = out_root / "verify-test-root"
    test_root.mkdir(parents=True, exist_ok=True)
    case_ids = [
        "femacs-remote09-diag-self-test-region-byte255-failure",
        "femacs-remote09-diag-self-test-string-and-multibyte-failure",
        "femacs-remote09-diag-self-test-success-path",
    ]
    rows = []
    for idx, case_id in enumerate(case_ids, start=1):
        row = _run_iteration(
            suite=suite,
            out_root=out_root,
            program=paths["program"],
            dump=paths["dump"],
            lisp_root=paths["lisp_root"],
            fixture=fixture,
            test_root=test_root,
            test_id=case_id,
            iteration=idx,
            timeout=timeout,
            label=label + "-verify",
            logical_file=DIAG_FIXTURE_REL,
            file_path=fixture,
            extra_env={"FEMACS_REMOTE09_DIAG_SELF_TEST": "1"},
            execution=execution,
        )
        rows.append(row)

    r1, r2, r3 = rows
    ret1 = _check_post_exit_retention(out_root, r1, expect_diag_file=True)
    ret2 = _check_post_exit_retention(out_root, r2, expect_diag_file=True)
    ret3 = _check_post_exit_retention(out_root, r3, expect_diag_file=False)

    cap1 = r1["base64_error_captures"][0] if r1["base64_error_capture_count"] == 1 else {}
    pay1 = cap1.get("payload") or {}
    inv1 = pay1.get("first_invalid_byte") or {}
    procs1 = cap1.get("processes") if isinstance(cap1.get("processes"), list) else []
    dummy_procs = [p for p in procs1 if isinstance(p, dict) and p.get("name") == "femacs-remote09-diag-dummy"]
    dummy_proc = dummy_procs[0] if dummy_procs else {}
    dummy_rest_inv = ((dummy_proc.get("tramp_rest_string") or {}).get("first_invalid_byte") or {})
    dummy_buf_inv = ((dummy_proc.get("buffer_summary") or {}).get("first_invalid_byte") or {})

    case1_ok = (
        r1["passed"] is False
        and r1["status"] == "failed"
        and r1["complete"] is True
        and r1["exit_code"] == 0
        and r1["remaining_group_terminated"] is False
        and '(error "Invalid base64 data")' in (r1["assessment"].get("diagnostic") or "")
        and r1["base64_error_capture_count"] == 1
        and cap1.get("operation") == "base64-decode-region"
        and cap1.get("error_symbol") == "error"
        and cap1.get("error_message") == "Invalid base64 data"
        and cap1.get("base64url") is False
        and cap1.get("ignore_invalid") is False
        and cap1.get("buffer_name") == " *tramp/mock-diag-region-test*"
        and cap1.get("multibyte") is False
        and cap1.get("buffer_modified") is False
        and cap1.get("point") == 11
        and cap1.get("point_min") == 8
        and cap1.get("point_max") == 17
        and cap1.get("region_start") == 8
        and cap1.get("region_end") == 17
        and pay1.get("exact_input_bytes") is True
        and pay1.get("representation") == "unibyte-input"
        and pay1.get("alphabet_checked") == "standard-base64"
        and pay1.get("byte_length") == 9
        and pay1.get("raw_hex") == "54574675ff59574a6a"
        and inv1.get("offset") == 4
        and inv1.get("byte") == 255
        and dummy_proc.get("command") == ["sh", "-c", "cat # 100%"]
        and dummy_rest_inv.get("byte") == 255
        and dummy_buf_inv.get("byte") == 255
        and isinstance(cap1.get("backtrace"), list)
        and len(cap1.get("backtrace")) > 0
        and isinstance(r1.get("diagnostic_payload"), dict)
        and r1["diagnostic_payload"].get("capture_count") == 1
        and ret1["passed"] is True
    )

    caps2 = r2["base64_error_captures"] if r2["base64_error_capture_count"] == 2 else [{}, {}]
    c2_uni, c2_multi = caps2[0], caps2[1]
    p2_uni = c2_uni.get("payload") or {}
    p2_multi = c2_multi.get("payload") or {}
    case2_ok = (
        r2["passed"] is False
        and r2["status"] == "failed"
        and r2["complete"] is True
        and r2["exit_code"] == 0
        and r2["remaining_group_terminated"] is False
        and r2["base64_error_capture_count"] == 2
        and c2_uni.get("operation") == "base64-decode-string"
        and c2_uni.get("base64url") is True
        and c2_uni.get("multibyte") is False
        and p2_uni.get("exact_input_bytes") is True
        and p2_uni.get("representation") == "unibyte-input"
        and p2_uni.get("alphabet_checked") == "base64url"
        and (p2_uni.get("first_invalid_byte") or {}).get("offset") == 4
        and (p2_uni.get("first_invalid_byte") or {}).get("byte") == 255
        and c2_multi.get("operation") == "base64-decode-string"
        and c2_multi.get("multibyte") is True
        and p2_multi.get("exact_input_bytes") is False
        and p2_multi.get("representation") == "utf8-emacs-text"
        and (p2_multi.get("first_invalid_byte") or {}).get("offset") == 4
        and (p2_multi.get("first_invalid_byte") or {}).get("byte") == 255
        and isinstance(r2.get("diagnostic_payload"), dict)
        and r2["diagnostic_payload"].get("capture_count") == 2
        and ret2["passed"] is True
    )

    case3_ok = (
        r3["passed"] is True
        and r3["assessment"]["passed"] is True
        and r3["status"] == "passed"
        and r3["complete"] is True
        and r3["exit_code"] == 0
        and r3["remaining_group_terminated"] is False
        and r3["base64_error_capture_count"] == 0
        and r3["base64_error_captures"] == []
        and r3["diagnostic_file"] is None
        and r3["diagnostic_payload"] is None
        and ret3["passed"] is True
    )

    return {
        "passed": bool(case1_ok and case2_ok and case3_ok),
        "assertions": {
            "region_byte255_and_state_preservation": case1_ok,
            "string_base64url_and_multibyte_byte255": case2_ok,
            "success_path_does_not_fabricate_failure": case3_ok,
            "post_exit_forensic_state_retained": bool(
                ret1["passed"] and ret2["passed"] and ret3["passed"]
            ),
        },
        "retention_checks": [ret1, ret2, ret3],
        "cases": rows,
    }


def main(argv: list[str] | None = None) -> int:
    started = time.monotonic()
    parser = argparse.ArgumentParser(
        description="Run bounded error-only Base64 diagnostic on remote09."
    )
    parser.add_argument("--program", required=True, help="Path to product binary.")
    parser.add_argument("--dump", default=None, help="Optional path to .pdmp image.")
    parser.add_argument("--label", default="candidate", help="Product label.")
    parser.add_argument("--test-id", default=DEFAULT_TEST_ID, help="ERT test ID.")
    parser.add_argument(
        "--iterations", type=int, default=1, help="Bounded iteration count (>= 1)."
    )
    parser.add_argument(
        "--timeout", type=float, default=300.0, help="Per-iteration timeout in seconds."
    )
    parser.add_argument(
        "--stop-on-failure",
        action="store_true",
        help="Stop immediately when an iteration fails or captures a Base64 error.",
    )
    parser.add_argument(
        "--verify-capture",
        action="store_true",
        help="Run self-contained capture verification cases before or instead of remote09.",
    )
    parser.add_argument(
        "--skip-remote09",
        action="store_true",
        help="When combined with --verify-capture, skip running the remote09 ERT test.",
    )
    parser.add_argument(
        "--out", required=True, help="Fresh output directory under build/."
    )
    args = parser.parse_args(argv)

    suite = _load_stable_suite()
    execution = _load_runtime_execution()
    if not args.label or any(c not in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_-" for c in args.label):
        raise suite.SuiteError("--label must be a nonempty local product identifier")
    if args.iterations < 1 or args.iterations > 50:
        raise suite.SuiteError("--iterations must be between 1 and 50")
    if not math.isfinite(args.timeout) or args.timeout <= 0:
        raise suite.SuiteError("--timeout must be positive")
    if args.skip_remote09 and not args.verify_capture:
        raise suite.SuiteError("--skip-remote09 requires --verify-capture")

    out_root = suite._output_path(args.out, REPO)
    if not out_root.is_relative_to(REPO / "build"):
        raise suite.SuiteError("--out must be inside build/")

    dump_args = ["--dump-file=" + os.path.abspath(args.dump)] if args.dump else []
    paths = suite._product_paths(args.program, dump_args)
    fixture = suite._source_path(REPO, DIAG_FIXTURE_REL)
    base_runner = suite._source_path(REPO, BASE_RUNNER_REL)
    manifest = suite.load_manifest(REPO / MANIFEST_REL, REPO)

    source_names = [str(Path(__file__).resolve().relative_to(REPO)),
                    "tools/stable-suite.py", "tools/runtime-execution.py",
                    MANIFEST_REL, DIAG_FIXTURE_REL, BASE_RUNNER_REL]
    source_hashes = {name: execution.digest(REPO / name) for name in source_names}
    product_before = suite._public_product_identity(paths)
    fixture_before = suite._stat_identity(fixture)
    runner_before = suite._stat_identity(base_runner)

    out_root.mkdir(parents=True, mode=0o700)

    capture_verification = None
    if args.verify_capture:
        capture_verification = _run_verify_capture(
            suite=suite,
            execution=execution,
            out_root=out_root,
            paths=paths,
            fixture=fixture,
            timeout=min(args.timeout, 30.0),
            label=args.label,
        )

    materialized = None
    runs = []
    if not args.skip_remote09:
        materialized = _materialize_single_file(suite, manifest, out_root)
        test_root = out_root / "test"
        for index in range(1, args.iterations + 1):
            row = _run_iteration(
                suite=suite,
                out_root=out_root,
                program=paths["program"],
                dump=paths["dump"],
                lisp_root=paths["lisp_root"],
                fixture=fixture,
                test_root=test_root,
                test_id=args.test_id,
                iteration=index,
                timeout=args.timeout,
                label=args.label,
                execution=execution,
            )
            runs.append(row)
            if args.stop_on_failure and not row["passed"]:
                break

    runs_passed = bool(runs) and all(row["passed"] for row in runs) if not args.skip_remote09 else True
    verify_passed = capture_verification["passed"] if capture_verification is not None else True
    test_tree_retirement = {
        "status": "retained", "reason": "await final receipt and identity-checked cleanup"
    }

    product_after = suite._public_product_identity(paths)
    fixture_after = suite._stat_identity(fixture)
    runner_after = suite._stat_identity(base_runner)
    inputs_unchanged = (
        product_before == product_after
        and fixture_before == fixture_after
        and runner_before == runner_after
        and all(execution.digest(REPO / name) == value
                for name, value in source_hashes.items())
    )

    report = {
        "schema": RESULT_SCHEMA,
        "source_sha256": source_hashes,
        "test_id": args.test_id if not args.skip_remote09 else None,
        "logical_file": LOGICAL_FILE if not args.skip_remote09 else DIAG_FIXTURE_REL,
        "product_label": args.label,
        "verify_capture_requested": bool(args.verify_capture),
        "skip_remote09": bool(args.skip_remote09),
        "capture_verification": capture_verification,
        "iterations_requested": 0 if args.skip_remote09 else args.iterations,
        "iterations_executed": len(runs),
        "passed_count": sum(1 for row in runs if row["passed"]),
        "failed_count": sum(1 for row in runs if not row["passed"]),
        "base64_capture_count": sum(
            row.get("base64_error_capture_count", 0) for row in runs
        ),
        "passed": runs_passed and verify_passed and inputs_unchanged,
        "inputs_unchanged": inputs_unchanged,
        "materialized_input": materialized,
        "product_identity_before": product_before,
        "product_identity_after": product_after,
        "diagnostic_fixture_before": fixture_before,
        "diagnostic_fixture_after": fixture_after,
        "base_runner_before": runner_before,
        "base_runner_after": runner_after,
        "test_tree_retirement": test_tree_retirement,
        "runs": runs,
        "seconds": time.monotonic() - started,
    }
    _write_json(out_root / "results.json", report)
    return 0 if report["passed"] else 1


if __name__ == "__main__":
    sys.exit(main())
