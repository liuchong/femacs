"""Retire consumed, reaped case state with a durable pre-delete identity ledger.

The caller retains original inputs, outputs, failures and product receipts.
Only named children of its fresh owned case directory may be selected. Raw
machine paths remain private; public ledgers use relative identities.
"""
from pathlib import Path
import gzip
import hashlib
import json
import os
import stat
import subprocess


def digest(path):
    value = hashlib.sha256()
    with Path(path).open('rb') as stream:
        for block in iter(lambda: stream.read(262144), b''):
            value.update(block)
    return value.hexdigest()


def retire(root, names, evidence, process):
    """Require a successful reaped process; preserve failures and unknown scope."""
    root = Path(root).absolute()
    evidence = Path(evidence).absolute()
    if (process.get('complete') is not True or process.get('exit_code') != 0
            or process.get('remaining_group_terminated') is not False):
        return dict(status='retained', reason='process not successfully reaped')
    if (root.resolve() != root or not root.is_dir() or root.is_symlink()
            or evidence.is_symlink() or not evidence.is_file()
            or any(not isinstance(name, str) or Path(name).name != name
                   or name in ('', '.', '..') for name in names)):
        raise ValueError('unsafe owned-state retirement scope')
    check = subprocess.run(['lsof', '+D', str(root)], capture_output=True)
    if check.returncode != 1 or check.stdout.strip() or check.stderr.strip():
        return dict(status='retained', reason='case handles present or occupancy unknown')
    report = evidence.with_name(evidence.stem+'.state-retirement.json')
    ledger = evidence.with_name(evidence.stem+'.state-retirement.jsonl.gz')
    if report.exists() or ledger.exists():
        raise ValueError('retirement identity already exists; inspect original outcome')
    rows, directories = [], []
    for name in sorted(set(names)):
        child = root/name
        if not child.exists() and not child.is_symlink():
            continue
        if child.is_symlink() or not child.is_dir():
            raise ValueError('owned-state child is not an isolated directory')
        for parent, dirs, files in os.walk(child, topdown=False, followlinks=False):
            for leaf in files+dirs:
                path = Path(parent)/leaf
                metadata = path.lstat()
                if stat.S_ISDIR(metadata.st_mode):
                    continue
                if not stat.S_ISREG(metadata.st_mode) or getattr(metadata, 'st_flags', 0):
                    raise ValueError('unclassified or protected state retained')
                relative = path.relative_to(root).as_posix()
                rows.append(dict(path=relative, path_id_sha256=hashlib.sha256(relative.encode()).hexdigest(),
                    sha256=digest(path), bytes=metadata.st_size, allocated_bytes=metadata.st_blocks*512,
                    mode=stat.S_IMODE(metadata.st_mode), device=metadata.st_dev,
                    inode=metadata.st_ino, mtime_ns=metadata.st_mtime_ns))
            directories.append(Path(parent))
    original_evidence = digest(evidence)
    with ledger.open('xb') as raw:
        with gzip.GzipFile(filename='', mode='wb', fileobj=raw, mtime=0) as stream:
            for row in rows:
                # Never publish machine-encoded compiler/cache leaf names.
                public = {key: value for key, value in row.items()
                          if key not in ('path', 'device', 'inode', 'mtime_ns')}
                stream.write((json.dumps(public, sort_keys=True)+'\n').encode())
        raw.flush()
        os.fsync(raw.fileno())
    private = report.with_suffix('.private.json')
    with private.open('x') as stream:
        json.dump(dict(root=str(root), files=rows, evidence_sha256=original_evidence), stream)
        stream.flush()
        os.fsync(stream.fileno())
    fd = os.open(report.parent, os.O_RDONLY)
    try:
        os.fsync(fd)
    finally:
        os.close(fd)
    if digest(evidence) != original_evidence:
        raise ValueError('case evidence changed before retirement')
    for row in rows:
        path = root/row['path']
        metadata = path.lstat()
        if ((metadata.st_dev, metadata.st_ino, metadata.st_mtime_ns, metadata.st_size,
             stat.S_IMODE(metadata.st_mode)) !=
            (row['device'], row['inode'], row['mtime_ns'], row['bytes'], row['mode'])
                or path.is_symlink() or digest(path) != row['sha256']):
            raise ValueError('owned state changed before deletion')
        path.unlink()
    for directory in directories:
        directory.rmdir()
    if digest(evidence) != original_evidence:
        raise ValueError('case evidence changed during retirement')
    result = dict(status='complete', files=len(rows),
        logical_bytes=sum(row['bytes'] for row in rows),
        allocated_bytes=sum(row['allocated_bytes'] for row in rows),
        evidence_sha256=original_evidence, ledger=ledger.name,
        ledger_sha256=digest(ledger), all_selected_absent=True,
        restore='regenerate consumed cache from retained inputs; original random names not promised')
    with report.open('x') as stream:
        json.dump(result, stream, indent=2)
        stream.flush()
        os.fsync(stream.fileno())
    return result
