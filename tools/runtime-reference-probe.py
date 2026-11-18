"""Sample the installed reference and bind its epoch to a fresh completed matrix.

This tool never installs a package, changes its configuration, or relabels an
earlier run. Recipe/install/tree provenance is explicit input; the sampled
facts, products and source export must agree with the existing strict reader.
"""
import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import sys
import time

ROOT = Path(__file__).resolve().parent.parent


def load_stage():
    spec = importlib.util.spec_from_file_location('current_reference_probe_stage', ROOT/'tools/runtime-stage.py')
    stage = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(stage)
    return stage


def capture_inputs_main(argv):
    """Create reproducible diagnostic inputs without a matrix or editor run."""
    parser = argparse.ArgumentParser(description='Capture explicit installed-reference diagnostic inputs')
    for name in ('program', 'dump', 'lisp-root', 'data-root', 'install-receipt', 'build-config'):
        parser.add_argument('--'+name, type=Path, required=True)
    parser.add_argument('--epoch', required=True)
    parser.add_argument('--native', action=argparse.BooleanOptionalAction, required=True,
                        help='declared feature only; capture does not sample runtime capabilities')
    parser.add_argument('--out', type=Path, required=True)
    args = parser.parse_args(argv)
    try:
        stage = load_stage()
        e, reference = stage.execution, stage.reference_selection
        product = dict(reference.INVARIANTS, native=args.native, program=str(args.program),
            dump=str(args.dump), resource_roots=dict(lisp=str(args.lisp_root), data=str(args.data_root)))
        value = reference.capture_installation_inputs(e, product, args.install_receipt,
                                                     args.build_config, args.epoch)
        # Validate every input before creating output. No old cache is searched.
        out = e.new_output(args.out)
        path = out/'reference-inputs.json'
        e.write_json(path, value)
        reference.read_installation_inputs(e, path, value['product'])
        e.write_json(out/'products.json', dict(schema=1, products=[value['product']]))
        print(json.dumps(dict(reference_inputs=str(path), reference_inputs_sha256=e.digest(path),
                              actual_editor_execution=False, full_matrix_admission=False)))
        return 0
    except (OSError, ValueError, KeyError, TypeError) as error:
        parser.error(str(error))


def fingerprints(execution, products):
    return {p['id']: hashlib.sha256(json.dumps(execution.product_identity(p, resources=True),
        sort_keys=True, separators=(',', ':'), allow_nan=False).encode()).hexdigest() for p in products}


def tree_comparison(stage, args, recipe, build, out):
    """Compare the explicit Git data trees, without updating either checkout."""
    e = stage.execution
    upstream = recipe['formulae'][0]['urls']['stable']['revision']
    scopes = ('lisp', 'leim', 'test', 'etc', 'doc', 'admin')
    maps = []
    env = dict(os.environ, GIT_NO_REPLACE_OBJECTS='1', GIT_OPTIONAL_LOCKS='0',
        GIT_CONFIG_NOSYSTEM='1', GIT_CONFIG_GLOBAL='/dev/null')
    for label, repository, commit in (('candidate', stage.REPO, args.candidate_data_commit),
            ('upstream', e.safe_path(args.upstream_repository), upstream)):
        if not isinstance(commit, str) or not re.fullmatch('[0-9a-f]{40}', commit):
            raise ValueError('explicit full Git commit required for data-tree comparison')
        directory = out/'git-trees'/label
        process = e.run_process(['git', '-C', repository, 'ls-tree', '-r', '-z', commit,
            '--', *scopes], directory, env, 30, cwd=stage.REPO)
        data = directory/'stdout.txt'
        if (not process['complete'] or process['exit_code'] != 0
                or process['remaining_group_terminated'] or not 0 < data.stat().st_size <= 8*1024*1024):
            raise ValueError('bounded Git data-tree query failed')
        raw = data.read_bytes()
        if not raw.endswith(b'\0'):
            raise ValueError('incomplete NUL-delimited Git tree')
        entries = {}
        for row in raw[:-1].split(b'\0'):
            header, path = row.split(b'\t', 1)
            mode, kind, blob = header.decode('ascii').split()
            path = path.decode('utf-8')
            relative = Path(path)
            if (relative.is_absolute() or '..' in relative.parts or relative.parts[0] not in scopes
                    or path in entries or len(entries) >= 100000
                    or not re.fullmatch('[0-9a-f]{40}', blob)):
                raise ValueError('invalid or oversized Git data-tree scope')
            entries[path] = dict(mode=mode, type=kind, blob=blob)
        maps.append(entries)
    candidate, reference = maps
    return dict(schema='femacs-upstream-data-git-tree-comparison-v1',
        candidate_compiled_source=args.candidate_data_commit, upstream_commit=upstream,
        candidate_commit_describes_data_tree_only=True,
        candidate_source_manifest_sha256=build['source_export']['source_manifest_sha256'],
        candidate_raw_entries=len(candidate), upstream_raw_entries=len(reference),
        identical_entries=sum(reference.get(path) == item for path, item in candidate.items()),
        candidate_changed_or_extra=[dict(path=path, candidate=item, upstream=reference.get(path))
            for path,item in sorted(candidate.items()) if reference.get(path) != item],
        upstream_entries_not_tracked_by_candidate=sorted(set(reference)-set(candidate)),
        git_metadata_only=True, physical_source_files_verified=False,
        installed_product_files_verified=False)


