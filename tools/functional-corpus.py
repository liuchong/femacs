#!/usr/bin/env python3
"""Official-scale functional corpus lane.

Runs the original test files against one built product with upstream-level
isolation (one fresh HOME/TMPDIR per file, the corpus as source context, no
per-file copy of the test tree), records every per-case event, and compares the
result with an already closed strict reference baseline.

Judgement rules:

* A file passes only on a normal exit, a complete `finished` record, exact
  requested/ended counters and an exact unique registration set.  Timeouts,
  crashes, protocol errors and partial runs are never a pass.
* A baseline is only used when its manifest identity, entry point set and the
  per-file input identity all match the current manifest; otherwise the file is
  marked `baseline-unavailable` and no verdict is invented.
* `failed` is never compared as a bare string: the recorded condition must also
  match, and a differing condition stays `pending-condition-diff`, which counts
  as unresolved rather than as same or worse.
"""
from __future__ import annotations

import argparse
import hashlib
import re
import shutil
import tempfile
import json
import os
import subprocess
import sys
import time
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
SAFE_LIMIT = 600


def load_module(name, path):
    import importlib.util
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


def digest(path):
    handle = hashlib.sha256()
    with Path(path).open('rb') as stream:
        for block in iter(lambda: stream.read(1 << 20), b''):
            handle.update(block)
    return handle.hexdigest()


def read_json(path):
    return json.loads(Path(path).read_text(encoding='utf-8'))


def finished_from_progress(result, pair_root):
    """Read the closed `finished` record from the recorded event stream.

    The strict per-file result schema carries no `finished` field: it binds the
    progress file instead, so closure is read from that stream and never
    assumed from a missing key.
    """
    pin = result.get('progress') or {}
    relative = pin.get('path')
    if not isinstance(relative, str) or not relative:
        return None
    path = Path(pair_root) / relative
    if not path.is_file():
        return None
    if pin.get('sha256') and digest(path) != pin['sha256']:
        return None
    found = None
    for line in path.read_text(errors='replace').splitlines():
        if not line.strip():
            continue
        try:
            event = json.loads(line)
        except json.JSONDecodeError:
            return None
        if event.get('event') == 'finished':
            found = event
    return found


def baseline_index(root):
    """Index a closed strict pair's reference file results by logical path."""
    index = {}
    root = Path(root)
    results = root / 'results' / 'reference'
    for path in sorted(results.glob('*.json')):
        if path.name.endswith('.environment.json'):
            continue
        try:
            value = read_json(path)
        except (OSError, ValueError):
            continue
        if value.get('schema') != 'femacs-stable-file-batch-v1':
            continue
        cases = value.get('cases') or {}
        ids = value.get('ids') or []
        finished = finished_from_progress(value, root)
        closed = (value.get('complete') is True and value.get('exit_code') == 0
                  and value.get('timed_out') is not True
                  and value.get('remaining_group_terminated') is not True
                  and finished is not None and finished.get('status') == 'complete'
                  and finished.get('requested_count') == len(ids)
                  and finished.get('ended_count') == len(cases)
                  and len(ids) == len(set(ids)) and set(ids) == set(cases))
        fixture = value.get('working_fixture') or {}
        # The upstream-level lane runs from the corpus itself, so sibling files
        # need no copy.  A file is out of this lane only when its strict run had
        # to build something the corpus cannot supply on its own.
        lane_eligible = not any(fixture.get(key) for key in (
            'module_layout', 'source_context_layout', 'relative_path_layout',
            'generated_working_members', 'changed_working_members',
            'outcome_overrides', 'original_test_body_modified'))
        index[value['file']] = dict(
            closed=closed,
            lane_eligible=lane_eligible,
            exit_code=value.get('exit_code'),
            finished_status=(finished or {}).get('status'),
            cases={key: dict(status=row.get('status'),
                             diagnostic=row.get('diagnostic', ''))
                   for key, row in cases.items()})
    return index


