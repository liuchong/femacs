"""Contracts for the version-bound corpus runner; execute only in final D.

These cases exercise manifest, isolation and result-accounting helpers. They
do not run an editor and cannot establish language compatibility.
"""

import copy
import hashlib
import importlib.util
import math
import os
from pathlib import Path
import stat
import sys
import tempfile
import time
import unittest
from unittest import mock


REPO = Path(__file__).resolve().parent.parent
SPEC = importlib.util.spec_from_file_location("stable_suite", REPO / "tools/stable-suite.py")
SUITE = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = SUITE
SPEC.loader.exec_module(SUITE)
BATCH_SPEC = importlib.util.spec_from_file_location("stable_suite_batch", REPO / "tools/stable-suite-batch.py")
BATCH = importlib.util.module_from_spec(BATCH_SPEC)
sys.modules[BATCH_SPEC.name] = BATCH
BATCH_SPEC.loader.exec_module(BATCH)


def sha256(data):
    return hashlib.sha256(data).hexdigest()


def git_blob(data):
    return hashlib.sha1(b"blob " + str(len(data)).encode("ascii") + b"\0" + data).hexdigest()


class TimedResult(unittest.TextTestResult):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.case_seconds = []

    def startTest(self, test):
        self.case_started = time.perf_counter()
        super().startTest(test)

    def stopTest(self, test):
        self.case_seconds.append(dict(test=test.id(), seconds=time.perf_counter() - self.case_started))
        super().stopTest(test)


class TimedRunner(unittest.TextTestRunner):
    resultclass = TimedResult


