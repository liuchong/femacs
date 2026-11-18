"""Observe the same bounded terminal frame transitions on selected products.

This collects runtime facts for diagnosis. It neither certifies the original
corpus nor changes a library, original test, hook or expected result.
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


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--products', type=Path, required=True)
    parser.add_argument('--profile', action='append', default=[])
    parser.add_argument('--contract', action='store_true', help='verify the owned dframe behavior contract')
    binding = parser.add_mutually_exclusive_group()
    binding.add_argument('--reference-selection', type=Path)
    binding.add_argument('--reference-inputs', type=Path,
        help='bounded installed-reference diagnostics, never formal matrix selection')
    parser.add_argument('--dframe', type=Path, help='integration-only generated library; never product acceptance')
    parser.add_argument('--out', type=Path, required=True)
    args = parser.parse_args()
    if (args.contract and not (args.reference_selection or args.reference_inputs)) or (args.dframe and not args.contract):
        parser.error('contract collection requires the selected reference; --dframe is contract integration only')
    if sys.platform != 'darwin':
        parser.error('this diagnostic uses the declared macOS script utility')
    spec = importlib.util.spec_from_file_location('frame_probe_stage', ROOT/'tools/runtime-stage.py')
    stage = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(stage)
    e = stage.execution
    suite = e.load_tool('stable-suite.py')
    registry = {p['id']:p for p in e.load_products(args.products)}
    reference_pins = {}
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
    out = e.new_output(args.out)
    fixture = ROOT/'tools/fixtures'/('runtime-library-contracts.el' if args.contract
                                   else 'runtime-frame-buffer-observation.el')
    inputs = {**reference_pins, **{str(p):e.file_identity(p) for p in
              (Path(__file__).resolve(), fixture, args.products)}}
    if args.reference_selection:
        inputs[str(args.reference_selection)] = e.file_identity(args.reference_selection)
    if args.reference_inputs:
        inputs[str(args.reference_inputs)] = e.file_identity(args.reference_inputs)
    if args.dframe:
        args.dframe = e.safe_path(args.dframe)
        if not args.dframe.is_file() or args.dframe.suffix not in ('.el', '.elc'):
            parser.error('explicit generated ordinary Lisp source or bytecode required')
        inputs[str(args.dframe)] = e.file_identity(args.dframe)
    before = {p:e.product_identity(registry[p],resources=True) for p in selected}
    rows, errors = [], []
    started = time.monotonic()
    try:
        for label in selected:
            product = registry[label]
            destination = out/(label+'.json')
            environment = dict(os.environ)
            environment.update(suite.case_environment('frame-'+label, out))
            environment.update(stage.stable_product_environment(product,True))
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
            argv = ['/usr/bin/script','-e','-q',str(out/(label+'.terminal.txt')),
                    product['program'],'--dump-file='+product['dump'],'-nw','-Q',
                    '--eval','(setq native-comp-async-jobs-number 1)',
                    *(['-l', str(contract_library)] if args.contract else []),
                    '-l',str(fixture),'-f',('runtime-library-contracts-run' if args.contract
                                          else 'runtime-frame-buffer-observation')]
            process = e.run_process(argv,out/(label+'-process'),environment,30,cwd=ROOT)
            observation = e.bounded_json(destination) if destination.exists() else None
            complete = (process['complete'] and process['exit_code'] == 0
                and not process['remaining_group_terminated']
                and observation is not None and observation.get('collection_complete') is True
                and observation.get('noninteractive') is False
                and observation.get('window_system') is None
                and len(observation.get('rows',[])) == 2)
            rows.append(dict(profile=label,process=process,observation=observation,
                             stable_product_identity=suite._public_product_identity(paths),
                             collection_complete=complete))
            e.write_json(out/'partial.json',dict(cases=rows,diagnostic_only=True))
    except Exception as error:
        errors.append(type(error).__name__+': '+str(error)[:2048])
    finally:
        try:
            unchanged = (before == {p:e.product_identity(registry[p],resources=True) for p in selected}
                and all(e.file_identity(Path(p)) == pin for p,pin in inputs.items()))
        except Exception as error:
            unchanged = False
            errors.append(type(error).__name__+': '+str(error)[:2048])
        def digest(value):
            return hashlib.sha256(json.dumps(value,sort_keys=True,separators=(',',':')).encode()).hexdigest()
        complete = (not errors and unchanged and len(rows) == len(selected)
                    and all(row['collection_complete'] for row in rows))
        e.write_json(out/'results.json',dict(schema='femacs-runtime-frame-buffer-probe-v1',
            contract='dframe-create-buffer-v1' if args.contract else None,
            reference_selection_sha256=(e.digest(args.reference_selection) if args.reference_selection else None),
            reference_inputs_sha256=(e.digest(args.reference_inputs) if args.reference_inputs else None),
            reference_scope='bounded-installation-diagnostic' if args.reference_inputs else 'formal-selection',
            generated_override_used=args.dframe is not None,
            passed=complete,diagnostic_collection_complete=complete,cases=rows,errors=errors,
            selected_inputs_unchanged=unchanged,selected_identity_sha256=digest(before),
            input_pins=inputs,diagnostic_only=True,original_corpus_execution=False,
            formal_corpus_complete=False,compatibility_accepted=False,
            performance_accepted=False,goal_complete=False,seconds=time.monotonic()-started))
    return 0 if complete else 1


if __name__ == '__main__':
    raise SystemExit(main())
