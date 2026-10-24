#!/usr/bin/env python3
"""Compare complete per-test ERT outcomes. Strict by default; --collect-only records debt.

The editor's native reporter remains the authority. Raw logs, every retry and test timings
are retained separately from file summaries. An optional reference is required for this
*differential* audit, never for independently building or running the editor.
"""
from __future__ import annotations
import argparse
from dataclasses import asdict, dataclass, field
import json
import os
from pathlib import Path
import re
import resource
import subprocess
import sys
import threading
import time
import uuid

REPO = Path(__file__).resolve().parent.parent
PORT_PROG = REPO / 'build/terminal/port/src/emacs'
REF_PROG = REPO / 'build/emacs-ref/src/emacs'
PORT_TEST = REPO / 'test'
REF_TEST = REPO / 'emacs/test'
SELECTOR_DEFAULT = '(not (or (tag :expensive-test) (tag :unstable)))'
SELECTOR_PRESETS = {'default': SELECTOR_DEFAULT, 'expensive': '(not (tag :unstable))', 'all': 't'}
PRUNE_DIRS = {'manual', 'data', 'infra'}
SUMMARY = re.compile(r'^(Aborted: )?Ran (\d+) tests?, (\d+) results? as expected, (\d+) unexpected(?:, (\d+) skipped)?', re.M)
DETAIL = re.compile(r'^\s*(passed|failed|skipped|unknown|aborted|quit)\s+(\d+)/(\d+)\s+(.+?)\s+\(([0-9.eE+-]+) sec\)', re.M | re.I)
EVIDENCE_ROOT: Path | None = None

@dataclass
class RunResult:
    status: str = 'incomplete'
    ran: int = 0
    expected: int = 0
    unexpected: int = 0
    results: dict[str, str] = field(default_factory=dict)
    test_seconds: dict[str, float] = field(default_factory=dict)
    elapsed_seconds: float = 0.0
    note: str = ''
    log: str = ''
    exit_code: int | None = None


def parse_result(text: str, returncode: int, elapsed_seconds: float) -> RunResult:
    r = RunResult(elapsed_seconds=elapsed_seconds, exit_code=returncode)
    summaries = list(SUMMARY.finditer(text))
    if len(summaries) != 1:
        r.status = 'crashed' if returncode else 'incomplete'
        r.note = f'expected one final summary, found {len(summaries)}; exit={returncode}'
        return r
    m = summaries[0]
    r.ran, r.expected, r.unexpected = map(int, m.group(2, 3, 4))
    skipped = int(m.group(5) or 0)
    if m.group(1) or returncode < 0 or returncode > 1 or (returncode != 0 and r.unexpected == 0):
        r.status, r.note = 'crashed', f'aborted or inconsistent exit={returncode}'
        return r
    if returncode == 0 and r.unexpected:
        r.note = 'successful process exit contradicts unexpected results'
        return r
    positions: set[int] = set()
    totals: set[int] = set()
    for d in DETAIL.finditer(text):
        status, position, total, name, duration = d.groups()
        if name in r.results or int(position) in positions:
            r.note = 'duplicate test identity or progress position'
            return r
        try:
            seconds = float(duration)
        except ValueError:
            r.note = 'invalid per-test duration'
            return r
        if not (0 <= seconds < float('inf')):
            r.note = 'non-finite or negative test duration'
            return r
        r.results[name] = status
        r.test_seconds[name] = seconds
        positions.add(int(position)); totals.add(int(total))
    seen_skips = sum(s.lower() == 'skipped' for s in r.results.values())
    seen_unexpected = sum(s.isupper() and s.lower() != 'skipped' for s in r.results.values())
    seen_expected = len(r.results) - seen_skips - seen_unexpected
    if (len(r.results) != r.ran or positions != set(range(1, r.ran + 1))
            or (r.ran and totals != {r.ran}) or seen_skips != skipped
            or seen_unexpected != r.unexpected or seen_expected != r.expected
            or r.ran != r.expected + r.unexpected + skipped):
        r.note = 'per-test details, positions or outcomes contradict the summary'
        return r
    if any(s.lower() in {'unknown', 'aborted', 'quit'} for s in r.results.values()):
        r.note = 'an interrupted or unknown outcome is not a completed test'
        return r
    r.status = 'ok'
    return r


