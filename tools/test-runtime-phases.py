"""Exercise phase dispatch through real Python children and strict receipt readers.

The owned source/product descriptors are synthetic; this is coordinator file/process
integration, not editor, compiler, corpus or performance acceptance.
"""
import argparse
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

CHILD = '''import argparse, importlib.util, os
from pathlib import Path
p=argparse.ArgumentParser()
p.add_argument('--out',type=Path,required=True)
p.add_argument('--action')
p.add_argument('--products',type=Path)
a,rest=p.parse_known_args()
s=importlib.util.spec_from_file_location('phase_receipts',os.environ['PHASE_CONTRACT_DRIVER'])
e=importlib.util.module_from_spec(s);s.loader.exec_module(e)
a.out.mkdir(parents=True)
if a.action:
 e.write_json(a.out/'inputs.json',dict(action=a.action,product_list_sha256=e.digest(a.products)))
 report=dict(schema='femacs-runtime-execution-v1',action=a.action,passed=True,
  cases=[dict(id='synthetic-phase-dispatch',passed=True)],
  full_product_inputs_unchanged=True,fixtures_unchanged=True)
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
 harness_sources={str(Path(__file__)):e.digest(Path(__file__))})
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
        for name,selection,outcome,expected,passed,complete in [
            ('selected',['second'],'passed',['second'],True,True),
            ('all-selected',['second','first'],'passed',['first','second'],True,True),
            ('tampered',['second'],'tampered',['second'],False,False),
            ('failed-outcome',['second'],'failed',['second'],False,True)]:
            root=fixture/name;root.mkdir();case_started=time.monotonic()
            env=dict(os.environ,PHASE_CONTRACT_DRIVER=str(REPO/'tools/runtime-execution.py'),SCOPE_OUTCOME=outcome)
            with mock.patch.object(stage,'read_action_build',return_value=build), \
                 mock.patch.object(stage.execution,'load_products',return_value=products), \
                 mock.patch.object(stage,'clean_environment',return_value=env):
                result=stage.execute_matrix(SimpleNamespace(action='execute',execution_phase='stable',
                    stable_profile=selection,timeout=5),root)
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
            if outcome=='tampered':assert 'receipt_validation_error' in result['cases'][0]
            if outcome=='failed':assert result['cases'][0]['failure_count']==1
            rows.append(dict(id=name,passed=True,actual_python_children=len(expected),
                             seconds=time.monotonic()-case_started))
        assert before=={p['id']:stage.execution.product_identity(p,resources=True) for p in products}
    finally:
        shutil.rmtree(fixture)
    report=dict(schema='femacs-stable-scope-file-process-integration-v1',passed=len(rows)==4,
        cases=rows,actual_python_children=sum(row['actual_python_children'] for row in rows),
        real_dispatch_writer_reader_and_all_product_identity_boundary=True,
        compiled_product_admission_isolated=True,actual_editor_execution=False,
        original_corpus_execution=False,owned_fixture_retired=not fixture.exists(),
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
        env = dict(os.environ, PHASE_CONTRACT_DRIVER=str(REPO/'tools/runtime-execution.py'))
        for phase, expected in [('semantics',['ert','negative','dump','relocation']),
                                ('performance',['performance']),
                                ('stable',['stable-candidate']),
                                ('all',['ert','negative','dump','relocation','performance','stable-candidate'])]:
            root = fixture/phase
            root.mkdir()
            options = SimpleNamespace(action='execute', execution_phase=phase, timeout=5)
            case_started = time.monotonic()
            with mock.patch.object(stage,'read_action_build',return_value=build), \
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
            if not valid:
                assert 'receipt_validation_error' in result['cases'][-1]
            for row in result['cases']:
                if row['id'].startswith('stable-'):
                    continue
                actual = stage.execution.read_completion(source/'build/acceptance'/phase/row['id'],
                                                         row['id'],products_path)
                assert actual['passed'] is True
            rows.append(dict(phase=phase, passed=True, child_processes=len(expected),
                             invalid_corpus_receipt_rejected=not valid,
                             seconds=time.monotonic()-case_started))
        assert before=={p['id']:stage.execution.product_identity(p,resources=True) for p in products}
    finally:
        # Only this fresh fixture subtree is selected; no real build inputs enter it.
        shutil.rmtree(fixture)
    report = dict(schema='femacs-runtime-phase-file-process-integration-v1',
                  passed=len(rows)==4 and all(row['passed'] for row in rows), cases=rows,
                  actual_python_children=sum(row['child_processes'] for row in rows),
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
                  'performance':['performance'], 'stable':['stable-candidate'],
                  'all':['ert','negative','dump','relocation','performance','stable-candidate']}
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
        schema='femacs-runtime-phase-protection-v1',passed=len(rows)==14,
        cases=rows,owned_fixture_retired=not fixture.exists(),seconds=time.monotonic()-started))
    return len(rows)==14


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out',type=Path,required=True)
    parser.add_argument('--mode',choices=('integration','protection','stable-integration','stable-protection'),default='integration')
    options=parser.parse_args()
    out=stage.execution.new_output(options.out)
    run={'integration':integration,'protection':protection,
         'stable-integration':stable_integration,'stable-protection':stable_protection}
    return 0 if run[options.mode](out) else 1


if __name__=='__main__':
    raise SystemExit(main())
