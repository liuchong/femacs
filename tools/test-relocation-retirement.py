"""Protect pre-delete sync failures after real filesystem integration passed."""
from pathlib import Path
import errno
import importlib.util
import os
import time
import unittest
from unittest.mock import patch
import uuid

ROOT = Path(__file__).resolve().parent.parent
SPEC = importlib.util.spec_from_file_location('retirement_execution', ROOT/'tools/runtime-execution.py')
EXECUTION = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(EXECUTION)


class RetirementSyncFailures(unittest.TestCase):
    def setUp(self):
        self.root = ROOT/'build'/('relocation-retirement-unit-'+uuid.uuid4().hex)
        self.root.mkdir()
        self.package = self.root/'terminal'
        self.package.mkdir()
        self.payload = self.package/'payload.bin'
        self.payload.write_bytes(b'keep this owned payload on pre-delete failure')
        self.evidence = self.root/'evidence'
        self.evidence.mkdir()
        info = self.package.stat()
        self.owner = info.st_dev, info.st_ino

    def reject_sync(self, target_name):
        actual_sync = os.fsync
        failed = []

        def fail_selected(descriptor):
            target = self.evidence/target_name
            if target.exists():
                observed, expected = os.fstat(descriptor), target.stat()
                if (observed.st_dev, observed.st_ino) == (expected.st_dev, expected.st_ino):
                    failed.append(target_name)
                    raise OSError(errno.EIO, 'injected pre-delete persistence failure')
            return actual_sync(descriptor)

        with patch.object(EXECUTION.os, 'fsync', side_effect=fail_selected), \
                patch.object(EXECUTION.shutil, 'rmtree') as removal:
            with self.assertRaises(OSError):
                EXECUTION.retire_relocation_copy(self.package, self.root, self.owner,
                                                  evidence_root=self.evidence)
            removal.assert_not_called()
        self.assertEqual(failed, [target_name])
        self.assertTrue(self.package.is_dir())
        self.assertEqual(self.payload.read_bytes(), b'keep this owned payload on pre-delete failure')

    def test_manifest_sync_failure_preserves_copy(self):
        self.reject_sync('consumed-package.jsonl.gz')

    def test_lifecycle_sync_failure_preserves_copy(self):
        self.reject_sync('package-copy-lifecycle.json')

    def test_unlink_intent_sync_failure_preserves_copy(self):
        self.reject_sync('package-copy-operations.jsonl')


class TimedResult(unittest.TextTestResult):
    def startTest(self, test):
        self.started = time.monotonic()
        super().startTest(test)

    def stopTest(self, test):
        self.stream.writeln(f'{test.id()} {time.monotonic()-self.started:.3f}s')
        super().stopTest(test)


if __name__ == '__main__':
    unittest.main(testRunner=unittest.TextTestRunner(resultclass=TimedResult, verbosity=2))
