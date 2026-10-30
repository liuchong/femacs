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


def retire(root, names, evidence, process, *, empty_parents=()):
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
    parent_rows = []
    if empty_parents:
        # Only the two directories created by the per-file runner qualify.
        # Their original nodes prove ownership; fresh metadata records the
        # actual state after the completed process, before any deletion.
        if (len(empty_parents) != 2
                or tuple(Path(path) for path, _ in empty_parents) != (root, root.parent)
                or evidence.resolve().is_relative_to(root.parent)):
            raise ValueError('unsafe owned environment parents or evidence location')
        for (directory, original), relative in zip(empty_parents, ('.', '..')):
            directory = Path(directory)
            node = directory.lstat()
            if (directory.resolve() != directory or not stat.S_ISDIR(node.st_mode)
                    or getattr(node, 'st_flags', 0)
                    or (node.st_dev, node.st_ino) != (original.st_dev, original.st_ino)):
                raise ValueError('owned environment parent changed before retirement')
            parent_rows.append(dict(path=relative, kind='directory', empty_parent=True,
                path_id_sha256=hashlib.sha256(relative.encode()).hexdigest(),
                mode=stat.S_IMODE(node.st_mode), device=node.st_dev,
                inode=node.st_ino, mtime_ns=node.st_mtime_ns))
        if (any(child.name not in names for child in root.iterdir())
                or set(root.parent.iterdir()) != {root}):
            raise ValueError('unselected environment state retained')
    try:
        check = subprocess.run(['lsof', '+D', str(root.parent if parent_rows else root)],
                               capture_output=True, timeout=10)
    except subprocess.TimeoutExpired:
        return dict(status='retained', reason='case occupancy query timed out')
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
            directory = Path(parent)
            metadata = directory.lstat()
            if (not stat.S_ISDIR(metadata.st_mode)
                    or getattr(metadata, 'st_flags', 0)):
                raise ValueError('unclassified or protected directory retained')
            relative = directory.relative_to(root).as_posix()
            directories.append(dict(path=relative, kind='directory',
                path_id_sha256=hashlib.sha256(relative.encode()).hexdigest(),
                mode=stat.S_IMODE(metadata.st_mode), device=metadata.st_dev,
                inode=metadata.st_ino, mtime_ns=metadata.st_mtime_ns))
    directories.extend(parent_rows)
    schema = 3 if parent_rows else 2
    original_evidence = digest(evidence)
    with ledger.open('xb') as raw:
        with gzip.GzipFile(filename='', mode='wb', fileobj=raw, mtime=0) as stream:
            for row in [*rows, *directories]:
                # Never publish machine-encoded compiler/cache leaf names.
                public = {key: value for key, value in row.items()
                          if key not in ('path', 'device', 'inode', 'mtime_ns')}
                stream.write((json.dumps(public, sort_keys=True)+'\n').encode())
        raw.flush()
        os.fsync(raw.fileno())
    private = report.with_suffix('.private.json')
    with private.open('x') as stream:
        json.dump(dict(schema=schema, root=str(root), files=rows, directories=directories,
                       evidence_sha256=original_evidence), stream)
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
    for row in directories:
        # root/'..' no longer resolves after root itself has been removed.
        directory = root.parent if row.get('empty_parent') and row['path'] == '..' else root/row['path']
        metadata = directory.lstat()
        if (not stat.S_ISDIR(metadata.st_mode)
                or (metadata.st_dev, metadata.st_ino, stat.S_IMODE(metadata.st_mode)) !=
                (row['device'], row['inode'], row['mode'])
                or getattr(metadata, 'st_flags', 0)):
            raise ValueError('owned directory changed before deletion')
        directory.rmdir()
    if digest(evidence) != original_evidence:
        raise ValueError('case evidence changed during retirement')
    result = dict(status='complete', files=len(rows), directories=len(directories),
        ledger_schema=schema,
        logical_bytes=sum(row['bytes'] for row in rows),
        allocated_bytes=sum(row['allocated_bytes'] for row in rows),
        evidence_sha256=original_evidence, ledger=ledger.name,
        ledger_sha256=digest(ledger), all_selected_absent=True,
        restore='regenerate consumed cache from retained inputs; original random names not promised')
    if parent_rows:
        result.update(empty_parents_retired=True, parent_directories=len(parent_rows))
    with report.open('x') as stream:
        json.dump(result, stream, indent=2)
        stream.flush()
        os.fsync(stream.fileno())
    return result
