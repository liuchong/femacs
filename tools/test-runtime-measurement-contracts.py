"""Actual file/process integration of measurement tools, not editor performance.

Run under controlled-run. The full-shape input values and editor descriptors
are explicitly synthetic; the real operations are owned Python children,
sampler/exit timing, orchestration, accounting and bounded report CLI reading.
"""
import argparse
import importlib.util
import json
import os
from pathlib import Path
import shutil
import sys
import threading
import time
from types import SimpleNamespace
from unittest import mock

REPO=Path(__file__).resolve().parent.parent
spec=importlib.util.spec_from_file_location('measurement_contract_execution',REPO/'tools/runtime-execution.py')
execution=importlib.util.module_from_spec(spec);spec.loader.exec_module(execution)
accounting=execution.load_tool('runtime-acceptance.py')
reporter=execution.load_tool('runtime-measurement-report.py')
EXPECTED={'load':'5','edit':'1200','regex':'1000','bytecode':'1249975000',
          'gc-churn':'30000','gc-retained':'10000'}

MOCK_EDITOR='''import json,os,sys,time
from pathlib import Path
time.sleep(.15)
if '--eval' in sys.argv:
 print('FEMACS-RUNTIME-STARTUP-READY')
else:
 mode=os.environ['FEMACS_RUNTIME_EXECUTION_MODE']
 expected={'load':'5','edit':'1200','regex':'1000','bytecode':'1249975000','gc-churn':'30000','gc-retained':'10000'}
 rows=[dict(case=name,result=value,wall_ms=1.0,gc_ms=0,gc_count=0,pauses_ms=[],
  allocation_delta=[0]*7,heap_before=[['cons',1,2]],heap_after=[['cons',1,2]],
  execution_mode=mode,actual_function_kind=mode) for name,value in expected.items()]
 root=Path(os.environ['FEMACS_RUNTIME_TEST_ROOT'])
 (root/'profile-observations.json').write_text(json.dumps(dict(
  schema='femacs-runtime-performance-observation-v1',mode=mode,synthetic_contract_fixture=True)))
 if Path(os.environ['FEMACS_RUNTIME_PROFILE_FIXTURE']).name=='runtime-profile-fallback.el':
  status=[0]*24;status[14]=1
  (root/'fallback-context.json').write_text(json.dumps(dict(schema='femacs-runtime-performance-fallback-v1',
   module_error=dict(symbol='module-open-failed',path_matched=True,message_nonempty=True),
   after=[1,status,[0]*13],latched=[1,status,[0]*13],synthetic_contract_fixture=True)))
 print('FEMACS-PROFILE '+json.dumps(dict(schema='femacs-runtime-profile-v1',cases=rows)))
'''

def specs():
    values=[dict(id='reference',native=True,observer=False,collector='off',test_hooks=False,kind='reference')]
    for native in (False,True):
        for observer in (False,True):
            for family in ('terminal','cocoa'):
                for leg in ('normal','checking'):
                    shared=f'{family}-{leg}-n{int(native)}-o{int(observer)}'
                    for collector in ('off','major','young'):
                        values.append(dict(id=shared+'-'+collector,native=native,observer=observer,
                            collector=collector,test_hooks=False,kind=family,layout=shared))
    return values

