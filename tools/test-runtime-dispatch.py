"""Real Python dispatch/writer/reader overlap and cancellation integration.

Products and corpus descriptors are synthetic. No editor or compiler runs.
The existing eight scope flows remain the writer/reader integration baseline.
"""
import argparse
import copy
from contextlib import ExitStack
from contextlib import redirect_stderr
import io
import importlib.util
import json
import os
from pathlib import Path
import shutil
import signal
import sys
import threading
import time
from types import SimpleNamespace
from unittest import mock

ROOT = Path(__file__).resolve().parent.parent


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    value = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(value)
    return value


base = load('dispatch_scope_integration', ROOT/'tools/test-runtime-phases.py')
stage, e = base.stage, base.stage.execution

PRELUDE = '''import os, sys, time, json
from pathlib import Path
trace=os.environ.get('DISPATCH_TRACE')
if trace:
 dispatch_root=Path(trace);dispatch_root.mkdir(parents=True,exist_ok=True)
 dispatch_name=Path(sys.argv[sys.argv.index('--out')+1]).name
 begin=time.monotonic()
 marker=Path(os.environ['DISPATCH_OWNER_MARKER'])
 (dispatch_root/(dispatch_name+'.ready')).write_text(json.dumps(dict(pid=os.getpid(),started=begin,
  owner_immutable=bool(marker.stat().st_flags & 2))))
 deadline=begin+5
 while len(list(dispatch_root.glob('*.ready')))<2 and time.monotonic()<deadline:time.sleep(.005)
 if len(list(dispatch_root.glob('*.ready')))!=2:raise RuntimeError('two real children did not overlap')
 time.sleep(.15 if dispatch_name.endswith('first') else .02)
'''
EPILOGUE = '''if trace:
 (dispatch_root/(dispatch_name+'.ended')).write_text(json.dumps(dict(ended=time.monotonic())))
'''


def alive(pid):
    try:
        os.kill(pid, 0)
        return True
    except ProcessLookupError:
        return False


def ordered_overlap(out):
    started = time.monotonic()
    original = stage._execute_profiles
    observed = []
    child = PRELUDE + base.STABLE_CHILD.replace('raise SystemExit(1 if failed else 0)',
                                               EPILOGUE+'raise SystemExit(1 if failed else 0)')

    def execute(options, output, source, products_path, products, inner, env, **kwargs):
        options.stable_workers = 2
        if len(stage.execution_plan('stable', products, options.stable_profile)[1]) != 2:
            return original(options, output, source, products_path, products, inner, env, **kwargs)
        trace = output/'overlap'
        marker = output/'owner-input'
        marker.write_bytes(b'one artifact owner throughout both real children\n')
        with stage.owner.ArtifactRun(ROOT, output) as owner:
            stage.owner.capture_artifacts(owner.journal, {str(marker):e.file_identity(marker)})
            partial_order = []
            write = e.write_json
            def record(path, value):
                if path == output/'partial.json':
                    partial_order.append([row['id'] for row in value['cases']])
                return write(path, value)
            with mock.patch.object(e, 'write_json', side_effect=record):
                result = original(options, output, source, products_path, products, inner,
                    dict(env, DISPATCH_TRACE=str(trace), DISPATCH_OWNER_MARKER=str(marker)), **kwargs)
            assert marker.stat().st_flags & 2
            if not all((trace/('stable-'+name+'.ended')).exists() for name in ('first', 'second')):
                e.write_json(out/'failed-child.json', dict(cases=result,
                    trace={p.name:p.read_text()[:2048] for p in trace.iterdir()},
                    child_logs={row['id']:{name:(Path(row['process']['evidence_directory'])/(name+'.txt')).read_text()[-2048:]
                        for name in ('stdout', 'stderr')} for row in result}))
                raise AssertionError('parallel child completion trace missing; failed-child.json retained')
            times = {name: {**json.loads((trace/('stable-'+name+'.ready')).read_text()),
                            **json.loads((trace/('stable-'+name+'.ended')).read_text())}
                     for name in ('first', 'second')}
            # Older Python/macOS monotonic clocks have process-relative epochs.
            # The two-child barrier proves overlap; parent receipt publication
            # proves completion order without comparing different child clocks.
            assert partial_order == [['stable-second'], ['stable-first', 'stable-second']]
            assert all(x['owner_immutable'] and not alive(x['pid']) for x in times.values())
            assert [x['id'] for x in result] == ['stable-first', 'stable-second']
        assert not marker.stat().st_flags & 2
        assert e.bounded_json(output/'artifact-preservation.json')['state'] == 'released'
        observed.append(dict(children=times, parent_publication_order=partial_order,
                             child_timestamps_are_process_relative=True))
        return result

    with mock.patch.object(base, 'STABLE_CHILD', child), mock.patch.object(stage, '_execute_profiles', side_effect=execute):
        assert base.stable_integration(out)
    baseline = e.bounded_json(out/'results.json')
    assert len(observed) == 1 and baseline['passed'] and baseline['owned_fixture_retired']
    return dict(id='ordered-overlap-with-existing-eight-raw-writer-reader-flows', passed=True,
        seconds=time.monotonic()-started, baseline_cases=8, overlap=observed[0],
        raw_exit1_and_counts_preserved=True, shared_owner_released_after_children=True)


