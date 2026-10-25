#!/usr/bin/env python3
"""Explicit local G4 integration recipes; written before production, run only in D.

Choose one layer. Components/build/host invoke compilers and need external build
resource protection; device invokes an emulator and needs runtime protection.
This runner never starts remote CI, installs dependencies or nests a controller.
Native macOS input/IME/Finder acceptance remains a separate real desktop task.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parent.parent

INFO_OBSERVATION = r'''
(progn
  (require 'json)
  (require 'info)
  (info-initialize)
  (let* ((manifest (json-read-file (getenv "FEMACS_MANUAL_MANIFEST")))
         (manuals (append (cdr (assq 'manuals manifest)) nil))
         (resource-root (file-truename (getenv "FEMACS_EXPECT_INFO")))
         (observations nil)
         (info-read-files nil)
         (read-observer (lambda (result)
                          (when (stringp (car-safe result))
                            (push (car result) info-read-files))
                          result)))
    (unless (member (file-name-as-directory resource-root)
                    (mapcar (lambda (path)
                              (file-name-as-directory (file-truename path)))
                            Info-directory-list))
      (error "Default Info search does not include the delivered resource tree"))
    ;; Info-current-file deliberately preserves the requested logical stem.
    ;; Observe actual file reads rather than guessing its optional suffix.
    (advice-add 'insert-file-contents :filter-return read-observer)
    (unwind-protect
        (progn
          (dolist (manual manuals)
            (let* ((name (cdr (assq 'info_name manual)))
                   (file (expand-file-name (concat name ".info") resource-root)))
              (unless (file-exists-p file) (error "Missing manual %s" name))
              (setq info-read-files nil)
              (Info-find-node name "Top")
              (let ((actual-file (car info-read-files)))
                (unless (and actual-file
                             (file-equal-p actual-file file)
                             (string-equal (downcase Info-current-node) "top"))
                  (error "Wrong manual node/resource %s: logical=%s actual=%s"
                         name Info-current-file actual-file))
                (push `((name . ,name) (node . ,Info-current-node)
                        (logical_file . ,Info-current-file)
                        (file . ,(file-truename actual-file))) observations))))
          (setq info-read-files nil)
          (Info-find-node "emacs" "Basic")
          (unless (and (equal Info-current-node "Basic")
                       (car info-read-files)
                       (file-equal-p (car info-read-files)
                                     (expand-file-name "emacs.info" resource-root)))
            (error "Non-Top lookup failed"))
          ;; Node variables are buffer-local; capture before switching buffers.
          (let ((observed-non-top-node Info-current-node))
            (with-temp-file (getenv "FEMACS_INFO_REPORT")
              (insert (json-encode `((schema . "femacs-info-observation-v1")
                                     (pid . ,(emacs-pid))
                                     (manuals . ,(vconcat (nreverse observations)))
                                     (non_top_node . ,observed-non-top-node)))))))
      (advice-remove 'insert-file-contents read-observer))))
'''


def digest(path: Path) -> str:
    h = hashlib.sha256()
    with path.open('rb') as source:
        for block in iter(lambda: source.read(1024 * 1024), b''):
            h.update(block)
    return h.hexdigest()


def owned_fresh(path: Path) -> Path:
    path = path.resolve()
    if not path.is_relative_to((ROOT / 'build').resolve()):
        raise ValueError('acceptance output must be a fresh owned build child')
    if path == (ROOT / 'build').resolve() or path.is_relative_to((ROOT / 'emacs').resolve()):
        raise ValueError('unsafe acceptance output')
    path.mkdir(parents=True, exist_ok=False)
    return path


def source_files(repo: Path) -> list[str]:
    proc = subprocess.run(['git', 'ls-files', '-z', '--cached', '--others',
                           '--exclude-standard'], cwd=repo, check=True,
                          stdout=subprocess.PIPE)
    names = sorted(set(os.fsdecode(row) for row in proc.stdout.split(b'\0') if row))
    # The reference/cache/output trees are never export inputs, even with bad ignores.
    excluded = {'.git', 'emacs', 'build', 'target', 'dist', '.femacs-cache', 'info'}
    return [name for name in names if Path(name).parts[0] not in excluded
            and not Path(name).parts[0].startswith('.info.')
            and (repo / name).is_file()]


def snapshot(repo: Path) -> dict[str, str]:
    return {name: digest(repo / name) for name in source_files(repo)}


class Runner:
    def __init__(self, out: Path, repo: Path):
        self.out, self.repo = out, repo
        self.cases: list[dict] = []

    def run(self, name: str, args: list[str | Path], *, cwd: Path | None = None,
            env: dict[str, str] | None = None, cases: list[str] | None = None) -> bool:
        if not name or any(c not in 'abcdefghijklmnopqrstuvwxyz0123456789-_' for c in name):
            raise ValueError('invalid evidence name')
        start = time.monotonic()
        argv = [str(arg) for arg in args]
        log = self.out / (name + '.log')
        with log.open('xb') as output:
            try:
                p = subprocess.run(argv, cwd=cwd or self.repo, env=env,
                                   stdout=output, stderr=subprocess.STDOUT)
                code = p.returncode
            except OSError as exc:
                output.write(str(exc).encode('utf-8', errors='replace'))
                code = 127
        result = {'id': name, 'cases': cases or [], 'argv': argv,
                  'exit_code': code, 'passed': code == 0,
                  'seconds': time.monotonic() - start, 'log': log.name}
        self.cases.append(result)
        print(f'{name}: exit={code} seconds={result["seconds"]:.3f}', flush=True)
        return code == 0

    def observation(self, name: str, passed: bool, detail: dict) -> None:
        self.cases.append({'id': name, 'passed': passed, 'detail': detail})

    def xtask(self, name: str, args: list[str | Path], **kw) -> bool:
        return self.run(name, ['cargo', 'run', '-q', '-p', 'xtask', '--', *args], **kw)


def components(r: Runner) -> None:
    env=dict(os.environ,CARGO_TARGET_DIR=str(r.repo/'build/cargo'))
    r.run('rust-components', ['cargo', 'test', '-p', 'xtask', '--', '--nocapture',
                              '--test-threads=1'], env=env, cases=['T04-T15', 'T23-T30', 'T36-T40'])
    r.run('python-components', ['python3', 'tools/test-tooling-modernization.py'],
          cases=['T01-T48 component contracts'])
    r.run('workflow-components', ['python3', 'tools/test-g4-workflows.py'],
          cases=['T27-T39 failures/ownership', 'T42-T44 recovery/support'])
    r.run('review-regressions', ['python3', 'tools/test-g4-review-regressions.py'],
          cases=['D-review import command failures/Org staging/cache identity'])
    r.run('ert-wrapper-contracts', ['python3', 'tools/test-ert-harness.py'],
          cases=['T16-T20 retained contracts'])
    r.run('android-keysym-capacity', ['python3', 'tools/test-android-keysym-capacity.py'],
          cases=['P-344 actual extracted Rust function; canaries and controlled JNI paths'])
    r.run('keyboard-platform-offset', ['python3', 'tools/test-keyboard-platform-offset.py'],
          cases=['P-345 actual extracted platform constants and Android key table'])


def clean_environment() -> dict[str, str]:
    env = dict(os.environ)
    for key in list(env):
        if key.startswith('FEMACS_') or key in {
                'EMACSDATA', 'EMACSDOC', 'EMACSLOADPATH', 'EMACSPATH',
                'CARGO_TARGET_DIR', 'RUSTFLAGS', 'RUSTC_WRAPPER', 'INFOPATH'}:
            env.pop(key, None)
    return env


def info_lookup(r: Runner, name: str, program: Path, info: Path,
                manifest: Path, env: dict[str, str]) -> bool:
    receipt = r.out / (name + '.json')
    run_env = dict(env, FEMACS_MANUAL_MANIFEST=str(manifest),
                   FEMACS_EXPECT_INFO=str(info), FEMACS_INFO_REPORT=str(receipt))
    ran = r.run(name, [program, '-Q', '--batch', '--eval', INFO_OBSERVATION],
                cwd=r.out, env=run_env, cases=['T39-T40'])
    detail = {'report_written': receipt.is_file()}
    passed = False
    if ran and receipt.is_file():
        payload = json.loads(receipt.read_text())
        expected = json.loads(manifest.read_text())['manuals']
        actual = payload.get('manuals', [])
        passed = (payload.get('schema') == 'femacs-info-observation-v1'
                  and {row['name'] for row in actual} == {row['info_name'] for row in expected}
                  and len(actual) == len(expected)
                  and all(row.get('node','').lower() == 'top'
                          and Path(row.get('file','')).resolve() == (info / (row['name']+'.info')).resolve()
                          for row in actual)
                  and payload.get('non_top_node') == 'Basic'
                  and isinstance(payload.get('pid'), int))
        detail.update(count=len(actual), expected=len(expected), non_top_node=payload.get("non_top_node"),
                      wrong_resources=[row.get("name") for row in actual if Path(row.get("file", "")).resolve() != (info/(row["name"]+".info")).resolve()])
    r.observation(name + '-assertions', passed, detail)
    return passed


def independent_build(r: Runner) -> None:
    before = snapshot(r.repo)
    export = r.out / 'source-export'
    export.mkdir()
    for name in before:
        source = r.repo / name
        if source.is_symlink():
            raise ValueError(f'tracked export input needs explicit symlink policy: {name}')
        target = export / name
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(source, target)
    (r.out / 'export-inputs.json').write_text(json.dumps(before, indent=2) + '\n')
    if (export / 'emacs').exists():
        raise ValueError('reference entered independent export')
    env = clean_environment()
    env['CARGO_TARGET_DIR'] = str(export / 'build/cargo')
    terminal = r.xtask('clean-terminal-build', ['build-terminal'], cwd=export, env=env,
                       cases=['T36-T40', 'T44 fresh independent build'])
    if terminal:
        r.run('terminal-ccl-regressions', [export / 'build/terminal/port/src/emacs',
              '--batch', '-Q', '-l', export / 'test/lisp/international/ccl-tests.el',
              '-l', export / 'tools/fixtures/ccl-finish.el',
              '-f', 'ert-run-tests-batch-and-exit'], cwd=export, env=env,
              cases=['P-340 actual CCL success/suspension/error and fixture serialization'])
    cocoa = r.xtask('clean-cocoa-build', ['build-macos'], cwd=export, env=env,
                    cases=['T36-T40', 'T44 fresh independent build'])
    if cocoa:
        r.run('cocoa-ccl-regressions', [export / 'dist/macos/Femacs.app/Contents/MacOS/Femacs',
              '--batch', '-Q', '-l', export / 'test/lisp/international/ccl-tests.el',
              '-l', export / 'tools/fixtures/ccl-finish.el',
              '-f', 'ert-run-tests-batch-and-exit'], cwd=export, env=env,
              cases=['P-340 Cocoa-configured actual CCL and serializer component; not native GUI acceptance'])
    r.xtask('clean-android-build', ['android', 'build', '--assets', '--package',
             '--debuggable', '--out', export / 'build/android-acceptance'],
             cwd=export, env=env, cases=['T23-T30 reference-free native/package build'])
    manifest = export / 'build-support/manuals.json'
    if terminal:
        info_lookup(r, 'terminal-info', export / 'build/terminal/port/src/emacs',
                    export / 'info', manifest, env)
    if cocoa:
        app = export / 'dist/macos/Femacs.app'
        moved = r.out / 'relocated/Femacs.app'
        moved.parent.mkdir()
        shutil.copytree(app, moved, symlinks=False)
        manifest_copy = r.out / 'manuals-manifest.json'
        shutil.copy2(manifest, manifest_copy)
        binary = moved / 'Contents/MacOS/Femacs'
        dump = moved / 'Contents/Resources/libexec/Femacs.pdmp'
        identities = {'binary': digest(binary), 'dump': digest(dump)}
        hidden = r.out / 'owned-source-unavailable'
        inode = export.stat().st_ino
        export.rename(hidden)
        try:
            info_lookup(r, 'relocated-info', binary,
                        moved / 'Contents/Resources/info', manifest_copy, env)
        finally:
            hidden.rename(export)
        r.observation('relocation-integrity',
                      identities == {'binary': digest(binary), 'dump': digest(dump)}
                      and export.stat().st_ino == inode,
                      {'identities': identities, 'owned_source_restored': export.is_dir()})
    r.observation('source-preservation', snapshot(r.repo) == before,
                  {'input_count': len(before), 'reference_exported': False})


def host(r: Runner, args) -> None:
    for name in ['va', 'syshdrs', 'fam', 'shadow', 'macro', 'miri']:
        r.xtask('probe-' + name, ['audit-probe', name, '--out', r.out / ('probe-' + name)],
                 cases=['T13-T15 actual compiler/runtime'])
    if args.reference_source:
        for name in ['inline', 'accessor', 'write', 'gmp-abi', 'lisp-abi', 'bringup']:
            command = ['audit-probe', name, '--out', r.out / ('probe-' + name),
                       '--reference-source', args.reference_source]
            if args.reference_build:
                command += ['--reference-build', args.reference_build]
            r.xtask('probe-' + name, command, cases=['T13-T15 explicit reference audit'])
    r.xtask('jni-ndk-probe', ['audit-probe', 'jni', '--out', r.out / 'jni'],
             cases=['T13-T15 NDK ABI'])
    r.xtask('kotlin-probe', ['audit-probe', 'kotlin', '--out', r.out / 'kotlin'],
             cases=['T13-T15 own Kotlin closure'])
    r.run('api-floor-matrix', ['python3', 'tools/test-g4-platform-floor.py',
          '--toolchain-report', r.out / 'kotlin/kotlin/report.json',
          '--out', r.out / 'api-floor'], cases=['T45-T48 actual method branch equivalence'])
    # The clean build layer exercises the complete native/package build.
    # Use the aggregate's static leg here; dispatch/failure contracts cover its other
    # selections without rebuilding the same native inputs a second time.
    r.xtask('platform-host-gates', ['gate-platforms', '--android-static',
             '--out', r.out / 'platform-gates'], cases=['T23-T30 aggregate static leg'])
    r.xtask('manual-html', ['build-manuals', '--format', 'html', '--program', args.program,
             '--out', r.out / 'manual-html'], cases=['T36-T38 optional HTML'])


def device(r: Runner, args) -> None:
    command = ['python3', 'tools/android-device.py', 'accept', '--apk', args.apk,
               '--so', args.so, '--out', r.out / 'device-accept']
    r.run('android-device-accept', command, cases=['T31-T35', 'T47-T48'])
    r.run('current-readiness', ['python3', 'tools/android-readiness.py', '--require',
          '--receipt', r.out / 'device-accept/receipt.json', '--apk', args.apk,
          '--so', args.so], cases=['T35 current bound acceptance'])
    batch = r.run('android-batch-diagnostic', ['python3', 'tools/android-device.py', 'batch',
          '--apk', args.apk, '--so', args.so, '--out', r.out / 'batch',
          '--eval', '(list (+ 1 2) (string-match-p "b" "abc"))'],
          cases=['T35 real diagnostic, not GUI acceptance'])
    with (r.out / 'android-batch-diagnostic.log').open(errors='replace') as log:
        exact_lines = sum(line.strip() == '(3 1)' for line in log)
    r.observation('android-batch-exact-result', batch and exact_lines == 1,
                  {'exact_result_lines': exact_lines, 'expected': '(3 1)',
                   'scope': 'batch diagnostic; not GUI acceptance'})


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--layer', required=True, choices=('components', 'build', 'host', 'device'))
    parser.add_argument('--out', type=Path, required=True)
    parser.add_argument('--reference-source', type=Path)
    parser.add_argument('--reference-build', type=Path)
    parser.add_argument('--program', type=Path)
    parser.add_argument('--apk', type=Path)
    parser.add_argument('--so', type=Path)
    args = parser.parse_args(argv)
    if args.layer == 'host' and not args.program:
        parser.error('host layer requires the current owned --program for manual export')
    if args.layer == 'device' and not (args.apk and args.so):
        parser.error('device layer requires current --apk and --so')
    for name in ('program', 'apk', 'so', 'reference_source', 'reference_build'):
        value = getattr(args, name)
        if value is not None:
            setattr(args, name, value.resolve())
    out = owned_fresh(args.out)
    runner = Runner(out, ROOT)
    started = time.monotonic()
    error = None
    try:
        if args.layer == 'components': components(runner)
        elif args.layer == 'build': independent_build(runner)
        elif args.layer == 'host': host(runner, args)
        else: device(runner, args)
    except BaseException as exc:
        error = f'{type(exc).__name__}: {exc}'
        runner.observation('incomplete-layer', False, {'error': error})
    passed = bool(runner.cases) and all(row['passed'] for row in runner.cases) and error is None
    receipt = {'schema': 'femacs-g4-acceptance-v1', 'layer': args.layer,
               'passed': passed, 'cases': runner.cases, 'error': error,
               'seconds': time.monotonic() - started, 'remote_ci_used': False,
               'macos_native_input_requested': False}
    (out / 'receipt.json').write_text(json.dumps(receipt, ensure_ascii=False, indent=2) + '\n')
    print(f'{args.layer}: {"PASS" if passed else "FAIL"}; {out / "receipt.json"}')
    return 0 if passed else 1


if __name__ == '__main__':
    raise SystemExit(main())
