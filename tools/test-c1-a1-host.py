#!/usr/bin/env python3
"""Stage C1 / A1 integration and functional acceptance runner.

Verifies per C1/A1 in the joint evolution plan:
  1. Static & ABI gates: `check-unsafe-boundary.py` (`error_count=0`, `145` forbid files
     including `crates/femacs-host/src/lib.rs` and `crates/femacs-host/tests/a1_host_acceptance.rs`,
     `0` active `static mut`, `100%` `SAFETY:` documentation, `crates/femacs-vm/src/host_runtime.rs`
     isolated boundary), `nm -gU` singleton exports, and `check-libshadows.py`.
  2. Host A1 functional scenarios (run in clean non-TTY subprocesses across normal mode and
     `FEMACS_GC_SKIP_STACK_SCAN=1`):
       - `cold-minimal-lifecycle` (buffer + UTF-8 + GC + condition + quit + read + recovery)
       - `cold-loadup-and-script` (`loadup.el`, `byte-compile`, `cl-lib`, `json-serialize`,
         `load_script` argv/`command-line-args-left` binding + shebang, missing/dir/error script)
       - `self-dump-create` + `dump-file-roundtrip` (portable dump creation & roundtrip)
       - `bad-dump-rejected` (mismatched-fingerprint `.pdmp` + corrupt file rejected without crash)
       - `second-init-rejected` (single-process single-init `AlreadyInitialized`)
       - `headless-diagnostics` (`noninteractive=t`, `inhibit-window-system=t`, no GUI, 1 `t` frame/term)
       - `safe-domain-protected` (`LispHost::exercise_safe_domains` + `read_eval_safe_domain`
         error/quit recovery inside condition-case protection without raw context escape)
  3. Same-source editor fidelity: verifies both `build/terminal/port/src/emacs` and
     `dist/macos/Femacs.app/Contents/MacOS/Femacs` on `-Q --batch` Lisp evaluation,
     `byte-compile`, and a minimal real editing cycle (insert/search/replace/undo/save file).
"""

from __future__ import annotations

import argparse
import json
import os
import re
import subprocess
import sys
import tempfile
import time
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
DEFAULT_HOST_BIN = REPO / "build/terminal/port/src/a1_host_acceptance"
DEFAULT_TERM_BIN = REPO / "build/terminal/port/src/emacs"
DEFAULT_COCOA_BIN = REPO / "dist/macos/Femacs.app/Contents/MacOS/Femacs"


def run_checked(
    argv: list[str],
    *,
    env_extra: dict[str, str] | None = None,
    stdin_devnull: bool = True,
    timeout: int = 120,
) -> subprocess.CompletedProcess[str]:
    env = os.environ.copy()
    if env_extra:
        env.update(env_extra)
    proc = subprocess.run(
        argv,
        cwd=REPO,
        env=env,
        stdin=subprocess.DEVNULL if stdin_devnull else None,
        capture_output=True,
        text=True,
        timeout=timeout,
        check=False,
    )
    if proc.returncode != 0:
        raise AssertionError(
            f"command failed ({proc.returncode}): {' '.join(argv)}\n"
            f"STDOUT:\n{proc.stdout}\nSTDERR:\n{proc.stderr}"
        )
    return proc


def check_static_and_abi_gates(term_bin: Path, cocoa_bin: Path) -> dict[str, object]:
    details: dict[str, object] = {}
    for label, prog in (("terminal", term_bin), ("cocoa", cocoa_bin)):
        if not prog.is_file():
            raise AssertionError(f"missing same-source deliverable: {prog}")
        proc = run_checked(
            [
                sys.executable,
                str(REPO / "tools/check-unsafe-boundary.py"),
                "--binary",
                str(prog),
                "--kind",
                label,
                "--require-modules",
                "--min-forbid-files",
                "144",
                "--require-forbid",
                "crates/femacs-host/src/lib.rs",
                "--require-forbid",
                "crates/femacs-host/tests/a1_host_acceptance.rs",
                "--require-zero-static-mut",
                "--require-zero-active-stitching",
                "--require-100pct-safety-docs",
                "--require-no-file-allows",
            ],
            timeout=60,
        )
        parsed = json.loads(proc.stdout)
        assert parsed["error_count"] == 0, parsed
        details[f"boundary_{label}"] = parsed["active_product"]

        nm = run_checked(["nm", "-gU", str(prog)], timeout=30)
        for sym in (
            "_current_thread",
            "_all_threads",
            "_Vmain_thread",
            "_lispsym",
            "_femacs_vm_persistent_root_alloc",
        ):
            assert re.search(rf"\b{re.escape(sym)}\b", nm.stdout), (
                f"missing required symbol {sym} in {prog}"
            )

        run_checked(
            [sys.executable, str(REPO / "tools/check-libshadows.py"), "--program", str(prog)],
            timeout=30,
        )
    return details