def cancellation(out):
    started = time.monotonic()
    root = out/'owned-cancellation';root.mkdir()
    source = root/'source';(source/'tools').mkdir(parents=True)
    trace = root/'trace';trace.mkdir()
    (source/'tools/stable-suite.py').write_text('''import os, sys, time, json
from pathlib import Path
name=Path(sys.argv[sys.argv.index('--out')+1]).name
(Path(os.environ['DISPATCH_TRACE'])/(name+'.ready')).write_text(json.dumps(dict(pid=os.getpid())))
time.sleep(30)
''')
    output = root/'stage';output.mkdir()
    marker = output/'owner-input';marker.write_bytes(b'input remains protected until cancellation reaps children\n')
    reference = dict(id='reference', kind='reference', test_hooks=False, collector='off', program='synthetic', dump='synthetic')
    candidates = [dict(reference, id='candidate-'+str(i), kind='terminal') for i in range(4)]
    products = [reference, *candidates]
    monitor_errors = []

    def interrupt():
        deadline = time.monotonic()+5
        while len(list(trace.glob('*.ready'))) < 2 and time.monotonic() < deadline:
            time.sleep(.01)
        if len(list(trace.glob('*.ready'))) != 2:
            monitor_errors.append('two active children were not observed')
        os.kill(os.getpid(), signal.SIGINT)

    watcher = threading.Thread(target=interrupt, name='dispatch-integration-interrupt')
    interrupted = False
    try:
        with stage.owner.ArtifactRun(ROOT, output) as owner:
            stage.owner.capture_artifacts(owner.journal, {str(marker):e.file_identity(marker)})
            watcher.start()
            try:
                stage._execute_profiles(SimpleNamespace(execution_phase='stable', stable_profile=[],
                    stable_workers=2, timeout=5), output, source, root/'products.json', products,
                    root/'inner', dict(os.environ, DISPATCH_TRACE=str(trace)))
            except SystemExit as error:
                assert error.code == 128+signal.SIGINT
                interrupted = True
            finally:
                watcher.join(timeout=6)
            assert not watcher.is_alive() and not monitor_errors and interrupted
            nodes = [json.loads(p.read_text()) for p in trace.glob('*.ready')]
            assert len(nodes) == 2 and all(not alive(item['pid']) for item in nodes)
            processes = [e.bounded_json(p) for p in output.glob('stable-*/process.json')]
            assert len(processes) >= 2 and all(p.get('cancelled') is True
                and p['complete'] is False and p['timed_out'] is False for p in processes)
            assert marker.stat().st_flags & 2
        assert not marker.stat().st_flags & 2
        assert e.bounded_json(output/'artifact-preservation.json')['state'] == 'released'
        return dict(id='interrupt-cancels-queue-and-reaps-before-owner-release', passed=True,
            seconds=time.monotonic()-started, active_children=2, queued_children_started=0,
            cancellation_distinct_from_timeout=True)
    finally:
        if watcher.ident is not None:
            watcher.join(timeout=6)
        shutil.rmtree(root)