def provenance(stage, args, suite, batch):
    """Normalize explicit current provenance into one sampling path."""
    e = stage.execution
    if args.prior_selection is not None:
        prior = e.safe_path(args.prior_selection)
        selected = batch._reuse_json(suite, prior)
        if selected.get('schema') != stage.reference_selection.SCHEMA:
            raise ValueError('actual reference-selection provenance required')
        probe = batch._reuse_json(suite, prior.parent/'reference-probe.json')
        context_sources = [prior, prior.parent/'reference-probe.json']
        for key, name in (('recipe_info','recipe-info.json'), ('reference_probe','reference-probe.json'),
                ('upstream_tree_comparison','upstream-tree-comparison.json')):
            pin = selected.get('evidence_inputs', {}).get(key, {})
            expected_path = prior.parent/name
            if (pin.get('path') != str(expected_path)
                    or e.file_identity(e.safe_path(expected_path)) != pin.get('identity')):
                raise ValueError('prior reference evidence binding changed')
        recipe_path, tree_path = prior.parent/'recipe-info.json', prior.parent/'upstream-tree-comparison.json'
        external = {key: probe[key] for key in ('install_receipt', 'build_config')}
        patches = probe['patch_files']
        epoch, icon = selected['epoch'], probe.get('icon_setting')
        expected_reference = selected.get('selected_reference', {}).get('identity')
    else:
        recipe_path = e.safe_path(args.recipe_info)
        tree_path = e.safe_path(args.upstream_tree_comparison) if args.upstream_tree_comparison is not None else None
        external = {key: dict(path=str(e.safe_path(value)), identity=e.file_identity(e.safe_path(value)))
            for key, value in (('install_receipt', args.install_receipt), ('build_config', args.build_config))}
        recipe = batch._reuse_json(suite, recipe_path)
        directory = e.safe_path(args.patch_directory)
        if not directory.is_dir():
            raise ValueError('physical captured patch directory required')
        patches = {row['url']: dict(path=str(directory/(row['sha256']+'.patch')),
            identity=e.file_identity(e.safe_path(directory/(row['sha256']+'.patch'))))
            for row in recipe['formulae'][0]['patches']}
        epoch, icon, expected_reference, context_sources = args.epoch, None, None, []
    if not isinstance(epoch, str) or not epoch:
        raise ValueError('explicit nonempty reference epoch required')
    for pin in [*external.values(), *patches.values()]:
        if e.file_identity(e.safe_path(pin['path'])) != pin['identity']:
            raise ValueError('reference provenance input changed')
    return recipe_path, tree_path, external, patches, epoch, icon, expected_reference, context_sources


