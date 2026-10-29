"""Render the versioned public module ABI from tracked, identity-bound inputs."""
from pathlib import Path
import hashlib
import json
import os
import re
import tempfile


def render_module_api(root: Path) -> Path:
    root = Path(root).resolve()
    inputs = root / 'build-support/module-api'
    manifest = json.loads((inputs / 'sources.json').read_text())
    if manifest.get('schema') != 'femacs-module-api-inputs-v1':
        raise ValueError('unknown public module API input schema')
    source = (root / 'src/emacs.rs').read_text()
    version = re.search(r'PACKAGE_VERSION:\s*&CStr\s*=\s*c"([0-9]+)\.[^"]+"', source)
    if version is None or int(version[1]) != manifest['api_major']:
        raise ValueError('owned version requires an explicitly updated module ABI')
    major = manifest['api_major']
    names = ['emacs-module.in.h', *(f'module-env-{n}.h' for n in range(25, major + 1))]
    if set(manifest['files']) != set(names):
        raise ValueError('missing or unexpected versioned module API inputs')
    contents = {}
    for name in names:
        path = inputs / name
        if path.is_symlink() or not path.is_file() or path.stat().st_size > 1024 * 1024:
            raise ValueError('unsafe public module API input')
        data = path.read_bytes()
        identity = manifest['files'][name]
        if len(data) != identity['bytes'] or hashlib.sha256(data).hexdigest() != identity['sha256']:
            raise ValueError('public module API input identity changed: ' + name)
        contents[name] = data.decode('utf-8')
    result = contents['emacs-module.in.h'].replace('@emacs_major_version@', str(major))
    for n in range(25, major + 1):
        result = result.replace(f'@module_env_snippet_{n}@', contents[f'module-env-{n}.h'])
    if re.search(r'@[A-Za-z_][A-Za-z_0-9]*@', result):
        raise ValueError('unresolved public module API placeholder')
    destination = root / 'build/generation/module-api/emacs-module.h'
    parent = destination.parent
    for path in (root / 'build', root / 'build/generation', parent):
        if path.is_symlink():
            raise ValueError('module ABI output may not follow symlinks')
        path.mkdir(exist_ok=True)
    if destination.is_symlink():
        raise ValueError('module ABI destination may not be a symlink')
    descriptor, temporary = tempfile.mkstemp(prefix='.module-api-', dir=parent)
    try:
        with os.fdopen(descriptor, 'wb') as output:
            output.write(result.encode('utf-8'))
            os.fchmod(output.fileno(), 0o644)
            output.flush()
            os.fsync(output.fileno())
        os.replace(temporary, destination)
    finally:
        Path(temporary).unlink(missing_ok=True)
    return destination
