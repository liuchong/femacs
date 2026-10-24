#!/usr/bin/env python3
"""Independent partner-owned Stage III bytecode runtime verification driver and contract suite.

Covers Stage III requirements III-D6 and T-VM1 through T-VM9 in docs/runtime-modernization.md:
- Multi-artifact execution of `tools/fixtures/bytecode-runtime.el` via `tools/check-ert.py`'s
  `run` and `parse_result` with isolated `HOME`, raw logs, per-case durations, and a per-run
  observation JSON file (`femacs-bytecode-runtime-observation-file`).
- CLI interface: `--candidate`, `--baseline`, optional `--reference`, `--out` (new directory only),
  `--timeout` (positive seconds), plus `--self-test` / `--json-out` for deterministic contract tests.
- Strict verification: requires all 10 expected ERT tests to pass with zero skips, zero unexpected
  outcomes, complete expected identity sets, and identical deterministic critical observations across
  `candidate`, `baseline`, and optional `reference` while allowing `diagnostics` (e.g., `gc_delta`)
  to vary across runs.
- Records source/fixture/binary/dump SHA256 fingerprints before and after measurement and rejects
  reused output directories or mid-run artifact tampering.
"""

from __future__ import annotations

import argparse
from dataclasses import asdict
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import stat
import subprocess
import sys
import tempfile
import time
import unittest
from unittest import mock

REPO = Path(__file__).resolve().parent.parent
CHECK_ERT_PATH = REPO / "tools/check-ert.py"
FIXTURES_DIR = REPO / "tools/fixtures"
FIXTURE_NAME = "bytecode-runtime.el"
FIXTURE_PATH = FIXTURES_DIR / FIXTURE_NAME
BYTECODE_RS_PATH = REPO / "src/bytecode.rs"
VM_DISPATCH_RS_PATH = REPO / "xtask/src/vm_dispatch.rs"
XTASK_CARGO_PATH = REPO / "xtask/Cargo.toml"

OBSERVATION_SCHEMA = "femacs-bytecode-runtime-observations-v1"

EXPECTED_ERT_TESTS: tuple[str, ...] = (
    "tvm1-varref-varset-varbind-short-and-1byte",
    "tvm1-varref7-varset7-varbind7-unbind7-and-constant2-wide",
    "tvm2-calls-subr-interpreted-lexbound-and-wide-arity",
    "tvm2-wrong-arity-and-eval-depth-restoration",
    "tvm3-branches-forward-backward-and-else-pop",
    "tvm3-switch-linear-and-hash-jump-tables",
    "tvm4-dynamic-binding-watchers-aliases-and-unbound",
    "tvm5-handlers-nonlocal-exit-and-gc-live-roots",
    "tvm6-backward-branch-quit-and-post-setjmp-quitcounter",
    "tvm6-deep-calls-stack-growth-and-forced-gc-stress",
)


def load_check_ert():
    """Load tools/check-ert.py as a module and register it in sys.modules."""
    spec = importlib.util.spec_from_file_location("check_ert", CHECK_ERT_PATH)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load module spec from {CHECK_ERT_PATH}")
    mod = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = mod
    spec.loader.exec_module(mod)
    return mod


def sha256_file(path: Path) -> str | None:
    if not path.is_file():
        return None
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(65536), b""):
            h.update(chunk)
    return h.hexdigest()


def discover_dump_paths(binary: Path) -> list[Path]:
    """Return candidate .pdmp files adjacent to binary for identity hashing."""
    parent = binary.resolve().parent
    candidates = [
        parent / f"{binary.name}.pdmp",
        parent / "emacs.pdmp",
        parent / "bootstrap-emacs.pdmp",
    ]
    seen: set[Path] = set()
    found: list[Path] = []
    for p in candidates:
        if p.is_file() and p not in seen:
            seen.add(p)
            found.append(p)
    return found


def artifact_fingerprint(binary: Path) -> dict:
    resolved = binary.resolve()
    dumps = {str(p): sha256_file(p) for p in discover_dump_paths(resolved)}
    return {
        "binary": str(resolved),
        "binary_sha256": sha256_file(resolved),
        "dumps": dumps,
    }


