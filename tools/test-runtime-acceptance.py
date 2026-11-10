"""B tests for the future tools/runtime-acceptance.py helper module.

These tests are source-only until C implements the module.  They load the
future module with importlib from ``tools/runtime-acceptance.py`` and register
it as ``runtime_acceptance_future``.  Absence now is expected; running this
file before C therefore reports a clear setup failure, not a skipped pass.

Fixed pure interface:

    AcceptanceError
    validate_product_identity(expected: dict, observed: dict) -> dict
    assess_process(expected_ids: list[str], result: dict) -> dict
    acceptance_decision(report: dict) -> dict
    summarize_performance(samples: list[dict],
                          max_load_per_core: float = .10) -> dict
    paired_speed_interval(reference_ms: list[float],
                          candidate_ms: list[float],
                          seed: int = 0,
                          resamples: int = 10000) -> dict

B-fixed result schemas used by these tests:

    validate_product_identity -> {passed: bool, mismatches: list[str]}
    assess_process -> {passed: bool, failures: list[str],
                       case_count: int, raw: list[dict]}
    acceptance_decision -> {passed: bool, failures: list[str],
                            inconclusive: list[str]}
    summarize_performance -> {
        samples: list[dict],          # unchanged input rows, same order
        cases: list[dict],            # one row per (case, mode, execution_identity)
        failures: list[str],
        inconclusive: list[str],
    }
    each summarize case row:
        {case, mode, execution_identity, n, median_ms, mad_ms, min_ms,
         max_ms, range_ms, pause_count, max_pause_ms, p95_pause_ms,
         p99_pause_ms, peak_rss_bytes, max_load_per_core, quiet,
         correctness, inconclusive, reasons}
    paired_speed_interval -> {median_delta_ms, low_ms, high_ms,
                              reliable_gain}

``p95_pause_ms`` is present only with at least 20 actual pause samples;
``p99_pause_ms`` only with at least 100.  Otherwise both are None and
``max_pause_ms``/``pause_count`` remain.  Fewer than seven measured rounds or
load/core > max_load_per_core is inconclusive, never a win.  A nonzero process
exit, timeout, incomplete case set, non-passing case, invalid duration or
changed source identity is a failure, not an inconclusive or a reference pass.

Per-case wall timing and a retained aggregate JSON are written below
``FEMACS_RUNTIME_TEST_ROOT`` when supplied, otherwise under
``build/acceptance/test-runtime-acceptance``.  No test writes to source,
reference or an installed product, and no input file is deleted.
"""
from __future__ import annotations

import copy
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import sys
import time
import traceback
import unittest

TOOLS = Path(__file__).resolve().parent
REPO_ROOT = TOOLS.parent
MODULE_PATH = TOOLS / 'runtime-acceptance.py'
MODULE_NAME = 'runtime_acceptance_future'


def load_future_module():
    if not MODULE_PATH.is_file():
        return None
    spec = importlib.util.spec_from_file_location(MODULE_NAME, MODULE_PATH)
    if spec is None or spec.loader is None:
        raise RuntimeError('cannot create an import spec for runtime-acceptance.py')
    module = importlib.util.module_from_spec(spec)
    sys.modules[MODULE_NAME] = module
    spec.loader.exec_module(module)
    return module


MODULE = load_future_module()


def evidence_root() -> Path:
    raw = os.environ.get('FEMACS_RUNTIME_TEST_ROOT')
    base = Path(raw) if raw else REPO_ROOT / 'build' / 'acceptance'
    root = base / 'test-runtime-acceptance'
    root.mkdir(parents=True, exist_ok=True)
    return root


def identity_fixture() -> dict:
    return {
        'source': {'commit': 'a' * 40, 'digest': 'b' * 64},
        'config': {'digest': 'c' * 64},
        'target': {'triple': 'aarch64-apple-darwin', 'deployment': '14.0'},
        'optimization': {'mode': 'O2'},
        'binary': {'sha256': 'd' * 64, 'bytes': 12345},
        'dump': {'sha256': 'e' * 64, 'bytes': 67890},
        'resources': {'sha256': 'f' * 64, 'files': 10},
        'backend': {
            'logical_id': 'gccjit',
            'version': [16, 2, 0],
            'cache_key': '1' * 64,
            'native_abi': '2' * 64,
        },
        'native_abi': '3' * 64,
        'native': {'required': True, 'compiled': True, 'cache_key': '4' * 64},
        'collector': {
            'required': True, 'selected_mode': 'young', 'cache_key': '5' * 64
        },
    }


