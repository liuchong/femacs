"""Bind actual old per-case evidence before a separately identified affected replay."""
from pathlib import Path
import collections
import importlib.util
import json


def verify(execution, source, products_path, products, original, index, audit_path, out):
    original, index, audit_path = map(execution.safe_path, (original, index, audit_path))
    audit = execution.bounded_json(audit_path)
    if (execution.digest(index) != audit.get('case_index_sha256')
            or execution.digest(original/'results.json') != audit.get('detailed_result_sha256')
            or (original/'results.json').stat().st_size != audit.get('detailed_result_bytes')):
        raise ValueError('original case index/detailed audit identity mismatch')
    recovery = execution.safe_path(execution.REPO/audit['retained_recovery_archive'])
    protection = execution.bounded_json(recovery.parent/'artifact-preservation.json',128*1024**2)
    archive = protection['archive']
    if (protection['state'] != 'released' or protection['release_failures']
            or recovery.stat().st_size != archive['bytes']
            or execution.digest(recovery) != archive['sha256']):
        raise ValueError('retained source/product recovery archive changed')
    recovery_identity = dict(path=str(recovery), bytes=archive['bytes'], sha256=archive['sha256'])
    del protection
    inputs = execution.bounded_json(original/'inputs.json')
    if (inputs.get('action') != 'ert'
            or inputs.get('product_list_sha256') != execution.digest(products_path)
            or inputs.get('harness_sha256') != execution.digest(source/'tools/fixtures/runtime-process-runner.el')):
        raise ValueError('original ERT product/harness binding changed')
    before = execution.bounded_json(original/'product-identities-before.json', 128*1024**2)
    after = execution.bounded_json(original/'product-identities-after.json', 128*1024**2)
    if before != after or set(before) != {p['id'] for p in products}:
        raise ValueError('original full product boundary incomplete/changed')
    for product in products:
        if before[product['id']] != execution.product_identity(product, resources=True):
            raise ValueError('original products no longer match actual products')
    del before, after
    spec = importlib.util.spec_from_file_location('femacs_original_fixture_groups', source/'tools/runtime-execution.py')
    frozen = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(frozen)
    expected = {}
    for product in products:
        for mode in ('off','phases','survival','all') if product['observer'] else ('off',):
            for name, fixtures, selector in frozen.fixture_groups(product, mode):
                expected[(product['id'], mode, name)] = fixtures
    discoveries, cases, rows = {}, collections.defaultdict(list), {}
    case_products = {p['id']: execution.product_identity(p) for p in products}
    counts = collections.Counter()
    references, language = {}, []
    with index.open() as stream:
        for line in stream:
            if len(line) > 32768:
                raise ValueError('oversized original case-index line')
            identity = json.loads(line)
            path = execution.REPO/identity['path']
            path = execution.safe_path(path)
            if not path.is_relative_to(original) or path.name != 'result.json':
                raise ValueError('case index outside original ERT root')
            if path.stat().st_size != identity['bytes'] or execution.digest(path) != identity['sha256']:
                raise ValueError('original per-case evidence changed')
            row = execution.bounded_json(path, 8*1024**2)
            key = tuple(path.relative_to(original).parts[:3])
            if key not in expected or row.get('profile') != key[0]:
                raise ValueError('unexpected original case group/profile')
            request, response = row['request'], row['response']
            if (row.get('complete') is not True or row.get('timed_out') is not False
                    or row.get('remaining_group_terminated') is not False
                    or response.get('complete') is not True
                    or row['source_identity_before'] != row['source_identity_after']
                    or row['source_identity_before'] != case_products[key[0]]
                    or type(row.get('passed')) is not bool
                    or request.get('fixtures') != [str(source/'tools/fixtures'/name) for name in expected[key]]):
                raise ValueError('incomplete or differently bound original case')
            operation = request['operation']
            case_id = request.get('id')
            if operation == 'discover':
                ids = response.get('ids')
                if key in discoveries or not isinstance(ids,list) or not ids or len(set(ids)) != len(ids):
                    raise ValueError('missing or duplicate original discovered IDs')
                discoveries[key] = ids
            elif operation == 'ert':
                if response.get('id') != case_id:
                    raise ValueError('original ERT response ID mismatch')
                cases[key].append(case_id)
                if case_id.startswith('rtl'):
                    observation = response['observations']['language']
                    if not isinstance(observation,list) or not observation:
                        raise ValueError('missing original language observation')
                    language.append((key[0],case_id,observation))
            else:
                raise ValueError('unknown original process operation')
            if row.get('passed') is True and (row['exit_code'] != 0
                    or (operation == 'ert' and response.get('status') != 'passed')):
                raise ValueError('original passed row contradicts actual process/outcome')
            full_key = (*key, operation, case_id)
            if full_key in rows:
                raise ValueError('duplicate original process identity')
            rows[full_key] = dict(path=str(path), sha256=identity['sha256'], passed=row['passed'])
            counts[operation] += 1
            counts['failed'] += row['passed'] is False
    if set(discoveries) != set(expected) or set(cases) != set(expected):
        raise ValueError('original groups missing or extra')
    for key, ids in discoveries.items():
        if sorted(cases[key]) != sorted(ids):
            raise ValueError('original group cases do not account for every discovered ID')
    reference_ids = {p['id'] for p in products if p['kind'] == 'reference'}
    for profile, case_id, observation in language:
        if profile in reference_ids:
            if case_id in references and references[case_id] != observation:
                raise ValueError('reference observations disagree')
            references[case_id] = observation
    if not references or any(references.get(case_id) != value for _,case_id,value in language):
        raise ValueError('original language comparison failed')
    for key, ids in discoveries.items():
        if key[2] == 'language' and set(ids) != set(references):
            raise ValueError('original language registrations disagree')
    receipt = dict(schema='femacs-runtime-original-checkpoint-v1', action='ert',
        post_hoc=True, execution_complete=True, passed=not counts['failed'],
        counts=dict(counts), profiles=len(products), groups=len(expected),
        language_observations=len(language), language_matches=True,
        product_list_sha256=execution.digest(products_path),
        audit_sha256=execution.digest(audit_path), case_index_sha256=execution.digest(index),
        detailed=dict(path=str(original/'results.json'),
            bytes=audit['detailed_result_bytes'], sha256=audit['detailed_result_sha256']),
        all_expected_groups_and_ids_accounted=True, original_failures_retained=True,
        retained_recovery_archive=recovery_identity,
        goal_complete=False, compatibility_accepted=False)
    execution.write_json(out/'original-ert-checkpoint.json', receipt)
    return rows, receipt