def full_document(fallback):
    products=specs();controls=[]
    for product in products:
        modes=('interpreted','bytecode') if fallback or not product['native'] else ('interpreted','bytecode','native')
        observations=('off','phases','survival','all') if product['observer'] and not fallback else ('off',)
        controls.extend((product,mode,observer) for mode in modes for observer in observations)
    processes=[];startups=[];samples=[]
    # Bounded realistic raw metadata deliberately exceeds the old16MiB gate.
    snapshot=[1,[0]*21,[[1,0,22,0,100,200]]*16,[[0]*5,[0]*5]]
    for index in range(-2,7):
        for product,mode,observer in controls:
            processes.append(dict(warmup=index<0))
            if index<0:continue
            for case in EXPECTED:
                samples.append(dict(case=case,mode=mode,execution_identity=product['id']+':'+observer,
                    round=index+1,wall_ms=1+index/1000,pauses_ms=[.05,.1,.15] if case=='gc-churn' else [],
                    peak_rss_bytes=4096,peak_footprint_bytes=8192,peak_accounted_bytes=8192,
                    load_per_core=.01,correctness=True,phase_snapshot=snapshot))
        if not fallback:
            for product in products:
                startups.append(dict(warmup=index<0))
                if index>=0:samples.append(dict(case='startup',mode='startup',
                    execution_identity=product['id']+':off',round=index+1,wall_ms=1+index/1000,
                    pauses_ms=[],peak_rss_bytes=4096,peak_footprint_bytes=8192,
                    peak_accounted_bytes=8192,load_per_core=.01,correctness=True))
    summary=accounting.summarize_performance(samples)
    groups={(row['execution_identity'],row['mode'],row['case']) for row in summary['cases']}
    interval=accounting.paired_speed_interval([1]*7,[1]*7,seed=0,resamples=10000)
    comparisons=[]
    def compare(left,right,layout):
        for mode in ('interpreted','bytecode','native','startup'):
            for case in (('startup',) if mode=='startup' else EXPECTED):
                if (left['id']+':off',mode,case) not in groups or (right['id']+':off',mode,case) not in groups:continue
                row=dict(case=case,mode=mode,paired_rounds=7,interval=dict(interval),eligible=True)
                row.update(dict(kind='same-layout-major-young',major=left['id'],young=right['id'])
                    if layout else dict(reference=left['id'],candidate=right['id']))
                comparisons.append(row)
    for product in products[1:]:compare(products[0],product,False)
    for major in products:
        if major['collector']!='major':continue
        young=next(p for p in products if p['collector']=='young' and p['layout']==major['layout'])
        compare(major,young,True)
    return dict(schema=reporter.SCHEMA,action='performance',processes=processes,samples=samples,startups=startups,
        summary=summary,comparisons=comparisons,errors=[],excluded=[],
        fixture_sources={'synthetic-contract-fixture':'not actual editor measurements'},
        fixtures_unchanged=True,passed=True,performance_accepted=False,limitation=reporter.LIMITATION,
        full_product_inputs_unchanged=True,driver_sources_unchanged=True,goal_complete=False,
        compatibility_accepted=False,state_retirement_complete=True,
        permanent_full_fallback=fallback,startup_measured=not fallback)

