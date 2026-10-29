"""Version-bound stable-test corpus materialization and accounting helpers.

This module owns the deterministic source-corpus layer and the actual stable
31.1 execution bridge.  Materialization is always from the tracked/overlay
manifest. Execution mode follows the original file-entrypoint recipe in isolated
reference/candidate processes and runs all discovered IDs per file, with exact
per-case progress and deadlines. It retains all failures and does not make
a blanket full-compatibility claim.  Commands are run only by the final D harness.

Public helpers:

    load_manifest(path, repo) -> dict
    materialize(manifest, repo, out, *, corpus_root=None) -> {"file_count": int, "bytes": int}
    compare_registrations(reference, candidate) -> dict
    assess_case(expected_id, observed) -> dict
    classify_applicability(platform, required_platform, feature,
                           reference_features, candidate_features) -> dict
    case_environment(case_id, out) -> dict

The manifest is schema 1 for source release 31.1, commit
a360712c9d272d950d8d8255ef74570f7e90b7d9.  Entries map an original
``test/...`` path either to the identical tracked root path or to the
``compatibility/stable-31.1/test/...`` overlay path.  Sources are read only,
verified by mode, streamed size, Git blob SHA-1 and SHA-256, then copied in
bounded chunks.  The optional reference checkout is never consulted.

CLI materialize mode is the legacy no-product/default or explicit
``--materialize`` path.  Execution mode requires both ``--reference`` and
``--candidate``, optional ``--reference-dump-args``/``--candidate-dump-args``
JSON arrays, ``--timeout``, and a fresh ``--out``; it materializes all manifest
inputs, discovers ERT identities per original Lisp file in isolated processes,
compares generated IDs, and runs every reference ID by exact interned name in
fresh reference/candidate processes.  It retains logs and failures and never
declares blanket full compatibility.
"""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import math
import os
from pathlib import Path, PurePosixPath
import re
import signal
import stat
import subprocess
import sys
import time

SCHEMA = 1
RELEASE = '31.1'
COMMIT = 'a360712c9d272d950d8d8255ef74570f7e90b7d9'
OVERLAY_ROOT = 'compatibility/stable-31.1'
MAX_MANIFEST_BYTES = 16 * 1024 * 1024
CHUNK = 1024 * 1024
VALID_MODES = {'100644': 0o644, '100755': 0o755}
HEX40 = re.compile(r'^[0-9a-f]{40}$')
HEX64 = re.compile(r'^[0-9a-f]{64}$')
DRIVE = re.compile(r'^[A-Za-z]:')


class SuiteError(Exception):
    """Invalid corpus, unsafe path, changed input or result-accounting error."""


def _int(value, name, *, minimum=0):
    if isinstance(value, bool) or type(value) is not int or value < minimum:
        raise SuiteError('{0} must be an integer >= {1}'.format(name, minimum))
    return value


def _relative_path(value, name):
    if not isinstance(value, str) or not value or '\x00' in value:
        raise SuiteError('{0} must be a non-empty POSIX path'.format(name))
    if '\\' in value or value.startswith('/') or DRIVE.match(value):
        raise SuiteError('{0} must not be absolute, Windows or escaped'.format(name))
    if '//' in value or value.endswith('/'):
        raise SuiteError('{0} has an empty component'.format(name))
    parts = value.split('/')
    if any(part in ('', '.', '..') for part in parts):
        raise SuiteError('{0} has an unsafe component'.format(name))
    return value


def _original_path(value):
    value = _relative_path(value, 'entry.path')
    if not value.startswith('test/'):
        raise SuiteError('entry.path must be an original test/... path')
    return value


def _mapping(entry):
    original = _original_path(entry.get('path'))
    source = _relative_path(entry.get('source_path'), 'entry.source_path')
    kind = entry.get('input')
    if kind == 'tracked':
        expected = original
    elif kind == 'overlay':
        expected = OVERLAY_ROOT + '/' + original
    else:
        raise SuiteError('entry.input must be tracked or overlay')
    if source != expected:
        raise SuiteError('source_path does not match the {0} mapping'.format(kind))
    return original, source


def _validate_manifest(manifest):
    if not isinstance(manifest, dict):
        raise SuiteError('manifest must be an object')
    if type(manifest.get('schema')) is not int or manifest.get('schema') != SCHEMA:
        raise SuiteError('manifest schema must be 1')
    source = manifest.get('source')
    if not isinstance(source, dict):
        raise SuiteError('manifest source must be an object')
    if source.get('release') != RELEASE:
        raise SuiteError('source release must be 31.1')
    if source.get('commit') != COMMIT:
        raise SuiteError('source commit is not the exact stable 31.1 commit')
    entries = manifest.get('entries')
    if not isinstance(entries, list) or not entries:
        raise SuiteError('entries must be a non-empty list')
    file_count = _int(manifest.get('file_count'), 'file_count', minimum=1)
    lisp_count = _int(manifest.get('lisp_file_count'), 'lisp_file_count')
    total_bytes = _int(manifest.get('total_bytes'), 'total_bytes')
    seen = set()
    actual_lisp = 0
    actual_bytes = 0
    for index, entry in enumerate(entries):
        if not isinstance(entry, dict):
            raise SuiteError('entry {0} must be an object'.format(index))
        original, source_path = _mapping(entry)
        if original in seen:
            raise SuiteError('duplicate destination path: {0}'.format(original))
        seen.add(original)
        mode = entry.get('mode')
        if mode not in VALID_MODES:
            raise SuiteError('entry {0} has unsupported mode'.format(index))
        size = _int(entry.get('size'), 'entry.size')
        blob = entry.get('blob')
        digest = entry.get('sha256')
        if not isinstance(blob, str) or not HEX40.fullmatch(blob):
            raise SuiteError('entry {0} has invalid Git blob hash'.format(index))
        if not isinstance(digest, str) or not HEX64.fullmatch(digest):
            raise SuiteError('entry {0} has invalid SHA-256'.format(index))
        actual_bytes += size
        if original.endswith('.el'):
            actual_lisp += 1
    if file_count != len(entries):
        raise SuiteError('file_count does not match entries')
    if lisp_count != actual_lisp:
        raise SuiteError('lisp_file_count does not match entries')
    if total_bytes != actual_bytes:
        raise SuiteError('total_bytes does not match entries')
    return manifest


