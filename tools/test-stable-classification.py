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
APPLICABILITY_FLOWS = (
    ("(skip-unless (featurep 'synthetic-optional))", '(featurep synthetic-optional)',
        dict(kind='feature-absent', feature='synthetic-optional'), dict(features=[])),
    ("(skip-when (featurep 'threads))", '(featurep threads)',
        dict(kind='feature-present', feature='threads'), dict(features=['threads'])),
    ('(skip-unless (native-comp-available-p))', '(native-comp-available-p)',
        dict(kind='native-compiler-absent'), dict(native_comp_available=False)),
    ("(skip-unless (string-match-p \"SECCOMP\" system-configuration-features))",
        '(string-match-p "SECCOMP" "NS THREADS SQLITE")',
        dict(kind='configuration-token-absent', token='SECCOMP'),
        dict(configuration_features='NS THREADS SQLITE')),
    ("(skip-unless (treesit-language-available-p 'python))", '(treesit-language-available-p python)',
        dict(kind='external-predicate', dependency_type='grammar', predicate='treesit-language-available-p'),
        dict(features=['treesit'])),
    ('(skip-unless tramp-archive-enabled)', 'tramp-archive-enabled',
        dict(kind='external-predicate', dependency_type='service', predicate='tramp-archive-enabled'),
        dict(features=[])),
    ("(skip-unless (image-type-available-p 'png))", '(image-type-available-p png)',
        dict(kind='external-predicate', dependency_type='optional-api', predicate='image-type-available-p'),
        dict(features=['ns'])),
    ("(skip-unless (eq system-type 'windows-nt))", '(eq darwin windows-nt)',
        dict(kind='platform', applicable_systems=['windows-nt']), dict(system_type='darwin')),
    ("(skip-when (memq system-type '(darwin)))", '(memq darwin (darwin))',
        dict(kind='platform', applicable_systems=['windows-nt'], condition_values=dict(candidate='(darwin)')),
        dict(system_type='darwin')),
    ('(skip-unless (< emacs-major-version 29))', '(< 32 29)',
        dict(kind='version-scope', applicable_major_versions=[27, 28]), dict(product_version='32.0.50')),
    ('(skip-unless (file-readable-p "emacs.info"))', '(file-readable-p "emacs.info")',
        dict(kind='host-facility', facility='info-manual', predicate='file-readable-p'), dict(features=[])),
    ('(skip-unless (display-graphic-p))', '(display-graphic-p)',
        dict(kind='display-context', applicable_display='cocoa'),
        dict(display_mode='batch', noninteractive=True, display_graphic=False)),
    ('(skip-unless dbus--test-enabled-session-bus)', 'dbus--test-enabled-session-bus',
        dict(kind='external-predicate', dependency_type='service', predicate='dbus--test-enabled-session-bus'),
        dict(features=[])),
    ("(skip-unless (not (eq system-type 'darwin)))", '(not t)',
        dict(kind='platform', applicable_systems=['gnu/linux']), dict(system_type='darwin')),
    ("(skip-unless (not (equal (file-selinux-context temporary-file-directory) '(nil nil nil nil))))", '(not t)',
        dict(kind='host-facility', facility='selinux', predicate='file-selinux-context'), dict(features=[])),
    ('(skip-unless (and (file-exists-p "/usr/sbin/slapd") (file-exists-p "/usr/bin/ldapsearch")))',
        '(and (file-exists-p "/usr/sbin/slapd") (file-exists-p "/usr/bin/ldapsearch"))',
        dict(kind='external-predicate', dependency_type='executable', predicate='file-exists-p'), dict(features=[])),
    ("(skip-unless (functionp 'markdown-mode))", '(functionp markdown-mode)',
        dict(kind='external-predicate', dependency_type='package', predicate='markdown-mode'), dict(features=[])),
)


def child(root, manifest_path, program, dump, failure_mode, *, fixture_repo=REPO):
    manifest = suite.load_manifest(manifest_path, fixture_repo)
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
            if index == 1 and status == 'skipped':
                event['condition'] = "(ert-test-skipped ((skip-unless (featurep 'synthetic-optional)) :form (featurep synthetic-optional) :value nil))"
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
                runtime_environment=dict(envs[label]),
                metadata=dict(features=[], emacs_version='Synthetic full version description', product_version='32.0.50'),
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
        raw=(';; synthetic unchanged input '+str(index)
             + (' '+' '.join(flow[0] for flow in APPLICABILITY_FLOWS) if index == 1 else '')
             + (' (skip-unless nil) (skip-unless (synthetic-unreachable-gate))' if index == 0 else '')+'\n').encode()
        path.write_bytes(raw)
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


def _applicability_flows(context, root, rules, rows):
    """Exercise real files/writer/reader over explicit synthetic gate facts."""
    report=json.loads((root/'stable-suite-run.json').read_text())
    paths=[root/('1-'+label+suffix) for label in ('reference','candidate')
           for suffix in ('.json','.events.jsonl')]
    saved={path:path.read_bytes() for path in paths}
    try:
        for form, observed_form, gate, facts in APPLICABILITY_FLOWS:
            started=time.monotonic();current=copy.deepcopy(report)
            case=current['cases'][1]
            both=(gate['kind'] in {'feature-present','platform','version-scope','host-facility'}
                  or (gate['kind']=='external-predicate' and gate['dependency_type']!='optional-api'))
            labels=['reference','candidate'] if both else ['candidate']
            condition='(ert-test-skipped ('+form+' :form '+observed_form+' :value '+gate.get(
                'condition_values',{}).get('candidate','t' if form.startswith('(skip-when ') else 'nil')+'))'
            for label in labels:
                final=root/('1-'+label+'.json');progress=root/('1-'+label+'.events.jsonl')
                file_row=current['files'][1][label];value=json.loads(saved[final])
                value['metadata'].update(facts);file_row['metadata']=value['metadata']
                case[label].update(status='skipped',diagnostic='synthetic skip',complete=True,exit_code=0)
                case[label]['assessment']=suite.assess_case(case['id'],case[label])
                event=dict(event='test-ended',file=case['file'],test_id=case['id'],status='skipped',
                           diagnostic=case[label]['diagnostic'],condition=condition)
                raw=json.dumps(event).encode();progress.write_bytes(raw+b'\n')
                proof=dict(path=progress.name,event_sha256=hashlib.sha256(raw).hexdigest(),event_ordinal=0)
                case[label]['evidence']=proof;value['cases'][case['id']]=case[label]
                value['passed']=False;file_row['passed']=False
                value['progress']=dict(path=progress.name,**accounting.bound_file(progress))
                file_row['progress']=value['progress'];e.write_json(final,value)
                file_row['file_result']=dict(path=final.name,**accounting.bound_file(final))
            # Restore any side unused in this flow after a preceding symmetric
            # scenario, so its exact raw per-file pin still matches the report.
            for label in set(('reference','candidate'))-set(labels):
                for suffix in ('.json','.events.jsonl'):
                    path=root/('1-'+label+suffix);path.write_bytes(saved[path])
            suite._write_run_report(root,current)
            receipt=_read(context,root);chosen=copy.deepcopy(rules)
            chosen[1]['observations']={label:accounting.observation_signature(case[label])
                                      for label in ('reference','candidate')}
            static_gate={key:value for key,value in gate.items() if key != 'condition_values'}
            chosen[1]['optional_gate']=accounting._capture_optional_gate(
                dict(static_gate,source_form=form),root,suite,case)
            assert chosen[1]['optional_gate']['missing_on']==labels
            assert chosen[1]['optional_gate']['condition_forms']=={label:observed_form for label in labels}
            assert chosen[1]['optional_gate']['condition_values']=={label:gate.get(
                'condition_values',{}).get('candidate','t' if form.startswith('(skip-when ') else 'nil') for label in labels}
            summary=_read(context,root,_catalog(context,root,receipt,chosen))['classification']
            assert summary['classified_corpus_accepted'] and summary['classified_case_count']==3
            assert summary['raw_passed_count']==552 and summary['raw_failure_count']==3
            rows.append(dict(id='typed-original-gate-'+gate['kind']+'-'+str(len(rows)),passed=True,
                             seconds=time.monotonic()-started,synthetic_observed_facts=True))
    finally:
        for path,raw in saved.items():path.write_bytes(raw)
        suite._write_run_report(root,report)



