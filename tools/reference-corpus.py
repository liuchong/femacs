"""Acquire an unchanged, completely pinned stable corpus into an ignored cache.

The optional reference checkout is never consulted. Existing mapped inputs may
seed a fresh cache; otherwise a version-bound public archive is streamed. No
existing input/cache is overwritten or deleted, including failed acquisitions.
Consumers must copy verified inputs into their own acceptance outputs.
"""
from __future__ import annotations

import argparse
from datetime import datetime, timezone
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import stat
import sys
import tarfile
import urllib.parse
import urllib.request
import uuid

REPO = Path(__file__).resolve().parent.parent
CHUNK = 1024 * 1024
SELECTION = 'aa2c3767ac8fc2d43a279e73eb5d80f53ec66f807d083e363cbe15cbfaa4b6e0'
spec = importlib.util.spec_from_file_location('femacs_corpus_contract', REPO/'tools/stable-suite.py')
suite = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = suite
spec.loader.exec_module(suite)


def directory(path, *, create=False):
    path = Path(os.path.abspath(path))
    current = Path(path.anchor)
    for part in path.parts[1:]:
        current = current/part
        if create:
            current.mkdir(exist_ok=True)
        metadata = current.lstat()
        if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
            raise suite.SuiteError('cache/input directory is not a physical directory')
    return path


def validate_selection(value):
    value = suite._validate_manifest(value)
    selection = [{name:entry[name] for name in ('path','mode','size','blob','sha256')}
                 for entry in sorted(value['entries'], key=lambda entry:entry['path'])]
    identity = hashlib.sha256(json.dumps(selection, sort_keys=True, separators=(',',':'),
                                         ensure_ascii=False).encode()).hexdigest()
    if identity != SELECTION:
        raise suite.SuiteError('corpus selection differs from the complete pinned source set')
    upstream = urllib.parse.urlsplit(value['source']['upstream'])
    if (upstream.scheme != 'https' or upstream.hostname != 'github.com'
            or upstream.username or upstream.password or upstream.query or upstream.fragment
            or upstream.path.strip('/').split('/') != ['emacs-mirror','emacs']):
        raise suite.SuiteError('corpus upstream identity differs from the pinned source')
    return value


def metadata(path):
    path = suite._absolute(path, 'corpus manifest')
    with path.open('rb') as source:
        if os.fstat(source.fileno()).st_size > suite.MAX_MANIFEST_BYTES:
            raise suite.SuiteError('corpus manifest exceeds the supported bound')
        raw = source.read(suite.MAX_MANIFEST_BYTES + 1)
    if len(raw) > suite.MAX_MANIFEST_BYTES:
        raise suite.SuiteError('corpus manifest exceeds the supported bound')
    value = validate_selection(json.loads(raw))
    return value, hashlib.sha256(raw).hexdigest()


def transfer(stream, destination, entry):
    size = entry['size']
    blob = hashlib.sha1(('blob '+str(size)+'\0').encode())
    content, count = hashlib.sha256(), 0
    output = None
    try:
        if destination is not None:
            directory(destination.parent, create=True)
            output = destination.open('xb')
        for block in iter(lambda:stream.read(CHUNK), b''):
            count += len(block)
            if count > size:
                raise suite.SuiteError('upstream input is larger than its pinned size: '+entry['path'])
            blob.update(block)
            content.update(block)
            if output is not None:
                output.write(block)
        if count != size or blob.hexdigest() != entry['blob'] or content.hexdigest() != entry['sha256']:
            raise suite.SuiteError('upstream input identity differs: '+entry['path'])
        if output is not None:
            output.flush()
            os.fchmod(output.fileno(), suite.VALID_MODES[entry['mode']])
    finally:
        if output is not None:
            output.close()


