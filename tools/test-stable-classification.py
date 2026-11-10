"""Original writer/reader/classifier file and subprocess integration.

The 555-file descriptors and products are synthetic. No editor, original test
body or compiler runs here. Whole-product build admission is isolated; actual
reference/candidate admission has its separate P462 evidence.
"""
import argparse
import copy
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import shutil
import sys
import time
from types import SimpleNamespace
from unittest import mock

REPO = Path(__file__).resolve().parent.parent


def module(name, file):
    spec = importlib.util.spec_from_file_location(name, REPO/'tools'/file)
    result = importlib.util.module_from_spec(spec)
    sys.modules[name] = result
    spec.loader.exec_module(result)
    return result


stage = module('classification_test_stage', 'runtime-stage.py')
e = stage.execution
suite = e.load_tool('stable-suite.py')
accounting = module('classification_test_subject', 'stable-classification.py')


def child(root, manifest_path, program, dump, failure_mode):
    manifest = suite.load_manifest(manifest_path, REPO)
    entries = suite._load_batch().select_entrypoints(manifest)
    envs = dict(reference={'FEMACS_GC_OBSERVATION':'off'}, candidate={
        'FEMACS_GC_OBSERVATION':'off', 'FEMACS_GC_COLLECTOR':'major'})
    products = {label: suite._public_product_identity(suite._product_paths(
        program, ['--dump-file='+str(dump)], envs[label])) for label in envs}
    runner = REPO/'tools/fixtures/stable-suite-runner.el'
    cases, files, failures = [], [], []
    root.mkdir()
    for index, entry in enumerate(entries):
        logical, identity = entry['path'], 'synthetic-case-'+str(index)
        pair = {}
        for label in envs:
            status = ('failed' if index in (0, 2) else
                      'skipped' if index == 1 and label == 'candidate' else 'passed') if failure_mode else 'passed'
            diagnostic = 'synthetic '+status+' evidence '+str(index)
            event = dict(event='test-ended', file=logical, test_id=identity,
                status=status, duration=.001, diagnostic=diagnostic,
                expected=status=='passed', skipped=status=='skipped', condition='synthetic-condition')
            raw = json.dumps(event, sort_keys=True).encode()
            progress = root/(str(index)+'-'+label+'.events.jsonl')
            progress.write_bytes(raw+b'\n')
            case = dict(test_id=identity, status=status, complete=True, exit_code=0,
                        seconds=.001, diagnostic=diagnostic,
                        evidence=dict(path=progress.name, event_sha256=hashlib.sha256(raw).hexdigest(), event_ordinal=0))
            case['assessment'] = suite.assess_case(identity, case)
            final = root/(str(index)+'-'+label+'.json')
            row = dict(schema='femacs-stable-file-batch-v1', file=logical, product=label,
                ids=[identity], cases={identity:case}, complete=True, passed=status=='passed',
                timed_out=False, remaining_group_terminated=False, exit_code=0,
                argv=[str(program), '--dump-file='+str(dump)],
                metadata=dict(features=[], emacs_version='32.0.50'),
                progress=dict(path=progress.name, **accounting.bound_file(progress)))
            e.write_json(final, row)
            pair[label] = dict(row, file_result=dict(path=final.name, **accounting.bound_file(final)))
        passed = all(row['cases'][identity]['assessment']['passed'] for row in pair.values())
        cases.append(dict(file=logical, id=identity, passed=passed,
            reference=pair['reference']['cases'][identity], candidate=pair['candidate']['cases'][identity]))
        if not passed:
            failures.append('case failed/unverified: '+logical+' :: '+identity)
        files.append(dict(file=logical, registration=dict(passed=True, reference_count=1,
            missing=[], extra=[], duplicates=[]), reference_id_count=1, candidate_id_count=1,
            **{label:{key:value for key,value in row.items() if key not in ('cases', 'ids')}
               for label,row in pair.items()}))
    report = dict(schema=suite.RESULT_SCHEMA, source=manifest['source'],
        manifest_sha256=e.digest(manifest_path), selector='all-discovered-ids',
        entrypoint_count=len(entries), resource_only_lisp_inputs=manifest['lisp_file_count']-len(entries),
        cases=cases, files=files, failures=failures, corpus_complete=not failures,
        products_before=products, products_after=products, identities_unchanged=True,
        corpus_inputs_unchanged=True, source_context_unchanged=True,
        registration=dict(passed=True, reference_count=len(cases), candidate_count=len(cases)),
        runtime_environment=envs, harness_sources={str(runner):e.digest(runner),
            str(Path(__file__)):e.digest(Path(__file__))})
    suite._write_run_report(root, report)
    return 1 if failures else 0


