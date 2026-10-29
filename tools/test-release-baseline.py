#!/usr/bin/env python3
"""Independent partner-owned Stage IV release baseline & profile contract test driver.

Covers Stage IV requirements IV-D4, IV-A2, and IV-A3 in docs/modernization.md:
1. Pure component & schema contract tests for `tools/runtime-profile.py` (`parse_profile`,
   `summarize`, `new_output`) covering `femacs-runtime-profile-v1`, the 6 required cases
   (`load`, `edit`, `regex`, `bytecode`, `gc-churn`, `gc-retained`), invalid markers,
   missing/duplicate/unknown cases, invalid timings/counts/pauses/checksums, median/MAD/p95
   aggregation, and output directory exclusivity / evidence preservation.
2. Strict ERT batch execution of `tools/fixtures/release-baseline.el` across terminal
   (`build/terminal/port/src/emacs`) and native Cocoa (`build/macos/Femacs.app/Contents/MacOS/Femacs`)
   products (plus optional `--reference`) via `tools/check-ert.py`, requiring the exact 10
   expected ERT test IDs, zero skips, zero unexpected results, per-case timings, before/after
   artifact SHA256 fingerprints, and identical deterministic cross-product observations.
"""

from __future__ import annotations

import argparse
from dataclasses import asdict
import copy
import hashlib
import importlib.util
import json
import math
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import time
import unittest

REPO = Path(__file__).resolve().parent.parent
CHECK_ERT_PATH = REPO / "tools/check-ert.py"
RUNTIME_PROFILE_PATH = REPO / "tools/runtime-profile.py"
FIXTURES_DIR = REPO / "tools/fixtures"
FIXTURE_NAME = "release-baseline.el"
FIXTURE_PATH = FIXTURES_DIR / FIXTURE_NAME

DEFAULT_TERMINAL = REPO / "build/terminal/port/src/emacs"
DEFAULT_MACOS = REPO / "dist/macos/Femacs.app"


def resolve_app_binary(app_or_bin: Path) -> Path:
    resolved = Path(app_or_bin).resolve()
    if resolved.is_dir():
        candidate = resolved / "Contents/MacOS/Femacs"
        if candidate.is_file():
            return candidate
    if not resolved.exists():
        fallback = REPO / "build/macos/Femacs.app/Contents/MacOS/Femacs"
        if fallback.is_file():
            return fallback
    return resolved

PROFILE_SCHEMA = "femacs-runtime-profile-v1"
OBSERVATION_SCHEMA = "femacs-release-baseline-observations-v1"

PROFILE_CASES: tuple[str, ...] = (
    "load",
    "edit",
    "regex",
    "bytecode",
    "gc-churn",
    "gc-retained",
)

EXPECTED_PROFILE_RESULTS: dict[str, str] = {
    "load": "5",
    "edit": "1200",
    "regex": "1000",
    "bytecode": "1249975000",
    "gc-churn": "30000",
    "gc-retained": "10000",
}

EXPECTED_ERT_TESTS: tuple[str, ...] = (
    "rb01-user-config-init-and-custom-roundtrip",
    "rb02-shipped-libraries-cl-seq-subr-x-and-rx",
    "rb03-shipped-modes-outline-tabulated-list-and-dired",
    "rb04-shipped-tooling-compile-diff-and-electric-pair",
    "rb05-unicode-coding-eol-and-file-roundtrip",
    "rb06-search-regexp-replace-narrowing-and-case-fold",
    "rb07-undo-redo-boundaries-and-atomic-change-group",
    "rb08-markers-overlays-text-properties-and-registers",
    "rb09-errors-condition-case-unwind-protect-and-catch-throw",
    "rb10-gc-live-roots-weak-hash-and-closures",
)

HEAP_CATEGORIES: tuple[tuple[str, int], ...] = (
    ("conses", 16),
    ("symbols", 48),
    ("strings", 24),
    ("string-bytes", 1),
    ("vectors", 16),
    ("vector-slots", 8),
    ("floats", 8),
    ("intervals", 56),
    ("buffers", 960),
)