def run_probe(stage, args):
    e, reference_tool = stage.execution, stage.reference_selection
    retirement, suite = e.load_tool('runtime-retire.py'), e.load_tool('stable-suite.py')
    batch = suite._load_batch()
    target = e.safe_path(args.out, must_exist=False)
    if not target.is_relative_to(stage.REPO/'build') or target == stage.REPO/'build':
        raise ValueError('fresh reference evidence must be inside the owned build directory')
    original, source, product_list, products, build = stage.read_build(args.build)
    recipe_path, tree_path, external, patch_sources, epoch, icon, expected, context = provenance(stage, args, suite, batch)
    fixture = e.safe_path(ROOT/'tools/fixtures/reference-install-probe.el')
    sources = [Path(__file__), fixture, recipe_path, *context,
               *[Path(module.__file__) for module in (stage, e, reference_tool, stage.owner, retirement, suite, batch)],
               *([tree_path] if tree_path is not None else []),
               *[e.safe_path(pin['path']) for pin in external.values()],
               *[e.safe_path(pin['path']) for pin in patch_sources.values()]]
    inputs = {str(p): batch._reuse_identity(suite, p) for p in sources}
    before_all = fingerprints(e, products)
    reference = products[0]
    before = e.product_identity(reference, resources=True)
    if args.prior_selection is not None and before != expected:
        raise ValueError('installed reference differs from prior evidence; explicit current provenance required')
    out = e.new_output(target)
    started = time.monotonic()
    checks, errors, process, summary = {}, [], None, {}
    captured, probe_path, state = {}, out/'reference-probe.json', None
    with stage.owner.ArtifactRun(stage.REPO, out) as artifacts:
        try:
            (out/'patches').mkdir()
            copies = [(recipe_path, out/'recipe-info.json'), (fixture, out/'reference-install-probe.el')]
            if tree_path is not None:
                copies.append((tree_path, out/'upstream-tree-comparison.json'))
            for origin, destination in copies:
                batch._copy_reuse_artifact(suite, origin, destination)
                captured[str(destination)] = e.file_identity(destination)
            recipe = batch._reuse_json(suite, out/'recipe-info.json')
            if tree_path is None:
                destination = out/'upstream-tree-comparison.json'
                e.write_json(destination, tree_comparison(stage, args, recipe, build, out))
                captured[str(destination)] = e.file_identity(destination)
            patches = {}
            for patch in recipe['formulae'][0]['patches']:
                binding = patch_sources[patch['url']]
                if binding['identity']['sha256'] != patch['sha256']:
                    raise ValueError('reference patch disagrees with actual recipe')
                destination = out/'patches'/(patch['sha256']+'.patch')
                batch._copy_reuse_artifact(suite, e.safe_path(binding['path']), destination, limit=1024*1024)
                captured[str(destination)] = e.file_identity(destination)
                patches[patch['url']] = dict(path=str(destination), identity=captured[str(destination)])
            stage.owner.capture_artifacts(artifacts.journal, captured)
            case = out/'installation-case'
            case.mkdir()
            process = e.run_process([reference['program'], '--dump-file='+reference['dump'],
                '-Q', '--batch', '--load', out/'reference-install-probe.el'], case,
                e.owned_environment(case, reference), args.timeout, cwd=case)
            stdout = case/'stdout.txt'
            if (process['complete'] is not True or process['exit_code'] != 0
                    or process['remaining_group_terminated'] is not False or stdout.stat().st_size > 1024*1024):
                raise ValueError('actual reference sampling did not close successfully')
            lines = [line[len('FEMACS-REFERENCE-INSTALL '):] for line in stdout.read_text().split('\n')
                     if line.startswith('FEMACS-REFERENCE-INSTALL ')]
            if len(lines) != 1:
                raise ValueError('one actual reference observation required')
            facts = json.loads(lines[0])
            if facts.get('arithmetic') != 42:
                raise ValueError('actual reference arithmetic probe failed')
            for key in ('lisp_directory', 'data_directory'):
                facts[key] = str(e.safe_path(facts[key]))
            after = e.product_identity(reference, resources=True)
            if before != after:
                raise ValueError('reference product changed during sampling')
            value = dict(schema='femacs-reference-install-probe-v1', passed=True,
                repository_version_embedded=facts.get('emacs_repository_version') is not None,
                source_revision_provenance='pinned-recipe-and-install-receipt',
                patch_files=patches, process=process, product_identity_before=before,
                product_identity_after=after, sampled_facts=facts, **external,
                raw_output_identities={name: dict(path=str(case/name), identity=e.file_identity(case/name))
                    for name in ('stdout.txt', 'stderr.txt')},
                probe_source=dict(path=str(out/'reference-install-probe.el'),
                    identity=e.file_identity(out/'reference-install-probe.el')),
                icon_setting=icon, icon_setting_unchanged=True, actual_editor_execution=True,
                sampled_native_paths_are_observations_only=True)
            e.write_json(probe_path, value)
            state = retirement.retire(case, ('home', 'cache', 'config', 'state', 'tmp', 'eln', 'out'),
                probe_path, process)
        except Exception as error:
            errors.append(f'{type(error).__name__}: {error}')
        finally:
            operations = {
                'all61_product_identities_unchanged': lambda: before_all == fingerprints(e, products),
                'all61_complete_receipts_valid': lambda: len(e.load_products(product_list)) == 61,
                'source_export_unchanged': lambda: e.verify_source_export(
                    stage.build_source_owner(original, build), build['source_export']) == source,
                'driver_and_provenance_unchanged': lambda: all(batch._reuse_identity(suite, Path(p)) == pin
                    for p, pin in inputs.items()),
                'captured_inputs_unchanged': lambda: bool(captured) and all(e.file_identity(
                    e.safe_path(p)) == pin for p, pin in captured.items()),
                'installation_configuration_unchanged': lambda: all(e.file_identity(
                    e.safe_path(pin['path'])) == pin['identity'] for pin in external.values()),
                'case_state_retirement_complete': lambda: isinstance(state, dict) and state.get('status') == 'complete',
            }
            for key, operation in operations.items():
                try:
                    checks[key] = operation() is True
                except Exception as error:
                    checks[key] = False
                    errors.append(f'{key}: {type(error).__name__}: {error}')
        if not errors and all(checks.values()):
            try:
                summary['reference_selection'] = reference_tool.bind_current_reference(stage, original, out, epoch)
            except Exception as error:
                errors.append(f'reference-binding: {type(error).__name__}: {error}')
        result = dict(schema='femacs-current-matrix-reference-probe-v1',
            passed=not errors and all(checks.values()) and 'reference_selection' in summary,
            checks=checks, errors=errors, process=process, state_retirement=state,
            build_receipt=str(original), seconds=time.monotonic()-started,
            input_identities=inputs, product_fingerprints_before=before_all,
            actual_editor_execution=process is not None and process.get('pid') is not None,
            installation_changed=False, diagnostic_only=True,
            compatibility_accepted=False, performance_accepted=False, goal_complete=False, **summary)
        result['artifact_retention'] = artifacts.retain_current_files()
        e.write_json(out/'results.json', result)
    return 0 if result['passed'] else 1


