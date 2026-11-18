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
        expected = SUITE.MOCK_SHELL_CONTENT
        with mock.patch.object(SUITE.sys, "platform", "darwin"):
            for case in (
                "test/lisp/autorevert-tests.el|one|1",
                "test/lisp/filenotify-tests.el",
                "test/lisp/dnd-tests.el",
                "test/lisp/net/tramp-tests.el",
                "test/lisp/eshell/esh-var-tests.el",
            ):
                environment = SUITE.case_environment(case, self.root)
                script = Path(environment["FEMACS_STABLE_MOCK_SHELL_ENV"])
                self.assertEqual(environment["ENV"], str(script))
                self.assertEqual(
                    environment["FEMACS_STABLE_MOCK_TRANSPORT_PROFILE"],
                    SUITE.MOCK_SHELL_PROFILE,
                )
                self.assertEqual(
                    environment["FEMACS_STABLE_MOCK_TRANSPORT_SHA256"],
                    SUITE.MOCK_SHELL_CONTENT_SHA256,
                )
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
            outside.write_bytes(SUITE.MOCK_SHELL_CONTENT)
            script.symlink_to(outside)
            with self.assertRaises(SUITE.SuiteError):
                SUITE.case_environment(case, self.root)


class DeclaredSubsetContracts(unittest.TestCase):
    """A declared subset narrows scope and can never claim whole-corpus coverage."""

    @classmethod
    def setUpClass(cls):
        cls.suite = SUITE
        cls.manifest = SUITE.load_manifest(
            REPO/'compatibility/stable-31.1/sources.json', Path(REPO).resolve())
        cls.batch = BATCH

    def test_subset_selects_exactly_the_named_entrypoints(self):
        full = self.batch.select_entrypoints(self.manifest)
        named = [full[0]['path'], full[-1]['path']]
        selected = self.batch.select_entrypoints(self.manifest, named)
        self.assertEqual([entry['path'] for entry in selected], sorted(named))
        self.assertLess(len(selected), len(full))

    def test_subset_rejects_empty_unknown_and_duplicate_names(self):
        for bad in ([], ['test/not-a-real-file.el'], ['test/src/data-tests.el'] * 2, [''], [None]):
            with self.assertRaises(ValueError):
                self.batch.select_entrypoints(self.manifest, bad)

    def test_subset_never_reports_corpus_completion(self):
        report = dict(schema=self.suite.RESULT_SCHEMA, selector='all-discovered-ids',
                      scope='stable-31.1-original-ELFILES-declared-subset',
                      subset=['test/src/data-tests.el'], full_corpus_entrypoint_count=555,
                      corpus_complete=False, subset_complete=True)
        self.assertFalse(report['corpus_complete'])
        self.assertTrue(report['subset_complete'])
        self.assertEqual(report['full_corpus_entrypoint_count'], 555)


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

    def test_compact_cases_are_isolated_by_their_unique_containers(self):
        one = SUITE.case_environment('test/lisp/emacs-lisp/package-tests.el',
                                     self.root / 'one', compact=True)
        two = SUITE.case_environment('test/lisp/emacs-lisp/package-tests.el',
                                     self.root / 'two', compact=True)
        self.assertEqual(Path(one['FEMACS_RUNTIME_CASE_ROOT']), self.root / 'one/c')
        for key in ('HOME', 'TMPDIR', 'FEMACS_RUNTIME_NATIVE_CACHE', 'FEMACS_RUNTIME_OUT'):
            self.assertNotEqual(one[key], two[key])
            self.assertTrue(Path(one[key]).is_relative_to(self.root / 'one/c'))
            self.assertTrue(Path(two[key]).is_relative_to(self.root / 'two/c'))
        self.assertNotIn('GNUPGHOME', one)
        self.assertNotIn('GNUPGHOME', two)

    def test_package_path_still_too_long_is_retained_before_editor_spawn(self):
        # A long user cache can defeat the short-container attempt too.
        # Keep the real environment builder and prove the fail-closed boundary.
        out = self.root / 'output'
        container = self.root / ('long-user-cache-' + 'x' * 80)
        container.mkdir()
        with mock.patch.object(BATCH.tempfile, 'mkdtemp', return_value=str(container)), \
                mock.patch.object(BATCH.subprocess, 'Popen') as spawn:
            with self.assertRaisesRegex(SUITE.SuiteError, 'original GPG agent sockets'):
                BATCH._run_file(SUITE, out, {}, out / 'runner.el', out / 'test',
                                'test/lisp/emacs-lisp/package-tests.el', 1, 'candidate')
            spawn.assert_not_called()
        import json
        receipts = list((out / 'results/candidate').glob('*.environment.json'))
        self.assertEqual(len(receipts), 1)
        record = json.loads(receipts[0].read_text())
        self.assertEqual(record['state'], 'failed')
        self.assertTrue(record['retained'])
        self.assertTrue(record['short_container'])
        self.assertEqual(record['inode'], container.lstat().st_ino)
        self.assertTrue((container / 'c/tmp').is_dir())

    def test_replaced_empty_parent_is_preserved(self):
        container = self.root / "container"
        case = container / "case"
        case.mkdir(parents=True)
        nodes = ((case, case.lstat()), (container, container.lstat()))
        original = container / "original"
        case.rename(original)
        case.mkdir()
        final, result = self._owner_receipt_fixture('complete')
        result.update(complete=True, exit_code=0, remaining_group_terminated=False)
        with self.assertRaisesRegex(ValueError, "parent changed"):
            SUITE._retire_state(case, (), final, result, empty_parents=nodes)
        self.assertTrue(case.is_dir())
        self.assertTrue(original.is_dir())

    def test_unexpected_state_is_never_recursively_removed(self):
        container = self.root / "container"
        case = container / "case"
        case.mkdir(parents=True)
        retained = case / "unexpected"
        retained.write_bytes(b"preserve this state")
        final, result = self._owner_receipt_fixture('complete')
        result.update(complete=True, exit_code=0, remaining_group_terminated=False)
        with self.assertRaisesRegex(ValueError, 'unselected'):
            SUITE._retire_state(case, (), final, result,
                empty_parents=((case, case.lstat()), (container, container.lstat())))
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

    def test_fixture_entries_case_deadline_and_compiler_output_contracts(self):
        manifest = {
            "entries": [
                {"path": "test/src/data-tests.el"},
                {"path": "test/src/comp-tests.el"},
                {"path": "test/src/comp-resources/helper.el"},
                {"path": "test/lisp/arc-mode-tests.el"},
                {"path": "test/data/decompress/foo.gz"},
                {"path": "test/lisp/eshell/esh-var-tests.el"},
                {"path": "test/lisp/eshell/em-glob-tests.el"},
                {"path": "test/lisp/eshell/resources/eshell-Resource.txt"},
                {"path": "test/src/decompress-tests.el"},
                {"path": "test/src/regex-emacs-tests.el"},
                {"path": "test/src/regex-resources/Perl_Tests.txt"},
                {"path": "test/src/fileio-tests.el"},
                {"path": "test/src/lread-resources/somelib.el"},
                {"path": "test/lisp/image/exif-tests.el"},
                {"path": "test/lisp/image/image-dired-tests.el"},
                {"path": "test/lisp/image-file-tests.el"},
                {"path": "test/lisp/net/shr-tests.el"},
                {"path": "test/data/image/black.jpg"},
                {"path": "test/lisp/international/mule-tests.el"},
                {"path": "test/lisp/international/mule-util-resources/test.utf-16le"},
                {"path": "test/lisp/progmodes/project-tests.el"},
                {"path": "test/lisp/progmodes/autoconf-resources/configure.ac"},
                {"path": "test/lisp/progmodes/elisp-mode-resources/simple-shorthand-test.el"},
                {"path": "test/lisp/progmodes/ruby-ts-mode-tests.el"},
                {"path": "test/lisp/progmodes/ruby-mode-resources/ruby.rb"},
                {"path": "test/lisp/progmodes/etags-tests.el"},
                {"path": "test/manual/etags/ETAGS.good_1"},
            ]
        }
        src_paths = [e["path"] for e in BATCH._fixture_entries(manifest, "test/src/comp-tests.el")]
        self.assertIn("test/src/data-tests.el", src_paths)
        self.assertIn("test/src/comp-tests.el", src_paths)
        self.assertIn("test/src/comp-resources/helper.el", src_paths)
        self.assertNotIn("test/lisp/arc-mode-tests.el", src_paths)
        self.assertNotIn("test/src/regex-resources/Perl_Tests.txt", src_paths)

        arc_paths = [e["path"] for e in BATCH._fixture_entries(manifest, "test/lisp/arc-mode-tests.el")]
        self.assertIn("test/lisp/arc-mode-tests.el", arc_paths)
        self.assertIn("test/data/decompress/foo.gz", arc_paths)
        self.assertNotIn("test/data/image/black.jpg", arc_paths)

        decompress_paths = [e["path"] for e in BATCH._fixture_entries(manifest, "test/src/decompress-tests.el")]
        self.assertIn("test/src/decompress-tests.el", decompress_paths)
        self.assertIn("test/data/decompress/foo.gz", decompress_paths)

        for image_entry in (
            "test/lisp/image/exif-tests.el",
            "test/lisp/image/image-dired-tests.el",
            "test/lisp/image-file-tests.el",
            "test/lisp/net/shr-tests.el",
        ):
            img_paths = [e["path"] for e in BATCH._fixture_entries(manifest, image_entry)]
            self.assertIn(image_entry, img_paths)
            self.assertIn("test/data/image/black.jpg", img_paths)

        regex_paths = [e["path"] for e in BATCH._fixture_entries(manifest, "test/src/regex-emacs-tests.el")]
        self.assertIn("test/src/regex-resources/Perl_Tests.txt", regex_paths)

        fileio_paths = [e["path"] for e in BATCH._fixture_entries(manifest, "test/src/fileio-tests.el")]
        self.assertIn("test/src/lread-resources/somelib.el", fileio_paths)

        mule_paths = [e["path"] for e in BATCH._fixture_entries(manifest, "test/lisp/international/mule-tests.el")]
        self.assertIn("test/lisp/international/mule-util-resources/test.utf-16le", mule_paths)

        project_paths = [e["path"] for e in BATCH._fixture_entries(manifest, "test/lisp/progmodes/project-tests.el")]
        self.assertIn("test/lisp/progmodes/autoconf-resources/configure.ac", project_paths)
        self.assertIn("test/lisp/progmodes/elisp-mode-resources/simple-shorthand-test.el", project_paths)

        ruby_paths = [e["path"] for e in BATCH._fixture_entries(manifest, "test/lisp/progmodes/ruby-ts-mode-tests.el")]
        self.assertIn("test/lisp/progmodes/ruby-mode-resources/ruby.rb", ruby_paths)

        etags_paths = [e["path"] for e in BATCH._fixture_entries(manifest, "test/lisp/progmodes/etags-tests.el")]
        self.assertIn("test/manual/etags/ETAGS.good_1", etags_paths)

        sub_paths = [e["path"] for e in BATCH._fixture_entries(manifest, "test/lisp/eshell/esh-var-tests.el")]
        self.assertIn("test/lisp/eshell/esh-var-tests.el", sub_paths)
        self.assertIn("test/lisp/eshell/em-glob-tests.el", sub_paths)
        self.assertIn("test/lisp/eshell/resources/eshell-Resource.txt", sub_paths)
        self.assertNotIn("test/lisp/arc-mode-tests.el", sub_paths)

        with mock.patch.object(BATCH.sys, "platform", "darwin"):
            self.assertEqual(
                BATCH._progress_timeout(BATCH.CASE_BUDGET_FILE, BATCH.CASE_BUDGET_ID, 60.0),
                BATCH.CASE_BUDGET_SECONDS,
            )
            self.assertEqual(
                BATCH._progress_timeout(BATCH.CASE_BUDGET_FILE, "other-test", 60.0),
                60.0,
            )
            ctx = BATCH._case_deadline_context(BATCH.CASE_BUDGET_FILE, 60.0)
            self.assertIsNotNone(ctx)
            self.assertEqual(ctx["profile"], BATCH.CASE_BUDGET_PROFILE)
            self.assertEqual(ctx["selected_case_seconds"], BATCH.CASE_BUDGET_SECONDS)
            self.assertIsNone(BATCH._case_deadline_context("test/lisp/arc-mode-tests.el", 60.0))

        case_root = self.root / "comp-case"
        base_eln = str(case_root / "eln-a") + "/"
        target = str(case_root / "native-cache/compiler-output") + "/"
        enabled_ctx = {
            "profile": BATCH.COMPILER_OUTPUT_PROFILE,
            "scope": "test/src/comp-tests.el-only",
            "enabled": True,
            "before": [base_eln],
            "after": [base_eln, target],
            "target": target,
            "actual_paths": [base_eln, target],
        }
        self.assertEqual(BATCH._verify_compiler_output_context(enabled_ctx, case_root), enabled_ctx)
        disabled_ctx = {
            "profile": BATCH.COMPILER_OUTPUT_PROFILE,
            "scope": "test/src/comp-tests.el-only",
            "enabled": False,
            "before": [],
            "after": [],
            "target": None,
            "actual_paths": [],
        }
        self.assertEqual(BATCH._verify_compiler_output_context(disabled_ctx, case_root), disabled_ctx)
        with self.assertRaises(AssertionError):
            bad_ctx = dict(enabled_ctx, actual_paths=[base_eln])
            BATCH._verify_compiler_output_context(bad_ctx, case_root)

    def test_archive_and_product_library_fixtures_accept_symlinked_product_lisp(self):
        out = self.root / "archive-out"
        (out / "test/lisp").mkdir(parents=True)
        for name in ("src", "admin", "nextstep"):
            (out / "source-context-data" / name).mkdir(parents=True)
        (out / "source-context-data/README").write_bytes(b"readme\n")
        real_lisp = self.root / "real-lisp"
        (real_lisp / "emacs-lisp").mkdir(parents=True)
        (real_lisp / "emacs-lisp/comp.el").write_bytes(b";;; comp.el\n")
        (real_lisp / "emacs-lisp/comp.elc").write_bytes(b";;; comp.elc\n")
        symlink_lisp = self.root / "symlink-lisp"
        symlink_lisp.symlink_to(real_lisp)

        shared = out / "source-contexts/terminal"
        shared.mkdir(parents=True)
        (shared / "test").symlink_to(out / "test")
        (shared / "lisp").symlink_to(real_lisp)
        for name in ("src", "admin", "nextstep", "README"):
            (shared / name).symlink_to(out / "source-context-data" / name)
        paths = {"test_context": shared, "lisp_root": symlink_lisp}

        arc_case = self.root / "arc-case"
        (arc_case / "out/test/lisp").mkdir(parents=True)
        (arc_case / "out/test/lisp/arc-mode-tests.el").write_bytes(b";;; arc\n")
        arc_layout = BATCH._prepare_source_context_fixture(
            SUITE, out, paths, arc_case, "test/lisp/arc-mode-tests.el",
            {
                "root": str(arc_case / "out"),
                "files": [
                    {"path": "test/lisp/arc-mode-tests.el"},
                    {"path": "test/data/decompress/foo.gz"},
                ],
            },
        )
        self.assertIsNotNone(arc_layout)
        self.assertEqual(
            BATCH._finish_source_context_fixture(SUITE, arc_layout)["shared_context_and_targets_unchanged"],
            True,
        )

        comp_case = self.root / "comp-lib-case"
        (comp_case / "out").mkdir(parents=True)
        lib_layout = BATCH._prepare_product_library_fixture(
            SUITE, paths, comp_case, "test/src/comp-tests.el",
        )
        self.assertIsNotNone(lib_layout)
        self.assertEqual(
            BATCH._finish_product_library_fixture(SUITE, lib_layout)["product_sources_unchanged"],
            True,
        )


