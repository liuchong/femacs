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
import stat
import argparse
import importlib.util
import sys
import time
from types import SimpleNamespace

SCHEMA = 'femacs-stable-classification-v1'
RESULT_SCHEMA = 'femacs-stable-classified-accounting-v1'
MAX_CATALOG = 16 * 1024 * 1024
MAX_ITEM = 64 * 1024 * 1024
MAX_CASES = 100000
CATEGORIES = {'upstream-invalid-test', 'explicit-optional-capability',
              'proven-version-evolution'}
REQUIRED_FEATURES = {'threads', 'thread', 'modules', 'module', 'dynamic-modules',
                     'gc', 'bytecode', 'pdumper', 'terminal', 'input-method'}


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
            and len(value['contexts']) <= 48 and len(value['rules']) <= 8192,
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


def detailed_rows(path):
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
                if key in {'cases', 'files', 'failures'}:
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


def _source_evidence(rule, entry):
    evidence = rule.get('source_evidence')
    require(isinstance(evidence, list) and 1 <= len(evidence) <= 16,
            'exact source anchors required for every classified case')
    original = False
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
        bound_file(pin['path'], dict(bytes=pin['bytes'], sha256=pin['sha256']), 32 * 1024 * 1024)
    require(original, 'classification does not anchor the unchanged original test')


