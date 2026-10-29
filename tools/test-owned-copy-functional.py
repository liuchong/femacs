"""Actual Rust/Python copy + macOS preservation integration; use build protection.

Compiles the actual owned_copy module, not a substitute implementation. Tiny
owned files exercise copy/flag/permission boundaries, not editor behavior.
"""
from pathlib import Path
import argparse
import importlib.util
import json
import os
import signal
import stat
import subprocess
import sys
import time
import traceback

REPO = Path(__file__).resolve().parent.parent


def module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    value = importlib.util.module_from_spec(spec)
    sys.modules[name] = value
    spec.loader.exec_module(value)
    return value


execution = module('owned_copy_actual_execution', REPO / 'tools/runtime-execution.py')
owner = module('owned_copy_actual_owner', REPO / 'tools/runtime-owner.py')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out', type=Path, required=True)
    parser.add_argument('--case', choices=('rust', 'python'))
    options = parser.parse_args()
    if sys.platform != 'darwin':
        parser.error('actual macOS copy/immutable behavior required')
    out = execution.new_output(options.out)
    actual_module = REPO / 'xtask/src/owned_copy.rs'
    actual_hash = execution.digest(actual_module)
    probe = out / 'actual-owned-copy.rs'
    probe.write_text('#[path = ' + json.dumps(str(actual_module)) + '] mod selected;\n' + '''
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let source = std::path::Path::new(&args[2]);
    let target = std::path::Path::new(&args[3]);
    let result = if args[1] == "platform-control" {
        std::fs::copy(source, target).map(|_| ()).map_err(|e| e.to_string())
    } else { selected::copy_file(source, target) };
    if let Err(error) = result { eprintln!("{error}"); std::process::exit(1); }
}
''')
    binary = out / 'actual-owned-copy'
    compiled = execution.run_process(['rustc', '--edition', '2024', probe, '-o', binary],
                                     out / 'compile', dict(os.environ), None, cwd=REPO)
    if compiled['exit_code'] != 0:
        raise ValueError('actual copy module did not compile')
    rows = []

    def run_case(name, body):
        directory = out / name
        directory.mkdir()
        started = time.monotonic()
        try:
            row = dict(id=name, passed=True, details=body(directory))
        except BaseException as error:
            row = dict(id=name, passed=False, error=str(error), traceback=traceback.format_exc())
        row['seconds'] = time.monotonic() - started
        rows.append(row)
        print(json.dumps({k: row[k] for k in ('id', 'passed', 'seconds')}), flush=True)

    def invoke(directory, mode, source, target):
        result = execution.run_process([binary, mode, source, target], directory,
                                       dict(os.environ), 15, cwd=REPO)
        assert result['complete'] and not result['remaining_group_terminated']
        return result

    def rust_copy(directory):
        source = directory / 'selected-input'
        source.write_bytes(b'owned source bytes\n' * 17000)
        source.chmod(0o640)
        before = execution.file_identity(source)
        control, selected = directory / 'platform-copy', directory / 'owned-copy'
        alias = directory / 'same-inode-alias'
        os.link(source, alias)
        try:
            with owner.ArtifactRun(REPO, directory) as guard:
                owner.capture_artifacts(guard.journal, {str(source): before})
                assert source.stat().st_flags & stat.UF_IMMUTABLE
                assert invoke(directory / 'platform', 'platform-control', source, control)['exit_code'] == 0
                assert execution.file_identity(control) == before
                inherited = bool(control.stat().st_flags & stat.UF_IMMUTABLE)
                assert inherited, 'actual platform control did not reproduce the observed propagation'
                assert invoke(directory / 'selected', 'owned', source, selected)['exit_code'] == 0
                assert execution.file_identity(selected) == before
                assert not selected.stat().st_flags & stat.UF_IMMUTABLE
                assert source.stat().st_flags & stat.UF_IMMUTABLE
                for name, target in [('same-input', source), ('same-inode', alias)]:
                    refused = invoke(directory / name, 'owned', source, target)
                    assert refused['exit_code'] != 0
                    assert 'source inode as destination' in (directory / name / 'stderr.txt').read_text()
                assert execution.file_identity(source) == before
            assert not source.stat().st_flags & stat.UF_IMMUTABLE
            return dict(platform_flag_propagation_reproduced=True, real_selected_module_used=True,
                        exact_bytes_and_mode=True, copied_flags_clear=True, selected_input_unchanged=True,
                        source_and_hardlink_alias_refused=True)
        finally:
            # Only the control copy created here inherited our input guard bit.
            if control.exists() and control.stat().st_flags & stat.UF_IMMUTABLE:
                assert execution.file_identity(control) == before
                os.chflags(control, control.stat().st_flags & ~stat.UF_IMMUTABLE)

    def python_copy(directory):
        package = directory / 'selected-package'
        package.mkdir()
        source = package / 'resource'
        source.write_bytes(b'actual protected package resource\n')
        source.chmod(0o600)
        expected = execution.file_identity(source)
        with owner.ArtifactRun(REPO, directory) as guard:
            owner.capture_artifacts(guard.journal, {str(source): expected})
            copied = directory / 'single-copy'
            execution.copy_owned_data(source, copied)
            assert execution.file_identity(copied) == expected and not copied.stat().st_flags & stat.UF_IMMUTABLE
            import shutil
            moved = directory / 'relocated-package'
            shutil.copytree(package, moved, symlinks=True, copy_function=execution.copy_owned_data)
            assert execution.tree_identity(moved) == execution.tree_identity(package)
            assert not (moved / 'resource').stat().st_flags & stat.UF_IMMUTABLE
            assert source.stat().st_flags & stat.UF_IMMUTABLE
        assert not source.stat().st_flags & stat.UF_IMMUTABLE
        return dict(actual_python_copy_and_relocation_path=True, exact_bytes_and_mode=True,
                    output_flags_clear=True, input_protection_retained_until_release=True)

    if options.case in (None, 'rust'):
        run_case('actual-rust-copy-boundary', rust_copy)
    if options.case in (None, 'python'):
        run_case('actual-python-relocation-copy', python_copy)
    report = dict(passed=all(row['passed'] for row in rows), complete=True, cases=rows,
                  selected_scope=options.case or 'rust-and-python',
                  actual_module=str(actual_module), module_sha256=actual_hash,
                  module_unchanged=execution.digest(actual_module) == actual_hash,
                  compiled_process=compiled, scope='actual owned copy module and OS flags only',
                  editor_native_gc_compatibility_performance_accepted=False)
    report['passed'] = report['passed'] and report['module_unchanged']
    execution.write_json(out / 'results.json', report)
    return 0 if report['passed'] else 1


if __name__ == '__main__':
    raise SystemExit(main())