def _absolute(path, role):
    try:
        absolute = Path(os.path.abspath(os.fspath(path)))
    except (TypeError, OSError) as exc:
        raise SuiteError('invalid {0} path: {1}'.format(role, exc)) from exc
    _reject_symlink_components(absolute, role)
    return absolute


def _reject_symlink_components(path, role):
    current = Path(path.anchor)
    for part in path.parts[1:]:
        current = current / part
        try:
            observed = os.lstat(current)
        except OSError as exc:
            raise SuiteError('cannot inspect {0} component: {1}'.format(role, exc)) from exc
        if stat.S_ISLNK(observed.st_mode):
            raise SuiteError('{0} must not contain symlinks'.format(role))
        if current != path and not stat.S_ISDIR(observed.st_mode):
            raise SuiteError('{0} has a non-directory parent'.format(role))


def _inside(path, root):
    try:
        path.relative_to(root)
        return True
    except ValueError:
        return False


def _message_for_io(action, path, exc):
    return SuiteError('{0} failed for {1}: {2}'.format(action, path, exc))


def load_manifest(path, repo):
    """Read and validate the stable corpus manifest without reading sources."""
    repo_path = _absolute(repo, 'repository')
    if not repo_path.is_dir():
        raise SuiteError('repository is not a directory')
    manifest_path = _absolute(path, 'manifest')
    if not _inside(manifest_path, repo_path):
        raise SuiteError('manifest must remain inside the repository')
    try:
        with manifest_path.open('rb') as handle:
            observed_size = os.fstat(handle.fileno()).st_size
            if observed_size > MAX_MANIFEST_BYTES:
                raise SuiteError('manifest is larger than the bounded limit')
            data = handle.read(MAX_MANIFEST_BYTES + 1)
    except SuiteError:
        raise
    except OSError as exc:
        raise _message_for_io('read manifest', manifest_path, exc) from exc
    if len(data) > MAX_MANIFEST_BYTES:
        raise SuiteError('manifest is larger than the bounded limit')
    try:
        manifest = json.loads(data.decode('utf-8'))
    except (UnicodeError, json.JSONDecodeError) as exc:
        raise SuiteError('manifest JSON is invalid: {0}'.format(exc)) from exc
    return _validate_manifest(manifest)


def _source_path(repo_path, source_path):
    source = repo_path.joinpath(*PurePosixPath(source_path).parts)
    absolute = _absolute(source, 'source')
    if not _inside(absolute, repo_path):
        raise SuiteError('source escapes the repository')
    if _inside(absolute, repo_path / 'emacs'):
        raise SuiteError('source is under the optional reference checkout')
    return absolute


def _check_mode(observed, expected_mode, label):
    executable = bool(observed & 0o111)
    if expected_mode == '100755' and not executable:
        raise SuiteError('{0} is not executable under mode 100755'.format(label))
    if expected_mode == '100644' and executable:
        raise SuiteError('{0} is executable under mode 100644'.format(label))


def _verify_source(source, entry):
    try:
        with source.open('rb') as handle:
            observed = os.fstat(handle.fileno())
            if not stat.S_ISREG(observed.st_mode):
                raise SuiteError('source is not a regular file')
            _check_mode(observed.st_mode, entry['mode'], 'source')
            size = observed.st_size
            if size != entry['size']:
                raise SuiteError('source size does not match manifest')
            sha = hashlib.sha256()
            blob = hashlib.sha1(
                b'blob ' + str(size).encode('ascii') + b'\0'
            )
            while True:
                chunk = handle.read(CHUNK)
                if not chunk:
                    break
                sha.update(chunk)
                blob.update(chunk)
    except SuiteError:
        raise
    except OSError as exc:
        raise _message_for_io('read source', source, exc) from exc
    if sha.hexdigest() != entry['sha256']:
        raise SuiteError('source SHA-256 does not match manifest')
    if blob.hexdigest() != entry['blob']:
        raise SuiteError('source Git blob SHA-1 does not match manifest')


def _output_path(out, repo_path):
    out_path = Path(os.path.abspath(os.fspath(out)))
    if os.path.lexists(out_path):
        raise SuiteError('output already exists')
    parent = out_path.parent
    _reject_symlink_components(parent, 'output parent')
    if not parent.is_dir():
        raise SuiteError('output parent is not a directory')
    if out_path == repo_path or _inside(repo_path, out_path):
        raise SuiteError('output overlaps the repository root')
    for owned in (repo_path / 'emacs', repo_path / 'test',
                  repo_path / OVERLAY_ROOT):
        if _inside(out_path, owned):
            raise SuiteError('output overlaps a corpus/reference tree')
    return out_path


def _copy_source(source, target, entry):
    expected_mode = VALID_MODES[entry['mode']]
    try:
        target.parent.mkdir(parents=True, exist_ok=True)
        with source.open('rb') as source_handle, target.open('xb') as target_handle:
            observed = os.fstat(source_handle.fileno())
            if not stat.S_ISREG(observed.st_mode):
                raise SuiteError('source is not a regular file')
            _check_mode(observed.st_mode, entry['mode'], 'source')
            size = observed.st_size
            if size != entry['size']:
                raise SuiteError('source size changed before copy')
            sha = hashlib.sha256()
            blob = hashlib.sha1(
                b'blob ' + str(size).encode('ascii') + b'\0'
            )
            copied = 0
            while True:
                chunk = source_handle.read(CHUNK)
                if not chunk:
                    break
                target_handle.write(chunk)
                sha.update(chunk)
                blob.update(chunk)
                copied += len(chunk)
            if copied != size:
                raise SuiteError('source size changed during copy')
            if sha.hexdigest() != entry['sha256']:
                raise SuiteError('source SHA-256 changed during copy')
            if blob.hexdigest() != entry['blob']:
                raise SuiteError('source Git blob SHA-1 changed during copy')
        os.chmod(target, expected_mode)
    except SuiteError:
        raise
    except OSError as exc:
        raise _message_for_io('copy source', source, exc) from exc