def _accept_rule(rule, case, file_row, entry, root, api, reference_selection, profile):
    category = rule.get('category')
    require(category in CATEGORIES and isinstance(rule.get('reason'), str)
            and bool(rule['reason'].strip()), 'classification category and reason required')
    require(rule.get('input_sha256') == entry['sha256'], 'classified original test identity changed')
    actual = {label: observation_signature(case[label]) for label in ('reference', 'candidate')}
    require(rule.get('observations') == actual, 'classification does not match actual observations')
    for label in ('reference', 'candidate'):
        evidence = case[label].get('evidence', {})
        require(evidence.get('path') == file_row[label].get('progress', {}).get('path'),
                'classified case progress identity differs from file')
        event = None
        with (root/api._relative_path(evidence['path'], 'case progress')).open('rb') as incoming:
            while raw := incoming.readline(MAX_ITEM + 1):
                require(len(raw) <= MAX_ITEM, 'oversized case event')
                if hashlib.sha256(raw.rstrip(b'\n')).hexdigest() == evidence.get('event_sha256'):
                    event = json.loads(raw, object_pairs_hook=_pairs)
                    break
        require(isinstance(event, dict) and event.get('event') == 'test-ended'
                and event.get('file') == case['file'] and event.get('test_id') == case['id']
                and event.get('status') == case[label]['status']
                and event.get('diagnostic', '') == case[label]['diagnostic'],
                'classified outcome is not the actual bound runner event')
    statuses = [case[label]['status'] for label in ('reference', 'candidate')]
    if category == 'upstream-invalid-test':
        require(statuses == ['failed', 'failed'], 'invalid-test rule must retain two actual failures')
    elif category == 'explicit-optional-capability':
        require('skipped' in statuses and set(statuses) <= {'passed', 'skipped'},
                'optional capability cannot excuse a failed test')
        gate = rule.get('optional_gate', {})
        feature = gate.get('feature')
        missing = gate.get('missing_on')
        require(isinstance(feature, str) and feature and feature not in REQUIRED_FEATURES
                and isinstance(missing, list) and bool(missing)
                and len(set(missing)) == len(missing)
                and set(missing) <= {'reference', 'candidate'}, 'invalid optional feature boundary')
        for label in missing:
            metadata = file_row[label].get('metadata', {})
            require(case[label]['status'] == 'skipped'
                    and isinstance(metadata.get('features'), list)
                    and feature not in metadata['features'], 'optional feature absence not observed')
        if feature == 'native-compile' and 'candidate' in missing:
            require(profile.get('native') is False, 'declared native capability remains required')
        if feature in {'ns', 'window-system'} and 'candidate' in missing:
            require(profile.get('kind') == 'terminal', 'declared window system remains required')
    else:
        require(statuses == ['failed', 'failed'] and len(rule.get('source_evidence', [])) >= 2,
                'version evolution requires matching failures and both source versions')
        control = rule.get('version_control', {})
        require(isinstance(control.get('path'), str), 'passing same-version control required')
        bound_file(control['path'], control.get('identity'), MAX_ITEM)
        value = json.loads(read_bytes(control['path'], control.get('identity'), MAX_ITEM),
                           object_pairs_hook=_pairs)
        fixture = value.get('working_fixture', {})
        control_input = control.get('test_input', {})
        bound_file(control_input.get('path', ''), control_input.get('identity'), 32 * 1024 * 1024)
        require(value.get('schema') == 'femacs-stable-file-batch-v1'
                and value.get('file') == case['file'] and value.get('complete') is True
                and value.get('passed') is True and value.get('exit_code') == 0
                and value.get('timed_out') is False and value.get('remaining_group_terminated') is False
                and value.get('argv', [None])[0] == file_row['candidate']['argv'][0]
                and [arg for arg in value['argv'] if arg.startswith('--dump-file=')]
                    == [arg for arg in file_row['candidate']['argv'] if arg.startswith('--dump-file=')]
                and value.get('metadata', {}).get('emacs_version') == reference_selection['upstream']['version']
                and fixture.get('canonical_fixture_inputs_unchanged') is True
                and any(pin.get('path') == case['file'] and pin.get('before', {}).get('sha256')
                    == control_input.get('identity', {}).get('sha256') for pin in fixture.get('files', []))
                and api.assess_case(case['id'], value.get('cases', {}).get(case['id']))['passed'] is True,
                'same-version control did not actually pass the matching case')
        bound_file(control['path'], control.get('identity'), MAX_ITEM)
    _source_evidence(rule, entry)


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
    require(selected == dict(context_sha256=context, run_sha256=receipt['detailed']['sha256'],
            completion_sha256=api._sha256_path(root/'stable-suite-completion.json')),
            'classification does not bind this exact complete run and candidate')
    entries = api._load_batch().select_entrypoints(manifest)
    by_path = {entry['path']: entry for entry in entries}
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
                     'metadata', 'file_result', 'progress', 'argv')}
        elif name == 'cases':
            # Cases can precede file metadata in the frozen writer. Read them
            # in a second streaming pass, never keep all diagnostic payloads.
            pass
        elif name == 'failures':
            require(isinstance(value, str) and value not in failures, 'invalid/duplicate raw failure')
            failures.add(value)
        elif not name.endswith('-end'):
            metadata[name] = value
    structural = []
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
    ordinary, classified, blocked, used, cases, proof_inputs = 0, [], [], set(), set(), {}
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
        if raw_passed:
            ordinary += 1
            continue
        failure = 'case failed/unverified: ' + name + ' :: ' + identity
        require(failure in failures, 'nonpassing case missing its raw failure')
        failures.remove(failure)
        key = (name, identity)
        if key in rules:
            _accept_rule(rules[key], case, files[name], by_path[name], root, api, reference_selection, profile)
            for pin in rules[key]['source_evidence']:
                proof_inputs[pin['path']] = dict(bytes=pin['bytes'], sha256=pin['sha256'])
            control = rules[key].get('version_control')
            if control is not None:
                proof_inputs[control['path']] = control['identity']
                proof_inputs[control['test_input']['path']] = control['test_input']['identity']
            used.add(key)
            classified.append(dict(file=name, id=identity, category=rules[key]['category'],
                                   observations=rules[key]['observations']))
        else:
            blocked.append(dict(file=name, id=identity,
                reason=('potential-port-regression' if case['reference']['status'] == 'passed'
                        else 'unexplained-failure-or-skip'),
                reference_status=case['reference']['status'], candidate_status=case['candidate']['status']))
    require(used == set(rules), 'classification contains unused or ordinarily passing cases')
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
        bound_file(path, identity, MAX_ITEM)
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
        classified_corpus_accepted=bool(cases) and not structural and not blocked,
        raw_results_modified=False, classified_cases_counted_as_passed=False,
        retained_failure_state_removed=False, compatibility_accepted=False, goal_complete=False)


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--build', type=Path, required=True)
    parser.add_argument('--reference-selection', type=Path, required=True)
    parser.add_argument('--classification', type=Path, required=True)
    parser.add_argument('--run-results', type=Path, action='append', required=True,
                        help='closed original runtime-stage results; repeat for distinct stable scopes')
    parser.add_argument('--out', type=Path, required=True)
    args = parser.parse_args(argv)
    tool_root = Path(__file__).resolve().parent
    spec = importlib.util.spec_from_file_location('classification_stage', tool_root/'runtime-stage.py')
    stage = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(stage)
    return run(stage, args)


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
