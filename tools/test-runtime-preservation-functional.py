"""Actual macOS preservation/producer/archive/release integration.

Tiny format-valid files exercise the owner/OS boundary, never an editor, native
backend, ABI or performance result. Intentional mutation/unlink assertions stay
inside this explicit fresh fixture output. No production inputs are modified.
"""
from pathlib import Path
import argparse
import hashlib
import importlib.util
import json
import os
import shutil
import signal
import stat
import subprocess
import sys
import tarfile
import time
import traceback
from types import SimpleNamespace

REPO = Path(__file__).resolve().parent.parent


def module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    result = importlib.util.module_from_spec(spec)
    sys.modules[name] = result
    spec.loader.exec_module(result)
    return result


stage = module('preservation_actual_stage', REPO / 'tools/runtime-stage.py')
owner, execution = stage.owner, stage.execution


def put(path, data):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(data)
    return path


def fixture(out):
    original = out / 'selected'
    source = original / 'source'
    source.mkdir(parents=True)
    for name in ('runtime-owner.py', 'runtime-execution.py'):
        put(source / 'tools' / name, (REPO / 'tools' / name).read_bytes())
    put(source / 'input.rs', b'owned preservation fixture, not executable code\n')
    (source / 'input-link').symlink_to('input.rs')
    names = ['input-link', 'input.rs', 'tools/runtime-execution.py', 'tools/runtime-owner.py']
    execution.write_json(source / '.femacs-runtime-sources.json', dict(schema=1, files=names))
    execution.write_json(original / 'source-inputs.json',
                         dict(schema=1, files={n: execution.file_identity(source / n) for n in names}))
    export = dict(source=str(source), file_count=len(names),
                  source_manifest_sha256=execution.digest(original / 'source-inputs.json'))
    root = source / 'build/runtime/fixture'
    root.mkdir(parents=True)
    binding = dict(product='terminal', variant='baseline', compiler_leg='normal',
                   observer=False, test_hooks=False)
    execution.write_json(root / 'runtime-binding.json', binding)
    execution.write_json(root / 'runtime-config.json', dict(schema=1, fixture_only=True))
    execution.write_json(root / 'runtime-inputs.json', dict(source_root=str(source)))
    info = source / 'build/manuals/fixture/info'
    put(info / 'fixture.info', b'fixture manual\n')
    put(root / 'config/fixture.h', b'fixture configuration\n')
    execution.write_json(root / 'config/manual-set.json', dict(info=str(info)))
    files = ['port/src/temacs', 'port/src/emacs', 'port/src/emacs.pdmp',
             'port/src/bootstrap-emacs.pdmp', 'dist/terminal/src/emacs', 'dist/terminal/src/emacs.pdmp']
    for name in files:
        put(root / name, b'nonexecuted product identity fixture\n')
    for name in ('lisp', 'etc', 'info'):
        put(root / 'dist/terminal' / name / 'fixture', b'fixture resource\n')
    obj = put(root / 'objects/fixture.o', b'not a real object; identity-only fixture\n')
    archive = put(root / 'load/fixture.a', b'not a real archive; identity-only fixture\n')
    link = dict(link_complete=True, objects={'objects/fixture.o': dict(
        **execution.file_identity(obj), role='fixture')},
        archive=dict(**execution.file_identity(archive), path='load/fixture.a'))
    execution.write_json(root / 'runtime-link-inputs.json', link)
    receipt = dict(schema=1, identity_version=2, build_complete=True, binding=binding, link_inputs=link,
                   file_identities={n: execution.file_identity(root / n) for n in files},
                   files={n: execution.digest(root / n) for n in files}, resources={
                       'generated-config': execution.tree_identity(root / 'config'),
                       'terminal-package': execution.tree_identity(root / 'dist/terminal'),
                       'info': execution.tree_identity(info)})
    execution.write_json(root / 'runtime-product.json', receipt)
    product = dict(id='fixture-normal', kind='terminal', compiler_leg='normal', native=False,
                   observer=False, test_hooks=False, collector='off',
                   program=str(root / files[4]), dump=str(root / files[5]),
                   product_receipt=str(root / 'runtime-product.json'), resource_roots={
                       n: str(root / 'dist/terminal' / n) for n in ('lisp', 'etc', 'info')})
    execution.write_json(original / 'products.json', dict(schema=1, products=[product]))
    execution.write_json(original / 'results.json', dict(action='build', passed=True,
                         source_root=str(source), source_export=export,
                         product_list=str(original / 'products.json'), fixture_only=True))
    return original, source, root, export


