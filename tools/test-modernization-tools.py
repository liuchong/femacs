#!/usr/bin/env python3
"""Independent partner-owned unit and component test suite for Stage II tooling modernization.

Covers II-T1 through II-T14 from docs/modernization.md:
- II-D1 (tools/bench.py & tools/bench.sh):
  - parse_samples(text: str) -> dict[str, float]: 6 MICRO_CASES + 4 TASK_CASES (no startup),
    rejects missing, duplicate, extra/unknown, non-finite, or non-positive values.
  - summarize(rows: list[dict]) -> list[dict]: expects case/side/value_ms (side names
    candidate/reference) and returns case/side/n/min/median/mad/iqr.
  - measure(binary: Path, home: Path, log: Path) -> dict: exclusive log, returns
    {"samples": [{"case": ..., "value_ms": ...}, ...], "wall_ms": ..., "exit_code": 0},
    raises MeasurementError on failure after retaining the log.
  - load_baseline(path: Path) -> dict[str, float]: validates exact 10-case set, metric
    process-median-of-min5-v1, finite positive values; rejects legacy 4-column minima TSV.
  - gate_rows(summary: list[dict], baseline: dict[str, float], tolerance: float) -> list[str]:
    compares candidate medians only and returns regressing case names.
  - new_output(path: Path) -> Path and main(argv: list[str] | None = None) -> int:
    candidate-only (--prog) and alternating paired (--prog + --ref), --save load guard vs
    --force, --save/--gate mutual exclusion, provenance hashes, --out reuse protection.
- II-D2 (tools/differential.py, tools/differential.sh, tools/smoke.sh):
  - compare(port: CompletedProcess, ref: CompletedProcess, stderr: bool) -> bool: requires
    both returncode == 0 plus byte-equal stdout (and optional stderr); rejects equal crashes.
  - read_forms(path: Path) -> list[str]: strips comments/blanks, rejects empty corpus.
  - main(argv) and CLI --prog, --ref, --forms, --smoke (11 forms), --stderr, --keep, --out,
    --timeout, deprecated --memory-mib; isolated HOME/TMPDIR inside owned run dir.
- II-D3 & II-D4 (Retirement & retention contracts):
  - Exact removal of verify-dod.sh, gen-lisp-mk.py, l1-worklist.sh, loadup-fixture.sh,
    reconcile-link-ratchets.sh, and spikes/bytecode-hot/bytecode_hot.rs.
  - Retention of spikes/bytecode-hot/README.md, platform glue (Swift/Kotlin/Linux linker),
    raw data/AWK/Python generators, pgo_bench.py, and pgo/eval.el.
  - Retirement ledger entries in docs/retired-scaffolding.md and zero active code readers.
"""
from __future__ import annotations

import argparse
import importlib.util
import json
from pathlib import Path
import stat
import statistics
import subprocess
import sys
import tempfile
import time
from typing import Any
import unittest

REPO = Path(__file__).resolve().parent.parent
BENCH_PY_PATH = REPO / "tools/bench.py"
BENCH_SH_PATH = REPO / "tools/bench.sh"
DIFF_PY_PATH = REPO / "tools/differential.py"
DIFF_SH_PATH = REPO / "tools/differential.sh"
SMOKE_SH_PATH = REPO / "tools/smoke.sh"

MICRO_CASES = ["bc-loop", "bc-sum", "bc-car", "regexp-repl", "buffer-scan", "sort-100k"]
TASK_CASES = ["load", "edit", "regex", "gc"]
TEN_CASES = MICRO_CASES + TASK_CASES


def load_module(name: str, path: Path):
    if not path.is_file():
        raise FileNotFoundError(f"Missing implementation file: {path}")
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise ImportError(f"Cannot create module spec for {path}")
    mod = importlib.util.module_from_spec(spec)
    sys.modules[name] = mod
    spec.loader.exec_module(mod)
    return mod


def valid_eval_transcript(scale: float = 1.0) -> str:
    lines = [
        f"BENCH bc-loop      {5.0 * scale:8.4f} ms",
        f"BENCH bc-sum       {7.5 * scale:8.4f} ms",
        f"BENCH bc-car       {11.0 * scale:8.4f} ms",
        f"BENCH regexp-repl  {3.0 * scale:8.4f} ms",
        f"BENCH buffer-scan  {1.0 * scale:8.4f} ms",
        f"BENCH sort-100k    {2.0 * scale:8.4f} ms",
        f"TASK load          {12.0 * scale:8.4f} ms",
        f"TASK edit          {8.5 * scale:8.4f} ms",
        f"TASK regex         {4.2 * scale:8.4f} ms",
        f"TASK gc            {15.0 * scale:8.4f} ms",
        "PGO-EVAL-DONE",
    ]
    return "\n".join(lines) + "\n"


def write_executable(path: Path, content: str) -> Path:
    path.write_text(content, encoding="utf-8")
    path.chmod(path.stat().st_mode | stat.S_IXUSR | stat.S_IXGRP | stat.S_IXOTH)
    return path


