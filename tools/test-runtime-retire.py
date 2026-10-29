"""Protective ownership races after cleanup integration; tiny owned fixtures."""
from pathlib import Path
import argparse
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
    result = unittest.TextTestRunner(stream=stream, verbosity=2, resultclass=TimedResult).run(
        unittest.defaultTestLoader.loadTestsFromTestCase(OwnershipTests))
    (OUT / 'unittest.txt').write_text(stream.getvalue())
    (OUT / 'results.json').write_text(json.dumps(dict(passed=result.wasSuccessful(),
        tests=result.testsRun, source_sha256=RETIRE.digest(args.module), cases=result.timings,
        failures=[test.id() for test, _ in result.failures],
        errors=[test.id() for test, _ in result.errors], editor_accepted=False), indent=2) + '\n')
    print(stream.getvalue(), end='')
    return 0 if result.wasSuccessful() else 1


if __name__ == '__main__':
    raise SystemExit(main())
