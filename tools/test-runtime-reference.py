"""File/process integration for explicit reference epochs; no editor is executed.

Synthetic candidate admission is isolated. Actual whole-product admission is a
separate installed-reference verification, never inferred from this fixture.
"""
import argparse
import copy
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import time
from types import SimpleNamespace
from unittest import mock

REPO = Path(__file__).resolve().parent.parent


def module(name, file):
    spec = importlib.util.spec_from_file_location(name, REPO / 'tools' / file)
    value = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(value)
    return value


stage = module('reference_phase_stage', 'runtime-stage.py')
phases = module('reference_phase_children', 'test-runtime-phases.py')
e = stage.execution


def _file(path, text):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text)
    return path


def _fixture(root):
    original = root / 'original'; source = original / 'source'
    _file(source / 'tools/stable-suite.py', phases.STABLE_CHILD)
    _file(source / 'tools/fixtures/stable-suite-runner.el', ';; frozen runner fixture\n')
    test = _file(source / 'test/lisp/control.el', ';; synthetic original entry\n')
    manifest = dict(schema=1, source=dict(release='31.1', commit='a360712c9d272d950d8d8255ef74570f7e90b7d9'),
        file_count=1, lisp_file_count=1, total_bytes=test.stat().st_size,
        entries=[dict(path='test/lisp/control.el', source_path='test/lisp/control.el',
                      input='tracked', mode='100644', size=test.stat().st_size,
                      sha256=e.digest(test), blob=hashlib.sha1(
                          b'blob ' + str(test.stat().st_size).encode() + b'\0' + test.read_bytes()).hexdigest())])
    target = source / 'compatibility/stable-31.1/sources.json'
    target.parent.mkdir(parents=True)
    e.write_json(target, manifest)
    names = sorted(p.relative_to(source).as_posix() for p in source.rglob('*') if p.is_file())
    e.write_json(original / 'source-inputs.json', dict(schema=1,
        files={name: e.file_identity(source / name) for name in names}))
    e.write_json(source / '.femacs-runtime-sources.json', dict(schema=1, files=names))
    exported = dict(passed=True, source=str(source), file_count=len(names),
                    source_manifest_sha256=e.digest(original / 'source-inputs.json'), tests_executed=False)
    old = _file(root / 'retired-reference', 'historical program\n')
    old_dump = _file(root / 'retired-dump', 'historical dump\n')
    program = _file(root / 'product/bin/reference-program', '#!' + sys.executable + '\n'
        'import sys\nprint("SYNTHETIC-REFERENCE-PROBE", sys.argv[1:])\n')
    program.chmod(0o700)
    dump = _file(root / 'reference-dump', 'selected dump fixture\n')
    lisp = _file(root / 'product/lisp/control.el', ';; synthetic library\n').parent
    data = _file(root / 'product/etc/control', 'synthetic data\n').parent
    reference = dict(stage.reference_selection.INVARIANTS, native=True, program=str(program),
                     dump=str(dump), resource_roots=dict(lisp=str(lisp), data=str(data)))
    historical = dict(reference, program=str(old), dump=str(old_dump))
    old_identity = e.product_identity(historical, resources=True)
    candidate_program = _file(root / 'product/bin/candidate-program', 'compiled candidate substitute\n')
    candidate_dump = _file(root / 'candidate-dump', 'candidate dump substitute\n')
    candidate_receipt = _file(root / 'candidate-receipt.json', '{"synthetic":true}\n')
    candidates = [dict(id='candidate-' + str(i).zfill(2), kind='terminal', compiler_leg='normal',
        native=False, observer=False, test_hooks=False, collector='off', program=str(candidate_program),
        dump=str(candidate_dump), resource_roots=dict(lisp=str(lisp), data=str(data)),
        product_receipt=str(candidate_receipt)) for i in range(60)]
    products = [historical, *candidates]
    document = dict(schema=1, products=products, source_cohorts=[exported])
    e.write_json(original / 'products.json', document)
    build = dict(action='build', passed=True, source_export=exported, source_cohorts=[exported],
        products=products, reference_identity=old_identity, source_root=str(source),
        product_list=str(original / 'products.json'))
    e.write_json(original / 'results.json', build)
    old.unlink(); old_dump.unlink()
    selected_root = root / 'selection'; selected_root.mkdir()
    patch = _file(selected_root / 'patches/control.patch', 'synthetic patch bytes\n')
    revision = '3'*40
    recipe = dict(formulae=[dict(name='reference@32', aliases=['reference@master'],
        full_name='fixture/tap/reference@32', tap='fixture/tap',
        versions=dict(stable='32.0.50'), urls=dict(stable=dict(revision=revision)),
        ruby_source_checksum=dict(sha256='4'*64),
        patches=[dict(url='synthetic-patch-origin', strip='p1', sha256=e.digest(patch))])])
    install = root / 'keg/INSTALL_RECEIPT.json'; install.parent.mkdir()
    e.write_json(install, dict(poured_from_bottle=False, source=dict(tap='fixture/tap',
        scm_revision=revision, path='Aliases/reference@master', versions=dict(stable='32.0.50'))))
    config = _file(root / 'build.yml', 'icon: retained-synthetic-icon\n')
    process = e.run_process([program, '--dump-file=' + str(dump), '--batch'],
                             root / 'installation-probe', dict(os.environ), 5, cwd=root)
    assert process['complete'] and process['exit_code'] == 0
    live = e.product_identity(reference, resources=True)
    facts = dict(emacs_version='32.0.50', emacs_repository_version=revision,
        system_configuration='synthetic', system_configuration_features='synthetic features',
        lisp_directory=str(lisp), data_directory=str(data),
        capabilities={name: True for name in stage.reference_selection.CAPABILITIES})
    binding = lambda p: dict(path=str(p), identity=e.file_identity(p))
    probe = dict(schema='femacs-reference-install-probe-v1', passed=True,
        repository_version_embedded=True,source_revision_provenance='pinned-recipe-and-install-receipt',
        install_receipt=binding(install), build_config=binding(config),
        patch_files={'synthetic-patch-origin': binding(patch)}, process=process,
        product_identity_before=live, product_identity_after=live, sampled_facts=facts)
    tree = dict(schema='femacs-upstream-data-git-tree-comparison-v1', upstream_commit=revision,
        candidate_compiled_source='5'*40, candidate_raw_entries=1, upstream_raw_entries=1,
        candidate_changed_or_extra=[], upstream_entries_not_tracked_by_candidate=[],
        git_metadata_only=True, installed_product_files_verified=False)
    evidence = {}
    for key, name, value in [('recipe_info', 'recipe-info.json', recipe),
                              ('reference_probe', 'reference-probe.json', probe),
                              ('upstream_tree_comparison', 'upstream-tree-comparison.json', tree)]:
        path = selected_root / name; e.write_json(path, value); evidence[key] = binding(path)
    selected_document = dict(document, products=[reference, *candidates])
    e.write_json(selected_root / 'products.json', selected_document)
    upstream = dict(version='32.0.50', commit=revision, formula='fixture/tap/reference@32',
        tap='fixture/tap', formula_sha256='4'*64, patches=recipe['formulae'][0]['patches'])
    selection = dict(schema=stage.reference_selection.SCHEMA, scope=stage.reference_selection.SCOPE,
        epoch='selected-synthetic', build_root=str(original), build_result_sha256=e.digest(original / 'results.json'),
        source_manifest_sha256=exported['source_manifest_sha256'], original_products_path=str(original / 'products.json'),
        original_products_sha256=e.digest(original / 'products.json'),
        selected_products_path=str(selected_root / 'products.json'),
        selected_products_sha256=e.digest(selected_root / 'products.json'), evidence_inputs=evidence,
        profile_count=61, candidate_count=60, profile_order=[p['id'] for p in products],
        original_reference=dict(product=historical, identity=old_identity, historical_epoch='retired-synthetic',
                                historical_observations_retained=True),
        selected_reference=dict(product=reference, identity=live, upstream=upstream,
            capabilities=facts['capabilities'], upstream_tree_comparison=tree),
        **{key: False for key in ('candidate_and_reference_libraries_byte_identical',
            'existing_components_reverted', 'new_waivers_added', 'reference_semantics_accepted',
            'compatibility_accepted', 'performance_accepted', 'goal_complete')})
    e.write_json(selected_root / 'reference-selection.json', selection)
    selected_runner = _file(root / 'selected-runner.el', ';; explicitly selected runner fixture\n')
    return dict(original=original, source=source, selection=selected_root / 'reference-selection.json',
                products=selected_document['products'], selected_runner=selected_runner,
                historical_document=document)


