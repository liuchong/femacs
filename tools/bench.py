#!/usr/bin/env python3
"""Measure the actual editor; optional reference, complete samples, explicit baseline metric.

Run the whole invocation under the resource controller. This tool never builds a
product, fetches a dependency or starts another controller. Existing shell callers
may keep using bench.sh. Old minimum-only baseline files are deliberately rejected.
"""
from __future__ import annotations
import argparse
import csv
from datetime import date
import hashlib
import importlib.util
import json
import math
import os
from pathlib import Path
import re
import subprocess
import sys
import tempfile
import time

ROOT = Path(__file__).resolve().parent.parent
CORPUS = ROOT / 'tools/pgo/eval.el'
spec = importlib.util.spec_from_file_location('bench_statistics', ROOT / 'tools/pgo_bench.py')
shared = importlib.util.module_from_spec(spec)
spec.loader.exec_module(shared)
MeasurementError = shared.MeasurementError
MICRO_CASES = shared.MICRO_CASES
TASK_CASES = ['load', 'edit', 'regex', 'gc']
CASES = MICRO_CASES + TASK_CASES
METRIC = 'process-median-of-min5-v1'
TIMEOUT = 300
_MACHO_MAGICS = (
    b'\xcf\xfa\xed\xfe',
    b'\xfe\xed\xfa\xcf',
    b'\xce\xfa\xed\xfe',
    b'\xfe\xed\xfa\xce',
    b'\xca\xfe\xba\xbe',
    b'\xbe\xba\xfe\xca',
)


def _has_macho_magic(path: Path) -> bool:
    try:
        with path.open('rb') as handle:
            return handle.read(4) in _MACHO_MAGICS
    except OSError:
        return False


def probe_bundle_dump(binary: Path) -> Path:
    expr = (
        "(progn (require 'json) (princ \"FEMACS-BUNDLE-DUMP \") "
        "(let ((stats (and (fboundp 'pdumper-stats) (pdumper-stats)))) "
        "(princ (json-encode (or (cdr (assq 'dump-file-name stats)) "
        "(cdr (assq 'dump-file stats)))))) (terpri))"
    )
    with tempfile.TemporaryDirectory() as tmp:
        env = dict(HOME=tmp, TMPDIR=tmp, PATH=os.defpath, LC_ALL='C')
        res = subprocess.run([str(binary), '-Q', '--batch', '--eval', expr],
                             cwd=tmp, env=env, capture_output=True, text=True,
                             timeout=15, check=False)
    if res.returncode != 0:
        raise MeasurementError(f'missing dump: {binary.with_suffix(".pdmp")}')
    lines = [line[len('FEMACS-BUNDLE-DUMP '):]
             for line in (res.stdout + '\n' + res.stderr).splitlines()
             if line.startswith('FEMACS-BUNDLE-DUMP ')]
    if len(lines) != 1:
        raise MeasurementError(f'missing dump: {binary.with_suffix(".pdmp")}')
    raw = json.loads(lines[0])
    if not isinstance(raw, str) or not raw:
        raise MeasurementError(f'missing dump: {binary.with_suffix(".pdmp")}')
    dump = Path(raw).resolve()
    if not dump.is_file():
        raise MeasurementError(f'missing dump: {dump}')
    return dump


def discover_dump(binary: Path, explicit: Path | None = None) -> Path:
    if explicit is not None:
        resolved = Path(explicit).resolve()
        if not resolved.is_file():
            raise MeasurementError(f'missing dump: {resolved}')
        return resolved
    binary = Path(binary).resolve()
    default = binary.with_suffix('.pdmp')
    candidates = [default]
    bundle = binary.parent.name == 'MacOS' and binary.parent.parent.name == 'Contents'
    if bundle:
        candidates.extend(directory / (binary.name + '.pdmp') for directory in
                          (binary.parent.parent / 'Resources/libexec',
                           binary.parent / 'libexec'))
    found = {path.resolve() for path in candidates if path.is_file()}
    if len(found) == 1:
        return found.pop()
    if len(found) > 1:
        raise MeasurementError(f'ambiguous dump for {binary}')
    if bundle and _has_macho_magic(binary):
        return probe_bundle_dump(binary)
    raise MeasurementError(f'missing dump: {default}')


def new_output(path: Path) -> Path:
    path = Path(path).resolve()
    path.mkdir(parents=True, exist_ok=False)
    return path


def parse_samples(text: str) -> dict[str, float]:
    found = {}
    for line in text.splitlines():
        line = line.strip()
        if not line.startswith(('BENCH ', 'TASK ')):
            continue
        m = re.fullmatch(r'(BENCH|TASK)\s+(\S+)\s+(\S+)\s+ms', line)
        if not m:
            raise MeasurementError('malformed measurement marker')
        kind, name, raw = m.groups()
        expected = MICRO_CASES if kind == 'BENCH' else TASK_CASES
        if name not in expected or name in found:
            raise MeasurementError(f'unknown or duplicate measurement: {name}')
        try:
            value = float(raw)
        except ValueError as e:
            raise MeasurementError(f'invalid measurement: {name}') from e
        if not math.isfinite(value) or value <= 0:
            raise MeasurementError(f'nonpositive/nonfinite measurement: {name}')
        found[name] = value
    if set(found) != set(CASES):
        raise MeasurementError(f'incomplete measurement: {sorted(set(CASES) - set(found))}')
    return found


