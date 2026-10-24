#!/usr/bin/env python3
"""Bounded latest-stable differential acceptance, run explicitly under controlled-run.

No building, fetching, GUI input, remote CI, or fallback reference selection. The
required reference's actual version is checked; candidate versions are recorded.
"""
from __future__ import annotations
import argparse
from dataclasses import asdict
import importlib.util
import json
import math
import os
from pathlib import Path
import plistlib
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parent.parent
SCHEMA = 'femacs-stable-language-v1'
CASES = (
    'sl01-lexical-closure-and-redefinition',
    'sl02-dynamic-binding-and-unwind',
    'sl03-multibyte-markers-and-narrowing',
    'sl04-bytecode-evaluation-order-and-error',
    'sl05-weak-table-closure-and-finalizer-roots',
    'sl06-reader-printer-and-circular-data',
)
FIXTURE = ROOT/'tools/fixtures/stable-language.el'


def load_module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


baseline = load_module('stable_release_baseline', ROOT/'tools/test-release-baseline.py')
ert = load_module('stable_ert', ROOT/'tools/check-ert.py')

def resolve_product(path):
    """Bind a real executable, including an explicit .app's declared entry."""
    path=Path(path).resolve()
    if path.is_dir():
        with (path/'Contents/Info.plist').open('rb') as source:
            name=plistlib.load(source).get('CFBundleExecutable')
        if (not isinstance(name,str) or not name or name in ['.','..']
                or Path(name).name!=name):
            raise ValueError('bundle executable must be a simple declared filename')
        path=(path/'Contents/MacOS'/name).resolve()
    with path.open('rb') as source:magic=source.read(4)
    if magic not in [b'\xcf\xfa\xed\xfe',b'\xfe\xed\xfa\xcf',b'\xce\xfa\xed\xfe',
                     b'\xfe\xed\xfa\xce',b'\xca\xfe\xba\xbe',b'\xbe\xba\xfe\xca',b'\x7fELF']:
        raise ValueError('request the real native executable or .app, not a launcher/wrapper')
    return path


def parse_version(text, expected=None):
    lines = [line.removeprefix('FEMACS-STABLE-VERSION ')
             for line in text.splitlines() if line.startswith('FEMACS-STABLE-VERSION ')]
    if len(lines) != 1:
        raise ValueError(f'expected exactly one version record, got {len(lines)}')
    record = json.loads(lines[0])
    if (not isinstance(record, dict)
            or any(not isinstance(record.get(key), str) or not record[key]
                   for key in ['version', 'system', 'configuration'])):
        raise ValueError('version/system/configuration must be nonempty strings')
    if expected is not None and record['version'] != expected:
        raise ValueError(f'reference version {record["version"]!r} != required {expected!r}')
    return record


def validate_observations(payload):
    if not isinstance(payload, dict) or payload.get('schema') != SCHEMA:
        raise ValueError('invalid stable-language observation schema')
    observations = payload.get('observations')
    if not isinstance(observations, dict) or set(observations) != set(CASES):
        raise ValueError('stable-language observation identities differ')
    if any(not isinstance(value, str) or not value for value in observations.values()):
        raise ValueError('observations must be nonempty printed Lisp values')
    return observations


def run_errors(run):
    errors = []
    if (run.status != 'ok' or run.exit_code != 0 or run.ran != len(CASES)
            or run.expected != len(CASES) or run.unexpected != 0):
        errors.append(f'incomplete/non-passing ERT run: {run.status}, exit={run.exit_code}, ran={run.ran}')
    if set(run.results) != set(CASES) or any(value != 'passed' for value in run.results.values()):
        errors.append('exact case identities and zero skips/failures are required')
    for name in CASES:
        seconds = run.test_seconds.get(name)
        if (isinstance(seconds, bool) or not isinstance(seconds, (int, float))
                or not math.isfinite(seconds) or seconds < 0):
            errors.append(f'missing/invalid elapsed seconds: {name}')
    return errors


def comparison_errors(observations):
    reference = observations.get('reference')
    errors = []
    if not isinstance(reference, dict) or set(reference) != set(CASES):
        return ['missing complete reference observations']
    for side, values in observations.items():
        if not isinstance(values, dict) or set(values) != set(CASES):
            errors.append(f'{side}: missing/extra observation identities')
            continue
        for name in CASES:
            if values[name] != reference[name]:
                errors.append(f'{side}: observation differs from reference: {name}')
    return errors


def artifact_errors(before, after):
    return [f'{name}: product/dump fingerprint changed' for name in set(before) | set(after)
            if before.get(name) != after.get(name)]