def integration(out):
    started = time.monotonic(); fixture = out / 'owned-fixtures'; fixture.mkdir()
    rows = []; retained = None; result = None
    try:
        context = _fixture(fixture)
        before = {p['id']: e.product_identity(p, resources=True) for p in context['products']}
        original_digest = e.digest(context['original'] / 'products.json')
        # Only compiled candidate receipt admission is isolated; file/source/proof
        # reads, child dispatch, completion validation and owner lifecycle are real.
        def admitted(path):
            value = e.bounded_json(path)
            assert value['products'] == context['products']
            return value['products']
        with mock.patch.object(e, 'load_products', side_effect=admitted):
            start = time.monotonic()
            options = SimpleNamespace(build=context['original'], action='execute', reference_selection=context['selection'],
                execution_products=None, execution_phase='stable', stable_profile=['candidate-00'],
                stable_runner=context['selected_runner'], timeout=5)
            original, source, selected_path, products, build = stage.read_action_build(options)
            assert products == context['products'] and original == context['original']
            assert source == context['source'] and selected_path == context['selection'].parent / 'products.json'
            assert products[1:] == context['historical_document']['products'][1:]
            rows.append(dict(id='absent-historical-reference-admission', passed=True,
                             seconds=time.monotonic() - start))
            for explicit in (False, True):
                start = time.monotonic(); target = out / ('selected-runner' if explicit else 'default-runner')
                target.mkdir(); options.stable_runner = context['selected_runner'] if explicit else None
                with mock.patch.dict(os.environ, PHASE_CONTRACT_DRIVER=str(REPO / 'tools/runtime-execution.py')):
                    with stage.owner.ArtifactRun(REPO, target) as artifacts:
                        with mock.patch.object(stage.owner, 'capture_product_files') as capture:
                            stage.select_execution_inputs(artifacts, original, source, selected_path, products, build, options)
                            assert capture.call_count == 1
                        assert context['selection'].stat().st_flags & 2 if sys.platform == 'darwin' else True
                        result = stage.execute_matrix(options, target, artifacts)
                        assert result['passed'] and result['execution_complete']
                        assert result['reference_selection']['epoch'] == 'selected-synthetic'
                        assert result['requested_stable_profiles'] == ['candidate-00']
                        assert len(result['remaining_stable_profiles']) == 59
                        assert result['compatibility_accepted'] is False and result['goal_complete'] is False
                        retained = artifacts.retain_current_files()
                        assert retained['all_original_files_verified']
                if sys.platform == 'darwin': assert not context['selection'].stat().st_flags & 2
                rows.append(dict(id='selected-reference-' + ('explicit' if explicit else 'default') + '-runner',
                    passed=True, actual_python_children=1, seconds=time.monotonic() - start))
            start = time.monotonic()
            stage.verify_reference_selection(options, products)
            assert original_digest == e.digest(original / 'products.json')
            assert before == {p['id']: e.product_identity(p, resources=True) for p in products}
            rows.append(dict(id='all61-and-historical-inputs-unchanged', passed=True,
                             seconds=time.monotonic() - start))
            start=time.monotonic()
            probe_path=context['selection'].parent/'reference-probe.json'
            captured=e.bounded_json(probe_path)
            captured['sampled_facts']['emacs_repository_version']=None
            captured['repository_version_embedded']=False
            e.write_json(probe_path,captured)
            selected=e.bounded_json(context['selection'])
            selected['evidence_inputs']['reference_probe']['identity']=e.file_identity(probe_path)
            e.write_json(context['selection'],selected)
            options=SimpleNamespace(build=context['original'],reference_selection=context['selection'],execution_products=None)
            stage.read_action_build(options)
            assert options._reference_selection_summary['repository_version_embedded'] is False
            assert options._reference_selection_summary['runtime_repository_version'] is None
            rows.append(dict(id='package-without-embedded-repository-variable',passed=True,
                             seconds=time.monotonic()-start))
    except Exception as error:
        diagnostics = {}
        if result is not None:
            for case in result.get('cases', []):
                directory = Path(case['process']['evidence_directory'])
                diagnostics[case['id']] = {p.name: p.read_text(errors='replace')[-4096:]
                    for p in directory.iterdir() if p.is_file() and p.suffix in ('.log', '.txt')}
        e.write_json(out / 'results.json', dict(passed=False, execution_complete=False,
            cases=rows, last_result=result, error_type=type(error).__name__, error=str(error),
            child_diagnostics=diagnostics, actual_editor_execution=False, goal_complete=False))
        raise
    finally:
        shutil.rmtree(fixture)
        for name in ('default-runner', 'selected-runner'):
            path = out / name
            if path.exists(): shutil.rmtree(path)
    report = dict(schema='femacs-reference-selection-file-process-integration-v1', passed=len(rows) == 5,
        cases=rows, installation_python_children=1, stable_python_children=2,
        artifact_owner_lifecycle_exercised=sys.platform == 'darwin',
        compiled_product_admission_isolated=True, actual_editor_execution=False,
        original_corpus_execution=False, owned_fixture_retired=not fixture.exists(),
        seconds=time.monotonic() - started, goal_complete=False)
    e.write_json(out / 'results.json', report)
    return report['passed']


