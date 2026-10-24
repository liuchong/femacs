#!/usr/bin/env python3
"""Partner-owned unit and component integration tests for ERT harness and L5 sweep wrapper.

Covers Stage I requirements R1 and R2 (cases T1-T16 in docs/modernization.md):
- Real ERT batch transcript parsing (passed, FAILED, SKIPPED/skipped, expected-failure, timings)
- Rejection of missing, duplicate, or contradictory detail lines and non-zero exit after summary
- Identity-first comparison (test name swaps, pass/skip swaps, count mismatches)
- Zero-tests classification as untested (`zero-tests`), never `same`
- Strict gate exit (`gate_exit`) vs explicit `--collect-only` collection mode
- Retry handling: retains every attempt, still executes reference, never masks a failed retry as
  a passing `flaky` case
- `--list` mode without compiled binaries and without `.git` file creation or Git mutation
- `tools/l5-sweep.sh` normal vs `--gc-stress` argument expansion, chunk validation, `--list` error
  propagation, and multi-chunk gate exit aggregation
"""

from __future__ import annotations

import argparse
import importlib.util
import inspect
import json
import os
import shutil
import stat
import subprocess
import sys
import tempfile
import time
import unittest
from pathlib import Path
from unittest import mock

REPO = Path(__file__).resolve().parent.parent
CHECK_ERT_PATH = REPO / "tools/check-ert.py"
L5_SWEEP_PATH = REPO / "tools/l5-sweep.sh"


def load_check_ert():
    """Load tools/check-ert.py as a module and register it in sys.modules for @dataclass."""
    spec = importlib.util.spec_from_file_location("check_ert", CHECK_ERT_PATH)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load spec from {CHECK_ERT_PATH}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def invoke_check_ert_main(mod, argv: list[str]) -> int:
    """Call check_ert.main with argv whether main accepts an argv list or reads sys.argv."""
    sig = inspect.signature(mod.main)
    with mock.patch.object(sys, "argv", ["check-ert.py", *argv]):
        if len(sig.parameters) >= 1:
            return int(mod.main(argv))
        return int(mod.main())


def write_executable(path: Path, content: str) -> Path:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")
    path.chmod(path.stat().st_mode | stat.S_IXUSR | stat.S_IXGRP | stat.S_IXOTH)
    return path


REAL_ERT_TRANSCRIPT_MIXED = """\
Running 4 tests (2026-10-24 17:00:00+0000, selector `(not (or (tag :expensive-test) (tag :unstable)))')
   passed  1/4  ert-alpha-pass (0.001250 sec)
Test ert-beta-fail backtrace:
  signal(ert-test-failed (((should (= 1 2)) :form (= 1 2) :value nil)))
Test ert-beta-fail condition:
    (ert-test-failed
     ((should (= 1 2)) :form (= 1 2) :value nil))
   FAILED  2/4  ert-beta-fail (0.003400 sec) at lisp/emacs-lisp/ert-tests.el:42
  skipped  3/4  ert-gamma-skip (0.000500 sec)
   failed  4/4  ert-delta-xfail (0.002100 sec)

Ran 4 tests, 2 results as expected, 1 unexpected, 1 skipped (2026-10-24 17:00:00+0000, 0.007250 sec)
1 expected failures

1 unexpected results:
   FAILED  ert-beta-fail

1 skipped results:
  SKIPPED  ert-gamma-skip
"""

REAL_ERT_TRANSCRIPT_CLEAN_TWO = """\
Running 2 tests (2026-10-24 17:00:00+0000, selector `t')
   passed  1/2  case-one (0.001000 sec)
   passed  2/2  case-two (0.002000 sec)

Ran 2 tests, 2 results as expected, 0 unexpected (2026-10-24 17:00:00+0000, 0.003000 sec)
"""

REAL_ERT_TRANSCRIPT_ZERO = """\
Running 0 tests (2026-10-24 17:00:00+0000, selector `t')

Ran 0 tests, 0 results as expected, 0 unexpected (2026-10-24 17:00:00+0000, 0.000100 sec)
"""


