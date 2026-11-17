#!/usr/bin/env python3
"""Protective tests for tools/runtime-comparison.py.

Pure plan/aggregation/expectation logic only: no editor is started, no profile
run is replaced by a synthetic success, and no comparison is promoted.
"""
from __future__ import annotations
import argparse
import importlib.util
import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parent.parent
SPEC = importlib.util.spec_from_file_location('runtime_comparison', ROOT / 'tools/runtime-comparison.py')
comparison = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(comparison)


def plan(**overrides):
    base = dict(schema=comparison.PLAN_SCHEMA, warmup=2, rounds=7, max_load_per_core=.10,
                candidates=[dict(name='a', binary='x', env={'K': 'v'},
                                 expect={'execution': 'interpreted'}),
                            dict(name='b', binary='y')],
                comparisons=[dict(name='ab', left='a', right='b')])
    base.update(overrides)
    return base


def expect_error(callable_object, message):
    try:
        callable_object()
    except ValueError as error:
        assert message in str(error), (message, str(error))
    else:
        raise AssertionError(f'expected ValueError: {message}')


def test_plan_validation():
    valid = comparison.validate_plan(plan())
    assert valid['warmup'] == 2 and valid['rounds'] == 7
    expect_error(lambda: comparison.validate_plan(dict(plan(), schema='other')), 'plan schema')
    expect_error(lambda: comparison.validate_plan(plan(warmup=-1)), 'warmup')
    expect_error(lambda: comparison.validate_plan(plan(rounds=0)), 'rounds')
    expect_error(lambda: comparison.validate_plan(plan(max_load_per_core=-0.1)), 'max_load_per_core')
    expect_error(lambda: comparison.validate_plan(plan(candidates=[])), 'nonempty')
    duplicate = [dict(name='a', binary='x'), dict(name='a', binary='y')]
    expect_error(lambda: comparison.validate_plan(plan(candidates=duplicate)), 'unique')
    expect_error(lambda: comparison.validate_plan(plan(candidates=[dict(name='a')])), 'binary path')
    expect_error(lambda: comparison.validate_plan(plan(candidates=[dict(name='a', binary='x', expect={'execution': 'jit'})])), 'execution expectation')
    expect_error(lambda: comparison.validate_plan(plan(candidates=[dict(name='a', binary='x', expect={'collector': 'moving'})])), 'collector expectation')
    expect_error(lambda: comparison.validate_plan(plan(candidates=[dict(name='a', binary='x', expect={'speed': 'fast'})])), 'unknown expectations')
    expect_error(lambda: comparison.validate_plan(plan(comparisons=[dict(name='ab', left='a', right='a')])), 'itself')
    expect_error(lambda: comparison.validate_plan(plan(comparisons=[dict(name='ab', left='a', right='zz')])), 'unknown candidate')


def test_candidate_command():
    command = comparison.candidate_command(dict(name='a', binary='bin', dump='img.pdmp'),
                                           Path('/tmp/out'), 2, 7, .10)
    assert command[command.index('--warmup') + 1] == '2'
    assert command[command.index('--rounds') + 1] == '7'
    assert command[command.index('--max-load-per-core') + 1] == '0.1'
    assert command[command.index('--dump') + 1] == 'img.pdmp'
    assert '--env' not in command
    env_candidate = dict(name='e', binary='bin', env={'B': '2', 'A': '1'})
    ordered = comparison.candidate_command(env_candidate, Path('/tmp/out'), 2, 7, .10)
    assert [ordered[i + 1] for i, item in enumerate(ordered) if item == '--env'] == ['A=1', 'B=2']
    plain = comparison.candidate_command(dict(name='b', binary='bin'), Path('/tmp/out'), 0, 7, .10)
    assert '--dump' not in plain
    assert plain[plain.index('--warmup') + 1] == '0'


