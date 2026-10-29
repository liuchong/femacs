"""File-boundary regressions for retaining the runtime matrix build owner.

These cases copy synthetic bytes without executing them. Real compiler and
product acceptance remain independent. Preserve per-case timing under build.
"""
import importlib.util
import json
import os
from pathlib import Path
import sys
import tempfile
import time
import unittest
from unittest import mock

REPO = Path(__file__).resolve().parent.parent
spec = importlib.util.spec_from_file_location("runtime_stage_contracts", REPO / "tools/runtime-stage.py")
stage = importlib.util.module_from_spec(spec)
spec.loader.exec_module(stage)
OUT = Path(os.environ.get("FEMACS_RUNTIME_TEST_ROOT", REPO / "build/acceptance/test-runtime-stage"))
ROWS = []


class BuildOwnerContracts(unittest.TestCase):
    def setUp(self):
        OUT.mkdir(parents=True, exist_ok=True)
        self.root = Path(tempfile.mkdtemp(prefix=self._testMethodName + "-", dir=OUT))
        self.source = self.root / "cache-tool"
        self.source.write_bytes(b"synthetic executable input\n")
        self.source.chmod(0o755)

    def test_captured_owner_survives_original_cache_removal(self):
        owner, identity = stage.stage_build_owner(self.source, self.root)
        self.source.unlink()
        stage.verify_build_owner(owner, identity)
        self.assertEqual(owner.read_bytes(), b"synthetic executable input\n")
        self.assertTrue(os.access(owner, os.X_OK))
        receipt = json.loads((owner.parent / "receipt.json").read_text())
        self.assertEqual(receipt["sha256"], identity)
        self.assertEqual(receipt["path"], "runtime-build-driver")

    def test_owner_survives_generic_cache_and_legacy_owner_removal(self):
        owner, identity = stage.stage_build_owner(self.source, self.root)
        self.assertFalse(owner.is_relative_to(REPO / "build"))
        legacy = self.root / "build-owner"
        legacy.mkdir()
        old_tool = legacy / "xtask"
        old_tool.write_bytes(b"legacy owner\n")
        old_tool.unlink()
        self.source.unlink()
        stage.verify_build_owner(owner, identity)
        receipt = json.loads((self.root / "build-owner-receipt.json").read_text())
        self.assertEqual(receipt, json.loads((owner.parent / "receipt.json").read_text()))
        self.assertEqual(receipt["sha256"], identity)

    def test_separate_outputs_never_share_the_retained_driver(self):
        other = self.root / "other-output"
        other.mkdir()
        other_source = other / "cache-tool"
        other_source.write_bytes(self.source.read_bytes())
        other_source.chmod(0o755)
        first, identity = stage.stage_build_owner(self.source, self.root)
        second, second_identity = stage.stage_build_owner(other_source, other)
        self.assertNotEqual(first, second)
        self.assertEqual(identity, second_identity)
        first.write_bytes(b"first owner changed\n")
        stage.verify_build_owner(second, second_identity)
        with self.assertRaises(ValueError):
            stage.verify_build_owner(first, identity)

    def test_retained_owner_rejects_output_outside_owned_build(self):
        with mock.patch.object(stage, "REPO", self.root):
            with self.assertRaisesRegex(ValueError, "owned build"):
                stage.stage_build_owner(self.source, self.root)

    def test_retained_owner_rejects_symlinked_cache_parent(self):
        out = self.root / "build/output"
        out.mkdir(parents=True)
        target = self.root / "cache-target"
        target.mkdir()
        (self.root / ".femacs-cache").symlink_to(target.name)
        with mock.patch.object(stage, "REPO", self.root):
            with self.assertRaisesRegex(ValueError, "symlink"):
                stage.stage_build_owner(self.source, out)

    def test_relative_output_locator_preserves_capture_identity(self):
        relative = Path(os.path.relpath(self.root))
        owner, identity = stage.stage_build_owner(self.source, relative)
        self.assertTrue(owner.is_absolute())
        stage.verify_build_owner(owner, identity)
        self.assertEqual(owner.read_bytes(), b"synthetic executable input\n")

    def test_changed_and_missing_captured_owners_are_rejected(self):
        owner, identity = stage.stage_build_owner(self.source, self.root)
        owner.write_bytes(b"changed bytes\n")
        with self.assertRaises(ValueError):
            stage.verify_build_owner(owner, identity)
        owner.unlink()
        with self.assertRaises(ValueError):
            stage.verify_build_owner(owner, identity)

    def test_reused_destination_never_overwrites_captured_owner(self):
        owner, identity = stage.stage_build_owner(self.source, self.root)
        self.source.write_bytes(b"different source\n")
        with self.assertRaises(OSError):
            stage.stage_build_owner(self.source, self.root)
        stage.verify_build_owner(owner, identity)

    def test_symlink_and_nonexecutable_inputs_are_rejected(self):
        self.source.chmod(0o644)
        with self.assertRaises(ValueError):
            stage.stage_build_owner(self.source, self.root)
        self.source.chmod(0o755)
        link = self.root / "tool-link"
        link.symlink_to(self.source.name)
        with self.assertRaises(ValueError):
            stage.stage_build_owner(link, self.root)
        real = self.root / "real-parent"
        real.mkdir()
        parent_link = self.root / "parent-link"
        parent_link.symlink_to(real.name)
        with self.assertRaises(ValueError):
            stage.stage_build_owner(self.source, parent_link)

    def test_source_change_during_capture_is_rejected(self):
        original_digest = stage.execution.digest
        source_reads = 0

        def change_after_first_read(path):
            nonlocal source_reads
            if Path(path).absolute() == self.source.absolute():
                source_reads += 1
                if source_reads == 2:
                    self.source.write_bytes(b"source changed during capture\n")
            return original_digest(path)

        with mock.patch.object(stage.execution, "digest", side_effect=change_after_first_read):
            with self.assertRaisesRegex(ValueError, "input changed during capture"):
                stage.stage_build_owner(self.source, self.root)
        self.assertGreaterEqual(source_reads, 2)
        self.assertEqual(self.source.read_bytes(), b"source changed during capture\n")

    def test_missing_compiled_owner_retains_partial_without_retry(self):
        out = self.root / "matrix"
        out.mkdir()
        source = out / "source"
        source.mkdir()
        export = dict(source=str(source), passed=True)
        compiled = dict(exit_code=0, complete=True)
        with mock.patch.object(stage.execution, "freeze_source", return_value=export), \
             mock.patch.object(stage.execution, "run_process", return_value=compiled) as run:
            result = stage.build_matrix(None, out)
        self.assertFalse(result["passed"])
        self.assertEqual(run.call_count, 1)
        partial = json.loads((out / "partial.json").read_text())
        self.assertEqual(partial["source_export"], export)
        self.assertEqual(partial["xtask_build"], compiled)
        self.assertEqual(partial["builds"], [])

    def test_capture_failure_retains_partial_without_recompiling(self):
        out = self.root / "matrix"
        out.mkdir()
        source = out / "source"
        source.mkdir()
        cache = out / "cargo/debug"
        cache.mkdir(parents=True)
        tool = cache / "xtask"
        tool.write_bytes(b"synthetic cache owner\n")
        tool.chmod(0o755)
        export = dict(source=str(source), passed=True)
        compiled = dict(exit_code=0, complete=True)
        with mock.patch.object(stage.execution, "freeze_source", return_value=export), \
             mock.patch.object(stage.execution, "run_process", return_value=compiled) as run, \
             mock.patch.object(stage, "stage_build_owner", side_effect=ValueError("capture changed")):
            with self.assertRaises(ValueError):
                stage.build_matrix(None, out)
        self.assertEqual(run.call_count, 1)
        partial = json.loads((out / "partial.json").read_text())
        self.assertEqual(partial["source_export"], export)
        self.assertEqual(partial["xtask_build"], compiled)
        self.assertEqual(partial["builds"], [])


class TimedResult(unittest.TextTestResult):
    def startTest(self, test):
        super().startTest(test)
        self.started = time.perf_counter()
        self.current_status = "incomplete"

    def addSuccess(self, test):
        self.current_status = "passed"
        super().addSuccess(test)

    def addFailure(self, test, error):
        self.current_status = "failed"
        super().addFailure(test, error)

    def addError(self, test, error):
        self.current_status = "error"
        super().addError(test, error)

    def stopTest(self, test):
        ROWS.append(dict(id=test.id(), status=self.current_status,
                         seconds=time.perf_counter() - self.started))
        super().stopTest(test)


if __name__ == "__main__":
    suite = unittest.defaultTestLoader.loadTestsFromTestCase(BuildOwnerContracts)
    result = unittest.TextTestRunner(verbosity=2, resultclass=TimedResult).run(suite)
    OUT.mkdir(parents=True, exist_ok=True)
    passed = result.wasSuccessful() and len(ROWS) == 12 and all(row["status"] == "passed" for row in ROWS)
    (OUT / "results.json").write_text(json.dumps(dict(cases=ROWS, passed=passed), indent=2) + "\n")
    raise SystemExit(0 if passed else 1)
