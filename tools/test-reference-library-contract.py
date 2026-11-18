"""Integration of generated library builds and expiring reference defect rules.

Synthetic full555 wire cases exercise classification, never editor acceptance.
The optional real-product group compiles the generated component and exercises
actual frame callbacks with an explicit integration override. Full product and
original corpus acceptance must subsequently use built product resources.
"""
import argparse
import copy
from contextlib import ExitStack, redirect_stderr
import hashlib
import importlib.util
import io
import json
import os
from pathlib import Path
import shutil
import sys
import time
from types import SimpleNamespace
from unittest.mock import patch

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT/'tools'))
import library_corrections as libraries


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


base = load('reference_library_test_base', ROOT/'tools/test-stable-classification.py')
stage, e, suite, accounting = base.stage, base.e, base.suite, base.accounting


def owned_test_output(path):
    """Keep explicit disposable test output out of a cleared product tree."""
    path = libraries.physical(path)
    allowed = (ROOT/'build', Path.home()/'AI/caches')
    libraries.require(any(path.is_relative_to(base) and path != base for base in allowed),
                      'test output must be a fresh owned build/cache child')
    path.parent.mkdir(parents=True, exist_ok=True)
    path.mkdir(mode=0o700)
    return path


def lifecycle_integration(fixture, rows, *, on_active=None):
    context = base._fixture(fixture/'synthetic')
    definitions = json.loads((ROOT/'tools/fixtures/stable-classification-source-rules.json').read_text())
    definition = copy.deepcopy(next(row for row in definitions['rules']
                                   if row['category'] == 'known-reference-library-defect'))
    definition['profile'] = context.profile['id']
    # These are explicitly synthetic library descriptors, not the installed
    # reference. Bind their actual copied bytes rather than pretending that an
    # unpatched source copy has the installed distribution's file identity.
    definition['reference_defect_gate']['reference_sources'] = {
        name: dict(bytes=(ROOT/'lisp'/name).stat().st_size, sha256=e.digest(ROOT/'lisp'/name))
        for name in definition['reference_defect_gate']['reference_sources']}
    logical, test_id = definition['file'], definition['id']
    original = ROOT/logical
    target = context.source/logical
    target.parent.mkdir(parents=True)
    shutil.copy2(original, target)
    raw = target.read_bytes()
    context.entries[0].update(path=logical, source_path=logical, size=len(raw), sha256=e.digest(target),
        blob=hashlib.sha1(b'blob '+str(len(raw)).encode()+b'\0'+raw).hexdigest())
    manifest = e.bounded_json(context.manifest)
    manifest.update(entries=context.entries, total_bytes=sum(v['size'] for v in context.entries))
    e.write_json(context.manifest, manifest)
    root = fixture/'raw-full555'
    process = e.run_process([sys.executable, ROOT/'tools/test-stable-classification.py',
        '--child', root, '--manifest', context.manifest, '--fixture-repo', context.source,
        '--program', context.program,
        '--dump', context.dump], fixture/'synthetic-writer', dict(os.environ), 30, cwd=ROOT)
    if not (process['complete'] and process['exit_code'] == 0):
        e.write_json(fixture.parent/'failed-child.json',dict(process=process,
            **{name:(fixture/'synthetic-writer'/(name+'.txt')).read_bytes()[:32768].decode('utf-8','replace')
               for name in ('stdout','stderr')}))
        raise AssertionError('synthetic writer did not complete; failed-child.json retained')
    template = e.bounded_json(root/'stable-suite-run.json')
    envs, paths = {}, {}
    for label in ('reference', 'candidate'):
        library = fixture/(label+'-library')
        for name in definition['reference_defect_gate']['reference_sources']:
            destination = library/name
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(ROOT/'lisp'/name, destination)
        (library/'dframe.elc').write_bytes(b'synthetic bytecode descriptor\n')
        envs[label] = dict(template['runtime_environment'][label], FEMACS_STABLE_PRODUCT_LISP=str(library))
        paths[label] = suite._product_paths(context.program, ['--dump-file='+str(context.dump)], envs[label])
    reviewed = fixture/'reviewed.json'
    e.write_json(reviewed, dict(schema='femacs-stable-reviewed-rule-definitions-v1', rules=[definition]))
    product_file = fixture/'products.json'
    e.write_json(product_file, dict(schema=1, products=context.products))
    build = dict(source_export=dict(synthetic=True))

    def admission(options):
        options._reference_selection_summary = context.selection
        return context.original, context.source, product_file, context.products, build

    flows = [('active-reference-only-defect', 'failed', 'passed', False, True, 1),
             ('resolved-upstream-fixed', 'passed', 'passed', True, True, 0),
             ('candidate-regression-after-upstream-fix', 'passed', 'failed', True, False, 0),
             ('candidate-correct-behavior-required', 'failed', 'failed', False, False, 0),
             ('reference-changed-revalidation-required', 'failed', 'passed', True, False, 0),
             ('correct-behavior-control-required', 'passed', 'passed', True, False, 0)]
    if on_active is not None:
        flows = flows[:1]
    for index, (expected, reference_status, candidate_status, changed, accepted, count) in enumerate(flows):
        started = time.monotonic()
        reference_source = Path(envs['reference']['FEMACS_STABLE_PRODUCT_LISP'])/'dframe.el'
        reference_source.write_bytes((ROOT/'lisp/dframe.el').read_bytes() + (b'\n;; changed source\n' if changed else b''))
        report = copy.deepcopy(template)
        identities = {label:suite._public_product_identity(path) for label,path in paths.items()}
        report.update(runtime_environment=envs, products_before=identities, products_after=identities)
        case, file = report['cases'][0], report['files'][0]
        case.update(file=logical, id=test_id, passed=reference_status == candidate_status == 'passed')
        file['file'] = logical
        for label, status in (('reference', reference_status), ('candidate', candidate_status)):
            observed = dict(case[label], test_id=test_id, status=status, diagnostic='synthetic '+status)
            observed['assessment'] = suite.assess_case(test_id, observed)
            event = dict(event='test-ended', file=logical, test_id=test_id, status=status,
                diagnostic=observed['diagnostic'], condition=(
                    definition['reference_defect_gate']['expected_reference_condition'] if status == 'failed' else None))
            payload = json.dumps(event).encode()
            progress, result = root/('0-'+label+'.events.jsonl'), root/('0-'+label+'.json')
            progress.write_bytes(payload+b'\n')
            observed['evidence'] = dict(path=progress.name, event_sha256=hashlib.sha256(payload).hexdigest(), event_ordinal=0)
            value = dict(file[label], file=logical, ids=[test_id], cases={test_id:observed},
                passed=status == 'passed', runtime_environment=envs[label],
                progress=dict(path=progress.name, **accounting.bound_file(progress)))
            value.pop('file_result', None)
            e.write_json(result, value)
            file[label] = dict(value, file_result=dict(path=result.name, **accounting.bound_file(result)))
            case[label] = observed
        trace = root/'reference-child.runtime-observations.jsonl'
        trace.write_text(json.dumps(dict(schema='femacs-stable-runtime-observation-v1',
            file=logical, test_id=test_id, pid=12345, phase='original-terminal-child-test-ended',
            child_test_result=dict(program=str(context.program), dump='--dump-file='+str(context.dump),
                test_id=test_id, status=reference_status,
                condition=definition['reference_defect_gate']['expected_child_condition'])))+'\n')
        file['reference']['diagnostic_trace'] = dict(path=trace.name, **accounting.bound_file(trace))
        report['failures'] = [] if case['passed'] else ['case failed/unverified: '+logical+' :: '+test_id]
        report['corpus_complete'] = not report['failures']
        suite._write_run_report(root, report)
        control = fixture/('contract-'+str(index)+'.json')
        observation = dict(contract='dframe-create-buffer-v1', correct_behavior=True, noninteractive=False, rows=[dict(
            passed=True, callback_calls=1, callback_buffer_correct=True, callback_frame_correct=True,
            continuation_correct=True) for _ in range(2)])
        e.write_json(control, dict(schema='femacs-runtime-frame-buffer-probe-v1', contract='dframe-create-buffer-v1',
            passed=True, selected_inputs_unchanged=True, generated_override_used=False,
            reference_selection_sha256=context.selection['sha256'],
            input_pins={str(path):e.file_identity(path) for path in [
                *(ROOT/'tools'/name for name in ('runtime-frame-buffer-probe.py', 'fixtures/runtime-library-contracts.el')),
                *(Path(envs[label]['FEMACS_STABLE_PRODUCT_LISP'])/'dframe.elc' for label in ('reference', 'candidate'))]},
            cases=[dict(profile=(context.profile['id'] if label == 'candidate' else 'installed-reference'),
                stable_product_identity=identities[label], collection_complete=True,
                process=dict(complete=True, exit_code=0, remaining_group_terminated=False,
                    argv=['/usr/bin/script','-e','-q','synthetic-transcript',str(context.program),
                          '--dump-file='+str(context.dump),'-nw','-Q','-l',
                          str(Path(envs[label]['FEMACS_STABLE_PRODUCT_LISP'])/'dframe.elc')]),
                observation=dict(observation, library_origin=str(Path(envs[label]['FEMACS_STABLE_PRODUCT_LISP'])/'dframe.elc')))
                for label in ('reference', 'candidate')]))
        stage_result = fixture/('stage-'+str(index)+'.json')
        e.write_json(stage_result, dict(schema='femacs-runtime-stage-v1', action='execute', execution_phase='stable',
            execution_complete=True, product_identity_unchanged=True, complete_receipts_valid=True,
            build_receipt=str(context.original), reference_selection=context.selection,
            stable_product_libraries=True, stable_harness_selection=dict(unchanged=True),
            cases=[dict(id='stable-'+context.profile['id'], receipt=str(root/'stable-suite-run.json'),
                receipt_sha256=e.digest(root/'stable-suite-run.json'), completion_sha256=e.digest(root/'stable-suite-completion.json'),
                execution_complete=True, process=dict(complete=True, remaining_group_terminated=False,
                    exit_code=0 if report['corpus_complete'] else 1))]))
        args = SimpleNamespace(build=context.original, reference_selection=fixture/'selection.json',
            approved_rules=reviewed, run_results=[stage_result], control_run=[],
            library_contract_run=[] if expected == 'correct-behavior-control-required' else [control],
            out=fixture/('assembly-'+str(index)))
        def environment(product, _explicit):
            return envs['reference' if product['kind'] == 'reference' else 'candidate']
        with ExitStack() as stack:
            for obj, name, kw in ((stage, 'read_action_build', dict(side_effect=admission)),
                (stage, 'verify_reference_selection', dict(return_value=None)),
                (stage, 'build_source_owner', dict(return_value=context.original)),
                (stage, 'stable_product_environment', dict(side_effect=environment)),
                (e, 'load_products', dict(return_value=context.products)),
                (e, 'verify_source_export', dict(return_value=context.source)),
                (e, 'new_output', dict(side_effect=owned_test_output))):
                stack.enter_context(patch.object(obj, name, **kw))
            code = accounting.assemble(stage, args)
        result = e.bounded_json(args.out/'results.json')
        assert result['reference_defect_lifecycle'][0]['state'] == expected, result
        assert result['passed'] is accepted and code == (0 if accepted else 1), result
        assert result['bound_rule_count'] == count
        if expected == 'correct-behavior-control-required':
            ledger = e.bounded_json(args.out/'accounting/results.json')
            assert ledger['passed'] is False
            assert ledger['cases'][0]['classification']
            accounting_result = e.bounded_json(Path(ledger['cases'][0]['classification']['path']))
            assert accounting_result['raw_passed_count'] == accounting_result['raw_case_count'] == 555
            assert accounting_result['raw_failure_count'] == 0
            assert accounting_result['structural_failure_count'] == 2
        assert e.bounded_json(root/'stable-suite-completion.json')['failure_count'] == (0 if case['passed'] else 1)
        rows.append(dict(id='reference-lifecycle-'+expected, passed=True, seconds=time.monotonic()-started,
                         actual_editor_execution=False, full_synthetic_file_count=555))
        if on_active is not None:
            on_active(context, root, report, file, args, result)


