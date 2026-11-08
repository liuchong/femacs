"""Pinned source-test context for installed products, never a product dependency."""
from __future__ import annotations

import hashlib
import json
import os
from pathlib import Path
import uuid

SELECTION = 'f23d2dde45c9bb0eb3dcd5757cbfc90a71eeeb36b40697590a7fc4ce1f65e999'
EXTRA = {'README', 'admin/cus-test.el', 'admin/unidata/NormalizationTest.txt',
         'nextstep/GNUstep/\x45macs.base/Resources/emacs.tiff'}


def metadata(api, repo, corpus):
    path = api._absolute(repo/'compatibility/stable-31.1/source-context.json', 'source context manifest')
    if path.stat().st_size > api.MAX_MANIFEST_BYTES:
        raise api.SuiteError('source context manifest exceeds bound')
    value = json.loads(path.read_text())
    if value.get('schema') != 1 or value.get('source') != corpus['source']:
        raise api.SuiteError('source context version differs from original corpus')
    entries = value.get('entries')
    if not isinstance(entries, list) or not entries:
        raise api.SuiteError('source context has no pinned entries')
    seen = set()
    for entry in entries:
        if set(entry) != {'path', 'mode', 'size', 'blob', 'sha256'}:
            raise api.SuiteError('invalid source context entry')
        name = api._relative_path(entry['path'], 'source context entry')
        if name in seen or not (name.startswith('src/') or name in EXTRA):
            raise api.SuiteError('duplicate or out-of-scope context entry')
        seen.add(name)
        if (entry['mode'] not in api.VALID_MODES or type(entry['size']) is not int
                or entry['size'] < 0 or not isinstance(entry['blob'], str)
                or not api.HEX40.fullmatch(entry['blob'])
                or not isinstance(entry['sha256'], str) or not api.HEX64.fullmatch(entry['sha256'])):
            raise api.SuiteError('invalid context file identity')
    identity = hashlib.sha256(json.dumps(entries, sort_keys=True, separators=(',', ':')).encode()).hexdigest()
    if (identity != SELECTION or value.get('selection_sha256') != identity
            or value.get('file_count') != len(entries)
            or value.get('total_bytes') != sum(entry['size'] for entry in entries)
            or not EXTRA.issubset(seen)):
        raise api.SuiteError('source context differs from complete pinned selection')
    return value, api._sha256_path(path)


def _verify(provider, root, merged, manifest_sha):
    root = provider.directory(root)
    receipt = root/'receipt.json'
    if receipt.is_symlink() or receipt.stat().st_size > provider.suite.MAX_MANIFEST_BYTES:
        raise provider.suite.SuiteError('invalid context cache receipt')
    value = json.loads(receipt.read_text())
    if (value.get('complete') is not True or value.get('context_manifest_sha256') != manifest_sha
            or value.get('commit') != merged['source']['commit']
            or value.get('selection_sha256') != SELECTION):
        raise provider.suite.SuiteError('context cache has no matching completed receipt')
    expected = {entry['path'] for entry in merged['entries']} | {'receipt.json'}
    actual = set()
    for base, dirs, files in os.walk(root, followlinks=False):
        for name in dirs + files:
            if (Path(base)/name).is_symlink():
                raise provider.suite.SuiteError('context cache contains a symlink')
        actual.update((Path(base)/name).relative_to(root).as_posix() for name in files)
    if actual != expected:
        raise provider.suite.SuiteError('context cache file set differs')
    for entry in merged['entries']:
        provider.local_input(root, entry['path'], entry, canonical_mode=True)
    return value


def prepare(api, repo, out, corpus, products):
    provider = api._corpus_provider(repo)
    context, manifest_sha = metadata(api, repo, corpus)
    merged = dict(source=corpus['source'], entries=[*corpus['entries'], *context['entries']])
    if len({entry['path'] for entry in merged['entries']}) != len(merged['entries']):
        raise api.SuiteError('context and test input paths overlap')
    cache_root = provider.directory(repo/'.femacs-cache/stable-context', create=True)
    destination = cache_root/(corpus['source']['commit']+'-'+SELECTION[:16])
    if not os.path.lexists(destination):
        stage = cache_root/('.'+destination.name+'-'+uuid.uuid4().hex)
        stage.mkdir()
        # The same pinned archive reader validates every selected Git blob and
        # SHA256, rejects extra test members and streams without a full archive.
        acquired = provider.archive_inputs(merged, stage)
        value = dict(schema=1, complete=True, commit=corpus['source']['commit'],
                     context_manifest_sha256=manifest_sha, selection_sha256=SELECTION,
                     acquisition=acquired)
        with (stage/'receipt.json').open('x') as stream:
            json.dump(value, stream, indent=2); stream.write('\n')
        _verify(provider, stage, merged, manifest_sha)
        if os.path.lexists(destination):
            raise api.SuiteError('context cache concurrently published; staging retained')
        stage.rename(destination)
    acquired = _verify(provider, destination, merged, manifest_sha)
    data = out/'source-context-data'
    data.mkdir()
    for entry in context['entries']:
        provider.local_input(destination, entry['path'], entry, data/entry['path'])
    contexts = {}
    for label, paths in products.items():
        root = out/'source-contexts'/label
        root.mkdir(parents=True)
        links = {name: data/name for name in ('src', 'admin', 'nextstep', 'README')}
        links.update(lisp=paths['lisp_root'], test=out/'test')
        for name, target in links.items():
            # Context links are test-only. Never insert them into a product.
            (root/name).symlink_to(os.path.relpath(target, root), target_is_directory=target.is_dir())
        paths['test_context'] = root
        contexts[label] = dict(root=str(root), links={name:os.readlink(root/name) for name in links})
    record = dict(manifest_sha256=manifest_sha, selection_sha256=SELECTION,
                  file_count=context['file_count'], bytes=context['total_bytes'],
                  acquisition=acquired, contexts=contexts)
    (out/'source-context.json').write_text(json.dumps(record, indent=2)+'\n')
    return context, record


def verify(api, repo, out, context, record):
    provider = api._corpus_provider(repo)
    if api._sha256_path(repo/'compatibility/stable-31.1/source-context.json') != record['manifest_sha256']:
        raise api.SuiteError('source context manifest changed during execution')
    data = out/'source-context-data'
    for entry in context['entries']:
        provider.local_input(data, entry['path'], entry, canonical_mode=True)
    for value in record['contexts'].values():
        root = Path(value['root'])
        if {path.name for path in root.iterdir()} != set(value['links']):
            raise api.SuiteError('source context link set changed')
        for name, target in value['links'].items():
            if not (root/name).is_symlink() or os.readlink(root/name) != target:
                raise api.SuiteError('source context link changed')
