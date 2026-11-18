"""Exercise phase dispatch through real Python children and strict receipt readers.

The owned source/product descriptors are synthetic; this is coordinator file/process
integration, not editor, compiler, corpus or performance acceptance.
"""
import argparse
from contextlib import nullcontext
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
spec = importlib.util.spec_from_file_location('phase_stage', REPO/'tools/runtime-stage.py')
stage = importlib.util.module_from_spec(spec)
spec.loader.exec_module(stage)

CHILD = '''import argparse, importlib.util, json, os
from pathlib import Path
p=argparse.ArgumentParser()
p.add_argument('--out',type=Path,required=True)
p.add_argument('--action')
p.add_argument('--products',type=Path)
p.add_argument('--permanent-full-fallback',action='store_true')
a,rest=p.parse_known_args()
s=importlib.util.spec_from_file_location('phase_receipts',os.environ['PHASE_CONTRACT_DRIVER'])
e=importlib.util.module_from_spec(s);s.loader.exec_module(e)
a.out.mkdir(parents=True)
if a.action:
 fallback=a.permanent_full_fallback and os.environ.get('PHASE_BAD_FALLBACK_MODE')!='1'
 e.write_json(a.out/'inputs.json',dict(action=a.action,product_list_sha256=e.digest(a.products),
  permanent_full_fallback=fallback))
 report=dict(schema='femacs-runtime-execution-v1',action=a.action,passed=True,
  cases=[dict(id='synthetic-phase-dispatch',passed=True)],
  full_product_inputs_unchanged=True,fixtures_unchanged=True)
 if a.action=='ert':
  spec=importlib.util.spec_from_file_location('captured_language_contract',Path(__file__).with_name('runtime-language-contract.py'))
  contract=importlib.util.module_from_spec(spec);spec.loader.exec_module(contract)
  observed=json.loads(Path(os.environ['PHASE_LANGUAGE_OBSERVATION']).read_text())
  comparison=contract.compare(observed['id'],observed['reference'],observed['candidate'])
  report.update(passed=comparison['passed'],language_observation_comparisons=[comparison],raw_observations=observed)
  report['cases'][0]['passed']=comparison['passed']
 if a.action=='performance':report.update(permanent_full_fallback=fallback,startup_measured=not fallback)
 e.write_json(a.out/'state-retirement.json',dict(complete=True))
 e.write_json(a.out/'results.json',report)
 e.write_completion(a.out,report)
else:
 # A successful child without a valid corpus receipt must never pass the stage.
 e.write_json(a.out/'stable-suite-run.json',dict(synthetic=True))
 e.write_json(a.out/'stable-suite-completion.json',dict(corpus_complete=True))
'''

STABLE_CHILD = '''import argparse, importlib.util, json, os
from pathlib import Path
p=argparse.ArgumentParser()
for name in ('out','repo','manifest','reference','candidate','reference-dump-args',
             'candidate-dump-args','reference-env','candidate-env'):
 p.add_argument('--'+name,required=True)
p.add_argument('--runner',required=True)
a,rest=p.parse_known_args()
s=importlib.util.spec_from_file_location('scope_execution',os.environ['PHASE_CONTRACT_DRIVER'])
e=importlib.util.module_from_spec(s);s.loader.exec_module(e)
suite=e.load_tool('stable-suite.py')
manifest=suite.load_manifest(a.manifest,a.repo)
entries=suite._load_batch().select_entrypoints(manifest)
environments={label:json.loads(getattr(a,label+'_env')) for label in ('reference','candidate')}
products={label:suite._public_product_identity(suite._product_paths(getattr(a,label),
 json.loads(getattr(a,label+'_dump_args')),environments[label])) for label in environments}
out=Path(a.out);out.mkdir(parents=True)
runner=Path(a.runner)
assert runner.is_file() and not runner.is_symlink()
sources={str(Path(__file__)):e.digest(Path(__file__)),str(runner):e.digest(runner)}
if os.environ.get('SCOPE_OUTCOME')=='wrong-runner':
 sources.pop(str(runner));old=Path(a.repo)/'tools/fixtures/stable-suite-runner.el'
 sources[str(old)]=e.digest(old)
elif os.environ.get('SCOPE_OUTCOME')=='missing-runner':sources.pop(str(runner))
elif os.environ.get('SCOPE_OUTCOME')=='wrong-runner-sha':sources[str(runner)]='0'*64
failed=os.environ.get('SCOPE_OUTCOME')=='failed'
report=dict(synthetic_file_process_contract=True,selector='all-discovered-ids',
 source={key:manifest['source'][key] for key in ('release','commit')},
 manifest_sha256=e.digest(Path(a.manifest)),entrypoint_count=len(entries),
 resource_only_lisp_inputs=manifest['lisp_file_count']-len(entries),
 files=[dict(file=entry['path'],reference=dict(complete=True),candidate=dict(complete=True))
        for entry in entries],cases=[dict(passed=not failed) for entry in entries],
 failures=['synthetic original outcome retained'] if failed else [],corpus_complete=not failed,
 products_before=products,products_after=products,identities_unchanged=True,
 corpus_inputs_unchanged=True,runtime_environment=environments,
 harness_sources=sources)
suite._write_run_report(out,report)
if os.environ.get('SCOPE_OUTCOME')=='tampered':
 with (out/'stable-suite-run.json').open('a') as f:f.write('changed')
raise SystemExit(1 if failed else 0)
'''