def git_provenance() -> dict:
    try:
        head = subprocess.check_output(
            ["git", "rev-parse", "HEAD"], cwd=REPO, text=True, stderr=subprocess.DEVNULL
        ).strip()
        diff = subprocess.check_output(
            ["git", "diff", "HEAD"], cwd=REPO, stderr=subprocess.DEVNULL
        )
        dirty_sha256 = hashlib.sha256(diff).hexdigest() if diff else None
    except (OSError, subprocess.SubprocessError):
        head = "unknown"
        dirty_sha256 = None
    return {
        "source_head": head,
        "dirty_diff_sha256": dirty_sha256,
        "fixture_sha256": sha256_file(FIXTURE_PATH),
        "bytecode_rs_sha256": sha256_file(BYTECODE_RS_PATH),
    }


def new_output(path: Path) -> Path:
    """Create a fresh output directory; reject any pre-existing file or directory."""
    resolved = path.resolve()
    if resolved.exists() or resolved.is_symlink():
        raise ValueError(f"refusing to reuse existing output path: {resolved}")
    resolved.mkdir(parents=True, exist_ok=False)
    return resolved


def elisp_quote_string(value: str) -> str:
    escaped = value.replace("\\", "\\\\").replace('"', '\\"')
    return f'"{escaped}"'


def load_observations(path: Path) -> tuple[dict, dict, list[str]]:
    """Load and validate a per-run observation JSON file."""
    errors: list[str] = []
    if not path.is_file():
        return {}, {}, [f"missing observation file: {path}"]
    try:
        payload = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        return {}, {}, [f"invalid observation JSON in {path}: {exc}"]
    if not isinstance(payload, dict):
        return {}, {}, [f"observation root must be an object in {path}"]
    if payload.get("schema") != OBSERVATION_SCHEMA:
        errors.append(
            f"unexpected observation schema in {path}: {payload.get('schema')!r}"
        )
    obs = payload.get("observations")
    diag = payload.get("diagnostics", {})
    if not isinstance(obs, dict):
        errors.append(f"observations field must be an object in {path}")
        obs = {}
    if not isinstance(diag, dict):
        errors.append(f"diagnostics field must be an object in {path}")
        diag = {}
    expected_set = set(EXPECTED_ERT_TESTS)
    actual_set = set(obs.keys())
    if actual_set != expected_set:
        missing = sorted(expected_set - actual_set)
        extra = sorted(actual_set - expected_set)
        errors.append(
            f"observation key mismatch in {path}: missing={missing}, extra={extra}"
        )
    return obs, diag, errors


def validate_ert_run(side: str, run_result) -> list[str]:
    """Validate that a single ERT RunResult passed all expected tests with zero skips."""
    errors: list[str] = []
    if run_result.status != "ok" or run_result.exit_code != 0:
        errors.append(
            f"{side}: status={run_result.status!r} exit_code={run_result.exit_code} note={run_result.note!r}"
        )
    if run_result.unexpected != 0:
        errors.append(f"{side}: unexpected={run_result.unexpected}")
    results = getattr(run_result, "results", {}) or {}
    expected_set = set(EXPECTED_ERT_TESTS)
    actual_set = set(results.keys())
    if actual_set != expected_set:
        missing = sorted(expected_set - actual_set)
        extra = sorted(actual_set - expected_set)
        errors.append(
            f"{side}: ERT test identity mismatch: missing={missing}, extra={extra}"
        )
    skipped = sorted(k for k, v in results.items() if str(v).lower() == "skipped")
    if skipped:
        errors.append(f"{side}: skipped tests are forbidden: {skipped}")
    non_passed = sorted(
        f"{k}={v}" for k, v in results.items() if str(v) != "passed"
    )
    if non_passed:
        errors.append(f"{side}: non-passed test outcomes: {non_passed}")
    test_seconds = getattr(run_result, "test_seconds", {}) or {}
    if set(test_seconds.keys()) != actual_set:
        errors.append(f"{side}: missing per-test durations for some executed tests")
    return errors


def compare_observations(
    left_name: str, left_obs: dict, right_name: str, right_obs: dict
) -> list[str]:
    """Compare deterministic observations between two artifacts."""
    diffs: list[str] = []
    all_keys = sorted(set(left_obs.keys()) | set(right_obs.keys()))
    for key in all_keys:
        if key not in left_obs:
            diffs.append(f"observation {key!r} missing on {left_name}")
            continue
        if key not in right_obs:
            diffs.append(f"observation {key!r} missing on {right_name}")
            continue
        if left_obs[key] != right_obs[key]:
            diffs.append(
                f"observation mismatch in {key}: {left_name}={left_obs[key]!r} vs {right_name}={right_obs[key]!r}"
            )
    return diffs