def process_fixture(ids=('case-a', 'case-b'), status='passed') -> dict:
    tests = [
        {
            'id': case_id,
            'status': status,
            'seconds': 0.01 * (index + 1),
            'diagnostic': {'index': index, 'note': 'raw-{0}'.format(case_id)},
        }
        for index, case_id in enumerate(ids)
    ]
    return {
        'exit_code': 0,
        'timed_out': False,
        'complete': True,
        'tests': tests,
        'stdout': 'stdout-raw',
        'stderr': 'stderr-raw',
        'source_identity_before': identity_fixture(),
        'source_identity_after': identity_fixture(),
    }


def decision_fixture() -> dict:
    return {
        'required_cases': ['case-a', 'case-b'],
        'cases': [
            {'id': 'case-a', 'status': 'passed'},
            {'id': 'case-b', 'status': 'passed'},
        ],
        'identities_valid': True,
        'source_preserved': True,
        'compatibility_complete': True,
        'native_required': False,
        'native_exercised': False,
        'collector_required': False,
        'actual_minors': 0,
        'performance': {
            'passed': True,
            'reliable_gain': True,
            'inconclusive': [],
            'failures': [],
        },
        'unresolved': [],
    }


def performance_sample(case='edit', mode='interpreted',
                       execution_identity='reference:interpreted:abc',
                       round_number=1, wall_ms=10.0, pauses=(1.0,),
                       peak_rss_bytes=1000, load_per_core=0.01,
                       correctness=True) -> dict:
    return {
        'case': case,
        'mode': mode,
        'execution_identity': execution_identity,
        'round': round_number,
        'wall_ms': wall_ms,
        'pauses_ms': list(pauses),
        'peak_rss_bytes': peak_rss_bytes,
        'load_per_core': load_per_core,
        'correctness': correctness,
    }


def seven_samples(walls, pauses_per_round, **kwargs):
    return [
        performance_sample(round_number=index + 1, wall_ms=wall,
                           pauses=pauses_per_round, **kwargs)
        for index, wall in enumerate(walls)
    ]


def summary_rows(report):
    return {
        (row['case'], row['mode'], row['execution_identity']): row
        for row in report['cases']
    }


def as_text(items):
    return ' '.join(str(item) for item in items)


class AcceptanceTestCase(unittest.TestCase):
    def setUp(self):
        super().setUp()
        if MODULE is None:
            self.fail(
                'tools/runtime-acceptance.py is not implemented yet; '
                'B source is present and C has not supplied the module'
            )


class IdentityValidationTests(AcceptanceTestCase):
    def test_identical_synthetic_identity_is_helper_not_execution(self):
        expected = identity_fixture()
        observed = copy.deepcopy(expected)
        result = MODULE.validate_product_identity(expected, observed)
        self.assertTrue(result['passed'])
        self.assertEqual(result['mismatches'], [])
        # A synthetic equal hash proves comparison only, not execution.

    def test_rejects_missing_null_unknown_and_changed_identity(self):
        expected = identity_fixture()
        mutations = {
            'source-digest': lambda item: item['source'].__setitem__(
                'digest', '0' * 64),
            'missing-backend': lambda item: item.pop('backend'),
            'null-native-abi': lambda item: item.__setitem__('native_abi', None),
            'unknown-target': lambda item: item['target'].__setitem__(
                'triple', 'unknown'),
            'native-not-compiled': lambda item: item['native'].__setitem__(
                'compiled', False),
            'wrong-collector-mode': lambda item: item['collector'].__setitem__(
                'selected_mode', 'major'),
            'changed-native-cache': lambda item: item['native'].__setitem__(
                'cache_key', '9' * 64),
            'changed-backend-cache': lambda item: item['backend'].__setitem__(
                'cache_key', '8' * 64),
        }
        for name, mutate in mutations.items():
            with self.subTest(name=name):
                observed = copy.deepcopy(expected)
                mutate(observed)
                result = MODULE.validate_product_identity(expected, observed)
                self.assertFalse(result['passed'])
                self.assertTrue(result['mismatches'])

    def test_rejects_stale_source_and_changed_product_after_tests(self):
        expected = identity_fixture()
        observed = copy.deepcopy(expected)
        self.assertTrue(MODULE.validate_product_identity(expected, observed)['passed'])
        observed['source']['commit'] = '0' * 40
        result = MODULE.validate_product_identity(expected, observed)
        self.assertFalse(result['passed'])
        self.assertTrue(result['mismatches'])

    def test_acceptance_error_is_an_exception(self):
        self.assertTrue(issubclass(MODULE.AcceptanceError, Exception))
        with self.assertRaises(MODULE.AcceptanceError):
            raise MODULE.AcceptanceError('expected failure')


