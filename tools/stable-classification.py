"""Evidence-bound accounting over the unchanged original stable-run protocol.

Rules describe exact observations from one completed run and profile. They are
not a test selector, a tag exemption, or a replacement for raw passing counts.
This module neither starts editors nor removes retained failure state.
"""
from __future__ import annotations

import hashlib
import json
import math
import os
from pathlib import Path
import re
import stat
import argparse
import importlib.util
import sys
import time
from types import SimpleNamespace

SCHEMA = 'femacs-stable-classification-v1'
RESULT_SCHEMA = 'femacs-stable-classified-accounting-v1'
# The observed nonordinary denominator is several hundred cases per profile.
# A full 48-profile catalog therefore needs more than the old 8192-rule bound.
MAX_CATALOG = 64 * 1024 * 1024
MAX_RULES = 65536
MAX_ITEM = 64 * 1024 * 1024
MAX_CASES = 100000
CATEGORIES = {'upstream-invalid-test', 'explicit-optional-capability',
              'proven-version-evolution', 'known-reference-library-defect',
              'approved-host-environment-limitation'}
REQUIRED_FEATURES = {'threads', 'thread', 'modules', 'module', 'dynamic-modules',
                     'gc', 'bytecode', 'pdumper', 'terminal', 'input-method'}
ERC_GRAPHICAL_SCENARIO_CONTROLS = (
    ('test/lisp/erc/erc-scenarios-keep-place-indicator-trunc.el', 'erc-scenarios-keep-place-indicator-trunc'),
    ('test/lisp/erc/erc-scenarios-keep-place-indicator.el', 'erc-scenarios-keep-place-indicator--follow'),
    ('test/lisp/erc/erc-scenarios-scrolltobottom-relaxed.el', 'erc-scenarios-scrolltobottom--relaxed'),
    ('test/lisp/erc/erc-scenarios-scrolltobottom.el', 'erc-scenarios-scrolltobottom--normal'),
    ('test/lisp/erc/erc-scenarios-scrolltobottom.el', 'erc-scenarios-scrolltobottom--all'),
)
ERC_GRAPHICAL_TEST_ENVIRONMENT = {'ERC_TESTS_GRAPHICAL': '1', 'ERC_TESTS_GRAPHICAL_ALL': '1'}
SVG_CONFIGURATION_TOKEN = 'RSVG'
CONFIGURATION_TOKENS = {'SECCOMP', 'DBUS', 'INOTIFY', 'PNG', 'JPEG', 'GIF',
                       'TIFF', 'WEBP', SVG_CONFIGURATION_TOKEN, 'XPM', 'IMAGEMAGICK'}


def require(condition, message):
    if not condition:
        raise ValueError(message)


def digest(value):
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(',', ':'),
                                     allow_nan=False).encode()).hexdigest()


def _pairs(values):
    result = {}
    for key, value in values:
        require(key not in result, 'duplicate JSON object key')
        result[key] = value
    return result


def bound_file(path, expected=None, limit=None):
    """Read identity through one FD; reject links and concurrent replacement."""
    path = Path(os.path.abspath(path))
    require(path.resolve() == path, 'classification evidence cannot traverse symlinks')
    fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
    with os.fdopen(fd, 'rb') as stream:
        before = os.fstat(stream.fileno())
        require(stat.S_ISREG(before.st_mode) and (limit is None or before.st_size <= limit),
                'classification evidence must be a bounded ordinary file')
        sha, count = hashlib.sha256(), 0
        while block := stream.read(65536):
            count += len(block)
            require(limit is None or count <= limit, 'classification evidence grew beyond bound')
            sha.update(block)
        after = os.fstat(stream.fileno())
    fields = ('st_dev', 'st_ino', 'st_size', 'st_mtime_ns', 'st_mode')
    current = path.lstat()
    require(all(getattr(before, key) == getattr(after, key) == getattr(current, key)
                for key in fields) and count == before.st_size,
            'classification evidence changed while reading')
    identity = dict(bytes=count, sha256=sha.hexdigest())
    require(expected is None or identity == expected, 'classification evidence identity mismatch')
    return identity


def load_catalog(path):
    identity = bound_file(path, limit=MAX_CATALOG)
    value = json.loads(read_bytes(path, identity, MAX_CATALOG), object_pairs_hook=_pairs)
    require(isinstance(value, dict) and set(value) == {'schema', 'manifest_sha256',
            'reference_selection_sha256', 'contexts', 'rules'} and value['schema'] == SCHEMA,
            'invalid classification catalog schema')
    require(isinstance(value['contexts'], dict) and isinstance(value['rules'], list)
            and len(value['contexts']) <= 48 and len(value['rules']) <= MAX_RULES,
            'classification catalog scope exceeded')
    require(bound_file(path, identity, MAX_CATALOG) == identity, 'classification catalog changed')
    return value, identity


def read_bytes(path, identity, limit):
    fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
    with os.fdopen(fd, 'rb') as stream:
        require(stat.S_ISREG(os.fstat(stream.fileno()).st_mode), 'ordinary evidence required')
        data = stream.read(limit + 1)
    require(len(data) <= limit and isinstance(identity, dict)
            and identity == dict(bytes=len(data), sha256=hashlib.sha256(data).hexdigest()),
            'bounded evidence bytes differ from identity')
    bound_file(path, identity, limit)
    return data


class _Stream:
    """A bounded stdlib decoder for top-level rows, including nested strings."""
    def __init__(self, stream):
        self.stream, self.buffer, self.eof = stream, '', False
        self.decoder = json.JSONDecoder(object_pairs_hook=_pairs)

    def fill(self):
        block = self.stream.read(65536)
        self.buffer += block
        self.eof = not block
        require(len(self.buffer.encode('utf-8')) <= MAX_ITEM, 'oversized detailed JSON item')

    def peek(self):
        while True:
            self.buffer = self.buffer.lstrip()
            if self.buffer or self.eof:
                return self.buffer[:1]
            self.fill()

    def token(self, expected):
        require(self.peek() == expected, 'invalid detailed JSON delimiter')
        self.buffer = self.buffer[1:]

    def value(self):
        self.peek()
        while True:
            try:
                value, end = self.decoder.raw_decode(self.buffer)
                if end == len(self.buffer) and not self.eof:
                    self.fill()
                    continue
                require(end == len(self.buffer) or self.buffer[end] in ' \r\n\t,]}:',
                        'invalid detailed JSON value boundary')
                self.buffer = self.buffer[end:]
                return value
            except json.JSONDecodeError as error:
                if self.eof:
                    raise ValueError('truncated/invalid detailed JSON') from error
                self.fill()


def detailed_rows(path, *, stream_arrays=frozenset({'cases', 'files', 'failures'})):
    """Yield each cases/files/failures item without materializing those arrays."""
    with Path(path).open(encoding='utf-8') as incoming:
        reader, seen = _Stream(incoming), set()
        reader.token('{')
        if reader.peek() != '}':
            while True:
                key = reader.value()
                require(isinstance(key, str) and key not in seen, 'duplicate/invalid detailed field')
                seen.add(key)
                reader.token(':')
                if key in stream_arrays:
                    reader.token('[')
                    if reader.peek() != ']':
                        while True:
                            yield key, reader.value()
                            if reader.peek() == ']':
                                break
                            reader.token(',')
                    reader.token(']')
                    yield key + '-end', None
                else:
                    yield key, reader.value()
                if reader.peek() == '}':
                    break
                reader.token(',')
        reader.token('}')
        require(not reader.peek(), 'trailing detailed JSON content')


def context_identity(receipt, profile, reference_selection):
    """Include actual products, libraries, environment, harness and epoch."""
    return digest(dict(manifest_sha256=receipt['manifest_sha256'],
        source=receipt['source'], products=receipt['products_before'],
        environment=receipt['runtime_environment'], harness=receipt['harness_sources'],
        profile=profile, reference_selection_sha256=reference_selection['sha256'],
        epoch=reference_selection['epoch'], upstream=reference_selection['upstream']))


def observation_signature(observation):
    return dict(status=observation.get('status'),
                diagnostic_sha256=hashlib.sha256(observation.get('diagnostic', '').encode()).hexdigest(),
                event_sha256=observation.get('evidence', {}).get('event_sha256'))


def _original_case_event(root, api, case, label):
    """Read the original event, preserving its condition and exact ordinal."""
    evidence = case[label].get('evidence', {})
    path = Path(root)/api._relative_path(evidence.get('path'), 'case progress')
    require(path.resolve() == path, 'case progress cannot traverse a symlink')
    fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
    found, ordinal = None, 0
    with os.fdopen(fd, 'rb') as stream:
        before = os.fstat(stream.fileno())
        require(stat.S_ISREG(before.st_mode) and before.st_size <= MAX_ITEM,
                'bounded regular original progress required')
        while raw := stream.readline(MAX_ITEM+1):
            require(len(raw) <= MAX_ITEM and raw.endswith(b'\n'), 'oversized or incomplete original event')
            event = json.loads(raw, object_pairs_hook=_pairs)
            if event.get('event') != 'test-ended':
                continue
            if hashlib.sha256(raw[:-1]).hexdigest() == evidence.get('event_sha256'):
                require(ordinal == evidence.get('event_ordinal'), 'original event ordinal differs')
                found = event
                break
            ordinal += 1
        after = os.fstat(stream.fileno())
    current = path.lstat()
    require(all(getattr(before,k) == getattr(after,k) == getattr(current,k)
                for k in ('st_dev','st_ino','st_size','st_mtime_ns','st_mode')),
            'original event changed while reading')
    require(isinstance(found, dict) and found.get('file') == case['file']
            and found.get('test_id') == case['id'] and found.get('status') == case[label]['status']
            and found.get('diagnostic', '') == case[label]['diagnostic'],
            'classified outcome is not the actual bound original runner event')
    return found


def _source_evidence(rule, entry):
    evidence = rule.get('source_evidence')
    require(isinstance(evidence, list) and 1 <= len(evidence) <= 16,
            'exact source anchors required for every classified case')
    original, excerpts = False, []
    for pin in evidence:
        require(isinstance(pin, dict) and set(pin) == {'path', 'bytes', 'sha256',
                'start_line', 'end_line', 'lf_sha256'}, 'invalid source anchor')
        bound_file(pin['path'], dict(bytes=pin['bytes'], sha256=pin['sha256']), 32 * 1024 * 1024)
        start, end = pin['start_line'], pin['end_line']
        require(type(start) is int and type(end) is int and 1 <= start <= end <= start + 500,
                'bounded literal-LF source anchor required')
        raw = read_bytes(pin['path'], dict(bytes=pin['bytes'], sha256=pin['sha256']),
                         32 * 1024 * 1024).replace(b'\r\n', b'\n')
        lines = raw.split(b'\n')
        require(end <= len(lines), 'source anchor outside file')
        excerpt = b'\n'.join(lines[start-1:end]) + b'\n'
        require(hashlib.sha256(excerpt).hexdigest() == pin['lf_sha256'], 'source anchor changed')
        original |= pin['sha256'] == entry['sha256'] and pin['bytes'] == entry['size']
        excerpts.append(excerpt.decode('utf-8'))
        bound_file(pin['path'], dict(bytes=pin['bytes'], sha256=pin['sha256']), 32 * 1024 * 1024)
    require(original, 'classification does not anchor the unchanged original test')
    return excerpts


def _control_json(pin, cache):
    require(isinstance(pin, dict) and isinstance(pin.get('identity'), dict),
            'explicit control file pin required')
    key = (pin.get('path'), pin['identity'].get('bytes'), pin['identity'].get('sha256'))
    require(isinstance(key[0], str) and type(key[1]) is int, 'invalid control file identity')
    if key not in cache:
        # Scoped to one assess call; every distinct proof is rehashed again
        # at exit. Avoid reparsing a whole multi-profile envelope per case.
        if sum(k[1] for k in cache if isinstance(k, tuple) and len(k) == 3
               and type(k[1]) is int) + key[1] > MAX_ITEM:
            cache.clear()
        cache[key] = json.loads(read_bytes(key[0], pin['identity'], MAX_ITEM), object_pairs_hook=_pairs)
    return cache[key]


def read_owner_summary(path, identity, root):
    """Bind a canonical journal while streaming its file and symlink arrays."""
    path, root = Path(path), Path(root)
    require(isinstance(identity, dict) and set(identity) == {'bytes', 'sha256'}
            and path.parent == root, 'owner journal is outside its bound output')
    bound_file(path, identity, 512*1024*1024)
    fields = {'schema', 'state', 'output', 'owner_pid', 'owner_birth', 'release_failures'}
    scalars = {}
    for field, value in detailed_rows(path, stream_arrays={'files', 'symlinks'}):
        if field in fields:
            scalars[field] = value
    require(set(scalars) == fields and scalars['schema'] == 'femacs-runtime-artifacts-v1'
            and scalars['state'] == 'released' and scalars['release_failures'] == []
            and scalars['output'] == str(root) and type(scalars['owner_pid']) is int
            and isinstance(scalars['owner_birth'], str) and bool(scalars['owner_birth']),
            'artifact owner did not release successfully')
    bound_file(path, identity, 512*1024*1024)
    return scalars


def _control_owner(pin, controller, root, cache):
    """Reuse the bounded journal reader for actual passing controls."""
    require(isinstance(pin, dict) and isinstance(pin.get('identity'), dict),
            'explicit control owner journal pin required')
    key = ('owner', pin.get('path'), digest(pin['identity']))
    if key not in cache:
        cache[key] = read_owner_summary(pin.get('path', ''), pin['identity'], root)
    require(cache[key]['owner_pid'] == controller['root_pid'],
            'control controller and artifact output have different owners')
    return {pin['path']: pin['identity']}


def _passing_control(control, case, label, file_row, receipt, api,
                     reference_selection, profile, *, display=None, counterpart=None,
                     purpose='applicability', test_environment=None, cache=None):
    """Bind an actual applicable run, not a claimed capability boolean."""
    require(isinstance(control, dict) and isinstance(control.get('path'), str),
            'passing applicable original-file control required')
    cache = {} if cache is None else cache
    value = _control_json(control, cache)
    control_input = control.get('test_input', {})
    bound_file(control_input.get('path', ''), control_input.get('identity'), 32*1024*1024)
    target_file = counterpart or case['file']
    target_id = control.get('test_id', case['id'])
    require(isinstance(target_id, str) and bool(target_id), 'control test identity required')
    argv = file_row[label].get('argv', [])
    fixture = value.get('working_fixture', {})
    admission = value.get('resource_admission') or {}
    source_hashes = {Path(name).name: sha for name, sha in receipt['harness_sources'].items()}
    require(value.get('schema') == 'femacs-stable-file-batch-v1'
            and value.get('file') == target_file and value.get('complete') is True
            and value.get('exit_code') == 0 and value.get('timed_out') is False
            and value.get('remaining_group_terminated') is False
            and bool(argv) and value.get('argv', [None])[0] == argv[0]
            and [a for a in value['argv'] if a.startswith('--dump-file=')]
                == [a for a in argv if a.startswith('--dump-file=')]
            and value.get('runtime_environment') == file_row[label].get('runtime_environment')
            and value.get('metadata', {}).get('product_version') == reference_selection['upstream']['version']
            and fixture.get('canonical_fixture_inputs_unchanged') is True
            and any(p.get('path') == target_file and p.get('before', {}).get('sha256')
                    == control_input.get('identity', {}).get('sha256') for p in fixture.get('files', []))
            and api.assess_case(target_id, value.get('cases', {}).get(target_id))['passed'] is True,
            'applicable control did not actually pass the exact original case and product')
    require(admission.get('schema') == 'femacs-stable-control-admission-v1'
            and admission.get('purpose') == purpose
            and admission.get('product') == receipt['products_before'][label]
            and admission.get('runtime_environment') == receipt['runtime_environment'][label]
            and admission.get('test_environment', {}) == value.get('test_environment', {})
            and (test_environment is None or value.get('test_environment', {}) == test_environment)
            and admission.get('reference_selection_sha256') == reference_selection['sha256']
            and admission.get('profile') == (profile['id'] if label == 'candidate' else 'installed-reference')
            and admission.get('harness_sources_by_role') == source_hashes
            and admission.get('module_header_sha256') == bound_file(
                Path(api.__file__).parent/'fixtures/module-thread-config.h')['sha256'],
            'applicable control has no matching product, epoch and captured harness proof')
    if purpose == 'applicability':
        require(admission.get('manifest_sha256') == receipt['manifest_sha256'],
                'applicability control must use the exact unchanged original corpus')
    if display is not None:
        metadata = value.get('metadata', {})
        require(display in {'terminal', 'cocoa'} and metadata.get('display_mode') == display
                and metadata.get('noninteractive') is False,
                'applicable display was not actually exercised')
        if display == 'cocoa':
            require(metadata.get('window_system') == 'ns' and metadata.get('display_graphic') is True,
                    'graphical control did not use a real Cocoa frame')
        else:
            require(metadata.get('window_system') is None
                    and value.get('execution_context', {}).get('owned_terminal_name'),
                    'terminal control has no owned controlling TTY')
    progress = control.get('progress', {})
    bound_file(progress.get('path', ''), progress.get('identity'), MAX_ITEM)
    evidence = value['cases'][target_id].get('evidence', {})
    require(value.get('progress', {}).get('sha256') == progress.get('identity', {}).get('sha256')
            and value.get('progress', {}).get('bytes') == progress.get('identity', {}).get('bytes')
            and evidence.get('path') == value.get('progress', {}).get('path'),
            'applicable control progress is not bound to its file')
    found = False
    with Path(progress['path']).open('rb') as stream:
        while raw := stream.readline(MAX_ITEM+1):
            require(len(raw) <= MAX_ITEM, 'oversized control event')
            if hashlib.sha256(raw.rstrip(b'\n')).hexdigest() == evidence.get('event_sha256'):
                event = json.loads(raw, object_pairs_hook=_pairs)
                found = (event.get('event') == 'test-ended' and event.get('file') == target_file
                    and event.get('test_id') == target_id and event.get('status') == 'passed'
                    and event.get('diagnostic', '') == value['cases'][target_id].get('diagnostic', ''))
                break
    require(found, 'applicable control has no exact passing original event')
    closed = control.get('closed_run', {})
    run = _control_json(closed, cache)
    expected_checks = {'all61_product_identities_unchanged', 'all61_complete_receipts_valid',
        'source_export_unchanged', 'reference_selection_unchanged', 'captured_harness_runner_unchanged',
        'original_current_version_inputs_unchanged', 'copied_current_version_inputs_unchanged',
        'current_source_context_links_unchanged', 'stable_source_context_unchanged'}
    require(run.get('schema') == 'femacs-current-version-controls-v1'
            and run.get('diagnostic_collection_complete') is True and run.get('passed') is True
            and run.get('reference_selection') == reference_selection and run.get('errors') == []
            and set(run.get('checks', {})) == expected_checks and all(run['checks'].values())
            and any(row.get('profile') == admission['profile'] and row.get('file') == target_file
                and row.get('complete') is True and row.get('test_input') == control_input
                and row.get('file_result', {}).get('sha256') == control['identity']['sha256']
                and row.get('file_result', {}).get('bytes') == control['identity']['bytes']
                and str(Path(run.get('corpus_root', ''))/row['file_result']['path']) == control['path']
                for row in run.get('cases', [])),
            'control is not in a closed full-product postflight envelope')
    controller = control.get('controller', {})
    lines = read_bytes(controller.get('path', ''), controller.get('identity'), 65536).decode('utf-8').splitlines()
    records = [json.loads(line[len('controlled-run: '):], object_pairs_hook=_pairs)
               for line in lines if line.startswith('controlled-run: {')]
    require(len(records) == 1 and records[0].get('reason') == 'exited'
            and records[0].get('exit_code') == 0 and records[0].get('cleanup_complete') is True
            and type(records[0].get('root_pid')) is int and records[0]['root_pid'] > 0
            and isinstance(records[0].get('argv_sha256'), str) and len(records[0]['argv_sha256']) == 64,
            'control outer controller did not close and reap successfully')
    proofs = _control_owner(control.get('owner'), records[0], Path(closed['path']).parent, cache)
    proofs.update({control['path']: control['identity'], control_input['path']: control_input['identity'],
            progress['path']: progress['identity'], closed['path']: closed['identity'],
            controller['path']: controller['identity']})
    return proofs


