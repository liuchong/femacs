#!/usr/bin/env python3
"""Unit tests for tools/pgo_bench.py.

Run only in the centralized acceptance pass (PGO plan, R39); these tests do not
compile or execute the port.
"""
import hashlib
import os
import tempfile
import unittest

import pgo_bench


class MarkerParsing(unittest.TestCase):
    def test_reads_each_case_once(self):
        text = "noise\nBENCH bc-loop 1.5 ms\nBENCH regexp 2.0 ms\n"
        self.assertEqual(pgo_bench.parse_markers(text, "BENCH", ["bc-loop", "regexp"]),
                         {"bc-loop": 1.5, "regexp": 2.0})

    def test_missing_case_is_an_error(self):
        with self.assertRaises(pgo_bench.MeasurementError):
            pgo_bench.parse_markers("BENCH bc-loop 1.0 ms\n", "BENCH", ["bc-loop", "regexp"])

    def test_duplicate_case_is_an_error(self):
        text = "BENCH bc-loop 1.0 ms\nBENCH bc-loop 2.0 ms\n"
        with self.assertRaises(pgo_bench.MeasurementError):
            pgo_bench.parse_markers(text, "BENCH", ["bc-loop"])

    def test_non_finite_is_an_error(self):
        with self.assertRaises(pgo_bench.MeasurementError):
            pgo_bench.parse_markers("BENCH bc-loop nan ms\n", "BENCH", ["bc-loop"])


class Statistics(unittest.TestCase):
    def test_median_mad_iqr(self):
        xs = [1.0, 2.0, 3.0, 4.0, 100.0]
        self.assertEqual(pgo_bench.median(xs), 3.0)
        self.assertEqual(pgo_bench.mad(xs), 1.0)
        self.assertAlmostEqual(pgo_bench.iqr(xs), 2.0)

    def test_summary_keeps_every_case_and_side(self):
        rows = [{"case": "a", "side": "o2", "value_ms": v} for v in (1.0, 2.0, 3.0)]
        rows += [{"case": "a", "side": "pgo", "value_ms": v} for v in (0.5, 1.0, 1.5)]
        summary = pgo_bench.summarize(rows, ["a"], ["o2", "pgo"])
        self.assertEqual(len(summary), 2)
        self.assertEqual(summary[0]["n"], 3)
        self.assertEqual(summary[0]["min"], 1.0)
        self.assertEqual(summary[1]["median"], 1.0)

    def test_missing_side_in_summary_is_an_error(self):
        rows = [{"case": "a", "side": "o2", "value_ms": 1.0}]
        with self.assertRaises(pgo_bench.MeasurementError):
            pgo_bench.summarize(rows, ["a"], ["o2", "pgo"])


class PairVerification(unittest.TestCase):
    def _write(self, path, data):
        with open(path, "wb") as f:
            f.write(data)
        return hashlib.sha256(data).hexdigest()

    def test_accepts_matching_manifest(self):
        with tempfile.TemporaryDirectory() as d:
            src = os.path.join(d, "port", "src")
            os.makedirs(src)
            evidence = os.path.join(src, "pair.sha256")
            digests = []
            for name in ("temacs", "emacs", "emacs.pdmp", "bootstrap-emacs.pdmp"):
                digests.append((name, self._write(os.path.join(src, name), name.encode())))
            with open(evidence, "w", encoding="utf-8") as f:
                for name, digest in digests:
                    f.write(f"{digest}  {name}\n")
            pgo_bench.verify_pair(d)

    def test_rejects_modified_file(self):
        with tempfile.TemporaryDirectory() as d:
            src = os.path.join(d, "port", "src")
            os.makedirs(src)
            names = ("temacs", "emacs", "emacs.pdmp", "bootstrap-emacs.pdmp")
            digests = [(n, self._write(os.path.join(src, n), n.encode())) for n in names]
            with open(os.path.join(src, "pair.sha256"), "w", encoding="utf-8") as f:
                for name, digest in digests:
                    f.write(f"{digest}  {name}\n")
            with open(os.path.join(src, "emacs.pdmp"), "ab") as f:
                f.write(b"stale")
            with self.assertRaises(pgo_bench.MeasurementError):
                pgo_bench.verify_pair(d)

    def test_rejects_missing_manifest_row(self):
        with tempfile.TemporaryDirectory() as d:
            src = os.path.join(d, "port", "src")
            os.makedirs(src)
            self._write(os.path.join(src, "temacs"), b"x")
            with open(os.path.join(src, "pair.sha256"), "w", encoding="utf-8") as f:
                f.write(hashlib.sha256(b"x").hexdigest() + "  temacs\n")
            with self.assertRaises(pgo_bench.MeasurementError):
                pgo_bench.verify_pair(d)


class PlaceholderScan(unittest.TestCase):
    def test_finds_placeholder_across_chunk_boundary(self):
        marker = bytes(range(32))
        with tempfile.TemporaryDirectory() as d:
            path = os.path.join(d, "blob")
            with open(path, "wb") as f:
                f.write(b"x" * (pgo_bench.SCAN_CHUNK - 7))
                f.write(marker)
                f.write(b"y")
            self.assertEqual(pgo_bench.count_placeholder(path, marker), 1)

    def test_reports_multiple_occurrences(self):
        marker = b"0123456789abcdef0123456789abcdef"
        with tempfile.TemporaryDirectory() as d:
            path = os.path.join(d, "blob")
            with open(path, "wb") as f:
                f.write(marker + b"gap" + marker)
            self.assertEqual(pgo_bench.count_placeholder(path, marker), 2)


if __name__ == "__main__":
    unittest.main()
