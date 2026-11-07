"""Per-run preservation of an owned macOS build driver, never a shared lock."""
from __future__ import annotations

from contextlib import contextmanager
import ctypes
import hashlib
import io
import json
import os
from pathlib import Path
import signal
import stat
import sys
import tarfile
import uuid


def digest(path):
    result = hashlib.sha256()
    with Path(path).open('rb') as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b''):
            result.update(block)
    return result.hexdigest()


def write_json(path, value):
    temporary = path.with_name(path.name + '.writing')
    with temporary.open('x') as output:
        json.dump(value, output, indent=2)
        output.write('\n')
    os.replace(temporary, path)


def snapshot(path, descriptor=None):
    current = path.lstat() if descriptor is None else os.fstat(descriptor)
    return dict(path=str(path), device=current.st_dev, inode=current.st_ino,
                birth=getattr(current, 'st_birthtime', None), flags=current.st_flags,
                kind=stat.S_IFMT(current.st_mode))


def same_inode(row):
    path = Path(row['path'])
    current = path.lstat()
    return (not path.is_symlink() and current.st_dev == row['device']
            and current.st_ino == row['inode'] and stat.S_IFMT(current.st_mode) == row['kind']
            and getattr(current, 'st_birthtime', None) == row['birth'])


def descriptor_matches(descriptor, row):
    current = os.fstat(descriptor)
    return (current.st_dev == row['device'] and current.st_ino == row['inode']
            and stat.S_IFMT(current.st_mode) == row['kind']
            and getattr(current, 'st_birthtime', None) == row['birth'])


def descriptor_digest(descriptor):
    result, offset = hashlib.sha256(), 0
    while block := os.pread(descriptor, 1024 * 1024, offset):
        result.update(block)
        offset += len(block)
    return result.hexdigest()


def set_flags(descriptor, flags):
    # Python's macOS os module has chflags but not fchflags. Use the native
    # descriptor API so a concurrent pathname replacement is never modified.
    function = ctypes.CDLL(None, use_errno=True).fchflags
    function.argtypes = (ctypes.c_int, ctypes.c_uint32)
    function.restype = ctypes.c_int
    if function(descriptor, flags) != 0:
        error = ctypes.get_errno()
        raise OSError(error, os.strerror(error))


def retain_archive(owner, identity, out):
    archive = out / 'build-owner.tar.gz'
    with owner.open('rb') as original, archive.open('xb') as archived, tarfile.open(fileobj=archived, mode='w:gz', compresslevel=1) as target:
        item = tarfile.TarInfo('runtime-build-driver')
        item.size = owner.stat().st_size
        item.mode = owner.stat().st_mode & 0o777
        target.addfile(item, original)
    with tarfile.open(archive, 'r:gz') as source:
        members = source.getmembers()
        if len(members) != 1 or not members[0].isfile() or members[0].name != 'runtime-build-driver':
            raise ValueError('invalid retained owner archive')
        result = hashlib.sha256()
        with source.extractfile(members[0]) as stream:
            for block in iter(lambda: stream.read(1024 * 1024), b''):
                result.update(block)
        if result.hexdigest() != identity or digest(owner) != identity:
            raise ValueError('retained owner archive/input changed')
    write_json(out / 'build-owner-archive.json',
               dict(path=archive.name, sha256=digest(archive), executable_sha256=identity,
                    bytes=archive.stat().st_size, member='runtime-build-driver'))