def stable_integration(out):
    """Real dispatch/writer/reader integration; editor and build admission are isolated."""
    fixture=out/'owned-stable-scope-fixtures'
    fixture.mkdir()
    started=time.monotonic()
    rows=[]
    try:
        source=fixture/'source'
        (source/'tools').mkdir(parents=True)
        (source/'tools/stable-suite.py').write_text(STABLE_CHILD)
        default_runner=source/'tools/fixtures/stable-suite-runner.el'
        default_runner.parent.mkdir(parents=True)
        default_runner.write_text(';; synthetic frozen runner input\n')
        selected_runner=fixture/'selected-runner.el'
        selected_runner.write_text(';; synthetic repaired runner input\n')
        entries=[]
        for name in ('first','second'):
            path=source/'test/lisp'/(name+'.el');path.parent.mkdir(parents=True,exist_ok=True)
            data=b';; synthetic corpus descriptor; never loaded by an editor\n'
            path.write_bytes(data)
            entries.append(dict(path=path.relative_to(source).as_posix(),
                source_path=path.relative_to(source).as_posix(),input='tracked',mode='100644',
                size=len(data),sha256=stage.execution.digest(path),
                blob=__import__('hashlib').sha1(b'blob '+str(len(data)).encode()+b'\0'+data).hexdigest()))
        manifest=source/'compatibility/stable-31.1/sources.json'
        manifest.parent.mkdir(parents=True)
        stage.execution.write_json(manifest,dict(schema=1,
            source=dict(release='31.1',commit='a360712c9d272d950d8d8255ef74570f7e90b7d9'),
            file_count=2,lisp_file_count=2,total_bytes=sum(e['size'] for e in entries),entries=entries))
        program=fixture/'product/bin/program';program.parent.mkdir(parents=True)
        program.write_bytes(b'synthetic product; never executed\n')
        dump=program.with_name('dump');dump.write_bytes(b'synthetic dump; never loaded\n')
        library=fixture/'product/lisp';library.mkdir();(library/'control.el').write_bytes(b'control\n')
        products=[dict(id=name,kind=kind,test_hooks=hooks,collector='off',program=str(program),
                       dump=str(dump),resource_roots={}) for name,kind,hooks in
                  [('reference','reference',False),('first','terminal',False),
                   ('second','cocoa',False),('diagnostic','cocoa',True)]]
        product_path=fixture/'products.json';stage.execution.write_json(product_path,dict(products=products))
        build=(fixture,source,product_path,products,dict(passed=True))
        before={p['id']:stage.execution.product_identity(p,resources=True) for p in products}
        for name,selection,outcome,expected,passed,complete,explicit_runner in [
            ('selected',['second'],'passed',['second'],True,True,False),
            ('all-selected',['second','first'],'passed',['first','second'],True,True,False),
            ('tampered',['second'],'tampered',['second'],False,False,False),
            ('failed-outcome',['second'],'failed',['second'],False,True,False),
            ('explicit-runner',['second'],'passed',['second'],True,True,True),
            ('wrong-runner',['second'],'wrong-runner',['second'],False,False,True),
            ('missing-runner',['second'],'missing-runner',['second'],False,False,True),
            ('wrong-runner-sha',['second'],'wrong-runner-sha',['second'],False,False,True)]:
            root=fixture/name;root.mkdir();case_started=time.monotonic()
            env=dict(os.environ,PHASE_CONTRACT_DRIVER=str(REPO/'tools/runtime-execution.py'),SCOPE_OUTCOME=outcome)
            with mock.patch.object(stage,'read_action_build',return_value=build), \
                 mock.patch.object(stage.execution,'load_products',return_value=products), \
                 mock.patch.object(stage,'clean_environment',return_value=env):
                protected=explicit_runner and name=='explicit-runner' and sys.platform=='darwin'
                context=stage.owner.ArtifactRun(REPO,root) if protected else nullcontext(None)
                with context as artifacts:
                    result=stage.execute_matrix(SimpleNamespace(action='execute',execution_phase='stable',
                        stable_profile=selection,timeout=5,
                        stable_runner=selected_runner if explicit_runner else None),root,artifacts)
                    if protected:
                        captured=root/'stable-runner/stable-suite-runner.el'
                        assert captured.stat().st_flags & 2
                        assert (root/'stable-runner/stable-runner-selection.json').stat().st_flags & 2
                        assert not selected_runner.stat().st_flags & 2
                        retained=artifacts.retain_current_files()
                        assert retained['all_original_files_verified']
                if protected:
                    assert not (root/'stable-runner/stable-suite-runner.el').stat().st_flags & 2
                    assert not (root/'stable-runner/stable-runner-selection.json').stat().st_flags & 2
            assert [row['id'] for row in result['cases']]==['stable-'+identity for identity in expected]
            assert result['requested_stable_profiles']==expected and result['available_stable_pairs']==2
            assert result['remaining_stable_profiles']==[x for x in ('first','second') if x not in expected]
            assert result['passed'] is passed and result['execution_complete'] is complete
            assert result['stable_phase_complete'] is (complete and len(expected)==2)
            assert ('stable' in result['remaining_execution_phases']) is (len(expected)!=2)
            assert not result['full_matrix_execution_complete'] and not result['goal_complete']
            assert result['product_identity_unchanged'] and result['complete_receipts_valid']
            identities=stage.execution.bounded_json(root/'product-identities.json')
            assert set(identities['before'])==set(before)==set(identities['after'])
            for row in result['cases']:
                process=row['process']
                assert process['complete'] and not process['remaining_group_terminated']
                assert process['exit_code']==(1 if outcome=='failed' else 0)
                assert process['argv'][process['argv'].index('--manifest')+1]==str(manifest)
                assert not any('filter' in arg or 'selector' in arg for arg in process['argv'])
                actual_runner=root/'stable-runner/stable-suite-runner.el' if explicit_runner else default_runner
                assert process['argv'][process['argv'].index('--runner')+1]==str(actual_runner)
                receipt=stage.execution.bounded_json(Path(row['receipt']).parent/'stable-suite-completion.json')
                assert 'runner_identity' not in receipt
            if explicit_runner:
                binding=result['stable_runner_selection']
                assert binding['unchanged'] and binding['preserves_compiled_product_fields']
                assert binding['source_runner']['path']==str(selected_runner)
                captured=Path(binding['captured_runner']['path'])
                assert captured==root/'stable-runner/stable-suite-runner.el'
                assert captured.read_bytes()==selected_runner.read_bytes()
                assert stage.execution.file_identity(captured)==stage.execution.file_identity(selected_runner)
                assert (root/'stable-runner/stable-runner-selection.json').is_file()
            else:assert result['stable_runner_selection'] is None
            if outcome in ('wrong-runner','missing-runner','wrong-runner-sha'):
                assert 'receipt_validation_error' in result['cases'][0]
            if outcome=='tampered':assert 'receipt_validation_error' in result['cases'][0]
            if outcome=='failed':assert result['cases'][0]['failure_count']==1
            rows.append(dict(id=name,passed=True,actual_python_children=len(expected),
                             seconds=time.monotonic()-case_started))
        assert before=={p['id']:stage.execution.product_identity(p,resources=True) for p in products}
    finally:
        shutil.rmtree(fixture)
    report=dict(schema='femacs-stable-scope-file-process-integration-v1',passed=len(rows)==8,
        cases=rows,actual_python_children=sum(row['actual_python_children'] for row in rows),
        real_dispatch_writer_reader_and_all_product_identity_boundary=True,
        runner_capture_owner_lifecycle_exercised=sys.platform=='darwin',
        compiled_product_admission_isolated=True,actual_editor_execution=False,
        original_corpus_execution=False,owned_fixture_retired=not fixture.exists(),
        seconds=time.monotonic()-started)
    stage.execution.write_json(out/'results.json',report)
    return report['passed']