def compare_results(port: RunResult, reference: RunResult) -> tuple[str, str]:
    if port.status == reference.status == 'out-of-memory':
        return 'both-capped', 'both runs hit the optional per-process cap; unverified'
    if 'crashed' in {port.status, reference.status} or 'timeout' in {port.status, reference.status}:
        return 'crashed', f'port={port.status}; reference={reference.status}'
    if port.status != 'ok' or reference.status != 'ok':
        return 'incomplete', f'port={port.status}: {port.note}; reference={reference.status}: {reference.note}'
    if not port.ran or not reference.ran:
        return 'zero-tests', 'no executed tests on at least one side'
    if port.ran != reference.ran or set(port.results) != set(reference.results):
        return 'different', 'test count or identity set differs'
    if not any(s.lower() != 'skipped' for s in port.results.values()):
        return 'zero-tests', 'all selected tests were skipped'
    if port.results == reference.results:
        return ('both-fail', 'identical unexpected outcomes') if port.unexpected else ('same', '')
    improvements = regressions = 0
    for name, p in port.results.items():
        q = reference.results[name]
        if p == q:
            continue
        if p.lower() == 'skipped' or q.lower() == 'skipped':
            return 'different', f'skip status differs for {name}'
        if p.isupper() and not q.isupper():
            regressions += 1
        elif q.isupper() and not p.isupper():
            improvements += 1
        else:
            return 'different', f'outcome differs for {name}: {p} vs {q}'
    if regressions and not improvements:
        return 'port-worse', f'{regressions} per-test regressions'
    if improvements and not regressions and port.unexpected == 0:
        return 'port-better', f'{improvements} per-test improvements, no remaining unexpected results'
    return 'different', 'mixed per-test outcome differences'


def gate_exit(verdicts: list[str]) -> int:
    return 0 if verdicts and all(v in {'same', 'port-better'} for v in verdicts) else 1


def enumerate_tests(root: Path) -> list[str]:
    return sorted(str(p.relative_to(root)) for p in root.rglob('*.el')
                  if not p.name.startswith('.') and not any(
                      x in PRUNE_DIRS or x.endswith('resources') for x in p.relative_to(root).parts[:-1]))


def cap_memory(mib: int):
    def apply():
        try:
            resource.setrlimit(resource.RLIMIT_AS, (mib << 20, mib << 20))
        except (ValueError, OSError):
            pass  # Darwin may reject address-space caps; RSS watch is supplementary.
    return apply


def watchdog(proc: subprocess.Popen, mib: int, stopped: threading.Event, fired: list[bool]):
    while not stopped.wait(.5) and proc.poll() is None:
        try:
            result = subprocess.run(['ps', '-o', 'rss=', '-p', str(proc.pid)], capture_output=True, text=True, timeout=3)
            if int(result.stdout.strip() or 0) > (mib << 10):
                fired[0] = True; proc.kill(); return
        except (ValueError, OSError, subprocess.SubprocessError):
            continue


def run(prog: Path, test_root: Path, rel: str, timeout: int,
        selector: str = SELECTOR_DEFAULT, memory_mib: int = 0, prelude: str = '') -> RunResult:
    base = EVIDENCE_ROOT or REPO/'build/acceptance/ert'/str(time.monotonic_ns())
    base.mkdir(parents=True, exist_ok=True)
    log = base/(uuid.uuid4().hex + '.log')
    argv = [str(prog.resolve()), '--no-init-file', '--no-site-file', '--no-site-lisp', '-L', f':{test_root.resolve()}', '-l', 'ert']
    if prelude: argv += ['--eval', prelude]
    argv += ['-l', str((test_root/rel).resolve()), '--batch', '--eval', f'(ert-run-tests-batch-and-exit (quote {selector}))']
    started = time.monotonic(); stopped = threading.Event(); fired = [False]
    # A unique nonexistent HOME avoids loading user state and accidental shared test state.
    env = {'HOME': str(base/('home-'+uuid.uuid4().hex)), 'PATH': os.defpath, 'LC_ALL': 'C'}
    with log.open('wb') as output:
        proc = subprocess.Popen(argv, cwd=REPO, env=env, stdout=output, stderr=subprocess.STDOUT,
                                preexec_fn=cap_memory(memory_mib) if memory_mib else None)
        watcher = None
        if memory_mib:
            watcher = threading.Thread(target=watchdog, args=(proc,memory_mib,stopped,fired), daemon=True); watcher.start()
        timed_out = False
        try:
            proc.wait(timeout=timeout)
        except subprocess.TimeoutExpired:
            timed_out = True; proc.kill(); proc.wait()
        finally:
            stopped.set()
            if watcher: watcher.join(timeout=4)
            if proc.poll() is None: proc.kill(); proc.wait()
    elapsed = time.monotonic()-started
    if timed_out or fired[0]:
        r = RunResult(status='out-of-memory' if fired[0] else 'timeout', elapsed_seconds=elapsed,
                      note='run interrupted; see raw log')
    elif log.stat().st_size > 16*1024*1024:
        r = RunResult(status='incomplete', elapsed_seconds=elapsed, note='raw report exceeds bounded parser input; log retained')
    else:
        r = parse_result(log.read_text(errors='replace'), proc.returncode, elapsed)
    r.log = str(log); r.exit_code = proc.returncode
    return r