class ProcessAssessmentTests(AcceptanceTestCase):
    def test_clean_process_with_all_expected_cases_passes(self):
        result = process_fixture()
        assessed = MODULE.assess_process(['case-a', 'case-b'], result)
        self.assertTrue(assessed['passed'])
        self.assertEqual(assessed['failures'], [])
        self.assertEqual(assessed['case_count'], 2)
        self.assertEqual(assessed['raw'], result['tests'])
        self.assertEqual(assessed['raw'][0]['diagnostic']['note'],
                         'raw-case-a')

    def test_missing_extra_duplicate_and_empty_cases_fail(self):
        cases = [
            ('missing', ['case-a', 'case-b'], process_fixture(('case-a',))),
            ('extra', ['case-a'], process_fixture(('case-a', 'case-b'))),
            ('duplicate', ['case-a', 'case-b'],
             dict(process_fixture(('case-a',)),
                  tests=[process_fixture(('case-a',))['tests'][0],
                         process_fixture(('case-a',))['tests'][0]])),
            ('empty', [], process_fixture(())),
        ]
        for name, expected_ids, result in cases:
            with self.subTest(name=name):
                assessed = MODULE.assess_process(expected_ids, result)
                self.assertFalse(assessed['passed'])
                self.assertTrue(assessed['failures'])

    def test_nonpassing_statuses_and_known_failure_diagnostics_fail(self):
        for status in ('skip', 'failure', 'error', 'load-error',
                       'resource-error'):
            with self.subTest(status=status):
                result = process_fixture(status=status)
                result['tests'][0]['diagnostic']['known_broken'] = (
                    'reference also failed'
                )
                assessed = MODULE.assess_process(['case-a', 'case-b'], result)
                self.assertFalse(assessed['passed'])
                self.assertTrue(assessed['failures'])
                self.assertEqual(assessed['raw'], result['tests'])

    def test_invalid_durations_and_process_conditions_fail(self):
        for value in (-0.1, float('nan'), float('inf'), float('-inf'),
                      True, '0.1', None):
            with self.subTest(seconds=repr(value)):
                result = process_fixture()
                result['tests'][0]['seconds'] = value
                assessed = MODULE.assess_process(['case-a', 'case-b'], result)
                self.assertFalse(assessed['passed'])
                self.assertTrue(assessed['failures'])
        for name, mutate in {
            'exit': lambda item: item.__setitem__('exit_code', 1),
            'timeout': lambda item: item.__setitem__('timed_out', True),
            'incomplete': lambda item: item.__setitem__('complete', False),
        }.items():
            with self.subTest(name=name):
                result = process_fixture()
                mutate(result)
                self.assertFalse(
                    MODULE.assess_process(['case-a', 'case-b'], result)['passed']
                )

    def test_source_identity_change_fails(self):
        result = process_fixture()
        result['source_identity_after'] = identity_fixture()
        result['source_identity_after']['dump']['sha256'] = '0' * 64
        assessed = MODULE.assess_process(['case-a', 'case-b'], result)
        self.assertFalse(assessed['passed'])
        self.assertTrue(assessed['failures'])

    def test_failures_preserve_original_diagnostics(self):
        result = process_fixture()
        result['tests'][1]['status'] = 'failure'
        result['tests'][1]['diagnostic'] = {
            'reason': 'expected failure output',
            'nested': {'line': 12, 'text': 'boom'},
        }
        assessed = MODULE.assess_process(['case-a', 'case-b'], result)
        self.assertFalse(assessed['passed'])
        self.assertEqual(assessed['raw'][1], result['tests'][1])
        self.assertEqual(assessed['raw'][1]['diagnostic']['nested']['text'],
                         'boom')