def control_registry(out):
    """A reference-defect row must not replace later control-run selection.

    The existing full555 synthetic writer/reader lifecycle is exercised first.
    Stop the additional route at control selection: this verifies registry
    routing, not acceptance of the deliberately incomplete control descriptor.
    """
    started = time.monotonic()
    subject = load('dispatch_reference_control_integration', ROOT/'tools/test-reference-library-contract.py')
    root = out/'owned-control-registry';root.mkdir()
    checked, lifecycle = [], []

    class SelectedControl(Exception):
        pass

    def verify(context, raw_root, report, _file, arguments, result):
        if checked or result['reference_defect_lifecycle'][0]['state'] != 'active-reference-only-defect':
            return
        current = copy.deepcopy(report)
        case = current['cases'][1]
        case['passed'] = False
        current['corpus_complete'] = False
        current['failures'].append('case failed/unverified: '+case['file']+' :: '+case['id'])
        for label in ('reference', 'candidate'):
            case[label]['status'] = 'failed'
        subject.suite._write_run_report(raw_root, current)
        definition = copy.deepcopy(subject.e.bounded_json(arguments.approved_rules))
        rule = dict(definition['rules'][0], file=case['file'], id=case['id'],
                    category='proven-version-evolution', reason='synthetic control routing boundary')
        rule.pop('reference_defect_gate')
        definition['rules'].append(rule)
        approved = root/'route-reviewed.json';subject.e.write_json(approved, definition)
        stage_input = subject.e.bounded_json(arguments.run_results[0])
        stage_input['cases'][0].update(receipt_sha256=subject.e.digest(raw_root/'stable-suite-run.json'),
            completion_sha256=subject.e.digest(raw_root/'stable-suite-completion.json'))
        stage_path = root/'route-stage.json';subject.e.write_json(stage_path, stage_input)
        registry = [object()]

        def select(runs, _case, _label, _profile, **kwargs):
            assert runs is registry and kwargs['purpose'] == 'current-version'
            checked.append(True)
            raise SelectedControl()

        products_path = root/'products.json'

        def admission(options):
            options._reference_selection_summary = context.selection
            return context.original, context.source, products_path, context.products, dict(source_export=dict(synthetic=True))

        envs = report['runtime_environment']
        with ExitStack() as stack:
            for obj, name, replacement in (
                (subject.stage, 'read_action_build', admission),
                (subject.stage, 'stable_product_environment',
                 lambda product, _explicit: envs['reference' if product['kind'] == 'reference' else 'candidate']),
                (subject.accounting, '_closed_controls', lambda _arguments: (registry, {})),
                (subject.accounting, '_select_control', select)):
                stack.enter_context(mock.patch.object(obj, name, side_effect=replacement))
            changed = SimpleNamespace(**vars(arguments))
            changed.approved_rules, changed.run_results = approved, [stage_path]
            changed.out = root/'unreached-acceptance'
            try:
                subject.accounting.assemble(subject.stage, changed)
            except SelectedControl:
                pass
            else:
                raise AssertionError('current-version control selection was not reached')

    try:
        subject.lifecycle_integration(root, lifecycle, on_active=verify)
        assert checked == [True] and len(lifecycle) == 1 and all(row['passed'] for row in lifecycle), (checked, lifecycle)
        return dict(id='reference-defect-keeps-later-version-control-registry', passed=True,
                    lifecycle_cases=1, seconds=time.monotonic()-started,
                    route_stops_before_control_acceptance=True)
    finally:
        shutil.rmtree(root)


