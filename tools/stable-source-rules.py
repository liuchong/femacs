"""Expand portable reviewed source rules without observing or executing cases.

The generated definitions retain the existing classification input schema.
Runtime outcomes, conditions, products and epochs are bound only by assembly.
"""
import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import stat

ROOT = Path(__file__).resolve().parent.parent
MAX_BYTES = 64 * 1024 * 1024
MAX_RULES = 65536
HEX = re.compile(r'^[0-9a-f]{64}$')
NAME = re.compile(r'^[A-Za-z0-9][A-Za-z0-9._-]*$')


def require(value, message):
    if not value:
        raise ValueError(message)


def pairs(values):
    result = {}
    for key, value in values:
        require(key not in result, 'duplicate source-rule field')
        result[key] = value
    return result


def physical(path):
    path = Path(os.path.abspath(path))
    require(path.resolve() == path and not path.is_symlink() and path.is_file(),
            'ordinary physical source input required')
    return path


def read_input(path, limit=MAX_BYTES):
    path = physical(path)
    descriptor = os.open(path, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
    with os.fdopen(descriptor, 'rb') as incoming:
        before = os.fstat(incoming.fileno())
        require(stat.S_ISREG(before.st_mode) and before.st_size <= limit, 'source input exceeds bound')
        raw = incoming.read(limit+1)
        after = os.fstat(incoming.fileno())
    fields = ('st_dev', 'st_ino', 'st_size', 'st_mtime_ns', 'st_mode')
    current = path.lstat()
    require(len(raw) <= limit and all(getattr(before, key) == getattr(after, key)
            == getattr(current, key) for key in fields), 'source input changed while read')
    return raw, dict(bytes=len(raw), sha256=hashlib.sha256(raw).hexdigest())


def relative_path(value):
    require(isinstance(value, str) and bool(value), 'source-relative path required')
    path = Path(value)
    require(not path.is_absolute() and '..' not in path.parts and path.as_posix() == value,
            'canonical source-relative path required')
    return path


def prepare_overlay_inputs(spec_path, repository, cache_root):
    """Explicitly acquire missing rule anchors from the pinned corpus provider."""
    raw, _ = read_input(spec_path)
    spec = json.loads(raw, object_pairs_hook=pairs)
    required = {relative_path(anchor['path']).as_posix()
                for rule in spec['rules'] for anchor in rule['source_evidence']}
    manifest_path = repository/'compatibility/stable-31.1/sources.json'
    provider_path = Path(__file__).with_name('reference-corpus.py')
    module_spec = importlib.util.spec_from_file_location('source_rule_corpus_provider', provider_path)
    provider = importlib.util.module_from_spec(module_spec)
    module_spec.loader.exec_module(provider)
    acquired = provider.export_inputs(manifest_path, cache_root, repo=repository)
    entries = {entry['source_path']: entry for entry in acquired['manifest']['entries']}
    created = []
    for name in sorted(required):
        target = repository/name
        if os.path.lexists(target):
            physical(target)
            continue
        require(name.startswith('compatibility/stable-31.1/test/') and name in entries
                and entries[name]['input'] == 'overlay',
                'missing source anchor is not a declared original overlay: '+name)
        # The provider verified the entire selection. Copy only needed missing
        # inputs, with exclusive creation and the original blob/hash checks.
        provider.directory(target.parent, create=True)
        provider.suite._copy_source(acquired['sources'][name], target, entries[name])
        created.append(name)
    return dict(acquisition=acquired['acquisition'], created_overlay_paths=created,
                tracked_source_inputs_modified=False)


def expand(spec_path, repository):
    raw, spec_pin = read_input(spec_path)
    spec = json.loads(raw, object_pairs_hook=pairs)
    require(isinstance(spec, dict) and set(spec) == {'schema', 'profiles', 'rules'}
            and spec['schema'] == 'femacs-stable-source-rule-specs-v1', 'invalid portable source-rule specification')
    profiles, rules = spec['profiles'], spec['rules']
    require(isinstance(profiles, list) and len(profiles) == len(set(profiles)) == 48
            and all(isinstance(name, str) and NAME.fullmatch(name) for name in profiles),
            'explicit unique 48-profile source scope required')
    require(isinstance(rules, list) and 0 < len(rules)*len(profiles) <= MAX_RULES,
            'source-rule count exceeds the classification bound')
    manifest_raw, manifest_pin = read_input(repository/'compatibility/stable-31.1/sources.json')
    manifest = json.loads(manifest_raw, object_pairs_hook=pairs)
    entries = {entry['path']: entry for entry in manifest['entries']}
    inputs = {str(physical(spec_path)): spec_pin,
              str(repository/'compatibility/stable-31.1/sources.json'): manifest_pin}
    cache, seen = {}, set()
    mandatory = {'file', 'id', 'category', 'input_sha256', 'reason', 'source_evidence'}
    for rule in rules:
        require(isinstance(rule, dict) and mandatory <= set(rule)
                and set(rule) <= mandatory | {'optional_gate', 'invalid_gate', 'version_gate',
                                              'reference_defect_gate', 'environment_gate'}
                and rule['category'] in {'upstream-invalid-test', 'explicit-optional-capability',
                                         'proven-version-evolution', 'known-reference-library-defect',
                                         'approved-host-environment-limitation'}
                and isinstance(rule['reason'], str) and bool(rule['reason'].strip())
                and isinstance(rule['id'], str) and bool(rule['id']), 'invalid source-rule definition')
        logical = relative_path(rule['file']).as_posix()
        require(logical in entries and entries[logical]['sha256'] == rule['input_sha256']
                and (logical, rule['id']) not in seen, 'source rule has changed input or duplicate identity')
        seen.add((logical, rule['id']))
        anchors, original = rule['source_evidence'], False
        require(isinstance(anchors, list) and 1 <= len(anchors) <= 16, 'bounded source anchors required')
        for anchor in anchors:
            require(isinstance(anchor, dict) and set(anchor) == {'path', 'bytes', 'sha256',
                    'start_line', 'end_line', 'lf_sha256'}, 'invalid portable source anchor')
            relative = relative_path(anchor['path'])
            path = repository/relative
            if str(path) not in cache:
                contents, pin = read_input(path, 32*1024*1024)
                require(sum(len(value) for value in cache.values())+len(contents) <= MAX_BYTES,
                        'source anchor cache exceeds bound')
                cache[str(path)], inputs[str(path)] = contents, pin
            contents = cache[str(path)]
            require(inputs[str(path)] == {key: anchor[key] for key in ('bytes', 'sha256')},
                    'portable source anchor identity changed')
            start, end = anchor['start_line'], anchor['end_line']
            lines = contents.replace(b'\r\n', b'\n').split(b'\n')
            require(type(start) is int and type(end) is int and 1 <= start <= end <= len(lines)
                    and end-start <= 500, 'invalid source anchor LF range')
            excerpt = b'\n'.join(lines[start-1:end])+b'\n'
            require(hashlib.sha256(excerpt).hexdigest() == anchor['lf_sha256'], 'source anchor LF digest changed')
            original |= relative.as_posix() == entries[logical]['source_path']
        require(original, 'rule lacks its exact original test source anchor')
        if 'reference_defect_gate' in rule:
            gate = rule['reference_defect_gate']
            require(rule['category'] == 'known-reference-library-defect'
                    and isinstance(gate, dict) and gate.get('contract') == 'dframe-create-buffer-v1'
                    and gate.get('kind') == 'dframe-create-buffer'
                    and isinstance(gate.get('source_form'), str)
                    and any(gate['source_form'].encode() in b'\n'.join(
                        cache[str(repository/a['path'])].split(b'\n')[a['start_line']-1:a['end_line']])+b'\n'
                        for a in anchors), 'reference defect requires its explicit original behavior contract')
        if 'optional_gate' in rule:
            gate = rule['optional_gate']
            require(isinstance(gate, dict) and not {'missing_on', 'condition_forms', 'condition_values', 'conditions',
                    'controls', 'observations', 'context', 'epoch'} & set(gate),
                    'source gate cannot contain runtime observations or controls')
            require(rule['category'] == 'explicit-optional-capability'
                    or (rule['category'] == 'proven-version-evolution'
                        and isinstance(rule.get('version_gate'), dict)
                        and rule['version_gate'].get('kind') == 'filenotify-stopped-event-file'
                        and gate.get('kind') == 'file-monitor-identity'),
                    'optional_gate requires an explicit-optional-capability or stopped-event remote rule')
            form = gate.get('source_form')
            require(isinstance(form, str) and 0 < len(form) <= 8192
                    and any(form.encode('utf-8') in b'\n'.join(
                        cache[str(repository/a['path'])].replace(b'\r\n', b'\n').split(b'\n')[
                            a['start_line']-1:a['end_line']])+b'\n' for a in anchors),
                    'optional source form must be literal in its pinned source anchors')
            require(gate.get('kind') != 'display-svg-subcapability'
                    or (rule['category'] == 'explicit-optional-capability'
                        and logical == 'test/lisp/image-tests.el'
                        and rule['id'] in {'image-create-image-with-map', 'image--compute-map-and-original-map'}
                        and set(gate) == {'kind', 'applicable_display', 'source_form',
                                          'svg_call_form', 'control_test_id', 'control_corpus'}
                        and gate['applicable_display'] == 'cocoa'
                        and gate['source_form'] == '(skip-unless (display-images-p))'
                        and gate['control_test_id'] == 'image-supported-file-p/built-in'
                        and gate['control_corpus'] == (
                            'current' if rule['id'] == 'image--compute-map-and-original-map' else 'stable')
                        and (gate['control_corpus'] != 'current' or len(anchors) >= 2)
                        and isinstance(gate['svg_call_form'], str)
                        and any(gate['svg_call_form'].encode('utf-8') in b'\n'.join(
                            cache[str(repository/a['path'])].replace(b'\r\n', b'\n').split(b'\n')[
                                a['start_line']-1:a['end_line']])+b'\n' for a in anchors)),
                    'SVG subcapability gate requires its exact source forms and control declarations')
        if 'invalid_gate' in rule:
            invalid = rule['invalid_gate']
            require(isinstance(invalid, dict) and not {'conditions','controls','observations','context','epoch'} & set(invalid),
                    'invalid source gate cannot prefill runtime observations or controls')
            require(invalid.get('kind') != 'wall-clock-format-precision'
                    or (rule['category'] == 'upstream-invalid-test'
                        and set(invalid) == {'kind', 'source_form', 'control_test_id'}
                        and invalid['control_test_id'] == 'erc-d-run-linger-direct'),
                    'wall-clock precision gate requires its exact same-file direct control test')
            require(invalid.get('kind') != 'unpopulated-graphical-snapshot-wrapper'
                    or (rule['category'] == 'upstream-invalid-test'
                        and logical == 'test/lisp/erc/erc-scenarios-internal.el'
                        and rule['id'] == 'erc-scenarios-internal--run-graphical-all'
                        and set(invalid) == {'kind', 'source_form'}
                        and invalid['source_form'] == '(ert-skip "Environmental conditions unmet")'
                        and len(anchors) == 3),
                    'graphical snapshot wrapper gate requires its exact three upstream source anchors')
            require(invalid.get('kind') != 'display-gated-upstream-broken-test'
                    or (rule['category'] == 'upstream-invalid-test'
                        and (logical, rule['id']) in {
                            ('test/lisp/hl-line-tests.el', 'hl-line-tests-sticky-across-frames'),
                        }
                        and set(invalid) == {'kind', 'applicable_display', 'source_form',
                                             'broken_assertion_form', 'control_test_id'}
                        and invalid['applicable_display'] in {'cocoa', 'terminal'}
                        and isinstance(invalid['broken_assertion_form'], str)
                        and isinstance(invalid['control_test_id'], str)
                        and invalid['control_test_id'] != rule['id']
                        and any(invalid['broken_assertion_form'].encode('utf-8') in b'\n'.join(
                            cache[str(repository/a['path'])].replace(b'\r\n', b'\n').split(b'\n')[
                                a['start_line']-1:a['end_line']])+b'\n' for a in anchors)
                        and any(('(ert-deftest '+invalid['control_test_id']+' ()').encode('utf-8') in b'\n'.join(
                            cache[str(repository/a['path'])].replace(b'\r\n', b'\n').split(b'\n')[
                                a['start_line']-1:a['end_line']])+b'\n' for a in anchors)),
                    'display-gated broken test requires its exact assertion and sibling control anchors')
            require(invalid.get('kind') != 'removed-upstream-producer'
                    or (rule['category'] == 'upstream-invalid-test'
                        and (logical, rule['id']) == (
                            'test/lisp/emacs-lisp/eieio-tests/eieio-tests.el',
                            'eieio-test-01-mix-alloc-initarg')
                        and set(invalid) == {'kind', 'applicable_display', 'skip_form', 'source_form',
                                             'removed_producer_form', 'producer_path',
                                             'upstream_commit', 'control_test_id'}
                        and invalid['applicable_display'] == 'terminal'
                        and isinstance(invalid['skip_form'], str)
                        and isinstance(invalid['source_form'], str)
                        and isinstance(invalid['removed_producer_form'], str)
                        and isinstance(invalid['control_test_id'], str)
                        and invalid['control_test_id'] != rule['id']
                        and isinstance(invalid['upstream_commit'], str)
                        and re.fullmatch(r'[0-9a-f]{40}', invalid['upstream_commit'])
                        and (lambda producer: (
                            producer.is_file()
                            and invalid['removed_producer_form'] not in
                                producer.read_text(encoding='utf-8', errors='replace')))(
                            repository/relative_path(invalid['producer_path']))
                        and any(invalid['source_form'].encode('utf-8') in b'\n'.join(
                            cache[str(repository/a['path'])].replace(b'\r\n', b'\n').split(b'\n')[
                                a['start_line']-1:a['end_line']])+b'\n' for a in anchors)
                        and any(invalid['skip_form'].encode('utf-8') in b'\n'.join(
                            cache[str(repository/a['path'])].replace(b'\r\n', b'\n').split(b'\n')[
                                a['start_line']-1:a['end_line']])+b'\n' for a in anchors)
                        and any(('(ert-deftest '+invalid['control_test_id']+' ()').encode('utf-8') in b'\n'.join(
                            cache[str(repository/a['path'])].replace(b'\r\n', b'\n').split(b'\n')[
                                a['start_line']-1:a['end_line']])+b'\n' for a in anchors)),
                    'removed upstream producer requires its exact obsolete assertion, the absent '
                    'producer form, the pinned upstream commit and its passing sibling control')
            require(isinstance(invalid.get('source_form'), str) and 0 < len(invalid['source_form']) <= 8192
                    and any(invalid['source_form'].encode('utf-8') in b'\n'.join(
                        cache[str(repository/a['path'])].replace(b'\r\n', b'\n').split(b'\n')[
                            a['start_line']-1:a['end_line']])+b'\n' for a in anchors),
                    'invalid gate must be literal in its pinned source anchors')
        if 'version_gate' in rule:
            gate = rule['version_gate']
            expected_keys = ({'kind', 'source_form', 'upstream_commit', 'reference_library',
                              'candidate_library', 'contract'}
                             if isinstance(gate, dict) and gate.get('kind') == 'filenotify-stopped-event-file'
                             else {'kind', 'source_form', 'upstream_commit',
                                   'reference_library', 'candidate_library'})
            require(rule['category'] == 'proven-version-evolution' and isinstance(gate, dict)
                    and set(gate) == expected_keys
                    and gate['kind'] in {'ert-description-format', 'filenotify-descriptor-api',
                                         'filenotify-stopped-event-file'}
                    and (gate['kind'] != 'filenotify-stopped-event-file'
                         or gate['contract'] == 'remote-watch-lifecycle-v1')
                    and isinstance(gate['source_form'], str)
                    and any(gate['source_form'].encode('utf-8') in b'\n'.join(
                        cache[str(repository/a['path'])].replace(b'\r\n', b'\n').split(b'\n')[
                            a['start_line']-1:a['end_line']])+b'\n' for a in anchors),
                    'component version gate must bind its exact original source form')
        if 'environment_gate' in rule:
            gate = rule['environment_gate']
            require(rule['category'] == 'approved-host-environment-limitation'
                    and logical == 'test/src/process-tests.el'
                    and rule['id'] == 'non-existent-lookup-failure'
                    and isinstance(gate, dict)
                    and set(gate) == {'kind', 'source_form', 'contrast_evidence'}
                    and gate['kind'] == 'reserved-invalid-dns-interception'
                    and relative_path(gate['contrast_evidence']).as_posix()
                        == 'tools/fixtures/reserved-invalid-dns-contrast.json'
                    and isinstance(gate['source_form'], str)
                    and any(gate['source_form'].encode('utf-8') in b'\n'.join(
                        cache[str(repository/a['path'])].replace(b'\r\n', b'\n').split(b'\n')[
                            a['start_line']-1:a['end_line']])+b'\n' for a in anchors),
                    'host environment limitation gate must bind only the approved DNS case and contrast')
            contrast_path = repository/relative_path(gate['contrast_evidence'])
            if str(contrast_path) not in inputs:
                _, contrast_pin = read_input(contrast_path, 1024*1024)
                inputs[str(contrast_path)] = contrast_pin
        else:
            require(rule['category'] != 'approved-host-environment-limitation',
                    'approved host environment limitation requires an explicit environment_gate')
    profile_overhead = max(len(json.dumps(dict(profile=name), separators=(',', ':'),
        ensure_ascii=False).encode('utf-8')) for name in profiles)
    estimated_bytes = 4096
    for rule in rules:
        estimated_bytes += len(profiles) * (len(json.dumps(rule, separators=(',', ':'),
            ensure_ascii=False, allow_nan=False).encode('utf-8'))+profile_overhead)
        require(estimated_bytes <= MAX_BYTES, 'expanded source definitions exceed the catalog byte bound')
    definitions = [dict(profile=profile, **rule) for rule in rules for profile in profiles]
    return dict(schema='femacs-stable-reviewed-rule-definitions-v1', rules=definitions), inputs


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--spec', type=Path, default=ROOT/'tools/fixtures/stable-classification-source-rules.json')
    parser.add_argument('--repo', type=Path, default=ROOT)
    parser.add_argument('--prepare-overlays', action='store_true',
                        help='acquire missing original rule inputs from the pinned corpus')
    parser.add_argument('--cache-root', type=Path,
                        help='explicit corpus acquisition cache; required with --prepare-overlays')
    parser.add_argument('--out', type=Path, required=True)
    args = parser.parse_args(argv)
    try:
        repository = Path(os.path.abspath(args.repo))
        require(repository.resolve() == repository and repository.is_dir(), 'physical repository required')
        out = Path(os.path.abspath(args.out))
        require(out.resolve() == out and not os.path.lexists(out)
                and not out.is_relative_to(repository/'emacs'), 'fresh owned output outside reference source required')
        require(args.prepare_overlays == (args.cache_root is not None),
                '--prepare-overlays and --cache-root must be supplied together')
        preparation = (prepare_overlay_inputs(args.spec, repository, args.cache_root)
                       if args.prepare_overlays else None)
        definitions, inputs = expand(args.spec, repository)
        out.mkdir(parents=True)
        path = out/'reviewed-rule-definitions.json'
        with path.open('x', encoding='utf-8') as outgoing:
            json.dump(definitions, outgoing, separators=(',', ':'), ensure_ascii=False, allow_nan=False)
            outgoing.write('\n'); outgoing.flush(); os.fsync(outgoing.fileno())
        raw, definition_pin = read_input(path)
        del raw
        require(all(read_input(name)[1] == pin for name, pin in inputs.items()), 'source inputs changed at exit')
        result = dict(schema='femacs-stable-source-rule-expansion-v1', passed=True,
            definitions=dict(path=str(path), **definition_pin), source_inputs=inputs,
            profiles=48, source_rules=len(definitions['rules'])//48, expanded_rules=len(definitions['rules']),
            source_preparation=preparation,
            actual_editor_execution=False, runtime_observations_bound=False, goal_complete=False)
        (out/'results.json').write_text(json.dumps(result, indent=2)+'\n')
        print(json.dumps(dict(passed=True, source_rules=result['source_rules'], expanded_rules=result['expanded_rules'])))
        return 0
    except (OSError, ValueError, KeyError, TypeError) as error:
        parser.error(str(error))


if __name__ == '__main__':
    raise SystemExit(main())