def _anchor(path):
    data = path.read_bytes()
    return dict(path=str(path), **accounting.bound_file(path), start_line=1, end_line=1,
                lf_sha256=hashlib.sha256(data.replace(b'\r\n',b'\n').split(b'\n')[0]+b'\n').hexdigest())


def _fixture(root):
    root.mkdir()
    program = root/'product/bin/program'; program.parent.mkdir(parents=True)
    program.write_bytes(b'synthetic program; never executed\n')
    dump = root/'dump'; dump.write_bytes(b'synthetic dump\n')
    library = root/'product/lisp'; library.mkdir(); (library/'control.el').write_text(';; library\n')
    original = root/'original'; original.mkdir()
    source = original/'source'; source.mkdir()
    entries=[]
    for index in range(555):
        rel='test/lisp/synthetic-'+str(index).zfill(3)+'.el'
        path=source/rel; path.parent.mkdir(parents=True,exist_ok=True)
        raw=(';; synthetic unchanged input '+str(index)+'\n').encode(); path.write_bytes(raw)
        entries.append(dict(path=rel,source_path=rel,input='tracked',mode='100644',size=len(raw),
            sha256=e.digest(path),blob=hashlib.sha1(b'blob '+str(len(raw)).encode()+b'\0'+raw).hexdigest()))
    manifest=source/'compatibility/stable-31.1/sources.json';manifest.parent.mkdir(parents=True)
    e.write_json(manifest,dict(schema=1,source=dict(release=suite.RELEASE,commit=suite.COMMIT),
        file_count=len(entries),lisp_file_count=len(entries),total_bytes=sum(row['size'] for row in entries),entries=entries))
    candidates=[dict(id='candidate-'+str(index).zfill(2),kind='terminal',native=False,
        observer=False,test_hooks=index>=48,collector='off',compiler_leg='normal',
        program=str(program),dump=str(dump),resource_roots=dict(lisp=str(library))) for index in range(60)]
    reference=dict(candidates[0],id='installed-reference',kind='reference')
    products=[reference,*candidates]
    selection=dict(path=str(root/'reference-selection.json'),sha256='a'*64,epoch='synthetic-32',
                   upstream=dict(version='32.0.50',commit='b'*40))
    return SimpleNamespace(root=root,program=program,dump=dump,source=source,original=original,
        manifest=manifest,entries=entries,products=products,profile=candidates[0],selection=selection)


def _read(context, root, catalog=None):
    return suite.read_completion(root, context.manifest, context.program, context.program,
        ['--dump-file='+str(context.dump)], ['--dump-file='+str(context.dump)],
        {'FEMACS_GC_OBSERVATION':'off'}, {'FEMACS_GC_OBSERVATION':'off','FEMACS_GC_COLLECTOR':'major'},
        runner=REPO/'tools/fixtures/stable-suite-runner.el', classification=catalog,
        candidate_profile=context.profile, reference_selection=context.selection)


def _catalog(context, root, receipt, rules):
    value=dict(schema=accounting.SCHEMA,manifest_sha256=e.digest(context.manifest),
        reference_selection_sha256=context.selection['sha256'],contexts={context.profile['id']:dict(
            context_sha256=accounting.context_identity(receipt,context.profile,context.selection),
            run_sha256=receipt['detailed']['sha256'],completion_sha256=e.digest(root/'stable-suite-completion.json'))},
        rules=rules)
    target=context.root/('catalog-'+str(len(list(context.root.glob('catalog-*.json'))))+'.json')
    e.write_json(target,value)
    return target


