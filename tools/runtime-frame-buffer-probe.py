"""Observe the same bounded terminal frame transitions on selected products.

This collects runtime facts for diagnosis. It neither certifies the original
corpus nor changes a library, original test, hook or expected result.

Default product admission uses an explicit products registry. Selected-product
mode admits closed runtime-product roots via runtime_product_profile and never
pretends those roots are a 61-product registry. Correct-behavior contract
collections still require an independent matching-epoch control; raw candidate
passes or same-failure pairs are not enough.
"""
import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import sys
import time

ROOT = Path(__file__).resolve().parent.parent


def _pairs(values):
    result = {}
    for key, value in values:
        if key in result:
            raise ValueError('duplicate JSON object key')
        result[key] = value
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    products = parser.add_mutually_exclusive_group(required=True)
    products.add_argument('--products', type=Path,
                          help='matrix/registry admission from an explicit products list')
    products.add_argument('--product', type=Path, action='append', default=None,
                          help='selected-product admission from one closed runtime-product root')
    parser.add_argument('--profile', action='append', default=[])
    parser.add_argument('--collector', choices=('young', 'major'),
                        help='required for selected-product collector / native-collector roots')
    parser.add_argument('--include-reference', action='store_true')
    parser.add_argument('--contract', action='store_true', help='verify the owned dframe behavior contract')
    binding = parser.add_mutually_exclusive_group()
    binding.add_argument('--reference-selection', type=Path)
    binding.add_argument('--reference-inputs', type=Path,
        help='bounded installed-reference diagnostics, never formal matrix selection')
    parser.add_argument('--dframe', type=Path, help='integration-only generated library; never product acceptance')
    parser.add_argument('--out', type=Path, required=True)
    args = parser.parse_args()
    selected_mode = args.product is not None
    if (args.contract and not (args.reference_selection or args.reference_inputs)) or (args.dframe and not args.contract):
        parser.error('contract collection requires the selected reference; --dframe is contract integration only')
    if selected_mode:
        if args.reference_inputs is not None:
            parser.error('selected-product contract/admission uses --reference-selection, not reference-inputs')
        if args.contract and args.reference_selection is None:
            parser.error('selected-product contract collection requires --reference-selection')
        if len(set(str(Path(os.path.abspath(path))) for path in args.product)) != len(args.product):
            parser.error('unique selected runtime-product roots required')
    elif args.collector is not None:
        parser.error('--collector is only valid with --product')
    if sys.platform != 'darwin':
        parser.error('this diagnostic uses the declared macOS script utility')
    spec = importlib.util.spec_from_file_location('frame_probe_stage', ROOT/'tools/runtime-stage.py')
    stage = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(stage)
    e = stage.execution
    accounting = importlib.util.spec_from_file_location('frame_probe_accounting',
                                                        ROOT/'tools/stable-classification.py')
    accounting_mod = importlib.util.module_from_spec(accounting)
    accounting.loader.exec_module(accounting_mod)
    suite = e.load_tool('stable-suite.py')
    reference_pins = {}
    selection_document = selection_path = selection_pin = None
    inputs_pins = {}
    if selected_mode:
        if args.collector is None and any(
                'collector' in json.loads((e.safe_path(path)/'runtime-binding.json').read_text(encoding='utf-8')).get('variant', '')
                for path in args.product):
            parser.error('selected collector products require --collector young|major')
        if args.reference_selection is None:
            parser.error('selected-product admission requires --reference-selection')
        selection_path = e.safe_path(args.reference_selection)
        selection_document = json.loads(selection_path.read_text(encoding='utf-8'), object_pairs_hook=_pairs)
        selection_pin = e.file_identity(selection_path)
        if (selection_document.get('schema') != 'femacs-reference-selection-v1'
                or not isinstance(selection_document.get('sha256'), str)
                or len(selection_document['sha256']) != 64
                or not isinstance(selection_document.get('product'), dict)
                or selection_document['product'].get('id') != 'installed-reference'):
            parser.error('selected-product requires an embedded installed-reference selection document')
        selected_products = []
        source_roots = set()
        for path in args.product:
            profile = accounting_mod.runtime_product_profile(
                e.safe_path(path), collector=args.collector or 'off')
            selected_products.append(profile)
            source_roots.add(profile['source_root'])
            inputs_pins[profile['inputs_path']] = e.file_identity(Path(profile['inputs_path']))
        if len(source_roots) != 1:
            parser.error('selected products must share one declared source_root')
        source = e.safe_path(Path(next(iter(source_roots))))
        if source != ROOT:
            parser.error('selected-product source_root must be this repository root')
        reference = dict(selection_document['product'])
        registry = {reference['id']: reference, **{p['id']: p for p in selected_products}}
        candidates = {p['id'] for p in selected_products}
        if set(args.profile) - candidates:
            parser.error('declared profiles must match selected runtime-product identities')
        selected = args.profile or list(candidates)
        if args.include_reference:
            selected = ['installed-reference', *selected]
    else:
        registry = {p['id']: p for p in e.load_products(args.products)}
        if args.reference_selection:
            if e.bounded_json(args.reference_selection).get('schema') != stage.reference_selection.SCHEMA:
                parser.error('formal reference selection schema required')
        if args.reference_inputs:
            try:
                _, reference_pins = stage.reference_selection.read_installation_inputs(
                    e, args.reference_inputs, registry.get('installed-reference'))
            except (OSError, ValueError, KeyError, TypeError) as error:
                parser.error(str(error))
        selected = args.profile or ['installed-reference', 'cocoa-native-normal', 'terminal-native-normal']
        available = {name for name, product in registry.items() if not product['test_hooks']}
        if (not 1 <= len(selected) <= len(available) or len(set(selected)) != len(selected)
                or any(p not in registry or registry[p]['test_hooks'] for p in selected)):
            parser.error('unique declared non-diagnostic products required')
        if args.include_reference and 'installed-reference' not in selected:
            selected = ['installed-reference', *selected]
    out = e.new_output(args.out)
    fixture = ROOT/'tools/fixtures'/('runtime-library-contracts.el' if args.contract
                                   else 'runtime-frame-buffer-observation.el')
    inputs = {**reference_pins, **inputs_pins, **{str(p): e.file_identity(p) for p in
              (Path(__file__).resolve(), fixture)}}
    if args.products is not None:
        inputs[str(args.products)] = e.file_identity(args.products)
    if args.reference_selection:
        inputs[str(args.reference_selection)] = e.file_identity(args.reference_selection)
    if args.reference_inputs:
        inputs[str(args.reference_inputs)] = e.file_identity(args.reference_inputs)
    if args.dframe:
        args.dframe = e.safe_path(args.dframe)
        if not args.dframe.is_file() or args.dframe.suffix not in ('.el', '.elc'):
            parser.error('explicit generated ordinary Lisp source or bytecode required')
        inputs[str(args.dframe)] = e.file_identity(args.dframe)
    before = {p: e.product_identity(registry[p], resources=True) for p in selected}
    rows, errors, checks = [], [], {}
    started = time.monotonic()
    try:
        for label in selected:
            product = registry[label]
            destination = out/(label+'.json')
            environment = dict(os.environ)
            environment.update(suite.case_environment('frame-'+label, out))
            environment.update(stage.stable_product_environment(product, True))
            environment.update(FE_FRAME_BUFFER_OBSERVATION=str(destination),
                               OMP_THREAD_LIMIT='1', TERM='xterm-256color')
            paths = suite._product_paths(product['program'], ['--dump-file='+product['dump']],
                stage.stable_product_environment(product, True))
            contract_library = args.dframe
            if args.contract and contract_library is None:
                contract_library = paths['lisp_root']/'dframe.elc'
                if not contract_library.exists():
                    contract_library = paths['lisp_root']/'dframe.el'
                e.safe_path(contract_library)
                inputs[str(contract_library)] = e.file_identity(contract_library)
            argv = ['/usr/bin/script', '-e', '-q', str(out/(label+'.terminal.txt')),
                    product['program'], '--dump-file='+product['dump'], '-nw', '-Q',
                    '--eval', '(setq native-comp-async-jobs-number 1)',
                    *(['-l', str(contract_library)] if args.contract else []),
                    '-l', str(fixture), '-f', ('runtime-library-contracts-run' if args.contract
                                          else 'runtime-frame-buffer-observation')]
            process = e.run_process(argv, out/(label+'-process'), environment, 30, cwd=ROOT)
            observation = e.bounded_json(destination) if destination.exists() else None
            complete = (process['complete'] and process['exit_code'] == 0
                and not process['remaining_group_terminated']
                and observation is not None and observation.get('collection_complete') is True
                and observation.get('noninteractive') is False
                and observation.get('window_system') is None
                and len(observation.get('rows', [])) == 2)
            rows.append(dict(profile=label, process=process, observation=observation,
                             stable_product_identity=suite._public_product_identity(paths),
                             collection_complete=complete))
            e.write_json(out/'partial.json', dict(cases=rows, diagnostic_only=True))
    except Exception as error:
        errors.append(type(error).__name__+': '+str(error)[:2048])
    finally:
        try:
            identities_unchanged = before == {
                p: e.product_identity(registry[p], resources=True) for p in selected}
            pins_unchanged = all(e.file_identity(Path(p)) == pin for p, pin in inputs.items())
            unchanged = identities_unchanged and pins_unchanged
        except Exception as error:
            unchanged = False
            identities_unchanged = pins_unchanged = False
            errors.append(type(error).__name__+': '+str(error)[:2048])
        def digest(value):
            return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(',', ':')).encode()).hexdigest()
        if selected_mode:
            def selected_receipts_valid():
                for product in (registry[name] for name in registry if name != 'installed-reference'):
                    accounting_mod.runtime_product_profile(
                        product['runtime_root'], collector=product['collector'])
                live = e.product_identity(registry['installed-reference'], resources=False)
                expected = selection_document['identity']
                return (live['program']['sha256'] == expected['program']['sha256']
                        and live['program']['bytes'] == expected['program']['bytes']
                        and live['dump']['sha256'] == expected['dump']['sha256']
                        and live['dump']['bytes'] == expected['dump']['bytes'])

            def reference_selection_unchanged():
                current = e.file_identity(selection_path)
                document = json.loads(selection_path.read_text(encoding='utf-8'),
                                      object_pairs_hook=_pairs)
                return (current == selection_pin and document == selection_document
                        and selected_receipts_valid() is True)

            def declared_source_root_unchanged():
                if any(e.file_identity(Path(path)) != pin for path, pin in inputs_pins.items()):
                    return False
                roots = {json.loads(Path(path).read_text(encoding='utf-8'),
                                    object_pairs_hook=_pairs)['source_root']
                         for path in inputs_pins}
                return roots == {str(ROOT)} and (ROOT/'test').is_dir()

            operations = {
                'selected_product_identities_unchanged': lambda: identities_unchanged is True,
                'selected_product_receipts_valid': selected_receipts_valid,
                'declared_source_root_unchanged': declared_source_root_unchanged,
                'reference_selection_unchanged': reference_selection_unchanged,
                'declared_input_pins_unchanged': lambda: pins_unchanged is True,
            }
        else:
            operations = {
                'declared_product_identities_unchanged': lambda: identities_unchanged is True,
                'declared_input_pins_unchanged': lambda: pins_unchanged is True,
            }
        for key, operation in operations.items():
            try:
                checks[key] = operation() is True
            except Exception as error:
                checks[key] = False
                errors.append(f'{key}: {type(error).__name__}: {error}')
        unchanged = unchanged and all(checks.values())
        complete = (not errors and unchanged and len(rows) == len(selected)
                    and all(row['collection_complete'] for row in rows))
        e.write_json(out/'results.json', dict(schema='femacs-runtime-frame-buffer-probe-v1',
            contract='dframe-create-buffer-v1' if args.contract else None,
            reference_selection_sha256=(selection_document['sha256'] if selected_mode and selection_document
                                        else (e.digest(args.reference_selection) if args.reference_selection else None)),
            reference_selection=(selection_document if selected_mode else None),
            reference_inputs_sha256=(e.digest(args.reference_inputs) if args.reference_inputs else None),
            reference_scope=('selected-product' if selected_mode
                             else 'bounded-installation-diagnostic' if args.reference_inputs
                             else 'formal-selection'),
            generated_override_used=args.dframe is not None,
            passed=complete, diagnostic_collection_complete=complete, cases=rows, errors=errors,
            checks=checks,
            selected_inputs_unchanged=unchanged, selected_identity_sha256=digest(before),
            selected_product_admission=selected_mode,
            full_matrix_admission=False if selected_mode else args.products is not None,
            input_pins=inputs, diagnostic_only=True, original_corpus_execution=False,
            formal_corpus_complete=False, compatibility_accepted=False,
            performance_accepted=False, goal_complete=False, seconds=time.monotonic()-started))
    return 0 if complete else 1


if __name__ == '__main__':
    raise SystemExit(main())
