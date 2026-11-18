"""Retire consumed, reaped case state with a durable pre-delete identity ledger.

The caller retains original inputs, outputs, failures and product receipts.
Only named children of its fresh owned case directory may be selected. Raw
machine paths remain private; public ledgers use relative identities.
"""
from pathlib import Path
from contextlib import contextmanager
import errno
import gzip
import hashlib
import json
import math
import os
import selectors
import signal
import stat
import subprocess
import time
import threading

SNAPSHOT_SCHEMA = 'femacs-owned-state-snapshot-v1'
DEFAULT_OCCUPANCY_TIMEOUT = 10.0
DEFAULT_MAX_OUTPUT_BYTES = 65536
DEFAULT_MAX_ENTRIES = 100000
DEFAULT_MAX_TOTAL_BYTES = 1073741824
MAX_HOLDER_COMMANDS = 16
STREAM_CHUNK_BYTES = 262144

_DIR_OPEN_FLAGS = (
    os.O_RDONLY
    | getattr(os, 'O_NOFOLLOW', 0)
    | getattr(os, 'O_DIRECTORY', 0)
)
_FILE_OPEN_FLAGS = (
    os.O_RDONLY
    | getattr(os, 'O_NOFOLLOW', 0)
    | getattr(os, 'O_NONBLOCK', 0)
)


@contextmanager
def retirement_deadline(seconds):
    """Bound a caller's final sweep without interrupting its receipt sealing.

    Only the main Python owner thread may temporarily own an otherwise unused
    real-time alarm. A timeout preserves the pre-delete ledger and propagates;
    callers must retain that case rather than retrying a partial operation.
    """
    if (not hasattr(signal, 'ITIMER_REAL') or threading.current_thread() is not threading.main_thread()
            or not math.isfinite(seconds) or seconds <= 0
            or signal.getitimer(signal.ITIMER_REAL) != (0.0, 0.0)):
        raise ValueError('an exclusive finite retirement deadline is required')
    original = signal.getsignal(signal.SIGALRM)
    def expired(signum, frame):
        raise TimeoutError('final retirement sweep deadline expired')
    signal.signal(signal.SIGALRM, expired)
    signal.setitimer(signal.ITIMER_REAL, seconds)
    try:
        yield
    finally:
        signal.setitimer(signal.ITIMER_REAL, 0)
        signal.signal(signal.SIGALRM, original)


def digest(path):
    value = hashlib.sha256()
    with Path(path).open('rb') as stream:
        for block in iter(lambda: stream.read(STREAM_CHUNK_BYTES), b''):
            value.update(block)
    return value.hexdigest()


def _validate_names(names):
    if isinstance(names, (str, bytes)):
        raise ValueError('unsafe owned-state retirement scope')
    normalized = []
    for name in names:
        if (not isinstance(name, str) or Path(name).name != name
                or name in ('', '.', '..') or '/' in name or '\\' in name):
            raise ValueError('unsafe owned-state retirement scope')
        normalized.append(name)
    return sorted(set(normalized))


def _validate_positive_int(value, label):
    if isinstance(value, bool) or not isinstance(value, int) or value <= 0:
        raise ValueError(f'invalid {label}')
    return value


def _extract_node_pair(original):
    if isinstance(original, dict):
        dev = original.get('device')
        ino = original.get('inode')
    else:
        dev = getattr(original, 'st_dev', None)
        ino = getattr(original, 'st_ino', None)
    if (isinstance(dev, bool) or not isinstance(dev, int)
            or isinstance(ino, bool) or not isinstance(ino, int)):
        raise ValueError('invalid original parent node identity')
    return dev, ino


def _open_verified_dir(path, expected_dev_ino=None, uid=None, error_msg='owned directory changed'):
    try:
        fd = os.open(path, _DIR_OPEN_FLAGS)
    except OSError as exc:
        raise ValueError(error_msg) from exc
    try:
        st = os.fstat(fd)
        if not stat.S_ISDIR(st.st_mode) or getattr(st, 'st_flags', 0):
            raise ValueError(error_msg)
        if uid is not None and st.st_uid != uid:
            raise ValueError(error_msg)
        if expected_dev_ino is not None and (st.st_dev, st.st_ino) != expected_dev_ino:
            raise ValueError(error_msg)
        return fd, st
    except BaseException:
        os.close(fd)
        raise


def _hash_regular_via_dirfd(dir_fd, leaf_name, expected_lstat, uid, remaining_budget,
                           change_msg='owned regular file changed during snapshot'):
    try:
        fd = os.open(leaf_name, _FILE_OPEN_FLAGS, dir_fd=dir_fd)
    except OSError as exc:
        raise ValueError(change_msg) from exc
    try:
        st_before = os.fstat(fd)
        if (not stat.S_ISREG(st_before.st_mode)
                or st_before.st_uid != uid
                or getattr(st_before, 'st_flags', 0)
                or (st_before.st_dev, st_before.st_ino, st_before.st_mode,
                    st_before.st_size, st_before.st_mtime_ns) !=
                (expected_lstat.st_dev, expected_lstat.st_ino, expected_lstat.st_mode,
                 expected_lstat.st_size, expected_lstat.st_mtime_ns)):
            raise ValueError(change_msg)
        if remaining_budget is not None and st_before.st_size > remaining_budget:
            raise ValueError('owned state exceeds max_total_bytes limit')
        hasher = hashlib.sha256()
        read_bytes = 0
        while True:
            chunk = os.read(fd, STREAM_CHUNK_BYTES)
            if not chunk:
                break
            read_bytes += len(chunk)
            if remaining_budget is not None and read_bytes > remaining_budget:
                raise ValueError('owned state exceeds max_total_bytes limit')
            hasher.update(chunk)
        st_after = os.fstat(fd)
        if (read_bytes != st_before.st_size
                or (st_after.st_dev, st_after.st_ino, st_after.st_mode,
                    st_after.st_uid, getattr(st_after, 'st_flags', 0),
                    st_after.st_size, st_after.st_mtime_ns) !=
                (st_before.st_dev, st_before.st_ino, st_before.st_mode,
                 uid, 0, st_before.st_size, st_before.st_mtime_ns)):
            raise ValueError(change_msg)
    finally:
        os.close(fd)
    try:
        recheck = os.stat(leaf_name, dir_fd=dir_fd, follow_symlinks=False)
    except OSError as exc:
        raise ValueError(change_msg) from exc
    if ((recheck.st_dev, recheck.st_ino, recheck.st_mode, recheck.st_uid,
         getattr(recheck, 'st_flags', 0),
         recheck.st_size, recheck.st_mtime_ns) !=
            (st_before.st_dev, st_before.st_ino, st_before.st_mode,
             uid, 0, st_before.st_size, st_before.st_mtime_ns)):
        raise ValueError(change_msg)
    return st_before, hasher.hexdigest()