class CorpusContracts(unittest.TestCase):
    def setUp(self):
        # Retain owned cases/failed evidence; never use a system temporary root.
        parent = Path(os.environ.get("FEMACS_RUNTIME_TEST_ROOT", REPO / "build/acceptance/runtime-contracts"))
        parent.mkdir(parents=True, exist_ok=True)
        self.root = Path(tempfile.mkdtemp(prefix=self._testMethodName + "-", dir=parent))
        self.repo = self.root / "repo"
        self.repo.mkdir()
        self.entries = []

    def entry(self, relative, data, *, overlay=False, executable=False):
        source = ("compatibility/stable-31.1/" + relative) if overlay else relative
        path = self.repo / source
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(data)
        path.chmod(0o755 if executable else 0o644)
        item = dict(path=relative, source_path=source, input="overlay" if overlay else "tracked",
                    mode="100755" if executable else "100644", size=len(data),
                    blob=git_blob(data), sha256=sha256(data))
        self.entries.append(item)
        return item

    def manifest(self):
        return dict(schema=1, source=dict(release="31.1",
                    commit="a360712c9d272d950d8d8255ef74570f7e90b7d9"),
                    file_count=len(self.entries),
                    lisp_file_count=sum(e["path"].endswith(".el") for e in self.entries),
                    total_bytes=sum(e["size"] for e in self.entries), entries=copy.deepcopy(self.entries))

    def save_manifest(self, manifest=None):
        import json
        path = self.repo / "compatibility/stable-31.1/sources.json"
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(json.dumps(self.manifest() if manifest is None else manifest), encoding="utf-8")
        return path

    def load(self, manifest=None):
        return SUITE.load_manifest(self.save_manifest(manifest), self.repo)

    def test_exact_tracked_and_overlay_bytes_without_reference(self):
        self.entry("test/lisp/example.el", b";; original\n(ert-deftest example () (should t))\n")
        overlay = self.entry("test/infra/bin/example", b"executable fixture payload\n", overlay=True, executable=True)
        before = {e["source_path"]: (self.repo / e["source_path"]).read_bytes() for e in self.entries}
        result = SUITE.materialize(self.load(), self.repo, self.root / "materialized")
        self.assertEqual(result["file_count"], 2)
        for item in self.entries:
            target = self.root / "materialized" / item["path"]
            self.assertEqual(sha256(target.read_bytes()), item["sha256"])
            self.assertEqual((self.repo / item["source_path"]).read_bytes(), before[item["source_path"]])
        self.assertTrue((self.root / "materialized" / overlay["path"]).stat().st_mode & stat.S_IXUSR)
        self.assertFalse((self.repo / "emacs").exists())

    def test_source_hash_size_and_blob_are_independent_guards(self):
        self.entry("test/lisp/example.el", b"(provide 'example)\n")
        for field, wrong in (("sha256", "0" * 64), ("blob", "0" * 40), ("size", 999)):
            with self.subTest(field=field):
                manifest = self.manifest()
                manifest["entries"][0][field] = wrong
                with self.assertRaises(SUITE.SuiteError):
                    SUITE.materialize(self.load(manifest), self.repo, self.root / ("bad-" + field))

    def test_missing_source_is_not_filled_from_ignored_reference(self):
        item = self.entry("test/lisp/example.el", b"source\n", overlay=True)
        (self.repo / item["source_path"]).unlink()
        reference = self.repo / "emacs" / item["path"]
        reference.parent.mkdir(parents=True)
        reference.write_bytes(b"source\n")
        with self.assertRaises(SUITE.SuiteError):
            SUITE.materialize(self.load(), self.repo, self.root / "output")

    def test_source_symlink_is_rejected_even_when_contents_match(self):
        item = self.entry("test/lisp/example.el", b"source\n")
        original = self.repo / item["source_path"]
        moved = self.root / "outside.el"
        original.rename(moved)
        original.symlink_to(moved)
        with self.assertRaises(SUITE.SuiteError):
            SUITE.materialize(self.load(), self.repo, self.root / "output")

    def test_duplicate_destination_is_rejected(self):
        self.entry("test/lisp/example.el", b"source\n")
        manifest = self.manifest()
        manifest["entries"].append(copy.deepcopy(manifest["entries"][0]))
        manifest["file_count"] = 2
        with self.assertRaises(SUITE.SuiteError):
            self.load(manifest)

    def test_manifest_totals_are_checked_not_trusted(self):
        self.entry("test/lisp/example.el", b"source\n")
        for field in ("file_count", "lisp_file_count", "total_bytes"):
            with self.subTest(field=field):
                manifest = self.manifest()
                manifest[field] += 1
                with self.assertRaises(SUITE.SuiteError):
                    self.load(manifest)

    def test_schema_release_commit_and_modes_are_required(self):
        self.entry("test/lisp/example.el", b"source\n")
        mutations = (
            lambda m: m.update(schema=2),
            lambda m: m["source"].update(release="unknown"),
            lambda m: m["source"].update(commit="not-a-commit"),
            lambda m: m["entries"][0].update(mode="120000"),
        )
        for index, mutate in enumerate(mutations):
            with self.subTest(index=index):
                manifest = self.manifest()
                mutate(manifest)
                with self.assertRaises(SUITE.SuiteError):
                    self.load(manifest)

    def test_unsafe_manifest_paths_are_rejected(self):
        self.entry("test/lisp/example.el", b"source\n")
        absolute = Path.cwd().anchor + "outside.el"
        windows = chr(67) + chr(58) + chr(92) + "outside.el"
        for bad in ("../outside.el", "test/../../outside.el", absolute, windows, "test//example.el"):
            for field in ("path", "source_path"):
                with self.subTest(path=bad, field=field):
                    manifest = self.manifest()
                    manifest["entries"][0][field] = bad
                    with self.assertRaises(SUITE.SuiteError):
                        self.load(manifest)

    def test_output_cannot_overwrite_repo_reference_or_existing_tree(self):
        self.entry("test/lisp/example.el", b"source\n")
        manifest = self.load()
        existing = self.root / "existing"
        existing.mkdir()
        (existing / "keep").write_bytes(b"keep")
        for target in (self.repo, self.repo / "emacs", existing):
            with self.subTest(target=str(target)):
                with self.assertRaises(SUITE.SuiteError):
                    SUITE.materialize(manifest, self.repo, target)
        self.assertEqual((existing / "keep").read_bytes(), b"keep")

    def test_symlinked_output_parent_is_rejected(self):
        self.entry("test/lisp/example.el", b"source\n")
        link = self.root / "alias"
        link.symlink_to(self.repo, target_is_directory=True)
        with self.assertRaises(SUITE.SuiteError):
            SUITE.materialize(self.load(), self.repo, link / "hidden-output")