@contextmanager
def protect(owner, identity, out, repository):
    """Protect only this fresh driver's file/directory; restore their own flags."""
    owner, out, repository = Path(owner), Path(out), Path(repository)
    expected_parent = repository / '.femacs-cache/runtime-build-owners'
    if (sys.platform != 'darwin' or not hasattr(os, 'chflags')
            or owner.parent.parent != expected_parent or owner.name != 'runtime-build-driver'):
        raise ValueError('owner preservation requires an owned macOS driver')
    for path in (owner, owner.parent):
        if path.is_symlink() or path.resolve() != path:
            raise ValueError('unsafe owner preservation path')
    if not owner.is_file() or digest(owner) != identity:
        raise ValueError('owner missing or changed before preservation')
    receipt = json.loads((owner.parent / 'receipt.json').read_text())
    if receipt.get('sha256') != identity or receipt.get('path') != owner.name:
        raise ValueError('owner preservation receipt mismatch')
    retain_archive(owner, identity, out)
    rows, descriptors = [], []
    try:
        for path in (owner, owner.parent):
            descriptor = os.open(path, os.O_RDONLY | os.O_NOFOLLOW
                                 | (os.O_DIRECTORY if path == owner.parent else 0))
            descriptors.append(descriptor)
            rows.append(snapshot(path, descriptor))
    except BaseException:
        for descriptor in descriptors:
            os.close(descriptor)
        raise
    record = dict(schema=1, executable_sha256=identity, paths=rows, state='prepared',
                  scope='only this run executable and its immediate private directory')
    journal = out / 'build-owner-preservation.json'
    held, handlers, critical, interruption = [], {}, True, None
    signals = {signal.SIGINT, signal.SIGTERM}
    original_mask = signal.pthread_sigmask(signal.SIG_BLOCK, signals)

    def interrupted(number, frame):
        nonlocal interruption
        if interruption is None:
            interruption = number
            if not critical:
                # A second signal cannot interrupt the finally release.
                signal.pthread_sigmask(signal.SIG_BLOCK, signals)
                raise SystemExit(128 + number)

    try:
        write_json(journal, record)
        for number in signals:
            handlers[number] = signal.getsignal(number)
            signal.signal(number, interrupted)
        for row, descriptor in zip(rows, descriptors):
            if not descriptor_matches(descriptor, row) or not same_inode(row):
                raise ValueError('owner identity changed before preservation')
            held.append((row, descriptor))
            set_flags(descriptor, row['flags'] | stat.UF_IMMUTABLE)
        if descriptor_digest(descriptors[0]) != identity or not all(same_inode(row) for row in rows):
            raise ValueError('owner changed during preservation')
        record['state'] = 'active'
        write_json(journal, record)
        critical = False
        signal.pthread_sigmask(signal.SIG_SETMASK, original_mask)
        try:
            yield
        finally:
            critical = True
    finally:
        critical = True
        signal.pthread_sigmask(signal.SIG_BLOCK, signals)
        failures = []
        for row, descriptor in reversed(held):
            try:
                if not descriptor_matches(descriptor, row):
                    raise ValueError('refuse to alter replaced owner descriptor')
                if stat.S_ISREG(row['kind']) and descriptor_digest(descriptor) != identity:
                    raise ValueError('refuse to alter changed owner executable')
                set_flags(descriptor, row['flags'])
                if os.fstat(descriptor).st_flags != row['flags']:
                    raise ValueError('owner flags not restored')
                if not same_inode(row):
                    raise ValueError('owned pathname replaced; original descriptor flags restored')
            except (OSError, ValueError) as error:
                failures.append(dict(path=row['path'], error=str(error)))
        record.update(state='released' if not failures else 'release-failed', failures=failures)
        try:
            write_json(journal, record)
        finally:
            for descriptor in descriptors:
                os.close(descriptor)
            # Deliver queued signals to the deferring handler only after flags
            # and the durable journal have been restored, then restore callers.
            signal.pthread_sigmask(signal.SIG_SETMASK, original_mask)
            for number, handler in handlers.items():
                signal.signal(number, handler)
        if failures:
            raise ValueError('owned build driver preservation release failed; see its journal')
        if interruption is not None:
            raise SystemExit(128 + interruption)


ARTIFACT_ENV = 'FEMACS_RUNTIME_ARTIFACT_JOURNAL'


def artifact_write(path, value):
    """Commit capture intent before flags change, including child interruption."""
    # Signal unwind may write again in this process while an interrupted
    # partial write is retained. Never let that evidence block release.
    temporary = path.with_name(path.name + '.writing.' + str(os.getpid()) + '.' + uuid.uuid4().hex)
    with temporary.open('x') as stream:
        json.dump(value, stream, separators=(',', ':'))
        stream.write('\n')
        stream.flush()
        os.fsync(stream.fileno())
    os.replace(temporary, path)