def wait_for(predicate, child, seconds=15):
    end = time.monotonic() + seconds
    while not predicate():
        if child.poll() is not None:
            raise AssertionError('owned worker exited before its handshake')
        if time.monotonic() >= end:
            raise AssertionError('owned worker handshake timed out')
        time.sleep(.02)


def flags_released(journal):
    value = execution.bounded_json(journal, 128 * 1024 * 1024)
    assert value['state'] == 'released', value.get('release_failures')
    for row in value['files']:
        assert owner.same_inode(row)
        assert Path(row['path']).stat().st_flags & stat.UF_IMMUTABLE == row['flags'] & stat.UF_IMMUTABLE
    return len(value['files'])


def cli(journal, *extra, env=None):
    return subprocess.run([sys.executable, REPO / 'tools/runtime-owner.py', *extra],
                          env=env, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)


def worker(mode, out):
    original, source, root, export = fixture(out)
    with owner.ArtifactRun(REPO, out) as guard:
        guard.select_source(original, export)
        owner.capture_product(guard.journal, root / 'runtime-product.json', source)
        if mode == 'journal-signal':
            original_dump = owner.json.dump
            interrupted = False

            def interrupt_write(value, stream, **kwargs):
                nonlocal interrupted
                if not interrupted:
                    interrupted = True
                    stream.write('{"interrupted":')
                    stream.flush()
                    os.kill(os.getpid(), signal.SIGTERM)
                return original_dump(value, stream, **kwargs)

            owner.json.dump = interrupt_write
            owner.artifact_write(guard.journal, owner.artifact_read(guard.journal))
            raise AssertionError('SIGTERM did not interrupt journal writing')
        if mode == 'dead':
            os._exit(19)
        argv = [sys.executable, '-c',
                'import os,time,pathlib;pathlib.Path(' + repr(str(out / 'producer-ready'))
                + ').write_text(str(os.getpid()));time.sleep(30)']
        execution.run_process(argv, out / 'producer', dict(os.environ), 35)


def normal(out):
    original, source, root, export = fixture(out)
    with owner.ArtifactRun(REPO, out) as guard:
        guard.select_source(original, export)
        env = dict(os.environ, HOME=str(out / 'isolated-home'))
        Path(env['HOME']).mkdir()
        result = cli(guard.journal, '--journal', str(guard.journal), '--product-receipt',
                     str(root / 'runtime-product.json'), '--source', str(source), env=env)
        assert result.returncode == 0, result.stderr
        program = root / 'dist/terminal/src/emacs'
        for operation in (lambda: program.unlink(), lambda: program.write_bytes(b'wrong')):
            try:
                operation()
            except PermissionError:
                pass
            else:
                raise AssertionError('real immutable artifact mutation was accepted')
        # Parent directory is deliberately writable for checking/result creation.
        put(root / 'checking/next-phase', b'next lane can be created\n')
        live = cli(guard.journal, '--release-journal', str(guard.journal))
        assert live.returncode != 0 and program.stat().st_flags & stat.UF_IMMUTABLE
        journal = execution.bounded_json(guard.journal, 128 * 1024 * 1024)
        assert any(r['path'] == str(source / 'input-link') for r in journal['symlinks'])
        assert not any(r['path'] == str(source / 'input-link') for r in journal['files'])
        guard.archive()
        journal = execution.bounded_json(guard.journal, 128 * 1024 * 1024)
        assert journal['archive']['all_members_verified']
        with tarfile.open(out / journal['archive']['path'], 'r:gz') as archive:
            contents = {m.name: archive.extractfile(m).read() for m in archive}
        assert json.loads(contents['symlinks.json']) == journal['symlinks']
        assert hashlib.sha256(contents['symlinks.json']).hexdigest() == journal['archive']['symlink_manifest']['sha256']
        assert len(contents) < len(journal['files']), 'duplicate contents were not deduplicated'
        for row in journal['files']:
            assert hashlib.sha256(contents[row['sha256']]).hexdigest() == row['sha256']
            assert contents[row['sha256']] == Path(row['path']).read_bytes()
            assert stat.S_IMODE(Path(row['path']).stat().st_mode) == row['mode']
    count = flags_released(out / 'artifact-preservation.json')
    return dict(regular_files=count, actual_unlink_and_truncate_rejected=True,
                actual_child_cli=True, isolated_home=True, exact_content_archive_verified=True)