def local_input(root, relative, entry, destination=None, *, canonical_mode=False):
    path = suite._absolute(root/relative, 'corpus input')
    before = path.lstat()
    if not stat.S_ISREG(before.st_mode) or before.st_size != entry['size']:
        raise suite.SuiteError('upstream input mode/size differs: '+entry['path'])
    suite._check_mode(before.st_mode, entry['mode'], 'upstream input')
    if canonical_mode and stat.S_IMODE(before.st_mode) != suite.VALID_MODES[entry['mode']]:
        raise suite.SuiteError('cache input permissions are not canonical: '+entry['path'])
    with path.open('rb') as source:
        opened = os.fstat(source.fileno())
        if (opened.st_dev, opened.st_ino) != (before.st_dev, before.st_ino):
            raise suite.SuiteError('upstream input changed before reading')
        transfer(source, destination, entry)
        after = os.fstat(source.fileno())
    if (before.st_dev, before.st_ino, before.st_size, before.st_mode, before.st_mtime_ns) != (
            after.st_dev, after.st_ino, after.st_size, after.st_mode, after.st_mtime_ns):
        raise suite.SuiteError('upstream input changed during reading')


def verify_cache(root, manifest):
    manifest = validate_selection(manifest)
    root = directory(root)
    if suite._inside(root, REPO/'emacs'):
        raise suite.SuiteError('source cache is under the optional reference checkout')
    receipt_path = suite._absolute(root/'receipt.json', 'corpus cache receipt')
    observed = receipt_path.lstat()
    if not stat.S_ISREG(observed.st_mode) or observed.st_size > suite.MAX_MANIFEST_BYTES:
        raise suite.SuiteError('cache receipt is not a bounded regular file')
    receipt = json.loads(receipt_path.read_text())
    if (receipt.get('schema') != 1 or receipt.get('complete') is not True
            or receipt.get('commit') != manifest['source']['commit']
            or receipt.get('selection_sha256') != SELECTION):
        raise suite.SuiteError('cache has no matching complete source receipt')
    expected = {entry['path'] for entry in manifest['entries']} | {'receipt.json'}
    actual = set()
    for base, dirs, files in os.walk(root, followlinks=False):
        for name in dirs + files:
            path = Path(base)/name
            if path.is_symlink():
                raise suite.SuiteError('source cache contains a symlink')
        actual.update((Path(base)/name).relative_to(root).as_posix() for name in files)
    if actual != expected:
        raise suite.SuiteError('cache file set differs from the complete pinned selection')
    for entry in manifest['entries']:
        local_input(root, entry['path'], entry, canonical_mode=True)
    return dict(file_count=manifest['file_count'], lisp_file_count=manifest['lisp_file_count'],
                bytes=manifest['total_bytes'])


class Download:
    """Bound reads without keeping the compressed archive or member inventory."""
    def __init__(self, source):
        self.source = source
        self.sha256 = hashlib.sha256()
        self.bytes = 0

    def read(self, size=CHUNK):
        block = self.source.read(min(size if size >= 0 else CHUNK, CHUNK))
        self.sha256.update(block)
        self.bytes += len(block)
        return block


def archive_inputs(manifest, stage):
    source = manifest['source']
    location = urllib.parse.urlsplit(source['upstream']).path.strip('/')
    url = 'https://codeload.github.com/'+location+'/tar.gz/'+source['commit']
    wanted = {entry['path']:entry for entry in manifest['entries']}
    seen, prefix = set(), None
    request = urllib.request.Request(url, headers={'User-Agent':'version-bound-corpus-provider'})
    with urllib.request.urlopen(request, timeout=60) as response:
        if urllib.parse.urlsplit(response.url).hostname != 'codeload.github.com':
            raise suite.SuiteError('archive redirected away from the pinned archive host')
        download = Download(response)
        with tarfile.open(fileobj=download, mode='r|gz') as archive:
            for member in archive:
                parts = member.name.split('/', 1)
                if prefix is None:
                    prefix = parts[0]
                if parts[0] != prefix:
                    raise suite.SuiteError('archive has multiple top-level source roots')
                relative = parts[1] if len(parts) == 2 else ''
                if relative in wanted:
                    entry = wanted[relative]
                    if relative in seen or not member.isfile() or member.size != entry['size']:
                        raise suite.SuiteError('archive input type/identity differs: '+relative)
                    with archive.extractfile(member) as stream:
                        transfer(stream, stage/relative, entry)
                    seen.add(relative)
                elif relative.startswith('test/') and not member.isdir():
                    raise suite.SuiteError('upstream test tree contains an unlisted input: '+relative)
                archive.members.clear()
        for block in iter(lambda:download.read(CHUNK), b''):
            pass
        if seen != set(wanted):
            raise suite.SuiteError('archive omits pinned source inputs')
        return dict(kind='pinned-public-archive', url=url, compressed_bytes=download.bytes,
                    compressed_sha256=download.sha256.hexdigest())