def artifact_read(path, *, require_owner=True):
    path = Path(path)
    if path.stat().st_size > 128 * 1024 * 1024:
        raise ValueError('oversized owned artifact journal')
    value = json.loads(path.read_text())
    repository = Path(value['repository'])
    output = Path(value['output'])
    if (value.get('schema') != 'femacs-runtime-artifacts-v1'
            or path != output / 'artifact-preservation.json'
            or not output.is_relative_to(repository / 'build')
            or output == repository / 'build'):
        raise ValueError('artifact journal is not an owned run')
    for item in (repository, output, path):
        if item.is_symlink() or item.resolve() != item:
            raise ValueError('unsafe artifact journal locator')
    if require_owner:
        if (value['state'] != 'active'
                or artifact_process(value['owner_pid'])[1] != value['owner_birth']):
            raise ValueError('artifact journal owner is not active')
        current = os.getpid()
        for _ in range(64):
            if current == value['owner_pid']:
                break
            current, _ = artifact_process(current)
            if current <= 1:
                break
        if current != value['owner_pid']:
            raise ValueError('artifact capture is outside its owning process tree')
    return value


def artifact_process(pid):
    """Read only this process/ancestor; no shared process inventory or quota."""
    import subprocess
    result = subprocess.run(['ps', '-p', str(pid), '-o', 'ppid=,lstart='],
                            check=False, stdout=subprocess.PIPE, text=True)
    if result.returncode != 0 or not result.stdout.strip():
        return 0, None
    parent, birth = result.stdout.strip().split(None, 1)
    return int(parent), birth


def artifact_path(path, value):
    path = Path(path)
    roots = [Path(p) for p in value['allowed_roots']]
    if not any(path.is_relative_to(root) for root in roots):
        raise ValueError('artifact is outside the selected run inputs')
    if path.parent.resolve() != path.parent:
        raise ValueError('artifact parent has a symlink or changed locator')
    return path


def capture_artifacts(journal, expected):
    """Capture regular files; retain explicit symlink identities without following."""
    journal = Path(journal)
    value = artifact_read(journal)
    known = {row['path']: row for row in value['files']}
    links = {row['path']: row for row in value['symlinks']}
    pending = []
    for name, identity in sorted(expected.items()):
        path = artifact_path(name, value)
        if identity.get('kind') == 'symlink':
            current = path.lstat()
            if (not stat.S_ISLNK(current.st_mode) or os.readlink(path) != identity['target']
                    or stat.S_IMODE(current.st_mode) != identity['mode']):
                raise ValueError('selected artifact symlink changed')
            previous = links.get(name)
            row = dict(path=name, identity=identity, snapshot=snapshot(path))
            if previous:
                verify_artifact_link(previous)
                if previous['identity'] != identity:
                    raise ValueError('previously captured artifact symlink changed')
            links[name] = previous or row
            continue
        descriptor = os.open(path, os.O_RDONLY | os.O_NOFOLLOW)
        try:
            current = os.fstat(descriptor)
            if (not stat.S_ISREG(current.st_mode)
                    or descriptor_digest(descriptor) != identity['sha256']
                    or ('bytes' in identity and current.st_size != identity['bytes'])
                    or ('mode' in identity and stat.S_IMODE(current.st_mode) != identity['mode'])):
                raise ValueError('selected artifact bytes or mode changed')
            row = snapshot(path, descriptor)
            if not same_inode(row):
                raise ValueError('selected artifact pathname changed')
            row.update(sha256=identity['sha256'], bytes=current.st_size,
                       mode=stat.S_IMODE(current.st_mode))
            previous = known.get(name)
            if previous:
                if (not descriptor_matches(descriptor, previous)
                        or previous['sha256'] != row['sha256']
                        or previous['mode'] != row['mode']
                        or not current.st_flags & stat.UF_IMMUTABLE):
                    raise ValueError('previously captured artifact changed')
                continue
            if current.st_flags & stat.UF_IMMUTABLE:
                raise ValueError('refuse to adopt a preexisting immutable artifact')
            pending.append(row)
        finally:
            os.close(descriptor)
    value['files'].extend(pending)
    value['symlinks'] = list(links.values())
    artifact_write(journal, value)
    # Intent covers the whole set before any flag mutation. A killed child can
    # leave a partially acquired set; the parent releases every recorded inode.
    for row in pending:
        descriptor = os.open(row['path'], os.O_RDONLY | os.O_NOFOLLOW)
        try:
            if (not descriptor_matches(descriptor, row) or not same_inode(row)
                    or descriptor_digest(descriptor) != row['sha256']):
                raise ValueError('artifact changed during preservation capture')
            set_flags(descriptor, os.fstat(descriptor).st_flags | stat.UF_IMMUTABLE)
            if not os.fstat(descriptor).st_flags & stat.UF_IMMUTABLE:
                raise ValueError('artifact preservation flag did not take effect')
        finally:
            os.close(descriptor)


