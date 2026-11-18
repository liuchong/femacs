"""Exercise the owned mock initialization on actual host-shell PTYs.

The positive control re-enables job control after initialization. No editor,
original test, library, host configuration or user terminal is modified.
"""
import errno
import importlib.util
import json
import os
from pathlib import Path
import select
import shlex
import signal
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parent.parent


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    value = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(value)
    return value


def shell_case(stage, suite, root, enable_monitor):
    root.mkdir()
    env = stage.clean_environment()
    env.update(suite.case_environment('test/lisp/net/tramp-tests.el', root))
    helper = ('import os,json,termios; a=termios.tcgetattr(0); '
              'print("MOCK-CHILD "+json.dumps(dict(pid=os.getpid(),pgid=os.getpgrp(),'
              'sid=os.getsid(0),icanon=bool(a[3]&termios.ICANON),'
              'echo=bool(a[3]&termios.ECHO),icrnl=bool(a[0]&termios.ICRNL))))')
    command = ('set -m; ' if enable_monitor else '')
    command += ('printf "MOCK-SHELL %s\\n" "$$"; '
                + shlex.quote(sys.executable)+' -c '+shlex.quote(helper)
                + '; printf "MOCK-TRAPS-BEGIN\\n"; trap -p DEBUG; printf "MOCK-TRAPS-END\\n"; exit')
    started = time.monotonic()
    pid, master = os.forkpty()
    if pid == 0:
        try:
            os.execve('/bin/sh', ['sh', '-i', '-c', command], env)
        except BaseException:
            os._exit(127)
    os.set_blocking(master, False)
    output, status, ended = bytearray(), None, False
    try:
        deadline = started + 8
        while time.monotonic() < deadline:
            if select.select([master], [], [], .05)[0]:
                try:
                    block = os.read(master, 4096)
                except OSError as error:
                    if error.errno != errno.EIO:
                        raise
                    block = b''
                if not block:
                    ended = True
                output.extend(block)
                if len(output) > 65536:
                    raise ValueError('owned shell output exceeds64KiB')
            if status is None:
                found, value = os.waitpid(pid, os.WNOHANG)
                if found:
                    status = value
            if status is not None and ended:
                break
        if status is None:
            raise TimeoutError('owned shell PTY exceeded8 seconds')
    finally:
        # Reap only this PTY's unique process group; never a host terminal.
        try:
            os.killpg(pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        if status is None:
            _, status = os.waitpid(pid, 0)
        os.close(master)
    (root/'pty.txt').write_bytes(output)
    text = output.decode('utf-8', errors='replace').replace('\r\n', '\n')
    shell = [line for line in text.splitlines() if line.startswith('MOCK-SHELL ')]
    children = [json.loads(line[len('MOCK-CHILD '):]) for line in text.splitlines()
                if line.startswith('MOCK-CHILD ')]
    assert os.waitstatus_to_exitcode(status) == 0, text
    assert shell == ['MOCK-SHELL '+str(pid)] and len(children) == 1, text
    child = children[0]
    assert child['sid'] == pid
    traps = text.split('MOCK-TRAPS-BEGIN\n', 1)[1].split('MOCK-TRAPS-END\n', 1)[0]
    assert not traps.strip(), 'DEBUG hook remained installed'
    assert 'child setpgid' not in text, text
    if enable_monitor:
        assert child['pgid'] == child['pid'] and child['pgid'] != pid, child
    else:
        assert child['pgid'] == pid, child
        assert child['icanon'] is False and child['echo'] is False and child['icrnl'] is True, child
    return dict(passed=True, seconds=time.monotonic()-started,
                shell_pid=pid, child=child, self_removing_hook=True,
                explicit_monitor_reenable=enable_monitor)


def protection(stage, suite, out, shell_results, reference_results):
    e = stage.execution
    if shell_results is None or reference_results is None:
        raise ValueError('passing shell and actual reference functional receipts required')
    shell, reference = e.bounded_json(shell_results), e.bounded_json(reference_results)
    if not (shell.get('passed') is True and len(shell.get('cases', [])) == 2
            and all(row['passed'] for row in shell['cases'])
            and shell.get('mock_transport', {}).get('sha256') == suite.MOCK_SHELL_CONTENT_SHA256
            and reference.get('passed') is True and reference.get('actual_editor_execution') is True
            and reference.get('full_matrix_admission') is False and all(reference['checks'].values())):
        raise ValueError('current successful functional receipts required')
    pins = {str(path):e.file_identity(path) for path in (shell_results, reference_results)}
    original_input = Path(reference['reference_diagnostic_input']['path'])
    def bounded_not_formal():
        accounting = load('mock_shell_accounting', ROOT/'tools/stable-classification.py')
        try:
            accounting._remote_collections([(reference_results, out/'unused-controller')])
        except ValueError as error:
            assert str(error) == 'fresh completed remote behavior collection required', str(error)
        else:
            raise AssertionError('reference-only diagnostic entered matrix classification')
        return dict(actual_editor_diagnostic_rejected_as_matrix=True)
    def rejected_cli(name, extra, reason):
        target = out/(name+'-output')
        result = subprocess.run([sys.executable, ROOT/'tools/runtime-tramp-process-probe.py',
            '--reference-inputs', original_input, '--out', target, *extra],
            capture_output=True, text=True, timeout=5)
        assert result.returncode != 0 and reason in result.stderr, result.stderr
        assert not target.exists()
        return dict(returncode=result.returncode, output_absent=True)
    def changed_installation():
        value = e.bounded_json(original_input)
        value['upstream']['commit'] = '0'*40
        altered = out/'altered-reference-inputs.json'
        e.write_json(altered, value)
        target = out/'altered-input-output'
        result = subprocess.run([sys.executable, ROOT/'tools/runtime-tramp-process-probe.py',
            '--reference-inputs', altered, '--out', target],capture_output=True,text=True,timeout=5)
        assert result.returncode != 0 and 'bounded reference installation input identity changed' in result.stderr
        assert not target.exists()
        return dict(changed_installation_provenance_rejected=True, output_absent=True)
    cases = [('reference-only-collection-is-not-matrix-proof', bounded_not_formal)]
    for name, args in [('build', ['--build', str(out/'missing-build')]),
                       ('selection', ['--reference-selection', str(out/'missing-selection')]),
                       ('profile', ['--profile', 'cocoa-native-normal']),
                       ('include-reference', ['--include-reference'])]:
        cases.append(('mixed-'+name+'-rejected', lambda n=name,a=args:rejected_cli(
            n,a,'reference-inputs selects one reference diagnostic without matrix/profile options')))
    cases.append(('changed-installation-provenance-rejected', changed_installation))
    rows = []
    for name, body in cases:
        started = time.monotonic()
        try:
            row = dict(id=name, passed=True, details=body())
        except Exception as error:
            row = dict(id=name, passed=False, error=f'{type(error).__name__}: {error}')
        rows.append(dict(row, seconds=time.monotonic()-started))
    assert all(e.file_identity(Path(path)) == pin for path,pin in pins.items())
    e.write_json(out/'results.json', dict(schema='femacs-mock-shell-reference-protection-v1',
        passed=all(row['passed'] for row in rows), cases=rows,
        functional_acceptance_sha256={path:pin['sha256'] for path,pin in pins.items()},
        actual_editor_execution=False, host_configuration_modified=False, goal_complete=False))
    return 0 if all(row['passed'] for row in rows) else 1


def main():
    import argparse
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out', type=Path, required=True)
    parser.add_argument('--mode', choices=('integration', 'protection'), default='integration')
    parser.add_argument('--functional', type=Path)
    parser.add_argument('--reference-functional', type=Path)
    args = parser.parse_args()
    stage = load('mock_shell_stage', ROOT/'tools/runtime-stage.py')
    suite = stage.execution.load_tool('stable-suite.py')
    out = stage.execution.new_output(args.out)
    if args.mode == 'protection':
        return protection(stage, suite, out, args.functional, args.reference_functional)
    rows = []
    for name, monitor in [('owned-initialization', False), ('explicit-job-control-positive-control', True)]:
        started = time.monotonic()
        try:
            row = dict(id=name, **shell_case(stage, suite, out/name, monitor))
        except Exception as error:
            row = dict(id=name, passed=False, error=f'{type(error).__name__}: {error}',
                       seconds=time.monotonic()-started)
        rows.append(row)
    version = subprocess.run(['/bin/sh', '--version'], capture_output=True, text=True, timeout=2)
    stage.execution.write_json(out/'results.json', dict(schema='femacs-stable-mock-shell-integration-v1',
        passed=all(row['passed'] for row in rows), cases=rows,
        shell_identity=stage.execution.file_identity('/bin/sh'), shell_version=version.stdout.splitlines()[0],
        mock_transport=dict(profile=suite.MOCK_SHELL_PROFILE, sha256=suite.MOCK_SHELL_CONTENT_SHA256),
        actual_shell_execution=True, actual_editor_execution=False, original_test_execution=False,
        host_configuration_modified=False, compatibility_accepted=False, goal_complete=False))
    return 0 if all(row['passed'] for row in rows) else 1


if __name__ == '__main__':
    raise SystemExit(main())
