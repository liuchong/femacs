"""Post-integration protections for the real permanent-fallback measurement mode.

Run under the runtime resource controller. These are intentional ordinary Lisp
errors, not crash/abort tests, and never load a real extension module.
"""
from pathlib import Path
import argparse
import importlib.util
import json
import math
import os
import sys

ROOT = Path(__file__).resolve().parent.parent
spec = importlib.util.spec_from_file_location('fallback_mode_execution', ROOT / 'tools/runtime-execution.py')
a = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = a
spec.loader.exec_module(a)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--products', type=Path, required=True)
    parser.add_argument('--out', type=Path, required=True)
    parser.add_argument('--timeout', type=float, default=20)
    options = parser.parse_args()
    if not math.isfinite(options.timeout) or options.timeout <= 0:
        parser.error('positive finite process timeout required')
    products = a.load_products(options.products)
    reference = [p for p in products if p['kind'] == 'reference']
    young = [p for p in products if p['kind'] != 'reference'
             and p['compiler_leg'] == 'normal' and p['collector'] == 'young']
    if len(reference) != 1 or len(young) != 1:
        parser.error('one official reference and one normal young product required')
    selected = reference + young
    out = a.new_output(options.out)
    fixture = ROOT / 'tools/fixtures/runtime-profile-fallback.el'
    before = {p['id']: a.product_identity(p, resources=True) for p in selected}
    source_hashes = {str(p): a.digest(p) for p in [fixture, ROOT / 'tools/runtime-execution.py', Path(__file__)]}
    cases = []
    for product in selected:
        for fault, message in [('missing-root', 'Owned runtime root is required'),
                               ('existing-input', 'Fallback input must be an absent owned path')]:
            directory = out / product['id'] / fault
            directory.mkdir(parents=True)
            env = a.owned_environment(directory, product)
            marker = directory / 'nonexistent-performance-producer.so'
            if fault == 'missing-root':
                env.pop('FEMACS_RUNTIME_TEST_ROOT')
            else:
                # An empty owned file must be rejected before module-load.
                marker.write_bytes(b'')
            row = a.run_process([product['program'], '--dump-file=' + product['dump'],
                                 '-Q', '--batch', '--load', str(fixture)],
                                directory, env, options.timeout)
            stderr = (directory / 'stderr.txt').read_text(errors='replace')
            stdout = (directory / 'stdout.txt').read_text(errors='replace')
            row.update(profile=product['id'], fault=fault,
                       passed=row['complete'] and row['exit_code'] == 255
                       and not row['remaining_group_terminated'] and message in stderr
                       and 'FEMACS-PROFILE ' not in stdout and not (directory / 'fallback-context.json').exists()
                       and (fault != 'existing-input' or marker.read_bytes() == b''))
            cases.append(row)
            a.write_json(directory / 'result.json', row)
    # Invalid action must be rejected before loading products or creating output.
    guarded = out / 'invalid-action-output'
    row = a.run_process([sys.executable, '-B', str(ROOT / 'tools/runtime-execution.py'),
                         '--action', 'ert', '--out', str(guarded), '--permanent-full-fallback'],
                        out / 'invalid-action', dict(os.environ), options.timeout)
    row['passed'] = (row['complete'] and row['exit_code'] == 2
                     and not row['remaining_group_terminated'] and not guarded.exists()
                     and 'applies only to performance measurement' in (out / 'invalid-action/stderr.txt').read_text())
    cases.append(row)
    a.write_json(out / 'invalid-action/result.json', row)
    unchanged = before == {p['id']: a.product_identity(p, resources=True) for p in selected}
    unchanged = unchanged and all(a.digest(Path(p)) == h for p, h in source_hashes.items())
    report = dict(passed=len(cases) == 5 and all(r['passed'] for r in cases) and unchanged,
                  cases=cases, full_inputs_unchanged=unchanged, source_sha256=source_hashes,
                  expected_ordinary_errors_retained=True, no_crash_or_abort_test=True)
    a.write_json(out / 'results.json', report)
    print(json.dumps(dict(passed=report['passed'], processes=len(cases))))
    return 0 if report['passed'] else 1


if __name__ == '__main__':
    raise SystemExit(main())