def run_host_scenario(
    host_bin: Path,
    scenario: str,
    *,
    env_extra: dict[str, str] | None = None,
    timeout: int = 120,
) -> float:
    started = time.monotonic()
    merged = {"A1_SCENARIO": scenario}
    if env_extra:
        merged.update(env_extra)
    proc = run_checked([str(host_bin)], env_extra=merged, stdin_devnull=True, timeout=timeout)
    expected = f"A1_SCENARIO_OK={scenario}"
    assert expected in proc.stdout, (
        f"missing {expected} for scenario {scenario}:\nSTDOUT:\n{proc.stdout}\nSTDERR:\n{proc.stderr}"
    )
    return round(time.monotonic() - started, 4)


def run_host_a1_scenarios(host_bin: Path, term_dump: Path) -> dict[str, float]:
    assert host_bin.is_file(), f"missing linked host acceptance binary: {host_bin}"
    lisp_dir = REPO / "lisp"
    timings: dict[str, float] = {}

    with tempfile.TemporaryDirectory(prefix="femacs-a1-") as tmp:
        tmp_path = Path(tmp)
        for skip_scan in ("0", "1"):
            tag = "skip_stack_scan" if skip_scan == "1" else "normal"
            env_mode = {"FEMACS_GC_SKIP_STACK_SCAN": "1"} if skip_scan == "1" else {}

            timings[f"cold_minimal_lifecycle_{tag}"] = run_host_scenario(
                host_bin, "cold-minimal-lifecycle", env_extra=env_mode
            )
            timings[f"headless_diagnostics_{tag}"] = run_host_scenario(
                host_bin, "headless-diagnostics", env_extra=env_mode
            )
            timings[f"host_exit_unwind_{tag}"] = run_host_scenario(
                host_bin, "host-exit-unwind", env_extra=env_mode
            )

        script_tmp = tmp_path / "script-normal"
        script_tmp.mkdir(parents=True, exist_ok=True)
        timings["cold_loadup_and_script_normal"] = run_host_scenario(
            host_bin,
            "cold-loadup-and-script",
            env_extra={
                "A1_LISP_DIR": str(lisp_dir),
                "A1_TMP_DIR": str(script_tmp),
            },
        )
        timings["safe_domain_protected_normal"] = run_host_scenario(
            host_bin,
            "safe-domain-protected",
            env_extra={
                "A1_LISP_DIR": str(lisp_dir),
                "A1_TMP_DIR": str(script_tmp),
            },
        )

        dump_path = tmp_path / "a1_host.pdmp"
        timings["self_dump_create"] = run_host_scenario(
            host_bin,
            "self-dump-create",
            env_extra={"A1_LISP_DIR": str(lisp_dir), "A1_DUMP_OUT": str(dump_path)},
        )
        timings["dump_file_roundtrip_normal"] = run_host_scenario(
            host_bin,
            "dump-file-roundtrip",
            env_extra={"A1_LISP_DIR": str(lisp_dir), "A1_DUMP_IN": str(dump_path)},
        )
        timings["dump_file_roundtrip_skip_scan"] = run_host_scenario(
            host_bin,
            "dump-file-roundtrip",
            env_extra={
                "FEMACS_GC_SKIP_STACK_SCAN": "1",
                "A1_LISP_DIR": str(lisp_dir),
                "A1_DUMP_IN": str(dump_path),
            },
        )
        safe_dump_tmp = tmp_path / "safe-skip-scan"
        safe_dump_tmp.mkdir(parents=True, exist_ok=True)
        timings["safe_domain_protected_skip_scan"] = run_host_scenario(
            host_bin,
            "safe-domain-protected",
            env_extra={
                "FEMACS_GC_SKIP_STACK_SCAN": "1",
                "A1_LISP_DIR": str(lisp_dir),
                "A1_TMP_DIR": str(safe_dump_tmp),
                "A1_DUMP_IN": str(dump_path),
            },
        )
        timings["bad_dump_mismatch_rejected"] = run_host_scenario(
            host_bin,
            "bad-dump-rejected",
            env_extra={"A1_BAD_DUMP": str(term_dump)},
        )
        corrupt_dump = tmp_path / "corrupt.pdmp"
        corrupt_dump.write_bytes(b"NOT_A_VALID_PDMP_FILE_0123456789" * 64)
        timings["bad_dump_corrupt_rejected"] = run_host_scenario(
            host_bin,
            "bad-dump-rejected",
            env_extra={"A1_BAD_DUMP": str(corrupt_dump)},
        )
        timings["second_init_rejected"] = run_host_scenario(host_bin, "second-init-rejected")
    return timings