class AcceptanceDecisionTests(AcceptanceTestCase):
    def assert_blocked(self, report):
        result = MODULE.acceptance_decision(report)
        self.assertFalse(result['passed'])
        return result

    def test_complete_report_passes(self):
        result = MODULE.acceptance_decision(decision_fixture())
        self.assertTrue(result['passed'])
        self.assertEqual(result['failures'], [])
        self.assertEqual(result['inconclusive'], [])

    def test_missing_required_cases_and_empty_required_fail(self):
        report = decision_fixture()
        report['required_cases'].append('case-c')
        result = self.assert_blocked(report)
        self.assertTrue(result['failures'])

        for name, mutate in {
            'empty-required': lambda item: item.__setitem__(
                'required_cases', []),
            'empty-cases': lambda item: item.__setitem__('cases', []),
            'case-not-passed': lambda item: item['cases'][0].__setitem__(
                'status', 'failed'),
            'duplicate-required': lambda item: item['required_cases'].append(
                'case-a'),
        }.items():
            with self.subTest(name=name):
                report = decision_fixture()
                mutate(report)
                self.assertTrue(self.assert_blocked(report)['failures'])

    def test_identity_source_compatibility_failures_block_completion(self):
        for name in ('identities_valid', 'source_preserved'):
            with self.subTest(name=name):
                report = decision_fixture()
                report[name] = False
                self.assertTrue(self.assert_blocked(report)['failures'])

        report = decision_fixture()
        report['compatibility_complete'] = False
        result = self.assert_blocked(report)
        self.assertTrue(result['failures'] or result['inconclusive'])

    def test_requested_native_and_collector_are_required(self):
        report = decision_fixture()
        report['native_required'] = True
        report['native_exercised'] = False
        self.assertTrue(self.assert_blocked(report)['failures'])

        report = decision_fixture()
        report['native_required'] = True
        report['native_exercised'] = True
        self.assertTrue(MODULE.acceptance_decision(report)['passed'])

        report = decision_fixture()
        report['collector_required'] = True
        report['actual_minors'] = 2
        self.assertTrue(MODULE.acceptance_decision(report)['passed'])

    def test_collector_always_major_fails(self):
        report = decision_fixture()
        report['collector_required'] = True
        report['actual_minors'] = 0
        result = self.assert_blocked(report)
        self.assertTrue(result['failures'])

    def test_performance_inconclusive_is_explicit_not_success(self):
        report = decision_fixture()
        report['performance'] = {
            'passed': False,
            'reliable_gain': False,
            'inconclusive': ['busy-host', 'fewer-than-seven-rounds'],
        }
        result = self.assert_blocked(report)
        self.assertEqual(result['failures'], [])
        self.assertTrue(result['inconclusive'])
        self.assertIn('busy-host', as_text(result['inconclusive']))

    def test_performance_failure_and_unresolved_regression_fail(self):
        report = decision_fixture()
        report['performance'] = {
            'passed': False,
            'reliable_gain': False,
            'inconclusive': [],
            'failures': ['candidate regression'],
        }
        self.assertTrue(self.assert_blocked(report)['failures'])

        report = decision_fixture()
        report['unresolved'] = ['important-regression']
        result = self.assert_blocked(report)
        self.assertTrue(result['failures'])

    def test_partial_g4_delivery_does_not_close_runtime_stage(self):
        report = decision_fixture()
        report['required_cases'].append('runtime-native-required')
        report['native_required'] = True
        report['native_exercised'] = False
        report['compatibility_complete'] = False
        self.assertTrue(self.assert_blocked(report)['failures'])