def runner_protection(out, functional=None):
    fixture=out/'owned-runner-protections';fixture.mkdir()
    rows=[];started=time.monotonic()
    try:
        if functional is not None:
            accepted=stage.execution.bounded_json(functional)
            if not accepted.get('passed') or not accepted.get('owned_fixture_retired'):
                raise AssertionError('supplied functional acceptance is not complete')
        valid=fixture/'runner.el';valid.write_text(';; valid selected runner control\n')
        empty=fixture/'empty.el';empty.write_bytes(b'')
        large=fixture/'large.el';large.write_bytes(b'x'*(1024*1024+1))
        wrong=fixture/'runner.txt';wrong.write_text('control\n')
        directory=fixture/'directory.el';directory.mkdir()
        link=fixture/'linked.el';link.symlink_to(valid)
        parent=fixture/'linked-parent';parent.symlink_to(fixture,target_is_directory=True)
        for name,args in [
            ('build-scope',['--action','build']),
            ('prepare-scope',['--action','prepare']),
            ('components-scope',['--action','components']),
            ('followup-scope',['--action','followup']),
            ('semantics-scope',['--action','execute','--execution-phase','semantics']),
            ('performance-scope',['--action','execute','--execution-phase','performance'])]:
            start=time.monotonic();unused=fixture/name
            child=subprocess.run([sys.executable,REPO/'tools/runtime-stage.py',*args,
                '--stable-runner',valid,'--out',unused],capture_output=True,timeout=5)
            assert child.returncode==2 and not unused.exists() and b'stable-runner' in child.stderr
            rows.append(dict(id=name,passed=True,seconds=time.monotonic()-start))
        for name,path in [('missing',fixture/'absent.el'),('empty',empty),('oversized',large),
                          ('wrong-suffix',wrong),('directory',directory),('symlink',link),
                          ('symlink-parent',parent/'runner.el')]:
            start=time.monotonic();unused=fixture/('invalid-'+name)
            child=subprocess.run([sys.executable,REPO/'tools/runtime-stage.py','--action','execute',
                '--execution-phase','stable','--stable-runner',path,'--out',unused],
                capture_output=True,timeout=5)
            assert child.returncode==2 and not unused.exists()
            rows.append(dict(id='invalid-'+name,passed=True,seconds=time.monotonic()-start))
        products=[dict(id=n,kind=k,test_hooks=False,collector='off',resource_roots={})
                  for n,k in [('reference','reference'),('first','terminal'),('second','cocoa')]]
        for phase,subset in [('semantics',products),('performance',products),('stable',products[:1])]:
            start=time.monotonic()
            try:stage.execution_plan(phase,subset,stable_runner=valid)
            except ValueError:pass
            else:raise AssertionError('invalid runner scope accepted')
            rows.append(dict(id='plan-'+phase,passed=True,seconds=time.monotonic()-start))
        for target in ('source','captured','selection','frozen'):
            start=time.monotonic();root=fixture/('mutation-'+target);root.mkdir()
            source=root/'source';frozen=source/'tools/fixtures/stable-suite-runner.el'
            frozen.parent.mkdir(parents=True);frozen.write_text(';; frozen control\n')
            selected=root/'selected.el';selected.write_text(';; selected control\n')
            for product in products:
                program=root/product['id'];program.write_bytes(b'unchanged product control\n')
                product.update(program=str(program),dump=str(program))
            product_path=root/'products.json';stage.execution.write_json(product_path,dict(products=products))
            build=(root,source,product_path,products,dict(passed=True))
            def dispatch(*args,**kwargs):
                paths={'source':selected,'captured':root/'stable-runner/stable-suite-runner.el',
                       'selection':root/'stable-runner/stable-runner-selection.json','frozen':frozen}
                with paths[target].open('a') as f:f.write('changed')
                return [dict(id='stable-first',passed=True,execution_complete=True)]
            with mock.patch.object(stage,'read_action_build',return_value=build), \
                 mock.patch.object(stage.execution,'load_products',return_value=products), \
                 mock.patch.object(stage,'_execute_profiles',side_effect=dispatch):
                result=stage.execute_matrix(SimpleNamespace(action='execute',execution_phase='stable',
                    stable_profile=['first'],stable_runner=selected),root)
            assert result['passed'] is False and result['execution_complete'] is False
            assert result['stable_runner_selection']['unchanged'] is False
            identities=stage.execution.bounded_json(root/'product-identities.json')
            assert identities['before']==identities['after']
            assert 'stable-runner-selection' in identities['identity_errors']
            rows.append(dict(id='mutation-'+target,passed=True,seconds=time.monotonic()-start))
    finally:shutil.rmtree(fixture)
    report=dict(schema='femacs-stable-runner-protection-v1',passed=len(rows)==20,cases=rows,
        functional_acceptance_sha256=stage.execution.digest(functional) if functional is not None else None,
        owned_fixture_retired=not fixture.exists(),actual_editor_execution=False,
        seconds=time.monotonic()-started)
    stage.execution.write_json(out/'results.json',report)
    return report['passed']


