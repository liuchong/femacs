#!/usr/bin/env python3
"""Cross-candidate comparison built on the formal runtime-profile entry.

Runs `tools/runtime-profile.py` once per candidate (same fixture, warmup and
measured rounds, one load threshold) and reports, per workload case: wall time
median/MAD/p95, allocation counters, heap inventory movement, GC count and
pause distribution, survivor ages when the observer records them, phase
intervals when phase recording is enabled, collector reason/cadence counters,
process memory and fresh-startup time.

It never promotes a result: a comparison is only marked promotion-grade when
every candidate run was quiet under the declared load threshold, and even then
the report carries observations, not a speedup claim. An empty or skipped
metric stays empty; nothing is synthesized.
"""
from __future__ import annotations
import argparse
import importlib.util
import json
import math
import os
from pathlib import Path
import statistics
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parent.parent
PROFILE_TOOL = ROOT / 'tools/runtime-profile.py'
FIXTURE = ROOT / 'tools/fixtures/runtime-profile.el'
SCHEMA = 'femacs-runtime-comparison-report-v1'
PLAN_SCHEMA = 'femacs-runtime-comparison-plan-v1'
RUN_SCHEMA = 'femacs-runtime-profile-report-v1'
EXECUTION_KINDS = ('interpreted', 'bytecode', 'native')


def load_profile_module():
    spec = importlib.util.spec_from_file_location('runtime_profile_for_comparison', PROFILE_TOOL)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def positive_number(value, name):
    if isinstance(value, bool) or not isinstance(value, (int, float)) or not math.isfinite(value) or value < 0:
        raise ValueError(f'{name} must be a finite nonnegative number')
    return value


def validate_plan(plan: dict) -> dict:
    """Reject anything the runner cannot execute as declared, before any child runs."""
    if not isinstance(plan, dict) or plan.get('schema') != PLAN_SCHEMA:
        raise ValueError(f'plan schema must be {PLAN_SCHEMA}')
    warmup = plan.get('warmup', 2)
    rounds = plan.get('rounds', 7)
    threshold = plan.get('max_load_per_core', .10)
    if not isinstance(warmup, int) or isinstance(warmup, bool) or warmup < 0:
        raise ValueError('warmup must be a nonnegative integer')
    if not isinstance(rounds, int) or isinstance(rounds, bool) or rounds <= 0:
        raise ValueError('rounds must be a positive integer')
    positive_number(threshold, 'max_load_per_core')
    candidates = plan.get('candidates')
    if not isinstance(candidates, list) or not candidates:
        raise ValueError('candidates must be a nonempty list')
    seen = set()
    for candidate in candidates:
        if not isinstance(candidate, dict):
            raise ValueError('candidate must be an object')
        name = candidate.get('name')
        if not isinstance(name, str) or not name or name in seen:
            raise ValueError('candidate names must be unique nonempty strings')
        seen.add(name)
        binary = candidate.get('binary')
        if not isinstance(binary, str) or not binary:
            raise ValueError(f'candidate {name} needs a binary path')
        dump = candidate.get('dump')
        if dump is not None and (not isinstance(dump, str) or not dump):
            raise ValueError(f'candidate {name} dump must be a path or absent')
        lisp_root = candidate.get('lisp_root')
        if lisp_root is not None and (not isinstance(lisp_root, str) or not lisp_root):
            raise ValueError(f'candidate {name} lisp_root must be a path or absent')
        env = candidate.get('env', {})
        if not isinstance(env, dict) or any(not isinstance(k, str) or not isinstance(v, str) for k, v in env.items()):
            raise ValueError(f'candidate {name} env must map strings to strings')
        expect = candidate.get('expect', {})
        if not isinstance(expect, dict):
            raise ValueError(f'candidate {name} expect must be an object')
        if set(expect) - {'execution', 'collector'}:
            raise ValueError(f'candidate {name} has unknown expectations: {sorted(set(expect))}')
        if 'execution' in expect and expect['execution'] not in EXECUTION_KINDS:
            raise ValueError(f'candidate {name} execution expectation must be one of {EXECUTION_KINDS}')
        if 'collector' in expect and expect['collector'] not in ('young', 'major'):
            raise ValueError(f'candidate {name} collector expectation must be young or major')
    comparisons = plan.get('comparisons', [])
    if not isinstance(comparisons, list):
        raise ValueError('comparisons must be a list')
    for comparison in comparisons:
        if not isinstance(comparison, dict):
            raise ValueError('comparison must be an object')
        for key in ('name', 'left', 'right'):
            if not isinstance(comparison.get(key), str) or not comparison[key]:
                raise ValueError(f'comparison needs a nonempty {key}')
        for side in ('left', 'right'):
            if comparison[side] not in seen:
                raise ValueError(f'comparison {comparison["name"]} names unknown candidate {comparison[side]}')
        if comparison['left'] == comparison['right']:
            raise ValueError(f'comparison {comparison["name"]} compares one candidate with itself')
    return dict(warmup=warmup, rounds=rounds, max_load_per_core=threshold,
                candidates=candidates, comparisons=comparisons)