class PerformanceSummaryTests(AcceptanceTestCase):
    def test_preserves_samples_and_computes_median_mad_range(self):
        samples = seven_samples(
            [10.0, 12.0, 11.0, 13.0, 10.0, 12.0, 14.0],
            (1.0, 2.0), peak_rss_bytes=1000, load_per_core=0.05,
        )
        report = MODULE.summarize_performance(samples)
        self.assertEqual(report['samples'], samples)
        row = summary_rows(report)[('edit', 'interpreted',
                                    'reference:interpreted:abc')]
        self.assertEqual(row['n'], 7)
        self.assertEqual(row['median_ms'], 12.0)
        self.assertEqual(row['mad_ms'], 1.0)
        self.assertEqual(row['min_ms'], 10.0)
        self.assertEqual(row['max_ms'], 14.0)
        self.assertEqual(row['range_ms'], 4.0)
        self.assertEqual(row['pause_count'], 14)
        self.assertEqual(row['median_pause_ms'], 1.5)
        self.assertEqual(row['mad_pause_ms'], .5)
        self.assertEqual(row['max_pause_ms'], 2.0)
        self.assertEqual(row['peak_rss_bytes'], 1000)
        self.assertIsNone(row['peak_footprint_bytes'])
        self.assertIsNone(row['peak_accounted_bytes'])
        self.assertIsNone(row['p95_pause_ms'])
        self.assertIsNone(row['p99_pause_ms'])
        self.assertTrue(row['quiet'])
        self.assertFalse(row['inconclusive'])
        self.assertTrue(row['correctness'])

    def test_pause_tails_require_actual_minimum_counts(self):
        samples = seven_samples([1.0] * 7, [1.0] * 15)
        # Seven rounds times fifteen pauses = 105, so both tails are available.
        report = MODULE.summarize_performance(samples)
        row = summary_rows(report)[('edit', 'interpreted',
                                    'reference:interpreted:abc')]
        self.assertEqual(row['pause_count'], 105)
        self.assertIsNotNone(row['p95_pause_ms'])
        self.assertIsNotNone(row['p99_pause_ms'])

        samples = seven_samples([1.0] * 7, [1.0] * 3)
        samples[0]['pauses_ms'] = samples[0]['pauses_ms'][:2]
        samples[1]['pauses_ms'] = samples[1]['pauses_ms'][:2]
        # 5*3 + 2*2 = 19; still below both thresholds.
        report = MODULE.summarize_performance(samples)
        row = summary_rows(report)[('edit', 'interpreted',
                                    'reference:interpreted:abc')]
        self.assertEqual(row['pause_count'], 19)
        self.assertIsNone(row['p95_pause_ms'])
        self.assertIsNone(row['p99_pause_ms'])
        self.assertEqual(row['max_pause_ms'], 1.0)

        samples = seven_samples(
            [1.0] * 7, (1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0)
        )
        for sample in samples:
            sample['pauses_ms'] = sample['pauses_ms'][:3]
        samples[0]['pauses_ms'] = samples[0]['pauses_ms'][:2]
        # 6*3 + 2 = 20; p95 available, p99 not.
        report = MODULE.summarize_performance(samples)
        row = summary_rows(report)[('edit', 'interpreted',
                                    'reference:interpreted:abc')]
        self.assertEqual(row['pause_count'], 20)
        self.assertIsNotNone(row['p95_pause_ms'])
        self.assertIsNone(row['p99_pause_ms'])

    def test_fewer_than_seven_rounds_is_inconclusive(self):
        samples = seven_samples([10.0] * 6, (1.0,))
        report = MODULE.summarize_performance(samples)
        row = summary_rows(report)[('edit', 'interpreted',
                                    'reference:interpreted:abc')]
        self.assertEqual(row['n'], 6)
        self.assertTrue(row['inconclusive'])
        self.assertTrue(report['inconclusive'])
        self.assertTrue(row['reasons'])

    def test_busy_host_is_inconclusive_even_when_faster(self):
        samples = seven_samples([1.0] * 7, (1.0,),
                                load_per_core=0.1001)
        report = MODULE.summarize_performance(samples)
        row = summary_rows(report)[('edit', 'interpreted',
                                    'reference:interpreted:abc')]
        self.assertFalse(row['quiet'])
        self.assertTrue(row['inconclusive'])
        self.assertTrue(report['inconclusive'])
        self.assertTrue(row['reasons'])
        self.assertEqual(report['failures'], [])

    def test_correctness_failure_is_retained_not_deleted(self):
        samples = seven_samples([10.0] * 7, (1.0,))
        samples[3]['correctness'] = False
        report = MODULE.summarize_performance(samples)
        row = summary_rows(report)[('edit', 'interpreted',
                                    'reference:interpreted:abc')]
        self.assertEqual(row['n'], 7)
        self.assertFalse(row['correctness'])
        self.assertTrue(report['failures'])
        self.assertEqual(len(report['samples']), 7)

    def test_mode_and_identity_mismatch_groups_are_separate(self):
        reference = seven_samples(
            [10.0] * 7, (1.0,),
            execution_identity='reference:interpreted:aaa')
        native = seven_samples(
            [9.0] * 7, (1.0,), mode='native',
            execution_identity='candidate:native:bbb')
        report = MODULE.summarize_performance(reference + native)
        rows = summary_rows(report)
        self.assertIn(('edit', 'interpreted', 'reference:interpreted:aaa'), rows)
        self.assertIn(('edit', 'native', 'candidate:native:bbb'), rows)
        self.assertEqual(rows[('edit', 'interpreted',
                               'reference:interpreted:aaa')]['n'], 7)
        self.assertEqual(rows[('edit', 'native',
                               'candidate:native:bbb')]['n'], 7)
        self.assertEqual(len(rows), 2)

    def test_cold_startup_compile_and_steady_cases_stay_separate(self):
        samples = (
            seven_samples([1.0] * 7, (), case='startup',mode='startup',execution_identity='reference:off')
            + seven_samples([2.0] * 7, (1.0,), case='native-compile')
            + seven_samples([3.0] * 7, (1.0,), case='edit')
        )
        report = MODULE.summarize_performance(samples)
        rows = summary_rows(report)
        self.assertEqual(len(rows), 3)
        self.assertEqual(rows[('startup', 'startup', 'reference:off')]['median_ms'], 1.0)
        self.assertEqual(rows[('native-compile', 'interpreted',
                               'reference:interpreted:abc')]['median_ms'], 2.0)
        self.assertEqual(rows[('edit', 'interpreted',
                               'reference:interpreted:abc')]['median_ms'], 3.0)

    def test_no_outlier_deletion_and_no_fabricated_zero(self):
        samples = seven_samples([1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1000.0],
                                ())
        report = MODULE.summarize_performance(samples)
        row = summary_rows(report)[('edit', 'interpreted',
                                    'reference:interpreted:abc')]
        self.assertEqual(row['n'], 7)
        self.assertEqual(row['max_ms'], 1000.0)
        self.assertEqual(row['pause_count'], 0)
        self.assertIsNone(row['max_pause_ms'])
        self.assertIsNone(row['median_pause_ms'])
        self.assertIsNone(row['mad_pause_ms'])
        self.assertIsNone(row['p95_pause_ms'])
        self.assertIsNone(row['p99_pause_ms'])

    def test_actual_physical_maxima_and_unobserved_channel(self):
        samples=seven_samples([1.0]*7,())
        for index,sample in enumerate(samples):
            sample.update(peak_footprint_bytes=2000+index,peak_accounted_bytes=2000+index)
        row=MODULE.summarize_performance(samples)['cases'][0]
        self.assertEqual(row['peak_footprint_bytes'],2006)
        self.assertEqual(row['peak_accounted_bytes'],2006)
        samples[0]['peak_footprint_bytes']=None
        row=MODULE.summarize_performance(samples)['cases'][0]
        self.assertIsNone(row['peak_footprint_bytes'])
        self.assertEqual(row['peak_accounted_bytes'],2006)

    def test_duplicate_rounds_and_mislabelled_startup_are_rejected(self):
        samples=seven_samples([1.0]*7,())
        samples[-1]['round']=1
        with self.assertRaises(MODULE.AcceptanceError):MODULE.summarize_performance(samples)
        for case,mode,identity in [('startup','interpreted','ref:off'),
                                   ('edit','startup','ref:off'),('startup','startup','ref:all')]:
            with self.subTest(case=case,mode=mode,identity=identity):
                item=performance_sample(case=case,mode=mode,execution_identity=identity)
                with self.assertRaises(MODULE.AcceptanceError):MODULE.summarize_performance([item])

    def test_load_threshold_boundary(self):
        for load, quiet in ((0.10, True), (0.1001, False)):
            with self.subTest(load=load):
                samples = seven_samples([1.0] * 7, (1.0,),
                                        load_per_core=load)
                report = MODULE.summarize_performance(
                    samples, max_load_per_core=0.10)
                row = summary_rows(report)[('edit', 'interpreted',
                                            'reference:interpreted:abc')]
                self.assertEqual(row['quiet'], quiet)
                self.assertAlmostEqual(row['max_load_per_core'], load)

    def test_invalid_performance_samples_raise(self):
        mutations = {
            'negative-wall': lambda item: item.__setitem__('wall_ms', -1.0),
            'nan-wall': lambda item: item.__setitem__('wall_ms', float('nan')),
            'inf-wall': lambda item: item.__setitem__('wall_ms', float('inf')),
            'bool-wall': lambda item: item.__setitem__('wall_ms', True),
            'text-wall': lambda item: item.__setitem__('wall_ms', '1.0'),
            'pauses-not-list': lambda item: item.__setitem__(
                'pauses_ms', 1.0),
            'negative-pause': lambda item: item.__setitem__(
                'pauses_ms', [-1.0]),
            'nan-pause': lambda item: item.__setitem__(
                'pauses_ms', [float('nan')]),
            'negative-rss': lambda item: item.__setitem__(
                'peak_rss_bytes', -1),
            'bool-rss': lambda item: item.__setitem__(
                'peak_rss_bytes', True),
            'bool-footprint': lambda item: item.__setitem__('peak_footprint_bytes',True),
            'negative-footprint': lambda item: item.__setitem__('peak_footprint_bytes',-1),
            'under-accounted': lambda item: item.__setitem__('peak_accounted_bytes',1),
            'negative-load': lambda item: item.__setitem__(
                'load_per_core', -0.01),
            'nan-load': lambda item: item.__setitem__(
                'load_per_core', float('nan')),
        }
        for name, mutate in mutations.items():
            with self.subTest(name=name):
                sample = performance_sample()
                mutate(sample)
                with self.assertRaises(MODULE.AcceptanceError):
                    MODULE.summarize_performance([sample])

    def test_synthetic_hash_fixture_is_owned_and_not_execution_proof(self):
        root = evidence_root() / 'synthetic-identity'
        root.mkdir(exist_ok=True)
        payload = b'synthetic-identity-only\n'
        path = root / 'binary.sha256-fixture'
        path.write_bytes(payload)
        digest = hashlib.sha256(payload).hexdigest()
        expected = identity_fixture()
        observed = copy.deepcopy(expected)
        observed['binary'] = {'sha256': digest, 'bytes': len(payload)}
        expected['binary'] = {'sha256': digest, 'bytes': len(payload)}
        self.assertTrue(MODULE.validate_product_identity(expected,
                                                         observed)['passed'])
        self.assertEqual(path.read_bytes(), payload)