def _control_case_event(control, target_file, target_id, cache):
    """Read a completed sibling case event from an already bound control file."""
    value = _control_json(control, cache)
    observed = value.get('cases', {}).get(target_id)
    require(isinstance(observed, dict) and observed.get('complete') is True
            and observed.get('exit_code') == 0,
            'control sibling case did not complete cleanly')
    progress = control.get('progress', {})
    bound_file(progress.get('path', ''), progress.get('identity'), MAX_ITEM)
    evidence = observed.get('evidence', {})
    require(evidence.get('path') == value.get('progress', {}).get('path'),
            'control sibling case progress is not bound to its file')
    found = None
    with Path(progress['path']).open('rb') as stream:
        while raw := stream.readline(MAX_ITEM+1):
            require(len(raw) <= MAX_ITEM, 'oversized control event')
            if hashlib.sha256(raw.rstrip(b'\n')).hexdigest() == evidence.get('event_sha256'):
                candidate_event = json.loads(raw, object_pairs_hook=_pairs)
                if (candidate_event.get('event') == 'test-ended'
                        and candidate_event.get('file') == target_file
                        and candidate_event.get('test_id') == target_id
                        and candidate_event.get('status') == observed.get('status')
                        and candidate_event.get('diagnostic', '') == observed.get('diagnostic', '')):
                    found = candidate_event
                break
    require(isinstance(found, dict), 'control sibling case lacks its exact runner event')
    return value, observed, found


def _native_positive_applicability(gate, case, events, file_row, excerpts, profile):
    """Explain only the declared native-off outcome of an original native test."""
    require(set(gate) == {'kind', 'source_form', 'source_declaration', 'failure_kind',
                         'missing_on', 'conditions'}
            and gate['missing_on'] == ['candidate']
            and [case[s]['status'] for s in ('reference', 'candidate')] == ['passed', 'failed']
            and case['file'] == 'test/src/comp-tests.el'
            and profile.get('native') is False,
            'native positive-test scope or actual outcomes differ')
    require(all(case[s].get('complete') is True and case[s].get('exit_code') == 0
                and file_row[s].get('complete') is True and file_row[s].get('exit_code') == 0
                for s in ('reference', 'candidate')),
            'native positive-test classification requires complete original processes')
    reference = file_row['reference'].get('metadata', {})
    candidate = file_row['candidate'].get('metadata', {})
    require(reference.get('native_comp_available') is True
            and candidate.get('native_comp_available') is False
            and isinstance(reference.get('features'), list)
            and isinstance(candidate.get('features'), list)
            and 'native-compile' in reference['features']
            and 'native-compile' not in candidate['features'],
            'native prerequisite absence is not proven by the actual products')
    source = '\n'.join(excerpts)
    declaration = gate['source_declaration']
    require(isinstance(declaration, str) and declaration in source,
            'native test declaration is outside its original source anchors')
    match = re.fullmatch(r'\((comp-deftest|ert-deftest) ([^\s()]+) \([^\n]*', declaration)
    generated = re.fullmatch(r'\s*;;\s*([1-9][0-9]*)(?:[^0-9\n].*)?', declaration)
    if match is not None:
        require(case['id'] == ('comp-tests-' + match[2] if match[1] == 'comp-deftest' else match[2]),
                'native test declaration differs from its original registered ID')
    else:
        require(generated is not None and int(generated[1]) <= 81
                and case['id'] == 'comp-tests-ret-type-spec-' + generated[1]
                and '(format "ret-type-spec-%d" number)' in source
                and '(native-compile f-name)' in source
                and 'for n from 1' in source,
                'generated native type test lacks its original declaration and compile dependency')
    require(gate['source_form'] == '(native-comp-available-p)'
            and '(when (native-comp-available-p)' in source
            and '(load (native-compile comp-test-src))' in source
            and 'Define a test for the native compiler' in source,
            'native test lacks its exact declaration and precompiled fixture prerequisite')
    conditions = gate['conditions']
    condition = events['candidate'].get('condition')
    require(isinstance(condition, str) and conditions == {'candidate': condition},
            'native prerequisite outcome is not the bound original event')
    kind = gate['failure_kind']
    if kind == 'compiler-unavailable':
        expected = re.findall(r'\(\(null \(featurep \'native-compile\)\)\s*'
                              r'\(error ("[^"\\\n]+")\)\)', source)
        require(len(expected) == 1 and condition == '(error ' + expected[0] + ')',
                'native-off failure is not the original capability error')
    elif kind == 'native-function-assertion':
        require('native-comp-function-p' in source
                and condition.startswith('(ert-test-failed ((should (native-comp-function-p ')
                and condition.endswith(' :value nil))'),
                'native-off failure is not a positive native function assertion')
    elif kind == 'native-subr-assertion':
        require('(subr-type (symbol-function ' in source
                and condition.startswith('(wrong-type-argument subrp #[')
                and condition.endswith(')'),
                'native-off failure is not the native subr assertion on bytecode')
    elif kind == 'native-string-sharing':
        require('Verify string hash consing strategy.' in source
                and '(should (eq (comp-test-47868-1-f) (comp-test-47868-3-f)))' in source
                and ':form (eq " " " ") :value nil))' in condition
                and condition.startswith('(ert-test-failed ((should (eq '),
                'native-off failure is not the pinned native string sharing assertion')
    else:
        raise ValueError('unreviewed native positive-test outcome remains blocking')
    return {}


def _formal_counterpart(root, api, case, label, file_row, target_id, cache):
    """Bind an applicable sibling already executed in this original file."""
    cache = {} if cache is None else cache
    require(isinstance(target_id, str) and target_id != case['id'],
            'a distinct explicitly declared original counterpart is required')
    row = file_row[label]
    relative = api._relative_path(row.get('file_result', {}).get('path'), 'counterpart file')
    path = Path(root)/relative
    pin = {key: row['file_result'].get(key) for key in ('bytes', 'sha256')}
    value = _control_json(dict(path=str(path), identity=pin), cache)
    require(value.get('schema') == 'femacs-stable-file-batch-v1'
            and value.get('file') == case['file'] and value.get('complete') is True
            and value.get('exit_code') == 0 and value.get('timed_out') is False
            and value.get('remaining_group_terminated') is False
            and all(value.get(key) == row.get(key) for key in
                    ('argv', 'runtime_environment', 'metadata', 'progress')),
            'counterpart is outside the exact complete original file')
    observed = value.get('cases', {}).get(target_id)
    require(api.assess_case(target_id, observed)['passed'] is True,
            'the applicable original counterpart did not actually pass')
    event = _original_case_event(root, api,
        dict(file=case['file'], id=target_id, **{label: observed}), label)
    require(event.get('status') == 'passed', 'counterpart lacks its original passing event')
    progress = row['progress']
    return {str(path): pin, str(Path(root)/api._relative_path(progress['path'], 'counterpart progress')):
            {key: progress[key] for key in ('bytes', 'sha256')}}


def _direct_skip_applicability(gate, case, events, file_row, excerpts, receipt,
                               root, api, reference_selection, profile, cache):
    """Validate source-authored messages without fabricating a :form/:value."""
    kind, form = gate['kind'], gate.get('source_form')
    source = '\n'.join(excerpts)
    require(isinstance(form, str) and form in source
            and gate.get('conditions') == {s: events[s].get('condition') for s in gate['missing_on']},
            'direct skip lacks its exact source and original message')
    proofs = {}
    if kind == 'file-monitor-backend':
        match = re.fullmatch(r'(auto-revert-test0[0-7]-[^/]+)-remote', case['id'])
        require(case['file'] == 'test/lisp/autorevert-tests.el' and match is not None
                and gate.get('backend') == 'GKqueueFileMonitor'
                and gate.get('control_test_id') == match[1]
                and form == '(ert-skip (format "Monitor %s does not support auto-revert"\n'
                            '                        (auto-revert--test-monitor)))'
                and "(when (eq (auto-revert--test-monitor) 'GKqueueFileMonitor)" in source
                and '(defmacro auto-revert--deftest-remote (test docstring)' in source
                and '(funcall (ert-test-body ert-test))' in source
                and [case[s]['status'] for s in ('reference', 'candidate')] == ['skipped', 'skipped'],
                'unreviewed monitor exclusion remains blocking')
        for label in gate['missing_on']:
            require(file_row[label].get('metadata', {}).get('system_type') == 'darwin'
                    and events[label]['condition'] ==
                        '(ert-test-skipped "Monitor GKqueueFileMonitor does not support auto-revert")',
                    'actual monitor exclusion differs from the original backend gate')
            proofs.update(_formal_counterpart(root, api, case, label, file_row, match[1], cache))
    elif kind == 'file-monitor-identity':
        require(set(gate) == {'kind','source_form','contract','missing_on','conditions','controls'}
                and case['file'] == 'test/lisp/filenotify-tests.el'
                and case['id'] in {'file-notify-test05-file-validity-remote',
                                   'file-notify-test06-dir-validity-remote',
                                   'file-notify-test11-symlinks-remote',
                                   'file-notify-test12-unmount-remote'}
                and form == '(ert-skip "Cannot determine test monitor")'
                and gate['contract'] == 'remote-watch-lifecycle-v1'
                and '(defun file-notify--test-monitor ()' in source
                and '(defmacro file-notify--deftest-remote (test docstring &optional unstable)' in source
                and '(funcall (ert-test-body ert-test))' in source
                and set(gate['controls']) == {'reference','candidate'}
                and all(events[label].get('condition') == '(ert-test-skipped "Cannot determine test monitor")'
                        for label in gate['missing_on']),
                'unknown monitor identity has no exact source-authored applicability gate')
        for label in ('reference','candidate'):
            for counterpart in ('file-notify-test00-availability-remote','file-notify-test01-add-watch-remote',
                                'file-notify-test02-rm-watch-remote','file-notify-test03-events-remote'):
                proofs.update(_formal_counterpart(root, api, case, label, file_row, counterpart, cache))
            proofs.update(_remote_control(gate['controls'][label], receipt, label, profile,
                reference_selection, file_row, 'watch', cache, root=root, api=api))
    elif kind == 'external-message':
        require(case['file'] == 'test/lisp/erc/erc-scenarios-spelling.el'
                and case['id'] == 'erc-scenarios-spelling--auto-correct'
                and form == '(ert-skip "Missing ispell program")'
                and '(executable-find ispell-program-name)' in source
                and '(progn (ispell-check-version) t)' in source
                and '(member "american" (ispell-valid-dictionary-list))' in source
                and '(unless (ignore-errors' in source
                and [case[s]['status'] for s in ('reference', 'candidate')] == ['skipped', 'skipped']
                and all(events[s].get('condition') == '(ert-test-skipped "Missing ispell program")'
                        for s in gate['missing_on']),
                'external spelling prerequisite has no matching source-authored skip')
    elif kind == 'external-program-missing':
        require(case['file'] == 'test/lisp/net/dbus-tests.el'
                and case['id'] == 'dbus-test02-register-service-own-bus'
                and re.search(r'\(call-process\s+"dbus-daemon" nil t nil', form)
                and re.match(r'\(skip-unless\s+\(zerop\s+', form)
                and [case[s]['status'] for s in ('reference', 'candidate')] == ['skipped', 'skipped']
                and all('(signal file-missing ("Searching for program" '
                        '"No such file or directory" "dbus-daemon"))' in events[s]['condition']
                        and ' :value ' not in events[s]['condition'] for s in gate['missing_on']),
                'external daemon absence lacks its exact original missing-program error')
    else:
        scopes = {
            ('test/lisp/erc/erc-tests.el', 'erc--channel-modes/graphic-p'):
                ('(ert-skip "See non-/graphic-p variant")', 'See non-/graphic-p variant',
                 '(unless (char-displayable-p ?…)'),
        }
        scope = scopes.get((case['file'], case['id']))
        require(kind == 'direct-display-context' and scope is not None
                and form == scope[0] and scope[2] in source
                and gate.get('applicable_display') == 'cocoa',
                'unreviewed direct display skip remains blocking')
        for label in gate['missing_on']:
            metadata = file_row[label].get('metadata', {})
            require(metadata.get('display_mode') == 'batch' and metadata.get('noninteractive') is True
                    and events[label].get('condition') == '(ert-test-skipped "'+scope[1]+'")',
                    'direct display skip was not observed in the original batch context')
            if label == 'candidate' and profile.get('kind') == 'terminal':
                require(profile.get('native') in (True, False), 'declared terminal profile required')
            else:
                proofs.update(_passing_control(gate.get('controls', {}).get(label), case, label,
                    file_row, receipt, api, reference_selection, profile, display='cocoa', cache=cache))
    return proofs


def _component_version_applicability(gate, case, events, file_row, excerpts,
                                     receipt, reference_selection, source_anchors,
                                     *, root=None, api=None, profile=None, cache=None):
    """Bind a declared stable component against the selected reference API."""
    kind = gate.get('kind')
    ert = kind == 'ert-description-format'
    filenotify = (kind == 'filenotify-descriptor-api'
        and case['file'] == 'test/lisp/filenotify-tests.el'
        and case['id'] in {'file-notify-test07-many-events',
                          'file-notify-test07-many-events-remote', 'file-notify-test08-backup',
                          'file-notify-test08-backup-remote'})
    stopped_file = (kind == 'filenotify-stopped-event-file'
        and case['file'] == 'test/lisp/filenotify-tests.el'
        and case['id'] == 'file-notify-test12-unmount-remote')
    expected_keys = ({'kind', 'source_form', 'upstream_commit', 'reference_library',
                      'candidate_library', 'contract', 'controls'}
                     if stopped_file else
                     {'kind', 'source_form', 'upstream_commit', 'reference_library', 'candidate_library'})
    expected_form = (
        "(cdr (assq 'name desc))" if ert
        else '(string-prefix-p\n    file-notify--test-file\n    (file-notify--test-event-file file-notify--test-event))'
        if stopped_file
        else '(file-notify--rm-descriptor file-notify--test-desc)'
    )
    require(set(gate) == expected_keys
            and ((ert and case['file'] == 'test/lisp/emacs-lisp/ert-tests.el'
                    and case['id'] == 'ert-test-list-of-erts-file-tests') or filenotify or stopped_file)
            and [case[s]['status'] for s in ('reference', 'candidate')] == ['failed', 'passed']
            and gate['source_form'] == expected_form
            and (not stopped_file or (gate['contract'] == 'remote-watch-lifecycle-v1'
                                      and set(gate['controls']) == {'reference', 'candidate'}))
            and any(gate['source_form'] in text for text in excerpts)
            and reference_selection.get('upstream', {}).get('commit') == gate['upstream_commit']
            and re.fullmatch(r'[0-9a-f]{40}', gate['upstream_commit']) is not None,
            'unreviewed component version scope or actual outcomes differ')
    condition = events['reference'].get('condition', '')
    if ert:
        require(condition.startswith('(wrong-type-argument listp #s(ert--erts-file-test-description "foo" ')
                and condition.endswith(' 16 nil))'),
                'reference failure does not match the source-bound component API difference')
    elif stopped_file:
        match = re.fullmatch(
            r'\(ert-test-failed \(\(should \(string-prefix-p file-notify--test-file '
            r'\(file-notify--test-event-file file-notify--test-event\)\)\) '
            r':form \(string-prefix-p "([^"\n]+)" "([^"\n]+)"\) :value nil\)\)',
            condition)
        require(match is not None and match[1].startswith(match[2] + '/')
                and bool(match[1][len(match[2]) + 1:])
                and '/' not in match[1][len(match[2]) + 1:],
                'reference failure does not match the stopped-event parent-directory difference')
    else:
        require(condition == '(void-function file-notify--rm-descriptor)',
                'reference failure does not match the source-bound component API difference')
    proofs = {}
    for label in ('reference', 'candidate'):
        row, observed = file_row[label], case[label]
        require(row.get('complete') is True and row.get('exit_code') == 0
                and observed.get('complete') is True and observed.get('exit_code') == 0,
                'component version proof requires complete original processes')
        pin = gate[label+'_library']
        require(isinstance(pin, dict) and set(pin) == {'relative_path', 'bytes', 'sha256',
                'start_line', 'end_line', 'lf_sha256'}
                and pin['relative_path'] == ('emacs-lisp/ert.el' if ert else 'filenotify.el'),
                'an exact declared component source is required')
        environment = receipt.get('runtime_environment', {}).get(label, {})
        library = environment.get('FEMACS_STABLE_PRODUCT_LISP')
        require(isinstance(library, str) and Path(library).is_absolute()
                and row.get('runtime_environment') == environment,
                'component source must come from the admitted explicit product library')
        path = Path(library)/pin['relative_path']
        identity = {key: pin[key] for key in ('bytes', 'sha256')}
        raw = read_bytes(path, identity, 1024*1024)
        lines = raw.replace(b'\r\n', b'\n').split(b'\n')
        start, end = pin['start_line'], pin['end_line']
        require(type(start) is int and type(end) is int and 1 <= start <= end <= len(lines)
                and end-start <= 500, 'invalid component LF anchor range')
        text = b'\n'.join(lines[start-1:end])+b'\n'
        require(hashlib.sha256(text).hexdigest() == pin['lf_sha256'],
                'actual component source anchor differs')
        if label == 'reference':
            require((b'(ert--make-erts-file-test-description' in text and b':name name' in text)
                    if ert else (b'(file-notify--call-handler watch desc action file file1)' in text
                        and b"(when (eq action 'stopped)" in text)
                    if stopped_file else (b'(defun file-notify-rm-watch (descriptor)' in text
                        and b'(remhash descriptor file-notify-descriptors)' in text
                        and b'(defun file-notify--rm-descriptor ' not in raw),
                    'reference source does not establish its declared component API')
        else:
            require((b'(ert--signal-erts-file-test-execution `((name . ,name)' in text if ert
                        else (b"((eq action 'stopped)" in text and b'(file-notify-rm-watch desc)' in text)
                        if stopped_file
                        else b'(defun file-notify--rm-descriptor (descriptor)' in text)
                    and any(anchor.get('sha256') == pin['sha256']
                            and anchor.get('bytes') == pin['bytes'] for anchor in source_anchors),
                    'candidate component does not preserve its source-bound alist contract')
        proofs[str(path)] = identity
        if stopped_file:
            require(root is not None and api is not None and profile is not None,
                    'stopped-event version proof requires remote watch controls')
            local_cache = {} if cache is None else cache
            for counterpart in ('file-notify-test00-availability-remote', 'file-notify-test01-add-watch-remote',
                                'file-notify-test02-rm-watch-remote', 'file-notify-test03-events-remote'):
                proofs.update(_formal_counterpart(root, api, case, label, file_row, counterpart, local_cache))
            proofs.update(_remote_control(gate['controls'][label], receipt, label, profile,
                reference_selection, file_row, 'watch', local_cache, root=root, api=api))
    return proofs


