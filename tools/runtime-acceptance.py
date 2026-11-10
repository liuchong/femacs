"""Pure identity, process-result and performance acceptance helpers.

Importing this module is side-effect free. The helpers validate observed rows;
the CLI delegates to runtime-execution.py for actual isolated product processes.
Neither receipt equality nor a format-control library certifies compatibility.

Fixed result schemas:
    validate_product_identity -> {passed, mismatches}
    assess_process -> {passed, failures, case_count, raw}
    acceptance_decision -> {passed, failures, inconclusive}
    summarize_performance -> {samples, cases, failures, inconclusive}
    paired_speed_interval -> {median_delta_ms, low_ms, high_ms, reliable_gain}

A synthetic hash equality check is an identity comparison helper, not proof
that either product executed.
"""
from __future__ import annotations

import math
import random
import statistics

PERF_MODES = {'interpreted', 'bytecode', 'native', 'startup'}
PERF_ROUND_MIN = 1
PERF_ROUND_PROMOTION = 7
DEFAULT_MAX_LOAD = 0.10


class AcceptanceError(Exception):
    """Malformed input or an internally inconsistent acceptance observation."""


def _add_mismatch(mismatches, path, message):
    mismatches.append('{0}: {1}'.format(path, message))


def _compare_identity(expected, observed, path, mismatches):
    if expected is None or observed is None:
        _add_mismatch(mismatches, path, 'null/unverified value')
        return
    if isinstance(expected, dict):
        if not isinstance(observed, dict):
            _add_mismatch(mismatches, path, 'expected object')
            return
        for key, value in expected.items():
            child = '{0}.{1}'.format(path, key) if path else str(key)
            if key not in observed:
                _add_mismatch(mismatches, child, 'missing')
            else:
                _compare_identity(value, observed[key], child, mismatches)
        for key in observed:
            if key not in expected:
                child = '{0}.{1}'.format(path, key) if path else str(key)
                _add_mismatch(mismatches, child, 'unknown field')
        return
    if isinstance(expected, list):
        if not isinstance(observed, list):
            _add_mismatch(mismatches, path, 'expected list')
            return
        if len(expected) != len(observed):
            _add_mismatch(mismatches, path, 'list length differs')
            return
        for index, value in enumerate(expected):
            _compare_identity(value, observed[index],
                              '{0}[{1}]'.format(path, index), mismatches)
        return
    if type(expected) is not type(observed) or expected != observed:
        _add_mismatch(mismatches, path, 'value differs')


def validate_product_identity(expected, observed):
    """Compare exact product identities; equal synthetic hashes are not execution."""
    mismatches = []
    if not isinstance(expected, dict) or not isinstance(observed, dict):
        _add_mismatch(mismatches, 'identity', 'expected two objects')
    elif not expected:
        _add_mismatch(mismatches, 'identity', 'empty identity is not verified')
    else:
        _compare_identity(expected, observed, '', mismatches)
    return {'passed': not mismatches, 'mismatches': mismatches}


def _seconds(value):
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        return None
    if not math.isfinite(value) or value < 0:
        return None
    return value


def _id_list(values, label):
    if not isinstance(values, list):
        raise AcceptanceError('{0} must be a list'.format(label))
    result = []
    for value in values:
        if not isinstance(value, str) or not value:
            raise AcceptanceError('{0} contains a malformed ID'.format(label))
        result.append(value)
    return result


