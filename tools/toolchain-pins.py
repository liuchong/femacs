"""Provide the pinned external toolchains described by build-support/toolchain-pins.json.

The build gate (`xtask/src/treesit.rs`) only accepts exact header/library
identities, so a prefix has to exist somewhere before a compiler starts. This
tool makes that prefix reconstructible from the recorded upstream input instead
of depending on whatever directory happened to hold an earlier download.

Two modes:

    verify  --prefix DIR      check DIR against the recorded identities
    install --prefix DIR      download, build, install, then verify

`install` needs network access for the pinned tarball and the usual C toolchain
(`make`, a C compiler). It never touches a system installation and never writes
outside the prefix it was given. Nothing here is a build of the editor.

The tool also rewrites the installed `tree-sitter.pc` so its `prefix=` names the
actual chosen directory: a pkg-config file carrying somebody else's absolute
path is how a cache silently becomes the only source.
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
import tarfile
import tempfile
import urllib.request

REPO = Path(__file__).resolve().parent.parent
PINS = REPO / 'build-support/toolchain-pins.json'
CHUNK = 1024 * 1024
MAX_ARCHIVE_BYTES = 64 * 1024 * 1024


class PinError(Exception):
    pass


def _require(condition, message):
    if not condition:
        raise PinError(message)


def load_pin(name):
    value = json.loads(PINS.read_text(encoding='utf-8'))
    _require(value.get('schema') == 'femacs-external-toolchain-pins-v1',
             'unexpected toolchain pin schema')
    pin = value['toolchains'].get(name)
    _require(isinstance(pin, dict), 'unknown toolchain ' + name)
    return pin


def digest(path):
    handle = hashlib.sha256()
    with Path(path).open('rb') as stream:
        for block in iter(lambda: stream.read(CHUNK), b''):
            handle.update(block)
    return handle.hexdigest()


def verify(name, prefix):
    pin = load_pin(name)
    prefix = Path(os.path.abspath(prefix))
    _require(prefix.is_dir(), 'toolchain prefix is not a directory: %s' % prefix)
    seen = {}
    for relative, expected in pin['identities'].items():
        path = prefix / relative
        _require(path.is_file(), 'missing pinned file: %s' % relative)
        actual = digest(path)
        _require(actual == expected,
                 '%s identity differs from the pin (%s != %s)' % (relative, actual, expected))
        seen[relative] = actual
    # The library bytes are not pinned: a bottle and a source build of the same
    # tree legitimately differ, and the build gate links whatever this prefix
    # provides. What must exist is a real dynamic library to link against.
    library = prefix / 'lib/libtree-sitter.dylib'
    _require(library.is_file(), 'missing lib/libtree-sitter.dylib')
    seen['lib/libtree-sitter.dylib'] = digest(library)
    package = prefix / 'lib/pkgconfig/tree-sitter.pc'
    _require(package.is_file(), 'missing tree-sitter.pc')
    versions = [line.strip()[len('Version:'):].strip()
                for line in package.read_text(encoding='utf-8').splitlines()
                if line.startswith('Version:')]
    _require(versions == [pin['version']],
             'tree-sitter.pc must declare exactly version ' + pin['version'])
    prefixes = [line.strip()[len('prefix='):]
                for line in package.read_text(encoding='utf-8').splitlines()
                if line.startswith('prefix=')]
    _require(prefixes == [str(prefix)],
             'tree-sitter.pc prefix= must name the selected prefix, not another path')
    return dict(toolchain=name, prefix=str(prefix), version=pin['version'],
                identities=seen, package=dict(path='lib/pkgconfig/tree-sitter.pc',
                                              sha256=digest(package)))


def _download(url, target, expected):
    with urllib.request.urlopen(url, timeout=120) as response, target.open('xb') as out:
        total = 0
        while True:
            block = response.read(CHUNK)
            if not block:
                break
            total += len(block)
            _require(total <= MAX_ARCHIVE_BYTES, 'pinned archive exceeds the byte bound')
            out.write(block)
    actual = digest(target)
    _require(actual == expected,
             'downloaded archive identity differs from the pin (%s != %s)' % (actual, expected))


def _rewrite_package(prefix):
    package = prefix / 'lib/pkgconfig/tree-sitter.pc'
    lines = package.read_text(encoding='utf-8').splitlines()
    rewritten, replaced = [], False
    for line in lines:
        if line.startswith('prefix='):
            rewritten.append('prefix=' + str(prefix))
            replaced = True
        else:
            rewritten.append(line)
    _require(replaced, 'tree-sitter.pc has no prefix= line to bind')
    package.write_text('\n'.join(rewritten) + '\n', encoding='utf-8')


def install(name, prefix):
    pin = load_pin(name)
    _require(name == 'tree-sitter', 'only tree-sitter has a recorded recipe')
    _require(os.uname().machine == 'arm64' and sys.platform == 'darwin',
             'the pinned toolchain is validated for Apple Silicon macOS only')
    prefix = Path(os.path.abspath(prefix))
    _require(not os.path.lexists(prefix), 'install prefix must not exist: %s' % prefix)
    source = pin['upstream_source']
    with tempfile.TemporaryDirectory(prefix='femacs-toolchain-') as scratch:
        scratch = Path(scratch)
        archive = scratch / 'tree-sitter.tar.gz'
        _download(source['url'], archive, source['sha256'])
        with tarfile.open(archive) as package:
            for member in package.getmembers():
                target = (scratch / member.name).resolve()
                _require(str(target).startswith(str(scratch.resolve())),
                         'pinned archive escapes its extraction root')
            package.extractall(scratch)
        roots = [entry for entry in scratch.iterdir()
                 if entry.is_dir() and entry.name.startswith('tree-sitter-')]
        _require(len(roots) == 1, 'pinned archive must hold exactly one source tree')
        # The macOS product build stages the linked library's license text and
        # looks for it in the conventional package layout
        # (share/licenses/<package>/).  `make install` does not create that
        # directory, so a source-built prefix would otherwise be unusable even
        # though the library and header are correct.  Place the upstream
        # LICENSE where the collector expects it and keep its identity.
        upstream_license = None
        for candidate in ('LICENSE', 'LICENSE.md', 'COPYING'):
            path = roots[0] / candidate
            if path.is_file():
                upstream_license = path
                break
        _require(upstream_license is not None,
                 'pinned source tree carries no LICENSE text to stage')
        env = dict(os.environ, PREFIX=str(prefix))
        for command in (['make', 'install', 'AMALGAMATED=1', 'PREFIX=' + str(prefix)],):
            result = subprocess.run(command, cwd=roots[0], env=env,
                                    capture_output=True, text=True, timeout=1800)
            _require(result.returncode == 0,
                     'pinned toolchain build failed: ' + (result.stderr or result.stdout)[-2048:])
        # Stage the upstream license while the source tree still exists: the
        # macOS product build collects the linked library's license text from
        # share/licenses/<package>/, which `make install` does not create.
        # `find_package_root` in the product builder derives the package name
        # from the prefix directory name and then looks under
        # share/licenses/<that-name>/.  Use the same name so the prefix works
        # whatever the caller called it, instead of hard-coding "tree-sitter".
        license_dir = prefix / 'share' / 'licenses' / prefix.name
        license_dir.mkdir(parents=True, exist_ok=True)
        staged = license_dir / 'LICENSE'
        shutil.copyfile(upstream_license, staged)
    _rewrite_package(prefix)
    result = verify(name, prefix)
    result['license'] = dict(path='share/licenses/%s/LICENSE' % prefix.name,
                             sha256=digest(staged))
    return result


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('mode', choices=('verify', 'install'))
    parser.add_argument('--toolchain', default='tree-sitter')
    parser.add_argument('--prefix', required=True, type=Path)
    args = parser.parse_args(argv)
    try:
        result = (verify(args.toolchain, args.prefix) if args.mode == 'verify'
                  else install(args.toolchain, args.prefix))
    except (PinError, OSError, ValueError, tarfile.TarError) as error:
        print(json.dumps(dict(passed=False, error=str(error))), file=sys.stderr)
        return 1
    print(json.dumps(dict(passed=True, **result)))
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
