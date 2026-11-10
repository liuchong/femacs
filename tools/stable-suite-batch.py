"""Original per-file ERT execution with bounded, per-case progress accounting."""
from __future__ import annotations

import hashlib
import errno
import json
import math
import os
from pathlib import Path
import re
import shutil
import signal
import select
import subprocess
import sys
import tempfile
import time
import uuid


# Keep Python out of the forked, potentially large acceptance-driver image.
# These Popen arguments permit the standard library's posix_spawn path.  The
# fresh interpreter becomes the session leader, attaches the slave and execs
# the editor in the same PID; it never starts a second persistent process.
_PTY_EXEC = """import fcntl, os, sys, termios
os.setsid()
fcntl.ioctl(0, termios.TIOCSCTTY, 0)
os.chdir(sys.argv[1])
os.closerange(3, os.sysconf('SC_OPEN_MAX'))
os.execv(sys.argv[2], sys.argv[2:])
"""


def select_entrypoints(manifest):
    """Unconditional ELFILES rule of the pinned original test recipe.

    Module/feature availability is reported by actual execution, never hidden
    by dropping a candidate's missing capability. No test tag is filtered.
    """
    result = []
    for entry in manifest['entries']:
        path = Path(entry['path'])
        if path.suffix != '.el' or path.name.startswith('.'):
            continue
        if any(part in {'manual', 'data', 'infra'} or part.endswith('resources')
               for part in path.parts[:-1]):
            continue
        result.append(entry)
    return sorted(result, key=lambda entry: entry['path'])


def _process_socket_path_bytes(directory):
    # Pinned ert-x.el: prefix, six random bytes, source-derived suffix, socket.
    return len(os.fsencode(Path(directory) / 'emacs-test-XXXXXX-process' / 'socket'))


def _package_socket_path_bytes(directory):
    # Original with-package-test uses ert-x's six-byte temp name and gnupg.
    # Include the longest gpg-agent socket suffix, not only S.gpg-agent.
    return len(os.fsencode(Path(directory) / 'emacs-test-XXXXXX-package'
                           / 'gnupg' / 'S.gpg-agent.browser'))


def _record_retirement(api, final_path, result):
    """Bind the final owner outcome without changing pre-retirement evidence."""
    if api._sha256_path(final_path) != result['file_result']['sha256']:
        raise ValueError('file result changed before owner-retirement receipt')
    receipt = final_path.with_name(final_path.stem + '.owner-retirement.json')
    with receipt.open('x', encoding='utf-8') as stream:
        json.dump(dict(schema='femacs-stable-owner-retirement-v1',
                       file_result=result['file_result'],
                       state_retirement=result['state_retirement']), stream, indent=2)
        stream.write('\n')
        stream.flush()
        os.fsync(stream.fileno())
    result['state_retirement_receipt'] = dict(
        path=result['state_retirement_path'], sha256=api._sha256_path(receipt),
        bytes=receipt.stat().st_size)


PRIVATE_SOURCE_CONTEXT_FILES = ('test/lisp/arc-mode-tests.el',
                                'test/lisp/tar-mode-tests.el')
PRIVATE_SOURCE_CONTEXT_PROFILE = 'owned-original-archive-source-context-v1'
EXTRA_FIXTURE_PREFIXES = {
    'test/lisp/arc-mode-tests.el': ('test/data/decompress/',),
    'test/lisp/tar-mode-tests.el': ('test/data/decompress/',),
    'test/src/decompress-tests.el': ('test/data/decompress/',),
    'test/lisp/image-file-tests.el': ('test/data/image/',),
    'test/lisp/image/exif-tests.el': ('test/data/image/',),
    'test/lisp/image/image-dired-tests.el': ('test/data/image/',),
    'test/lisp/net/shr-tests.el': ('test/data/image/',),
    'test/src/regex-emacs-tests.el': ('test/src/regex-resources/',),
    'test/src/fileio-tests.el': ('test/src/lread-resources/',),
    'test/lisp/international/mule-tests.el': ('test/lisp/international/mule-util-resources/',),
    'test/lisp/progmodes/project-tests.el': (
        'test/lisp/progmodes/autoconf-resources/',
        'test/lisp/progmodes/elisp-mode-resources/',
    ),
    'test/lisp/progmodes/ruby-ts-mode-tests.el': ('test/lisp/progmodes/ruby-mode-resources/',),
    'test/lisp/progmodes/etags-tests.el': ('test/manual/etags/',),
}
MODULE_TEST_FILE = 'test/src/emacs-module-tests.el'
MODULE_TEST_PROFILE = 'owned-original-module-fixture-v1'
CASE_BUDGET_FILE = 'test/lisp/filenotify-tests.el'
CASE_BUDGET_ID = 'file-notify-test07-many-events-remote'
CASE_BUDGET_PROFILE = 'darwin-gio-many-events-case-budget-v1'
CASE_BUDGET_SECONDS = 600.0
COMPILER_OUTPUT_PROFILE = 'owned-original-compiler-output-v1'
COMPILER_OUTPUT_FILE = 'test/src/comp-tests.el'


def _context_node(path):
    node = path.lstat()
    return dict(device=node.st_dev, inode=node.st_ino, mode=node.st_mode,
                bytes=node.st_size, mtime_ns=node.st_mtime_ns,
                link=os.readlink(path) if path.is_symlink() else None)