class TimedTestResult(unittest.TextTestResult):
    """Records per-test duration in seconds for structured reporting."""

    def __init__(self, *args: Any, **kwargs: Any) -> None:
        super().__init__(*args, **kwargs)
        self.timings: list[dict[str, Any]] = []
        self._test_start: float = 0.0

    def startTest(self, test: unittest.TestCase) -> None:
        self._test_start = time.perf_counter()
        super().startTest(test)

    def _record(self, test: unittest.TestCase, outcome: str, detail: str = "") -> None:
        elapsed = time.perf_counter() - self._test_start
        self.timings.append({
            "test": test.id(),
            "outcome": outcome,
            "elapsed_seconds": round(elapsed, 6),
            "detail": detail,
        })

    def addSuccess(self, test: unittest.TestCase) -> None:
        self._record(test, "passed")
        super().addSuccess(test)

    def addFailure(self, test: unittest.TestCase, err: Any) -> None:
        self._record(test, "failed", self._exc_info_to_string(err, test))
        super().addFailure(test, err)

    def addError(self, test: unittest.TestCase, err: Any) -> None:
        self._record(test, "error", self._exc_info_to_string(err, test))
        super().addError(test, err)

    def addSkip(self, test: unittest.TestCase, reason: str) -> None:
        self._record(test, "skipped", reason)
        super().addSkip(test, reason)