def stable_protection(out):
    fixture=out/'owned-stable-protections';fixture.mkdir()
    rows=[];started=time.monotonic()
    try:
        products=[dict(id=name,kind=kind,test_hooks=hooks,collector='off',resource_roots={})
                  for name,kind,hooks in [('reference','reference',False),
                    ('first','terminal',False),('second','cocoa',False),('diagnostic','cocoa',True)]]
        for name,phase,selection in [
            ('duplicate-selection','stable',['first','first']),
            ('unknown-selection','stable',['missing']),
            ('reference-selection','stable',['reference']),
            ('diagnostic-selection','stable',['diagnostic']),
            ('empty-id','stable',['']),
            ('selection-outside-stable','all',['first'])]:
            case_started=time.monotonic()
            try:stage.execution_plan(phase,products,selection)
            except ValueError:pass
            else:raise AssertionError(name+' accepted')
            rows.append(dict(id=name,passed=True,seconds=time.monotonic()-case_started))
        case_started=time.monotonic()
        assert stage.execution_plan('stable',products)[2]==['stable-first','stable-second']
        assert stage.execution_plan('stable',products,['second','first'])[2]==['stable-first','stable-second']
        rows.append(dict(id='default-full-and-canonical-product-order',passed=True,
                         seconds=time.monotonic()-case_started))
        for name,arguments in [
            ('wrong-cli-phase',['--action','execute','--execution-phase','semantics','--stable-profile','first']),
            ('wrong-cli-action',['--action','components','--stable-profile','first']),
            ('duplicate-cli-selection',['--action','execute','--execution-phase','stable',
                                        '--stable-profile','first','--stable-profile','first'])]:
            case_started=time.monotonic();unused=fixture/name
            child=subprocess.run([sys.executable,REPO/'tools/runtime-stage.py',*arguments,'--out',unused],
                stdout=subprocess.PIPE,stderr=subprocess.PIPE,timeout=5)
            assert child.returncode==2 and not unused.exists() and b'stable' in child.stderr
            rows.append(dict(id=name,passed=True,seconds=time.monotonic()-case_started))
        source=fixture/'source';source.mkdir()
        for product in products:
            program=fixture/product['id'];program.write_bytes(b'unchanged control\n')
            product.update(program=str(program),dump=str(program))
        product_path=fixture/'products.json';stage.execution.write_json(product_path,dict(products=products))
        build=(fixture,source,product_path,products,dict(passed=True))
        for name,ids,inner_complete,outcome,complete in [
            ('missing-pair',[],True,True,False),
            ('duplicate-pair',['stable-first','stable-first'],True,True,False),
            ('unexpected-unselected-pair',['stable-first','stable-second'],True,True,False),
            ('incomplete-original',['stable-first'],False,False,False),
            ('failed-original-retained',['stable-first'],True,False,True),
            ('changed-unselected-product',['stable-first'],True,True,False)]:
            root=fixture/name;root.mkdir();case_started=time.monotonic()
            observed=[dict(id=identity,execution_complete=inner_complete,passed=outcome) for identity in ids]
            def dispatch(*args,**kwargs):
                if name=='changed-unselected-product':Path(products[-1]['program']).write_bytes(b'changed diagnostic control\n')
                return observed
            with mock.patch.object(stage,'read_action_build',return_value=build), \
                 mock.patch.object(stage.execution,'load_products',return_value=products), \
                 mock.patch.object(stage,'_execute_profiles',side_effect=dispatch):
                result=stage.execute_matrix(SimpleNamespace(action='execute',execution_phase='stable',
                    stable_profile=['first']),root)
            assert not result['passed'] and result['execution_complete'] is complete
            assert not result['stable_phase_complete'] and not result['full_matrix_execution_complete']
            assert result['remaining_stable_profiles']==['second'] and result['cases']==observed
            assert 'stable' in result['remaining_execution_phases']
            rows.append(dict(id=name,passed=True,seconds=time.monotonic()-case_started))
    finally:shutil.rmtree(fixture)
    report=dict(schema='femacs-stable-scope-protection-v1',passed=len(rows)==16,cases=rows,
        owned_fixture_retired=not fixture.exists(),actual_editor_execution=False,
        seconds=time.monotonic()-started)
    stage.execution.write_json(out/'results.json',report)
    return report['passed']