def sample(kind='interpreted', mode=1):
    totals = [mode, 1] + [0] * 22
    if mode == 1:
        totals[4] = 1          # one completed minor cycle
        reasons = [1] + [0] * 12
    else:
        reasons = [0, 0, 0, 1] + [0] * 9
    return dict(case='load', round=1, wall_ms=1.0, gc_count=1, gc_ms=.5, pauses_ms=[.5],
                execution_mode='mixed', actual_function_kind=kind, setup_ms=.0,
                phase_snapshot=[1, [0] * 21, [], [[0] * 5, [0] * 5]],
                collector_status=[1, totals, reasons], allocation_delta=[1] * 7,
                heap_before=[['cons', 1, 2]], heap_after=[['cons', 2, 2]], result='x')


def run_report(samples, quiet=True):
    return dict(schema=comparison.RUN_SCHEMA, passed=True, samples=samples,
                processes=[dict(round=1, load_per_core=.01, peak_sampled_rss_bytes=100)],
                startup_samples=[dict(wall_ms=10.0)], warmup=[dict(round=1)],
                artifacts={'binary': {'sha256': 'abc'}}, source_head='head',
                source_patch_sha256='patch', runtime_image='img', quiet=quiet,
                max_load_per_core=.01, startup=dict(median_ms=10.0, p95_ms=11.0),
                limitations='observations')


def test_expectations():
    candidate = dict(name='n', expect={'execution': 'native'})
    comparison.check_expectations(candidate, run_report([sample('native')]))
    expect_error(lambda: comparison.check_expectations(candidate, run_report([sample('bytecode')])),
                 'expected native code')
    collector = dict(name='c', expect={'collector': 'young'})
    growing = sample(mode=1)
    growing['collector_status'] = [1, [1, 1, 0, 0, 2] + [0] * 19, [2] + [0] * 12]
    comparison.check_expectations(collector, run_report([sample(mode=1), growing]))
    expect_error(lambda: comparison.check_expectations(collector, run_report([sample(mode=0)])),
                 'expected collector mode 1')
    # A young-mode flag without any completed minor cycle is not a working
    # collector and must not be reported as a finished comparison input.
    inactive = sample(mode=1)
    inactive['collector_status'] = [1, [1, 1] + [0] * 22, [0] * 13]
    checked = comparison.check_expectations(collector, run_report([inactive, inactive]))
    activity = [item for item in checked if item['check'] == 'collector-activity'][0]
    assert activity['passed'] is False and 'reachability-only' in activity['note']
    major = dict(name='m', expect={'collector': 'major'})
    comparison.check_expectations(major, run_report([sample(mode=0), sample(mode=0)]))
    minor_in_major = sample(mode=0)
    minor_in_major['collector_status'] = [1, [0, 1, 0, 0, 2] + [0] * 19, [1] + [0] * 12]
    checked = comparison.check_expectations(major, run_report([sample(mode=0), minor_in_major]))
    assert [item for item in checked if item['check'] == 'collector-activity'][0]['passed'] is False
    expect_error(lambda: comparison.check_expectations(collector, run_report([dict(sample(), collector_status=None)])),
                 'reported no status')


def test_cadence():
    assert comparison.cadence(None, None) is None
    first = sample(mode=1)
    last = sample(mode=1)
    last['collector_status'][1][4] += 3          # three more minors
    last['collector_status'][1][3] += 1          # one more major
    last['collector_status'][2][0] += 3
    last['collector_status'][2][4] += 1
    summary = comparison.cadence(first['collector_status'], last['collector_status'])
    assert summary['delta']['young_cycles'] == 3 and summary['delta']['minors'] == 3
    assert summary['delta']['majors'] == 1 and summary['delta']['full_reasons'] == {'4': 1}
    same = comparison.cadence(first['collector_status'], first['collector_status'])
    assert same['delta']['young_cycles'] == 0 and same['delta']['minors'] == 0
    # The first sample of a run has no earlier status: keep the absolute
    # counters and leave the delta unknown instead of reporting zero work.
    opening = comparison.cadence(None, last['collector_status'])
    assert opening['delta'] is None and opening['absolute']['minors'] == last['collector_status'][1][4]
    assert 'no earlier sample' in opening['basis']


