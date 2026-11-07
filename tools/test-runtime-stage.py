"""File-boundary regressions for retaining the runtime matrix build owner.

These cases copy synthetic bytes without executing them. Real compiler and
product acceptance remain independent. Preserve per-case timing under build.
"""
import importlib.util
import json
import os
from pathlib import Path
import sys
import tempfile
import time
import unittest
from unittest import mock

REPO = Path(__file__).resolve().parent.parent
spec = importlib.util.spec_from_file_location("runtime_stage_contracts", REPO / "tools/runtime-stage.py")
stage = importlib.util.module_from_spec(spec)
spec.loader.exec_module(stage)
OUT = Path(os.environ.get("FEMACS_RUNTIME_TEST_ROOT", REPO / "build/acceptance/test-runtime-stage")).resolve()
ROWS = []


class BuildOwnerContracts(unittest.TestCase):
    def setUp(self):
        OUT.mkdir(parents=True, exist_ok=True)
        self.root = Path(tempfile.mkdtemp(prefix=self._testMethodName + "-", dir=OUT))
        self.source = self.root / "cache-tool"
        self.source.write_bytes(b"synthetic executable input\n")
        self.source.chmod(0o755)

    def test_captured_owner_survives_original_cache_removal(self):
        owner, identity = stage.stage_build_owner(self.source, self.root)
        self.source.unlink()
        stage.verify_build_owner(owner, identity)
        self.assertEqual(owner.read_bytes(), b"synthetic executable input\n")
        self.assertTrue(os.access(owner, os.X_OK))
        receipt = json.loads((owner.parent / "receipt.json").read_text())
        self.assertEqual(receipt["sha256"], identity)
        self.assertEqual(receipt["path"], "runtime-build-driver")

    def test_owner_survives_generic_cache_and_legacy_owner_removal(self):
        owner, identity = stage.stage_build_owner(self.source, self.root)
        self.assertFalse(owner.is_relative_to(REPO / "build"))
        legacy = self.root / "build-owner"
        legacy.mkdir()
        old_tool = legacy / "xtask"
        old_tool.write_bytes(b"legacy owner\n")
        old_tool.unlink()
        self.source.unlink()
        stage.verify_build_owner(owner, identity)
        receipt = json.loads((self.root / "build-owner-receipt.json").read_text())
        self.assertEqual(receipt, json.loads((owner.parent / "receipt.json").read_text()))
        self.assertEqual(receipt["sha256"], identity)

    def test_separate_outputs_never_share_the_retained_driver(self):
        other = self.root / "other-output"
        other.mkdir()
        other_source = other / "cache-tool"
        other_source.write_bytes(self.source.read_bytes())
        other_source.chmod(0o755)
        first, identity = stage.stage_build_owner(self.source, self.root)
        second, second_identity = stage.stage_build_owner(other_source, other)
        self.assertNotEqual(first, second)
        self.assertEqual(identity, second_identity)
        first.write_bytes(b"first owner changed\n")
        stage.verify_build_owner(second, second_identity)
        with self.assertRaises(ValueError):
            stage.verify_build_owner(first, identity)

    def test_retained_owner_rejects_output_outside_owned_build(self):
        with mock.patch.object(stage, "REPO", self.root):
            with self.assertRaisesRegex(ValueError, "owned build"):
                stage.stage_build_owner(self.source, self.root)

    def test_retained_owner_rejects_symlinked_cache_parent(self):
        out = self.root / "build/output"
        out.mkdir(parents=True)
        target = self.root / "cache-target"
        target.mkdir()
        (self.root / ".femacs-cache").symlink_to(target.name)
        with mock.patch.object(stage, "REPO", self.root):
            with self.assertRaisesRegex(ValueError, "symlink"):
                stage.stage_build_owner(self.source, out)

    def test_relative_output_locator_preserves_capture_identity(self):
        relative = Path(os.path.relpath(self.root))
        owner, identity = stage.stage_build_owner(self.source, relative)
        self.assertTrue(owner.is_absolute())
        stage.verify_build_owner(owner, identity)
        self.assertEqual(owner.read_bytes(), b"synthetic executable input\n")

    def test_changed_and_missing_captured_owners_are_rejected(self):
        owner, identity = stage.stage_build_owner(self.source, self.root)
        owner.write_bytes(b"changed bytes\n")
        with self.assertRaises(ValueError):
            stage.verify_build_owner(owner, identity)
        owner.unlink()
        with self.assertRaises(ValueError):
            stage.verify_build_owner(owner, identity)

    def test_reused_destination_never_overwrites_captured_owner(self):
        owner, identity = stage.stage_build_owner(self.source, self.root)
        self.source.write_bytes(b"different source\n")
        with self.assertRaises(OSError):
            stage.stage_build_owner(self.source, self.root)
        stage.verify_build_owner(owner, identity)

    def test_symlink_and_nonexecutable_inputs_are_rejected(self):
        self.source.chmod(0o644)
        with self.assertRaises(ValueError):
            stage.stage_build_owner(self.source, self.root)
        self.source.chmod(0o755)
        link = self.root / "tool-link"
        link.symlink_to(self.source.name)
        with self.assertRaises(ValueError):
            stage.stage_build_owner(link, self.root)
        real = self.root / "real-parent"
        real.mkdir()
        parent_link = self.root / "parent-link"
        parent_link.symlink_to(real.name)
        with self.assertRaises(ValueError):
            stage.stage_build_owner(self.source, parent_link)

    def test_source_change_during_capture_is_rejected(self):
        original_digest = stage.execution.digest
        source_reads = 0

        def change_after_first_read(path):
            nonlocal source_reads
            if Path(path).absolute() == self.source.absolute():
                source_reads += 1
                if source_reads == 2:
                    self.source.write_bytes(b"source changed during capture\n")
            return original_digest(path)

        with mock.patch.object(stage.execution, "digest", side_effect=change_after_first_read):
            with self.assertRaisesRegex(ValueError, "input changed during capture"):
                stage.stage_build_owner(self.source, self.root)
        self.assertGreaterEqual(source_reads, 2)
        self.assertEqual(self.source.read_bytes(), b"source changed during capture\n")

    def _synthetic_options(self):
        return mock.MagicMock(
            reference_native=False,
            reference=self.source,
            reference_dump=self.source,
            reference_lisp=self.root,
            reference_data=self.root,
            carry_terminal_from=None,
            carry_native_cocoa_from=None,
        )

    def test_missing_compiled_owner_retains_partial_without_retry(self):
        out = self.root / "matrix"
        out.mkdir()
        source = out / "source"
        source.mkdir()
        export = dict(source=str(source), source_manifest_sha256="abc", passed=True)
        compiled = dict(exit_code=0, complete=True)
        with mock.patch.object(stage.execution, "freeze_source", return_value=export), \
             mock.patch.object(stage.execution, "verify_source_export", return_value=source), \
             mock.patch.object(stage.execution, "product_identity", return_value=dict(id="installed-reference")), \
             mock.patch.object(stage.execution, "run_process", return_value=compiled) as run:
            result = stage.build_matrix(self._synthetic_options(), out)
        self.assertFalse(result["passed"])
        self.assertEqual(run.call_count, 1)
        partial = json.loads((out / "partial.json").read_text())
        self.assertEqual(partial["source_export"], export)
        self.assertEqual(partial["xtask_build"], compiled)
        self.assertEqual(partial["builds"], [])

    def test_capture_failure_retains_partial_without_recompiling(self):
        out = self.root / "matrix"
        out.mkdir()
        source = out / "source"
        source.mkdir()
        cache = out / "cargo/release"
        cache.mkdir(parents=True)
        tool = cache / "xtask"
        tool.write_bytes(b"synthetic cache owner\n")
        tool.chmod(0o755)
        export = dict(source=str(source), source_manifest_sha256="abc", passed=True)
        compiled = dict(exit_code=0, complete=True)
        with mock.patch.object(stage.execution, "freeze_source", return_value=export), \
             mock.patch.object(stage.execution, "verify_source_export", return_value=source), \
             mock.patch.object(stage.execution, "product_identity", return_value=dict(id="installed-reference")), \
             mock.patch.object(stage.execution, "run_process", return_value=compiled) as run, \
             mock.patch.object(stage, "stage_build_owner", side_effect=ValueError("capture changed")):
            with self.assertRaises(ValueError):
                stage.build_matrix(self._synthetic_options(), out)
        self.assertEqual(run.call_count, 1)
        partial = json.loads((out / "partial.json").read_text())
        self.assertEqual(partial["source_export"], export)
        self.assertEqual(partial["xtask_build"], compiled)
        self.assertEqual(partial["builds"], [])

    def test_select_execution_inputs_captures_finished_products_without_link_objects(self):
        original = self.root / "completed-build"
        original.mkdir()
        source = original / "source"
        (source / "lisp").mkdir(parents=True)
        source_file = source / "lisp/loadup.el"
        source_file.write_bytes(b";; synthetic frozen source\n")
        source_file.chmod(0o644)
        manifest_payload = dict(
            schema=1,
            files={
                "lisp/loadup.el": dict(
                    kind="file",
                    sha256=stage.execution.digest(source_file),
                    bytes=source_file.stat().st_size,
                    mode=0o644,
                )
            },
        )
        manifest = original / "source-inputs.json"
        stage.execution.write_json(manifest, manifest_payload)
        stage.execution.write_json(source / ".femacs-runtime-sources.json", manifest_payload)
        export = dict(
            source=str(source),
            source_manifest=str(manifest),
            source_manifest_sha256=stage.execution.digest(manifest),
            file_count=1,
            passed=True,
        )
        products_path = original / "products.json"
        receipt_path = source / "build/runtime/terminal-baseline/runtime-product.json"
        receipt_path.parent.mkdir(parents=True)
        receipt_path.write_bytes(b"{}\n")
        products = [
            dict(name="reference-31-1", kind="reference"),
            dict(name="terminal-baseline", kind="candidate", product_receipt=str(receipt_path)),
        ]
        stage.execution.write_json(products_path, dict(schema=1, products=products))
        stage.execution.write_json(
            original / "results.json",
            dict(
                schema=1,
                action="build",
                passed=True,
                source_export=export,
                source_cohorts=[export],
                product_list=str(products_path),
            ),
        )
        build = stage.execution.bounded_json(original / "results.json")
        run_out = self.root / "execute-stage"
        run_out.mkdir()
        options = mock.MagicMock(out=run_out, build_out=original)
        with stage.owner.ArtifactRun(REPO, run_out) as artifacts:
            with mock.patch.object(stage.owner, "capture_product_files") as capture_files, \
                 mock.patch.object(stage.owner, "capture_product") as capture_strict:
                stage.select_execution_inputs(
                    artifacts, original, source, products_path, products, build, options
                )
        capture_strict.assert_not_called()
        capture_files.assert_called_once_with(artifacts.journal, receipt_path, source)
        journal = stage.execution.bounded_json(run_out / "artifact-preservation.json")
        self.assertEqual(journal["state"], "released")
        captured_paths = {row["path"] for row in journal["files"]}
        self.assertIn(str((original / "results.json").resolve(strict=True)), captured_paths)
        self.assertIn(str(products_path.resolve(strict=True)), captured_paths)

    def test_build_source_lineage_preserves_non_recovery_mismatch_check(self):
        original = self.root / "non-recovery-build"
        original.mkdir()
        source = original / "source"
        source_other = original / "source-other"
        source.mkdir()
        source_other.mkdir()
        manifest = original / "source-inputs.json"
        stage.execution.write_json(manifest, dict(schema=1, files={}))
        export = dict(
            source=str(source),
            source_manifest=str(manifest),
            source_manifest_sha256=stage.execution.digest(manifest),
            passed=True,
        )
        products_path = original / "products.json"
        stage.execution.write_json(
            products_path,
            dict(schema=1, products=[dict(name="reference-31-1", kind="reference")]),
        )
        stage.execution.write_json(
            original / "results.json",
            dict(
                schema=1,
                action="build",
                passed=True,
                source_root=str(source_other),
                source_export=export,
                product_list=str(products_path),
                builds=[],
            ),
        )
        with mock.patch.object(stage.execution, "verify_source_export", return_value=source):
            with self.assertRaisesRegex(ValueError, "selected source does not match the completed build export"):
                stage.read_build(original)


class TimedResult(unittest.TextTestResult):
    def startTest(self, test):
        super().startTest(test)
        self.started = time.perf_counter()
        self.current_status = "incomplete"

    def addSuccess(self, test):
        self.current_status = "passed"
        super().addSuccess(test)

    def addFailure(self, test, error):
        self.current_status = "failed"
        super().addFailure(test, error)

    def addError(self, test, error):
        self.current_status = "error"
        super().addError(test, error)

    def stopTest(self, test):
        ROWS.append(dict(id=test.id(), status=self.current_status,
                         seconds=time.perf_counter() - self.started))
        super().stopTest(test)


if __name__ == "__main__":
    suite = unittest.defaultTestLoader.loadTestsFromTestCase(BuildOwnerContracts)
    result = unittest.TextTestRunner(verbosity=2, resultclass=TimedResult).run(suite)
    OUT.mkdir(parents=True, exist_ok=True)
    passed = result.wasSuccessful() and len(ROWS) == 14 and all(row["status"] == "passed" for row in ROWS)
    (OUT / "results.json").write_text(json.dumps(dict(cases=ROWS, passed=passed), indent=2) + "\n")
    raise SystemExit(0 if passed else 1)
