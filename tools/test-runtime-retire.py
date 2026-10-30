"""Protective ownership races after cleanup integration; tiny owned fixtures."""
from pathlib import Path
import argparse
import gzip
import importlib.util
import io
import json
import os
import subprocess
import time
import unittest
from unittest import mock

RETIRE = None
OUT = None


class OwnershipTests(unittest.TestCase):
    def setUp(self):
        self.root = OUT / self._testMethodName
        self.root.mkdir()
        self.cache = self.root / 'cache'
        self.cache.mkdir(mode=0o700)
        self.evidence = self.root / 'result.json'
        self.evidence.write_text('{"complete":true}\n')
        self.process = dict(complete=True, exit_code=0, remaining_group_terminated=False)
        self.digest = RETIRE.digest

    def retire(self):
        return RETIRE.retire(self.root, ['cache'], self.evidence, self.process)

    def before_delete(self, action):
        calls = 0

        def digest(path):
            nonlocal calls
            if Path(path) == self.evidence:
                calls += 1
                if calls == 2:
                    action()
            return self.digest(path)

        return mock.patch.object(RETIRE, 'digest', side_effect=digest)

    def test_replaced_empty_directory_is_retained(self):
        original = self.root / 'original-cache'

        def replace():
            self.cache.rename(original)
            self.cache.mkdir()

        with self.before_delete(replace):
            with self.assertRaisesRegex(ValueError, 'directory changed'):
                self.retire()
        self.assertTrue(self.cache.is_dir())
        self.assertTrue(original.is_dir())

    def test_directory_mode_change_is_retained(self):
        with self.before_delete(lambda: self.cache.chmod(0o750)):
            with self.assertRaisesRegex(ValueError, 'directory changed'):
                self.retire()
        self.assertEqual(self.cache.stat().st_mode & 0o777, 0o750)

    def test_symlink_replacement_preserves_target(self):
        target = self.root / 'target'

        def replace():
            self.cache.rename(target)
            self.cache.symlink_to(target, target_is_directory=True)

        with self.before_delete(replace):
            with self.assertRaisesRegex(ValueError, 'directory changed'):
                self.retire()
        self.assertTrue(self.cache.is_symlink())
        self.assertTrue(target.is_dir())

    def test_new_unplanned_entry_is_retained(self):
        extra = self.cache / 'not-in-plan'
        with self.before_delete(lambda: extra.write_bytes(b'new owner')):
            with self.assertRaises(OSError):
                self.retire()
        self.assertEqual(extra.read_bytes(), b'new owner')

    def test_failed_process_never_removes_state(self):
        self.process['exit_code'] = 1
        with mock.patch.object(RETIRE.subprocess, 'run') as query:
            self.assertEqual(self.retire()['status'], 'retained')
            query.assert_not_called()
        self.assertTrue(self.cache.is_dir())

    def test_open_handles_retain_state(self):
        occupied = subprocess.CompletedProcess([], 0, stdout=b'open', stderr=b'')
        with mock.patch.object(RETIRE.subprocess, 'run', return_value=occupied):
            self.assertEqual(self.retire()['status'], 'retained')
        self.assertTrue(self.cache.is_dir())

    def test_occupancy_timeout_retains_without_retry(self):
        with mock.patch.object(RETIRE.subprocess, 'run',
                side_effect=subprocess.TimeoutExpired('lsof', 10)) as query:
            self.assertEqual(self.retire()['status'], 'retained')
            self.assertEqual(query.call_count, 1)
            self.assertEqual(query.call_args.kwargs['timeout'], 10)
        self.assertTrue(self.cache.is_dir())