def test_phase_filtering():
    clean = [1, 0, 4, 1, 1_000_000, 3_000_000]
    incomplete = [1, 0, 4, 5, 0, 9_000_000]
    clock = [1, 0, 4, 3, 0, 9_000_000]
    backwards = [1, 0, 4, 1, 5_000_000, 1_000_000]
    exported = [1, 0, 4, 9, 0, 2_000_000]
    row = dict(phase_snapshot=[1, [0] * 21, [clean, incomplete, clock, backwards, exported],
                               [[0] * 5, [0] * 5]])
    intervals = comparison.phase_intervals(row)
    assert len(intervals) == 2
    assert intervals[0]['ms'] == 2.0 and intervals[0]['export_overhead'] is False
    assert intervals[1]['export_overhead'] is True
    assert comparison.phase_intervals(dict(phase_snapshot=None)) == []
    assert comparison.phase_intervals(dict(phase_snapshot=[1, [0] * 21, [['short']], None])) == []


def test_aggregation_and_comparison():
    left = comparison.aggregate_candidate('left', dict(name='left', expect={'execution': 'native'}),
                                         run_report([sample('native')], quiet=True))
    assert left['cases'][0]['n'] == 1
    assert left['cases'][0]['allocation_delta'] == [1] * 7
    assert left['peak_rss_bytes'] == 100 and left['warmup_rounds'] == 1
    assert left['checks'][0]['observed'] == ['native']
    assert left['cases'][0]['remembered_set']['valid'] == 1
    assert left['cases'][0]['remembered_set']['length'] == 0
    assert comparison.remembered_set(None) is None
    assert comparison.remembered_set([1, [0] * 23, [0] * 13]) is None
    right = comparison.aggregate_candidate('right', dict(name='right', expect={'execution': 'interpreted'}),
                                           run_report([sample('interpreted')], quiet=False))
    result = comparison.compare('l-r', left, right)
    assert result['promotion_grade'] is False and 'not promotion grade' in result['claim']
    assert result['cases'][0]['median_ratio'] == 1.0
    quiet = comparison.compare('l-r', left, comparison.aggregate_candidate(
        'right', dict(name='right'), run_report([sample('interpreted')], quiet=True)))
    assert quiet['promotion_grade'] is True and 'observations only' in quiet['claim']
    # A quiet pair with a young candidate that never ran a minor stays
    # non-promotion-grade and says why.
    idle_young = sample(mode=1)
    idle_young['collector_status'] = [1, [1, 1] + [0] * 22, [0] * 13]
    idle = comparison.aggregate_candidate('idle', dict(name='idle', expect={'collector': 'young'}),
                                          run_report([idle_young, idle_young], quiet=True))
    blocked = comparison.compare('i-r', idle, comparison.aggregate_candidate(
        'right', dict(name='right'), run_report([sample('interpreted')], quiet=True)))
    assert blocked['promotion_grade'] is False and 'reachability-only' in blocked['claim']
    assert comparison.ratio(1, 0) is None and comparison.ratio(None, 2) is None


def test_ratio_semantics():
    # The compare() ratio is left/right, so a candidate that is faster shows < 1.
    left = comparison.aggregate_candidate('fast', dict(name='fast'), run_report([dict(sample(), wall_ms=1.0)]))
    right = comparison.aggregate_candidate('slow', dict(name='slow'), run_report([dict(sample(), wall_ms=2.0)]))
    assert comparison.compare('f-s', left, right)['cases'][0]['median_ratio'] == .5


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out', type=Path, required=True)
    options = parser.parse_args()
    cases = []
    for name, function in sorted({
            'plan-validation': test_plan_validation,
            'candidate-command': test_candidate_command,
            'expectations': test_expectations,
            'cadence': test_cadence,
            'phase-filtering': test_phase_filtering,
            'aggregation-and-comparison': test_aggregation_and_comparison,
            'ratio-semantics': test_ratio_semantics}.items()):
        function()
        cases.append(dict(id=name, passed=True))
    options.out.mkdir(parents=True, exist_ok=False)
    (options.out / 'results.json').write_text(json.dumps(
        dict(schema='femacs-runtime-comparison-protection-v1', passed=True, cases=cases), indent=2) + '\n')
    print(json.dumps(dict(passed=True, cases=len(cases))))
    return 0


if __name__ == '__main__':
    sys.exit(main())
