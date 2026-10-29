#!/usr/bin/env python3
"""Real Lisp/collector observations; run under controlled-run, never a speedup claim.

Preserves all instrumented workload samples. Heap inventories describe the last
collector observation, not exact allocation sizes, object ages or all live memory.
"""
from __future__ import annotations
import argparse
import hashlib
import json
import math
import os
from pathlib import Path
import platform
import statistics
import subprocess
import sys
import time
import psutil

ROOT = Path(__file__).resolve().parent.parent
FIXTURE = ROOT / 'tools/fixtures/runtime-profile.el'
EXPECTED = {'load': '5', 'edit': '1200', 'regex': '1000', 'bytecode': '1249975000',
            'gc-churn': '30000', 'gc-retained': '10000'}


def new_output(path: Path) -> Path:
    path = Path(path).resolve()
    path.mkdir(parents=True, exist_ok=False)
    return path


def number(value, integer=False):
    if (isinstance(value, bool) or not isinstance(value, (int, float))
            or not math.isfinite(value) or value < 0
            or (integer and not isinstance(value, int))):
        raise ValueError('invalid nonnegative numeric observation')
    return value


def parse_profile(text: str) -> list[dict]:
    markers = [line[len('FEMACS-PROFILE '):] for line in text.splitlines()
               if line.startswith('FEMACS-PROFILE ')]
    if len(markers) != 1:
        raise ValueError('expected exactly one completed profile marker')
    payload = json.loads(markers[0])
    if not isinstance(payload, dict) or payload.get('schema') != 'femacs-runtime-profile-v1':
        raise ValueError('invalid profile schema')
    rows = payload.get('cases')
    if not isinstance(rows, list) or len(rows) != len(EXPECTED):
        raise ValueError('incomplete workload cases')
    seen = set()
    for row in rows:
        if not isinstance(row, dict):raise ValueError('case must be an object')
        name = row.get('case')
        if name not in EXPECTED or name in seen or row.get('result') != EXPECTED[name]:
            raise ValueError('unknown/duplicate case or incorrect deterministic result')
        seen.add(name)
        for key in ('wall_ms', 'gc_ms'):number(row.get(key))
        number(row.get('gc_count'), True)
        pauses = row.get('pauses_ms')
        if not isinstance(pauses, list) or len(pauses) != row['gc_count']:
            raise ValueError('GC count/pause observations disagree')
        for value in pauses:number(value)
        # Timers expose collector elapsed deltas; tiny floating point rounding is allowed.
        if not math.isclose(sum(pauses), row['gc_ms'], rel_tol=1e-5, abs_tol=.01):
            raise ValueError('GC total and pause observations disagree')
        allocations = row.get('allocation_delta')
        if not isinstance(allocations, list) or len(allocations) != 7:
            raise ValueError('expected seven allocation counters')
        for value in allocations:number(value, True)
        for key in ('heap_before', 'heap_after'):
            heap = row.get(key)
            if not isinstance(heap, list) or not heap:
                raise ValueError('missing collector-reported inventory')
            kinds = set()
            for entry in heap:
                if (not isinstance(entry, list) or len(entry) not in (3, 4)
                        or not isinstance(entry[0], str) or entry[0] in kinds):
                    raise ValueError('invalid or duplicate heap inventory row')
                kinds.add(entry[0])
                for value in entry[1:]:number(value, True)
    if seen != set(EXPECTED):raise ValueError('missing workload case')
    return rows


def percentile(xs: list[float], fraction: float) -> float:
    return sorted(xs)[max(0, math.ceil(len(xs) * fraction) - 1)]