def _corpus_provider(repo):
    """Load acquisition only when explicitly needed; pure readers stay offline."""
    source = _source_path(repo, 'tools/reference-corpus.py')
    spec = importlib.util.spec_from_file_location('femacs_reference_corpus', source)
    provider = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(provider)
    return provider


def _cached_corpus(manifest, repo, root):
    root = _absolute(root, 'source cache')
    if _inside(root, repo/'emacs'):
        raise SuiteError('source cache is under the optional reference checkout')
    provider = _corpus_provider(repo)
    try:
        # The acquisition contract pins the complete selection, not only counts.
        provider.validate_selection(manifest)
        provider.verify_cache(root, manifest)
    except provider.suite.SuiteError as exc:
        raise SuiteError(str(exc)) from exc
    except (OSError, ValueError) as exc:
        raise SuiteError('cannot verify source cache: {0}'.format(exc)) from exc
    return root


def materialize(manifest, repo, out, *, corpus_root=None):
    """Copy exactly the manifest sources into a fresh owned output tree."""
    manifest = _validate_manifest(manifest)
    repo_path = _absolute(repo, 'repository')
    if not repo_path.is_dir():
        raise SuiteError('repository is not a directory')
    out_path = _output_path(out, repo_path)
    cache = _cached_corpus(manifest, repo_path, corpus_root) if corpus_root is not None else None
    if cache is not None and (_inside(out_path, cache) or _inside(cache, out_path)):
        raise SuiteError('output overlaps the verified source cache')
    prepared = []
    for entry in manifest['entries']:
        source = (_source_path(repo_path, entry['source_path']) if cache is None
                  else _absolute(cache.joinpath(*PurePosixPath(entry['path']).parts), 'cached source'))
        _verify_source(source, entry)
        prepared.append((entry, source))
    try:
        out_path.mkdir(mode=0o700)
    except OSError as exc:
        raise _message_for_io('create output', out_path, exc) from exc
    total = 0
    for entry, source in prepared:
        target = out_path.joinpath(*PurePosixPath(entry['path']).parts)
        _copy_source(source, target, entry)
        total += entry['size']
    return {'file_count': len(prepared), 'bytes': total}


def _ids(values, label):
    if not isinstance(values, (list, tuple)):
        raise SuiteError('{0} must be a list'.format(label))
    result = []
    for value in values:
        if not isinstance(value, str) or not value:
            raise SuiteError('{0} contains a malformed ID'.format(label))
        result.append(value)
    return result


def compare_registrations(reference, candidate):
    """Compare generated ERT identities; file/defun counts are never used."""
    reference_ids = _ids(reference, 'reference')
    candidate_ids = _ids(candidate, 'candidate')
    reference_set = set(reference_ids)
    if len(reference_set) != len(reference_ids):
        raise SuiteError('reference registration contains duplicate IDs')
    candidate_set = set(candidate_ids)
    duplicates = sorted(
        case_id for case_id in candidate_set
        if candidate_ids.count(case_id) > 1
    )
    missing = sorted(reference_set - candidate_set)
    extra = sorted(candidate_set - reference_set)
    return {
        'passed': not missing and not extra and not duplicates,
        'reference_count': len(reference_set),
        'missing': missing,
        'extra': extra,
        'duplicates': duplicates,
    }


def _seconds(value):
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        return None
    if not math.isfinite(value) or value < 0:
        return None
    return value


def assess_case(expected_id, observed):
    """Assess one already-run ERT row without erasing skips or diagnostics."""
    if not isinstance(expected_id, str) or not expected_id:
        expected_id = None
    if not isinstance(observed, dict):
        return {
            'passed': False, 'status': None, 'seconds': None,
            'diagnostic': '',
        }
    status = observed.get('status')
    seconds = _seconds(observed.get('seconds'))
    diagnostic = observed.get('diagnostic', '')
    complete = observed.get('complete') is True
    exit_value = observed.get('exit_code')
    exit_ok = type(exit_value) is int and exit_value == 0
    identity_ok = expected_id is not None and observed.get('test_id') == expected_id
    passed = (status == 'passed' and complete and exit_ok and
              identity_ok and seconds is not None)
    return {
        'passed': passed,
        'status': status,
        'seconds': seconds,
        'diagnostic': diagnostic,
    }


def _feature_set(value, label):
    if value is None:
        return set()
    if not isinstance(value, (set, frozenset, list, tuple)):
        raise SuiteError('{0} must be a feature collection'.format(label))
    return set(value)


def classify_applicability(platform, required_platform, feature,
                           reference_features, candidate_features):
    """Classify platform/feature scope without turning foreign scope into a pass."""
    if not isinstance(platform, str) or not platform:
        raise SuiteError('platform must be a non-empty string')
    reference_set = _feature_set(reference_features, 'reference_features')
    candidate_set = _feature_set(candidate_features, 'candidate_features')
    if required_platform is not None and required_platform != platform:
        return {'applicable': False, 'available': False,
                'reason': 'other-platform'}
    if feature is None:
        return {'applicable': True, 'available': True,
                'reason': 'applicable'}
    if not isinstance(feature, str) or not feature:
        raise SuiteError('feature must be a non-empty string or None')
    if feature not in reference_set:
        return {'applicable': False, 'available': False,
                'reason': 'reference-feature-missing'}
    if feature in candidate_set:
        return {'applicable': True, 'available': True,
                'reason': 'available'}
    return {'applicable': True, 'available': False,
            'reason': 'candidate-feature-missing'}


def _safe_case_component(case_id):
    if not isinstance(case_id, str) or not case_id:
        raise SuiteError('case_id must be a non-empty string')
    # Logical IDs remain in the result protocol.  Repeating them in temporary
    # paths can exceed sockaddr_un limits before a socket test reaches its body.
    digest = hashlib.sha256(case_id.encode('utf-8')).hexdigest()[:16]
    return 'case-' + digest