def assess_process(expected_ids, result):
    """Assess one completed process without dropping failures or raw rows."""
    try:
        expected = _id_list(expected_ids, 'expected_ids')
    except AcceptanceError as exc:
        return {'passed': False, 'failures': [str(exc)],
                'case_count': 0, 'raw': []}
    if not isinstance(result, dict):
        return {'passed': False, 'failures': ['result is not an object'],
                'case_count': 0, 'raw': []}
    tests = result.get('tests')
    if not isinstance(tests, list):
        return {'passed': False, 'failures': ['tests is not a list'],
                'case_count': 0, 'raw': []}
    raw = tests
    failures = []
    if len(set(expected)) != len(expected):
        failures.append('expected IDs contain duplicates')
    if not expected:
        failures.append('expected ID set is empty')
    if not tests:
        failures.append('no observed cases')
    exit_value = result.get('exit_code')
    if type(exit_value) is not int or exit_value != 0:
        failures.append('process exit code is not zero')
    if result.get('timed_out') is not False:
        failures.append('process timed out')
    if result.get('complete') is not True:
        failures.append('process is incomplete')
    before = result.get('source_identity_before')
    after = result.get('source_identity_after')
    if not isinstance(before, dict) or not isinstance(after, dict):
        failures.append('source identity before/after is missing')
    else:
        identity = validate_product_identity(before, after)
        if not identity['passed']:
            failures.append('source identity changed or is unverified')
    observed_ids = []
    for index, test in enumerate(tests):
        if not isinstance(test, dict):
            failures.append('case {0} is not an object'.format(index))
            continue
        case_id = test.get('id')
        if not isinstance(case_id, str) or not case_id:
            failures.append('case {0} has malformed id'.format(index))
        else:
            observed_ids.append(case_id)
        if test.get('status') != 'passed':
            failures.append('case {0} status is not passed'.format(case_id or index))
        if _seconds(test.get('seconds')) is None:
            failures.append('case {0} has invalid duration'.format(case_id or index))
    if len(set(observed_ids)) != len(observed_ids):
        failures.append('observed IDs contain duplicates')
    expected_set = set(expected)
    observed_set = set(observed_ids)
    missing = sorted(expected_set - observed_set)
    extra = sorted(observed_set - expected_set)
    if missing:
        failures.append('missing cases: {0}'.format(', '.join(missing)))
    if extra:
        failures.append('extra cases: {0}'.format(', '.join(extra)))
    return {
        'passed': not failures,
        'failures': failures,
        'case_count': len(tests),
        'raw': raw,
    }


def _string_list(value, label, failures):
    if value is None:
        return []
    if not isinstance(value, list):
        failures.append('{0} is not a list'.format(label))
        return []
    result = []
    for item in value:
        if not isinstance(item, str) or not item:
            failures.append('{0} contains a malformed reason'.format(label))
        else:
            result.append(item)
    return result


def acceptance_decision(report):
    """Return a strict, non-narrowing compatibility-stage decision."""
    failures = []
    inconclusive = []
    if not isinstance(report, dict):
        return {'passed': False, 'failures': ['report is not an object'],
                'inconclusive': []}
    required = report.get('required_cases')
    if not isinstance(required, list) or not required:
        failures.append('required_cases is missing or empty')
        required = []
    elif any(not isinstance(item, str) or not item for item in required):
        failures.append('required_cases contains malformed IDs')
        required = []
    elif len(set(required)) != len(required):
        failures.append('required_cases contains duplicates')
    cases = report.get('cases')
    if not isinstance(cases, list) or not cases:
        failures.append('cases is missing or empty')
        cases = []
    seen = set()
    passed_case_ids = set()
    for index, case in enumerate(cases):
        if not isinstance(case, dict):
            failures.append('case {0} is not an object'.format(index))
            continue
        case_id = case.get('id')
        if not isinstance(case_id, str) or not case_id:
            failures.append('case {0} has malformed id'.format(index))
            continue
        if case_id in seen:
            failures.append('case IDs contain duplicate: {0}'.format(case_id))
            continue
        seen.add(case_id)
        if case.get('status') != 'passed':
            failures.append('case did not pass: {0}'.format(case_id))
        else:
            passed_case_ids.add(case_id)
    for case_id in required:
        if case_id not in seen:
            failures.append('required case missing: {0}'.format(case_id))
        elif case_id not in passed_case_ids:
            failures.append('required case did not pass: {0}'.format(case_id))
    for key, label in (('identities_valid', 'product identities'),
                       ('source_preserved', 'source preservation'),
                       ('compatibility_complete', 'compatibility completeness')):
        if report.get(key) is not True:
            failures.append('{0} is not satisfied'.format(label))
    native_required = report.get('native_required')
    if native_required is True:
        if report.get('native_exercised') is not True:
            failures.append('requested native path was not executed')
    elif type(native_required) is not bool:
        failures.append('native_required is not boolean')
    collector_required = report.get('collector_required')
    if collector_required is True:
        minors = report.get('actual_minors')
        if type(minors) is not int or minors <= 0:
            failures.append('collector had zero actual minors')
    elif type(collector_required) is not bool:
        failures.append('collector_required is not boolean')
    performance = report.get('performance')
    if not isinstance(performance, dict):
        failures.append('performance evidence is missing')
    else:
        perf_failures = _string_list(performance.get('failures'),
                                     'performance failures', failures)
        perf_inconclusive = _string_list(performance.get('inconclusive'),
                                         'performance inconclusive', failures)
        failures.extend(perf_failures)
        inconclusive.extend(perf_inconclusive)
        if not perf_failures and not perf_inconclusive:
            if performance.get('passed') is not True:
                failures.append('performance did not pass')
            if performance.get('reliable_gain') is not True:
                failures.append('performance reliable gain not established')
    unresolved = report.get('unresolved')
    if unresolved is None:
        unresolved = []
    if not isinstance(unresolved, list):
        failures.append('unresolved is not a list')
    else:
        for item in unresolved:
            if item:
                failures.append('unresolved: {0}'.format(item))
    return {
        'passed': not failures and not inconclusive,
        'failures': failures,
        'inconclusive': inconclusive,
    }