def summarize(rows: list[dict]) -> list[dict]:
    if not rows:raise ValueError('empty profile')
    groups = {};seen = set()
    for row in rows:
        name = row['case'];identity = name, row['round']
        if name not in EXPECTED or identity in seen:raise ValueError('invalid or duplicate sample identity')
        seen.add(identity);number(row['wall_ms'])
        groups.setdefault(name, []).append(row)
    result = []
    for name, samples in groups.items():
        xs = [r['wall_ms'] for r in samples];median = statistics.median(xs)
        pauses = [p for r in samples for p in r['pauses_ms']]
        result.append(dict(case=name, n=len(xs), median_ms=median,
                           mad_ms=statistics.median([abs(x-median) for x in xs]),
                           p95_ms=percentile(xs, .95), gc_count=sum(r['gc_count'] for r in samples),
                           gc_ms=sum(r['gc_ms'] for r in samples),
                           median_pause_ms=statistics.median(pauses) if pauses else None,
                           p95_pause_ms=percentile(pauses, .95) if pauses else None,
                           max_pause_ms=max(pauses) if pauses else None))
    return result


def fingerprint(path: Path) -> dict:
    h = hashlib.sha256()
    with path.open('rb') as source:
        for chunk in iter(lambda: source.read(1024*1024), b''):h.update(chunk)
    return dict(path=str(path), sha256=h.hexdigest(), bytes=path.stat().st_size)


def profile_inputs(binary: Path, dump: Path | None = None) -> tuple[Path, Path]:
    """Select a real product image, never an arbitrary/bootstrap dump."""
    binary = binary.resolve(strict=True)
    if not binary.is_file():raise ValueError('profile binary must be a file')
    bundle = binary.parent.name == 'MacOS' and binary.parent.parent.name == 'Contents'
    lisp = binary.parent.parent / ('Resources/lisp' if bundle else 'lisp')
    if not lisp.is_dir():raise ValueError('missing product Lisp directory')
    if dump is None:
        candidates = [binary.parent / (binary.name + '.pdmp')]
        if bundle:
            candidates.extend(directory / (binary.name + '.pdmp') for directory in
                              (binary.parent.parent / 'Resources/libexec',
                               binary.parent / 'libexec'))
        # Two aliases for the same image are one choice; two distinct images
        # need an explicit selection even when their contents happen to match.
        found = {path.resolve() for path in candidates if path.is_file()}
        if len(found) != 1:
            raise ValueError('expected one named product dump; select --dump explicitly')
        dump = found.pop()
    else:
        dump = dump.resolve(strict=True)
        if not dump.is_file():raise ValueError('profile dump must be a file')
    return dump, lisp.resolve(strict=True)


def artifacts(binary: Path, dump: Path | None = None) -> dict:
    binary = binary.resolve(strict=True)
    dump, lisp = profile_inputs(binary, dump)
    paths = [('binary', binary), ('dump', dump), ('fixture', FIXTURE),
             ('tool', Path(__file__).resolve())]
    # A stable executable/dump cannot prove the libraries stayed unchanged while
    # another build regenerated them. Bind both source and selected bytecode.
    for name in ('byte-opt', 'bytecomp', 'macroexp', 'cconv', 'cl-extra'):
        for suffix in ('.el', '.elc'):
            path = lisp / 'emacs-lisp' / (name+suffix)
            if suffix == '.el' or path.is_file():paths.append(('lisp/'+name+suffix, path))
    return {key:fingerprint(path) for key,path in paths}