class ErtHarnessUnitAndComponentTests(unittest.TestCase):
    """T1-T16 unit and component integration tests for check-ert.py and l5-sweep.sh."""

    @classmethod
    def setUpClass(cls) -> None:
        cls.mod = load_check_ert()

    def test_T01_parse_real_ert_batch_transcript(self) -> None:
        """T1: RunResult defaults and parse_result on real ERT passed/FAILED/SKIPPED/xfail lines."""
        default_rr = self.mod.RunResult()
        self.assertEqual(default_rr.ran, 0)
        self.assertEqual(default_rr.expected, 0)
        self.assertEqual(default_rr.unexpected, 0)
        self.assertEqual(default_rr.results, {})
        self.assertEqual(default_rr.test_seconds, {})
        self.assertEqual(default_rr.elapsed_seconds, 0.0)
        self.assertEqual(default_rr.note, "")

        parsed = self.mod.parse_result(REAL_ERT_TRANSCRIPT_MIXED, 1, 0.125)
        self.assertEqual(parsed.status, "ok")
        self.assertEqual(parsed.ran, 4)
        self.assertEqual(parsed.expected, 2)
        self.assertEqual(parsed.unexpected, 1)
        self.assertAlmostEqual(parsed.elapsed_seconds, 0.125, places=6)
        self.assertEqual(
            set(parsed.results.keys()),
            {"ert-alpha-pass", "ert-beta-fail", "ert-gamma-skip", "ert-delta-xfail"},
        )
        self.assertEqual(parsed.results["ert-alpha-pass"].lower(), "passed")
        self.assertEqual(parsed.results["ert-beta-fail"].upper(), "FAILED")
        self.assertIn(parsed.results["ert-gamma-skip"].lower(), ("skipped", "skip"))
        self.assertIn(
            parsed.results["ert-delta-xfail"].lower(),
            ("failed", "expected-failure", "xfail"),
        )
        self.assertEqual(len(set(parsed.results.values())), 4)
        self.assertAlmostEqual(parsed.test_seconds["ert-alpha-pass"], 0.00125, places=6)
        self.assertAlmostEqual(parsed.test_seconds["ert-beta-fail"], 0.0034, places=6)
        self.assertAlmostEqual(parsed.test_seconds["ert-gamma-skip"], 0.0005, places=6)
        self.assertAlmostEqual(parsed.test_seconds["ert-delta-xfail"], 0.0021, places=6)

        # Also verify %S-escaped test names and real ERT skipped progress + SKIPPED footer.
        escaped_skip_text = (
            "Running 1 tests (2026-10-24 17:00:00+0000, selector `t')\n"
            "  skipped  1/1  ert-escaped\\name\\? (0.000200 sec)\n\n"
            "Ran 1 tests, 0 results as expected, 0 unexpected, 1 skipped "
            "(2026-10-24 17:00:00+0000, 0.000200 sec)\n\n"
            "1 skipped results:\n"
            "  SKIPPED  ert-escaped\\name\\?\n"
        )
        parsed_skip = self.mod.parse_result(escaped_skip_text, 0, 0.01)
        self.assertEqual(parsed_skip.status, "ok")
        self.assertEqual(parsed_skip.ran, 1)
        self.assertIn("ert-escaped\\name\\?", parsed_skip.results)
        self.assertIn(parsed_skip.results["ert-escaped\\name\\?"].lower(), ("skipped", "skip"))

    def test_T02_parse_rejects_missing_duplicate_contradictory_or_crashed(self) -> None:
        """T2: parse_result rejects missing detail lines, duplicates, count contradictions, crashes."""
        # 1. Summary claims 2 tests ran, but only 1 detail line is present -> incomplete.
        missing_detail = (
            "Running 2 tests\n"
            "   passed  1/2  case-one (0.001000 sec)\n\n"
            "Ran 2 tests, 2 results as expected, 0 unexpected (2026-10-24 17:00:00+0000, 0.002 sec)\n"
        )
        r_missing = self.mod.parse_result(missing_detail, 0, 0.02)
        self.assertEqual(r_missing.status, "incomplete")

        # 2. Duplicate test name in detail lines -> incomplete.
        duplicate_name = (
            "Running 2 tests\n"
            "   passed  1/2  dup-case (0.001000 sec)\n"
            "   passed  2/2  dup-case (0.001000 sec)\n\n"
            "Ran 2 tests, 2 results as expected, 0 unexpected (2026-10-24 17:00:00+0000, 0.002 sec)\n"
        )
        r_dup = self.mod.parse_result(duplicate_name, 0, 0.02)
        self.assertEqual(r_dup.status, "incomplete")

        # 3. Contradictory detail vs summary (FAILED detail line but summary claims 0 unexpected).
        contradictory = (
            "Running 2 tests\n"
            "   passed  1/2  case-one (0.001000 sec)\n"
            "   FAILED  2/2  case-two (0.001000 sec)\n\n"
            "Ran 2 tests, 2 results as expected, 0 unexpected (2026-10-24 17:00:00+0000, 0.002 sec)\n"
        )
        r_contra = self.mod.parse_result(contradictory, 0, 0.02)
        self.assertEqual(r_contra.status, "incomplete")

        # 4. Non-zero exit code after a clean 0-unexpected summary -> crashed.
        r_crash_after = self.mod.parse_result(REAL_ERT_TRANSCRIPT_CLEAN_TWO, 1, 0.02)
        self.assertEqual(r_crash_after.status, "crashed")

        # 5. No summary at all: returncode 0 -> no-summary / incomplete; returncode != 0 -> crashed.
        r_no_sum = self.mod.parse_result("Loading foo...\n", 0, 0.01)
        self.assertIn(r_no_sum.status, ("no-summary", "incomplete", "crashed"))
        r_segv = self.mod.parse_result("Fatal error 11: Segmentation fault\n", -11, 0.01)
        self.assertEqual(r_segv.status, "crashed")

    def test_T03_compare_identity_and_outcome_swap_with_equal_counts(self) -> None:
        """T3: compare_results detects test name differences and swapped pass/fail/skip outcomes."""
        base_ok = self.mod.parse_result(REAL_ERT_TRANSCRIPT_CLEAN_TWO, 0, 0.01)

        # Subcase A: Both ran 2 tests with 0 unexpected, but test names differ.
        renamed_text = REAL_ERT_TRANSCRIPT_CLEAN_TWO.replace("case-two", "case-three")
        renamed_ok = self.mod.parse_result(renamed_text, 0, 0.01)
        verdict_a, _ = self.mod.compare_results(renamed_ok, base_ok)
        self.assertEqual(verdict_a, "different")

        # Subcase B: Both ran 2 tests with 1 unexpected, but different tests failed!
        fail_one_text = (
            "Running 2 tests\n"
            "   FAILED  1/2  case-one (0.001000 sec)\n"
            "   passed  2/2  case-two (0.001000 sec)\n\n"
            "Ran 2 tests, 1 results as expected, 1 unexpected (2026-10-24 17:00:00+0000, 0.002 sec)\n"
        )
        fail_two_text = (
            "Running 2 tests\n"
            "   passed  1/2  case-one (0.001000 sec)\n"
            "   FAILED  2/2  case-two (0.001000 sec)\n\n"
            "Ran 2 tests, 1 results as expected, 1 unexpected (2026-10-24 17:00:00+0000, 0.002 sec)\n"
        )
        r_fail_one = self.mod.parse_result(fail_one_text, 1, 0.01)
        r_fail_two = self.mod.parse_result(fail_two_text, 1, 0.01)
        verdict_b, _ = self.mod.compare_results(r_fail_one, r_fail_two)
        self.assertNotIn(verdict_b, ("same", "both-fail", "port-better"))
        self.assertIn(verdict_b, ("different", "port-worse"))

        # Subcase C: Both have 0 unexpected, but port skipped case-two while reference passed it.
        skip_two_text = (
            "Running 2 tests\n"
            "   passed  1/2  case-one (0.001000 sec)\n"
            "  skipped  2/2  case-two (0.001000 sec)\n\n"
            "Ran 2 tests, 1 results as expected, 0 unexpected, 1 skipped "
            "(2026-10-24 17:00:00+0000, 0.002 sec)\n"
        )
        r_skip_two = self.mod.parse_result(skip_two_text, 0, 0.01)
        verdict_c, _ = self.mod.compare_results(r_skip_two, base_ok)
        self.assertNotEqual(verdict_c, "same")
        self.assertIn(verdict_c, ("different", "port-worse"))

    def test_T04_compare_count_mismatch_and_expected_failure_equivalence(self) -> None:
        """T4: compare_results rejects ran count mismatch and accepts identical expected-failure."""
        two_ok = self.mod.parse_result(REAL_ERT_TRANSCRIPT_CLEAN_TWO, 0, 0.01)
        one_text = (
            "Running 1 tests\n"
            "   passed  1/1  case-one (0.001000 sec)\n\n"
            "Ran 1 tests, 1 results as expected, 0 unexpected (2026-10-24 17:00:00+0000, 0.001 sec)\n"
        )
        one_ok = self.mod.parse_result(one_text, 0, 0.01)
        verdict_mismatch, _ = self.mod.compare_results(one_ok, two_ok)
        self.assertNotEqual(verdict_mismatch, "same")
        self.assertIn(verdict_mismatch, ("different", "port-worse"))

        xfail_text = (
            "Running 2 tests\n"
            "   passed  1/2  case-one (0.001000 sec)\n"
            "   failed  2/2  case-xfail (0.001000 sec)\n\n"
            "Ran 2 tests, 2 results as expected, 0 unexpected (2026-10-24 17:00:00+0000, 0.002 sec)\n"
            "1 expected failures\n"
        )
        port_xfail = self.mod.parse_result(xfail_text, 0, 0.01)
        ref_xfail = self.mod.parse_result(xfail_text, 0, 0.01)
        verdict_xfail, _ = self.mod.compare_results(port_xfail, ref_xfail)
        self.assertEqual(verdict_xfail, "same")

    def test_T05_compare_zero_tests_is_untested_not_same(self) -> None:
        """T5: 0/0 executed tests is classified as zero-tests (untested), never same."""
        port_zero = self.mod.parse_result(REAL_ERT_TRANSCRIPT_ZERO, 0, 0.01)
        ref_zero = self.mod.parse_result(REAL_ERT_TRANSCRIPT_ZERO, 0, 0.01)
        verdict, _ = self.mod.compare_results(port_zero, ref_zero)
        self.assertEqual(verdict, "zero-tests")
        self.assertNotEqual(self.mod.gate_exit([verdict]), 0)

    def test_T06_compare_incomplete_crashed_and_both_capped(self) -> None:
        """T6: compare_results propagates incomplete, crashed, and both-capped statuses."""
        clean = self.mod.parse_result(REAL_ERT_TRANSCRIPT_CLEAN_TWO, 0, 0.01)
        incomplete = self.mod.RunResult(status="incomplete", ran=2, expected=2, unexpected=0, note="missing lines")
        crashed = self.mod.RunResult(status="crashed", note="exit -11")
        oom_a = self.mod.RunResult(status="out-of-memory", note="cap")
        oom_b = self.mod.RunResult(status="out-of-memory", note="cap")

        v_inc, _ = self.mod.compare_results(incomplete, clean)
        self.assertEqual(v_inc, "incomplete")

        v_crash, _ = self.mod.compare_results(crashed, clean)
        self.assertEqual(v_crash, "crashed")

        v_oom, _ = self.mod.compare_results(oom_a, oom_b)
        self.assertEqual(v_oom, "both-capped")

    def test_T07_compare_port_worse_port_better_and_both_fail(self) -> None:
        """T7: compare_results distinguishes port-worse, port-better, and identical both-fail."""
        clean = self.mod.parse_result(REAL_ERT_TRANSCRIPT_CLEAN_TWO, 0, 0.01)
        failed_one_text = (
            "Running 2 tests\n"
            "   passed  1/2  case-one (0.001000 sec)\n"
            "   FAILED  2/2  case-two (0.001000 sec)\n\n"
            "Ran 2 tests, 1 results as expected, 1 unexpected (2026-10-24 17:00:00+0000, 0.002 sec)\n"
        )
        failed_one = self.mod.parse_result(failed_one_text, 1, 0.01)

        v_worse, _ = self.mod.compare_results(failed_one, clean)
        self.assertEqual(v_worse, "port-worse")

        v_better, _ = self.mod.compare_results(clean, failed_one)
        self.assertEqual(v_better, "port-better")

        v_both, _ = self.mod.compare_results(failed_one, failed_one)
        self.assertEqual(v_both, "both-fail")

    def test_T08_gate_exit_strict_policy(self) -> None:
        """T8: gate_exit passes only non-empty same/port-better lists and fails all other verdicts."""
        self.assertEqual(self.mod.gate_exit(["same"]), 0)
        self.assertEqual(self.mod.gate_exit(["same", "port-better"]), 0)
        self.assertNotEqual(self.mod.gate_exit([]), 0)
        for bad_verdict in (
            "zero-tests",
            "incomplete",
            "different",
            "both-fail",
            "port-worse",
            "crashed",
            "flaky",
            "both-capped",
            "unknown-verdict",
        ):
            with self.subTest(verdict=bad_verdict):
                self.assertNotEqual(self.mod.gate_exit(["same", bad_verdict]), 0)

    def test_T09_retry_retains_attempts_runs_reference_and_never_masks_failed_retry(self) -> None:
        """T9: Retries retain every attempt, always run reference, and never call a failed retry passing flaky."""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            port_test = root / "test"
            ref_test = root / "ref-test"
            port_test.mkdir()
            ref_test.mkdir()
            (port_test / "sample-tests.el").write_text(";; sample\n", encoding="utf-8")
            (ref_test / "sample-tests.el").write_text(";; sample\n", encoding="utf-8")

            state_file = root / "port-attempt-count.txt"
            mode_file = root / "port-retry-mode.txt"
            ref_log = root / "ref-invocations.txt"

            fake_port = write_executable(
                root / "port-emacs",
                f"""\
#!/usr/bin/env python3
import pathlib, sys
state = pathlib.Path({str(state_file)!r})
mode = pathlib.Path({str(mode_file)!r}).read_text(encoding="utf-8").strip()
n = int(state.read_text(encoding="utf-8")) + 1 if state.exists() else 1
state.write_text(str(n), encoding="utf-8")
if n == 1:
    sys.stdout.write("Fatal crash on first attempt\\n")
    sys.exit(6)
if mode == "clean":
    sys.stdout.write({REAL_ERT_TRANSCRIPT_CLEAN_TWO!r})
    sys.exit(0)
sys.stdout.write(
    "Running 2 tests\\n"
    "   passed  1/2  case-one (0.001000 sec)\\n"
    "   FAILED  2/2  case-two (0.001000 sec)\\n\\n"
    "Ran 2 tests, 1 results as expected, 1 unexpected (2026-10-24 17:00:00+0000, 0.002 sec)\\n"
)
sys.exit(1)
""",
            )
            fake_ref = write_executable(
                root / "ref-emacs",
                f"""\
#!/usr/bin/env python3
import pathlib, sys
log = pathlib.Path({str(ref_log)!r})
log.write_text((log.read_text(encoding="utf-8") if log.exists() else "") + "ran\\n", encoding="utf-8")
sys.stdout.write({REAL_ERT_TRANSCRIPT_CLEAN_TWO!r})
sys.exit(0)
""",
            )

            # Subcase A: Port crashes on attempt 1, passes cleanly on attempt 2; reference runs and passes.
            mode_file.write_text("clean", encoding="utf-8")
            json_a = root / "results-a.json"
            tsv_a = root / "results-a.tsv"
            with (
                mock.patch.object(self.mod, "PORT_TEST", port_test),
                mock.patch.object(self.mod, "REF_TEST", ref_test),
            ):
                rc_strict = invoke_check_ert_main(
                    self.mod,
                    [
                        "--glob",
                        "sample-tests.el",
                        "--retries",
                        "1",
                        "--port-prog",
                        str(fake_port),
                        "--ref-prog",
                        str(fake_ref),
                        "--tsv",
                        str(tsv_a),
                        "--results-json",
                        str(json_a),
                    ],
                )
            self.assertNotEqual(rc_strict, 0, "flaky file must fail strict gate")
            self.assertTrue(ref_log.is_file(), "reference program must still run when retry succeeds")
            tsv_lines_a = tsv_a.read_text(encoding="utf-8").strip().splitlines()
            self.assertEqual(len(tsv_lines_a), 2)
            fields_a = tsv_lines_a[1].split("\t")
            self.assertEqual(fields_a[1], "flaky")
            self.assertEqual(int(fields_a[4]), 2, "ref_ran must reflect actual reference execution")
            data_a = json.loads(json_a.read_text(encoding="utf-8"))
            file_rec_a = data_a["files"][0] if isinstance(data_a, dict) and "files" in data_a else data_a[0]
            attempts_a = (
                file_rec_a.get("port_attempts")
                or file_rec_a.get("attempts")
                or file_rec_a.get("port", {}).get("attempts")
            )
            self.assertIsNotNone(attempts_a)
            self.assertEqual(len(attempts_a), 2)

            # Subcase B: Port crashes on attempt 1, runs but fails 1 test on attempt 2; must NOT be passing flaky.
            state_file.unlink()
            mode_file.write_text("fail_one", encoding="utf-8")
            json_b = root / "results-b.json"
            tsv_b = root / "results-b.tsv"
            with (
                mock.patch.object(self.mod, "PORT_TEST", port_test),
                mock.patch.object(self.mod, "REF_TEST", ref_test),
            ):
                rc_b = invoke_check_ert_main(
                    self.mod,
                    [
                        "--glob",
                        "sample-tests.el",
                        "--retries",
                        "1",
                        "--port-prog",
                        str(fake_port),
                        "--ref-prog",
                        str(fake_ref),
                        "--tsv",
                        str(tsv_b),
                        "--results-json",
                        str(json_b),
                    ],
                )
            self.assertNotEqual(rc_b, 0)
            fields_b = tsv_b.read_text(encoding="utf-8").strip().splitlines()[1].split("\t")
            self.assertNotEqual(fields_b[1], "same")
            self.assertIn(fields_b[1], ("port-worse", "crashed"))

    def test_T10_strict_gate_vs_collect_only_and_results_json_schema(self) -> None:
        """T10: Strict gate returns non-zero on failure while --collect-only returns 0 and saves JSON/TSV."""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            port_test = root / "test"
            ref_test = root / "ref-test"
            port_test.mkdir()
            ref_test.mkdir()
            (port_test / "zero-tests.el").write_text(";; zero\n", encoding="utf-8")
            (ref_test / "zero-tests.el").write_text(";; zero\n", encoding="utf-8")

            zero_script = f"#!/usr/bin/env python3\nimport sys\nsys.stdout.write({REAL_ERT_TRANSCRIPT_ZERO!r})\nsys.exit(0)\n"
            port_prog = write_executable(root / "port-emacs", zero_script)
            ref_prog = write_executable(root / "ref-emacs", zero_script)

            tsv_path = root / "out.tsv"
            json_path = root / "out.json"
            common_args = [
                "--glob",
                "zero-tests.el",
                "--port-prog",
                str(port_prog),
                "--ref-prog",
                str(ref_prog),
                "--tsv",
                str(tsv_path),
                "--results-json",
                str(json_path),
            ]
            with (
                mock.patch.object(self.mod, "PORT_TEST", port_test),
                mock.patch.object(self.mod, "REF_TEST", ref_test),
            ):
                rc_strict = invoke_check_ert_main(self.mod, common_args)
                rc_collect = invoke_check_ert_main(self.mod, [*common_args, "--collect-only"])

            self.assertNotEqual(rc_strict, 0)
            self.assertEqual(rc_collect, 0)
            tsv_lines = tsv_path.read_text(encoding="utf-8").strip().splitlines()
            self.assertEqual(
                tsv_lines[0],
                "file\tverdict\tport_ran\tport_unexpected\tref_ran\tref_unexpected\tnote",
            )
            self.assertEqual(tsv_lines[1].split("\t")[1], "zero-tests")
            payload = json.loads(json_path.read_text(encoding="utf-8"))
            files_list = payload["files"] if isinstance(payload, dict) and "files" in payload else payload
            self.assertEqual(files_list[0]["file"], "zero-tests.el")
            self.assertEqual(files_list[0]["verdict"], "zero-tests")

    def test_T11_list_mode_requires_no_binaries_and_never_mutates_git(self) -> None:
        """T11: --list works when binaries are absent and never writes a .git file."""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            test_dir = root / "test"
            (test_dir / "lisp").mkdir(parents=True)
            (test_dir / "manual").mkdir(parents=True)
            (test_dir / "lisp/good-tests.el").write_text(";; ok\n", encoding="utf-8")
            (test_dir / "manual/ignored-tests.el").write_text(";; pruned\n", encoding="utf-8")
            git_marker = root / ".git"

            with (
                mock.patch.object(self.mod, "PORT_TEST", test_dir),
                mock.patch.object(self.mod, "REF_TEST", test_dir),
            ):
                rc = invoke_check_ert_main(
                    self.mod,
                    [
                        "--list",
                        "--port-prog",
                        str(root / "missing-port"),
                        "--ref-prog",
                        str(root / "missing-ref"),
                    ],
                )
            self.assertEqual(rc, 0)
            self.assertFalse(git_marker.exists(), "--list must never create a .git file")

    def test_T12_default_paths_point_to_standalone_terminal_product(self) -> None:
        """T12: Default PORT_PROG points to build/terminal/port/src/emacs and gitfile helper is gone."""
        self.assertEqual(
            Path(self.mod.PORT_PROG),
            REPO / "build/terminal/port/src/emacs",
        )
        self.assertFalse(
            hasattr(self.mod, "ensure_port_tree_is_a_project_root"),
            "ensure_port_tree_is_a_project_root must be removed in Stage I",
        )

    def _setup_sweep_workspace(self, root: Path, check_ert_script: str) -> Path:
        tools_dir = root / "tools"
        tools_dir.mkdir(parents=True, exist_ok=True)
        sweep_copy = tools_dir / "l5-sweep.sh"
        shutil.copy2(L5_SWEEP_PATH, sweep_copy)
        sweep_copy.chmod(sweep_copy.stat().st_mode | stat.S_IXUSR)
        write_executable(tools_dir / "check-ert.py", check_ert_script)
        write_executable(root / "build/terminal/port/src/emacs", "#!/bin/sh\nexit 0\n")
        write_executable(root / "build/emacs-ref/src/emacs", "#!/bin/sh\nexit 0\n")
        return sweep_copy

    def test_T13_l5_sweep_normal_vs_gc_stress_argv(self) -> None:
        """T13: l5-sweep.sh does not pass --gc-stress in normal mode and passes it with --gc-stress."""
        stub = """\
#!/usr/bin/env python3
import json, pathlib, sys
args = sys.argv[1:]
if "--list" in args:
    print("lisp/one-tests.el")
    print("lisp/two-tests.el")
    sys.exit(0)
log = pathlib.Path(__file__).resolve().parent.parent / "argv-log.jsonl"
with log.open("a", encoding="utf-8") as fh:
    fh.write(json.dumps(args) + "\\n")
if "--tsv" in args:
    tsv = pathlib.Path(args[args.index("--tsv") + 1])
    tsv.parent.mkdir(parents=True, exist_ok=True)
    tsv.write_text(
        "file\\tverdict\\tport_ran\\tport_unexpected\\tref_ran\\tref_unexpected\\tnote\\n"
        "lisp/one-tests.el\\tsame\\t2\\t0\\t2\\t0\\t\\n",
        encoding="utf-8",
    )
if "--results-json" in args:
    jpath = pathlib.Path(args[args.index("--results-json") + 1])
    jpath.parent.mkdir(parents=True, exist_ok=True)
    jpath.write_text(json.dumps({"files": [{"file": "lisp/one-tests.el", "verdict": "same"}]}), encoding="utf-8")
sys.exit(0)
"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            sweep = self._setup_sweep_workspace(root, stub)
            out_normal = root / "out-normal"
            out_stress = root / "out-stress"

            res_normal = subprocess.run(
                ["bash", str(sweep), "--chunk", "2", "--out", str(out_normal)],
                cwd=root,
                capture_output=True,
                text=True,
                timeout=15,
            )
            self.assertEqual(res_normal.returncode, 0, res_normal.stderr + res_normal.stdout)

            res_stress = subprocess.run(
                ["bash", str(sweep), "--chunk", "2", "--gc-stress", "--out", str(out_stress)],
                cwd=root,
                capture_output=True,
                text=True,
                timeout=15,
            )
            self.assertEqual(res_stress.returncode, 0, res_stress.stderr + res_stress.stdout)

            recorded = [
                json.loads(line)
                for line in (root / "argv-log.jsonl").read_text(encoding="utf-8").splitlines()
                if line.strip()
            ]
            self.assertGreaterEqual(len(recorded), 2)
            normal_argv, stress_argv = recorded[0], recorded[1]
            self.assertNotIn("--gc-stress", normal_argv)
            self.assertIn("--gc-stress", stress_argv)

    def test_T14_l5_sweep_rejects_nonpositive_chunk_and_bad_args(self) -> None:
        """T14: l5-sweep.sh rejects non-positive --chunk values and unknown flags."""
        stub = "#!/usr/bin/env python3\nimport sys\nsys.exit(0)\n"
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            sweep = self._setup_sweep_workspace(root, stub)
            for bad_args in (["--chunk", "0"], ["--chunk", "-3"], ["--unknown-flag"]):
                with self.subTest(args=bad_args):
                    proc = subprocess.run(
                        ["bash", str(sweep), *bad_args],
                        cwd=root,
                        capture_output=True,
                        text=True,
                        timeout=10,
                    )
                    self.assertNotEqual(proc.returncode, 0)

    def test_T15_l5_sweep_propagates_list_or_pipeline_failure(self) -> None:
        """T15: l5-sweep.sh fails when --list fails or a chunk command errors in the pipeline."""
        failing_list_stub = """\
