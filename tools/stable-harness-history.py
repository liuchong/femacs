#!/usr/bin/env python3
"""Resolve a historical receipt's harness roles to Git-recoverable bytes.

A closed run records the SHA-256 of every harness file it executed. Re-hashing
the current working tree against those values fails as soon as the tools change,
which is correct for a new run but says nothing about the historical fact.

This tool answers only that historical question: for each recorded role, is
there a Git object whose bytes hash to the recorded SHA-256? It reads Git, never
the working tree, never rewrites the receipt, and reports every role it cannot
resolve as an explicit gap. It grants nothing to a new run: a gap is a gap, and
the caller must not turn this into a hash exemption.
"""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import sys


def git(*args: str, cwd: Path, binary: bool = False):
    result = subprocess.run(['git', *args], cwd=cwd, capture_output=True)
    if result.returncode:
        return None
    return result.stdout if binary else result.stdout.decode().strip()


def commits_for(repo: Path, path: str) -> list[str]:
    out = git('rev-list', '--all', '--', path, cwd=repo)
    return out.split() if out else []


def repo_relative(repo: Path, path: str) -> str | None:
    """Map a recorded absolute path onto the repository, or None if outside it.

    A role recorded outside the repository cannot be resolved from Git history,
    and is reported as a gap rather than looked up by name elsewhere.
    """
    candidate = Path(path)
    try:
        relative = candidate.resolve().relative_to(repo)
    except (OSError, ValueError):
        return None
    return relative.as_posix()


def resolve_role(repo: Path, path: str, sha256: str) -> dict:
    """Find a commit whose version of `path` hashes to `sha256`."""
    relative = repo_relative(repo, path)
    if relative is None:
        return dict(path=path, sha256=sha256, source='gap',
                    reason='recorded path is outside this repository')
    for commit in commits_for(repo, relative):
        blob = git('rev-parse', f'{commit}:{relative}', cwd=repo)
        if not blob:
            continue
        data = git('cat-file', '-p', blob, cwd=repo, binary=True)
        if data is None:
            continue
        if hashlib.sha256(data).hexdigest() == sha256:
            return dict(path=path, sha256=sha256, source='git', commit=commit, blob=blob)
    return dict(path=path, sha256=sha256, source='gap',
                reason='no Git object for this path hashes to the recorded SHA-256')


def snapshot_roles(paths: list[Path]) -> dict:
    """Read recorded read-only harness snapshots into source-path -> captured file.

    Each snapshot is a `stable-harness-selection.json` that already records, per
    role, the original source path, its identity, and the captured copy. The
    captured copy is used only when its bytes still match the recorded identity,
    so a drifted or edited snapshot cannot stand in for history.
    """
    resolved = {}
    for selection in paths:
        try:
            document = json.loads(selection.resolve().read_text(encoding='utf-8'))
        except (OSError, UnicodeError, json.JSONDecodeError) as error:
            raise ValueError(f'cannot read harness snapshot {selection}: {error}') from error
        # A runner selection records one role under source_runner/captured_runner;
        # a harness selection records the rest under a files map. Both are
        # recorded provenance with the same source-identity contract.
        if 'source_runner' in document or 'captured_runner' in document:
            entry = dict(source=document.get('source_runner'),
                         captured=document.get('captured_runner'))
            files = {'runner': entry}
        else:
            files = document.get('files')
            if not isinstance(files, dict):
                raise ValueError(f'harness snapshot {selection} has no files map')
        for entry in files.values():
            try:
                source_path = entry['source']['path']
                expected = entry['source']['identity']['sha256']
                captured = Path(entry['captured']['path'])
            except (KeyError, TypeError) as error:
                raise ValueError(f'harness snapshot {selection} has an incomplete role') from error
            if not captured.is_file() or captured.is_symlink():
                raise ValueError(f'captured harness copy is missing or linked: {captured}')
            observed = hashlib.sha256(captured.read_bytes()).hexdigest()
            if observed != expected:
                raise ValueError(f'captured harness copy no longer matches its identity: {captured}')
            resolved.setdefault(source_path, captured)
    return resolved


def resolve(repo: Path, receipt: dict, snapshots: dict | None = None) -> dict:
    sources = receipt.get('harness_sources')
    if not isinstance(sources, dict) or not sources:
        raise ValueError('receipt has no harness_sources')
    roles = []
    for path, sha256 in sorted(sources.items()):
        if not isinstance(sha256, str) or len(sha256) != 64:
            raise ValueError(f'invalid recorded harness SHA-256 for {path}')
        role = resolve_role(repo, path, sha256)
        if role['source'] != 'git' and snapshots and path in snapshots:
            captured = snapshots[path]
            if hashlib.sha256(captured.read_bytes()).hexdigest() == sha256:
                role = dict(path=path, sha256=sha256, source='snapshot', captured=str(captured))
        roles.append(role)
    gaps = [role for role in roles if role['source'] != 'git']
    return dict(schema='femacs-stable-harness-history-v1',
                receipt=receipt.get('detailed', {}).get('path'),
                roles=roles, gaps=gaps, complete=not gaps,
                note='historical identity only; this document never admits a new run or exempts a hash')


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--receipt', type=Path, required=True,
                        help='closed stable-suite-run.json whose harness_sources are resolved')
    parser.add_argument('--repo', type=Path, default=Path(__file__).resolve().parent.parent)
    parser.add_argument('--out', type=Path, required=True)
    parser.add_argument('--snapshot', type=Path, action='append', default=[],
                        help='recorded stable-harness-selection.json whose captured copies are '
                             'verified against their own recorded identities before use')
    options = parser.parse_args(argv)
    repo = options.repo.resolve()
    try:
        receipt = json.loads(options.receipt.resolve().read_text(encoding='utf-8'))
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        print(f'stable-harness-history: cannot read receipt: {error}', file=sys.stderr)
        return 2
    try:
        snapshots = snapshot_roles(options.snapshot)
        report = resolve(repo, receipt, snapshots)
    except ValueError as error:
        print(f'stable-harness-history: {error}', file=sys.stderr)
        return 2
    out = options.out
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    print(json.dumps(dict(complete=report['complete'], roles=len(report['roles']),
                          gaps=[role['path'] for role in report['gaps']])))
    return 0 if report['complete'] else 1


if __name__ == '__main__':
    sys.exit(main())
