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
    report = dict(passed=True, source_sha256=source_sha, files=3, directories=5,
        ledger_rows=8, actual_process_reaped=True, actual_filesystem_cleanup=True,
        editor_accepted=False, seconds=time.monotonic() - started)
    (out / 'results.json').write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps(report))


if __name__ == '__main__':
    main()
