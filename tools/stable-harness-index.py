#!/usr/bin/env python3
"""Verify a closed run's harness roles against repository-relative recovery sources.

The index committed next to the recovered sources records, per role, where the
bytes can still be obtained: a Git object (`git show <commit>:<path>`) or a file
kept in the repository. Verification always re-reads those bytes and recomputes
SHA-256; a previously generated report is never trusted, and a role whose source
is missing, changed or unreadable is rejected.

Index entries use repository-relative paths only. The absolute paths a closed
receipt recorded stay in that receipt; callers map them onto roles by file name.
"""
from __future__ import annotations
import hashlib
import json
from pathlib import Path
import subprocess


class HarnessIndexError(RuntimeError):
    """The index or a recovery source cannot satisfy the recorded identity."""


def _read_git(repo: Path, commit: str, path: str) -> bytes:
    result = subprocess.run(['git', 'show', f'{commit}:{path}'], cwd=repo, capture_output=True)
    if result.returncode:
        raise HarnessIndexError(f'Git object unavailable: {commit}:{path}')
    return result.stdout


def load_index(index_path) -> dict:
    path = Path(index_path)
    if path.is_symlink() or not path.is_file():
        raise HarnessIndexError('harness index must be a physical ordinary file')
    try:
        document = json.loads(path.read_text(encoding='utf-8'))
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        raise HarnessIndexError(f'harness index is unreadable: {error}') from None
    if document.get('schema') != 'femacs-stable-harness-index-v1':
        raise HarnessIndexError('harness index has an unknown schema')
    roles = document.get('roles')
    if not isinstance(roles, list) or not roles:
        raise HarnessIndexError('harness index has no roles')
    by_role = {}
    for entry in roles:
        if not isinstance(entry, dict):
            raise HarnessIndexError('harness index role must be an object')
        role = entry.get('role')
        recovery = entry.get('recovery')
        if not isinstance(role, str) or not role or role in by_role:
            raise HarnessIndexError('harness index roles must be unique names')
        if (not isinstance(entry.get('sha256'), str) or len(entry['sha256']) != 64
                or not isinstance(recovery, dict)):
            raise HarnessIndexError(f'harness index role {role} is incomplete')
        kind = recovery.get('kind')
        if kind == 'git':
            commit, path = recovery.get('commit'), recovery.get('path')
            if not isinstance(commit, str) or not isinstance(path, str) or not path:
                raise HarnessIndexError(f'harness index role {role} has an incomplete Git source')
            if path.startswith('/'):
                raise HarnessIndexError(f'harness index role {role} must use a repository-relative path')
        elif kind == 'file':
            path = recovery.get('path')
            if not isinstance(path, str) or not path:
                raise HarnessIndexError(f'harness index role {role} has no file path')
            if path.startswith('/'):
                raise HarnessIndexError(f'harness index role {role} must use a repository-relative path')
        else:
            raise HarnessIndexError(f'harness index role {role} has an unknown recovery kind')
        by_role[role] = entry
    return document


def verify(repo, index_path, recorded: dict[str, str]) -> dict:
    """Re-read every recorded role's bytes and check the recorded SHA-256."""
    repo = Path(repo).resolve()
    document = load_index(index_path)
    by_role = {entry['role']: entry for entry in document['roles']}
    verified = []
    problems = []
    for role, sha256 in sorted(recorded.items()):
        entry = by_role.get(role)
        if entry is None:
            problems.append('no index entry: ' + role)
            continue
        if entry['sha256'] != sha256:
            problems.append('index SHA differs from the receipt: ' + role)
            continue
        recovery = entry['recovery']
        try:
            if recovery['kind'] == 'git':
                data = _read_git(repo, recovery['commit'], recovery['path'])
            else:
                source = repo / recovery['path']
                if source.is_symlink() or not source.is_file():
                    raise HarnessIndexError('recovery file is missing or linked: ' + str(source))
                data = source.read_bytes()
        except (OSError, HarnessIndexError) as error:
            problems.append(f'{role}: {error}')
            continue
        observed = hashlib.sha256(data).hexdigest()
        if observed != sha256:
            problems.append(f'{role}: recovered bytes differ from the recorded SHA-256')
            continue
        verified.append(dict(role=role, source=recovery['kind'], sha256=observed,
                             path=recovery['path'], commit=recovery.get('commit')))
    if problems:
        raise HarnessIndexError('harness index cannot satisfy the receipt: ' + '; '.join(problems))
    return dict(schema='femacs-stable-harness-verification-v1', roles=verified)