def real_integration(fixture, products, selection, rows, *, binding_arg='--reference-selection'):
    started = time.monotonic()
    repo = fixture/'candidate'
    (repo/'tools/fixtures').mkdir(parents=True)
    shutil.copy2(ROOT/libraries.RECIPE, repo/libraries.RECIPE)
    _, prepared = libraries.recipe(ROOT)
    for name in prepared[0][0]['source_inputs']:
        target = repo/name
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(ROOT/name, target)
    generated = libraries.generate(repo)
    registry = e.bounded_json(products)
    program = next(p for p in registry['products'] if p['id'] == 'cocoa-native-normal')
    environment = dict(os.environ, OMP_THREAD_LIMIT='1')
    compiled = e.run_process([program['program'], '--dump-file='+program['dump'], '-Q', '--batch',
        '--eval', '(setq native-comp-async-jobs-number 1)', '-f', 'batch-byte-compile', *generated],
        fixture/'byte-compile', environment, 90, cwd=ROOT)
    assert compiled['complete'] and compiled['exit_code'] == 0, compiled
    libraries.seal(repo)
    package = repo/'dist/product/lisp'
    package.mkdir(parents=True)
    shutil.copy2(ROOT/'lisp/dframe.el', package/'dframe.el')
    libraries.install(package, repo)
    assert libraries.read(package/'dframe.el')[1]['sha256'] == prepared[0][0]['generated_sha256']
    probe_out = fixture/'real-contract'
    process = e.run_process([sys.executable, ROOT/'tools/runtime-frame-buffer-probe.py', '--products', products,
        binding_arg, selection, '--contract', '--dframe', package/'dframe.elc',
        '--profile', 'installed-reference', '--profile', 'cocoa-native-normal', '--out', probe_out],
        fixture/'contract-process', environment, 90, cwd=ROOT)
    result = e.bounded_json(probe_out/'results.json')
    assert process['complete'] and process['exit_code'] == 0 and result['passed'], result
    assert result['generated_override_used'] and all(row['observation']['correct_behavior'] for row in result['cases']), result
    rows.append(dict(id='compiled-component-package-and-real-frame-callbacks', passed=True,
        seconds=time.monotonic()-started, actual_editor_execution=True, real_editor_processes=3,
        contract_cases=4, product_override_used=True, original_products_unchanged=result['selected_inputs_unchanged']))
    started = time.monotonic()
    product = next(p for p in registry['products'] if p['id'] == 'installed-reference')
    before = e.product_identity(product, resources=True)
    trace = fixture/'real-child-result.jsonl'
    environment.update(suite.case_environment('erc-result-functional', fixture))
    environment.update(STABLE_SUITE_MACHINE_ERROR_DELIVERY='erc-original-terminal-errors-v1',
        STABLE_SUITE_FILE_ID='test/lisp/erc/erc-scenarios-status-sidebar.el',
        STABLE_SUITE_PRODUCT_PROGRAM=product['program'], STABLE_SUITE_PRODUCT_DUMP='--dump-file='+product['dump'],
        STABLE_SUITE_DIAGNOSTIC=str(trace), ERC_TESTS_SUBPROCESS='1', TERM='xterm-256color')
    process = e.run_process(['/usr/bin/script','-e','-q',str(fixture/'child.terminal.txt'),
        product['program'],'--dump-file='+product['dump'],'-nw','-Q',
        '-l',ROOT/'tools/fixtures/stable-suite-runner.el','-f','stable-suite--erc-child-setup',
        '-l',ROOT/'tools/fixtures/stable-terminal-result-functional.el',
        '-f','fe-original-terminal-result-functional'], fixture/'real-child-process',environment,30,cwd=ROOT)
    if not (process['complete'] and process['exit_code'] == 0):
        e.write_json(fixture.parent/'failed-child.json',dict(process=process,
            **{name:(fixture/'real-child-process'/(name+'.txt')).read_bytes()[:32768].decode('utf-8','replace')
               for name in ('stdout','stderr')}))
        raise AssertionError('real terminal result transport failed; failed-child.json retained')
    observed = [v for v in (json.loads(raw) for raw in trace.read_text().split('\n') if raw)
                if v.get('phase') == 'original-terminal-child-test-ended']
    assert len(observed) == 1
    event = observed[0]
    assert event['child_test_result'] == dict(program=product['program'],dump='--dump-file='+product['dump'],
        test_id='fe-original-terminal-result-sentinel',status='failed',condition='(error "owned terminal result sentinel")')
    assert before == e.product_identity(product, resources=True)
    rows.append(dict(id='real-original-terminal-ert-result-transport',passed=True,
        seconds=time.monotonic()-started,actual_editor_execution=True,real_editor_processes=1,
        original_ert_outcome_preserved=True,original_corpus_execution=False))