def _number(value, label, *, positive=False):
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        raise AcceptanceError('{0} must be numeric'.format(label))
    if not math.isfinite(value):
        raise AcceptanceError('{0} must be finite'.format(label))
    if positive and value <= 0:
        raise AcceptanceError('{0} must be positive'.format(label))
    if not positive and value < 0:
        raise AcceptanceError('{0} must be nonnegative'.format(label))
    return float(value)


def _percentile(values, fraction):
    ordered = sorted(values)
    position = (len(ordered) - 1) * fraction
    lower = int(math.floor(position))
    upper = int(math.ceil(position))
    if lower == upper:
        return ordered[lower]
    return ordered[lower] + (ordered[upper] - ordered[lower]) * (position - lower)


def summarize_performance(samples, max_load_per_core=DEFAULT_MAX_LOAD):
    """Summarize all measured rows; enforce tail-evidence and quiet thresholds."""
    if not isinstance(samples, (list, tuple)):
        raise AcceptanceError('samples must be a list')
    threshold = _number(max_load_per_core, 'max_load_per_core')
    raw = list(samples)
    groups = {}
    rounds_seen = {}
    order = []
    failures = []
    inconclusive = []
    for index, sample in enumerate(samples):
        if not isinstance(sample, dict):
            raise AcceptanceError('sample {0} is not an object'.format(index))
        for key in ('case', 'mode', 'execution_identity'):
            value = sample.get(key)
            if not isinstance(value, str) or not value:
                raise AcceptanceError('sample {0} has malformed {1}'.format(index, key))
        if sample.get('mode') not in PERF_MODES:
            raise AcceptanceError('sample {0} has unknown execution mode'.format(index))
        if ((sample['mode'] == 'startup') != (sample['case'] == 'startup')
                or (sample['mode'] == 'startup' and not sample['execution_identity'].endswith(':off'))):
            raise AcceptanceError('startup requires its distinct case/mode and observer off')
        round_number = sample.get('round')
        if (type(round_number) is not int or
                round_number < PERF_ROUND_MIN):
            raise AcceptanceError('sample {0} has invalid measured round'.format(index))
        wall = _number(sample.get('wall_ms'), 'wall_ms')
        pauses = sample.get('pauses_ms')
        if not isinstance(pauses, (list, tuple)):
            raise AcceptanceError('pauses_ms must be a list')
        pauses = [_number(value, 'pause_ms') for value in pauses]
        rss = sample.get('peak_rss_bytes')
        if isinstance(rss, bool) or type(rss) is not int or rss < 0:
            raise AcceptanceError('peak_rss_bytes must be nonnegative integer')
        memory = {}
        for name in ('peak_footprint_bytes', 'peak_accounted_bytes'):
            value = sample.get(name)
            if value is not None and (type(value) is not int or value < 0):
                raise AcceptanceError(name+' must be a nonnegative integer or unobserved')
            memory[name] = value
        if (memory['peak_accounted_bytes'] is not None
                and memory['peak_accounted_bytes'] < max(rss, memory['peak_footprint_bytes'] or 0)):
            raise AcceptanceError('accounted memory is less than an observed component')
        load = _number(sample.get('load_per_core'), 'load_per_core')
        correctness = sample.get('correctness')
        if type(correctness) is not bool:
            raise AcceptanceError('correctness must be boolean')
        key = (sample['case'], sample['mode'], sample['execution_identity'])
        if key not in groups:
            groups[key] = []
            rounds_seen[key] = set()
            order.append(key)
        if round_number in rounds_seen[key]:
            raise AcceptanceError('duplicate measured round within a case')
        rounds_seen[key].add(round_number)
        groups[key].append({
            'round': round_number,
            'wall': wall,
            'pauses': pauses,
            'rss': rss,
            **memory,
            'load': load,
            'correctness': correctness,
        })
    rows = []
    for key in order:
        entries = groups[key]
        walls = [entry['wall'] for entry in entries]
        pauses = [value for entry in entries for value in entry['pauses']]
        median = statistics.median(walls)
        mad = statistics.median([abs(value - median) for value in walls])
        pause_count = len(pauses)
        max_pause = max(pauses) if pauses else None
        median_pause = statistics.median(pauses) if pauses else None
        mad_pause = statistics.median([abs(value-median_pause) for value in pauses]) if pauses else None
        p95 = _percentile(pauses, 0.95) if pause_count >= 20 else None
        p99 = _percentile(pauses, 0.99) if pause_count >= 100 else None
        load = max(entry['load'] for entry in entries)
        quiet = load <= threshold
        correctness = all(entry['correctness'] for entry in entries)
        reasons = []
        if len(entries) < PERF_ROUND_PROMOTION:
            reasons.append('fewer-than-seven-rounds')
        if not quiet:
            reasons.append('busy-host')
        row = {
            'case': key[0],
            'mode': key[1],
            'execution_identity': key[2],
            'n': len(entries),
            'median_ms': median,
            'mad_ms': mad,
            'min_ms': min(walls),
            'max_ms': max(walls),
            'range_ms': max(walls) - min(walls),
            'pause_count': pause_count,
            'median_pause_ms': median_pause,
            'mad_pause_ms': mad_pause,
            'max_pause_ms': max_pause,
            'p95_pause_ms': p95,
            'p99_pause_ms': p99,
            'peak_rss_bytes': max(entry['rss'] for entry in entries),
            **{name:(max(entry[name] for entry in entries)
                     if all(entry[name] is not None for entry in entries) else None)
               for name in ('peak_footprint_bytes','peak_accounted_bytes')},
            'max_load_per_core': load,
            'quiet': quiet,
            'correctness': correctness,
            'inconclusive': bool(reasons),
            'reasons': reasons,
        }
        rows.append(row)
        for reason in reasons:
            inconclusive.append('{0}/{1}: {2}'.format(key[0], key[1], reason))
        if not correctness:
            failures.append('{0}/{1}: correctness failure'.format(key[0], key[1]))
    return {
        'samples': raw,
        'cases': rows,
        'failures': failures,
        'inconclusive': inconclusive,
    }