def case_environment(case_id, out):
    """Create isolated HOME, cache, native-cache and runtime-owned paths."""
    try:
        base = Path(os.path.abspath(os.fspath(out)))
    except (TypeError, OSError) as exc:
        raise SuiteError('invalid case environment output') from exc
    if os.path.lexists(base) and not base.is_dir():
        raise SuiteError('case environment output is not a directory')
    try:
        base.mkdir(parents=True, exist_ok=True)
        root = base / _safe_case_component(case_id)
        root.mkdir(parents=True, exist_ok=True)
        paths = {
            'HOME': root / 'home',
            'XDG_CACHE_HOME': root / 'cache',
            'XDG_CONFIG_HOME': root / 'config',
            'XDG_STATE_HOME': root / 'state',
            'TMPDIR': root / 'tmp',
            'FEMACS_RUNTIME_NATIVE_CACHE': root / 'native-cache',
            'EMACSNATIVELOADPATH': root / 'native-cache',
            'FEMACS_RUNTIME_CASE_ROOT': root,
            'FEMACS_RUNTIME_OUT': root / 'out',
        }
        for path in paths.values():
            path.mkdir(parents=True, exist_ok=True)
    except OSError as exc:
        raise _message_for_io('create case environment', base, exc) from exc
    paths['TMP'] = paths['TMPDIR']
    environment = {key: str(value) for key, value in paths.items()}
    logical_file = case_id.split('|', 1)[0]
    if sys.platform == 'darwin' and logical_file in (
            'test/lisp/autorevert-tests.el', 'test/lisp/filenotify-tests.el'):
        # The original mock transport uses an interactive sh.  Its monitor
        # diagnostics must not become file-transfer bytes.  Keep this local
        # to these owned mock fixtures, identical on reference and candidates.
        script = paths['HOME'] / 'mock-shell-env'
        content = b'case $- in\n  *m*) set +m ;;\nesac\n'
        if script.is_symlink() or not script.resolve().is_relative_to(root.resolve()):
            raise SuiteError('mock shell startup file escapes its owned case')
        if script.exists():
            if not script.is_file() or script.read_bytes() != content:
                raise SuiteError('mock shell startup file differs')
        else:
            with script.open('xb') as stream:
                stream.write(content)
            script.chmod(0o600)
        environment['ENV'] = str(script)
        environment['FEMACS_STABLE_MOCK_SHELL_ENV'] = str(script)
    return environment



MAX_RESULT_BYTES = 16 * 1024 * 1024
DEFAULT_TIMEOUT = 300.0
RESULT_SCHEMA = 'femacs-stable-suite-run-v2'


def _sha256_path(path):
    digest = hashlib.sha256()
    try:
        with path.open('rb') as handle:
            for chunk in iter(lambda: handle.read(CHUNK), b''):
                digest.update(chunk)
    except OSError as exc:
        raise _message_for_io('hash file', path, exc) from exc
    return digest.hexdigest()


def _tree_identity(root):
    root = Path(root)
    if not root.is_dir():
        raise SuiteError('product library root is not a directory: {0}'.format(root))
    digest = hashlib.sha256()
    count = 0
    total = 0
    try:
        for base, dirs, files in os.walk(root, followlinks=False):
            dirs[:] = sorted(
                name for name in dirs
                if not (Path(base) / name).is_symlink()
            )
            for name in sorted(files):
                path = Path(base) / name
                if path.is_symlink():
                    continue
                metadata = path.stat()
                if not stat.S_ISREG(metadata.st_mode):
                    continue
                relative = path.relative_to(root).as_posix()
                file_hash = _sha256_path(path)
                digest.update(relative.encode('utf-8'))
                digest.update(b'\0')
                digest.update(str(metadata.st_size).encode('ascii'))
                digest.update(b'\0')
                digest.update(bytes.fromhex(file_hash))
                count += 1
                total += metadata.st_size
    except OSError as exc:
        raise _message_for_io('hash library tree', root, exc) from exc
    return {
        'name': root.name,
        'sha256': digest.hexdigest(),
        'file_count': count,
        'bytes': total,
    }


def _parse_argv_json(text, label):
    if text is None:
        return []
    try:
        value = json.loads(text)
    except json.JSONDecodeError as exc:
        raise SuiteError('{0} is not valid JSON: {1}'.format(label, exc)) from exc
    if not isinstance(value, list) or any(not isinstance(item, str) for item in value):
        raise SuiteError('{0} must be a JSON array of strings'.format(label))
    return value


def _dump_path(arguments):
    found = []
    index = 0
    while index < len(arguments):
        argument = arguments[index]
        if argument == '--dump-file':
            if index + 1 >= len(arguments):
                raise SuiteError('--dump-file has no value')
            found.append(arguments[index + 1])
            index += 2
            continue
        if argument.startswith('--dump-file='):
            found.append(argument.split('=', 1)[1])
            index += 1
            continue
        index += 1
    if len(found) > 1:
        raise SuiteError('multiple --dump-file arguments')
    if not found:
        return None
    value = Path(found[0]).expanduser()
    if not value.is_absolute():
        raise SuiteError('dump file path must be absolute for identity capture')
    try:
        value = value.resolve()
    except OSError as exc:
        raise SuiteError('cannot resolve dump file: {0}'.format(exc)) from exc
    if not value.is_file():
        raise SuiteError('dump file is not a regular file: {0}'.format(value))
    return value


def _product_paths(program, dump_args, runtime_env=None):
    path = Path(program).expanduser()
    try:
        path = path.resolve()
    except OSError as exc:
        raise SuiteError('cannot resolve product program: {0}'.format(exc)) from exc
    if not path.is_file():
        raise SuiteError('product program is not a regular file: {0}'.format(path))
    parent = path.parent
    if parent.name == 'MacOS':
        lisp_root = parent.parent / 'Resources' / 'lisp'
    else:
        lisp_root = parent.parent / 'lisp'
    if not lisp_root.is_dir():
        raise SuiteError('product library root is missing: {0}'.format(lisp_root))
    return {
        'program': path,
        'dump': _dump_path(dump_args),
        'lisp_root': lisp_root,
        'dump_args': list(dump_args),
        'runtime_env': dict(runtime_env or {}),
    }