class PairedSpeedIntervalTests(AcceptanceTestCase):
    def test_constant_gain_is_reliable_and_reference_minus_candidate(self):
        reference = [10.0] * 7
        candidate = [8.0] * 7
        result = MODULE.paired_speed_interval(reference, candidate,
                                              seed=0, resamples=256)
        self.assertEqual(result['median_delta_ms'], 2.0)
        self.assertGreater(result['low_ms'], 0.0)
        self.assertGreaterEqual(result['high_ms'], result['low_ms'])
        self.assertTrue(result['reliable_gain'])

    def test_small_consistent_gain_has_no_ten_percent_threshold(self):
        reference = [10.0] * 7
        candidate = [9.9] * 7
        result = MODULE.paired_speed_interval(reference, candidate,
                                              seed=0, resamples=256)
        self.assertAlmostEqual(result['median_delta_ms'], 0.1)
        self.assertGreater(result['low_ms'], 0.0)
        self.assertTrue(result['reliable_gain'])

    def test_uses_median_of_paired_differences(self):
        result = MODULE.paired_speed_interval(
            [1.0, 2.0, 100.0], [2.0, 1.0, 50.0],
            seed=0, resamples=128)
        self.assertEqual(result['median_delta_ms'], 1.0)

    def test_zero_or_negative_gain_is_not_reliable(self):
        for candidate in ([10.0] * 7, [11.0] * 7):
            with self.subTest(candidate=candidate[0]):
                result = MODULE.paired_speed_interval(
                    [10.0] * 7, candidate, seed=0, resamples=128)
                self.assertLessEqual(result['median_delta_ms'], 0.0)
                self.assertFalse(result['reliable_gain'])

    def test_fixed_seed_is_deterministic(self):
        reference = [10.0, 11.0, 9.5, 10.5, 12.0, 9.0, 11.5]
        candidate = [9.8, 10.7, 9.7, 10.1, 11.3, 9.2, 10.9]
        first = MODULE.paired_speed_interval(reference, candidate,
                                             seed=0, resamples=128)
        second = MODULE.paired_speed_interval(reference, candidate,
                                              seed=0, resamples=128)
        self.assertEqual(first, second)

    def test_rejects_empty_mismatched_nonpositive_and_nonfinite_inputs(self):
        invalid = [
            ([], []),
            ([1.0], []),
            ([1.0], [1.0, 2.0]),
            ([0.0], [1.0]),
            ([-1.0], [1.0]),
            ([float('nan')], [1.0]),
            ([1.0], [float('inf')]),
            ([True], [1.0]),
            (['1.0'], [1.0]),
        ]
        for reference, candidate in invalid:
            with self.subTest(reference=reference, candidate=candidate):
                with self.assertRaises(MODULE.AcceptanceError):
                    MODULE.paired_speed_interval(reference, candidate,
                                                 seed=0, resamples=128)
        for resamples in (0, -1):
            with self.subTest(resamples=resamples):
                with self.assertRaises(MODULE.AcceptanceError):
                    MODULE.paired_speed_interval([1.0], [1.0], seed=0,
                                                 resamples=resamples)


