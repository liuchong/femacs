#!/usr/bin/env python3
"""Independent regressions for the accepted G4 D review findings."""
from __future__ import annotations

import hashlib
import importlib.util
import json
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest import mock

REPO = Path(__file__).resolve().parent.parent


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load {path}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


workflows = load("g4_review_workflows", REPO / "tools/test-g4-workflows.py")
check_imports = load("g4_review_check_imports", REPO / "tools/check-android-imports.py")


def fixture():
    class ManualFixture(workflows.ManualWorkflows):
        pass

    instance = ManualFixture("runTest")
    instance.setUp()
    return instance


class ManualOrgRegression(unittest.TestCase):
    def setUp(self):
        self.fixture = fixture()
        self.addCleanup(self.fixture.doCleanups)

    def _run(self):
        root, script, program, env, manifest = self.fixture.prepare()
        result, out = self.fixture.invoke(root, script, program, env)
        self.assertEqual(result.returncode, 0, result.stdout)
        return root, out, manifest

    def _events(self):
        path = self.fixture.root / "manual-events.jsonl"
        return [json.loads(line) for line in path.read_text().splitlines()]

    def test_org_export_load_path_is_the_staged_lisp_tree(self):
        _, out, _ = self._run()
        editor = [row for row in self._events() if row["name"] == "owned-editor"]
        self.assertTrue(editor)
        arguments = " ".join(editor[0]["args"])
        self.assertIn(str(out / "src/lisp/org"), arguments)

    def test_all_declared_el_inputs_are_staged_and_hash_recorded(self):
        _, out, manifest = self._run()
        receipt = json.loads((out / "receipt.json").read_text())
        declared = [name for name in manifest["staged_inputs"] if name.endswith(".el")]
        self.assertTrue(declared)
        for name in declared:
            staged = out / "src" / name
            self.assertTrue(staged.is_file(), name)
            self.assertIn(name, receipt["inputs"])
            self.assertEqual(
                receipt["inputs"][name],
                hashlib.sha256(staged.read_bytes()).hexdigest(),
                name,
            )

    def test_manual_cache_registry_retains_builder_and_version_recipe(self):
        _, out, _ = self._run()
        receipt = json.loads((out / "receipt.json").read_text())
        registry = json.loads((out / "info/.femacs-manuals.json").read_text())
        for key in ("builder_sha256", "version_recipe_sha256"):
            self.assertIn(key, registry)
            self.assertEqual(registry[key], receipt[key])


class ManualManifestRegression(unittest.TestCase):
    def test_production_manifest_declares_org_exporter_inputs(self):
        manifest = json.loads((REPO / "build-support/manuals.json").read_text())
        staged = set(manifest["staged_inputs"])
        self.assertIn("lisp/org/org.el", staged)
        self.assertIn("lisp/org/ox-texinfo.el", staged)


class ImportFailureRegression(unittest.TestCase):
    def _run_failure(self, failing):
        with tempfile.TemporaryDirectory(dir=REPO / "build") as temp:
            root = Path(temp)
            so = root / "libemacs.so"
            so.write_bytes(b"owned ELF fixture")
            bin_dir = root / "bin"
            bin_dir.mkdir()
            (bin_dir / "llvm-readelf").write_text("")
            (bin_dir / "llvm-nm").write_text("")

            def run(argv, **kwargs):
                name = Path(str(argv[0])).name
                if failing == "readelf" and name == "llvm-readelf":
                    return subprocess.CompletedProcess(argv, 2, "", "readelf failed")
                if failing == "nm" and name == "llvm-nm":
                    return subprocess.CompletedProcess(argv, 3, "", "nm failed")
                return subprocess.CompletedProcess(argv, 0, "", "")

            with mock.patch.object(check_imports, "ndk_bin", return_value=bin_dir), \
                    mock.patch.object(check_imports.subprocess, "run", side_effect=run):
                with self.assertRaises(RuntimeError):
                    check_imports.measure(so, "21")

    def test_nonzero_readelf_is_not_an_empty_success(self):
        self._run_failure("readelf")

    def test_nonzero_nm_is_not_an_empty_success(self):
        self._run_failure("nm")


if __name__ == "__main__":
    unittest.main(testRunner=unittest.TextTestRunner(verbosity=2,resultclass=workflows.TimedResult))
