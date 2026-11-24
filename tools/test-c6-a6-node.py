#!/usr/bin/env python3
"""Verify Phase I Stage C6/A6 Node/V8 external ESM consumer integration.

Consumes an existing verified frozen Eliscript runtime package as strictly
read-only input, builds a Bun-free PATH containing only Node, verifies standard
node_modules ESM import of ``eliscript/runtime/lisp`` plus all shared editor
worker assertions inside Cocoa Fe.app and Terminal PTY sessions, and tests a
relocated module outside any node_modules ancestor against advertised
``runtime-resolution`` capability honesty.
"""

from __future__ import annotations

import argparse
import importlib.util
import json
import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
DEFAULT_CLIENT = REPO / "examples/eliscript-worker/femacs-eliscript-worker.el"
DEFAULT_FIXTURE = REPO / "tools/fixtures/c5-a5-worker-cases.el"

CHECK_RUNTIME_LISP_MJS = """\
import { createSession, evalProgram, evalSession, printValue } from "eliscript/runtime/lisp";

const v1 = evalProgram("(+ 20 22)");
const session = createSession();
const v2 = evalSession(session, "(let ((x 6)) (* x 7))");
const printed = printValue(v1);
if (v1 !== 42 || v2 !== "42" || printed !== "42") {
  throw new Error(`unexpected Lisp runtime result: v1=${v1} v2=${v2} printed=${printed}`);
}
console.log(JSON.stringify({ passed: true, specifier: "eliscript/runtime/lisp", v1, v2, printed }));
"""


def load_c5_harness():
    spec = importlib.util.spec_from_file_location(
        "test_c5_a5_worker", Path(__file__).with_name("test-c5-a5-worker.py")
    )
    if spec is None or spec.loader is None:
        raise RuntimeError("unable to load tools/test-c5-a5-worker.py")
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


c5 = load_c5_harness()


def resolve_node_binary(node_arg: str | None) -> Path:
    candidate = node_arg or shutil.which("node")
    if not candidate:
        raise FileNotFoundError("node runtime executable not found")
    resolved = Path(candidate).expanduser().resolve()
    if not resolved.is_file() or not os.access(resolved, os.X_OK):
        raise FileNotFoundError(f"node runtime is not executable: {resolved}")
    return resolved


UNSET_NODE_ENV_KEYS = ("NODE_OPTIONS", "NODE_PATH", "FEMACS_ELISCRIPT_JS_RUNNER")


def build_bun_free_env(
    node_real: Path,
    bin_dir: Path,
    *,
    base_env: dict[str, str] | None = None,
) -> tuple[dict[str, str], str]:
    bin_dir.mkdir(parents=True, exist_ok=True)
    node_link = bin_dir / "node"
    if not node_link.exists():
        node_link.symlink_to(node_real)
    node_only_path = f"{bin_dir}:/usr/bin:/bin"
    found_bun = shutil.which("bun", path=node_only_path)
    found_node = shutil.which("node", path=node_only_path)
    if found_bun is not None:
        raise RuntimeError(f"expected Bun-free PATH, but found bun at {found_bun}")
    if found_node != str(node_link):
        raise RuntimeError(f"expected node at {node_link}, got {found_node!r}")
    env = dict(os.environ if base_env is None else base_env)
    env["PATH"] = node_only_path
    for key in UNSET_NODE_ENV_KEYS:
        env.pop(key, None)
        assert key not in env, f"expected {key} stripped from Node environment"
    return env, node_only_path


def ensure_no_node_modules_ancestor(path: Path) -> None:
    for probe_root in (path, path.resolve()):
        for ancestor in (probe_root, *probe_root.parents):
            nm = ancestor / "node_modules"
            if nm.exists() or nm.is_symlink():
                raise RuntimeError(f"unexpected node_modules ancestor above relocated probe: {nm}")


def check_standard_esm_lisp_import(consumer_dir: Path, node_exec: Path, env: dict[str, str]) -> dict[str, object]:
    entry = consumer_dir / "check-runtime-lisp.mjs"
    entry.write_text(CHECK_RUNTIME_LISP_MJS, encoding="utf-8")
    proc = subprocess.run(
        [str(node_exec), str(entry)],
        cwd=consumer_dir,
        env=env,
        text=True,
        capture_output=True,
        timeout=30.0,
    )
    if proc.returncode != 0:
        raise RuntimeError(
            f"Node standard ESM eliscript/runtime/lisp check failed (rc={proc.returncode}):\n"
            f"STDOUT:\n{proc.stdout}\nSTDERR:\n{proc.stderr}"
        )
    data = json.loads(proc.stdout.strip())
    if not data.get("passed"):
        raise RuntimeError(f"unexpected eliscript/runtime/lisp output: {data}")
    return data