def _prepare_source_context_fixture(api, out, paths, case_root, logical, fixture):
    if logical not in PRIVATE_SOURCE_CONTEXT_FILES:
        return None
    case_root, out = Path(case_root), Path(out)
    working = case_root / 'out'
    shared = Path(paths['test_context'])
    if (case_root.resolve() != case_root or working.resolve() != working
            or shared.resolve() != shared
            or shared.parent != out / 'source-contexts'
            or shared.name not in {'reference', 'candidate', 'terminal', 'cocoa'}):
        raise ValueError('archive source context roots are not bound and canonical')
    names = {'src', 'admin', 'nextstep', 'README', 'lisp', 'test'}
    if {p.name for p in shared.iterdir()} != names:
        raise ValueError('archive shared source context link set differs')
    selected = {row['path'] for row in fixture['files']}
    decompression = {p for p in selected if p.startswith('test/data/decompress/')}
    if not decompression or logical not in selected:
        raise ValueError('archive source context lacks manifest-bound private data')
    product_lisp = Path(paths['lisp_root']).resolve()
    if (not (working / 'test').is_dir()
            or (working / 'test').resolve() != working / 'test'
            or (shared / 'test').resolve() != out / 'test'
            or (shared / 'lisp').resolve() != product_lisp):
        raise ValueError('archive test or product-library target differs')
    links = []
    for name in sorted(names):
        path = shared / name
        if not path.is_symlink():
            raise ValueError('shared source context member is not a declared link')
        destination = path.resolve(strict=True)
        expected = (out / 'test' if name == 'test' else
                    product_lisp if name == 'lisp' else
                    out / 'source-context-data' / name)
        if destination != expected or destination.resolve() != destination:
            raise ValueError('archive shared source context target differs')
        links.append(dict(name=name, path=str(path), node=_context_node(path),
                          target=str(destination), target_node=_context_node(destination)))
    node = working.lstat()
    return dict(schema='femacs-owned-archive-source-context-v1',
                profile=PRIVATE_SOURCE_CONTEXT_PROFILE, scope=list(PRIVATE_SOURCE_CONTEXT_FILES),
                root=str(working), root_node=dict(device=node.st_dev, inode=node.st_ino, mode=node.st_mode),
                shared_root=str(shared), shared_root_node=_context_node(shared), links=links,
                entrypoint=logical, private_test_root=str(working / 'test'),
                private_decompression_members=sorted(decompression),
                product_lisp=str(product_lisp), c_source_root=str(shared / 'src'),
                original_test_body_modified=False, corpus_inputs_modified=False,
                product_library_replaced=False, sandbox_policy_changed=False,
                private_context_links_created=0, semantic_outcome_overrides=False)


def _finish_source_context_fixture(api, fixture):
    if fixture is None:
        return None
    root, shared = Path(fixture['root']), Path(fixture['shared_root'])
    node = root.lstat()
    if (root.resolve() != root
            or dict(device=node.st_dev, inode=node.st_ino, mode=node.st_mode) != fixture['root_node']
            or _context_node(shared) != fixture['shared_root_node']
            or {p.name for p in shared.iterdir()} != {row['name'] for row in fixture['links']}):
        raise api.SuiteError('archive source context root or link set changed')
    for row in fixture['links']:
        path = Path(row['path'])
        if (not path.is_symlink() or _context_node(path) != row['node']
                or path.resolve(strict=True) != Path(row['target'])
                or _context_node(Path(row['target'])) != row['target_node']):
            raise api.SuiteError('archive shared context link or target changed')
    fixture.update(shared_context_and_targets_unchanged=True,
                   private_context_root_unchanged=True)
    return fixture


def _fixture_entries(manifest, logical):
    path = Path(logical)
    parent = path.parent
    if parent.name.endswith('-tests'):
        base = parent.parent
        stem = parent.name[:-len('-tests')]
        prefixes = (parent, base / (stem + '-resources'), base / 'resources')
    else:
        stem = path.stem
        for suffix in ('-tests', '-test'):
            if stem.endswith(suffix):
                stem = stem[:-len(suffix)]
                break
        prefixes = (parent / (stem + '-resources'), parent / 'resources')
    extra_prefixes = EXTRA_FIXTURE_PREFIXES.get(logical, ())
    entries = [e for e in manifest['entries'] if Path(e['path']).parent == parent
        or any(e['path'].startswith(prefix.as_posix() + '/') for prefix in prefixes)
        or any(e['path'].startswith(extra) for extra in extra_prefixes)]
    if logical not in {e['path'] for e in entries}:
        raise ValueError('unbound original fixture entrypoint')
    return sorted(entries, key=lambda e: e['path'])


def _fixture_node(api, path):
    if not path.exists() and not path.is_symlink():
        return dict(kind='absent')
    node = path.lstat()
    if path.is_symlink():
        return dict(kind='symlink', target=os.readlink(path))
    if not path.is_file():
        return dict(kind='other', mode=node.st_mode)
    return dict(kind='file', bytes=node.st_size, mode=node.st_mode & 0o777,
                sha256=api._sha256_path(path))


def _prepare_working_fixture(api, out, case_root, logical, entries):
    root = case_root / 'out'
    if root.resolve() != root or not root.is_dir() or any(root.iterdir()):
        raise ValueError('original working fixture directory is not empty and owned')
    bindings = []
    for entry in entries:
        relative = entry['path']
        rel = Path(relative)
        if rel.is_absolute() or '..' in rel.parts or rel.parts[0] != 'test':
            raise ValueError('unsafe original working fixture member')
        source, target = out / relative, root / relative
        expected = dict(kind='file', bytes=entry['size'],
                        mode=int(entry['mode'], 8) & 0o777, sha256=entry['sha256'])
        if source.resolve() != source or _fixture_node(api, source) != expected:
            raise ValueError('canonical original fixture changed before copying')
        target.parent.mkdir(parents=True, exist_ok=True)
        with source.open('rb') as incoming, target.open('xb') as outgoing:
            shutil.copyfileobj(incoming, outgoing, 1024 * 1024)
        target.chmod(expected['mode'])
        if _fixture_node(api, target) != expected or target.stat().st_nlink != 1:
            raise ValueError('original working fixture identity differs')
        bindings.append(dict(path=relative, before=expected))
    if logical not in {e['path'] for e in bindings}:
        raise ValueError('original working fixture omitted its entrypoint')
    return root / logical, dict(schema='femacs-original-mutable-fixture-v1',
        root=str(root), entrypoint=logical, files=bindings,
        source='manifest-identical-owned-materialization',
        original_test_body_modified=False, outcome_overrides=False)


