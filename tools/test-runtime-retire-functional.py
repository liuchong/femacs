"""Real process/filesystem cleanup integration; no editor acceptance claim."""
from pathlib import Path
import argparse
import errno
import gzip
import hashlib
import importlib.util
import json
import os
import subprocess
import sys
import time
from unittest import mock


def occupancy_integration(retirement, out):
    rows = []
    root = out/'occupancy.noindex'
    root.mkdir()
    started = time.monotonic()
    clear = retirement.check_occupancy(root)
    assert clear['clear'] is True, clear
    rows.append(dict(id='actual-empty-query', passed=True,
                     seconds=time.monotonic()-started, observation=clear))
    started = time.monotonic()
    payload = root/'held'
    payload.write_bytes(b'owned live holder')
    with payload.open('rb'):
        occupied = retirement.check_occupancy(root)
    assert occupied['clear'] is False and occupied['holder_pid_count'] >= 1
    assert payload.read_bytes() == b'owned live holder'
    payload.unlink()
    rows.append(dict(id='actual-held-file-is-retained', passed=True,
                     seconds=time.monotonic()-started, observation=occupied))

    # Fault only a pipe belonging to the real lsof child, after Popen has
    # completed its own startup error-pipe handling.
    started = time.monotonic()
    source = '''from pathlib import Path
import sys
p=Path(sys.argv[1])/'cache'/'payload'
p.parent.mkdir();p.write_bytes(b'producer data')
'''
    producer = subprocess.run([sys.executable, '-B', '-c', source, str(root)],
                              capture_output=True, timeout=5)
    assert producer.returncode == 0, producer.stderr
    evidence = root/'result.json'
    process = dict(complete=True, exit_code=producer.returncode,
                   remaining_group_terminated=False)
    evidence.write_text(json.dumps(process)+'\n')
    popen, read = retirement.subprocess.Popen, retirement.os.read
    descriptors, children, faults = set(), [], []
    def spawn(*args, **kwargs):
        child = popen(*args, **kwargs)
        children.append(child)
        descriptors.update((child.stdout.fileno(), child.stderr.fileno()))
        return child
    def broken_read(descriptor, count):
        if descriptor in descriptors and not faults:
            faults.append(descriptor)
            raise OSError(errno.EIO, 'owned occupancy pipe read fault')
        return read(descriptor, count)
    with mock.patch.object(retirement.subprocess, 'Popen', side_effect=spawn), \
            mock.patch.object(retirement.os, 'read', side_effect=broken_read):
        retained = retirement.retire(root, ['cache'], evidence, process, record_occupancy=True)
    assert faults and children and all(child.poll() is not None for child in children)
    assert retained['status'] == 'retained'
    assert retained['attempt_observation']['clear'] is False
    assert retained['attempt_observation']['error_type'] == 'OSError'
    assert (root/'cache/payload').read_bytes() == b'producer data'
    assert not (root/'result.state-retirement.jsonl.gz').exists()
    assert not (root/'result.state-retirement.private.json').exists()
    assert not (root/'result.state-retirement.json').exists()
    completed = retirement.retire(root, ['cache'], evidence, process, record_occupancy=True)
    assert completed['status'] == 'complete' and completed['all_selected_absent'] is True
    rows.append(dict(id='real-query-read-fault-blocks-deletion', passed=True,
                     seconds=time.monotonic()-started, observation=retained['attempt_observation'],
                     original_data_retained=True, query_child_reaped=True,
                     fresh_clear_query_retired_state=True))
    return rows


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out', type=Path, required=True)
    options = parser.parse_args()
    out = options.out.absolute()
    if out.exists() or out.resolve() != out:
        parser.error('fresh owned output required')
    out.mkdir(parents=True)
    source = Path(__file__).with_name('runtime-retire.py')
    spec = importlib.util.spec_from_file_location('retire_functional', source)
    retirement = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(retirement)
    source_sha = retirement.digest(source)
    root = out / 'completed-case'
    root.mkdir()
    started = time.monotonic()
    writer = '''from pathlib import Path
import sys
r = Path(sys.argv[1])
for name, content in [('home/settings', b'config'),
                      ('cache/nested/unit', b'compiled fixture'),
                      ('tmp/empty-file', b'')]:
    p = r / name
    p.parent.mkdir(parents=True, exist_ok=True)
    p.write_bytes(content)
(r / 'cache/empty').mkdir()
'''
    child = subprocess.run([sys.executable, '-B', '-c', writer, str(root)],
                           capture_output=True, timeout=5)
    assert child.returncode == 0, child.stderr
    evidence = root / 'result.json'
    evidence.write_text(json.dumps(dict(complete=True, exit_code=child.returncode,
        remaining_group_terminated=False)) + '\n')
    evidence_sha = retirement.digest(evidence)
    before = {}
    for path in root.rglob('*'):
        if path.is_dir():
            node = path.lstat()
            before[path.relative_to(root).as_posix()] = (node.st_dev, node.st_ino)
    process = json.loads(evidence.read_text())
    result = retirement.retire(root, ['home', 'cache', 'tmp'], evidence, process)
    assert result['status'] == 'complete' and result['files'] == 3
    assert result['directories'] == len(before) == 5
    assert result['ledger_schema'] == 2
    private = json.loads(evidence.with_name('result.state-retirement.private.json').read_text())
    assert private['schema'] == 2 and len(private['files']) == 3
    assert {r['path']: (r['device'], r['inode']) for r in private['directories']} == before
    with gzip.open(root / result['ledger'], 'rt') as stream:
        public = [json.loads(line) for line in stream]
    assert len(public) == 8
    directory_rows = [r for r in public if r.get('kind') == 'directory']
    assert {r['path_id_sha256'] for r in directory_rows} == {
        hashlib.sha256(name.encode()).hexdigest() for name in before}
    assert all('path' not in r and 'inode' not in r for r in public)
    assert not any((root / leaf).exists() for leaf in ['home', 'cache', 'tmp'])
    assert retirement.digest(evidence) == evidence_sha
    assert retirement.digest(source) == source_sha

    # Exercise parent-inclusive retirement with a real, reaped producer and
    # evidence outside the removed container. Child-only callers above stay v2.
    parent_started = time.monotonic()
    container = out / 'owned-container'
    owned = container / 'case'
    owned.mkdir(parents=True)
    nodes = ((owned, owned.lstat()), (container, container.lstat()))
    producer = subprocess.run([sys.executable, '-B', '-c', writer, str(owned)],
                              capture_output=True, timeout=5)
    assert producer.returncode == 0, producer.stderr
    parent_evidence = out / 'parent-result.json'
    parent_evidence.write_text(json.dumps(dict(complete=True, exit_code=0,
        remaining_group_terminated=False)) + '\n')
    parent_evidence_sha = retirement.digest(parent_evidence)
    identities = {}
    for directory in [p for p in owned.rglob('*') if p.is_dir()] + [owned, container]:
        node = directory.lstat()
        relative = '..' if directory == container else directory.relative_to(owned).as_posix()
        identities[relative] = (node.st_dev, node.st_ino, node.st_mtime_ns, node.st_mode & 0o7777)
    parents = retirement.retire(owned, ['home', 'cache', 'tmp'], parent_evidence,
                               process, empty_parents=nodes)
    assert parents['status'] == 'complete' and parents['empty_parents_retired'] is True
    assert parents['files'] == 3 and parents['directories'] == 7
    assert parents['parent_directories'] == 2 and parents['ledger_schema'] == 3
    plan = json.loads((out / 'parent-result.state-retirement.private.json').read_text())
    assert plan['schema'] == 3
    assert {r['path']: (r['device'], r['inode'], r['mtime_ns'], r['mode'])
            for r in plan['directories']} == identities
    with gzip.open(out / parents['ledger'], 'rt') as stream:
        parent_public = [json.loads(line) for line in stream]
    assert len(parent_public) == 10
    assert sum(r.get('empty_parent', False) for r in parent_public) == 2
    assert all('path' not in r and 'inode' not in r for r in parent_public)
    assert retirement.digest(parent_evidence) == parent_evidence_sha
    assert not container.exists() and not owned.exists()
    parent_seconds = time.monotonic() - parent_started

    failed_started = time.monotonic()
    failed = out / 'failed-container' / 'case'
    failed.mkdir(parents=True)
    failed_child = subprocess.run([sys.executable, '-B', '-c',
        'from pathlib import Path; import sys; (Path(sys.argv[1])/"keep").write_bytes(b"failure"); sys.exit(1)',
        str(failed)], capture_output=True, timeout=5)
    failed_evidence = out / 'failed-result.json'
    failed_evidence.write_text(json.dumps(dict(complete=True, exit_code=failed_child.returncode,
        remaining_group_terminated=False)) + '\n')
    retained = retirement.retire(failed, [], failed_evidence,
        json.loads(failed_evidence.read_text()),
        empty_parents=((failed, failed.lstat()), (failed.parent, failed.parent.lstat())))
    assert failed_child.returncode == 1 and retained['status'] == 'retained'
    assert (failed / 'keep').read_bytes() == b'failure'
    assert not (out / 'failed-result.state-retirement.jsonl.gz').exists()
    assert retirement.digest(source) == source_sha
    occupancy = occupancy_integration(retirement, out)
    report = dict(passed=True, source_sha256=source_sha, files=3, directories=5,
        ledger_rows=8, actual_process_reaped=True, actual_filesystem_cleanup=True,
        parent_inclusive=dict(files=3, directories=7, parent_directories=2,
            ledger_rows=10, ledger_schema=3, full_predelete_identities=True,
            evidence_unchanged=True, parents_absent=True, seconds=parent_seconds),
        failed_producer=dict(exit_code=failed_child.returncode, retained=True,
            seconds=time.monotonic() - failed_started),
        occupancy=occupancy,
        editor_accepted=False, seconds=time.monotonic() - started)
    (out / 'results.json').write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps(report))


if __name__ == '__main__':
    main()