def run_artifact_suite(
    candidate: Path,
    baseline: Path,
    out_dir: Path,
    timeout: int = 120,
    reference: Path | None = None,
    test_root: Path = FIXTURES_DIR,
    fixture_rel: str = FIXTURE_NAME,
) -> dict:
    """Execute bytecode-runtime.el on candidate, baseline, and optional reference."""
    if timeout <= 0:
        raise ValueError("timeout must be positive")
    out = new_output(out_dir)
    check_ert = load_check_ert()

    sides: list[tuple[str, Path]] = [
        ("candidate", candidate),
        ("baseline", baseline),
    ]
    if reference is not None:
        sides.append(("reference", reference))

    failures: list[str] = []
    for name, prog in sides:
        if not prog.is_file():
            failures.append(f"missing {name} program: {prog}")
    fixture_file = test_root / fixture_rel
    if not fixture_file.is_file():
        failures.append(f"missing fixture file: {fixture_file}")

    fingerprints_before = {
        name: artifact_fingerprint(prog) for name, prog in sides if prog.is_file()
    }
    provenance = git_provenance()

    runs: dict[str, dict] = {}
    observations: dict[str, dict] = {}
    diagnostics: dict[str, dict] = {}
    comparisons: dict[str, dict] = {}

    if not failures:
        ert_root = out / "ert-logs"
        ert_root.mkdir(parents=True, exist_ok=True)
        old_evidence_root = check_ert.EVIDENCE_ROOT
        check_ert.EVIDENCE_ROOT = ert_root
        try:
            run_objects = {}
            for name, prog in sides:
                obs_file = out / f"observations-{name}.json"
                prelude = (
                    f"(setq femacs-bytecode-runtime-observation-file "
                    f"{elisp_quote_string(str(obs_file))})"
                )
                res = check_ert.run(
                    prog=prog,
                    test_root=test_root,
                    rel=fixture_rel,
                    timeout=timeout,
                    selector="t",
                    memory_mib=0,
                    prelude=prelude,
                )
                run_objects[name] = res
                runs[name] = {
                    **asdict(res),
                    "observation_file": str(obs_file),
                }
                failures.extend(validate_ert_run(name, res))
                obs, diag, obs_errors = load_observations(obs_file)
                observations[name] = obs
                diagnostics[name] = diag
                failures.extend(f"{name}: {e}" for e in obs_errors)
        finally:
            check_ert.EVIDENCE_ROOT = old_evidence_root

        # Pairwise ERT and deterministic observation comparisons
        verdict_cb, note_cb = check_ert.compare_results(
            run_objects["candidate"], run_objects["baseline"]
        )
        obs_diffs_cb = compare_observations(
            "candidate",
            observations.get("candidate", {}),
            "baseline",
            observations.get("baseline", {}),
        )
        comparisons["candidate_vs_baseline"] = {
            "ert_verdict": verdict_cb,
            "ert_note": note_cb,
            "observation_diffs": obs_diffs_cb,
        }
        if verdict_cb != "same":
            failures.append(
                f"candidate vs baseline ERT verdict={verdict_cb!r} ({note_cb})"
            )
        failures.extend(obs_diffs_cb)

        if "reference" in run_objects:
            verdict_cr, note_cr = check_ert.compare_results(
                run_objects["candidate"], run_objects["reference"]
            )
            obs_diffs_cr = compare_observations(
                "candidate",
                observations.get("candidate", {}),
                "reference",
                observations.get("reference", {}),
            )
            comparisons["candidate_vs_reference"] = {
                "ert_verdict": verdict_cr,
                "ert_note": note_cr,
                "observation_diffs": obs_diffs_cr,
            }
            if verdict_cr != "same":
                failures.append(
                    f"candidate vs reference ERT verdict={verdict_cr!r} ({note_cr})"
                )
            failures.extend(obs_diffs_cr)

    fingerprints_after = {
        name: artifact_fingerprint(prog) for name, prog in sides if prog.is_file()
    }
    for name, before_fp in fingerprints_before.items():
        after_fp = fingerprints_after.get(name)
        if before_fp != after_fp:
            failures.append(
                f"artifact fingerprint changed during run for {name}: before={before_fp} after={after_fp}"
            )

    passed = len(failures) == 0
    report = {
        "schema": "femacs-bytecode-runtime-report-v1",
        "passed": passed,
        "timeout_seconds": timeout,
        "expected_tests": list(EXPECTED_ERT_TESTS),
        "provenance": provenance,
        "fingerprints_before": fingerprints_before,
        "fingerprints_after": fingerprints_after,
        "runs": runs,
        "observations": observations,
        "diagnostics": diagnostics,
        "comparisons": comparisons,
        "failures": failures,
    }
    (out / "results.json").write_text(
        json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8"
    )
    return report