def run_guard_self_tests(
    valid_pkg: Path,
    node_real: Path,
    terminal_bin: Path,
) -> dict[str, object]:
    verified = c5.verify_frozen_package(valid_pkg)
    script = str(Path(__file__).resolve())
    checks: list[dict[str, object]] = []

    with tempfile.TemporaryDirectory(prefix="femacs-c6-guards-") as tmp_dir:
        tmp = Path(tmp_dir)

        # 1. --out inside read-only --frozen-package is refused and closure stays intact.
        nested_pkg = tmp / "nested-pkg"
        shutil.copytree(valid_pkg, nested_pkg, symlinks=False)
        before_closure = c5.verify_frozen_package(nested_pkg)["closure_sha256"]
        nested_out = nested_pkg / "nested-c6-out"
        p1 = subprocess.run(
            [
                sys.executable,
                "-B",
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
        after_closure = c5.verify_frozen_package(nested_pkg)["closure_sha256"]
        assert p1.returncode != 0 and not nested_out.exists() and after_closure == before_closure
        checks.append(
            {
                "id": "refuse_out_inside_frozen_package",
                "passed": True,
                "rc": p1.returncode,
                "closure_sha256": after_closure,
            }
        )

        # 2. Existing --out is refused and marker preserved untouched.
        existing_out = tmp / "existing-out"
        existing_out.mkdir()
        marker = existing_out / "keep.txt"
        marker.write_text("preserve-c6", encoding="utf-8")
        p2 = subprocess.run(
            [
                sys.executable,
                "-B",
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
        assert p2.returncode != 0 and marker.read_text(encoding="utf-8") == "preserve-c6"
        checks.append({"id": "refuse_existing_out_dir", "passed": True, "rc": p2.returncode})

        # 3. Tampered frozen package closure is rejected before launch.
        tampered_pkg = tmp / "tampered-pkg"
        shutil.copytree(valid_pkg, tampered_pkg, symlinks=False)
        with (tampered_pkg / "dist/runtime/lisp.mjs").open("a", encoding="utf-8") as f:
            f.write("\n// tampered\n")
        p3 = subprocess.run(
            [
                sys.executable,
                "-B",
                script,
                "--frozen-package",
                str(tampered_pkg),
                "--out",
                str(tmp / "out-3"),
            ],
            capture_output=True,
            text=True,
            timeout=15.0,
        )
        assert p3.returncode != 0 and not (tmp / "out-3").exists()
        checks.append({"id": "reject_tampered_frozen_closure", "passed": True, "rc": p3.returncode})

        # 4. Constructed Node-only PATH excludes bun and finds symlinked node.
        _, node_only_path = build_bun_free_env(node_real, tmp / "node-only-bin")
        checks.append(
            {
                "id": "verify_bun_free_path",
                "passed": True,
                "bun_found": shutil.which("bun", path=node_only_path) is not None,
                "node_found": shutil.which("node", path=node_only_path) is not None,
            }
        )

        # 5. Inherited NODE_OPTIONS / NODE_PATH / FEMACS_ELISCRIPT_JS_RUNNER are stripped.
        polluted = {
            "PATH": os.environ.get("PATH", "/usr/bin:/bin"),
            "NODE_OPTIONS": "--require=/nonexistent-preload.cjs",
            "NODE_PATH": "/nonexistent-node-path",
            "FEMACS_ELISCRIPT_JS_RUNNER": "bun",
        }
        san_env, san_path = build_bun_free_env(node_real, tmp / "sanitized-bin", base_env=polluted)
        assert (
            "NODE_OPTIONS" not in san_env
            and "NODE_PATH" not in san_env
            and "FEMACS_ELISCRIPT_JS_RUNNER" not in san_env
            and san_env["PATH"] == san_path
        )
        checks.append({"id": "verify_node_env_sanitized", "passed": True})

        # 6. Only explicit missing-resolution errors classify as missing-resolution (not timeout/protocol/crash).
        assert terminal_bin.is_file(), f"missing terminal binary for unit check: {terminal_bin}"
        elisp_unit = """
(progn
  (unless (and (femacs-eliscript--module-resolution-missing-p '((code . "ERR_MODULE_NOT_FOUND") (message . "Cannot find package \\x27eliscript\\x27 imported from /x/y.mjs")))
               (femacs-eliscript--module-resolution-missing-p '((code . "ResolveMessage") (message . "Cannot find module \\x27eliscript/platform/worker.mjs\\x27")))
               (femacs-eliscript--module-resolution-missing-p '((code . "MODULE_NOT_FOUND") (message . "Cannot find package \\x27eliscript\\x27"))))
    (error "Positive missing-resolution cases failed"))
  (dolist (non-res '(((code . "timeout") (message . "worker did not stop"))
                     ((code . "probe-timeout") (message . "Runtime-resolution probe timed out"))
                     ((code . "worker-exit") (message . "worker exited: killed"))
                     ((code . "worker-restart") (message . "worker restarted"))
                     ((code . "worker-stop") (message . "worker stopped"))
                     ((code . "client-protocol-error") (message . "bad json"))
                     ((code . "cancelled") (message . "cancelled"))
                     ((code . "ResolveMessage") (message . "SyntaxError: unexpected token"))
                     nil))
    (when (femacs-eliscript--module-resolution-missing-p non-res)
      (error "Non-resolution error misclassified as unsupported: %S" non-res)))
  (princ "PROBE_CLASSIFICATION_OK\\n"))
"""
        uproc = subprocess.run(
            [str(terminal_bin), "-Q", "--batch", "-l", str(DEFAULT_CLIENT), "--eval", elisp_unit],
            capture_output=True,
            text=True,
            timeout=20.0,
        )
        assert uproc.returncode == 0 and "PROBE_CLASSIFICATION_OK" in uproc.stdout, (
            f"probe error classification unit failed: rc={uproc.returncode} out={uproc.stdout!r} err={uproc.stderr!r}"
        )
        checks.append({"id": "verify_probe_error_classification_unit", "passed": True})

    return {
        "schema": "femacs-c6-guards-v1",
        "passed": True,
        "closure_sha256": verified["closure_sha256"],
        "file_count": verified["file_count"],
        "checks": checks,
    }


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--frozen-package", type=Path, required=True)
    parser.add_argument("--out", type=Path, default=None)
    parser.add_argument("--app", type=Path, default=Path("/Applications/Fe.app"))
    parser.add_argument("--terminal", type=Path, default=REPO / "build/terminal/port/src/emacs")
    parser.add_argument("--node", default=None)
    parser.add_argument(
        "--allow-dishonest-diagnostic",
        action="store_true",
        help="Record baseline advertised-true/unsupported false-advertisement as rc=0 diagnostic instead of failing strictly.",
    )
    parser.add_argument("--self-test-guards", action="store_true")
    args = parser.parse_args(argv)

    pkg_dir = args.frozen_package.resolve()
    terminal_bin = args.terminal.resolve()
    node_real = resolve_node_binary(args.node)

    if args.self_test_guards:
        guard_report = run_guard_self_tests(pkg_dir, node_real, terminal_bin)
        print(json.dumps(guard_report, indent=2))
        return 0

    if args.out is None:
        parser.error("--out is required unless --self-test-guards is set")

    out_dir = args.out.resolve()
    manifest = c5.verify_frozen_package(pkg_dir)
    if out_dir == pkg_dir or out_dir.is_relative_to(pkg_dir):
        raise ValueError(
            f"refusing to place --out inside read-only --frozen-package: out={out_dir} pkg={pkg_dir}"
        )
    if out_dir.exists():
        raise FileExistsError(f"refusing to overwrite existing output directory: {out_dir}")

    app_bundle = args.app.resolve()
    node_ver = c5.run_checked([str(node_real), "--version"], timeout=15.0).stdout.strip()
    node_sha = c5.sha256_file(node_real)

    with tempfile.TemporaryDirectory(prefix="femacs-c6-detached-") as detached_tmp:
        detached = Path(detached_tmp)
        env, node_only_path = build_bun_free_env(node_real, detached / "node-only-bin")
        node_link = detached / "node-only-bin/node"

        consumer_dir = detached / "consumer"
        consumer_dir.mkdir(parents=True, exist_ok=True)
        (consumer_dir / "package.json").write_text(
            json.dumps({"name": "femacs-c6-external-consumer", "private": True, "type": "module"}, indent=2) + "\n",
            encoding="utf-8",
        )
        nm_pkg = consumer_dir / "node_modules/eliscript"
        nm_pkg.parent.mkdir(parents=True, exist_ok=True)
        shutil.copytree(pkg_dir, nm_pkg, symlinks=False)
        c5.verify_frozen_package(nm_pkg)
        shutil.copytree(pkg_dir / "analysis", consumer_dir / "analysis", symlinks=False)
        shutil.copytree(pkg_dir / "probes", consumer_dir / "probes", symlinks=False)

        relocated_dir = detached / "relocated-no-nm"
        relocated_dir.mkdir(parents=True, exist_ok=True)
        ensure_no_node_modules_ancestor(relocated_dir)
        shutil.copytree(pkg_dir / "probes", relocated_dir / "probes", symlinks=False)

        esm_lisp_res = check_standard_esm_lisp_import(consumer_dir, node_link, env)

        worker_copy = detached / "femacs-eliscript-worker.el"
        fixture_copy = detached / "c5-a5-worker-cases.el"
        shutil.copy2(DEFAULT_CLIENT, worker_copy)
        shutil.copy2(DEFAULT_FIXTURE, fixture_copy)

        opt_in_app = c5.check_opt_in_clean_batch(app_bundle / "Contents/MacOS/Femacs", "Fe.app")
        opt_in_tty = c5.check_opt_in_clean_batch(terminal_bin, "terminal")

        extra_env = {
            "PATH": node_only_path,
            "FEMACS_A5_MODULE_ROOT": str(consumer_dir),
            "FEMACS_A5_RELOCATED_PROBE_MOD": str(relocated_dir / "probes/c5-probes.mjs"),
            "FEMACS_A5_RELOCATED_PROBE_MANIFEST": str(relocated_dir / "probes/eliscript-project.json"),
        }

        cocoa_work = detached / "cocoa-work"
        cocoa_work.mkdir(parents=True, exist_ok=True)
        cocoa_res = c5.run_cocoa_gui_cases(
            app_bundle,
            worker_copy,
            fixture_copy,
            nm_pkg,
            "node",
            cocoa_work,
            extra_env=extra_env,
            unset_env=UNSET_NODE_ENV_KEYS,
        )

        term_work = detached / "terminal-work"
        term_work.mkdir(parents=True, exist_ok=True)
        term_res = c5.run_terminal_pty_cases(
            terminal_bin,
            worker_copy,
            fixture_copy,
            nm_pkg,
            "node",
            term_work,
            extra_env=extra_env,
            unset_env=UNSET_NODE_ENV_KEYS,
        )

    cocoa_reloc = next((c for c in cocoa_res["cases"] if c.get("id") == "relocated_runtime_resolution_probe"), None)
    term_reloc = next((c for c in term_res["cases"] if c.get("id") == "relocated_runtime_resolution_probe"), None)
    if cocoa_reloc is None or term_reloc is None:
        raise RuntimeError("missing relocated_runtime_resolution_probe in editor report")

    cocoa_outcome = cocoa_reloc.get("outcome")
    term_outcome = term_reloc.get("outcome")
    valid_outcomes = cocoa_outcome in ("supported", "unsupported") and term_outcome in ("supported", "unsupported")
    honesty_passed = bool(
        valid_outcomes
        and cocoa_reloc.get("honest") is True
        and term_reloc.get("honest") is True
    )
    known_dishonest_baseline = bool(
        cocoa_outcome == "unsupported"
        and term_outcome == "unsupported"
        and cocoa_reloc.get("advertised") is True
        and term_reloc.get("advertised") is True
    )

    out_dir.mkdir(parents=True, exist_ok=False)
    report = {
        "schema": "femacs-c6-a6-node-v1",
        "runner": "node",
        "node_version": node_ver,
        "node_sha256": node_sha,
        "bun_in_path": False,
        "sanitized_node_env": list(UNSET_NODE_ENV_KEYS),
        "frozen_package_eliscript_commit": manifest["eliscript_commit"],
        "frozen_package_closure_sha256": manifest["closure_sha256"],
        "frozen_package_file_count": manifest["file_count"],
        "standard_esm_lisp_import": esm_lisp_res,
        "opt_in_clean": [opt_in_app, opt_in_tty],
        "cocoa": cocoa_res,
        "terminal": term_res,
        "upstream_capability_honesty_passed": honesty_passed,
        "relocated_runtime_resolution": {
            "cocoa": cocoa_reloc,
            "terminal": term_reloc,
        },
        "upstream_gap": (
            None
            if honesty_passed
            else (
                "runtime/worker.mjs unconditionally advertises runtime-resolution on Node "
                "even though installPackageResolver only registers Bun.plugin and relocated "
                "modules outside node_modules fail with ERR_MODULE_NOT_FOUND"
                if known_dishonest_baseline
                else f"unexpected relocated probe outcome: cocoa={cocoa_outcome!r} terminal={term_outcome!r}"
            )
        ),
    }
    report_file = out_dir / "c6-a6-node-report.json"
    report_file.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")

    if not honesty_passed:
        if args.allow_dishonest_diagnostic and known_dishonest_baseline:
            print(
                f"C6/A6 diagnostic baseline recorded (honesty=false, outcomes=unsupported): {report_file}"
            )
            return 0
        print(
            f"C6/A6 STRICT GATE FAILED (honesty={honesty_passed}, outcomes=({cocoa_outcome}, {term_outcome})): {report_file}",
            file=sys.stderr,
        )
        return 1

    print(
        f"C6/A6 strict Node verification PASSED ({node_ver}, outcomes=({cocoa_outcome}, {term_outcome})) -> {report_file}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