def _read_symlink_via_dirfd(dir_fd, leaf_name, expected_lstat, uid, remaining_budget):
    name_bytes = os.fsencode(leaf_name)
    try:
        raw_target = os.readlink(name_bytes, dir_fd=dir_fd)
        recheck = os.stat(leaf_name, dir_fd=dir_fd, follow_symlinks=False)
        raw_target_after = os.readlink(name_bytes, dir_fd=dir_fd)
    except OSError as exc:
        raise ValueError('owned symlink changed during snapshot') from exc
    if (not stat.S_ISLNK(recheck.st_mode)
            or recheck.st_uid != uid
            or getattr(recheck, 'st_flags', 0)
            or raw_target_after != raw_target
            or (recheck.st_dev, recheck.st_ino, recheck.st_mode,
                recheck.st_size, recheck.st_mtime_ns) !=
            (expected_lstat.st_dev, expected_lstat.st_ino, expected_lstat.st_mode,
             expected_lstat.st_size, expected_lstat.st_mtime_ns)):
        raise ValueError('owned symlink changed during snapshot')
    if remaining_budget is not None and len(raw_target) > remaining_budget:
        raise ValueError('owned state exceeds max_total_bytes limit')
    try:
        link_target = raw_target.decode('utf-8')
    except UnicodeDecodeError as exc:
        raise ValueError('non-utf8 symlink target in owned state') from exc
    return len(raw_target), link_target, hashlib.sha256(raw_target).hexdigest()