# ---------------------------------------------------------------------------
# Partner-Owned Unit & Contract Test Cases (T-VM1 .. T-VM9)
# ---------------------------------------------------------------------------


def write_fake_emacs(path: Path, script_body: str) -> Path:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(script_body, encoding="utf-8")
    path.chmod(path.stat().st_mode | stat.S_IXUSR | stat.S_IXGRP | stat.S_IXOTH)
    return path


def make_fake_ert_binary(
    path: Path,
    *,
    obs_override: dict | None = None,
    gc_delta: int = 2,
    skip_test: str | None = None,
    fail_test: str | None = None,
    omit_test: str | None = None,
    skip_obs_write: bool = False,
    mutate_self_on_exit: bool = False,
) -> Path:
    """Create a deterministic fake editor binary that emits native ERT batch lines and observations."""
    expected_list = [t for t in EXPECTED_ERT_TESTS if t != omit_test]
    default_obs = {
        t: {"status": "ok", "index": idx} for idx, t in enumerate(expected_list)
    }
    if obs_override:
        for k, v in obs_override.items():
            if k in default_obs:
                default_obs[k] = v

    payload_json = json.dumps(
        {
            "schema": OBSERVATION_SCHEMA,
            "test_count": len(default_obs),
            "observations": default_obs,
            "diagnostics": {t: {"gc_delta": gc_delta} for t in expected_list},
        }
    )

    script = f"""#!/usr/bin/env python3
import os, re, sys
from pathlib import Path

argv = sys.argv[1:]
obs_file = None
for i, arg in enumerate(argv):
    if arg == "--eval" and i + 1 < len(argv):
        m = re.search(r'femacs-bytecode-runtime-observation-file\\s+"([^"]+)"', argv[i + 1])
        if m:
            obs_file = Path(m.group(1))

if obs_file is not None and not {skip_obs_write!r}:
    obs_file.parent.mkdir(parents=True, exist_ok=True)
    obs_file.write_text({payload_json!r}, encoding="utf-8")

tests = {expected_list!r}
skip_test = {skip_test!r}
fail_test = {fail_test!r}

print(f"Running {{len(tests)}} tests (2026-10-24 19:30:00+0000, selector `t')")
expected = 0
unexpected = 0
skipped = 0
for idx, name in enumerate(tests, 1):
    if name == skip_test:
        print(f"  skipped  {{idx}}/{{len(tests)}}  {{name}} (0.001000 sec)")
        skipped += 1
    elif name == fail_test:
        print(f"   FAILED  {{idx}}/{{len(tests)}}  {{name}} (0.001000 sec)")
        unexpected += 1
    else:
        print(f"   passed  {{idx}}/{{len(tests)}}  {{name}} (0.001000 sec)")
        expected += 1

if skipped:
    print(f"\\nRan {{len(tests)}} tests, {{expected}} results as expected, {{unexpected}} unexpected, {{skipped}} skipped")
else:
    print(f"\\nRan {{len(tests)}} tests, {{expected}} results as expected, {{unexpected}} unexpected")

if {mutate_self_on_exit!r}:
    with open(__file__, "ab") as f:
        f.write(b"\\n# mutated\\n")

sys.exit(1 if unexpected else 0)
"""
    return write_fake_emacs(path, script)