def _capture_product_impl(journal, receipt_path, source, *, require_link_inputs):
    import importlib.util
    journal, receipt_path, source = Path(journal), Path(receipt_path), Path(source)
    value = artifact_read(journal)
    if source not in [Path(p) for p in value['allowed_roots']]:
        raise ValueError('product source was not selected by the owner')
    if not receipt_path.is_relative_to(source / 'build/runtime'):
        raise ValueError('product receipt is outside the selected source build')
    spec = importlib.util.spec_from_file_location('femacs_artifact_execution', source / 'tools/runtime-execution.py')
    execution = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = execution
    spec.loader.exec_module(execution)
    receipt = execution.bounded_json(receipt_path, 64 * 1024 * 1024)
    root = receipt_path.parent
    binding = receipt.get('binding')
    if (receipt.get('build_complete') is not True
            or not isinstance(binding, dict)
            or binding != execution.bounded_json(root / 'runtime-binding.json')
            or receipt.get('link_inputs', {}).get('link_complete') is not True):
        raise ValueError('cannot preserve an incomplete runtime product')
    if 'config_relative' in binding or 'config_digest' in binding:
        config_rel = Path(binding.get('config_relative', ''))
        if (not binding.get('config_relative') or config_rel.is_absolute() or '..' in config_rel.parts
                or execution.digest(execution.safe_path(root / config_rel)) != binding.get('config_digest')):
            raise ValueError('completed product configuration changed')
    _, _, resources, _ = execution.product_receipt_layout(receipt, root)
    repository_build = Path(value['repository']) / 'build'
    roots_updated = False
    for path in resources.values():
        path = execution.safe_path(path)
        if not path.is_relative_to(repository_build) or path == repository_build:
            raise ValueError('product resource is outside the owned build tree')
        if not any(path.is_relative_to(Path(allowed)) for allowed in value['allowed_roots']):
            value['allowed_roots'].append(str(path))
            roots_updated = True
    if roots_updated:
        artifact_write(journal, value)
    metadata = receipt.get('file_identities', {})
    if receipt.get('identity_version') not in (2, 3) or set(metadata) != set(receipt['files']):
        raise ValueError('new artifact capture requires complete bound file metadata')
    for name, identity in metadata.items():
        if (identity.get('sha256') != receipt['files'][name]
                or identity.get('kind') != 'file' or type(identity.get('mode')) is not int
                or type(identity.get('bytes')) is not int):
            raise ValueError('completed product file metadata does not match its receipt')
    expected = {str(root / name): identity for name, identity in metadata.items()}
    for name, path in resources.items():
        expected.update({str(path / relative): identity
                         for relative, identity in receipt['resources'][name].items()})
        # Unexpected additional members are not hidden by the recorded set.
        if execution.tree_identity(path) != receipt['resources'][name]:
            raise ValueError('completed product resource tree changed')
    for relative, identity in receipt['link_inputs']['objects'].items():
        rel_path = Path(relative)
        if rel_path.is_absolute() or '..' in rel_path.parts:
            raise ValueError('unsafe product link object path')
        target = root / rel_path
        if not require_link_inputs:
            if target.is_symlink():
                raise ValueError('product link input is a symlink')
            execution.safe_path(target.parent)
            if not target.exists():
                continue
        if type(identity.get('mode')) is not int or type(identity.get('bytes')) is not int:
            raise ValueError('new artifact capture requires link object permission identity')
        expected[str(target)] = dict(kind='file', sha256=identity['sha256'],
                                     bytes=identity['bytes'], mode=identity['mode'])
    archive = receipt['link_inputs']['archive']
    archive_rel = Path(archive.get('path', ''))
    if not archive.get('path') or archive_rel.is_absolute() or '..' in archive_rel.parts:
        raise ValueError('unsafe product link archive path')
    archive_target = root / archive_rel
    include_archive = True
    if not require_link_inputs:
        if archive_target.is_symlink():
            raise ValueError('product link archive is a symlink')
        execution.safe_path(archive_target.parent)
        include_archive = archive_target.exists()
    if include_archive:
        if type(archive.get('mode')) is not int or type(archive.get('bytes')) is not int:
            raise ValueError('new artifact capture requires archive permission identity')
        expected[str(archive_target)] = dict(kind='file', sha256=archive['sha256'],
                                             bytes=archive['bytes'], mode=archive['mode'])
    metadata_names = ['runtime-product.json', 'runtime-binding.json', 'runtime-config.json',
                      'runtime-inputs.json', 'runtime-link-inputs.json']
    if 'native' in binding.get('variant', ''):
        backend_sources = root / 'backend-sources.json'
        if not require_link_inputs or backend_sources.exists() or backend_sources.is_symlink():
            metadata_names.append('backend-sources.json')
    for relative in metadata_names:
        path = execution.safe_path(root / relative)
        expected[str(path)] = execution.file_identity(path)
    capture_artifacts(journal, expected)