def measure(binary: Path, argv: list[str], home: Path, log: Path, timeout=120) -> dict:
    env = dict(HOME=str(home), TMPDIR=str(home), PATH=os.defpath, LC_ALL='en_US.UTF-8')
    home.mkdir(parents=True, exist_ok=False)
    started = time.perf_counter();process = None;peak = 0;samples = 0
    with log.open('xb') as output:
        try:
            process = subprocess.Popen([str(binary), '-Q', '--batch', *argv], cwd=home,
                                       env=env, stdout=output, stderr=subprocess.STDOUT)
            observed = psutil.Process(process.pid)
            while process.poll() is None:
                if time.perf_counter()-started > timeout:raise TimeoutError('profile process timeout')
                try:
                    peak = max(peak, observed.memory_info().rss);samples += 1
                except psutil.NoSuchProcess:pass
                time.sleep(.01)
            if process.returncode:raise RuntimeError(f'profile process exit {process.returncode}: {log}')
        finally:
            if process is not None and process.poll() is None:
                process.terminate()
                try:process.wait(timeout=2)
                except subprocess.TimeoutExpired:process.kill();process.wait(timeout=2)
    return dict(wall_ms=(time.perf_counter()-started)*1000, peak_sampled_rss_bytes=peak,
                rss_samples=samples, sampling_interval_seconds=.01)


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--binary', type=Path, required=True)
    ap.add_argument('--dump', type=Path,
                    help='Explicit runtime image; otherwise require one named product dump')
    ap.add_argument('--rounds', type=int, default=7)
    ap.add_argument('--out', type=Path, required=True)
    ap.add_argument('--max-load-per-core', type=float, default=.10)
    args = ap.parse_args(argv)
    if args.rounds <= 0 or not math.isfinite(args.max_load_per_core) or args.max_load_per_core < 0:
        ap.error('positive rounds and finite nonnegative load threshold required')
    out = new_output(args.out);started = time.monotonic()
    report = dict(passed=False, schema='femacs-runtime-profile-report-v1', samples=[],
                  processes=[], startup_samples=[], host=platform.platform())
    try:
        binary = args.binary.resolve(strict=True)
        dump, _ = profile_inputs(binary, args.dump)
        identity = artifacts(binary, dump);report['artifacts'] = identity
        report['runtime_image'] = str(dump)
        image_args = ['--dump-file=' + str(dump)]
        report['source_head'] = subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip()
        report['source_patch_sha256'] = hashlib.sha256(subprocess.check_output(['git','diff','HEAD','--'],cwd=ROOT)).hexdigest()
        loads = []
        for n in range(1, args.rounds+1):
            load = os.getloadavg()[0]/(os.cpu_count() or 1);loads.append(load)
            log = out/f'round-{n}.log'
            process = measure(binary, [*image_args, '-l', str(FIXTURE)], out/f'home-{n}', log)
            if log.stat().st_size > 4*1024*1024:raise ValueError('oversized profile log')
            rows = parse_profile(log.read_text())
            report['samples'].extend(dict(row, round=n) for row in rows)
            report['processes'].append(dict(process, round=n, load_per_core=load))
            startup = measure(binary, [*image_args, '--eval', '(kill-emacs 0)'], out/f'startup-home-{n}', out/f'startup-{n}.log')
            report['startup_samples'].append(startup)
            print(json.dumps(dict(round=n, cases=len(rows), **process)), flush=True)
        if artifacts(binary, dump) != identity:raise ValueError('artifact changed during profiling')
        report['summary'] = summarize(report['samples'])
        wall = [r['wall_ms'] for r in report['startup_samples']]
        report['startup'] = dict(metric='fresh-process-wall-ms; filesystem cache uncontrolled',
                                 median_ms=statistics.median(wall), p95_ms=percentile(wall,.95))
        report['max_load_per_core'] = max(loads)
        report['quiet'] = all(load <= args.max_load_per_core for load in loads)
        report['limitations'] = ('Instrumented workload observations, not an overall editor speedup. '
            'Heap inventory may be stale since last GC; allocation counters are object/slot/character counts, not bytes or object ages. '
            'post-gc-hook captures collector elapsed deltas, not per-phase timing or application response latency. '
            'RSS is sampled, may miss peaks and excludes compressed footprint; controlled-run reports the protected whole-tree footprint. '
            'Startup wall time includes polling overhead up to 10ms. '
            'No promotion-grade performance claim when quiet=false.')
        report['passed'] = True
    except (OSError, ValueError, KeyError, TypeError, RuntimeError, TimeoutError, subprocess.SubprocessError, psutil.Error) as error:
        report['error'] = str(error)
    report['seconds'] = time.monotonic()-started
    (out/'results.json').write_text(json.dumps(report, ensure_ascii=False, indent=2)+'\n')
    print(json.dumps(dict(passed=report['passed'], quiet=report.get('quiet'), error=report.get('error'), report=str(out/'results.json'))))
    return 0 if report['passed'] else 1


if __name__ == '__main__':sys.exit(main())
