"""Real process/filesystem cleanup integration; no editor acceptance claim."""
from pathlib import Path
import argparse
import gzip
import hashlib
import importlib.util
import json
import subprocess
import sys
import time


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
    report = dict(passed=True, source_sha256=source_sha, files=3, directories=5,
        ledger_rows=8, actual_process_reaped=True, actual_filesystem_cleanup=True,
        parent_inclusive=dict(files=3, directories=7, parent_directories=2,
            ledger_rows=10, ledger_schema=3, full_predelete_identities=True,
            evidence_unchanged=True, parents_absent=True, seconds=parent_seconds),
        failed_producer=dict(exit_code=failed_child.returncode, retained=True,
            seconds=time.monotonic() - failed_started),
        editor_accepted=False, seconds=time.monotonic() - started)
    (out / 'results.json').write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps(report))


if __name__ == '__main__':
    main()
