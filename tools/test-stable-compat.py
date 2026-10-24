#!/usr/bin/env python3
"""Units for strict bounded stable comparison. No editor or GUI is started."""
import copy
import importlib.util
import json
from pathlib import Path
import plistlib
import sys
import tempfile
import time
import unittest

ROOT = Path(__file__).resolve().parent.parent
spec = importlib.util.spec_from_file_location('stable_compat', ROOT/'tools/stable-compat.py')
compat = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = compat
spec.loader.exec_module(compat)


class Cases(unittest.TestCase):
    def test_bundle_entry_dump_and_wrapper_identity(self):
        base=ROOT/'build/stable-compat-unit';base.mkdir(parents=True,exist_ok=True)
        with tempfile.TemporaryDirectory(dir=base) as directory:
            root=Path(directory);app=root/'Stable.app';macos=app/'Contents/MacOS';macos.mkdir(parents=True)
            with (app/'Contents/Info.plist').open('wb') as out:
                plistlib.dump({'CFBundleExecutable':'Stable'},out)
            binary=macos/'Stable';binary.write_bytes(b'\xcf\xfa\xed\xfe' + b'fixture')
            dump=macos/'libexec/Stable.pdmp';dump.parent.mkdir();dump.write_bytes(b'owned dump')
            self.assertEqual(compat.resolve_product(app),binary)
            self.assertEqual(compat.resolve_product(binary),binary)
            self.assertIn(dump,compat.baseline.discover_dump_paths(binary))
            before=compat.baseline.artifact_fingerprint(binary)
            dump.write_bytes(b'changed dump')
            self.assertNotEqual(before,compat.baseline.artifact_fingerprint(binary))
            wrapper=root/'wrapper';wrapper.write_text('#!/bin/sh\nexec native\n')
            with self.assertRaises(ValueError):compat.resolve_product(wrapper)
            with (app/'Contents/Info.plist').open('wb') as out:
                plistlib.dump({'CFBundleExecutable':'../escape'},out)
            with self.assertRaises(ValueError):compat.resolve_product(app)

    def test_exact_reference_version(self):
        row = {'version':'31.1', 'system':'darwin', 'configuration':'aarch64-apple-darwin'}
        text = 'noise\nFEMACS-STABLE-VERSION '+json.dumps(row)+'\n'
        self.assertEqual(compat.parse_version(text, '31.1'), row)
        with self.assertRaises(ValueError): compat.parse_version(text, '32.0.50')

    def test_bad_version_records_fail(self):
        for text in ['', 'FEMACS-STABLE-VERSION {bad}\n',
                     'FEMACS-STABLE-VERSION {"version":true}\n',
                     'FEMACS-STABLE-VERSION {}\n',
                     'FEMACS-STABLE-VERSION {"version":"31.1"}\n'*2]:
            with self.subTest(text=text), self.assertRaises(ValueError): compat.parse_version(text)

    def test_observation_exact_identities(self):
        row = {'schema':compat.SCHEMA, 'observations':{key:'(1 "中")' for key in compat.CASES}}
        self.assertEqual(compat.validate_observations(row), row['observations'])
        for change in ['missing', 'extra', 'empty', 'not-string', 'schema']:
            bad = copy.deepcopy(row)
            if change == 'missing': bad['observations'].pop(compat.CASES[0])
            if change == 'extra': bad['observations']['other'] = '()'
            if change == 'empty': bad['observations'][compat.CASES[0]] = ''
            if change == 'not-string': bad['observations'][compat.CASES[0]] = None
            if change == 'schema': bad['schema'] = 'other'
            with self.subTest(change=change), self.assertRaises(ValueError): compat.validate_observations(bad)

    def test_strict_ert_outcomes(self):
        run = compat.ert.RunResult(status='ok', ran=6, expected=6, unexpected=0,
                                  results={key:'passed' for key in compat.CASES},
                                  test_seconds={key:0.001 for key in compat.CASES}, exit_code=0)
        self.assertEqual(compat.run_errors(run), [])
        for field, value in [('status','incomplete'), ('ran',0), ('exit_code',1), ('unexpected',1)]:
            bad = copy.deepcopy(run); setattr(bad, field, value)
            with self.subTest(field=field): self.assertTrue(compat.run_errors(bad))
        for value in ['skipped','FAILED','unknown']:
            bad = copy.deepcopy(run); bad.results[compat.CASES[0]] = value
            self.assertTrue(compat.run_errors(bad))
        bad = copy.deepcopy(run); bad.test_seconds[compat.CASES[0]] = float('nan')
        self.assertTrue(compat.run_errors(bad))

    def test_missing_observation_and_changed_product_are_failures(self):
        obs = {key:'value' for key in compat.CASES}
        self.assertEqual(compat.comparison_errors({'reference':obs,'terminal':dict(obs)}), [])
        bad = dict(obs); bad.pop(compat.CASES[0])
        self.assertTrue(compat.comparison_errors({'reference':obs,'terminal':bad}))
        bad = dict(obs); bad[compat.CASES[0]] = 'changed'
        self.assertTrue(compat.comparison_errors({'reference':obs,'terminal':bad}))
        self.assertTrue(compat.artifact_errors({'terminal':{'sha':'a'}}, {'terminal':{'sha':'b'}}))
        self.assertEqual(compat.artifact_errors({'terminal':{'sha':'a'}}, {'terminal':{'sha':'a'}}), [])


class Timing(unittest.TextTestResult):
    def startTest(self, test): self.started=time.monotonic(); super().startTest(test)
    def stopTest(self, test):
        print(json.dumps({'case':test.id(), 'seconds':time.monotonic()-self.started}), flush=True)
        super().stopTest(test)


if __name__ == '__main__':
    unittest.main(testRunner=unittest.TextTestRunner(resultclass=Timing))