def _invalid_skip_flows(context, root, rules, rows):
    """Preserve both skipped states while binding their exact source/error gates."""
    report=json.loads((root/'stable-suite-run.json').read_text())
    paths=[root/('0-'+label+suffix) for label in ('reference','candidate')
           for suffix in ('.json','.events.jsonl')]
    saved={path:path.read_bytes() for path in paths}
    helper=context.root/'invalid-helper.el'
    helper.write_text('(defun synthetic-unreachable-gate () (error "Unsupported version: synthetic"))\n')
    try:
        for kind,form in (('unconditional-skip','(skip-unless nil)'),
                          ('unreachable-gate','(skip-unless (synthetic-unreachable-gate))')):
            started=time.monotonic();current=copy.deepcopy(report);case=current['cases'][0]
            condition=('(ert-test-skipped ('+form+' :form nil :value nil))' if kind=='unconditional-skip' else
                       '(ert-test-skipped ('+form+' :form (synthetic-unreachable-gate) :condition (error "Unsupported version: synthetic")))')
            for label in ('reference','candidate'):
                final=root/('0-'+label+'.json');progress=root/('0-'+label+'.events.jsonl')
                value=json.loads(saved[final]);file_row=current['files'][0][label]
                case[label].update(status='skipped',diagnostic='synthetic invalid source gate',complete=True,exit_code=0)
                case[label]['assessment']=suite.assess_case(case['id'],case[label])
                event=dict(event='test-ended',file=case['file'],test_id=case['id'],status='skipped',
                           diagnostic=case[label]['diagnostic'],condition=condition)
                raw=json.dumps(event).encode();progress.write_bytes(raw+b'\n')
                case[label]['evidence']=dict(path=progress.name,event_sha256=hashlib.sha256(raw).hexdigest(),event_ordinal=0)
                value['cases'][case['id']]=case[label];value['passed']=False;file_row['passed']=False
                value['progress']=dict(path=progress.name,**accounting.bound_file(progress))
                file_row['progress']=value['progress'];e.write_json(final,value)
                file_row['file_result']=dict(path=final.name,**accounting.bound_file(final))
            suite._write_run_report(root,current);receipt=_read(context,root);chosen=copy.deepcopy(rules)
            chosen[0]['observations']={label:accounting.observation_signature(case[label]) for label in ('reference','candidate')}
            chosen[0]['invalid_gate']=dict(kind=kind,source_form=form)
            if kind=='unreachable-gate':
                chosen[0]['source_evidence'].append(_anchor(helper))
                chosen[0]['invalid_gate']['conditions']={label:accounting._original_case_event(
                    root,suite,case,label)['condition'] for label in ('reference','candidate')}
            summary=_read(context,root,_catalog(context,root,receipt,chosen))['classification']
            assert summary['classified_corpus_accepted'] and summary['classified_case_count']==3
            assert summary['raw_passed_count']==552 and summary['raw_failure_count']==3
            rows.append(dict(id='source-invalid-skip-'+kind,passed=True,
                seconds=time.monotonic()-started,synthetic_observed_facts=True))
    finally:
        for path,raw in saved.items():path.write_bytes(raw)
        suite._write_run_report(root,report)


def _removed_producer_flows(fixture, rows):
    """Bind the obsolete eieio assertion to the commit that deleted its producer."""
    started = time.monotonic()
    logical = 'test/lisp/emacs-lisp/eieio-tests/eieio-tests.el'
    original = REPO/logical
    entry = dict(path=str(original), size=original.stat().st_size,
                 sha256=hashlib.sha256(original.read_bytes()).hexdigest())
    lines = original.read_bytes().replace(b'\r\n', b'\n').split(b'\n')

    def anchor(start, end):
        excerpt = b'\n'.join(lines[start-1:end])+b'\n'
        return dict(path=str(original), bytes=entry['size'], sha256=entry['sha256'],
                    start_line=start, end_line=end,
                    lf_sha256=hashlib.sha256(excerpt).hexdigest())

    obsolete, sibling = 'eieio-test-01-mix-alloc-initarg', 'eieio-test-20-class-allocated-slots'
    skip_form = '(skip-unless (and (message "foo") (string= "foo" (current-message))))'
    assertion = ('(should (string-match "Class allocated slots do not need :initarg"\n'
                 '                        (current-message)))')
    failed_condition = ('(ert-test-failed ((should (string-match "Class allocated slots do not need :initarg" '
                        '(current-message))) :form (string-match "Class allocated slots do not need :initarg" '
                        '"foo") :value nil))')
    skipped_condition = ('(ert-test-skipped ('+skip_form+' :form (and (message "foo") '
                         '(string= "foo" (current-message))) :value nil))')

    def manifestation(name, status, display_mode, noninteractive, condition):
        root = fixture/('removed-producer-'+name)
        root.mkdir()
        case, file_row = dict(file=logical, id=obsolete, passed=False), {}
        for label in ('reference', 'candidate'):
            pins = {}
            for test_id, outcome, cond in ((obsolete, status, condition),
                                           (sibling, 'passed', None)):
                event = dict(event='test-ended', file=logical, test_id=test_id,
                             status=outcome, duration=.01, diagnostic='synthetic '+outcome,
                             expected=outcome == 'passed', skipped=False)
                if cond is not None:
                    event['condition'] = cond
                raw = json.dumps(event, sort_keys=True).encode()
                path = root/(label+'-'+test_id+'.events.jsonl')
                path.write_bytes(raw+b'\n')
                pin = dict(path=path.name, event_sha256=hashlib.sha256(raw).hexdigest(),
                           event_ordinal=0)
                observed = dict(test_id=test_id, status=outcome, complete=True, exit_code=0,
                                seconds=.01, diagnostic='synthetic '+outcome, evidence=pin)
                observed['assessment'] = suite.assess_case(test_id, observed)
                pins[test_id] = observed
            progress = dict(path=pins[obsolete]['evidence']['path'],
                            **accounting.bound_file(root/pins[obsolete]['evidence']['path']))
            final = root/(label+'-file.json')
            value = dict(schema='femacs-stable-file-batch-v1', file=logical, product=label,
                         ids=[obsolete, sibling], cases=pins, complete=True, passed=False,
                         timed_out=False, remaining_group_terminated=False, exit_code=0,
                         argv=['synthetic', '--dump-file=synthetic'],
                         runtime_environment={'FEMACS_GC_OBSERVATION': 'off'},
                         metadata=dict(display_mode=display_mode,
                                       noninteractive=noninteractive,
                                       product_version='32.0.50'),
                         progress=progress)
            e.write_json(final, value)
            file_row[label] = dict(value, file_result=dict(path=final.name,
                                                           **accounting.bound_file(final)))
            file_row[label].pop('cases')
            file_row[label].pop('ids')
            case[label] = pins[obsolete]
        rule = dict(category='upstream-invalid-test',
                    reason='synthetic removed-producer binding; no real case is accepted here',
                    input_sha256=entry['sha256'],
                    source_evidence=[anchor(150, 165), anchor(511, 530)],
                    observations={label: accounting.observation_signature(case[label])
                                  for label in ('reference', 'candidate')},
                    invalid_gate=dict(kind='removed-upstream-producer',
                        applicable_display='terminal', skip_form=skip_form,
                        source_form=assertion,
                        removed_producer_form='(message "Class allocated slots do not need :initarg")',
                        producer_path=str(REPO/'lisp/emacs-lisp/eieio-core.el'),
                        upstream_commit='4c6554413d318069239ba83f4f42fa2452801d30',
                        control_test_id=sibling,
                        conditions={label: condition for label in ('reference', 'candidate')}))

        def exercise(rule_change=None, case_change=None, file_change=None):
            chosen, chosen_case = copy.deepcopy(rule), copy.deepcopy(case)
            rows_value = copy.deepcopy(file_row)
            if rule_change:
                rule_change(chosen)
            if case_change:
                case_change(chosen_case)
            if file_change:
                file_change(rows_value)
            return accounting._accept_rule(chosen, chosen_case, rows_value, entry,
                root, suite, None, None, None) and True

        def check(name, operation, rejected=True):
            start = time.monotonic()
            try:
                result = operation()
            except (ValueError, suite.SuiteError, TypeError, KeyError):
                assert rejected, name
            else:
                assert not rejected and result is True, name
            rows.append(dict(id=name, passed=True, seconds=time.monotonic()-start,
                             synthetic_observed_facts=True))

        def failing_sibling():
            saved = {}
            try:
                for label in ('reference', 'candidate'):
                    final = root/(label+'-file.json')
                    event = root/(label+'-'+sibling+'.events.jsonl')
                    saved[final] = final.read_bytes()
                    saved[event] = event.read_bytes()
                    raw = json.dumps(dict(event='test-ended', file=logical, test_id=sibling,
                                          status='failed', duration=.01,
                                          diagnostic='synthetic failed', expected=False,
                                          skipped=False), sort_keys=True).encode()
                    event.write_bytes(raw+b'\n')
                    value = json.loads(saved[final])
                    value['cases'][sibling].update(status='failed', diagnostic='synthetic failed',
                        assessment=suite.assess_case(sibling, dict(value['cases'][sibling],
                                                                   status='failed')),
                        evidence=dict(path=event.name,
                                      event_sha256=hashlib.sha256(raw).hexdigest(), event_ordinal=0))
                    e.write_json(final, value)
                chosen_case = copy.deepcopy(case)
                rows_value = copy.deepcopy(file_row)
                for label in ('reference', 'candidate'):
                    rows_value[label]['file_result'] = dict(
                        path=label+'-file.json', **accounting.bound_file(root/(label+'-file.json')))
                return accounting._accept_rule(copy.deepcopy(rule), chosen_case, rows_value, entry,
                    root, suite, None, None, None)
            finally:
                for path, raw in saved.items():
                    path.write_bytes(raw)
        return check, exercise, skipped_condition, failing_sibling

    check, exercise, _, failing_sibling = manifestation('interactive', 'failed', 'terminal', False, failed_condition)
    check('removed-producer-interactive-binding-accepted', lambda: exercise(), False)
    producer = fixture/'producer-with-message.el'
    producer.write_text('(message "Class allocated slots do not need :initarg")\n')
    check('removed-producer-present-producer-rejected', lambda: exercise(
        lambda rule: rule['invalid_gate'].update(producer_path=str(producer))))
    check('removed-producer-short-commit-rejected', lambda: exercise(
        lambda rule: rule['invalid_gate'].update(upstream_commit='4c6554413d3')))
    check('removed-producer-nonliteral-assertion-rejected', lambda: exercise(
        lambda rule: rule['invalid_gate'].update(
            source_form='(should (string-match "fabricated producer message" (current-message)))')))
    check('removed-producer-batch-context-rejected', lambda: exercise(file_change=lambda value: [
        row['metadata'].update(display_mode='batch') for row in value.values()]))
    check('removed-producer-differing-condition-rejected', lambda: exercise(
        lambda rule: rule['invalid_gate']['conditions'].update(
            candidate='(ert-test-failed ((should nil)))')))
    check('removed-producer-failing-sibling-rejected', failing_sibling)

    batch_check, batch_exercise, _, _ = manifestation('batch', 'skipped', 'batch', True,
                                                   skipped_condition)
    batch_check('removed-producer-batch-skip-binding-accepted', lambda: batch_exercise(), False)
    batch_check('removed-producer-batch-skip-without-exact-condition-rejected',
        lambda: batch_exercise(lambda rule: rule['invalid_gate'].update(
            skip_form='(skip-unless nil)')), True)