def _optional_applicability(gate, case, events, file_row, excerpts, receipt,
                            api, reference_selection, profile, cache, root=None,
                            source_anchors=()):
    """Validate typed source gates without inventing names in Lisp features."""
    require(isinstance(gate, dict), 'typed optional applicability gate required')
    kind, missing = gate.get('kind'), gate.get('missing_on')
    require(isinstance(missing, list) and bool(missing) and len(set(missing)) == len(missing)
            and set(missing) == {s for s in ('reference', 'candidate') if case[s]['status'] == 'skipped'},
            'applicability must explain every actual skipped side exactly')
    if kind in {'file-monitor-backend', 'file-monitor-identity', 'external-message', 'direct-display-context',
                'external-program-missing'}:
        return _direct_skip_applicability(gate, case, events, file_row, excerpts, receipt,
            root, api, reference_selection, profile, cache)
    form = gate.get('source_form')
    require(isinstance(form, str) and 0 < len(form) <= 8192
            and any(form in text for text in excerpts), 'source gate is outside the pinned anchors')
    forms = gate.get('condition_forms')
    require(isinstance(forms, dict) and set(forms) == set(missing), 'exact observed gate forms required')
    proofs = {}
    for label in missing:
        condition = events[label].get('condition', '')
        observed_form = forms[label]
        inverse = form.startswith('(skip-when ')
        values = gate.get('condition_values', {})
        require(isinstance(values, dict) and set(values) <= set(missing), 'invalid observed gate values')
        value = values.get(label, 't' if inverse else 'nil')
        require(isinstance(value, str) and 0 < len(value) <= 8192
                and (value != 'nil' if inverse else value == 'nil'),
                'recorded gate truth contradicts the original skip direction')
        require(isinstance(observed_form, str) and 0 < len(observed_form) <= 8192
                and isinstance(condition, str) and condition.startswith('(ert-test-skipped ')
                and ' :form '+observed_form+' :value '+value in condition,
                'source gate does not match the actual skipped condition and value')
        metadata = file_row[label].get('metadata', {})
        if kind in {'feature-absent', 'feature-present'}:
            feature = gate.get('feature')
            require(isinstance(feature, str) and bool(feature)
                    and isinstance(metadata.get('features'), list)
                    and ('featurep' in observed_form or 'featurep' in form), 'feature gate lacks actual feature facts')
            if kind == 'feature-present':
                require(feature in metadata['features'] and '(skip-when ' in form
                        and ("'"+feature in form)
                        and all(case[s]['status'] == 'skipped' for s in ('reference', 'candidate')),
                        'inverse required-feature test needs matching inactive configuration outcomes')
            else:
                require(feature not in REQUIRED_FEATURES and feature not in metadata['features'],
                        'required capability or observed present feature cannot be waived')
                require("'"+feature in form and feature in observed_form,
                        'missing feature name does not match the source and actual predicate')
                if label == 'candidate' and feature == 'native-compile':
                    require(profile.get('native') is False, 'declared native capability remains required')
                if label == 'candidate' and feature in {'ns', 'window-system'}:
                    require(profile.get('kind') == 'terminal', 'declared GUI capability remains required')
        elif kind == 'native-compiler-absent':
            require(label == 'candidate' and profile.get('native') is False
                    and metadata.get('native_comp_available') is False
                    and 'native-comp-available-p' in form and 'native-comp-available-p' in observed_form,
                    'declared native capability or unobserved compiler absence cannot be excused')
        elif kind == 'configuration-token-absent':
            token = gate.get('token')
            require(token in CONFIGURATION_TOKENS
                    and isinstance(metadata.get('configuration_features'), str)
                    and token not in metadata['configuration_features'].split()
                    and token in form, 'optional configuration token absence is not observed')
        elif kind == 'external-lexical-binding':
            require((case['file'], case['id']) in {
                        ('test/lisp/emacs-lisp/bytecomp-tests.el', 'bytecomp-tests--dest-mountpoint'),
                        ('test/src/emacs-tests.el', 'emacs-tests/bwrap/allows-stdout')}
                    and form == '(skip-unless bwrap)' and observed_form == 'bwrap'
                    and gate.get('source_binding') == '(bwrap (executable-find "bwrap"))'
                    and gate['source_binding'] in '\n'.join(excerpts)
                    and all(case[s]['status'] == 'skipped' for s in ('reference', 'candidate')),
                    'lexical external prerequisite is not its original nil binding')
        elif kind == 'external-package-binding':
            source = '\n'.join(excerpts)
            require(case['file'] == 'test/lisp/net/ntlm-tests.el'
                    and case['id'] in {'ntlm-authentication', 'ntlm-authentication-old-compatibility-level'}
                    and form == '(skip-unless ntlm-tests--dependencies-present)'
                    and observed_form == 'ntlm-tests--dependencies-present'
                    and gate.get('packages') == ['url-http-ntlm', 'web-server']
                    and '(defvar ntlm-tests--dependencies-present' in source
                    and all("(featurep '"+name+')' in source for name in gate['packages'])
                    and '(version<= "2.0.4"' in source
                    and isinstance(metadata.get('features'), list)
                    and any(name not in metadata['features'] for name in gate['packages'])
                    and all(case[s]['status'] == 'skipped' for s in ('reference', 'candidate')),
                    'optional package binding has no observed missing loaded prerequisite')
        elif kind == 'external-predicate':
            predicate = gate.get('predicate')
            allowed = {
                'executable': {'executable-find', 'locate-file', 'ert-gcc-is-clang-p',
                               'tramp-get-remote-stat', 'file-exists-p'},
                'grammar': {'treesit-language-available-p', 'treesit-ready-p'},
                'package': {'require', 'locate-library', 'python-shell-internal-send-string',
                            'php-mode', 'markdown-mode', 'zig-ts-mode'},
                'service': {'tramp-archive-enabled', 'tramp-gvfs-enabled', 'secrets-enabled',
                            'dbus-ping', 'dbus-get-unique-name',
                            'dbus--test-enabled-session-bus', 'dbus--test-enabled-system-bus'},
                'optional-api': {'sqlite-load-extension', 'imagemagick-types', 'debug-timer-check',
                                 'image-type-available-p'},
            }
            require(predicate in allowed.get(gate.get('dependency_type'), set())
                    and predicate in form and predicate in observed_form,
                    'unrecognized external or optional subcapability predicate')
            if gate['dependency_type'] != 'optional-api':
                require(all(case[s]['status'] == 'skipped' for s in ('reference', 'candidate')),
                        'asymmetric external availability needs diagnosis, not an optional waiver')
            # A recorded nil predicate is evidence of its gate, not a claim
            # that the executable, grammar or daemon is a Lisp feature.
        elif kind == 'platform':
            systems = gate.get('applicable_systems')
            windows_api = ((case['file'], case['id']) ==
                ('test/lisp/subr-tests.el', 'shell-quote-argument-%-on-w32')
                and form == "(skip-unless (and (fboundp 'w32-shell-dos-semantics)\n"
                            "                    (w32-shell-dos-semantics)))"
                and set(systems or []) == {'windows-nt'}
                and observed_form == '(and (fboundp w32-shell-dos-semantics) (w32-shell-dos-semantics))')
            require(isinstance(systems, list) and bool(systems)
                    and set(systems) <= {'gnu/linux', 'darwin', 'windows-nt', 'ms-dos', 'cygwin', 'berkeley-unix'}
                    and metadata.get('system_type') not in systems
                    and ('system-type' in form or windows_api)
                    and isinstance(metadata.get('system_type'), str)
                    and all(case[s]['status'] == 'skipped' for s in ('reference', 'candidate')),
                    'platform gate does not establish non-applicability')
        elif kind == 'platform-helper':
            require(case['file'] == 'test/lisp/gnus/mml-sec-tests.el'
                    and form == '(skip-unless (test-conf))' and observed_form == '(test-conf)'
                    and gate.get('source_helper') == "(and (not (eq system-type 'darwin))\n       (ignore-errors (epg-find-configuration 'OpenPGP)))"
                    and gate['source_helper'] in '\n'.join(excerpts)
                    and '(defun test-conf ()' in '\n'.join(excerpts)
                    and metadata.get('system_type') == 'darwin'
                    and all(case[s]['status'] == 'skipped' for s in ('reference', 'candidate')),
                    'original platform helper does not prove an inactive test scope')
        elif kind == 'version-scope':
            majors = gate.get('applicable_major_versions')
            version = metadata.get('product_version', '')
            source = '\n'.join(excerpts)
            tramp_helper = (case['file'] == 'test/lisp/net/tramp-tests.el'
                and case['id'] in {'tramp-test46-dired-compress-dir', 'tramp-test46-dired-compress-file'}
                and form == '(skip-unless (not (tramp--test-emacs29-p)))'
                and observed_form == '(not t)' and majors == list(range(1, 29))
                and '(defun tramp--test-emacs29-p ()' in source
                and '(>= emacs-major-version 29)' in source)
            require(isinstance(majors, list) and bool(majors)
                    and all(type(v) is int and 1 <= v <= 100 for v in majors)
                    and isinstance(version, str) and version.split('.')[0].isdigit()
                    and int(version.split('.')[0]) not in majors
                    and (tramp_helper or any(p in form for p in
                        ('emacs-major-version', 'version<', 'version<=', 'emacs-version')))
                    and all(case[s]['status'] == 'skipped' for s in ('reference', 'candidate')),
                    'version-specific test does not have a proven inactive version scope')
        elif kind == 'host-facility':
            predicates = {
                'locale': {'locale-info', 'set-locale-environment', 'string-collate-lessp', 'get-locale-names'},
                'collation-locale': {'fns-tests--collate-enabled-p'},
                'non-loopback-hostname': {'network-test--resolve-system-name'},
                'global-unicast-ipv6': {'network-interface-list', 'network-interface-info', 'ipv6-is-available'},
                'dictionary-data': {'file-readable-p', 'file-exists-p'},
                'info-manual': {'file-readable-p', 'file-exists-p', 'locate-file'},
                'filesystem-acl': {'file-acl', 'file-extended-attributes'},
                'selinux': {'file-selinux-context'},
                'file-monitor-backend': {'file-notify-supported-p'},
            }
            predicate = gate.get('predicate')
            # ERT folds the inner equal before printing the outer not. Its
            # original nil skip, source form and exact event remain bound.
            folded_selinux = (gate.get('facility') == 'selinux'
                and predicate == 'file-selinux-context' and observed_form == '(not t)'
                and "'(nil nil nil nil)" in form)
            folded_locale = (case['file'] == 'test/src/casefiddle-tests.el'
                and case['id'] == 'casefiddle-turkish' and predicate == 'get-locale-names'
                and form == '(skip-unless (member "tr_TR.utf8" (get-locale-names)))'
                and observed_form.startswith('(member "tr_TR.utf8" (')
                and observed_form.endswith('))') and observed_form.count('"tr_TR.utf8"') == 1)
            require(predicate in predicates.get(gate.get('facility'), set())
                    and predicate in form and (predicate in observed_form or folded_selinux or folded_locale)
                    and all(case[s]['status'] == 'skipped' for s in ('reference', 'candidate')),
                    'unknown host facility; required paths and unexplained environment failures remain blocking')
            source = '\n'.join(excerpts)
            helpers = {
                'fns-tests--collate-enabled-p': ('test/src/fns-tests.el',
                    {'fns-tests-collate-sort', 'fns-tests-collate-strings'},
                    ['(defun fns-tests--collate-enabled-p ()', '(string-collate-equalp "" "" t)', 'en_US.UTF-8']),
                'network-test--resolve-system-name': ('test/lisp/net/network-stream-tests.el',
                    {'echo-server-with-dns'}, ['(network-lookup-address-info (system-name))',
                    '(= (elt address 0) 127)', '(equal address [0 0 0 0 0 0 0 1 0])']),
                'ipv6-is-available': ('test/lisp/net/nsm-tests.el', {'nsm-check-local-subnet-ipv6'},
                    ["(featurep 'make-network-process '(:family ipv6))",
                     '(= (logand (aref elt 0) #xe000) #x2000)', '(network-interface-list)']),
                'get-locale-names': ('test/src/casefiddle-tests.el', {'casefiddle-turkish'},
                    ['(member "tr_TR.utf8" (get-locale-names))']),
            }
            if predicate in helpers:
                expected_file, expected_ids, markers = helpers[predicate]
                require(case['file'] == expected_file and case['id'] in expected_ids
                        and all(marker in source for marker in markers),
                        'host helper scope or literal prerequisite implementation changed')
        elif kind == 'python-readline-backend':
            source = '\n'.join(excerpts)
            require(case['file'] == 'test/lisp/progmodes/python-tests.el'
                    and case['id'] == 'python-shell-completion-at-point-jedi-completer'
                    and form == '(skip-unless (string= python-shell-readline-completer-delims ""))'
                    and re.fullmatch(r'\(string= "(?:[^"\\]|\\.)+" ""\)', observed_form)
                    and all(marker in source for marker in (
                        '(defun python-tests--pythonstartup-file ()',
                        '(getenv "PYTHONSTARTUP")', '"-m" "jedi" "repl"',
                        '(python-shell-with-shell-buffer',
                        'readline.get_completer_delims()',
                        '(defvar python-shell-readline-completer-delims nil')),
                    'Jedi integration requires the exact observed nonempty readline delimiter gate')
            # The original shell directly sampled its configured completer.
            # This does not establish that the external package is uninstalled.
        elif kind == 'empty-known-failures':
            source = '\n'.join(excerpts)
            require(case['file'] == 'test/lisp/international/ucs-normalize-tests.el'
                    and case['id'] == 'ucs-normalize-part1-failing'
                    and form == '(skip-unless ucs-normalize-tests--failing-lines-part1)'
                    and observed_form == 'ucs-normalize-tests--failing-lines-part1'
                    and re.search(r'\(defconst ucs-normalize-tests--failing-lines-part1\s+\(list\s*\)\)', source)
                    and ':expected-result :failed' in source
                    and all(case[s]['status'] == 'skipped' for s in ('reference', 'candidate')),
                    'inactive known-failure variant must bind its actual empty source set')
        elif kind == 'monitor-scope':
            scopes = {
                'file-notify-test04-autorevert-remote': ("(skip-when (eq (file-notify--test-monitor) 'GKqueueFileMonitor))",
                    '(eq GKqueueFileMonitor GKqueueFileMonitor)'),
                'file-notify-test10-sufficient-resources-remote': ('(skip-unless (string-equal (file-notify--test-library) "kqueue"))',
                    '(string-equal "gio" "kqueue")'),
                'file-notify-test11-symlinks': ('(skip-when (string-equal (file-notify--test-library) "kqueue"))',
                    '(string-equal "kqueue" "kqueue")'),
            }
            require(case['file'] == 'test/lisp/filenotify-tests.el' and case['id'] in scopes
                    and (form, observed_form) == scopes[case['id']]
                    and metadata.get('system_type') == 'darwin'
                    and all(case[s]['status'] == 'skipped' for s in ('reference', 'candidate')),
                    'only the exact source-excluded monitor scope is non-applicable')
        elif kind == 'entrypoint-variant':
            require(case['file'] == 'test/lisp/progmodes/perl-mode-tests.el'
                    and '(skip-unless (eq cperl-test-mode' in form
                    and observed_form == '(eq perl-mode cperl-mode)',
                    'unrecognized original entrypoint variant')
            proofs.update(_passing_control(gate.get('controls', {}).get(label), case, label,
                file_row, receipt, api, reference_selection, profile,
                counterpart='test/lisp/progmodes/cperl-mode-tests.el', cache=cache))
        elif kind == 'parameter-variant':
            if gate.get('control_source') == 'same-file':
                base, separator, variant = case['id'].rpartition('/')
                source = '\n'.join(excerpts)
                installer = ('package-vc-tests-checkout-from-elpa-install-from-checkout'
                    if variant in {'test-package-three', 'test-package-eight'}
                    else 'package-vc-tests-checkout-with-git-install-from-checkout')
                require(case['file'] == 'test/lisp/emacs-lisp/package-vc-tests.el'
                        and separator and base in {'package-vc-tests-pkg-spec-info-manual',
                                                  'package-vc-tests-pkg-spec-make-shell-command'}
                        and variant in {'test-package-three', 'test-package-four',
                                        'test-package-six', 'test-package-eight'}
                        and gate.get('control_test_id') == base+'/test-package-one'
                        and isinstance(gate.get('shared_test_body'), str)
                        and bool(gate['shared_test_body']) and gate['shared_test_body'] in source
                        and re.search(r'\(test-package-def\s+'+re.escape(variant)
                            +r'\s+tests-dir\s+(?:nil|"lisp")\s+'
                            +re.escape(installer)+r'\)', source) is not None
                        and '(format "package-vc-tests-%s/%s" name pkg)' in source
                        and all(token in source for token in
                            ('package-vc-tests-install-from-elpa', 'package-vc-tests-install-from-spec',
                             'package-vc-tests-checkout-from-elpa-install-from-checkout',
                             'package-vc-tests-checkout-with-git-install-from-checkout'))
                        and 'caddr (alist-get pkg (package-vc-tests-packages t))' in form
                        and all(case[s]['status'] == 'skipped' for s in ('reference', 'candidate')),
                        'package parameter exclusion lacks its source and applicable original sibling')
                proofs.update(_formal_counterpart(root, api, case, label, file_row,
                    gate['control_test_id'], cache))
                continue
            control = gate.get('controls', {}).get(label, {})
            require(case['file'] == 'test/lisp/emacs-lisp/package-vc-tests.el'
                    and isinstance(control.get('test_id'), str) and control['test_id'] != case['id']
                    and isinstance(gate.get('shared_test_body'), str)
                    and bool(gate['shared_test_body'])
                    and gate['shared_test_body'] in ''.join(excerpts),
                    'original parameter variant requires its shared source body and different applicable ID')
            proofs.update(_passing_control(control, case, label, file_row, receipt,
                                          api, reference_selection, profile, cache=cache))
        elif kind in {'display-context', 'display-helper'}:
            display = gate.get('applicable_display')
            helper = (kind == 'display-helper' and case['file'] == 'test/lisp/server-tests.el'
                and case['id'] in {'server-tests/emacsclient/create-frame',
                                  'server-tests/server-force-stop/keeps-frames',
                                  'server-tests/server-start/stop-prompt-with-client'}
                and form == '(skip-unless server-tests/can-create-frames-p)'
                and observed_form == 'server-tests/can-create-frames-p' and display == 'terminal'
                and '(defconst server-tests/can-create-frames-p' in '\n'.join(excerpts)
                and '(getenv "TERM")' in '\n'.join(excerpts))
            require(display in {'terminal', 'cocoa'}
                    and metadata.get('display_mode') == 'batch'
                    and metadata.get('noninteractive') is True
                    and (helper or any(p in form and p in observed_form for p in
                            ('display-graphic-p', 'display-images-p', 'noninteractive',
                             'current-message', 'tty-type', 'getenv'))),
                    'display boundary is not an actual batch-context gate')
            if display == 'cocoa' and label == 'candidate' and profile.get('kind') == 'terminal':
                require(profile.get('native') in (True, False), 'declared terminal profile required')
            else:
                proofs.update(_passing_control(gate.get('controls', {}).get(label), case, label,
                    file_row, receipt, api, reference_selection, profile, display=display, cache=cache))
        elif kind == 'display-svg-subcapability':
            local_cache = {} if cache is None else cache
            source = '\n'.join(excerpts)
            purpose = 'current-version' if gate.get('control_corpus') == 'current' else 'applicability'
            require((case['file'], case['id']) in {
                        ('test/lisp/image-tests.el', 'image-create-image-with-map'),
                        ('test/lisp/image-tests.el', 'image--compute-map-and-original-map'),
                    }
                    and gate.get('applicable_display') == 'cocoa'
                    and form == '(skip-unless (display-images-p))'
                    and observed_form == '(display-images-p)'
                    and gate.get('control_test_id') == 'image-supported-file-p/built-in'
                    and gate.get('control_corpus') == (
                        'current' if case['id'] == 'image--compute-map-and-original-map' else 'stable')
                    and isinstance(gate.get('svg_call_form'), str)
                    and gate['svg_call_form'] in source
                    and metadata.get('display_mode') == 'batch'
                    and metadata.get('noninteractive') is True
                    and all(case[s]['status'] == 'skipped' for s in ('reference', 'candidate')),
                    'SVG subcapability rule lacks its exact original image-display gate and SVG call')
            if label == 'reference':
                ref_control = gate.get('controls', {}).get('reference')
                proofs.update(_passing_control(ref_control, case, 'reference', file_row,
                    receipt, api, reference_selection, profile, display='cocoa',
                    purpose=purpose, cache=local_cache))
                ref_val, ref_obs, ref_ev = _control_case_event(
                    ref_control, case['file'], gate['control_test_id'], local_cache)
                require(SVG_CONFIGURATION_TOKEN in metadata.get('configuration_features', '').split()
                        and SVG_CONFIGURATION_TOKEN in ref_val.get('metadata', {}).get('configuration_features', '').split()
                        and api.assess_case(gate['control_test_id'], ref_obs)['passed'] is True
                        and ref_ev.get('status') == 'passed'
                        and (purpose != 'current-version' or any(
                            a.get('sha256') == ref_control.get('test_input', {}).get('identity', {}).get('sha256')
                            and a.get('bytes') == ref_control.get('test_input', {}).get('identity', {}).get('bytes')
                            for a in source_anchors)),
                        'SVG-enabled reference must pass both target and built-in image control in Cocoa')
            elif profile.get('kind') == 'terminal':
                require(profile.get('native') in (True, False)
                        and 'candidate' not in gate.get('controls', {}),
                        'declared terminal profile required')
            else:
                cand_control = gate.get('controls', {}).get('candidate')
                require(isinstance(cand_control, dict)
                        and cand_control.get('test_id') == gate['control_test_id']
                        and SVG_CONFIGURATION_TOKEN not in metadata.get('configuration_features', '').split(),
                        'Cocoa candidate must lack SVG token and supply built-in image control')
                proofs.update(_passing_control(cand_control, case, 'candidate', file_row,
                    receipt, api, reference_selection, profile, display='cocoa',
                    purpose=purpose, cache=local_cache))
                cand_val, cand_obs, cand_ev = _control_case_event(
                    cand_control, case['file'], case['id'], local_cache)
                cand_cond = cand_ev.get('condition') or ''
                require(SVG_CONFIGURATION_TOKEN not in cand_val.get('metadata', {}).get('configuration_features', '').split()
                        and cand_obs.get('status') == 'failed'
                        and cand_ev.get('status') == 'failed'
                        and 'Invalid image type' in cand_cond
                        and 'svg' in cand_cond.lower()
                        and (purpose != 'current-version' or any(
                            a.get('sha256') == cand_control.get('test_input', {}).get('identity', {}).get('sha256')
                            and a.get('bytes') == cand_control.get('test_input', {}).get('identity', {}).get('bytes')
                            for a in source_anchors)),
                        'Cocoa candidate without librsvg must retain its raw SVG image-type failure')
        else:
            raise ValueError('unsupported applicability gate; unexplained skips remain blocking')
    return proofs


