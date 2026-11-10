"""Admit an evidence-bound reference epoch while preserving compiled candidates.

This reader consumes existing installation/runtime observations. It does not
install packages, invoke editors or rewrite historical build receipts.
"""
from pathlib import Path
import re


SCHEMA = 'femacs-runtime-reference-selection-v1'
SCOPE = 'explicit-reference-replacement-preserving-60-compiled-candidates'
CAPABILITIES = {'native', 'treesit', 'sqlite', 'libxml2', 'gnutls', 'lcms2', 'modules'}
REFERENCE_KEYS = {'id', 'kind', 'compiler_leg', 'native', 'observer', 'test_hooks',
                  'collector', 'program', 'dump', 'resource_roots'}
INVARIANTS = dict(id='installed-reference', kind='reference', compiler_leg='normal',
                  observer=False, test_hooks=False, collector='off')


def require(condition, message):
    if not condition:
        raise ValueError(message)


def historical_document(execution, product_list, build):
    """Check the immutable dictionary without opening its retired reference."""
    value = execution.bounded_json(product_list, 64 * 1024 * 1024)
    require(set(value) == {'schema', 'products', 'source_cohorts'}
            and value['schema'] == 1, 'unexpected original product document')
    require(isinstance(build.get('products'), list) and len(build['products']) == 61
            and value['products'] == build['products'], 'original products differ from build')
    cohorts = build.get('source_cohorts')
    require(isinstance(cohorts, list) and bool(cohorts)
            and build.get('source_export') in cohorts and value['source_cohorts'] == cohorts,
            'original source cohorts differ from build')
    require(isinstance(build.get('reference_identity'), dict)
            and set(build['reference_identity']) == {'program', 'dump', 'resources'},
            'missing original reference identity')
    refs = [row for row in value['products'] if row.get('kind') == 'reference']
    require(len(refs) == 1 and refs[0] == value['products'][0]
            and refs[0].get('id') == 'installed-reference', 'one leading reference required')
    require(all('_harness' not in row for row in value['products']),
            'original compiled registry contains a reporting override')
    return value


def _bound_file(execution, binding, *, within=None):
    require(isinstance(binding, dict) and isinstance(binding.get('path'), str),
            'missing bound evidence file')
    path = execution.safe_path(binding['path'])
    require(path.is_file() and (within is None or path.is_relative_to(within)),
            'evidence must be an ordinary file in its owned selection')
    identity = execution.file_identity(path)
    require(identity == binding.get('identity'), 'bound evidence identity changed')
    return path, identity