def _portable_rule_flow(context, root, raw_stage, fixture, rows, version_rule):
    """Expand relocatable source definitions and bind only fresh non-passes."""
    started = time.monotonic()
    definitions = []
    for index in range(4):
        entry = context.entries[index]
        evidence = _anchor(context.source/entry['source_path'])
        evidence['path'] = entry['source_path']
        definition = dict(file=entry['path'], id='synthetic-case-'+str(index),
            category='upstream-invalid-test', input_sha256=entry['sha256'],
            reason='Synthetic source definition for the portable assembly integration; no real test acceptance.',
            source_evidence=[evidence])
        if index == 1:
            definition.update(category='explicit-optional-capability', optional_gate=dict(
                kind='feature-absent', feature='synthetic-optional',
                source_form="(skip-unless (featurep 'synthetic-optional))"))
        if index == 2:
            control_input = Path(version_rule['version_control']['test_input']['path'])
            relative = 'control-sources/current-version-case.el'
            helper = context.source/relative
            helper.parent.mkdir()
            shutil.copy2(control_input, helper)
            evidence = _anchor(helper)
            evidence['path'] = relative
            definition['category'] = 'proven-version-evolution'
            definition['source_evidence'].append(evidence)
        definitions.append(definition)
    spec = fixture/'portable-source-rules.json'
    e.write_json(spec, dict(schema='femacs-stable-source-rule-specs-v1',
        profiles=[p['id'] for p in context.products if p['kind'] != 'reference' and not p['test_hooks']],
        rules=definitions))
    expanded = fixture/'expanded-source-rules'
    process = e.run_process([sys.executable, REPO/'tools/stable-source-rules.py',
        '--spec', spec, '--repo', context.source, '--out', expanded],
        fixture/'source-expansion-process', dict(os.environ), 20, cwd=REPO)
    if process['exit_code'] != 0:
        e.write_json(fixture.parent/'failed-source-expansion.json', dict(process=process,
            stderr=(fixture/'source-expansion-process/stderr.txt').read_text()[:16384]))
    assert process['complete'] and process['exit_code'] == 0 and not process['remaining_group_terminated']
    expansion = e.bounded_json(expanded/'results.json')
    assert expansion['passed'] and expansion['profiles'] == 48 and expansion['source_rules'] == 4
    assert expansion['expanded_rules'] == 192 and expansion['runtime_observations_bound'] is False
    raw_inputs = {str(p): accounting.bound_file(p) for p in
                  (raw_stage, root/'stable-suite-run.json', root/'stable-suite-completion.json')}
    def admission(options):
        options._reference_selection_summary = context.selection
        return context.original, context.source, fixture/'products.json', context.products, dict(
            source_export=dict(synthetic=True))
    out = fixture/'assembled-source-rules'
    args = SimpleNamespace(approved_rules=expanded/'reviewed-rule-definitions.json',
        reference_selection=fixture/'selection.json', build=context.original,
        run_results=[raw_stage], out=out, control_run=[[
            Path(version_rule['version_control']['closed_run']['path']),
            Path(version_rule['version_control']['controller']['path'])]])
    with mock.patch.object(stage, 'REPO', context.source), \
         mock.patch.object(stage, 'read_action_build', side_effect=admission), \
         mock.patch.object(stage, 'verify_reference_selection', return_value=None), \
         mock.patch.object(stage, 'build_source_owner', return_value=context.original), \
         mock.patch.object(e, 'load_products', return_value=context.products), \
         mock.patch.object(e, 'verify_source_export', return_value=context.source):
        assert accounting.assemble(stage, args) == 0
    result = e.bounded_json(out/'results.json')
    assert result['passed'] and result['bound_rule_count'] == 3 and result['bound_profile_count'] == 1
    assert result['ordinary_rule_not_applied_count'] == 1 and result['unexecuted_profile_rule_count'] == 188
    assert result['automatic_categories_added'] == 0 and not result['raw_results_modified']
    catalog = e.bounded_json(out/'classification.json')
    assert len(catalog['rules']) == 3
    assert catalog['rules'][2]['version_control']['test_id'] == 'synthetic-case-2'
    assert catalog['rules'][2]['version_control']['identity'] == version_rule['version_control']['identity']
    assert all(Path(a['path']).is_absolute() for r in catalog['rules'] for a in r['source_evidence'])
    accepted = e.bounded_json(out/'accounting/results.json')
    assert accepted['passed'] and not accepted['classified_stable_phase_complete']
    assert accepted['cases'][0]['raw_passed_count'] == 552
    assert accepted['cases'][0]['raw_failure_count'] == 3
    assert all(accounting.bound_file(path) == pin for path, pin in raw_inputs.items())
    rows.append(dict(id='portable-source-expansion-to-fresh-closed-accounting', passed=True,
        seconds=time.monotonic()-started, source_definitions=4, expanded_definitions=192,
        bound_rules=3, ordinary_rule_not_applied=1, unexecuted_profile_rules=188,
        raw_inputs_unchanged=True, actual_editor_execution=False))


def _native_positive_flows(fixture, rows):
    """Exercise native-off accounting through fresh original-protocol files."""
    context = _fixture(fixture/'native-positive-fixture')
    logical = 'test/src/comp-tests.el'
    source = context.source/logical
    source.parent.mkdir(parents=True, exist_ok=True)
    prelude = ('(when (native-comp-available-p) (load (native-compile comp-test-src))) '
        '"Define a test for the native compiler" '
        '((null (featurep \'native-compile)) (error "Unavailable native compiler")) '
        '(subr-type (symbol-function f)) (native-comp-function-p f) '
        '"Verify string hash consing strategy." '
        '(should (eq (comp-test-47868-1-f) (comp-test-47868-3-f))) '
        '(format "ret-type-spec-%d" number) (native-compile f-name) for n from 1 ')
    flows = [
        ('compiler-unavailable', 'comp-tests-unavailable', '(comp-deftest unavailable ()',
         '(error "Unavailable native compiler")'),
        ('native-function-assertion', 'comp-tests-function', '(comp-deftest function ()',
         '(ert-test-failed ((should (native-comp-function-p f)) :form (native-comp-function-p nil) :value nil))'),
        ('native-subr-assertion', 'comp-tests-subr', '(comp-deftest subr ()',
         '(wrong-type-argument subrp #[nil (nil) (t)])'),
        ('native-string-sharing', 'comp-tests-sharing', '(comp-deftest sharing ()',
         '(ert-test-failed ((should (eq (comp-test-47868-1-f) (comp-test-47868-3-f))) :form (eq " " " ") :value nil))'),
        ('compiler-unavailable', 'comp-tests-ret-type-spec-1', '      ;; 1',
         '(error "Unavailable native compiler")'),
    ]
    source.write_text(prelude + ' '.join(flow[2] for flow in flows) + '\n')
    entry = context.entries[0]
    data = source.read_bytes()
    entry.update(path=logical, source_path=logical, size=len(data), sha256=e.digest(source),
        blob=hashlib.sha1(b'blob '+str(len(data)).encode()+b'\0'+data).hexdigest())
    context.entries.sort(key=lambda row: row['path'])
    manifest = e.bounded_json(context.manifest)
    manifest.update(entries=context.entries, total_bytes=sum(row['size'] for row in context.entries))
    e.write_json(context.manifest, manifest)
    root = context.root/'raw'
    assert child(root, context.manifest, context.program, context.dump, False) == 0
    base = e.bounded_json(root/'stable-suite-run.json')
    index = next(i for i, row in enumerate(base['files']) if row['file'] == logical)
    prefix = str(index)+'-'
    for kind, identity, declaration, condition in flows:
        started = time.monotonic()
        current = copy.deepcopy(base)
        case = current['cases'][index]
        case.update(id=identity, passed=False)
        for label in ('reference', 'candidate'):
            passed = label == 'reference'
            value = e.bounded_json(root/(prefix+label+'.json'))
            case[label].update(test_id=identity, status='passed' if passed else 'failed',
                diagnostic='synthetic native positive '+label, complete=True, exit_code=0)
            case[label]['assessment'] = suite.assess_case(identity, case[label])
            event = dict(event='test-ended', file=logical, test_id=identity,
                status=case[label]['status'], diagnostic=case[label]['diagnostic'],
                condition='nil' if passed else condition)
            raw = json.dumps(event).encode()
            progress = root/(prefix+label+'.events.jsonl')
            progress.write_bytes(raw+b'\n')
            case[label]['evidence'] = dict(path=progress.name,
                event_sha256=hashlib.sha256(raw).hexdigest(), event_ordinal=0)
            value.update(ids=[identity], cases={identity:case[label]}, passed=passed,
                metadata=dict(native_comp_available=passed,
                              features=['native-compile'] if passed else []),
                progress=dict(path=progress.name, **accounting.bound_file(progress)))
            final = root/(prefix+label+'.json')
            e.write_json(final, value)
            current['files'][index][label] = dict(
                **{k:v for k,v in value.items() if k not in ('cases','ids')},
                file_result=dict(path=final.name, **accounting.bound_file(final)))
        current.update(corpus_complete=False, failures=['case failed/unverified: '+logical+' :: '+identity])
        suite._write_run_report(root, current)
        raw_receipt = _read(context, root)
        definition = dict(kind='native-positive-test', source_form='(native-comp-available-p)',
                          source_declaration=declaration, failure_kind=kind)
        gate = accounting._capture_optional_gate(definition, root, suite, case)
        rule = dict(profile=context.profile['id'], file=logical, id=identity,
            category='explicit-optional-capability', input_sha256=entry['sha256'],
            reason='Synthetic native capability contract; no editor execution.',
            source_evidence=[_anchor(source)], optional_gate=gate,
            observations={s:accounting.observation_signature(case[s]) for s in ('reference','candidate')})
        summary = _read(context, root, _catalog(context, root, raw_receipt, [rule]))['classification']
        assert summary['classified_corpus_accepted'] and summary['classified_case_count'] == 1
        assert summary['raw_passed_count'] == 554 and summary['raw_failure_count'] == 1
        assert _read(context, root)['corpus_complete'] is False
        rows.append(dict(id='native-positive-'+kind+'-'+identity, passed=True,
            seconds=time.monotonic()-started, actual_editor_execution=False,
            raw_failed_state_preserved=True))