class OwnedTerminalContracts(unittest.TestCase):
    """Python control-plane probes for the owned PTY launcher/reader cleanup.

    These are not editor, Lisp, native, GC, physical-UI or performance claims.
    Every case creates retained evidence under FEMACS_RUNTIME_TEST_ROOT (or the
    repository's build/acceptance/runtime-contracts) and uses the real
    SUITE.case_environment/BATCH._run_file path.  The executable is only a
    lightweight probe; it declares the protocol fields the runner checks.
    """

    _LOGICAL = "test/lisp/p449-owned-terminal-probe.el"
    _TEST_ID = "p449-owned-terminal-probe"
    _PAYLOAD = b"A" * (512 * 1024)

    _PROBE_BODY = r'''import errno
import json
import os
from pathlib import Path
import subprocess
import sys
import time

PROGRESS = os.environ["STABLE_SUITE_PROGRESS"]
FILE_ID = os.environ["STABLE_SUITE_FILE_ID"]
MODE = os.environ.get("P449_PROBE_MODE", "normal")
INHERITED_FD = int(os.environ.get("P449_INHERITED_FD", "200"))
TEST_ID = "p449-owned-terminal-probe"

def tty_name(fd):
    try:
        return os.ttyname(fd)
    except OSError:
        return None

def evidence(loaded, child=None):
    try:
        os.fstat(INHERITED_FD)
        inherited_error = None
    except OSError as exc:
        inherited_error = errno.errorcode.get(exc.errno, str(exc.errno))
    payload = {
        "mode": MODE,
        "pid": os.getpid(),
        "sid": os.getsid(0),
        "pgrp": os.getpgrp(),
        "argv": sys.argv,
        "isatty": [bool(os.isatty(i)) for i in (0, 1, 2)],
        "ttynames": [tty_name(i) for i in (0, 1, 2)],
        "terminal_name": tty_name(0),
        "inherited_fd": INHERITED_FD,
        "inherited_fd_error": inherited_error,
        "env": {
            "STABLE_SUITE_DISPLAY": os.environ.get("STABLE_SUITE_DISPLAY"),
            "OMP_THREAD_LIMIT": os.environ.get("OMP_THREAD_LIMIT"),
            "P449_PROBE_MODE": MODE,
        },
        "loaded": loaded,
    }
    if child is not None:
        payload["child"] = child
    Path(PROGRESS + ".probe.json").write_text(
        json.dumps(payload, sort_keys=True), encoding="utf-8")

def emit(value):
    with open(PROGRESS, "ab", buffering=0) as stream:
        stream.write(json.dumps(value, separators=(",", ":")).encode("utf-8") + b"\n")

terminal = os.environ.get("STABLE_SUITE_DISPLAY") == "terminal"
loaded = {
    "event": "loaded",
    "file": FILE_ID,
    "selector": "all-discovered-ids",
    "ids": [TEST_ID],
    "tests": [{"id": TEST_ID, "tags": []}],
}
if terminal:
    loaded.update({
        "noninteractive": False,
        "window_system": None,
        "terminal_name": tty_name(0),
        "native_comp_async_jobs": 1,
        "native_comp_thread_limit": os.environ.get("OMP_THREAD_LIMIT"),
        "available_processors": 1,
    })
if MODE == "mismatch":
    loaded["terminal_name"] = "not-this-owned-tty"

started = {"event": "test-started", "file": FILE_ID, "test_id": TEST_ID}
ended = {
    "event": "test-ended",
    "file": FILE_ID,
    "test_id": TEST_ID,
    "duration": 0.001,
    "status": "passed",
    "expected": True,
    "skipped": False,
    "result_type": "ert-test-passed",
    "diagnostic": "probe control-plane completion",
}
finished = {
    "event": "finished",
    "file": FILE_ID,
    "status": "complete",
    "requested_count": 1,
    "ended_count": 1,
}

if MODE == "drain":
    view = 0
    data = b"A" * (512 * 1024)
    while view < len(data):
        view += os.write(1, data[view:view + 65536])

if MODE == "timeout":
    # Resolve the identity dependency before any sleeping child is created.
    import psutil
    emit(loaded)
    emit(started)
    child = subprocess.Popen([sys.executable, "-c", "import time; time.sleep(30)"])
    birth = psutil.Process(child.pid).create_time()
    evidence(loaded, {"pid": child.pid, "birth": birth})
    time.sleep(30)
    sys.exit(0)

if MODE == "mismatch":
    evidence(loaded)
    emit(loaded)
    sys.exit(0)

evidence(loaded)
emit(loaded)
emit(started)
emit(ended)
emit(finished)
sys.exit(0)
'''

    def setUp(self):
        parent = Path(os.environ.get(
            "FEMACS_RUNTIME_TEST_ROOT",
            REPO / "build/acceptance/runtime-contracts"))
        parent.mkdir(parents=True, exist_ok=True)
        # Deliberately retained; do not use TemporaryDirectory/addCleanup/rmtree.
        self.root = Path(tempfile.mkdtemp(prefix="owned-terminal-", dir=parent))
        self.probe = self.root / "probe.py"
        self.probe.write_text(
            "#!" + sys.executable + "\n" + self._PROBE_BODY,
            encoding="utf-8")
        self.probe.chmod(0o755)

    def _layout(self, name, mode, *, inherited_fd=200):
        base = self.root / name
        out = base / "out"
        out.mkdir(parents=True)
        test = base / "test"
        test.mkdir()
        context = base / "context"
        context.mkdir()
        lisp = base / "lisp"
        lisp.mkdir()
        runner = base / "runner.el"
        runner.write_bytes(b"")
        paths = dict(
            program=self.probe,
            dump_args=[],
            runtime_env=dict(
                P449_PROBE_MODE=mode,
                P449_INHERITED_FD=str(inherited_fd)),
            test_context=context,
            lisp_root=lisp)
        if mode == "timeout":
            # case_environment deliberately does not inherit PYTHONPATH.
            # Supply only the already loaded identity dependency's package
            # root to this Python probe, never to an editor invocation.
            import psutil
            paths["runtime_env"]["PYTHONPATH"] = str(
                Path(psutil.__file__).resolve().parent.parent)
        return out, paths, test, runner

    def _run(self, name, mode, *, display="terminal", timeout=3,
             inherited_fd=200):
        out, paths, test, runner = self._layout(
            name, mode, inherited_fd=inherited_fd)
        if display == "default":
            result = BATCH._run_file(
                SUITE, out, paths, runner, test, self._LOGICAL,
                timeout, "probe")
        else:
            result = BATCH._run_file(
                SUITE, out, paths, runner, test, self._LOGICAL,
                timeout, "probe", display=display)
        return result, out

    def _evidence(self, out):
        import json
        matches = list((out / "results" / "probe").glob("*.probe.json"))
        if len(matches) != 1:
            self.fail("expected one probe evidence file, got %r" % (matches,))
        return json.loads(matches[0].read_text(encoding="utf-8"))

    def _read_log(self, out, result, key):
        return (out / result[key]).read_bytes()

    def _assert_owner(self, out, result):
        receipt = out / result["state_retirement_receipt"]["path"]
        self.assertTrue(receipt.is_file())
        self.assertEqual(sha256(receipt.read_bytes()),
                         result["state_retirement_receipt"]["sha256"])
        sealed = out / result["file_result"]["path"]
        self.assertEqual(sha256(sealed.read_bytes()), result["file_result"]["sha256"])
        self.assertEqual(
            receipt.stat().st_size,
            result["state_retirement_receipt"]["bytes"])

    def _high_fd(self):
        import errno
        limit = os.sysconf("SC_OPEN_MAX")
        for fd in range(200, min(int(limit), 400)):
            try:
                os.fstat(fd)
            except OSError as error:
                if error.errno != errno.EBADF:
                    raise
                return fd
        self.fail("no free high descriptor for inherited-FD probe")

    def _child_is_alive(self, pid, birth):
        import psutil
        try:
            process = psutil.Process(pid)
            return (process.create_time() == birth
                    and process.status() != psutil.STATUS_ZOMBIE)
        except psutil.NoSuchProcess:
            return False

    def _child_alive(self, pid, birth, timeout=2.0):
        end = time.monotonic() + timeout
        while time.monotonic() < end:
            if not self._child_is_alive(pid, birth):
                return False
            time.sleep(0.02)
        return True

    def test_terminal_has_owned_tty_session_and_no_inherited_fd(self):
        source = os.open(os.devnull, os.O_RDONLY)
        try:
            fd = self._high_fd()
            os.dup2(source, fd, inheritable=True)
            self.assertTrue(os.get_inheritable(fd))
        finally:
            os.close(source)
        try:
            result, out = self._run(
                "owned-tty", "normal", inherited_fd=fd)
            evidence = self._evidence(out)
            self.assertTrue(result["complete"])
            self.assertTrue(result["passed"])
            self.assertEqual(result["exit_code"], 0)
            self.assertEqual(result["pid"], evidence["pid"])
            self.assertEqual(evidence["sid"], evidence["pid"])
            self.assertEqual(evidence["pgrp"], evidence["pid"])
            self.assertEqual(evidence["isatty"], [True, True, True])
            self.assertEqual(
                len(set(evidence["ttynames"])), 1)
            self.assertEqual(
                evidence["ttynames"][0], evidence["terminal_name"])
            self.assertEqual(
                evidence["terminal_name"],
                evidence["loaded"]["terminal_name"])
            self.assertEqual(
                result["execution_context"]["owned_terminal_name"],
                evidence["terminal_name"])
            self.assertEqual(
                result["execution_context"]["terminal_name"],
                evidence["terminal_name"])
            self.assertEqual(
                result["execution_context"]["native_comp_async_jobs"], 1)
            self.assertEqual(
                result["execution_context"]["native_comp_thread_limit"], "1")
            self.assertEqual(
                result["execution_context"]["available_processors"], 1)
            self.assertEqual(evidence["env"]["OMP_THREAD_LIMIT"], "1")
            self.assertEqual(evidence["inherited_fd"], fd)
            self.assertEqual(evidence["inherited_fd_error"], "EBADF")
            self.assertEqual(
                result["state_retirement"]["status"], "complete")
            self._assert_owner(out, result)
        finally:
            try:
                os.close(fd)
            except OSError:
                pass

    def test_default_batch_keeps_original_context(self):
        before = os.environ.get("STABLE_SUITE_DISPLAY")
        out, paths, test, runner = self._layout("default-batch", "batch")
        paths["runtime_env"]["OMP_THREAD_LIMIT"] = "7"
        with mock.patch.dict(
                os.environ, {"STABLE_SUITE_DISPLAY": "terminal"}, clear=False):
            result = BATCH._run_file(
                SUITE, out, paths, runner, test, self._LOGICAL, 3, "probe")
        self.assertEqual(os.environ.get("STABLE_SUITE_DISPLAY"), before)
        evidence = self._evidence(out)
        self.assertTrue(result["complete"])
        self.assertTrue(result["passed"])
        self.assertNotIn("execution_context", result)
        self.assertNotIn("noninteractive", evidence["loaded"])
        self.assertNotIn("terminal_name", evidence["loaded"])
        self.assertNotIn("native_comp_async_jobs", evidence["loaded"])
        self.assertIsNone(evidence["env"]["STABLE_SUITE_DISPLAY"])
        self.assertEqual(evidence["env"]["OMP_THREAD_LIMIT"], "7")
        self.assertEqual(evidence["isatty"], [False, False, False])
        self.assertIn("--batch", result["argv"])
        self.assertIn("-Q", result["argv"])
        self.assertNotIn("-nw", result["argv"])
        self.assertNotIn("--terminal", result["argv"])
        self.assertNotIn("--eval", result["argv"])
        self.assertEqual(result["state_retirement"]["status"], "complete")
        self._assert_owner(out, result)

    def test_terminal_drains_more_than_pty_buffer(self):
        result, out = self._run("drained-pty", "drain")
        evidence = self._evidence(out)
        stdout = self._read_log(out, result, "stdout_log")
        stderr = self._read_log(out, result, "stderr_log")
        self.assertTrue(result["complete"])
        self.assertTrue(result["passed"])
        self.assertEqual(result["exit_code"], 0)
        self.assertEqual(stdout, self._PAYLOAD)
        self.assertEqual(stderr, b"")
        self.assertGreaterEqual(
            result["execution_context"]["terminal_output_bytes"],
            len(self._PAYLOAD))
        self.assertEqual(
            result["execution_context"]["terminal_output_bytes"],
            len(stdout))
        self.assertEqual(evidence["isatty"], [True, True, True])
        self.assertEqual(result["state_retirement"]["status"], "complete")
        self._assert_owner(out, result)

    def test_terminal_timeout_reaps_owned_child(self):
        # Allow the fresh interpreter to load and publish the child identity.
        # The same finite limit is reset at test-started; the body sleeps 30s.
        result, out = self._run("timeout-reap", "timeout", timeout=3)
        evidence = self._evidence(out)
        child = evidence.get("child")
        self.assertIsNotNone(child)
        self.assertTrue(result["timed_out"])
        self.assertFalse(result["complete"])
        self.assertFalse(result["passed"])
        self.assertIsNone(result["protocol_error"])
        self.assertIsNotNone(result["exit_code"])
        self.assertEqual(
            result["state_retirement"]["status"], "retained")
        self._assert_owner(out, result)
        pid = child["pid"]
        birth = child["birth"]
        try:
            self.assertFalse(
                self._child_alive(pid, birth),
                "owned sleeping child survived timeout cleanup")
        finally:
            if self._child_is_alive(pid, birth):
                os.kill(pid, 9)

    def test_terminal_context_mismatch_is_retained(self):
        result, out = self._run("context-mismatch", "mismatch")
        evidence = self._evidence(out)
        self.assertFalse(result["complete"])
        self.assertFalse(result["passed"])
        self.assertIsNotNone(result["protocol_error"])
        self.assertIsNotNone(result["exit_code"])
        self.assertEqual(
            result["state_retirement"]["status"], "retained")
        self.assertFalse(result["cases"].get(self._TEST_ID, {}).get(
            "assessment", {}).get("passed", False))
        self.assertEqual(
            evidence["loaded"]["terminal_name"], "not-this-owned-tty")
        self._assert_owner(out, result)

        out, paths, test, runner = self._layout(
            "unsupported-display", "normal")
        bad = out.parent / "unsupported-out"
        with self.assertRaises(SUITE.SuiteError):
            BATCH._run_file(
                SUITE, bad, paths, runner, test, self._LOGICAL,
                0.1, "probe", display="unsupported")
        self.assertFalse((bad / "logs").exists())
        self.assertFalse((bad / "results").exists())

    def test_cancel_before_launcher_setsid_kills_pid(self):
        import errno
        import pty

        class FakeProcess:
            pid = 999999

            def __init__(self):
                self.returncode = None
                self.kills = 0
                self.waits = 0

            def poll(self):
                return None

            def kill(self):
                self.kills += 1

            def wait(self):
                self.waits += 1
                self.returncode = -9
                return -9

        fake = FakeProcess()
        fds = []
        real_openpty = pty.openpty

        def openpty():
            master, slave = real_openpty()
            fds.extend([master, slave])
            return master, slave

        out, paths, test, runner = self._layout("cancel-before-setsid", "normal")
        with mock.patch.object(pty, "openpty", side_effect=openpty), \
                mock.patch.object(
                    BATCH.subprocess, "Popen", return_value=fake) as popen, \
                mock.patch.object(
                    BATCH.os, "killpg",
                    side_effect=ProcessLookupError) as killpg, \
                mock.patch.object(
                    BATCH.time, "monotonic",
                    side_effect=iter((0.0, 10.0, 10.1, 10.2))):
            result = BATCH._run_file(
                SUITE, out, paths, runner, test, self._LOGICAL,
                0.25, "probe", display="terminal")
        popen.assert_called_once()
        killpg.assert_called_once()
        self.assertEqual(fake.kills, 1)
        self.assertEqual(fake.waits, 1)
        self.assertEqual(result["pid"], fake.pid)
        self.assertTrue(result["timed_out"])
        self.assertFalse(result["complete"])
        self.assertFalse(result["passed"])
        self.assertEqual(
            result["state_retirement"]["status"], "retained")
        self._assert_owner(out, result)
        self.assertEqual(len(fds), 2)
        for descriptor in fds:
            with self.assertRaises(OSError) as raised:
                os.fstat(descriptor)
            self.assertEqual(raised.exception.errno, errno.EBADF)

    def test_permission_retry_requires_reaped_child_and_stays_bounded(self):
        import errno
        import itertools
        import pty

        class Process:
            pid = 999997

            def __init__(self, code):
                self.returncode = code
                self.kills = self.waits = 0

            def poll(self):
                return self.returncode

            def kill(self):
                self.kills += 1

            def wait(self):
                self.waits += 1
                return self.returncode

        real_openpty = pty.openpty
        scenarios = (("exited", 0, [PermissionError, ProcessLookupError], False),
                     ("live", None, [PermissionError], True),
                     ("persistent", 0, [PermissionError, PermissionError], True))
        for name, code, signals, rejected in scenarios:
            with self.subTest(name=name):
                process = Process(code)
                descriptors = []

                def allocate():
                    pair = real_openpty()
                    descriptors.extend(pair)
                    return pair

                out, paths, test, runner = self._layout(name, "normal")
                with mock.patch.object(pty, "openpty", side_effect=allocate), \
                        mock.patch.object(BATCH.subprocess, "Popen", return_value=process), \
                        mock.patch.object(BATCH.os, "killpg", side_effect=signals) as group, \
                        mock.patch.object(BATCH.time, "monotonic", side_effect=itertools.count()):
                    if rejected:
                        with self.assertRaises(PermissionError):
                            BATCH._run_file(SUITE, out, paths, runner, test,
                                            self._LOGICAL, .25, "probe", display="terminal")
                    else:
                        result = BATCH._run_file(SUITE, out, paths, runner, test,
                                                 self._LOGICAL, .25, "probe", display="terminal")
                        self.assertFalse(result["complete"])
                        self.assertFalse(result["passed"])
                        self.assertEqual(process.waits, 1)
                    self.assertEqual(group.call_count, len(signals))
                    self.assertEqual(process.kills, 0)
                self.assertEqual(len(descriptors), 2)
                for descriptor in descriptors:
                    with self.assertRaises(OSError) as error:
                        os.fstat(descriptor)
                    self.assertEqual(error.exception.errno, errno.EBADF)


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