class AccountingContracts(unittest.TestCase):
    def test_generated_test_ids_not_file_count_define_denominator(self):
        identities = ["generated/one", "generated/two", "literal/three"]
        result = SUITE.compare_registrations(identities, list(reversed(identities)))
        self.assertTrue(result["passed"])
        self.assertEqual(result["reference_count"], 3)

    def test_missing_extra_and_duplicate_candidate_ids_remain_failures(self):
        result = SUITE.compare_registrations(["a", "b", "c"], ["a", "a", "b", "extra"])
        self.assertFalse(result["passed"])
        self.assertEqual(result["missing"], ["c"])
        self.assertEqual(result["extra"], ["extra"])
        self.assertEqual(result["duplicates"], ["a"])
        self.assertEqual(result["reference_count"], 3)

    def test_duplicate_reference_ids_cannot_be_frozen_as_valid(self):
        with self.assertRaises(SUITE.SuiteError):
            SUITE.compare_registrations(["a", "a"], ["a"])

    def observed(self, **changes):
        result = dict(test_id="case/a", status="passed", complete=True,
                      exit_code=0, seconds=0.125, diagnostic="")
        result.update(changes)
        return result

    def test_complete_exact_case_is_the_only_success(self):
        result = SUITE.assess_case("case/a", self.observed())
        self.assertTrue(result["passed"])
        self.assertEqual(result["seconds"], 0.125)

    def test_skip_timeout_resource_missing_and_load_error_are_not_passes(self):
        for status in ("skipped", "timeout", "resource-error", "missing", "load-error", "failed"):
            with self.subTest(status=status):
                result = SUITE.assess_case("case/a", self.observed(status=status, diagnostic="retained original"))
                self.assertFalse(result["passed"])
                self.assertEqual(result["status"], status)
                self.assertEqual(result["diagnostic"], "retained original")

    def test_identity_incomplete_exit_and_invalid_time_cannot_pass(self):
        changes = (dict(test_id="other"), dict(complete=False), dict(complete=1),
                   dict(exit_code=1), dict(exit_code=False), dict(seconds=True),
                   dict(seconds=-1), dict(seconds=math.nan), dict(seconds=math.inf))
        for change in changes:
            with self.subTest(change=change):
                self.assertFalse(SUITE.assess_case("case/a", self.observed(**change))["passed"])

    def test_required_reference_feature_cannot_be_waived_for_candidate(self):
        result = SUITE.classify_applicability(platform="macos", required_platform="macos",
                                             feature="native-comp", reference_features={"native-comp"},
                                             candidate_features=set())
        self.assertTrue(result["applicable"])
        self.assertFalse(result["available"])
        self.assertEqual(result["reason"], "candidate-feature-missing")

    def test_foreign_platform_is_explicit_not_a_local_pass(self):
        result = SUITE.classify_applicability(platform="macos", required_platform="android",
                                             feature=None, reference_features=set(), candidate_features=set())
        self.assertFalse(result["applicable"])
        self.assertEqual(result["reason"], "other-platform")

    def test_each_case_has_isolated_home_cache_and_native_output(self):
        base = REPO / "build/acceptance/runtime-contract-environments"
        base.mkdir(parents=True, exist_ok=True)
        parent = Path(tempfile.mkdtemp(prefix="isolated-", dir=base))
        one = SUITE.case_environment("case-a", parent)
        two = SUITE.case_environment("case-b", parent)
        for key in ("HOME", "XDG_CACHE_HOME", "FEMACS_RUNTIME_NATIVE_CACHE"):
            self.assertNotEqual(one[key], two[key])
            self.assertTrue(Path(one[key]).is_relative_to(parent.resolve()))
            self.assertTrue(Path(two[key]).is_relative_to(parent.resolve()))