def integration(out):
    fixture = out/'owned-phase-fixtures'
    fixture.mkdir()
    started = time.monotonic()
    rows = []
    try:
        source = fixture/'source'
        (source/'tools').mkdir(parents=True)
        for name in ('runtime-acceptance.py', 'stable-suite.py'):
            (source/'tools'/name).write_text(CHILD)
        (fixture/'program').write_bytes(b'synthetic product, never executed\n')
        (fixture/'dump').write_bytes(b'synthetic dump, never loaded\n')
        products = [dict(id=name, kind=kind, test_hooks=hooks, collector='off',
                         program=str(fixture/'program'), dump=str(fixture/'dump'), resource_roots={})
                    for name, kind, hooks in [('reference','reference',False),
                                              ('candidate','terminal',False),
                                              ('diagnostic','terminal',True)]]
        products_path = fixture/'products.json'
        stage.execution.write_json(products_path, dict(products=products))
        build = (fixture, source, products_path, products, dict(passed=True))
        before = {p['id']:stage.execution.product_identity(p,resources=True) for p in products}
        current = fixture/'current-harness-source'
        (current/'tools').mkdir(parents=True)
        for name in ('runtime-acceptance.py', 'runtime-execution.py', 'runtime-retire.py',
                     'runtime-profile.py', 'runtime-language-contract.py', 'reference-corpus.py'):
            shutil.copy2(REPO/'tools'/name, current/'tools'/name)
        shutil.copytree(REPO/'tools/fixtures', current/'tools/fixtures')
        (current/'tools/runtime-acceptance.py').write_text(CHILD+'\n# synthetic current driver\n')
        identity='rtl08-weak-modes-after-explicit-full'
        live='((key (:value) 1) (value (:value) 1) (key-or-value (:value) 1) (key-and-value (:value) 1))'
        reference='('+live+' ((key . 0) (value . 0) (key-or-value . 0) (key-and-value . 0)))'
        candidate='('+live+' ((key . 0) (value . 0) (key-or-value . 1) (key-and-value . 0)))'
        observation=fixture/'language-observation.json'
        stage.execution.write_json(observation,dict(id=identity,
            reference=[[identity,reference]],candidate=[[identity,candidate]]))
        env = dict(os.environ, PHASE_CONTRACT_DRIVER=str(REPO/'tools/runtime-execution.py'),
                   PHASE_LANGUAGE_OBSERVATION=str(observation))
        for phase, expected in [('semantics',['ert','negative','dump','relocation']),
                                ('performance',['performance','performance-fallback']),
                                ('stable',['stable-candidate']),
                                ('all',['ert','negative','dump','relocation','performance','performance-fallback','stable-candidate'])]:
            root = fixture/phase
            root.mkdir()
            options = SimpleNamespace(action='execute', execution_phase=phase, timeout=5)
            case_started = time.monotonic()
            with mock.patch.object(stage,'read_action_build',return_value=build), \
                 mock.patch.object(stage,'REPO',current), \
                 mock.patch.object(stage.execution,'load_products',return_value=products), \
                 mock.patch.object(stage,'clean_environment',return_value=env):
                result = stage.execute_matrix(options,root)
            assert [row['id'] for row in result['cases']] == expected
            assert all(row['process']['exit_code']==0 and row['process']['complete']
                       and not row['process']['remaining_group_terminated'] for row in result['cases'])
            valid = phase in ('semantics','performance')
            assert result['passed'] is valid and result['execution_complete'] is valid
            assert result['full_matrix_execution_complete'] is False
            assert result['execution_phase']==phase
            assert result['product_identity_unchanged'] and result['complete_receipts_valid']
            if phase != 'stable':
                selected=result['authored_harness_selection']
                assert selected['unchanged'] is True
                assert selected['schema']=='femacs-authored-python-harness-v1'
                assert result['harness_root']==str(root/'authored-harness')
                for _, files, _ in stage.execution.fixture_groups(dict(kind='terminal',
                        observer=True, test_hooks=True, collector='young', native=True), 'all'):
                    for name in files:
                        captured=root/'authored-harness/tools/fixtures'/name
                        assert captured.is_file()
                        assert stage.execution.digest(captured)==stage.execution.digest(current/'tools/fixtures'/name)
                driver=root/'authored-harness/tools/runtime-acceptance.py'
                assert stage.execution.digest(driver)==stage.execution.digest(current/'tools/runtime-acceptance.py')
                assert stage.execution.digest(driver)!=stage.execution.digest(source/'tools/runtime-acceptance.py')
                stage.execution.write_json(root/'results.json',result)
                audit=stage.execution.load_tool('runtime-final-audit.py')
                registry=audit.InputPinRegistry()
                _,_,saved=registry.read_json(root/'results.json',label='synthetic-authored-stage')
                audit._verify_authored_harness(registry,root/'results.json',saved,
                    dict(source_root=str(source)))
                assert registry.verify_all_unchanged()[0]
                for row in result['cases']:
                    if not row['id'].startswith('stable-'):
                        assert row['process']['argv'][1]==str(driver)
            if not valid:
                assert 'receipt_validation_error' in result['cases'][-1]
            for row in result['cases']:
                if row['id'].startswith('stable-'):
                    continue
                actual = stage.execution.read_completion(source/'build/acceptance'/phase/row['id'],
                    'performance' if row['id']=='performance-fallback' else row['id'],products_path)
                assert actual['passed'] is True
                if row['id']=='ert':
                    detail=json.loads(Path(row['receipt']).read_text())
                    comparison=detail['language_observation_comparisons'][0]
                    assert comparison['passed'] and comparison['literal_equal'] is False
                    assert comparison['contract']=='conservative-weak-reachability-v1'
                    assert detail['raw_observations']['reference']!=detail['raw_observations']['candidate']
                if row['id'].startswith('performance'):
                    assert actual['performance_control']['permanent_full_fallback'] is (row['id']=='performance-fallback')
            rows.append(dict(phase=phase, passed=True, child_processes=len(expected),
                             invalid_corpus_receipt_rejected=not valid,
                             seconds=time.monotonic()-case_started))
        root=fixture/'wrong-mode';root.mkdir();case_started=time.monotonic()
        wrong_env=dict(env,PHASE_BAD_FALLBACK_MODE='1')
        with mock.patch.object(stage,'read_action_build',return_value=build), \
             mock.patch.object(stage,'REPO',current), \
             mock.patch.object(stage.execution,'load_products',return_value=products), \
             mock.patch.object(stage,'clean_environment',return_value=wrong_env):
            result=stage.execute_matrix(SimpleNamespace(action='execute',execution_phase='performance',timeout=5),root)
        assert [row['id'] for row in result['cases']]==['performance','performance-fallback']
        assert result['cases'][0]['passed'] and not result['cases'][1]['passed']
        assert not result['passed'] and not result['execution_complete']
        assert 'performance mode differs' in result['cases'][1]['receipt_validation_error']
        rows.append(dict(phase='wrong-performance-mode',passed=True,child_processes=2,
                         seconds=time.monotonic()-case_started))
        assert before=={p['id']:stage.execution.product_identity(p,resources=True) for p in products}
    finally:
        # Only this fresh fixture subtree is selected; no real build inputs enter it.
        shutil.rmtree(fixture)
    report = dict(schema='femacs-runtime-phase-file-process-integration-v1',
                  passed=len(rows)==5 and all(row['passed'] for row in rows), cases=rows,
                  actual_python_children=sum(row['child_processes'] for row in rows),
                  authored_harness_snapshot_and_audit_verified=True,
                  real_completion_readers_exercised=True,
                  compiled_product_admission_isolated=True, actual_editor_execution=False,
                  owned_fixture_retired=not fixture.exists(), seconds=time.monotonic()-started)
    stage.execution.write_json(out/'results.json',report)
    return report['passed']


