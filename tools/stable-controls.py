"""Execute unchanged current-version controls, separate from the formal corpus.

This is not a classification rule generator or a waiver. Failing controls stay
failed. A version-evolution rule can consume only an actually passing matching
file/case result and the separately bound original current-version input.

Default admission remains the complete product matrix. Selected-product mode
admits only the declared runtime-product roots and never pretends those roots
are a 61-product registry.
"""
from pathlib import Path
import argparse
import importlib.util
import json
import os
import sys
import time
import traceback

R = Path(__file__).resolve().parent.parent
FILES = ('test/lisp/erc/erc-fill-tests.el', 'test/lisp/erc/erc-tests.el',
    'test/lisp/eshell/esh-util-tests.el', 'test/lisp/filenotify-tests.el',
    'test/lisp/international/textsec-tests.el', 'test/lisp/international/ucs-normalize-tests.el',
    'test/lisp/minibuffer-tests.el', 'test/src/lread-tests.el',
    'test/lisp/progmodes/flymake-tests.el',
    'test/lisp/use-package/use-package-tests.el', 'test/lisp/visual-wrap-tests.el',
    'test/lisp/net/tramp-tests.el')


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def _pairs(values):
    result = {}
    for key, value in values:
        if key in result:
            raise ValueError('duplicate JSON object key')
        result[key] = value
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    scope = parser.add_mutually_exclusive_group(required=True)
    scope.add_argument('--build', type=Path,
        help='full-matrix admission from a completed 61-product build receipt')
    scope.add_argument('--product', type=Path, action='append', default=None,
        help='selected-product admission from one closed runtime-product root; repeatable')
    parser.add_argument('--reference-selection', type=Path, required=True)
    parser.add_argument('--collector', choices=('young', 'major'),
        help='required for selected-product collector / native-collector roots')
    parser.add_argument('--profile', action='append', default=[])
    parser.add_argument('--file', action='append', default=[])
    parser.add_argument('--test-id', action='append', default=[],
        help='declarative ERT member scope: run only these registered IDs (plus no extras); '
             'records requested/registered scope; uses existing exclude+member selection on an '
             'explicit batch|terminal|cocoa display (not a silent rewrite, not a shortened timeout)')
    parser.add_argument('--corpus', choices=('current', 'stable'), default='current')
    parser.add_argument('--display', choices=('original', 'batch', 'terminal', 'cocoa'), default='original')
    parser.add_argument('--include-reference', action='store_true')
    erc_mode = parser.add_mutually_exclusive_group()
    erc_mode.add_argument('--erc-graphical', action='store_true')
    erc_mode.add_argument('--erc-graphical-all', action='store_true',
        help='exercise the original graphical-all coordinator without setting its child-only flag')
    parser.add_argument('--timeout', type=float, default=300)
    parser.add_argument('--out', type=Path, required=True)
    args = parser.parse_args()
    if (not 0 < args.timeout <= 600 or len(set(args.profile)) != len(args.profile)
            or len(set(args.file)) != len(args.file)
            or len(set(args.test_id)) != len(args.test_id)):
        parser.error('finite control timeout and unique declared scopes required')
    if args.test_id and any(not isinstance(name, str) or not name or ',' in name for name in args.test_id):
        parser.error('exact non-empty ERT test identities required')
    if args.test_id and not args.file:
        parser.error('--test-id requires explicit --file so requested scope stays file-local')
    if args.test_id and args.display not in ('batch', 'terminal', 'cocoa'):
        parser.error('--test-id requires explicit --display batch|terminal|cocoa; original is rejected')
    selected_mode = args.product is not None
    if selected_mode and len(set(str(Path(os.path.abspath(path))) for path in args.product)) != len(args.product):
        parser.error('unique selected runtime-product roots required')
    stage = load('version_control_stage', R/'tools/runtime-stage.py')
    e = stage.execution
    accounting = load('version_control_accounting', R/'tools/stable-classification.py')
    selection_path = e.safe_path(args.reference_selection)
    selection_document = json.loads(selection_path.read_text(encoding='utf-8'), object_pairs_hook=_pairs)
    selection_pin = e.file_identity(selection_path)
    if selection_document.get('schema') != 'femacs-reference-selection-v1':
        parser.error('reference-selection must be a femacs-reference-selection-v1 document')
    if not isinstance(selection_document.get('epoch'), str) or not isinstance(selection_document.get('upstream'), dict):
        parser.error('reference-selection must bind epoch and upstream identity')
    if not isinstance(selection_document.get('sha256'), str) or len(selection_document['sha256']) != 64:
        parser.error('reference-selection must carry its content sha256')
    if not isinstance(selection_document.get('product'), dict) or selection_document['product'].get('id') != 'installed-reference':
        parser.error('reference-selection must embed the installed-reference product')
    original = source = products_path = build = None
    options = argparse.Namespace(build=args.build, reference_selection=args.reference_selection,
                                 execution_products=None)
    products = []
    candidates = {}
    declared_source_root = None
    inputs_pins = {}
    if selected_mode:
        if args.collector is None and any(
                'collector' in json.loads((e.safe_path(path)/'runtime-binding.json').read_text(encoding='utf-8')).get('variant', '')
                for path in args.product):
            parser.error('selected collector products require --collector young|major')
        selected_products = []
        source_roots = set()
        for path in args.product:
            profile = accounting.runtime_product_profile(
                e.safe_path(path), collector=args.collector or 'off')
            selected_products.append(profile)
            source_roots.add(profile['source_root'])
            inputs_pins[profile['inputs_path']] = e.file_identity(Path(profile['inputs_path']))
        if len(source_roots) != 1:
            parser.error('selected products must share one declared source_root')
        declared_source_root = Path(next(iter(source_roots)))
        source = e.safe_path(declared_source_root)
        if source != R:
            parser.error('selected-product source_root must be this repository root')
        reference = dict(selection_document['product'])
        products = [reference, *selected_products]
        candidates = {p['id']: p for p in selected_products}
        if set(args.profile) - set(candidates):
            parser.error('declared profiles must match selected runtime-product identities')
        selected = args.profile or list(candidates)
        registry = {p['id']: p for p in products}
        options._reference_selection_summary = selection_document
        options._selected_reference_identity = e.product_identity(reference, resources=False)
        options._reference_selection_identities = {str(selection_path): selection_pin}
        options._reference_external_identities = {}
    else:
        if args.collector is not None:
            parser.error('--collector is only valid with --product')
        original, source, products_path, products, build = stage.read_action_build(options)
        candidates = {p['id']: p for p in products if p['kind'] != 'reference' and not p['test_hooks']}
        if len(candidates) != 48 or set(args.profile) - set(candidates):
            parser.error('complete registry and known non-diagnostic profiles required')
        selected = args.profile or list(candidates)
        registry = {p['id']: p for p in products if not p['test_hooks']}
    if args.include_reference:
        selected = ['installed-reference', *selected]
    logical_files = args.file or FILES
    for logical in logical_files:
        path = Path(logical)
        if path.is_absolute() or '..' in path.parts or not logical.startswith('test/') or path.suffix != '.el':
            parser.error('original test-relative Lisp files required')
        if not (source/logical).is_file():
            parser.error('current source test input is missing: '+logical)
    if args.display == 'cocoa' and any(registry[p]['kind'] == 'terminal' for p in selected):
        parser.error('Cocoa controls require a declared graphical product')
    if args.erc_graphical and (args.display != 'cocoa' or any(not name.startswith('test/lisp/erc/') for name in logical_files)):
        parser.error('graphical opt-in is limited to explicit Cocoa ERC controls')
    if args.erc_graphical_all and (args.display != 'cocoa' or logical_files != [
            'test/lisp/erc/erc-scenarios-internal.el']):
        parser.error('graphical-all opt-in requires the explicit Cocoa ERC coordinator file')
    out = e.new_output(args.out)
    before = {p['id']: e.product_identity(p, resources=True) for p in products}
    rows, checks, errors, pins, contexts = [], {}, [], {}, {}
    started = time.monotonic()
    with stage.owner.ArtifactRun(R, out) as artifacts:
        corpus = out/'current-version-inputs'
        corpus_entrypoints = None
        harness_sources, hi, ri = {}, {}, {}
        driver, driver_identity = Path(__file__).resolve(), None
        suite = batch = context_record = context_value = provider = manifest = None
        requested_scope, harness_sources = {}, {}
        try:
            if not selected_mode:
                stage.select_execution_inputs(artifacts, original, source, products_path, products, build, options)
            harness, hs, hi = stage.stage_stable_harness(out, source, artifacts)
            runner, rs, ri = stage.stage_stable_runner(R/'tools/fixtures/stable-suite-runner.el', out, source, artifacts)
            suite = load('current_control_suite', harness/'tools/stable-suite.py')
            batch = suite._load_batch()
            driver_identity = batch._reuse_identity(suite, driver)
            pins[str(driver)] = driver_identity
            entries = []
            context_record = None
            if args.corpus == 'stable':
                # The admitted build export contains every pinned original
                # input, including ignored overlays acquired for that build.
                manifest_path = source/'compatibility/stable-31.1/sources.json'
                manifest = suite.load_manifest(manifest_path, source)
                materialized = suite.materialize(manifest, source, corpus)
                suite._write_receipt(corpus, manifest, materialized)
                for entry in manifest['entries']:
                    path = source/entry['source_path']
                    pins[str(path)] = batch._reuse_identity(suite, path)
                entries = manifest['entries']
            else:
                corpus.mkdir()
                for base, directories, names in os.walk(source/'test', followlinks=False):
                    if any((Path(base)/name).is_symlink() for name in directories):
                        raise ValueError('current-version source contains a symlink directory')
                    for name in names:
                        path = Path(base)/name
                        identity = batch._reuse_identity(suite, path)
                        logical = path.relative_to(source).as_posix()
                        batch._copy_reuse_artifact(suite, path, corpus/logical)
                        entries.append(dict(path=logical, size=identity['bytes'], sha256=identity['sha256'],
                            mode=format(0o100000 | identity['mode'], 'o')))
                        pins[str(path)] = identity
                        if len(entries) > 10000:
                            raise ValueError('current-version control input count exceeds bound')
                manifest = dict(entries=entries)
                # Current controls use the declared source export directly.
                # The archive-only README comes from the pinned provider in
                # stable mode; it is not an input to these current controls.
            selected_entries = {entry['path']: entry for entry in entries}
            if any(name not in selected_entries for name in logical_files):
                raise ValueError('requested control is outside its exact original corpus')
            harness_sources = {str(path): suite._sha256_path(path) for path in
                [runner, *[harness/'tools'/name for name in
                    ('stable-suite.py', 'stable-suite-batch.py', 'stable-context.py',
                     'runtime-retire.py', 'reference-corpus.py')]]}
            header_sha = suite._sha256_path(harness/'tools/fixtures/module-thread-config.h')
            all_paths = {}
            for profile in selected:
                product = registry[profile]
                environment = stage.stable_product_environment(product, True)
                all_paths[profile] = suite._product_paths(product['program'], ['--dump-file='+product['dump']], environment)
            corpus_entrypoints = (len(batch.select_entrypoints(manifest))
                                  if manifest is not None and 'entries' in manifest else None)
            if args.corpus == 'stable':
                provider = load('control_context_provider', harness/'tools/stable-context.py')
                context_value, context_record = provider.prepare(suite, R, corpus, manifest, all_paths)
            for profile in selected:
                product = registry[profile]
                environment = stage.stable_product_environment(product, True)
                paths = all_paths[profile]
                paths['test_environment'] = ({'ERC_TESTS_GRAPHICAL':'1', 'ERC_TESTS_GRAPHICAL_ALL':'1'}
                    if args.erc_graphical else {'ERC_TESTS_GRAPHICAL_ALL':'1'}
                    if args.erc_graphical_all else {})
                context = corpus/'source-contexts'/profile
                if args.corpus == 'current':
                    context.mkdir(parents=True)
                links = dict(src=source/'src', admin=source/'admin', lisp=paths['lisp_root'], test=corpus/'test')
                for name in ('nextstep',):
                    if (source/name).exists():
                        links[name] = source/name
                if args.corpus == 'current':
                    for name, target in links.items():
                        (context/name).symlink_to(os.path.relpath(target, context), target_is_directory=target.is_dir())
                    contexts[str(context)] = {name: os.readlink(context/name) for name in links}
                paths['test_context'] = context
                admission_base = dict(schema='femacs-stable-control-admission-v1',
                    purpose='current-version' if args.corpus == 'current' else 'applicability',
                    profile=profile, product=suite._public_product_identity(paths),
                    runtime_environment=environment, test_environment=dict(paths['test_environment']),
                    reference_selection_sha256=selection_document['sha256'] if selected_mode
                        else options._reference_selection_summary['sha256'],
                    harness_sources_by_role={Path(name).name: sha for name, sha in harness_sources.items()},
                    module_header_sha256=header_sha,
                    manifest_sha256=suite._sha256_path(source/'compatibility/stable-31.1/sources.json')
                        if args.corpus == 'stable' else None,
                    selected_product_admission=selected_mode,
                    full_matrix_admission=not selected_mode)
                for logical in logical_files:
                    row_merged = None
                    display = (batch.original_file_display(logical)
                               if args.display == 'original' else args.display)
                    exclude_ids = None
                    scoped_display = display
                    admission = dict(admission_base)
                    if args.test_id:
                        # Reuse the runner's existing ERT (member ...) selection by
                        # discovering the file's registered IDs, then excluding every
                        # identity outside the declared request. Default full-file
                        # contract stays available when --test-id is omitted.
                        discovery = suite._run_process(
                            corpus, paths, runner, corpus/'test', logical, 'discover',
                            None, args.timeout, profile+'-discover')
                        payload = discovery.get('json') if isinstance(discovery, dict) else None
                        discovered = payload.get('new_ids') if isinstance(payload, dict) else None
                        if (discovery.get('status') != 'loaded' or not isinstance(discovered, list)
                                or any(not isinstance(item, str) or not item for item in discovered)
                                or len(set(discovered)) != len(discovered)):
                            raise ValueError('requested control scope requires exact file discovery: '+logical)
                        missing = [name for name in args.test_id if name not in discovered]
                        if missing:
                            raise ValueError('requested test ids are not registered in '+logical+': '+','.join(missing))
                        # Keep only IDs requested for this file when multiple --file
                        # values share one CLI; foreign requested ids stay out of exclude math.
                        requested_for_file = [name for name in args.test_id if name in discovered]
                        if not requested_for_file:
                            raise ValueError('no requested test ids belong to '+logical)
                        exclude_ids = sorted(set(discovered) - set(requested_for_file))
                        requested_scope[logical] = list(requested_for_file)
                        admission = dict(admission_base,
                            requested_test_ids=list(requested_for_file),
                            discovered_test_ids=list(discovered),
                            discovered_test_count=len(discovered),
                            excluded_outside_request=list(exclude_ids))
                    result = batch._run_file(suite, corpus, paths, runner, corpus/'test', logical,
                        args.timeout, profile, display=scoped_display,
                        resource_admission=admission, fixture_entries=batch._fixture_entries(manifest, logical),
                        exclude_ids=exclude_ids if exclude_ids else None,
                        reduced_invocation=False)
                    # A case that wedges inside a blocking system call cannot be
                    # resumed in this process, and the runner's own file
                    # completion never arrives, so the remaining cases of that
                    # file would never execute.  Each per-case deadline is
                    # bounded, so the file gets at most MAX_REDUCTIONS reduced
                    # invocations, each excluding exactly the identities that
                    # already reached their real deadline.  Every blocked case
                    # keeps its actual timeout status, every registered ID stays
                    # represented, and unrun identities are recorded as such:
                    # the file is never reported complete or passing.
                    MAX_REDUCTIONS = 2
                    blocked_rows, invocations, excluded = [], [], []
                    first = result
                    scope_excluded = list(exclude_ids or [])
                    for _ in range(MAX_REDUCTIONS + 1):
                        if not (scoped_display != 'batch' and result.get('timed_out')
                                and result.get('timed_out_id')):
                            break
                        blocked = result['timed_out_id']
                        blocked_rows.append((blocked, (result.get('cases') or {}).get(blocked) or {}))
                        excluded.append(blocked)
                        invocations.append(dict(
                            invocation=len(invocations) + 1,
                            excluded=list(excluded),
                            timed_out=bool(result.get('timed_out')),
                            timed_out_id=blocked,
                            complete=result.get('complete'),
                            file_result=result.get('file_result')))
                        result = batch._run_file(suite, corpus, paths, runner, corpus/'test', logical,
                            args.timeout, '%s-reduced-%d' % (profile, len(excluded)),
                            display=scoped_display,
                            resource_admission=admission,
                            fixture_entries=batch._fixture_entries(manifest, logical),
                            exclude_ids=sorted(set(scope_excluded) | set(excluded)),
                            reduced_invocation=True)
                    if invocations:
                        merged_cases = dict(result['cases'])
                        for blocked, row in blocked_rows:
                            if row:
                                merged_cases[blocked] = row
                        unrun = sorted(key for key, value in merged_cases.items()
                                       if value.get('status') == 'not-run')
                        stopped = bool(unrun) and bool(result.get('timed_out')
                                                       and result.get('timed_out_id'))
                        result = dict(result,
                            cases=merged_cases,
                            ids=list(first['ids']),
                            complete=False,
                            passed=False,
                            merged_complete=bool(result.get('complete')),
                            blocked_cases=[dict(test_id=test_id,
                                                status=row.get('status'),
                                                complete=row.get('complete'),
                                                source='per-case deadline')
                                           for test_id, row in blocked_rows],
                            unrun_ids=unrun,
                            invocations=invocations,
                            reduction_stopped_at=((result.get('timed_out_id'), unrun)
                                                  if stopped else None),
                            excluded_confirmed=result.get('excluded_confirmed'),
                            first_invocation=dict(
                                exit_code=first.get('exit_code'),
                                timed_out=first.get('timed_out'),
                                complete=dict((k, v.get('complete'))
                                              for k, v in first['cases'].items()),
                                timed_out_id=(invocations[0]['timed_out_id']
                                              if invocations else None)))
                        merged_dir = out/'merged'/profile
                        merged_dir.mkdir(parents=True, exist_ok=True)
                        merged_path = merged_dir/(logical.replace('/', '-')+'.json')
                        e.write_json(merged_path, dict(
                            schema='femacs-terminal-reduced-observations-v1',
                            profile=profile, file=logical,
                            first_invocation=result['first_invocation'],
                            blocked_cases=result['blocked_cases'],
                            invocations=invocations,
                            ids=result['ids'],
                            cases={key: {name: value.get(name) for name in
                                         ('test_id', 'status', 'complete', 'exit_code', 'seconds')}
                                   for key, value in merged_cases.items()},
                            unrun_ids=result['unrun_ids'],
                            reduction_stopped_at=result['reduction_stopped_at'],
                            results_map={key: result.get(key) for key in
                                         ('complete', 'passed', 'timed_out', 'timed_out_id',
                                          'exit_code', 'excluded_ids', 'excluded_confirmed')},
                            blocking_retained=True, counts_as_complete=False,
                            counts_as_passed=False))
                        row_merged = dict(path=str(merged_path.relative_to(out)),
                                          bytes=merged_path.stat().st_size,
                                          sha256=e.digest(merged_path))
                    input_file = (source/selected_entries[logical]['source_path']
                        if args.corpus == 'stable' else source/logical)
                    row = dict(profile=profile, file=logical, complete=result['complete'], passed=result['passed'],
                        file_result=result['file_result'], state_retirement=result['state_retirement'],
                        blocked_cases=result.get('blocked_cases'),
                        unrun_ids=result.get('unrun_ids'),
                        reduction_stopped_at=result.get('reduction_stopped_at'),
                        merged_observations=row_merged,
                        merged_case_status={k: v.get('status') for k, v in result.get('cases', {}).items()},
                        first_invocation=result.get('first_invocation'),
                        requested_test_ids=admission.get('requested_test_ids'),
                        test_input=dict(path=str(input_file), identity={key: pins[str(input_file)][key]
                                        for key in ('bytes', 'sha256')}))
                    rows.append(row)
                    e.write_json(out/'partial.json', dict(cases=rows, final=False, diagnostic_only=True))
        except Exception as error:
            errors.append(f'{type(error).__name__}: {error}')
            e.write_json(out/'failed-child.json', dict(error_type=type(error).__name__,
                error=str(error), traceback=traceback.format_exc()[-16384:], editor_file_processes_completed=len(rows)))
        finally:
            reference_selection_value = (selection_document if selected_mode
                                         else options._reference_selection_summary)
            if selected_mode:
                def selected_receipts_valid():
                    for product in candidates.values():
                        accounting.runtime_product_profile(
                            product['runtime_root'], collector=product['collector'])
                    reference = registry['installed-reference']
                    live = e.product_identity(reference, resources=False)
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
                    return roots == {str(source)} and source == R and (source/'test').is_dir()

                operations = {
                    'selected_product_identities_unchanged': lambda: before == {
                        p['id']: e.product_identity(p, resources=True) for p in products},
                    'selected_product_receipts_valid': selected_receipts_valid,
                    'declared_source_root_unchanged': declared_source_root_unchanged,
                    'reference_selection_unchanged': reference_selection_unchanged,
                    'captured_harness_runner_unchanged': lambda: bool(hi) and bool(ri) and all(
                        e.file_identity(e.safe_path(name)) == pin
                        for name, pin in {**hi, **ri}.items()),
                    'original_current_version_inputs_unchanged': lambda: (
                        batch is not None and suite is not None
                        and all(batch._reuse_identity(suite, Path(name)) == pin
                                for name, pin in pins.items())),
                    'copied_current_version_inputs_unchanged': lambda: (
                        batch is not None and suite is not None and manifest is not None
                        and batch._source_inputs_unchanged(suite, manifest, corpus)),
                    'stable_source_context_unchanged': lambda: args.corpus != 'stable' or (
                        context_record is not None and provider.verify(
                            suite, R, corpus, context_value, context_record) is None),
                    'current_source_context_links_unchanged': lambda: all(
                        {p.name for p in Path(root).iterdir()} == set(links) and all(
                            (Path(root)/name).is_symlink() and os.readlink(Path(root)/name) == target
                            for name, target in links.items()) for root, links in contexts.items()),
                }
            else:
                operations = {
                    'all61_product_identities_unchanged': lambda: before == {
                        p['id']: e.product_identity(p, resources=True) for p in products},
                    'all61_complete_receipts_valid': lambda: len(e.load_products(products_path)) == 61,
                    'source_export_unchanged': lambda: e.verify_source_export(
                        stage.build_source_owner(original, build), build['source_export']) == source,
                    'reference_selection_unchanged': lambda: stage.verify_reference_selection(
                        options, products) is None,
                    'captured_harness_runner_unchanged': lambda: bool(hi) and bool(ri) and all(
                        e.file_identity(e.safe_path(name)) == pin
                        for name, pin in {**hi, **ri}.items()),
                    'original_current_version_inputs_unchanged': lambda: (
                        batch is not None and suite is not None
                        and all(batch._reuse_identity(suite, Path(name)) == pin
                                for name, pin in pins.items())),
                    'copied_current_version_inputs_unchanged': lambda: (
                        batch is not None and suite is not None and manifest is not None
                        and batch._source_inputs_unchanged(suite, manifest, corpus)),
                    'stable_source_context_unchanged': lambda: args.corpus != 'stable' or (
                        context_record is not None and provider.verify(
                            suite, R, corpus, context_value, context_record) is None),
                    'current_source_context_links_unchanged': lambda: all(
                        {p.name for p in Path(root).iterdir()} == set(links) and all(
                            (Path(root)/name).is_symlink() and os.readlink(Path(root)/name) == target
                            for name, target in links.items()) for root, links in contexts.items()),
                }
            for key, operation in operations.items():
                try:
                    checks[key] = operation() is True
                except Exception as error:
                    checks[key] = False
                    errors.append(f'{key}: {type(error).__name__}: {error}')
            complete = (not errors and all(checks.values()) and len(rows) == len(selected)*len(logical_files)
                        and all(row['complete'] for row in rows))
            e.write_json(out/'results.json', dict(schema='femacs-current-version-controls-v1',
                passed=complete, diagnostic_collection_complete=complete,
                test_acceptance_passed=complete and all(row['passed'] for row in rows),
                cases=rows, checks=checks, errors=errors,
                reference_selection=reference_selection_value,
                build_receipt=None if selected_mode else str(original),
                selected_product_admission=selected_mode,
                full_matrix_admission=not selected_mode,
                registry_product_count=None if selected_mode else 61,
                selected_product_count=len(candidates),
                requested_profiles=selected,
                remaining_profiles=[] if selected_mode else [p for p in candidates if p not in selected],
                requested_files=list(logical_files),
                requested_test_ids=list(args.test_id) if args.test_id else None,
                requested_scope=requested_scope or None,
                source_version=selection_document['upstream']['version'] if selected_mode
                    else options._reference_selection_summary['upstream']['version'],
                current_source_context_links=contexts, corpus_root=str(corpus), corpus_kind=args.corpus,
                requested_display=args.display, erc_graphical_opt_in=args.erc_graphical, harness_sources=harness_sources,
                erc_graphical_all_opt_in=args.erc_graphical_all,
                control_driver=dict(path=str(driver), identity=driver_identity),
                retained_blocking=[dict(profile=r['profile'], file=r['file'],
                                        blocked=[dict(test_id=item.get('test_id'),
                                                      status=item.get('status'))
                                                 for item in (r.get('blocked_cases') or [])],
                                        unrun_ids=r.get('unrun_ids'),
                                        reduction_stopped_at=r.get('reduction_stopped_at'),
                                        note='per-case deadlines retained; unrun identities stay '
                                             'explicit and the file stays incomplete and blocking')
                                   for r in rows if r.get('blocked_cases')],
                reduced_observations_are_diagnostic_only=True,
                default_scope_unchanged=dict(
                    uses_default_files=not args.file,
                    uses_default_profiles=not args.profile,
                    default_file_count=len(FILES),
                    default_profile_count=len(candidates),
                    selected_file_count=len(logical_files),
                    selected_profile_count=len(selected),
                    declared_corpus_entrypoints=corpus_entrypoints),
                raw_failures_promoted=False, formal_corpus_cases_reexecuted=0, diagnostic_only=True,
                formal_corpus_complete=False, compatibility_accepted=False, performance_accepted=False,
                goal_complete=False, seconds=time.monotonic()-started,
                corpus_retirement_pending=True, artifact_retention=artifacts.retain_current_files()))
    return 0 if complete else 1


if __name__ == '__main__':
    raise SystemExit(main())