def candidate_command(candidate: dict, out: Path, warmup: int, rounds: int, threshold: float) -> list[str]:
    """The profile entry runs the candidate; its own environment stays neutral.

    Candidate variables travel as explicit `--env` entries so the formal report
    records exactly the configuration the candidate ran with.
    """
    command = [sys.executable, str(PROFILE_TOOL), '--binary', candidate['binary'],
               '--rounds', str(rounds), '--warmup', str(warmup),
               '--max-load-per-core', str(threshold), '--out', str(out)]
    for name, value in sorted(candidate.get('env', {}).items()):
        command += ['--env', f'{name}={value}']
    if candidate.get('dump'):
        command += ['--dump', candidate['dump']]
    if candidate.get('lisp_root'):
        command += ['--lisp-root', candidate['lisp_root']]
    return command


def check_expectations(candidate: dict, report: dict) -> list[dict]:
    """The declared mode must be the mode that actually ran, on every sample."""
    checked = []
    expect = candidate.get('expect', {})
    samples = report.get('samples', [])
    if 'execution' in expect:
        expected = expect['execution']
        observed = sorted({row.get('actual_function_kind') for row in samples})
        if observed != [expected]:
            raise ValueError(f'candidate {candidate["name"]} expected {expected} code but ran {observed}')
        checked.append(dict(check='execution', expected=expected, observed=observed))
    if 'collector' in expect:
        expected_mode = 1 if expect['collector'] == 'young' else 0
        statuses = [row['collector_status'] for row in samples
                    if isinstance(row.get('collector_status'), list)]
        observed = sorted({status[1][0] for status in statuses})
        if not observed:
            raise ValueError(f'candidate {candidate["name"]} expected a collector but reported no status')
        if observed != [expected_mode]:
            raise ValueError(f'candidate {candidate["name"]} expected collector mode {expected_mode} but reported {observed}')
        # A mode flag is not a working collector: the run must show the cycles
        # that mode implies. Young needs completed Young cycles (reason 0 and a
        # minor count); major must not perform any minor at all.
        # Activity during this run: the last sample minus the first. An absolute
        # count inherited from process startup is not evidence that the measured
        # workloads reached the collector.
        first, last = statuses[0], statuses[-1]
        minors = last[1][4] - first[1][4]
        young_cycles = last[2][0] - first[2][0]
        if expected_mode == 1:
            active = minors > 0 and young_cycles > 0
            checked.append(dict(check='collector-activity', passed=active,
                                expected='at least one minor cycle in this run',
                                observed=dict(minors=minors, reason0=young_cycles),
                                note=None if active else
                                'young mode produced no minor: this is a reachability-only result, '
                                'not a completed G7 comparison'))
        else:
            active = minors == 0 and young_cycles == 0
            checked.append(dict(check='collector-activity', passed=active,
                                expected='no minor cycle in this run',
                                observed=dict(minors=minors, reason0=young_cycles),
                                note=None if active else 'a major-mode candidate performed a minor'))
        checked.append(dict(check='collector', expected=expect['collector'], observed=observed))
    return checked