def load_module(name: str, path: Path):
    """Dynamically load a Python module from path."""
    if not path.is_file():
        raise FileNotFoundError(f"missing module file: {path}")
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load module spec from {path}")
    mod = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = mod
    spec.loader.exec_module(mod)
    return mod


def load_check_ert():
    return load_module("check_ert", CHECK_ERT_PATH)


def load_runtime_profile():
    return load_module("runtime_profile", RUNTIME_PROFILE_PATH)


def sha256_file(path: Path) -> str | None:
    if not path.is_file():
        return None
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(65536), b""):
            h.update(chunk)
    return h.hexdigest()


def discover_dump_paths(binary: Path) -> list[Path]:
    """Return candidate .pdmp files associated with a terminal or .app binary."""
    resolved = binary.resolve()
    parent = resolved.parent
    candidates = [
        parent / f"{resolved.name}.pdmp",
        parent / "emacs.pdmp",
        parent / "bootstrap-emacs.pdmp",
    ]
    # Owned bundles put the dump in Resources/libexec; some installed stable
    # bundles put it next to the real executable under MacOS/libexec.
    if parent.name == "MacOS" and parent.parent.name == "Contents":
        for libexec in [parent.parent / "Resources/libexec", parent / "libexec"]:
            if libexec.is_dir():
                candidates.extend(sorted(libexec.glob("*.pdmp")))
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
        "tool_sha256": sha256_file(Path(__file__).resolve()),
        "runtime_profile_sha256": sha256_file(RUNTIME_PROFILE_PATH),
    }


def checked_output_path(path: Path) -> Path:
    """Resolve and reject writes into the optional read-only reference."""
    resolved = Path(path).resolve()
    reference = (REPO / "emacs").resolve()
    if resolved == reference or reference in resolved.parents:
        raise ValueError("output must not be inside the read-only reference tree")
    return resolved


def new_output(path: Path) -> Path:
    """Create a fresh output directory; reject any pre-existing file or directory."""
    resolved = checked_output_path(path)
    if resolved.exists() or resolved.is_symlink():
        raise FileExistsError(f"refusing to reuse existing output path: {resolved}")
    resolved.mkdir(parents=True, exist_ok=False)
    return resolved


def elisp_quote_string(value: str) -> str:
    escaped = value.replace("\\", "\\\\").replace('"', '\\"')
    return f'"{escaped}"'


def load_observations(path: Path) -> tuple[dict, dict, list[str]]:
    """Load and validate a per-run release-baseline observation JSON file."""
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
    """Validate that an ERT RunResult passed all 10 expected tests with zero skips."""
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
        f"{k}={v}" for k, v in results.items() if str(v).lower() != "passed"
    )
    if non_passed:
        errors.append(f"{side}: non-passed tests: {non_passed}")
    timings = getattr(run_result, "test_seconds", {}) or {}
    for test_id in EXPECTED_ERT_TESTS:
        sec = timings.get(test_id)
        if not isinstance(sec, (int, float)) or isinstance(sec, bool) or not math.isfinite(sec) or sec < 0:
            errors.append(f"{side}: missing or invalid duration for {test_id}: {sec!r}")
    return errors


def make_synthetic_heap(offset: int = 0) -> list[list]:
    return [
        [name, size, 1000 + idx * 100 + offset, 50 + idx * 5]
        for idx, (name, size) in enumerate(HEAP_CATEGORIES)
    ]


