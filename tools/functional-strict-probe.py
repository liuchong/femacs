#!/usr/bin/env python3
"""Strict-lane probe for files the upstream-level lane cannot reproduce.

Some original files need per-file resources, module or private source-context
layouts, or tests that write inside their own tree.  Those files keep the strict
per-file fixture lane; this probe runs exactly them for one product and compares
the result with the already closed strict reference baseline.
"""
from __future__ import annotations

import argparse, json, os, sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent


def load(name, path):
    import importlib.util
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


def read_json(path):
    return json.loads(Path(path).read_text(encoding='utf-8'))


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--program', type=Path, required=True)
    parser.add_argument('--dump', type=Path, required=True)
    parser.add_argument('--corpus', type=Path, required=True)
    parser.add_argument('--out', type=Path, required=True)
    parser.add_argument('--label', required=True)
    parser.add_argument('--baseline-run', type=Path, required=True)
    parser.add_argument('--files', type=Path, required=True,
                        help='newline-separated logical test paths')
    parser.add_argument('--timeout', type=float, default=300)
    args = parser.parse_args(argv)

    suite = load('probe_suite', REPO / 'tools/stable-suite.py')
    batch = suite._load_batch()
    corpus = args.corpus.resolve()
    manifest = suite.load_manifest(corpus / 'compatibility/stable-31.1/sources.json', corpus)
    entries = {entry['path']: entry for entry in batch.select_entrypoints(manifest)}
    files = [line.strip() for line in args.files.read_text().splitlines() if line.strip()]
    if not files or any(name not in entries for name in files):
        parser.error('strict probe needs known original entry points')
    env = {'FEMACS_GC_OBSERVATION': 'off'}
    paths = suite._product_paths(args.program, ['--dump-file=' + str(args.dump.resolve())], env)
    paths['test_environment'] = {}
    # The installed reference application keeps its Lisp library outside the
    # bundle (P-464: `/Applications/Emacs.app/Contents/Resources/lisp` does not
    # exist in this layout), so its product paths need the verified library
    # root the reference selection receipt records.  `FEMACS_REFERENCE_LISP`
    # supplies it for the reference side only; the candidate keeps its own
    # build-tree root.
    reference_env = {'FEMACS_GC_OBSERVATION': 'off'}
    if os.environ.get('FEMACS_REFERENCE_LISP'):
        reference_env['FEMACS_STABLE_PRODUCT_LISP'] = os.environ['FEMACS_REFERENCE_LISP']
    baseline_root = args.baseline_run.resolve()
    context = baseline_root / 'source-context.json'
    if not context.is_file():
        parser.error('baseline has no source-context receipt')
    out = args.out.resolve()
    out.mkdir(parents=True, exist_ok=False)
    adjudicated = {}
    for logical in files:
        paths['test_context'] = baseline_root / 'source-contexts' / logical.replace('/', '-')
        pair = {}
        for label, product in (('reference', suite._product_paths(
                '/Applications/Emacs.app/Contents/MacOS/Emacs',
                ['--dump-file=/opt/homebrew/Cellar/emacs-plus@32/32.0.50/libexec/emacs/32.0.50/'
                 'aarch64-apple-darwin27.2.0/emacs-9f9536b204dba77ae4e3ec8713908975e621804233f819dc8e41aa3f2220b37c.pdmp'],
                reference_env)), ('candidate', paths)):
            product = dict(product, test_environment={})
            product['test_context'] = baseline_root / 'test'
            pair[label] = batch._run_file(suite, out, product, REPO / 'tools/fixtures/stable-suite-runner.el',
                                          corpus / 'test', logical, args.timeout, label,
                                          display='batch', fixture_entries=batch._fixture_entries(manifest, logical))
        want = None
        baseline_file = baseline_root / 'results' / 'reference'
        for path in baseline_file.glob('*.json'):
            if path.name.endswith('.environment.json'):
                continue
            value = read_json(path)
            if value.get('schema') == 'femacs-stable-file-batch-v1' and value.get('file') == logical:
                want = value
                break
        if want is None:
            adjudicated[logical] = dict(verdict='baseline-unavailable')
            continue
        old = {k: (v.get('status'), v.get('diagnostic', '')) for k, v in want.get('cases', {}).items()}
        new = {k: (v.get('status'), v.get('diagnostic', '')) for k, v in pair['candidate']['cases'].items()}
        worse, pending = [], []
        for key in set(old) | set(new):
            a, b = old.get(key), new.get(key)
            if a == b:
                continue
            if a and a[0] == 'passed' and (not b or b[0] != 'passed'):
                worse.append(key)
            elif a and b and a[0] == b[0] == 'failed' and a[1] != b[1]:
                pending.append(key)
            elif a and b and a[0] == 'failed' and b[0] == 'skipped':
                worse.append(key)
        complete = (pair['candidate']['complete'] is True and pair['candidate']['exit_code'] == 0
                    and pair['candidate']['timed_out'] is False)
        verdict = ('incomplete' if not complete else 'port-worse' if worse
                   else 'pending-condition-diff' if pending else 'same')
        adjudicated[logical] = dict(verdict=verdict, worse=worse, pending=pending,
                                    complete=complete,
                                    exit_code=pair['candidate']['exit_code'])
    counts = {}
    for row in adjudicated.values():
        counts[row['verdict']] = counts.get(row['verdict'], 0) + 1
    (out / 'adjudication.json').write_text(json.dumps(dict(
        schema='femacs-functional-strict-probe-v1', label=args.label,
        baseline=str(baseline_root), files=adjudicated, counts=counts), indent=1) + '\n')
    print(json.dumps(dict(files=len(adjudicated), counts=counts)))
    return 0 if set(counts) <= {'same'} else 1


if __name__ == '__main__':
    raise SystemExit(main())