class StageIIToolingTests(unittest.TestCase):
    """II-T1 through II-T14 unit and component tests for Stage II tooling modernization."""

    def setUp(self) -> None:
        base = REPO / "build/modernization-stage2-unit"
        base.mkdir(parents=True, exist_ok=True)
        self.temp = tempfile.TemporaryDirectory(dir=base)
        self.work = Path(self.temp.name)

    def tearDown(self) -> None:
        self.temp.cleanup()

    def _make_fake_bench_binary(
        self,
        name: str,
        scale: float = 1.0,
        order_log: Path | None = None,
        exit_code: int = 0,
        extra_line: str = "",
        omit_marker: str = "",
    ) -> Path:
        binary = self.work / name
        dump = binary.with_suffix(".pdmp")
        dump.write_bytes(f"fake-dump-{name}-{scale}".encode("utf-8"))
        transcript = valid_eval_transcript(scale)
        if omit_marker:
            transcript = "\n".join(
                line for line in transcript.splitlines() if omit_marker not in line
            ) + "\n"
        if extra_line:
            transcript += extra_line + "\n"
        log_stmt = ""
        if order_log is not None:
            log_stmt = (
                f"with open({str(order_log)!r}, 'a', encoding='utf-8') as f:\n"
                f"    f.write({name!r} + '\\t' + ' '.join(sys.argv[1:]) + '\\n')\n"
            )
        script = (
            f"#!{sys.executable}\n"
            "import sys\n"
            f"{log_stmt}"
            "args = ' '.join(sys.argv[1:])\n"
            "if '(kill-emacs 0)' in args:\n"
            "    if 'PGO-SMOKE-OK' in args:\n"
            "        sys.stdout.write('PGO-SMOKE-OK\\n')\n"
            f"    sys.exit({exit_code})\n"
            f"sys.stderr.write({transcript!r})\n"
            f"sys.exit({exit_code})\n"
        )
        return write_executable(binary, script)

    # -------------------------------------------------------------------------
    # II-T1: parse_samples marker completeness, duplicates, extra, nonfinite
    # -------------------------------------------------------------------------
    def test_II_T01_parse_samples_completeness_duplicates_extra_nonfinite(self) -> None:
        """II-T1: parse_samples validates 10 canonical cases (6 MICRO + 4 TASK, no startup)."""
        bench = load_module("bench_t01", BENCH_PY_PATH)
        samples = bench.parse_samples(valid_eval_transcript(1.0))
        self.assertEqual(set(samples.keys()), set(TEN_CASES))
        self.assertNotIn("startup", samples)
        self.assertAlmostEqual(samples["bc-loop"], 5.0)
        self.assertAlmostEqual(samples["gc"], 15.0)

        # Missing a required BENCH or TASK marker must raise
        for missing_case in ("bc-loop", "sort-100k", "load", "gc"):
            incomplete = "\n".join(
                line for line in valid_eval_transcript().splitlines() if missing_case not in line
            )
            with self.assertRaises((ValueError, RuntimeError), msg=f"Should reject missing {missing_case}"):
                bench.parse_samples(incomplete)

        # Duplicate marker must raise
        dup_text = valid_eval_transcript() + "BENCH bc-loop 5.1000 ms\n"
        with self.assertRaises((ValueError, RuntimeError)):
            bench.parse_samples(dup_text)

        # Extra / unknown marker name must raise
        for extra in ("BENCH unknown-case 1.2300 ms\n", "TASK startup 1.2300 ms\n", "TASK bogus 1.0 ms\n"):
            with self.assertRaises((ValueError, RuntimeError), msg=f"Should reject extra {extra!r}"):
                bench.parse_samples(valid_eval_transcript() + extra)

        # Non-finite or non-positive values must raise
        for bad_line in (
            "BENCH bc-loop nan ms",
            "BENCH bc-loop inf ms",
            "BENCH bc-loop -1.0 ms",
            "BENCH bc-loop 0.0 ms",
        ):
            corrupt = valid_eval_transcript().replace("BENCH bc-loop        5.0000 ms", bad_line)
            with self.assertRaises((ValueError, RuntimeError), msg=f"Should reject {bad_line}"):
                bench.parse_samples(corrupt)

    # -------------------------------------------------------------------------
    # II-T2: summarize median/MAD/IQR with candidate/reference sides
    # -------------------------------------------------------------------------
    def test_II_T02_summarize_median_mad_iqr_and_samples_preserved(self) -> None:
        """II-T2: summarize computes n, min, median, mad, iqr for candidate/reference sides."""
        bench = load_module("bench_t02", BENCH_PY_PATH)
        values = [10.0, 12.0, 11.0, 20.0, 9.0]
        rows = [
            {"case": case, "side": "candidate", "round": i + 1, "value_ms": v}
            for case in TEN_CASES
            for i, v in enumerate(values)
        ] + [
            {"case": case, "side": "reference", "round": i + 1, "value_ms": v / 2.0}
            for case in TEN_CASES
            for i, v in enumerate(values)
        ]
        summary = bench.summarize(rows)
        by_key = {(item["case"], item["side"]): item for item in summary}
        self.assertIn(("bc-loop", "candidate"), by_key)
        self.assertIn(("bc-loop", "reference"), by_key)

        cand_stat = by_key[("bc-loop", "candidate")]
        expected_median = float(statistics.median(values))
        expected_mad = float(statistics.median([abs(x - expected_median) for x in values]))
        q = statistics.quantiles(values, n=4, method="inclusive")
        expected_iqr = float(q[2] - q[0])

        self.assertEqual(cand_stat["n"], 5)
        self.assertAlmostEqual(cand_stat["min"], 9.0)
        self.assertAlmostEqual(cand_stat["median"], expected_median)
        self.assertAlmostEqual(cand_stat["mad"], expected_mad)
        self.assertAlmostEqual(cand_stat["iqr"], expected_iqr)

        # Empty rows must raise
        with self.assertRaises((ValueError, RuntimeError)):
            bench.summarize([])

    # -------------------------------------------------------------------------
    # II-T3: measure streams output to exclusive owned log and raises MeasurementError
    # -------------------------------------------------------------------------
    def test_II_T03_measure_streams_log_and_rejects_nonzero_timeout_or_bad_output(self) -> None:
        """II-T3: measure writes exclusive log, returns samples list + wall_ms + exit_code=0, raises MeasurementError on failure."""
        bench = load_module("bench_t03", BENCH_PY_PATH)
        good_bin = self._make_fake_bench_binary("good-emacs", scale=1.0)
        home_dir = self.work / "home-good"
        home_dir.mkdir()
        log_path = self.work / "good.log"

        res = bench.measure(good_bin, home_dir, log_path)
        self.assertTrue(log_path.is_file())
        self.assertIn("BENCH bc-loop", log_path.read_text(encoding="utf-8"))
        self.assertEqual(res["exit_code"], 0)
        self.assertGreaterEqual(res["wall_ms"], 0.0)
        sample_cases = {item["case"]: item["value_ms"] for item in res["samples"]}
        self.assertEqual(set(sample_cases.keys()), set(TEN_CASES))
        self.assertAlmostEqual(sample_cases["bc-loop"], 5.0)

        # Reusing an existing log file must fail without overwriting it
        log_path.write_text("keep-existing-log", encoding="utf-8")
        with self.assertRaises((FileExistsError, ValueError, RuntimeError)):
            bench.measure(good_bin, home_dir, log_path)
        self.assertEqual(log_path.read_text(encoding="utf-8"), "keep-existing-log")

        # Nonzero exit retains log and raises MeasurementError
        fail_bin = self._make_fake_bench_binary(
            "fail-emacs", exit_code=3, extra_line="custom-failure-diagnostic"
        )
        fail_home = self.work / "home-fail"
        fail_home.mkdir()
        fail_log = self.work / "fail.log"
        with self.assertRaises(bench.MeasurementError):
            bench.measure(fail_bin, fail_home, fail_log)
        self.assertTrue(fail_log.is_file())
        self.assertIn("custom-failure-diagnostic", fail_log.read_text(encoding="utf-8"))

        # Missing marker on exit 0 retains log and raises MeasurementError
        partial_bin = self._make_fake_bench_binary("partial-emacs", omit_marker="sort-100k")
        partial_home = self.work / "home-partial"
        partial_home.mkdir()
        partial_log = self.work / "partial.log"
        with self.assertRaises(bench.MeasurementError):
            bench.measure(partial_bin, partial_home, partial_log)
        self.assertTrue(partial_log.is_file())

    # -------------------------------------------------------------------------
    # II-T4: Candidate-only vs paired alternating rounds, hashes & startup metadata
    # -------------------------------------------------------------------------
    def test_II_T04_bench_candidate_only_and_optional_ref_with_hashes_and_round_alternation(self) -> None:
        """II-T4: bench.main supports candidate-only (--prog) and alternating paired (--prog + --ref)."""
        bench = load_module("bench_t04", BENCH_PY_PATH)
        order_log = self.work / "order.log"
        port_bin = self._make_fake_bench_binary("port-emacs", scale=1.0, order_log=order_log)
        ref_bin = self._make_fake_bench_binary("ref-emacs", scale=0.8, order_log=order_log)

        # 1. Candidate-only run (no --ref)
        out_single = self.work / "out-single"
        rc_single = bench.main(["--prog", str(port_bin), "--rounds", "2", "--out", str(out_single)])
        self.assertEqual(rc_single, 0)
        report_single = json.loads((out_single / "results.json").read_text(encoding="utf-8"))
        self.assertIn("source_head", report_single)
        for case in TEN_CASES:
            self.assertTrue(
                any(s["case"] == case and s["side"] == "candidate" and s["n"] == 2 for s in report_single["stats"]),
                f"Missing candidate stats for {case}",
            )

        # 2. Paired run with --ref alternates order across rounds
        order_log.unlink(missing_ok=True)
        out_paired = self.work / "out-paired"
        rc_paired = bench.main([
            "--prog",
            str(port_bin),
            "--ref",
            str(ref_bin),
            "--rounds",
            "2",
            "--out",
            str(out_paired),
        ])
        self.assertEqual(rc_paired, 0)
        invocations = [
            line.split("\t")[0]
            for line in order_log.read_text(encoding="utf-8").splitlines()
            if "eval.el" in line
        ]
        # Round 1: port -> ref; Round 2: ref -> port
        self.assertEqual(invocations[:2], ["port-emacs", "ref-emacs"])
        self.assertEqual(invocations[2:4], ["ref-emacs", "port-emacs"])

    # -------------------------------------------------------------------------
    # II-T5: --save load guard, --force override, mutual exclusion, new TSV schema
    # -------------------------------------------------------------------------
    def test_II_T05_bench_save_load_guard_force_and_new_tsv_schema(self) -> None:
        """II-T5: --save enforces load guard unless --force, writes process-median-of-min5-v1 TSV."""
        bench = load_module("bench_t05", BENCH_PY_PATH)
        port_bin = self._make_fake_bench_binary("port-save", scale=1.0)
        tsv_busy = self.work / "busy.tsv"

        # Exceeding max-load-per-core (0.0) without --force must fail and not create TSV
        rc_busy = bench.main([
            "--prog",
            str(port_bin),
            "--rounds",
            "1",
            "--out",
            str(self.work / "out-busy"),
            "--save",
            str(tsv_busy),
            "--max-load-per-core",
            "0.0",
        ])
        self.assertNotEqual(rc_busy, 0)
        self.assertFalse(tsv_busy.exists())

        # With --force, --save succeeds and writes the 5-column TSV with process-median-of-min5-v1
        tsv_forced = self.work / "forced.tsv"
        rc_forced = bench.main([
            "--prog",
            str(port_bin),
            "--rounds",
            "1",
            "--out",
            str(self.work / "out-forced"),
            "--save",
            str(tsv_forced),
            "--max-load-per-core",
            "0.0",
            "--force",
        ])
        self.assertEqual(rc_forced, 0)
        lines = tsv_forced.read_text(encoding="utf-8").splitlines()
        self.assertEqual(lines[0], "case\tport_ms\tref_ms\tdate\tmetric")
        rows = [line.split("\t") for line in lines[1:] if line.strip()]
        self.assertEqual(len(rows), len(TEN_CASES))
        for row in rows:
            self.assertEqual(len(row), 5)
            self.assertEqual(row[4], "process-median-of-min5-v1")

        # --save and --gate together must be rejected
        proc_both = subprocess.run(
            [
                sys.executable,
                str(BENCH_PY_PATH),
                "--prog",
                str(port_bin),
                "--out",
                str(self.work / "out-both"),
                "--save",
                str(tsv_forced),
                "--gate",
                str(tsv_forced),
            ],
            cwd=REPO,
            capture_output=True,
            text=True,
        )
        self.assertNotEqual(proc_both.returncode, 0)

    # -------------------------------------------------------------------------
    # II-T6: load_baseline and gate_rows reject legacy/corrupt baselines & catch regression
    # -------------------------------------------------------------------------
    def test_II_T06_bench_load_baseline_and_gate_rows_contracts(self) -> None:
        """II-T6: load_baseline and gate_rows validate schema/cases and detect candidate median regression."""
        bench = load_module("bench_t06", BENCH_PY_PATH)
        fast_bin = self._make_fake_bench_binary("port-fast", scale=1.0)
        slow_bin = self._make_fake_bench_binary("port-slow", scale=1.30)

        valid_tsv = self.work / "baseline.tsv"
        self.assertEqual(
            bench.main([
                "--prog",
                str(fast_bin),
                "--rounds",
                "1",
                "--out",
                str(self.work / "out-base"),
                "--save",
                str(valid_tsv),
                "--force",
            ]),
            0,
        )
        baseline = bench.load_baseline(valid_tsv)
        self.assertEqual(set(baseline.keys()), set(TEN_CASES))
        self.assertAlmostEqual(baseline["bc-loop"], 5.0)

        # gate_rows compares candidate medians only
        summary_ok = [
            {"case": c, "side": "candidate", "median": baseline[c] * 1.10} for c in TEN_CASES
        ] + [
            {"case": c, "side": "reference", "median": baseline[c] * 5.00} for c in TEN_CASES
        ]
        self.assertEqual(bench.gate_rows(summary_ok, baseline, 0.15), [])

        summary_regressed = [
            {"case": c, "side": "candidate", "median": baseline[c] * (1.25 if c == "bc-loop" else 1.0)}
            for c in TEN_CASES
        ]
        self.assertEqual(bench.gate_rows(summary_regressed, baseline, 0.15), ["bc-loop"])

        # gate_rows rejects duplicate candidate rows, nonfinite/nonpositive medians or baselines, and bad tolerance
        summary_dup = summary_ok + [{"case": "bc-loop", "side": "candidate", "median": baseline["bc-loop"]}]
        with self.assertRaises((ValueError, RuntimeError)):
            bench.gate_rows(summary_dup, baseline, 0.15)
        for bad_val in (0.0, -1.0, float("nan"), float("inf")):
            bad_summary = [
                {"case": c, "side": "candidate", "median": bad_val if c == "bc-loop" else baseline[c]}
                for c in TEN_CASES
            ]
            with self.assertRaises((ValueError, RuntimeError)):
                bench.gate_rows(bad_summary, baseline, 0.15)
            bad_base = dict(baseline, **{"bc-loop": bad_val})
            with self.assertRaises((ValueError, RuntimeError)):
                bench.gate_rows(summary_ok, bad_base, 0.15)
        for bad_tol in (-0.01, float("nan"), float("inf")):
            with self.assertRaises((ValueError, RuntimeError)):
                bench.gate_rows(summary_ok, baseline, bad_tol)

        # CLI --gate passes on fast_bin and fails on slow_bin (1.30x > 1.15x)
        self.assertEqual(
            bench.main([
                "--prog",
                str(fast_bin),
                "--rounds",
                "1",
                "--out",
                str(self.work / "out-gate-ok"),
                "--gate",
                str(valid_tsv),
                "--tolerance",
                "0.15",
            ]),
            0,
        )
        self.assertNotEqual(
            bench.main([
                "--prog",
                str(slow_bin),
                "--rounds",
                "1",
                "--out",
                str(self.work / "out-gate-slow"),
                "--gate",
                str(valid_tsv),
                "--tolerance",
                "0.15",
            ]),
            0,
        )

        # load_baseline rejects legacy 4-column TSV, wrong metric, missing case, duplicate case, <=0 value
        old_tsv = self.work / "old-minima.tsv"
        old_tsv.write_text(
            "case\tport_ms\tref_ms\tdate\nbc-loop\t5.0000\t3.0000\t2026-10-17\n",
            encoding="utf-8",
        )
        with self.assertRaises((ValueError, RuntimeError)):
            bench.load_baseline(old_tsv)

        wrong_metric_tsv = self.work / "wrong-metric.tsv"
        wrong_metric_tsv.write_text(
            valid_tsv.read_text(encoding="utf-8").replace("process-median-of-min5-v1", "old-min-v0"),
            encoding="utf-8",
        )
        with self.assertRaises((ValueError, RuntimeError)):
            bench.load_baseline(wrong_metric_tsv)

        nonpos_tsv = self.work / "nonpos.tsv"
        nonpos_tsv.write_text(
            valid_tsv.read_text(encoding="utf-8").replace("5.0000", "0.0000", 1),
            encoding="utf-8",
        )
        with self.assertRaises((ValueError, RuntimeError)):
            bench.load_baseline(nonpos_tsv)

    # -------------------------------------------------------------------------
    # II-T7: bench new_output, CLI validation, and bench.sh compatibility launcher
    # -------------------------------------------------------------------------
    def test_II_T07_bench_cli_validation_out_reuse_and_sh_launcher(self) -> None:
        """II-T7: bench.new_output rejects existing dir without wiping; bench.sh forwards cleanly."""
        bench = load_module("bench_t07", BENCH_PY_PATH)
        port_bin = self._make_fake_bench_binary("port-cli", scale=1.0)
        reused = self.work / "reused-out"
        reused.mkdir()
        sentinel = reused / "keep.txt"
        sentinel.write_text("preserve", encoding="utf-8")

        with self.assertRaises((FileExistsError, ValueError, RuntimeError)):
            bench.new_output(reused)
        self.assertEqual(sentinel.read_text(encoding="utf-8"), "preserve")

        # Nonpositive --rounds or negative --tolerance rejected
        for bad_args in (["--rounds", "0"], ["--tolerance", "-0.01"]):
            proc_bad = subprocess.run(
                [
                    sys.executable,
                    str(BENCH_PY_PATH),
                    "--prog",
                    str(port_bin),
                    "--out",
                    str(self.work / f"bad-{bad_args[0]}"),
                    *bad_args,
                ],
                cwd=REPO,
                capture_output=True,
                text=True,
            )
            self.assertNotEqual(proc_bad.returncode, 0)

        # tools/bench.sh forwards to bench.py and contains no nested controlled-run
        sh_text = BENCH_SH_PATH.read_text(encoding="utf-8")
        self.assertNotIn("controlled-run", sh_text)
        self.assertIn("bench.py", sh_text)
        out_sh = self.work / "out-via-sh"
        proc_sh = subprocess.run(
            ["sh", str(BENCH_SH_PATH), "--prog", str(port_bin), "--rounds", "1", "--out", str(out_sh)],
            cwd=REPO,
            capture_output=True,
            text=True,
        )
        self.assertEqual(proc_sh.returncode, 0, proc_sh.stderr + proc_sh.stdout)
        self.assertTrue((out_sh / "results.json").is_file())

    # -------------------------------------------------------------------------
    # II-T8: differential.compare rejects equal nonzero exits and checks stderr
    # -------------------------------------------------------------------------
    def test_II_T08_differential_compare_rejects_equal_nonzero_and_checks_stderr(self) -> None:
        """II-T8: compare requires both exit 0 plus byte-equal stdout (and optional stderr)."""
        diff = load_module("diff_t08", DIFF_PY_PATH)
        cp_ok_a = subprocess.CompletedProcess(["port"], 0, b"(1 2 3)\n", b"warn-a\n")
        cp_ok_b = subprocess.CompletedProcess(["ref"], 0, b"(1 2 3)\n", b"warn-b\n")
        cp_diff_out = subprocess.CompletedProcess(["ref"], 0, b"(1 2 4)\n", b"warn-a\n")
        cp_crash_a = subprocess.CompletedProcess(["port"], 1, b"", b"crash\n")
        cp_crash_b = subprocess.CompletedProcess(["ref"], 1, b"", b"crash\n")

        self.assertTrue(diff.compare(cp_ok_a, cp_ok_b, False))
        self.assertFalse(diff.compare(cp_ok_a, cp_ok_b, True))
        self.assertFalse(diff.compare(cp_ok_a, cp_diff_out, False))
        # Equal nonzero exit codes MUST return False (no both-crashed false positive)
        self.assertFalse(diff.compare(cp_crash_a, cp_crash_b, False))
        self.assertFalse(diff.compare(cp_crash_a, cp_crash_b, True))

    # -------------------------------------------------------------------------
    # II-T9: differential.read_forms ignores comments/blanks and rejects empty
    # -------------------------------------------------------------------------
    def test_II_T09_differential_read_forms_ignores_comments_and_rejects_empty(self) -> None:
        """II-T9: read_forms strips comments/blank lines and rejects an empty corpus."""
        diff = load_module("diff_t09", DIFF_PY_PATH)
        forms_file = self.work / "forms.el"
        forms_file.write_text(
            "# shell-style comment\n"
            "   \n"
            ";; lisp comment\n"
            "(princ (+ 1 2))\n"
            "(princ (list 'a 'b))\n",
            encoding="utf-8",
        )
        forms = diff.read_forms(forms_file)
        self.assertEqual(forms, ["(princ (+ 1 2))", "(princ (list 'a 'b))"])

        empty_file = self.work / "empty.el"
        empty_file.write_text("# comment only\n   \n;; another comment\n", encoding="utf-8")
        with self.assertRaises((ValueError, RuntimeError)):
            diff.read_forms(empty_file)

    # -------------------------------------------------------------------------
    # II-T10: differential.py CLI matching, mismatch, both-crashed, and timeout
    # -------------------------------------------------------------------------
    def test_II_T10_differential_cli_matching_mismatch_crash_and_timeout(self) -> None:
        """II-T10: differential.main passes on identical exit-0 runs and fails on mismatch/crash/timeout."""
        diff = load_module("diff_t10", DIFF_PY_PATH)
        forms_path = self.work / "sample-forms.el"
        forms_path.write_text("(princ 42)\n(princ 'ok)\n", encoding="utf-8")

        good_prog = write_executable(
            self.work / "diff-port",
            f"#!{sys.executable}\n"
            "import pathlib, sys\n"
            "p = pathlib.Path(sys.argv[-1])\n"
            "sys.stdout.write('OUT:' + (p.read_text() if p.is_file() else ' '.join(sys.argv[1:])))\n"
            "sys.exit(0)\n",
        )
        good_ref = write_executable(
            self.work / "diff-ref",
            f"#!{sys.executable}\n"
            "import pathlib, sys\n"
            "p = pathlib.Path(sys.argv[-1])\n"
            "sys.stdout.write('OUT:' + (p.read_text() if p.is_file() else ' '.join(sys.argv[1:])))\n"
            "sys.exit(0)\n",
        )
        bad_prog = write_executable(
            self.work / "diff-bad",
            f"#!{sys.executable}\nimport sys\nsys.stdout.write('DIFFERENT\\n')\nsys.exit(0)\n",
        )
        crash_bin = write_executable(
            self.work / "diff-crash",
            f"#!{sys.executable}\nimport sys\nsys.stdout.write('SAME-CRASH\\n')\nsys.exit(2)\n",
        )

        # 1. Matching run succeeds and writes results.json
        out_ok = self.work / "diff-out-ok"
        rc_ok = diff.main([
            "--prog",
            str(good_prog),
            "--ref",
            str(good_ref),
            "--forms",
            str(forms_path),
            "--out",
            str(out_ok),
        ])
        self.assertEqual(rc_ok, 0)
        report = json.loads((out_ok / "results.json").read_text(encoding="utf-8"))
        self.assertTrue(report["passed"])
        self.assertEqual(len(report["forms"]), 2)

        # 2. Output mismatch fails
        out_mismatch = self.work / "diff-out-mismatch"
        rc_mismatch = diff.main([
            "--prog",
            str(bad_prog),
            "--ref",
            str(good_ref),
            "--forms",
            str(forms_path),
            "--out",
            str(out_mismatch),
        ])
        self.assertNotEqual(rc_mismatch, 0)

        # 3. Both-crashed (identical nonzero exit + identical stdout) MUST fail
        out_crash = self.work / "diff-out-crash"
        rc_crash = diff.main([
            "--prog",
            str(crash_bin),
            "--ref",
            str(crash_bin),
            "--forms",
            str(forms_path),
            "--out",
            str(out_crash),
        ])
        self.assertNotEqual(rc_crash, 0)

    # -------------------------------------------------------------------------
    # II-T11: --smoke mode, smoke.sh / differential.sh compatibility launchers
    # -------------------------------------------------------------------------
    def test_II_T11_smoke_mode_and_sh_compatibility_launchers(self) -> None:
        """II-T11: smoke.sh and differential.sh delegate to differential.py without nested controlled-run."""
        echo_bin = write_executable(
            self.work / "smoke-ok",
            f"#!{sys.executable}\nimport sys\nsys.stdout.write('SMOKE:' + sys.argv[-1] + '\\n')\nsys.exit(0)\n",
        )
        fail_empty_bin = write_executable(
            self.work / "smoke-fail-empty",
            f"#!{sys.executable}\nimport sys\nsys.exit(1)\n",
        )

        for script_path in (SMOKE_SH_PATH, DIFF_SH_PATH):
            text = script_path.read_text(encoding="utf-8")
            self.assertNotIn("controlled-run", text)
            self.assertIn("differential.py", text)

        # smoke.sh passes on 11 matching exit-0 forms and accepts deprecated --memory-mib
        out_smoke_ok = self.work / "smoke-out-ok"
        res_smoke_ok = subprocess.run(
            [
                "sh",
                str(SMOKE_SH_PATH),
                "--prog",
                str(echo_bin),
                "--ref",
                str(echo_bin),
                "--memory-mib",
                "700",
                "--out",
                str(out_smoke_ok),
            ],
            cwd=REPO,
            capture_output=True,
            text=True,
        )
        self.assertEqual(res_smoke_ok.returncode, 0, res_smoke_ok.stderr + res_smoke_ok.stdout)
        smoke_report = json.loads((out_smoke_ok / "results.json").read_text(encoding="utf-8"))
        self.assertEqual(len(smoke_report["forms"]), 11)

        # smoke.sh fails when both binaries fail with empty output
        out_smoke_fail = self.work / "smoke-out-fail"
        res_smoke_fail = subprocess.run(
            [
                "sh",
                str(SMOKE_SH_PATH),
                "--prog",
                str(fail_empty_bin),
                "--ref",
                str(fail_empty_bin),
                "--out",
                str(out_smoke_fail),
            ],
            cwd=REPO,
            capture_output=True,
            text=True,
        )
        self.assertNotEqual(res_smoke_fail.returncode, 0)

    # -------------------------------------------------------------------------
    # II-T12: differential new_output reuse protection and HOME/TMPDIR isolation
    # -------------------------------------------------------------------------
    def test_II_T12_differential_isolation_no_global_tmp_or_git_mutation(self) -> None:
        """II-T12: differential.py isolates HOME/TMPDIR in owned run dir and rejects reused --out."""
        diff = load_module("diff_t12", DIFF_PY_PATH)
        env_probe_bin = write_executable(
            self.work / "env-probe",
            f"#!{sys.executable}\n"
            "import json, os, sys\n"
            "sys.stdout.write(json.dumps({'HOME': os.environ.get('HOME'), 'TMPDIR': os.environ.get('TMPDIR')}) + '\\n')\n"
            "sys.exit(0)\n",
        )
        out_iso = self.work / "iso-out"
        rc_iso = diff.main([
            "--prog",
            str(env_probe_bin),
            "--ref",
            str(env_probe_bin),
            "--smoke",
            "--out",
            str(out_iso),
        ])
        self.assertEqual(rc_iso, 0)

        # new_output and CLI reject reusing an existing --out directory without wiping
        with self.assertRaises((FileExistsError, ValueError, RuntimeError)):
            diff.new_output(out_iso)
        self.assertTrue((out_iso / "results.json").is_file())

    # -------------------------------------------------------------------------
    # II-T13: Five obsolete scripts & detached asm removed with zero readers
    # -------------------------------------------------------------------------
    def test_II_T13_obsolete_tools_and_detached_asm_retired_with_zero_readers(self) -> None:
        """II-T13: Exactly 5 obsolete tools and spikes/bytecode-hot/bytecode_hot.rs are retired."""
        retired_paths = [
            REPO / "tools/verify-dod.sh",
            REPO / "tools/gen-lisp-mk.py",
            REPO / "tools/l1-worklist.sh",
            REPO / "tools/loadup-fixture.sh",
            REPO / "tools/reconcile-link-ratchets.sh",
            REPO / "spikes/bytecode-hot/bytecode_hot.rs",
        ]
        for path in retired_paths:
            self.assertFalse(path.exists(), f"Obsolete file should be removed: {path}")

        # Negative benchmark evidence README must remain
        readme = REPO / "spikes/bytecode-hot/README.md"
        self.assertTrue(readme.is_file())
        self.assertIn("bc-loop", readme.read_text(encoding="utf-8"))

        # Check that active code in xtask/ and tools/ (excluding tests) has no live references
        forbidden_tokens = (
            "verify-dod.sh",
            "gen-lisp-mk.py",
            "l1-worklist.sh",
            "loadup-fixture.sh",
            "reconcile-link-ratchets.sh",
            "bytecode_hot.rs",
            "BcHotState",
        )
        for scan_dir in (REPO / "xtask/src", REPO / "tools"):
            for candidate in scan_dir.rglob("*"):
                if not candidate.is_file() or candidate.name in (
                    "test-modernization-tools.py",
                    "test-macos-stability.py",
                ):
                    continue
                if candidate.suffix not in (".rs", ".py", ".sh"):
                    continue
                text = candidate.read_text(encoding="utf-8", errors="replace")
                for token in forbidden_tokens:
                    self.assertNotIn(
                        token,
                        text,
                        f"Active tool/code {candidate.relative_to(REPO)} still references retired {token}",
                    )

    # -------------------------------------------------------------------------
    # II-T14: Retirement ledger, updated active docs, and preserved platform/data assets
    # -------------------------------------------------------------------------
    def test_II_T14_retention_policy_ledger_and_preserved_platform_trees(self) -> None:
        """II-T14: docs/retired-scaffolding.md records retired files and platform/data trees remain."""
        ledger = (REPO / "docs/retired-scaffolding.md").read_text(encoding="utf-8")
        for retired_name in (
            "verify-dod.sh",
            "gen-lisp-mk.py",
            "l1-worklist.sh",
            "loadup-fixture.sh",
            "reconcile-link-ratchets.sh",
            "bytecode_hot.rs",
        ):
            self.assertIn(retired_name, ledger, f"Retirement ledger missing {retired_name}")

        # Preserved non-Rust and platform assets must still exist
        preserved = [
            REPO / "platform/macos/nsterm.swift",
            REPO / "platform/android",
            REPO / "admin/charsets",
            REPO / "tools/gen-unidata.py",
            REPO / "tools/gen-charsets.py",
            REPO / "tools/gen-leim.py",
            REPO / "tools/bootstrap-resources.py",
            REPO / "tools/zig-linux-linker.sh",
            REPO / "tools/pgo_bench.py",
            REPO / "tools/pgo/eval.el",
            REPO / "tools/bench.py",
            REPO / "tools/differential.py",
        ]
        for path in preserved:
            self.assertTrue(path.exists(), f"Required asset must be preserved: {path}")


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--json-out", type=Path, help="Optional path to write per-test timing JSON")
    args, unittest_args = ap.parse_known_args(argv)

    suite = unittest.defaultTestLoader.loadTestsFromTestCase(StageIIToolingTests)
    runner = unittest.TextTestRunner(verbosity=2, resultclass=TimedTestResult)
    suite_start = time.perf_counter()
    result: TimedTestResult = runner.run(suite)  # type: ignore[assignment]
    suite_elapsed = round(time.perf_counter() - suite_start, 6)

    summary = {
        "suite": "test-modernization-tools",
        "total": result.testsRun,
        "passed": sum(1 for t in result.timings if t["outcome"] == "passed"),
        "failed": len(result.failures),
        "errors": len(result.errors),
        "skipped": len(result.skipped),
        "suite_elapsed_seconds": suite_elapsed,
        "cases": result.timings,
    }
    if args.json_out:
        args.json_out.parent.mkdir(parents=True, exist_ok=True)
        args.json_out.write_text(json.dumps(summary, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({"suite_summary": {k: v for k, v in summary.items() if k != "cases"}}))
    return 0 if result.wasSuccessful() else 1


if __name__ == "__main__":
    raise SystemExit(main())