#!/usr/bin/env python3
import sys
print("error: missing test root", file=sys.stderr)
print("details line 2", file=sys.stderr)
sys.exit(2)
"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            sweep = self._setup_sweep_workspace(root, failing_list_stub)
            proc = subprocess.run(
                ["bash", str(sweep), "--out", str(root / "out")],
                cwd=root,
                capture_output=True,
                text=True,
                timeout=10,
            )
            self.assertNotEqual(proc.returncode, 0)

    def test_T16_l5_sweep_collects_chunks_and_propagates_gate_failure(self) -> None:
        """T16: l5-sweep.sh collects all chunks without discarding failure rows, then exits non-zero."""
        chunk_stub = """\
#!/usr/bin/env python3
import json, pathlib, sys
args = sys.argv[1:]
if "--list" in args:
    print("lisp/first-tests.el")
    print("lisp/second-tests.el")
    print("lisp/third-tests.el")
    sys.exit(0)
start = int(args[args.index("--start-at") + 1]) if "--start-at" in args else 0
verdict = "port-worse" if start == 0 else "same"
unexp = 1 if start == 0 else 0
note = "1 unexpected vs 0" if start == 0 else ""
name = f"lisp/chunk-{start}-tests.el"
if "--tsv" in args:
    tsv = pathlib.Path(args[args.index("--tsv") + 1])
    tsv.parent.mkdir(parents=True, exist_ok=True)
    tsv.write_text(
        "file\\tverdict\\tport_ran\\tport_unexpected\\tref_ran\\tref_unexpected\\tnote\\n"
        f"{name}\\t{verdict}\\t2\\t{unexp}\\t2\\t0\\t{note}\\n",
        encoding="utf-8",
    )
if "--results-json" in args:
    jpath = pathlib.Path(args[args.index("--results-json") + 1])
    jpath.parent.mkdir(parents=True, exist_ok=True)
    jpath.write_text(json.dumps({"files": [{"file": name, "verdict": verdict}]}), encoding="utf-8")
if start == 0 and "--collect-only" not in args:
    sys.exit(1)
sys.exit(0)
"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            sweep = self._setup_sweep_workspace(root, chunk_stub)
            out_dir = root / "out"
            proc = subprocess.run(
                ["bash", str(sweep), "--chunk", "1", "--out", str(out_dir)],
                cwd=root,
                capture_output=True,
                text=True,
                timeout=15,
            )
            self.assertNotEqual(
                proc.returncode,
                0,
                "sweep must exit non-zero when any chunk contains a non-passing verdict",
            )
            self.assertTrue((out_dir / "ert-l5-chunk001.tsv").is_file())
            self.assertTrue(
                (out_dir / "ert-l5-chunk002.tsv").is_file(),
                "second chunk must still be collected even when first chunk records a failure row",
            )


class TimedTestResult(unittest.TextTestResult):
    """Unittest result adapter that records per-case monotonic duration and status."""

    def __init__(self, *args, **kwargs) -> None:
        super().__init__(*args, **kwargs)
        self._case_started: dict[str, float] = {}
        self.case_records: list[dict[str, object]] = []

    def startTest(self, test: unittest.TestCase) -> None:
        self._case_started[test.id()] = time.monotonic()
        super().startTest(test)

    def _record(self, test: unittest.TestCase, status: str, detail: str = "") -> None:
        started = self._case_started.pop(test.id(), time.monotonic())
        elapsed = round(max(0.0, time.monotonic() - started), 6)
        self.case_records.append(
            {
                "id": test.id(),
                "status": status,
                "elapsed_seconds": elapsed,
                "detail": detail,
            }
        )

    def addSuccess(self, test: unittest.TestCase) -> None:
        self._record(test, "passed")
        super().addSuccess(test)

    def addFailure(self, test: unittest.TestCase, err) -> None:
        self._record(test, "failed", self._exc_info_to_string(err, test))
        super().addFailure(test, err)

    def addError(self, test: unittest.TestCase, err) -> None:
        self._record(test, "error", self._exc_info_to_string(err, test))
        super().addError(test, err)

    def addSkip(self, test: unittest.TestCase, reason: str) -> None:
        self._record(test, "skipped", reason)
        super().addSkip(test, reason)


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--json-out", type=Path, help="optional path to write per-case timing JSON")
    parser.add_argument("-k", "--pattern", help="optional substring filter on test case names")
    args = parser.parse_args(argv)

    suite = unittest.defaultTestLoader.loadTestsFromTestCase(ErtHarnessUnitAndComponentTests)
    if args.pattern:
        filtered = unittest.TestSuite()
        for test in suite:
            if args.pattern in test.id():
                filtered.addTest(test)
        suite = filtered

    suite_started = time.monotonic()
    runner = unittest.TextTestRunner(verbosity=2, resultclass=TimedTestResult)
    result: TimedTestResult = runner.run(suite)  # type: ignore[assignment]
    suite_elapsed = round(max(0.0, time.monotonic() - suite_started), 6)

    summary = {
        "suite": "test-ert-harness",
        "total": result.testsRun,
        "passed": sum(1 for r in result.case_records if r["status"] == "passed"),
        "failed": len(result.failures),
        "errors": len(result.errors),
        "skipped": len(result.skipped),
        "suite_elapsed_seconds": suite_elapsed,
        "cases": result.case_records,
    }
    print(json.dumps({"suite_summary": {k: v for k, v in summary.items() if k != "cases"}}))
    if args.json_out:
        args.json_out.parent.mkdir(parents=True, exist_ok=True)
        args.json_out.write_text(json.dumps(summary, indent=2) + "\n", encoding="utf-8")
    return 0 if result.wasSuccessful() else 1


if __name__ == "__main__":
    raise SystemExit(main())