def _source_scope_flows(fixture, rows, *, prerequisites=False, protective_rows=None, late_gates=False):
    """Integrate typed gates with real synthetic file/event/receipt bindings."""
    started = time.monotonic()
    def source_anchor(path):
        raw = path.read_bytes().replace(b'\r\n', b'\n')
        lines = raw.split(b'\n')
        end = len(lines)-1 if not lines[-1] else len(lines)
        return dict(path=str(path), **accounting.bound_file(path), start_line=1, end_line=end,
            lf_sha256=hashlib.sha256(b'\n'.join(lines[:end])+b'\n').hexdigest())
    context = _fixture(fixture/('late-source-gates' if late_gates else
        'source-prerequisite-fixture' if prerequisites else 'source-scope-fixture'))
    formats = {
        'reference': '(ert--make-erts-file-test-description :name name)\n',
        'candidate': '(ert--signal-erts-file-test-execution `((name . ,name)))\n',
    }
    libraries, descriptor_libraries, environments = {}, {}, {}
    for label in formats:
        library = context.root/(label+'-lisp')
        (library/'emacs-lisp').mkdir(parents=True)
        path = library/'emacs-lisp/ert.el'
        path.write_text(formats[label])
        libraries[label] = path
        descriptor = library/'filenotify.el'
        descriptor.write_text('(defun file-notify-rm-watch (descriptor) (remhash descriptor file-notify-descriptors))\n'
            if label == 'reference' else '(defun file-notify--rm-descriptor (descriptor) nil)\n')
        descriptor_libraries[label] = descriptor
        environments[label] = dict(FEMACS_GC_OBSERVATION='off', FEMACS_STABLE_PRODUCT_LISP=str(library))
        if label == 'candidate':
            environments[label]['FEMACS_GC_COLLECTOR'] = 'major'
    monitor_form = ('(ert-skip (format "Monitor %s does not support auto-revert"\n'
                    '                        (auto-revert--test-monitor)))')
    monitor_id = 'auto-revert-test00-auto-revert-mode'
    package_id = 'package-vc-tests-pkg-spec-info-manual'
    package_form = "(skip-unless (memq (caddr (alist-get pkg (package-vc-tests-packages t))) '(package-vc-tests-install-from-elpa package-vc-tests-install-from-spec)))"
    flows = [
        ('test/lisp/emacs-lisp/ert-tests.el', 'ert-test-list-of-erts-file-tests',
         "(cdr (assq 'name desc))", 'proven-version-evolution',
         dict(kind='ert-description-format', source_form="(cdr (assq 'name desc))",
              upstream_commit=context.selection['upstream']['commit'], **{label+'_library':
                  dict(relative_path='emacs-lisp/ert.el', **{k:v for k,v in _anchor(path).items() if k != 'path'})
                  for label,path in libraries.items()}),
         ['failed', 'passed'], '(wrong-type-argument listp #s(ert--erts-file-test-description "foo" nil "fixture.erts" 16 nil))', None),
        ('test/lisp/filenotify-tests.el', 'file-notify-test07-many-events',
         '(file-notify--rm-descriptor file-notify--test-desc)', 'proven-version-evolution',
         dict(kind='filenotify-descriptor-api', source_form='(file-notify--rm-descriptor file-notify--test-desc)',
              upstream_commit=context.selection['upstream']['commit'], **{label+'_library':
                  dict(relative_path='filenotify.el', **{k:v for k,v in _anchor(path).items() if k != 'path'})
                  for label,path in descriptor_libraries.items()}),
         ['failed', 'passed'], '(void-function file-notify--rm-descriptor)', None),
        ('test/lisp/gnus/mml-sec-tests.el', 'mml-secure-en-decrypt-1',
         "(defun test-conf () (and (not (eq system-type 'darwin))\n       (ignore-errors (epg-find-configuration 'OpenPGP)))) (skip-unless (test-conf))",
         'explicit-optional-capability', dict(kind='platform-helper', source_form='(skip-unless (test-conf))',
              source_helper="(and (not (eq system-type 'darwin))\n       (ignore-errors (epg-find-configuration 'OpenPGP)))"),
         ['skipped', 'skipped'], '(ert-test-skipped ((skip-unless (test-conf)) :form (test-conf) :value nil))', None),
        ('test/lisp/emacs-lisp/bytecomp-tests.el', 'bytecomp-tests--dest-mountpoint',
         '(bwrap (executable-find "bwrap")) (skip-unless bwrap)', 'explicit-optional-capability',
         dict(kind='external-lexical-binding', source_form='(skip-unless bwrap)',
              source_binding='(bwrap (executable-find "bwrap"))'),
         ['skipped', 'skipped'], '(ert-test-skipped ((skip-unless bwrap) :form bwrap :value nil))', None),
        ('test/lisp/autorevert-tests.el', monitor_id+'-remote',
         monitor_form+" (when (eq (auto-revert--test-monitor) 'GKqueueFileMonitor) "
            '(defmacro auto-revert--deftest-remote (test docstring) (funcall (ert-test-body ert-test))',
         'explicit-optional-capability', dict(kind='file-monitor-backend', source_form=monitor_form,
              backend='GKqueueFileMonitor', control_test_id=monitor_id),
         ['skipped', 'skipped'], '(ert-test-skipped "Monitor GKqueueFileMonitor does not support auto-revert")', monitor_id),
        ('test/lisp/emacs-lisp/package-vc-tests.el', package_id+'/test-package-three',
         package_form+' shared-package-body package-vc-tests-checkout-from-elpa-install-from-checkout '
            'package-vc-tests-checkout-with-git-install-from-checkout '
            '(test-package-def test-package-three tests-dir nil package-vc-tests-checkout-from-elpa-install-from-checkout) '
            '(format "package-vc-tests-%s/%s" name pkg)', 'explicit-optional-capability',
         dict(kind='parameter-variant', source_form=package_form, shared_test_body='shared-package-body',
              control_source='same-file', control_test_id=package_id+'/test-package-one'),
         ['skipped', 'skipped'], '(ert-test-skipped ((skip-unless x) :form (memq checkout (install)) :value nil))',
         package_id+'/test-package-one'),
        ('test/lisp/erc/erc-scenarios-spelling.el', 'erc-scenarios-spelling--auto-correct',
         '(unless (ignore-errors (executable-find ispell-program-name) (progn (ispell-check-version) t) '
            '(member "american" (ispell-valid-dictionary-list))) (ert-skip "Missing ispell program")',
         'explicit-optional-capability', dict(kind='external-message', source_form='(ert-skip "Missing ispell program")'),
         ['skipped', 'skipped'], '(ert-test-skipped "Missing ispell program")', None),
    ]
    if late_gates:
        descriptor = copy.deepcopy(flows[1])
        descriptor = (descriptor[0], 'file-notify-test08-backup-remote', *descriptor[2:])
        form = '(skip-unless (string= python-shell-readline-completer-delims ""))'
        selector = ('(skip-unless\n   (eq (ert--stats-selector ert--current-run-stats)\n'
                    '       (ert-test-name (ert--stats-current-test ert--current-run-stats))))')
        manual_condition = ('(ert-test-skipped ((skip-unless (eq (ert--stats-selector '
            'ert--current-run-stats) (ert-test-name (ert--stats-current-test ert--current-run-stats)))) '
            ':form (eq (member synthetic-other tramp-test45-force-remote-file-error) '
            'tramp-test45-force-remote-file-error) :value nil))')
        flows = [descriptor,
            ('test/lisp/progmodes/python-tests.el', 'python-shell-completion-at-point-jedi-completer',
             '(defun python-tests--pythonstartup-file () (getenv "PYTHONSTARTUP") "-m" "jedi" "repl")\n'
             '(python-shell-with-shell-buffer) (defvar python-shell-readline-completer-delims nil)\n'
             'readline.get_completer_delims()\n'+form, 'explicit-optional-capability',
             dict(kind='python-readline-backend',source_form=form), ['skipped','skipped'],
             '(ert-test-skipped ('+form+' :form (string= " :;" "") :value nil))',None),
            ('test/lisp/net/tramp-tests.el', 'tramp-test45-force-remote-file-error',
             ';; It shall run only if selected explicitly.\n'+selector+'\n'
             '"while true; do echo test; sleep 0.2; done"\n(run-at-time\n     0 0.2 nil)\n'
             '(while t\n      (accept-process-output))\n', 'upstream-invalid-test',
             dict(kind='manual-nonterminating-diagnostic',source_form=selector),
             ['skipped','skipped'],manual_condition,None)]
    if prerequisites:
        flows = []
        def add(file, identity, source, definition, observed_form, value='nil', condition=None):
            form = definition['source_form']
            flows.append((file, identity, source, 'explicit-optional-capability', definition,
                ['skipped', 'skipped'], condition or
                '(ert-test-skipped ('+form+' :form '+observed_form+' :value '+value+'))', None))
        add('test/src/emacs-tests.el', 'emacs-tests/bwrap/allows-stdout',
            '(bwrap (executable-find "bwrap")) (skip-unless bwrap)',
            dict(kind='external-lexical-binding', source_form='(skip-unless bwrap)',
                 source_binding='(bwrap (executable-find "bwrap"))'), 'bwrap')
        add('test/lisp/net/ntlm-tests.el', 'ntlm-authentication',
            "(defvar ntlm-tests--dependencies-present (and (featurep 'url-http-ntlm) "
            '(version<= "2.0.4" nil) '+"(featurep 'web-server))) (skip-unless ntlm-tests--dependencies-present)",
            dict(kind='external-package-binding', packages=['url-http-ntlm','web-server'],
                 source_form='(skip-unless ntlm-tests--dependencies-present)'), 'ntlm-tests--dependencies-present')
        helper_flows = [
            ('test/src/casefiddle-tests.el', 'casefiddle-turkish', 'locale', 'get-locale-names',
             '(skip-unless (member "tr_TR.utf8" (get-locale-names)))', '', '(member "tr_TR.utf8" ("C"))', 'nil'),
            ('test/src/fns-tests.el', 'fns-tests-collate-strings', 'collation-locale', 'fns-tests--collate-enabled-p',
             '(skip-unless (fns-tests--collate-enabled-p))',
             '(defun fns-tests--collate-enabled-p () (string-collate-equalp "" "" t) "en_US.UTF-8")',
             '(fns-tests--collate-enabled-p)', 'nil'),
            ('test/lisp/net/nsm-tests.el', 'nsm-check-local-subnet-ipv6', 'global-unicast-ipv6', 'ipv6-is-available',
             '(skip-unless (ipv6-is-available))',
             "(featurep 'make-network-process '(:family ipv6)) "
             '(= (logand (aref elt 0) #xe000) #x2000) (network-interface-list)', '(ipv6-is-available)', 'nil'),
            ('test/lisp/net/network-stream-tests.el', 'echo-server-with-dns', 'non-loopback-hostname',
             'network-test--resolve-system-name', '(skip-when (network-test--resolve-system-name))',
             '(network-lookup-address-info (system-name)) (= (elt address 0) 127) '
             '(equal address [0 0 0 0 0 0 0 1 0])', '(network-test--resolve-system-name)', 't'),
        ]
        for file, identity, facility, predicate, form, helper, observed, truth in helper_flows:
            add(file, identity, helper+' '+form,
                dict(kind='host-facility', facility=facility, predicate=predicate, source_form=form), observed, truth)
        form = '(skip-unless (not (tramp--test-emacs29-p)))'
        add('test/lisp/net/tramp-tests.el', 'tramp-test46-dired-compress-dir',
            '(defun tramp--test-emacs29-p () (>= emacs-major-version 29)) '+form,
            dict(kind='version-scope', source_form=form, applicable_major_versions=list(range(1,29))), '(not t)')
        form = '(skip-unless ucs-normalize-tests--failing-lines-part1)'
        add('test/lisp/international/ucs-normalize-tests.el', 'ucs-normalize-part1-failing',
            '(defconst ucs-normalize-tests--failing-lines-part1 (list )) :expected-result :failed '+form,
            dict(kind='empty-known-failures', source_form=form), 'ucs-normalize-tests--failing-lines-part1')
        form = "(skip-unless (and (fboundp 'w32-shell-dos-semantics)\n                    (w32-shell-dos-semantics)))"
        add('test/lisp/subr-tests.el', 'shell-quote-argument-%-on-w32', form,
            dict(kind='platform', applicable_systems=['windows-nt'], source_form=form),
            "(and (fboundp 'w32-shell-dos-semantics) (w32-shell-dos-semantics))")
        form = '(skip-unless\n (zerop\n  (call-process\n   "dbus-daemon" nil t nil "--fork")))'
        add('test/lisp/net/dbus-tests.el', 'dbus-test02-register-service-own-bus', form,
            dict(kind='external-program-missing', source_form=form), '',
            condition='(ert-test-skipped ((skip-unless x) :form (signal file-missing '
                '("Searching for program" "No such file or directory" "dbus-daemon"))))')
        form = '(skip-unless server-tests/can-create-frames-p)'
        add('test/lisp/server-tests.el', 'server-tests/emacsclient/create-frame',
            '(defconst server-tests/can-create-frames-p (getenv "TERM")) '+form,
            dict(kind='display-helper', source_form=form, applicable_display='terminal'),
            'server-tests/can-create-frames-p')
        form = '(skip-when (string-equal (file-notify--test-library) "kqueue"))'
        add('test/lisp/filenotify-tests.el', 'file-notify-test11-symlinks', form,
            dict(kind='monitor-scope', source_form=form), '(string-equal "kqueue" "kqueue")', 't')
    entries = {}
    for index, (logical, identity, source, *_rest) in enumerate(flows):
        path = context.source/logical
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(source+'\n')
        raw = path.read_bytes()
        entry = dict(context.entries[index], path=logical, source_path=logical,
            size=len(raw), sha256=e.digest(path),
            blob=hashlib.sha1(b'blob '+str(len(raw)).encode()+b'\0'+raw).hexdigest())
        entries[logical] = entry
    context.entries[:len(flows)] = entries.values()
    context.entries.sort(key=lambda row: row['path'])
    manifest = e.bounded_json(context.manifest)
    manifest.update(entries=context.entries, total_bytes=sum(row['size'] for row in context.entries))
    e.write_json(context.manifest, manifest)
    root = context.root/'raw'
    assert child(root, context.manifest, context.program, context.dump, False) == 0
    report = e.bounded_json(root/'stable-suite-run.json')
    report['runtime_environment'] = environments
    products = {label:suite._public_product_identity(suite._product_paths(context.program,
        ['--dump-file='+str(context.dump)], environments[label])) for label in environments}
    report['products_before'] = report['products_after'] = products
    rules = []
    for logical, identity, source, category, definition, statuses, condition, counterpart in flows:
        index = next(i for i,row in enumerate(report['files']) if row['file'] == logical)
        case = report['cases'][index]
        case.update(id=identity, passed=False)
        sibling = dict(file=logical, id=counterpart, passed=True) if counterpart else None
        for label, status in zip(('reference','candidate'), statuses):
            value = e.bounded_json(root/(str(index)+'-'+label+'.json'))
            case[label].update(test_id=identity, status=status, diagnostic='synthetic source gate '+status)
            observations = [case[label]]
            if sibling is not None:
                sibling[label] = dict(test_id=counterpart, status='passed', diagnostic='synthetic applicable sibling',
                    complete=True, exit_code=0, seconds=.001)
                observations.append(sibling[label])
            progress = root/(str(index)+'-'+label+'.events.jsonl')
            data = []
            for ordinal, observed in enumerate(observations):
                event = dict(event='test-ended', file=logical, test_id=observed['test_id'],
                    status=observed['status'], diagnostic=observed['diagnostic'],
                    condition=condition if observed['status'] != 'passed' else 'nil')
                raw = json.dumps(event).encode()
                data.append(raw+b'\n')
                observed['assessment'] = suite.assess_case(observed['test_id'], observed)
                observed['evidence'] = dict(path=progress.name,
                    event_sha256=hashlib.sha256(raw).hexdigest(), event_ordinal=ordinal)
            progress.write_bytes(b''.join(data))
            value.update(ids=[v['test_id'] for v in observations], cases={v['test_id']:v for v in observations},
                runtime_environment=environments[label], metadata=dict(system_type='darwin',
                    features=[], product_version='32.0.50', display_mode='batch', noninteractive=True),
                passed=False, progress=dict(path=progress.name, **accounting.bound_file(progress)))
            if late_gates:
                value['selector'] = 'all-discovered-ids'
            final = root/(str(index)+'-'+label+'.json')
            e.write_json(final,value)
            report['files'][index][label] = dict(**{k:v for k,v in value.items() if k not in ('cases','ids')},
                file_result=dict(path=final.name, **accounting.bound_file(final)))
        count = 2 if counterpart else 1
        report['files'][index].update(reference_id_count=count, candidate_id_count=count,
            registration=dict(passed=True, reference_count=count, missing=[], extra=[], duplicates=[]))
        if sibling is not None:
            report['cases'].append(sibling)
        rule = dict(profile=context.profile['id'],file=logical,id=identity,category=category,
            reason='Synthetic source-gate integration; no actual editor acceptance.',
            input_sha256=entries[logical]['sha256'],source_evidence=[source_anchor(context.source/logical)],
            observations={label:accounting.observation_signature(case[label]) for label in ('reference','candidate')})
        if category == 'proven-version-evolution':
            rule['version_gate'] = definition
            rule['source_evidence'].append(_anchor((libraries if definition['kind'] == 'ert-description-format'
                else descriptor_libraries)['candidate']))
        elif category == 'upstream-invalid-test':
            rule['invalid_gate'] = dict(definition,conditions={label:condition
                for label in ('reference','candidate')})
        else:
            rule['optional_gate'] = accounting._capture_optional_gate(definition, root, suite, case)
        rules.append(rule)
    report.update(corpus_complete=False, failures=['case failed/unverified: '+f+' :: '+i for f,i,*_ in flows])
    report['registration']['reference_count'] = report['registration']['candidate_count'] = len(report['cases'])
    suite._write_run_report(root, report)
    def read(catalog=None):
        return suite.read_completion(root,context.manifest,context.program,context.program,
            ['--dump-file='+str(context.dump)],['--dump-file='+str(context.dump)],
            environments['reference'],environments['candidate'],runner=REPO/'tools/fixtures/stable-suite-runner.el',
            classification=catalog,candidate_profile=context.profile,reference_selection=context.selection)
    receipt = read()
    if prerequisites:
        # Real synthetic files/events and closed envelope; no actual editor or
        # controlling terminal is claimed by this protocol integration flow.
        control_root = context.root/'applicable-controls'
        control_root.mkdir()
        run_path, owner_path = control_root/'results.json', control_root/'artifact-preservation.json'
        controller = control_root/'controller.log'
        controller.write_text('controlled-run: '+json.dumps(dict(reason='exited',exit_code=0,
            cleanup_complete=True,root_pid=1,argv_sha256='c'*64))+'\n')
        e.write_json(owner_path,dict(schema='femacs-runtime-artifacts-v1',state='released',
            output=str(control_root),owner_pid=1,owner_birth='synthetic birth',release_failures=[]))
        control_rows = []
        for rule in rules:
            if rule['optional_gate']['kind'] != 'display-helper':
                continue
            logical, identity = rule['file'], rule['id']
            index = next(i for i,row in enumerate(report['files']) if row['file']==logical)
            rule['optional_gate']['controls'] = {}
            source_path = context.source/logical
            test_input = dict(path=str(source_path),identity=accounting.bound_file(source_path))
            for label in ('reference','candidate'):
                value = e.bounded_json(root/(str(index)+'-'+label+'.json'))
                observed = copy.deepcopy(value['cases'][identity])
                observed.update(status='passed',diagnostic='synthetic terminal helper pass')
                observed['assessment']=suite.assess_case(identity,observed)
                event = dict(event='test-ended',file=logical,test_id=identity,status='passed',
                    diagnostic=observed['diagnostic'])
                raw=json.dumps(event).encode();progress=control_root/(label+'.events.jsonl')
                progress.write_bytes(raw+b'\n')
                observed['evidence']=dict(path=progress.name,event_sha256=hashlib.sha256(raw).hexdigest(),event_ordinal=0)
                value.update(passed=True,cases={identity:observed},
                    metadata=dict(product_version='32.0.50',display_mode='terminal',noninteractive=False,window_system=None),
                    execution_context=dict(owned_terminal_name='synthetic-owned-terminal'),
                    progress=dict(path=progress.name,**accounting.bound_file(progress)),
                    working_fixture=dict(canonical_fixture_inputs_unchanged=True,
                        files=[dict(path=logical,before=test_input['identity'])]),
                    resource_admission=dict(schema='femacs-stable-control-admission-v1',purpose='applicability',
                        profile=context.profile['id'] if label=='candidate' else 'installed-reference',
                        product=receipt['products_before'][label],runtime_environment=environments[label],
                        reference_selection_sha256=context.selection['sha256'],manifest_sha256=receipt['manifest_sha256'],
                        harness_sources_by_role={Path(n).name:h for n,h in receipt['harness_sources'].items()},
                        module_header_sha256=e.digest(REPO/'tools/fixtures/module-thread-config.h')))
                path=control_root/(label+'.json');e.write_json(path,value)
                control_rows.append(dict(profile=value['resource_admission']['profile'],file=logical,
                    complete=True,test_input=test_input,file_result=dict(path=path.name,**accounting.bound_file(path))))
                rule['optional_gate']['controls'][label]=dict(path=str(path),identity=accounting.bound_file(path),
                    test_input=test_input,progress=dict(path=str(progress),identity=accounting.bound_file(progress)),
                    closed_run=dict(path=str(run_path),identity={}),controller=dict(path=str(controller),identity=accounting.bound_file(controller)),
                    owner=dict(path=str(owner_path),identity=accounting.bound_file(owner_path)))
            e.write_json(run_path,dict(schema='femacs-current-version-controls-v1',passed=True,
                diagnostic_collection_complete=True,errors=[],reference_selection=context.selection,
                corpus_root=str(control_root),
                harness_sources={('synthetic/'+Path(n).name):h for n,h in receipt['harness_sources'].items()},
                cases=control_rows,checks={k:True for k in (
                    'all61_product_identities_unchanged','all61_complete_receipts_valid','source_export_unchanged',
                    'reference_selection_unchanged','captured_harness_runner_unchanged',
                    'original_current_version_inputs_unchanged','copied_current_version_inputs_unchanged',
                    'current_source_context_links_unchanged','stable_source_context_unchanged')}))
            for control in rule['optional_gate']['controls'].values():
                control['closed_run']['identity']=accounting.bound_file(run_path)
        independent_errors = []
        for rule in rules:
            case = next(v for v in report['cases'] if (v['file'],v['id']) == (rule['file'],rule['id']))
            file_row = next(v for v in report['files'] if v['file'] == rule['file'])
            try:
                accounting._accept_rule(rule,case,file_row,entries[rule['file']],root,suite,
                    context.selection,context.profile,receipt,{})
            except Exception as error:
                independent_errors.append(dict(file=rule['file'],id=rule['id'],
                    kind=rule['optional_gate']['kind'],error=f'{type(error).__name__}: {error}'))
        assert not independent_errors, independent_errors
        if protective_rows is not None:
            def reject(name, kind, change):
                rule = copy.deepcopy(next(v for v in rules if v['optional_gate']['kind'] == kind))
                case = copy.deepcopy(next(v for v in report['cases']
                    if (v['file'],v['id']) == (rule['file'],rule['id'])))
                file_row = copy.deepcopy(next(v for v in report['files'] if v['file'] == rule['file']))
                change(rule, file_row)
                before = time.monotonic()
                try:
                    accounting._accept_rule(rule,case,file_row,entries[rule['file']],root,suite,
                        context.selection,context.profile,receipt,{})
                except ValueError:
                    pass
                else:
                    raise AssertionError(name+' unexpectedly accepted an unproved exclusion')
                protective_rows.append(dict(id=name,passed=True,seconds=time.monotonic()-before))

            def metadata_both(key, value):
                def change(_rule, file_row):
                    for label in ('reference','candidate'):
                        file_row[label]['metadata'][key] = copy.deepcopy(value)
                return change

            reject('loaded-ntlm-prerequisites-cannot-be-called-missing', 'external-package-binding',
                metadata_both('features',['url-http-ntlm','web-server']))
            reject('windows-api-exclusion-cannot-hide-active-platform', 'platform',
                metadata_both('system_type','windows-nt'))
            reject('legacy-version-exclusion-cannot-hide-active-version', 'version-scope',
                metadata_both('product_version','28.2'))
            reject('darwin-monitor-exclusion-cannot-be-generalized-to-other-hosts', 'monitor-scope',
                metadata_both('system_type','gnu/linux'))
            reject('server-frame-exclusion-requires-closed-applicable-control', 'display-helper',
                lambda rule,_row:rule['optional_gate'].pop('controls'))
            reject('locale-gate-cannot-be-used-as-an-unrelated-host-facility', 'host-facility',
                lambda rule,_row:rule['optional_gate'].update(facility='non-loopback-hostname'))
    result = read(_catalog(context,root,receipt,rules))['classification']
    siblings = sum(bool(flow[-1]) for flow in flows)
    assert result['classified_corpus_accepted'] and result['raw_case_count'] == 555+siblings
    assert result['raw_passed_count'] == 555+siblings-len(flows) and result['raw_failure_count'] == len(flows)
    assert result['classified_case_count'] == len(flows) and not read()['corpus_complete']
    if late_gates and protective_rows is not None:
        jedi = next(rule for rule in rules if rule.get('optional_gate', {}).get('kind') == 'python-readline-backend')
        def reject_jedi(name, *, other_case=False):
            rule = copy.deepcopy(jedi)
            case = copy.deepcopy(next(v for v in report['cases'] if v['id'] == rule['id']))
            row = next(v for v in report['files'] if v['file'] == rule['file'])
            events = {label:accounting._original_case_event(root,suite,case,label)
                      for label in ('reference','candidate')}
            if other_case:
                case['id'] = 'required-core-operation'
            else:
                rule['optional_gate']['condition_forms']['reference'] = '(string= "" "")'
                events['reference']['condition'] = events['reference']['condition'].replace(
                    '(string= " :;" "")', '(string= "" "")')
            before = time.monotonic()
            try:
                accounting._optional_applicability(rule['optional_gate'],case,events,row,
                    [source for logical,_id,source,*_ in flows if logical == case['file']],
                    receipt,suite,context.selection,context.profile,{},root=root)
            except ValueError:
                protective_rows.append(dict(id=name,passed=True,seconds=time.monotonic()-before))
            else:
                raise AssertionError(name+' was unexpectedly accepted')
        reject_jedi('jedi-backend-gate-cannot-excuse-another-required-case',other_case=True)
        reject_jedi('jedi-backend-rejects-a-configured-empty-delimiter')
        manual = next(rule for rule in rules if rule.get('invalid_gate', {}).get('kind') == 'manual-nonterminating-diagnostic')
        for field,value in (('selector','single-explicit-test'),('ids',[])):
            rule = copy.deepcopy(manual)
            case = next(v for v in report['cases'] if v['id'] == rule['id'])
            row = copy.deepcopy(next(v for v in report['files'] if v['file'] == rule['file']))
            pin = row['reference']['file_result'];path=root/pin['path'];saved=path.read_bytes()
            try:
                altered=json.loads(saved);altered[field]=value;e.write_json(path,altered)
                row['reference']['file_result']=dict(path=pin['path'],**accounting.bound_file(path))
                before=time.monotonic()
                try:
                    accounting._accept_rule(rule,case,row,entries[rule['file']],root,suite,
                        context.selection,context.profile,receipt,{})
                except ValueError:
                    protective_rows.append(dict(id='manual-diagnostic-rejects-changed-'+field,
                        passed=True,seconds=time.monotonic()-before))
                else:
                    raise AssertionError('manual diagnostic accepted changed '+field)
            finally:
                path.write_bytes(saved)
    rows.append(dict(id=('remote-component-jedi-and-manual-diagnostic-full-555' if late_gates else
        'source-prerequisites-and-closed-terminal-control-full-555' if prerequisites
        else 'source-component-and-applicability-seven-gates-full-555'), passed=True,
        seconds=time.monotonic()-started, source_gate_count=len(flows), actual_editor_execution=False,
        raw_failed_and_skipped_states_preserved=True, same_file_applicable_controls=2*siblings))


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
                    if index==1:
                        rule['optional_gate']=dict(kind='feature-absent',feature='synthetic-optional',
                            missing_on=['candidate'],source_form="(skip-unless (featurep 'synthetic-optional))",
                            condition_forms=dict(candidate='(featurep synthetic-optional)'))
                    if index==2:
                        control_source=fixture/'same-version-test.el';control_source.write_text(';; synthetic changed-version test\n')
                        rule['source_evidence'].append(_anchor(control_source))
                        control=json.loads((root/'2-candidate.json').read_text())
                        observed=control['cases'][case['id']];observed.update(status='passed',diagnostic='synthetic pass')
                        observed['assessment']=suite.assess_case(case['id'],observed)
                        progress=fixture/'version-control.events.jsonl'
                        event=dict(event='test-ended',file=case['file'],test_id=case['id'],
                            status='passed',diagnostic='synthetic pass')
                        raw=json.dumps(event).encode();progress.write_bytes(raw+b'\n')
                        observed['evidence']=dict(path=progress.name,
                            event_sha256=hashlib.sha256(raw).hexdigest(),event_ordinal=0)
                        control['progress']=dict(path=progress.name,**accounting.bound_file(progress))
                        control['resource_admission']=dict(schema='femacs-stable-control-admission-v1',
                            purpose='current-version',profile=context.profile['id'],
                            product=receipt['products_before']['candidate'],
                            runtime_environment=receipt['runtime_environment']['candidate'],
                            reference_selection_sha256=context.selection['sha256'],
                            harness_sources_by_role={Path(n).name:h for n,h in receipt['harness_sources'].items()},
                            module_header_sha256=e.digest(REPO/'tools/fixtures/module-thread-config.h'))
                        # An unrelated original skip does not make a passing
                        # matching case unusable as a version control. Keep the
                        # whole file's raw non-passed outcome intact.
                        other='synthetic-control-optional'
                        control['ids'].append(other)
                        control['cases'][other]=dict(test_id=other,status='skipped',complete=True,
                            exit_code=0,diagnostic='synthetic unrelated optional skip')
                        control['cases'][other]['assessment']=suite.assess_case(other,control['cases'][other])
                        control.update(passed=False,working_fixture=dict(canonical_fixture_inputs_unchanged=True,
                            files=[dict(path=case['file'],before=accounting.bound_file(control_source))]))
                        control_path=fixture/'version-control.json';e.write_json(control_path,control)
                        run_path=fixture/'version-control-run.json'
                        test_input=dict(path=str(control_source),identity=accounting.bound_file(control_source))
                        e.write_json(run_path,dict(schema='femacs-current-version-controls-v1',
                            diagnostic_collection_complete=True,passed=True,errors=[],
                            reference_selection=context.selection,corpus_root=str(fixture),
                            harness_sources={('synthetic/'+Path(n).name):h
                                             for n,h in receipt['harness_sources'].items()},
                            checks={k:True for k in ('all61_product_identities_unchanged',
                                'all61_complete_receipts_valid','source_export_unchanged',
                                'reference_selection_unchanged','captured_harness_runner_unchanged',
                                'original_current_version_inputs_unchanged','copied_current_version_inputs_unchanged',
                                'current_source_context_links_unchanged','stable_source_context_unchanged')},
                            cases=[dict(profile=context.profile['id'],file=case['file'],complete=True,
                                test_input=test_input,file_result=dict(path=control_path.name,
                                    **accounting.bound_file(control_path)))]))
                        controller=fixture/'synthetic-control-controller.log'
                        controller.write_text('controlled-run: '+json.dumps(dict(reason='exited',exit_code=0,
                            cleanup_complete=True,root_pid=1,argv_sha256='c'*64))+'\n')
                        owner=fixture/'artifact-preservation.json'
                        e.write_json(owner,dict(schema='femacs-runtime-artifacts-v1',state='released',
                            output=str(fixture),owner_pid=1,owner_birth='synthetic birth',release_failures=[]))
                        rule['version_control']=dict(path=str(control_path),identity=accounting.bound_file(control_path),
                            test_input=test_input,progress=dict(path=str(progress),identity=accounting.bound_file(progress)),
                            closed_run=dict(path=str(run_path),identity=accounting.bound_file(run_path)),
                            controller=dict(path=str(controller),identity=accounting.bound_file(controller)),
                            owner=dict(path=str(owner),identity=accounting.bound_file(owner)))
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
                _portable_rule_flow(context, root, stage_result, fixture, rows, rules[2])
                _applicability_flows(context,root,rules,rows)
                _invalid_skip_flows(context,root,rules,rows)
                _removed_producer_flows(fixture, rows)
                _native_positive_flows(fixture, rows)
                _source_scope_flows(fixture, rows)
                _source_scope_flows(fixture, rows, prerequisites=True)
                _source_scope_flows(fixture, rows, late_gates=True)
        start=time.monotonic()
        assert accounting.SVG_CONFIGURATION_TOKEN == 'RSVG'
        assert 'RSVG' in accounting.CONFIGURATION_TOKENS
        assert 'SVG' not in accounting.CONFIGURATION_TOKENS
        rows.append(dict(id='rsvg-configuration-token-is-the-actual-svg-feature',
            passed=True,seconds=time.monotonic()-start))
        start=time.monotonic()
        product=fixture/'scoped-product';product.mkdir()
        program=product/'dist/Femacs.app/Contents/MacOS/Femacs'
        dump=product/'dist/Femacs.app/Contents/Resources/libexec/Femacs.pdmp'
        lisp=product/'dist/Femacs.app/Contents/Resources/lisp'
        program.parent.mkdir(parents=True); dump.parent.mkdir(parents=True); lisp.mkdir(parents=True)
        program.write_bytes(b'scoped-program\n'); dump.write_bytes(b'scoped-dump\n')
        (lisp/'x.el').write_text(';; lisp\n')
        binding=dict(schema=1,product='cocoa',variant='collector',observer=True,test_hooks=False,
                     compiler_leg='normal',app_relative='dist/Femacs.app',config_relative='runtime-config.json',
                     config_digest='a'*64,source_digest='b'*64,native_abi='c'*64,optimization='2',
                     deployment_target='27.2',target='aarch64-apple-darwin',backend_identity=None)
        e.write_json(product/'runtime-binding.json',binding)
        e.write_json(product/'runtime-product.json',dict(schema=1,build_complete=True,binding=binding,
            compatibility_accepted=False,performance_accepted=False,semantic_tests_executed=False,
            identity_version=1,files={},file_identities={},link_inputs={},resources={}))
        e.write_json(product/'runtime-inputs.json',dict(schema=1,source_root=str(REPO),
            backend_prefix=None,native_cache=None))
        receipt=dict(runtime_environment=dict(candidate=dict(
            FEMACS_GC_COLLECTOR='young',FEMACS_STABLE_PRODUCT_LISP=str(lisp))),
            products_before=dict(candidate=dict(
                program=dict(sha256=e.digest(program),bytes=program.stat().st_size,name='Femacs'),
                dump=dict(sha256=e.digest(dump),bytes=dump.stat().st_size,name='Femacs.pdmp'))))
        direct=accounting.runtime_product_profile(product, collector='young')
        assert direct['id']=='cocoa-collector-observer-normal-young'
        assert direct['source_root']==str(REPO.resolve())
        derived=accounting.scoped_candidate_profile(product, receipt)
        assert derived['id']=='cocoa-collector-observer-normal-young'
        assert derived['collector']=='young' and derived['kind']=='cocoa' and derived['native'] is False
        assert accounting.product_root_from_receipt(receipt)==product.resolve()
        try:
            accounting.scoped_candidate_profile(product, dict(receipt, products_before=dict(
                candidate=dict(receipt['products_before']['candidate'],
                               program=dict(sha256='0'*64,bytes=1,name='Femacs')))))
            raise AssertionError('program identity mismatch must remain blocking')
        except ValueError:
            pass
        native_binding=dict(binding, variant='native-collector')
        e.write_json(product/'runtime-binding.json',native_binding)
        e.write_json(product/'runtime-product.json',dict(schema=1,build_complete=True,binding=native_binding,
            compatibility_accepted=False,performance_accepted=False,semantic_tests_executed=False,
            identity_version=1,files={},file_identities={},link_inputs={},resources={}))
        native=accounting.scoped_candidate_profile(product, receipt)
        assert native['id']=='cocoa-native-collector-observer-normal-young' and native['native'] is True
        matrix_checks={k:True for k in accounting.MATRIX_CONTROL_CHECKS}
        selected_checks={k:True for k in accounting.SELECTED_CONTROL_CHECKS}
        assert accounting.control_postflight_kind(dict(checks=matrix_checks))=='matrix'
        assert accounting.control_postflight_kind(dict(
            checks=selected_checks,selected_product_admission=True))=='selected'
        try:
            accounting.control_postflight_kind(dict(checks=selected_checks))
            raise AssertionError('selected checks without selected_product_admission must remain blocking')
        except ValueError:
            pass
        assert accounting.remote_postflight_kind(dict(
            checks={k:True for k in accounting.MATRIX_REMOTE_CHECKS},
            full_matrix_admission=True))=='matrix'
        assert accounting.remote_postflight_kind(dict(
            checks={k:True for k in accounting.SELECTED_REMOTE_CHECKS},
            selected_product_admission=True))=='selected'
        assert accounting.library_postflight_kind(dict(
            selected_inputs_unchanged=True, generated_override_used=False))=='matrix'
        assert accounting.library_postflight_kind(dict(
            selected_inputs_unchanged=True, generated_override_used=False,
            checks={k:True for k in accounting.MATRIX_LIBRARY_CHECKS}))=='matrix'
        assert accounting.library_postflight_kind(dict(
            selected_inputs_unchanged=True, selected_product_admission=True,
            checks={k:True for k in accounting.SELECTED_LIBRARY_CHECKS}))=='selected'
        try:
            accounting.library_postflight_kind(dict(
                selected_inputs_unchanged=True, selected_product_admission=True,
                checks={k:True for k in accounting.MATRIX_LIBRARY_CHECKS}))
            raise AssertionError('selected library admission cannot reuse matrix check names')
        except ValueError:
            pass
        rows.append(dict(id='scoped-profile-from-runtime-binding-and-env',passed=True,
            seconds=time.monotonic()-start))
        rows.append(dict(id='selected-product-control-postflight-contract',passed=True,
            seconds=0))
        subject={role:'a'*64 for role in accounting.CONTROL_HARNESS_ROLES}
        control={role:'b'*64 for role in accounting.CONTROL_HARNESS_ROLES}
        assert accounting.control_executor_harness_roles(
            dict(harness_sources_by_role=control),
            dict(harness_sources={('captured/'+role):digest for role,digest in control.items()}))==control
        assert accounting.subject_harness_roles(dict(
            harness_sources={('subject/'+role):digest for role,digest in subject.items()}))==subject
        try:
            accounting.control_executor_harness_roles(
                dict(harness_sources_by_role=control),
                dict(harness_sources={('captured/'+role):digest for role,digest in subject.items()}))
            raise AssertionError('selected own-proof must not accept a mismatched closed harness')
        except ValueError:
            pass
        rows.append(dict(id='selected-product-library-remote-postflight-contract',passed=True,
            seconds=0))
        rows.append(dict(id='control-executor-harness-independent-of-subject',passed=True,
            seconds=0))
        rows.append(dict(id='matrix-keeps-subject-harness-equality-gate',passed=True,
            seconds=0))
        return dict(passed=all(row['passed'] for row in rows),cases=rows,actual_python_children=3,
            original_files_per_synthetic_run=555,synthetic_case_observations=1110,
            applicability_reaccounting_runs=len(APPLICABILITY_FLOWS),invalid_gate_reaccounting_runs=2,
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

        owner_root = fixture/'owner-output'
        owner_root.mkdir()
        def owner_summary(*, change=None, wrong_pin=False, duplicate=False, wrong_controller=False):
            value = dict(schema='femacs-runtime-artifacts-v1',state='released',
                output=str(owner_root),owner_pid=311,owner_birth='synthetic closed birth',
                files=[],symlinks=[],release_failures=[])
            if change:
                change(value)
            path = owner_root/'artifact-preservation.json'
            text = json.dumps(value)
            if duplicate:
                text = text[:-1]+',"state":"released"}'
            path.write_text(text+'\n')
            pin = accounting.bound_file(path)
            if wrong_pin:
                pin['sha256'] = '0'*64
            if wrong_controller:
                return accounting._control_owner(dict(path=str(path),identity=pin),
                    dict(root_pid=312),owner_root,{})
            return accounting.read_owner_summary(path,pin,owner_root)

        check('streamed-owner-rejects-unreleased-state',lambda:owner_summary(
            change=lambda value:value.update(state='active')))
        check('streamed-owner-rejects-release-failures',lambda:owner_summary(
            change=lambda value:value.update(release_failures=['synthetic failure'])))
        check('streamed-owner-requires-explicit-release-failure-field',lambda:owner_summary(
            change=lambda value:value.pop('release_failures')))
        check('streamed-owner-rejects-another-output',lambda:owner_summary(
            change=lambda value:value.update(output=str(fixture/'different-output'))))
        check('streamed-owner-rejects-changed-content-pin',lambda:owner_summary(wrong_pin=True))
        check('streamed-owner-rejects-duplicate-header-field',lambda:owner_summary(duplicate=True))
        check('streamed-owner-rejects-another-controller',lambda:owner_summary(wrong_controller=True))
        prerequisite_fixture = fixture/'prerequisite-protection'
        prerequisite_fixture.mkdir()
        _source_scope_flows(prerequisite_fixture,[],prerequisites=True,protective_rows=rows)
        _source_scope_flows(prerequisite_fixture,[],late_gates=True,protective_rows=rows)
        return dict(passed=all(row['passed'] for row in rows),cases=rows,
            functional_acceptance_sha256=e.digest(functional),actual_editor_execution=False,
            seconds=time.monotonic()-started,goal_complete=False)
    finally:
        shutil.rmtree(fixture)


def main():
    parser=argparse.ArgumentParser()
    parser.add_argument('--child',type=Path)
    parser.add_argument('--manifest',type=Path)
    parser.add_argument('--fixture-repo',type=Path,default=REPO)
    parser.add_argument('--program',type=Path)
    parser.add_argument('--dump',type=Path)
    parser.add_argument('--failed',action='store_true')
    parser.add_argument('--mode',choices=('integration','protection'),default='integration')
    parser.add_argument('--functional',type=Path)
    parser.add_argument('--out',type=Path)
    args=parser.parse_args()
    if args.child:return child(args.child,args.manifest,args.program,args.dump,args.failed,
                               fixture_repo=args.fixture_repo)
    out=e.new_output(args.out)
    result=integration(out) if args.mode=='integration' else protection(out,args.functional)
    result['owned_fixture_retired']=not (out/'owned-fixtures').exists()
    e.write_json(out/'results.json',result)
    return 0 if result['passed'] else 1


if __name__=='__main__':raise SystemExit(main())