class MockShellEnvironmentContracts(unittest.TestCase):
    def setUp(self):
        parent = Path(os.environ.get("FEMACS_RUNTIME_TEST_ROOT", REPO / "build/acceptance/runtime-contracts"))
        parent.mkdir(parents=True, exist_ok=True)
        self.root = Path(tempfile.mkdtemp(prefix="mock shell's ", dir=parent))

    def test_only_named_darwin_mock_files_receive_identical_owned_initialization(self):
        expected = b"case $- in\n  *m*) set +m ;;\nesac\n"
        with mock.patch.object(SUITE.sys, "platform", "darwin"):
            for case in ("test/lisp/autorevert-tests.el|one|1", "test/lisp/filenotify-tests.el"):
                environment = SUITE.case_environment(case, self.root)
                script = Path(environment["FEMACS_STABLE_MOCK_SHELL_ENV"])
                self.assertEqual(environment["ENV"], str(script))
                self.assertEqual(script.read_bytes(), expected)
                self.assertEqual(stat.S_IMODE(script.stat().st_mode), 0o600)
                self.assertTrue(script.is_relative_to(self.root))
            unrelated = SUITE.case_environment("test/src/keyboard-tests.el", self.root)
            self.assertNotIn("ENV", unrelated)
            self.assertNotIn("FEMACS_STABLE_MOCK_SHELL_ENV", unrelated)
        with mock.patch.object(SUITE.sys, "platform", "linux"):
            other = SUITE.case_environment("test/lisp/autorevert-tests.el", self.root / "other-platform")
            self.assertNotIn("ENV", other)
            self.assertNotIn("FEMACS_STABLE_MOCK_SHELL_ENV", other)

    def test_changed_or_symlinked_mock_startup_file_is_rejected(self):
        with mock.patch.object(SUITE.sys, "platform", "darwin"):
            case = "test/lisp/autorevert-tests.el"
            environment = SUITE.case_environment(case, self.root)
            script = Path(environment["ENV"])
            script.write_bytes(b"echo unexpected\n")
            with self.assertRaises(SUITE.SuiteError):
                SUITE.case_environment(case, self.root)
            script.unlink()
            outside = self.root / "other-script"
            outside.write_bytes(b"case $- in\n  *m*) set +m ;;\nesac\n")
            script.symlink_to(outside)
            with self.assertRaises(SUITE.SuiteError):
                SUITE.case_environment(case, self.root)