def main(argv: list[str] | None = None) -> int:
    global EVIDENCE_ROOT
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--glob', default='**/*.el'); ap.add_argument('--list', action='store_true')
    ap.add_argument('--timeout', type=int, default=300); ap.add_argument('--limit', type=int, default=40)
    ap.add_argument('--tsv', type=Path); ap.add_argument('--results-json', type=Path)
    ap.add_argument('--start-at', type=int, default=0); ap.add_argument('--max-files', type=int, default=0)
    ap.add_argument('--retries', type=int, default=0)
    ap.add_argument('--selector', default=SELECTOR_DEFAULT); ap.add_argument('--selector-preset', choices=SELECTOR_PRESETS)
    ap.add_argument('--prelude-eval', default=''); ap.add_argument('--gc-stress', action='store_true')
    ap.add_argument('--memory-mib', type=int, default=0)
    ap.add_argument('--port-prog', type=Path, default=PORT_PROG); ap.add_argument('--ref-prog', type=Path, default=REF_PROG)
    ap.add_argument('--collect-only', action='store_true')
    a = ap.parse_args(argv)
    if a.timeout <= 0 or min(a.start_at,a.max_files,a.retries,a.memory_mib) < 0:
        ap.error('timeout must be positive; range, retries and cap must be nonnegative')
    if a.gc_stress and a.prelude_eval: ap.error('--gc-stress and --prelude-eval are mutually exclusive')
    if a.gc_stress: a.prelude_eval = '(setq gc-cons-threshold 1)'
    if a.selector_preset: a.selector = SELECTOR_PRESETS[a.selector_preset]
    if not PORT_TEST.is_dir():
        print('missing owned test root', file=sys.stderr); return 2
    files = enumerate_tests(PORT_TEST) if a.glob == '**/*.el' else sorted(str(p.relative_to(PORT_TEST)) for p in PORT_TEST.glob(a.glob) if p.is_file() and not p.name.startswith('.'))
    print(f'enumerated {len(files)} file(s)')
    if not files: print('no matching test files', file=sys.stderr); return 2
    if a.list:
        for f in files: print(f)
        return 0
    for p in (a.port_prog, a.ref_prog):
        if not p.is_file(): print(f'missing program: {p}', file=sys.stderr); return 2
    if not REF_TEST.is_dir(): print('missing optional reference tests for differential audit', file=sys.stderr); return 2
    files = files[a.start_at:]
    if a.max_files: files = files[:a.max_files]
    EVIDENCE_ROOT = REPO/'build/acceptance/ert'/str(time.monotonic_ns()); EVIDENCE_ROOT.mkdir(parents=True)
    rows = []
    for rel in files:
        attempts = [run(a.port_prog,PORT_TEST,rel,a.timeout,a.selector,a.memory_mib,a.prelude_eval)]
        while attempts[-1].status != 'ok' and len(attempts) <= a.retries:
            attempts.append(run(a.port_prog,PORT_TEST,rel,a.timeout,a.selector,a.memory_mib,a.prelude_eval))
        reference = run(a.ref_prog,REF_TEST,rel,a.timeout,a.selector,a.memory_mib,a.prelude_eval)
        port = attempts[-1]; verdict, note = compare_results(port,reference)
        if len(attempts) > 1 and verdict in {'same','port-better'}:
            verdict, note = 'flaky', 'earlier incomplete/failed process attempt retained; reference still executed'
        rows.append({'file':rel,'verdict':verdict,'note':note,'port':asdict(port),
                     'reference':asdict(reference),'port_attempts':[asdict(r) for r in attempts]})
        print(f'{verdict}: {rel}', flush=True)
    payload = {'source_head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=REPO,text=True).strip(),
               'selector':a.selector,'prelude':a.prelude_eval,'port_program':str(a.port_prog.resolve()),
               'reference_program':str(a.ref_prog.resolve()),'files':rows,'collect_only':a.collect_only}
    json_path = a.results_json or EVIDENCE_ROOT/'results.json'; json_path.parent.mkdir(parents=True,exist_ok=True)
    json_path.write_text(json.dumps(payload,ensure_ascii=False,indent=2)+'\n')
    if a.tsv:
        a.tsv.parent.mkdir(parents=True,exist_ok=True)
        with a.tsv.open('w') as out:
            out.write('file\tverdict\tport_ran\tport_unexpected\tref_ran\tref_unexpected\tnote\n')
            for r in rows:
                fields=[r['file'],r['verdict'],r['port']['ran'],r['port']['unexpected'],r['reference']['ran'],r['reference']['unexpected'],r['note']]
                out.write('\t'.join(str(f).replace('\t',' ').replace('\n',' ') for f in fields)+'\n')
    print(f"test files: {len(rows)}; gate={'pass' if gate_exit([r['verdict'] for r in rows]) == 0 else 'fail'}; JSON: {json_path}")
    return 0 if a.collect_only else gate_exit([r['verdict'] for r in rows])

if __name__ == '__main__': sys.exit(main())