def _finish_working_fixture(api, out, fixture):
    root = Path(fixture['root'])
    changes = []
    unchanged = True
    for binding in fixture['files']:
        relative = binding['path']
        binding['after'] = _fixture_node(api, root / relative)
        if binding['after'] != binding['before']:
            changes.append(relative)
        unchanged = unchanged and _fixture_node(api, out / relative) == binding['before']
    selected = {e['path'] for e in fixture['files']}
    generated = []
    for directory, names, files in os.walk(root, followlinks=False):
        for name in [*names, *files]:
            path = Path(directory) / name
            relative = path.relative_to(root).as_posix()
            if relative not in selected and (path.is_symlink() or not path.is_dir()):
                generated.append(dict(path=relative, identity=_fixture_node(api, path)))
    fixture.update(changed_working_members=changes, generated_working_members=generated,
                   canonical_fixture_inputs_unchanged=unchanged)
    return fixture


def _prepare_product_library_fixture(api, paths, case_root, logical):
    """Supply the product's own compiler at the original resource-relative path."""
    if logical != COMPILER_OUTPUT_FILE:
        return None
    case_root = Path(case_root)
    root = case_root / 'out'
    library = Path(paths['lisp_root']).resolve()
    if root.resolve() != root or not root.is_dir() or not library.is_dir():
        raise ValueError('compiler library layout roots are not canonical')
    files = []
    for name in ('comp.el', 'comp.elc'):
        source = library / 'emacs-lisp' / name
        target = root / 'lisp' / 'emacs-lisp' / name
        expected = _fixture_node(api, source)
        if source.resolve() != source or expected['kind'] != 'file':
            raise ValueError('compiler product library must be a regular file')
        target.parent.mkdir(parents=True, exist_ok=True)
        if target.parent.resolve() != target.parent:
            raise ValueError('compiler fixture parent escaped the owned case')
        with source.open('rb') as incoming, target.open('xb') as outgoing:
            shutil.copyfileobj(incoming, outgoing, 1024 * 1024)
            outgoing.flush()
            os.fsync(outgoing.fileno())
        target.chmod(expected['mode'])
        if (_fixture_node(api, source) != expected or _fixture_node(api, target) != expected
                or target.stat().st_nlink != 1):
            raise ValueError('compiler library copy differs from its product')
        files.append(dict(path=str(target.relative_to(root)), source=str(source), before=expected))
    return dict(schema='femacs-owned-product-compiler-library-layout-v1', root=str(root),
        product_lisp=str(library), files=files, scope='test/src/comp-tests.el-only',
        product_version_retained=True, original_test_body_modified=False,
        compiler_frontend_replaced_by_another_product=False)


def _finish_product_library_fixture(api, fixture):
    if fixture is None:
        return None
    root = Path(fixture['root'])
    for row in fixture['files']:
        row['after'] = _fixture_node(api, root / row['path'])
        if _fixture_node(api, Path(row['source'])) != row['before']:
            raise api.SuiteError('compiler product source changed during original-file execution')
    fixture['product_sources_unchanged'] = True
    return fixture


def _render_stable_module_header(context_src, destination):
    template_path = context_src / 'emacs-module.in.h'
    if not template_path.is_file() or template_path.is_symlink():
        raise ValueError('missing stable emacs-module.in.h in source context')
    content = template_path.read_text(encoding='utf-8').replace('@emacs_major_version@', '31')
    for major in range(25, 32):
        snippet = context_src / f'module-env-{major}.h'
        if not snippet.is_file() or snippet.is_symlink():
            raise ValueError(f'missing stable module-env-{major}.h in source context')
        content = content.replace(f'@module_env_snippet_{major}@', snippet.read_text(encoding='utf-8'))
    if re.search(r'@[A-Za-z_][A-Za-z_0-9]*@', content):
        raise ValueError('unresolved placeholder in stable emacs-module.h')
    destination.write_text(content, encoding='utf-8')
    destination.chmod(0o644)


def _module_wrapper_script(program, dump_args, invocation_directory):
    eval_value = json.dumps(str(invocation_directory) + os.sep)
    argv = [str(program), *map(str, dump_args), '--eval',
            f'(setq invocation-directory {eval_value})']
    return ('#!' + sys.executable + '\nimport os, sys\n'
            + 'os.execv(' + repr(str(program)) + ', ' + repr(argv) + ' + sys.argv[1:])\n')


def _prepare_module_fixture(api, out, paths, case_root, logical, timeout):
    if logical != MODULE_TEST_FILE:
        return None
    case_root, out = Path(case_root), Path(out)
    working = case_root / 'out'
    build_dir = case_root / 'cache' / 'module-build'
    build_dir.mkdir(parents=True, exist_ok=True)
    context_src = out / 'source-context-data' / 'src'
    header_path = build_dir / 'emacs-module.h'
    _render_stable_module_header(context_src, header_path)
    repo = Path(__file__).resolve().parent.parent
    config_source = repo / 'tools' / 'fixtures' / 'module-thread-config.h'
    if not config_source.is_file() or config_source.is_symlink():
        raise ValueError('missing tools/fixtures/module-thread-config.h')
    config_path = build_dir / 'config.h'
    config_path.write_bytes(config_source.read_bytes())
    config_path.chmod(0o644)
    mod_source = working / 'test/src/emacs-module-resources/mod-test.c'
    if not mod_source.is_file() or mod_source.is_symlink():
        raise ValueError('missing working mod-test.c fixture')
    mod_output = working / 'test/src/emacs-module-resources/mod-test.dylib'
    cc_cmd = [os.environ.get('CC', 'cc'), '-std=gnu2x', '-shared', '-fPIC',
              '-I', str(build_dir)]
    gmp_prefix = os.environ.get('FEMACS_GMP_PREFIX', '').strip()
    if gmp_prefix:
        prefix = Path(gmp_prefix)
        cc_cmd.extend(['-I', str(prefix / 'include'), '-L', str(prefix / 'lib'), '-lgmp'])
    else:
        pkg_cfg = subprocess.run(
            ['pkg-config', '--cflags', '--libs', 'gmp'],
            stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=15.0, check=False,
        )
        if pkg_cfg.returncode == 0 and pkg_cfg.stdout.strip():
            cc_cmd.extend(pkg_cfg.stdout.decode('utf-8', errors='replace').split())
        else:
            cc_cmd.append('-lgmp')
    cc_cmd.extend([str(mod_source), '-o', str(mod_output)])
    if sys.platform == 'darwin':
        cc_cmd.append('-Wl,-undefined,dynamic_lookup')
    completed = subprocess.run(
        cc_cmd, cwd=str(build_dir), stdout=subprocess.PIPE, stderr=subprocess.PIPE,
        timeout=min(float(timeout), 120.0), check=False,
    )
    if completed.returncode != 0 or not mod_output.is_file():
        raise ValueError('stable module fixture compilation failed: '
                         + completed.stderr.decode('utf-8', errors='replace')[:512])
    bin_dir = working / 'bin'
    bin_dir.mkdir(parents=True, exist_ok=True)
    invocation_name = Path(paths['program']).name
    wrapper = bin_dir / invocation_name
    wrapper.write_text(
        _module_wrapper_script(paths['program'], paths['dump_args'], bin_dir),
        encoding='utf-8',
    )
    wrapper.chmod(0o755)
    return dict(
        schema='femacs-owned-module-fixture-v1',
        profile=MODULE_TEST_PROFILE,
        root=str(working),
        invocation_directory=str(bin_dir) + os.sep,
        invocation_name=invocation_name,
        wrapper=str(wrapper),
        wrapper_before=_fixture_node(api, wrapper),
        module_path=str(mod_output),
        module_before=_fixture_node(api, mod_output),
        source_path=str(mod_source),
        source_before=_fixture_node(api, mod_source),
        header_sha256=api._sha256_path(header_path),
        config_sha256=api._sha256_path(config_path),
        compile_argv=cc_cmd,
    )