def _stat_identity(path):
    try:
        metadata = path.stat()
    except OSError as exc:
        raise _message_for_io('inspect product input', path, exc) from exc
    if not stat.S_ISREG(metadata.st_mode):
        raise SuiteError('product input is not a regular file: {0}'.format(path))
    return {
        'name': path.name,
        'sha256': _sha256_path(path),
        'bytes': metadata.st_size,
        'mode': stat.S_IMODE(metadata.st_mode),
    }


def _public_product_identity(paths):
    dump = _stat_identity(paths['dump']) if paths['dump'] is not None else None
    result = {
        'program': _stat_identity(paths['program']),
        'dump': dump,
        'lisp': _tree_identity(paths['lisp_root']),
    }
    resources = paths['lisp_root'].parent
    library = (resources.parent / 'Frameworks/runtime-lcms'
               if resources.name == 'Resources' else resources / 'runtime-lcms')
    licenses = resources / 'licenses/lcms'
    if library.exists() or licenses.exists():
        if not library.is_dir() or not licenses.is_dir():
            raise SuiteError('incomplete LCMS runtime/license package')
        result['lcms_runtime'] = _tree_identity(library)
        result['lcms_licenses'] = _tree_identity(licenses)
    library = (resources.parent / 'Frameworks/runtime-treesit'
               if resources.name == 'Resources' else resources / 'runtime-treesit')
    licenses = resources / 'licenses/treesit'
    if library.exists() or licenses.exists():
        if not library.is_dir() or not licenses.is_dir():
            raise SuiteError('incomplete Tree-sitter runtime/license package')
        result['treesit_runtime'] = _tree_identity(library)
        result['treesit_licenses'] = _tree_identity(licenses)

    return result


def _safe_component(value):
    safe = re.sub(r'[^A-Za-z0-9._-]+', '-', value).strip('.-')
    if not safe:
        safe = 'case'
    digest = hashlib.sha256(value.encode('utf-8')).hexdigest()[:12]
    return '{0}-{1}'.format(safe, digest)


def _relative_to(out, path):
    try:
        return path.resolve().relative_to(out.resolve()).as_posix()
    except ValueError as exc:
        raise SuiteError('generated file escaped owned output') from exc


def _read_machine_result(path):
    try:
        size = path.stat().st_size
    except OSError as exc:
        raise _message_for_io('inspect machine result', path, exc) from exc
    if size > MAX_RESULT_BYTES:
        raise SuiteError('machine result exceeded the bounded limit')
    try:
        value = json.loads(path.read_text(encoding='utf-8'))
    except (OSError, UnicodeError, json.JSONDecodeError) as exc:
        raise SuiteError('machine result is not valid JSON: {0}'.format(exc)) from exc
    if not isinstance(value, dict):
        raise SuiteError('machine result must be an object')
    return value


def _metadata(payload):
    if not isinstance(payload, dict):
        return {}
    features = payload.get('features')
    if not isinstance(features, list) or any(not isinstance(item, str) for item in features):
        features = []
    return {
        'emacs_version': payload.get('emacs_version')
            if isinstance(payload.get('emacs_version'), str) else None,
        'system_configuration': payload.get('system_configuration')
            if isinstance(payload.get('system_configuration'), str) else None,
        'system_type': payload.get('system_type')
            if isinstance(payload.get('system_type'), str) else None,
        'window_system': payload.get('window_system')
            if isinstance(payload.get('window_system'), str) else None,
        'features': sorted(features),
    }


def _run_process(out, paths, runner, test_root, logical_file, mode,
                 test_id, timeout, label):
    case_parent = out / 'environments' / label / mode
    case_label = '{0}|{1}|{2}'.format(logical_file, test_id or 'discover',
                                      time.time_ns())
    try:
        environment = case_environment(case_label, case_parent)
        case_root = Path(environment['FEMACS_RUNTIME_CASE_ROOT'])
        cwd = case_root / 'cwd'
        cwd.mkdir(parents=True, exist_ok=True)
        log_dir = out / 'logs' / label / mode
        result_dir = out / 'results' / label / mode
        log_dir.mkdir(parents=True, exist_ok=True)
        result_dir.mkdir(parents=True, exist_ok=True)
        safe = _safe_component('{0}-{1}'.format(logical_file, test_id or mode))
        stdout_path = log_dir / (safe + '.stdout.log')
        stderr_path = log_dir / (safe + '.stderr.log')
        result_path = result_dir / (safe + '.json')
    except (SuiteError, OSError) as exc:
        return {
            'product': label, 'mode': mode, 'file': logical_file,
            'test_id': test_id, 'status': 'environment-error',
            'complete': False, 'exit_code': None, 'timed_out': False,
            'seconds': 0.0, 'stdout_log': None, 'stderr_log': None,
            'result_file': None, 'json': None, 'error': str(exc),
        }
    file_path = out.joinpath(*PurePosixPath(logical_file).parts)
    argv = ([str(paths['program'])] + list(paths['dump_args']) +
            ['--batch', '-Q', '-L', str(test_root), '-l', str(runner),
             '-f', 'stable-suite-main'])
    child_env = dict(environment)
    child_env.update({
        'PATH': os.environ.get('PATH', os.defpath),
        'LC_ALL': 'C',
        'LANG': 'C',
        'TERM': 'dumb',
        'PWD': str(cwd),
        'EMACS_TEST_DIRECTORY': str(test_root),
        'FEMACS_RUNTIME_TEST_ROOT': str(case_root),
        'STABLE_SUITE_MODE': mode,
        'STABLE_SUITE_FILE': str(file_path),
        'STABLE_SUITE_FILE_ID': logical_file,
        'STABLE_SUITE_TEST': test_id or '',
        'STABLE_SUITE_RESULT': str(result_path),
    })
    child_env.update(paths.get('runtime_env', {}))
    if paths.get('test_context') is not None:
        child_env.update(STABLE_SUITE_SOURCE_CONTEXT=str(paths['test_context']),
                         STABLE_SUITE_PRODUCT_LISP=str(paths['lisp_root']))
    started = time.monotonic()
    process = None
    timed_out = False
    exit_code = None
    spawn_error = None
    try:
        with stdout_path.open('xb') as stdout, stderr_path.open('xb') as stderr:
            try:
                process = subprocess.Popen(
                    argv, cwd=str(cwd), env=child_env,
                    stdout=stdout, stderr=stderr, start_new_session=True,
                )
            except OSError as exc:
                spawn_error = str(exc)
            if process is not None:
                try:
                    exit_code = process.wait(timeout=timeout)
                except subprocess.TimeoutExpired:
                    timed_out = True
                    try:
                        os.killpg(process.pid, signal.SIGKILL)
                    except OSError:
                        process.kill()
                    exit_code = process.wait()
    except OSError as exc:
        spawn_error = str(exc)
    elapsed = time.monotonic() - started
    payload = None
    json_error = None
    if result_path.is_file():
        try:
            payload = _read_machine_result(result_path)
        except SuiteError as exc:
            json_error = str(exc)
    if spawn_error is not None:
        status = 'environment-error'
    elif timed_out:
        status = 'timeout'
    elif payload is None:
        status = 'result-error'
    else:
        status = payload.get('status')
        if not isinstance(status, str) or not status:
            status = 'result-error'
        if exit_code != 0:
            status = 'process-exit'
    complete = (not timed_out and exit_code == 0 and
                isinstance(payload, dict) and status not in
                ('timeout', 'process-exit', 'result-error', 'environment-error'))
    error = json_error or spawn_error
    if payload is not None and error is None:
        value = payload.get('error')
        if isinstance(value, str):
            error = value
    return {
        'product': label,
        'mode': mode,
        'file': logical_file,
        'test_id': test_id,
        'status': status,
        'complete': complete,
        'exit_code': exit_code,
        'timed_out': timed_out,
        'seconds': elapsed,
        'stdout_log': _relative_to(out, stdout_path),
        'stderr_log': _relative_to(out, stderr_path),
        'result_file': _relative_to(out, result_path),
        'json': payload,
        'error': error,
    }