def error_unwind(out):
    original, source, root, export = fixture(out)
    rejected = []
    try:
        with owner.ArtifactRun(REPO, out) as guard:
            guard.select_source(original, export)
            owner.capture_product(guard.journal, root / 'runtime-product.json', source)
            sibling = put(out.parent / 'outside-selected-input', b'outside selected input\n')
            try:
                owner.capture_artifacts(guard.journal, {str(sibling): execution.file_identity(sibling)})
            except ValueError:
                rejected.append('outside-selected-input')
            else:
                raise AssertionError('unselected outside path was captured')
            wrong = execution.file_identity(root / 'port/src/emacs')
            wrong['sha256'] = '0' * 64
            try:
                owner.capture_artifacts(guard.journal, {str(root / 'port/src/emacs'): wrong})
            except ValueError:
                rejected.append('changed-hash')
            else:
                raise AssertionError('changed artifact expectation accepted')
            raise RuntimeError('intentional owned scope failure')
    except RuntimeError as error:
        assert str(error) == 'intentional owned scope failure'
    assert len(rejected) == 2
    return dict(rejected=rejected, released=flags_released(out / 'artifact-preservation.json'))


def dead_recovery(out):
    result = subprocess.run([sys.executable, __file__, '--worker', 'dead', '--out', out],
                            stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    assert result.returncode == 19, result.stderr
    journal = out / 'artifact-preservation.json'
    value = execution.bounded_json(journal, 128 * 1024 * 1024)
    assert value['state'] == 'active' and any(Path(r['path']).stat().st_flags & stat.UF_IMMUTABLE for r in value['files'])
    recovered = cli(journal, '--release-journal', str(journal))
    assert recovered.returncode == 0, recovered.stderr
    return dict(uncatchable_worker_exit=19, released=flags_released(journal))


def terminated_producer(out):
    with (out / 'worker.stdout').open('xb') as stdout, (out / 'worker.stderr').open('xb') as stderr:
        process = subprocess.Popen([sys.executable, __file__, '--worker', 'signal', '--out', out],
                                   stdout=stdout, stderr=stderr)
        try:
            wait_for(lambda: (out / 'producer-ready').exists(), process)
            child_pid = int((out / 'producer-ready').read_text())
            child_birth = owner.artifact_process(child_pid)[1]
            assert child_birth is not None
            process.send_signal(signal.SIGTERM)
            assert process.wait(timeout=15) == 143
            assert owner.artifact_process(child_pid)[1] != child_birth, 'owned producer still alive after guard unwind'
        finally:
            if process.poll() is None:
                process.kill()
                process.wait()
    return dict(sigterm_worker_exit=143, actual_producer_reaped=True,
                released=flags_released(out / 'artifact-preservation.json'))


def interrupted_journal(out):
    result = subprocess.run([sys.executable, __file__, '--worker', 'journal-signal', '--out', out],
                            stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    assert result.returncode == 143, result.stderr
    journal = out / 'artifact-preservation.json'
    partial = list(out.glob('artifact-preservation.json.writing.*'))
    assert len(partial) == 1 and partial[0].read_bytes() == b'{"interrupted":'
    return dict(actual_sigterm_during_partial_write=True, sigterm_worker_exit=143,
                interrupted_write_retained=True, last_committed_journal_usable=True,
                released=flags_released(journal))


def source_binding(out):
    original, source, root, export = fixture(out)
    assert stage.read_build(original)[1] == source
    path = source / 'input.rs'
    original_bytes = path.read_bytes()
    path.write_bytes(b'intentional fixture source mismatch\n')
    rejected = []
    for name, method in [('prepare', stage.prepare_matrix), ('components', stage.component_matrix),
                         ('execute', stage.execute_matrix)]:
        action = out / name
        try:
            method(SimpleNamespace(build=original, timeout=1), action)
        except ValueError as error:
            assert 'frozen source input changed' in str(error), str(error)
            rejected.append(name)
        else:
            raise AssertionError('changed snapshot entered action ' + name)
        assert not action.exists(), 'child action ran before frozen-source rejection'
    path.write_bytes(original_bytes)
    report_path = original / 'results.json'
    report = execution.bounded_json(report_path)
    report['source_root'] = str(root)
    execution.write_json(report_path, report)
    try:
        stage.read_build(original)
    except ValueError as error:
        assert 'does not match' in str(error)
    else:
        raise AssertionError('unrelated selected source root was accepted')
    return dict(actual_action_entry_rejections=rejected, wrong_named_source_rejected=True,
                editors_or_compilers_started=False)


def metadata_refusal(out):
    mode_out = out / 'mode'
    mode_out.mkdir()
    original, source, root, export = fixture(mode_out)
    with owner.ArtifactRun(REPO, mode_out) as guard:
        guard.select_source(original, export)
        owner.capture_product(guard.journal, root / 'runtime-product.json', source)
        program = root / 'dist/terminal/src/emacs'
        previous = stat.S_IMODE(program.stat().st_mode)
        # Simulate an external privileged flag/mode change, in this fixture only.
        os.chflags(program, program.stat().st_flags & ~stat.UF_IMMUTABLE)
        program.chmod(0o600 if previous != 0o600 else 0o640)
        try:
            guard.archive()
        except ValueError as error:
            assert 'mode changed' in str(error)
        else:
            raise AssertionError('changed program mode was archived as exact')
        program.chmod(previous)
    flags_released(mode_out / 'artifact-preservation.json')
    link_out = out / 'symlink'
    link_out.mkdir()
    original, source, root, export = fixture(link_out)
    archive_rejected = False
    try:
        with owner.ArtifactRun(REPO, link_out) as guard:
            guard.select_source(original, export)
            owner.capture_product(guard.journal, root / 'runtime-product.json', source)
            link = source / 'input-link'
            link.unlink()
            link.symlink_to('tools/runtime-owner.py')
            try:
                guard.archive()
            except ValueError as error:
                assert 'symlink changed' in str(error)
                archive_rejected = True
            else:
                raise AssertionError('changed symlink was archived as exact')
    except ValueError as error:
        assert archive_rejected and 'release failed' in str(error)
    journal = execution.bounded_json(link_out / 'artifact-preservation.json', 128 * 1024 * 1024)
    assert journal['state'] == 'release-failed' and journal['release_failures']
    assert all(not Path(row['path']).stat().st_flags & stat.UF_IMMUTABLE for row in journal['files'])
    return dict(changed_mode_archive_rejected=True, changed_symlink_archive_and_release_rejected=True,
                regular_owned_flags_restored_on_negative_path=True)


def replaced_inode(out):
    result = subprocess.run([sys.executable, __file__, '--worker', 'dead', '--out', out],
                            stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    assert result.returncode == 19, result.stderr
    journal = out / 'artifact-preservation.json'
    value = execution.bounded_json(journal, 128 * 1024 * 1024)
    path = out / 'selected/source/input.rs'
    original_inode = path.stat().st_ino
    data = path.read_bytes()
    os.chflags(path, path.stat().st_flags & ~stat.UF_IMMUTABLE)
    path.rename(path.with_name('original-inode-retained'))
    path.write_bytes(data)
    assert path.stat().st_ino != original_inode
    os.chflags(path, path.stat().st_flags | stat.UF_IMMUTABLE)
    rejected = cli(journal, '--release-journal', str(journal))
    assert rejected.returncode != 0 and path.stat().st_flags & stat.UF_IMMUTABLE
    assert execution.bounded_json(journal, 128 * 1024 * 1024)['state'] == 'release-failed'
    # Remove the flag introduced by this test on its own replacement; production
    # release correctly refused it, and the original fixture inode is retained.
    os.chflags(path, path.stat().st_flags & ~stat.UF_IMMUTABLE)
    return dict(replaced_inode_release_rejected=True, replacement_flag_not_changed_by_release=True,
                original_fixture_inode_retained=True)


def finished_product_execution(out):
    original, source, root, export = fixture(out)
    # Retire link intermediates (.o and .a) as in post-build intermediate cleanup;
    # strict build-leg capture must reject, while finished-product execution
    # preservation verifies and locks all runtime artifacts and top-level metadata.
    (root / 'objects/fixture.o').unlink()
    (root / 'load/fixture.a').unlink()
    strict_rejected = False
    (out / 'strict-check').mkdir()
    with owner.ArtifactRun(REPO, out / 'strict-check') as strict_guard:
        strict_guard.select_source(original, export)
        try:
            owner.capture_product(strict_guard.journal, root / 'runtime-product.json', source)
        except OSError:
            strict_rejected = True
    assert strict_rejected, 'strict capture_product accepted retired link intermediates'
    flags_released(out / 'strict-check/artifact-preservation.json')
    action_out = out / 'execution-stage'
    action_out.mkdir()
    with owner.ArtifactRun(REPO, action_out) as guard:
        read_original, read_source, products_path, products, build = stage.read_action_build(
            SimpleNamespace(build=original, execution_products=None))
        stage.select_execution_inputs(guard, read_original, read_source, products_path,
                                      products, build, SimpleNamespace(execution_products=None))
        for protected in (root / 'dist/terminal/src/emacs',
                          original / 'results.json',
                          original / 'products.json'):
            assert protected.stat().st_flags & stat.UF_IMMUTABLE
            try:
                protected.write_bytes(b'mutation-blocked')
            except PermissionError:
                pass
            else:
                raise AssertionError('immutable execution input mutation was accepted: ' + str(protected))
        retention = guard.retain_current_files()
        assert retention['all_original_files_verified'] is True
        assert retention['duplicate_content_archive_created'] is False
    released = flags_released(action_out / 'artifact-preservation.json')
    return dict(strict_capture_rejected_missing_intermediates=True,
                finished_product_and_top_metadata_protected=True,
                retained_files_verified=retention['files'],
                released=released)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out', type=Path, required=True)
    parser.add_argument('--worker', choices=('dead', 'signal', 'journal-signal'))
    options = parser.parse_args()
    if sys.platform != 'darwin' or not hasattr(os, 'chflags'):
        parser.error('actual macOS file flag behavior is required')
    out = options.out.resolve()
    if not out.is_relative_to(REPO / 'build') or out == REPO / 'build':
        parser.error('fresh owned build output required')
    if options.worker:
        worker(options.worker, out)
        return 0
    out.mkdir(parents=True, exist_ok=False)
    rows = []
    for name, body in [('completed-leg-child-archive-release', normal), ('error-scope-unwind', error_unwind),
                       ('dead-owner-recovery', dead_recovery), ('sigterm-owned-producer-reaped', terminated_producer),
                       ('sigterm-during-journal-write', interrupted_journal),
                       ('frozen-source-action-boundaries', source_binding),
                       ('mode-and-symlink-archive-invariants', metadata_refusal),
                       ('replaced-inode-release-boundary', replaced_inode),
                       ('finished-product-execution-preservation', finished_product_execution)]:
        case = out / name
        case.mkdir()
        started = time.monotonic()
        try:
            row = dict(id=name, passed=True, details=body(case))
        except BaseException as error:
            row = dict(id=name, passed=False, error=f'{type(error).__name__}: {error}', traceback=traceback.format_exc())
        row['seconds'] = time.monotonic() - started
        rows.append(row)
        execution.write_json(out / 'results.json', dict(passed=all(r['passed'] for r in rows),
                             complete=len(rows) == 9, cases=rows, scope='actual owner/OS integration with nonexecuted format fixtures',
                             whole_editor_native_gc_compatibility_performance_accepted=False))
        print(json.dumps({key: row[key] for key in ('id', 'passed', 'seconds')}), flush=True)
    return 0 if all(row['passed'] for row in rows) else 1


if __name__ == '__main__':
    raise SystemExit(main())
