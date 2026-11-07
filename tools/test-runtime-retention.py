"""Protective component cases for retained link inputs; no editor/compile claims.

These small synthetic file receipts exercise the validation component after
actual source/product integration. Historical-seal authorization itself is
covered by the separate real twenty-product integration, not fabricated here.
"""
from pathlib import Path
import argparse
import importlib.util
import io
import json
import sys
import time
import unittest

REPO = Path(__file__).resolve().parent.parent
spec = importlib.util.spec_from_file_location('retention_under_test', REPO/'tools/runtime-resume.py')
recovery = importlib.util.module_from_spec(spec)
spec.loader.exec_module(recovery)
ex = recovery.execution
OUT = None


class RetentionTests(unittest.TestCase):
    def setUp(self):
        self.root = OUT/self._testMethodName
        self.root.mkdir()
        root = self.root
        for name in ('info/manual', 'config/config.rs',
                     'port/src/temacs', 'port/src/emacs', 'port/src/emacs.pdmp',
                     'port/src/bootstrap-emacs.pdmp', 'dist/terminal/src/emacs',
                     'dist/terminal/src/emacs.pdmp', 'objects/unit.o', 'load/library.a'):
            path = root/name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(('synthetic fixture '+name).encode())
        ex.write_json(root/'runtime-config.json', dict(schema=1, macros={}, features={}))
        ex.write_json(root/'config/manual-set.json', dict(info=str(root/'info')))
        binding = dict(product='terminal', variant='baseline', compiler_leg='normal',
                       config_relative='runtime-config.json', config_digest=ex.digest(root/'runtime-config.json'))
        ex.write_json(root/'runtime-binding.json', binding)
        value = dict(schema=1, binding=binding, files={}, resources={}, build_complete=True,
            link_inputs=dict(link_complete=True, objects={'objects/unit.o':dict(sha256=ex.digest(root/'objects/unit.o'))},
                             archive=dict(path='load/library.a',sha256=ex.digest(root/'load/library.a'))))
        program, dump, resources, _ = ex.product_receipt_layout(dict(value,
            files={name:None for name in ('port/src/temacs','port/src/emacs','port/src/emacs.pdmp',
                'port/src/bootstrap-emacs.pdmp','dist/terminal/src/emacs','dist/terminal/src/emacs.pdmp')},
            resources={name:None for name in ('generated-config','terminal-package','info')}), root)
        value['files']={name:ex.digest(root/name) for name in ('port/src/temacs','port/src/emacs',
            'port/src/emacs.pdmp','port/src/bootstrap-emacs.pdmp',program,dump)}
        value['resources']={name:ex.tree_identity(path) for name,path in resources.items()}
        self.receipt = root/'runtime-product.json'
        ex.write_json(self.receipt, value)
        # Component boundary: supply the already-authorized receipt map. Do not
        # pretend this synthetic fixture passed historical-seal authorization.
        self.sealed = object.__new__(recovery.SealedTerminals)
        self.sealed.receipts = {str(self.receipt):ex.digest(self.receipt)}
        self.sealed.inventory = {}

    def test_present_inputs_verified(self):
        self.sealed.verify(self.receipt)
        row=self.sealed.inventory[str(self.receipt)]
        self.assertEqual(row['present_verified_objects'],['objects/unit.o'])
        self.assertEqual(row['archive']['status'],'present-verified')
        recovery.verify_product(self.receipt)

    def test_missing_inputs_explicit_and_strict_rejects(self):
        (self.root/'objects/unit.o').unlink()
        (self.root/'load/library.a').unlink()
        self.sealed.verify(self.receipt)
        row=self.sealed.inventory[str(self.receipt)]
        self.assertEqual(row['historical_missing_objects'],['objects/unit.o'])
        self.assertEqual(row['archive']['status'],'historical-missing')
        verified = recovery.verify_product_files(self.receipt)
        self.assertEqual(verified['binding']['product'], 'terminal')
        with self.assertRaisesRegex(ValueError,'missing input'):
            recovery.verify_product(self.receipt)

    def test_present_wrong_object_rejected(self):
        (self.root/'objects/unit.o').write_bytes(b'changed')
        with self.assertRaisesRegex(ValueError,'present sealed terminal link input changed'):
            self.sealed.verify(self.receipt)

    def test_present_wrong_archive_rejected(self):
        (self.root/'load/library.a').write_bytes(b'changed')
        with self.assertRaisesRegex(ValueError,'present sealed terminal link archive changed'):
            self.sealed.verify(self.receipt)

    def test_final_artifact_missing_rejected(self):
        (self.root/'port/src/temacs').unlink()
        with self.assertRaisesRegex(ValueError,'missing input'):
            self.sealed.verify(self.receipt)

    def test_resource_changed_rejected(self):
        (self.root/'info/manual').write_bytes(b'changed')
        with self.assertRaisesRegex(ValueError,'completed product resource changed'):
            self.sealed.verify(self.receipt)

    def test_receipt_changed_rejected(self):
        self.receipt.write_bytes(self.receipt.read_bytes()+b'\n')
        with self.assertRaisesRegex(ValueError,'not authorized'):
            self.sealed.verify(self.receipt)

    def test_dangling_object_symlink_rejected(self):
        path=self.root/'objects/unit.o'
        path.unlink()
        path.symlink_to('absent.o')
        with self.assertRaisesRegex(ValueError,'link input is a symlink'):
            self.sealed.verify(self.receipt)


class TimedResult(unittest.TextTestResult):
    def startTest(self, test):
        self.started = time.monotonic()
        super().startTest(test)

    def stopTest(self, test):
        self.timings.append(dict(id=test.id(),seconds=time.monotonic()-self.started))
        super().stopTest(test)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out', required=True, type=Path)
    options = parser.parse_args()
    OUT = ex.new_output(options.out)
    stream=io.StringIO()
    runner=unittest.TextTestRunner(stream=stream,verbosity=2,resultclass=TimedResult)
    # The result is created by the runner; initialize per-run timing storage.
    TimedResult.timings=[]
    result=runner.run(unittest.defaultTestLoader.loadTestsFromTestCase(RetentionTests))
    (OUT/'unittest.txt').write_text(stream.getvalue())
    ex.write_json(OUT/'results.json',dict(passed=result.wasSuccessful(),count=result.testsRun,
        cases=result.timings,failures=[dict(id=t.id(),traceback=s) for t,s in result.failures],
        errors=[dict(id=t.id(),traceback=s) for t,s in result.errors],
        component_only=True,actual_editor_accepted=False))
    print(stream.getvalue(),end='')
    raise SystemExit(0 if result.wasSuccessful() else 1)