def make_synthetic_profile_cases() -> list[dict]:
    cases: list[dict] = []
    for idx, name in enumerate(PROFILE_CASES):
        has_gc = name in ("gc-churn", "gc-retained")
        pauses = [1.25 + idx * 0.1, 1.75 + idx * 0.1] if has_gc else []
        gc_count = len(pauses)
        gc_ms = round(sum(pauses), 4)
        cases.append(
            {
                "case": name,
                "wall_ms": round(12.5 + idx * 3.0, 4),
                "gc_count": gc_count,
                "gc_ms": gc_ms,
                "pauses_ms": pauses,
                "allocation_delta": [100 + idx * 10, 5, 200 + idx * 20, 10, 500, 12, 80],
                "heap_before": make_synthetic_heap(0),
                "heap_after": make_synthetic_heap(10 + idx),
                "result": EXPECTED_PROFILE_RESULTS[name],
            }
        )
    return cases


def format_profile_output(payload: dict, prefix_noise: str = "Loading runtime-profile...\n") -> str:
    return f"{prefix_noise}FEMACS-PROFILE {json.dumps(payload)}\n"


def format_synthetic_ert_log(
    test_ids: tuple[str, ...] = EXPECTED_ERT_TESTS,
    statuses: dict[str, str] | None = None,
) -> str:
    statuses = statuses or {}
    total = len(test_ids)
    lines = [f"Running {total} tests (2026-10-24 22:00:00+0800, selector `t')"]
    expected = unexpected = skipped = 0
    for idx, name in enumerate(test_ids, 1):
        st = statuses.get(name, "passed")
        lines.append(f"   {st}  {idx}/{total}  {name} (0.012345 sec)")
        if st.lower() == "skipped":
            skipped += 1
        elif st.isupper():
            unexpected += 1
        else:
            expected += 1
    skip_part = f", {skipped} skipped" if skipped else ""
    lines.append(f"Ran {total} tests, {expected} results as expected, {unexpected} unexpected{skip_part}")
    return "\n".join(lines) + "\n"