def _proofs(execution, directory, selection, reference, live_identity):
    inputs = selection.get('evidence_inputs', {})
    names = {'recipe_info': 'recipe-info.json', 'reference_probe': 'reference-probe.json',
             'upstream_tree_comparison': 'upstream-tree-comparison.json'}
    require(set(inputs) == set(names), 'complete reference evidence set required')
    documents, identities = {}, {}
    for key, name in names.items():
        path, identity = _bound_file(execution, inputs[key], within=directory)
        require(path == directory / name, 'reference evidence must be adjacent')
        documents[key] = execution.bounded_json(path, 64 * 1024 * 1024)
        identities[str(path)] = identity
    recipe = documents['recipe_info']
    require(isinstance(recipe.get('formulae'), list) and len(recipe['formulae']) == 1,
            'one actual reference recipe required')
    formula = recipe['formulae'][0]
    version = formula.get('versions', {}).get('stable')
    revision = formula.get('urls', {}).get('stable', {}).get('revision')
    formula_sha = formula.get('ruby_source_checksum', {}).get('sha256')
    require(isinstance(version, str) and bool(version)
            and isinstance(revision, str) and re.fullmatch('[0-9a-f]{40}', revision)
            and isinstance(formula_sha, str) and execution.HEX.fullmatch(formula_sha)
            and isinstance(formula.get('full_name'), str)
            and isinstance(formula.get('tap'), str), 'incomplete pinned reference recipe')
    probe = documents['reference_probe']
    require(probe.get('schema') == 'femacs-reference-install-probe-v1'
            and probe.get('passed') is True, 'successful actual installation probe required')
    install_path, install_identity = _bound_file(execution, probe.get('install_receipt'))
    install = execution.bounded_json(install_path)
    source = install.get('source', {})
    require(install_path.name == 'INSTALL_RECEIPT.json'
            and install.get('poured_from_bottle') is False
            and source.get('tap') == formula['tap']
            and source.get('scm_revision') == revision
            and source.get('versions', {}).get('stable') == version
            and Path(source.get('path', '')).stem
                in {formula.get('name'), *formula.get('aliases', [])},
            'installed reference receipt disagrees with recipe')
    external = {str(install_path): install_identity}
    config_path, config_identity = _bound_file(execution, probe.get('build_config'))
    external[str(config_path)] = config_identity
    process = probe.get('process', {})
    argv = process.get('argv', [])
    require(process.get('complete') is True and process.get('exit_code') == 0
            and process.get('timed_out') is False
            and process.get('remaining_group_terminated') is False
            and isinstance(argv, list) and bool(argv) and argv[0] == reference['program']
            and '--dump-file=' + reference['dump'] in argv,
            'reference probe did not complete against selected program/dump')
    require(probe.get('product_identity_before') == probe.get('product_identity_after')
            == live_identity, 'installed reference differs from actual runtime probe')
    facts = probe.get('sampled_facts', {})
    capabilities = facts.get('capabilities', {})
    require(set(capabilities) == CAPABILITIES
            and all(type(value) is bool for value in capabilities.values())
            and capabilities['native'] is reference['native']
            and facts.get('emacs_version') == version
            and 'emacs_repository_version' in facts
            and facts.get('emacs_repository_version') in (None, revision)
            and probe.get('repository_version_embedded') is (facts.get('emacs_repository_version') is not None)
            and probe.get('source_revision_provenance') == 'pinned-recipe-and-install-receipt'
            and isinstance(facts.get('system_configuration'), str)
            and bool(facts['system_configuration'])
            and isinstance(facts.get('system_configuration_features'), str)
            and bool(facts['system_configuration_features']),
            'runtime version, revision or capabilities disagree with recipe')
    require(facts.get('lisp_directory') == reference['resource_roots']['lisp']
            and facts.get('data_directory') == reference['resource_roots']['data'],
            'runtime resource directories differ from selected reference')
    patches = formula.get('patches', [])
    captured = probe.get('patch_files', {})
    require(isinstance(patches, list) and isinstance(captured, dict)
            and len({row.get('url') for row in patches}) == len(patches)
            and set(captured) == {row.get('url') for row in patches},
            'reference patch evidence incomplete or duplicated')
    for patch in patches:
        path, identity = _bound_file(execution, captured[patch['url']], within=directory / 'patches')
        require(identity['sha256'] == patch.get('sha256'), 'captured reference patch digest mismatch')
        identities[str(path)] = identity
    tree = documents['upstream_tree_comparison']
    require(tree.get('schema') == 'femacs-upstream-data-git-tree-comparison-v1'
            and tree.get('upstream_commit') == revision
            and isinstance(tree.get('candidate_compiled_source'), str)
            and re.fullmatch('[0-9a-f]{40}', tree['candidate_compiled_source'])
            and type(tree.get('candidate_raw_entries')) is int
            and tree['candidate_raw_entries'] > 0
            and type(tree.get('upstream_raw_entries')) is int
            and tree['upstream_raw_entries'] > 0
            and isinstance(tree.get('candidate_changed_or_extra'), list)
            and isinstance(tree.get('upstream_entries_not_tracked_by_candidate'), list)
            and tree.get('git_metadata_only') is True
            and tree.get('installed_product_files_verified') is False,
            'pinned source-tree metadata comparison required')
    upstream = dict(version=version, commit=revision, formula=formula['full_name'],
                    tap=formula['tap'], formula_sha256=formula_sha, patches=patches)
    return upstream, capabilities, tree, identities, external