class StageIIIBytecodeRuntimeTests(unittest.TestCase):
    """Timed unit and contract tests for T-VM1 through T-VM9."""

    def setUp(self) -> None:
        self.fixture_text = FIXTURE_PATH.read_text(encoding="utf-8")

    def test_TVM1_fixture_covers_short_1byte_2byte_widths_and_disassembly(self) -> None:
        """T-VM1: varref/set/bind/unbind short, 1-byte, 2-byte operands and >255 constants."""
        self.assertIn(
            "(ert-deftest tvm1-varref-varset-varbind-short-and-1byte ()",
            self.fixture_text,
        )
        self.assertIn(
            "(ert-deftest tvm1-varref7-varset7-varbind7-unbind7-and-constant2-wide ()",
            self.fixture_text,
        )
        # Prove fixture checks short ranges (8..13, 16..21, 24..29), 1-byte (14, 22, 30, 46),
        # and 2-byte (15=Bvarref7, 23=Bvarset7, 31=Bvarbind7, 47=Bunbind7, 129=Bconstant2)
        for token in (
            "(femacs-bc--has-opcode-range-p ops 8 13)",
            "(memq 14 ops)",
            "(femacs-bc--has-opcode-range-p ops 16 21)",
            "(memq 22 ops)",
            "(femacs-bc--has-opcode-range-p ops 24 29)",
            "(memq 30 ops)",
            "(memq 46 ops)",
            "(memq 15 ops)",
            "(memq 23 ops)",
            "(memq 31 ops)",
            "(memq 47 ops)",
            "(memq 129 ops)",
            "(>= max-varref7-arg 256)",
            "(>= max-varset7-arg 256)",
            "(>= max-varbind7-arg 256)",
            "(>= max-const2-arg 256)",
            "femacs-bc--disassemble-string",
        ):
            self.assertIn(token, self.fixture_text)

    def test_TVM2_fixture_covers_calls_arity_and_eval_depth_restoration(self) -> None:
        """T-VM2: subr/interpreted/lexbound calls, optional/rest/wide arity, wrong arity, depth."""
        self.assertIn(
            "(ert-deftest tvm2-calls-subr-interpreted-lexbound-and-wide-arity ()",
            self.fixture_text,
        )
        self.assertIn(
            "(ert-deftest tvm2-wrong-arity-and-eval-depth-restoration ()",
            self.fixture_text,
        )
        for token in (
            "(femacs-bc--has-opcode-range-p wide-ops 32 37)",
            "(memq 38 wide-ops)",
            "(memq 39 wide-ops)",
            "(equal call6-operands '(8))",
            "(equal call7-operands '(260))",
            "wrong-number-of-arguments",
            "max-lisp-eval-depth",
            "(should (equal depth-err '(caught-depth-error excessive-lisp-nesting)))",
        ):
            self.assertIn(token, self.fixture_text)

    def test_TVM3_fixture_covers_branches_else_pop_and_switch_tables(self) -> None:
        """T-VM3: forward/backward branches, else-pop stack behavior, linear and hash Bswitch."""
        self.assertIn(
            "(ert-deftest tvm3-branches-forward-backward-and-else-pop ()",
            self.fixture_text,
        )
        self.assertIn(
            "(ert-deftest tvm3-switch-linear-and-hash-jump-tables ()",
            self.fixture_text,
        )
        for token in (
            "(memq 130 loop-ops)",
            "(memq 131 loop-ops)",
            "(memq 132 loop-ops)",
            "(memq 133 and-ops)",
            "(memq 134 or-ops)",
            "(memq 183 linear-ops)",
            "(memq 183 hash-ops)",
            "(= (hash-table-count linear-ht) 4)",
            "(= (hash-table-count hash-ht) 8)",
        ):
            self.assertIn(token, self.fixture_text)

    def test_TVM4_fixture_covers_dynamic_binding_watchers_aliases_and_unbound(self) -> None:
        """T-VM4: dynamic binding/unbinding, variable watchers, varalias, and void-variable."""
        self.assertIn(
            "(ert-deftest tvm4-dynamic-binding-watchers-aliases-and-unbound ()",
            self.fixture_text,
        )
        for token in (
            "add-variable-watcher",
            "remove-variable-watcher",
            "defvaralias",
            "makunbound",
            "void-variable",
        ):
            self.assertIn(token, self.fixture_text)

    def test_TVM5_fixture_covers_handlers_nonlocal_returns_and_gc_live_roots(self) -> None:
        """T-VM5: nested catch/throw/condition-case/unwind-protect, pophandler, and GC roots."""
        self.assertIn(
            "(ert-deftest tvm5-handlers-nonlocal-exit-and-gc-live-roots ()",
            self.fixture_text,
        )
        for token in (
            "(memq 48 ops)",
            "(memq 49 ops)",
            "(memq 50 ops)",
            "(memq 142 ops)",
            "(garbage-collect)",
        ):
            self.assertIn(token, self.fixture_text)

    def test_TVM6_fixture_covers_quitcounter_and_gc_stress_separation(self) -> None:
        """T-VM6: backward-loop quitcounter (255 fresh, 1 post-setjmp) and non-strict GC delta."""
        self.assertIn(
            "(ert-deftest tvm6-backward-branch-quit-and-post-setjmp-quitcounter ()",
            self.fixture_text,
        )
        self.assertIn(
            "(ert-deftest tvm6-deep-calls-stack-growth-and-forced-gc-stress ()",
            self.fixture_text,
        )
        self.assertIn("(equal fresh-quit '(caught-quit quit 255))", self.fixture_text)
        self.assertIn(
            "(equal post-setjmp-quit '(caught-quit quit 1))", self.fixture_text
        )
        self.assertIn("(list :gc_delta gc-delta)", self.fixture_text)

    def test_TVM7_bytecode_abi_and_dispatch_mode_source_contracts(self) -> None:
        """T-VM7: unchanged public exec_byte_code ABI, bc_frame layout, opcode set, and retired experiment."""
        bytecode_src = BYTECODE_RS_PATH.read_text(encoding="utf-8")
        for abi_fn in (
            "pub unsafe extern \"C\" fn init_bc_thread",
            "pub unsafe extern \"C\" fn free_bc_thread",
            "pub unsafe extern \"C\" fn mark_bytecode",
            "pub unsafe extern \"C\" fn exec_byte_code",
            "pub unsafe extern \"C\" fn get_byte_code_arity",
            "pub unsafe extern \"C\" fn syms_of_bytecode",
        ):
            self.assertIn(abi_fn, bytecode_src)
        self.assertIn("pub struct bc_frame", bytecode_src)
        self.assertIn("std::mem::size_of::<bc_frame> () == 32", bytecode_src)
        for opcode_def in (
            "DEFINE! (Bvarref, 0o010);",
            "DEFINE! (Bvarset, 0o020);",
            "DEFINE! (Bvarbind, 0o030);",
            "DEFINE! (Bcall, 0o040);",
            "DEFINE! (Bunbind, 0o050);",
            "DEFINE! (Bpophandler, 0o060);",
            "DEFINE! (Bpushconditioncase, 0o061);",
            "DEFINE! (Bpushcatch, 0o062);",
            "DEFINE! (Bconstant2, 0o0201);",
            "DEFINE! (Bgoto, 0o0202);",
            "DEFINE! (Breturn, 0o0207);",
            "DEFINE! (Bswitch, 0o0267);",
            "DEFINE! (Bconstant, 0o0300);",
        ):
            self.assertIn(opcode_def, bytecode_src)
        cargo_toml = XTASK_CARGO_PATH.read_text(encoding="utf-8")
        self.assertNotIn("vm_direct_dispatch", bytecode_src)
        self.assertNotIn("vm_direct_dispatch", cargo_toml)
        self.assertFalse(
            VM_DISPATCH_RS_PATH.exists(),
            f"expected retired {VM_DISPATCH_RS_PATH} to be absent",
        )

    def test_TVM8_driver_passes_matching_artifacts_and_rejects_defects(self) -> None:
        """T-VM8: multi-artifact driver passes identical observations and rejects skips/mismatches."""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            cand = make_fake_ert_binary(root / "cand/emacs", gc_delta=3)
            base = make_fake_ert_binary(root / "base/emacs", gc_delta=9)
            ref = make_fake_ert_binary(root / "ref/emacs", gc_delta=1)
            (root / "cand/emacs.pdmp").write_bytes(b"cand-dump")
            (root / "base/emacs.pdmp").write_bytes(b"base-dump")
            (root / "ref/emacs.pdmp").write_bytes(b"ref-dump")

            # 1) Clean matching run across candidate, baseline, and reference (different gc_delta allowed)
            out1 = root / "out-pass"
            rc1 = main(
                [
                    "--candidate",
                    str(cand),
                    "--baseline",
                    str(base),
                    "--reference",
                    str(ref),
                    "--out",
                    str(out1),
                    "--timeout",
                    "30",
                ]
            )
            self.assertEqual(rc1, 0)
            rep1 = json.loads((out1 / "results.json").read_text(encoding="utf-8"))
            self.assertTrue(rep1["passed"])
            self.assertEqual(rep1["failures"], [])
            self.assertEqual(
                rep1["diagnostics"]["candidate"][EXPECTED_ERT_TESTS[0]]["gc_delta"], 3
            )
            self.assertEqual(
                rep1["diagnostics"]["baseline"][EXPECTED_ERT_TESTS[0]]["gc_delta"], 9
            )

            # 2) Reused --out directory must be rejected
            with self.assertRaises(ValueError):
                new_output(out1)
            rc_reused = main(
                [
                    "--candidate",
                    str(cand),
                    "--baseline",
                    str(base),
                    "--out",
                    str(out1),
                ]
            )
            self.assertNotEqual(rc_reused, 0)

            # 3) Skipped test must fail strict gate
            cand_skip = make_fake_ert_binary(
                root / "cand-skip/emacs", skip_test=EXPECTED_ERT_TESTS[2]
            )
            rc_skip = main(
                [
                    "--candidate",
                    str(cand_skip),
                    "--baseline",
                    str(base),
                    "--out",
                    str(root / "out-skip"),
                ]
            )
            self.assertEqual(rc_skip, 1)

            # 4) Missing expected test identity must fail
            cand_omit = make_fake_ert_binary(
                root / "cand-omit/emacs", omit_test=EXPECTED_ERT_TESTS[-1]
            )
            rc_omit = main(
                [
                    "--candidate",
                    str(cand_omit),
                    "--baseline",
                    str(base),
                    "--out",
                    str(root / "out-omit"),
                ]
            )
            self.assertEqual(rc_omit, 1)

            # 5) Passing ERT counts with diverging deterministic observation must fail
            cand_diff_obs = make_fake_ert_binary(
                root / "cand-diff/emacs",
                obs_override={EXPECTED_ERT_TESTS[0]: {"status": "tampered", "index": 999}},
            )
            out_diff = root / "out-diff"
            rc_diff = main(
                [
                    "--candidate",
                    str(cand_diff_obs),
                    "--baseline",
                    str(base),
                    "--out",
                    str(out_diff),
                ]
            )
            self.assertEqual(rc_diff, 1)
            rep_diff = json.loads((out_diff / "results.json").read_text(encoding="utf-8"))
            self.assertFalse(rep_diff["passed"])
            self.assertTrue(
                any("observation mismatch" in f for f in rep_diff["failures"])
            )

            # 6) Missing observation file must fail even if ERT output says 10/10 passed
            cand_no_obs = make_fake_ert_binary(
                root / "cand-no-obs/emacs", skip_obs_write=True
            )
            rc_no_obs = main(
                [
                    "--candidate",
                    str(cand_no_obs),
                    "--baseline",
                    str(base),
                    "--out",
                    str(root / "out-no-obs"),
                ]
            )
            self.assertEqual(rc_no_obs, 1)

    def test_TVM9_fingerprint_provenance_and_rehash_tamper_detection(self) -> None:
        """T-VM9: binary/dump/source hashes recorded and mid-run artifact mutation rejected."""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            cand_mut = make_fake_ert_binary(
                root / "cand-mut/emacs", mutate_self_on_exit=True
            )
            base = make_fake_ert_binary(root / "base/emacs")
            out_mut = root / "out-mut"
            rc = main(
                [
                    "--candidate",
                    str(cand_mut),
                    "--baseline",
                    str(base),
                    "--out",
                    str(out_mut),
                ]
            )
            self.assertEqual(rc, 1)
            rep = json.loads((out_mut / "results.json").read_text(encoding="utf-8"))
            self.assertFalse(rep["passed"])
            self.assertTrue(
                any(
                    "artifact fingerprint changed during run" in f
                    for f in rep["failures"]
                )
            )
            self.assertIsNotNone(rep["provenance"]["fixture_sha256"])
            self.assertIsNotNone(rep["provenance"]["bytecode_rs_sha256"])


