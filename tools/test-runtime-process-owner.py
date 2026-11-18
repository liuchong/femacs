"""Real process-parent loss and artifact-release integration, without a VM build."""
import argparse
import importlib.util
import json
import os
from pathlib import Path
from types import SimpleNamespace
import subprocess
import sys
import tempfile
import time
from unittest import mock

import psutil

ROOT = Path(__file__).resolve().parent.parent


def load(path, name):
    spec = importlib.util.spec_from_file_location(name, path)
    value = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(value)
    return value


WORK = '''import json, os, signal, subprocess, sys, time
from pathlib import Path
import psutil
root=Path(sys.argv[1]); nested=sys.argv[2].startswith('new-session')
grandchild=subprocess.Popen([sys.executable,'-B','-c','import time; time.sleep(60)'],
 start_new_session=nested)
def stop(number,frame):
 grandchild.terminate();grandchild.wait(timeout=2)
 (root/'worker-unwound').write_text('grandchild reaped before worker exit')
 raise SystemExit(128+number)
if not nested:signal.signal(signal.SIGTERM,stop)
if sys.argv[2]=='uncooperative':signal.signal(signal.SIGTERM,signal.SIG_IGN)
(root/'ready.json').write_text(json.dumps(dict(pid=os.getpid(),birth=psutil.Process().create_time(),
 grandchild_pid=grandchild.pid,grandchild_birth=psutil.Process(grandchild.pid).create_time())))
time.sleep(60)
'''

COORDINATOR = '''import importlib.util, json, os, sys, time
from pathlib import Path
def load(path,name):
 spec=importlib.util.spec_from_file_location(name,path)
 value=importlib.util.module_from_spec(spec);spec.loader.exec_module(value);return value
root=Path(sys.argv[1]); tools=Path(sys.argv[2]); mode=sys.argv[3]
e=load(tools/'runtime-execution.py','owned_execution')
owner=load(tools/'runtime-owner.py','owned_artifacts')
repo=root/'repo'; out=repo/'build/stage'; out.mkdir(parents=True)
marker=out/'input';marker.write_bytes(b'protected owned input')
code=0
try:
 with owner.ArtifactRun(repo,out) as artifacts:
  owner.capture_artifacts(artifacts.journal,{str(marker):e.file_identity(marker)})
  if mode=='before-spawn':
   (root/'ready.json').write_text(json.dumps(dict(pid=os.getpid(),grandchild_pid=None)))
   deadline=time.monotonic()+5
   while os.getppid()==e._RUN_OWNER_PARENT_PID and time.monotonic()<deadline:time.sleep(.01)
  e.run_process([sys.executable,'-B',root/'worker.py',str(root),mode],out/'process',os.environ,30,
   metrics='metrics' in mode,observe_exit='metrics' in mode)
  (root/'later-work-started').write_text('unexpected')
except SystemExit as failure:
 code=failure.code
finally:
 (root/'coordinator-exit.json').write_text(json.dumps(dict(exit_code=code)))
raise SystemExit(code)
'''

PARENT = '''import json, subprocess, sys, time
from pathlib import Path
import psutil
root=Path(sys.argv[1])
child=subprocess.Popen([sys.executable,'-B',root/'coordinator.py',str(root),sys.argv[2],sys.argv[3]])
(root/'coordinator-pid.json').write_text(json.dumps(dict(pid=child.pid,birth=psutil.Process(child.pid).create_time())))
sys.stdin.buffer.read(1)
'''


def wait_for(predicate, message, timeout=8):
    deadline = time.monotonic()+timeout
    while time.monotonic() < deadline:
        if predicate():
            return
        time.sleep(.01)
    raise AssertionError(message)


def exited(pid, birth):
    try:
        p = psutil.Process(pid)
        return p.create_time() != birth or not p.is_running() or p.status() == psutil.STATUS_ZOMBIE
    except psutil.NoSuchProcess:
        return True