def _terminate_own_query(proc):
    """Terminate and reap only the lsof subprocess started by check_occupancy."""
    if proc.poll() is None:
        try:
            os.killpg(proc.pid, signal.SIGTERM)
        except ProcessLookupError:
            pass
        except OSError:
            try:
                proc.terminate()
            except OSError:
                pass
        try:
            proc.wait(timeout=0.5)
        except subprocess.TimeoutExpired:
            try:
                os.killpg(proc.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            except OSError:
                try:
                    proc.kill()
                except OSError:
                    pass
    try:
        proc.wait()
    except OSError:
        pass


def _sanitize_command_token(raw_token):
    unescaped = raw_token.replace('\\x20', '_')
    base = unescaped.rsplit('/', 1)[-1]
    cleaned = ''.join(ch for ch in base if '!' <= ch <= '~' and ch != '\\')
    return cleaned[:64]


def _parse_lsof_holders(stdout_sample, truncated):
    if not stdout_sample:
        return [], 0
    raw = bytes(stdout_sample)
    lines = raw.split(b'\n')
    if truncated and not raw.endswith(b'\n'):
        lines = lines[:-1]
    commands = []
    seen_commands = set()
    pids = set()
    for index, raw_line in enumerate(lines):
        if not raw_line:
            continue
        text = raw_line.decode('utf-8', errors='replace')
        parts = text.split()
        if len(parts) < 2:
            continue
        if index == 0 and parts[0] == 'COMMAND' and parts[1] == 'PID':
            continue
        if not parts[1].isdigit():
            continue
        pids.add(int(parts[1]))
        cmd = _sanitize_command_token(parts[0])
        if cmd and cmd not in seen_commands and len(commands) < MAX_HOLDER_COMMANDS:
            seen_commands.add(cmd)
            commands.append(cmd)
    return commands, len(pids)


def check_occupancy(root, *, timeout=DEFAULT_OCCUPANCY_TIMEOUT,
                    max_output_bytes=DEFAULT_MAX_OUTPUT_BYTES):
    """Query lsof +D under strict time and output bounds without raw output leaks."""
    if isinstance(timeout, bool) or not isinstance(timeout, (int, float)):
        raise ValueError('invalid occupancy timeout')
    timeout = float(timeout)
    if not math.isfinite(timeout) or timeout <= 0.0:
        raise ValueError('invalid occupancy timeout')
    max_output_bytes = _validate_positive_int(max_output_bytes, 'max_output_bytes')
    target = Path(root).absolute()
    if target.is_symlink() or not target.is_dir() or target.resolve() != target:
        raise ValueError('occupancy target must be a physical directory')

    started = time.monotonic()
    deadline = started + timeout
    stdout_sample = bytearray()
    stderr_sample = bytearray()
    stdout_bytes = 0
    stderr_bytes = 0
    stdout_lines = 0
    stderr_lines = 0
    timed_out = False
    output_truncated = False
    read_error = None

    try:
        proc = subprocess.Popen(
            ['lsof', '+D', str(target)],
            stdin=subprocess.DEVNULL,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            start_new_session=True,
        )
    except OSError as exc:
        return dict(
            clear=False,
            timed_out=False,
            returncode=None,
            stdout_bytes=0,
            stderr_bytes=0,
            stdout_lines=0,
            stderr_lines=0,
            output_truncated=False,
            duration_seconds=time.monotonic() - started,
            holder_commands=[],
            holder_pid_count=0,
            error_type=type(exc).__name__,
        )

    selector = selectors.DefaultSelector()
    try:
        for pipe, label in ((proc.stdout, 'stdout'), (proc.stderr, 'stderr')):
            os.set_blocking(pipe.fileno(), False)
            selector.register(pipe, selectors.EVENT_READ, label)
        while selector.get_map():
            remaining = deadline - time.monotonic()
            if remaining <= 0.0:
                timed_out = True
                _terminate_own_query(proc)
                break
            events = selector.select(timeout=min(remaining, 0.25))
            if not events:
                if deadline - time.monotonic() <= 0.0:
                    timed_out = True
                    _terminate_own_query(proc)
                    break
                continue
            for key, _ in events:
                try:
                    chunk = os.read(key.fd, 16384)
                except BlockingIOError:
                    continue
                except OSError as exc:
                    if exc.errno == errno.EINTR:
                        continue
                    # An unread pipe is unknown occupancy, never empty output.
                    read_error = type(exc).__name__
                    _terminate_own_query(proc)
                    break
                if not chunk:
                    selector.unregister(key.fileobj)
                    continue
                if key.data == 'stdout':
                    stdout_bytes += len(chunk)
                    stdout_lines += chunk.count(b'\n')
                    if len(stdout_sample) < max_output_bytes:
                        keep = max_output_bytes - len(stdout_sample)
                        stdout_sample.extend(chunk[:keep])
                    if stdout_bytes > max_output_bytes:
                        output_truncated = True
                        _terminate_own_query(proc)
                        break
                else:
                    stderr_bytes += len(chunk)
                    stderr_lines += chunk.count(b'\n')
                    if len(stderr_sample) < max_output_bytes:
                        keep = max_output_bytes - len(stderr_sample)
                        stderr_sample.extend(chunk[:keep])
                    if stderr_bytes > max_output_bytes:
                        output_truncated = True
                        _terminate_own_query(proc)
                        break
            if timed_out or output_truncated or read_error is not None:
                break
        if not timed_out and not output_truncated and read_error is None:
            remaining = max(0.0, deadline - time.monotonic())
            try:
                proc.wait(timeout=remaining)
            except subprocess.TimeoutExpired:
                timed_out = True
                _terminate_own_query(proc)
    finally:
        selector.close()
        if proc.stdout is not None:
            proc.stdout.close()
        if proc.stderr is not None:
            proc.stderr.close()
        if proc.poll() is None:
            _terminate_own_query(proc)

    elapsed = time.monotonic() - started
    returncode = proc.returncode
    holder_commands, holder_pid_count = _parse_lsof_holders(stdout_sample, output_truncated)
    clear = (
        not timed_out
        and not output_truncated
        and read_error is None
        and returncode == 1
        and stdout_bytes == 0
        and stderr_bytes == 0
    )
    return dict(
        clear=clear,
        timed_out=timed_out,
        returncode=returncode,
        stdout_bytes=stdout_bytes,
        stderr_bytes=stderr_bytes,
        stdout_lines=stdout_lines,
        stderr_lines=stderr_lines,
        output_truncated=output_truncated,
        duration_seconds=elapsed,
        holder_commands=holder_commands,
        holder_pid_count=holder_pid_count,
        error_type=read_error,
    )


def _compute_snapshot_digest(payload):
    canonical = json.dumps(payload, sort_keys=True, separators=(',', ':')).encode('utf-8')
    return hashlib.sha256(canonical).hexdigest()


def _capture_owned_state_once(root, selected_names, *, empty_parents=(),
                              allow_symlinks=False, max_entries=DEFAULT_MAX_ENTRIES,
                              max_total_bytes=DEFAULT_MAX_TOTAL_BYTES):
    uid = os.getuid()
    if root.is_symlink() or not root.is_dir() or root.resolve() != root:
        raise ValueError('unsafe owned-state retirement scope')
    nodes = []
    entry_count = 0
    total_bytes = 0
    file_count = 0
    dir_count = 0
    symlink_count = 0

    root_fd, root_st = _open_verified_dir(
        root, uid=uid, error_msg='unsafe owned-state retirement scope')
    try:
        root_children = set(os.listdir(root_fd))
        selected_present = []
        top_dirs = []
        for name in selected_names:
            if name not in root_children:
                try:
                    os.stat(name, dir_fd=root_fd, follow_symlinks=False)
                except FileNotFoundError:
                    continue
                raise ValueError('owned root changed during snapshot')
            try:
                child_st = os.stat(name, dir_fd=root_fd, follow_symlinks=False)
            except OSError as exc:
                raise ValueError('owned entry disappeared during snapshot') from exc
            if child_st.st_uid != uid or getattr(child_st, 'st_flags', 0):
                if not allow_symlinks:
                    raise ValueError('owned-state child is not an isolated directory')
                raise ValueError('unclassified or protected state retained')
            if stat.S_ISDIR(child_st.st_mode):
                selected_present.append(name)
                top_dirs.append((root / name, name, child_st))
            elif not allow_symlinks:
                raise ValueError('owned-state child is not an isolated directory')
            elif stat.S_ISREG(child_st.st_mode):
                entry_count += 1
                if entry_count > max_entries:
                    raise ValueError('owned state exceeds max_entries limit')
                st_reg, file_sha = _hash_regular_via_dirfd(
                    root_fd, name, child_st, uid, max_total_bytes - total_bytes)
                total_bytes += st_reg.st_size
                file_count += 1
                selected_present.append(name)
                nodes.append(dict(
                    path=name,
                    kind='file',
                    mode=stat.S_IMODE(st_reg.st_mode),
                    flags=getattr(st_reg, 'st_flags', 0),
                    uid=st_reg.st_uid,
                    device=st_reg.st_dev,
                    inode=st_reg.st_ino,
                    mtime_ns=st_reg.st_mtime_ns,
                    bytes=st_reg.st_size,
                    sha256=file_sha,
                ))
            elif stat.S_ISLNK(child_st.st_mode):
                entry_count += 1
                if entry_count > max_entries:
                    raise ValueError('owned state exceeds max_entries limit')
                link_len, link_target, link_sha = _read_symlink_via_dirfd(
                    root_fd, name, child_st, uid, max_total_bytes - total_bytes)
                total_bytes += link_len
                symlink_count += 1
                selected_present.append(name)
                nodes.append(dict(
                    path=name,
                    kind='symlink',
                    mode=stat.S_IMODE(child_st.st_mode),
                    flags=getattr(child_st, 'st_flags', 0),
                    uid=child_st.st_uid,
                    device=child_st.st_dev,
                    inode=child_st.st_ino,
                    mtime_ns=child_st.st_mtime_ns,
                    bytes=link_len,
                    link_target=link_target,
                    sha256=link_sha,
                ))
            else:
                raise ValueError('unsupported special file in owned state')
    finally:
        os.close(root_fd)

    parents = []
    if empty_parents:
        if (len(empty_parents) != 2
                or tuple(Path(p).absolute() for p, _ in empty_parents) != (root, root.parent)):
            raise ValueError('unsafe owned environment parents or evidence location')
        for (directory, original), relative in zip(empty_parents, ('.', '..')):
            directory = Path(directory).absolute()
            orig_dev_ino = _extract_node_pair(original)
            if directory.is_symlink() or directory.resolve() != directory:
                raise ValueError('owned environment parent changed before retirement')
            dir_fd, node = _open_verified_dir(
                directory, expected_dev_ino=orig_dev_ino, uid=uid,
                error_msg='owned environment parent changed before retirement')
            try:
                children = sorted(os.listdir(dir_fd))
            finally:
                os.close(dir_fd)
            parents.append(dict(
                path=relative,
                kind='directory',
                mode=stat.S_IMODE(node.st_mode),
                flags=getattr(node, 'st_flags', 0),
                uid=node.st_uid,
                device=node.st_dev,
                inode=node.st_ino,
                mtime_ns=node.st_mtime_ns,
                bytes=node.st_size,
                children=children,
            ))
        if (any(child not in selected_names for child in parents[0]['children'])
                or parents[1]['children'] != [root.name]):
            raise ValueError('unselected environment state retained')

    root_state = dict(
        path='.',
        kind='directory',
        mode=stat.S_IMODE(root_st.st_mode),
        flags=getattr(root_st, 'st_flags', 0),
        uid=root_st.st_uid,
        device=root_st.st_dev,
        inode=root_st.st_ino,
        selected_present=selected_present,
    )

    stack = list(reversed(top_dirs))
    while stack:
        dir_path, rel_dir, expected_st = stack.pop()
        rel_parts = Path(rel_dir)
        if rel_parts.is_absolute() or '..' in rel_parts.parts:
            raise ValueError('unsafe relative path in owned state')
        entry_count += 1
        if entry_count > max_entries:
            raise ValueError('owned state exceeds max_entries limit')
        if dir_path.is_symlink() or dir_path.resolve() != dir_path:
            raise ValueError('owned directory changed during snapshot')
        dir_fd, dir_st = _open_verified_dir(
            dir_path,
            expected_dev_ino=(expected_st.st_dev, expected_st.st_ino),
            uid=uid,
            error_msg='owned directory changed during snapshot',
        )
        subdirs = []
        try:
            entries = sorted(os.listdir(dir_fd))
            nodes.append(dict(
                path=rel_dir,
                kind='directory',
                mode=stat.S_IMODE(dir_st.st_mode),
                flags=getattr(dir_st, 'st_flags', 0),
                uid=dir_st.st_uid,
                device=dir_st.st_dev,
                inode=dir_st.st_ino,
                mtime_ns=dir_st.st_mtime_ns,
                bytes=dir_st.st_size,
                children=entries,
            ))
            dir_count += 1
            for leaf in entries:
                if leaf in ('', '.', '..') or '/' in leaf or '\\' in leaf:
                    raise ValueError('unsafe entry name in owned state')
                rel_leaf = f'{rel_dir}/{leaf}'
                try:
                    lst = os.stat(leaf, dir_fd=dir_fd, follow_symlinks=False)
                except OSError as exc:
                    raise ValueError('owned entry disappeared during snapshot') from exc
                if lst.st_uid != uid or getattr(lst, 'st_flags', 0):
                    raise ValueError('unclassified or protected state retained')
                if stat.S_ISDIR(lst.st_mode):
                    subdirs.append((dir_path / leaf, rel_leaf, lst))
                elif stat.S_ISREG(lst.st_mode):
                    entry_count += 1
                    if entry_count > max_entries:
                        raise ValueError('owned state exceeds max_entries limit')
                    st_reg, file_sha = _hash_regular_via_dirfd(
                        dir_fd, leaf, lst, uid, max_total_bytes - total_bytes)
                    total_bytes += st_reg.st_size
                    file_count += 1
                    nodes.append(dict(
                        path=rel_leaf,
                        kind='file',
                        mode=stat.S_IMODE(st_reg.st_mode),
                        flags=getattr(st_reg, 'st_flags', 0),
                        uid=st_reg.st_uid,
                        device=st_reg.st_dev,
                        inode=st_reg.st_ino,
                        mtime_ns=st_reg.st_mtime_ns,
                        bytes=st_reg.st_size,
                        sha256=file_sha,
                    ))
                elif stat.S_ISLNK(lst.st_mode):
                    if not allow_symlinks:
                        raise ValueError('symlink not permitted in owned state')
                    entry_count += 1
                    if entry_count > max_entries:
                        raise ValueError('owned state exceeds max_entries limit')
                    link_len, link_target, link_sha = _read_symlink_via_dirfd(
                        dir_fd, leaf, lst, uid, max_total_bytes - total_bytes)
                    total_bytes += link_len
                    symlink_count += 1
                    nodes.append(dict(
                        path=rel_leaf,
                        kind='symlink',
                        mode=stat.S_IMODE(lst.st_mode),
                        flags=getattr(lst, 'st_flags', 0),
                        uid=lst.st_uid,
                        device=lst.st_dev,
                        inode=lst.st_ino,
                        mtime_ns=lst.st_mtime_ns,
                        bytes=link_len,
                        link_target=link_target,
                        sha256=link_sha,
                    ))
                else:
                    raise ValueError('unsupported special file in owned state')
        finally:
            os.close(dir_fd)
        for item in reversed(subdirs):
            stack.append(item)

    nodes.sort(key=lambda row: row['path'])
    counts = dict(
        entries=len(nodes),
        files=file_count,
        directories=dir_count,
        symlinks=symlink_count,
        parents=len(parents),
        total_bytes=total_bytes,
    )
    body = dict(
        schema=SNAPSHOT_SCHEMA,
        root=str(root),
        root_state=root_state,
        names=selected_names,
        allow_symlinks=allow_symlinks,
        max_entries=max_entries,
        max_total_bytes=max_total_bytes,
        parents=parents,
        nodes=nodes,
        counts=counts,
    )
    body['tree_sha256'] = _compute_snapshot_digest(body)
    return body


def snapshot_owned_state(root, names, *, empty_parents=(), allow_symlinks=False,
                         max_entries=DEFAULT_MAX_ENTRIES,
                         max_total_bytes=DEFAULT_MAX_TOTAL_BYTES):
    """Capture a verified, bounded closure-time proof of an owned case directory."""
    if not isinstance(allow_symlinks, bool):
        raise ValueError('allow_symlinks must be a boolean')
    max_entries = _validate_positive_int(max_entries, 'max_entries')
    max_total_bytes = _validate_positive_int(max_total_bytes, 'max_total_bytes')
    root = Path(root).absolute()
    selected_names = _validate_names(names)
    snapshot = _capture_owned_state_once(
        root,
        selected_names,
        empty_parents=empty_parents,
        allow_symlinks=allow_symlinks,
        max_entries=max_entries,
        max_total_bytes=max_total_bytes,
    )
    # Re-verify root/parent inodes, directory sets, and full streaming content hashes
    # after traversal completes so concurrent mid-walk mutations never yield a proof.
    verify_owned_state(snapshot)
    return snapshot


def verify_owned_state(snapshot):
    """Verify that an owned directory tree still matches a prior snapshot_owned_state."""
    if not isinstance(snapshot, dict) or snapshot.get('schema') != SNAPSHOT_SCHEMA:
        raise ValueError('invalid owned-state snapshot schema')
    raw_root = snapshot.get('root')
    if not isinstance(raw_root, str) or not raw_root or not Path(raw_root).is_absolute():
        raise ValueError('invalid owned-state snapshot root')
    root = Path(raw_root)
    allow_symlinks = snapshot.get('allow_symlinks')
    if not isinstance(allow_symlinks, bool):
        raise ValueError('invalid owned-state snapshot symlink policy')
    max_entries = _validate_positive_int(snapshot.get('max_entries'), 'max_entries')
    max_total_bytes = _validate_positive_int(snapshot.get('max_total_bytes'), 'max_total_bytes')
    names_raw = snapshot.get('names')
    if not isinstance(names_raw, list):
        raise ValueError('invalid owned-state snapshot names')
    selected_names = _validate_names(names_raw)
    if names_raw != selected_names:
        raise ValueError('non-canonical owned-state snapshot names')
    parents_raw = snapshot.get('parents')
    if (not isinstance(parents_raw, list)
            or len(parents_raw) not in (0, 2)
            or not isinstance(snapshot.get('nodes'), list)
            or not isinstance(snapshot.get('counts'), dict)
            or not isinstance(snapshot.get('root_state'), dict)
            or not isinstance(snapshot.get('tree_sha256'), str)):
        raise ValueError('malformed owned-state snapshot structure')

    verify_payload = {k: v for k, v in snapshot.items() if k != 'tree_sha256'}
    if _compute_snapshot_digest(verify_payload) != snapshot['tree_sha256']:
        raise ValueError('owned-state snapshot digest mismatch')

    empty_parents = ()
    if parents_raw:
        if [row.get('path') for row in parents_raw] != ['.', '..']:
            raise ValueError('invalid owned-state snapshot parents')
        empty_parents = (
            (root, parents_raw[0]),
            (root.parent, parents_raw[1]),
        )

    current = _capture_owned_state_once(
        root,
        selected_names,
        empty_parents=empty_parents,
        allow_symlinks=allow_symlinks,
        max_entries=max_entries,
        max_total_bytes=max_total_bytes,
    )
    if current['root_state'] != snapshot['root_state']:
        raise ValueError('owned root state changed')
    if current['parents'] != snapshot['parents']:
        raise ValueError('owned parent directory state changed')
    if current['counts'] != snapshot['counts']:
        raise ValueError('owned state node counts or byte totals changed')
    if current['nodes'] != snapshot['nodes'] or current['tree_sha256'] != snapshot['tree_sha256']:
        raise ValueError('owned state nodes or content changed')
    return None


def _collect_retirement_plan(root, selected_names, uid):
    """Build file rows and bottom-up directory rows with single-FD content hashes."""
    rows = []
    directories = []
    root_fd, _ = _open_verified_dir(
        root, uid=uid, error_msg='unsafe owned-state retirement scope')
    try:
        root_entries = set(os.listdir(root_fd))
        top_children = []
        for name in selected_names:
            if name not in root_entries:
                try:
                    os.stat(name, dir_fd=root_fd, follow_symlinks=False)
                except FileNotFoundError:
                    continue
            child_st = os.stat(name, dir_fd=root_fd, follow_symlinks=False)
            if (stat.S_ISLNK(child_st.st_mode)
                    or not stat.S_ISDIR(child_st.st_mode)
                    or child_st.st_uid != uid
                    or getattr(child_st, 'st_flags', 0)):
                raise ValueError('owned-state child is not an isolated directory')
            top_children.append((root / name, name, child_st))
    finally:
        os.close(root_fd)

    for top_path, top_rel, top_st in top_children:
        stack = [(top_path, top_rel, top_st, False)]
        while stack:
            dir_path, rel_dir, expected_st, visited = stack.pop()
            if visited:
                directories.append(expected_st)
                continue
            if dir_path.is_symlink() or dir_path.resolve() != dir_path:
                raise ValueError('unclassified or protected directory retained')
            dir_fd, dir_st = _open_verified_dir(
                dir_path,
                expected_dev_ino=(expected_st.st_dev, expected_st.st_ino),
                uid=uid,
                error_msg='unclassified or protected directory retained',
            )
            dir_record = dict(
                path=rel_dir,
                kind='directory',
                path_id_sha256=hashlib.sha256(rel_dir.encode()).hexdigest(),
                mode=stat.S_IMODE(dir_st.st_mode),
                device=dir_st.st_dev,
                inode=dir_st.st_ino,
                mtime_ns=dir_st.st_mtime_ns,
            )
            stack.append((dir_path, rel_dir, dir_record, True))
            subdirs = []
            try:
                for leaf in sorted(os.listdir(dir_fd)):
                    rel_leaf = f'{rel_dir}/{leaf}'
                    lst = os.stat(leaf, dir_fd=dir_fd, follow_symlinks=False)
                    if lst.st_uid != uid or getattr(lst, 'st_flags', 0):
                        if stat.S_ISDIR(lst.st_mode):
                            raise ValueError('unclassified or protected directory retained')
                        raise ValueError('unclassified or protected state retained')
                    if stat.S_ISDIR(lst.st_mode):
                        subdirs.append((dir_path / leaf, rel_leaf, lst, False))
                    elif stat.S_ISREG(lst.st_mode):
                        st_reg, file_sha = _hash_regular_via_dirfd(
                            dir_fd, leaf, lst, uid, None,
                            change_msg='owned state changed before deletion')
                        rows.append(dict(
                            path=rel_leaf,
                            path_id_sha256=hashlib.sha256(rel_leaf.encode()).hexdigest(),
                            sha256=file_sha,
                            bytes=st_reg.st_size,
                            allocated_bytes=st_reg.st_blocks * 512,
                            mode=stat.S_IMODE(st_reg.st_mode),
                            device=st_reg.st_dev,
                            inode=st_reg.st_ino,
                            mtime_ns=st_reg.st_mtime_ns,
                        ))
                    else:
                        raise ValueError('unclassified or protected state retained')
            finally:
                os.close(dir_fd)
            for item in reversed(subdirs):
                stack.append(item)
    return rows, directories


def _validate_expected_state_scope(expected_state, root, selected_names, has_parents):
    if (not isinstance(expected_state, dict)
            or expected_state.get('schema') != SNAPSHOT_SCHEMA
            or expected_state.get('root') != str(root)
            or expected_state.get('names') != selected_names
            or expected_state.get('allow_symlinks') is not False
            or not isinstance(expected_state.get('parents'), list)
            or bool(expected_state.get('parents')) != has_parents
            or not isinstance(expected_state.get('nodes'), list)
            or not isinstance(expected_state.get('counts'), dict)
            or not isinstance(expected_state.get('root_state'), dict)
            or not isinstance(expected_state.get('tree_sha256'), str)):
        raise ValueError('expected_state scope does not match retirement target')
    verify_payload = {k: v for k, v in expected_state.items() if k != 'tree_sha256'}
    if _compute_snapshot_digest(verify_payload) != expected_state['tree_sha256']:
        raise ValueError('owned-state snapshot digest mismatch')


def _plan_matches_expected_state(rows, directories, expected_state):
    nodes = expected_state['nodes']
    if any(node.get('kind') not in ('file', 'directory') for node in nodes):
        return False
    expected_files = {
        node['path']: (
            node['device'], node['inode'], node['mode'],
            node['bytes'], node['mtime_ns'], node['sha256'],
        )
        for node in nodes if node['kind'] == 'file'
    }
    actual_files = {
        row['path']: (
            row['device'], row['inode'], row['mode'],
            row['bytes'], row['mtime_ns'], row['sha256'],
        )
        for row in rows
    }
    if actual_files != expected_files or len(rows) != len(expected_files):
        return False
    expected_dirs = {
        node['path']: (node['device'], node['inode'], node['mode'], node['mtime_ns'])
        for node in nodes if node['kind'] == 'directory'
    }
    non_parent_dirs = [row for row in directories if not row.get('empty_parent')]
    actual_dirs = {
        row['path']: (row['device'], row['inode'], row['mode'], row['mtime_ns'])
        for row in non_parent_dirs
    }
    if actual_dirs != expected_dirs or len(non_parent_dirs) != len(expected_dirs):
        return False
    expected_parents = [
        (row['path'], row['device'], row['inode'], row['mode'], row['mtime_ns'])
        for row in expected_state['parents']
    ]
    actual_parents = [
        (row['path'], row['device'], row['inode'], row['mode'], row['mtime_ns'])
        for row in directories if row.get('empty_parent')
    ]
    return actual_parents == expected_parents


def _build_drift_retained(detail, include_occupancy_meta, attempt_index,
                          observation, prior_attempts):
    retained = dict(
        status='retained',
        reason='owned state changed after initial occupancy snapshot',
        state_drift=True,
        drift_detail=str(detail),
    )
    if include_occupancy_meta:
        retained['attempt_index'] = attempt_index
        retained['attempt_observation'] = observation
        retained['prior_occupancy_attempts'] = prior_attempts
    return retained


def retire(root, names, evidence, process, *, empty_parents=(), expected_state=None,
           attempt_index=1, prior_occupancy_attempts=(), record_occupancy=False):
    """Require a successful reaped process; preserve failures and unknown scope."""
    attempt_index = _validate_positive_int(attempt_index, 'attempt_index')
    if not isinstance(record_occupancy, bool):
        raise ValueError('record_occupancy must be a boolean')
    if isinstance(prior_occupancy_attempts, (str, bytes, dict)):
        raise ValueError('prior_occupancy_attempts must be a sequence of observations')
    prior_attempts = []
    for item in prior_occupancy_attempts:
        if not isinstance(item, dict):
            raise ValueError('prior_occupancy_attempts must contain dicts')
        prior_attempts.append(dict(item))

    root = Path(root).absolute()
    evidence = Path(evidence).absolute()
    if (process.get('complete') is not True or process.get('exit_code') != 0
            or process.get('remaining_group_terminated') is not False):
        return dict(status='retained', reason='process not successfully reaped')
    selected_names = _validate_names(names)
    has_parents = bool(empty_parents)
    if evidence.is_symlink() or not evidence.is_file():
        raise ValueError('unsafe owned-state retirement scope')
    if has_parents:
        if (len(empty_parents) != 2
                or tuple(Path(path).absolute() for path, _ in empty_parents) != (root, root.parent)
                or evidence.resolve().is_relative_to(root.parent)):
            raise ValueError('unsafe owned environment parents or evidence location')
        for _, original in empty_parents:
            _extract_node_pair(original)
    if expected_state is not None:
        _validate_expected_state_scope(expected_state, root, selected_names, has_parents)

    include_occupancy_meta = bool(record_occupancy or prior_attempts or attempt_index > 1)
    uid = os.getuid()

    def _inspect_roots_and_parents():
        if root.resolve() != root or not root.is_dir() or root.is_symlink():
            raise ValueError('unsafe owned-state retirement scope')
        r_st = root.lstat()
        if (not stat.S_ISDIR(r_st.st_mode)
                or r_st.st_uid != uid
                or getattr(r_st, 'st_flags', 0)):
            raise ValueError('unsafe owned-state retirement scope')
        if expected_state is not None:
            snap_root = expected_state['root_state']
            if (r_st.st_dev, r_st.st_ino) != (snap_root.get('device'), snap_root.get('inode')):
                raise ValueError('owned root inode changed')
        p_rows = []
        outer_id = None
        if has_parents:
            # Only the two directories created by the per-file runner qualify.
            # Their original nodes prove ownership; fresh metadata records the
            # actual state after the completed process, before any deletion.
            for (directory, original), relative in zip(empty_parents, ('.', '..')):
                directory = Path(directory).absolute()
                orig_dev_ino = _extract_node_pair(original)
                node = directory.lstat()
                if (directory.resolve() != directory or not stat.S_ISDIR(node.st_mode)
                        or node.st_uid != uid
                        or getattr(node, 'st_flags', 0)
                        or (node.st_dev, node.st_ino) != orig_dev_ino):
                    raise ValueError('owned environment parent changed before retirement')
                p_rows.append(dict(
                    path=relative,
                    kind='directory',
                    empty_parent=True,
                    path_id_sha256=hashlib.sha256(relative.encode()).hexdigest(),
                    mode=stat.S_IMODE(node.st_mode),
                    device=node.st_dev,
                    inode=node.st_ino,
                    mtime_ns=node.st_mtime_ns,
                ))
            if (any(child.name not in selected_names for child in root.iterdir())
                    or set(root.parent.iterdir()) != {root}):
                raise ValueError('unselected environment state retained')
            outer_parent = root.parent.parent
            outer_st = outer_parent.lstat()
            if outer_parent.resolve() != outer_parent or not stat.S_ISDIR(outer_st.st_mode):
                raise ValueError('unsafe owned environment parents or evidence location')
            outer_id = (outer_st.st_dev, outer_st.st_ino)
        return r_st, p_rows, outer_id

    if expected_state is None:
        root_st, parent_rows, outer_parent_dev_ino = _inspect_roots_and_parents()
    else:
        # Before running lsof +D, ensure the physical directory nodes still belong
        # to the original snapshot so we never probe a replaced symlink/foreign node.
        try:
            if root.is_symlink() or not root.is_dir() or root.resolve() != root:
                raise ValueError('owned root is no longer a physical directory')
            pre_root_st = root.lstat()
            snap_root = expected_state['root_state']
            if (not stat.S_ISDIR(pre_root_st.st_mode)
                    or pre_root_st.st_uid != uid
                    or getattr(pre_root_st, 'st_flags', 0)
                    or (pre_root_st.st_dev, pre_root_st.st_ino) !=
                    (snap_root.get('device'), snap_root.get('inode'))):
                raise ValueError('owned root node changed before occupancy recheck')
            if has_parents:
                parent_dir = root.parent
                if parent_dir.is_symlink() or not parent_dir.is_dir() or parent_dir.resolve() != parent_dir:
                    raise ValueError('owned parent is no longer a physical directory')
                pre_parent_st = parent_dir.lstat()
                orig_parent_dev_ino = _extract_node_pair(empty_parents[1][1])
                if (not stat.S_ISDIR(pre_parent_st.st_mode)
                        or pre_parent_st.st_uid != uid
                        or getattr(pre_parent_st, 'st_flags', 0)
                        or (pre_parent_st.st_dev, pre_parent_st.st_ino) != orig_parent_dev_ino):
                    raise ValueError('owned parent node changed before occupancy recheck')
        except (ValueError, OSError) as error:
            return _build_drift_retained(
                error, include_occupancy_meta, attempt_index, None, prior_attempts)

    occupancy = check_occupancy(root.parent if has_parents else root)
    observation = dict(occupancy, attempt_index=attempt_index)
    if not occupancy['clear']:
        reason = ('case occupancy query timed out'
                  if occupancy['timed_out']
                  else 'case handles present or occupancy unknown')
        retained = dict(status='retained', reason=reason)
        if include_occupancy_meta:
            retained['attempt_index'] = attempt_index
            retained['attempt_observation'] = observation
            retained['prior_occupancy_attempts'] = prior_attempts
        return retained

    report = evidence.with_name(evidence.stem + '.state-retirement.json')
    ledger = evidence.with_name(evidence.stem + '.state-retirement.jsonl.gz')
    private = report.with_suffix('.private.json')
    if (report.exists() or report.is_symlink()
            or ledger.exists() or ledger.is_symlink()
            or private.exists() or private.is_symlink()):
        raise ValueError('retirement identity already exists; inspect original outcome')

    if expected_state is not None:
        try:
            verify_owned_state(expected_state)
            root_st, parent_rows, outer_parent_dev_ino = _inspect_roots_and_parents()
            rows, directories = _collect_retirement_plan(root, selected_names, uid)
            directories.extend(parent_rows)
            if not _plan_matches_expected_state(rows, directories, expected_state):
                raise ValueError('retirement plan differs from initial occupancy snapshot')
        except (ValueError, OSError) as error:
            return _build_drift_retained(
                error, include_occupancy_meta, attempt_index, observation, prior_attempts)
    else:
        rows, directories = _collect_retirement_plan(root, selected_names, uid)
        directories.extend(parent_rows)

    dir_by_rel = {row['path']: row for row in directories}
    schema = 3 if parent_rows else 2
    original_evidence = digest(evidence)

    with ledger.open('xb') as raw:
        with gzip.GzipFile(filename='', mode='wb', fileobj=raw, mtime=0) as stream:
            for row in [*rows, *directories]:
                # Never publish machine-encoded compiler/cache leaf names.
                public = {key: value for key, value in row.items()
                          if key not in ('path', 'device', 'inode', 'mtime_ns')}
                stream.write((json.dumps(public, sort_keys=True) + '\n').encode())
        raw.flush()
        os.fsync(raw.fileno())

    private_payload = dict(
        schema=schema,
        root=str(root),
        files=rows,
        directories=directories,
        evidence_sha256=original_evidence,
    )
    if include_occupancy_meta:
        private_payload.update(
            attempt_index=attempt_index,
            attempt_observation=observation,
            prior_occupancy_attempts=prior_attempts,
        )
    if expected_state is not None:
        private_payload['expected_state_tree_sha256'] = expected_state['tree_sha256']

    with private.open('x') as stream:
        json.dump(private_payload, stream)
        stream.flush()
        os.fsync(stream.fileno())

    fd = os.open(report.parent, _DIR_OPEN_FLAGS)
    try:
        os.fsync(fd)
    finally:
        os.close(fd)

    if digest(evidence) != original_evidence:
        raise ValueError('case evidence changed before retirement')

    for row in rows:
        rel_path = Path(row['path'])
        parent_rel = rel_path.parent.as_posix()
        expected_parent = dir_by_rel.get(parent_rel)
        if expected_parent is None:
            raise ValueError('owned state changed before deletion')
        parent_fd, _ = _open_verified_dir(
            root / parent_rel,
            expected_dev_ino=(expected_parent['device'], expected_parent['inode']),
            uid=uid,
            error_msg='owned state changed before deletion',
        )
        try:
            leaf_name = rel_path.name
            try:
                metadata = os.stat(leaf_name, dir_fd=parent_fd, follow_symlinks=False)
            except OSError as exc:
                raise ValueError('owned state changed before deletion') from exc
            if (not stat.S_ISREG(metadata.st_mode)
                    or metadata.st_uid != uid
                    or getattr(metadata, 'st_flags', 0)
                    or (metadata.st_dev, metadata.st_ino, metadata.st_mtime_ns,
                        metadata.st_size, stat.S_IMODE(metadata.st_mode)) !=
                    (row['device'], row['inode'], row['mtime_ns'],
                     row['bytes'], row['mode'])):
                raise ValueError('owned state changed before deletion')
            _, file_sha = _hash_regular_via_dirfd(
                parent_fd, leaf_name, metadata, uid, None,
                change_msg='owned state changed before deletion')
            if file_sha != row['sha256']:
                raise ValueError('owned state changed before deletion')
            os.unlink(leaf_name, dir_fd=parent_fd)
        finally:
            os.close(parent_fd)

    for row in directories:
        if row.get('empty_parent') and row['path'] == '..':
            parent_dir = root.parent.parent
            expected_parent_id = outer_parent_dev_ino
            entry_name = root.parent.name
            check_parent_uid = False
        elif row.get('empty_parent') and row['path'] == '.':
            parent_dir = root.parent
            expected_parent_id = (dir_by_rel['..']['device'], dir_by_rel['..']['inode'])
            entry_name = root.name
            check_parent_uid = True
        else:
            rel_path = Path(row['path'])
            entry_name = rel_path.name
            if rel_path.parent == Path('.'):
                parent_dir = root
                expected_parent_id = (root_st.st_dev, root_st.st_ino)
            else:
                parent_rel = rel_path.parent.as_posix()
                parent_record = dir_by_rel.get(parent_rel)
                if parent_record is None:
                    raise ValueError('owned directory changed before deletion')
                parent_dir = root / parent_rel
                expected_parent_id = (parent_record['device'], parent_record['inode'])
            check_parent_uid = True

        parent_fd, _ = _open_verified_dir(
            parent_dir,
            expected_dev_ino=expected_parent_id,
            uid=uid if check_parent_uid else None,
            error_msg='owned directory changed before deletion',
        )
        try:
            try:
                metadata = os.stat(entry_name, dir_fd=parent_fd, follow_symlinks=False)
            except OSError as exc:
                raise ValueError('owned directory changed before deletion') from exc
            if (not stat.S_ISDIR(metadata.st_mode)
                    or metadata.st_uid != uid
                    or getattr(metadata, 'st_flags', 0)
                    or (metadata.st_dev, metadata.st_ino, stat.S_IMODE(metadata.st_mode)) !=
                    (row['device'], row['inode'], row['mode'])):
                raise ValueError('owned directory changed before deletion')
            os.rmdir(entry_name, dir_fd=parent_fd)
        finally:
            os.close(parent_fd)

    if digest(evidence) != original_evidence:
        raise ValueError('case evidence changed during retirement')

    result = dict(
        status='complete',
        files=len(rows),
        directories=len(directories),
        ledger_schema=schema,
        logical_bytes=sum(row['bytes'] for row in rows),
        allocated_bytes=sum(row['allocated_bytes'] for row in rows),
        evidence_sha256=original_evidence,
        ledger=ledger.name,
        ledger_sha256=digest(ledger),
        all_selected_absent=True,
        restore='regenerate consumed cache from retained inputs; original random names not promised',
    )
    if parent_rows:
        result.update(empty_parents_retired=True, parent_directories=len(parent_rows))
    if include_occupancy_meta:
        result.update(
            attempt_index=attempt_index,
            attempt_observation=observation,
            prior_occupancy_attempts=prior_attempts,
        )
    if expected_state is not None:
        result.update(
            expected_state_verified=True,
            expected_state_tree_sha256=expected_state['tree_sha256'],
        )
    with report.open('x') as stream:
        json.dump(result, stream, indent=2)
        stream.flush()
        os.fsync(stream.fileno())
    return result