def capture_product(journal, receipt_path, source):
    """The serial build driver calls this at each completed compiler leg."""
    _capture_product_impl(journal, receipt_path, source, require_link_inputs=True)


def capture_product_files(journal, receipt_path, source):
    """Execution stages preserve complete finished products and surviving link inputs."""
    _capture_product_impl(journal, receipt_path, source, require_link_inputs=False)


def verify_artifact_link(row):
    path, identity = Path(row['path']), row['identity']
    current = path.lstat()
    original = row['snapshot']
    if (not stat.S_ISLNK(current.st_mode)
            or current.st_dev != original['device'] or current.st_ino != original['inode']
            or stat.S_IFMT(current.st_mode) != original['kind']
            or getattr(current, 'st_birthtime', None) != original['birth']
            or os.readlink(path) != identity['target']
            or stat.S_IMODE(current.st_mode) != identity['mode']):
        raise ValueError('captured artifact symlink changed')


class ArtifactRun:
    """One finite stage's preservation; never a shared quota, lock or daemon."""
    def __init__(self, repository, output):
        self.repository, self.output = Path(repository), Path(output)
        if (sys.platform != 'darwin' or not self.output.is_relative_to(self.repository / 'build')
                or self.output == self.repository / 'build'):
            raise ValueError('runtime artifact preservation requires an owned macOS stage')
        self.journal = self.output / 'artifact-preservation.json'
        if self.journal.exists():
            raise ValueError('inspect existing artifact preservation before reuse')
        self.value = dict(schema='femacs-runtime-artifacts-v1', state='active',
                          repository=str(self.repository), output=str(self.output),
                          owner_pid=os.getpid(), owner_birth=artifact_process(os.getpid())[1],
                          owner_birth_precision='native ps lstart, one second; capture also requires current ancestry',
                          allowed_roots=[str(self.output)], files=[], symlinks=[])
        if self.value['owner_birth'] is None:
            raise ValueError('cannot establish artifact owner process identity')

    def __enter__(self):
        artifact_write(self.journal, self.value)
        self.handlers = {number: signal.getsignal(number) for number in (signal.SIGINT, signal.SIGTERM)}
        def interrupted(number, frame):
            raise SystemExit(128 + number)
        for number in self.handlers:
            signal.signal(number, interrupted)
        return self

    def select_source(self, original, source_export):
        source = Path(source_export['source'])
        if (source != Path(original) / 'source'
                or not source.is_relative_to(self.repository / 'build') or source.resolve() != source):
            raise ValueError('artifact source is not an owned frozen export')
        value = artifact_read(self.journal)
        if str(source) not in value['allowed_roots']:
            value['allowed_roots'].append(str(source))
        # Exact export metadata sit outside source; authorize only these files.
        names = [Path(original) / 'source-inputs.json', source / '.femacs-runtime-sources.json']
        value['allowed_roots'].append(str(Path(original) / 'source-inputs.json'))
        artifact_write(self.journal, value)
        manifest = json.loads(names[0].read_text())
        expected = {str(source / name): identity for name, identity in manifest['files'].items()}
        for path in names:
            expected[str(path)] = dict(kind='file', sha256=digest(path))
        capture_artifacts(self.journal, expected)

    def retain_current_files(self):
        """Verify physical recovery dependencies without a duplicate content archive."""
        value = artifact_read(self.journal)
        for row in value['files']:
            descriptor = os.open(row['path'], os.O_RDONLY | os.O_NOFOLLOW)
            try:
                current = os.fstat(descriptor)
                if (not descriptor_matches(descriptor, row) or not same_inode(row)
                        or stat.S_IMODE(current.st_mode) != row['mode']
                        or current.st_size != row['bytes']
                        or descriptor_digest(descriptor) != row['sha256']):
                    raise ValueError('selected physical recovery file changed')
            finally:
                os.close(descriptor)
        for row in value['symlinks']:
            verify_artifact_link(row)
        retention = dict(mode='verified-files', files=len(value['files']),
                         symlinks=len(value['symlinks']), all_original_files_verified=True,
                         duplicate_content_archive_created=False,
                         recovery='retain physical originals or transfer exact recovery before retirement')
        value['retention'] = retention
        artifact_write(self.journal, value)
        return retention

    def archive(self):
        value = artifact_read(self.journal)
        unique = {}
        for row in value['files']:
            descriptor = os.open(row['path'], os.O_RDONLY | os.O_NOFOLLOW)
            try:
                current = os.fstat(descriptor)
                if (not descriptor_matches(descriptor, row) or not same_inode(row)
                        or stat.S_IMODE(current.st_mode) != row['mode']
                        or current.st_size != row['bytes']
                        or descriptor_digest(descriptor) != row['sha256']):
                    raise ValueError('selected artifact content or mode changed before retention')
            finally:
                os.close(descriptor)
            unique.setdefault(row['sha256'], row)
        for row in value['symlinks']:
            verify_artifact_link(row)
        links = json.dumps(value['symlinks'], sort_keys=True, separators=(',', ':')).encode()
        links_hash = hashlib.sha256(links).hexdigest()
        archive = self.output / 'artifact-content.tar.gz'
        with archive.open('xb') as output, tarfile.open(fileobj=output, mode='w:gz', compresslevel=1) as target:
            for identity, row in sorted(unique.items()):
                descriptor = os.open(row['path'], os.O_RDONLY | os.O_NOFOLLOW)
                with os.fdopen(descriptor, 'rb') as original:
                    if (not descriptor_matches(original.fileno(), row) or not same_inode(row)
                            or descriptor_digest(original.fileno()) != identity
                            or stat.S_IMODE(os.fstat(original.fileno()).st_mode) != row['mode']):
                        raise ValueError('artifact changed before exact retention')
                    member = tarfile.TarInfo(identity)
                    member.size, member.mode, member.mtime = row['bytes'], 0o600, 0
                    target.addfile(member, original)
            member = tarfile.TarInfo('symlinks.json')
            member.size, member.mode, member.mtime = len(links), 0o600, 0
            target.addfile(member, io.BytesIO(links))
        seen = set()
        with tarfile.open(archive, 'r:gz') as retained:
            for member in retained:
                if (not member.isfile() or member.name not in {*unique, 'symlinks.json'}
                        or member.name in seen):
                    raise ValueError('unexpected artifact archive member')
                result, size = hashlib.sha256(), 0
                with retained.extractfile(member) as stream:
                    for block in iter(lambda: stream.read(1024 * 1024), b''):
                        result.update(block)
                        size += len(block)
                expected_hash = links_hash if member.name == 'symlinks.json' else member.name
                expected_size = len(links) if member.name == 'symlinks.json' else unique[member.name]['bytes']
                if result.hexdigest() != expected_hash or size != expected_size:
                    raise ValueError('artifact recovery archive content changed')
                seen.add(member.name)
        if seen != {*unique, 'symlinks.json'}:
            raise ValueError('artifact recovery archive is incomplete')
        for row in value['symlinks']:
            verify_artifact_link(row)
        value['archive'] = dict(path=archive.name, sha256=digest(archive), bytes=archive.stat().st_size,
                                unique_contents=len(unique), all_members_verified=True,
                                symlink_manifest=dict(member='symlinks.json', sha256=links_hash,
                                                      bytes=len(links), entries=len(value['symlinks'])),
                                modes_and_paths='artifact-preservation.json')
        artifact_write(self.journal, value)

    def __exit__(self, kind, error, traceback):
        signals = {signal.SIGINT, signal.SIGTERM}
        mask = signal.pthread_sigmask(signal.SIG_BLOCK, signals)
        failures = []
        try:
            failures = release_artifacts(self.journal, require_dead=False)
        finally:
            for number, handler in self.handlers.items():
                signal.signal(number, handler)
            signal.pthread_sigmask(signal.SIG_SETMASK, mask)
        if failures:
            raise ValueError('owned artifact preservation release failed; see its journal')