def _discover_product(out, manifest, paths, runner, test_root, label, timeout):
    records = []
    ids = []
    all_ids = []
    by_id = {}
    tags_by_id = {}
    duplicates = []
    failures = []
    metadata = {'features': set(), 'emacs_versions': set(),
                'system_configurations': set(), 'system_types': set(),
                'window_systems': set()}
    for entry in sorted(manifest['entries'], key=lambda item: item['path']):
        logical = entry['path']
        if not logical.endswith('.el'):
            continue
        result = _run_process(out, paths, runner, test_root, logical,
                              'discover', None, timeout, label)
        payload = result['json']
        status = result['status']
        new_ids = []
        tests = []
        error = result.get('error')
        if status == 'loaded' and isinstance(payload, dict):
            value = payload.get('new_ids')
            test_rows = payload.get('tests')
            if (not isinstance(value, list) or
                    any(not isinstance(item, str) or not item for item in value) or
                    not isinstance(test_rows, list)):
                status = 'result-error'
                error = error or 'invalid discovery machine record'
            else:
                new_ids = value
                for row in test_rows:
                    if not isinstance(row, dict):
                        status = 'result-error'
                        error = error or 'invalid test metadata row'
                        break
                    test_id = row.get('id')
                    test_tags = row.get('tags', [])
                    if (not isinstance(test_id, str) or
                            not isinstance(test_tags, list) or
                            any(not isinstance(tag, str) for tag in test_tags)):
                        status = 'result-error'
                        error = error or 'invalid discovered test identity'
                        break
                    tests.append({'id': test_id, 'tags': test_tags})
        if status != 'loaded':
            failures.append('{0} discovery {1}: {2}'.format(
                label, logical, error or status))
        for test_id in new_ids:
            all_ids.append(test_id)
            if test_id in by_id:
                duplicates.append(test_id)
                failures.append('{0} duplicate discovered ID: {1}'.format(
                    label, test_id))
            else:
                by_id[test_id] = logical
                ids.append(test_id)
        for row in tests:
            tags_by_id[row['id']] = row['tags']
        info = _metadata(payload)
        for key in ('features',):
            metadata[key].update(info.get(key, []))
        if info.get('emacs_version'):
            metadata['emacs_versions'].add(info['emacs_version'])
        if info.get('system_configuration'):
            metadata['system_configurations'].add(info['system_configuration'])
        if info.get('system_type'):
            metadata['system_types'].add(info['system_type'])
        if info.get('window_system'):
            metadata['window_systems'].add(info['window_system'])
        record = dict(result)
        record.update({
            'status': status,
            'new_ids': new_ids,
            'tests': tests,
            'error': error,
        })
        records.append(record)
    return {
        'product': label,
        'entries': records,
        'ids': ids,
        'all_ids': all_ids,
        'by_id': by_id,
        'tags_by_id': tags_by_id,
        'duplicates': sorted(set(duplicates)),
        'failures': failures,
        'metadata': {key: sorted(value) for key, value in metadata.items()},
    }


def _missing_case(test_id, logical_file, label, reason):
    return {
        'test_id': test_id,
        'product': label,
        'file': logical_file,
        'status': 'missing',
        'complete': False,
        'exit_code': None,
        'timed_out': False,
        'seconds': None,
        'stdout_log': None,
        'stderr_log': None,
        'result_file': None,
        'json': None,
        'record': None,
        'duration': None,
        'assessment': {
            'passed': False, 'status': 'missing',
            'seconds': None, 'diagnostic': reason,
        },
        'error': reason,
    }


def _run_case(out, paths, runner, test_root, logical_file, test_id,
              timeout, label):
    result = _run_process(out, paths, runner, test_root, logical_file,
                          'run', test_id, timeout, label)
    payload = result['json']
    record = payload.get('record') if isinstance(payload, dict) else None
    if not isinstance(record, dict):
        record = None
    test_status = result['status']
    if isinstance(payload, dict) and isinstance(payload.get('status'), str):
        test_status = payload['status']
    duration = result['seconds']
    if record is not None:
        value = record.get('duration')
        if (not isinstance(value, bool) and isinstance(value, (int, float))
                and math.isfinite(value) and value >= 0):
            duration = value
    diagnostic = ''
    if record is not None and isinstance(record.get('diagnostic'), str):
        diagnostic = record['diagnostic']
    elif isinstance(result.get('error'), str):
        diagnostic = result['error']
    complete = (not result['timed_out'] and result['exit_code'] == 0 and
                isinstance(payload, dict) and record is not None and
                test_status in ('passed', 'skipped', 'failed'))
    observed = {
        'test_id': payload.get('test_id')
            if isinstance(payload, dict) and isinstance(payload.get('test_id'), str)
            else test_id,
        'status': test_status,
        'complete': complete,
        'exit_code': result['exit_code'] if isinstance(result['exit_code'], int) else 1,
        'seconds': duration,
        'diagnostic': diagnostic,
    }
    assessment = assess_case(test_id, observed)
    result.update({
        'test_status': test_status,
        'record': record,
        'duration': duration,
        'assessment': assessment,
    })
    return result


