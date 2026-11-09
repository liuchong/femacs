"""Post-integration protection of companion-client receipt version boundaries.

These synthetic layouts test the file-set contract only; actual executable,
metadata, signing and OS preservation are covered by product integration.
"""
from pathlib import Path
import argparse
import copy
import importlib.util
import json
import sys
import time
import unittest

REPO = Path(__file__).resolve().parent.parent
spec = importlib.util.spec_from_file_location('client_receipt_execution', REPO/'tools/runtime-execution.py')
execution = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = execution
spec.loader.exec_module(execution)


class ReceiptProtection(unittest.TestCase):
    def fixture(self, product, version):
        root = OUT/self.id().split('.')[-1]/product
        root.mkdir(parents=True)
        (root/'runtime-config.json').write_text('{"features":{"have_gnutls":"0"}}')
        resources = {'generated-config': {}, 'bundle' if product == 'cocoa' else 'terminal-package': {}}
        if product == 'cocoa':
            program = 'dist/Femacs.app/Contents/MacOS/Femacs'
            dump = 'dist/Femacs.app/Contents/Resources/libexec/Femacs.pdmp'
            client = 'dist/Femacs.app/Contents/MacOS/bin/emacsclient'
        else:
            program, dump = 'dist/terminal/src/emacs', 'dist/terminal/src/emacs.pdmp'
            client = 'dist/terminal/lib-src/emacsclient'
            (root/'config').mkdir()
            info = root/'info';info.mkdir()
            (root/'config/manual-set.json').write_text(json.dumps(dict(info=str(info))))
            resources['info'] = {}
        names = {'port/src/temacs', 'port/src/emacs', 'port/src/emacs.pdmp',
                 'port/src/bootstrap-emacs.pdmp', program, dump}
        if version == 3:
            names.update(('port/lib-src/emacsclient', client))
        value = dict(schema=1, identity_version=version,
                     binding=dict(product=product, variant='baseline'),
                     files={name:None for name in names}, resources=resources)
        return root, value, client

    def test_v3_terminal_complete_layout(self):
        root, value, _ = self.fixture('terminal', 3)
        self.assertEqual(execution.product_receipt_layout(value, root)[0], 'dist/terminal/src/emacs')

    def test_v3_cocoa_complete_layout(self):
        root, value, _ = self.fixture('cocoa', 3)
        self.assertTrue(execution.product_receipt_layout(value, root)[0].endswith('MacOS/Femacs'))

    def test_v3_missing_built_client_rejected(self):
        root, value, _ = self.fixture('terminal', 3)
        del value['files']['port/lib-src/emacsclient']
        with self.assertRaisesRegex(ValueError, 'incomplete or unexpected'):
            execution.product_receipt_layout(value, root)

    def test_v3_missing_packaged_client_rejected(self):
        root, value, client = self.fixture('cocoa', 3)
        del value['files'][client]
        with self.assertRaisesRegex(ValueError, 'incomplete or unexpected'):
            execution.product_receipt_layout(value, root)

    def test_v2_layout_not_upgraded(self):
        root, value, _ = self.fixture('terminal', 2)
        execution.product_receipt_layout(value, root)
        self.assertNotIn('port/lib-src/emacsclient', value['files'])

    def test_v2_extra_client_rejected(self):
        root, value, client = self.fixture('terminal', 2)
        value['files'][client] = None
        with self.assertRaisesRegex(ValueError, 'incomplete or unexpected'):
            execution.product_receipt_layout(value, root)

    def test_unknown_identity_version_rejected(self):
        root, value, _ = self.fixture('terminal', 3)
        value['identity_version'] = 4
        with self.assertRaisesRegex(ValueError, 'unknown product file identity version'):
            execution.product_receipt_layout(value, root)


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
    args = parser.parse_args();OUT = args.out.resolve()
    if not OUT.is_relative_to(REPO/'build') or OUT.exists():
        raise SystemExit('fresh owned component output required')
    OUT.mkdir(parents=True);TimedResult.timings = []
    result = unittest.TextTestRunner(verbosity=2, resultclass=TimedResult).run(
        unittest.defaultTestLoader.loadTestsFromTestCase(ReceiptProtection))
    (OUT/'results.json').write_text(json.dumps(dict(passed=result.wasSuccessful(),
        count=result.testsRun, cases=TimedResult.timings, failures=len(result.failures),
        errors=len(result.errors), scope='synthetic receipt layout boundary'), indent=2)+'\n')
    raise SystemExit(0 if result.wasSuccessful() else 1)