def main(argv=None):
    argv = list(sys.argv[1:] if argv is None else argv)
    if argv and argv[0] == 'capture-inputs':
        return capture_inputs_main(argv[1:])
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--build', type=Path, required=True)
    inputs = parser.add_mutually_exclusive_group(required=True)
    inputs.add_argument('--prior-selection', type=Path)
    inputs.add_argument('--recipe-info', type=Path)
    tree = parser.add_mutually_exclusive_group()
    tree.add_argument('--upstream-tree-comparison', type=Path)
    tree.add_argument('--upstream-repository', type=Path)
    parser.add_argument('--candidate-data-commit')
    parser.add_argument('--install-receipt', type=Path)
    parser.add_argument('--build-config', type=Path)
    parser.add_argument('--patch-directory', type=Path)
    parser.add_argument('--epoch')
    parser.add_argument('--timeout', type=float, default=60)
    parser.add_argument('--out', type=Path, required=True)
    args = parser.parse_args(argv)
    direct = [args.install_receipt, args.build_config, args.patch_directory, args.epoch]
    if (not 0 < args.timeout <= 90 or (args.prior_selection is None and
            (not all(direct) or not (args.upstream_tree_comparison or args.upstream_repository)))
            or (args.upstream_repository is not None and (args.candidate_data_commit is None
                or not re.fullmatch('[0-9a-f]{40}', args.candidate_data_commit)))
            or (args.upstream_repository is None and args.candidate_data_commit is not None)
            or (args.prior_selection is not None and (any(value is not None for value in direct)
                or args.upstream_tree_comparison is not None or args.upstream_repository is not None))):
        parser.error('use one complete explicit provenance input set and a finite probe deadline')
    try:
        return run_probe(load_stage(), args)
    except (OSError, ValueError, KeyError, TypeError) as error:
        parser.error(str(error))


if __name__ == '__main__':
    raise SystemExit(main())
