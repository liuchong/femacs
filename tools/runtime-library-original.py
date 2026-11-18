"""Collect the unchanged sidebar file on a bounded, receipt-verified product set.

This is a focused diagnostic, not a matrix build, classification catalog or
closed-file reuse donor. Every registered original test runs; failures remain
failures. Use runtime-frame-buffer-probe.py separately for the behavior contract.
"""
import argparse
import importlib.util
import os
from pathlib import Path
import time

ROOT = Path(__file__).resolve().parent.parent
LOGICAL = 'test/lisp/erc/erc-scenarios-status-sidebar.el'
IDS = {'erc-scenarios-status-sidebar--bufbar', 'erc-scenarios-status-sidebar--nickbar'}


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    result = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(result)
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--products', type=Path, required=True)
    parser.add_argument('--reference-inputs', type=Path, required=True)
    parser.add_argument('--source-export', type=Path, required=True)
    parser.add_argument('--timeout', type=float, default=90)
    parser.add_argument('--out', type=Path, required=True)
    args = parser.parse_args()
    if not 0 < args.timeout <= 300:
        parser.error('a finite original case timeout of at most300 seconds is required')
    stage = load('library_original_stage', ROOT/'tools/runtime-stage.py')
    e = stage.execution
    products = e.load_products(args.products)
    if (not 2 <= len(products) <= 3 or products[0]['id'] != 'installed-reference'
            or products[0]['kind'] != 'reference'
            or any(p['kind'] == 'reference' or p['test_hooks'] for p in products[1:])):
        parser.error('one installed reference and one or two non-diagnostic candidates required')
    try:
        _, reference_pins = stage.reference_selection.read_installation_inputs(
            e, args.reference_inputs, products[0])
    except (OSError, ValueError, KeyError, TypeError) as error:
        parser.error(str(error))
    source_export = e.bounded_json(args.source_export)
    source_owner = e.safe_path(args.source_export).parent
    source = e.verify_source_export(source_owner, source_export)
    for product in products[1:]:
        inputs = e.bounded_json(Path(product['product_receipt']).with_name('runtime-inputs.json'))
        if inputs.get('source_root') != str(source):
            parser.error('candidate build does not name the supplied frozen source')
    pins = {str(p): e.file_identity(p) for p in
            (Path(__file__).resolve(), args.products, args.reference_inputs, args.source_export)}
    pins.update(reference_pins)
    out = e.new_output(args.out)
    before = {p['id']: e.product_identity(p, resources=True) for p in products}
    started = time.monotonic()
    rows, errors, checks, hi, ri = [], [], {}, {}, {}
    suite = provider = manifest = context = record = None
    corpus = out/'corpus'
    with stage.owner.ArtifactRun(ROOT, out) as artifacts:
        try:
            harness, _, hi = stage.stage_stable_harness(out, source, artifacts)
            runner, _, ri = stage.stage_stable_runner(ROOT/'tools/fixtures/stable-suite-runner.el',
                                                     out, source, artifacts)
            suite = load('library_original_suite', harness/'tools/stable-suite.py')
            batch = suite._load_batch()
            manifest = suite.load_manifest(source/'compatibility/stable-31.1/sources.json', source)
            materialized = suite.materialize(manifest, source, corpus)
            suite._write_receipt(corpus, manifest, materialized)
            paths = {p['id']: suite._product_paths(p['program'], ['--dump-file='+p['dump']],
                stage.stable_product_environment(p, True)) for p in products}
            provider = load('library_original_context', harness/'tools/stable-context.py')
            context, record = provider.prepare(suite, source, corpus, manifest, paths)
            for product in products:
                result = batch._run_file(suite, corpus, paths[product['id']], runner,
                    corpus/'test', LOGICAL, args.timeout, product['id'], display='batch',
                    fixture_entries=batch._fixture_entries(manifest, LOGICAL))
                rows.append(dict(profile=product['id'], result=result))
                e.write_json(out/'partial.json', dict(cases=rows, diagnostic_only=True))
        except Exception as error:
            errors.append(type(error).__name__+': '+str(error)[:4096])
        finally:
            operations = {
                'selected_product_identities_unchanged': lambda: before == {
                    p['id']: e.product_identity(p, resources=True) for p in products},
                'selected_complete_receipts_valid': lambda: e.load_products(args.products) == products,
                'source_export_unchanged': lambda: e.verify_source_export(source_owner, source_export) == source,
                'declared_inputs_unchanged': lambda: all(e.file_identity(Path(p)) == v for p, v in pins.items()),
                'captured_harness_runner_unchanged': lambda: bool(hi) and bool(ri) and all(
                    e.file_identity(Path(p)) == v for p, v in {**hi, **ri}.items()),
                'original_corpus_unchanged': lambda: manifest is not None and
                    suite._load_batch()._source_inputs_unchanged(suite, manifest, corpus),
                'source_context_unchanged': lambda: record is not None and
                    provider.verify(suite, source, corpus, context, record) is None,
            }
            for name, operation in operations.items():
                try:
                    checks[name] = operation() is True
                except Exception as error:
                    checks[name] = False
                    errors.append(name+': '+type(error).__name__+': '+str(error)[:2048])
            complete = (not errors and all(checks.values()) and len(rows) == len(products)
                and all(row['result']['complete'] is True and set(row['result']['ids']) == IDS
                        for row in rows))
            pending = [dict(profile=row['profile'], root=row['result']['owned_state_root'],
                container=row['result']['owned_container']) for row in rows
                if os.path.lexists(row['result']['owned_container'])]
            e.write_json(out/'results.json', dict(schema='femacs-bounded-original-library-run-v1',
                passed=complete, diagnostic_collection_complete=complete,
                test_acceptance_passed=complete and all(row['result']['passed'] for row in rows),
                candidate_original_tests_passed=complete and all(row['result']['passed'] for row in rows[1:]),
                cases=rows, checks=checks, errors=errors, input_pins=pins,
                source_export=source_export, reference_inputs_sha256=e.digest(args.reference_inputs),
                file=LOGICAL, original_selector='all-discovered-ids', selected_products=len(products),
                owned_case_state_pending=pending, diagnostic_only=True, full_matrix_admission=False,
                original_results_modified=False, original_failure_waivers=0,
                compatibility_accepted=False, goal_complete=False, seconds=time.monotonic()-started,
                artifact_retention=artifacts.retain_current_files()))
    return 0 if complete else 1


if __name__ == '__main__':
    raise SystemExit(main())