class TimedResult(unittest.TextTestResult):
    """Text result that retains per-test wall time and raw diagnostics."""

    def __init__(self, stream, descriptions, verbosity):
        super().__init__(stream, descriptions, verbosity)
        self.case_records = []
        self._started = None
        self._status = 'passed'
        self._details = []
        self._run_started = None
        self.run_seconds = None

    @staticmethod
    def _format_error(err):
        text = ''.join(traceback.format_exception(*err))
        return text.replace(str(REPO_ROOT), '<repo>')

    def startTestRun(self):
        self._run_started = time.perf_counter()
        super().startTestRun()

    def stopTestRun(self):
        self.run_seconds = time.perf_counter() - self._run_started
        super().stopTestRun()

    def startTest(self, test):
        self._started = time.perf_counter()
        self._status = 'passed'
        self._details = []
        super().startTest(test)

    def stopTest(self, test):
        self.case_records.append({
            'id': test.id(),
            'status': self._status,
            'seconds': time.perf_counter() - self._started,
            'details': self._details,
        })
        super().stopTest(test)

    def addSuccess(self, test):
        self._status = 'passed'
        super().addSuccess(test)

    def addFailure(self, test, err):
        self._status = 'failure'
        self._details.append(self._format_error(err))
        super().addFailure(test, err)

    def addError(self, test, err):
        self._status = 'error'
        self._details.append(self._format_error(err))
        super().addError(test, err)

    def addSkip(self, test, reason):
        self._status = 'skipped'
        self._details.append(str(reason))
        super().addSkip(test, reason)

    def addExpectedFailure(self, test, err):
        self._status = 'expected-failure'
        self._details.append(self._format_error(err))
        super().addExpectedFailure(test, err)

    def addUnexpectedSuccess(self, test):
        self._status = 'unexpected-success'
        super().addUnexpectedSuccess(test)

    def addSubTest(self, test, subtest, err):
        if err is not None:
            if issubclass(err[0], test.failureException):
                self._status = 'failure'
            else:
                self._status = 'error'
            self._details.append(self._format_error(err))
        super().addSubTest(test, subtest, err)