def retirement_integration(fixture, rows):
    """An upstream replacement must not leave a generated shadow behind."""
    started = time.monotonic()
    repo = fixture/'retirement-candidate'
    (repo/'tools/fixtures').mkdir(parents=True)
    shutil.copy2(ROOT/libraries.RECIPE, repo/libraries.RECIPE)
    _, prepared = libraries.recipe(ROOT)
    for name in prepared[0][0]['source_inputs']:
        target = repo/name
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(ROOT/name, target)
    generated = libraries.generate(repo)
    generated[0].with_suffix('.elc').write_bytes(b'synthetic bytecode for retirement accounting\n')
    libraries.seal(repo)
    corrected = generated[0].read_bytes()
    # This simulates adoption of already behavior-verified corrected upstream
    # bytes; it is not a claim that a new official revision has been published.
    (repo/'lisp/dframe.el').write_bytes(corrected)
    libraries.write_json(repo/libraries.RECIPE,
        dict(schema='femacs-library-corrections-v1', corrections=[]))
    assert libraries.generate(repo) == []
    assert not generated[0].exists() and not generated[0].with_suffix('.elc').exists()
    libraries.seal(repo)
    product = repo/'dist/new-product/lisp'
    product.mkdir(parents=True)
    shutil.copy2(repo/'lisp/dframe.el', product/'dframe.el')
    value = libraries.install(product, repo)
    assert value['corrections'] == [] and (product/'dframe.el').read_bytes() == corrected
    assert not (product/'dframe.elc').exists()
    rows.append(dict(id='upstream-adoption-retires-generated-shadow-and-old-bytecode', passed=True,
        seconds=time.monotonic()-started, actual_editor_execution=False,
        upstream_adoption_is_simulated=True, original_inputs_modified=False))