def prepare(manifest_path, cache_root, *, seed_repo=None, source='auto'):
    if source not in ('auto','local','archive'):
        raise suite.SuiteError('unsupported corpus acquisition source')
    manifest, manifest_sha = metadata(manifest_path)
    candidate = Path(os.path.abspath(cache_root))
    if suite._inside(candidate, REPO/'emacs'):
        raise suite.SuiteError('source cache is under the optional reference checkout')
    cache_root = directory(cache_root, create=True)
    key = manifest['source']['commit']+'-'+SELECTION[:16]
    destination = cache_root/key
    if os.path.lexists(destination):
        return dict(path=str(destination), reused=True, **verify_cache(destination, manifest))
    seed = directory(seed_repo if seed_repo is not None else REPO)
    seed_available = all((seed/entry['source_path']).is_file() for entry in manifest['entries'])
    if source == 'local' and not seed_available:
        raise suite.SuiteError('complete mapped local corpus is unavailable')
    stage = cache_root/('.'+key+'-'+uuid.uuid4().hex)
    stage.mkdir()
    try:
        if source == 'local' or (source == 'auto' and seed_available):
            for entry in manifest['entries']:
                local_input(seed, entry['source_path'], entry, stage/entry['path'])
            acquired = dict(kind='verified-local-manifest-inputs')
        else:
            acquired = archive_inputs(manifest, stage)
        receipt = dict(schema=1, complete=True, commit=manifest['source']['commit'],
                       release=manifest['source']['release'], selection_sha256=SELECTION,
                       manifest_sha256=manifest_sha, acquisition=acquired,
                       file_count=manifest['file_count'], bytes=manifest['total_bytes'],
                       created_at=datetime.now(timezone.utc).isoformat())
        with (stage/'receipt.json').open('x') as output:
            json.dump(receipt, output, indent=2)
            output.write('\n')
        result = verify_cache(stage, manifest)
        if os.path.lexists(destination):
            raise suite.SuiteError('another acquisition already published this cache; staging retained')
        answer = dict(path=str(destination), reused=False, **result)
        stage.rename(destination)
        return answer
    except BaseException as error:
        with (stage/'failure.json').open('x') as output:
            json.dump(dict(complete=False, commit=manifest['source']['commit'], error_type=type(error).__name__,
                           error=str(error), created_at=datetime.now(timezone.utc).isoformat()), output, indent=2)
            output.write('\n')
        raise


def export_inputs(manifest_path, cache_root, *, repo=None):
    """Bind snapshot destinations to verified bytes without rewriting the map.

    Present mapped inputs must still match their declared identity. A verified
    cache is a source for absent ignored payloads, never a way to conceal edits
    to an existing input. The caller retains ordinary snapshot before/after
    identity checks on every returned physical file.
    """
    manifest, manifest_sha = metadata(manifest_path)
    repo = directory(repo if repo is not None else REPO)
    if suite._inside(repo, REPO/'emacs'):
        raise suite.SuiteError('snapshot repository is the optional reference checkout')
    acquired = prepare(manifest_path, cache_root, seed_repo=repo)
    cache = directory(acquired['path'])
    sources = {}
    for entry in manifest['entries']:
        mapped = repo/entry['source_path']
        if os.path.lexists(mapped):
            local_input(repo, entry['source_path'], entry)
            sources[entry['source_path']] = mapped
        else:
            sources[entry['source_path']] = cache/entry['path']
    return dict(manifest=manifest, manifest_sha256=manifest_sha,
                acquisition=acquired, sources=sources)


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--manifest', type=Path, default=REPO/'compatibility/stable-31.1/sources.json')
    parser.add_argument('--cache-root', type=Path, default=REPO/'.femacs-cache/stable-corpus')
    parser.add_argument('--seed-repo', type=Path)
    parser.add_argument('--source', choices=('auto','local','archive'), default='auto')
    options = parser.parse_args(argv)
    result = prepare(options.manifest, options.cache_root, seed_repo=options.seed_repo, source=options.source)
    print(json.dumps(result))
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