def protection(out, functional):
    accepted = e.bounded_json(functional)
    assert accepted.get('passed') is True and accepted.get('owned_fixture_retired') is True
    fixture = out / 'owned-protections'; fixture.mkdir()
    started = time.monotonic(); rows = []
    try:
        context = _fixture(fixture)
        selection_path = context['selection']; directory = selection_path.parent
        baseline = json.loads(selection_path.read_text())
        products = e.bounded_json(directory / 'products.json')
        probe = e.bounded_json(directory / 'reference-probe.json')
        install_path = Path(probe['install_receipt']['path'])
        install = e.bounded_json(install_path)
        def reset():
            e.write_json(selection_path, copy.deepcopy(baseline))
            e.write_json(directory / 'products.json', copy.deepcopy(products))
            e.write_json(directory / 'reference-probe.json', copy.deepcopy(probe))
            e.write_json(install_path, copy.deepcopy(install))
        def admitted(path):
            return e.bounded_json(path)['products']
        def load():
            return stage.read_action_build(SimpleNamespace(build=context['original'],
                reference_selection=selection_path, execution_products=None))
        with mock.patch.object(e, 'load_products', side_effect=admitted):
            changes = [
                ('candidate-field', 'products', lambda d: d['products'][1].update(collector='young')),
                ('candidate-order', 'products', lambda d: d['products'].reverse()),
                ('missing-candidate', 'products', lambda d: d['products'].pop()),
                ('old-identity-relabeled', 'selection', lambda d: d['original_reference'].update(identity={})),
                ('same-epoch', 'selection', lambda d: d.update(epoch=d['original_reference']['historical_epoch'])),
                ('wrong-count', 'selection', lambda d: d.update(candidate_count=59)),
                ('false-acceptance', 'selection', lambda d: d.update(compatibility_accepted=True)),
                ('wrong-revision', 'probe', lambda d: d['sampled_facts'].update(emacs_repository_version='0'*40)),
                ('wrong-resources', 'probe', lambda d: d['sampled_facts'].update(lisp_directory='wrong-library')),
                ('wrong-native', 'probe', lambda d: d['sampled_facts']['capabilities'].update(native=False)),
                ('incomplete-process', 'probe', lambda d: d['process'].update(complete=False)),
                ('wrong-process-program', 'probe', lambda d: d['process']['argv'].__setitem__(0,'wrong-program')),
                ('wrong-install-tap', 'install', lambda d: d['source'].update(tap='wrong/tap')),
                ('wrong-install-revision', 'install', lambda d: d['source'].update(scm_revision='0'*40)),
                ('wrong-install-formula', 'install', lambda d: d['source'].update(path='Formula/unrelated.rb')),
                ('wrong-embedded-claim', 'probe', lambda d: d.update(repository_version_embedded=False)),
                ('missing-source-provenance', 'probe', lambda d: d.pop('source_revision_provenance')),
                ('unreported-runtime-revision', 'probe', lambda d: (d['sampled_facts'].pop('emacs_repository_version'),d.update(repository_version_embedded=False))),
            ]
            for name, scope, mutate in changes:
                reset(); start = time.monotonic()
                chosen = copy.deepcopy({'products':products,'probe':probe,'install':install,'selection':baseline}[scope])
                mutate(chosen)
                path = {'products':directory/'products.json','probe':directory/'reference-probe.json',
                        'install':install_path,'selection':selection_path}[scope]
                e.write_json(path,chosen)
                selected = e.bounded_json(selection_path)
                if scope == 'products': selected['selected_products_sha256'] = e.digest(path)
                elif scope == 'install':
                    current = copy.deepcopy(probe)
                    current['install_receipt']['identity'] = e.file_identity(path)
                    e.write_json(directory/'reference-probe.json',current)
                    selected['evidence_inputs']['reference_probe']['identity'] = e.file_identity(directory/'reference-probe.json')
                elif scope == 'probe':selected['evidence_inputs']['reference_probe']['identity'] = e.file_identity(path)
                if scope != 'selection':e.write_json(selection_path,selected)
                try:load()
                except (ValueError, OSError):pass
                else:raise AssertionError('invalid reference evidence accepted: '+name)
                rows.append(dict(id=name,passed=True,seconds=time.monotonic()-start))
            reset(); start=time.monotonic()
            options=SimpleNamespace(build=context['original'],reference_selection=selection_path,execution_products=None)
            _,_,_,current,_=stage.read_action_build(options)
            with (directory/'patches/control.patch').open('a') as f:f.write('changed')
            try:stage.verify_reference_selection(options,current)
            except ValueError:pass
            else:raise AssertionError('postflight patch mutation accepted')
            rows.append(dict(id='postflight-patch-mutation',passed=True,seconds=time.monotonic()-start))
            _file(directory/'patches/control.patch','synthetic patch bytes\n')
            reset();start=time.monotonic()
            options=SimpleNamespace(build=context['original'],reference_selection=selection_path,execution_products=None)
            _,_,_,current,_=stage.read_action_build(options)
            config=Path(probe['build_config']['path']);config.write_text('changed config\n')
            try:stage.verify_reference_selection(options,current)
            except ValueError:pass
            else:raise AssertionError('postflight config mutation accepted')
            rows.append(dict(id='postflight-config-mutation',passed=True,seconds=time.monotonic()-start))
        for name, args in [
            ('build-scope',['--action','build']),('followup-scope',['--action','followup']),
            ('mutually-exclusive',['--action','execute','--build',context['original'],
                                  '--execution-products',directory/'products.json'])]:
            start=time.monotonic();unused=fixture/name
            child=subprocess.run([sys.executable,REPO/'tools/runtime-stage.py',*args,
                '--reference-selection',selection_path,'--out',unused],capture_output=True,timeout=5)
            assert child.returncode==2 and not unused.exists()
            rows.append(dict(id=name,passed=True,seconds=time.monotonic()-start))
    finally:shutil.rmtree(fixture)
    report=dict(schema='femacs-reference-selection-protection-v1',passed=len(rows)==23,cases=rows,
        functional_acceptance_sha256=e.digest(functional),owned_fixture_retired=not fixture.exists(),
        actual_editor_execution=False,seconds=time.monotonic()-started,goal_complete=False)
    e.write_json(out/'results.json',report)
    return report['passed']


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out', type=Path, required=True)
    parser.add_argument('--mode', choices=('integration','protection'),default='integration')
    parser.add_argument('--functional-acceptance',type=Path)
    args = parser.parse_args()
    if args.mode=='protection' and args.functional_acceptance is None:
        parser.error('protection requires the accepted functional receipt')
    out=e.new_output(args.out)
    passed=(integration(out) if args.mode=='integration' else protection(out,args.functional_acceptance))
    return 0 if passed else 1


if __name__ == '__main__':
    raise SystemExit(main())