def release_artifacts(journal, *, require_dead):
    journal = Path(journal)
    value = artifact_read(journal, require_owner=False)
    birth = artifact_process(value['owner_pid'])[1]
    if require_dead:
        if birth == value['owner_birth']:
            raise ValueError('refuse to release another live artifact owner')
    elif os.getpid() != value['owner_pid'] or birth != value['owner_birth']:
        raise ValueError('artifact release is not its original owner')
    if value['state'] == 'released':
        return []
    failures = []
    for row in reversed(value['files']):
        try:
            path = artifact_path(row['path'], value)
            descriptor = os.open(path, os.O_RDONLY | os.O_NOFOLLOW)
            try:
                if not descriptor_matches(descriptor, row) or not same_inode(row):
                    raise ValueError('refuse to release a replaced artifact')
                if stat.S_IMODE(os.fstat(descriptor).st_mode) != row['mode']:
                    failures.append(dict(path=row['path'], error='captured artifact mode changed; only owned flags released'))
                current = os.fstat(descriptor).st_flags
                target = (current & ~stat.UF_IMMUTABLE) | (row['flags'] & stat.UF_IMMUTABLE)
                set_flags(descriptor, target)
            finally:
                os.close(descriptor)
        except (OSError, ValueError) as failure:
            failures.append(dict(path=row['path'], error=str(failure)))
    for row in value['symlinks']:
        try:
            artifact_path(row['path'], value)
            verify_artifact_link(row)
        except (OSError, ValueError) as failure:
            failures.append(dict(path=row['path'], error=str(failure)))
    value.update(state='release-failed' if failures else 'released', release_failures=failures)
    artifact_write(journal, value)
    return failures


if __name__ == '__main__':
    import argparse
    parser = argparse.ArgumentParser(description='Capture one completed owned runtime compiler leg.')
    parser.add_argument('--journal', type=Path)
    parser.add_argument('--release-journal', type=Path)
    parser.add_argument('--product-receipt', type=Path)
    parser.add_argument('--source', type=Path)
    parser.add_argument('--files-only', action='store_true')
    options = parser.parse_args()
    if options.release_journal is not None:
        if (options.journal is not None or options.product_receipt is not None
                or options.source is not None or options.files_only):
            parser.error('release accepts only its exact journal')
        failures = release_artifacts(options.release_journal, require_dead=True)
        if failures:
            raise SystemExit('artifact release failed; original journal retained')
    else:
        if options.journal is None or options.product_receipt is None or options.source is None:
            parser.error('capture requires journal, product receipt and frozen source')
        if options.files_only:
            capture_product_files(options.journal, options.product_receipt, options.source)
        else:
            capture_product(options.journal, options.product_receipt, options.source)