def run_self_tests(json_out: Path | None = None) -> int:
    loader = unittest.TestLoader()
    suite = loader.loadTestsFromTestCase(StageIIIBytecodeRuntimeTests)
    cases: list[dict] = []
    overall_start = time.monotonic()

    class TimedResult(unittest.TextTestResult):
        def startTest(self, test: unittest.TestCase) -> None:
            self._case_started = time.monotonic()
            super().startTest(test)

        def addSuccess(self, test: unittest.TestCase) -> None:
            elapsed = time.monotonic() - self._case_started
            cases.append(
                {"id": test.id(), "status": "passed", "seconds": round(elapsed, 6)}
            )
            super().addSuccess(test)

        def addFailure(self, test: unittest.TestCase, err) -> None:
            elapsed = time.monotonic() - self._case_started
            cases.append(
                {
                    "id": test.id(),
                    "status": "failed",
                    "seconds": round(elapsed, 6),
                    "detail": self._exc_info_to_string(err, test),
                }
            )
            super().addFailure(test, err)

        def addError(self, test: unittest.TestCase, err) -> None:
            elapsed = time.monotonic() - self._case_started
            cases.append(
                {
                    "id": test.id(),
                    "status": "error",
                    "seconds": round(elapsed, 6),
                    "detail": self._exc_info_to_string(err, test),
                }
            )
            super().addError(test, err)

        def addSkip(self, test: unittest.TestCase, reason: str) -> None:
            elapsed = time.monotonic() - self._case_started
            cases.append(
                {
                    "id": test.id(),
                    "status": "skipped",
                    "seconds": round(elapsed, 6),
                    "detail": reason,
                }
            )
            super().addSkip(test, reason)

    runner = unittest.TextTestRunner(
        stream=sys.stdout, verbosity=2, resultclass=TimedResult
    )
    result = runner.run(suite)
    total_elapsed = time.monotonic() - overall_start
    if json_out is not None:
        json_out.parent.mkdir(parents=True, exist_ok=True)
        payload = {
            "suite": "test-bytecode-runtime",
            "ran": result.testsRun,
            "passed": result.wasSuccessful() and len(result.skipped) == 0,
            "failures": len(result.failures),
            "errors": len(result.errors),
            "skipped": len(result.skipped),
            "elapsed_seconds": round(total_elapsed, 6),
            "cases": cases,
        }
        json_out.write_text(
            json.dumps(payload, ensure_ascii=False, indent=2) + "\n", encoding="utf-8"
        )
    return 0 if (result.wasSuccessful() and len(result.skipped) == 0) else 1


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--candidate", type=Path, help="Candidate editor binary path")
    ap.add_argument("--baseline", type=Path, help="Baseline editor binary path")
    ap.add_argument("--reference", type=Path, help="Optional reference binary path")
    ap.add_argument("--out", type=Path, help="Fresh output directory for evidence")
    ap.add_argument("--timeout", type=int, default=120, help="Per-process timeout in seconds")
    ap.add_argument(
        "--self-test",
        action="store_true",
        help="Run partner unit/contract self-tests (T-VM1..T-VM9)",
    )
    ap.add_argument(
        "--json-out",
        type=Path,
        help="Optional JSON output path when running --self-test",
    )
    args = ap.parse_args(argv)

    if args.self_test or (
        args.candidate is None and args.baseline is None and args.out is None
    ):
        return run_self_tests(args.json_out)

    if args.candidate is None or args.baseline is None or args.out is None:
        print(
            "error: --candidate, --baseline, and --out are required for artifact comparison",
            file=sys.stderr,
        )
        return 2
    if args.timeout <= 0:
        print("error: --timeout must be positive", file=sys.stderr)
        return 2
    try:
        report = run_artifact_suite(
            candidate=args.candidate,
            baseline=args.baseline,
            out_dir=args.out,
            timeout=args.timeout,
            reference=args.reference,
        )
    except ValueError as exc:
        print(f"error: {exc}", file=sys.stderr)
        return 2

    status_str = "PASS" if report["passed"] else "FAIL"
    print(
        f"bytecode-runtime [{status_str}]: {len(EXPECTED_ERT_TESTS)} test(s), "
        f"failures={len(report['failures'])}, out={args.out.resolve() / 'results.json'}"
    )
    for failure in report["failures"]:
        print(f"  - {failure}", file=sys.stderr)
    return 0 if report["passed"] else 1


if __name__ == "__main__":
    sys.exit(main())
