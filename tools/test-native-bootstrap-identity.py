"""File/CLI integration for container analysis; no editor or compiler execution."""
import argparse
import copy
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import shutil
import struct
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parent.parent


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def write(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open('x') as stream:
        json.dump(value, stream, indent=2)
        stream.write('\n')


def pin(path):
    return dict(bytes=path.stat().st_size, sha256=hashlib.sha256(path.read_bytes()).hexdigest())


def container(path, number):
    """Make a complete tiny supported layout with a real page-hash chain."""
    library = path.name.encode()+b'\0'
    name_field = library+b'\0'*((-len(library)) % 8)
    identifier = 88
    hashes = identifier+len(name_field)
    cd_size = hashes+32
    signature_size = 20+cd_size
    ident = b'/owned/'+library
    dylib_size = (24+len(ident)+7)//8*8
    section = struct.pack('<16s16sQQ8I', b'__text', b'__TEXT', 512, 128,
                          512, 2, 0, 0, 0x80000400, 0, 0, 0)
    text = struct.pack('<II16s4QiiII', 0x19, 152, b'__TEXT', 0, 4096,
                       0, 4096, 7, 5, 1, 0)+section
    link = struct.pack('<II16s4QiiII', 0x19, 72, b'__LINKEDIT', 4096, 4096,
                       4096, signature_size, 1, 1, 0, 0)
    dylib = struct.pack('<6I', 0xd, dylib_size, 24, 0, 0, 0)+ident
    dylib += b'\0'*(dylib_size-len(dylib))
    uuid = struct.pack('<II16s', 0x1b, 24, bytes([number])*16)
    signing = struct.pack('<4I', 0x1d, 16, 4096, signature_size)
    commands = text+link+dylib+uuid+signing
    raw = bytearray(4096)
    raw[:32] = struct.pack('<IiiIIIII', 0xfeedfacf, 0x0100000c, 0, 6, 5, len(commands), 0, 0)
    raw[32:32+len(commands)] = commands
    raw[512:640] = bytes(range(128))
    directory = struct.pack('>9I4BI', 0xfade0c02, cd_size, 0x20400, 0x20002,
        hashes, identifier, 0, 1, 4096, 32, 2, 0, 12, 0)
    directory += struct.pack('>III4Q', 0, 0, 0, 0, 512, 128, 0)
    directory += name_field+hashlib.sha256(raw).digest()
    blob = struct.pack('>5I', 0xfade0cc0, signature_size, 1, 0, 20)+directory
    with path.open('xb') as stream:
        stream.write(raw+blob)


def integration(out):
    owned = out/'owned-fixtures'
    owned.mkdir()
    rows, children = [], []
    started = time.monotonic()
    tool = load('native_identity_integration', ROOT/'tools/native-bootstrap-identity.py')
    retire = load('native_identity_retirement', ROOT/'tools/runtime-retire.py')
    suite = load('native_identity_file_protocol', ROOT/'tools/stable-suite.py')
    try:
        state = owned/'container'/'case'
        names = ('home', 'cache', 'config', 'state', 'tmp', 'native-cache', 'out', 'cwd')
        for name in names:
            (state/name).mkdir(parents=True)
        first, second = [state/'native-cache'/('fixture-comp-stage'+str(n)+'-aaaa.eln') for n in (1, 2)]
        container(first, 1)
        container(second, 2)
        tick = time.monotonic()
        value = tool.compare(first, second)
        assert value['differs_only_in_validated_metadata'] and value['all_sections_equal']
        assert value['normalized_signatures_equal'] and value['inputs_unchanged_at_exit']
        assert value['difference_counts']['outside_metadata'] == 0
        assert all(row['signature']['all_page_hashes_valid'] for row in value['layouts'])
        rows.append(dict(id='complete-container-identity-analysis', passed=True,
                         seconds=time.monotonic()-tick))
        tick = time.monotonic()
        same = tool.compare(first, first)
        assert same['byte_identical'] and not same['differs_only_in_validated_metadata']
        rows.append(dict(id='identical-container-is-recorded-without-invented-differences', passed=True,
                         seconds=time.monotonic()-tick))
        tick = time.monotonic()
        pair = owned/'pair'
        result_path = pair/'results'/'candidate'/'compiler.json'
        progress = result_path.with_suffix('.events.jsonl')
        progress.parent.mkdir(parents=True)
        condition = '(ert-test-failed ((should (= (call-process "cmp" nil nil nil comp1-eln comp2-eln) 0)) :form (= 1 0) :value nil))'
        event = dict(event='test-ended', file='test/src/comp-tests.el', test_id='comp-tests-bootstrap',
            status='failed', condition=condition, diagnostic='synthetic original-protocol cmp result',
            messages='Comparing '+str(first)+' '+str(second)+'\n', duration=0.01)
        raw = json.dumps(event).encode()
        loaded = dict(event='loaded', file=event['file'], ids=[event['test_id']],
            features=['native-compile'], native_comp_available=True,
            selector='all-discovered-ids', tests=[dict(id=event['test_id'], tags=[])])
        before = dict(event='test-started', file=event['file'], test_id=event['test_id'])
        finished = dict(event='finished', file=event['file'], status='complete', requested_count=1, ended_count=1)
        progress.write_bytes(b'\n'.join([json.dumps(loaded).encode(), json.dumps(before).encode(),
                                         raw, json.dumps(finished).encode()])+b'\n')
        progress_relative = progress.relative_to(pair).as_posix()
        case = dict(test_id=event['test_id'], status='failed', complete=True, exit_code=0, seconds=0.01,
            diagnostic=event['diagnostic'], evidence=dict(path=progress_relative,
                event_sha256=hashlib.sha256(raw).hexdigest(), event_ordinal=0))
        case['assessment'] = suite.assess_case(event['test_id'], case)
        owner_path = result_path.with_name(result_path.stem+'.owner-retirement.json')
        write(result_path, dict(schema='femacs-stable-file-batch-v1', product='candidate',
            file=event['file'], selector='all-discovered-ids', complete=True, exit_code=0,
            timed_out=False, remaining_group_terminated=False, cases={event['test_id']:case},
            ids=[event['test_id']], metadata=suite._metadata(loaded), tags={event['test_id']:[]},
            progress=dict(path=progress_relative, **pin(progress)), owned_state_root=str(state),
            owned_container=str(state.parent), state_retirement_path=owner_path.relative_to(pair).as_posix()))
        snapshot = retire.snapshot_owned_state(state, names, allow_symlinks=True,
            empty_parents=((state, state.stat()), (state.parent, state.parent.stat())))
        proof_path = result_path.with_name(result_path.stem+'.retained-state.private.json')
        write(proof_path, dict(schema='femacs-stable-retained-state-v1',
            evidence_sha256=pin(result_path)['sha256'], snapshot=snapshot))
        write(owner_path, dict(schema='femacs-stable-owner-retirement-v1',
            file_result=dict(path=result_path.relative_to(pair).as_posix(), **pin(result_path)),
            state_retirement=dict(status='retained', attempt_observation=dict(clear=True),
                closure_state_proof=dict(path=proof_path.name, **pin(proof_path)))))
        command = [sys.executable, '-B', str(ROOT/'tools/native-bootstrap-identity.py'),
            '--pair-root', str(pair), '--file-result', str(result_path), '--out', str(out/'closed-file')]
        process = subprocess.run(command, capture_output=True, text=True, timeout=20)
        children.append(dict(exit_code=process.returncode, complete=True,
            stdout=process.stdout[:2048], stderr=process.stderr[:8192]))
        assert process.returncode == 0, children[-1]
        closure = json.loads((out/'closed-file/results.json').read_bytes())
        assert closure['file_closure_bound'] and not closure['closed_outer_stage_bound']
        assert closure['differs_only_in_validated_metadata']
        assert closure['file_closure']['event_sha256'] == case['evidence']['event_sha256']
        assert closure['file_closure']['tree_sha256'] == snapshot['tree_sha256']
        assert not closure['original_failed_results_promoted'] and not closure['closed_case_bound']
        retire.verify_owned_state(snapshot)
        rows.append(dict(id='sealed-file-event-and-closure-state-cli-binding', passed=True,
                         seconds=time.monotonic()-tick))
        tick = time.monotonic()
        accounting = load('native_bootstrap_accounting', ROOT/'tools/stable-classification.py')
        original = owned/'compiler-source.el'
        form = '(should (= (call-process "cmp" nil nil nil comp1-eln comp2-eln) 0))'
        original.write_text('(when (native-comp-available-p) (load (native-compile comp-test-src)))\n'
            '(defvar native-comp-eln-load-path) (car (last native-comp-eln-load-path)) '+form+'\n')
        source_bytes = original.read_bytes()
        anchors = [dict(path=str(original), **pin(original), start_line=n, end_line=n,
            lf_sha256=hashlib.sha256(source_bytes.split(b'\n')[n-1]+b'\n').hexdigest()) for n in (1, 2)]
        reference_path = pair/'results'/'reference'/'compiler.json'
        reference_path.parent.mkdir()
        reference_progress = reference_path.with_suffix('.events.jsonl')
        reference_progress.write_bytes(progress.read_bytes())
        reference_file = json.loads(result_path.read_bytes())
        reference_file.update(product='reference', progress=dict(
            path=reference_progress.relative_to(pair).as_posix(), **pin(reference_progress)),
            state_retirement_path=reference_path.with_name('compiler.owner-retirement.json').relative_to(pair).as_posix())
        reference_file['cases'][event['test_id']]['evidence']['path'] = reference_file['progress']['path']
        write(reference_path, reference_file)
        reference_proof = reference_path.with_name('compiler.retained-state.private.json')
        write(reference_proof, dict(schema='femacs-stable-retained-state-v1',
            evidence_sha256=pin(reference_path)['sha256'], snapshot=snapshot))
        write(reference_path.with_name('compiler.owner-retirement.json'),
            dict(schema='femacs-stable-owner-retirement-v1',
                file_result=dict(path=reference_path.relative_to(pair).as_posix(), **pin(reference_path)),
                state_retirement=dict(status='retained', attempt_observation=dict(clear=True),
                    closure_state_proof=dict(path=reference_proof.name, **pin(reference_proof)))))
        both = dict(file=event['file'], id=event['test_id'],
                    reference=reference_file['cases'][event['test_id']], candidate=case)
        file_row = {}
        for label, path in [('reference', reference_path), ('candidate', result_path)]:
            file_row[label] = dict(json.loads(path.read_bytes()),
                                  file_result=dict(path=path.relative_to(pair).as_posix(), **pin(path)))
        rule = dict(category='upstream-invalid-test', reason='Synthetic sealed bootstrap integration.',
            input_sha256=pin(original)['sha256'], source_evidence=anchors,
            invalid_gate=dict(kind='native-bootstrap-container', source_form=form),
            observations={s:accounting.observation_signature(both[s]) for s in ('reference', 'candidate')})
        proofs = accounting._accept_rule(rule, both, file_row,
            dict(size=len(source_bytes), sha256=pin(original)['sha256']), pair, suite, {},
            dict(id='synthetic-native', native=True), {})
        assert str(first) in proofs and str(second) in proofs and both['candidate']['status'] == 'failed'
        assert not both['candidate']['assessment']['passed']
        for name, identity in proofs.items():
            assert accounting.bound_file(name, identity) == identity
        retire.verify_owned_state(snapshot)
        rows.append(dict(id='original-event-to-classification-reader-sealed-bootstrap-flow', passed=True,
                         seconds=time.monotonic()-tick, raw_failures_preserved=True))
        tick = time.monotonic()
        off_root = owned/'native-off-pair'
        shutil.copytree(pair, off_root)
        off_file = copy.deepcopy(json.loads(result_path.read_bytes()))
        off_loaded = dict(loaded, features=[], native_comp_available=False)
        off_event = dict(event, condition='(void-variable native-comp-eln-load-path)',
                         diagnostic='synthetic original-protocol unavailable bootstrap variable')
        off_raw = json.dumps(off_event).encode()
        off_progress = off_root/progress_relative
        off_progress.write_bytes(b'\n'.join([json.dumps(off_loaded).encode(), json.dumps(before).encode(),
                                           off_raw, json.dumps(finished).encode()])+b'\n')
        off_case = dict(case, diagnostic=off_event['diagnostic'], evidence=dict(path=progress_relative,
            event_sha256=hashlib.sha256(off_raw).hexdigest(), event_ordinal=0))
        off_case['assessment'] = suite.assess_case(event['test_id'], off_case)
        off_file.update(metadata=suite._metadata(off_loaded), cases={event['test_id']:off_case},
                        progress=dict(path=progress_relative, **pin(off_progress)))
        off_path = off_root/result_path.relative_to(pair)
        # This second flow uses its own wire files. No native image proof is
        # claimed for the disabled candidate; the reference proof is retained.
        off_path.unlink()
        write(off_path, off_file)
        off_proof = off_root/proof_path.relative_to(pair)
        off_proof.unlink()
        write(off_proof, dict(schema='femacs-stable-retained-state-v1',
                             evidence_sha256=pin(off_path)['sha256'], snapshot=snapshot))
        off_owner = off_root/owner_path.relative_to(pair)
        off_owner.unlink()
        write(off_owner, dict(schema='femacs-stable-owner-retirement-v1',
            file_result=dict(path=off_path.relative_to(off_root).as_posix(), **pin(off_path)),
            state_retirement=dict(status='retained', attempt_observation=dict(clear=True),
                closure_state_proof=dict(path=off_proof.name, **pin(off_proof)))))
        off_pair = dict(both, candidate=off_case)
        off_rows = copy.deepcopy(file_row)
        off_rows['candidate'] = dict(off_file, file_result=dict(
            path=off_path.relative_to(off_root).as_posix(), **pin(off_path)))
        off_rule = dict(rule, observations={s:accounting.observation_signature(off_pair[s])
                                            for s in ('reference', 'candidate')})
        proofs = accounting._accept_rule(off_rule, off_pair, off_rows,
            dict(size=len(source_bytes), sha256=pin(original)['sha256']), off_root, suite, {},
            dict(id='synthetic-native-off', native=False), {})
        assert str(first) in proofs and str(second) in proofs
        assert off_pair['candidate']['status'] == 'failed' and not off_pair['candidate']['assessment']['passed']
        retire.verify_owned_state(snapshot)
        rows.append(dict(id='native-disabled-bootstrap-distinct-per-side-explanation', passed=True,
                         seconds=time.monotonic()-tick, raw_failures_preserved=True))
        return dict(schema='femacs-native-bootstrap-integration-v1', passed=True, cases=rows,
            processes=children, actual_python_children=len(children),
            actual_editor_execution=False, actual_compiler_execution=False,
            synthetic_container_inputs=True, original_corpus_execution=False,
            original_failed_results_promoted=False, owned_fixture_retired=True,
            seconds=time.monotonic()-started, goal_complete=False)
    finally:
        # Only fresh synthetic files under this invocation; no editor helpers,
        # external case directory or immutable product is part of this scope.
        shutil.rmtree(owned)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out', type=Path, required=True)
    args = parser.parse_args()
    out = Path(os.path.abspath(args.out))
    if out.resolve() != out or out.exists() or out.is_relative_to(ROOT/'emacs'):
        parser.error('fresh physical owned output required')
    out.mkdir(parents=True)
    result = integration(out)
    write(out/'results.json', result)
    print(json.dumps(dict(passed=result['passed'], cases=len(result['cases']))))
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