def _finish_module_fixture(api, fixture):
    if fixture is None:
        return None
    wrapper = Path(fixture['wrapper'])
    module_path = Path(fixture['module_path'])
    source_path = Path(fixture['source_path'])
    if (_fixture_node(api, wrapper) != fixture['wrapper_before']
            or _fixture_node(api, module_path) != fixture['module_before']
            or _fixture_node(api, source_path) != fixture['source_before']):
        raise api.SuiteError('module test fixture changed during execution')
    fixture['module_fixture_unchanged'] = True
    return fixture


def _progress_timeout(logical, active, default):
    if sys.platform == 'darwin' and logical == CASE_BUDGET_FILE and active == CASE_BUDGET_ID:
        return max(float(default), CASE_BUDGET_SECONDS)
    return default


def _case_deadline_context(logical, default):
    if sys.platform != 'darwin' or logical != CASE_BUDGET_FILE:
        return None
    return dict(profile=CASE_BUDGET_PROFILE, test_id=CASE_BUDGET_ID,
                original_default_seconds=default,
                selected_case_seconds=max(float(default), CASE_BUDGET_SECONDS),
                explicit_monitor_wait_calls=43, monitor_wait_seconds=7,
                explicit_wait_seconds=301, original_test_and_assertions_unchanged=True)


def _verify_compiler_output_context(value, case_root, require_current=True):
    assert isinstance(value, dict) and value['profile'] == COMPILER_OUTPUT_PROFILE
    assert value['scope'] == 'test/src/comp-tests.el-only' and type(value['enabled']) is bool
    before, after = value['before'], value['after']
    assert isinstance(before, list) and all(isinstance(p, str) for p in before)
    if value['enabled']:
        target = str(Path(case_root) / 'native-cache/compiler-output') + '/'
        assert value['target'] == target and target not in before
        assert after == before + [target]
    else:
        assert value['target'] is None and after == before
    if require_current:
        assert value['actual_paths'] == after
    return value