class ParentLedgerTests(unittest.TestCase):
    def setUp(self):
        self.root = OUT / self._testMethodName
        self.case = self.root / 'container' / 'case'
        self.cache = self.case / 'cache'
        self.cache.mkdir(parents=True)
        self.payload = self.cache / 'payload'
        self.payload.write_bytes(b'owned fixture')
        self.parents = ((self.case, self.case.lstat()),
                        (self.case.parent, self.case.parent.lstat()))
        self.evidence = self.root / 'result.json'
        self.evidence.write_text('{"complete":true}\n')
        self.process = dict(complete=True, exit_code=0, remaining_group_terminated=False)
        self.private = self.root / 'result.state-retirement.private.json'
        self.ledger = self.root / 'result.state-retirement.jsonl.gz'

    def retire(self):
        return RETIRE.retire(self.case, ['cache'], self.evidence, self.process,
                             empty_parents=self.parents)

    def test_parent_identities_are_durable_before_first_unlink(self):
        unlink, fsync = Path.unlink, os.fsync
        synced = []
        def sync(fd):
            fsync(fd)
            synced.append(fd)
        def remove(path, *args, **kwargs):
            plan = json.loads(self.private.read_text())
            with gzip.open(self.ledger, 'rt') as stream:
                public = [json.loads(line) for line in stream]
            parents = [row for row in plan['directories'] if row.get('empty_parent')]
            self.assertEqual([row['path'] for row in parents], ['.', '..'])
            self.assertTrue(all({'device', 'inode', 'mtime_ns', 'mode'} <= set(row)
                                for row in parents))
            self.assertEqual(sum(row.get('empty_parent', False) for row in public), 2)
            self.assertGreaterEqual(len(synced), 3)  # ledger, private plan, their directory
            return unlink(path, *args, **kwargs)
        before = self.evidence.read_bytes()
        with mock.patch.object(RETIRE.os, 'fsync', side_effect=sync), \
                mock.patch.object(Path, 'unlink', new=remove):
            result = self.retire()
        self.assertEqual(result['directories'], 3)  # cache plus both parents, once
        self.assertEqual(result['parent_directories'], 2)
        self.assertEqual(result['ledger_schema'], 3)
        self.assertFalse(self.case.parent.exists())
        self.assertEqual(self.evidence.read_bytes(), before)

    def test_replaced_container_preserves_both_owners(self):
        original = self.root / 'original'
        self.case.parent.rename(original)
        self.payload.parent.mkdir(parents=True)
        self.payload.write_bytes(b'new owner')
        with self.assertRaisesRegex(ValueError, 'parent changed'):
            self.retire()
        self.assertEqual(self.payload.read_bytes(), b'new owner')
        self.assertEqual((original / 'case/cache/payload').read_bytes(), b'owned fixture')
        self.assertFalse(self.ledger.exists())

    def test_unselected_sibling_preserves_all_state(self):
        sibling = self.case.parent / 'unexpected'
        sibling.write_bytes(b'other owner')
        with self.assertRaisesRegex(ValueError, 'unselected'):
            self.retire()
        self.assertEqual(sibling.read_bytes(), b'other owner')
        self.assertTrue(self.payload.is_file())
        self.assertFalse(self.ledger.exists())

    def test_evidence_cannot_be_inside_selected_container(self):
        self.evidence = self.case / 'result.json'
        self.evidence.write_bytes(b'preserve evidence')
        with self.assertRaisesRegex(ValueError, 'evidence location'):
            self.retire()
        self.assertEqual(self.evidence.read_bytes(), b'preserve evidence')
        self.assertTrue(self.payload.is_file())

    def test_failed_durable_write_never_deletes_payload(self):
        with mock.patch.object(RETIRE.os, 'fsync', side_effect=OSError('injected fsync failure')):
            with self.assertRaises(OSError):
                self.retire()
        self.assertEqual(self.payload.read_bytes(), b'owned fixture')
        self.assertTrue(self.case.parent.is_dir())
        self.assertFalse((self.root / 'result.state-retirement.json').exists())

    def test_parent_occupancy_checks_entire_owned_container(self):
        occupied = subprocess.CompletedProcess([], 0, stdout=b'open parent', stderr=b'')
        with mock.patch.object(RETIRE.subprocess, 'run', return_value=occupied) as query:
            self.assertEqual(self.retire()['status'], 'retained')
        self.assertEqual(query.call_args.args[0], ['lsof', '+D', str(self.case.parent)])
        self.assertTrue(self.payload.is_file())
        self.assertFalse(self.ledger.exists())

    def test_partial_parent_removal_keeps_plan_without_completion_claim(self):
        rmdir = Path.rmdir
        container = self.case.parent
        def remove(path):
            if path == container:
                raise OSError('injected final parent failure')
            return rmdir(path)
        with mock.patch.object(Path, 'rmdir', new=remove):
            with self.assertRaises(OSError):
                self.retire()
        self.assertFalse(self.case.exists())
        self.assertTrue(container.is_dir())
        self.assertEqual(len(json.loads(self.private.read_text())['directories']), 3)
        self.assertTrue(self.ledger.is_file())
        self.assertFalse((self.root / 'result.state-retirement.json').exists())


class TimedResult(unittest.TextTestResult):
    def startTest(self, test):
        self.started = time.monotonic()
        super().startTest(test)

    def stopTest(self, test):
        self.timings.append(dict(id=test.id(), seconds=time.monotonic() - self.started))
        super().stopTest(test)


def main():
    global OUT, RETIRE
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out', type=Path, required=True)
    parser.add_argument('--module', type=Path, default=Path(__file__).with_name('runtime-retire.py'))
    args = parser.parse_args()
    OUT = args.out.absolute()
    if OUT.exists() or OUT.resolve() != OUT:
        parser.error('fresh owned output required')
    OUT.mkdir(parents=True)
    spec = importlib.util.spec_from_file_location('retire_protection', args.module)
    RETIRE = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(RETIRE)
    stream = io.StringIO()
    TimedResult.timings = []
    suite = unittest.TestSuite(unittest.defaultTestLoader.loadTestsFromTestCase(case)
                               for case in (OwnershipTests, ParentLedgerTests))
    result = unittest.TextTestRunner(stream=stream, verbosity=2, resultclass=TimedResult).run(suite)
    (OUT / 'unittest.txt').write_text(stream.getvalue())
    (OUT / 'results.json').write_text(json.dumps(dict(passed=result.wasSuccessful(),
        tests=result.testsRun, source_sha256=RETIRE.digest(args.module), cases=result.timings,
        failures=[test.id() for test, _ in result.failures],
        errors=[test.id() for test, _ in result.errors], editor_accepted=False), indent=2) + '\n')
    print(stream.getvalue(), end='')
    return 0 if result.wasSuccessful() else 1


if __name__ == '__main__':
    raise SystemExit(main())