def remembered_set(status) -> dict | None:
    """Collector-reported remembered set (write-point bookkeeping) and reasons.

    Index 1 is registry validity in the status wire, index 11 the registry
    length, and the third vector is the per-reason cycle counts. Absent status
    stays absent rather than becoming a zeroed success.
    """
    if not isinstance(status, list) or len(status) != 3:
        return None
    totals, reasons = status[1], status[2]
    if not isinstance(totals, list) or len(totals) != 24 or not isinstance(reasons, list) or len(reasons) != 13:
        return None
    return dict(valid=totals[1], length=totals[11], reasons=reasons)


def valid_status(status) -> bool:
    return (isinstance(status, list) and len(status) == 3
            and isinstance(status[1], list) and len(status[1]) == 24
            and isinstance(status[2], list) and len(status[2]) == 13)


def cadence(previous, current) -> dict | None:
    """Collector cycles that happened between two status wires.

    The fixture samples the status once per case, so a per-case cycle count is
    a difference against the previous sample in run order, not an absolute
    counter read as if it were this case's work. `previous=None` means the
    window is unknown: the absolute counters are reported and the delta fields
    stay null instead of being filled with zeros.
    """
    if not valid_status(current):
        return None
    totals, reasons_now = current[1], current[2]
    absolute = dict(minors=totals[4], majors=totals[3], last_kind=totals[2],
                    last_reason=totals[18], remembered_set_len=totals[11],
                    remembered_set_valid=totals[1],
                    reason_counts={str(index): reasons_now[index] for index in range(13)})
    if not valid_status(previous):
        return dict(absolute=absolute, delta=None, basis='absolute counters only; no earlier sample')
    totals_before, reasons_before = previous[1], previous[2]
    deltas = [reasons_now[index] - reasons_before[index] for index in range(13)]
    return dict(absolute=absolute,
                delta=dict(young_cycles=deltas[0],
                           full_reasons={str(index): deltas[index] for index in range(1, 13) if deltas[index]},
                           minors=totals[4] - totals_before[4],
                           majors=totals[3] - totals_before[3],
                           full_fallbacks=totals[17] - totals_before[17]),
                basis='difference from the previous sample in run order')


def percentile(values: list[float], fraction: float) -> float:
    return sorted(values)[max(0, math.ceil(len(values) * fraction) - 1)]


def phase_intervals(row: dict) -> list[dict]:
    """Complete, clock-valid phase records only; nothing is inferred."""
    snapshot = row.get('phase_snapshot')
    if not isinstance(snapshot, list) or len(snapshot) != 4:
        return []
    records = snapshot[2]
    if not isinstance(records, list):
        return []
    intervals = []
    for record in records:
        if not isinstance(record, list) or len(record) != 6:
            continue
        cycle, parent_cycle, phase, flags, start_ns, end_ns = record
        # 1 complete; 2 invalid clock; 4 incomplete/unwind; 16 depth overflow;
        # 32 owner mismatch; 64 counter exhaustion; 128 dropped coverage.
        # 8 marks snapshot-export overhead and keeps a usable interval.
        if not isinstance(flags, int) or flags & 1 != 1 or flags & (2 | 4 | 16 | 32 | 64 | 128):
            continue
        if end_ns < start_ns:
            continue
        intervals.append(dict(cycle=cycle, parent_cycle=parent_cycle, phase=phase,
                              start_ns=start_ns, end_ns=end_ns,
                              export_overhead=bool(flags & 8),
                              ms=(end_ns - start_ns) / 1e6))
    return intervals