def _paired_values(values, label):
    if not isinstance(values, (list, tuple)) or not values:
        raise AcceptanceError('{0} must be a non-empty list'.format(label))
    result = []
    for value in values:
        if isinstance(value, bool) or not isinstance(value, (int, float)):
            raise AcceptanceError('{0} contains nonnumeric input'.format(label))
        if not math.isfinite(value) or value <= 0:
            raise AcceptanceError('{0} contains nonpositive/nonfinite input'.format(label))
        result.append(float(value))
    return result


def paired_speed_interval(reference_ms, candidate_ms, seed=0, resamples=10000):
    """Paired median-difference bootstrap; reliable only with a positive lower bound."""
    reference = _paired_values(reference_ms, 'reference_ms')
    candidate = _paired_values(candidate_ms, 'candidate_ms')
    if len(reference) != len(candidate):
        raise AcceptanceError('paired inputs have different lengths')
    if type(seed) is not int:
        raise AcceptanceError('seed must be an integer')
    if type(resamples) is not int or resamples <= 0:
        raise AcceptanceError('resamples must be a positive integer')
    deltas = [left - right for left, right in zip(reference, candidate)]
    median_delta = statistics.median(deltas)
    generator = random.Random(seed)
    medians = []
    for _ in range(resamples):
        indices = [generator.randrange(len(deltas)) for _ in range(len(deltas))]
        medians.append(statistics.median([deltas[index] for index in indices]))
    low = _percentile(medians, 0.025)
    high = _percentile(medians, 0.975)
    return {
        'median_delta_ms': median_delta,
        'low_ms': low,
        'high_ms': high,
        'reliable_gain': low > 0.0,
    }


if __name__ == '__main__':
    # Lazy CLI only: importing the pure accounting helpers does not start a
    # product, inspect installed software, write evidence or require psutil.
    import importlib.util
    from pathlib import Path
    path = Path(__file__).with_name('runtime-execution.py')
    spec = importlib.util.spec_from_file_location('femacs_runtime_execution', path)
    execution = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(execution)
    raise SystemExit(execution.main())