def _bootstrap_container_applicability(gate, case, events, file_row, excerpts, root, api, profile):
    """Explain this exact original assertion with fresh, sealed per-side proof."""
    form = '(should (= (call-process "cmp" nil nil nil comp1-eln comp2-eln) 0))'
    require(gate == {'kind': 'native-bootstrap-container', 'source_form': form}
            and case['file'] == 'test/src/comp-tests.el' and case['id'] == 'comp-tests-bootstrap'
            and [case[s]['status'] for s in ('reference', 'candidate')] == ['failed', 'failed']
            and type(profile.get('native')) is bool
            and any(form in text for text in excerpts)
            and any('(defvar native-comp-eln-load-path)' in text
                    and '(car (last native-comp-eln-load-path))' in text for text in excerpts)
            and any('(when (native-comp-available-p)' in text
                    and '(load (native-compile comp-test-src))' in text for text in excerpts),
            'bootstrap rule lacks its exact original comparison and availability boundary')
    module_path = Path(__file__).with_name('native-bootstrap-identity.py')
    module_identity = bound_file(module_path, limit=1024*1024)
    spec = importlib.util.spec_from_file_location('classified_bootstrap_identity', module_path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    proofs = {str(module_path): module_identity}
    expected_cmp = ('(ert-test-failed (' + form + ' :form (= 1 0) :value nil))')
    for label in ('reference', 'candidate'):
        metadata = file_row[label].get('metadata', {})
        native = label == 'reference' or profile['native']
        require(isinstance(metadata.get('features'), list)
                and metadata.get('native_comp_available') is native
                and ('native-compile' in metadata['features']) is native
                and case[label].get('complete') is True and case[label].get('exit_code') == 0
                and file_row[label].get('complete') is True and file_row[label].get('exit_code') == 0,
                'bootstrap capability or original process boundary is not proven')
        if not native:
            require(events[label].get('condition') == '(void-variable native-comp-eln-load-path)',
                    'native-disabled bootstrap failed for another reason')
            continue
        require(events[label].get('condition') == expected_cmp,
                'native-enabled bootstrap did not fail the exact original cmp assertion')
        pin = file_row[label].get('file_result', {})
        file_path = root/api._relative_path(pin.get('path'), 'bootstrap file evidence')
        actual = module.compare_closed_file(root, file_path)
        closure = actual.get('file_closure', {})
        require(actual.get('file_closure_bound') is True
                and actual.get('differs_only_in_validated_metadata') is True
                and actual.get('inputs_unchanged_at_exit') is True
                and closure.get('product') == label
                and closure.get('event_sha256') == case[label].get('evidence', {}).get('event_sha256')
                and closure.get('event_ordinal') == case[label].get('evidence', {}).get('event_ordinal')
                and closure.get('original_condition') == events[label].get('condition')
                and {k: closure.get('file', {}).get(k) for k in ('bytes', 'sha256')}
                    == {k: pin.get(k) for k in ('bytes', 'sha256')},
                'bootstrap images do not prove this exact original file and event')
        for value in [*actual['inputs'], *closure['file_protocol_sources'],
                      *(closure[key] for key in ('file', 'owner', 'proof', 'progress', 'verifier'))]:
            proofs[value['path']] = {k:value[k] for k in ('bytes', 'sha256')}
    bound_file(module_path, module_identity, 1024*1024)
    return proofs


def _remote_collections(arguments):
    """Index complete owned remote controls; no category is inferred here."""
    require(len(arguments) <= 64, 'bounded remote-control collections required')
    result, inputs = [], {}
    for results, controller in arguments:
        results, controller = Path(os.path.abspath(results)), Path(os.path.abspath(controller))
        pin = dict(path=str(results), identity=bound_file(results, limit=MAX_ITEM))
        value = _control_json(pin, {})
        require(value.get('schema') == 'femacs-runtime-tramp-process-probe-v1'
                and value.get('passed') is True and value.get('diagnostic_collection_complete') is True
                and value.get('full_matrix_admission') is True
                and value.get('requested_contracts') == ['ready-remote-signal-delivery-v1', 'remote-watch-lifecycle-v1']
                and isinstance(value.get('cases'), list) and 0 < len(value['cases']) <= 49,
                'fresh completed remote behavior collection required')
        pin.update(controller=dict(path=str(controller), identity=bound_file(controller, limit=65536)),
                   owner=dict(path=str(results.parent/'artifact-preservation.json'),
                              identity=bound_file(results.parent/'artifact-preservation.json')))
        result.append(pin)
        inputs.update({pin['path']:pin['identity'], controller:pin['controller']['identity'],
                       pin['owner']['path']:pin['owner']['identity']})
    return result, {str(path):pin for path,pin in inputs.items()}


def _remote_control(pin, receipt, label, profile, reference_selection, file_row, contract, cache,
                    *, root=None, api=None):
    """Bind required behavior to actual image/dump, epoch, source and owner."""
    cache = {} if cache is None else cache
    require(isinstance(pin, dict), 'same-product remote behavior control required')
    value = _control_json(pin, cache)
    expected_checks = {'all61_product_identities_unchanged', 'all61_complete_receipts_valid',
        'source_export_unchanged', 'reference_selection_unchanged', 'driver_and_fixture_sources_unchanged',
        'captured_fixture_unchanged', 'original_test_unchanged'}
    require(value.get('schema') == 'femacs-runtime-tramp-process-probe-v1'
            and value.get('passed') is True and value.get('diagnostic_collection_complete') is True
            and value.get('full_matrix_admission') is True
            and value.get('actual_editor_execution') is True
            and value.get('original_formal_results_modified') is False
            and value.get('reference_selection') == reference_selection and value.get('errors') == []
            and set(value.get('checks', {})) == expected_checks and all(value['checks'].values()),
            'remote control has no exact complete product/source postflight')
    target = profile['id'] if label == 'candidate' else 'installed-reference'
    rows = [row for row in value['cases'] if row.get('profile') == target]
    require(len(rows) == 1, 'one exact remote-control profile required')
    row = rows[0]
    process, observed = row.get('process', {}), row.get('observation', {})
    argv, original_argv = process.get('argv', []), file_row[label].get('argv', [])
    require(row.get('collection_complete') is True
            and row.get('stable_product_identity') == receipt['products_before'][label]
            and row.get('runtime_environment') == receipt['runtime_environment'][label]
            and process.get('complete') is True and process.get('exit_code') == 0
            and process.get('timed_out') is False and process.get('remaining_group_terminated') is False
            and argv and original_argv and argv[0] == original_argv[0]
            and [a for a in argv if a.startswith('--dump-file=')]
                == [a for a in original_argv if a.startswith('--dump-file=')]
            and any(a.startswith('--dump-file=') for a in argv)
            and '--batch' in argv and '-Q' in argv
            and row.get('state_retirement', {}).get('status') == 'complete'
            and row['state_retirement'].get('all_selected_absent') is True,
            'remote control did not complete the same product/environment and owned lifecycle')
    container, state_root = Path(row.get('owned_container','')), Path(row.get('owned_state_root',''))
    require(container.parent == Path.home()/'AI/caches/.noindex'
            and state_root.parent == container and state_root.name == 'c'
            and observed.get('owned_remote_context') == dict(method='mock',local_directory=str(state_root/'tmp')),
            'remote control did not verify its own local mock scope before execution')
    transport = row.get('mock_transport', {})
    original_transport = file_row[label].get('mock_transport_context') or {}
    require(transport.get('profile') == original_transport.get('profile')
            and transport.get('content_sha256') == original_transport.get('content_sha256')
            and bool(transport.get('profile')), 'remote mock transport differs from the original case')
    library = Path(receipt['runtime_environment'][label]['FEMACS_STABLE_PRODUCT_LISP'])
    require(len(observed.get('library_files', [])) == 2 and all(
        isinstance(origin, str) and Path(origin).parent == library/'net'
        and Path(origin).stem == expected and Path(origin).suffix in {'.el', '.elc'}
        for origin, expected in zip(observed['library_files'], ('tramp', 'tramp-sh'))),
        'remote control loaded another product library')
    source = value.get('original_test' if contract == 'signal' else 'watch_original_test', {})
    working_fixture = file_row[label].get('working_fixture')
    if working_fixture is None:
        require(root is not None and api is not None, 'remote original file evidence root required')
        file_pin = file_row[label].get('file_result', {})
        file_path = Path(root)/api._relative_path(file_pin.get('path'), 'remote original file evidence')
        working_fixture = _control_json(dict(path=str(file_path), identity={
            k: file_pin.get(k) for k in ('bytes', 'sha256')}), cache).get('working_fixture', {})
    originals = [entry for entry in working_fixture.get('files', [])
        if entry['path'] == ('test/lisp/net/tramp-tests.el' if contract == 'signal' else 'test/lisp/filenotify-tests.el')]
    require(len(originals) == 1 and source.get('identity', {}).get('sha256') == originals[0].get('before', {}).get('sha256'),
        'remote control did not load the exact original input')
    proofs = {pin['path']:pin['identity'], source.get('path'):source.get('identity')}
    probe_root = Path(pin['path']).parent
    process_file = Path(process.get('evidence_directory', ''))/'process.json'
    require(process_file.parent.parent == probe_root and _control_json(dict(path=str(process_file),
            identity=bound_file(process_file)), cache) == process, 'remote process record is not bound to its collection')
    proofs[str(process_file)] = bound_file(process_file)
    observation_path = row.get('observation_path')
    require(Path(observation_path).parent == probe_root
            and _control_json(dict(path=observation_path, identity={key:row['observation_identity'][key]
                for key in ('bytes','sha256')}), cache) == observed, 'remote observations differ from their bound file')
    proofs[observation_path] = {key:row['observation_identity'][key] for key in ('bytes','sha256')}
    for relative in ('runtime-tramp-process-probe.py', 'fixtures/runtime-tramp-process-contract.el',
                     'fixtures/runtime-remote-watch-contract.el'):
        path = str(Path(__file__).resolve().parent/relative)
        identity = {key:value.get('input_pins', {}).get(path, {}).get(key) for key in ('bytes','sha256')}
        bound_file(path, identity, MAX_ITEM)
        proofs[path] = identity
    captures = value.get('captured_fixtures', {})
    for kind, name in (('signal', 'runtime-tramp-process-contract.el'), ('watch','runtime-remote-watch-contract.el')):
        captured = captures.get(kind, {})
        require(captured.get('path') == str(probe_root/name), 'remote fixture capture escaped its output')
        identity = {key:captured.get('identity', {}).get(key) for key in ('bytes','sha256')}
        require(identity == proofs[str(Path(__file__).resolve().parent/'fixtures'/name)],
                'remote fixture capture differs from its current source')
        bound_file(captured['path'], identity, MAX_ITEM)
        proofs[captured['path']] = identity
    require(['-l', captures['signal']['path']] in [argv[i:i+2] for i in range(len(argv)-1)],
            'remote process did not execute its captured authored fixture')
    if contract == 'signal':
        expected = [(kind, signal) for kind in ('process','name','pid','pid-string') for signal in ('2','INT')]
        observations = observed.get('signal_rows', [])
        require(observed.get('contract') == 'ready-remote-signal-delivery-v1'
                and observed.get('signal_contract_passed') is True
                and [(item.get('argument_kind'), item.get('signal')) for item in observations] == expected
                and all(item.get('passed') is True and item.get('ready') is True
                    and item.get('signal_return') == 0 and item.get('exit_status') == 17
                    and 'CONTRACT-SIGNAL\n' in item.get('output','') for item in observations),
                'required remote signal forms did not actually reach their installed handler and exit')
    else:
        watch = observed.get('watch_contract', {})
        observations = watch.get('rows', [])
        require(watch.get('contract') == 'remote-watch-lifecycle-v1'
                and watch.get('collection_complete') is True and watch.get('lifecycle_complete') is True
                and [(item.get('id')) for item in observations] == [
                    'remove-watch','delete-watched-directory','emulated-unmount']
                and all(item.get('passed') is True and item.get('valid_before') is True
                    and item.get('valid_after') is False for item in observations)
                and 'stopped' in observations[-1].get('events', [])
                and Path(watch.get('library_origin','')).parent == library
                and Path(watch['library_origin']).name in {'filenotify.el','filenotify.elc'},
                'required remote-watch validity/stop behavior did not actually pass')
        if label == 'candidate':
            require(watch.get('correct_behavior') is True
                    and observations[-1].get('stopped_file_matches_watched') is True
                    and observations[-1].get('stopped_local_file') == observations[-1].get('watched_local_file')
                    and isinstance(observations[-1].get('watched_local_file'),str),
                    'candidate stopped-event filename must identify its watched file')
    lines = read_bytes(pin['controller']['path'], pin['controller']['identity'], 65536).decode().splitlines()
    controllers = [json.loads(line[len('controlled-run: '):], object_pairs_hook=_pairs)
                   for line in lines if line.startswith('controlled-run: {')]
    require(len(controllers) == 1 and controllers[0].get('reason') == 'exited'
            and controllers[0].get('exit_code') == 0 and controllers[0].get('cleanup_complete') is True
            and isinstance(controllers[0].get('argv_sha256'), str) and len(controllers[0]['argv_sha256']) == 64,
            'remote controller did not exit and reap successfully')
    proofs.update(_control_owner(pin['owner'], controllers[0], probe_root, cache))
    proofs[pin['controller']['path']] = pin['controller']['identity']
    for path, identity in proofs.items():
        bound_file(path, {key:identity[key] for key in ('bytes','sha256')})
    return {path:{key:identity[key] for key in ('bytes','sha256')} for path,identity in proofs.items()}


def _select_remote_controls(collections, profile, cache):
    controls = {}
    for label, target in (('reference','installed-reference'), ('candidate',profile)):
        matches = [pin for pin in collections if any(row.get('profile') == target
            for row in _control_json(pin, cache).get('cases', []))]
        require(len(matches) == 1, 'one exact same-product remote behavior collection required')
        controls[label] = matches[0]
    return controls


def _correct_library_control(pin, receipt, label, profile, reference_selection, file_row):
    """Require an independent behavior check of the same actual product tree."""
    value = _control_json(pin, {})
    require(value.get('schema') == 'femacs-runtime-frame-buffer-probe-v1'
            and value.get('contract') == 'dframe-create-buffer-v1'
            and value.get('passed') is True and value.get('selected_inputs_unchanged') is True
            and value.get('generated_override_used') is False
            and value.get('reference_selection_sha256') == reference_selection['sha256'],
            'correct library behavior requires a fresh matching-epoch product control')
    required_profile = profile['id'] if label == 'candidate' else 'installed-reference'
    rows = [row for row in value.get('cases', []) if row.get('profile') == required_profile]
    require(len(rows) == 1 and rows[0].get('stable_product_identity') == receipt['products_before'][label]
            and rows[0].get('collection_complete') is True, 'library control product identity differs')
    row = rows[0]
    process, observation = row.get('process', {}), row.get('observation', {})
    library = Path(receipt['runtime_environment'][label]['FEMACS_STABLE_PRODUCT_LISP'])
    origin = observation.get('library_origin')
    expected_argv, argv = file_row[label].get('argv', []), process.get('argv', [])
    require(process.get('complete') is True and process.get('exit_code') == 0
            and process.get('remaining_group_terminated') is False
            and observation.get('contract') == 'dframe-create-buffer-v1'
            and observation.get('correct_behavior') is True
            and observation.get('noninteractive') is False
            and len(argv) >= 6 and expected_argv and argv[4] == expected_argv[0]
            and [a for a in argv if a.startswith('--dump-file=')]
                == [a for a in expected_argv if a.startswith('--dump-file=')]
            and bool([a for a in expected_argv if a.startswith('--dump-file=')])
            and ['-l', origin] in [argv[i:i+2] for i in range(len(argv)-1)]
            and isinstance(origin, str) and Path(origin).parent == library
            and Path(origin).name in {'dframe.el', 'dframe.elc'}
            and len(observation.get('rows', [])) == 2
            and all(r.get('passed') is True and r.get('callback_calls') == 1
                    and r.get('callback_buffer_correct') is True
                    and r.get('callback_frame_correct') is True
                    and r.get('continuation_correct') is True for r in observation['rows']),
            'candidate must satisfy callback and continuation behavior, not match a reference error')
    proofs = {pin['path']: pin['identity']}
    sources = value.get('input_pins', {})
    origin_pin = {key:sources.get(origin, {}).get(key) for key in ('bytes', 'sha256')}
    bound_file(origin, origin_pin, MAX_ITEM)
    proofs[origin] = origin_pin
    for name in ('runtime-frame-buffer-probe.py', 'fixtures/runtime-library-contracts.el'):
        path = str(Path(__file__).resolve().parent/name)
        identity = {key:sources.get(path, {}).get(key) for key in ('bytes', 'sha256')}
        bound_file(path, identity, MAX_ITEM)
        proofs[path] = identity
    return proofs


def reference_defect_state(rule, case, file_row, receipt, events, root, api):
    """Re-evaluate a reviewed defect on every epoch/run; never excuse candidate bugs.

    The original valid sidebar assertions are the executable behavior contract.
    A changed reference can resolve this record by passing that contract. An
    unknown reference failure cannot inherit the old source/signature proof.
    """
    gate = rule.get('reference_defect_gate')
    expected_paths = {'dframe.el', 'frame.el', 'term/ns-win.el', 'erc/erc-speedbar.el'}
    require(isinstance(gate, dict) and set(gate) == {'kind', 'contract', 'source_form',
            'reference_sources', 'expected_reference_condition', 'expected_child_condition'}
            and gate['kind'] == 'dframe-create-buffer'
            and gate['contract'] == 'dframe-create-buffer-v1'
            and gate['source_form'] == '(cl-assert (eq speedbar-buffer (current-buffer)))'
            and gate['expected_reference_condition'] ==
                '(ert-test-failed "Process erc-scenarios-status-sidebar--nickbar exited abnormally with code 1")'
            and gate['expected_child_condition'] ==
                '(cl-assertion-failed ((eq speedbar-buffer (current-buffer)) nil))'
            and set(gate['reference_sources']) == expected_paths
            and case['file'] == 'test/lisp/erc/erc-scenarios-status-sidebar.el'
            and case['id'] == 'erc-scenarios-status-sidebar--nickbar',
            'unreviewed reference defect or correct behavior contract')
    for label in ('reference', 'candidate'):
        row, observed = file_row[label], case[label]
        require(row.get('complete') is True and row.get('exit_code') == 0
                and row.get('timed_out') is False and row.get('remaining_group_terminated') is False
                and observed.get('complete') is True and observed.get('exit_code') == 0,
                'reference defect lifecycle requires complete original file processes')
    sources, proofs = {}, {}
    for label in ('reference', 'candidate'):
        environment = receipt.get('runtime_environment', {}).get(label, {})
        library = environment.get('FEMACS_STABLE_PRODUCT_LISP')
        require(isinstance(library, str) and Path(library).is_absolute()
                and file_row[label].get('runtime_environment') == environment,
                'reference defect sources require the admitted product library')
        sources[label] = {}
        for name in sorted(expected_paths):
            path = Path(library)/name
            pin = bound_file(path, limit=1024*1024)
            sources[label][name] = pin
            proofs[str(path)] = pin
    statuses = [case[s].get('status') for s in ('reference', 'candidate')]
    child_matches = False
    if statuses[0] == 'failed':
        trace = file_row['reference'].get('diagnostic_trace')
        if isinstance(trace, dict):
            path = Path(root)/api._relative_path(trace.get('path'), 'original terminal child trace')
            pin = {key:trace.get(key) for key in ('bytes', 'sha256')}
            bound_file(path, pin, 8*1024*1024)
            children = []
            descriptor = os.open(path, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
            with os.fdopen(descriptor, 'rb') as source:
                before = os.fstat(source.fileno())
                require(stat.S_ISREG(before.st_mode), 'ordinary child trace required')
                while raw := source.readline(65537):
                    require(len(raw) <= 65536 and raw.endswith(b'\n'), 'invalid bounded child result event')
                    value = json.loads(raw, object_pairs_hook=_pairs)
                    if (value.get('phase') == 'original-terminal-child-test-ended'
                            and value.get('test_id') == case['id']):
                        children.append(value)
                        require(len(children) <= 1, 'duplicate original child case result')
                after = os.fstat(source.fileno())
            current = path.lstat()
            require(all(getattr(before, key) == getattr(after, key) == getattr(current, key)
                    for key in ('st_dev','st_ino','st_size','st_mtime_ns','st_mode')),
                    'original child trace changed during read')
            argv = file_row['reference'].get('argv', [])
            if len(children) == 1 and argv:
                child = children[0]
                record = child.get('child_test_result', {})
                child_matches = (child.get('schema') == 'femacs-stable-runtime-observation-v1'
                    and child.get('file') == case['file'] and type(child.get('pid')) is int and child['pid'] > 0
                    and record.get('program') == argv[0]
                    and [record.get('dump')] == [a for a in argv if a.startswith('--dump-file=')]
                    and record.get('test_id') == case['id'] and record.get('status') == 'failed'
                    and record.get('condition') == gate['expected_child_condition'])
            bound_file(path, pin, 8*1024*1024)
            proofs[str(path)] = pin
    if statuses[0] == 'passed':
        state = ('resolved-upstream-fixed' if statuses[1] == 'passed'
                 else 'candidate-regression-after-upstream-fix')
    elif (sources['reference'] != gate['reference_sources']
            or events['reference'].get('condition') != gate['expected_reference_condition']
            or not child_matches
            or statuses[0] != 'failed'):
        state = 'reference-changed-revalidation-required'
    elif statuses[1] != 'passed':
        state = 'candidate-correct-behavior-required'
    else:
        state = 'active-reference-only-defect'
    return dict(contract=gate['contract'], state=state, sources=sources,
                candidate_original_case_passed=statuses[1] == 'passed',
                rule_retired=statuses[0] == 'passed', proof_inputs=proofs)


def reference_contract_labels(state):
    return {'resolved-upstream-fixed': ('reference', 'candidate'),
            'candidate-regression-after-upstream-fix': ('reference',),
            'active-reference-only-defect': ('candidate',)}.get(state, ())


def _accept_rule(rule, case, file_row, entry, root, api, reference_selection, profile, receipt, cache=None,
                 *, manifest=None):
    category = rule.get('category')
    require(category in CATEGORIES and isinstance(rule.get('reason'), str)
            and bool(rule['reason'].strip()), 'classification category and reason required')
    require(rule.get('input_sha256') == entry['sha256'], 'classified original test identity changed')
    actual = {label: observation_signature(case[label]) for label in ('reference', 'candidate')}
    require(rule.get('observations') == actual, 'classification does not match actual observations')
    events, proofs = {}, {}
    excerpts = _source_evidence(rule, entry)
    supporting = rule.get('supporting_evidence', [])
    require(isinstance(supporting, list) and len(supporting) <= 16,
            'supporting evidence must be a bounded list of explicit pins')
    for pin in supporting:
        require(isinstance(pin, dict) and set(pin) == {'path', 'identity'},
                'invalid supporting evidence binding')
        bound_file(pin['path'], pin['identity'], MAX_ITEM)
        proofs[pin['path']] = pin['identity']
    for label in ('reference', 'candidate'):
        evidence = case[label].get('evidence', {})
        require(evidence.get('path') == file_row[label].get('progress', {}).get('path'),
                'classified case progress identity differs from file')
        events[label] = _original_case_event(root, api, case, label)
    statuses = [case[label]['status'] for label in ('reference', 'candidate')]
    if category == 'upstream-invalid-test':
        invalid = rule.get('invalid_gate')
        if isinstance(invalid, dict) and invalid.get('kind') == 'native-bootstrap-container':
            proofs.update(_bootstrap_container_applicability(invalid, case, events, file_row,
                excerpts, Path(root), api, profile))
        elif isinstance(invalid, dict) and invalid.get('kind') == 'asynchronous-signal-liveness':
            failed = [label for label in ('reference','candidate') if case[label]['status'] == 'failed']
            require(set(invalid) == {'kind','source_form','contract','conditions','controls'}
                    and case['file'] == 'test/lisp/net/tramp-tests.el' and case['id'] == 'tramp-test31-signal-process'
                    and bool(failed) and set(statuses) <= {'failed','passed'}
                    and invalid['source_form'] == '(while (accept-process-output proc 0 nil t))'
                    and invalid['contract'] == 'ready-remote-signal-delivery-v1'
                    and '(should-not (process-live-p proc))' in '\n'.join(excerpts)
                    and '(signal-process (funcall func proc) sigcode)' in '\n'.join(excerpts)
                    and invalid['conditions'] == {label:events[label].get('condition') for label in failed}
                    and set(invalid['controls']) == {'reference','candidate'}
                    and all(re.fullmatch(r'\(ert-test-failed \(\(should-not \(process-live-p proc\)\) '
                        r':form \(process-live-p #<process test[12]-(?:2|INT)>\) '
                        r':value \(run open listen connect stop\)\)\)', events[label].get('condition',''))
                        for label in failed),
                    'only the original immediate-liveness assertion has an asynchronous contract mismatch')
            for label in ('reference','candidate'):
                proofs.update(_remote_control(invalid['controls'][label], receipt, label, profile,
                    reference_selection, file_row, 'signal', {} if cache is None else cache,
                    root=root, api=api))
        elif isinstance(invalid, dict) and invalid.get('kind') == 'wall-clock-format-precision':
            failed = [label for label in ('reference', 'candidate') if case[label]['status'] == 'failed']
            source = '\n'.join(excerpts)
            require(set(invalid) == {'kind', 'source_form', 'control_test_id', 'conditions'}
                    and case['file'] == 'test/lisp/erc/erc-scenarios-internal.el'
                    and case['id'] == 'erc-d-run-linger'
                    and bool(failed) and set(statuses) <= {'failed', 'passed'}
                    and invalid['source_form'] == '(erc-d-t-search-for 3 "Lingered for 1.00 seconds")'
                    and invalid['control_test_id'] == 'erc-d-run-linger-direct'
                    and all(marker in source for marker in (
                        '(load "erc-d-tests" nil \'silent)',
                        '(erc-d-t-search-for 2 "Lingering for 1.00 seconds")',
                        '(erc-d-t-search-for 3 "Lingered for 1.00 seconds")',
                        '"Lingered for %.2f seconds"',
                        '(float-time (time-subtract (current-time) start))',
                        '(ert-deftest erc-d-run-linger-direct ()',
                        '(error "Failed awaiting: %s" ,msg)'))
                    and invalid['conditions'] == {label: events[label].get('condition') for label in failed}
                    and all(events[label].get('condition')
                        == '(error "Failed awaiting: string: Lingered for 1.00 seconds")'
                        for label in failed),
                    'only the exact centisecond wall-clock formatting jitter in erc-d-run-linger is covered')
            local_cache = {} if cache is None else cache
            for label in ('reference', 'candidate'):
                proofs.update(_formal_counterpart(root, api, case, label, file_row,
                    invalid['control_test_id'], local_cache))
        elif isinstance(invalid, dict) and invalid.get('kind') == 'unpopulated-graphical-snapshot-wrapper':
            source = '\n'.join(excerpts)
            local_cache = {} if cache is None else cache
            manifest_entries = manifest.get('entries', []) if isinstance(manifest, dict) else []
            require(statuses == ['skipped', 'skipped']
                    and set(invalid) == {'kind', 'source_form', 'conditions', 'controls'}
                    and len(rule['source_evidence']) == 3
                    and case['file'] == 'test/lisp/erc/erc-scenarios-internal.el'
                    and case['id'] == 'erc-scenarios-internal--run-graphical-all'
                    and invalid['source_form'] == '(ert-skip "Environmental conditions unmet")'
                    and all(marker in source for marker in (
                        '(unless (and (getenv "ERC_TESTS_GRAPHICAL_ALL")',
                        '(not (getenv "ERC_TESTS_GRAPHICAL"))',
                        '(not (getenv "CI")))',
                        '(ert (quote (tag :erc--graphical)))',
                        '(erc-tests-common-create-subprocess program',
                        '(should (zerop (process-exit-status proc)))',
                        ';; On graphical displays, echo .graphic >> .git/info/exclude',
                        '(defvar erc-fill-tests--graphic-dir "fill/snapshots/.graphic/")',
                        ",@(and (getenv \"ERC_TESTS_GRAPHICAL\") '(:erc--graphical))",
                        '(erc-fill-tests--compare "monospace-01-start")',
                        '(insert-file-contents-literally expect-file)',
                        '(if (file-exists-p expect-file)',
                        ',(concat invocation-directory invocation-name)'))
                    and source.index('(insert-file-contents-literally expect-file)')
                        < source.index('(if (file-exists-p expect-file)')
                    and any(e.get('path', '').startswith('test/lisp/erc/resources/fill/snapshots/')
                            and e.get('path', '').endswith('.eld') for e in manifest_entries)
                    and not any('/fill/snapshots/.graphic/' in e.get('path', '') for e in manifest_entries)
                    and invalid['conditions'] == {s: events[s].get('condition') for s in ('reference', 'candidate')},
                    'graphical snapshot wrapper lacks its exact upstream exclusion and unpopulated snapshot proof')
            for label in ('reference', 'candidate'):
                metadata = file_row[label].get('metadata', {})
                pin = file_row[label].get('file_result', {})
                path = Path(root)/api._relative_path(pin.get('path'), 'graphical wrapper file evidence')
                identity = {k: pin.get(k) for k in ('bytes', 'sha256')}
                closed = _control_json(dict(path=str(path), identity=identity), local_cache)
                require(metadata.get('display_mode') == 'batch' and metadata.get('noninteractive') is True
                        and 'ERC_TESTS_GRAPHICAL_ALL' not in metadata.get('test_environment', {})
                        and events[label].get('condition') == '(ert-test-skipped "Environmental conditions unmet")'
                        and closed.get('file') == case['file']
                        and closed.get('selector') == 'all-discovered-ids'
                        and case['id'] in closed.get('ids', []),
                        'graphical snapshot wrapper skip was not observed in its complete original batch file')
                proofs[str(path)] = identity
                proofs.update(_formal_counterpart(root, api, case, label, file_row,
                    'erc-d-run-linger-direct', local_cache))
            if profile.get('kind') == 'terminal':
                require(profile.get('native') in (True, False) and invalid['controls'] == {},
                        'terminal profile cannot supply graphical scenario controls')
            else:
                require(profile.get('kind') == 'cocoa'
                        and set(invalid['controls']) == {'reference', 'candidate'},
                        'cocoa profile requires passing graphical scenario controls on both sides')
                for label in ('reference', 'candidate'):
                    items = invalid['controls'][label]
                    require(isinstance(items, list) and len(items) == len(ERC_GRAPHICAL_SCENARIO_CONTROLS),
                            'every applicable graphical ERC scenario control is required')
                    for (target_file, target_id), item in zip(ERC_GRAPHICAL_SCENARIO_CONTROLS, items):
                        require(isinstance(item, dict) and item.get('test_id') == target_id,
                                'graphical scenario control test identity mismatch')
                        proofs.update(_passing_control(item, case, label, file_row, receipt, api,
                            reference_selection, profile, display='cocoa', counterpart=target_file,
                            test_environment=ERC_GRAPHICAL_TEST_ENVIRONMENT, cache=local_cache))
        elif isinstance(invalid, dict) and invalid.get('kind') == 'display-gated-upstream-broken-test':
            source = '\n'.join(excerpts)
            local_cache = {} if cache is None else cache
            specs = {
                ('test/lisp/hl-line-tests.el', 'hl-line-tests-sticky-across-frames'): (
                    'cocoa',
                    '(skip-unless (display-graphic-p))',
                    '(should (hl-line-tests-verify 111 t))',
                    'hl-line-tests-sticky',
                    ("(call-interactively #'global-hl-line-mode)",
                     "(cl-some (apply-partially #'eq hl-line-overlay)"),
                    '(ert-test-failed ((should (hl-line-tests-verify 111 t)) :form ',
                ),
                ('test/lisp/emacs-lisp/eieio-tests/eieio-tests.el', 'eieio-test-01-mix-alloc-initarg'): (
                    'terminal',
                    '(skip-unless (and (message "foo") (string= "foo" (current-message))))',
                    '(should (string-match "Class allocated slots do not need :initarg"\n'
                    '                        (current-message)))',
                    'eieio-test-20-class-allocated-slots',
                    ('(ert-deftest eieio-test-20-class-allocated-slots ()',),
                    '(ert-test-failed ((should (string-match "Class allocated slots do not need :initarg" '
                    '(current-message))) :form (string-match "Class allocated slots do not need :initarg" ',
                ),
            }
            spec = specs.get((case['file'], case['id']))
            require(spec is not None and statuses == ['skipped', 'skipped']
                    and set(invalid) == {'kind', 'applicable_display', 'source_form',
                                         'broken_assertion_form', 'control_test_id',
                                         'conditions', 'controls'}
                    and invalid['applicable_display'] == spec[0]
                    and invalid['source_form'] == spec[1]
                    and invalid['broken_assertion_form'] == spec[2]
                    and invalid['control_test_id'] == spec[3]
                    and invalid['source_form'] in source
                    and invalid['broken_assertion_form'] in source
                    and all(marker in source for marker in spec[4])
                    and invalid['conditions'] == {s: events[s].get('condition')
                                                  for s in ('reference', 'candidate')},
                    'display-gated broken test lacks its exact original source and batch skip proof')
            display = invalid['applicable_display']
            for label in ('reference', 'candidate'):
                metadata = file_row[label].get('metadata', {})
                require(metadata.get('display_mode') == 'batch' and metadata.get('noninteractive') is True
                        and isinstance(events[label].get('condition'), str)
                        and events[label]['condition'].startswith('(ert-test-skipped '),
                        'display-gated broken test was not skipped in the original batch context')
                proofs.update(_formal_counterpart(root, api, case, label, file_row,
                    invalid['control_test_id'], local_cache))
            expected_labels = (['reference']
                               if display == 'cocoa' and profile.get('kind') == 'terminal'
                               else ['reference', 'candidate'])
            require(set(invalid['controls']) == set(expected_labels),
                    'display-gated broken test lacks its exact applicable display controls')
            for label in expected_labels:
                ctrl = invalid['controls'][label]
                require(isinstance(ctrl, dict) and ctrl.get('test_id') == invalid['control_test_id'],
                        'display-gated broken test control must target its passing sibling')
                proofs.update(_passing_control(ctrl, case, label, file_row, receipt, api,
                    reference_selection, profile, display=display, cache=local_cache))
                _, obs, ev = _control_case_event(ctrl, case['file'], case['id'], local_cache)
                cond = ev.get('condition') or ''
                require(obs.get('status') == 'failed' and ev.get('status') == 'failed'
                        and cond.startswith(spec[5]) and cond.endswith(' :value nil))'),
                        'display-gated broken test did not fail at its exact upstream assertion on both applicable sides')
        elif isinstance(invalid, dict) and invalid.get('kind') == 'removed-upstream-producer':
            # The expectation itself outlived its implementation upstream.  This
            # is not a batch skip: the interactive terminal passes the skip gate
            # and then fails at a string match whose producer was deleted before
            # the pinned commit.  Bind the exact obsolete assertion, the absent
            # producer form, the removing commit, the interactive display context
            # and the actual condition on both sides -- no fabricated pass.
            source = '\n'.join(excerpts)
            producer_path = invalid['producer_path']
            producer_identity = bound_file(producer_path, limit=32*1024*1024)
            producer = Path(producer_path).read_text(encoding='utf-8', errors='replace')
            cond_prefix = ('(ert-test-failed ((should (string-match '
                           '"Class allocated slots do not need :initarg" (current-message))) ')
            require(set(invalid) == {'kind', 'applicable_display', 'skip_form', 'source_form',
                                     'removed_producer_form', 'producer_path',
                                     'upstream_commit', 'control_test_id', 'conditions'}
                    and case['file'] == 'test/lisp/emacs-lisp/eieio-tests/eieio-tests.el'
                    and case['id'] == 'eieio-test-01-mix-alloc-initarg'
                    and invalid['applicable_display'] in {'terminal', 'cocoa'}
                    and invalid['source_form'] in source
                    and invalid['skip_form'] in source
                    and invalid['removed_producer_form'] not in producer
                    and isinstance(invalid['upstream_commit'], str)
                    and re.fullmatch(r'[0-9a-f]{40}', invalid['upstream_commit']) is not None
                    and invalid['conditions'] == {label: events[label].get('condition')
                                                  for label in ('reference', 'candidate')},
                    'removed upstream producer requires its exact source forms, the absent producer '
                    'form, the pinned commit and the actual condition on both sides')
            metadata = {label: file_row[label].get('metadata', {}) for label in ('reference', 'candidate')}
            if statuses == ['skipped', 'skipped']:
                # Batch execution never reaches the obsolete assertion: the
                # original skip gate declines first.  Both skipped states keep
                # their exact original condition and the same-file sibling must
                # still pass.
                require(all(metadata[label].get('display_mode') == 'batch'
                            and metadata[label].get('noninteractive') is True
                            for label in metadata)
                        and all(isinstance(events[label].get('condition'), str)
                                and events[label]['condition'].startswith('(ert-test-skipped (')
                                and invalid['skip_form'] in events[label]['condition']
                                for label in ('reference', 'candidate')),
                        'batch manifestation must be the original unconditional skip on both sides')
            else:
                require(statuses == ['failed', 'failed']
                        and all(metadata[label].get('display_mode') == invalid['applicable_display']
                                and metadata[label].get('noninteractive') is False
                                for label in metadata)
                        and all(isinstance(events[label].get('condition'), str)
                                and events[label]['condition'].startswith(cond_prefix)
                                and events[label]['condition'].endswith(' :value nil))')
                                and events[label]['condition'] == events['reference']['condition']
                                for label in ('reference', 'candidate')),
                        'interactive manifestation must fail identically at the obsolete assertion '
                        'in the declared display context')
            proofs[str(Path(producer_path).resolve())] = producer_identity
            local_cache = {} if cache is None else cache
            for label in ('reference', 'candidate'):
                proofs.update(_formal_counterpart(root, api, case, label, file_row,
                    invalid['control_test_id'], local_cache))
        elif invalid is not None:
            require(statuses == ['skipped', 'skipped'], 'invalid skip rule cannot excuse another outcome')
            invalid = rule.get('invalid_gate', {})
            form = invalid.get('source_form')
            require(invalid.get('kind') in {'unconditional-skip', 'unreachable-gate',
                                          'manual-nonterminating-diagnostic'}
                    and isinstance(form, str) and bool(form)
                    and any(form in text for text in excerpts), 'invalid skip lacks original source proof')
            if invalid['kind'] == 'unconditional-skip':
                require('(skip-unless nil)' in form or form.startswith('(ert-skip '),
                        'invalid skip is not unconditionally disabled upstream')
            elif invalid['kind'] == 'manual-nonterminating-diagnostic':
                source = '\n'.join(excerpts)
                closed = {}
                for label in ('reference', 'candidate'):
                    pin = file_row[label].get('file_result', {})
                    path = Path(root)/api._relative_path(pin.get('path'), 'manual diagnostic file evidence')
                    identity = {k:pin.get(k) for k in ('bytes', 'sha256')}
                    closed[label] = _control_json(dict(path=str(path), identity=identity),
                        {} if cache is None else cache)
                    proofs[str(path)] = identity
                selector = '(eq (ert--stats-selector ert--current-run-stats)\n       (ert-test-name (ert--stats-current-test ert--current-run-stats)))'
                prefix = ('(ert-test-skipped ((skip-unless '
                    '(eq (ert--stats-selector ert--current-run-stats) '
                    '(ert-test-name (ert--stats-current-test ert--current-run-stats)))) '
                    ':form (eq (member ')
                require(case['file'] == 'test/lisp/net/tramp-tests.el'
                        and case['id'] == 'tramp-test45-force-remote-file-error'
                        and form == '(skip-unless\n   '+selector+')'
                        and all(marker in source for marker in (
                            'It shall run only if selected explicitly.',
                            '"while true; do echo test; sleep 0.2; done"',
                            '(run-at-time\n     0 0.2',
                            '(while t\n      (accept-process-output)'))
                        and invalid.get('conditions') == {s: events[s].get('condition') for s in events}
                        and all(closed[s].get('file') == case['file']
                            and closed[s].get('selector') == 'all-discovered-ids'
                            and case['id'] in closed[s].get('ids', [])
                            and events[s]['condition'].startswith(prefix)
                            and events[s]['condition'].endswith(' :value nil))')
                            for s in events),
                        'only the original nonterminating manual diagnostic may retain this selector skip')
            else:
                require(len(rule['source_evidence']) >= 2
                        and invalid.get('conditions') == {s: events[s].get('condition') for s in events}
                        and all(isinstance(events[s].get('condition'), str)
                                and events[s]['condition'].startswith('(ert-test-skipped ')
                                and ' :value ' not in events[s]['condition'] for s in events),
                        'unreachable gate requires helper source and exact original errors')
        else:
            require(statuses == ['failed', 'failed'], 'invalid-test rule requires matching actual outcomes')
    elif category == 'explicit-optional-capability':
        gate = rule.get('optional_gate')
        if isinstance(gate, dict) and gate.get('kind') == 'native-positive-test':
            proofs.update(_native_positive_applicability(gate, case, events, file_row, excerpts, profile))
        else:
            require('skipped' in statuses and set(statuses) <= {'passed', 'skipped'},
                    'optional capability cannot excuse a failed test')
            proofs.update(_optional_applicability(gate, case, events, file_row,
                excerpts, receipt, api, reference_selection, profile, cache, root=root,
                source_anchors=rule.get('source_evidence', ())))
    elif category == 'known-reference-library-defect':
        gate = rule.get('reference_defect_gate', {})
        require(any(gate.get('source_form', '') in text for text in excerpts)
                and any('(run-hooks create-hook)' in text for text in excerpts)
                and any("(add-hook 'after-make-frame-functions 'select-frame)" in text for text in excerpts),
                'reference defect lacks its original callback and focus source proof')
        lifecycle = reference_defect_state(rule, case, file_row, receipt, events, root, api)
        require(lifecycle['state'] == 'active-reference-only-defect',
                'reference defect cannot excuse incorrect candidate behavior or stale upstream evidence: '
                + lifecycle['state'])
        proofs.update(lifecycle['proof_inputs'])
        proofs.update(_correct_library_control(rule.get('correct_behavior_control'), receipt,
                                               'candidate', profile, reference_selection, file_row))
    elif category == 'approved-host-environment-limitation':
        gate = rule.get('environment_gate')
        source = '\n'.join(excerpts)
        dns_pattern = (
            r'^\(ert-test-failed \(\(should \(eq nil \(network-lookup-address-info '
            r'"emacs\.invalid"\)\)\) :form \(eq nil \(\[[0-9 ]+\](?: \[[0-9 ]+\])*\)\) '
            r':value nil\)\)$'
        )
        require(isinstance(gate, dict)
                and set(gate) == {'kind', 'source_form', 'contrast_evidence', 'conditions'}
                and gate['kind'] == 'reserved-invalid-dns-interception'
                and case['file'] == 'test/src/process-tests.el'
                and case['id'] == 'non-existent-lookup-failure'
                and statuses == ['failed', 'failed']
                and gate['source_form'] == '(should (eq nil (network-lookup-address-info "emacs.invalid")))'
                and gate['contrast_evidence'] == 'tools/fixtures/reserved-invalid-dns-contrast.json'
                and all(marker in source for marker in (
                    '(ert-deftest lookup-unicode-domains ()',
                    '(ert-deftest unibyte-domain-name ()',
                    '(ert-deftest lookup-google ()',
                    '(ert-deftest non-existent-lookup-failure ()',
                    gate['source_form']))
                and gate['conditions'] == {label: events[label].get('condition')
                                           for label in ('reference', 'candidate')}
                and all(re.fullmatch(dns_pattern, events[label].get('condition', '') or '') is not None
                        for label in ('reference', 'candidate')),
                'approved host environment limitation covers only matching synthetic .invalid DNS interception')
        local_cache = {} if cache is None else cache
        contrast_path = Path(__file__).resolve().parent.parent/gate['contrast_evidence']
        contrast_identity = bound_file(contrast_path, limit=1024*1024)
        contrast = _control_json(dict(path=str(contrast_path), identity=contrast_identity), local_cache)
        observations = contrast.get('observations', [])
        by_pair = {(item.get('provider'), 'emacs.invalid' if 'emacs.invalid' in item.get('url', '')
                    else 'example.com' if 'example.com' in item.get('url', '') else None): item
                   for item in observations if isinstance(item, dict)}
        require(contrast.get('schema') == 'femacs-reserved-invalid-dns-contrast-v1'
                and contrast.get('rfc') == 'RFC 6761 Section 6.4'
                and contrast.get('queried_reserved_domain') == 'emacs.invalid'
                and contrast.get('queried_control_domain') == 'example.com'
                and contrast.get('local_proxy_preserved') is True
                and contrast.get('configuration_changed') is False
                and len(observations) == 4
                and set(by_pair) == {('google', 'example.com'), ('google', 'emacs.invalid'),
                                     ('cloudflare', 'example.com'), ('cloudflare', 'emacs.invalid')}
                and all(by_pair[(provider, 'example.com')].get('exit_code') == 0
                        and by_pair[(provider, 'example.com')].get('dns_status') == 0
                        and type(by_pair[(provider, 'example.com')].get('answer_count')) is int
                        and by_pair[(provider, 'example.com')]['answer_count'] >= 1
                        and by_pair[(provider, 'emacs.invalid')].get('exit_code') == 0
                        and by_pair[(provider, 'emacs.invalid')].get('dns_status') == 3
                        and by_pair[(provider, 'emacs.invalid')].get('answer_count') == 0
                        for provider in ('google', 'cloudflare')),
                'reserved .invalid HTTPS DNS contrast evidence is missing or invalid')
        proofs[str(contrast_path)] = contrast_identity
        for label in ('reference', 'candidate'):
            for counterpart in ('lookup-unicode-domains', 'unibyte-domain-name', 'lookup-google'):
                proofs.update(_formal_counterpart(root, api, case, label, file_row, counterpart, local_cache))
    else:
        require(category == 'proven-version-evolution', 'unknown classification category')
        if rule.get('version_gate') is not None:
            proofs.update(_component_version_applicability(rule['version_gate'], case, events,
                file_row, excerpts, receipt, reference_selection, rule['source_evidence'],
                root=root, api=api, profile=profile, cache=cache))
        else:
            require(statuses == ['failed', 'failed'] and len(rule.get('source_evidence', [])) >= 2,
                    'version evolution requires matching failures and both source versions')
            proofs.update(_passing_control(rule.get('version_control'), case, 'candidate', file_row,
                receipt, api, reference_selection, profile, purpose='current-version', cache=cache))
    return proofs


def assess(api, root, receipt, manifest, catalog_path, profile, reference_selection):
    """Return separate accounting; raw completion and detailed files stay intact."""
    root = Path(root)
    catalog, catalog_identity = load_catalog(catalog_path)
    require(isinstance(profile, dict) and profile.get('kind') != 'reference'
            and profile.get('test_hooks') is False and isinstance(profile.get('id'), str),
            'one explicit non-diagnostic candidate required')
    require(isinstance(reference_selection, dict) and reference_selection.get('epoch')
            and isinstance(reference_selection.get('upstream'), dict), 'bound reference epoch required')
    require(catalog['manifest_sha256'] == receipt['manifest_sha256']
            and catalog['reference_selection_sha256'] == reference_selection.get('sha256'),
            'classification epoch or original corpus mismatch')
    context = context_identity(receipt, profile, reference_selection)
    selected = catalog['contexts'].get(profile['id'])
    require(isinstance(selected, dict) and {key: value for key, value in selected.items()
            if key != 'reference_defects'} == dict(context_sha256=context, run_sha256=receipt['detailed']['sha256'],
            completion_sha256=api._sha256_path(root/'stable-suite-completion.json')),
            'classification does not bind this exact complete run and candidate')
    entries = api._load_batch().select_entrypoints(manifest)
    by_path = {entry['path']: entry for entry in entries}
    defects = {}
    records = selected.get('reference_defects', [])
    require(isinstance(records, list) and len(records) <= 8, 'bounded reference defect contracts required')
    for record in records:
        require(isinstance(record, dict) and set(record) == {'definition', 'controls'}
                and isinstance(record['definition'], dict) and isinstance(record['controls'], dict)
                and not set(record['controls'])-{'reference', 'candidate'}, 'invalid reference defect contract')
        definition = record['definition']
        key = (definition.get('file'), definition.get('id'))
        require(definition.get('profile') == profile['id'] and key[0] in by_path
                and isinstance(key[1], str) and key not in defects
                and definition.get('category') == 'known-reference-library-defect'
                and definition.get('input_sha256') == by_path[key[0]]['sha256'],
                'duplicate or unbound reference defect contract')
        defects[key] = record
    rules = {}
    for rule in catalog['rules']:
        require(isinstance(rule, dict) and isinstance(rule.get('profile'), str), 'invalid classification rule')
        if rule['profile'] != profile['id']:
            continue
        key = (rule.get('file'), rule.get('id'))
        require(key[0] in by_path and isinstance(key[1], str) and bool(key[1]) and key not in rules,
                'duplicate or out-of-scope classified case')
        rules[key] = rule
    detailed_path = root/'stable-suite-run.json'
    bound_file(detailed_path, dict(bytes=receipt['detailed']['bytes'], sha256=receipt['detailed']['sha256']))
    metadata, files, failures = {}, {}, set()
    for name, value in detailed_rows(detailed_path):
        if name == 'files':
            require(isinstance(value, dict) and value.get('file') in by_path
                    and value['file'] not in files, 'duplicate or unexpected original file')
            require(list(by_path)[len(files)] == value['file'], 'original file order changed')
            files[value['file']] = {key: value.get(key) for key in
                ('registration', 'reference_id_count', 'candidate_id_count')}
            for label in ('reference', 'candidate'):
                files[value['file']][label] = {key: value.get(label, {}).get(key) for key in
                    ('complete', 'exit_code', 'timed_out', 'remaining_group_terminated',
                     'metadata', 'file_result', 'progress', 'argv', 'runtime_environment',
                     'mock_transport_context', 'diagnostic_trace')}
        elif name == 'cases':
            # Cases can precede file metadata in the frozen writer. Read them
            # in a second streaming pass, never keep all diagnostic payloads.
            pass
        elif name == 'failures':
            require(isinstance(value, str) and value not in failures, 'invalid/duplicate raw failure')
            failures.add(value)
        elif not name.endswith('-end'):
            metadata[name] = value
    structural, raw_file_evidence = [], {}
    for key in ('execution_complete', 'identities_unchanged', 'corpus_inputs_unchanged'):
        if receipt.get(key) is not True:
            structural.append(key)
    for key in ('source_context_unchanged', 'identities_unchanged', 'corpus_inputs_unchanged'):
        if metadata.get(key) is not True:
            structural.append('detailed-' + key)
    for key in ('schema', 'selector', 'manifest_sha256', 'source', 'products_before',
                'products_after', 'runtime_environment', 'harness_sources', 'corpus_complete'):
        expected = receipt['result_schema'] if key == 'schema' else receipt.get(key)
        require(metadata.get(key) == expected, 'detailed metadata differs from completion: ' + key)
    require(list(files) == list(by_path), 'full original file denominator not executed')
    require(metadata.get('entrypoint_count') == len(entries)
            and metadata.get('resource_only_lisp_inputs') == receipt['resource_only_lisp_inputs'],
            'detailed original file denominator changed')
    counts_by_file = {name: 0 for name in files}
    for name, row in files.items():
        if row.get('registration', {}).get('passed') is not True:
            structural.append('registration:' + name)
        for label in ('reference', 'candidate'):
            actual = row.get(label, {})
            if not (actual.get('complete') is True and actual.get('exit_code') == 0
                    and actual.get('timed_out') is False and actual.get('remaining_group_terminated') is False):
                structural.append(label + '-incomplete:' + name)
            for field in ('file_result', 'progress'):
                pin = actual.get(field, {})
                relative = api._relative_path(pin.get('path'), 'per-file evidence')
                bound_file(root/relative, dict(bytes=pin.get('bytes'), sha256=pin.get('sha256')))
                raw_file_evidence[str(root/relative)] = dict(bytes=pin.get('bytes'), sha256=pin.get('sha256'))
    ordinary, classified, blocked, used, cases, proof_inputs = 0, [], [], set(), set(), raw_file_evidence
    proof_cache, defect_states, seen_defects = {}, [], set()
    for field, case in detailed_rows(detailed_path):
        if field != 'cases':
            continue
        require(isinstance(case, dict) and isinstance(case.get('file'), str)
                and isinstance(case.get('id'), str), 'invalid original case identity')
        name, identity = case['file'], case['id']
        require(name in by_path and (name, identity) not in cases and len(cases) < MAX_CASES,
                'duplicate, unknown or oversized case scope')
        cases.add((name, identity))
        counts_by_file[name] += 1
        raw_passed = True
        for label in ('reference', 'candidate'):
            observed = case.get(label, {})
            assessment = api.assess_case(identity, observed)
            require(observed.get('assessment') == assessment, 'raw case assessment changed')
            require(type(observed.get('complete')) is bool and isinstance(observed.get('diagnostic'), str),
                    'raw case shape changed')
            if not (observed['complete'] and observed.get('exit_code') == 0
                    and observed.get('status') in {'passed', 'failed', 'skipped'}
                    and type(observed.get('seconds')) in (int, float)
                    and math.isfinite(observed['seconds']) and observed['seconds'] >= 0):
                structural.append(label + '-case-incomplete:' + name + '::' + identity)
            raw_passed &= assessment['passed']
        require(case.get('passed') is raw_passed, 'raw pair passing flag changed')
        key = (name, identity)
        if key in defects:
            record = defects[key]
            definition = record['definition']
            _source_evidence(definition, by_path[name])
            for pin in definition['source_evidence']:
                proof_inputs[pin['path']] = dict(bytes=pin['bytes'], sha256=pin['sha256'])
            state = reference_defect_state(definition, case, files[name], receipt,
                {label: _original_case_event(root, api, case, label)
                 for label in ('reference', 'candidate')}, root, api)
            proof_inputs.update(state.pop('proof_inputs'))
            labels = reference_contract_labels(state['state'])
            verified = []
            for label in labels:
                control = record['controls'].get(label)
                if control is None:
                    structural.append('correct-behavior-control-required:'+name+'::'+identity+':'+label)
                    continue
                proof_inputs.update(_correct_library_control(control, receipt, label, profile,
                                                             reference_selection, files[name]))
                verified.append(label)
            if state['state'] not in {'active-reference-only-defect', 'resolved-upstream-fixed',
                                      'candidate-regression-after-upstream-fix'}:
                structural.append('reference-defect-revalidation:'+name+'::'+identity)
            state['candidate_correct_behavior_verified'] = 'candidate' in verified
            state['rule_retired'] = state['rule_retired'] and set(labels) <= set(verified)
            defect_states.append(dict(file=name, id=identity, **state))
            seen_defects.add(key)
        if raw_passed:
            ordinary += 1
            continue
        failure = 'case failed/unverified: ' + name + ' :: ' + identity
        require(failure in failures, 'nonpassing case missing its raw failure')
        failures.remove(failure)
        if key in rules:
            proof_inputs.update(_accept_rule(rules[key], case, files[name], by_path[name],
                root, api, reference_selection, profile, receipt, proof_cache, manifest=manifest))
            for pin in rules[key]['source_evidence']:
                proof_inputs[pin['path']] = dict(bytes=pin['bytes'], sha256=pin['sha256'])
            used.add(key)
            classified.append(dict(file=name, id=identity, category=rules[key]['category'],
                                   observations=rules[key]['observations']))
        else:
            blocked.append(dict(file=name, id=identity,
                reason=('potential-port-regression' if case['reference']['status'] == 'passed'
                        else 'unexplained-failure-or-skip'),
                reference_status=case['reference']['status'], candidate_status=case['candidate']['status']))
    require(used == set(rules), 'classification contains unused or ordinarily passing cases')
    require(seen_defects == set(defects), 'reference defect contract lacks its original registered case')
    registration = metadata.get('registration', {})
    if not (registration.get('passed') is True
            and registration.get('reference_count') == registration.get('candidate_count') == len(cases)):
        structural.append('full-registration')
    for name, count in counts_by_file.items():
        row = files[name]
        if row.get('reference_id_count') != count or row.get('candidate_id_count') != count:
            structural.append('file-registration-count:' + name)
    structural.extend(sorted(failures))
    require(receipt['case_count'] == len(cases) == ordinary + len(classified) + len(blocked)
            and receipt['passed_count'] == ordinary, 'classified/raw case counts do not conserve')
    require(receipt['failure_count'] == len(classified) + len(blocked) + len(failures),
            'classified/raw failure counts do not conserve')
    bound_file(detailed_path, dict(bytes=receipt['detailed']['bytes'], sha256=receipt['detailed']['sha256']))
    bound_file(catalog_path, catalog_identity, MAX_CATALOG)
    for path, identity in proof_inputs.items():
        # Owner journals can contain the whole product file list. Hash these
        # streams without materializing them; JSON items remain <= MAX_ITEM.
        bound_file(path, identity, 512*1024*1024)
    require(bound_file(root/'stable-suite-completion.json')['sha256'] == selected['completion_sha256'],
            'original completion changed during classification')
    return dict(schema=RESULT_SCHEMA, catalog_identity=catalog_identity,
        context_sha256=context, profile=profile['id'], reference_epoch=reference_selection['epoch'],
        raw_completion_sha256=selected['completion_sha256'], raw_detailed_sha256=selected['run_sha256'],
        original_file_count=len(files), raw_case_count=len(cases), raw_passed_count=ordinary,
        raw_failure_count=receipt['failure_count'], classified_case_count=len(classified),
        blocking_case_failure_count=len(blocked), structural_failure_count=len(structural),
        classified_cases=classified, blocking_cases=blocked, structural_failures=structural,
        source_and_control_evidence=proof_inputs,
        reference_defect_lifecycle=defect_states,
        classified_corpus_accepted=bool(cases) and not structural and not blocked,
        raw_results_modified=False, classified_cases_counted_as_passed=False,
        retained_failure_state_removed=False, compatibility_accepted=False, goal_complete=False)


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--build', type=Path)
    parser.add_argument('--reference-selection', type=Path, required=True)
    catalog_input = parser.add_mutually_exclusive_group(required=True)
    catalog_input.add_argument('--classification', type=Path)
    catalog_input.add_argument('--approved-rules', type=Path,
        help='assemble exact run/event bindings from individually reviewed source/control rule definitions')
    parser.add_argument('--scoped-assembly', action='store_true',
        help='bind reviewed rules to one real product channel instead of the 48-profile matrix')
    parser.add_argument('--receipt', type=Path,
        help='closed stable-suite-run.json for --scoped-assembly')
    parser.add_argument('--profile', help='declared profile name for --scoped-assembly')
    parser.add_argument('--run-results', type=Path, action='append', default=[],
                        help='closed original runtime-stage results; repeat for distinct stable scopes')
    parser.add_argument('--control-run', type=Path, nargs=2, action='append', default=[],
                        metavar=('RESULTS', 'CONTROLLER'),
                        help='closed stable-controls results and its outer controller stderr; repeat as needed')
    parser.add_argument('--library-contract-run', type=Path, action='append', default=[],
                        help='fresh closed real product library-contract collections; repeat for profile batches')
    parser.add_argument('--remote-contract-run', type=Path, nargs=2, action='append', default=[],
                        metavar=('RESULTS','CONTROLLER'),
                        help='closed same-product signal/watch controls and their outer controller')
    parser.add_argument('--out', type=Path, required=True)
    args = parser.parse_args(argv)
    if not args.scoped_assembly and (args.build is None or not args.run_results):
        parser.error('--build and at least one --run-results are required for the matrix assembly')
    if (args.control_run or args.library_contract_run or args.remote_contract_run) and args.approved_rules is None:
        parser.error('--control-run is only used when assembling source definitions')
    if args.scoped_assembly:
        if not args.receipt or not args.profile or args.approved_rules is None:
            parser.error('--scoped-assembly requires --receipt, --profile and --approved-rules')
        return assemble_scoped(args)
    tool_root = Path(__file__).resolve().parent
    spec = importlib.util.spec_from_file_location('classification_stage', tool_root/'runtime-stage.py')
    stage = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(stage)
    return assemble(stage, args) if args.approved_rules is not None else run(stage, args)



def _skipped_condition_fields(condition):
    """Extract printed ERT plist values without evaluating Lisp or guessing truth.

    Only top-level :form/:value fields of the skipped-condition payload count;
    quoted strings and nested predicates cannot supply counterfeit markers.
    Unreadable or unsupported printed structures remain a blocking condition.
    """
    require(isinstance(condition, str) and len(condition.encode('utf-8')) <= 65536
            and condition.startswith('(ert-test-skipped ('), 'exact skipped condition required')
    depth, quoted, escaped, markers, end = 0, False, False, [], None
    index = 0
    while index < len(condition):
        char = condition[index]
        if escaped:
            escaped = False
        elif char == '\\':
            escaped = True
        elif char == '"':
            quoted = not quoted
        elif not quoted:
            if char in '([':
                depth += 1
            elif char in ')]':
                if depth == 2 and char == ')':
                    require(end is None, 'ambiguous skipped payload boundary')
                    end = index
                depth -= 1
                require(depth >= 0, 'unbalanced skipped condition')
            elif char == ':' and depth == 2 and index and condition[index-1].isspace():
                stop = index+1
                while stop < len(condition) and not condition[stop].isspace() and condition[stop] not in '()[]':
                    stop += 1
                markers.append((condition[index:stop], index, stop))
                index = stop-1
        index += 1
    require(not quoted and not escaped and depth == 0 and end is not None,
            'unbalanced skipped condition')
    values = {}
    for offset, (key, start, stop) in enumerate(markers):
        require(key not in values, 'duplicate skipped condition field')
        boundary = markers[offset+1][1] if offset+1 < len(markers) else end
        value = condition[stop:boundary].strip()
        require(bool(value) and len(value.encode('utf-8')) <= 8192, 'unbounded skipped condition field')
        values[key] = value
    require(':form' in values and ':value' in values, 'skipped gate lacks an actual form/value')
    return values[':form'], values[':value']


def _capture_optional_gate(definition, root, api, case):
    """Bind a reviewed source gate only to this run's exact original events."""
    require(isinstance(definition, dict)
            and not {'missing_on', 'condition_forms', 'condition_values', 'conditions', 'observations'} & set(definition),
            'source gate definitions cannot supply runtime observations')
    if definition.get('kind') == 'native-positive-test':
        require([case[s].get('status') for s in ('reference', 'candidate')] == ['passed', 'failed'],
                'native positive-test capture requires actual reference pass and candidate failure')
        event = _original_case_event(root, api, case, 'candidate')
        return dict(definition, missing_on=['candidate'], conditions={'candidate': event.get('condition')})
    require(all(case[label].get('status') in {'passed', 'skipped'}
                for label in ('reference', 'candidate')),
            'optional source gate cannot capture a failure')
    missing = [label for label in ('reference', 'candidate') if case[label]['status'] == 'skipped']
    require(bool(missing), 'optional source gate has no actual skipped side')
    if definition.get('kind') in {'file-monitor-backend', 'file-monitor-identity', 'external-message', 'direct-display-context',
                                 'external-program-missing'}:
        return dict(definition, missing_on=missing, conditions={label:
            _original_case_event(root, api, case, label).get('condition') for label in missing})
    forms, values = {}, {}
    for label in missing:
        event = _original_case_event(root, api, case, label)
        forms[label], values[label] = _skipped_condition_fields(event.get('condition'))
    return dict(definition, missing_on=missing, condition_forms=forms, condition_values=values)


def _closed_controls(arguments):
    """Index existing control envelopes; acceptance remains in _passing_control."""
    require(len(arguments) <= 64, 'bounded control run scope required')
    runs, inputs = [], {}
    for results, controller in arguments:
        results, controller = Path(os.path.abspath(results)), Path(os.path.abspath(controller))
        identity = bound_file(results, limit=MAX_ITEM)
        require(sum(item['closed_run']['identity']['bytes'] for item in runs)
                + identity['bytes'] <= MAX_CATALOG,
                'combined control envelopes exceed the catalog memory bound')
        value = json.loads(read_bytes(results, identity, MAX_ITEM), object_pairs_hook=_pairs)
        require(value.get('schema') == 'femacs-current-version-controls-v1'
                and value.get('diagnostic_collection_complete') is True
                and value.get('passed') is True and isinstance(value.get('cases'), list),
                'only completed control envelopes can supply applicable observations')
        owner = results.parent/'artifact-preservation.json'
        controller_pin, owner_pin = bound_file(controller, limit=65536), bound_file(owner)
        for path, pin in ((results, identity), (controller, controller_pin), (owner, owner_pin)):
            require(str(path) not in inputs, 'duplicate control envelope input')
            inputs[str(path)] = pin
        runs.append(dict(value=value,
            closed_run=dict(path=str(results), identity=identity),
            controller=dict(path=str(controller), identity=controller_pin),
            owner=dict(path=str(owner), identity=owner_pin)))
    return runs, inputs


def _select_control(runs, case, label, profile, *, purpose, display=None,
                    counterpart=None, test_id=None, test_environment=None, cache=None):
    """Select an exact registered passing control without test-name inference."""
    target_file, target_id = counterpart or case['file'], test_id or case['id']
    target_profile = profile if label == 'candidate' else 'installed-reference'
    matches = []
    for run in runs:
        root = Path(run['value'].get('corpus_root', ''))
        require(root.is_absolute(), 'absolute owned control corpus required')
        for row in run['value']['cases']:
            if row.get('profile') != target_profile or row.get('file') != target_file:
                continue
            relative = Path(row.get('file_result', {}).get('path', ''))
            require(str(relative) not in ('', '.') and not relative.is_absolute()
                    and '..' not in relative.parts, 'relative control file result required')
            file = dict(path=str(root/relative), identity={key:row['file_result'][key]
                for key in ('bytes', 'sha256')})
            value = _control_json(file, cache)
            if (value.get('resource_admission', {}).get('purpose') != purpose
                    or (display is not None and value.get('metadata', {}).get('display_mode') != display)
                    or (test_environment is not None and value.get('test_environment', {}) != test_environment)
                    or target_id not in value.get('cases', {})
                    or value['cases'][target_id].get('assessment', {}).get('passed') is not True):
                continue
            relative_progress = Path(value.get('progress', {}).get('path', ''))
            require(str(relative_progress) not in ('', '.') and not relative_progress.is_absolute()
                    and '..' not in relative_progress.parts, 'relative control event progress required')
            matches.append(dict(**file, test_input=row['test_input'], test_id=target_id,
                progress=dict(path=str(root/relative_progress), identity={key:value['progress'][key]
                    for key in ('bytes', 'sha256')}),
                **{key:run[key] for key in ('closed_run', 'controller', 'owner')}))
    require(len(matches) == 1, 'one exact passing control required; missing or ambiguous observations remain blocking')
    return matches[0]


def _write_catalog(path, catalog):
    """Keep the full 48-profile catalog inside the existing byte contract.

    Whitespace is not a second protocol. Avoid pretty-print expansion of the
    repeated source/control bindings, and enforce the reader's exact limit
    while encoding instead of publishing an unreadable oversized catalog.
    """
    path = Path(path)
    temporary = path.with_name(path.name+'.pending')
    total = 0
    encoder = json.JSONEncoder(ensure_ascii=False, allow_nan=False, separators=(',', ':'))
    with temporary.open('xb') as outgoing:
        for token in encoder.iterencode(catalog):
            raw = token.encode('utf-8')
            total += len(raw)
            require(total+1 <= MAX_CATALOG, 'full classification catalog exceeds its reader byte bound')
            outgoing.write(raw)
        outgoing.write(b'\n');outgoing.flush();os.fsync(outgoing.fileno())
    os.link(temporary, path)
    temporary.unlink()
    descriptor = os.open(path.parent, os.O_RDONLY)
    try:
        os.fsync(descriptor)
    finally:
        os.close(descriptor)


def assemble(stage, args):
    """Capture mechanical bindings; never infer a category from a tag or status."""
    execution = stage.execution
    definition_pin = bound_file(args.approved_rules, limit=MAX_CATALOG)
    definition = json.loads(read_bytes(args.approved_rules, definition_pin, MAX_CATALOG), object_pairs_hook=_pairs)
    require(isinstance(definition, dict) and set(definition) == {'schema', 'rules'}
            and definition['schema'] == 'femacs-stable-reviewed-rule-definitions-v1'
            and isinstance(definition['rules'], list) and len(definition['rules']) <= MAX_RULES,
            'bounded explicitly reviewed rule definitions required')
    rules = {}
    for rule in definition['rules']:
        require(isinstance(rule, dict) and isinstance(rule.get('profile'), str)
                and isinstance(rule.get('file'), str) and isinstance(rule.get('id'), str)
                and rule.get('category') in CATEGORIES
                and isinstance(rule.get('reason'), str) and bool(rule['reason'].strip())
                and 'observations' not in rule, 'source-reviewed definition required; observations are captured from raw events')
        anchors = rule.get('source_evidence')
        require(isinstance(anchors, list) and 1 <= len(anchors) <= 16,
                'reviewed definitions require bounded portable source anchors')
        expanded = []
        for anchor in anchors:
            require(isinstance(anchor, dict) and isinstance(anchor.get('path'), str),
                    'invalid portable source anchor')
            relative = Path(anchor['path'])
            require(not relative.is_absolute() and '..' not in relative.parts
                    and relative.as_posix() == anchor['path'],
                    'source definitions must use canonical repository-relative anchors')
            expanded.append(dict(anchor, path=str(stage.REPO/relative)))
        rule = dict(rule, source_evidence=expanded)
        key = (rule['profile'], rule['file'], rule['id'])
        require(key not in rules, 'duplicate reviewed classification definition')
        rules[key] = rule
    options = SimpleNamespace(build=args.build, reference_selection=args.reference_selection,
                              execution_products=None)
    original, source, products_path, products, build = stage.read_action_build(options)
    selection = options._reference_selection_summary
    manifest_path = source/'compatibility/stable-31.1/sources.json'
    suite = execution.load_tool('stable-suite.py')
    candidates = {p['id']: p for p in products if p['kind'] != 'reference' and not p['test_hooks']}
    require(len(candidates) == 48, 'complete candidate registry required')
    require(all(key[0] in candidates for key in rules), 'reviewed definition has an undeclared profile')
    catalog = dict(schema=SCHEMA, manifest_sha256=execution.digest(manifest_path),
        reference_selection_sha256=selection['sha256'], contexts={}, rules=[])
    controls, control_inputs = _closed_controls(getattr(args, 'control_run', []))
    remote_controls, remote_inputs = _remote_collections(getattr(args, 'remote_contract_run', []))
    inputs, matched, discovered, ordinary_unused = {**control_inputs, **remote_inputs}, set(), set(), set()
    defect_lifecycle = []
    library_contracts = []
    for path in getattr(args, 'library_contract_run', []):
        path = str(Path(os.path.abspath(path)))
        identity = bound_file(path, limit=MAX_ITEM)
        library_contracts.append(dict(path=path, identity=identity))
        inputs[path] = identity
    control_cache = {}
    for path in args.run_results:
        path = execution.safe_path(path)
        inputs[str(path)] = bound_file(path, limit=MAX_ITEM)
        raw = execution.bounded_json(path, MAX_ITEM)
        require(raw.get('schema') == 'femacs-runtime-stage-v1' and raw.get('action') == 'execute'
                and raw.get('execution_phase') in ('all', 'stable')
                and raw.get('execution_complete') is True
                and raw.get('product_identity_unchanged') is True
                and raw.get('complete_receipts_valid') is True
                and raw.get('build_receipt') == str(original)
                and raw.get('reference_selection') == selection, 'only closed matching-epoch raw stages may supply rule observations')
        libraries = raw.get('stable_product_libraries', False)
        require(type(libraries) is bool and (not libraries
                or raw.get('stable_harness_selection', {}).get('unchanged') is True),
                'unverified stable harness cannot supply classification observations')
        for row in raw.get('cases', []):
            profile = row.get('id', '')
            if not profile.startswith('stable-'):
                continue
            profile = profile[len('stable-'):]
            require(profile in candidates and profile not in catalog['contexts'],
                    'duplicate or unknown executed profile')
            root = execution.safe_path(row['receipt']).parent
            reference, candidate = products[0], candidates[profile]
            receipt = suite.read_completion(root, manifest_path, reference['program'], candidate['program'],
                ['--dump-file='+reference['dump']], ['--dump-file='+candidate['dump']],
                stage.stable_product_environment(reference, libraries),
                stage.stable_product_environment(candidate, libraries))
            process = row.get('process', {})
            require(row.get('execution_complete') is True and receipt['execution_complete'] is True
                    and process.get('complete') is True
                    and process.get('remaining_group_terminated') is False
                    and process.get('exit_code') == (0 if receipt['corpus_complete'] else 1)
                    and row.get('receipt_sha256') == receipt['detailed']['sha256']
                    and row.get('completion_sha256') == execution.digest(root/'stable-suite-completion.json'),
                    'rule capture requires a closed exact original process/receipt')
            catalog['contexts'][profile] = dict(context_sha256=context_identity(receipt, candidate, selection),
                run_sha256=receipt['detailed']['sha256'], completion_sha256=row['completion_sha256'])
            # Keep only bounded file metadata for lifecycle checks; detailed cases
            # remain streamed and original outcomes are never rewritten.
            defect_files = {rule['file'] for key, rule in rules.items()
                            if key[0] == profile and rule['category'] == 'known-reference-library-defect'}
            lifecycle_files = {item['file']: item for kind, item in detailed_rows(root/'stable-suite-run.json')
                               if kind == 'files' and item.get('file') in defect_files} if defect_files else {}
            for kind, case in detailed_rows(root/'stable-suite-run.json'):
                if kind != 'cases':
                    continue
                key = (profile, case.get('file'), case.get('id'))
                if key not in rules:
                    continue
                require(key not in discovered and type(case.get('passed')) is bool,
                        'reviewed definition must match one actual registered observation')
                discovered.add(key)
                if rules[key]['category'] == 'known-reference-library-defect':
                    state = reference_defect_state(rules[key], case, lifecycle_files[case['file']],
                        receipt, {label: _original_case_event(root, suite, case, label)
                                  for label in ('reference', 'candidate')}, root, suite)
                    inputs.update(state.pop('proof_inputs'))
                    labels = reference_contract_labels(state['state'])
                    correct_controls = {}
                    for label in labels:
                        target = profile if label == 'candidate' else 'installed-reference'
                        matches = [pin for pin in library_contracts if any(
                            r.get('profile') == target for r in _control_json(pin, control_cache).get('cases', []))]
                        if len(matches) != 1:
                            state['state'] = 'correct-behavior-control-required'
                            break
                        inputs.update(_correct_library_control(matches[0], receipt, label, candidate, selection,
                                                               lifecycle_files[case['file']]))
                        correct_controls[label] = matches[0]
                    if state['state'] == 'correct-behavior-control-required':
                        state['rule_retired'] = False
                    state['candidate_correct_behavior_verified'] = 'candidate' in correct_controls
                    # The exception disappears after resolution, but its behavior
                    # checks must still reach the final accounting reader. Ordinary
                    # passing cases cannot bypass those independent contracts.
                    catalog['contexts'][profile].setdefault('reference_defects', []).append(
                        dict(definition=rules[key], controls=correct_controls))
                    defect_lifecycle.append(dict(profile=profile, file=case['file'], id=case['id'],
                        reference_epoch=selection['epoch'], **state))
                    if state['state'] != 'active-reference-only-defect':
                        if case['passed']:
                            ordinary_unused.add(key)
                        # Resolved rules disappear; failed candidates and changed
                        # reference failures stay unclassified and blocking.
                        continue
                if case['passed']:
                    ordinary_unused.add(key)
                    continue
                matched.add(key)
                captured = dict(rules[key], observations={label: observation_signature(case[label])
                    for label in ('reference', 'candidate')})
                if captured['category'] == 'known-reference-library-defect':
                    captured['correct_behavior_control'] = correct_controls['candidate']
                if (captured.get('category') == 'proven-version-evolution'
                        and isinstance(captured.get('version_gate'), dict)
                        and captured['version_gate'].get('kind') == 'filenotify-stopped-event-file'
                        and isinstance(captured.get('optional_gate'), dict)):
                    statuses = {case[s]['status'] for s in ('reference', 'candidate')}
                    if 'skipped' in statuses and 'failed' not in statuses:
                        captured.pop('version_gate')
                        captured['category'] = 'explicit-optional-capability'
                    else:
                        captured.pop('optional_gate')
                if captured.get('category') == 'explicit-optional-capability':
                    captured['optional_gate'] = _capture_optional_gate(
                        captured.get('optional_gate'), root, suite, case)
                    gate = captured['optional_gate']
                    if gate.get('kind') == 'file-monitor-identity':
                        gate['controls'] = _select_remote_controls(remote_controls, profile, control_cache)
                    if gate.get('kind') in {'entrypoint-variant', 'parameter-variant', 'display-context', 'display-helper',
                                          'direct-display-context'} and gate.get('control_source') != 'same-file':
                        gate_controls = {}
                        for label in gate['missing_on']:
                            display = gate.get('applicable_display') if gate['kind'] in {
                                'display-context', 'direct-display-context', 'display-helper'} else None
                            if display == 'cocoa' and label == 'candidate' and candidate.get('kind') == 'terminal':
                                continue
                            gate_controls[label] = _select_control(controls, case, label, profile,
                                purpose='applicability', display=display,
                                counterpart=('test/lisp/progmodes/cperl-mode-tests.el'
                                    if gate['kind'] == 'entrypoint-variant' else None),
                                test_id=gate.get('control_test_id'), cache=control_cache)
                        gate['controls'] = gate_controls
                    elif gate.get('kind') == 'display-svg-subcapability':
                        purpose = 'current-version' if gate.get('control_corpus') == 'current' else 'applicability'
                        gate_controls = {
                            'reference': _select_control(controls, case, 'reference', profile,
                                purpose=purpose, display='cocoa', test_id=case['id'], cache=control_cache),
                        }
                        if candidate.get('kind') != 'terminal':
                            gate_controls['candidate'] = _select_control(controls, case, 'candidate', profile,
                                purpose=purpose, display='cocoa', test_id=gate['control_test_id'],
                                cache=control_cache)
                        gate['controls'] = gate_controls
                if captured.get('category') == 'proven-version-evolution':
                    if captured.get('version_gate') is None:
                        captured['version_control'] = _select_control(controls, case, 'candidate', profile,
                            purpose='current-version', cache=control_cache)
                    elif captured['version_gate'].get('kind') == 'filenotify-stopped-event-file':
                        require('controls' not in captured['version_gate'],
                                'source version gates cannot supply remote controls')
                        captured['version_gate'] = dict(captured['version_gate'],
                            controls=_select_remote_controls(remote_controls, profile, control_cache))
                if captured.get('category') == 'approved-host-environment-limitation':
                    env_gate = captured.get('environment_gate')
                    require(isinstance(env_gate, dict) and 'conditions' not in env_gate,
                            'source environment gates cannot supply runtime conditions')
                    captured['environment_gate'] = dict(env_gate, conditions={
                        label: _original_case_event(root, suite, case, label).get('condition')
                        for label in ('reference', 'candidate')})
                invalid = captured.get('invalid_gate')
                if isinstance(invalid, dict) and invalid.get('kind') == 'asynchronous-signal-liveness':
                    require(not {'conditions','controls'} & set(invalid),
                            'source definitions cannot supply remote observations')
                    captured['invalid_gate'] = dict(invalid,
                        controls=_select_remote_controls(remote_controls, profile, control_cache),
                        conditions={label:_original_case_event(root, suite, case, label).get('condition')
                                    for label in ('reference','candidate') if case[label]['status'] == 'failed'})
                if isinstance(invalid, dict) and invalid.get('kind') == 'wall-clock-format-precision':
                    require('conditions' not in invalid, 'reviewed definitions cannot supply runtime conditions')
                    captured['invalid_gate'] = dict(invalid, conditions={
                        label: _original_case_event(root, suite, case, label).get('condition')
                        for label in ('reference', 'candidate') if case[label]['status'] == 'failed'})
                if isinstance(invalid, dict) and invalid.get('kind') == 'removed-upstream-producer':
                    require('conditions' not in invalid,
                            'reviewed definitions cannot supply runtime conditions')
                    captured['invalid_gate'] = dict(invalid, conditions={
                        label: _original_case_event(root, suite, case, label).get('condition')
                        for label in ('reference', 'candidate')})
                if isinstance(invalid, dict) and invalid.get('kind') in {
                        'unreachable-gate', 'manual-nonterminating-diagnostic'}:
                    require('conditions' not in invalid, 'reviewed definitions cannot supply runtime conditions')
                    captured['invalid_gate'] = dict(invalid, conditions={label:
                        _original_case_event(root, suite, case, label).get('condition')
                        for label in ('reference', 'candidate')})
                if isinstance(invalid, dict) and invalid.get('kind') == 'unpopulated-graphical-snapshot-wrapper':
                    require(not {'conditions', 'controls'} & set(invalid),
                            'reviewed definitions cannot supply runtime conditions or controls')
                    scenario_controls = ({} if candidate.get('kind') == 'terminal' else {
                        label: [_select_control(controls, case, label, profile,
                                    purpose='applicability', display='cocoa',
                                    counterpart=target_file, test_id=target_id,
                                    test_environment=ERC_GRAPHICAL_TEST_ENVIRONMENT,
                                    cache=control_cache)
                                for target_file, target_id in ERC_GRAPHICAL_SCENARIO_CONTROLS]
                        for label in ('reference', 'candidate')
                    })
                    captured['invalid_gate'] = dict(invalid,
                        conditions={label: _original_case_event(root, suite, case, label).get('condition')
                                    for label in ('reference', 'candidate')},
                        controls=scenario_controls)
                if isinstance(invalid, dict) and invalid.get('kind') == 'display-gated-upstream-broken-test':
                    require(not {'conditions', 'controls'} & set(invalid),
                            'reviewed definitions cannot supply runtime conditions or controls')
                    display = invalid.get('applicable_display')
                    labels = (['reference']
                              if display == 'cocoa' and candidate.get('kind') == 'terminal'
                              else ['reference', 'candidate'])
                    captured['invalid_gate'] = dict(invalid,
                        conditions={label: _original_case_event(root, suite, case, label).get('condition')
                                    for label in ('reference', 'candidate')},
                        controls={label: _select_control(controls, case, label, profile,
                                      purpose='applicability', display=display,
                                      test_id=invalid['control_test_id'], cache=control_cache)
                                  for label in labels})
                catalog['rules'].append(captured)
    requested = {key for key in rules if key[0] in catalog['contexts']}
    require(discovered == requested,
            'reviewed definition was not registered by the supplied complete original files')
    out = execution.new_output(args.out)
    path = out/'classification.json'
    _write_catalog(path, catalog)
    try:
        code = run(stage, SimpleNamespace(build=args.build, reference_selection=args.reference_selection,
            classification=path, run_results=args.run_results, out=out/'accounting'))
        unchanged = bound_file(args.approved_rules, definition_pin) == definition_pin and all(
            bound_file(name, pin) == pin for name, pin in inputs.items())
        result = dict(schema='femacs-stable-catalog-assembly-v1', passed=code == 0 and unchanged,
            classification_catalog=dict(path=str(path), **bound_file(path)),
            reviewed_definition=dict(path=str(args.approved_rules), **definition_pin),
            raw_stage_inputs=inputs, bound_rule_count=len(catalog['rules']),
            bound_profile_count=len(catalog['contexts']), inputs_unchanged=unchanged,
            reviewed_rule_count=len(rules), ordinary_rule_not_applied_count=len(ordinary_unused),
            unexecuted_profile_rule_count=len(set(rules)-requested),
            accounting_result=str(out/'accounting/results.json'), automatic_categories_added=0,
            raw_results_modified=False, compatibility_accepted=False, goal_complete=False)
    except Exception as error:
        result = dict(schema='femacs-stable-catalog-assembly-v1', passed=False,
            error=f'{type(error).__name__}: {error}', raw_results_modified=False,
            compatibility_accepted=False, goal_complete=False)
    result['reference_defect_lifecycle'] = defect_lifecycle
    result['reference_defect_rules_retired'] = sum(row['rule_retired'] for row in defect_lifecycle)
    (out/'results.json').write_text(json.dumps(result, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    return 0 if result['passed'] else 1


def assemble_scoped(args):
    """Bind reviewed rules to one real product channel, without the 48-profile matrix.

    The rule validation is the same one `assemble()` applies: every definition
    must carry a category, a non-empty reason, canonical repository-relative
    source anchors and no pre-baked observations. Only rules whose file and test
    id actually appear in the closed receipt are bound, so a scoped catalog
    claims nothing about files it did not run. The receipt, its reference
    selection and its own product identity stay untouched.
    """
    receipt_path = Path(os.path.abspath(args.receipt))
    receipt = json.loads(receipt_path.read_text(encoding='utf-8'))
    definition_path = Path(os.path.abspath(args.approved_rules))
    definition = json.loads(definition_path.read_text(encoding='utf-8'))
    require(isinstance(definition, dict) and set(definition) == {'schema', 'profiles', 'rules'}
            and definition['schema'] == 'femacs-stable-source-rule-specs-v1'
            and isinstance(definition['rules'], list) and len(definition['rules']) <= MAX_RULES,
            'reviewed source rule spec required')
    selection = json.loads(Path(os.path.abspath(args.reference_selection)).read_text(encoding='utf-8'))
    require(isinstance(selection.get('epoch'), str) and isinstance(selection.get('upstream'), dict)
            and isinstance(selection.get('sha256'), str), 'bound reference epoch required')
    profile = dict(id=args.profile, kind='cocoa', test_hooks=False)
    observed = {(case['file'], case['id']) for case in receipt['cases']}
    repository = Path(__file__).resolve().parent.parent
    rules, skipped = [], []
    for rule in definition['rules']:
        require(isinstance(rule.get('file'), str) and isinstance(rule.get('id'), str)
                and rule.get('category') in CATEGORIES
                and isinstance(rule.get('reason'), str) and bool(rule['reason'].strip())
                and 'observations' not in rule, 'source-reviewed definition required')
        anchors = rule.get('source_evidence')
        require(isinstance(anchors, list) and 1 <= len(anchors) <= 16,
                'reviewed definitions require bounded portable source anchors')
        expanded = []
        for anchor in anchors:
            relative = Path(anchor['path'])
            require(not relative.is_absolute() and '..' not in relative.parts
                    and relative.as_posix() == anchor['path'],
                    'source definitions must use canonical repository-relative anchors')
            expanded.append(dict(anchor, path=str(repository/relative)))
        if (rule['file'], rule['id']) not in observed:
            skipped.append(dict(file=rule['file'], id=rule['id'], reason='not present in this channel receipt'))
            continue
        rules.append(dict(rule, profile=args.profile, source_evidence=expanded))
    catalog = dict(schema=SCHEMA,
                   manifest_sha256=receipt['manifest_sha256'],
                   reference_selection_sha256=selection['sha256'],
                   contexts={args.profile: dict(
                       context_sha256=context_identity(receipt, profile, selection),
                       run_sha256=hashlib.sha256(receipt_path.read_bytes()).hexdigest(),
                       completion_sha256=None)},
                   rules=rules)
    out = Path(os.path.abspath(args.out))
    require(not os.path.lexists(out), 'fresh scoped assembly output required')
    out.mkdir(parents=True)
    path = out/'classification.json'
    _write_catalog(path, catalog)
    result = dict(schema='femacs-stable-scoped-assembly-v1', passed=True,
                  profile=args.profile, receipt=str(receipt_path),
                  reference_selection=str(Path(os.path.abspath(args.reference_selection))),
                  classification_catalog=dict(path=str(path), **bound_file(path)),
                  bound_rule_count=len(rules), unapplied_rule_count=len(skipped),
                  unapplied=skipped[:64], matrix_complete=False,
                  scope=dict(entrypoints=len(receipt.get('files', [])),
                             cases=len(receipt.get('cases', [])),
                             declared_subset=receipt.get('subset')),
                  note='one real product channel; no 48-profile matrix and no rewrite of closed outcomes',
                  compatibility_accepted=False, goal_complete=False)
    (out/'results.json').write_text(json.dumps(result, ensure_ascii=False, indent=2) + '\n',
                                    encoding='utf-8')
    return 0


def run(stage, args):
    """Consume admitted product inputs and closed stages through the same CLI path."""
    execution = stage.execution
    catalog, catalog_identity = load_catalog(args.classification)
    options = SimpleNamespace(build=args.build, reference_selection=args.reference_selection,
                              execution_products=None)
    original, source, products_path, products, build = stage.read_action_build(options)
    selection = options._reference_selection_summary
    require(catalog['reference_selection_sha256'] == selection['sha256'], 'reference epoch mismatch')
    manifest_path = source/'compatibility/stable-31.1/sources.json'
    require(catalog['manifest_sha256'] == execution.digest(manifest_path), 'original manifest mismatch')
    requested, library_selections, input_pins = {}, {}, {}
    candidates = {p['id']: p for p in products if p['kind'] != 'reference' and not p['test_hooks']}
    require(len(candidates) == 48, 'complete 48-candidate registry required')
    for path in args.run_results:
        path = execution.safe_path(path)
        pin = bound_file(path, limit=MAX_ITEM)
        raw = execution.bounded_json(path, MAX_ITEM)
        require(raw.get('schema') == 'femacs-runtime-stage-v1' and raw.get('action') == 'execute'
                and raw.get('execution_phase') in ('all', 'stable')
                and raw.get('execution_complete') is True
                and raw.get('product_identity_unchanged') is True
                and raw.get('complete_receipts_valid') is True
                and raw.get('build_receipt') == str(original)
                and raw.get('reference_selection') == selection,
                'only closed, unchanged, matching-epoch stable stages may be classified')
        input_pins[str(path)] = pin
        explicit_libraries = raw.get('stable_product_libraries', False)
        require(type(explicit_libraries) is bool
                and (not explicit_libraries or raw.get('stable_harness_selection', {}).get('unchanged') is True),
                'explicit stable library/harness selection was not verified')
        for row in raw.get('cases', []):
            name = row.get('id', '')
            if not name.startswith('stable-'):
                continue
            profile = name[len('stable-'):]
            require(profile in candidates and profile not in requested
                    and row.get('execution_complete') is True, 'duplicate or incomplete stable pair')
            requested[profile] = row
            library_selections[profile] = explicit_libraries
    require(requested, 'no executed stable pairs supplied')
    out = execution.new_output(args.out)
    started = time.monotonic()
    before = {p['id']: execution.product_identity(p, resources=True) for p in products}
    rows, errors, checks = [], [], {}
    suite = execution.load_tool('stable-suite.py')
    reference = products[0]
    try:
        for profile, candidate in candidates.items():
            if profile not in requested:
                continue
            raw = requested[profile]
            row = dict(profile=profile, passed=False, raw_result=raw)
            try:
                root = execution.safe_path(raw['receipt']).parent
                receipt = suite.read_completion(root, manifest_path,
                    reference['program'], candidate['program'], ['--dump-file='+reference['dump']],
                    ['--dump-file='+candidate['dump']],
                    stage.stable_product_environment(reference, library_selections[profile]),
                    stage.stable_product_environment(candidate, library_selections[profile]),
                    classification=args.classification, candidate_profile=candidate,
                    reference_selection=selection)
                process = raw.get('process', {})
                expected_exit = 0 if receipt['corpus_complete'] else 1
                require(process.get('complete') is True and process.get('exit_code') == expected_exit
                        and process.get('remaining_group_terminated') is False
                        and raw.get('completion_sha256') == execution.digest(root/'stable-suite-completion.json')
                        and raw.get('receipt_sha256') == receipt['detailed']['sha256'],
                        'original child exit or raw receipt binding differs')
                ledger = out/(profile+'.json')
                execution.write_json(ledger, receipt['classification'])
                row.update(passed=receipt['classification']['classified_corpus_accepted'],
                           classification=dict(path=str(ledger), **bound_file(ledger)),
                           raw_corpus_complete=receipt['corpus_complete'],
                           raw_failure_count=receipt['failure_count'], raw_passed_count=receipt['passed_count'])
            except Exception as error:
                row['error'] = f'{type(error).__name__}: {error}'
            rows.append(row)
    finally:
        for name, operation in {
            'all61_product_identities_unchanged': lambda: before == {
                p['id']: execution.product_identity(p, resources=True) for p in products},
            'all61_complete_receipts_valid': lambda: bool(execution.load_products(products_path)),
            'source_export_unchanged': lambda: execution.verify_source_export(
                stage.build_source_owner(original, build), build['source_export']) == source,
            'reference_selection_unchanged': lambda: stage.verify_reference_selection(options, products) is None,
            'classification_catalog_unchanged': lambda: bound_file(args.classification, catalog_identity) == catalog_identity,
            'raw_stage_results_unchanged': lambda: all(bound_file(path, pin) == pin for path, pin in input_pins.items()),
        }.items():
            try:
                checks[name] = operation() is True
            except Exception as error:
                checks[name] = False
                errors.append(dict(check=name, error=f'{type(error).__name__}: {error}'))
        accepted = bool(rows) and all(row['passed'] for row in rows) and all(checks.values())
        result = dict(schema='femacs-stable-classified-stage-v1', passed=accepted,
            requested_scope_accepted=accepted, requested_profiles=[r['profile'] for r in rows],
            remaining_profiles=[p for p in candidates if p not in requested], required_candidate_count=48,
            classified_stable_phase_complete=accepted and len(rows) == 48,
            reference_selection=selection, classification_catalog=dict(path=str(args.classification), **catalog_identity),
            original_stage_inputs=input_pins, cases=rows, checks=checks, postflight_errors=errors,
            seconds=time.monotonic()-started, compiler_processes_started=0, editor_processes_started=0,
            original_results_modified=False, raw_failures_promoted=False, compatibility_accepted=False,
            goal_complete=False)
        execution.write_json(out/'results.json', result)
    return 0 if result['passed'] else 1


if __name__ == '__main__':
    raise SystemExit(main())