def aggregate_candidate(name: str, candidate: dict, report: dict) -> dict:
    """Per-case observations plus the process-level accounting of one candidate."""
    samples = report.get('samples', [])
    # The profile writes samples in execution order, so the previous sample is
    # the actual prior status read: every per-case delta is attributed to the
    # case that followed it, not to an invented per-case reset.
    previous = None
    for row in samples:
        row['_cadence'] = cadence(previous, row.get('collector_status'))
        previous = row.get('collector_status')
    by_case: dict[str, list[dict]] = {}
    for row in samples:
        by_case.setdefault(row['case'], []).append(row)
    cases = []
    for case, rows in by_case.items():
        walls = [row['wall_ms'] for row in rows]
        median = statistics.median(walls)
        pauses = [value for row in rows for value in row['pauses_ms']]
        allocations = [0] * 7
        for row in rows:
            for index, value in enumerate(row['allocation_delta']):
                allocations[index] += int(value)
        phases: dict[int, list[float]] = {}
        for row in rows:
            for interval in phase_intervals(row):
                phases.setdefault(interval['phase'], []).append(interval['ms'])
        ages = [snap[3] for row in rows
                for snap in (row.get('phase_snapshot'),)
                if isinstance(snap, list) and len(snap) >= 4 and isinstance(snap[3], list)]
        cases.append(dict(
            case=case, n=len(rows),
            median_ms=median, mad_ms=statistics.median([abs(value - median) for value in walls]),
            p95_ms=percentile(walls, .95),
            gc_count=sum(row['gc_count'] for row in rows),
            gc_ms=sum(row['gc_ms'] for row in rows),
            median_pause_ms=statistics.median(pauses) if pauses else None,
            p95_pause_ms=percentile(pauses, .95) if pauses else None,
            max_pause_ms=max(pauses) if pauses else None,
            allocation_delta=allocations,
            setup_ms_median=statistics.median([row['setup_ms'] for row in rows]),
            heap_after_last=rows[-1]['heap_after'],
            heap_before_first=rows[0]['heap_before'],
            ages_last=ages[-1] if ages else None,
            phases=[dict(phase=phase, n=len(values), median_ms=statistics.median(values),
                         max_ms=max(values), total_ms=sum(values))
                    for phase, values in sorted(phases.items())],
            collector_status_last=rows[-1].get('collector_status'),
            remembered_set=remembered_set(rows[-1].get('collector_status')),
            cadence=rows[-1].get('_cadence'),
            execution_mode=sorted({row.get('execution_mode') for row in rows}),
            actual_function_kind=sorted({row.get('actual_function_kind') for row in rows})))
    processes = report.get('processes', [])
    startup = report.get('startup_samples', [])
    return dict(
        name=name, expected=candidate.get('expect', {}),
        candidate_env=report.get('candidate_env', {}),
        artifacts=report.get('artifacts', {}),
        source_head=report.get('source_head'),
        source_patch_sha256=report.get('source_patch_sha256'),
        runtime_image=report.get('runtime_image'),
        quiet=report.get('quiet'), max_load_per_core=report.get('max_load_per_core'),
        load_per_round=[process.get('load_per_core') for process in processes],
        peak_rss_bytes=max((process.get('peak_sampled_rss_bytes', 0) for process in processes), default=0),
        rounds=len(processes), warmup_rounds=len(report.get('warmup', [])),
        cases=cases,
        startup_ms=report.get('startup'),
        limitations=report.get('limitations'),
        checks=check_expectations(candidate, report))


def ratio(left, right):
    if left is None or right in (None, 0):
        return None
    return left / right


def activity_ok(candidate: dict) -> tuple[bool, list[str]]:
    """False when a declared collector expectation did not actually happen."""
    problems = [f"{check['check']}: {check['note']}" for check in candidate.get('checks', [])
                if check.get('passed') is False]
    return not problems, problems