def merge(execution, original_rows, replay_root, completion, out):
    """Replace every affected group only, never overwrite original attempts."""
    inputs = completion['input_binding']
    if inputs.get('requested_groups') != ['collector']:
        raise ValueError('only the declared affected collector group may replace original evidence')
    replacements = {}
    with (replay_root/'case-index.jsonl').open() as stream:
        for line in stream:
            item = json.loads(line)
            path = execution.safe_path(replay_root/item['path'])
            if (not path.is_relative_to(replay_root) or execution.digest(path) != item['sha256']
                    or path.stat().st_size != item['bytes']):
                raise ValueError('repaired per-case evidence identity changed')
            row = execution.bounded_json(path, 8*1024**2)
            group = tuple(path.relative_to(replay_root).parts[:3])
            request = row['request']
            key = (*group, request['operation'], request.get('id'))
            if group[2] != 'collector' or key in replacements:
                raise ValueError('unexpected/duplicate repaired process identity')
            replacements[key] = dict(path=str(path), sha256=item['sha256'], passed=row['passed'])
    affected = {key for key in original_rows if key[2] == 'collector'}
    if set(replacements) != affected:
        raise ValueError('repaired groups do not cover every original affected ID')
    final = {key: replacements.get(key,value) for key,value in original_rows.items()}
    receipt = dict(schema='femacs-runtime-affected-merge-v1',
        original_checkpoint_sha256=execution.digest(out/'original-ert-checkpoint.json'),
        replay_completion_sha256=execution.digest(replay_root/'completion.json'),
        original_processes=len(original_rows), reused_processes=len(final)-len(affected),
        repaired_processes=len(affected),
        original_failed_processes=sum(row['passed'] is False for row in original_rows.values()),
        remaining_failed_processes=sum(row['passed'] is False for row in final.values()),
        complete_group_and_id_match=True, passed=all(row['passed'] for row in final.values()),
        original_failures_retained=True, goal_complete=False, compatibility_accepted=False)
    execution.write_json(out/'ert-affected-merge.json', receipt)
    return receipt