class ReleaseBaselineComponentTests(unittest.TestCase):
    """Deterministic component tests for runtime-profile.py and release-baseline validators."""

    @classmethod
    def setUpClass(cls):
        cls.check_ert = load_check_ert()
        cls.runtime_profile = load_runtime_profile()

    def _assert_rejected(self, fn, *args, **kwargs):
        with self.assertRaises((ValueError, KeyError, TypeError, RuntimeError, OSError)):
            fn(*args, **kwargs)

    def test_parse_profile_valid_six_cases(self):
        cases = make_synthetic_profile_cases()
        text = format_profile_output({"schema": PROFILE_SCHEMA, "cases": cases})
        parsed = self.runtime_profile.parse_profile(text)
        self.assertEqual(len(parsed), 6)
        self.assertEqual([c["case"] for c in parsed], list(PROFILE_CASES))
        for item, orig in zip(parsed, cases):
            self.assertAlmostEqual(float(item["wall_ms"]), orig["wall_ms"], places=4)
            self.assertEqual(int(item["gc_count"]), orig["gc_count"])
            self.assertAlmostEqual(float(item["gc_ms"]), orig["gc_ms"], places=4)
            self.assertEqual(len(item["pauses_ms"]), orig["gc_count"])
            self.assertEqual(list(item["allocation_delta"]), orig["allocation_delta"])
            self.assertEqual(len(item["heap_before"]), len(HEAP_CATEGORIES))
            self.assertEqual(len(item["heap_after"]), len(HEAP_CATEGORIES))
            self.assertEqual(str(item["result"]), orig["result"])

    def test_parse_profile_rejects_invalid_markers_and_schema(self):
        cases = make_synthetic_profile_cases()
        valid_line = format_profile_output({"schema": PROFILE_SCHEMA, "cases": cases}, prefix_noise="")
        # Missing marker
        self._assert_rejected(self.runtime_profile.parse_profile, "no profile line here\n")
        # Duplicate markers
        self._assert_rejected(self.runtime_profile.parse_profile, valid_line + valid_line)
        # Malformed JSON
        self._assert_rejected(self.runtime_profile.parse_profile, "FEMACS-PROFILE {broken\n")
        # Wrong schema
        self._assert_rejected(
            self.runtime_profile.parse_profile,
            format_profile_output({"schema": "femacs-runtime-profile-v0", "cases": cases}),
        )

    def test_parse_profile_rejects_missing_duplicate_or_unknown_cases(self):
        cases = make_synthetic_profile_cases()
        # Missing 6th case
        self._assert_rejected(
            self.runtime_profile.parse_profile,
            format_profile_output({"schema": PROFILE_SCHEMA, "cases": cases[:-1]}),
        )
        # Duplicate case name
        dup_cases = copy.deepcopy(cases)
        dup_cases[-1]["case"] = "load"
        self._assert_rejected(
            self.runtime_profile.parse_profile,
            format_profile_output({"schema": PROFILE_SCHEMA, "cases": dup_cases}),
        )
        # Unknown case name
        unk_cases = copy.deepcopy(cases)
        unk_cases[-1]["case"] = "unknown-workload"
        self._assert_rejected(
            self.runtime_profile.parse_profile,
            format_profile_output({"schema": PROFILE_SCHEMA, "cases": unk_cases}),
        )

    def test_parse_profile_rejects_invalid_metrics_and_checksums(self):
        base = make_synthetic_profile_cases()

        # Negative wall_ms
        neg_wall = copy.deepcopy(base)
        neg_wall[0]["wall_ms"] = -1.0
        self._assert_rejected(
            self.runtime_profile.parse_profile,
            format_profile_output({"schema": PROFILE_SCHEMA, "cases": neg_wall}),
        )

        # Negative gc_count
        neg_gc = copy.deepcopy(base)
        neg_gc[0]["gc_count"] = -1
        self._assert_rejected(
            self.runtime_profile.parse_profile,
            format_profile_output({"schema": PROFILE_SCHEMA, "cases": neg_gc}),
        )

        # Negative pause in pauses_ms
        neg_pause = copy.deepcopy(base)
        neg_pause[4]["pauses_ms"] = [1.0, -0.5]
        neg_pause[4]["gc_ms"] = 0.5
        self._assert_rejected(
            self.runtime_profile.parse_profile,
            format_profile_output({"schema": PROFILE_SCHEMA, "cases": neg_pause}),
        )

        # Disagreement between sum(pauses_ms) and gc_ms
        mismatch_pause = copy.deepcopy(base)
        mismatch_pause[4]["gc_ms"] = mismatch_pause[4]["gc_ms"] + 5.0
        self._assert_rejected(
            self.runtime_profile.parse_profile,
            format_profile_output({"schema": PROFILE_SCHEMA, "cases": mismatch_pause}),
        )

        # Disagreement between len(pauses_ms) and gc_count
        mismatch_count = copy.deepcopy(base)
        mismatch_count[4]["gc_count"] = 5
        self._assert_rejected(
            self.runtime_profile.parse_profile,
            format_profile_output({"schema": PROFILE_SCHEMA, "cases": mismatch_count}),
        )

        # Wrong number of allocation_delta counters (must be 7)
        bad_alloc = copy.deepcopy(base)
        bad_alloc[0]["allocation_delta"] = [1, 2, 3]
        self._assert_rejected(
            self.runtime_profile.parse_profile,
            format_profile_output({"schema": PROFILE_SCHEMA, "cases": bad_alloc}),
        )

        # Empty or duplicate heap_before categories
        bad_heap = copy.deepcopy(base)
        bad_heap[0]["heap_before"] = []
        self._assert_rejected(
            self.runtime_profile.parse_profile,
            format_profile_output({"schema": PROFILE_SCHEMA, "cases": bad_heap}),
        )

        dup_heap = copy.deepcopy(base)
        dup_heap[0]["heap_before"][1][0] = dup_heap[0]["heap_before"][0][0]
        self._assert_rejected(
            self.runtime_profile.parse_profile,
            format_profile_output({"schema": PROFILE_SCHEMA, "cases": dup_heap}),
        )

        # Empty / invalid result checksum
        empty_res = copy.deepcopy(base)
        empty_res[0]["result"] = ""
        self._assert_rejected(
            self.runtime_profile.parse_profile,
            format_profile_output({"schema": PROFILE_SCHEMA, "cases": empty_res}),
        )

        wrong_res = copy.deepcopy(base)
        wrong_res[0]["result"] = "wrong-checksum"
        self._assert_rejected(
            self.runtime_profile.parse_profile,
            format_profile_output({"schema": PROFILE_SCHEMA, "cases": wrong_res}),
        )

    def test_summarize_statistics_and_row_preservation(self):
        base_cases = make_synthetic_profile_cases()
        multipliers = [1.0, 1.1, 0.9, 1.2, 1.0, 0.95, 1.5]
        rows: list[dict] = []
        for round_idx, mult in enumerate(multipliers, 1):
            for c in base_cases:
                item = copy.deepcopy(c)
                item["round"] = round_idx
                item["wall_ms"] = round(c["wall_ms"] * mult, 4)
                rows.append(item)
        snapshot_before = copy.deepcopy(rows)
        summary = self.runtime_profile.summarize(rows)
        # Input rows must be preserved without mutation
        self.assertEqual(rows, snapshot_before)
        self.assertEqual(len(summary), 6)
        by_case = {row["case"]: row for row in summary}
        self.assertEqual(set(by_case.keys()), set(PROFILE_CASES))
        required_keys = {
            "case",
            "n",
            "median_ms",
            "mad_ms",
            "p95_ms",
            "gc_count",
            "gc_ms",
            "median_pause_ms",
            "p95_pause_ms",
            "max_pause_ms",
        }
        for name in PROFILE_CASES:
            srow = by_case[name]
            self.assertTrue(required_keys.issubset(srow.keys()), f"missing keys in {srow}")
            self.assertNotIn("min_ms", srow)
            self.assertEqual(srow["n"], 7)
            self.assertGreater(srow["median_ms"], 0.0)
            self.assertGreaterEqual(srow["mad_ms"], 0.0)
            self.assertGreaterEqual(srow["p95_ms"], srow["median_ms"])
            if name in ("gc-churn", "gc-retained"):
                self.assertEqual(srow["gc_count"], 14)
                self.assertGreater(srow["gc_ms"], 0.0)
                self.assertIsNotNone(srow["median_pause_ms"])
                self.assertIsNotNone(srow["p95_pause_ms"])
                self.assertIsNotNone(srow["max_pause_ms"])
                self.assertGreaterEqual(srow["max_pause_ms"], srow["p95_pause_ms"])
                self.assertGreaterEqual(srow["p95_pause_ms"], srow["median_pause_ms"])
            else:
                self.assertEqual(srow["gc_count"], 0)
                self.assertEqual(srow["gc_ms"], 0.0)
                self.assertIsNone(srow["median_pause_ms"])
                self.assertIsNone(srow["p95_pause_ms"])
                self.assertIsNone(srow["max_pause_ms"])

        # Duplicate (case, round) must be rejected
        dup_rows = copy.deepcopy(rows)
        dup_rows.append(copy.deepcopy(rows[0]))
        self._assert_rejected(self.runtime_profile.summarize, dup_rows)

        # Empty input must be rejected
        self._assert_rejected(self.runtime_profile.summarize, [])

    def test_new_output_exclusivity_and_evidence_preservation(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            fresh_a = self.runtime_profile.new_output(root / "run-a")
            self.assertTrue(fresh_a.is_dir())
            marker = fresh_a / "evidence.json"
            marker.write_text('{"preserved": true}\n', encoding="utf-8")
            # Reusing existing directory must fail and preserve existing evidence
            self._assert_rejected(self.runtime_profile.new_output, fresh_a)
            self.assertEqual(marker.read_text(encoding="utf-8"), '{"preserved": true}\n')

            fresh_b = new_output(root / "run-b")
            self.assertTrue(fresh_b.is_dir())
            self._assert_rejected(new_output, fresh_b)

    def test_profile_inputs_support_both_bundle_dump_locations(self):
        with tempfile.TemporaryDirectory() as tmp:
            for directory in ("Resources/libexec", "MacOS/libexec"):
                contents = Path(tmp).resolve() / directory.replace("/", "-") / "Contents"
                binary = contents / "MacOS" / "editor"
                binary.parent.mkdir(parents=True)
                binary.write_bytes(b"program")
                lisp = contents / "Resources/lisp"
                lisp.mkdir(parents=True)
                dump = contents / directory / "editor.pdmp"
                dump.parent.mkdir(parents=True, exist_ok=True)
                dump.write_bytes(b"selected image")
                self.assertEqual(self.runtime_profile.profile_inputs(binary), (dump, lisp))

    def test_profile_inputs_preserve_versioned_executable_name(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp).resolve()
            binary = root / "src/editor-32.0.50.1"
            binary.parent.mkdir()
            binary.write_bytes(b"program")
            lisp = root / "lisp"
            lisp.mkdir()
            wrong = binary.with_suffix(".pdmp")
            wrong.write_bytes(b"wrong truncated image name")
            self._assert_rejected(self.runtime_profile.profile_inputs, binary)
            selected = binary.parent / (binary.name + ".pdmp")
            selected.write_bytes(b"correct image")
            self.assertEqual(self.runtime_profile.profile_inputs(binary), (selected, lisp))

    def test_profile_inputs_reject_ambiguity_and_bind_explicit_image(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp).resolve()
            binary = root / "Contents/MacOS/editor"
            binary.parent.mkdir(parents=True)
            binary.write_bytes(b"program")
            (root / "Contents/Resources/lisp").mkdir(parents=True)
            one = binary.parent / "editor.pdmp"
            two = root / "Contents/Resources/libexec/editor.pdmp"
            two.parent.mkdir()
            one.write_bytes(b"image one")
            two.write_bytes(b"image two")
            self._assert_rejected(self.runtime_profile.profile_inputs, binary)
            self.assertEqual(self.runtime_profile.profile_inputs(binary, two)[0], two)
            two.unlink()
            two.symlink_to(one)
            self.assertEqual(self.runtime_profile.profile_inputs(binary)[0], one)
            self._assert_rejected(self.runtime_profile.profile_inputs, binary, two.parent)

    def test_profile_artifacts_bind_tool_and_detect_library_drift(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            binary = root / "src/editor"
            binary.parent.mkdir()
            binary.write_bytes(b"program")
            (binary.parent / "editor.pdmp").write_bytes(b"image")
            lisp = root / "lisp/emacs-lisp"
            lisp.mkdir(parents=True)
            for name in ("byte-opt", "bytecomp", "macroexp", "cconv", "cl-extra"):
                (lisp / (name + ".el")).write_bytes(b"source")
            before = self.runtime_profile.artifacts(binary)
            self.assertEqual(before["tool"]["sha256"], sha256_file(RUNTIME_PROFILE_PATH))
            generated = lisp / "bytecomp.elc"
            generated.write_bytes(b"generated")
            after = self.runtime_profile.artifacts(binary)
            self.assertNotEqual(before, after)
            self.assertIn("lisp/bytecomp.elc", after)
            generated.unlink()
            self.assertEqual(self.runtime_profile.artifacts(binary), before)

    def test_ert_and_observation_validators(self):
        ok_log = format_synthetic_ert_log()
        parsed_ok = self.check_ert.parse_result(ok_log, 0, 0.25)
        self.assertEqual(validate_ert_run("terminal", parsed_ok), [])

        # Skipped test must be rejected
        skip_log = format_synthetic_ert_log(
            statuses={EXPECTED_ERT_TESTS[0]: "skipped"}
        )
        parsed_skip = self.check_ert.parse_result(skip_log, 0, 0.25)
        self.assertTrue(
            any("skipped" in e for e in validate_ert_run("terminal", parsed_skip))
        )

        # Failed test must be rejected
        fail_log = format_synthetic_ert_log(
            statuses={EXPECTED_ERT_TESTS[1]: "FAILED"}
        )
        parsed_fail = self.check_ert.parse_result(fail_log, 1, 0.25)
        self.assertTrue(validate_ert_run("terminal", parsed_fail))

        # Missing test ID must be rejected
        short_log = format_synthetic_ert_log(test_ids=EXPECTED_ERT_TESTS[:-1])
        parsed_short = self.check_ert.parse_result(short_log, 0, 0.20)
        self.assertTrue(
            any("mismatch" in e for e in validate_ert_run("terminal", parsed_short))
        )


def run_component_tests() -> dict:
    """Run the real suite lifecycle once, preserving each test's timing."""
    cases_report = []
    class TimedResult(unittest.TestResult):
        def startTest(self, test):
            self.started = time.monotonic()
            super().startTest(test)
        def stopTest(self, test):
            errors = [detail for item, detail in self.failures + self.errors if item is test]
            cases_report.append({"test": test.id().split(".")[-1], "passed": not errors,
                                 "elapsed_ms": round((time.monotonic()-self.started)*1000, 3),
                                 "error": "\n".join(errors)})
            super().stopTest(test)
    started = time.monotonic()
    result = TimedResult()
    suite = unittest.defaultTestLoader.loadTestsFromTestCase(ReleaseBaselineComponentTests)
    suite.run(result)
    return {"passed": result.wasSuccessful() and result.testsRun > 0,
            "ran": result.testsRun, "cases": cases_report,
            "failures": [f"{test}: {detail}" for test, detail in result.failures + result.errors],
            "elapsed_seconds": round(time.monotonic()-started, 4)}


def run_ert_suite(
    out_dir: Path,
    terminal: Path,
    macos: Path | None,
    reference: Path | None,
    timeout: int,
) -> dict:
    """Run tools/fixtures/release-baseline.el across terminal, macOS batch, and optional reference."""
    check_ert = load_check_ert()
    products: dict[str, Path] = {"terminal": terminal.resolve()}
    if macos is not None:
        products["macos"] = resolve_app_binary(macos)
    if reference is not None:
        products["reference"] = reference.resolve()

    errors: list[str] = []
    fingerprints_before: dict[str, dict] = {}
    for side, binary in products.items():
        if not binary.is_file() or not os.access(binary, os.X_OK):
            errors.append(f"missing executable for {side}: {binary}")
        else:
            fingerprints_before[side] = artifact_fingerprint(binary)
    if not FIXTURE_PATH.is_file():
        errors.append(f"missing ERT fixture: {FIXTURE_PATH}")
    if errors:
        return {"passed": False, "errors": errors, "runs": {}}

    ert_dir = out_dir / "ert"
    ert_dir.mkdir(parents=True, exist_ok=False)
    check_ert.EVIDENCE_ROOT = ert_dir

    runs: dict[str, dict] = {}
    observations_by_side: dict[str, dict] = {}
    diagnostics_by_side: dict[str, dict] = {}

    for side, binary in products.items():
        obs_path = out_dir / f"observations-{side}.json"
        prelude = (
            f"(setq femacs-release-baseline-observation-file "
            f"{elisp_quote_string(str(obs_path.resolve()))})"
        )
        run_res = check_ert.run(
            binary,
            FIXTURES_DIR,
            FIXTURE_NAME,
            timeout=timeout,
            selector="t",
            memory_mib=0,
            prelude=prelude,
        )
        side_errors = validate_ert_run(side, run_res)
        obs, diag, obs_errors = load_observations(obs_path)
        side_errors.extend(f"{side}: {e}" for e in obs_errors)
        errors.extend(side_errors)
        observations_by_side[side] = obs
        diagnostics_by_side[side] = diag
        runs[side] = {
            "ert": asdict(run_res),
            "observation_file": str(obs_path),
            "observations": obs,
            "diagnostics": diag,
            "errors": side_errors,
        }

    # Verify cross-product deterministic observation equality
    baseline_obs = observations_by_side.get("terminal", {})
    for side, obs in observations_by_side.items():
        if side == "terminal":
            continue
        for test_id in EXPECTED_ERT_TESTS:
            if obs.get(test_id) != baseline_obs.get(test_id):
                errors.append(
                    f"observation mismatch for {test_id} between terminal and {side}: "
                    f"{baseline_obs.get(test_id)!r} != {obs.get(test_id)!r}"
                )

    fingerprints_after: dict[str, dict] = {}
    for side, binary in products.items():
        after = artifact_fingerprint(binary)
        fingerprints_after[side] = after
        if after != fingerprints_before.get(side):
            errors.append(f"artifact fingerprint changed during ERT run for {side}")

    return {
        "passed": not errors,
        "expected_tests": list(EXPECTED_ERT_TESTS),
        "fingerprints_before": fingerprints_before,
        "fingerprints_after": fingerprints_after,
        "runs": runs,
        "errors": errors,
    }


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--binary",
        "--terminal",
        dest="binary",
        type=Path,
        default=DEFAULT_TERMINAL,
        help="Terminal product binary path",
    )
    parser.add_argument(
        "--app",
        "--macos",
        dest="app",
        type=Path,
        default=DEFAULT_MACOS,
        help="macOS bundle product executable path",
    )
    parser.add_argument(
        "--no-macos",
        "--no-app",
        dest="no_app",
        action="store_true",
        help="Skip macOS binary when running terminal-only verification",
    )
    parser.add_argument(
        "--reference",
        type=Path,
        default=None,
        help="Optional reference binary for differential verification",
    )
    parser.add_argument(
        "--timeout",
        type=int,
        default=300,
        help="Per-product ERT timeout in seconds",
    )
    parser.add_argument(
        "--out",
        type=Path,
        default=REPO / "build/acceptance/release-baseline" / str(time.monotonic_ns()),
        help="Fresh output directory (must not exist)",
    )
    parser.add_argument(
        "--json-out",
        type=Path,
        default=None,
        help="Optional additional JSON summary path",
    )
    parser.add_argument(
        "--self-test",
        "--component-only",
        dest="component_only",
        action="store_true",
        help="Run pure component contract tests only without starting compiled binaries",
    )
    args = parser.parse_args(argv)
    if args.timeout <= 0:
        parser.error("--timeout must be positive")

    if args.json_out is not None:
        args.json_out = checked_output_path(args.json_out)
    started = time.monotonic()
    out_dir = new_output(args.out)
    component_report = run_component_tests()
    runtime_report = None
    if not args.component_only:
        macos_bin = None if args.no_app else args.app
        runtime_report = run_ert_suite(
            out_dir=out_dir,
            terminal=args.binary,
            macos=macos_bin,
            reference=args.reference,
            timeout=args.timeout,
        )

    passed = component_report["passed"] and (
        True if runtime_report is None else runtime_report["passed"]
    )
    payload = {
        "schema": "femacs-release-baseline-report-v1",
        "passed": passed,
        "component_only": bool(args.component_only),
        "provenance": git_provenance(),
        "component_tests": component_report,
        "runtime_ert": runtime_report,
        "elapsed_seconds": round(time.monotonic() - started, 4),
    }
    results_path = out_dir / "results.json"
    results_path.write_text(json.dumps(payload, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    if args.json_out is not None:
        args.json_out.parent.mkdir(parents=True, exist_ok=True)
        args.json_out.write_text(
            json.dumps(payload, ensure_ascii=False, indent=2) + "\n", encoding="utf-8"
        )
    print(
        f"release-baseline: component={component_report['ran']} "
        f"({'pass' if component_report['passed'] else 'fail'})"
        + (
            ""
            if runtime_report is None
            else f", runtime_ert={'pass' if runtime_report['passed'] else 'fail'}"
        )
        + f"; JSON: {results_path}"
    )
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