def compare(name: str, left: dict, right: dict) -> dict:
    left_cases = {case['case']: case for case in left['cases']}
    right_cases = {case['case']: case for case in right['cases']}
    cases = []
    for case in sorted(set(left_cases) & set(right_cases)):
        a, b = left_cases[case], right_cases[case]
        cases.append(dict(
            case=case,
            median_ratio=ratio(a['median_ms'], b['median_ms']),
            median_ms=[a['median_ms'], b['median_ms']],
            gc_ms_ratio=ratio(a['gc_ms'], b['gc_ms']),
            max_pause_ms=[a['max_pause_ms'], b['max_pause_ms']],
            allocation_delta=[a['allocation_delta'], b['allocation_delta']],
            cadence=[a['cadence'], b['cadence']],
            phases=[dict(phase=phase_id,
                         left_median_ms=next((item['median_ms'] for item in a['phases'] if item['phase'] == phase_id), None),
                         right_median_ms=next((item['median_ms'] for item in b['phases'] if item['phase'] == phase_id), None))
                    for phase_id in sorted({item['phase'] for item in a['phases']} | {item['phase'] for item in b['phases']})]))
    quiet = bool(left['quiet'] and right['quiet'])
    active_left, problems_left = activity_ok(left)
    active_right, problems_right = activity_ok(right)
    reasons = []
    if not quiet:
        reasons.append('at least one candidate ran above the load threshold')
    reasons += problems_left + problems_right
    promotion = quiet and not reasons
    return dict(name=name, axis=f"{left['name']} vs {right['name']}", cases=cases,
                memory_ratio=ratio(left['peak_rss_bytes'], right['peak_rss_bytes']),
                startup_ratio=ratio((left['startup_ms'] or {}).get('median_ms'),
                                    (right['startup_ms'] or {}).get('median_ms')),
                quiet=quiet, promotion_grade=promotion, limitations=reasons,
                claim=('observations only; a quiet pair is still not a published speedup target'
                       if promotion else 'not promotion grade: ' + '; '.join(reasons)))


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--plan', type=Path, required=True)
    parser.add_argument('--out', type=Path, required=True)
    parser.add_argument('--require-quiet', action='store_true',
                        help='refuse to start when the host already exceeds the declared load threshold')
    options = parser.parse_args(argv)
    out = options.out.resolve()
    out.mkdir(parents=True, exist_ok=False)
    started = time.monotonic()
    report = dict(passed=False, schema=SCHEMA, candidates=[], comparisons=[],
                  plan=str(options.plan.resolve()), fixture=dict(
                      path=str(FIXTURE),
                      sha256=load_profile_module().fingerprint(FIXTURE)['sha256']),
                  host=os.uname().nodename, started_unix=time.time())
    try:
        plan = validate_plan(json.loads(options.plan.read_text()))
        report['plan'] = plan
        load = os.getloadavg()[0] / (os.cpu_count() or 1)
        report['entry_load_per_core'] = load
        if options.require_quiet and load > plan['max_load_per_core']:
            # Do not spend a full measured batch on a busy host: record the
            # current state once and stop with the release condition stated.
            raise RuntimeError(
                f'host is not quiet: load_per_core {load:.3f} > {plan["max_load_per_core"]};'
                ' the batch needs a host at or below the declared threshold before it can run')
        reports = {}
        for candidate in plan['candidates']:
            directory = out / candidate['name']
            log = out / f"{candidate['name']}.log"
            command = candidate_command(candidate, directory, plan['warmup'], plan['rounds'],
                                        plan['max_load_per_core'])
            with log.open('xb') as output:
                process = subprocess.run(command, cwd=ROOT, stdout=output,
                                         stderr=subprocess.STDOUT, check=False)
            results = directory / 'results.json'
            if process.returncode != 0 or not results.is_file():
                raise RuntimeError(f"candidate {candidate['name']} exited {process.returncode}; see {log}")
            run = json.loads(results.read_text())
            if run.get('schema') != RUN_SCHEMA or not run.get('passed'):
                raise RuntimeError(f"candidate {candidate['name']} did not produce a passed profile report")
            aggregate = aggregate_candidate(candidate['name'], candidate, run)
            aggregate['run_report'] = str(results)
            reports[candidate['name']] = aggregate
            report['candidates'].append(aggregate)
        for comparison in plan['comparisons']:
            report['comparisons'].append(compare(comparison['name'],
                                                 reports[comparison['left']],
                                                 reports[comparison['right']]))
        report['quiet'] = all(candidate['quiet'] for candidate in report['candidates'])
        report['passed'] = True
    except (OSError, ValueError, KeyError, TypeError, RuntimeError) as error:
        report['error'] = str(error)
    report['seconds'] = time.monotonic() - started
    (out / 'results.json').write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n')
    print(json.dumps(dict(passed=report['passed'], quiet=report.get('quiet'),
                          error=report.get('error'), report=str(out / 'results.json'))))
    return 0 if report['passed'] else 1


if __name__ == '__main__':
    sys.exit(main())