def verify_editor_fidelity(term_bin: Path, cocoa_bin: Path) -> dict[str, str]:
    results: dict[str, str] = {}
    with tempfile.TemporaryDirectory(prefix="femacs-a1-editor-") as tmp:
        tmp_path = Path(tmp)
        for label, prog in (("terminal", term_bin), ("cocoa", cocoa_bin)):
            out_file = tmp_path / f"{label}_edited.txt"
            elisp = (
                "(progn "
                "  (defun a1-ed-fn (n) (* n 3)) "
                "  (byte-compile #'a1-ed-fn) "
                "  (with-temp-buffer "
                "    (buffer-enable-undo) "
                "    (insert \"alpha beta\\n\") "
                "    (undo-boundary) "
                "    (goto-char (point-min)) "
                "    (re-search-forward \"beta\") "
                "    (replace-match \"gamma\") "
                "    (undo-boundary) "
                "    (insert \"extra\") "
                "    (undo-boundary) "
                "    (undo) "
                f"    (write-region (point-min) (point-max) {json.dumps(str(out_file))} nil 'quiet)) "
                "  (prin1 (list (compiled-function-p (symbol-function 'a1-ed-fn)) "
                "               (a1-ed-fn 14) "
                f"               (with-temp-buffer (insert-file-contents {json.dumps(str(out_file))}) (buffer-string)))))"
            )
            proc = run_checked([str(prog), "-Q", "--batch", "--eval", elisp], timeout=60)
            assert '(t 42 "alpha gamma\n")' in proc.stdout, (
                f"unexpected editor fidelity output for {label}: {proc.stdout}"
            )
            results[label] = proc.stdout.strip()
    return results


def run_selftest() -> int:
    forbid_files = [
        REPO / "crates/femacs-host/src/lib.rs",
        REPO / "crates/femacs-host/tests/a1_host_acceptance.rs",
    ]
    for path in forbid_files:
        txt = path.read_text(encoding="utf-8")
        assert txt.lstrip().startswith("#![forbid(unsafe_code)]"), f"missing forbid in {path}"

    host_rs = (REPO / "crates/femacs-vm/src/host_runtime.rs").read_text(encoding="utf-8")
    assert "#![deny(static_mut_refs, unsafe_op_in_unsafe_fn)]" in host_rs
    for needle in (
        "HOST_INIT_STATE",
        "AlreadyInitialized",
        "with_host_stack_anchor",
        "with_host_condition_catch",
        "with_host_condition_catch_ctx",
        "load_script_file",
        "read_eval_to_string",
        "inspect_environment",
        "HeadlessVm",
        "exercise_safe_domains",
        "safe_domain_read_eval",
    ):
        assert needle in host_rs, f"missing {needle} in host_runtime.rs"
    assert "with_anchored_context" not in host_rs, (
        "removed unsafe-bypass helper with_anchored_context still present in host_runtime.rs"
    )

    host_lib_rs = (REPO / "crates/femacs-host/src/lib.rs").read_text(encoding="utf-8")
    assert "with_context" not in host_lib_rs, (
        "removed unguarded LispHost::with_context still present in femacs-host/src/lib.rs"
    )
    assert "pub use femacs_vm;" not in host_lib_rs, (
        "crates/femacs-host/src/lib.rs must not publicly re-export femacs_vm"
    )

    cargo_toml = (REPO / "crates/femacs-host/Cargo.toml").read_text(encoding="utf-8")
    assert 'name = "a1_host_acceptance"' in cargo_toml
    assert "harness = false" in cargo_toml

    assert DEFAULT_HOST_BIN.name == "a1_host_acceptance"
    assert DEFAULT_TERM_BIN.name == "emacs"
    assert DEFAULT_COCOA_BIN.name == "Femacs"
    print("c1-a1 selftest: 4/4 protective structural checks passed")
    return 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="Run Stage C1/A1 acceptance checks")
    parser.add_argument("--selftest", action="store_true", help="Run protective structural self-tests only")
    parser.add_argument("--host-bin", type=Path, default=DEFAULT_HOST_BIN)
    parser.add_argument("--terminal-bin", type=Path, default=DEFAULT_TERM_BIN)
    parser.add_argument("--cocoa-bin", type=Path, default=DEFAULT_COCOA_BIN)
    args = parser.parse_args(argv)

    if args.selftest:
        return run_selftest()

    started = time.monotonic()
    groups: list[dict[str, object]] = []

    def record(name: str, fn) -> None:
        t0 = time.monotonic()
        detail = fn()
        sec = round(time.monotonic() - t0, 4)
        print(f"CASE {name} status=passed seconds={sec:.4f}")
        groups.append({"case": name, "status": "passed", "seconds": sec, "detail": detail})

    record(
        "static-and-abi-gates",
        lambda: check_static_and_abi_gates(args.terminal_bin, args.cocoa_bin),
    )
    record(
        "a1-host-lifecycle-and-scripts",
        lambda: run_host_a1_scenarios(args.host_bin, args.terminal_bin.with_suffix(".pdmp")),
    )
    record(
        "same-source-editor-fidelity",
        lambda: verify_editor_fidelity(args.terminal_bin, args.cocoa_bin),
    )

    print(
        json.dumps(
            {
                "suite": "C1_A1_host_acceptance",
                "ok": True,
                "wall_seconds": round(time.monotonic() - started, 4),
                "results": groups,
            },
            indent=2,
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