def integration(out):
    fixture=out/'owned-measurement-fixtures';fixture.mkdir()
    started=time.monotonic();cases=[];completed=False
    try:
        for name,delay,timeout,code in (('exit',.15,2,0),('nonzero',.15,2,7),('timeout',5,.25,None)):
            root=fixture/name;env=dict(os.environ)
            row=execution.run_process([sys.executable,'-c',f'import time;time.sleep({delay});raise SystemExit({code or 0})'],
                                      root,env,timeout,metrics=True,observe_exit=True)
            entry=dict(id=name,passed=False,process=row);cases.append(entry)
            execution.write_json(out/(name+'-process.json'),row)
            assert row['exit_timing_complete'] and 0<row['process_lifetime_seconds']<=row['seconds']
            assert row['exit_timing_method']=='spawn-to-reaped-exit-waiter'
            assert row['memory_samples']>0 and row['peak_accounted_bytes']>0
            assert row['timed_out'] is (name=='timeout') and row['complete'] is (name!='timeout')
            if code is not None:assert row['exit_code']==code
            assert not any(t.name=='runtime-owned-child-exit' for t in threading.enumerate())
            entry['passed']=True
        mock=fixture/'synthetic-editor';mock.write_text('#!'+sys.executable+'\n'+MOCK_EDITOR);mock.chmod(0o700)
        dump=fixture/'synthetic-dump';dump.write_bytes(b'never loaded by an editor\n')
        products=[dict(id=name,kind=kind,collector=collector,native=True,observer=False,test_hooks=False,
            program=str(mock),dump=str(dump),compiler_leg='normal',resource_roots={})
            for name,kind,collector in (('reference','reference','off'),('major','terminal','major'),('young','terminal','young'))]
        for fallback in (False,True):
            root=fixture/('mechanism-fallback' if fallback else 'mechanism-primary');root.mkdir()
            result=execution.run_performance(products,root,5,permanent_full_fallback=fallback)
            execution.write_json(out/(root.name+'-result.json'),result)
            assert result['passed'] and result['performance_accepted'] is False
            assert len(result['processes'])==(54 if fallback else 81)
            assert len(result['startups'])==(0 if fallback else 27)
            startup=[s for s in result['samples'] if s['mode']=='startup']
            assert len(startup)==(0 if fallback else 21)
            assert all(s['case']=='startup' and s['execution_identity'].endswith(':off') for s in startup)
            assert all(s['mode'] in ('interpreted','bytecode') for s in result['samples']) if fallback else True
            assert any(c['case']=='startup' and c.get('kind')=='same-layout-major-young' for c in result['comparisons']) is (not fallback)
            cases.append(dict(id='mechanism-fallback' if fallback else 'mechanism-primary',passed=True,
                actual_python_children=len(result['processes'])+len(result['startups']),
                samples=len(result['samples']),cases=len(result['summary']['cases']),
                comparisons=len(result['comparisons']),errors=result['errors'],
                actual_editor_execution=False))
        for fallback in (False,True):
            name='fallback' if fallback else 'primary';document=full_document(fallback)
            source=fixture/(name+'.json');execution.write_json(source,document)
            counts=(4116,588,768) if fallback else (13069,1867,1024)
            assert (len(document['samples']),len(document['summary']['cases']),len(document['comparisons']))==counts
            if not fallback:assert 16*1024*1024<source.stat().st_size<=reporter.MAX_INPUT_BYTES
            process_root=fixture/(name+'-reader');destination=out/(name+'-report.json')
            row=execution.run_process([sys.executable,REPO/'tools/runtime-measurement-report.py',
                '--input',source,'--format','json','--out',destination],process_root,dict(os.environ),60)
            assert row['exit_code']==0 and row['complete'] and not row['remaining_group_terminated']
            public=json.loads(destination.read_text())
            assert public['counts']['sample_count']==counts[0] and public['eligibility']['case_count']==counts[1]
            assert public['eligibility']['comparison_count']==counts[2] and not public['performance_accepted']
            assert public['permanent_full_fallback'] is fallback and public['startup_measured'] is (not fallback)
            assert all(p['peak_footprint_bytes']==8192 and p['peak_accounted_bytes']==8192 for p in public['cases'])
            assert all(p['median_pause_ms']==.1 and abs(p['mad_pause_ms']-.05)<1e-12
                       for p in public['cases'] if p['case']=='gc-churn')
            cases.append(dict(id='full-shape-'+name,passed=True,input_bytes=source.stat().st_size,
                input_sha256=execution.digest(source),process=row,samples=counts[0],summary_cases=counts[1],
                comparisons=counts[2],synthetic_grid=True,actual_editor_execution=False))
        completed=True
    finally:
        error=sys.exc_info()[1]
        shutil.rmtree(fixture)
        report=dict(schema='femacs-measurement-contract-functional-integration-v1',
            passed=completed and len(cases)==7 and all(row['passed'] for row in cases),
            cases=cases,owned_fixture_retired=not fixture.exists(),seconds=time.monotonic()-started,
            error=None if error is None else type(error).__name__+': '+str(error),
            actual_editor_execution=False,real_performance_measurement=False,goal_complete=False)
        execution.write_json(out/'results.json',report)
    return report['passed']

