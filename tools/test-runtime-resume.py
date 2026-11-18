"""Actual interrupted-matrix recovery integration; use build protection.

The supplied checkpoint is read-only. This driver runs the maintained recovery
CLI once, then uses the ordinary stage/reference reader on its complete result.
It is build integration, not editor semantics or performance acceptance.
"""
import argparse
import importlib.util
import json
import os
from pathlib import Path
import selectors
import subprocess
import sys
import tempfile
import time

REPO = Path(__file__).resolve().parent.parent


def load_resume():
    spec = importlib.util.spec_from_file_location('actual_runtime_recovery', REPO/'tools/runtime-resume.py')
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def admission(out):
    """Exercise recovery admission with real files, holders and query failure."""
    recovery = load_resume()
    execution = recovery.execution
    out = execution.safe_path(out, must_exist=False)
    allowed = (REPO/'build', Path.home()/'AI/caches')
    if not any(out != base and out.is_relative_to(base) for base in allowed):
        raise ValueError('admission output must be an owned build or cache child')
    out.parent.mkdir(parents=True, exist_ok=True)
    out.mkdir(mode=0o700)
    results = []
    started = time.monotonic()
    with tempfile.TemporaryDirectory(prefix='owned-fixtures-', dir=out) as temporary:
        root = Path(temporary)
        original = root/'original'
        original.mkdir()
        held_file = original/'held.txt'
        held_file.write_text('owned admission fixture\n')
        stopped = subprocess.Popen([sys.executable, '-B', '-c', 'pass'])
        stopped.wait(timeout=5)
        assert stopped.returncode == 0
        receipt = root/'controller.json'
        controller = dict(root_pid=stopped.pid, exit_code=1, cleanup_complete=True)
        execution.write_json(receipt, controller)

        before = time.monotonic()
        assert recovery.assert_stopped(receipt, original) == controller
        results.append(dict(id='idle-output-admitted', passed=True,
                            seconds=time.monotonic()-before))

        before = time.monotonic()
        holder = subprocess.Popen([sys.executable, '-B', '-c',
            'import sys\nf=open(sys.argv[1],"rb")\nprint("ready",flush=True)\nsys.stdin.buffer.read(1)\nf.close()',
            str(held_file)], stdin=subprocess.PIPE, stdout=subprocess.PIPE,
            stderr=subprocess.DEVNULL)
        try:
            with selectors.DefaultSelector() as ready:
                ready.register(holder.stdout, selectors.EVENT_READ)
                assert ready.select(timeout=5), 'holder did not open its file'
                assert holder.stdout.readline() == b'ready\n'
            try:
                recovery.assert_stopped(receipt, original)
            except recovery.OutputOccupancyError as error:
                occupied = error.output_occupancy
                assert occupied['clear'] is False and occupied['returncode'] in (0, 1)
                assert occupied['stdout_bytes'] > 0
                assert occupied['holder_pid_count'] >= 1 and occupied['holder_commands']
                assert 'stdout' not in occupied and 'stderr' not in occupied
                assert str(original) not in str(error)
                assert holder.poll() is None, 'admission must not terminate the holder'
            else:
                raise AssertionError('occupied output was admitted')
        finally:
            holder.stdin.close()
            try:
                holder.wait(timeout=5)
            except subprocess.TimeoutExpired:
                holder.terminate()
                try:
                    holder.wait(timeout=2)
                except subprocess.TimeoutExpired:
                    holder.kill()
                    holder.wait(timeout=2)
            holder.stdout.close()
        assert holder.returncode == 0, 'holder did not exit normally'
        assert recovery.assert_stopped(receipt, original) == controller
        results.append(dict(id='actual-holder-refused-without-signalling', passed=True,
                            seconds=time.monotonic()-before, output_occupancy=occupied))

        before = time.monotonic()
        empty_path = root/'empty-bin'
        empty_path.mkdir()
        old_path = os.environ.get('PATH')
        try:
            os.environ['PATH'] = str(empty_path)
            try:
                recovery.require_idle_output(original)
            except recovery.OutputOccupancyError as error:
                unknown = error.output_occupancy
                assert unknown['clear'] is False and unknown['returncode'] is None
                assert unknown['error_type'] == 'FileNotFoundError'
                assert unknown['holder_pid_count'] == 0
                assert 'FileNotFoundError' in str(error)
            else:
                raise AssertionError('unavailable query was admitted')
        finally:
            if old_path is None:
                os.environ.pop('PATH', None)
            else:
                os.environ['PATH'] = old_path
        assert recovery.assert_stopped(receipt, original) == controller
        results.append(dict(id='unavailable-query-refused-with-cause', passed=True,
                            seconds=time.monotonic()-before, output_occupancy=unknown))
    report = dict(schema='femacs-runtime-recovery-admission-integration-v1',
                  passed=all(row['passed'] for row in results), cases=results,
                  seconds=time.monotonic()-started, owned_fixture_retired=not root.exists(),
                  owned_child_processes_reaped=True, actual_compiler_execution=False,
                  actual_editor_execution=False, full_matrix_recovery_accepted=False,
                  compatibility_accepted=False, goal_complete=False)
    execution.write_json(out/'results.json', report)
    print(json.dumps(dict(passed=report['passed'], results=str(out/'results.json'))))
    return 0 if report['passed'] else 1


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--mode', choices=('integration', 'admission'), default='integration')
    parser.add_argument('--build', type=Path)
    parser.add_argument('--observation', type=Path)
    parser.add_argument('--recovered-out', type=Path)
    parser.add_argument('--backend-prefix', type=Path)
    parser.add_argument('--out', type=Path, required=True)
    parser.add_argument('--timeout', type=float, default=2700)
    args = parser.parse_args(argv)
    if args.mode == 'admission':
        if any((args.build, args.observation, args.recovered_out, args.backend_prefix)):
            parser.error('admission uses only owned fixtures, not build inputs')
        return admission(args.out)
    if any(value is None for value in
           (args.build, args.observation, args.recovered_out, args.backend_prefix)):
        parser.error('integration requires build, observation, recovered-out and backend-prefix')
    if not 0 < args.timeout <= 5400:
        parser.error('finite recovery process deadline required')
    recovery = load_resume()
    execution, stage = recovery.execution, recovery.stage
    original = execution.safe_path(args.build)
    target = execution.safe_path(args.recovered_out, must_exist=False)
    if target.exists():
        parser.error('fresh recovered output required')
    observation = execution.safe_path(args.observation)
    out = execution.new_output(args.out)
    source_paths = [Path(__file__), Path(recovery.__file__), Path(stage.__file__),
                    Path(execution.__file__), Path(stage.owner.__file__)]
    source_pins = {str(p): execution.file_identity(p) for p in source_paths}
    old_partial = execution.digest(original/'partial.json')
    checks, errors, process, occupancy = {}, [], None, None
    started = time.monotonic()
    try:
        admitted = recovery.assert_stopped(observation, original)
        expected = admitted['verified_completed_configurations']
        checks['guarded_prefix_only'] = bool(expected) and len(expected) < 20
        command = [sys.executable, '-B', str(REPO/'tools/runtime-resume.py'),
                   '--build', str(original), '--controller-result', str(observation),
                   '--backend-prefix', str(execution.safe_path(args.backend_prefix)),
                   '--retention-mode', 'files', '--out', str(target)]
        original_report = execution.bounded_json(original/'partial.json', 128*1024*1024)
        frozen_source = Path(original_report['source_export']['source'])
        compiled_pins = {name: execution.digest(frozen_source/name) for name in
            ('tools/runtime-stage.py', 'tools/runtime-owner.py', 'tools/runtime-execution.py',
             'tools/stable-suite.py', 'src/lread.rs')}
        execution.write_json(out/'launch.json', dict(argv=command,
                             source_pins_at_launch=compiled_pins,
                             original_partial_sha256=old_partial,
                             expected_reused_configurations=expected,
                             expected_new_configurations=20-len(expected)))
        process = execution.run_process(command, out/'recovery-child', stage.clean_environment(),
                                        args.timeout, cwd=REPO)
        checks['actual_recovery_process_closed'] = (process['complete'] is True
            and process['exit_code'] == 0 and process['remaining_group_terminated'] is False)
        if not checks['actual_recovery_process_closed']:
            raise ValueError('actual recovery CLI failed; original child logs retained')
        root, source, products_path, products, report = stage.read_build(target)
        checks['ordinary_build_reader_accepts'] = root == target and len(products) == 61
        checks['all20_configurations_complete'] = (len(report['builds']) == 20
            and all(row['passed'] is True for row in report['builds']))
        checks['exact_guarded_prefix_reused'] = (set(report['reused_configurations']) == set(expected)
            and sum(row.get('reused') is False for row in report['builds']) == 20-len(expected))
        checks['all48_candidates_and41_physical_products'] = (
            len([p for p in products if p['kind'] != 'reference' and not p['test_hooks']]) == 48
            and len({(p['program'], p['dump']) for p in products}) == 41)
        document = execution.bounded_json(products_path)
        checks['reference_selection_registry_contract'] = (set(document) == {'schema', 'products', 'source_cohorts'}
            and document['source_cohorts'] == report['source_cohorts']
            and document['products'] == report['products'])
        checks['physical_file_retention_only'] = report['artifact_retention'] == dict(
            report['artifact_retention'], mode='verified-files', all_original_files_verified=True,
            duplicate_content_archive_created=False)
        checks['recovery_inputs_unchanged'] = report['recovery_inputs_unchanged'] is True
    except Exception as error:
        errors.append(f'{type(error).__name__}: {error}')
        if isinstance(error, recovery.OutputOccupancyError):
            occupancy = error.output_occupancy
    finally:
        checks['original_partial_unchanged'] = execution.digest(original/'partial.json') == old_partial
        checks['driver_sources_unchanged'] = all(execution.file_identity(Path(p)) == pin
                                               for p, pin in source_pins.items())
    result = dict(schema='femacs-runtime-recovery-functional-v1', passed=all(checks.values()) and not errors,
        checks=checks, errors=errors, process=process, output_occupancy=occupancy,
        seconds=time.monotonic()-started,
        recovered_build=str(target), source_pins=source_pins,
        recovery_subprocess_started=process is not None and process.get('pid') is not None,
        actual_compiler_execution_confirmed=checks.get('all20_configurations_complete') is True
            and checks.get('exact_guarded_prefix_reused') is True,
        actual_editor_semantics_acceptance=False, original_corpus_execution=False,
        compatibility_accepted=False, performance_accepted=False, goal_complete=False)
    execution.write_json(out/'results.json', result)
    print(json.dumps(dict(passed=result['passed'], results=str(out/'results.json'))))
    return 0 if result['passed'] else 1


if __name__ == '__main__':
    raise SystemExit(main())