def _load_batch():
    import importlib.util
    path = Path(__file__).with_name('stable-suite-batch.py')
    spec = importlib.util.spec_from_file_location('femacs_stable_file_batches', path)
    batch = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(batch)
    return batch


def _retire_state(*args):
    import importlib.util
    path = Path(__file__).with_name('runtime-retire.py')
    spec = importlib.util.spec_from_file_location('femacs_stable_retirement', path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module.retire(*args)


def _execute(manifest, repo, out, reference, candidate,
             reference_dump_args, candidate_dump_args, timeout,
             reference_env=None, candidate_env=None, *, corpus_root=None,
             runner_path=None):
    from types import SimpleNamespace
    batch = _load_batch()
    return batch.execute(SimpleNamespace(**globals()), manifest, repo, out,
        reference, candidate, reference_dump_args, candidate_dump_args, timeout,
        reference_env, candidate_env, corpus_root=corpus_root, runner_path=runner_path)


def _write_run_report(out_path, report):
    path = out_path / 'stable-suite-run.json'
    try:
        path.write_text(
            json.dumps(report, indent=2, sort_keys=True) + '\n',
            encoding='utf-8',
        )
    except OSError as exc:
        raise _message_for_io('write stable-suite receipt', path, exc) from exc
    files = report['files']
    completion = dict(schema='femacs-stable-completion-v1', result_schema=RESULT_SCHEMA,
        selector=report['selector'], source=report['source'],
        manifest_sha256=report['manifest_sha256'],
        execution_complete=bool(files) and all(row[label]['complete']
            for row in files for label in ('reference', 'candidate')),
        corpus_complete=report['corpus_complete'],
        products_before=report['products_before'], products_after=report['products_after'],
        identities_unchanged=report['identities_unchanged'],
        corpus_inputs_unchanged=report['corpus_inputs_unchanged'],
        harness_sources=report['harness_sources'],
        runtime_environment=report['runtime_environment'],
        entrypoint_count=report['entrypoint_count'], file_count=len(files),
        resource_only_lisp_inputs=report['resource_only_lisp_inputs'],
        case_count=len(report['cases']), failure_count=len(report['failures']),
        passed_count=sum(row['passed'] is True for row in report['cases']),
        detailed=dict(path=path.name, bytes=path.stat().st_size, sha256=_sha256_path(path)),
        full_compatibility_claim=False)
    (out_path/'stable-suite-completion.json').write_text(
        json.dumps(completion, indent=2, sort_keys=True)+'\n', encoding='utf-8')
    return path


def _write_receipt(out_path, manifest, result):
    receipt = out_path / 'stable-suite-materialize.json'
    payload = {
        'schema': 'femacs-stable-suite-materialize-v1',
        'operation': 'materialize',
        'source': {
            'release': manifest['source']['release'],
            'commit': manifest['source']['commit'],
        },
        'file_count': result['file_count'],
        'bytes': result['bytes'],
        'executed': False,
        'compatibility': None,
    }
    try:
        receipt.write_text(
            json.dumps(payload, indent=2, sort_keys=True) + '\n',
            encoding='utf-8',
        )
    except OSError as exc:
        raise _message_for_io('write materialization receipt', receipt, exc) from exc


def read_completion(out, manifest_path, reference, candidate,
                    reference_args, candidate_args, reference_env, candidate_env):
    """Read a bounded summary and stream-bind the full original-file evidence."""
    root = Path(out)
    receipt = _read_machine_result(root/'stable-suite-completion.json')
    if (receipt.get('schema') != 'femacs-stable-completion-v1'
            or receipt.get('result_schema') != RESULT_SCHEMA
            or receipt.get('selector') != 'all-discovered-ids'
            or any(type(receipt.get(key)) is not bool for key in (
                'execution_complete', 'corpus_complete', 'identities_unchanged',
                'corpus_inputs_unchanged', 'full_compatibility_claim'))
            or receipt['full_compatibility_claim']):
        raise SuiteError('invalid stable completion schema or scope')
    manifest_path = Path(manifest_path)
    manifest = load_manifest(manifest_path, manifest_path.parents[2])
    batch = _load_batch()
    expected_count = len(batch.select_entrypoints(manifest))
    if (receipt.get('manifest_sha256') != _sha256_path(manifest_path)
            or receipt.get('source') != {key: manifest['source'][key] for key in ('release', 'commit')}
            or receipt.get('entrypoint_count') != expected_count
            or receipt.get('file_count') != expected_count
            or receipt.get('resource_only_lisp_inputs') != manifest['lisp_file_count']-expected_count):
        raise SuiteError('stable completion manifest/entrypoint identity mismatch')
    environments = {'reference': reference_env, 'candidate': candidate_env}
    expected_products = {label: _public_product_identity(_product_paths(program, args, environments[label]))
        for label, program, args in (('reference', reference, reference_args),
                                     ('candidate', candidate, candidate_args))}
    if (receipt.get('products_before') != expected_products
            or receipt.get('products_after') != expected_products
            or receipt.get('runtime_environment') != environments):
        raise SuiteError('stable completion product/environment identity mismatch')
    detailed = receipt.get('detailed', {})
    path = root/'stable-suite-run.json'
    if (detailed.get('path') != path.name or path.is_symlink()
            or type(detailed.get('bytes')) is not int
            or path.stat().st_size != detailed['bytes']
            or _sha256_path(path) != detailed.get('sha256')):
        raise SuiteError('stable detailed evidence changed')
    sources = receipt.get('harness_sources')
    if not isinstance(sources, dict) or not sources:
        raise SuiteError('stable harness identities missing')
    for name, identity in sources.items():
        if _sha256_path(Path(name)) != identity:
            raise SuiteError('stable harness changed after execution')
    for key in ('case_count', 'passed_count', 'failure_count'):
        if type(receipt.get(key)) is not int or receipt[key] < 0:
            raise SuiteError('invalid stable completion count')
    if receipt['corpus_complete'] and not (
            receipt['execution_complete'] and receipt['identities_unchanged']
            and receipt['corpus_inputs_unchanged'] and receipt['failure_count'] == 0
            and receipt['passed_count'] == receipt['case_count'] and receipt['case_count'] > 0):
        raise SuiteError('stable completion falsely claims a passing corpus')
    return receipt


def main(argv=None):
    parser = argparse.ArgumentParser(
        description='Materialize or execute the stable 31.1 test corpus.'
    )
    parser.add_argument('--manifest', required=True)
    parser.add_argument('--out', required=True)
    parser.add_argument('--repo', default=str(Path(__file__).resolve().parent.parent))
    parser.add_argument('--materialize', action='store_true')
    corpus = parser.add_mutually_exclusive_group()
    corpus.add_argument('--corpus-root', help='Existing complete verified source cache')
    corpus.add_argument('--corpus-cache', help='Persistent acquisition cache; no reference checkout is used')
    parser.add_argument('--reference')
    parser.add_argument('--candidate')
    parser.add_argument('--reference-dump-args')
    parser.add_argument('--candidate-dump-args')
    parser.add_argument('--reference-env', default='{}')
    parser.add_argument('--candidate-env', default='{}')
    parser.add_argument('--timeout', type=float, default=DEFAULT_TIMEOUT)
    parser.add_argument('--runner', help='Explicit bound per-file ERT bridge; does not alter corpus inputs')
    args = parser.parse_args(argv)
    try:
        has_reference = args.reference is not None
        has_candidate = args.candidate is not None
        if has_reference != has_candidate:
            raise SuiteError('--reference and --candidate must be supplied together')
        if args.materialize and (has_reference or has_candidate):
            raise SuiteError('--materialize cannot be combined with products')
        if not has_reference and (args.reference_dump_args is not None or
                                  args.candidate_dump_args is not None):
            raise SuiteError('dump arguments require reference and candidate')
        if not has_reference and (args.reference_env != '{}' or args.candidate_env != '{}'):
            raise SuiteError('runtime environments require reference and candidate')
        if not math.isfinite(args.timeout) or args.timeout <= 0:
            raise SuiteError('--timeout must be positive and finite')
        manifest_sha256 = _sha256_path(Path(args.manifest))
        manifest = load_manifest(args.manifest, args.repo)
        if _sha256_path(Path(args.manifest)) != manifest_sha256:
            raise SuiteError('source manifest changed while loading')
        corpus_root = args.corpus_root
        repo = _absolute(args.repo, 'repository')
        # Reject invalid destinations before acquiring any missing source bytes.
        out = _output_path(args.out, repo)
        mapped_inputs_present = all(
            os.path.lexists(repo.joinpath(*PurePosixPath(entry['source_path']).parts))
            for entry in manifest['entries'])
        if corpus_root is None and (args.corpus_cache is not None or not mapped_inputs_present):
            provider = _corpus_provider(repo)
            try:
                acquired = provider.prepare(args.manifest,
                    args.corpus_cache or repo/'.femacs-cache/stable-corpus', seed_repo=repo)
                corpus_root = acquired['path']
            except provider.suite.SuiteError as exc:
                raise SuiteError(str(exc)) from exc
            except (OSError, ValueError) as exc:
                raise SuiteError('cannot acquire pinned source corpus: {0}'.format(exc)) from exc
        if not has_reference:
            result = materialize(manifest, args.repo, out, corpus_root=corpus_root)
            _write_receipt(out, manifest, result)
            print(json.dumps({
                'operation': 'materialize',
                'file_count': result['file_count'],
                'bytes': result['bytes'],
                'executed': False,
            }, sort_keys=True))
            return 0
        reference_args = _parse_argv_json(
            args.reference_dump_args, '--reference-dump-args')
        candidate_args = _parse_argv_json(
            args.candidate_dump_args, '--candidate-dump-args')
        def runtime_environment(text):
            try:
                value = json.loads(text)
            except ValueError as error:
                raise SuiteError('runtime environment must be an explicit JSON object') from error
            allowed = {'FEMACS_GC_COLLECTOR': {'major', 'young'},
                       'FEMACS_GC_OBSERVATION': {'off', 'phases', 'survival', 'all'}}
            if not isinstance(value, dict) or any(key not in allowed or not isinstance(item, str) or item not in allowed[key]
                                                  for key, item in value.items()):
                raise SuiteError('only explicit collector/observation modes are allowed')
            return value
        reference_env = runtime_environment(args.reference_env)
        candidate_env = runtime_environment(args.candidate_env)
        started = time.monotonic()
        report = _execute(manifest, Path(args.repo), out, args.reference,
                          args.candidate, reference_args, candidate_args,
                          args.timeout, reference_env, candidate_env, corpus_root=corpus_root, runner_path=args.runner)
        report['runtime_environment'] = {'reference':reference_env, 'candidate':candidate_env}
        report['manifest_sha256'] = manifest_sha256
        if _sha256_path(Path(args.manifest)) != manifest_sha256:
            report['failures'].append('source manifest changed during execution')
            report['corpus_complete'] = False
        report['elapsed_seconds'] = time.monotonic() - started
        _write_run_report(out, report)
        print(json.dumps({
            'operation': 'execute',
            'corpus_complete': report['corpus_complete'],
            'case_count': len(report['cases']),
            'failure_count': len(report['failures']),
            'full_compatibility_claim': False,
        }, sort_keys=True))
        return 0 if report['corpus_complete'] else 1
    except SuiteError as exc:
        print('stable-suite: {0}'.format(exc), file=sys.stderr)
        return 2


if __name__ == '__main__':
    raise SystemExit(main())