def summarize(rows: list[dict]) -> list[dict]:
    if not rows:
        raise MeasurementError('no samples')
    groups = {}
    identities = set()
    for row in rows:
        key = row['case'], row['side']
        identity = (*key, row['round'])
        value = row['value_ms']
        if identity in identities or not math.isfinite(value) or value <= 0:
            raise MeasurementError('duplicate or invalid sample')
        identities.add(identity)
        groups.setdefault(key, []).append(value)
    return [dict(case=case, side=side, n=len(xs), min=min(xs), median=shared.median(xs),
                 mad=shared.mad(xs), iqr=shared.iqr(xs))
            for (case, side), xs in groups.items()]


def isolated_env(home: Path) -> dict[str, str]:
    return dict(HOME=str(home), TMPDIR=str(home), PATH=os.defpath, LC_ALL='C')


def process(binary: Path, arguments: list[str], home: Path, log: Path) -> float:
    started = time.perf_counter()
    with log.open('xb') as output:
        try:
            result = subprocess.run([str(binary), '--batch', '-Q', *arguments], cwd=ROOT,
                                    env=isolated_env(home), stdout=output,
                                    stderr=subprocess.STDOUT, timeout=TIMEOUT)
        except (OSError, subprocess.TimeoutExpired) as e:
            output.write(f'process failure: {type(e).__name__}\n'.encode())
            raise MeasurementError(f'process failed; see {log}') from e
    if result.returncode:
        raise MeasurementError(f'exit {result.returncode}; see {log}')
    return (time.perf_counter() - started) * 1000


def measure(binary: Path, home: Path, log: Path) -> dict:
    binary = Path(binary).resolve()
    if log.exists() or log.with_name(log.stem + '-startup.log').exists():
        raise FileExistsError('measurement log already exists')
    home = Path(home).resolve()
    home.mkdir(parents=True, exist_ok=True)
    corpus_ms = process(binary, ['-l', str(CORPUS)], home, log)
    if log.stat().st_size > 16 * 1024 * 1024:
        raise MeasurementError('measurement log exceeds bounded parser input')
    text = log.read_text(errors='replace')
    if any(token in text.lower() for token in ('fatal error', 'segmentation fault', 'abort trap')):
        raise MeasurementError('fatal diagnostic in measurement log')
    samples = parse_samples(text)
    if 'PGO-EVAL-DONE' not in text:
        raise MeasurementError('missing corpus completion marker')
    wall_ms = process(binary, ['--eval', '(kill-emacs 0)'], home,
                      log.with_name(log.stem + '-startup.log'))
    return dict(samples=[dict(case=name, value_ms=value) for name, value in samples.items()],
                wall_ms=wall_ms, corpus_wall_ms=corpus_ms, exit_code=0)


def load_baseline(path: Path) -> dict[str, float]:
    with Path(path).open(newline='') as source:
        reader = csv.DictReader(source, delimiter='\t')
        if reader.fieldnames != ['case', 'port_ms', 'ref_ms', 'date', 'metric']:
            raise MeasurementError('incompatible legacy baseline; record the new metric explicitly')
        found = {}
        for row in reader:
            name = row['case']
            try:
                value = float(row['port_ms'])
            except (TypeError, ValueError) as e:
                raise MeasurementError('invalid baseline value') from e
            if (name not in CASES or name in found or row['metric'] != METRIC
                    or not math.isfinite(value) or value <= 0):
                raise MeasurementError('invalid, duplicate or incompatible baseline row')
            found[name] = value
        if set(found) != set(CASES):
            raise MeasurementError('incomplete baseline')
        return found