def _run_file(api, out, paths, runner, test_root, logical, timeout, label,
              *, display='batch', fixture_entries=None, resource_admission=None):
    if not isinstance(display, str) or display not in {'batch', 'terminal'}:
        raise api.SuiteError('unsupported original-file execution display')
    if display == 'terminal' and os.name != 'posix':
        raise api.SuiteError('controlling terminal execution requires POSIX')
    safe = api._safe_component(logical)
    logs = out / 'logs' / label
    results = out / 'results' / label
    logs.mkdir(parents=True, exist_ok=True)
    results.mkdir(parents=True, exist_ok=True)
    stdout_path = logs / (safe + '.stdout.log')
    stderr_path = logs / (safe + '.stderr.log')
    progress_path = results / (safe + '.events.jsonl')
    final_path = results / (safe + '.json')
    preparation_path = results / (safe + '.environment.json')
    container = out / 'environments' / label / (safe + '-' + uuid.uuid4().hex)
    planned_tmp = container / api._safe_case_component(logical) / 'tmp'
    socket_case = os.name == 'posix' and logical == 'test/src/process-tests.el'
    package_case = os.name == 'posix' and logical == 'test/lisp/emacs-lisp/package-tests.el'
    use_compact = fixture_entries is not None
    short_container = (use_compact
                       or (socket_case and _process_socket_path_bytes(planned_tmp) + 8 >= 104)
                       or (package_case and _package_socket_path_bytes(planned_tmp) >= 104))
    if short_container:
        cache = Path.home() / 'AI' / 'caches'
        cache.mkdir(parents=True, exist_ok=True)
        container = Path(tempfile.mkdtemp(prefix='fe-', dir=cache))
    else:
        container.mkdir(parents=True, exist_ok=False)
    container_node = container.lstat()
    preparation = dict(state='created', file=logical, product=label,
        container=str(container), device=container_node.st_dev,
        inode=container_node.st_ino, short_container=short_container,
        planned_socket_bytes=(_process_socket_path_bytes(planned_tmp) if socket_case
                              else _package_socket_path_bytes(planned_tmp) if package_case else None))
    preparation_path.write_text(json.dumps(preparation, indent=2)+'\n', encoding='utf-8')
    try:
        if use_compact or (package_case and short_container):
            # This container belongs to exactly one file execution.  Keeping
            # another 21-byte case hash would still overflow auxiliary sockets.
            environment = api.case_environment(logical, container, compact=True)
        else:
            environment = api.case_environment(logical, container)
        case_root = Path(environment['FEMACS_RUNTIME_CASE_ROOT'])
        case_node = case_root.lstat()
        if (use_compact or socket_case) and _process_socket_path_bytes(environment['TMPDIR']) + 8 >= 104:
            raise api.SuiteError('owned temporary path cannot fit the original local socket')
        if (use_compact or package_case) and _package_socket_path_bytes(environment['TMPDIR']) >= 104:
            raise api.SuiteError('owned temporary path cannot fit the original GPG agent sockets')
        cwd = case_root / 'cwd'
        cwd.mkdir()
        preparation.update(state='ready', case_root=str(case_root),
            temporary_directory=environment['TMPDIR'],
            socket_bytes=(_process_socket_path_bytes(environment['TMPDIR']) if socket_case
                          else _package_socket_path_bytes(environment['TMPDIR']) if package_case else None))
    except BaseException as error:
        # Keep partial state for diagnosis and record its actual owner/path.
        preparation.update(state='failed', retained=True,
                           error_type=type(error).__name__, error=str(error)[:1024])
        preparation_path.write_text(json.dumps(preparation, indent=2)+'\n', encoding='utf-8')
        raise
    preparation_path.write_text(json.dumps(preparation, indent=2)+'\n', encoding='utf-8')
    physical_file, fixture = out / logical, None
    if fixture_entries is not None:
        try:
            if not fixture_entries:
                raise ValueError('every original file requires a private fixture')
            physical_file, fixture = _prepare_working_fixture(
                api, out, case_root, logical, fixture_entries)
            fixture['product_library_layout'] = _prepare_product_library_fixture(
                api, paths, case_root, logical)
            fixture['source_context_layout'] = _prepare_source_context_fixture(
                api, out, paths, case_root, logical, fixture)
            fixture['module_layout'] = _prepare_module_fixture(
                api, out, paths, case_root, logical, timeout)
            preparation['working_fixture'] = fixture
            preparation_path.write_text(json.dumps(preparation, indent=2)+'\n', encoding='utf-8')
        except BaseException as error:
            preparation.update(state='failed', retained=True, stage='working-fixture',
                error_type=type(error).__name__, error=str(error)[:1024])
            preparation_path.write_text(json.dumps(preparation, indent=2)+'\n', encoding='utf-8')
            raise
    effective_test_root = (case_root / 'out' / 'test') if fixture is not None else test_root
    environment.update(PATH=os.environ.get('PATH', os.defpath), LC_ALL='C', LANG='C',
        TERM='xterm-256color' if display == 'terminal' else 'dumb',
        PWD=str(cwd), EMACS_TEST_DIRECTORY=str(effective_test_root),
        FEMACS_RUNTIME_TEST_ROOT=str(case_root), STABLE_SUITE_MODE='batch',
        STABLE_SUITE_FILE=str(physical_file), STABLE_SUITE_FILE_ID=logical,
        STABLE_SUITE_PROGRESS=str(progress_path), STABLE_SUITE_RESULT=str(final_path))
    environment.update(paths.get('runtime_env', {}))
    if display == 'terminal':
        environment['STABLE_SUITE_DISPLAY'] = 'terminal'
        # num-processors honors this limit before command-line Lisp runs.
        # Its default native-worker policy uses half that available count.
        environment['OMP_THREAD_LIMIT'] = '1'
    else:
        environment.pop('STABLE_SUITE_DISPLAY', None)
    for key in ('STABLE_SUITE_ARCHIVE_SOURCE_PROFILE', 'STABLE_SUITE_ARCHIVE_SHARED_CONTEXT',
                'STABLE_SUITE_INVOCATION_DIRECTORY', 'STABLE_SUITE_INVOCATION_NAME'):
        environment.pop(key, None)
    source_context = paths['test_context']
    if fixture and fixture.get('source_context_layout'):
        source_context = fixture['source_context_layout']['root']
        environment.update(STABLE_SUITE_ARCHIVE_SOURCE_PROFILE=PRIVATE_SOURCE_CONTEXT_PROFILE,
                           STABLE_SUITE_ARCHIVE_SHARED_CONTEXT=str(paths['test_context']))
    if fixture and fixture.get('module_layout'):
        environment.update(
            STABLE_SUITE_INVOCATION_DIRECTORY=fixture['module_layout']['invocation_directory'],
            STABLE_SUITE_INVOCATION_NAME=fixture['module_layout']['invocation_name'],
        )
    environment.update(STABLE_SUITE_SOURCE_CONTEXT=str(source_context),
                       STABLE_SUITE_PRODUCT_LISP=str(paths['lisp_root']))
    display_args = ['--batch', '-Q'] if display == 'batch' else ['-nw', '-Q', '--no-splash']
    if display == 'terminal':
        # Interactive startup enables deferred native compilation. Keep it
        # real but serial within this runner's bounded process tree; zero
        # otherwise selects half the host's CPUs before loading the fixture.
        display_args.extend(['--eval', '(setq native-comp-async-jobs-number 1)'])
    load_dirs = ['-L', str(case_root / 'out' / 'test'), '-L', str(test_root)] if fixture is not None else ['-L', str(test_root)]
    argv = [str(paths['program']), *paths['dump_args'], *display_args,
            *load_dirs, '-l', str(runner), '-f', 'stable-suite-main']
    if display == 'terminal':
        # Unlike batch startup, interactive startup does not exit after -f.
        argv.extend(['--eval', '(kill-emacs 0)'])
    loaded = None
    finished = None
    active = None
    records = {}
    started_ids = set()
    protocol_error = None
    timed_out_id = None
    timed_out = False
    remaining_group_terminated = False
    process = None
    terminal_master = None
    terminal_slave = None
    terminal_name = None
    terminal_output_bytes = 0
    started = time.monotonic()
    deadline = started + timeout
    pending = b''

    def consume(raw):
        nonlocal loaded, finished, active, deadline
        value = json.loads(raw.decode('utf-8'))
        if not isinstance(value, dict) or value.get('file') != logical or finished is not None:
            raise api.SuiteError('invalid per-file progress identity/order')
        event = value.get('event')
        if event == 'loaded':
            ids = value.get('ids')
            tests = value.get('tests')
            if (loaded is not None or not isinstance(ids, list)
                    or any(not isinstance(identity, str) or not identity for identity in ids)
                    or len(set(ids)) != len(ids) or value.get('selector') != 'all-discovered-ids'
                    or not isinstance(tests, list)
                    or any(not isinstance(test, dict) or not isinstance(test.get('tags'), list)
                           or any(not isinstance(tag, str) for tag in test['tags'])
                           for test in tests)
                    or [test.get('id') for test in tests] != ids):
                raise api.SuiteError('invalid/duplicate per-file registration')
            loaded = value
            if logical == COMPILER_OUTPUT_FILE and 'compiler_output_context' in value:
                try:
                    _verify_compiler_output_context(value.get('compiler_output_context'), case_root)
                except (AssertionError, KeyError, TypeError) as error:
                    raise api.SuiteError('original compiler output context is invalid') from error
            if display == 'terminal' and (
                    value.get('noninteractive') is not False
                    or value.get('window_system') is not None
                    or value.get('terminal_name') != terminal_name
                    or value.get('native_comp_async_jobs') != 1
                    or value.get('native_comp_thread_limit') != '1'
                    or value.get('available_processors') != 1):
                raise api.SuiteError('original-file context is not the owned interactive terminal')
        elif event == 'test-started':
            identity = value.get('test_id')
            if (loaded is None or active is not None or identity not in loaded['ids']
                    or identity in started_ids):
                raise api.SuiteError('unexpected/duplicate test-started event')
            active = identity
            started_ids.add(identity)
        elif event == 'test-ended':
            identity = value.get('test_id')
            duration = value.get('duration')
            if (loaded is None or active != identity or identity in records
                    or type(duration) not in (int, float) or not math.isfinite(duration)
                    or duration < 0 or value.get('status') not in {'passed', 'failed', 'skipped'}):
                raise api.SuiteError('invalid/missing/duplicate per-case completion')
            if value['status'] == 'passed' and (value.get('expected') is not True or value.get('skipped') is not False):
                raise api.SuiteError('passed event contradicts actual ERT outcome')
            records[identity] = dict(value, event_sha256=hashlib.sha256(raw).hexdigest(),
                                     event_ordinal=len(records))
            active = None
        elif event == 'finished':
            if active is not None:
                raise api.SuiteError('file ended while a case was still active')
            if loaded is not None and (value.get('requested_count') != len(loaded['ids'])
                                      or value.get('ended_count') != len(records)):
                raise api.SuiteError('file completion counters do not match actual events')
            finished = value
        else:
            raise api.SuiteError('unknown progress event')
        # Loading, each active case, and completion all have finite deadlines.
        # A new case gets its own original timeout, not a whole-file extension.
        deadline = time.monotonic() + _progress_timeout(logical, active, timeout)

    def drain_terminal(output):
        nonlocal terminal_output_bytes
        # Bound each drain so continuous human output cannot starve the
        # machine progress/deadline checks.  No terminal input is injected.
        for _ in range(4):
            if not select.select([terminal_master], [], [], 0)[0]:
                break
            try:
                block = os.read(terminal_master, 65536)
            except BlockingIOError:
                break
            except OSError as error:
                if error.errno == errno.EIO:  # The last slave has closed.
                    break
                raise
            if not block:
                break
            output.write(block)
            terminal_output_bytes += len(block)

    try:
        with progress_path.open('xb'):
            pass
        with progress_path.open('rb') as progress, stdout_path.open('xb') as stdout, stderr_path.open('xb') as stderr:
            if display == 'terminal':
                import fcntl
                import pty
                import struct
                import termios
                terminal_master, terminal_slave = pty.openpty()
                terminal_name = os.ttyname(terminal_slave)
                # With no explicit device the editor names its terminal by
                # the controlling-device alias, not by this slave's name.
                argv[1:1] = ['--terminal', terminal_name]
                fcntl.ioctl(terminal_slave, termios.TIOCSWINSZ, struct.pack('HHHH', 24, 80, 0, 0))
                os.set_blocking(terminal_master, False)

                process = subprocess.Popen(
                    [sys.executable, '-c', _PTY_EXEC, str(cwd), *argv],
                    env=environment, close_fds=False,
                    stdin=terminal_slave, stdout=terminal_slave, stderr=terminal_slave,
                )
                os.close(terminal_slave)
                terminal_slave = None
            else:
                process = subprocess.Popen(argv, cwd=cwd, env=environment,
                    stdin=subprocess.DEVNULL, stdout=stdout, stderr=stderr, start_new_session=True)
            while True:
                if terminal_master is not None:
                    drain_terminal(stdout)
                block = progress.read(262144)
                if block:
                    pending += block
                    if len(pending) > api.MAX_RESULT_BYTES and b'\n' not in pending:
                        raise api.SuiteError('oversized progress record retained, not accepted')
                    while b'\n' in pending:
                        raw, pending = pending.split(b'\n', 1)
                        if len(raw) > api.MAX_RESULT_BYTES:
                            raise api.SuiteError('oversized progress record retained, not accepted')
                        consume(raw)
                    continue
                if process.poll() is not None:
                    if progress.tell() < os.fstat(progress.fileno()).st_size:
                        continue
                    if pending:
                        raise api.SuiteError('unterminated progress record at process exit')
                    break
                if time.monotonic() > deadline:
                    timed_out = True
                    timed_out_id = active
                    break
                if terminal_master is not None:
                    # Wake on output instead of blocking its reader for the
                    # whole process-wait interval during interactive redisplay.
                    select.select([terminal_master], [], [], .05)
                else:
                    try:
                        process.wait(timeout=.05)
                    except subprocess.TimeoutExpired:
                        pass
            if terminal_master is not None:
                drain_terminal(stdout)
    except (OSError, ValueError, subprocess.SubprocessError, api.SuiteError) as error:
        protocol_error = str(error)
    finally:
        try:
            if process is not None:
                try:
                    os.killpg(process.pid, signal.SIGKILL)
                    # A deliberately timed-out/protocol-error process is not a
                    # successful parent with unexplained surviving descendants.
                    remaining_group_terminated = not timed_out and protocol_error is None
                except ProcessLookupError:
                    # Cancellation can precede the fresh launcher's setsid.
                    # It has no children before exec, so also reap that PID.
                    if process.poll() is None:
                        process.kill()
                except PermissionError:
                    # A terminating group leader may not yet have been reaped.
                    # Retry only after waitpid confirms that our child exited;
                    # a live-child or persistent permission error stays fatal.
                    if process.poll() is None:
                        raise
                    try:
                        os.killpg(process.pid, signal.SIGKILL)
                        remaining_group_terminated = not timed_out and protocol_error is None
                    except ProcessLookupError:
                        pass
                process.wait()
        finally:
            for descriptor in (terminal_slave, terminal_master):
                if descriptor is not None:
                    os.close(descriptor)
    complete = (process is not None and process.returncode == 0 and not timed_out
        and protocol_error is None and not remaining_group_terminated
        and loaded is not None and finished is not None
        and finished.get('status') == 'complete' and set(records) == set(loaded['ids']))
    observed = {}
    for identity in loaded['ids'] if loaded else []:
        record = records.get(identity)
        status = record['status'] if record else ('timeout' if identity == timed_out_id else 'not-run')
        case = dict(test_id=identity, status=status, complete=record is not None,
            exit_code=process.returncode if process else 1,
            seconds=record.get('duration') if record else None,
            diagnostic=record.get('diagnostic', '') if record else (protocol_error or 'case did not complete'))
        case['assessment'] = api.assess_case(identity, case)
        case['evidence'] = dict(path=str(progress_path.relative_to(out)),
            event_sha256=record.get('event_sha256') if record else None,
            event_ordinal=record.get('event_ordinal') if record else None)
        observed[identity] = case
    result = dict(schema='femacs-stable-file-batch-v1', file=logical, product=label,
        selector='all-discovered-ids', ids=loaded['ids'] if loaded else [],
        tags={row['id']: row['tags'] for row in loaded['tests']} if loaded else {},
        metadata=api._metadata(loaded), cases=observed, complete=complete,
        passed=complete and all(row['assessment']['passed'] for row in observed.values()),
        timed_out=timed_out, timed_out_id=timed_out_id, protocol_error=protocol_error,
        runner_error=finished.get('error') if finished else None,
        argv=argv, pid=process.pid if process else None, cwd=str(cwd),
        runtime_environment=dict(paths.get('runtime_env', {})),
        mock_transport_context=(dict(profile=environment['FEMACS_STABLE_MOCK_TRANSPORT_PROFILE'],
            content_sha256=environment['FEMACS_STABLE_MOCK_TRANSPORT_SHA256'],
            script=environment['FEMACS_STABLE_MOCK_SHELL_ENV'])
            if 'FEMACS_STABLE_MOCK_TRANSPORT_PROFILE' in environment else None),
        seconds=time.monotonic()-started, exit_code=process.returncode if process else None,
        remaining_group_terminated=remaining_group_terminated,
        stdout_log=str(stdout_path.relative_to(out)), stderr_log=str(stderr_path.relative_to(out)),
        progress=(dict(path=str(progress_path.relative_to(out)), bytes=progress_path.stat().st_size,
                       sha256=api._sha256_path(progress_path)) if progress_path.is_file() else None),
        owned_state_root=str(case_root), owned_container=str(container),
        environment_preparation=str(preparation_path.relative_to(out)),
        resource_admission=resource_admission,
        case_deadline_context=_case_deadline_context(logical, timeout),
        compiler_output_policy=COMPILER_OUTPUT_PROFILE if logical == COMPILER_OUTPUT_FILE else None,
        compiler_output_context=((loaded or finished or {}).get('compiler_output_context')
            if logical == COMPILER_OUTPUT_FILE else None),
        state_retirement_path=str(final_path.with_name(
            final_path.stem + '.owner-retirement.json').relative_to(out)))
    if fixture is not None:
        result['working_fixture'] = _finish_working_fixture(api, out, fixture)
        _finish_product_library_fixture(api, result['working_fixture']['product_library_layout'])
        _finish_source_context_fixture(api, result['working_fixture']['source_context_layout'])
        _finish_module_fixture(api, result['working_fixture'].get('module_layout'))
    if display == 'terminal':
        result['execution_context'] = dict(display=display,
            noninteractive=loaded.get('noninteractive') if loaded else None,
            window_system=loaded.get('window_system') if loaded else None,
            terminal_name=loaded.get('terminal_name') if loaded else None,
            owned_terminal_name=terminal_name, io_layout='combined-pty',
            terminal_output_bytes=terminal_output_bytes,
            native_comp_async_jobs=loaded.get('native_comp_async_jobs') if loaded else None,
            native_comp_thread_limit=loaded.get('native_comp_thread_limit') if loaded else None,
            available_processors=loaded.get('available_processors') if loaded else None)
    final_path.write_text(json.dumps(result, indent=2)+'\n', encoding='utf-8')
    result['file_result'] = dict(path=str(final_path.relative_to(out)),
                               sha256=api._sha256_path(final_path), bytes=final_path.stat().st_size)
    if result['passed']:
        try:
            result['state_retirement'] = api._retire_state(case_root,
                ('home','cache','config','state','tmp','native-cache','out','cwd'), final_path, result,
                empty_parents=((case_root, case_node), (container, container_node)))
        except (OSError, ValueError) as error:
            result['state_retirement'] = dict(status='retained',
                member_retirement=result.get('state_retirement'),
                completion_unverified=True,
                error=f'{type(error).__name__}: {error}')
    else:
        result['state_retirement'] = dict(status='retained', reason='file failed/incomplete; preserve original state')
    _record_retirement(api, final_path, result)
    return result


