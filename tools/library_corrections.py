"""Generate explicit product library corrections without editing original inputs.

Unknown upstream content is a review boundary, never an invitation to apply an
old replacement heuristically. Correct behavior is verified by real acceptance;
this generator proves only the pinned source-to-product transformation.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import stat
import tempfile

ROOT = Path(__file__).resolve().parent.parent
RECIPE = 'tools/fixtures/library-corrections.json'
OUTPUT = 'build/generation/library-corrections'
SCHEMA = 'femacs-library-correction-build-v1'
LIMIT = 1024 * 1024


def require(value, message):
    if not value:
        raise ValueError(message)


def physical(path):
    path = Path(os.path.abspath(path))
    require(path.resolve() == path, 'library correction path traverses a symlink')
    return path


def relative(value):
    require(isinstance(value, str) and value, 'library correction relative path required')
    path = Path(value)
    require(not path.is_absolute() and '..' not in path.parts
            and path.as_posix() == value, 'noncanonical library correction path')
    return path


def read(path):
    path = physical(path)
    descriptor = os.open(path, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
    with os.fdopen(descriptor, 'rb') as source:
        before = os.fstat(source.fileno())
        require(stat.S_ISREG(before.st_mode) and before.st_size <= LIMIT,
                'bounded ordinary library correction input required')
        raw = source.read(LIMIT+1)
        after = os.fstat(source.fileno())
    current = path.lstat()
    require(len(raw) <= LIMIT and all(getattr(before, key) == getattr(after, key)
            == getattr(current, key) for key in
            ('st_dev', 'st_ino', 'st_mode', 'st_size', 'st_mtime_ns')),
            'library correction input changed while reading')
    return raw, dict(bytes=len(raw), sha256=hashlib.sha256(raw).hexdigest())


def pairs(items):
    value = {}
    for key, item in items:
        require(key not in value, 'duplicate library correction key')
        value[key] = item
    return value


def document(path):
    raw, identity = read(path)
    return json.loads(raw, object_pairs_hook=pairs), identity


def write(path, raw, mode=0o644):
    path = physical(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = None
    try:
        descriptor, temporary = tempfile.mkstemp(prefix='.correction-', dir=path.parent)
        with os.fdopen(descriptor, 'wb') as target:
            target.write(raw)
            target.flush()
            os.fsync(target.fileno())
        os.chmod(temporary, mode)
        os.replace(temporary, path)
    finally:
        if temporary and os.path.lexists(temporary):
            os.unlink(temporary)


def write_json(path, value):
    write(path, (json.dumps(value, indent=2, sort_keys=True, allow_nan=False)+'\n').encode())


def recipe(repo):
    repo = physical(repo)
    require(repo.is_dir() and repo.name != 'emacs', 'owned candidate source root required')
    value, pin = document(repo/RECIPE)
    require(set(value) == {'schema', 'corrections'}
            and value['schema'] == 'femacs-library-corrections-v1'
            and isinstance(value['corrections'], list) and len(value['corrections']) <= 8,
            'invalid bounded library correction recipe')
    seen, prepared = set(), []
    for row in value['corrections']:
        require(isinstance(row, dict) and set(row) == {'id', 'contract', 'source',
                'destination', 'feature', 'source_inputs', 'before', 'after', 'generated_sha256'}
                and isinstance(row['contract'], str) and row['contract'].strip()
                and isinstance(row['feature'], str)
                and re.fullmatch(r'[A-Za-z_][A-Za-z0-9_-]*', row['feature'])
                and row['id'] not in seen, 'invalid or duplicate library correction')
        seen.add(row['id'])
        source, destination = relative(row['source']), relative(row['destination'])
        require(source.parts[0] == 'lisp' and source.suffix == destination.suffix == '.el'
                and source.relative_to('lisp') == destination,
                'correction must preserve the original library location')
        pins = row['source_inputs']
        require(isinstance(pins, dict) and row['source'] in pins and 1 <= len(pins) <= 16,
                'library correction source and related inputs required')
        original = None
        for name, expected in pins.items():
            name_path = relative(name)
            require(name_path.parts[0] == 'lisp', 'library source pin escaped original library tree')
            contents, actual = read(repo/name_path)
            require(actual == expected,
                    'upstream library changed; revalidate correct behavior and retire or revise correction: '+name)
            if name == row['source']:
                original = contents
        require(isinstance(row['before'], str) and row['before']
                and isinstance(row['after'], str) and row['after'] != row['before']
                and original.count(row['before'].encode()) == 1,
                'library correction requires one exact original source range')
        generated = original.replace(row['before'].encode(), row['after'].encode())
        require(hashlib.sha256(generated).hexdigest() == row['generated_sha256'],
                'generated library differs from reviewed correction')
        prepared.append((row, generated))
    return pin, prepared


def generate(repo=ROOT):
    repo = physical(repo)
    pin, prepared = recipe(repo)
    root = physical(repo/OUTPUT)
    if root.exists():
        previous, _ = document(root/'receipt.json')
        require(previous.get('schema') == SCHEMA and isinstance(previous.get('corrections'), list)
                and len(previous['corrections']) <= 8, 'unowned or incomplete library correction output')
        active = {row['destination'] for row, _ in prepared}
        known = set()
        for row in previous['corrections']:
            destination = relative(row['destination'])
            require(destination.suffix == '.el', 'invalid prior generated library path')
            for suffix, key in (('.el', 'generated'), ('.elc', 'bytecode')):
                path = root/'lisp'/destination.with_suffix(suffix)
                known.add(path)
                if row['destination'] not in active and os.path.lexists(path):
                    require(row.get(key) is not None and read(path)[1] == row[key],
                            'retired correction output changed; cannot silently keep or remove it')
                    path.unlink()
        # An orphan derived library must never become a hidden load-path input.
        count = 0
        for base, directories, names in os.walk(root/'lisp', followlinks=False):
            for name in (*directories, *names):
                count += 1
                require(count <= 128, 'unexpected generated correction tree size')
                path = physical(Path(base)/name)
                require(not path.is_symlink() and (path.is_dir() or path in known),
                        'unknown generated library input; clean the owned build output explicitly')
    (root/'lisp').mkdir(parents=True, exist_ok=True)
    rows = []
    for row, raw in prepared:
        target = root/'lisp'/relative(row['destination'])
        write(target, raw, (repo/row['source']).stat().st_mode & 0o777)
        # No old compiled library may hide a changed source or satisfy a fresh build.
        physical(target.with_suffix('.elc')).unlink(missing_ok=True)
        rows.append(dict(id=row['id'], contract=row['contract'], source=row['source'],
            destination=row['destination'], feature=row['feature'], source_inputs=row['source_inputs'],
            generated=read(target)[1], bytecode=None))
    result = dict(schema=SCHEMA, recipe=pin, corrections=rows,
                  original_inputs_modified=False, behavior_accepted=False)
    write_json(root/'receipt.json', result)
    return [root/'lisp'/row['destination'] for row in rows]


def seal(repo=ROOT):
    repo = physical(repo)
    pin, prepared = recipe(repo)
    root = physical(repo/OUTPUT)
    value, _ = document(root/'receipt.json')
    require(value.get('schema') == SCHEMA and value.get('recipe') == pin
            and len(value.get('corrections', [])) == len(prepared),
            'generated library receipt does not match current recipe')
    for actual, (row, raw) in zip(value['corrections'], prepared):
        target = root/'lisp'/relative(row['destination'])
        require(actual['id'] == row['id'] and read(target)[0] == raw
                and actual['generated'] == read(target)[1], 'generated library changed before compilation seal')
        bytecode, bytecode_pin = read(target.with_suffix('.elc'))
        require(bool(bytecode), 'generated library bytecode is missing or empty')
        actual['bytecode'] = bytecode_pin
    write_json(root/'receipt.json', value)
    return value


def install(lisp_root, repo=ROOT):
    repo, lisp_root = physical(repo), physical(lisp_root)
    require(any(lisp_root.is_relative_to(repo/name) for name in ('build', 'dist'))
            and lisp_root.is_dir(), 'library corrections install only into owned product resources')
    pin, prepared = recipe(repo)
    root = physical(repo/OUTPUT)
    value, _ = document(root/'receipt.json')
    require(value.get('schema') == SCHEMA and value.get('recipe') == pin
            and len(value.get('corrections', [])) == len(prepared), 'missing sealed library correction build')
    for actual, (row, raw) in zip(value['corrections'], prepared):
        source = root/'lisp'/relative(row['destination'])
        destination = lisp_root/relative(row['destination'])
        require(actual['id'] == row['id'] and read(source)[0] == raw
                and read(source)[1] == actual['generated']
                and read(source.with_suffix('.elc'))[1] == actual['bytecode'],
                'sealed generated library or bytecode changed')
        require(read(destination)[1] == row['source_inputs'][row['source']],
                'product library correction requires a fresh original resource copy')
        for suffix in ('.el', '.elc'):
            incoming, target = source.with_suffix(suffix), destination.with_suffix(suffix)
            data, _ = read(incoming)
            write(target, data, incoming.stat().st_mode & 0o777)
    # This receipt is part of the product's existing whole-library tree identity.
    write_json(lisp_root/'.femacs-library-corrections.json', value)
    return value


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--action', choices=('generate', 'seal', 'install'), required=True)
    parser.add_argument('--lisp-root', type=Path)
    args = parser.parse_args()
    if (args.action == 'install') != (args.lisp_root is not None):
        parser.error('--lisp-root is required exactly for install')
    try:
        install(args.lisp_root) if args.action == 'install' else globals()[args.action]()
    except (ValueError, OSError) as error:
        parser.exit(1, str(error)+'\n')


if __name__ == '__main__':
    main()