def protection(out):
    fixture=out/'owned-scope-fixtures'
    fixture.mkdir()
    rows=[]
    started=time.monotonic()
    try:
        products=[dict(id='reference',kind='reference',test_hooks=False),
                  dict(id='candidate',kind='terminal',test_hooks=False),
                  dict(id='diagnostic',kind='terminal',test_hooks=True)]
        expected={'semantics':['ert','negative','dump','relocation'],
                  'performance':['performance','performance-fallback'], 'stable':['stable-candidate'],
                  'all':['ert','negative','dump','relocation','performance','performance-fallback','stable-candidate']}
        for phase,ids in expected.items():
            assert stage.execution_plan(phase,products)[2]==ids
        rows.append(dict(id='complete-phase-partition-and-diagnostic-exclusion',passed=True))
        for name,phase,values in [
            ('unknown-phase','unknown',products),
            ('empty-stable-scope','stable',products[:1]),
            ('duplicate-candidate-scope','all',products+[products[1]]),
            ('duplicate-reference-scope','all',products+[products[0]])]:
            try:
                stage.execution_plan(phase,values)
            except ValueError:
                rows.append(dict(id=name,passed=True))
            else:
                raise AssertionError(name+' accepted')
        unused=fixture/'must-not-exist'
        child=subprocess.run([sys.executable,REPO/'tools/runtime-stage.py','--action','prepare',
                              '--execution-phase','stable','--out',unused],
                             stdout=subprocess.PIPE,stderr=subprocess.PIPE,timeout=5)
        assert child.returncode==2 and not unused.exists()
        rows.append(dict(id='phase-option-rejected-outside-execute-before-output',passed=True))
        (fixture/'program').write_bytes(b'scope control\n')
        (fixture/'dump').write_bytes(b'dump control\n')
        for product in products:
            product.update(program=str(fixture/'program'),dump=str(fixture/'dump'),resource_roots={})
        source=fixture/'source';source.mkdir()
        product_path=fixture/'products.json';stage.execution.write_json(product_path,dict(products=products))
        build=(fixture,source,product_path,products,dict(passed=True))
        scenarios=[('missing-case','semantics',expected['semantics'][:-1],False,False,False),
                   ('missing-fallback','performance',['performance'],False,False,False),
                   ('duplicate-case','semantics',expected['semantics']+['ert'],False,False,False),
                   ('incomplete-inner','semantics',expected['semantics'],False,False,False),
                   ('valid-phase-not-complete-matrix','semantics',expected['semantics'],True,True,False),
                   ('valid-all-complete-matrix','all',expected['all'],True,True,True),
                   ('failed-outcome-retained','semantics',expected['semantics'],False,True,False),
                   ('changed-product-rejected','semantics',expected['semantics'],False,False,False),
                   ('invalid-product-receipt-rejected','semantics',expected['semantics'],False,False,False)]
        for name,phase,ids,passed,complete,full in scenarios:
            root=fixture/name;root.mkdir()
            observed=[dict(id=identity,passed=True,execution_complete=True) for identity in ids]
            if name=='incomplete-inner':observed[-1]['execution_complete']=False
            if name=='failed-outcome-retained':observed[-1]['passed']=False
            def dispatch(*args,**kwargs):
                if name=='changed-product-rejected':(fixture/'program').write_bytes(b'changed control\n')
                return observed
            loader=ValueError('synthetic product receipt changed') if name=='invalid-product-receipt-rejected' else None
            with mock.patch.object(stage,'read_action_build',return_value=build), \
                 mock.patch.object(stage.execution,'load_products',return_value=products,side_effect=loader), \
                 mock.patch.object(stage,'_execute_profiles',side_effect=dispatch):
                result=stage.execute_matrix(SimpleNamespace(action='execute',execution_phase=phase),root)
            assert result['passed'] is passed and result['execution_complete'] is complete
            assert result['full_matrix_execution_complete'] is full
            assert result['cases']==observed
            assert set(result['remaining_execution_phases'])==(
                set() if phase=='all' else {'semantics','stable','performance'}-{phase})
            rows.append(dict(id=name,passed=True))
    finally:
        shutil.rmtree(fixture)
    stage.execution.write_json(out/'results.json',dict(
        schema='femacs-runtime-phase-protection-v1',passed=len(rows)==15,
        cases=rows,owned_fixture_retired=not fixture.exists(),seconds=time.monotonic()-started))
    return len(rows)==15


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out',type=Path,required=True)
    parser.add_argument('--mode',choices=('integration','protection','stable-integration','stable-protection','runner-protection'),default='integration')
    parser.add_argument('--functional-acceptance',type=Path)
    options=parser.parse_args()
    if options.functional_acceptance is not None and options.mode != 'runner-protection':
        parser.error('functional-acceptance applies only to runner-protection')
    out=stage.execution.new_output(options.out)
    run={'integration':integration,'protection':protection,
         'stable-integration':stable_integration,'stable-protection':stable_protection,
         'runner-protection':runner_protection}
    if options.mode=='runner-protection':
        return 0 if runner_protection(out,options.functional_acceptance) else 1
    return 0 if run[options.mode](out) else 1


if __name__=='__main__':
    raise SystemExit(main())