VOLATILE = (
    (re.compile(r'#<[^<>]*?\s[0-9a-f]{6,}>'), '#<object>'),
    # per-run owned containers and per-run temporary/buffer suffixes
    (re.compile(r'/\.noindex/[A-Za-z0-9_]+/'), '/.noindex/<container>/'),
    (re.compile(r'\*temp\*-\d+'), '*temp*-<n>'),
    (re.compile(r'\*-\d{5,}'), '*-<n>'),
    # progress percentages and reported durations inside messages
    (re.compile(r'\b\d+%'), '<n>%'),
    (re.compile(r'\bin \d+ secs'), 'in <n> secs'),
    # per-run fixture and native-compiler cache names
    (re.compile(r'eglot--fixture-[A-Za-z0-9]{6}'), 'eglot--fixture-<id>'),
    (re.compile(r'emacs-test-[A-Za-z0-9]{6}-'), 'emacs-test-<id>-'),
    (re.compile(r'compiler-output/32\.0\.50-[0-9a-f]+'), 'compiler-output/32.0.50-<hash>'),
    (re.compile(r'-[0-9a-f]{8}(?:-[0-9a-f]{8})?\.eln'), '-<hash>.eln'),
)


def normalize(text):
    """Drop only run-local identities, never the condition itself."""
    text = text or ''
    for pattern, replacement in VOLATILE:
        text = pattern.sub(replacement, text)
    return text


def case_verdict(baseline, observed):
    """Compare one case; conditions are part of the identity of a failure."""
    old, new = baseline['status'], observed['status']
    if old == new:
        if new != 'failed':
            return 'same'
        return ('same' if normalize(baseline['diagnostic']) == normalize(observed['diagnostic'])
                else 'pending-condition-diff')
    if old == 'passed' and new != 'passed':
        return 'port-worse'
    if old == 'failed' and new == 'skipped':
        return 'port-worse'
    if old in ('failed', 'skipped') and new == 'passed':
        return 'resolved'
    if old == 'skipped' and new == 'failed':
        return 'port-worse'
    return 'pending-condition-diff'


