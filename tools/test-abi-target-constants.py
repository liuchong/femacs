"""Mutation checks for target-dependent event numbering and portable doc examples."""
import importlib.util
from pathlib import Path
import unittest

spec = importlib.util.spec_from_file_location('abi', Path(__file__).with_name('extract-abi.py'))
abi = importlib.util.module_from_spec(spec)
spec.loader.exec_module(abi)

class TargetConstants(unittest.TestCase):
    constants = {
        'CONFIG': '27 + cfg!(feature = "threads_enabled") as c_int',
        'SLEEP': 'CONFIG + 1 + 2 * cfg!(feature = "have_ns") as c_int',
    }
    def test_target_changes_numbering(self):
        for macros, expected in [({}, 28), ({'HAVE_NS': 'undef'}, 28), ({'HAVE_NS': '1'}, 30),
                                 ({'HAVE_NS': '1', 'THREADS_ENABLED': '1'}, 31)]:
            self.assertEqual(abi.rust_integer_value('SLEEP', self.constants, macros), expected)
    def test_off_by_one_is_detected(self):
        self.assertNotEqual(abi.rust_integer_value('SLEEP + 1', self.constants, {}), 28)
    def test_unknown_target_and_executable_source_are_not_proof(self):
        self.assertIsNone(abi.rust_integer_value('SLEEP', self.constants, None))
        self.assertIsNone(abi.rust_integer_value('dangerous()', self.constants, {}))
        self.assertIsNone(abi.rust_integer_value('A', {'A': 'B', 'B': 'A'}, {}))
    def test_portable_doc_example_still_checks_remaining_text(self):
        import os
        root = str(Path(os.sep) / 'dev')
        self.assertEqual(abi.digest('${DEVICE_ROOT}/pts/5'), abi.digest(root + '/pts/5'))
        self.assertNotEqual(abi.digest('${DEVICE_ROOT}/pts/6'), abi.digest(root + '/pts/5'))

if __name__ == '__main__': unittest.main()
