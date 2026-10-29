"""Post-integration component protection for pinned installed-product contexts."""
from pathlib import Path
import argparse
import copy
import importlib.util
import json
import sys
import time
import unittest

REPO = Path(__file__).resolve().parent.parent


def module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    value = importlib.util.module_from_spec(spec)
    sys.modules[name] = value
    spec.loader.exec_module(value)
    return value


api = module('stable_context_test_suite', REPO/'tools/stable-suite.py')
context = module('stable_context_test_provider', REPO/'tools/stable-context.py')


class ContextProtection(unittest.TestCase):
    def setUp(self):
        self.root = OUT/self.id().split('.')[-1]
        self.root.mkdir()
        self.manifest_path = self.root/'compatibility/stable-31.1/source-context.json'
        self.manifest_path.parent.mkdir(parents=True)
        self.original = json.loads((REPO/'compatibility/stable-31.1/source-context.json').read_text())
        self.corpus = json.loads((REPO/'compatibility/stable-31.1/sources.json').read_text())
        self.write(self.original)

    def write(self, value):
        self.manifest_path.write_text(json.dumps(value))

    def test_pinned_manifest_accepted(self):
        value, identity = context.metadata(api, self.root, self.corpus)
        self.assertEqual(value['selection_sha256'], context.SELECTION)
        self.assertEqual(identity, api._sha256_path(self.manifest_path))

    def test_changed_version_rejected(self):
        value = copy.deepcopy(self.original)
        value['source']['commit'] = '0'*40
        self.write(value)
        with self.assertRaisesRegex(api.SuiteError, 'version differs'):
            context.metadata(api, self.root, self.corpus)

    def test_changed_content_identity_rejected(self):
        value = copy.deepcopy(self.original)
        value['entries'][0]['sha256'] = '0'*64
        self.write(value)
        with self.assertRaisesRegex(api.SuiteError, 'complete pinned selection'):
            context.metadata(api, self.root, self.corpus)

    def test_duplicate_source_path_rejected(self):
        value = copy.deepcopy(self.original)
        value['entries'].append(copy.deepcopy(value['entries'][0]))
        self.write(value)
        with self.assertRaisesRegex(api.SuiteError, 'duplicate'):
            context.metadata(api, self.root, self.corpus)

    def test_path_traversal_rejected(self):
        value = copy.deepcopy(self.original)
        value['entries'][0]['path'] = 'src/../../outside'
        self.write(value)
        with self.assertRaises(api.SuiteError):
            context.metadata(api, self.root, self.corpus)

    def test_manifest_symlink_rejected(self):
        saved = self.manifest_path.with_suffix('.saved')
        self.manifest_path.rename(saved)
        self.manifest_path.symlink_to(saved.name)
        with self.assertRaisesRegex(api.SuiteError, 'symlink'):
            context.metadata(api, self.root, self.corpus)


class TimedResult(unittest.TextTestResult):
    def startTest(self, test):
        self.started = time.monotonic()
        super().startTest(test)

    def stopTest(self, test):
        self.timings.append(dict(id=test.id(), seconds=time.monotonic()-self.started))
        super().stopTest(test)


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--out', type=Path, required=True)
    args = parser.parse_args()
    OUT = args.out.resolve()
    if not OUT.is_relative_to(REPO/'build') or OUT.exists():
        raise SystemExit('fresh owned build output required')
    OUT.mkdir(parents=True)
    TimedResult.timings = []
    result = unittest.TextTestRunner(verbosity=2, resultclass=TimedResult).run(
        unittest.defaultTestLoader.loadTestsFromTestCase(ContextProtection))
    (OUT/'results.json').write_text(json.dumps(dict(passed=result.wasSuccessful(),
        cases=TimedResult.timings, count=result.testsRun, failures=len(result.failures),
        errors=len(result.errors), scope='component protection; no editor executed'), indent=2)+'\n')
    raise SystemExit(0 if result.wasSuccessful() else 1)