def parent_loss(root, mode):
    root.mkdir()
    started = time.monotonic()
    for name, text in (('worker.py', WORK), ('coordinator.py', COORDINATOR), ('parent.py', PARENT)):
        (root/name).write_text(text)
    with (root/'stdout').open('xb') as stdout, (root/'stderr').open('xb') as stderr:
        parent = subprocess.Popen([sys.executable, '-B', root/'parent.py', str(root),
                                   str(ROOT/'tools'), mode], stdin=subprocess.PIPE,
                                  stdout=stdout, stderr=stderr)
        try:
            wait_for(lambda:(root/'ready.json').exists(), 'owned workload did not become ready')
            ready = json.loads((root/'ready.json').read_text())
            coordinator = json.loads((root/'coordinator-pid.json').read_text())
            out = root/'repo/build/stage'
            assert (out/'input').stat().st_flags & 2, 'input must be protected during work'
            parent.stdin.close()
            parent.wait(timeout=5)
            assert parent.returncode == 0
            wait_for(lambda:(root/'coordinator-exit.json').exists(), 'orphaned coordinator did not unwind',
                     timeout=35 if mode == 'uncooperative' else 8)
            assert json.loads((root/'coordinator-exit.json').read_text())['exit_code'] == 125
            wait_for(lambda:exited(coordinator['pid'], coordinator['birth']), 'coordinator remained alive')
            row = json.loads((out/'process/process.json').read_text())
            assert row['owner_parent_lost'] is True and row['complete'] is False
            assert row['cancelled'] is True and row['timed_out'] is False
            assert row['status'] == 'cancelled' and row['owner_parent_pid'] == parent.pid
            assert not (root/'later-work-started').exists()
            assert not (out/'input').stat().st_flags & 2
            journal = json.loads((out/'artifact-preservation.json').read_text())
            assert journal['state'] == 'released' and journal['release_failures'] == []
            if mode == 'before-spawn':
                assert 'pid' not in row and row['exit_code'] is None
            else:
                assert row['pid'] == ready['pid']
                if mode == 'uncooperative':
                    assert row['exit_code'] == -9 and not (root/'worker-unwound').exists()
                elif mode.startswith('new-session'):
                    assert row['exit_code'] == -15 and not (root/'worker-unwound').exists()
                else:
                    assert row['exit_code'] == 143 and (root/'worker-unwound').is_file()
                assert row['owned_process_cleanup']['complete'] is True
                assert row['owned_process_cleanup']['observed_processes'] >= 2
                assert exited(ready['pid'], ready['birth'])
                assert exited(ready['grandchild_pid'], ready['grandchild_birth'])
                if 'metrics' in mode:
                    assert row['memory_samples'] > 0 and row['exit_timing_complete'] is True
            return dict(id='parent-loss-'+mode, passed=True, seconds=time.monotonic()-started,
                        process=row, artifact_owner_released=True, later_work_started=False)
        except BaseException as failure:
            failure_path = root.parent.parent/(mode+'.failed-child.json')
            process_path = root/'repo/build/stage/process/process.json'
            with failure_path.open('x') as stream:
                json.dump(dict(error=f'{type(failure).__name__}: {failure}',
                    process=json.loads(process_path.read_text()) if process_path.exists() else None,
                    stdout=(root/'stdout').read_text()[-4096:],
                    stderr=(root/'stderr').read_text()[-4096:]), stream, indent=2)
                stream.write('\n')
            raise
        finally:
            if not parent.stdin.closed:
                parent.stdin.close()
            if parent.poll() is None:
                parent.terminate()
                parent.wait(timeout=3)
            # Exact PID/birth identities only, including cleanup on an assertion.
            for path, fields in ((root/'ready.json', (('pid','birth'),('grandchild_pid','grandchild_birth'))),
                                 (root/'coordinator-pid.json', (('pid','birth'),))):
                if not path.exists():
                    continue
                value = json.loads(path.read_text())
                for pid_key, birth_key in fields:
                    pid, birth = value.get(pid_key), value.get(birth_key)
                    if pid and birth and not exited(pid, birth):
                        process = psutil.Process(pid)
                        process.terminate()
                        try:
                            process.wait(timeout=3)
                        except psutil.TimeoutExpired:
                            if not exited(pid, birth):
                                process.kill()
                                process.wait(timeout=3)