def functional_inputs(paths):
    """Bind passed affected groups without relabelling a failed parent attempt."""
    values, pins = [], {}
    for path in paths:
        path = e.safe_path(path)
        if path.stat().st_size > 16*1024*1024:
            raise ValueError('bounded functional receipt required')
        pins[str(path)] = e.file_identity(path)
        values.append(e.bounded_json(path))
    required = {'ordered-overlap-with-existing-eight-raw-writer-reader-flows',
                'interrupt-cancels-queue-and-reaps-before-owner-release',
                'reference-defect-keeps-later-version-control-registry'}
    passed = {row['id'] for value in values
              if value.get('schema') == 'femacs-runtime-dispatch-integration-v1'
              and value.get('owned_fixture_retired') is True
              for row in value.get('cases', []) if row.get('passed') is True}
    if not required <= passed:
        raise ValueError('each completed functional dispatch/control group required')
    probes = [v for v in values if v.get('schema') == 'femacs-runtime-tramp-process-probe-v1'
              and v.get('passed') is True and v.get('signal_contract_passed') is True]
    libraries = [v for v in values if v.get('schema') == 'femacs-runtime-frame-buffer-probe-v1'
                 and v.get('passed') is True and v.get('selected_inputs_unchanged') is True
                 and v.get('generated_override_used') is False and len(v.get('cases', [])) == 49
                 and all(row.get('observation', {}).get('correct_behavior') is True
                         for row in v['cases'] if row.get('profile') != 'installed-reference')]
    if len(probes) != 1 or len(libraries) != 1:
        raise ValueError('actual signal and all49 product-library functional receipts required')
    return pins


