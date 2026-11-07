#!/usr/bin/env python3
"""Focused protections for the bounded measurement report.

Pass a fresh ``--out`` directory. An existing path is refused. Fixtures and
the result file stay for later cleanup; this runner does not delete them.
"""
from __future__ import annotations

import argparse
import importlib.util
import json
import os
import signal
import stat
import subprocess
import sys
import time
import unittest
from pathlib import Path

TOOLS = Path(__file__).resolve().parent
MODULE_PATH = TOOLS / "runtime-measurement-report.py"
REPO = TOOLS.parent


def load_module():
    spec = importlib.util.spec_from_file_location(
        "runtime_measurement_report_under_test", MODULE_PATH)
    if spec is None or spec.loader is None:
        raise RuntimeError("cannot load runtime-measurement-report.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


REPORT = load_module()


class OpenBoundExceeded(Exception):
    """Raised when a protection open stays blocked past its time bound."""


def minimal_document(median=1.0):
    def case(identity, value):
        return {
            "case": "load",
            "mode": "native",
            "execution_identity": identity,
            "n": 7,
            "median_ms": value,
            "mad_ms": 0.0,
            "min_ms": 1.0,
            "max_ms": 1.0,
            "range_ms": 0.0,
            "pause_count": 0,
            "max_pause_ms": None,
            "p95_pause_ms": None,
            "p99_pause_ms": None,
            "peak_rss_bytes": 1,
            "max_load_per_core": 1.0,
            "quiet": False,
            "correctness": True,
            "inconclusive": True,
            "reasons": ["busy-host"],
        }
    return {
        "schema": REPORT.SCHEMA,
        "action": REPORT.ACTION,
        "processes": [{"warmup": False}],
        "samples": [{}],
        "startups": [{"warmup": True}],
        "summary": {
            "samples": [{}],
            "cases": [case("ref:off", median), case("cand:off", 1.0)],
            "failures": [],
            "inconclusive": ["load/native: busy-host", "load/native: busy-host"],
        },
        "comparisons": [{
            "reference": "ref",
            "candidate": "cand",
            "case": "load",
            "mode": "native",
            "paired_rounds": 7,
            "interval": None,
            "eligible": False,
        }],
        "errors": [],
        "excluded": [],
        "fixture_sources": {"fixture": "abc"},
        "fixtures_unchanged": True,
        "passed": True,
        "performance_accepted": False,
        "limitation": REPORT.LIMITATION,
        "full_product_inputs_unchanged": True,
        "driver_sources_unchanged": True,
        "goal_complete": False,
        "compatibility_accepted": False,
        "state_retirement_complete": True,
    }


class Protections(unittest.TestCase):
    out = None

    def setUp(self):
        if self.out is None:
            self.fail("explicit fresh --out directory required")
        self.root = self.out / self._testMethodName
        try:
            self.root.lstat()
        except FileNotFoundError:
            pass
        else:
            self.fail("protection fixture path exists")
        self.root.mkdir()
        self._reads = 0
        self._fstats = 0
        self._original_read = REPORT.os.read
        self._original_open = REPORT.os.open
        self._original_fstat = REPORT.os.fstat

    def tearDown(self):
        REPORT.os.read = self._original_read
        REPORT.os.open = self._original_open
        REPORT.os.fstat = self._original_fstat

    def _track_reads(self, transform=None):
        def tracking(fd, size):
            if transform is not None:
                transform(size)
            block = self._original_read(fd, size)
            self._reads += len(block)
            return block
        REPORT.os.read = tracking

    def test_ordinary_summary_is_copied(self):
        path = self.root / "ordinary.json"
        path.write_text(json.dumps(minimal_document()), encoding="utf-8")
        report = REPORT.build_report(REPORT.load_document(path))
        self.assertIs(report["performance_accepted"], False)
        self.assertIs(report["permanent_full_fallback"], False)
        self.assertIs(report["startup_measured"], True)
        self.assertEqual(report["counts"]["process_count"], 1)
        self.assertEqual(report["counts"]["process_warmup_count"], 0)
        self.assertEqual(report["counts"]["sample_count"], 1)
        self.assertEqual(report["counts"]["startup_count"], 1)
        self.assertEqual(report["counts"]["startup_warmup_count"], 1)
        self.assertEqual(report["counts"]["error_count"], 0)
        self.assertEqual(report["eligibility"]["busy_count"], 2)
        self.assertEqual(report["eligibility"]["eligible_count"], 0)
        self.assertEqual(report["cases"][0]["median_ms"], 1.0)

    def test_current_schema_ordinary_and_fallback_documents(self):
        ordinary = minimal_document()
        ordinary["permanent_full_fallback"] = False
        ordinary["startup_measured"] = True
        ordinary_report = REPORT.build_report(ordinary)
        self.assertIs(ordinary_report["permanent_full_fallback"], False)
        self.assertIs(ordinary_report["startup_measured"], True)
        self.assertIn("`permanent_full_fallback`: false", REPORT.render_markdown(ordinary_report))
        self.assertIn("`startup_measured`: true", REPORT.render_markdown(ordinary_report))

        fallback = minimal_document()
        fallback["permanent_full_fallback"] = True
        fallback["startup_measured"] = False
        fallback["startups"] = []
        for item in fallback["summary"]["cases"]:
            item["mode"] = "bytecode"
        fallback["summary"]["inconclusive"] = [
            "load/bytecode: busy-host",
            "load/bytecode: busy-host",
        ]
        fallback["comparisons"][0]["mode"] = "bytecode"
        fallback_report = REPORT.build_report(fallback)
        self.assertIs(fallback_report["permanent_full_fallback"], True)
        self.assertIs(fallback_report["startup_measured"], False)
        self.assertEqual(fallback_report["counts"]["startup_count"], 0)
        self.assertIn("`permanent_full_fallback`: true", REPORT.render_markdown(fallback_report))
        self.assertIn("`startup_measured`: false", REPORT.render_markdown(fallback_report))

    def test_fallback_contract_violations_are_rejected(self):
        partial = minimal_document()
        partial["permanent_full_fallback"] = False
        with self.assertRaises(REPORT.ReportError) as caught:
            REPORT.build_report(partial)
        self.assertEqual(caught.exception.code, "malformed-report: top")

        mismatch = minimal_document()
        mismatch["permanent_full_fallback"] = True
        mismatch["startup_measured"] = True
        with self.assertRaises(REPORT.ReportError) as caught:
            REPORT.build_report(mismatch)
        self.assertEqual(caught.exception.code, "malformed-report: startup_measured")

        with_startups = minimal_document()
        with_startups["permanent_full_fallback"] = True
        with_startups["startup_measured"] = False
        with self.assertRaises(REPORT.ReportError) as caught:
            REPORT.build_report(with_startups)
        self.assertEqual(caught.exception.code, "malformed-report: startups")

        native_fallback = minimal_document()
        native_fallback["permanent_full_fallback"] = True
        native_fallback["startup_measured"] = False
        native_fallback["startups"] = []
        with self.assertRaises(REPORT.ReportError) as caught:
            REPORT.build_report(native_fallback)
        self.assertEqual(caught.exception.code, "malformed-report: summary.cases[0]")

        observer_fallback = minimal_document()
        observer_fallback["permanent_full_fallback"] = True
        observer_fallback["startup_measured"] = False
        observer_fallback["startups"] = []
        for item in observer_fallback["summary"]["cases"]:
            item["mode"] = "bytecode"
        observer_fallback["summary"]["cases"][0]["execution_identity"] = "cand:phases"
        observer_fallback["comparisons"][0]["mode"] = "bytecode"
        with self.assertRaises(REPORT.ReportError) as caught:
            REPORT.build_report(observer_fallback)
        self.assertEqual(
            caught.exception.code,
            "malformed-report: summary.cases[0]",
        )

    def test_existing_oversize_file_is_not_read_whole(self):
        path = self.root / "large.json"
        path.write_bytes(b"{" + (b"x" * 200000) + b"}")
        self._track_reads()
        with self.assertRaises(REPORT.ReportError) as caught:
            REPORT.load_document(path, max_bytes=1000)
        self.assertEqual(caught.exception.code, "input-too-large")
        self.assertLessEqual(self._reads, 1001)
        self.assertGreater(path.stat().st_size, 1001)

    def test_growth_after_stat_stops_at_cap(self):
        path = self.root / "grow.json"
        path.write_bytes(b"{" + (b" " * 32) + b"}")
        max_bytes = 64

        def grow(_size):
            if path.stat().st_size < 1024:
                with path.open("ab") as stream:
                    stream.write(b"x" * (256 * 1024))

        self._track_reads(grow)
        with self.assertRaises(REPORT.ReportError) as caught:
            REPORT.load_document(path, max_bytes=max_bytes)
        self.assertEqual(caught.exception.code, "input-too-large")
        self.assertGreater(self._reads, 0)
        self.assertLessEqual(self._reads, max_bytes + 1)
        self.assertGreater(path.stat().st_size, max_bytes + 1)

    def test_replaced_file_is_not_read(self):
        path = self.root / "replace.json"
        original_bytes = b"{}"
        path.write_bytes(original_bytes)
        captured = path.lstat()
        retained = self.root / "replace-original.json"

        def swap(pathname, flags):
            target = Path(pathname)
            os.rename(target, retained)
            target.write_bytes(b"y" * 4096)
            return self._original_open(pathname, flags)

        REPORT.os.open = swap
        self._track_reads()
        with self.assertRaises(REPORT.ReportError) as caught:
            REPORT.load_document(path, max_bytes=128)
        self.assertEqual(caught.exception.code, "input-changed")
        self.assertEqual(self._reads, 0)
        self.assertEqual(retained.read_bytes(), original_bytes)
        self.assertEqual(retained.lstat().st_ino, captured.st_ino)
        self.assertTrue(path.is_file())
        self.assertNotEqual(path.lstat().st_ino, captured.st_ino)

    def test_fifo_replacement_does_not_block(self):
        path = self.root / "fifo-source.json"
        original_bytes = b"{}"
        path.write_bytes(original_bytes)
        captured = path.lstat()
        retained = self.root / "fifo-original.json"

        def swap(pathname, flags):
            target = Path(pathname)
            os.rename(target, retained)
            os.mkfifo(target)
            if not flags & os.O_NONBLOCK:
                raise AssertionError("refusing blocking open of replacement fifo")
            return self._original_open(pathname, flags)

        def count_fstat(fd):
            self._fstats += 1
            return self._original_fstat(fd)

        REPORT.os.open = swap
        REPORT.os.fstat = count_fstat
        self._track_reads()

        def bounded(_signum, _frame):
            raise OpenBoundExceeded("fifo open exceeded bound")

        previous = signal.signal(signal.SIGALRM, bounded)
        signal.alarm(2)
        started = time.monotonic()
        try:
            with self.assertRaises(REPORT.ReportError) as caught:
                REPORT.load_document(path, max_bytes=128)
        finally:
            signal.alarm(0)
            signal.signal(signal.SIGALRM, previous)
        self.assertLess(time.monotonic() - started, 1.0)
        self.assertEqual(caught.exception.code, "not-regular-file")
        self.assertEqual(self._reads, 0)
        self.assertGreaterEqual(self._fstats, 1)
        self.assertEqual(retained.read_bytes(), original_bytes)
        self.assertEqual(retained.lstat().st_ino, captured.st_ino)
        self.assertTrue(stat.S_ISFIFO(path.lstat().st_mode))
        self.assertNotEqual(path.lstat().st_ino, captured.st_ino)

    def test_symlink_and_directory_are_refused(self):
        target = self.root / "target.json"
        target.write_text("{}", encoding="utf-8")
        link = self.root / "link.json"
        link.symlink_to(target)
        directory = self.root / "directory.json"
        directory.mkdir()
        for candidate in (link, directory):
            with self.assertRaises(REPORT.ReportError) as caught:
                REPORT.load_document(candidate)
            self.assertEqual(caught.exception.code, "not-regular-file")

    def test_huge_integer_has_bounded_error(self):
        try:
            REPORT.finite_number(10**1000, "summary.cases[0].median_ms", nonnegative=True)
        except REPORT.ReportError as error:
            self.assertEqual(error.code, "malformed-report: summary.cases[0].median_ms")
            self.assertLess(len(error.code), 80)
            self.assertNotIn(str(10**1000), error.code)
        except OverflowError:
            self.fail("uncaught OverflowError")
        else:
            self.fail("huge integer was accepted")
        self.assertEqual(
            REPORT.finite_number(10**15, "summary.cases[0].median_ms", nonnegative=True),
            10**15)
        with self.assertRaises(REPORT.ReportError):
            REPORT.finite_number(10**15 + 1, "summary.cases[0].median_ms", nonnegative=True)

    def _cli(self, name, text):
        path = self.root / name
        path.write_text(text, encoding="utf-8")
        completed = subprocess.run(
            [sys.executable, str(MODULE_PATH), "--input", str(path), "--format", "json"],
            capture_output=True, text=True, timeout=15, cwd=str(REPO),
        )
        return completed

    def test_cli_rejects_huge_integer_without_traceback(self):
        payload = json.dumps(minimal_document(10**1000))
        completed = self._cli("huge.json", payload)
        self.assertEqual(completed.returncode, 1)
        self.assertEqual(
            completed.stderr,
            "runtime-measurement-report: malformed-report: summary.cases[0].median_ms\n",
        )
        self.assertEqual(completed.stdout, "")
        self.assertNotIn("Traceback", completed.stderr)
        self.assertNotIn("OverflowError", completed.stderr)
        self.assertNotIn("0" * 40, completed.stderr)

    def test_cli_rejects_exponent_overflow_without_token(self):
        completed = self._cli("exponent.json", '{"median_ms": 1e400}\n')
        self.assertEqual(completed.returncode, 1)
        self.assertEqual(completed.stderr, "runtime-measurement-report: non-finite-number\n")
        self.assertEqual(completed.stdout, "")
        self.assertNotIn("1e400", completed.stderr)
        self.assertNotIn("Traceback", completed.stderr)
        self.assertNotIn("inf", completed.stderr.lower())

    def test_cli_rejects_excessive_nesting_without_parser_text(self):
        completed = self._cli("nested.json", "[" * 3000 + "]" * 3000)
        self.assertEqual(completed.returncode, 1)
        self.assertEqual(completed.stderr, "runtime-measurement-report: invalid-json\n")
        self.assertEqual(completed.stdout, "")
        lowered = completed.stderr.lower()
        self.assertNotIn("recursion", lowered)
        self.assertNotIn("traceback", lowered)
        self.assertNotIn("[[[", completed.stderr)


def fresh_output(path):
    try:
        path.lstat()
    except FileNotFoundError:
        pass
    else:
        print("runtime-measurement-report protections: output-exists", file=sys.stderr)
        return None
    try:
        path.mkdir()
    except OSError:
        print("runtime-measurement-report protections: output-unavailable", file=sys.stderr)
        return None
    return path


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path, required=True)
    options, remaining = parser.parse_known_args(argv)
    if remaining:
        print("runtime-measurement-report protections: unexpected arguments", file=sys.stderr)
        return 2
    destination = fresh_output(options.out)
    if destination is None:
        return 2
    Protections.out = destination
    suite = unittest.defaultTestLoader.loadTestsFromTestCase(Protections)
    result = unittest.TextTestRunner(verbosity=2).run(suite)
    summary = {
        "tests": result.testsRun,
        "failures": len(result.failures),
        "errors": len(result.errors),
        "fixtures_retained": True,
    }
    with (destination / "result.json").open("x", encoding="utf-8") as stream:
        stream.write(json.dumps(summary, indent=2) + "\n")
    return 0 if result.wasSuccessful() else 1


if __name__ == "__main__":
    raise SystemExit(main())