def nested_deadline(e, root):
    root.mkdir()
    started = time.monotonic()
    (root/'worker.py').write_text(WORK)
    try:
        row = e.run_process([sys.executable, '-B', root/'worker.py', str(root), 'new-session'],
                            root/'process', os.environ, 1)
        ready = json.loads((root/'ready.json').read_text())
        assert row['timed_out'] is True and row['complete'] is False
        assert row['exit_code'] == -9 and row['status'] == 'timeout'
        assert not row.get('owner_parent_lost')
        assert row['owned_process_cleanup']['complete'] is True
        assert exited(ready['pid'], ready['birth'])
        assert exited(ready['grandchild_pid'], ready['grandchild_birth'])
        return dict(id='original-deadline-reaps-separate-session', passed=True,
                    seconds=time.monotonic()-started, process=row)
    finally:
        if (root/'ready.json').exists():
            ready = json.loads((root/'ready.json').read_text())
            for pid_key, birth_key in (('pid','birth'),('grandchild_pid','grandchild_birth')):
                if not exited(ready[pid_key], ready[birth_key]):
                    process = psutil.Process(ready[pid_key])
                    process.kill()
                    process.wait(timeout=3)


def protection(e, fixture):
    rows = [parent_loss(fixture/'uncooperative', 'uncooperative')]
    before = time.monotonic()
    output = fixture/'fork-refusal'
    output.mkdir()
    child = os.fork()
    if child == 0:
        try:
            e.run_process([sys.executable, '-B', '-c',
                'from pathlib import Path; import sys; Path(sys.argv[1]).touch()',
                str(fixture/'unexpected-worker')], output, os.environ, 2)
        except SystemExit as failure:
            os._exit(failure.code)
        except BaseException:
            os._exit(126)
        os._exit(0)
    deadline = time.monotonic()+5
    while time.monotonic() < deadline:
        pid, status = os.waitpid(child, os.WNOHANG)
        if pid:
            break
        time.sleep(.01)
    else:
        os.kill(child, 9)
        os.waitpid(child, 0)
        raise AssertionError('forked helper did not reject inherited ownership')
    assert os.waitstatus_to_exitcode(status) == 125
    row = e.bounded_json(output/'process.json')
    assert row['owner_parent_lost'] is True and row['complete'] is False
    assert 'pid' not in row and row['exit_code'] is None
    assert not (fixture/'unexpected-worker').exists()
    rows.append(dict(id='fork-cannot-inherit-coordinator-ownership', passed=True,
                     seconds=time.monotonic()-before))
    before = time.monotonic()
    for child_exited, persistent in ((False, False), (True, False), (True, True)):
        child = SimpleNamespace(pid=12345, poll=mock.Mock(return_value=0 if child_exited else None))
        effects = [PermissionError('owned group probe')]
        if child_exited:
            effects.append(PermissionError('persistent probe') if persistent else ProcessLookupError())
        with mock.patch.object(e.os, 'killpg', side_effect=effects) as send:
            if not child_exited or persistent:
                try:
                    e._signal_child_group(child, 9)
                except PermissionError:
                    pass
                else:
                    raise AssertionError('unknown/live permission failure was swallowed')
            else:
                assert e._signal_child_group(child, 9) is False
            assert send.call_count == (2 if child_exited else 1)
            child.poll.assert_called_once()
    rows.append(dict(id='group-permission-retry-requires-reaped-owned-child', passed=True,
                     seconds=time.monotonic()-before))
    before = time.monotonic()
    tree = e._OwnedProcessTree()
    tree.root = os.getpid()
    tree.known[os.getpid()] = psutil.Process().create_time()+1
    with mock.patch.object(psutil.Process, 'send_signal') as send:
        tree.signal(9)
        send.assert_not_called()
    assert tree.live() == []
    rows.append(dict(id='changed-pid-birth-never-grants-signal-authority', passed=True,
                     seconds=time.monotonic()-before))
    return rows


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out', required=True, type=Path)
    parser.add_argument('--mode', choices=('integration', 'protection'), default='integration')
    parser.add_argument('--functional', type=Path)
    args = parser.parse_args()
    e = load(ROOT/'tools/runtime-execution.py', 'lifetime_execution')
    functional_sha = None
    if args.mode == 'protection':
        if args.functional is None:
            parser.error('protection requires the completed functional receipt')
        functional = e.bounded_json(e.safe_path(args.functional), 65536)
        if (functional.get('schema') != 'femacs-runtime-process-owner-integration-v1'
                or functional.get('passed') is not True
                or functional.get('owned_fixture_retired') is not True
                or [r.get('id') for r in functional.get('cases', [])] != [
                    'parent-loss-wait', 'parent-loss-metrics', 'parent-loss-before-spawn',
                    'parent-loss-new-session', 'parent-loss-new-session-metrics',
                    'original-deadline-reaps-separate-session',
                    'live-parent-ordinary-exit1', 'original-deadline-preserved']):
            parser.error('all eight real functional flows must have passed')
        functional_sha = e.digest(args.functional)
    elif args.functional is not None:
        parser.error('functional binding is only for protection')
    out = e.safe_path(args.out, must_exist=False)
    if not any(out != base and out.is_relative_to(base)
               for base in (ROOT/'build', Path.home()/'AI/caches')):
        parser.error('fresh owned build/cache output required')
    out.parent.mkdir(parents=True, exist_ok=True)
    out.mkdir(mode=0o700)
    started = time.monotonic()
    rows, error = [], None
    try:
        with tempfile.TemporaryDirectory(prefix='owned-fixtures-', dir=out) as temporary:
            fixture = Path(temporary)
            if args.mode == 'protection':
                rows = protection(e, fixture)
                assert e.digest(args.functional) == functional_sha
                return
            rows.extend(parent_loss(fixture/mode, mode)
                        for mode in ('wait', 'metrics', 'before-spawn',
                                     'new-session', 'new-session-metrics'))
            rows.append(nested_deadline(e, fixture/'nested-deadline'))
            before = time.monotonic()
            ordinary = e.run_process([sys.executable, '-B', '-c', 'raise SystemExit(1)'],
                                     fixture/'ordinary', os.environ, 5, observe_exit=True)
            assert ordinary['complete'] is True and ordinary['exit_code'] == 1
            assert not ordinary.get('cancelled') and not ordinary.get('owner_parent_lost')
            assert ordinary['exit_timing_complete'] is True
            rows.append(dict(id='live-parent-ordinary-exit1', passed=True,
                             seconds=time.monotonic()-before))
            before = time.monotonic()
            timed = e.run_process([sys.executable, '-B', '-c', 'import time; time.sleep(5)'],
                                  fixture/'deadline', os.environ, .05)
            assert timed['complete'] is False and timed['timed_out'] is True
            assert not timed.get('owner_parent_lost') and timed['exit_code'] != 0
            rows.append(dict(id='original-deadline-preserved', passed=True,
                             seconds=time.monotonic()-before))
    except BaseException as failure:
        error = f'{type(failure).__name__}: {failure}'
        raise
    finally:
        e.write_json(out/'results.json', dict(schema=('femacs-runtime-process-owner-protection-v1'
                if args.mode == 'protection' else 'femacs-runtime-process-owner-integration-v1'),
            passed=error is None and len(rows) == (4 if args.mode == 'protection' else 8), cases=rows, error=error,
            functional_acceptance_sha256=functional_sha,
            execution_source_sha256=e.digest(ROOT/'tools/runtime-execution.py'),
            seconds=time.monotonic()-started, owned_fixture_retired=not Path(temporary).exists()
                if 'temporary' in locals() else False,
            actual_compiler_execution=False, actual_editor_execution=False,
            original_corpus_execution=False, goal_complete=False))


if __name__ == '__main__':
    main()