def _source_inputs_unchanged(api, manifest, out):
    for entry in manifest['entries']:
        path = out / entry['path']
        if (path.is_symlink() or not path.is_file() or path.stat().st_size != entry['size']
                or path.resolve() != path
                or (path.stat().st_mode & 0o777) != (int(entry['mode'], 8) & 0o777)
                or api._sha256_path(path) != entry['sha256']):
            return False
    return True


def execute(api, manifest, repo, out, reference, candidate, reference_dump_args,
            candidate_dump_args, timeout, reference_env=None, candidate_env=None,
            *, corpus_root=None, runner_path=None):
    repo = Path(repo).resolve()
    out = Path(os.path.abspath(out))
    materialized = api.materialize(manifest, repo, out, corpus_root=corpus_root)
    api._write_receipt(out, manifest, materialized)
    runner = Path(runner_path) if runner_path else repo/'tools/fixtures/stable-suite-runner.el'
    if not runner.is_file() or runner.is_symlink():
        raise api.SuiteError('bound stable runner must be a regular file')
    runner_sha = api._sha256_path(runner)
    harness_sources = {str(path): api._sha256_path(path) for path in
                      (runner, Path(__file__).resolve(), Path(api.__file__).resolve(),
                       Path(api.__file__).with_name('runtime-retire.py').resolve())}
    paths = {'reference': api._product_paths(reference, reference_dump_args, reference_env),
             'candidate': api._product_paths(candidate, candidate_dump_args, candidate_env)}
    import importlib.util
    context_path = Path(api.__file__).with_name('stable-context.py')
    context_spec = importlib.util.spec_from_file_location('femacs_stable_context', context_path)
    context_provider = importlib.util.module_from_spec(context_spec)
    context_spec.loader.exec_module(context_provider)
    harness_sources[str(context_path)] = api._sha256_path(context_path)
    provider_path = Path(api.__file__).with_name('reference-corpus.py')
    harness_sources[str(provider_path)] = api._sha256_path(provider_path)
    context, context_record = context_provider.prepare(api, repo, out, manifest, paths)
    before = {label: api._public_product_identity(value) for label, value in paths.items()}
    entries = select_entrypoints(manifest)
    files, cases, failures = [], [], []
    corpus_unchanged = True
    reference_keys, candidate_keys = set(), set()
    try:
        for entry in entries:
            logical = entry['path']
            pair = {}
            for label, product in paths.items():
                if corpus_unchanged:
                    pair[label] = _run_file(
                        api, out, product, runner, out/'test', logical, timeout, label,
                        fixture_entries=_fixture_entries(manifest, logical),
                    )
                    corpus_unchanged = _source_inputs_unchanged(api, manifest, out)
                else:
                    pair[label] = dict(ids=[], cases={}, tags={}, complete=False, passed=False,
                                       file_result=None, protocol_error='prior file changed corpus inputs')
                if not pair[label]['complete']:
                    failures.append(label+' file incomplete: '+logical)
                if pair[label].get('passed') and pair[label].get('state_retirement', {}).get('status') != 'complete':
                    failures.append(label+' successful file state retirement incomplete: '+logical)
            registration = api.compare_registrations(pair['reference']['ids'], pair['candidate']['ids'])
            if not registration['passed']:
                failures.append('registration differs: '+logical)
            reference_keys.update((logical, name) for name in pair['reference']['ids'])
            candidate_keys.update((logical, name) for name in pair['candidate']['ids'])
            for identity in sorted(set(pair['reference']['ids']) | set(pair['candidate']['ids'])):
                observed = {}
                for label in paths:
                    value = pair[label]['cases'].get(identity)
                    observed[label] = value if value is not None else dict(test_id=identity,
                        status='not-registered', complete=False, assessment={'passed': False})
                passed = all(value['assessment']['passed'] for value in observed.values())
                if not passed:
                    failures.append('case failed/unverified: '+logical+' :: '+identity)
                cases.append(dict(file=logical, id=identity, key=[logical, identity],
                    reference=observed['reference'], candidate=observed['candidate'], passed=passed,
                    applicability=dict(classification='requested-original-entrypoint', excluded=False,
                        tags=pair['reference']['tags'].get(identity, []))))
            files.append(dict(file=logical, registration=registration,
                reference={key: value for key, value in pair['reference'].items() if key not in {'cases','ids','tags'}},
                candidate={key: value for key, value in pair['candidate'].items() if key not in {'cases','ids','tags'}},
                reference_id_count=len(pair['reference']['ids']), candidate_id_count=len(pair['candidate']['ids'])))
            (out/'partial.json').write_text(json.dumps(dict(schema='femacs-stable-batch-progress-v1',
                completed_files=len(files), requested_files=len(entries), cases=len(cases),
                failure_count=len(failures), latest_file=logical, corpus_inputs_unchanged=corpus_unchanged))+'\n')
    finally:
        after = {label: api._public_product_identity(value) for label, value in paths.items()}
        context_provider.verify(api, repo, out, context, context_record)
    if before != after:
        failures.append('actual product/library inputs changed')
    if not corpus_unchanged:
        failures.append('materialized original corpus input changed; following files not executed')
    if any(api._sha256_path(Path(path)) != expected for path, expected in harness_sources.items()):
        failures.append('bound runner/driver changed during execution')
    if not reference_keys:
        failures.append('original reference produced no ERT identities')
    if reference_keys != candidate_keys:
        failures.append('file-scoped registered identity sets differ')
    return dict(schema=api.RESULT_SCHEMA, mode='execute',
        scope='stable-31.1-original-ELFILES-all-discovered-ids', selector='all-discovered-ids',
        source=dict(release=manifest['source']['release'], commit=manifest['source']['commit']),
        materialized=materialized, timeout_seconds=timeout, timeout_scope='per-load-and-per-test',
        source_context=context_record, source_context_unchanged=True,
        entrypoint_count=len(entries), resource_only_lisp_inputs=manifest['lisp_file_count']-len(entries),
        products_before=before, products_after=after, identities_unchanged=before==after,
        corpus_inputs_unchanged=corpus_unchanged, runner_identity=dict(path=str(runner),sha256=runner_sha),
        harness_sources=harness_sources,
        batch_driver_sha256=api._sha256_path(Path(__file__)), files=files, cases=cases,
        registration=dict(passed=reference_keys==candidate_keys,
            reference_count=len(reference_keys), candidate_count=len(candidate_keys), key_scope='file-and-id'),
        failures=failures, corpus_complete=bool(reference_keys) and not failures,
        full_compatibility_claim=False, logs_root='logs', results_root='results', elapsed_seconds=None)
