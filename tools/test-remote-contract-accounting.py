"""Integrate fresh real controls with three exact closed original case pairs.

This invokes the production source/event/control gates on original evidence.
It does not rewrite the formal run or certify the other cases/profiles.
"""
import argparse
import importlib.util
import json
from pathlib import Path
import time

ROOT = Path(__file__).resolve().parent.parent
TARGETS = {('test/lisp/net/tramp-tests.el','tramp-test31-signal-process'),
           ('test/lisp/filenotify-tests.el','file-notify-test06-dir-validity-remote'),
           ('test/lisp/filenotify-tests.el','file-notify-test12-unmount-remote')}


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def prepare(args):
    stage = load('remote_accounting_stage', ROOT/'tools/runtime-stage.py')
    e = stage.execution
    accounting = load('remote_accounting_subject', ROOT/'tools/stable-classification.py')
    options = argparse.Namespace(build=args.build, reference_selection=args.reference_selection, execution_products=None)
    original, source, _products_path, products, _build = stage.read_action_build(options)
    selection = options._reference_selection_summary
    raw = e.bounded_json(args.raw_stage)
    if not (raw.get('execution_complete') is True and raw.get('product_identity_unchanged') is True
            and raw.get('complete_receipts_valid') is True and raw.get('reference_selection') == selection
            and raw.get('build_receipt') == str(original)):
        raise ValueError('closed matching-product/epoch original stable stage required')
    pair = next(row for row in raw['cases'] if row['id'] == 'stable-'+args.profile)
    profile = next(p for p in products if p['id'] == args.profile and not p['test_hooks'])
    root = e.safe_path(pair['receipt']).parent
    runner = Path(raw['stable_runner_selection']['captured_runner']['path'])
    suite = e.load_tool('stable-suite.py')
    receipt = suite.read_completion(root, source/'compatibility/stable-31.1/sources.json',
        products[0]['program'], profile['program'], ['--dump-file='+products[0]['dump']],
        ['--dump-file='+profile['dump']], stage.stable_product_environment(products[0], True),
        stage.stable_product_environment(profile, True), runner=runner)
    if (pair['receipt_sha256'] != receipt['detailed']['sha256']
            or pair['completion_sha256'] != e.digest(root/'stable-suite-completion.json')):
        raise ValueError('original stage/completion identity mismatch')
    files, cases = {}, {}
    for kind, row in accounting.detailed_rows(root/'stable-suite-run.json'):
        if kind == 'files' and row['file'] in {name for name,_ in TARGETS}:
            files[row['file']] = row
        elif kind == 'cases' and (row['file'],row['id']) in TARGETS:
            cases[(row['file'],row['id'])] = row
    rules = {}
    for kind, row in accounting.detailed_rows(args.approved_rules, stream_arrays={'rules'}):
        if kind == 'rules' and row.get('profile') == args.profile and (row['file'],row['id']) in TARGETS:
            rule = dict(row, source_evidence=[dict(pin, path=str(ROOT/pin['path']))
                                            for pin in row['source_evidence']])
            rules[(row['file'],row['id'])] = rule
    if set(cases) != TARGETS or set(rules) != TARGETS:
        raise ValueError('three exact registered pairs and reviewed definitions required')
    manifest = suite.load_manifest(source/'compatibility/stable-31.1/sources.json', source)
    entries = {entry['path']:entry for entry in manifest['entries']}
    collections, pins = accounting._remote_collections([args.remote_control])
    cache = {}
    controls = accounting._select_remote_controls(collections, args.profile, cache)
    for key, rule in rules.items():
        case = cases[key]
        rule['observations'] = {label:accounting.observation_signature(case[label]) for label in ('reference','candidate')}
        if rule['category'] == 'upstream-invalid-test':
            rule['invalid_gate'] = dict(rule['invalid_gate'], controls=controls,
                conditions={label:accounting._original_case_event(root,suite,case,label).get('condition')
                            for label in ('reference','candidate') if case[label]['status'] == 'failed'})
        else:
            rule['optional_gate'] = dict(accounting._capture_optional_gate(rule['optional_gate'],root,suite,case),
                                         controls=controls)
    for path in (args.approved_rules, args.raw_stage, root/'stable-suite-run.json', root/'stable-suite-completion.json',
                 Path(__file__).resolve(), ROOT/'tools/stable-classification.py'):
        pins[str(path)] = accounting.bound_file(path)
    return stage, accounting, suite, receipt, selection, profile, root, files, cases, entries, rules, cache, pins


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--build',type=Path,required=True)
    parser.add_argument('--reference-selection',type=Path,required=True)
    parser.add_argument('--raw-stage',type=Path,required=True)
    parser.add_argument('--approved-rules',type=Path,required=True)
    parser.add_argument('--remote-control',type=Path,nargs=2,required=True)
    parser.add_argument('--profile',default='cocoa-native-normal')
    parser.add_argument('--out',type=Path,required=True)
    args = parser.parse_args()
    started = time.monotonic()
    stage = load('remote_accounting_output_stage', ROOT/'tools/runtime-stage.py')
    out = stage.execution.new_output(args.out)
    rows, pins, error, counts = [], {}, None, {}
    try:
        stage, accounting, suite, receipt, selection, profile, root, files, cases, entries, rules, cache, pins = prepare(args)
        counts = {key:receipt[key] for key in ('case_count','passed_count','failure_count','corpus_complete')}
        for key in sorted(TARGETS):
            begin = time.monotonic()
            proofs = accounting._accept_rule(rules[key],cases[key],files[key[0]],entries[key[0]],
                root,suite,selection,profile,receipt,cache)
            pins.update(proofs)
            rows.append(dict(file=key[0],id=key[1],passed=True,category=rules[key]['category'],
                raw_statuses={label:cases[key][label]['status'] for label in ('reference','candidate')},
                proof_inputs=len(proofs),seconds=time.monotonic()-begin))
        for path, pin in pins.items():
            accounting.bound_file(path,pin,512*1024*1024)
    except Exception as failure:
        error = f'{type(failure).__name__}: {failure}'
        raise
    finally:
        stage.execution.write_json(out/'results.json',dict(schema='femacs-remote-contract-accounting-integration-v1',
            passed=error is None and len(rows)==3,cases=rows,error=error,seconds=time.monotonic()-started,
            raw_counts=counts,input_pins=pins,actual_editor_execution=False,
            consumes_actual_editor_controls=True,raw_results_modified=False,
            full_formal_classification_applied=False,formal_corpus_complete=False,
            compatibility_accepted=False,goal_complete=False))


if __name__ == '__main__':
    main()