def query_version(binary, out, expected, timeout):
    command = [str(binary), '-Q', '--batch', '--eval',
               '(progn (require \'json) (princ "FEMACS-STABLE-VERSION ") '
               '(princ (json-serialize (list :version emacs-version '
               ':system (symbol-name system-type) :configuration system-configuration))) (terpri))']
    home = out/'home'; home.mkdir()
    env = {'HOME':str(home), 'PATH':os.defpath, 'LC_ALL':'C'}
    started = time.monotonic()
    log = out/'version.log'
    with log.open('xb') as output:
        process = subprocess.run(command, cwd=home, env=env, stdout=output,
                                 stderr=subprocess.STDOUT, timeout=timeout)
    receipt = {'argv':command, 'exit_code':process.returncode,
               'seconds':time.monotonic()-started, 'log':str(log)}
    (out/'version-command.json').write_text(json.dumps(receipt, indent=2)+'\n')
    if process.returncode != 0 or log.stat().st_size > 1024*1024:
        raise ValueError(f'version command failed or exceeded bounded output: {log}')
    return parse_version(log.read_text(), expected)


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--terminal', type=Path, default=ROOT/'build/terminal/port/src/emacs')
    ap.add_argument('--macos', type=Path, default=ROOT/'dist/macos/Femacs.app')
    ap.add_argument('--reference', type=Path, required=True)
    ap.add_argument('--reference-version', default='31.1')
    ap.add_argument('--out', type=Path, required=True)
    ap.add_argument('--timeout', type=int, default=90)
    args = ap.parse_args(argv)
    if args.timeout <= 0 or not args.reference_version.strip():
        ap.error('positive timeout and an explicit nonempty reference version are required')
    out = baseline.new_output(args.out)
    started = time.monotonic()
    report = {'schema':'femacs-stable-comparison-v1', 'passed':False,
              'required_reference_version':args.reference_version, 'errors':[], 'runs':{},
              'scope':'six language cases and ten release-baseline groups; macOS batch only'}
    products = {}
    before = {}
    try:
        products = {'terminal':resolve_product(args.terminal),
                    'macos':resolve_product(args.macos),
                    'reference':resolve_product(args.reference)}
        for name, binary in products.items():
            if not binary.is_file() or not os.access(binary, os.X_OK):
                raise ValueError(f'{name}: missing requested executable {binary}')
            before[name] = baseline.artifact_fingerprint(binary)
        report['fingerprints_before'] = before
        report['inputs'] = {str(path.relative_to(ROOT)):baseline.sha256_file(path)
                            for path in [Path(__file__).resolve(), FIXTURE,
                                         ROOT/'tools/fixtures/release-baseline.el',
                                         ROOT/'tools/check-ert.py', ROOT/'tools/test-release-baseline.py']}
        report['provenance'] = baseline.git_provenance()
        versions = {}
        for name, binary in products.items():
            work = out/name; work.mkdir()
            versions[name] = query_version(binary, work,
                args.reference_version if name == 'reference' else None, args.timeout)
        report['versions'] = versions
        # All actual versions are known before comparison. No development binary
        # can silently substitute for the required released version.
        ert.EVIDENCE_ROOT = out/'language-ert'
        observations = {}
        for name, binary in products.items():
            obs_path = out/name/'observations.json'
            prelude = '(setq femacs-stable-observation-file '+baseline.elisp_quote_string(str(obs_path))+')'
            run = ert.run(binary, FIXTURE.parent, FIXTURE.name, args.timeout,
                          selector='t', memory_mib=0, prelude=prelude)
            errors = run_errors(run)
            try:
                if obs_path.stat().st_size > 1024*1024:
                    raise ValueError('observation output exceeds bounded input')
                observations[name] = validate_observations(json.loads(obs_path.read_text()))
            except (OSError, ValueError) as error:
                errors.append(str(error)); observations[name] = {}
            report['runs'][name] = {'ert':asdict(run), 'observations':observations[name], 'errors':errors}
            report['errors'].extend(f'{name}: {error}' for error in errors)
        report['errors'].extend(comparison_errors(observations))
        release_out = out/'release-baseline'; release_out.mkdir()
        release = baseline.run_ert_suite(release_out, products['terminal'], products['macos'],
                                         products['reference'], args.timeout)
        report['release_baseline'] = release
        if not release['passed']:
            report['errors'].extend(f'release: {error}' for error in release['errors'])
    except (OSError, ValueError, subprocess.SubprocessError) as error:
        report['errors'].append(str(error))
    finally:
        if before:
            after = {name:baseline.artifact_fingerprint(binary) for name, binary in products.items()}
            report['fingerprints_after'] = after
            report['errors'].extend(artifact_errors(before, after))
        if 'inputs' in report:
            after_inputs = {rel:baseline.sha256_file(ROOT/rel) for rel in report['inputs']}
            if after_inputs != report['inputs']:
                report['errors'].append('test/driver inputs changed during acceptance')
        report['passed'] = not report['errors']
        report['seconds'] = time.monotonic()-started
        (out/'results.json').write_text(json.dumps(report, ensure_ascii=False, indent=2)+'\n')
    print(json.dumps(report, ensure_ascii=False))
    return 0 if report['passed'] else 1


if __name__ == '__main__':
    sys.exit(main())