def materialize_corpus(suite, batch, corpus, manifest, entries, out):
    """Copy the selected sources and their fixtures into OUT/corpus.

    The stable corpus keeps the 31.1 test bodies in
    ``compatibility/stable-31.1/`` and their fixture files (``*-resources``,
    ``test/data``) in the repository's own ``test/`` tree.  A file loaded from
    the compatibility copy resolves ``ert-resource-file`` next to *itself*, so
    the upstream-level lane cannot reach those fixtures unless they are
    materialized beside the loaded file.  This does exactly that, verifies every
    copied byte against the manifest identity, and records the overlay so the
    run's evidence names the inputs it actually used instead of a scratch tree.
    """
    import shutil
    overlay = out / 'corpus'
    overlay.mkdir()
    records = {'files': [], 'symlinks': []}
    prefix = Path('compatibility/stable-31.1')
    selected = list(entries)
    fixture_paths = []
    for entry in selected:
        fixture_paths.extend(batch._fixture_entries(manifest, entry['path']))
    by_path = {}
    for entry in list(selected) + fixture_paths:
        by_path[entry['path']] = entry
    for entry in by_path.values():
        relative = Path(entry['source_path'])
        source = corpus / relative
        if not source.is_file():
            raise SystemExit('corpus overlay source is missing: %s' % source)
        data = source.read_bytes()
        if len(data) != entry['size'] or digest(source) != entry['sha256']:
            raise SystemExit('corpus overlay source identity differs: %s' % source)
        targets = [overlay / relative]
        # A fixture that lives in the repository test tree also belongs next to
        # the compatibility copy of the test file that loads it.
        if relative.parts and relative.parts[0] == 'test':
            targets.append(overlay / prefix / relative)
        for target in targets:
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(data)
            target.chmod(int(entry['mode'], 8) & 0o777)
            records['files'].append(dict(path=str(target.relative_to(overlay)),
                                         bytes=len(data), sha256=digest(target),
                                         mode=entry['mode']))
    for name in ('test', 'lisp', 'src'):
        source = corpus / name
        if not source.is_dir():
            continue
        link = overlay / name
        if link.exists():
            continue
        link.symlink_to(source, target_is_directory=True)
        records['symlinks'].append(dict(name=name, target=str(source)))
    identity = dict(schema='femacs-corpus-overlay-v1', root=str(overlay),
                    source_corpus=str(corpus), manifest_sha256=digest(
                        corpus / 'compatibility/stable-31.1/sources.json'),
                    files=records['files'], symlinks=records['symlinks'],
                    rules='manifest-identical source and fixture copies; repository trees linked read-only')
    (out / 'corpus-overlay.json').write_text(json.dumps(identity, indent=1) + '\n')
    return overlay, identity


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--program', type=Path, required=True)
    parser.add_argument('--dump', type=Path, required=True)
    parser.add_argument('--lisp-root', type=Path)
    parser.add_argument('--corpus', type=Path, required=True)
    parser.add_argument('--out', type=Path, required=True)
    parser.add_argument('--label', required=True)
    parser.add_argument('--baseline-run', type=Path)
    parser.add_argument('--limit', type=int, default=0)
    parser.add_argument('--only', help='comma-separated logical test paths')
    parser.add_argument('--from-strict-pair', type=Path,
                        help='derive the official per-file verdicts from a closed strict pair')
    parser.add_argument('--rejudge', type=Path,
                        help='re-judge an existing run directory from its raw per-file summaries')
    parser.add_argument('--timeout', type=float, default=SAFE_LIMIT)
    parser.add_argument('--materialize-fixtures', action='store_true',
                        help='build an identity-checked corpus overlay with the manifest '
                             'fixtures next to the loaded test file')
    parser.add_argument('--runner', type=Path,
                        default=REPO / 'tools/fixtures/stable-suite-runner.el')
    args = parser.parse_args(argv)

    suite = load_module('functional_suite', REPO / 'tools/stable-suite.py')
    batch = suite._load_batch()
    corpus = args.corpus.resolve()
    manifest_path = corpus / 'compatibility/stable-31.1/sources.json'
    manifest = suite.load_manifest(manifest_path, corpus)
    entries = batch.select_entrypoints(manifest)
    if args.only:
        wanted = {name.strip() for name in args.only.split(',') if name.strip()}
        known = {entry['path'] for entry in entries}
        missing = wanted - known
        if missing:
            parser.error('unknown logical test paths: ' + ', '.join(sorted(missing)))
        entries = [entry for entry in entries if entry['path'] in wanted]
    if args.limit:
        entries = entries[:args.limit]

    baseline = None
    baseline_manifest = None
    if args.baseline_run is not None:
        report_path = args.baseline_run / 'stable-suite-run.json'
        if not report_path.is_file():
            parser.error('baseline run has no stable-suite-run.json')
        baseline_manifest = read_json(report_path).get('manifest_sha256')
        baseline = baseline_index(args.baseline_run)
        if baseline_manifest != digest(manifest_path):
            print('baseline manifest differs: %s != %s' % (baseline_manifest, digest(manifest_path)),
                  file=sys.stderr)
            baseline = None

    if args.from_strict_pair is not None:
        pair = args.from_strict_pair.resolve()
        counts = {name: 0 for name in
                  ('same', 'resolved', 'port-worse', 'pending-condition-diff',
                   'baseline-unavailable', 'registration-mismatch', 'incomplete',
                   'lane-excluded')}
        rows = []
        for side in ('reference', 'candidate'):
            if not (pair / 'results' / side).is_dir():
                parser.error('strict pair lacks its %s results' % side)
        ref_index = baseline_index(pair)
        for path in sorted((pair / 'results' / 'candidate').glob('*.json')):
            if path.name.endswith('.environment.json'):
                continue
            value = read_json(path)
            if value.get('schema') != 'femacs-stable-file-batch-v1':
                continue
            logical = value['file']
            cases = {k: dict(status=(v or {}).get('status'), diagnostic=(v or {}).get('diagnostic', ''))
                     for k, v in (value.get('cases') or {}).items()}
            finished = finished_from_progress(value, pair)
            complete = (value.get('complete') is True and value.get('exit_code') == 0
                        and value.get('timed_out') is not True
                        and finished is not None and finished.get('status') == 'complete')
            base = ref_index.get(logical)
            if not complete:
                verdict = 'incomplete'
            elif base is None or not base['closed']:
                verdict = 'baseline-unavailable'
            elif set(base['cases']) != set(cases):
                verdict = 'registration-mismatch'
            else:
                per_case = {key: case_verdict(base['cases'][key], cases[key]) for key in cases}
                verdict = ('port-worse' if 'port-worse' in per_case.values() else
                           'pending-condition-diff' if 'pending-condition-diff' in per_case.values() else
                           'resolved' if 'resolved' in per_case.values() else 'same')
                rows.append(dict(file=logical, verdict=verdict,
                                 diff={k: v for k, v in per_case.items() if v != 'same'},
                                 exit_code=value.get('exit_code')))
                counts[verdict] += 1
                continue
            rows.append(dict(file=logical, verdict=verdict, exit_code=value.get('exit_code')))
            counts[verdict] += 1
        result = dict(schema='femacs-functional-corpus-v1', label=args.label,
                      source='closed-strict-pair', pair=str(pair), files=rows, counts=counts)
        (Path(args.out) / 'strict-verdicts.json').write_text(json.dumps(result, indent=1) + '\n')
        print(json.dumps(dict(files=len(rows), counts=counts)))
        unresolved = (counts['port-worse'] or counts['incomplete'] or counts['baseline-unavailable']
                      or counts['registration-mismatch'] or counts['pending-condition-diff'])
        return 0 if not unresolved else 1

    if args.rejudge is not None:
        run = args.rejudge.resolve()
        previous = read_json(run / 'summary.json')
        counts = {name: 0 for name in
                  ('same', 'resolved', 'port-worse', 'pending-condition-diff',
                   'baseline-unavailable', 'registration-mismatch', 'incomplete',
                   'lane-excluded')}
        rows = []
        for path in sorted((run / 'results').glob('*.summary.json')):
            row = read_json(path)
            if not (row.get('input_matches') and row.get('registered')):
                row['verdict'] = 'incomplete'
            elif baseline is None or row['file'] not in baseline:
                row['verdict'] = 'baseline-unavailable'
            else:
                old = baseline[row['file']]
                if not old['closed']:
                    row['verdict'] = 'baseline-unavailable'
                elif not old.get('lane_eligible', True):
                    row['verdict'] = 'lane-excluded'
                elif set(old['cases']) != set(row['cases']):
                    row['verdict'] = 'registration-mismatch'
                else:
                    per_case = {key: case_verdict(old['cases'][key], row['cases'][key])
                                for key in row['cases']}
                    if 'port-worse' in per_case.values():
                        verdict = 'port-worse'
                    elif 'pending-condition-diff' in per_case.values():
                        verdict = 'pending-condition-diff'
                    elif 'resolved' in per_case.values():
                        verdict = 'resolved'
                    else:
                        verdict = 'same'
                    row['verdict'] = verdict
                    row['cases_diff'] = {k: v for k, v in per_case.items() if v != 'same'}
            counts[row['verdict']] += 1
            rows.append(dict(file=row['file'], verdict=row['verdict'],
                             exit_code=row.get('exit_code'), timed_out=row.get('timed_out'),
                             registered=row.get('registered'),
                             input_matches=row.get('input_matches'),
                             cases=len(row.get('cases', {}))))
        result = dict(previous, counts=counts, files=rows, rejudged=True,
                      baseline=None if baseline is None else str(args.baseline_run.resolve()),
                      baseline_manifest_sha256=baseline_manifest)
        (run / 'summary-rejudged.json').write_text(json.dumps(result, indent=1) + '\n', encoding='utf-8')
        print(json.dumps(dict(files=len(rows), counts=counts)))
        unresolved = (counts['port-worse'] or counts['incomplete'] or counts['baseline-unavailable']
                      or counts['registration-mismatch'] or counts['pending-condition-diff'])
        return 0 if not unresolved else 1

    env = {'FEMACS_GC_OBSERVATION': 'off'}
    if args.lisp_root is not None:
        env['FEMACS_STABLE_PRODUCT_LISP'] = str(args.lisp_root.resolve())
    paths = suite._product_paths(args.program, ['--dump-file=' + str(args.dump.resolve())], env)
    library = paths['lisp_root']
    out = args.out.resolve()
    out.mkdir(parents=True, exist_ok=False)
    (out / 'results').mkdir()
    (out / 'logs').mkdir()
    overlay = None
    if args.materialize_fixtures:
        overlay, overlay_identity = materialize_corpus(suite, batch, corpus, manifest, entries, out)
        corpus = overlay
    base_env = {key: value for key, value in os.environ.items() if not key.startswith('FEMACS_')}
    base_env.update(LC_ALL='C', LANG='C', TERM='dumb')

    summary = dict(schema='femacs-functional-corpus-v1', label=args.label,
                   program=dict(path=str(paths['program']), sha256=digest(paths['program'])),
                   dump=dict(path=str(paths['dump']), sha256=digest(paths['dump'])),
                   library=str(library), manifest=dict(path=str(manifest_path), sha256=digest(manifest_path)),
                   corpus=dict(path=str(corpus), materialized=None if overlay is None
                               else str(overlay), identity=None if overlay is None
                               else overlay_identity),
                   baseline=None if baseline is None else str(args.baseline_run),
                   files=[], counts={})
    counts = {name: 0 for name in
              ('same', 'resolved', 'port-worse', 'pending-condition-diff', 'baseline-unavailable',
               'registration-mismatch', 'incomplete', 'lane-excluded')}
    started = time.time()
    for ordinal, entry in enumerate(entries):
        logical = entry['path']
        source = corpus / entry['source_path']
        safe = logical.replace('/', '-')
        # Per-run state must live outside the repository: original tests can
        # create build manifests or sockets that must never join this workspace.
        case_dir = Path(tempfile.mkdtemp(prefix='femacs-functional-'))
        (case_dir / 'home').mkdir()
        (case_dir / 'tmp').mkdir()
        progress = out / 'results' / (safe + '.events.jsonl')
        env_file = dict(base_env, HOME=str(case_dir / 'home'), TMPDIR=str(case_dir / 'tmp'),
                        EMACS_TEST_DIRECTORY=str(corpus / 'test'),
                        STABLE_SUITE_MODE='batch', STABLE_SUITE_FILE=str(source),
                        STABLE_SUITE_FILE_ID=logical, STABLE_SUITE_PROGRESS=str(progress),
                        STABLE_SUITE_RESULT=str(out / 'results' / (safe + '.json')),
                        STABLE_SUITE_SOURCE_CONTEXT=str(corpus),
                        STABLE_SUITE_PRODUCT_LISP=str(library),
                        FEMACS_GC_OBSERVATION='off')
        argv = [str(paths['program']), '--dump-file=' + str(paths['dump']), '--batch', '-Q',
                '-L', str(corpus / 'test'), '-l', str(args.runner), '-f', 'stable-suite-main']
        exit_code, timed_out = None, False
        with progress.open('xb') as stream, (out / 'logs' / (safe + '.log')).open('xb') as log:
            try:
                proc = subprocess.run(argv, env=env_file, cwd=case_dir, stdin=subprocess.DEVNULL,
                                      stdout=log, stderr=subprocess.STDOUT, timeout=args.timeout)
                exit_code = proc.returncode
            except subprocess.TimeoutExpired:
                timed_out = True
        ids, cases, finished = [], {}, None
        for line in progress.read_text(errors='replace').splitlines():
            if not line.strip():
                continue
            try:
                event = json.loads(line)
            except json.JSONDecodeError:
                continue
            if event.get('event') == 'loaded':
                ids = event.get('ids', [])
            elif event.get('event') == 'test-ended':
                if event.get('test_id') in cases:
                    ids = ids + ['duplicate:' + str(event.get('test_id'))]
                cases[event['test_id']] = dict(status=event.get('status'),
                                              diagnostic=event.get('diagnostic', ''))
            elif event.get('event') == 'finished':
                finished = event
        input_identity = dict(size=source.stat().st_size, sha256=digest(source),
                              mode=format(0o100000 | (source.stat().st_mode & 0o777), 'o'))
        expected = dict(size=entry['size'], sha256=entry['sha256'], mode=entry['mode'])
        registered = (bool(ids) and len(ids) == len(set(ids)) and set(ids) == set(cases)
                      and finished is not None and finished.get('status') == 'complete'
                      and finished.get('requested_count') == len(ids)
                      and finished.get('ended_count') == len(cases)
                      and exit_code == 0 and not timed_out)
        row = dict(file=logical, source_path=entry['source_path'],
                   input_identity=input_identity, expected_identity=expected,
                   input_matches=input_identity == expected,
                   exit_code=exit_code, timed_out=timed_out,
                   registered=registered, ids=len(ids), cases=cases,
                   finished=finished)
        if not (row['input_matches'] and registered):
            row['verdict'] = 'incomplete'
            counts['incomplete'] += 1
        elif baseline is None or logical not in baseline:
            row['verdict'] = 'baseline-unavailable'
            counts['baseline-unavailable'] += 1
        else:
            old = baseline[logical]
            if not old['closed'] or old['exit_code'] != 0 or old['finished_status'] != 'complete':
                row['verdict'] = 'baseline-unavailable'
                counts['baseline-unavailable'] += 1
            elif set(old['cases']) != set(cases):
                row['verdict'] = 'registration-mismatch'
                counts['registration-mismatch'] += 1
            else:
                per_case = {key: case_verdict(old['cases'][key], cases[key]) for key in cases}
                if 'port-worse' in per_case.values():
                    verdict = 'port-worse'
                elif 'pending-condition-diff' in per_case.values():
                    verdict = 'pending-condition-diff'
                elif 'resolved' in per_case.values():
                    verdict = 'resolved'
                else:
                    verdict = 'same'
                row['verdict'] = verdict
                row['cases_diff'] = {key: value for key, value in per_case.items() if value != 'same'}
                counts[verdict] += 1
        (out / 'results' / (safe + '.summary.json')).write_text(json.dumps(row) + '\n', encoding='utf-8')
        summary['files'].append(dict(file=logical, verdict=row['verdict'],
                                     exit_code=exit_code, timed_out=timed_out,
                                     registered=registered, input_matches=row['input_matches'],
                                     cases=len(cases)))
        summary['counts'] = counts
        summary['seconds'] = round(time.time() - started, 1)
        (out / 'summary.json').write_text(json.dumps(summary, indent=1) + '\n', encoding='utf-8')
        (out / 'progress.json').write_text(json.dumps(dict(
            completed=len(summary['files']), total=len(entries), counts=counts,
            latest=logical, seconds=summary['seconds'])) + '\n', encoding='utf-8')
        shutil.rmtree(case_dir, ignore_errors=True)
    print(json.dumps(dict(files=len(summary['files']), counts=counts,
                          seconds=summary['seconds'])))
    unresolved = (counts['port-worse'] or counts['incomplete'] or counts['baseline-unavailable']
                  or counts['registration-mismatch'] or counts['pending-condition-diff'])
    return 0 if not unresolved else 1


if __name__ == '__main__':
    raise SystemExit(main())