def protection(out):
    fixture=out/'owned-measurement-protection';fixture.mkdir()
    rows=[];started=time.monotonic();completed=False
    try:
        class NativeRead:
            def __call__(self,pid,version,pointer):
                assert version==1
                pointer._obj.phys_footprint=400+100*pid
                return 0
        class FakeProcess:
            def __init__(self,pid):self.pid=pid
            def children(self,recursive):return []
            def create_time(self):return float(self.pid)
            def memory_info(self):return SimpleNamespace(rss=100*self.pid)
        fake_psutil=SimpleNamespace(Process=FakeProcess,NoSuchProcess=type('Gone',(Exception,),{}))
        execution.prepare_memory_sampling.cache_clear()
        with mock.patch.dict(sys.modules,psutil=fake_psutil), \
             mock.patch.object(execution.sys,'platform','darwin'), \
             mock.patch.object(execution.ctypes.util,'find_library',return_value='synthetic-native-library'), \
             mock.patch.object(execution.ctypes,'CDLL',return_value=SimpleNamespace(proc_pid_rusage=NativeRead())) as library:
            assert execution.sample_memory(1)==(100,500,500)
            assert execution.sample_memory(2)==(200,600,600)
            assert library.call_count==1
        execution.prepare_memory_sampling.cache_clear()
        rows.append(dict(id='cache-bindings-but-read-fresh-pid-values',passed=True))
        for retired in (False,True):
            reads=[]
            def current_process(pid):
                reads.append(pid)
                if len(reads)==1:return FakeProcess(pid)
                if retired:raise fake_psutil.NoSuchProcess()
                return SimpleNamespace(create_time=lambda:float(pid)+1)
            binding=(SimpleNamespace(Process=current_process,NoSuchProcess=fake_psutil.NoSuchProcess),
                     None,None,None)
            with mock.patch.object(execution,'prepare_memory_sampling',return_value=binding), \
                 mock.patch.object(execution.sys,'platform','linux'):
                if retired:assert execution.sample_memory(1) is None
                else:
                    try:execution.sample_memory(1)
                    except RuntimeError as error:assert str(error)=='owned sampling PID identity changed'
                    else:raise AssertionError('cached original birth accepted a reused PID')
            assert reads==[1,1]
        rows.append(dict(id='fresh-identity-rejects-reuse-and-omits-retired-sample',passed=True))
        class Child:
            pid=12345
            returncode=0
            def __init__(self):self.polls=0;self.waits=0
            def poll(self):
                self.polls+=1
                return None if self.polls==1 else 0
            def wait(self,timeout=None):self.waits+=1;return 0
        class BrokenWaiter:
            ident=None
            def __init__(self,**kwargs):pass
            def start(self):raise RuntimeError('owned waiter start failed')
        for broken in (False,True):
            events=[];child=Child();root=fixture/('failed-waiter' if broken else 'prepare-order')
            def spawn(*args,**kwargs):
                assert events==['sampler-ready'];events.append('spawn');return child
            with mock.patch.object(execution,'prepare_memory_sampling',side_effect=lambda:events.append('sampler-ready')), \
                 mock.patch.object(execution.subprocess,'Popen',side_effect=spawn), \
                 mock.patch.object(execution,'sample_memory',return_value=(1,2,2)), \
                 mock.patch.object(execution,'_OwnedProcessTree',return_value=SimpleNamespace(
                    attach=lambda child:None,refresh=lambda **kwargs:None,live=lambda:[],
                    signal=lambda number,**kwargs:None,
                    finish=lambda:dict(complete=True,remaining=[],errors=[]))), \
                 mock.patch.object(execution.os,'killpg',side_effect=ProcessLookupError) as kill:
                if broken:
                    with mock.patch.object(execution.threading,'Thread',BrokenWaiter):
                        try:execution.run_process(['synthetic'],root,{},1,metrics=True,observe_exit=True)
                        except RuntimeError as error:assert str(error)=='owned waiter start failed'
                        else:raise AssertionError('failed waiter setup accepted')
                    row=execution.bounded_json(root/'process.json')
                    assert not row['complete'] and not row['exit_timing_complete']
                else:
                    row=execution.run_process(['synthetic'],root,{},1,metrics=True,observe_exit=True)
                    assert row['complete'] and row['exit_timing_complete']
                    assert 0<row['process_lifetime_seconds']<=row['seconds']
                    assert row['memory_sampling_complete']
                assert child.waits>0 and kill.called
            rows.append(dict(id='waiter-setup-failure-reaps-owned-child' if broken else 'sampler-prepared-before-spawn',passed=True))
        completed=True
    finally:
        execution.prepare_memory_sampling.cache_clear()
        shutil.rmtree(fixture)
        execution.write_json(out/'results.json',dict(schema='femacs-measurement-contract-protection-v1',
            passed=completed and len(rows)==4,cases=rows,seconds=time.monotonic()-started,
            actual_child_processes=0,actual_editor_execution=False,owned_fixture_retired=not fixture.exists()))
    return completed

def main():
    parser=argparse.ArgumentParser(description=__doc__);parser.add_argument('--out',type=Path,required=True)
    parser.add_argument('--mode',choices=('integration','protection'),default='integration')
    options=parser.parse_args();out=execution.new_output(options.out)
    return 0 if {'integration':integration,'protection':protection}[options.mode](out) else 1

if __name__=='__main__':
    raise SystemExit(main())