class StableBatchEnvironmentContracts(unittest.TestCase):
    def setUp(self):
        parent = Path(os.environ.get("FEMACS_RUNTIME_TEST_ROOT", REPO / "build/acceptance/runtime-contracts"))
        parent.mkdir(parents=True, exist_ok=True)
        self.temporary = tempfile.TemporaryDirectory(prefix="p431-", dir=parent)
        self.root = Path(self.temporary.name)
        self.addCleanup(self.temporary.cleanup)

    def test_long_logical_ids_keep_isolated_bounded_environment_names(self):
        logical = "test/" + "long-name-" * 40 + ".el|original-case"
        first = SUITE.case_environment(logical, self.root)
        second = SUITE.case_environment(logical + "-other", self.root)
        one = Path(first["FEMACS_RUNTIME_CASE_ROOT"])
        two = Path(second["FEMACS_RUNTIME_CASE_ROOT"])
        self.assertNotEqual(one, two)
        self.assertLess(len(os.fsencode(one.name)), 32)
        self.assertTrue(one.is_relative_to(self.root))

    def test_replaced_empty_parent_is_preserved(self):
        container = self.root / "container"
        case = container / "case"
        case.mkdir(parents=True)
        nodes = ((case, case.lstat()), (container, container.lstat()))
        original = container / "original"
        case.rename(original)
        case.mkdir()
        with self.assertRaisesRegex(ValueError, "parent changed"):
            BATCH._retire_empty_parents(nodes)
        self.assertTrue(case.is_dir())
        self.assertTrue(original.is_dir())

    def test_unexpected_state_is_never_recursively_removed(self):
        container = self.root / "container"
        case = container / "case"
        case.mkdir(parents=True)
        retained = case / "unexpected"
        retained.write_bytes(b"preserve this state")
        with self.assertRaises(OSError):
            BATCH._retire_empty_parents(((case, case.lstat()), (container, container.lstat())))
        self.assertEqual(retained.read_bytes(), b"preserve this state")
        self.assertTrue(container.is_dir())

    def _owner_receipt_fixture(self, status):
        import json
        final = self.root / 'original.json'
        final.write_text(json.dumps(dict(complete=True, passed=status == 'complete')))
        result = dict(file_result=dict(path=final.name,
                          sha256=SUITE._sha256_path(final), bytes=final.stat().st_size),
                      state_retirement_path='original.owner-retirement.json',
                      state_retirement=dict(status=status))
        return final, result

    def test_completed_retirement_receipt_keeps_original_evidence_hash(self):
        import json
        final, result = self._owner_receipt_fixture('complete')
        result['state_retirement']['empty_parents_retired'] = True
        before = final.read_bytes()
        BATCH._record_retirement(SUITE, final, result)
        receipt = self.root / result['state_retirement_receipt']['path']
        record = json.loads(receipt.read_text())
        self.assertEqual(final.read_bytes(), before)
        self.assertEqual(record['file_result'], result['file_result'])
        self.assertTrue(record['state_retirement']['empty_parents_retired'])
        self.assertEqual(SUITE._sha256_path(receipt), result['state_retirement_receipt']['sha256'])

    def test_retained_failure_reason_is_in_owner_receipt(self):
        import json
        final, result = self._owner_receipt_fixture('retained')
        result['state_retirement']['reason'] = 'original assertion failed'
        BATCH._record_retirement(SUITE, final, result)
        record = json.loads((self.root / result['state_retirement_receipt']['path']).read_text())
        self.assertFalse(json.loads(final.read_text())['passed'])
        self.assertEqual(record['state_retirement'], result['state_retirement'])
        self.assertEqual(record['file_result']['sha256'], SUITE._sha256_path(final))

    def test_changed_evidence_cannot_get_owner_receipt(self):
        final, result = self._owner_receipt_fixture('complete')
        final.write_bytes(b'changed evidence')
        with self.assertRaisesRegex(ValueError, 'file result changed'):
            BATCH._record_retirement(SUITE, final, result)
        self.assertFalse((self.root / result['state_retirement_path']).exists())

    def test_existing_owner_receipt_is_preserved(self):
        final, result = self._owner_receipt_fixture('complete')
        receipt = self.root / result['state_retirement_path']
        receipt.write_bytes(b'original receipt')
        with self.assertRaises(FileExistsError):
            BATCH._record_retirement(SUITE, final, result)
        self.assertEqual(receipt.read_bytes(), b'original receipt')

    def test_setup_failure_records_its_partial_state_before_spawning(self):
        out = self.root / ("deep-" + "x" * 80)
        real_mkdtemp = tempfile.mkdtemp
        created = []
        def allocate(**unused):
            path = Path(real_mkdtemp(prefix="setup-", dir=self.root))
            created.append(path)
            return str(path)
        def fail_setup(logical, container):
            (container / "partial").write_bytes(b"failed preparation")
            raise SUITE.SuiteError("injected preparation failure")
        with mock.patch.object(BATCH.tempfile, "mkdtemp", side_effect=allocate), \
                mock.patch.object(SUITE, "case_environment", side_effect=fail_setup), \
                mock.patch.object(BATCH.subprocess, "Popen") as spawn:
            with self.assertRaisesRegex(SUITE.SuiteError, "injected preparation failure"):
                BATCH._run_file(SUITE, out, {}, out / "runner.el", out / "test",
                                "test/src/process-tests.el", 1, "candidate")
            spawn.assert_not_called()
        receipts = list((out / "results/candidate").glob("*.environment.json"))
        self.assertEqual(len(receipts), 1)
        import json
        record = json.loads(receipts[0].read_text())
        self.assertEqual(record["state"], "failed")
        self.assertTrue(record["retained"])
        self.assertEqual(record["container"], str(created[0]))
        self.assertEqual(record["inode"], created[0].lstat().st_ino)
        self.assertEqual((created[0] / "partial").read_bytes(), b"failed preparation")


if __name__ == "__main__":
    import json
    parent = Path(os.environ.get("FEMACS_RUNTIME_TEST_ROOT", REPO / "build/acceptance/runtime-contracts"))
    parent.mkdir(parents=True, exist_ok=True)
    receipt = Path(tempfile.mkdtemp(prefix="stable-contract-receipt-", dir=parent)) / "results.json"
    program = unittest.main(testRunner=TimedRunner(verbosity=2), exit=False)
    result = program.result
    receipt.write_text(json.dumps(dict(passed=result.wasSuccessful(), tests=result.testsRun,
                                      failures=len(result.failures), errors=len(result.errors),
                                      skipped=len(result.skipped), case_seconds=result.case_seconds),
                                 indent=2), encoding="utf-8")
    raise SystemExit(0 if result.wasSuccessful() else 1)