def integration(out):
    fixture=out/'owned-fixtures';started=time.monotonic();rows=[]
    try:
        context=_fixture(fixture)
        processes=[]
        for mode in ('ordinary','classified'):
            root=fixture/mode
            process=e.run_process([sys.executable,Path(__file__),'--child',root,
                '--manifest',context.manifest,'--program',context.program,'--dump',context.dump,
                *(['--failed'] if mode=='classified' else [])],fixture/(mode+'-process'),dict(os.environ),20,cwd=REPO)
            assert process['complete'] and not process['remaining_group_terminated']
            processes.append(process)
            if process['exit_code'] != (1 if mode=='classified' else 0):
                logs=fixture/(mode+'-process')
                e.write_json(out/'failed-child.json',dict(mode=mode,process=process,
                    stdout=(logs/'stdout.txt').read_text()[:16384],
                    stderr=(logs/'stderr.txt').read_text()[:16384]))
                raise AssertionError('synthetic writer child exited unexpectedly; failed-child.json retained')
            receipt=_read(context,root)
            if mode=='ordinary':
                start=time.monotonic(); catalog=_catalog(context,root,receipt,[])
                summary=_read(context,root,catalog)['classification']
                assert summary['classified_corpus_accepted'] and summary['raw_case_count']==555
                assert summary['raw_passed_count']==555 and summary['classified_case_count']==0
                rows.append(dict(id='full-555-original-writer-reader',passed=True,seconds=time.monotonic()-start))
            else:
                start=time.monotonic()
                detailed=json.loads((root/'stable-suite-run.json').read_text())
                rules=[]
                for index,category in enumerate(('upstream-invalid-test','explicit-optional-capability','proven-version-evolution')):
                    case=detailed['cases'][index];entry=context.entries[index]
                    rule=dict(profile=context.profile['id'],file=case['file'],id=case['id'],category=category,
                        reason='Synthetic contract case, not an original test acceptance claim',
                        input_sha256=entry['sha256'],source_evidence=[_anchor(context.source/entry['path'])],
                        observations={label:accounting.observation_signature(case[label]) for label in ('reference','candidate')})
                    if index==1:rule['optional_gate']=dict(feature='synthetic-optional',missing_on=['candidate'])
                    if index==2:
                        control_source=fixture/'same-version-test.el';control_source.write_text(';; synthetic changed-version test\n')
                        rule['source_evidence'].append(_anchor(control_source))
                        control=json.loads((root/'2-candidate.json').read_text())
                        observed=control['cases'][case['id']];observed.update(status='passed',diagnostic='synthetic pass')
                        observed['assessment']=suite.assess_case(case['id'],observed)
                        control.update(passed=True,working_fixture=dict(canonical_fixture_inputs_unchanged=True,
                            files=[dict(path=case['file'],before=accounting.bound_file(control_source))]))
                        control_path=fixture/'version-control.json';e.write_json(control_path,control)
                        rule['version_control']=dict(path=str(control_path),identity=accounting.bound_file(control_path),
                            test_input=dict(path=str(control_source),identity=accounting.bound_file(control_source)))
                    rules.append(rule)
                catalog=_catalog(context,root,receipt,rules)
                summary=_read(context,root,catalog)['classification']
                assert summary['classified_corpus_accepted'] and summary['classified_case_count']==3
                assert summary['raw_passed_count']==552 and summary['raw_failure_count']==3
                assert _read(context,root)['corpus_complete'] is False
                rows.append(dict(id='three-evidence-bound-categories-preserve-raw-outcomes',passed=True,seconds=time.monotonic()-start))
                start=time.monotonic(); missing=_catalog(context,root,receipt,rules[:2])
                blocked=_read(context,root,missing)['classification']
                assert not blocked['classified_corpus_accepted'] and blocked['blocking_case_failure_count']==1
                rows.append(dict(id='unexplained-case-remains-blocking',passed=True,seconds=time.monotonic()-start))
                start=time.monotonic()
                stage_result=fixture/'stage-results.json'
                raw_row=dict(id='stable-'+context.profile['id'],receipt=str(root/'stable-suite-run.json'),
                    receipt_sha256=e.digest(root/'stable-suite-run.json'),completion_sha256=e.digest(root/'stable-suite-completion.json'),
                    execution_complete=True,passed=False,process=process)
                e.write_json(stage_result,dict(schema='femacs-runtime-stage-v1',action='execute',execution_phase='stable',
                    execution_complete=True,product_identity_unchanged=True,complete_receipts_valid=True,
                    build_receipt=str(context.original),reference_selection=context.selection,cases=[raw_row],passed=False))
                build=dict(source_export=dict(synthetic=True))
                def admission(options):
                    options._reference_selection_summary=context.selection
                    return context.original,context.source,fixture/'products.json',context.products,build
                args=SimpleNamespace(classification=catalog,reference_selection=fixture/'selection.json',
                    build=context.original,run_results=[stage_result],out=out/'closed-stage')
                with mock.patch.object(stage,'read_action_build',side_effect=admission), \
                     mock.patch.object(stage,'verify_reference_selection',return_value=None), \
                     mock.patch.object(stage,'build_source_owner',return_value=context.original), \
                     mock.patch.object(e,'load_products',return_value=context.products), \
                     mock.patch.object(e,'verify_source_export',return_value=context.source):
                    assert accounting.run(stage,args)==0
                result=json.loads((args.out/'results.json').read_text())
                assert result['passed'] and len(result['remaining_profiles'])==47
                assert result['classified_stable_phase_complete'] is False
                assert result['cases'][0]['raw_result']['passed'] is False
                assert not result['compatibility_accepted'] and not result['goal_complete']
                assert all(result['checks'].values())
                rows.append(dict(id='closed-stage-61-identities-and-48-scope',passed=True,seconds=time.monotonic()-start,
                    whole_product_admission_isolated=True))
        return dict(passed=all(row['passed'] for row in rows),cases=rows,actual_python_children=2,
            original_files_per_synthetic_run=555,synthetic_case_observations=1110,
            actual_editor_execution=False,original_corpus_execution=False,seconds=time.monotonic()-started,
            goal_complete=False)
    finally:
        shutil.rmtree(fixture)