def read_selection(execution, repo, original, product_list, build, path):
    """Return selected live inputs; all historical reference paths stay unopened."""
    historical = historical_document(execution, product_list, build)
    path = execution.safe_path(path)
    directory = path.parent
    require(path.name == 'reference-selection.json'
            and directory.is_relative_to(repo / 'build') and directory != repo / 'build',
            'reference selection must be an owned build input')
    value = execution.bounded_json(path, 64 * 1024 * 1024)
    require(value.get('schema') == SCHEMA and value.get('scope') == SCOPE
            and isinstance(value.get('epoch'), str) and bool(value['epoch'])
            and value.get('build_root') == str(original)
            and value.get('build_result_sha256') == execution.digest(original / 'results.json')
            and value.get('source_manifest_sha256') == build['source_export']['source_manifest_sha256']
            and value.get('original_products_path') == str(product_list)
            and value.get('original_products_sha256') == execution.digest(product_list),
            'reference selection is not bound to original build')
    selected_path = execution.safe_path(directory / 'products.json')
    require(value.get('selected_products_path') == str(selected_path)
            and value.get('selected_products_sha256') == execution.digest(selected_path),
            'selected product registry digest mismatch')
    document = execution.bounded_json(selected_path, 64 * 1024 * 1024)
    require(set(document) == set(historical) and document['schema'] == 1
            and document['source_cohorts'] == historical['source_cohorts']
            and isinstance(document.get('products'), list) and len(document['products']) == 61
            and document['products'][1:] == historical['products'][1:]
            and [p.get('id') for p in document['products']]
                == [p.get('id') for p in historical['products']] == value.get('profile_order')
            and value.get('profile_count') == 61 and value.get('candidate_count') == 60,
            'reference selection changed compiled candidates, order or source cohorts')
    ref = document['products'][0]
    require(set(ref) == REFERENCE_KEYS
            and all(ref.get(key) == expected for key, expected in INVARIANTS.items())
            and type(ref.get('native')) is bool
            and isinstance(ref.get('resource_roots'), dict)
            and set(ref['resource_roots']) == {'lisp', 'data'},
            'selected reference changed invariant profile fields')
    history = value.get('original_reference', {})
    require(history.get('product') == historical['products'][0]
            and history.get('identity') == build['reference_identity']
            and history.get('historical_observations_retained') is True
            and isinstance(history.get('historical_epoch'), str) and bool(history['historical_epoch'])
            and history['historical_epoch'] != value['epoch'], 'original reference history relabeled')
    # This is the existing complete admission of all60 candidate receipts/resources.
    products = execution.load_products(selected_path)
    require(products == document['products'], 'selected paths changed during complete admission')
    live = execution.product_identity(products[0], resources=True)
    selected = value.get('selected_reference', {})
    require(selected.get('product') == ref and selected.get('identity') == live
            and live != build['reference_identity'], 'selected reference identity mismatch')
    upstream, capabilities, tree, identities, external = _proofs(
        execution, directory, value, ref, live)
    require(selected.get('upstream') == upstream and selected.get('capabilities') == capabilities
            and selected.get('upstream_tree_comparison') == tree,
            'reference facts differ from bound actual observations')
    flags = ('candidate_and_reference_libraries_byte_identical', 'existing_components_reverted',
             'new_waivers_added', 'reference_semantics_accepted', 'compatibility_accepted',
             'performance_accepted', 'goal_complete')
    require(all(value.get(key) is False for key in flags),
            'reference selection cannot certify runtime acceptance')
    identities.update({str(p): execution.file_identity(p) for p in (path, selected_path)})
    summary = dict(path=str(path), sha256=identities[str(path)]['sha256'], epoch=value['epoch'],
                   historical_epoch=history['historical_epoch'], selected_products_sha256=
                   value['selected_products_sha256'], upstream=upstream, capabilities=capabilities,
                   upstream_tree_comparison=tree, **{key: False for key in flags})
    probe_path = Path(value['evidence_inputs']['reference_probe']['path'])
    observed = execution.bounded_json(probe_path, 64 * 1024 * 1024)
    summary.update(runtime_repository_version=observed['sampled_facts']['emacs_repository_version'],
                   repository_version_embedded=observed['repository_version_embedded'],
                   source_revision_provenance=observed['source_revision_provenance'])
    return selected_path, products, summary, identities, external, live


def verify_selection(execution, identities, external, reference, expected):
    for name, identity in {**identities, **external}.items():
        require(execution.file_identity(execution.safe_path(name)) == identity,
                'reference selection evidence changed')
    require(execution.product_identity(reference, resources=True) == expected,
            'selected reference changed after admission')