def protection(out, functional):
    pins = functional_inputs(functional)
    root = out/'owned-protection';root.mkdir()
    rows = []

    def case(name, action):
        started = time.monotonic()
        action()
        rows.append(dict(id=name, passed=True, seconds=time.monotonic()-started))

    def rejected_workers():
        for workers, pairs in ((0, [(1, 2)]), (3, [(1, 2)]), (True, [(1, 2)]), (2, [])):
            try:
                stage._stable_worker_count(SimpleNamespace(stable_workers=workers), pairs)
            except ValueError:
                pass
            else:
                raise AssertionError('invalid worker scope was accepted')

    def cli_gates():
        for index, (action, phase, workers) in enumerate((('execute','stable',0),
                ('execute','stable',3), ('prepare','all',2), ('execute','performance',2))):
            target = root/('rejected-output-'+str(index))
            with redirect_stderr(io.StringIO()):
                try:
                    stage.main(['--action', action, '--execution-phase', phase,
                                '--stable-workers', str(workers), '--out', str(target)])
                except SystemExit as failure:
                    assert failure.code == 2
                else:
                    raise AssertionError('invalid CLI worker scope was accepted')
            assert not target.exists()

    def cancelled_before_spawn():
        flag = threading.Event();flag.set()
        marker = root/'must-not-run'
        result = e.run_process([sys.executable, '-c',
            'from pathlib import Path;Path('+repr(str(marker))+').write_text("ran")'],
            root/'pre-cancelled', dict(os.environ), 2, cancel_event=flag)
        assert result['status'] == 'cancelled' and result['complete'] is False
        assert result['timed_out'] is False and result['exit_code'] is None
        assert 'pid' not in result and not marker.exists()

    def deadline():
        result = e.run_process([sys.executable, '-c', 'import time;time.sleep(30)'],
            root/'deadline', dict(os.environ), .05, cancel_event=threading.Event())
        assert result['timed_out'] is True and result['complete'] is False
        assert not result.get('cancelled', False) and not alive(result['pid'])

    def raw_exit():
        result = e.run_process([sys.executable, '-c', 'raise SystemExit(1)'],
            root/'raw-exit', dict(os.environ), 2)
        assert result['exit_code'] == 1 and result['complete'] is True
        assert 'cancelled' not in result and not result['remaining_group_terminated']

    def invalid_product_scope():
        probe = load('dispatch_scope_probe_subject', ROOT/'tools/runtime-frame-buffer-probe.py')
        products = [dict(id='p'+str(i), test_hooks=False) for i in range(49)]
        products.append(dict(id='diagnostic', test_hooks=True))
        fake = SimpleNamespace(execution=SimpleNamespace(load_tool=lambda _name: None,
            load_products=lambda _path: products))
        fake_spec = SimpleNamespace(loader=SimpleNamespace(exec_module=lambda _module: None))
        for selection in (['p0', 'p0'], ['unknown'], ['diagnostic']):
            with mock.patch.object(probe.importlib.util, 'spec_from_file_location', return_value=fake_spec), \
                    mock.patch.object(probe.importlib.util, 'module_from_spec', return_value=fake), \
                    mock.patch.object(sys, 'argv', ['probe', '--products', 'synthetic', '--out', str(root/'scope'),
                        *[arg for profile in selection for arg in ('--profile', profile)]]), \
                    redirect_stderr(io.StringIO()):
                try:
                    probe.main()
                except SystemExit as failure:
                    assert failure.code == 2
                else:
                    raise AssertionError('duplicate/unknown/diagnostic product accepted')
            assert not (root/'scope').exists()

    try:
        case('invalid-worker-count-or-empty-scope-rejected', rejected_workers)
        case('worker-cli-gates-run-before-output-creation', cli_gates)
        case('cancellation-before-spawn-executes-no-child', cancelled_before_spawn)
        case('cancellable-wait-keeps-original-hard-deadline', deadline)
        case('ordinary-exit1-remains-complete-and-unmodified', raw_exit)
        case('library-scope-keeps-duplicate-unknown-and-diagnostic-rejection', invalid_product_scope)
        assert all(e.file_identity(Path(path)) == pin for path, pin in pins.items())
        return rows, pins
    finally:
        shutil.rmtree(root)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out', type=Path, required=True)
    parser.add_argument('--group', choices=('all', 'dispatch', 'cancellation', 'control-registry'), default='all')
    parser.add_argument('--mode', choices=('integration', 'protection'), default='integration')
    parser.add_argument('--functional', type=Path, action='append', default=[])
    args = parser.parse_args()
    if (args.mode == 'protection') != bool(args.functional) or (args.mode == 'protection' and args.group != 'all'):
        parser.error('protection requires explicit functional receipts and no integration group selection')
    out = e.new_output(args.out)
    rows, error, prerequisite = [], None, {}
    started = time.monotonic()
    try:
        if args.mode == 'protection':
            rows, prerequisite = protection(out, args.functional)
        elif args.group in ('all', 'dispatch'):
            rows.append(ordered_overlap(out))
        if args.mode == 'integration' and args.group in ('all', 'cancellation'):
            rows.append(cancellation(out))
        if args.mode == 'integration' and args.group in ('all', 'control-registry'):
            rows.append(control_registry(out))
    except BaseException as failure:
        error = f'{type(failure).__name__}: {failure}'
        raise
    finally:
        e.write_json(out/'results.json', dict(schema=('femacs-runtime-dispatch-protection-v1'
                if args.mode == 'protection' else 'femacs-runtime-dispatch-integration-v1'),
            passed=error is None and len(rows) == (6 if args.mode == 'protection' else 3 if args.group == 'all' else 1),
            group=args.group, cases=rows, error=error,
            functional_inputs=prerequisite,
            seconds=time.monotonic()-started, actual_editor_execution=False,
            original_corpus_execution=False, owned_fixture_retired=not (out/'owned-cancellation').exists()
                and not (out/'owned-stable-scope-fixtures').exists()
                and not (out/'owned-control-registry').exists()
                and not (out/'owned-protection').exists(), goal_complete=False))


if __name__ == '__main__':
    main()
