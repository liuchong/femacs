#!/usr/bin/env python3
"""Strict-lane probe for files the upstream-level lane cannot reproduce.

Some original files need per-file resources, module or private source-context
layouts, or tests that write inside their own tree.  Those files keep the strict
per-file fixture lane; this probe runs exactly them for one product and compares
the result with the already closed strict reference baseline.
"""
from __future__ import annotations

import argparse, hashlib, json, os, sys
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
    parser.add_argument('--reference-selection', type=Path, required=True,
                        help='closed reference-selection.json bound to its matrix build and installed reference')
    args = parser.parse_args(argv)

    # The reference product comes from the closed selection document.  The
    # full runtime-reference admission also re-verifies the matrix source
    # export, which is retired for every surviving build root; the checks below
    # are the ones its receipt files still allow: document digests, the pinned
    # reference row, the live product identity and the closed baseline receipt.
    stage = load('probe_stage', REPO / 'tools/runtime-stage.py')
    execution = stage.execution
    selection_path = execution.safe_path(args.reference_selection)
    declared = execution.bounded_json(selection_path, 64 * 1024 * 1024)
    if declared.get('schema') != 'femacs-runtime-reference-selection-v1':
        parser.error('unexpected reference selection schema')
    for key, target in (('build_result_sha256', 'build_root/results.json'),
                        ('original_products_sha256', 'original_products_path'),
                        ('selected_products_sha256', 'selected_products_path')):
        path = (execution.safe_path(Path(declared['build_root']) / 'results.json')
                if key == 'build_result_sha256' else execution.safe_path(declared[target]))
        if declared.get(key) != execution.digest(path):
            parser.error('reference selection {0} does not match {1}'.format(key, target))
    selected_products_path = execution.safe_path(declared['selected_products_path'])
    selected_document = execution.bounded_json(selected_products_path, 64 * 1024 * 1024)
    rows = selected_document.get('products') or []
    rows_reference = [row for row in rows if row.get('kind') == 'reference']
    if (len(rows_reference) != 1 or not rows or rows_reference[0] != rows[0]
            or rows_reference[0].get('id') != 'installed-reference'
            or set((rows_reference[0].get('resource_roots') or {})) != {'lisp', 'data'}):
        parser.error('selected products need exactly one leading installed-reference row')
    # The 60 candidate rows resolve inside the retired matrix source export, so
    # they cannot be loaded here; the reference row is absolute and is the one
    # this probe runs.
    live = execution.product_identity(rows[0], resources=True)
    if declared.get('selected_reference', {}).get('identity') != live:
        parser.error('selected reference identity differs from the live product')
    for key in ('candidate_and_reference_libraries_byte_identical', 'existing_components_reverted',
                'new_waivers_added', 'reference_semantics_accepted', 'compatibility_accepted',
                'performance_accepted', 'goal_complete'):
        if declared.get(key) is not False:
            parser.error('reference selection cannot certify runtime acceptance: ' + key)
    reference = rows[0]
    reference_program = execution.safe_path(reference['program'])
    reference_dump = execution.safe_path(reference['dump'])
    reference_lisp = execution.safe_path(reference['resource_roots']['lisp'])
    # An explicit Lisp root may only confirm the bound resource, never replace it.
    override = os.environ.get('FEMACS_REFERENCE_LISP')
    if override and Path(override).resolve() != reference_lisp:
        parser.error('FEMACS_REFERENCE_LISP would replace the bound reference resource root')

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
    # The installed reference keeps its Lisp library outside the application
    # bundle (P-464), so its product paths take the verified library root from
    # the selection document.  `FEMACS_REFERENCE_LISP` stays an explicit
    # override for the reference side only; the candidate keeps its own
    # build-tree root.
    reference_env = {'FEMACS_GC_OBSERVATION': 'off',
                     'FEMACS_STABLE_PRODUCT_LISP': str(reference_lisp)}
    baseline_root = args.baseline_run.resolve()
    context = baseline_root / 'source-context.json'
    if not context.is_file():
        parser.error('baseline has no source-context receipt')
    contexts = {side: execution.safe_path(row['root'])
                for side, row in read_json(context)['contexts'].items()}
    if set(contexts) != {'candidate', 'reference'}:
        parser.error('baseline source contexts must name the candidate and reference sides')
    # The closed baseline records the reference program, dump and library
    # identity; the admitted selection must reproduce it exactly.
    inputs_receipt = baseline_root / 'stable-suite-inputs.json'
    if not inputs_receipt.is_file():
        parser.error('baseline has no stable-suite input receipt')
    recorded = read_json(inputs_receipt)['products']['reference']
    observed = suite._public_product_identity(
        suite._product_paths(reference_program, ['--dump-file=' + str(reference_dump.resolve())],
                             reference_env))
    for name in ('program', 'dump', 'lisp'):
        if observed.get(name) != recorded.get(name):
            parser.error('admitted reference {0} identity differs from the closed baseline receipt'
                         .format(name))
    # The strict lane copies each file and its fixtures beside itself and keeps
    # its evidence in the same root, so OUT is both the fixture base and the
    # evidence directory.  The corpus keeps the 31.1 bodies under
    # compatibility/stable-31.1, so the base is built with the manifest path as
    # destination and source_path as origin, every byte checked.
    out = args.out.resolve()
    if out.exists():
        if not out.is_dir() or any(out.iterdir()):
            parser.error('output must be absent or empty')
    else:
        out.mkdir(parents=True)
    fixture_records = []
    for logical in files:
        for entry in batch._fixture_entries(manifest, logical):
            destination = out / entry['path']
            if destination.exists():
                continue
            source = corpus / entry['source_path']
            data = source.read_bytes()
            if len(data) != entry['size'] or hashlib.sha256(data).hexdigest() != entry['sha256']:
                parser.error('corpus fixture identity differs: ' + str(source))
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_bytes(data)
            destination.chmod(int(entry['mode'], 8) & 0o777)
            expected = dict(kind='file', bytes=entry['size'],
                            mode=int(entry['mode'], 8) & 0o777, sha256=entry['sha256'])
            if batch._fixture_node(suite, destination) != expected:
                parser.error('materialized fixture identity differs: ' + str(destination))
            fixture_records.append(dict(path=entry['path'], source=str(source),
                                        bytes=entry['size'], sha256=entry['sha256']))
    overlay_identity = dict(schema='femacs-strict-fixture-overlay-v1', root=str(out),
                            manifest_sha256=hashlib.sha256(
                                (corpus / 'compatibility/stable-31.1/sources.json').read_bytes()).hexdigest(),
                            files=fixture_records)
    adjudicated = {}
    for logical in files:
        pair = {}
        for label, product in (('reference', suite._product_paths(
                reference_program, ['--dump-file=' + str(reference_dump.resolve())],
                reference_env)), ('candidate', paths)):
            product = dict(product, test_environment={})
            product['test_context'] = contexts[label]
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
        baseline=str(baseline_root), selection=str(selection_path),
        selection_sha256=execution.digest(selection_path),
        reference_identity=live, fixtures=overlay_identity,
        files=adjudicated, counts=counts), indent=1) + '\n')
    print(json.dumps(dict(files=len(adjudicated), counts=counts)))
    return 0 if set(counts) <= {'same'} else 1


if __name__ == '__main__':
    raise SystemExit(main())