def protection(out, functional):
    accepted=e.bounded_json(functional)
    assert accepted['passed'] and accepted['actual_editor_execution'] is False
    fixture=out/'owned-fixtures';started=time.monotonic();rows=[]
    try:
        context=_fixture(fixture);root=fixture/'raw'
        assert child(root,context.manifest,context.program,context.dump,True)==1
        base=json.loads((root/'stable-suite-run.json').read_text())
        entry=context.entries[0];case=base['cases'][0]
        rule=dict(profile=context.profile['id'],file=case['file'],id=case['id'],
            category='upstream-invalid-test',reason='Synthetic negative contract',
            input_sha256=entry['sha256'],source_evidence=[_anchor(context.source/entry['path'])],
            observations={label:accounting.observation_signature(case[label]) for label in ('reference','candidate')})

        def check(name, operation, rejected=True):
            start=time.monotonic()
            try: result=operation()
            except (ValueError,suite.SuiteError,TypeError,KeyError):
                assert rejected, name
            else:
                assert not rejected and result is True, name
            rows.append(dict(id=name,passed=True,seconds=time.monotonic()-start))

        def exercise(*, report_change=None, catalog_change=None, chosen=None, result_check=None):
            report=copy.deepcopy(base)
            if report_change:report_change(report)
            suite._write_run_report(root,report)
            raw=_read(context,root)
            catalog=_catalog(context,root,raw,copy.deepcopy([rule] if chosen is None else chosen))
            if catalog_change:
                value=json.loads(catalog.read_text());catalog_change(value);e.write_json(catalog,value)
            value=_read(context,root,catalog)['classification']
            return result_check(value) if result_check else value

        check('wrong-reference-epoch',lambda:exercise(catalog_change=lambda c:c.update(reference_selection_sha256='f'*64)))
        check('wrong-product-context',lambda:exercise(catalog_change=lambda c:c['contexts'][context.profile['id']].update(context_sha256='f'*64)))
        check('wrong-raw-run',lambda:exercise(catalog_change=lambda c:c['contexts'][context.profile['id']].update(run_sha256='f'*64)))
        check('duplicate-rule',lambda:exercise(chosen=[rule,rule]))
        check('unregistered-case-rule',lambda:exercise(catalog_change=lambda c:c['rules'][0].update(id='not-discovered')))
        check('observation-signature-mismatch',lambda:exercise(catalog_change=lambda c:c['rules'][0]['observations']['candidate'].update(diagnostic_sha256='f'*64)))
        check('original-input-pin-mismatch',lambda:exercise(catalog_change=lambda c:c['rules'][0].update(input_sha256='f'*64)))
        check('source-anchor-mismatch',lambda:exercise(catalog_change=lambda c:c['rules'][0]['source_evidence'][0].update(lf_sha256='f'*64)))
        check('classification-cannot-hide-source-context-change',lambda:exercise(
            report_change=lambda r:r.update(source_context_unchanged=False),
            result_check=lambda s:not s['classified_corpus_accepted'] and s['structural_failure_count']>0),False)
        check('classification-cannot-hide-incomplete-file',lambda:exercise(
            report_change=lambda r:r['files'][0]['candidate'].update(complete=False,timed_out=True),
            result_check=lambda s:not s['classified_corpus_accepted'] and s['structural_failure_count']>0),False)
        check('classification-cannot-hide-registration-difference',lambda:exercise(
            report_change=lambda r:r['files'][0]['registration'].update(passed=False),
            result_check=lambda s:not s['classified_corpus_accepted'] and s['structural_failure_count']>0),False)
        check('raw-pair-passing-flag-consistency',lambda:exercise(report_change=lambda r:r['cases'][0].update(passed=True)))
        check('unknown-structural-failure-remains-blocking',lambda:exercise(
            report_change=lambda r:r['failures'].append('case failed/unverified: invented'),
            result_check=lambda s:not s['classified_corpus_accepted'] and s['structural_failure_count']>0),False)
        check('duplicate-case-identity',lambda:exercise(report_change=lambda r:r['cases'].append(r['cases'][0])))
        check('missing-original-file',lambda:exercise(report_change=lambda r:r['files'].pop()))
        check('optional-category-cannot-excuse-failure',lambda:exercise(catalog_change=lambda c:c['rules'][0].update(
            category='explicit-optional-capability',optional_gate=dict(feature='optional',missing_on=['candidate']))))
        second=base['cases'][1];second_entry=context.entries[1]
        optional=dict(profile=context.profile['id'],file=second['file'],id=second['id'],
            category='explicit-optional-capability',reason='Synthetic optional boundary',
            input_sha256=second_entry['sha256'],source_evidence=[_anchor(context.source/second_entry['path'])],
            observations={label:accounting.observation_signature(second[label]) for label in ('reference','candidate')},
            optional_gate=dict(feature='threads',missing_on=['candidate']))
        check('required-threads-cannot-be-optional',lambda:exercise(chosen=[optional]))
        optional['optional_gate']['feature']='native-compile'
        native=dict(context.profile,native=True)
        check('declared-native-remains-required',lambda:accounting._accept_rule(optional,second,base['files'][1],
            second_entry,root,suite,context.selection,native))
        progress=root/'0-candidate.events.jsonl';old=progress.read_bytes();progress.write_bytes(old+b'changed')
        check('bound-progress-change-rejected',lambda:exercise())
        progress.write_bytes(old)
        source=context.source/entry['path'];link=fixture/'linked-source.el';link.symlink_to(source)
        check('source-symlink-rejected',lambda:accounting.bound_file(link))
        bad=fixture/'bad-json.json';bad.write_text('{"cases":[],"cases":[]}')
        check('duplicate-stream-key',lambda:list(accounting.detailed_rows(bad)))
        bad.write_text('{"cases":[]} trailing')
        check('trailing-stream-content',lambda:list(accounting.detailed_rows(bad)))
        bad.write_text(json.dumps(dict(cases=[dict(text='escaped \\" '+('x'*70000))])))
        check('stream-cross-chunk-string-preserved',lambda:list(accounting.detailed_rows(bad))[0][1]['text'].endswith('x'*70000),False)
        with mock.patch.object(accounting,'MAX_ITEM',128):
            check('oversized-stream-item-rejected',lambda:list(accounting.detailed_rows(bad)))
        return dict(passed=all(row['passed'] for row in rows),cases=rows,
            functional_acceptance_sha256=e.digest(functional),actual_editor_execution=False,
            seconds=time.monotonic()-started,goal_complete=False)
    finally:
        shutil.rmtree(fixture)


def main():
    parser=argparse.ArgumentParser()
    parser.add_argument('--child',type=Path)
    parser.add_argument('--manifest',type=Path)
    parser.add_argument('--program',type=Path)
    parser.add_argument('--dump',type=Path)
    parser.add_argument('--failed',action='store_true')
    parser.add_argument('--mode',choices=('integration','protection'),default='integration')
    parser.add_argument('--functional',type=Path)
    parser.add_argument('--out',type=Path)
    args=parser.parse_args()
    if args.child:return child(args.child,args.manifest,args.program,args.dump,args.failed)
    out=e.new_output(args.out)
    result=integration(out) if args.mode=='integration' else protection(out,args.functional)
    result['owned_fixture_retired']=not (out/'owned-fixtures').exists()
    e.write_json(out/'results.json',result)
    return 0 if result['passed'] else 1


if __name__=='__main__':raise SystemExit(main())
