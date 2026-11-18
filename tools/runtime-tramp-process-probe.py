"""Collect same-product remote process/watch contracts without promoting raw failures.

The original two ERT cases run only as diagnostics. Their result is separate
from the authored ready-handshake signal control and formal corpus acceptance.
"""
import argparse
import importlib.util
import json
import os
from pathlib import Path
import shutil
import tempfile
import time
import traceback

ROOT = Path(__file__).resolve().parent.parent
ORIGINAL = 'test/lisp/net/tramp-tests.el'
WATCH_ORIGINAL = 'test/lisp/filenotify-tests.el'


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--build', type=Path)
    parser.add_argument('--reference-selection', type=Path)
    parser.add_argument('--reference-inputs', type=Path,
                        help='explicit receipt-bound installed-reference diagnostic; no matrix admission')
    parser.add_argument('--profile', action='append', default=[])
    parser.add_argument('--include-reference', action='store_true')
    parser.add_argument('--out', type=Path, required=True)
    args = parser.parse_args(argv)
    bounded_reference = args.reference_inputs is not None
    if bounded_reference:
        if args.build is not None or args.reference_selection is not None or args.profile or args.include_reference:
            parser.error('reference-inputs selects one reference diagnostic without matrix/profile options')
    elif args.build is None or args.reference_selection is None:
        parser.error('matrix controls require build and reference-selection')
    stage = load('tramp_probe_stage', ROOT/'tools/runtime-stage.py')
    e = stage.execution
    suite = e.load_tool('stable-suite.py')
    batch = suite._load_batch()
    retire = batch._retirement_module(suite)
    options = argparse.Namespace(build=args.build, reference_selection=args.reference_selection,
                                 execution_products=None)
    reference_document, reference_pins = None, {}
    if bounded_reference:
        reference_document = e.bounded_json(e.safe_path(args.reference_inputs))
        product = reference_document.get('product')
        reference_document, reference_pins = stage.reference_selection.read_installation_inputs(
            e, args.reference_inputs, product)
        products, selected = [product], ['installed-reference']
        original = products_path = build = None
        source = ROOT
    else:
        original, source, products_path, products, build = stage.read_action_build(options)
        candidates = {p['id']: p for p in products if p['kind'] != 'reference' and not p['test_hooks']}
        selected = args.profile or ['cocoa-native-normal']
        if len(set(selected)) != len(selected) or set(selected)-set(candidates):
            parser.error('unique admitted non-diagnostic candidates required')
        selected = (['installed-reference'] if args.include_reference else []) + selected
    registry = {p['id']: p for p in products}
    manifest = suite.load_manifest(source/'compatibility/stable-31.1/sources.json', source)
    test_entry = next(entry for entry in manifest['entries'] if entry['path'] == ORIGINAL)
    watch_entry = next(entry for entry in manifest['entries'] if entry['path'] == WATCH_ORIGINAL)
    test_source = source/test_entry['source_path']
    watch_source = source/watch_entry['source_path']
    if e.digest(test_source) != test_entry['sha256'] or e.digest(watch_source) != watch_entry['sha256']:
        parser.error('exact original Tramp and file-notify test sources required')
    original_identity = e.file_identity(test_source)
    watch_original_identity = e.file_identity(watch_source)
    declared_pins = {}
    if bounded_reference:
        declared_pins[str(source/'compatibility/stable-31.1/sources.json')] = e.file_identity(
            source/'compatibility/stable-31.1/sources.json')
        for entry in manifest['entries']:
            path = e.safe_path(source/entry['source_path'])
            identity = e.file_identity(path)
            if identity.get('sha256') != entry['sha256'] or identity.get('bytes') != entry['size']:
                parser.error('declared original corpus input changed: '+entry['path'])
            declared_pins[str(path)] = identity
    out = e.new_output(args.out)
    started = time.monotonic()
    fixture = ROOT/'tools/fixtures/runtime-tramp-process-contract.el'
    watch_fixture = ROOT/'tools/fixtures/runtime-remote-watch-contract.el'
    pins = {str(path): e.file_identity(path) for path in
            (Path(__file__).resolve(), fixture, watch_fixture, ROOT/'tools/runtime-stage.py',
             ROOT/'tools/runtime-execution.py', ROOT/'tools/runtime-reference.py',
             ROOT/'tools/runtime-owner.py', ROOT/'tools/stable-suite.py',
             ROOT/'tools/stable-suite-batch.py', ROOT/'tools/runtime-retire.py')}
    pins.update(reference_pins)
    pins.update(declared_pins)
    if bounded_reference:
        pins[str(e.safe_path(args.reference_inputs))] = e.file_identity(args.reference_inputs)
    before = {p['id']: e.product_identity(p, resources=True) for p in products}
    rows, checks, errors = [], {}, []
    cancelled_exit = None

    def record_interruption(error):
        nonlocal cancelled_exit
        code = 130 if isinstance(error, KeyboardInterrupt) else error.code
        cancelled_exit = code if type(code) is int and code > 0 else 1
        errors.append(f'collection cancelled: {type(error).__name__} ({cancelled_exit})')
    with stage.owner.ArtifactRun(ROOT, out) as artifacts:
        if not bounded_reference:
            stage.select_execution_inputs(artifacts, original, source, products_path, products, build, options)
        captured = out/'runtime-tramp-process-contract.el'
        captured_watch = out/'runtime-remote-watch-contract.el'
        shutil.copy2(fixture, captured)
        shutil.copy2(watch_fixture, captured_watch)
        captured_identity = e.file_identity(captured)
        captured_watch_identity = e.file_identity(captured_watch)
        if captured_identity != pins[str(fixture)] or captured_watch_identity != pins[str(watch_fixture)]:
            raise ValueError('captured authored fixture identity differs')
        stage.owner.capture_artifacts(artifacts.journal, {
            str(captured): captured_identity, str(captured_watch): captured_watch_identity})
        try:
            cache = e.safe_path(Path.home()/'AI/caches/.noindex', must_exist=False)
            cache.mkdir(parents=True, exist_ok=True)
            for profile in selected:
                container = Path(tempfile.mkdtemp(prefix='f', dir=cache))
                container_node = container.stat()
                row = dict(profile=profile, owned_container=str(container),
                           collection_complete=False, signal_contract_passed=False,
                           watch_contract_passed=False, state_retirement=dict(
                               status='retained', reason='owned case has not completed'))
                rows.append(row)
                env = stage.clean_environment()
                env.update(suite.case_environment(ORIGINAL+'|'+profile, container, compact=True))
                if env.get('FEMACS_STABLE_MOCK_BIN'):
                    env['PATH'] = f"{env['FEMACS_STABLE_MOCK_BIN']}:{env.get('PATH', os.defpath)}"
                case_root = Path(env['FEMACS_RUNTIME_CASE_ROOT'])
                row['owned_state_root'] = str(case_root)
                (case_root/'cwd').mkdir()
                root_node = case_root.stat()
                product = registry[profile]
                runtime_environment = stage.stable_product_environment(product, True)
                env.update(runtime_environment)
                destination = out/(profile+'.observation.json')
                env.update(FEMACS_TRAMP_CONTRACT_RESULT=str(destination),
                           FEMACS_TRAMP_ORIGINAL_TEST=str(test_source),
                           FEMACS_REMOTE_WATCH_ORIGINAL_TEST=str(watch_source),
                           FEMACS_REMOTE_WATCH_FIXTURE=str(captured_watch), OMP_THREAD_LIMIT='1')
                e.write_json(out/'partial.json', dict(cases=rows, diagnostic_only=True))
                paths = suite._product_paths(product['program'], ['--dump-file='+product['dump']],
                                             runtime_environment)
                process = e.run_process([product['program'], '--dump-file='+product['dump'],
                    '-Q', '--batch', '-l', captured, '-f', 'runtime-tramp-process-contract-run'],
                    out/(profile+'-process'), env, 180, cwd=case_root/'cwd')
                observation = e.bounded_json(destination) if destination.is_file() else None
                collection = (process['complete'] and process['exit_code'] == 0
                    and not process['remaining_group_terminated'] and observation is not None
                    and observation.get('schema') == 'femacs-tramp-process-contract-v1'
                    and observation.get('collection_complete') is True)
                row.update(process=process, observation=observation,
                    observation_path=str(destination),
                    observation_identity=e.file_identity(destination) if destination.exists() else None,
                    runtime_environment=runtime_environment,
                    mock_transport=dict(profile=suite.MOCK_SHELL_PROFILE,
                                        content_sha256=suite.MOCK_SHELL_CONTENT_SHA256),
                    stable_product_identity=suite._public_product_identity(paths),
                    collection_complete=collection,
                    signal_contract_passed=collection and observation['signal_contract_passed'] is True,
                    watch_contract_passed=collection and observation.get('watch_contract', {}).get('correct_behavior') is True,
                    owned_state_root=str(case_root), owned_container=str(container))
                evidence = out/(profile+'.closed-process.json')
                e.write_json(evidence, row)
                try:
                    row['state_retirement'] = retire.retire(case_root, batch.STATE_NAMES, evidence, process,
                        empty_parents=((case_root, root_node), (container, container_node)),
                        record_occupancy=True)
                except (OSError, ValueError) as error:
                    row['state_retirement'] = dict(status='retained', error=f'{type(error).__name__}: {error}')
                    errors.append(f'{profile} state retirement: {type(error).__name__}: {error}')
                e.write_json(out/'partial.json', dict(cases=rows, diagnostic_only=True))
        except (KeyboardInterrupt, SystemExit) as error:
            record_interruption(error)
            if rows and rows[-1]['collection_complete'] is not True:
                row = rows[-1]
                process_path = out/(row['profile']+'-process')/'process.json'
                if process_path.is_file():
                    row['process'] = e.bounded_json(process_path)
                row['state_retirement'] = dict(status='retained',
                    reason='collection cancelled; owned child group reaped before guard release')
        except Exception as error:
            errors.append(f'{type(error).__name__}: {error}')
            e.write_json(out/'failed-child.json', dict(error=str(error), traceback=traceback.format_exc()[-8192:]))
        finally:
            operations = {
                'all61_product_identities_unchanged': lambda: before == {
                    p['id']: e.product_identity(p, resources=True) for p in products},
                'all61_complete_receipts_valid': lambda: len(e.load_products(products_path)) == 61,
                'source_export_unchanged': lambda: e.verify_source_export(stage.build_source_owner(original, build),
                    build['source_export']) == source,
                'reference_selection_unchanged': lambda: stage.verify_reference_selection(options, products) is None,
                'driver_and_fixture_sources_unchanged': lambda: all(e.file_identity(Path(path)) == pin
                    for path, pin in pins.items()),
                'captured_fixture_unchanged': lambda: e.file_identity(captured) == captured_identity
                    and e.file_identity(captured_watch) == captured_watch_identity,
                'original_test_unchanged': lambda: e.digest(test_source) == test_entry['sha256']
                    and e.digest(watch_source) == watch_entry['sha256'],
            }
            if bounded_reference:
                operations = {
                    'selected_reference_identity_unchanged': lambda: before == {
                        p['id']: e.product_identity(p, resources=True) for p in products},
                    'reference_installation_inputs_unchanged': lambda:
                        stage.reference_selection.read_installation_inputs(
                            e, args.reference_inputs, products[0])[0] == reference_document,
                    'declared_original_inputs_unchanged': lambda: all(
                        e.file_identity(Path(path)) == pin for path, pin in declared_pins.items()),
                    **{key:operations[key] for key in ('driver_and_fixture_sources_unchanged',
                                                     'captured_fixture_unchanged', 'original_test_unchanged')},
                }
            for key, operation in operations.items():
                if cancelled_exit is not None:
                    checks[key] = False
                    continue
                try:
                    checks[key] = operation() is True
                except (KeyboardInterrupt, SystemExit) as error:
                    record_interruption(error)
                    checks[key] = False
                except Exception as error:
                    checks[key] = False
                    errors.append(f'{key}: {type(error).__name__}: {error}')
            retention = dict(mode='cancelled', all_original_files_verified=False,
                             duplicate_content_archive_created=False)
            if cancelled_exit is None:
                try:
                    retention = artifacts.retain_current_files()
                except (KeyboardInterrupt, SystemExit) as error:
                    record_interruption(error)
                except Exception as error:
                    errors.append(f'artifact retention: {type(error).__name__}: {error}')
                    retention['mode'] = 'verification-failed'
            complete = (cancelled_exit is None and not errors and all(checks.values()) and len(rows) == len(selected)
                        and all(row['collection_complete'] for row in rows))
            e.write_json(out/'results.json', dict(schema='femacs-runtime-tramp-process-probe-v1',
                passed=complete, diagnostic_collection_complete=complete,
                signal_contract_passed=complete and all(row['signal_contract_passed'] for row in rows),
                watch_contract_passed=complete and all(row['watch_contract_passed'] for row in rows),
                requested_profiles=selected,
                requested_contracts=['ready-remote-signal-delivery-v1', 'remote-watch-lifecycle-v1'],
                cases=rows, checks=checks, errors=errors,
                build_receipt=str(original) if original is not None else None,
                full_matrix_admission=not bounded_reference,
                scope='installed-reference-diagnostic' if bounded_reference else 'matrix-controls',
                cancelled=cancelled_exit is not None, cancellation_exit_code=cancelled_exit,
                postflight_incomplete=cancelled_exit is not None,
                reference_selection=None if bounded_reference else options._reference_selection_summary,
                reference_diagnostic_input=dict(path=str(args.reference_inputs),
                    epoch=reference_document['epoch'], upstream=reference_document['upstream'])
                    if bounded_reference else None,
                original_test=dict(path=str(test_source), identity=original_identity),
                watch_original_test=dict(path=str(watch_source), identity=watch_original_identity),
                captured_fixtures={key:dict(path=str(path), identity=identity) for key, path, identity in (
                    ('signal', captured, captured_identity), ('watch', captured_watch, captured_watch_identity))},
                input_pins=pins, original_formal_results_modified=False,
                actual_editor_execution=any(row.get('process', {}).get('pid') for row in rows),
                formal_corpus_complete=False,
                compatibility_accepted=False, performance_accepted=False, goal_complete=False,
                seconds=time.monotonic()-started, artifact_retention=retention))
    return cancelled_exit if cancelled_exit is not None else 0 if complete else 1


if __name__ == '__main__':
    raise SystemExit(main())