def main():
    loader = unittest.defaultTestLoader
    suite = loader.loadTestsFromModule(sys.modules[__name__])
    started = time.perf_counter()
    # TextTestRunner supplies unittest's decorated stream and the run-level
    # callbacks. A raw sys.stderr lacks writeln and aborts on first success.
    result = unittest.TextTestRunner(stream=sys.stderr, descriptions=True,
                                    verbosity=2, resultclass=TimedResult).run(suite)
    total_seconds = time.perf_counter() - started
    root = evidence_root()
    stamp = time.strftime('%Y%m%dT%H%M%SZ', time.gmtime())
    path = root / 'results-{0}-{1}-{2}.json'.format(
        stamp, os.getpid(), time.time_ns())
    report = {
        'schema': 'femacs-runtime-acceptance-tests-v1',
        'test_file': 'tools/test-runtime-acceptance.py',
        'module_file': 'tools/runtime-acceptance.py',
        'module_available': MODULE is not None,
        'total': result.testsRun,
        'passed': result.wasSuccessful(),
        'failure_count': len(result.failures),
        'error_count': len(result.errors),
        'skip_count': len(result.skipped),
        'seconds': total_seconds,
        'cases': result.case_records,
    }
    path.write_text(json.dumps(report, indent=2, sort_keys=True) + '\n')
    print(json.dumps({
        'passed': report['passed'],
        'total': report['total'],
        'failure_count': report['failure_count'],
        'error_count': report['error_count'],
        'results_file': path.name,
    }, sort_keys=True))
    return 0 if report['passed'] else 1


if __name__ == '__main__':
    sys.exit(main())
