"""Protect pre-delete sync failures after real filesystem integration passed."""
from pathlib import Path
import errno
import importlib.util
import json
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

    def test_private_identity_sync_failure_preserves_copy(self):
        self.reject_sync('package-copy-node-identities.private.json')

    def test_member_and_directory_times_are_durable_before_first_unlink(self):
        nested = self.package/'nested'
        nested.mkdir()
        (nested/'alias').symlink_to('../payload.bin')
        members = {'payload.bin': self.payload.lstat().st_mtime_ns,
                   'nested/alias': (nested/'alias').lstat().st_mtime_ns}
        directories = {'.': self.package.lstat().st_mtime_ns,
                       'nested': nested.lstat().st_mtime_ns}
        synced, removed = [], []
        actual_sync, actual_unlink = EXECUTION.sync_case_record, Path.unlink

        def sync(path):
            actual_sync(path)
            synced.append(Path(path).name)

        def unlink(path, *args, **kwargs):
            if path.is_relative_to(self.package):
                self.assertIn('package-copy-node-identities.private.json', synced)
                plan = json.loads((self.evidence/'package-copy-node-identities.private.json').read_text())
                self.assertEqual(plan['member_mtime_ns'], members)
                self.assertEqual(plan['directory_mtime_ns'], directories)
                removed.append(path)
            return actual_unlink(path, *args, **kwargs)

        with patch.object(EXECUTION, 'sync_case_record', side_effect=sync), \
                patch.object(Path, 'unlink', unlink):
            result = EXECUTION.retire_relocation_copy(self.package, self.root, self.owner,
                                                       evidence_root=self.evidence)
        self.assertTrue(result['complete'])
        self.assertEqual(len(removed), 2)
        self.assertFalse(self.package.exists())
        self.assertTrue(self.root.is_dir())

    def reject_time_change(self, directory=False, at_unlink=False):
        target = self.package if directory else self.payload
        original = target.lstat()
        injected = []
        actual_sync, actual_fsync = EXECUTION.sync_case_record, os.fsync

        def change():
            if not injected:
                os.utime(target, ns=(original.st_atime_ns, original.st_mtime_ns+1_000_000_000))
                injected.append(True)

        def sync(path):
            actual_sync(path)
            if Path(path).name == 'package-copy-lifecycle.json':
                change()

        def fsync(descriptor):
            actual_fsync(descriptor)
            operations = self.evidence/'package-copy-operations.jsonl'
            if operations.exists():
                current, expected = os.fstat(descriptor), operations.stat()
                if (current.st_dev, current.st_ino) == (expected.st_dev, expected.st_ino):
                    change()

        with patch.object(EXECUTION, 'sync_case_record', side_effect=actual_sync if at_unlink else sync), \
                patch.object(EXECUTION.os, 'fsync', side_effect=fsync if at_unlink else actual_fsync):
            with self.assertRaisesRegex(ValueError, 'before unlink' if at_unlink else 'after its durable manifest'):
                EXECUTION.retire_relocation_copy(self.package, self.root, self.owner,
                                                  evidence_root=self.evidence)
        self.assertEqual(injected, [True])
        self.assertEqual(self.payload.read_bytes(), b'keep this owned payload on pre-delete failure')
        lifecycle = json.loads((self.evidence/'package-copy-lifecycle.json').read_text())
        self.assertFalse(lifecycle['complete'])
        self.assertEqual(lifecycle['deleted_files'], 0)

    def test_unchanged_bytes_with_new_member_time_are_retained(self):
        self.reject_time_change()

    def test_changed_directory_time_before_deletion_is_retained(self):
        self.reject_time_change(directory=True)

    def test_member_time_change_at_unlink_intent_is_retained(self):
        self.reject_time_change(at_unlink=True)


class TimedResult(unittest.TextTestResult):
    def startTest(self, test):
        self.started = time.monotonic()
        super().startTest(test)

    def stopTest(self, test):
        self.stream.writeln(f'{test.id()} {time.monotonic()-self.started:.3f}s')
        super().stopTest(test)


if __name__ == '__main__':
    unittest.main(testRunner=unittest.TextTestRunner(resultclass=TimedResult, verbosity=2))
