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
import stat
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


def select_entrypoints(manifest, subset=None):
    """Unconditional ELFILES rule of the pinned original test recipe.

    Module/feature availability is reported by actual execution, never hidden
    by dropping a candidate's missing capability. No test tag is filtered.

    `subset` narrows the run to an explicitly named, previously declared set of
    original entrypoints. It never invents files, never drops an unnamed one
    silently (an unknown or duplicated name is an error), and the narrowed
    scope is recorded in the report so a subset result cannot be read as full
    corpus coverage.
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
    result = sorted(result, key=lambda entry: entry['path'])
    if subset is None:
        return result
    names = list(subset)
    if (not names or any(not isinstance(name, str) or not name for name in names)
            or len(set(names)) != len(names)):
        raise ValueError('subset must be unique non-empty original entrypoint names')
    available = {entry['path'] for entry in result}
    unknown = sorted(set(names) - available)
    if unknown:
        raise ValueError('subset names are not original entrypoints: ' + ', '.join(unknown))
    wanted = set(names)
    return [entry for entry in result if entry['path'] in wanted]


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
        'test/lisp/files-x-tests.el',
    ),
    'test/lisp/progmodes/ruby-ts-mode-tests.el': ('test/lisp/progmodes/ruby-mode-resources/',),
    'test/lisp/progmodes/perl-mode-tests.el': ('test/lisp/progmodes/cperl-mode-resources/',),
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
BOOTSTRAP_CASE = 'comp-tests-bootstrap'
STATE_NAMES = ('home', 'cache', 'config', 'state', 'tmp', 'native-cache', 'out', 'cwd')
NESTED_TERMINAL_SCENARIO_FILES = (
    'test/lisp/erc/erc-scenarios-status-sidebar.el',
    'test/lisp/erc/erc-scenarios-keep-place-indicator.el',
    'test/lisp/erc/erc-scenarios-keep-place-indicator-trunc.el',
    'test/lisp/erc/erc-scenarios-scrolltobottom.el',
    'test/lisp/erc/erc-scenarios-scrolltobottom-relaxed.el',
)
ALWAYS_LIVE_FILES = (MODULE_TEST_FILE, 'test/lisp/uniquify-tests.el',
                     'test/lisp/progmodes/project-tests.el')


def original_file_display(logical):
    """Preserve the original batch parent and its per-test display helpers."""
    return 'batch'


def _file_argv(paths, runner, case_root, test_root, display, private_fixture):
    display_args = (['--batch', '-Q'] if display == 'batch' else
                    ['-nw', '-Q', '--no-splash'] if display == 'terminal' else
                    ['-Q', '--no-splash'])
    if display != 'batch':
        display_args.extend(['--eval', '(setq native-comp-async-jobs-number 1)'])
    load_dirs = (['-L', str(case_root / 'out/test'), '-L', str(test_root)]
                 if private_fixture else ['-L', str(test_root)])
    argv = [str(paths['program']), *paths['dump_args'], *display_args,
            *load_dirs, '-l', str(runner), '-f', 'stable-suite-main']
    if display != 'batch':
        argv.extend(['--eval', '(kill-emacs 0)'])
    return argv


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


def _prepare_test_path_fixture(api, out, paths, case_root, logical, environment):
    """Supply original relative paths only inside one owned file fixture."""
    root = Path(case_root) / 'out'
    if logical not in {'test/lisp/uniquify-tests.el',
                       'test/lisp/progmodes/project-tests.el',
                       'test/lisp/progmodes/elisp-mode-tests.el'}:
        return None
    if root.resolve() != root or not (root / 'test').is_dir():
        raise ValueError('relative test layout requires an owned physical test root')
    value = dict(schema='femacs-owned-test-path-layout-v1', root=str(root),
                 file=logical, original_test_modified=False, product_layout_modified=False)
    if logical in {'test/lisp/uniquify-tests.el', 'test/lisp/progmodes/project-tests.el'}:
        directory = root / 'lib-src'
        directory.mkdir()
        value.update(cwd=str(root / 'test'), directory=str(directory),
                     directory_node=_context_node(directory))
        source = Path(out) / 'source-context-data' / 'README'
        expected = _fixture_node(api, source)
        if source.resolve() != source or expected['kind'] != 'file':
            raise ValueError('pinned source-context README required')
        target = root / 'README'
        with source.open('rb') as incoming, target.open('xb') as outgoing:
            shutil.copyfileobj(incoming, outgoing, 65536)
            outgoing.flush()
            os.fsync(outgoing.fileno())
        target.chmod(expected['mode'])
        if _fixture_node(api, source) != expected or _fixture_node(api, target) != expected:
            raise ValueError('owned source-context README copy differs')
        # The original test assumes source-directory is the project root.
        # A context nested inside the developer's checkout cannot satisfy that
        # assumption: project-name transforms its deep relative suffix too.
        git = shutil.which('git', path=os.environ.get('PATH', os.defpath))
        if git is None or os.path.lexists(root / '.git'):
            raise ValueError('fresh owned project initialization required')
        argv = [git, '-c', 'init.templateDir=', '-c', 'core.hooksPath='+os.devnull,
                'init', '-q', str(root)]
        started = time.monotonic()
        environment.update(GIT_CONFIG_NOSYSTEM='1', GIT_CONFIG_GLOBAL=os.devnull)
        git_environment = dict(environment,
            PATH=os.environ.get('PATH', os.defpath), LC_ALL='C',
            GIT_CONFIG_NOSYSTEM='1', GIT_CONFIG_GLOBAL=os.devnull)
        process = subprocess.run(argv, cwd=root, env=git_environment,
            capture_output=True, timeout=5)
        if process.returncode != 0 or len(process.stdout)+len(process.stderr) > 65536:
            raise ValueError('owned test project initialization did not complete')
        value.update(source_context=str(root), readme_source=str(source),
            readme_before=expected,
            project_initialization=dict(argv=argv, exit_code=process.returncode,
                seconds=time.monotonic()-started, stdout_bytes=len(process.stdout),
                stderr_bytes=len(process.stderr), global_configuration_read=False,
                original_repository_modified=False))
        if logical == 'test/lisp/progmodes/project-tests.el':
            # The original project-files assertions require tracked resource
            # files. Index only this owned, manifest-bound fixture; no commit
            # or author identity and no writes to the developer's checkout.
            argv = [git, '-c', 'core.hooksPath='+os.devnull, 'add', '-f', '--', 'test']
            started = time.monotonic()
            process = subprocess.run(argv, cwd=root, env=git_environment,
                                     capture_output=True, timeout=5)
            if process.returncode != 0 or len(process.stdout)+len(process.stderr) > 65536:
                raise ValueError('owned project fixture indexing did not complete')
            value['project_indexing'] = dict(argv=argv, exit_code=process.returncode,
                seconds=time.monotonic()-started, stdout_bytes=len(process.stdout),
                stderr_bytes=len(process.stderr), original_repository_modified=False)
    library = Path(paths['lisp_root'])
    alias = root / 'lisp'
    if library.resolve() != library or not library.is_dir() or os.path.lexists(alias):
        raise ValueError('product library target or owned alias differs')
    alias.symlink_to(library, target_is_directory=True)
    value.update(alias=str(alias), alias_node=_context_node(alias),
                 product_lisp=str(library), alias_removed_after_process=False)
    return value


def _finish_test_path_fixture(api, fixture):
    if fixture is None:
        return None
    root = Path(fixture['root'])
    if root.resolve() != root:
        raise api.SuiteError('owned relative test root changed')
    if 'alias' in fixture:
        alias = Path(fixture['alias'])
        if (alias.parent != root or not alias.is_symlink()
                or _context_node(alias) != fixture['alias_node']
                or alias.resolve(strict=True) != Path(fixture['product_lisp'])):
            raise api.SuiteError('owned xref alias changed; preserve the fixture')
        # Only unlink the exact link this execution created, after its process
        # group has been reaped. Canonical state retirement never follows it.
        alias.unlink()
        fixture['alias_removed_after_process'] = not os.path.lexists(alias)
    if 'directory' in fixture:
        directory = Path(fixture['directory'])
        node = _context_node(directory)
        if (directory.parent != root or not directory.is_dir() or directory.is_symlink()
                or any(node[key] != fixture['directory_node'][key]
                       for key in ('device', 'inode', 'mode'))):
            raise api.SuiteError('owned relative test directory changed')
        fixture['directory_after'] = node
        if (_fixture_node(api, root / 'README') != fixture['readme_before']
                or _fixture_node(api, Path(fixture['readme_source'])) != fixture['readme_before']):
            raise api.SuiteError('owned project README or canonical source changed')
        fixture['readme_unchanged'] = True
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
    if logical == COMPILER_OUTPUT_FILE and active == BOOTSTRAP_CASE:
        # The unchanged original case performs two full compiler bootstraps.
        # Preserve its cmp assertion and expose the selected budget explicitly.
        return max(float(default), 600.0)
    return default


def _case_deadline_context(logical, default):
    if logical == COMPILER_OUTPUT_FILE:
        return dict(profile='original-two-stage-native-bootstrap-budget-v1',
                    test_id=BOOTSTRAP_CASE, original_default_seconds=default,
                    selected_case_seconds=max(float(default), 600.0),
                    sequential_full_compilations=2,
                    original_test_and_assertions_unchanged=True)
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


def _retirement_module(api):
    import importlib.util
    path = Path(api.__file__).with_name('runtime-retire.py')
    spec = importlib.util.spec_from_file_location('femacs_stable_retirement', path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def _exclusive_json(path, value):
    with Path(path).open('x', encoding='utf-8') as stream:
        json.dump(value, stream, indent=2, allow_nan=False)
        stream.write('\n')
        stream.flush()
        os.fsync(stream.fileno())


class _DeferredRetirements:
    """Resolve transient occupancy before sealing each single-write owner receipt."""
    def __init__(self, api):
        self.api, self.module, self.pending = api, _retirement_module(api), []

    def _attempt(self, item):
        item['attempts'] += 1
        item['last_attempt'] = time.monotonic()
        try:
            if item.get('snapshot') and self.api._sha256_path(item['snapshot']) != item['snapshot_sha256']:
                raise ValueError('closure-time retirement proof changed before retry')
            snapshot = (_reuse_json(self.api, item['snapshot'], 32*1024*1024)
                        if item.get('snapshot') else None)
            value = self.module.retire(item['root'], STATE_NAMES, item['path'], item['result'],
                empty_parents=item['parents'], expected_state=snapshot,
                attempt_index=item['attempts'], prior_occupancy_attempts=item['observations'],
                record_occupancy=True)
            observation = value.get('attempt_observation')
            if observation is not None:
                item['observations'].append(observation)
            item['result']['state_retirement'] = value
            if value.get('status') == 'complete':
                return True
            if snapshot is not None and value.get('state_drift') is True:
                item['locked'] = True
            # A deletion may only be retried when the canonical module reports
            # an occupancy observation, never after partial ledger creation.
            if observation is None or observation.get('clear') is True:
                item['locked'] = True
            if not item.get('snapshot') and not item.get('locked'):
                proof = self.module.snapshot_owned_state(item['root'], STATE_NAMES,
                    empty_parents=item['parents'])
                proof_path = item['path'].with_name(item['path'].stem+'.retirement-closure.private.json')
                _exclusive_json(proof_path, proof)
                item['snapshot'] = proof_path
                item['snapshot_sha256'] = self.api._sha256_path(proof_path)
            if item.get('snapshot') and self.api._sha256_path(item['snapshot']) != item['snapshot_sha256']:
                raise ValueError('closure-time retirement proof changed')
        except (OSError, ValueError) as error:
            item['locked'] = True
            item['result']['state_retirement'] = dict(status='retained',
                reason='retirement proof or canonical operation failed',
                error=f'{type(error).__name__}: {error}',
                prior_occupancy_attempts=item['observations'])
        return False

    def _seal(self, item):
        if item['result']['state_retirement'].get('status') != 'complete':
            item['result']['state_retirement']['prior_occupancy_attempts'] = list(item['observations'])
        _record_retirement(self.api, item['path'], item['result'])

    def submit(self, path, result, root, parents):
        item = dict(path=path, result=result, root=root, parents=parents,
                    attempts=0, observations=[], locked=False)
        complete = self._attempt(item)
        for delay in (0.15, 0.50):
            if complete or item['locked']:
                break
            time.sleep(delay)
            complete = self._attempt(item)
        if complete or item['locked']:
            self._seal(item)
        else:
            self.pending.append(item)

    def sweep(self, *, final=False):
        started = time.monotonic()
        for round_index in range(2 if final else 1):
            if round_index:
                if time.monotonic()-started >= 49.0:
                    break
                time.sleep(1.0)
            checked = 0
            for item in list(self.pending):
                if final and time.monotonic()-started >= 49.0:
                    break  # A lsof query itself has a ten-second hard bound.
                if item['attempts'] >= (7 if final else 5):
                    continue
                if not final and (checked >= 2 or time.monotonic()-item['last_attempt'] < 5.0):
                    continue
                if final:
                    # Reserve three seconds for owned-query reaping and seals.
                    with self.module.retirement_deadline(max(0.1, 57.0-(time.monotonic()-started))):
                        complete = self._attempt(item)
                else:
                    complete = self._attempt(item)
                checked += 1
                if complete or item['locked']:
                    self._seal(item)
                    self.pending.remove(item)
        if final:
            for item in self.pending:
                self._seal(item)
            self.pending.clear()


def _seal_retained_state(api, path, result, root, parents):
    value = dict(status='retained', reason='file failed/incomplete; preserve original state')
    if result['complete'] and result['exit_code'] == 0 and not result['remaining_group_terminated']:
        try:
            module = _retirement_module(api)
            observation = module.check_occupancy(root.parent)
            value['attempt_observation'] = observation
            if observation['clear'] is True:
                snapshot = module.snapshot_owned_state(root, STATE_NAMES,
                    empty_parents=parents, allow_symlinks=True)
                proof = path.with_name(path.stem+'.retained-state.private.json')
                _exclusive_json(proof, dict(schema='femacs-stable-retained-state-v1',
                    evidence_sha256=result['file_result']['sha256'], snapshot=snapshot))
                value['closure_state_proof'] = dict(path=proof.name,
                    bytes=proof.stat().st_size, sha256=api._sha256_path(proof))
        except (OSError, ValueError) as error:
            value['closure_state_proof_error'] = f'{type(error).__name__}: {error}'
    return value


def _run_file(api, out, paths, runner, test_root, logical, timeout, label,
              *, display='batch', fixture_entries=None, resource_admission=None,
              deferred_retirements=None, exclude_ids=None, reduced_invocation=False):
    if not isinstance(display, str) or display not in {'batch', 'terminal', 'cocoa'}:
        raise api.SuiteError('unsupported original-file execution display')
    if display == 'cocoa' and sys.platform != 'darwin':
        raise api.SuiteError('Cocoa control execution requires Darwin')
    if display == 'terminal' and os.name != 'posix':
        raise api.SuiteError('controlling terminal execution requires POSIX')
    if reduced_invocation not in (True, False):
        raise api.SuiteError('reduced_invocation must be an exact boolean')
    if exclude_ids is not None:
        if (not isinstance(exclude_ids, (list, tuple)) or not exclude_ids
                or any(not isinstance(name, str) or not name for name in exclude_ids)
                or len(set(exclude_ids)) != len(exclude_ids)):
            raise api.SuiteError('excluded IDs must be a unique non-empty string list')
        if display == 'batch' and reduced_invocation:
            # A blocked system call cannot be resumed in the same process, so a
            # reduced second invocation only makes sense for a real display run.
            # Declarative --test-id scope (reduced_invocation=False) may still
            # exclude unrequested identities on the first batch invocation.
            raise api.SuiteError('reduced invocation requires a display context')
    elif reduced_invocation:
        raise api.SuiteError('reduced invocation requires unique non-empty excluded IDs')
    test_environment = paths.get('test_environment', {})
    if (not isinstance(test_environment, dict)
            or set(test_environment) - {'ERC_TESTS_GRAPHICAL', 'ERC_TESTS_GRAPHICAL_ALL'}
            or any(value != '1' for value in test_environment.values())
            or (test_environment and (display != 'cocoa' or not logical.startswith('test/lisp/erc/')))):
        raise api.SuiteError('original graphical opt-in requires an explicit Cocoa ERC control')
    safe = api._safe_component(logical)
    logs = out / 'logs' / label
    results = out / 'results' / label
    logs.mkdir(parents=True, exist_ok=True)
    results.mkdir(parents=True, exist_ok=True)
    stdout_path = logs / (safe + '.stdout.log')
    stderr_path = logs / (safe + '.stderr.log')
    progress_path = results / (safe + '.events.jsonl')
    diagnostic_path = results / (safe + '.runtime-observations.jsonl')
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
        cache = Path.home() / 'AI' / 'caches' / '.noindex'
        cache.mkdir(parents=True, exist_ok=True)
        container = Path(tempfile.mkdtemp(prefix='f', dir=cache))
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
            fixture['relative_path_layout'] = _prepare_test_path_fixture(
                api, out, paths, case_root, logical, environment)
            if fixture['relative_path_layout'] and fixture['relative_path_layout'].get('cwd'):
                cwd = Path(fixture['relative_path_layout']['cwd'])
                preparation['working_directory'] = str(cwd)
            preparation['working_fixture'] = fixture
            preparation_path.write_text(json.dumps(preparation, indent=2)+'\n', encoding='utf-8')
        except BaseException as error:
            preparation.update(state='failed', retained=True, stage='working-fixture',
                error_type=type(error).__name__, error=str(error)[:1024])
            preparation_path.write_text(json.dumps(preparation, indent=2)+'\n', encoding='utf-8')
            raise
    effective_test_root = (case_root / 'out' / 'test') if fixture is not None else test_root
    base_path = os.environ.get('PATH', os.defpath)
    mock_bin = environment.get('FEMACS_STABLE_MOCK_BIN')
    environment.update(
        PATH=(f'{mock_bin}:{base_path}' if mock_bin else base_path),
        LC_ALL='C', LANG='C',
        TERM='xterm-256color' if display == 'terminal' else 'dumb',
        PWD=str(cwd), EMACS_TEST_DIRECTORY=str(effective_test_root),
        FEMACS_RUNTIME_TEST_ROOT=str(case_root), STABLE_SUITE_MODE='batch',
        STABLE_SUITE_FILE=str(physical_file), STABLE_SUITE_FILE_ID=logical,
        STABLE_SUITE_PROGRESS=str(progress_path), STABLE_SUITE_RESULT=str(final_path))
    if logical == COMPILER_OUTPUT_FILE or logical == 'test/lisp/net/tramp-tests.el' or logical.startswith('test/lisp/erc/erc-scenarios-'):
        budget = _case_deadline_context(logical, timeout) or {}
        environment.update(STABLE_SUITE_DIAGNOSTIC=str(diagnostic_path),
            STABLE_SUITE_TIMEOUT=str(timeout),
            STABLE_SUITE_DIAGNOSTIC_SPECIAL_CASE=budget.get('test_id', ''),
            STABLE_SUITE_DIAGNOSTIC_SPECIAL_SECONDS=str(budget.get('selected_case_seconds', timeout)))
    environment.update(paths.get('runtime_env', {}))
    for key in ('ERC_TESTS_GRAPHICAL', 'ERC_TESTS_GRAPHICAL_ALL'):
        environment.pop(key, None)
    environment.update(test_environment)
    if display != 'batch':
        environment['STABLE_SUITE_DISPLAY'] = display
        if display == 'cocoa':
            environment['FEMACS_COCOA_IGNORE_PERSISTENT_STATE'] = '1'
        else:
            environment.pop('FEMACS_COCOA_IGNORE_PERSISTENT_STATE', None)
        # num-processors honors this limit before command-line Lisp runs.
        # Its default native-worker policy uses half that available count.
        environment['OMP_THREAD_LIMIT'] = '1'
    else:
        environment.pop('STABLE_SUITE_DISPLAY', None)
        environment.pop('FEMACS_COCOA_IGNORE_PERSISTENT_STATE', None)
    if exclude_ids is not None:
        environment['STABLE_SUITE_EXCLUDE_IDS'] = ','.join(exclude_ids)
    else:
        environment.pop('STABLE_SUITE_EXCLUDE_IDS', None)
    environment.pop('STABLE_SUITE_MACHINE_ERROR_DELIVERY', None)
    for key in ('STABLE_SUITE_PRODUCT_PROGRAM', 'STABLE_SUITE_PRODUCT_DUMP'):
        environment.pop(key, None)
    if logical in NESTED_TERMINAL_SCENARIO_FILES:
        dumps = [arg for arg in paths['dump_args'] if arg.startswith('--dump-file=')]
        if len(dumps) != 1 or len(paths['dump_args']) != 1:
            raise api.SuiteError('original terminal helper requires the selected product dump')
        environment.update(
            STABLE_SUITE_MACHINE_ERROR_DELIVERY='erc-original-terminal-errors-v1',
            STABLE_SUITE_PRODUCT_PROGRAM=str(paths['program']),
            STABLE_SUITE_PRODUCT_DUMP=dumps[0])
    for key in ('STABLE_SUITE_ARCHIVE_SOURCE_PROFILE', 'STABLE_SUITE_ARCHIVE_SHARED_CONTEXT',
                'STABLE_SUITE_INVOCATION_DIRECTORY', 'STABLE_SUITE_INVOCATION_NAME'):
        environment.pop(key, None)
    source_context = paths['test_context']
    if (fixture and fixture.get('relative_path_layout')
            and fixture['relative_path_layout'].get('source_context')):
        source_context = fixture['relative_path_layout']['source_context']
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
    argv = _file_argv(paths, runner, case_root, test_root, display, fixture is not None)
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
    excluded_seen = []
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
            if display == 'cocoa' and (
                    value.get('noninteractive') is not False
                    or value.get('window_system') != 'ns'
                    or value.get('display_graphic') is not True
                    or value.get('native_comp_async_jobs') != 1
                    or value.get('native_comp_thread_limit') != '1'
                    or value.get('available_processors') != 1):
                raise api.SuiteError('original-file context is not a real Cocoa control')
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
        elif event == 'excluded':
            # Declared by the runner when the driver asked for a reduced
            # second invocation.  It must match the request exactly and may
            # only name identities that the file itself registered.
            if exclude_ids is None or value.get('excluded') != sorted(exclude_ids):
                raise api.SuiteError('runner exclusion does not match the driver request')
            if loaded is not None:
                raise api.SuiteError('exclusion arrived after registration')
            excluded_seen.extend(value['excluded'])
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
    # A reduced invocation must have announced exactly the requested exclusion,
    # and the file must have ended consistently with the reduced ID set.
    if exclude_ids is not None:
        if sorted(excluded_seen) != sorted(exclude_ids):
            protocol_error = protocol_error or 'declared exclusion was not confirmed by the runner'
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
        input_context=paths.get('reuse_input_binding'),
        passed=complete and all(row['assessment']['passed'] for row in observed.values()),
        timed_out=timed_out, timed_out_id=timed_out_id, protocol_error=protocol_error,
        runner_error=finished.get('error') if finished else None,
        argv=argv, pid=process.pid if process else None, cwd=str(cwd),
        runtime_environment=dict(paths.get('runtime_env', {})),
        test_environment=dict(test_environment),
        mock_transport_context=(dict(profile=environment['FEMACS_STABLE_MOCK_TRANSPORT_PROFILE'],
            content_sha256=environment['FEMACS_STABLE_MOCK_TRANSPORT_SHA256'],
            script=environment['FEMACS_STABLE_MOCK_SHELL_ENV'])
            if 'FEMACS_STABLE_MOCK_TRANSPORT_PROFILE' in environment else None),
        seconds=time.monotonic()-started, exit_code=process.returncode if process else None,
        remaining_group_terminated=remaining_group_terminated,
        stdout_log=str(stdout_path.relative_to(out)), stderr_log=str(stderr_path.relative_to(out)),
        progress=(dict(path=str(progress_path.relative_to(out)), bytes=progress_path.stat().st_size,
                       sha256=api._sha256_path(progress_path)) if progress_path.is_file() else None),
        diagnostic_trace=(dict(path=str(diagnostic_path.relative_to(out)), bytes=diagnostic_path.stat().st_size,
                               sha256=api._sha256_path(diagnostic_path)) if diagnostic_path.is_file() else None),
        owned_state_root=str(case_root), owned_container=str(container),
        environment_preparation=str(preparation_path.relative_to(out)),
        resource_admission=resource_admission,
        machine_error_delivery=environment.get('STABLE_SUITE_MACHINE_ERROR_DELIVERY'),
        case_deadline_context=_case_deadline_context(logical, timeout),
        compiler_output_policy=COMPILER_OUTPUT_PROFILE if logical == COMPILER_OUTPUT_FILE else None,
        compiler_output_context=((loaded or finished or {}).get('compiler_output_context')
            if logical == COMPILER_OUTPUT_FILE else None),
        excluded_ids=(list(exclude_ids) if exclude_ids is not None else None),
        excluded_confirmed=(sorted(excluded_seen) if exclude_ids is not None else None),
        state_retirement_path=str(final_path.with_name(
            final_path.stem + '.owner-retirement.json').relative_to(out)))
    if fixture is not None:
        _finish_test_path_fixture(api, fixture.get('relative_path_layout'))
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
    elif display == 'cocoa':
        result['execution_context'] = dict(display=display, owned_editor_pid=process.pid if process else None,
            noninteractive=loaded.get('noninteractive') if loaded else None,
            window_system=loaded.get('window_system') if loaded else None,
            display_graphic=loaded.get('display_graphic') if loaded else None,
            native_comp_async_jobs=loaded.get('native_comp_async_jobs') if loaded else None,
            native_comp_thread_limit=loaded.get('native_comp_thread_limit') if loaded else None)
    final_path.write_text(json.dumps(result, indent=2)+'\n', encoding='utf-8')
    result['file_result'] = dict(path=str(final_path.relative_to(out)),
                               sha256=api._sha256_path(final_path), bytes=final_path.stat().st_size)
    if result['passed']:
        queue = deferred_retirements or _DeferredRetirements(api)
        queue.submit(final_path, result, case_root,
                     ((case_root, case_node), (container, container_node)))
        if deferred_retirements is None:
            queue.sweep(final=True)
    else:
        result['state_retirement'] = _seal_retained_state(api, final_path, result, case_root,
            ((case_root, case_node), (container, container_node)))
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


def _reuse_json(api, path, limit=16*1024*1024):
    path = Path(path)
    if path.resolve() != path or path.is_symlink():
        raise ValueError('reuse evidence traverses a symlink')
    fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
    with os.fdopen(fd, 'rb') as stream:
        before = os.fstat(stream.fileno())
        if not stat.S_ISREG(before.st_mode) or before.st_size > limit:
            raise ValueError('reuse evidence is not a bounded ordinary file')
        raw = stream.read(limit+1)
        after = os.fstat(stream.fileno())
    fields = ('st_dev', 'st_ino', 'st_size', 'st_mtime_ns', 'st_mode')
    if len(raw) > limit or any(getattr(before, k) != getattr(after, k)
                              or getattr(after, k) != getattr(path.lstat(), k) for k in fields):
        raise ValueError('reuse evidence changed while reading')
    def pairs(rows):
        value = {}
        for key, item in rows:
            if key in value:
                raise ValueError('duplicate reuse evidence field')
            value[key] = item
        return value
    return json.loads(raw, object_pairs_hook=pairs)


def _reuse_identity(api, path):
    path = Path(path)
    if path.resolve() != path or not path.is_file() or path.is_symlink():
        raise ValueError('reuse input must be a physical ordinary file')
    fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
    with os.fdopen(fd, 'rb') as stream:
        node = os.fstat(stream.fileno())
        if not stat.S_ISREG(node.st_mode) or node.st_size > 256*1024*1024:
            raise ValueError('reuse identity exceeds ordinary-file bound')
        sha, count = hashlib.sha256(), 0
        while block := stream.read(65536):
            count += len(block)
            if count > 256*1024*1024:
                raise ValueError('reuse identity grew beyond bound')
            sha.update(block)
        reaped = os.fstat(stream.fileno())
    value = dict(bytes=count, sha256=sha.hexdigest(), mode=stat.S_IMODE(node.st_mode))
    after = path.lstat()
    fields = ('st_dev', 'st_ino', 'st_size', 'st_mtime_ns', 'st_mode')
    if count != node.st_size or any(getattr(node, key) != getattr(reaped, key)
                                   or getattr(reaped, key) != getattr(after, key) for key in fields):
        raise ValueError('reuse input changed during hashing')
    return value


def _copy_reuse_artifact(api, source, target, *, limit=128*1024*1024):
    identity = _reuse_identity(api, source)
    if identity['bytes'] > limit:
        raise ValueError('closed artifact exceeds reuse bound')
    target.parent.mkdir(parents=True, exist_ok=True)
    fd = os.open(source, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
    with os.fdopen(fd, 'rb') as incoming, target.open('xb') as outgoing:
        before = os.fstat(incoming.fileno())
        count = 0
        while block := incoming.read(65536):
            count += len(block)
            if count > limit:
                raise ValueError('closed artifact grew during copy')
            outgoing.write(block)
        outgoing.flush()
        os.fsync(outgoing.fileno())
        after = os.fstat(incoming.fileno())
    target.chmod(identity['mode'])
    fields = ('st_dev', 'st_ino', 'st_size', 'st_mtime_ns', 'st_mode')
    if (any(getattr(before, k) != getattr(after, k) for k in fields)
            or _reuse_identity(api, source) != identity or _reuse_identity(api, target) != identity):
        raise ValueError('closed artifact changed during copy')
    return identity


def _canonical_harness(api, sources):
    roles = {}
    for name in sources:
        role = Path(name).name if Path(name).suffix == '.py' else 'stable-suite-runner.el'
        if role in roles:
            raise ValueError('duplicate canonical harness role')
        roles[role] = _reuse_identity(api, Path(name))
    header = Path(api.__file__).parent/'fixtures/module-thread-config.h'
    roles['module-thread-config.h'] = _reuse_identity(api, header)
    expected = {'stable-suite.py', 'stable-suite-batch.py', 'stable-context.py',
                'runtime-retire.py', 'reference-corpus.py', 'module-thread-config.h',
                'stable-suite-runner.el'}
    if set(roles) != expected:
        raise ValueError('closed-file reuse requires exactly seven canonical harness roles')
    return roles


def _input_context(api, manifest, paths, sources, context, timeout, manifest_identity):
    return dict(schema='femacs-stable-reuse-inputs-v1',
        manifest_sha256=manifest_identity,
        products={label: api._public_product_identity(value) for label, value in paths.items()},
        runtime_environment={label: dict(value.get('runtime_env', {})) for label, value in paths.items()},
        harness_roles=_canonical_harness(api, sources), timeout_seconds=timeout,
        environment_policy=dict(PATH=os.environ.get('PATH', os.defpath), LC_ALL='C', LANG='C',
                                TERM='dumb', display='per-original-file-v1', platform=sys.platform,
                                nested_terminal_scenario_files=list(NESTED_TERMINAL_SCENARIO_FILES),
                                terminal_TERM='xterm-256color', terminal_OMP_THREAD_LIMIT='1',
                                nested_terminal_OMP_THREAD_LIMIT='1',
                                native_comp_async_jobs_number=1,
                                test_environment={},
                                always_live_files=list(ALWAYS_LIVE_FILES),
                                machine_error_delivery='erc-original-terminal-errors-v1',
                                python_version=sys.version,
                                python_program=_reuse_identity(api, Path(sys.executable).resolve()),
                                uname=list(os.uname()) if hasattr(os, 'uname') else None),
        source_context={key: context[key] for key in ('manifest_sha256', 'selection_sha256', 'file_count', 'bytes')})


def _verify_file_events(api, root, result):
    progress = result['progress']
    path = root/api._relative_path(progress['path'], 'closed progress')
    if _reuse_identity(api, path)['sha256'] != progress['sha256'] or path.stat().st_size != progress['bytes']:
        raise ValueError('closed progress identity changed')
    loaded, active, ended, finished, total = False, None, {}, False, 0
    with path.open('rb') as stream:
        for ordinal in range(300002):
            raw = stream.readline(1024*1024+1)
            if not raw:
                break
            total += len(raw)
            if len(raw) > 1024*1024 or total > 64*1024*1024 or not raw.endswith(b'\n'):
                raise ValueError('closed progress exceeds finite bounds')
            event = json.loads(raw)
            if event.get('file') != result['file'] or finished:
                raise ValueError('closed progress file/order mismatch')
            kind, identity = event.get('event'), event.get('test_id')
            if kind == 'loaded' and not loaded:
                if (event.get('ids') != result['ids'] or event.get('selector') != result['selector']
                        or api._metadata(event) != result['metadata']
                        or {row['id']: row['tags'] for row in event.get('tests', [])} != result['tags']):
                    raise ValueError('closed progress registration mismatch')
                loaded = True
            elif kind == 'test-started' and loaded and active is None and identity not in ended:
                if identity not in result['ids']:
                    raise ValueError('closed progress started an unregistered case')
                active = identity
            elif kind == 'test-ended' and active == identity:
                value = result['cases'][identity]
                if (event.get('status') != value['status']
                        or event.get('diagnostic', '') != value['diagnostic']
                        or event.get('duration') != value['seconds']
                        or value['evidence'] != dict(path=progress['path'],
                            event_sha256=hashlib.sha256(raw[:-1]).hexdigest(), event_ordinal=len(ended))
                        or value.get('complete') is not True or value.get('exit_code') != 0
                        or value['assessment'] != api.assess_case(identity, value)):
                    raise ValueError('closed case/event/assessment mismatch')
                ended[identity], active = True, None
            elif kind == 'finished' and loaded and active is None:
                if (event.get('status') != 'complete' or event.get('requested_count') != len(result['ids'])
                        or event.get('ended_count') != len(ended)):
                    raise ValueError('closed file did not finish every registered case')
                finished = True
            else:
                raise ValueError('invalid closed progress transition')
        else:
            raise ValueError('closed progress event count exceeds bound')
    if not loaded or not finished or set(ended) != set(result['ids']):
        raise ValueError('closed progress is incomplete')


def _try_reuse_file(api, spec, out, paths, runner, logical, timeout, label, fixtures):
    if logical in ALWAYS_LIVE_FILES:
        # These owned fixtures invoke an external compiler or Git project
        # setup whose complete toolchain is not part of the donor envelope.
        return None
    for donor in spec.get('donors', []):
        donor_label = donor['roles'].get(label)
        if donor_label is None:
            continue
        root = Path(donor['pair_root'])
        safe = api._safe_component(logical)
        file_path = root/'results'/donor_label/(safe+'.json')
        if not file_path.is_file():
            continue
        result = _reuse_json(api, file_path)
        if (result.get('complete') is not True or result.get('exit_code') != 0
                or result.get('timed_out') is not False
                or result.get('remaining_group_terminated') is not False):
            continue  # Incomplete files are always executed fresh.
        if result.get('input_context') != donor['input_context']:
            continue  # A copied file is not a new execution seed for another donor.
        if donor_label != label:
            raise ValueError('closed evidence cannot change its recorded role')
        if (result.get('schema') != 'femacs-stable-file-batch-v1'
                or result.get('file') != logical or result.get('product') != label
                or result.get('selector') != 'all-discovered-ids'
                or result.get('runtime_environment') != dict(paths.get('runtime_env', {}))
                or result.get('test_environment') != {}
                or len(result['ids']) != len(set(result['ids']))
                or set(result['ids']) != set(result['cases'])
                or result['passed'] is not all(row['assessment']['passed'] for row in result['cases'].values())):
            raise ValueError('closed file scope differs')
        fixture = result.get('working_fixture', {})
        if fixture.get('canonical_fixture_inputs_unchanged') is not True:
            raise ValueError('closed fixture has no unchanged-input proof')
        actual = {row['path']: row['before'] for row in fixture.get('files', [])}
        for entry in fixtures:
            expected = actual.get(entry['path'], {})
            if (expected.get('sha256') != entry['sha256'] or expected.get('bytes') != entry['size']
                    or expected.get('mode') != (int(entry['mode'], 8) & 0o777)):
                raise ValueError('closed fixture differs from manifest input')
        if set(actual) != {row['path'] for row in fixtures}:
            raise ValueError('closed fixture file set differs')
        case_root = Path(result['owned_state_root'])
        display = original_file_display(logical)
        argv = _file_argv(paths, donor['runner'], case_root, root/'test', display, True)
        delivery = ('erc-original-terminal-errors-v1'
                    if logical in NESTED_TERMINAL_SCENARIO_FILES else None)
        if result.get('machine_error_delivery') != delivery:
            raise ValueError('closed original-file error-delivery scope differs')
        if result.get('argv') != argv or result.get('case_deadline_context') != _case_deadline_context(logical, timeout):
            raise ValueError('closed file argv/display/deadline differs')
        _verify_file_events(api, root, result)
        owner_path = file_path.with_name(safe+'.owner-retirement.json')
        if not owner_path.exists():
            continue  # Deferred/interrupted owners have not sealed their outcome.
        ownership = _reuse_json(api, owner_path)
        file_identity = dict(path=str(file_path.relative_to(root)),
            sha256=api._sha256_path(file_path), bytes=file_path.stat().st_size)
        if ownership.get('schema') != 'femacs-stable-owner-retirement-v1' or ownership.get('file_result') != file_identity:
            raise ValueError('closed owner receipt does not bind the file evidence')
        retirement = ownership['state_retirement']
        relative = [file_identity['path'], result['progress']['path'], result['environment_preparation'],
                    result['stdout_log'], result['stderr_log'], str(owner_path.relative_to(root))]
        if result.get('diagnostic_trace') is not None:
            diagnostic = result['diagnostic_trace']
            diagnostic_path = root/api._relative_path(diagnostic['path'], 'closed diagnostic trace')
            if (api._sha256_path(diagnostic_path) != diagnostic['sha256']
                    or diagnostic_path.stat().st_size != diagnostic['bytes']):
                raise ValueError('closed diagnostic trace changed')
            relative.append(diagnostic['path'])
        if result['passed']:
            if (retirement.get('status') != 'complete' or retirement.get('all_selected_absent') is not True
                    or retirement.get('evidence_sha256') != file_identity['sha256']
                    or os.path.lexists(case_root) or os.path.lexists(result['owned_container'])):
                continue
            report_path = file_path.with_name(safe+'.state-retirement.json')
            report = _reuse_json(api, report_path)
            ledger_path = file_path.with_name(safe+'.state-retirement.jsonl.gz')
            private_path = file_path.with_name(safe+'.state-retirement.private.json')
            private = _reuse_json(api, private_path, 32*1024*1024)
            if (report != retirement or api._sha256_path(ledger_path) != report['ledger_sha256']
                    or private.get('evidence_sha256') != file_identity['sha256']):
                raise ValueError('closed retirement hash chain differs')
            relative.extend(str(p.relative_to(root)) for p in (report_path, ledger_path, private_path))
        else:
            proof = retirement.get('closure_state_proof')
            if not proof:
                continue  # A consumption-time snapshot cannot repair missing closure proof.
            proof_path = file_path.parent/api._relative_path(proof['path'], 'closure state proof')
            if api._sha256_path(proof_path) != proof['sha256'] or proof_path.stat().st_size != proof['bytes']:
                raise ValueError('retained closure-time proof changed')
            value = _reuse_json(api, proof_path, 32*1024*1024)
            if value.get('evidence_sha256') != file_identity['sha256']:
                raise ValueError('retained closure-time proof binds another file')
            module = _retirement_module(api)
            if not module.check_occupancy(Path(result['owned_container']))['clear']:
                # This donor file itself failed or was incomplete, so its case
                # state is retained by design and cannot be re-admitted here.
                # Refuse only that file's reuse and execute it fresh; aborting
                # the whole pair would discard every other file's valid donor.
                return None
            module.verify_owned_state(value['snapshot'])
            relative.append(str(proof_path.relative_to(root)))
        copied = {}
        for name in relative:
            name = api._relative_path(name, 'closed artifact path')
            copied[name] = _copy_reuse_artifact(api, root/name, out/name)
        if not result['passed']:
            module.verify_owned_state(value['snapshot'])
        result['file_result'] = file_identity
        result['state_retirement'] = retirement
        result['state_retirement_receipt'] = dict(path=result['state_retirement_path'],
            sha256=api._sha256_path(out/result['state_retirement_path']), bytes=owner_path.stat().st_size)
        result['reuse'] = dict(schema='femacs-stable-file-reuse-record-v1',
            donor_stage=donor['stage'], donor_pair_root=str(root), donor_file=file_identity,
            artifacts=copied, physical_retirement_performed_by_this_run=False,
            retired_logical_bytes_attributed_to_this_run=0,
            reused_state_retirement=result['passed'], original_statuses_preserved=True)
        _exclusive_json(out/'results'/label/(safe+'.reuse.json'), result['reuse'])
        return result
    return None


def execute(api, manifest, repo, out, reference, candidate, reference_dump_args,
            candidate_dump_args, timeout, reference_env=None, candidate_env=None,
            *, corpus_root=None, runner_path=None, reuse_files=None, candidate_profile=None,
            manifest_identity=None, subset=None):
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
    inputs = _input_context(api, manifest, paths, harness_sources, context_record, timeout,
        manifest_identity or api._sha256_path(repo/'compatibility/stable-31.1/sources.json'))
    inputs['candidate_profile'] = candidate_profile
    inputs['source_context_receipt_sha256'] = api._sha256_path(out/'source-context.json')
    inputs_path = out/'stable-suite-inputs.json'
    _exclusive_json(inputs_path, inputs)
    inputs_identity = _reuse_identity(api, inputs_path)
    for product in paths.values():
        product['reuse_input_binding'] = dict(path=str(inputs_path), **inputs_identity)
    reuse = dict(donors=[])
    if reuse_files is not None:
        reuse = _reuse_json(api, Path(reuse_files))
        if (reuse.get('schema') != 'femacs-stable-file-reuse-admission-v1'
                or not isinstance(reuse.get('donors'), list) or not 1 <= len(reuse['donors']) <= 48):
            raise api.SuiteError('closed-file reuse requires a bounded stage admission')
        for name, pin in reuse['pins'].items():
            if _reuse_identity(api, Path(name)) != pin:
                raise api.SuiteError('admitted donor input changed before reuse')
        for donor in reuse['donors']:
            donor_inputs = _reuse_json(api, Path(donor['pair_root'])/'stable-suite-inputs.json', 64*1024*1024)
            donor['input_context'] = dict(path=str(Path(donor['pair_root'])/'stable-suite-inputs.json'),
                **_reuse_identity(api, Path(donor['pair_root'])/'stable-suite-inputs.json'))
            for key in ('schema', 'manifest_sha256', 'harness_roles', 'timeout_seconds',
                        'environment_policy', 'source_context'):
                if donor_inputs[key] != inputs[key]:
                    raise api.SuiteError('closed-file donor input context differs: '+key)
            donor['roles'] = {label: label for label in paths
                if donor_inputs['products'][label] == inputs['products'][label]
                and donor_inputs['runtime_environment'][label] == inputs['runtime_environment'][label]
                and (label == 'reference' or (candidate_profile is not None
                    and donor_inputs.get('candidate_profile') == candidate_profile))}
            if not donor['roles']:
                raise api.SuiteError('closed-file donor has no identical product role')
            donor_out = Path(donor['pair_root'])
            if not _source_inputs_unchanged(api, manifest, donor_out):
                raise api.SuiteError('closed-file donor corpus changed')
            donor_context_record = _reuse_json(api, donor_out/'source-context.json')
            if api._sha256_path(donor_out/'source-context.json') != donor_inputs['source_context_receipt_sha256']:
                raise api.SuiteError('donor context differs from its execution-time binding')
            context_provider.verify(api, repo, donor_out, context, donor_context_record)
    queue = _DeferredRetirements(api)
    all_entries = select_entrypoints(manifest)
    entries = select_entrypoints(manifest, subset) if subset is not None else all_entries
    files, cases, failures = [], [], []
    subset_names = [entry['path'] for entry in entries] if subset is not None else None
    corpus_unchanged = True
    reference_keys, candidate_keys = set(), set()
    try:
        for entry in entries:
            logical = entry['path']
            pair = {}
            for label, product in paths.items():
                if corpus_unchanged:
                    fixtures = _fixture_entries(manifest, logical)
                    pair[label] = _try_reuse_file(api, reuse, out, product, runner,
                        logical, timeout, label, fixtures)
                    if pair[label] is None:
                        pair[label] = _run_file(
                            api, out, product, runner, out/'test', logical, timeout, label,
                            display=original_file_display(logical),
                            fixture_entries=fixtures, deferred_retirements=queue)
                    corpus_unchanged = _source_inputs_unchanged(api, manifest, out)
                else:
                    pair[label] = dict(ids=[], cases={}, tags={}, complete=False, passed=False,
                                       file_result=None, protocol_error='prior file changed corpus inputs')
                if not pair[label]['complete']:
                    failures.append(label+' file incomplete: '+logical)
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
            if len(files) % 25 == 0:
                queue.sweep()
            (out/'partial.json').write_text(json.dumps(dict(schema='femacs-stable-batch-progress-v1',
                completed_files=len(files), requested_files=len(entries), cases=len(cases),
                failure_count=len(failures), latest_file=logical, corpus_inputs_unchanged=corpus_unchanged))+'\n')
    finally:
        queue.sweep(final=True)
        after = {label: api._public_product_identity(value) for label, value in paths.items()}
        context_provider.verify(api, repo, out, context, context_record)
    for row in files:
        for label in paths:
            value = row[label]
            if value.get('file_result'):
                owner_path = out/value['state_retirement_path']
                sealed = _reuse_json(api, owner_path)
                if sealed['file_result'] != value['file_result']:
                    raise api.SuiteError('final owner receipt differs from closed file')
                value['state_retirement'] = sealed['state_retirement']
                value['state_retirement_receipt'] = dict(path=value['state_retirement_path'],
                    sha256=api._sha256_path(owner_path), bytes=owner_path.stat().st_size)
            if value.get('passed') and value.get('state_retirement', {}).get('status') != 'complete':
                failures.append(label+' successful file state retirement incomplete: '+row['file'])
    reused = [dict(file=row['file'], label=label, **row[label]['reuse'])
              for row in files for label in paths if row[label].get('reuse')]
    physically_retired = [row[label]['state_retirement'] for row in files for label in paths
        if not row[label].get('reuse') and row[label].get('state_retirement', {}).get('status') == 'complete']
    retirement_accounting = dict(
        physical_case_retirements=len(physically_retired),
        logical_bytes=sum(value['logical_bytes'] for value in physically_retired),
        prior_allocated_bytes=sum(value['allocated_bytes'] for value in physically_retired),
        reused_retirement_ledgers=[dict(file=row['file'], label=label,
            ledger_sha256=row[label]['state_retirement'].get('ledger_sha256'),
            original_owned_state_root=row[label]['owned_state_root'],
            physical_retirement_performed_by_this_run=False, logical_bytes_attributed_to_this_run=0)
            for row in files for label in paths
            if row[label].get('reuse') and row[label]['state_retirement'].get('status') == 'complete'],
        whole_volume_reclamation_causally_claimed=False)
    reuse_summary = dict(schema='femacs-stable-file-reuse-v1',
        reused_files=reused, count=len(reused), raw_results_modified=False,
        fresh_file_executions=2*len(files)-len(reused), covered_file_sides=2*len(files),
        physical_retirement_performed_by_this_run=False, retired_logical_bytes_attributed_to_this_run=0,
        module_file_always_executed_fresh=True)
    _exclusive_json(out/'stable-file-reuse.json', reuse_summary)
    if _reuse_identity(api, inputs_path) != inputs_identity:
        failures.append('closed-file input context changed during execution')
    if reuse_files is not None:
        if any(_reuse_identity(api, Path(name)) != pin for name, pin in reuse['pins'].items()):
            failures.append('closed-file donor admission changed during execution')
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
        scope=('stable-31.1-original-ELFILES-declared-subset' if subset is not None
               else 'stable-31.1-original-ELFILES-all-discovered-ids'),
        selector='all-discovered-ids',
        subset=subset_names,
        full_corpus_entrypoint_count=len(all_entries),
        source=dict(release=manifest['source']['release'], commit=manifest['source']['commit']),
        materialized=materialized, timeout_seconds=timeout, timeout_scope='per-load-and-per-test',
        source_context=context_record, source_context_unchanged=True,
        entrypoint_count=len(entries), resource_only_lisp_inputs=manifest['lisp_file_count']-len(all_entries),
        products_before=before, products_after=after, identities_unchanged=before==after,
        corpus_inputs_unchanged=corpus_unchanged, runner_identity=dict(path=str(runner),sha256=runner_sha),
        harness_sources=harness_sources,
        file_reuse={key: value for key, value in reuse_summary.items() if key != 'reused_files'},
        state_retirement_accounting=retirement_accounting,
        batch_driver_sha256=api._sha256_path(Path(__file__)), files=files, cases=cases,
        registration=dict(passed=reference_keys==candidate_keys,
            reference_count=len(reference_keys), candidate_count=len(candidate_keys), key_scope='file-and-id'),
        failures=failures,
        corpus_complete=(subset is None and bool(reference_keys) and not failures),
        subset_complete=(subset is not None and bool(reference_keys) and not failures),
        full_compatibility_claim=False, logs_root='logs', results_root='results', elapsed_seconds=None)