def gate_rows(summary: list[dict], baseline: dict[str, float], tolerance: float) -> list[str]:
    selected = [r for r in summary if r['side'] == 'candidate' and r['case'] != 'startup']
    current = {r['case']: r['median'] for r in selected}
    if len(selected) != len(CASES) or set(current) != set(CASES) or set(baseline) != set(CASES):
        raise MeasurementError('incomplete gate input')
    if (not math.isfinite(tolerance) or tolerance < 0
            or any(not math.isfinite(v) or v <= 0 for v in [*current.values(), *baseline.values()])):
        raise MeasurementError('invalid gate measurement or tolerance')
    return [name for name in CASES if current[name] > baseline[name] * (1 + tolerance)]


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--prog', type=Path, default=ROOT / 'build/terminal/port/src/emacs')
    parser.add_argument('--dump', type=Path)
    parser.add_argument('--ref', type=Path)
    parser.add_argument('--ref-dump', type=Path)
    parser.add_argument('--rounds', type=int, default=5)
    parser.add_argument('--out', type=Path, default=ROOT / 'build/bench' / str(time.monotonic_ns()))
    group = parser.add_mutually_exclusive_group()
    group.add_argument('--save', type=Path)
    group.add_argument('--gate', type=Path)
    parser.add_argument('--tolerance', type=float, default=.15)
    parser.add_argument('--max-load-per-core', type=float, default=.10)
    parser.add_argument('--force', action='store_true')
    args = parser.parse_args(argv)
    if args.rounds <= 0 or any(not math.isfinite(v) or v < 0 for v in (args.tolerance, args.max_load_per_core)):
        parser.error('rounds must be positive; tolerance/load threshold finite and nonnegative')
    started = time.monotonic()
    report = dict(passed=False, metric=METRIC, startup_metric='process-wall-ms', samples=[], stats=[])
    out = None
    try:
        out = new_output(args.out)
        binaries = {'candidate': args.prog.resolve()}
        if args.ref:
            binaries['reference'] = args.ref.resolve()
        report['source_head'] = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip()
        patch = subprocess.check_output(['git', 'diff', 'HEAD', '--'], cwd=ROOT)
        report['tracked_patch_sha256'] = hashlib.sha256(patch).hexdigest()
        report['artifacts'] = {}
        dumps: dict[str, Path] = {}
        for side, binary in binaries.items():
            if not binary.is_file() or not os.access(binary, os.X_OK):
                raise MeasurementError(f'missing executable: {binary}')
            dump = discover_dump(binary, args.dump if side == 'candidate' else args.ref_dump)
            dumps[side] = dump
            report['artifacts'][side] = dict(binary=str(binary), binary_sha256=shared.sha256_file(binary),
                                             dump=str(dump), dump_sha256=shared.sha256_file(dump))
        report['corpus_sha256'] = shared.sha256_file(CORPUS)
        report['tool_sha256'] = shared.sha256_file(Path(__file__))
        report['forced_baseline_save'] = bool(args.save and args.force)
        report['load_per_core'] = os.getloadavg()[0] / (os.cpu_count() or 1)
        if args.save and not args.force and report['load_per_core'] > args.max_load_per_core:
            raise MeasurementError('refusing to save a busy-machine baseline; retry when quiet or explicitly --force')
        baseline = load_baseline(args.gate) if args.gate else None
        load_per_round: list[float] = []
        for round_number in range(1, args.rounds + 1):
            load_per_round.append(os.getloadavg()[0] / (os.cpu_count() or 1))
            order = list(binaries)
            if round_number % 2 == 0:
                order.reverse()
            for position, side in enumerate(order, 1):
                home = out / f'round-{round_number}-{side}'
                result = measure(binaries[side], home, out / f'round-{round_number}-{side}.log')
                values = {item['case']: item['value_ms'] for item in result['samples']}
                values['startup'] = result['wall_ms']
                for case, value in values.items():
                    report['samples'].append(dict(round=round_number, order=position, side=side, case=case, value_ms=value))
        report['load_per_round'] = load_per_round
        report['max_load_per_core'] = max([report['load_per_core'], *load_per_round])
        report['quiet'] = bool(report['max_load_per_core'] <= args.max_load_per_core)
        if args.save and not args.force and not report['quiet']:
            raise MeasurementError('refusing to save a busy-machine baseline; retry when quiet or explicitly --force')
        for side, binary in binaries.items():
            identity = report['artifacts'][side]
            if (shared.sha256_file(binary) != identity['binary_sha256']
                    or shared.sha256_file(dumps[side]) != identity['dump_sha256']):
                raise MeasurementError('product changed during measurement')
        report['stats'] = summarize(report['samples'])
        regressions = gate_rows(report['stats'], baseline, args.tolerance) if baseline else []
        report['regressions'] = regressions
        if args.save:
            table = {(r['case'], r['side']): r['median'] for r in report['stats']}
            args.save.parent.mkdir(parents=True, exist_ok=True)
            with args.save.open('x', newline='') as target:
                writer = csv.writer(target, delimiter='\t', lineterminator='\n')
                writer.writerow(['case', 'port_ms', 'ref_ms', 'date', 'metric'])
                for case in CASES:
                    reference = table.get((case, 'reference'))
                    writer.writerow([case, f'{table[case,"candidate"]:.4f}', '' if reference is None else f'{reference:.4f}', date.today(), METRIC])
        report['passed'] = not regressions
        for row in report['stats']:
            print(f'{row["side"]:4} {row["case"]:12} n={row["n"]} median={row["median"]:.4f} MAD={row["mad"]:.4f} IQR={row["iqr"]:.4f} ms')
        return 0 if report['passed'] else 1
    except (OSError, ValueError, MeasurementError, subprocess.SubprocessError) as e:
        report['error'] = str(e)
        print(str(e), file=sys.stderr)
        return 2
    finally:
        if out:
            report['elapsed_seconds'] = time.monotonic() - started
            (out / 'results.json').write_text(json.dumps(report, indent=2) + '\n')


if __name__ == '__main__':
    raise SystemExit(main())