def functional_prerequisite(path):
    """Bind an explicit integration result, independent of any cleanup layout."""
    result, identity = libraries.document(path)
    expected = {'reference-lifecycle-'+name for name in (
        'active-reference-only-defect','resolved-upstream-fixed',
        'candidate-regression-after-upstream-fix','candidate-correct-behavior-required',
        'reference-changed-revalidation-required','correct-behavior-control-required')}
    expected.update(('upstream-adoption-retires-generated-shadow-and-old-bytecode',
        'compiled-component-package-and-real-frame-callbacks',
        'real-original-terminal-ert-result-transport'))
    libraries.require(result.get('schema') == 'femacs-reference-library-contract-integration-v1'
        and result.get('passed') is True and result.get('owned_fixture_retired') is True
        and len(result.get('cases', [])) == 9
        and {row['id'] for row in result['cases']} == expected
        and all(row.get('passed') is True for row in result['cases'])
        and sum(row.get('real_editor_processes', 0) for row in result['cases']) == 4,
        'key protection requires the exact completed nine-group integration closure')
    return dict(identity=identity, integration_result_sha256=identity['sha256'])


def protection(fixture, rows):
    """Protect the evidence boundaries, rather than repeating five workflows."""
    def reject(name, operation):
        started = time.monotonic()
        try:
            operation()
        except (ValueError, OSError):
            pass
        else:
            raise AssertionError('protection did not reject '+name)
        rows.append(dict(id=name, passed=True, seconds=time.monotonic()-started))

    def active(context, root, report, file_row, args, result):
        rule = e.bounded_json(args.out/'classification.json')['rules'][0]
        receipt = e.bounded_json(root/'stable-suite-completion.json')
        case = report['cases'][0]
        entry = context.entries[0]
        events = {label:accounting._original_case_event(root,suite,case,label)
                  for label in ('reference','candidate')}

        def attempt_trace(name, change=None, *, missing=False, bad_hash=False, duplicate=False, symlink=False):
            row = copy.deepcopy(file_row)
            trace = root/(name+'.jsonl')
            source = root/row['reference']['diagnostic_trace']['path']
            event = json.loads(source.read_text())
            if change is not None:
                change(event)
            if symlink:
                trace.symlink_to(source)
                pin = accounting.bound_file(source)
            else:
                raw = (json.dumps(event)+'\n').encode()
                trace.write_bytes(raw*(2 if duplicate else 1))
                pin = accounting.bound_file(trace)
            if bad_hash:
                pin['sha256'] = '0'*64
            if missing:
                row['reference'].pop('diagnostic_trace')
            else:
                row['reference']['diagnostic_trace'] = dict(path=trace.name, **pin)
            state = accounting.reference_defect_state(rule,case,row,receipt,events,root,suite)
            libraries.require(state['state'] == 'active-reference-only-defect',
                              'missing or false original child evidence stays blocking')

        reject('parent-exit-error-alone-cannot-prove-reference-defect',
               lambda:attempt_trace('missing-child',missing=True))
        reject('another-child-image-cannot-supply-defect-evidence',
               lambda:attempt_trace('wrong-image',lambda v:v['child_test_result'].update(program='another-program')))
        reject('another-child-dump-cannot-supply-defect-evidence',
               lambda:attempt_trace('wrong-dump',lambda v:v['child_test_result'].update(dump='--dump-file=another-dump')))
        reject('different-child-condition-is-unexplained',
               lambda:attempt_trace('wrong-condition',lambda v:v['child_test_result'].update(condition='(error "different defect")')))
        reject('changed-child-trace-hash-is-rejected',lambda:attempt_trace('changed-trace',bad_hash=True))
        reject('duplicate-original-child-results-are-rejected',lambda:attempt_trace('duplicate-trace',duplicate=True))
        reject('symlink-child-evidence-is-rejected',lambda:attempt_trace('symlink-trace',symlink=True))

        original_control = rule['correct_behavior_control']
        control = e.bounded_json(original_control['path'])
        for name, change in (
            ('integration-override-is-not-built-product-acceptance',lambda v:v.update(generated_override_used=True)),
            ('old-reference-epoch-contract-is-rejected',lambda v:v.update(reference_selection_sha256='0'*64)),
            ('another-program-contract-is-rejected',lambda v:v['cases'][1]['process']['argv'].__setitem__(4,'another-program')),
            ('another-dump-contract-is-rejected',lambda v:v['cases'][1]['process']['argv'].__setitem__(5,'--dump-file=another-dump')),
            ('callback-only-with-wrong-continuation-is-rejected',
             lambda v:v['cases'][1]['observation']['rows'][0].update(continuation_correct=False))):
            value = copy.deepcopy(control)
            change(value)
            path = fixture/(name+'.json')
            e.write_json(path,value)
            pin = dict(path=str(path),identity=accounting.bound_file(path))
            reject(name,lambda pin=pin:accounting._correct_library_control(
                pin,receipt,'candidate',context.profile,context.selection,file_row))

    lifecycle_integration(fixture, [], on_active=active)
    repo = fixture/'generator-protection'
    (repo/'tools/fixtures').mkdir(parents=True)
    shutil.copy2(ROOT/libraries.RECIPE, repo/libraries.RECIPE)
    _, prepared = libraries.recipe(ROOT)
    for name in prepared[0][0]['source_inputs']:
        target = repo/name
        target.parent.mkdir(parents=True,exist_ok=True)
        shutil.copy2(ROOT/name,target)
    source = repo/'lisp/dframe.el'
    original = source.read_bytes()
    source.write_bytes(original+b'\n;; changed upstream\n')
    reject('changed-upstream-source-needs-new-review',lambda:libraries.generate(repo))
    source.write_bytes(original)
    generated = libraries.generate(repo)[0]
    generated.with_suffix('.elc').write_bytes(b'synthetic bytecode descriptor\n')
    libraries.seal(repo)
    generated.with_suffix('.elc').write_bytes(b'changed synthetic bytecode\n')
    product = repo/'dist/product/lisp'
    product.mkdir(parents=True)
    shutil.copy2(source,product/'dframe.el')
    reject('tampered-sealed-bytecode-cannot-be-installed',lambda:libraries.install(product,repo))
    orphan = generated.parent/'orphan.el'
    orphan.write_text(';; stale generated shadow\n')
    reject('unregistered-generated-shadow-cannot-survive-rebuild',lambda:libraries.generate(repo))
    resources = load('library_resource_protection', ROOT/'tools/bootstrap-resources.py')
    generated_repo = fixture/'resource-source'
    generated_repo.mkdir()
    with patch.object(resources, 'ROOT', generated_repo):
        started = time.monotonic()
        diagnostic = io.StringIO()
        with patch.object(sys, 'argv', ['bootstrap-resources.py', '--family', 'bytecode']), \
                patch.object(resources, 'Generator', side_effect=AssertionError('implicit program selection')), \
                redirect_stderr(diagnostic):
            try:
                resources.main()
            except SystemExit as error:
                assert error.code == 2 and '--program' in diagnostic.getvalue()
            else:
                raise AssertionError('missing explicit program was accepted')
        rows.append(dict(id='missing-explicit-program-rejected-before-generation', passed=True,
                         seconds=time.monotonic()-started))
        started = time.monotonic()
        generator = resources.Generator.__new__(resources.Generator)
        generator.log = fixture/'resource-environment'
        generator.log.mkdir()
        generator.sequence = 0
        with patch.dict(os.environ, {name:str(fixture/'retired-product') for name in
                ('EMACSLOADPATH','EMACSDATA','EMACSDOC')}), \
                patch.object(resources.subprocess, 'run', return_value=SimpleNamespace(returncode=0)) as command:
            generator.run(['synthetic-editor'])
        environment = command.call_args.kwargs['env']
        assert environment['EMACSLOADPATH'] == str(generated_repo/'lisp')
        assert environment['EMACSDATA'] == environment['EMACSDOC'] == str(generated_repo/'etc')
        rows.append(dict(id='resource-generation-keeps-owned-canonical-roots', passed=True,
                         seconds=time.monotonic()-started))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--products', type=Path)
    binding = parser.add_mutually_exclusive_group()
    binding.add_argument('--reference-selection', type=Path)
    binding.add_argument('--reference-inputs', type=Path)
    parser.add_argument('--group', choices=('all','lifecycle','real','retirement'), default='all')
    parser.add_argument('--mode',choices=('integration','protection'),default='integration')
    parser.add_argument('--functional',type=Path)
    parser.add_argument('--out', type=Path, required=True)
    args = parser.parse_args()
    reference_binding = args.reference_selection or args.reference_inputs
    if (args.products is None) != (reference_binding is None):
        parser.error('products and reference selection are supplied together for real integration')
    if args.group == 'real' and args.products is None:
        parser.error('real integration requires products and selected reference')
    if (args.mode == 'protection') != (args.functional is not None):
        parser.error('--functional is required exactly for key protection')
    prerequisite = functional_prerequisite(args.functional) if args.functional else None
    out = owned_test_output(args.out)
    fixture = out/'owned-fixtures'
    fixture.mkdir()
    rows, error = [], None
    started = time.monotonic()
    try:
        if args.mode == 'protection':
            protection(fixture,rows)
        elif args.group in ('all','lifecycle'):
            lifecycle_integration(fixture, rows)
        if args.mode == 'integration' and args.group in ('all','retirement'):
            retirement_integration(fixture, rows)
        if args.mode == 'integration' and args.products and args.group in ('all','real'):
            real_integration(fixture, args.products, reference_binding, rows,
                binding_arg='--reference-inputs' if args.reference_inputs else '--reference-selection')
    except Exception as exception:
        error = type(exception).__name__+': '+str(exception)[:8192]
        raise
    finally:
        shutil.rmtree(fixture)
        e.write_json(out/'results.json', dict(schema=('femacs-reference-library-contract-protection-v1'
            if args.mode == 'protection' else 'femacs-reference-library-contract-integration-v1'),
            passed=error is None, cases=rows, error=error, seconds=time.monotonic()-started,
            functional_prerequisite=prerequisite,
            owned_fixture_retired=True, original_corpus_execution=False,
            full_product_build_acceptance=False, compatibility_accepted=False, goal_complete=False))


if __name__ == '__main__':
    main()
