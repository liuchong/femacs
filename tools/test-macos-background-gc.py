"""Real Cocoa background/GC regression; run under controlled-run runtime.

Uses the existing isolated environment and process owner.  The two native
frames are required: --batch or a missing frame is a failure, never a pass.
This does not load personal configuration or certify arbitrary packages.
"""
import argparse
import importlib.util
import json
import os
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parent.parent


def load(name, filename):
    spec = importlib.util.spec_from_file_location(name, ROOT / 'tools' / filename)
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', type=Path, required=True)
    parser.add_argument('--reference', type=Path)
    parser.add_argument('--out', type=Path, required=True)
    args = parser.parse_args()
    out = args.out.absolute()
    if (not out.is_relative_to(ROOT / 'build') or out == ROOT / 'build'
            or any(p.is_symlink() for p in [out, *out.parents])):
        raise ValueError('fresh nonsymlink build child required')
    out.mkdir(parents=True, exist_ok=False)
    api = load('background_gc_execution', 'runtime-execution.py')
    suite = load('background_gc_suite', 'stable-suite.py')
    fixture = ROOT / 'tools/fixtures/macos-background-gc.el'
    fixture_before = api.digest(fixture)
    report = dict(scope='actual Cocoa setter, two frames, four colors and collection',
                  fixture_sha256=fixture_before, cases=[], passed=False)
    expected = {frame + '/' + color for frame in ['A', 'B']
                for color in ['#feffff', 'white', '#123456', 'black']}
    products = ([('reference', args.reference)] if args.reference else [])
    products += [('candidate', args.binary)]
    for label, binary in products:
        binary = binary.resolve(strict=True)
        work = out / label
        work.mkdir()
        before = api.file_identity(binary)
        env = {k: v for k, v in os.environ.items()
               if k not in api.FORBIDDEN_ENV and not k.startswith(('FEMACS_', 'DYLD_', 'FE_'))}
        env.update(suite.case_environment('background-gc-' + label, work / 'environment'))
        env['FEMACS_BACKGROUND_GC_REPORT'] = str(work / 'editor.json')
        process = api.run_process([binary, '-Q', '-l', fixture],
                                  work / 'process', env, 35, cwd=ROOT)
        observation = (api.bounded_json(work / 'editor.json')
                       if (work / 'editor.json').is_file() else {})
        rows = observation.get('cases', [])
        passed = (process['complete'] and process['exit_code'] == 0
                  and not process['remaining_group_terminated']
                  and observation.get('passed') is True and len(rows) == 8
                  and {row['id'] for row in rows} == expected
                  and all(row.get('passed') is True for row in rows)
                  and api.file_identity(binary) == before)
        report['cases'].append(dict(id=label, program=before, process=process,
                                    observation=observation, passed=passed))
        api.write_json(out / 'results.json', report)
    report['fixture_unchanged'] = api.digest(fixture) == fixture_before
    report['passed'] = report['fixture_unchanged'] and all(row['passed'] for row in report['cases'])
    api.write_json(out / 'results.json', report)
    print(json.dumps(dict(passed=report['passed'], products=len(report['cases']))))
    return 0 if report['passed'] else 1


if __name__ == '__main__':
    raise SystemExit(main())
